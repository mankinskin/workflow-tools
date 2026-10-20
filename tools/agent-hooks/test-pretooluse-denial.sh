#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
WORKFLOW_TOOLS_ROOT="$(cd -- "$SCRIPT_DIR/../.." && pwd)"
WORKSPACE_ROOT="$(cd -- "$WORKFLOW_TOOLS_ROOT/.." && pwd)"
CONTEXT_ENGINE_ROOT="$WORKSPACE_ROOT/context-engine"
WRAPPER="$SCRIPT_DIR/rtk-hook-copilot.sh"
RUNNER="$SCRIPT_DIR/run-configured-hook.sh"
TEMP_DIR="$(mktemp -d)"
STUB_RTK="$TEMP_DIR/rtk"
STUB_BIN="$TEMP_DIR/bin"
mkdir -p "$STUB_BIN"

cleanup() {
    rm -rf "$TEMP_DIR"
}
trap cleanup EXIT

fail() {
    echo "FAIL: $*" >&2
    exit 1
}

assert_contains() {
    local needle="$1"
    local file="$2"
    grep -F -- "$needle" "$file" >/dev/null || fail "missing '$needle'"
}

ROOT_RTK_COMMAND='bash workflow-tools/tools/agent-hooks/run-configured-hook.sh wrapper "bash workflow-tools/tools/agent-hooks/rtk-hook-copilot.sh" -- bash workflow-tools/tools/agent-hooks/rtk-hook-copilot.sh'
ROOT_PREFLIGHT_COMMAND='bash workflow-tools/tools/agent-hooks/run-configured-hook.sh "another configured hook" "bash workflow-tools/tools/agent-hooks/preflight-write.sh" -- bash workflow-tools/tools/agent-hooks/preflight-write.sh'
ROOT_CAPTURE_COMMAND='bash workflow-tools/tools/agent-hooks/run-configured-hook.sh "another configured hook" "session-capture-hook --from-hook-stdin" -- session-capture-hook --from-hook-stdin'
ENGINE_PREFLIGHT_COMMAND='bash ../workflow-tools/tools/agent-hooks/run-configured-hook.sh "another configured hook" "bash ../workflow-tools/tools/agent-hooks/preflight-write.sh" -- bash ../workflow-tools/tools/agent-hooks/preflight-write.sh'

python3 - "$WORKSPACE_ROOT" "$CONTEXT_ENGINE_ROOT" <<'PY'
import json
import os
import sys

expected = [
    [
        'bash workflow-tools/tools/agent-hooks/run-configured-hook.sh wrapper "bash workflow-tools/tools/agent-hooks/rtk-hook-copilot.sh" -- bash workflow-tools/tools/agent-hooks/rtk-hook-copilot.sh',
        'bash workflow-tools/tools/agent-hooks/run-configured-hook.sh "another configured hook" "bash workflow-tools/tools/agent-hooks/preflight-write.sh" -- bash workflow-tools/tools/agent-hooks/preflight-write.sh',
        'bash workflow-tools/tools/agent-hooks/run-configured-hook.sh "another configured hook" "session-capture-hook --from-hook-stdin" -- session-capture-hook --from-hook-stdin',
    ],
    [
        'bash ../workflow-tools/tools/agent-hooks/run-configured-hook.sh wrapper "bash ../workflow-tools/tools/agent-hooks/rtk-hook-copilot.sh" -- bash ../workflow-tools/tools/agent-hooks/rtk-hook-copilot.sh',
        'bash ../workflow-tools/tools/agent-hooks/run-configured-hook.sh "another configured hook" "bash ../workflow-tools/tools/agent-hooks/preflight-write.sh" -- bash ../workflow-tools/tools/agent-hooks/preflight-write.sh',
        'bash ../workflow-tools/tools/agent-hooks/run-configured-hook.sh "another configured hook" "session-capture-hook --from-hook-stdin" -- session-capture-hook --from-hook-stdin',
    ],
]

for root, commands in zip(sys.argv[1:], expected):
    settings_path = os.path.join(root, ".vscode", "settings.json")
    hooks_path = os.path.join(root, ".github", "hooks", "hooks.json")
    with open(settings_path, encoding="utf-8") as stream:
        settings = json.load(stream)
    if not settings.get("chat.hookFilesLocations", {}).get(".github/hooks"):
        raise SystemExit(f"{settings_path}: hooks are not enabled")
    with open(hooks_path, encoding="utf-8") as stream:
        configured = json.load(stream)["hooks"]["PreToolUse"]
    actual = [hook["command"] for hook in configured]
    if actual != commands:
        raise SystemExit(f"{hooks_path}: PreToolUse command list is not fully diagnostic")
PY

cat > "$STUB_RTK" <<'RTK'
#!/usr/bin/env bash
cat > "$HOOK_TEST_INPUT"
printf '%s\n' "${HOOK_TEST_STDERR:-delegate rejected request}" >&2
printf '%s\n' "${HOOK_TEST_STDOUT:-delegate output}"
exit "${HOOK_TEST_STATUS:-0}"
RTK
chmod +x "$STUB_RTK"

payload='{"hook_event_name":"PreToolUse","tool_name":"powershell"}'
stdout="$TEMP_DIR/stdout"
stderr="$TEMP_DIR/stderr"

invoke_hook() (
    cd "$HOOK_CWD"
    PATH="$STUB_BIN:$PATH" \
        RTK_BIN_OVERRIDE="$STUB_RTK" \
        HOOK_TEST_INPUT="$TEMP_DIR/input" \
        HOOK_TEST_STATUS="${HOOK_TEST_STATUS:-0}" \
        HOOK_TEST_STDERR="${HOOK_TEST_STDERR:-delegate rejected request}" \
        HOOK_TEST_STDOUT="${HOOK_TEST_STDOUT:-delegate output}" \
        HOOK_TEST_CAPTURE_ARGS="$TEMP_DIR/capture.args" \
        HOOK_TEST_CAPTURE_INPUT="$TEMP_DIR/capture.input" \
        HOOK_TEST_CAPTURE_STATUS="${HOOK_TEST_CAPTURE_STATUS:-0}" \
        HOOK_TEST_CAPTURE_STDERR="${HOOK_TEST_CAPTURE_STDERR:-capture rejected request}" \
        bash -c "$HOOK_COMMAND"
)

run_hook() {
    local cwd="$1"
    local command="$2"
    local input="$3"

    HOOK_CWD="$cwd"
    HOOK_COMMAND="$command"
    if [[ -n "$input" ]]; then
        if printf '%s' "$input" | invoke_hook >"$stdout" 2>"$stderr"; then
            HOOK_STATUS=0
        else
            HOOK_STATUS=$?
        fi
    elif invoke_hook </dev/null >"$stdout" 2>"$stderr"; then
        HOOK_STATUS=0
    else
        HOOK_STATUS=$?
    fi
    HOOK_STDOUT="$(<"$stdout")"
    HOOK_STDERR="$(<"$stderr")"
}

if [[ ! -f "$RUNNER" ]]; then
    fail "configured-hook diagnostic runner is missing"
fi

HOOK_TEST_STATUS=23 HOOK_TEST_STDERR='delegate policy denied request' \
    run_hook "$WORKSPACE_ROOT" "$ROOT_RTK_COMMAND" "$payload"
if [[ "$HOOK_STATUS" == 0 ]]; then
    fail "delegate failure unexpectedly succeeded"
fi

[[ "$HOOK_STATUS" == 23 ]] || fail "delegate status (expected 23, got $HOOK_STATUS)"
[[ "$(<"$TEMP_DIR/input")" == "$payload" ]] || fail "wrapper did not forward input"
assert_contains '[configured-hook] origin: wrapper' "$stderr"
assert_contains '[configured-hook] failed path: workflow-tools/tools/agent-hooks/rtk-hook-copilot.sh' "$stderr"
assert_contains '[configured-hook] configured command: bash workflow-tools/tools/agent-hooks/rtk-hook-copilot.sh' "$stderr"
assert_contains '[rtk-hook-copilot] origin: rtk hook copilot' "$stderr"
assert_contains "[rtk-hook-copilot] failed command: $STUB_RTK hook copilot" "$stderr"
assert_contains 'delegate policy denied request' "$stderr"
assert_contains '[rtk-hook-copilot] exit status: 23' "$stderr"
assert_contains '[configured-hook] exit status: 23' "$stderr"

HOOK_TEST_STATUS=0 HOOK_TEST_STDERR='delegate warning' \
    run_hook "$WORKSPACE_ROOT" "$ROOT_RTK_COMMAND" "$payload"
[[ "$HOOK_STATUS" == 0 ]] || fail "successful delegate failed with status $HOOK_STATUS"
[[ "$(<"$TEMP_DIR/input")" == "$payload" ]] || fail "success did not forward input"
assert_contains 'delegate output' "$stdout"
assert_contains 'delegate warning' "$stderr"
[[ "$stderr" != *'hook denial'* ]] || fail "successful delegate emitted denial diagnostics"

rm -f "$TEMP_DIR/input"
run_hook "$WORKSPACE_ROOT" "$ROOT_RTK_COMMAND" ""
[[ "$HOOK_STATUS" == 0 ]] || fail "empty input failed with status $HOOK_STATUS"
[[ ! -e "$TEMP_DIR/input" ]] || fail "empty input invoked delegate"

invalid_file="$TEMP_DIR/invalid.sh"
printf 'if then\n' > "$invalid_file"
preflight_payload="{\"tool_name\":\"create_file\",\"tool_input\":{\"filePath\":\"$invalid_file\"}}"
HOOK_CWD="$CONTEXT_ENGINE_ROOT"
HOOK_COMMAND="$ENGINE_PREFLIGHT_COMMAND"
if printf '%s' "$preflight_payload" | invoke_hook >"$stdout" 2>"$stderr"; then
    fail "configured preflight hook unexpectedly succeeded"
else
    HOOK_STATUS=$?
fi
[[ "$HOOK_STATUS" == 1 ]] || fail "preflight status (expected 1, got $HOOK_STATUS)"
assert_contains '[configured-hook] origin: another configured hook' "$stderr"
assert_contains '[configured-hook] failed path: ../workflow-tools/tools/agent-hooks/preflight-write.sh' "$stderr"
assert_contains '[configured-hook] configured command: bash ../workflow-tools/tools/agent-hooks/preflight-write.sh' "$stderr"
assert_contains 'Shell syntax error' "$stderr"
assert_contains '[configured-hook] exit status: 1' "$stderr"
[[ "$(<"$stderr")" != *'origin: rtk hook copilot'* ]] || fail "preflight failure was attributed to RTK"

HOOK_CWD="$WORKSPACE_ROOT"
HOOK_COMMAND="$ROOT_PREFLIGHT_COMMAND"
if printf '%s' "$preflight_payload" | invoke_hook >"$stdout" 2>"$stderr"; then
    fail "workspace preflight hook unexpectedly succeeded"
else
    HOOK_STATUS=$?
fi
[[ "$HOOK_STATUS" == 1 ]] || fail "workspace preflight status (expected 1, got $HOOK_STATUS)"
assert_contains '[configured-hook] origin: another configured hook' "$stderr"
assert_contains '[configured-hook] failed path: workflow-tools/tools/agent-hooks/preflight-write.sh' "$stderr"
assert_contains '[configured-hook] configured command: bash workflow-tools/tools/agent-hooks/preflight-write.sh' "$stderr"
assert_contains 'Shell syntax error' "$stderr"
assert_contains '[configured-hook] exit status: 1' "$stderr"

cat > "$STUB_BIN/session-capture-hook" <<'CAPTURE'
#!/usr/bin/env bash
cat > "$HOOK_TEST_CAPTURE_INPUT"
printf '%s\n' "${HOOK_TEST_CAPTURE_STDERR:-capture rejected request}" >&2
exit "${HOOK_TEST_CAPTURE_STATUS:-0}"
CAPTURE
chmod +x "$STUB_BIN/session-capture-hook"
HOOK_TEST_CAPTURE_STATUS=41 HOOK_TEST_CAPTURE_STDERR='capture failed for test' \
    run_hook "$WORKSPACE_ROOT" "$ROOT_CAPTURE_COMMAND" "$payload"
[[ "$HOOK_STATUS" == 41 ]] || fail "capture status (expected 41, got $HOOK_STATUS)"
assert_contains '[configured-hook] origin: another configured hook' "$stderr"
assert_contains "[configured-hook] failed path: $STUB_BIN/session-capture-hook" "$stderr"
assert_contains '[configured-hook] configured command: session-capture-hook --from-hook-stdin' "$stderr"
assert_contains 'capture failed for test' "$stderr"
assert_contains '[configured-hook] exit status: 41' "$stderr"
[[ "$(<"$TEMP_DIR/capture.input")" == "$payload" ]] || fail "capture hook did not receive input"

echo "PASS: configured PreToolUse failures identify wrapper, delegate, and other hook boundaries"
