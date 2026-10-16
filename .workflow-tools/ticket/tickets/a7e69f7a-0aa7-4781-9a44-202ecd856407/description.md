## Observed
Repeated attempts to call `functions.view` and `functions.powershell` in `C:\Users\linus\git\3\meta-workspace` are denied with the exact message:
`Denied by preToolUse hook from ".github_hooks_hooks.json" (hook errored)`

The denial has also blocked read access and a commit attempt in this workspace.

## Expected
Tool use should either proceed or fail with actionable hook output that names the failing hook command and the underlying stderr/exit status.

## Evidence
- Hook config: `.github/hooks/hooks.json`
- Hook implementation: `context-engine/tools/agent-hooks/rtk-hook-copilot.sh`
- Manual check: `rtk hook copilot` reports `[rtk] /!\ No hook installed -- run \`rtk init -g\` for automatic token savings` and exits 0
- Search results show related but not exact tickets for hook capture / feedback follow-up; no exact duplicate found for this denial

## Impact
Blocks investigation and normal tool use in the workspace.

## Scope note
Keep clearer failure output in this bug only if it is the minimum change required to make the denial actionable. If it expands into general hook UX improvements, track that as a separate planning item.

## Acceptance criteria
- The failing hook path and command are visible when the PreToolUse hook errors.
- The denial no longer appears as a generic opaque `hook errored` message for this case.
- A maintainer can tell whether the failure is in the hook wrapper, `rtk hook copilot`, or another hook in the chain without reproducing locally.
