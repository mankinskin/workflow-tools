## W2 correction review

Corrected premature completion and repaired five coupled typed-persistence contracts in `mission/crates/mission-api`.

- Source, mission, Parts directory, and Part file reads reject symlinks.
- Draft saves remove a prior Markdown projection; publishing recreates it.
- Draft validation rejects unknown, self, and duplicate dependencies; publishing still derives serial predecessors.
- Post-install backup cleanup is recoverable and explicitly diagnosed rather than returning an error after acceptance.
- Per-Mission create-directory reservation serializes CAS writers; staging directories are UUID-unique.

Validation: `cargo test -p mission-api --test typed_source` (3 passed); `cargo fmt -p mission-api`; `git diff --check`. Commit: `5b02127479ed0eaa32d4ddc56ffaa82b6fdfdeb4`.