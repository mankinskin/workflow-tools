`memory_kernel::workspace::resolve_store_root_from_with_diagnostics` (workflow-tools/memory-kernel/src/workspace.rs) has a fallback bug that silently misplaces entity-store artifacts.

## Repro

When resolving a store root for a directory whose ancestor chain (all the way to the filesystem root) contains **no** existing `.ticket`/`.spec`/`.test`/`.workflow-tools/<domain>` marker, the walk-up loop's final `None => return StoreRootResolution { store_root: dir.to_path_buf(), .. }` branch returns the **original workspace directory itself**, unmodified — instead of `dir.join(dir_name)` (the legacy layout) or `dir.join(CANONICAL_STORES_DIR).join(domain)` (the canonical layout) that every other branch of this function produces.

Concretely this means: `TicketStore::init(&some_fresh_dir)` (or the equivalent `.test`/`.spec` open/init path) on a directory with no pre-existing store anywhere above it does **not** create `<some_fresh_dir>/.ticket/...` — it treats `some_fresh_dir` itself as the raw index root, so entity files land directly under the workspace directory (e.g. `tickets/<id>/ticket.toml` right next to other workspace files) rather than under a `.ticket`/`.workflow-tools/ticket` subdirectory.

## Real-world impact observed

This is very likely how `meta-workspace/default/{specs,executions}/*.json` came to exist directly at the meta-workspace repo root (reported by the user as "the bench execution evidence is incorrectly being placed in a 'meta-workspace/default' folder") — a `test_record_spec`/`test_record_execution` MCP call with `workspace: "default"` (or similar) resolved against a directory with no existing `.test` marker in its ancestor chain, and the fallback returned the bare directory, causing `TestStoreConfig { root: <bare dir>, workspace_slug: "default" }` to write to `<bare dir>/default/{specs,executions}/*.json` instead of `<bare dir>/.workflow-tools/test/default/{specs,executions}/*.json`.

I independently reproduced the same fallback while building a ticket-domain move-benchmark fixture in an isolated `tempfile::tempdir()` (no ancestor `.ticket` marker anywhere under the OS temp directory): `TicketStore::init(&source_root).index_root` resolved to `source_root` itself with no `.ticket` suffix, not `source_root/.ticket`. Workaround used there: create tickets directly at the bare resolved root (matches the buggy behavior) — not a real fix.

## Expected fix

The no-marker-found fallback in `resolve_store_root_from_with_diagnostics` should return `dir.join(CANONICAL_STORES_DIR).join(store_domain(dir_name))` (matching `canonical_store_root`), consistent with every other resolution branch, instead of returning `dir` unchanged.

## Scope

- File: `workflow-tools/memory-kernel/src/workspace.rs`, function `resolve_store_root_from_with_diagnostics`.
- Affects every domain that calls through this resolver: ticket, spec, session, test, rule, audit — anywhere a store is initialized/opened in a directory tree with no pre-existing marker.
- Add a regression test: resolving a store root in a directory with zero ancestor markers must append the canonical `dir_name`/`CANONICAL_STORES_DIR` suffix, never return the bare input directory.