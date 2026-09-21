# [AOH][Design] Full System Architecture — ADRs and Design Document

## Status

`COMPLETE` — All 15 ADRs are locked. All design blockers have been resolved:
- `02412b9a` — reconciled with Phase 2 execution tickets (**done**)
- `db784443` — operator authorization, secret lifecycle, trust boundaries (**done**)
- `ffa5361a` — session archive, artifact retention, revival schema (**done**)

---

## Architecture Decision Records (ADRs)

### ADR-1: Sandbox Isolation Tier
**Decision**: Container-based Browser-as-a-Service (BaaS) using Docker (primary) / Podman (Linux CI).  
**Rationale**: cloud-hypervisor / Firecracker lack `virtio-gpu`; browser workloads require GPU-capable containers.  
**Stack**:
- `bollard` for orchestration
- dedicated per-session Docker network
- allow-list proxy for outbound network control
- GPU flags: `--use-gl=angle` (Windows/WSL2), `--use-gl=egl` (Linux)
- bind-mounted git worktree
- `ContainerRuntime` trait for Docker/Podman portability
**Residual microVM use**: future non-browser isolation only.  
**Research gate**: `49d6fe2e` (**done** — bollard v0.20.2 API fully verified)

### ADR-2: Messaging Service
**Decision**: Telegram (primary MVP) + Discord + Slack behind a `Notifier` trait.  
**Rationale**: WhatsApp was dropped due to paid Meta Business requirements.  
**Routing**: `MultiNotifier` with configurable routing policy.  
**Research gate**: `89701593` (**done**)

### ADR-3: Git Hosting
**Decision**: GitHub remote with local-first PR management. Push remote branches only on explicit user trigger or merge. `git2` (libgit2) for v1; `gitoxide`/`gix` monitored for v2 when worktree + merge APIs stabilize.  
**Design gate**: `d3f76335` (**done** — full branch lifecycle, conflict detection, and git2 evaluation documented)

### ADR-4: Orchestrator Entrypoint
**Decision**: Rust daemon + ratatui TUI in v1. VS Code extension deferred to Phase 2.

### ADR-5: Agent API Provider
**Decision**: GitHub Copilot only in v1. Thin `CopilotClient` over `reqwest`. No provider abstraction in v1.  
**Research gate**: `cba080b5` (**done** — completions-only API confirmed, no session management, VS Code LM API requires VS Code running)

### ADR-6: Cross-Agent Coordination Protocol
**Decision**: `ticket-api` (tickets + draftboard) as the sole coordination layer in v1. Agents read/write ticket fields and board entries; orchestrator polls state. No in-process event bus for cross-agent coordination in v1. `tokio::broadcast` deferred to v2.  
**Clarification**: Intra-orchestrator event routing (runner → TUI, runner → notifier within the same Rust process) uses `tokio::mpsc` as an internal implementation detail. This is not a coordination protocol — it's process-internal plumbing. ticket-api remains the durable source of truth.

### ADR-7: MCP Routing for Parallel Sessions
**Decision**: Per-session MCP server sockets. Each session gets isolated MCP tool access. Uses `rmcp` v1.4.0 (official Anthropic Rust MCP SDK).  
**Research gate**: `1b681754` (**done** — rmcp identified as the canonical Rust MCP crate, 7.5M+ downloads)

### ADR-8: Agent Identity Scheme
**Decision**: Reusable nature-vocabulary personas with LRU assignment and same-persona revival. Pool of 25+ personas from `config/personas.toml`.  
**Design gate**: `d45826cd` (**done** — full schema, assignment algorithm, trait system, and Rust interface designed)

### ADR-9: Session Persistence and Revival
**Decision**: Summary-injected revival using `session-archive.toml` plus archived artifacts. On revival, agent branch is rebased onto latest main (ADR-15). Archive provides context; branch is authoritative after rebase.  
**Design gate**: `ffa5361a` (**done** — full TOML schema, artifact layout, retention policy, and revival reuse contract defined)

### ADR-10: Budget Controls
**Decision**: Tiered escalation with configurable token/time limits and sane defaults.  
**Defaults**:
- Soft token limit: 80,000
- Self-assessment window: 2,000 tokens
- User notify wait: 5 minutes
- Hard token limit: 200,000
- Time soft limit: 30 minutes
- Time hard limit: 90 minutes
- All thresholds configured in `orchestrator.toml`

### ADR-11: Branch Naming Convention
**Decision**: `aoh/{agent-id}/{ticket-slug}`  
**Rationale**: The `aoh/` prefix identifies all AOH-managed branches; agent-id scopes to a session; ticket-slug gives human-readable context without embedding the full UUID in the branch name.  
**Applies to**: git worktrees, remote branch pushes, PR metadata, archive artifact references, and reconciliation queries.

### ADR-12: Operator Authorization and Messenger Control
**Decision**: Full messenger control with a flat operator allow-list (Option C feature set, Option B identity model for v1).  
**Scope**: All control actions (approve review, request changes, reject, retry, stop session, extend budget, terminate) are accessible from Telegram, Discord, and Slack.  
**Identity (v1)**: Operator messenger user IDs listed in `orchestrator.toml`. Every inbound command checked against allow-list before dispatch.  
**Trust tiers**: Low (view), Medium (approve/reject/extend), High (terminate/push/merge), Critical (config changes — TUI only).  
**Design gate**: `db784443` (**done** — full secret inventory, replay protection, redaction rules, audit format)

### ADR-13: Secret Delivery
**Decision**: One-time HTTP or Unix-socket fetch from the orchestrator secret endpoint during container kickoff.  
**Mechanism**: UUIDv4 nonce per session, 60s TTL, single-use consumption. Docker Desktop uses `host.docker.internal:{port}` with per-session port. Linux CI uses bind-mounted Unix socket.  
**Storage at rest (v1)**: Plaintext in permission-restricted `orchestrator.toml` (0600). OS keyring deferred to v2.  
**Design gate**: `db784443` (**done**)

### ADR-14: Session Archive Layout and Retention
**Decision**: `.aoh/archive/{ticket-id}/{session-id}/` inside the repository working tree, excluded from git via `.aoh/` in `.gitignore`.  
**Layout**: `session-archive.toml` + `logs/` + `diffs/` + `evidence/` + `context/` subdirectories.  
**Retention**: Indefinite — archives kept until explicit `aoh archive prune`.  
**Revival chaining**: Each archive includes `revival_of` field forming a linked chain.  
**Design gate**: `ffa5361a` (**done** — full schema with 30+ fields, artifact naming conventions, and per-state cleanup policy)

### ADR-15: Revival Strategy
**Decision**: On revival, orchestrator rebases agent branch onto latest `main`. Agent resolves rebase conflicts in-container. After rebase, branch state is authoritative. Archive TOML supplies context (summary, change requests, open questions) via kickoff prompt injection.  
**Design gate**: `ffa5361a` (**done**)

---

## System Boundary Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│ User                                                            │
│   ↕ Telegram / Discord / Slack  (ADR-2, ADR-12)                │
│   ↕ ratatui TUI                 (ADR-4)                        │
│   ↕ VS Code (Phase 2 only)                                      │
└────────────────────────┬────────────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────────────┐
│ orchestrator-core daemon                   (NEW — needs ticket) │
│   Persona Store       LRU assignment + revival    (ADR-8)       │
│   Session Scheduler   parallel session planning   (ADR-10)      │
│   Secret Server       nonce-gated fetch endpoint  (ADR-13)      │
│   Conflict Detector   file-overlap detection                    │
│   PR Manager          local PR records + remote   (ADR-3)       │
│   Cost Watchdog       token/time budgets          (ADR-10)      │
│   MultiNotifier       Telegram → Discord → Slack  (ADR-2)       │
│   Audit Logger        .aoh/audit/audit.jsonl                    │
└──┬──────────────┬──────────────────────────────────────────────┘
   │              │
   ▼              ▼
[ticket-api]   [1..20 Agent Sessions]             (ADR-6)
 + draftboard  • Docker container via bollard       (ADR-1)
               • bind-mounted git worktree          (ADR-11)
               • per-session MCP via rmcp           (ADR-7)
               • persona-bound git identity         (ADR-8)
               • one-time secret fetch              (ADR-13)
               • archived to .aoh/archive/          (ADR-14)
```

## Crate Map

Planned AOH crates:

```
crates/
  orchestrator-core/     # daemon, scheduler, secret server, conflict detector
  sandbox-manager/       # ContainerRuntime trait, bollard impl, worktree mgmt
  agent-session/         # session state machine, kickoff, budget enforcement
  pr-manager/            # local PR records, review state, merge execution
  notifier/              # Notifier trait, Telegram/Discord/Slack/Desktop, CommandListener
  agent-identity/        # PersonaStore, LRU assignment, trait injection

tools/
  orchestrator-tui/      # ratatui application, operator command dispatch
```

## Session Lifecycle State Machine

```
[Ticket: ready]
       ↓
  Provisioning ── fail → ProvisionFailed
       ↓
  KickingOff ── fail → StartFailed
       ↓
  Running
       ├── soft budget → BudgetWarning → SelfAssessment → continue/escalate
       ├── hard budget → HardTerminate
       ↓
  Reporting
       ↓
  PROpen (local PR record)
       ├── approve → Merging → Merged → Archiving → Archived
       └── changes requested → RevivalQueue → Running (ADR-15: rebase + re-inject)
```

## Canonical Implementation Decomposition

> From reconciliation ticket `02412b9a` (**done**)

AOH implementation reuses the existing Phase 2 execution subtree under `d5ced7e2`:

| Track | Tickets | Dependencies |
|---|---|---|
| **A (Platform)** | `8c185de3` (Copilot client), `51471c3e` (Sandbox mgr), NEW (Persona store) | Bootstrap T1-T5 |
| **B (Orchestration)** | `a8632357` (Assignment runner), NEW (Orchestrator core) | Track A |
| **C (Operator UX)** | `8db8ef2f` (Notifier), `5af54f6c` (TUI), `d0cc3c8b` (Review coordinator) | Track B |
| **D (Quality)** | `0135d961` (E2E integration) | Tracks A+B+C |

**Gaps identified**: 2 new implementation tickets needed:
1. **orchestrator-core** — main daemon, scheduler, secret server
2. **agent-identity** — persona store crate

All 13 AOH research/design tickets are classified as inputs to these implementation tickets. No duplicate ownership.

## Integration with Existing System

| Existing Component | Integration |
|---|---|
| `ticket-api` | durable coordination, evidence refs, assigned agent IDs (ADR-6) |
| `rmcp` | per-session MCP tool servers/clients (ADR-7) |
| `ticket-mcp` | ticket state/update tool for agent sessions |
| `log-viewer` | streamed container/session logs |
| `ticket-viewer` | Phase 2 UI integration |
| bootstrap T1–T6 | execution lifecycle primitives that AOH builds on |

## Acceptance Criteria

- [x] All 15 interview-driven ADRs finalized
- [x] System boundary diagram approved
- [x] Crate map reviewed — no conflicts with existing crates
- [x] Session lifecycle state machine peer-reviewed
- [x] Integration touch-points verified
- [x] Existing Phase 2 implementation tickets reconciled with AOH planning (`02412b9a` **done**)
- [x] Trust-boundary / operator-authorization design approved (`db784443` **done**)
- [x] Archive / revival schema approved (`ffa5361a` **done**)
- [x] Architecture document saved as final version