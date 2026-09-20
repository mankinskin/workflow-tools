## Objective
Provide end-to-end evidence that deleting one worktree cannot affect another checkout's source, artifacts, stores, CLI, or MCP operations.

## Required behavior
The harness creates two isolated checkouts/worktrees, performs independent ticket/session/board/feedback/spec/test/asset operations, deletes one checkout, then proves the surviving checkout can:
- perform direct CLI ticket reads and writes from its own CWD;
- resolve MCP calls only through its own session assignment;
- show a non-phantom board snapshot;
- use its local feedback, spec, and test stores;
- retain its own prepared viewer assets;
- operate without any persisted sibling-worktree path.

## Validation
The harness must seed representative legacy path-bearing records so cleanup/migration behavior is exercised, then assert all cross-worktree references are gone or inert.

## Dependency
Run after tickets `8130027d`, `968e863b`, `f7a0f5b5`, `fde76de2`, and `461ddbb1` land. This ticket is the final acceptance gate for parent `3a624bf6`.