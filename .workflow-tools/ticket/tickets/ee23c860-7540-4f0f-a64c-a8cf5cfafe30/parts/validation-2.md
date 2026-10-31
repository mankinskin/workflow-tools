# W1 implementation and validation evidence (MN-W1-01..04)

Implemented on main checkout (uncommitted, parent owns review/commit). Paths:
- mission/crates/mission-api/src/lifecycle.rs (new), lib.rs (mod + re-export)
- mission/crates/mission/src/lifecycle.rs (new, WorkspaceLifecycle adapter), lib.rs (mod + re-export)
- mission/crates/mission-api/tests/lifecycle_contract.rs, mission/crates/mission/tests/lifecycle_contract.rs (new)
- mission/README.md (docs)

Coverage: MN-W1-01 mappings/unmapped/eligibility/Mission projection rows; MN-W1-02 Ticket store read + Ticket-domain single-hop transition, foreign-authority absence, rejected transition leaves Ticket unchanged; MN-W1-03 separate capability traits with explicit unsupported_capability, pause/cancel/resume/paused-completion edges; MN-W1-04 version-1 journal schema, corrupt/unsupported/conflicting histories, missing journal = no controls, plan revision unaffected.

G1 (spec mission-native-w1-shared-lifecycle-g1, execution w1-g1-20261030-mission-native-w1-shared-lifecycle-g1, capture w1-g1-20261030-mission-native-w1-shared-lifecycle-g1-log): outcome passed, exit 0; 31 suites, 457 passed, 0 failed, 2 ignored (pre-existing). New: 11 + 7 lifecycle_contract tests.
Not implemented (W5): durable control writes/recovery, pause/resume/cancel/complete/reopen operations (declared capability traits return unsupported_capability).
