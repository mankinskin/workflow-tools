#!/bin/bash
# Post-tool-use hook for terminal commands
# Adds pwd reminder to context after run_in_terminal executes
#
# Canonical location: tools/agent-hooks/terminal-pwd.sh
# Referenced by: .github/hooks/hooks.json

if read -t 0; then
    INPUT=$(cat)
else
    INPUT="{}"
fi
TOOL_NAME=$(echo "$INPUT" | jq -r '.tool_name // .toolName // "unknown"' 2>/dev/null)

# Only act on run_in_terminal calls (non-background)
if [[ "$TOOL_NAME" != "run_in_terminal" ]]; then
    exit 0
fi

IS_BACKGROUND=$(echo "$INPUT" | jq -r '.tool_input.isBackground // .toolArgs.isBackground // false' 2>/dev/null)
if [[ "$IS_BACKGROUND" == "true" ]]; then
    exit 0
fi

cat << 'EOF'
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "Run pwd to confirm cwd. If inside a session-assigned worktree, perform all implementation, validation, and board check-in operations from that directory."
  }
}
EOF