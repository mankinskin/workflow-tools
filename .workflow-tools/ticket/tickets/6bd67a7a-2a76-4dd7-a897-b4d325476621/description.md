Implement recursive multi-store workspace discovery and cross-store reference integration with URN-based identities across local and nested workspaces.

Decisions locked:
- default discovery is fully automatic recursive discovery
- canonical reference identity is URN `ce://<workspace>/<store>/<entity>`

Implementation plan:
1. Define workspace/store discovery algorithm and ownership metadata model.
2. Introduce URN parsing/formatting and cross-store reference resolution APIs.
3. Support late-added stores without destructive rebuild requirements.
4. Ensure scan/index/report outputs surface per-store integration state and failures.

Practical scenarios to verify:
- empty workspace bootstraps with one store and later adds others
- parent workspace references entities in nested workspace stores
- missing target store at reference time later resolves after discovery/index refresh

Validation evidence:
- integration tests for recursive discovery in nested workspace fixture trees
- reference resolution tests across store type boundaries and absent-then-present stores
- scan report checks for per-store discovered/integrated/diagnostic counters

Acceptance criteria:
- examples above are codified as repeatable tests
- reference resolution is deterministic and documented
- scan/index outputs report integration status per discovered store root and store type
