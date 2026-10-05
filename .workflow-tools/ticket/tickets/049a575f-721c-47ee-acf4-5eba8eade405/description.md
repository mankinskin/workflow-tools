## Objective
Adapt Log API's supported store operations to the kernel-owned `DomainStore` contract and correct the existing Test CLI Log-root derivation.

## Scope
Preserve Log record types and supported append/read behavior. Route Log selection through the shared contract. Update the Test CLI capture consumer so it writes to `<local_workspace>/.workflow-tools/log`, not the current derived `.workflow-tools/.log` sibling. Do not create unrelated Log transports.

## Acceptance criteria
1. LogStoreConfig uses the core trait and only the typed capabilities Log API supports.
2. Test CLI derives the canonical Log store from the selected local workspace independently of the Test store path.
3. Legacy-only reads, canonical-only writes, duplicate-layout diagnostics, explicit-dot normalization, and read-only no-auto-init match the workspace Spec.
4. A producer-shaped Test CLI fixture writes a Log record to `.workflow-tools/log`; read-back confirms the same record and no sibling/parent store was modified.

## Validation
Run G2 `cargo test --manifest-path workflow-tools/log/crates/log-api/Cargo.toml` and focused Log/Test CLI coverage, including producer-shaped read-back.