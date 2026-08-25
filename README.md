# workflow-tools

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
