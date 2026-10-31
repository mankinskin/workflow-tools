# W1 review-fix evidence (CHANGES REQUIRED round 1)

Fixes (regression tests written first; red = compile/assertion failures on missing `mission_api::lifecycle` public path, `Result` from `project_mission`, `validate_waypoint_targets`):
1. project_mission now returns Result and rejects a completion event unless every required waypoint is completed (pending/active/paused/cancelled/empty rejected; valid completed accepted). Facade surfaces it as control_history_invalid.
2. Cancelled or blocked first unsatisfied required entry makes the Mission blocked and ineligible; the Ticket stays cancelled.
3. LifecycleJournal::validate_waypoint_targets; facade load_mission rejects ghost, Ticket-backed and child-Mission targets (unsupported version/corrupt still explicit).
4. `pub mod lifecycle` in mission-api and mission (re-exports retained; `mission::lifecycle::*` includes API items + WorkspaceLifecycle). External-path tests added.

Tests: mission-api lifecycle_contract 15 passed; mission lifecycle_contract 12 passed.
G1 rerun: execution w1-g1-review-fix-20261030-mission-native-w1-shared-lifecycle-g1, log capture ...-log; passed exit 0; 31 suites, 466 passed, 0 failed, 2 ignored. Earlier execution w1-g1-20261030-... retained.
Criteria: MN-W1-01 (1,2), MN-W1-02 (2), MN-W1-03 (1), MN-W1-04 (3,4).
