# Goal

Map major operations across memory-domain crates to required spans, events, result summaries, and journal requirements before broad instrumentation begins.

## Scope

- Cover memory-api, ticket-api, spec-api, doc-api, rule-api, audit-api, session-api, test-api, and log-api.
- Identify store discovery, scan/index reconciliation, CRUD/query flows, graph/dependency traversal, board updates, move flows, validation evidence, and log/journal indexing.
- Classify each operation as log-only, replayable journal, rollbackable journal, or manual-recovery journal.
- Define stable targets and snake_case field names.

## Acceptance criteria

- Every domain crate has an instrumentation table.
- High-volume operations have filter/sampling guidance.
- Implementation tickets can instrument crate-by-crate without inventing field names.