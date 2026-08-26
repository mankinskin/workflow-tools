`transparent_reload_end_to_end_subprocess` (in [workflow-tools/session/crates/mcp-toolmon/tests/integration_reload_end_to_end.rs](workflow-tools/session/crates/mcp-toolmon/tests/integration_reload_end_to_end.rs)) is flaky, independent of any recent code change.

Evidence: cloned the repo at commit `1f858df` (the commit immediately before an unrelated `proxy.rs` refactor) into a scratch location and ran the test in isolation 4 times: 2 passed, 2 failed with the identical panic:

```
thread 'transparent_reload_end_to_end_subprocess' panicked at .../integration_reload_end_to_end.rs:394:5:
a post-swap generation call must eventually be served by v2 within the overall bound
```

The same failure signature reproduces on current `main` too, confirming this is a pre-existing timing flake, not a regression.

Root cause hypothesis: this is the only test in the swap/reload family that drives a *real* external `mcp-toolmon.exe` subprocess plus a real spawned `canonical.exe` child over actual OS process creation and disk polling (per its own doc comment: "no in-process shortcut is available once the proxy is an external process"). Sibling tests (`tests/watcher.rs`, `tests/supervisor.rs`) exercise the same swap/reload logic in-process via direct API calls and are consistently green. The real-subprocess path is far more sensitive to OS scheduling/antivirus/disk-flush jitter, and its bounded waits (`Duration::from_secs(15)` overall deadline for the post-swap retry loop, individual `wait_or_none` at `Duration::from_secs(2)`) are apparently too tight for this environment's occasional slow respawns.

A second, related bug found during investigation: when this test is interrupted (e.g. a terminal `kill`/Ctrl+C mid-run) or panics, the spawned `mcp-toolmon.exe`/`canonical.exe` child processes are not reliably cleaned up (no `Drop`-based child-process teardown on the panic path). On Windows, orphaned instances then hold file locks on `target/debug/mcp-toolmon.exe`, causing *subsequent, unrelated* `cargo build`/`cargo test` invocations to fail with `os error 5` (access denied) until the orphans are manually killed.

Scope:
1. Investigate why the real-subprocess swap/respawn occasionally exceeds the current bounded-wait budgets, and either loosen the timing bounds appropriately or make the poll/respawn path itself faster/more deterministic under load.
2. Add `Drop`-based (or `std::panic::catch_unwind`-wrapped) child-process cleanup for the spawned `mcp-toolmon.exe` (and its shadow-copied `canonical.exe` grandchild) so a test panic or interruption can never leave orphaned processes holding file locks.
3. Re-run the test in a loop (10+ iterations) after the fix to confirm the flake rate drops to zero, or document the residual flake rate if some environmental jitter is irreducible.
## Root cause found (revises the original hypothesis)

The flake was not irreducible OS/AV timing jitter in the test's wait budgets — it was a real bug in [shadow.rs](workflow-tools/session/crates/mcp-toolmon/src/shadow.rs)'s `make_shadow_copy`.

`make_shadow_copy` keys its destination directory only by `{name}-{pid}-{hash(canonical_path)}` — constant across every call within one `mcp-toolmon` process lifetime — and the doc comments/tests already establish that repeat calls for the same canonical path are *expected* to reuse that same destination (`swap_child_with_drain_ms` calls it again on every respawn, after the previous child has been killed and waited on). But the directory was created with exclusive `std::fs::create_dir`, which errors `AlreadyExists` on every call after the first.

Consequence: every respawn during a reload swap hit that error in `resolve_shadow_exe`, which silently falls back to spawning directly off the *canonical* (mutable) path instead of a private shadow copy. That fallback path is exactly what raced Windows' transient just-written-file lock right after the test overwrote the canonical binary with the v2 bytes — exhausting the 4 bounded respawn attempts (~375ms of backoff) and permanently falling back to serving the stale last-known-good v1 binary. That's precisely the observed panic ("a post-swap generation call must eventually be served by v2...").

### Fix

`make_shadow_copy` now treats `AlreadyExists` as success after verifying the existing path is a real directory (not a symlink someone else planted at the predictable pid+hash path — preserves the security property the exclusive `create_dir` existed for), so respawns always get a genuine private shadow copy instead of falling back to executing the canonical file directly.

### Validation

- Full `mcp-toolmon` test suite green (`shadow`, `supervisor`, `watcher`, `handshake`, `integration_gate`, unit tests, `integration_reload_end_to_end`).
- `transparent_reload_end_to_end_subprocess` run 6/6 consecutive times, all passed (previously ~50% flaky in isolated reproduction).

### Remaining scope (item 2 from the original ticket, not yet done)

No `Drop`-based cleanup was added for orphaned `mcp-toolmon.exe`/`canonical.exe` processes on test panic/interruption. That hygiene gap is still open — tracked here, not addressed by this fix.
## Item 2 fixed: orphaned-process cleanup

Added a `ChildGuard` RAII wrapper (in [integration_reload_end_to_end.rs](workflow-tools/session/crates/mcp-toolmon/tests/integration_reload_end_to_end.rs)) around the spawned `mcp-toolmon.exe` subprocess: its `Drop` kills and waits on the process unconditionally, including on the unwind path from a test panic, so a failing assertion can no longer leak the process.

On Windows, killing the direct child alone would not kill its shadow-copied `canonical.exe` grandchild (Windows does not propagate termination to descendants by default). `ChildGuard` assigns the spawned process to a Windows job object created with `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`; since job membership is inherited by spawned descendants, closing the job handle (on `ChildGuard` drop) terminates the whole process tree, not just the direct child.

Added `windows-sys` (already vetted elsewhere in this workspace, e.g. `install-ctl`) as a Windows-only dev-dependency for the job-object FFI calls.

### Validation
- Forced a panic immediately after spawn (temporary, reverted) and confirmed via `tasklist` that no `mcp-toolmon.exe`/`canonical.exe` process survived.
- Confirmed no orphan remains after normal successful runs either.
- Full `mcp-toolmon` suite still green after adding the guard.

Both original scope items are now done; closing.