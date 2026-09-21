Phase A extraction: create standalone `memory-kernel` from `memory-api/crates/memory-api`, freeing the legacy package name and supplying the neutral shared substrate for workflow domains.

## Delivered

- Published `mankinskin/memory-kernel` on `main` at commit `4c0c7a3` (`feat(kernel): extract standalone memory kernel`).
- Preserved source history using `git subtree split --prefix=crates/memory-api`.
- Renamed the package and public library crate to `memory-kernel` / `memory_kernel`.
- Removed the legacy `test-api` path dependency by moving the neutral `InteroperableArtifact` contract into the kernel; move journals remain validated against that contract.
- Added standalone workspace metadata, README versioning/migration contract, `.gitignore`, and GitHub Actions CI.
- Updated public doctests and package-facing identifiers to the new crate name. Remaining old names are intentional legacy migration documentation or path-normalization fixtures.

## Evidence

- Spec: [66538d9e Memory kernel standalone extraction](.spec/specs/66538d9e-c8ff-4dd8-b3df-a12dc9984a0e/body.md)
- Validation spec: `.test/default/specs/memory-kernel-standalone-extraction.json`
- Passing execution: `.test/default/executions/memory-kernel-standalone-extraction-20260725.json`
- `cargo test --all-features`: 150 passed, 1 ignored.
- `cargo fmt --check`: passed.
- `cargo clippy --all-targets --all-features`: passed with inherited non-fatal warnings.
- A fresh external path-dependent consumer compiled using `memory_kernel::URN_SCHEME` with no `memory-api` dependency.

## Follow-up

`cargo clippy -- -D warnings` reports 24 inherited findings across untouched legacy source. CI runs the non-fatal clippy baseline while that debt is addressed separately.

Phase 0 provisioning is complete, but the provisioning ticket cannot transition from `new` because ticket MCP returns `store error: no schema for type 'task'`. This administrative defect also prevents normal dependency-state progression; it does not invalidate the verified repository extraction.