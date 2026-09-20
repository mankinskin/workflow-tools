# Problem

Playwright ownership and configuration are fragmented. Ticket-viewer forks shared suites and viewer definitions, GPU launch profiles differ, and `cargo make test-e2e` omits ticket-viewer while retaining stale Windows paths.

# Scope

- Make `viewer-api/viewer-api/frontend/dioxus/e2e/shared` the canonical source for viewer registration, readiness, console/network diagnostics, tracing helpers, screenshots, and seeded-server fixtures where sharing is practical.
- Replace ticket-viewer copies of shared viewer, smoke, theme, and tracing helpers with imports or thin configuration-only adapters.
- Define one release-viewer registry covering ticket-viewer, spec-viewer, log-viewer, and doc-viewer.
- Repair cross-platform orchestration commands and paths.
- Provide focused per-viewer commands plus one deterministic all-viewer command.
- Standardize retry, timeout, output directory, trace, screenshot, and failure diagnostics policy while allowing explicit GPU-lane overrides.

# Acceptance criteria

- [ ] Ticket-viewer and spec-viewer consume the same canonical smoke, theme, tracing, and correlation contracts where behavior is shared.
- [ ] No copied shared suite can silently assert a weaker contract than its sibling.
- [ ] One command runs release-browser validation for all four managed viewers on Windows and Unix shells.
- [ ] The all-viewer command includes ticket-viewer and contains no obsolete `tools/viewer/doc-viewer` or `tools/viewer/log-viewer` paths.
- [ ] A failure reports viewer name, target URL, browser environment, console/page errors, failed requests, missing assets, and artifact paths.
- [ ] Existing viewer-specific feature suites remain owned by their viewers.
- [ ] The nearest shared and viewer release suites pass.

# Implementation steps

1. Inventory duplicate shared helpers and configuration deltas.
2. Introduce a canonical viewer registry and reusable fixtures.
3. Migrate ticket/spec registrations without moving feature-specific tests.
4. Repair Makefile/package entrypoints and document commands.
5. Run all viewers sequentially and record test-api evidence.

# Non-goals

- Forcing identical GPU flags across distinct performance lanes.
- Moving every viewer-specific test into viewer-api.