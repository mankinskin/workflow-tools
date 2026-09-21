# Task Tracker Plan — Global Execution Checklist

Status: ACTIVE
Owner: Coordinator
Last updated: 2026-03-21

## Goal

Drive the Task Tracker plan to completion with strict work-in-progress limits.
Do not run too many topics in parallel. Finish current topics before starting new ones.

## WIP Policy (mandatory)

- Global major-topic WIP limit: 2
- Per agent WIP limit: 1 claimed ticket
- Validation queue target: <= 5 tickets waiting
- New major topic can start only if one active topic is marked COMPLETE

If WIP limit is exceeded:
1. stop creating new tickets
2. pause non-critical work
3. push active tickets to validation and closure

## Active Topics (Now)

### Topic A — Phase 1 Core Backend + Search + Agent Exec
Reference:
- 01_phase_minimal_backend/PLAN.md
- PROTOCOL_LAYER.md

Scope:
- real storage wiring for create/get/update/list/delete/scan/search
- ticket exec single-command JSON path
- ticket exec --batch transactional behavior
- explicit index_root + full UUID enforcement
- fields projection support for agent responses

Exit gates:
- [x] command handlers are real (no stubs) — create/get/update/list/delete/scan/search/exec all wired to redb+tantivy+FS
- [ ] crash/reconcile smoke path passes — `ticket scan --reindex` rebuilds index from FS; need integration test
- [ ] search returns stable mixed metadata + text results — Tantivy wired, needs search integration test
- [x] exec path parity with CLI command behavior is verified — `ticket exec` reads TaskCommand JSON from stdin
- [ ] transactional batch rollback boundary is tested — `ticket exec --batch` rolls back on first error

### Topic B — Phase 1.5 Lease + Serve Stdio + Validation Assignment Rules
Reference:
- 015_phase_lease_protocol/PLAN.md
- VALIDATION_RELEASE_GOVERNANCE.md
- HOST_EXECUTOR_AUTH_PROVIDER.md

Scope:
- claim/unclaim/heartbeat/leases
- ticket serve --stdio request/response session
- session-liveness auto-renewal
- validator/worker separation-of-duties check on validating assignments
- host executor auth skeleton (ephemeral worker tokens, assignment-scoped authorization)

Tracked tickets (T1-T6):
- T1 `a8d6c1d2-2b64-4d9a-9f1d-1e2a3b4c5d61` startup/auth bootstrap
- T2 `b1f3e2a4-6c7d-4e8f-9a0b-2c3d4e5f6a72` assignment start branch/cwd checks
- T3 `c2a4b6d8-7e9f-4a1b-8c2d-3e4f5a6b7c83` lifecycle happy path
- T4 `d3b5c7e9-8f1a-4b2c-9d3e-4f5a6b7c8d94` validator handoff + separation-of-duties
- T5 `e4c6d8f1-9a2b-4c3d-8e4f-5a6b7c8d9ea5` early-stop recovery and reassignment
- T6 `f5d7e9a2-ab3c-4d5e-9f5a-6b7c8d9eaf16` merge/completion linkage

Exit gates:
- [ ] claim collision and stale reclaim tests pass
- [ ] serve session can process multi-command workflow with request IDs
- [ ] session drop stops auto-renewal and TTL expiry works
- [ ] validator cannot equal worker for validating assignment
- [ ] early integration tests T1-T6 for host executor lifecycle are green

## Priority Next Topic (Start Soon When a WIP Slot Opens)

### Topic C — Sandboxed Integration Test Program (Context-Tasks)
Reference:
- `../../plans/20260321_PLAN_CONTEXT_TASKS_SANDBOX_INTEGRATION_TESTS.md`

Scope:
- isolated end-to-end integration tests for `context-tasks`
- multi-ticket workflows: create/get/update/list/delete/search/scan
- lease/conflict and validate-release protocol chains
- `ticket exec --batch` rollback/error envelope behavior
- optional hardened lane via `SANDBOX_BACKEND=zeroboot`

Start trigger:
- Topic A or Topic B reaches COMPLETE, or one topic is explicitly paused by coordinator.

Exit gates:
- [ ] baseline sandbox integration lane is green in local + CI
- [ ] at least one end-to-end multi-ticket workflow test is green
- [ ] at least one validate-release command-chain test is green
- [ ] batch rollback behavior is verified by integration tests
- [ ] zeroboot smoke lane decision recorded (adopted or manual-only)

## Queued Topics (Do Not Start Until a Slot Opens)

- Topic D — Phase 2 History + Rollback
- Topic E — Phase 3 Graph + Merge Queue
- Topic F — Phase 4 Dogfooding rollout and gates
- Topic G — Phase 5 Integrations (HTTP/MCP dashboards + messenger)

## Ticket Flow Checklist (Coordinator)

For every ticket moved to in-progress:
- [ ] acceptance criteria is explicit
- [ ] risk_level is set
- [ ] validation_plan is set
- [ ] release_target is set
- [ ] owner/worker/validator assignment is valid

Before moving to validated:
- [ ] worker and validator identities differ
- [ ] evidence_refs include tests and command outputs
- [ ] linked bugs created for discovered defects

Before moving to release-candidate:
- [ ] validation_status = passed
- [ ] no open sev0/sev1 linked bugs
- [ ] dependency blockers are cleared

## Bug and Release Fast-Path

When validation fails:
1. open linked bug ticket immediately
2. set source ticket back to review
3. assign fix owner with high priority if sev0/sev1

For stable release candidate:
- [ ] all included tickets are validated
- [ ] release smoke suite passes
- [ ] rollback path is verified
- [ ] post-release monitoring window is planned

## Daily Execution Rhythm

1. WIP audit (enforce max 2 major topics)
2. close oldest in-progress tickets first
3. drain validation queue
4. promote validated tickets to release-candidate only when gates pass
5. publish short status digest: now, blocked, next

## Completion Rule

A topic is COMPLETE only when:
- all topic exit gates are checked
- no blocking sev0/sev1 bug remains linked to that topic
- coordinator publishes completion note and opens at most one new topic from queue
