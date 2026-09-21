---
tags: `#plan` `#testing` `#integration` `#context-tasks` `#sandbox`
summary: Dedicated plan for fully sandboxed context-tasks integration tests, including end-to-end CLI scenarios and optional zeroboot execution.
status: 📋
---

# Plan: Context-Tasks Sandboxed Integration Tests

## Objective

Create a dedicated, repeatable integration test suite for `context-tasks` that validates realistic workflows (create, search, update, lease, validate/release commands) in isolated sandbox environments.

## Context

### Why This Plan

- Existing contract tests validate schema/model pieces well but do not yet cover full end-to-end ticket workflows with multiple tickets and persistent indexes.
- We need high confidence that command flows behave correctly under realistic state changes and filesystem/index interactions.
- Isolation is required so every test run is deterministic and safe for local and CI execution.

### Current Baseline

- Implemented runtime: `TicketStore` + `ticket` CLI + `ticket exec` command path.
- Current integration tests: contracts for schema/query/filesystem/manifest/edge/history model.
- Gap: no dedicated workflow-level integration suite that drives multiple commands against a fresh sandbox per test.

### Files Affected

- `crates/context-tasks/tests/integration_sandbox/` (new test modules)
- `crates/context-tasks/tests/context_tasks_sandbox_integration.rs` (new test entry)
- `crates/context-tasks/tests/common/` (shared harness utilities)
- `crates/context-tasks/Cargo.toml` (optional dev-deps for harness)
- CI workflow file(s) invoking dedicated integration lane (target path to be confirmed)

### Dependencies

- Rust test runner + `tempfile` for per-test workspace/index roots.
- `std::process::Command` for black-box CLI invocation.
- Optional isolation backend: `zeroboot` profile for stronger process/filesystem sandboxing.

## Analysis

### Isolation Model

Two-layer isolation strategy:

1. Baseline sandbox (always on):
- Each test creates unique temp directories for:
  - `TICKET_INDEX_ROOT`
  - ticket scan roots
  - command input/output fixtures
- No shared mutable state across tests.

2. Hardened sandbox lane (optional):
- Same test scenarios executed inside isolated environment backend (candidate: `zeroboot`).
- Run in CI nightly or gated profile to verify no host coupling.

### Test Pyramid for Context-Tasks

- Contract tests (already present): schema and parsing invariants.
- Workflow integration tests (new): multi-command behavior against real storage and search index.
- Sandboxed environment tests (new lane): same workflows under stricter runtime isolation.

### Major Workflow Scenarios to Cover

1. Multi-ticket CRUD flow:
- Create N tickets with varied type/state/title.
- List + get + update + delete and verify persisted state.

2. Search flow:
- Create several tickets with overlapping text.
- Assert ranked search returns expected IDs/snippets and filtering semantics.

3. Scan/reconcile flow:
- Materialize ticket folders on disk.
- Run `scan --reindex` and verify index/search hydration.

4. Lease flow:
- Claim ticket, verify lease listing and expiry behavior.
- Verify conflict errors on concurrent claim attempts.

5. Validation/release flow (newly implemented commands):
- `task_validate_start` -> `task_validate_result` -> `task_release_candidate_create` -> `task_release_gate_check` -> `task_release_promote`.
- Assert state transitions and protocol errors for invalid transitions/missing evidence.

6. Batch exec flow:
- `ticket exec --batch` with mixed valid/invalid commands.
- Verify rollback/error envelope semantics expected by protocol.

## Execution Steps

- [ ] Step 1: Add shared sandbox harness in `tests/common`:
  - per-test temp roots
  - helper to invoke `ticket` binary with JSON output
  - helper to run `ticket exec` stdin payloads

- [ ] Step 2: Add workflow test module for multi-ticket CRUD/search/update.

- [ ] Step 3: Add workflow test module for scan/reconcile and filesystem ingestion.

- [ ] Step 4: Add workflow test module for lease conflict and expiry scenarios.

- [ ] Step 5: Add workflow test module for validate/release command chain and failure cases.

- [ ] Step 6: Add batch transaction semantics tests for `ticket exec --batch`.

- [ ] Step 7: Add test grouping/profile guidance:
  - fast local lane (baseline sandbox)
  - optional hardened lane (`zeroboot`) behind env/profile flag.

- [ ] Step 8: Wire CI jobs for baseline lane on PR and hardened lane on nightly/manual trigger.

## Validation

- [ ] `cargo test -p context-tasks` passes locally.
- [ ] New sandbox integration target passes standalone (exact command to be finalized once files exist).
- [ ] At least one test proves multi-ticket create/search/update behavior end-to-end via CLI.
- [ ] At least one test proves validate/release command chain end-to-end via `ticket exec`.
- [ ] CI publishes artifacts/logs for failed sandbox tests for debugging.

## Zeroboot Adoption Strategy

### Phase A (immediate)

- Implement all workflow tests with tempdir sandboxing first.
- Ensure deterministic behavior and low flakiness.

### Phase B (optional hardening)

- Add a `zeroboot` runner wrapper for the same test binaries (no test logic duplication).
- Gate this lane with env/profile (e.g., `SANDBOX_BACKEND=zeroboot`).

### Decision Gate

- Keep `zeroboot` lane if it catches host-coupling bugs or provides stronger confidence with acceptable runtime cost.
- Otherwise retain as manual security/regression profile and keep baseline lane mandatory.

## Risks

- Flaky tests due to filesystem watcher timing or search indexing races.
- Over-coupling tests to output formatting rather than protocol fields.
- Higher CI runtime from hardened sandbox lane.
- Unknown integration cost for `zeroboot` in current CI environment.

## Risk Mitigations

- Prefer assertions on JSON fields over text rendering.
- Add bounded retries only where eventual consistency is expected.
- Separate fast baseline and slower hardened lanes.
- Start `zeroboot` with one smoke scenario before scaling to full matrix.
