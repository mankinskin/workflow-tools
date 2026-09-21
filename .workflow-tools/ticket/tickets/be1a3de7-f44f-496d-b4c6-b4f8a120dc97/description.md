# Protocol Layer — Human CLI vs Agent Protocol

## Core Principle

`TaskCommand` is the canonical machine protocol.
The human CLI is a UX adapter on top of the same command contract.

All agent-oriented surfaces send structured `TaskCommand` JSON directly:

- `ticket exec` reads one command or a transactional batch from stdin
- `ticket serve --stdio` speaks persistent JSONL over stdio
- `context-http` accepts the same command payload over HTTP
- `context-mcp` wraps the same command payload for tool calls

The CLI subcommand layer exists for human ergonomics only.

## Split of Responsibilities

### Human Surface

Human operators use CLI subcommands optimized for discoverability and shell use:

- short flags
- prefix matching where safe
- cwd-based index-root inference
- terminal-oriented formatting

Example:

```bash
ticket update a3f2 -s done
```

### Agent Surface

Agents use explicit, self-contained JSON payloads with no shell quoting burden:

- full UUIDs only
- structured patch objects, not `--field k=v` parsing
- explicit `index_root`, no cwd inference
- response field selection to reduce token output
- optional persistent session transport for long-running work

Example:

```json
{
  "command": "task_update",
  "index_root": "/absolute/path/to/ticket-index",
  "id": "a3f2c7b1-4e9d-4f0a-8c3b-1d2e5f6a7b8c",
  "patch": {
    "state": "done"
  },
  "fields": ["id", "state", "updated_at"]
}
```

## Supported Protocol Modes

### Mode 1: Human CLI

`ticket create`, `ticket get`, `ticket update`, `ticket list`, etc.

Use case:
- humans doing ad-hoc operations
- local debugging
- shell scripting

Properties:
- excellent ergonomics for people
- poor token efficiency for agents
- one process spawn per operation

### Mode 2: `ticket exec`

Stateless stdin JSON command execution.

Examples:

```bash
echo '{"command":"task_create","index_root":"/abs/path","title":"wire CRUD","state":"open"}' | ticket exec
```

```bash
cat <<'JSON' | ticket exec --batch
{"command":"task_search","index_root":"/abs/path","query":"state:open","limit":5}
{"command":"task_claim","index_root":"/abs/path","id":"a3f2c7b1-4e9d-4f0a-8c3b-1d2e5f6a7b8c","intent":"implementing feature"}
JSON
```

Use case:
- agents doing discrete operations
- test harnesses
- non-persistent automation

Properties:
- no shell quoting issues
- lower token cost than CLI flags
- transactional batch support
- still pays process startup cost per invocation

### Mode 3: `ticket serve --stdio`

Persistent JSONL command transport over stdio.

Example session:

```json
{"id":1,"command":"task_search","query":"state:open","limit":1}
{"id":2,"command":"task_claim","id":"a3f2c7b1-4e9d-4f0a-8c3b-1d2e5f6a7b8c","intent":"refining"}
{"id":3,"command":"task_update","id":"a3f2c7b1-4e9d-4f0a-8c3b-1d2e5f6a7b8c","patch":{"state":"in-progress"}}
```

Use case:
- lease-heavy long-running agents
- swarm coordinators
- high-throughput workflows

Properties:
- one process spawn per session
- redb and Tantivy stay open for session lifetime
- request/response correlation by request ID
- server-side lease renewal while connection is alive

## Self-Containment Contract

Agent protocol payloads must be fully explicit.

Required rules:

- include `index_root` on every `ticket exec`, HTTP, and MCP command
- use full UUIDs, never short prefixes
- encode updates as structured objects
- never rely on cwd, environment, or shell escaping for correctness

`ticket serve --stdio` binds `index_root` at startup, then all later requests are scoped to that session.

## Batch Semantics

`ticket exec --batch` is transactional.

- the entire batch succeeds or fails as a unit
- no partial commits to filesystem, redb, Tantivy, or history
- deterministic replay is a design goal
- batch failure returns the failing command index and structured error envelope

This is intentionally stricter than "best effort" scripting because agent-driven automation
must not leave ambiguous partial state behind.

## Lease Renewal Policy

For `ticket serve --stdio`, the server automatically renews leases held by the session
while the connection remains healthy.

Implications:

- agents do not need to emit explicit heartbeat commands in the common persistent case
- `ticket heartbeat` remains available for stateless transports and external adapters
- if the process hangs but the connection stays alive, progress may stall while the lease remains valid
- lease events must record renewal source (`manual_heartbeat` vs `session_liveness`)

## Response Shape Controls

Agent protocol commands support response projection:

```json
{
  "command": "task_get",
  "index_root": "/absolute/path/to/ticket-index",
  "id": "a3f2c7b1-4e9d-4f0a-8c3b-1d2e5f6a7b8c",
  "fields": ["id", "state", "title"]
}
```

Rules:

- projection is optional
- omitted `fields` means full response envelope
- invalid field selectors return structured validation error
- selectors apply to result payloads, not to error envelopes

## Swarm Instruction Model

Swarm operation uses a dispatch model with independent workers.

- coordinator searches, schedules, and dispatches work
- workers independently call the ticket system to claim, update, and release work
- coordinator does not become the sole writer for all worker progress
- ticket system remains the shared source of truth for scalable swarm state

This avoids coordinator context overload while preserving distributed execution.

## Assignment Packet Contract

Coordinator-to-agent dispatch uses a single structured assignment packet.
This packet is transport-agnostic and can be delivered through CLI prompt injection,
MCP tool payloads, HTTP callbacks, or message queues.

### Worker Assignment Packet

```json
{
  "packet_version": "1",
  "assignment_id": "a-20260320-000123",
  "role": "worker",
  "agent_id": "worker/refiner-03",
  "ticket": {
    "id": "a3f2c7b1-4e9d-4f0a-8c3b-1d2e5f6a7b8c",
    "state": "open",
    "title": "wire create/get/update/list/delete to storage backend",
    "risk_level": "high",
    "acceptance_criteria": "...",
    "validation_plan": "...",
    "release_target": "2026.03-train-A"
  },
  "execution": {
    "intent": "implement",
    "constraints": [
      "no API break",
      "touch only context-tasks crate"
    ],
    "branch": {
      "feature": "feature/task-backend-core",
      "merge_target": "main"
    }
  },
  "protocol": {
    "mode": "serve-stdio",
    "index_root": "/absolute/path/to/ticket-index",
    "session_id": "sess-42",
    "required_commands": [
      "task_claim",
      "task_update",
      "task_unclaim"
    ]
  },
  "graph_context": {
    "blocked_by": ["..."],
    "blocking": ["..."],
    "conflict_domain": "storage-index"
  },
  "evidence_requirements": {
    "must_attach": [
      "commands_run",
      "test_results",
      "changed_files"
    ],
    "handoff_format": "swarm-worker-v1"
  }
}
```

### Validator Assignment Packet

```json
{
  "packet_version": "1",
  "assignment_id": "a-20260320-000124",
  "role": "validator",
  "agent_id": "validator/reliability-01",
  "ticket": {
    "id": "a3f2c7b1-4e9d-4f0a-8c3b-1d2e5f6a7b8c",
    "state": "validating",
    "risk_level": "high",
    "validation_status": "in-progress",
    "worker_id": "worker/refiner-03"
  },
  "validation": {
    "profile": "reliability",
    "required_checks": [
      "targeted-tests",
      "regression-suite",
      "crash-recovery-scenario"
    ],
    "pass_criteria": "all required checks green",
    "on_fail": "set review + attach evidence + open bug if defect"
  },
  "protocol": {
    "mode": "exec",
    "index_root": "/absolute/path/to/ticket-index",
    "required_commands": [
      "task_claim",
      "task_update",
      "task_unclaim"
    ]
  }
}
```

### Assignment Invariants

- `assignment_id` is unique and immutable.
- `ticket.id` must be full UUID.
- `protocol.index_root` is mandatory.
- `role=validator` requires `ticket.worker_id` and must not equal `agent_id`.
- packet must include explicit evidence requirements.

### Coordinator Dispatch Algorithm (minimal)

1. Select ready ticket from search/graph queue.
2. Build worker assignment packet with current ticket snapshot and constraints.
3. Dispatch to worker; worker claims ticket directly.
4. Track progress from ticket events (not from hidden coordinator state).
5. On transition to `validating`, build validator packet and dispatch.
6. Enforce separation-of-duties at packet creation and claim time.
7. Promote to release-candidate only after validation pass and bug gates.

## Validation and Release Command Contracts

The following commands are part of the canonical `TaskCommand` machine contract.
They are available through `ticket exec`, `ticket serve --stdio`, HTTP, and MCP adapters.

### `task_validate_start`

Purpose:
- move a ticket from `review` to `validating`
- bind validator identity and validation profile

Request:

```json
{
  "command": "task_validate_start",
  "index_root": "/absolute/path/to/ticket-index",
  "ticket_id": "a3f2c7b1-4e9d-4f0a-8c3b-1d2e5f6a7b8c",
  "assignment_id": "a-20260320-000124",
  "validator_id": "validator/reliability-01",
  "validation_profile": "reliability",
  "required_checks": ["targeted-tests", "regression-suite"]
}
```

Response:

```json
{
  "ok": true,
  "ticket": {
    "id": "a3f2c7b1-4e9d-4f0a-8c3b-1d2e5f6a7b8c",
    "state": "validating",
    "validation_status": "in-progress",
    "validator_id": "validator/reliability-01"
  }
}
```

Guards:
- current state must be `review`
- validator must differ from worker identity

Errors:
- `validate.invalid_state`
- `validate.same_identity`
- `validate.assignment_mismatch`

### `task_validate_result`

Purpose:
- submit validator outcome
- set ticket back to `review` on fail or to `validated` on pass

Request:

```json
{
  "command": "task_validate_result",
  "index_root": "/absolute/path/to/ticket-index",
  "ticket_id": "a3f2c7b1-4e9d-4f0a-8c3b-1d2e5f6a7b8c",
  "assignment_id": "a-20260320-000124",
  "validator_id": "validator/reliability-01",
  "result": "passed",
  "evidence_refs": [
    "test:target/test-logs/t3.log",
    "cmd:cargo test -p context-tasks"
  ],
  "summary": "All required checks green"
}
```

Response:

```json
{
  "ok": true,
  "ticket": {
    "id": "a3f2c7b1-4e9d-4f0a-8c3b-1d2e5f6a7b8c",
    "state": "validated",
    "validation_status": "passed"
  }
}
```

Failure result semantics:
- `result=failed` sets `state=review`, `validation_status=failed`
- may include linked bug ids in `bug_links`

Errors:
- `validate.invalid_state`
- `validate.assignment_mismatch`
- `validate.missing_evidence`

### `task_release_candidate_create`

Purpose:
- add a validated ticket to a release candidate

Request:

```json
{
  "command": "task_release_candidate_create",
  "index_root": "/absolute/path/to/ticket-index",
  "ticket_id": "a3f2c7b1-4e9d-4f0a-8c3b-1d2e5f6a7b8c",
  "release_target": "2026.03-train-A",
  "assignment_chain": [
    "a-20260320-000123",
    "a-20260320-000124"
  ]
}
```

Response:

```json
{
  "ok": true,
  "ticket": {
    "id": "a3f2c7b1-4e9d-4f0a-8c3b-1d2e5f6a7b8c",
    "state": "release-candidate",
    "release_target": "2026.03-train-A"
  }
}
```

Guards:
- current state must be `validated`
- `validation_status` must be `passed`
- assignment chain must contain worker + validator assignments

Errors:
- `release.invalid_state`
- `release.validation_not_passed`
- `release.assignment_chain_missing`

### `task_release_gate_check`

Purpose:
- evaluate release gates for a target train/candidate
- return machine-readable pass/fail with blocking reasons

Request:

```json
{
  "command": "task_release_gate_check",
  "index_root": "/absolute/path/to/ticket-index",
  "release_target": "2026.03-train-A",
  "required_gates": ["R1", "R2", "R3", "R4"]
}
```

Response:

```json
{
  "ok": true,
  "release_target": "2026.03-train-A",
  "gates": {
    "R1": "pass",
    "R2": "pass",
    "R3": "fail",
    "R4": "pass"
  },
  "blocking_reasons": [
    "R3: rollback verification missing for ticket 77f1eb5c-dc38-4221-89e9-2bdf2b8d3ca4"
  ]
}
```

Errors:
- `release.target_not_found`
- `release.gate_definition_missing`

### `task_release_promote`

Purpose:
- promote release candidate tickets from `release-candidate` to `released`
- attach merge/release metadata and start monitoring window

Request:

```json
{
  "command": "task_release_promote",
  "index_root": "/absolute/path/to/ticket-index",
  "release_target": "2026.03-train-A",
  "release_version": "2026.03.0",
  "merge_commit": "abc123def456",
  "gate_check_ref": "gate-20260320-001"
}
```

Response:

```json
{
  "ok": true,
  "release_target": "2026.03-train-A",
  "release_version": "2026.03.0",
  "promoted_ticket_count": 4,
  "monitoring_state": "active"
}
```

Guards:
- all required gates must be passing
- included tickets must be in `release-candidate`

Errors:
- `release.gates_not_satisfied`
- `release.ticket_state_invalid`
- `release.merge_metadata_missing`

### Cross-command invariants

- every command requires full UUIDs and explicit `index_root`
- every state-changing command must include `assignment_id` or assignment chain
- all decisions must be auditable via assignment-linked event records

## Rollout

### Phase 1

- human CLI subcommands
- `ticket exec` for agent JSON stdin
- shared `TaskCommand` contract for both

### Phase 1.5

- `ticket serve --stdio`
- session-bound lease renewal
- persistent redb/Tantivy handles for lease-heavy workflows

### Phase 5

- HTTP adapter
- MCP adapter
- same `TaskCommand` payload model across all transports

## Non-Goals

- no shell-string command construction requirement for agents
- no CLI flag parsing as the canonical machine interface
- no hidden context inference in machine protocol modes