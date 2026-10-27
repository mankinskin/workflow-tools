## Test CLI verification failure: confirmed root cause

The `verified -> verified` error during automatic W3 spec verification came from test-cli store resolution, not W3 recovery behavior. The old `resolve_spec_root()` appended `.spec` to the parent of the canonical test store, selecting `workflow-tools/.workflow-tools/.spec` instead of the canonical `workflow-tools/.workflow-tools/spec` store. The wrongly selected index had stale state `verified`; the canonical W3 manifest was `implemented` at revision 7.

Bug ticket `a7f9f4a0-ade7-4023-b45d-c6431a45de03` tracks canonical resolution, conflicting-store regression coverage, and idempotent spec verification. It is related to, but distinct from, ticket `50092f61-f4fb-49a1-ac67-64c8cb7c513e`, which concerns another ticket-MCP root-resolution divergence.

No W3 spec state was manually changed. W3 remained `in-implementation` pending its required review.

## W3 verification follow-up

- Test CLI now resolves the spec root through the shared canonical workspace/store helpers for both explicit and ambient workspace paths. It no longer derives or selects a legacy sibling path. Existing on-disk legacy data was not deleted or migrated.
- Regression coverage creates the same spec ID as `implemented` in the canonical store and `verified` in the wrongly-derived store, then proves the CLI updates the canonical copy. The path tests cover explicit and ambient selection.
- `cargo test --manifest-path workflow-tools/Cargo.toml --locked -p spec-api`: 99 passed.
- `cargo test --manifest-path workflow-tools/Cargo.toml --locked -p mission-api`: 41 passed.
- `cargo test --manifest-path workflow-tools/Cargo.toml --locked -p mission --all-features -- --test-threads=1`: 50 passed.
- W3 package evidence is recorded as `ae41c0ca-75fa-497e-b4ac-90e3fa04f6ec-mission-w3-canonical-root-fix-01`; linked guard evidence is `ae41c0ca-75fa-497e-b4ac-90e3fa04f6ec-mission-w3-canonical-root-fix-linked`. Canonical spec 44f100c4-215e-445a-b452-1a8a969d96cc now reads `verified` through the supported guard recomputation path.
- The earlier full test-cli result was 16 passed, 1 failed due to `run_flags_over_budget_against_spec_threshold` assuming `echo slow` exceeds a 0 ms threshold. The latest full-suite result is 17 passed, 1 failed for the same timing-sensitive test; it was not changed.

W3 remains `in-implementation` pending review. No live Mission repair or migration was run; no commit was created.

## Corrected W3 verification and package run (2026-10-26)

- `run_harness` now obtains actual spec UUIDs from the registered validation spec. The new W3 run `ae41c0ca-75fa-497e-b4ac-90e3fa04f6ec-mission-w3-canonical-root-fix-02` passed both package gates and reports spec `44f100c4-215e-445a-b452-1a8a969d96cc` as verified.
- Test execution read-back succeeded from the canonical test store; the spec CLI independently reports state `verified`. The spec was verified by guard recomputation, not manually changed.
- W3 remains `in-implementation` pending its distinct review. No Mission repair, move, migration, or commit was performed.
