# workflow-tools

## Installation Registry

`install/artifacts.toml` is the canonical inventory of workflow-tools source
artifacts. Each `source_path` is relative to this repository root. The
`install/install-ctl` crate reads the registry at runtime and generates
`COMMANDS.md`:

```bash
cargo run --manifest-path install/install-ctl/Cargo.toml -- catalog --check
```

## Public Install Entry Point

`install.sh` is the single, commit-pinned `curl | bash` entry point. It
installs `install-ctl` into a caller-supplied local directory (never a
system-wide bin path) at a pinned `workflow-tools` commit, then hands off to
it:

```bash
curl -fsSL https://raw.githubusercontent.com/mankinskin/workflow-tools/<rev>/install.sh \
  | bash -s -- --root "$HOME/.local/workflow-tools"
```

Use `--dry-run` to preview the pinned `cargo install` command, or append
`-- <install-ctl args>` to run `install-ctl` immediately after installing it
(for example `-- --list`).

## Bootstrap A Consumer

Install the minimal ticket/spec CLI bundle and initialize one consumer workspace
with a single, version-pinned command:

```bash
bash bootstrap.sh \
	--root "$HOME/.local/workflow-tools" \
	--workspace /path/to/minimal-demo
```

The bootstrap command installs only into the supplied `--root` and initializes
only the supplied `--workspace`. It never discovers a sibling consumer from a
superproject working directory. Use `--dry-run` to inspect the pinned install
commands before modifying a consumer workspace.

`workflow-minimal-demo` is the first top-level consumer workspace in the
meta-workspace and exercises this install path.
