# `install-guidance.sh` top-level wrapper

## Scope

Add `install-guidance.sh` at repo root, matching the existing install-script family shape: source `tools/install/common.sh`, expose a `<x>_names` array, a `<x>_path()` case dispatch, and a `usage()` function, consistent with `install-tools.sh`, `install-deps.sh`, and `install-extensions.sh`.

- Delegate to `rule install`, building the binary first if absent.
- Support `--client`, `--surface`, `--skills`, `--all`, `--dry-run`, `--check`, `--list`.
- `--list` prints available clients and surfaces from the committed manifest.
- Provide an interactive client picker when run with no arguments on a TTY.

## Acceptance criteria

1. `./install-guidance.sh --list` works on a fresh clone with no prior build.
2. `./install-guidance.sh --client copilot` produces the same result as the equivalent `rule install` invocation.
3. Help output matches the family conventions.
