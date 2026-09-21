Repro

`spec_get` fails to deserialize specs whose `code_refs` contain non-canonical `kind` values. The canonical `SymbolKind` vocabulary is `struct`, `function`, `trait`, `impl`, `enum`, `module`, `const`, `type` (see `#[serde(rename_all = "snake_case")]` in `memory-api/crates/spec-api/src/code_ref.rs#L8`), but authors (including this session's own spec-authoring flow) commonly write shorthand or descriptive values that aren't in that list: `fn` (shorthand for `function`), `block` (an arbitrary code region, not tied to a single symbol), `field` (a struct/enum field), and `enum_variant` (a single enum variant). None of these round-trip through `spec_get` today; the reader hard-errors on the first non-canonical value it hits instead of accepting any alias.

Verified source locations

- [memory-api/crates/spec-api/src/code_ref.rs](memory-api/crates/spec-api/src/code_ref.rs#L8) defines `SymbolKind` via `#[serde(rename_all = "snake_case")]`, with no `#[serde(alias = ...)]` entries.
- [memory-api/crates/spec-api/src/store/helpers.rs](memory-api/crates/spec-api/src/store/helpers.rs#L16) serializes `code_refs` through `serde_json::to_value(&spec.code_refs)` when writing specs.
- [memory-api/crates/spec-api/src/store.rs](memory-api/crates/spec-api/src/store.rs#L383) persists manifests through `spec_to_entity(manifest)`.
- [memory-api/crates/spec-api/src/manifest.rs](memory-api/crates/spec-api/src/manifest.rs#L128) exposes `code_refs` as part of `SpecManifest`.

Observed affected specs

- [1f77f652](.spec/specs/1f77f652-f883-4782-940a-39874dfe1382/spec.toml) and [351389c0](memory-api/.spec/specs/351389c0-0873-4c3c-bc46-3551459ba1cd/spec.toml), both authored in the same session that filed this ticket, used `kind = "fn"`, `"block"`, `"field"`, and `"enum_variant"` values. These are not legacy/back-compat data from an older schema version — they were produced live by this session's spec-authoring flow — so this is a live write/read (and write/vocabulary) mismatch, not a legacy-data compatibility problem. The `kind = "fn"` occurrences in both specs were already remapped to `kind = "function"` as a stopgap so both specs currently parse via the fixed reader; `block`, `field`, and `enum_variant` values remain unchanged in the two spec.toml files and are now covered by the widened vocabulary below.

Fix implemented

Updated the spec-api `SymbolKind` deserializer in [memory-api/crates/spec-api/src/code_ref.rs](memory-api/crates/spec-api/src/code_ref.rs#L8):
- `#[serde(alias = "fn")]` added to the `Function` variant — `fn` deserializes as `Function`, and `Function` still serializes to the canonical `"function"` (read-only alias, no change to write form).
- Three new real variants added: `Field`, `EnumVariant`, `Block` (snake_case: `field`, `enum_variant`, `block`) — genuinely distinct reference kinds, not aliases.
- No exhaustive `match` on `SymbolKind` existed anywhere in the crate, so no other call sites needed updates.
- 4 new regression tests added covering the alias, the three new variants (deserialize and serialize round-trip), and confirming existing canonical values are unaffected. `cd memory-api && cargo test -p spec-api` passes 72/72.

Verification status (as accepted by the ticket owner)

- **CLI-confirmed**: a freshly built `target/debug/spec.exe get 1f77f652 --workspace .` and `spec.exe get 351389c0 --workspace .` both return `"status": "ok"` and correctly parse all kind values (`enum, function, block, function, field, block, field, block` for 1f77f652; `struct, impl, function, enum_variant, enum_variant, field, block, function, field, block` for 351389c0).
- **Unit-test-confirmed**: the 4 new regression tests in spec-api's `code_ref` module pass.
- **MCP-tool NOT confirmed**: the live `spec-mcp` MCP server process used by this session continued returning the pre-fix deserialization error (`unknown variant "block"`/`"enum_variant"`) even after a requested restart/reload, indicating its running binary is not reading the rebuilt `target/debug/spec.exe` code path (likely a separately provisioned/cached binary). The ticket owner explicitly accepted CLI + unit-test evidence as sufficient to close this ticket without a passing live-MCP-tool check. If the MCP server's stale binary becomes a recurring problem, it should be filed as its own separate infrastructure ticket (server binary provisioning), not reopened against this one.

Acceptance criteria

- `spec_get` successfully reads specs containing `kind` values `fn`, `block`, `field`, and `enum_variant` without a TOML/JSON deserialization error, in both the top-level `.spec` store and `memory-api/.spec`. — **met via CLI**, MCP-tool check waived per owner decision above.
- `fn` deserializes as equivalent to `function` (no separate variant needed). — met.
- `block`, `field`, and `enum_variant` are real, distinct `SymbolKind` values usable when writing new code_refs (not just accepted on read). — met.
- A regression test in spec-api's code_ref module covers deserializing all four non-canonical/new values and confirms the existing canonical values are unaffected. — met.
- `spec_refs_validate` and the CLI/MCP `spec_get`/`spec_update` paths for both `.spec` stores continue to work against the two affected specs (1f77f652, 351389c0) after the fix. — met via CLI for both stores; MCP-tool check waived per owner decision above.
