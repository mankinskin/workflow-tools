## Problem

`test-mcp`'s `list_executions_aggregates_nested_descendant_store` test (workflow-tools/test/crates/test-mcp/src/server.rs) fails:

    assertion failed: nested_workspace.join(".test").is_dir()

Pre-existing, unrelated to the workspace-selector "." changes made in ticket db3ebe07 - confirmed via `git diff` that no touched file is anywhere near this test or the resolution function it depends on.

## Root cause

`memory_kernel::workspace::resolve_store_root_at_workspace` (workflow-tools/memory-kernel/src/workspace.rs) prefers the canonical `.workflow-tools/<domain>` layout over the legacy `<domain-dir>` layout whenever **neither** exists yet on disk:

    store_root: if canonical_exists {
        canonical_path
    } else if legacy_exists {
        legacy_path
    } else {
        canonical_path  // <-- always canonical when creating fresh, even if caller explicitly named the legacy path
    },

The failing test explicitly constructs `workspace: nested_workspace.join(".test").display().to_string()` - a path that lexically IS the legacy store root (`is_store_root` matches on file_name alone) - expecting the legacy `.test` directory to be created there. Instead, because neither layout exists yet under `nested_workspace`, resolution silently redirects to `nested_workspace/.workflow-tools/test`, ignoring the caller's explicit legacy-shaped path.

This is a distinct nuance from ticket 31e70dab (which covers the "no marker found anywhere in the ancestor chain" fallback returning the bare directory). This one is: "caller named an exact legacy store path that doesn't exist yet" being silently upgraded to canonical instead of honored.

## Scope

- File: workflow-tools/memory-kernel/src/workspace.rs, function resolve_store_root_at_workspace.
- Decide: should an explicit, lexically-legacy-shaped path (`.../.test`, `.../.ticket`, etc.) that doesn't exist yet be honored as-is (create legacy layout), or should ALL fresh-store creation always prefer canonical regardless of what the caller named? Either answer needs the test updated to match, or the resolver fixed to honor an explicit legacy path.
- Regression test: workflow-tools/test/crates/test-mcp/src/server.rs::list_executions_aggregates_nested_descendant_store currently encodes the "honor the explicit legacy path" expectation; keep or update it to match whatever the design decision above lands on.

## Relationship

Complementary to 31e70dab and db3ebe07 (both memory-kernel workspace-resolution/naming issues, same file).
