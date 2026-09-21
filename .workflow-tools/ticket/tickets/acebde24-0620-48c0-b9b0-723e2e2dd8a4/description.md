## Objective
Repair weak tests and remove superseded cases after new coverage lands.

## Context
Fix `context-stack/context-trace/src/tests/graph.rs:12-35`; delete the empty-data skip at `memory-viewers/log-viewer/e2e/tests/theme-and-readability.spec.ts:300-302`; replace bare `assert!(result.is_ok())`; remove duplicate capture/stop cases and superseded toy-fixture tests.

## Acceptance criteria
- Each listed weak pattern is repaired or removed with semantic replacement.
- Replay coverage supersedes obsolete toy fixture cases.

## Out of scope
- Product behavior changes.