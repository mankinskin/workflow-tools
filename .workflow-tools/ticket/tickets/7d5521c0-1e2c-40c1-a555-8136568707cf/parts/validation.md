# Validation results

All W2 acceptance criteria passed.

- `cargo test -p mission --features cli --test mission_cli` — passed, 3 tests. ValidationSpec `vt-mission-cli-help-integration`; execution `exec-mission-cli-help-integration-20261016`.
- `cargo run -p mission --features cli --bin mission -- --help` — usage printed, exit 0. ValidationSpec `vt-mission-cli-help-top-level`; execution `exec-mission-cli-help-top-level-20261016`.
- `cargo run -p mission --features cli --bin mission -- validate-preview --help` — usage printed, exit 0. ValidationSpec `vt-mission-cli-help-validate-preview`; execution `exec-mission-cli-help-validate-preview-20261016`.
- `cargo fmt --check -p mission` — passed.
- `git diff --check` — passed.

ValidationSpecs and passed executions were read back from the canonical root `.workflow-tools/test` store. Each links this ticket. Direct Mission spec IDs are omitted because the test CLI resolved a different `.spec` directory; no shadow store was created.