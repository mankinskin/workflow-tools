## W2 final validation

- RED before fix: `cargo test -p mission-api --test typed_source typed_source_rejects_a_symlinked_missions_ancestor` failed with `a symlinked managed ancestor must be rejected rather than read`.
- GREEN after root guard: same focused test: 1 passed.
- `cargo test -p mission-api --test typed_source`: 6 passed.
- `cargo test --manifest-path mission/crates/mission-api/Cargo.toml typed_source::tests::accepted_write_survives_recoverable_backup_cleanup_failure`: 1 passed, 35 filtered.
- `rustfmt --check mission/crates/mission-api/src/typed_source.rs` and `git diff --check`: passed before commit.

Native mission readback was attempted with installed `mission --workspace /home/linus/git/meta-workspace/workflow-tools get bd23f826-8f9f-4bae-8b75-0288ba1b4899`; it is blocked by installed CLI/schema incompatibility: persisted JSON has unknown `operator_results`. The installed CLI exposes no `publish-mission` command (only bundle `publish`), so native current/revision publish/check-generated evidence cannot be produced by this binary.