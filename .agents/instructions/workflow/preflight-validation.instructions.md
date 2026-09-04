---
description: "Use when encountering preflight write hook failures or configuring syntax validation for file-write operations."
applyTo: "**/*.rs,**/*.ts,**/*.toml,**/*.json"
---

## Pre-flight Write Validation

The `tools/agent-hooks/preflight-write.sh` hook runs automatically as a `PreToolUse` hook before file-write operations (`create_file`, `replace_string_in_file`, `multi_replace_string_in_file`). It:

- Runs `cargo check` for `.rs` files (nearest Cargo.toml).
- Runs `python3 -m py_compile` for `.py` files.
- Runs `bash -n` for `.sh` files.
- Parses TOML for `.toml` files (advisory, non-blocking).
- Runs `tsc --noEmit` for `.ts`/`.tsx` files (advisory, non-blocking).

**If a check fails (blocking):** The write is rejected with a diagnostic. Fix the syntax error before the tool call is retried.

**If a checker is unavailable:** A warning is emitted but the write is allowed. Record the missing checker gap in the ticket/spec status summary.

**Bypass:** Add `--no-verify` to the git commit or set `SKIP_PREFLIGHT=1` in the environment when the check is a false positive. Document why in the commit message.
