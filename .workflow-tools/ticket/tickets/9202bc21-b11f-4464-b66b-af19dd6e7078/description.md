# Problem

The server persists `x-session-id`, but the WASM `NetworkLayer` currently constructs its own request headers and does not visibly use the shared session helper. Ticket-viewer also carries an older tracing suite that does not enforce correlation. JSONL collection without guaranteed per-test correlation is not useful under concurrent or repeated browser runs.

# Scope

- Build on `b480632a`, `8f349d96`, and completed correlation ticket `363e26d6`.
- Ensure every frontend tracing batch includes the stable Playwright-injected session ID.
- Preserve that identifier in frontend JSONL, backend spans/logs, Playwright attachments, and test-api executions.
- Add a shared Playwright fixture that enables the sink, flushes it deterministically, queries/filter records for the current test, and attaches the filtered artifact.
- Fail tests on correlated frontend `error` records and uncaught exceptions unless explicitly allowlisted.
- Define bounded buffering, flush-on-page-close/test-end behavior, dropped-record counters, redaction, and sink-failure diagnostics.
- Verify log-viewer/log-MCP can query records by session ID.

# Acceptance criteria

- [ ] Network sink requests carry the exact `viewer-api-session-id` injected before navigation.
- [ ] Server-side persisted records include the same session ID without overwriting an existing record value.
- [ ] Ticket-viewer and spec-viewer run one canonical tracing/correlation suite.
- [ ] Each Playwright test report can attach only its own frontend records plus pointers to matching backend logs.
- [ ] A deliberate frontend tracing error causes the owning test to fail with the correlated record shown.
- [ ] Disabled sink mode sends no client-log traffic.
- [ ] Buffer overflow, failed POST, and final-flush behavior are test-covered and do not block the render loop.
- [ ] Sensitive fields have a documented redaction contract.

# Implementation steps

1. Add a failing correlation test around the raw WASM network sink.
2. Route sink headers through the shared session utility.
3. Add deterministic flush and dropped-record telemetry.
4. Build the Playwright correlation/artifact fixture.
5. Verify persisted logs through log-query tooling and record evidence.