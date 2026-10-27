## W3 validation update

- The former full-suite failure `typed_drafts_have_cli_mcp_parity_for_identity_gates_and_serial_sources` now passes. The API validation incorrectly treated empty rendered Part Markdown as missing required content, despite `PartSource` being distinct from prompt and the typed-lifecycle contract not requiring nonempty Part content.
- The implementation now permits empty Part Markdown while retaining checks for waypoint title, session package, prompt, validation command, and commit checkpoint. Added an API regression; the CLI/MCP lifecycle test also verifies incomplete save, publish rejection before gates are complete, and successful publication after completing the required gates.
- Updated W3 guard passed: partial-batch recovery 1/1, unchanged failed-record rollback 1/1, empty-Part validation 1/1, repair transport contracts 8/8, and typed draft CLI/MCP parity 1/1. Recorded as `ae41c0ca-75fa-497e-b4ac-90e3fa04f6ec-mission-recovery-repair-focused-04`.
- Full `mission-api` suite passed: 40 tests with `--test-threads=1`.
- `cargo check -p mission --features cli,mcp --locked`: passed (0 errors, 2 dead-code warnings); `rustfmt --check --edition 2024` on touched Mission Rust files: passed.

## W3 disposition update: `publish_failure_preserves_previous_file_set`

- Superseded earlier note: the failure is not caused by W3. The typed-authority writer (`typed_source.rs`) already stages under `.{id}.typed.tmp`; the test still pre-created the obsolete `.{id}.publish.tmp`, so no recovery conflict occurred, publication succeeded, and `diagnostics[0]` indexed an empty vector. W3 only added `repair: None` to snapshot construction.
- Test-only correction: `mission/crates/mission/tests/roadmap_commands.rs` now pre-creates `.{id}.typed.tmp`. Assertions are unchanged (publication-recovery-required, previous file set and accepted revision preserved). No product code changed.
- `cargo test -p mission --test roadmap_commands`: 11 passed.
- Full package gate via `test run`: `ae41c0ca-75fa-497e-b4ac-90e3fa04f6ec-mission-w3-package-gate-01` (spec `mission-w3-package-gate`, `cargo test -p mission --features cli,mcp -- --test-threads=1`): passed.
- Registered guard re-run via `test run`: `ae41c0ca-75fa-497e-b4ac-90e3fa04f6ec-mission-recovery-repair-focused-05`: passed.
- Evidence-state caveat at the time: `test run` auto-verification resolved the validation-spec id as a spec id and reported `spec not found` for both runs, so no spec transition occurred. W3 spec 44f100c4 was `implemented` then; it was not hand-set to `verified`.

## W3 recovery review-gap closure

- Journal apply, resume, and rollback now serialize on a stable per-journal cross-process file lock. The OS releases the advisory lock on process exit; an existing unlocked lock file does not prevent restart recovery.
- CLI preview and apply now parse the complete strict `RepairRequest` envelope from `--records`; the envelope `source_root` must match `--workspace`. The Mission README examples now document this contract.
- The recovery fixture uses the exact twelve native waypoint IDs from `HANDOFF-MAPPING.md`, preserves the three original completed results, and records the new completion evidence on `mission-repair`.
- A default-parallel API test run exposed a shared test-only backup-cleanup failure flag. It is now thread-local; the follow-up parallel run passes.
- Validation spec: `mission-w3-recovery-review-gaps`. Final passing execution: `ae41c0ca-75fa-497e-b4ac-90e3fa04f6ec-mission-w3-recovery-review-gaps-03`. API test-injector failure/recovery executions: `ae41c0ca-75fa-497e-b4ac-90e3fa04f6ec-mission-api-parallel-injector-red-01` and `ae41c0ca-75fa-497e-b4ac-90e3fa04f6ec-mission-api-parallel-injector-green-01`.
- Final gates: `cargo test --manifest-path workflow-tools/Cargo.toml --locked -p mission-api` passed 41 tests; `cargo test --manifest-path workflow-tools/Cargo.toml --locked -p mission --all-features -- --test-threads=1` passed 50 tests. The focused recovery transport target passed 8 tests.
- The test CLI reported a spec auto-verification transition error (`verified` to `verified`), while canonical `spec get` still reported W3 spec `implemented`. No spec state was changed manually.

W3 remains `in-implementation` pending the required W3 review. No live Mission repair, move, or migration was run.

## Spec-verification repair follow-up (2026-10-26)

- `test-cli` now resolves linked spec IDs from the registered validation spec, instead of trying to load the validation-spec ID as a spec UUID. The corrected end-to-end run `ae41c0ca-75fa-497e-b4ac-90e3fa04f6ec-mission-w3-canonical-root-fix-02` passed `mission-api` (41 tests) and `mission` (50 tests), and its response reported linked spec `44f100c4-215e-445a-b452-1a8a969d96cc` as verified.
- Read-back through `spec --workspace ./workflow-tools get 44f100c4-215e-445a-b452-1a8a969d96cc` reports canonical state `verified`. The earlier caveats above describe the pre-fix run; no manual state transition was used.
- The latest full test-cli suite result is 17 passed, 1 failed; the sole failure remains the unrelated timing-sensitive `run_flags_over_budget_against_spec_threshold` test. The focused canonical-resolution and linked-spec regressions passed (5 tests total).

W3 remains `in-implementation` pending review. No live Mission repair or migration was run; no commit was created.
