#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
WORKFLOW_TOOLS_ROOT="$(cd -- "$SCRIPT_DIR/../.." && pwd)"
WORKSPACE_ROOT="$(cd -- "$WORKFLOW_TOOLS_ROOT/.." && pwd)"
CONTEXT_ENGINE_ROOT="$WORKSPACE_ROOT/context-engine"
TEMP_DIR="$(mktemp -d)"
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

assert_eq() {
    local expected="$1"
    local actual="$2"
    local context="$3"
    [[ "$actual" == "$expected" ]] || fail "$context (expected '$expected', got '$actual')"
}

for dependency in bash python3 jq git; do
    command -v "$dependency" >/dev/null 2>&1 || fail "required command not found: $dependency"
done

cat > "$STUB_BIN/rtk" <<'RTK'
#!/usr/bin/env bash
printf '%s\n' "$*" > "$HOOK_TEST_RTK_ARGS"
cat > "$HOOK_TEST_RTK_INPUT"
exit "${HOOK_TEST_RTK_STATUS:-0}"
RTK

cat > "$STUB_BIN/session-capture-hook" <<'CAPTURE'
#!/usr/bin/env bash
printf '%s\n' "$*" > "$HOOK_TEST_CAPTURE_ARGS"
cat > "$HOOK_TEST_CAPTURE_INPUT"
CAPTURE
chmod +x "$STUB_BIN/rtk" "$STUB_BIN/session-capture-hook"

CONFIG_LIST="$TEMP_DIR/configs.tsv"
python3 - "$WORKSPACE_ROOT" "$CONTEXT_ENGINE_ROOT" > "$CONFIG_LIST" <<'PY'
import json
import os
import sys

roots = list(dict.fromkeys(sys.argv[1:]))
platform = {"win32": "windows", "darwin": "osx"}.get(sys.platform, "linux")

for root in roots:
    settings_path = os.path.join(root, ".vscode", "settings.json")
    config_path = os.path.join(root, ".github", "hooks", "hooks.json")
    if not os.path.isfile(settings_path) or not os.path.isfile(config_path):
        continue
    with open(settings_path, encoding="utf-8") as stream:
        settings = json.load(stream)
    locations = settings.get("chat.hookFilesLocations", {})
    if not locations.get(".github/hooks"):
        continue
    with open(config_path, encoding="utf-8") as stream:
        config = json.load(stream)
    for event, hooks in config.get("hooks", {}).items():
        for hook in hooks:
            if hook.get("type") != "command":
                raise SystemExit(f"{config_path}: unsupported hook type {hook.get('type')!r}")
            command = hook.get(platform) or hook.get("command")
            if not command:
                raise SystemExit(f"{config_path}: missing command for {event} on {platform}")
            cwd = os.path.abspath(os.path.join(root, hook.get("cwd", ".")))
            print(f"{config_path}\t{root}\t{cwd}\t{event}\t{command}")
PY

[[ -s "$CONFIG_LIST" ]] || fail "no enabled .github/hooks/hooks.json configuration found"

run_hook() {
    local cwd="$1"
    local command="$2"
    local payload="$3"
    local output_file="$TEMP_DIR/stdout"
    local stderr_file="$TEMP_DIR/stderr"
    local status

    invoke_hook() {
        (
            cd "$cwd"
            PATH="$STUB_BIN:$PATH" \
                RTK_BIN_OVERRIDE="$STUB_BIN/rtk" \
                HOOK_TEST_RTK_ARGS="$TEMP_DIR/rtk.args" \
                HOOK_TEST_RTK_INPUT="$TEMP_DIR/rtk.input" \
                HOOK_TEST_RTK_STATUS="${HOOK_TEST_RTK_STATUS:-0}" \
                HOOK_TEST_CAPTURE_ARGS="$TEMP_DIR/capture.args" \
                HOOK_TEST_CAPTURE_INPUT="$TEMP_DIR/capture.input" \
                bash -c "$command"
        )
    }

    if [[ -n "$payload" ]]; then
        if invoke_hook <<< "$payload" > "$output_file" 2> "$stderr_file"; then
            status=0
        else
            status=$?
        fi
    elif invoke_hook </dev/null > "$output_file" 2> "$stderr_file"; then
        status=0
    else
        status=$?
    fi

    HOOK_STATUS="$status"
    HOOK_STDOUT="$(<"$output_file")"
    HOOK_STDERR="$(<"$stderr_file")"
}

test_capture_hook() {
    local cwd="$1"
    local command="$2"
    local event="$3"
    local payload="{\"hook_event_name\":\"$event\",\"session_id\":\"hook-test\"}"

    run_hook "$cwd" "$command" "$payload"
    assert_eq 0 "$HOOK_STATUS" "$event capture hook status"
    assert_eq "--from-hook-stdin" "$(<"$TEMP_DIR/capture.args")" "$event capture hook arguments"
    assert_eq "$payload" "$(<"$TEMP_DIR/capture.input")" "$event capture hook input"
}

test_rtk_hook() {
    local cwd="$1"
    local command="$2"
    local payload='{"hook_event_name":"PreToolUse","tool_name":"apply_patch"}'

    rm -f "$TEMP_DIR/rtk.args" "$TEMP_DIR/rtk.input"
    run_hook "$cwd" "$command" "$payload"
    assert_eq 0 "$HOOK_STATUS" "RTK hook status"
    assert_eq "hook copilot" "$(<"$TEMP_DIR/rtk.args")" "RTK hook arguments"
    assert_eq "$payload" "$(<"$TEMP_DIR/rtk.input")" "RTK hook input"

    HOOK_TEST_RTK_STATUS=23 run_hook "$cwd" "$command" "$payload"
    assert_eq 23 "$HOOK_STATUS" "RTK hook propagates child failure"

    rm -f "$TEMP_DIR/rtk.args"
    run_hook "$cwd" "$command" ""
    assert_eq 0 "$HOOK_STATUS" "RTK hook empty-input status"
    [[ ! -e "$TEMP_DIR/rtk.args" ]] || fail "RTK hook invoked without stdin"
}

test_preflight_hook() {
    local cwd="$1"
    local command="$2"
    local valid_file="$TEMP_DIR/valid.sh"
    local invalid_file="$TEMP_DIR/invalid.sh"

    printf 'printf \"valid\\\\n\"\\n' > "$valid_file"
    printf 'if then\\n' > "$invalid_file"

    run_hook "$cwd" "$command" \
        "{\"tool_name\":\"create_file\",\"tool_input\":{\"filePath\":\"$valid_file\"}}"
    assert_eq 0 "$HOOK_STATUS" "preflight accepts valid shell"
    [[ "$HOOK_STDERR" == *"Shell syntax OK"* ]] || fail "preflight did not run bash -n for valid shell"

    run_hook "$cwd" "$command" \
        "{\"tool_name\":\"create_file\",\"tool_input\":{\"filePath\":\"$invalid_file\"}}"
    assert_eq 1 "$HOOK_STATUS" "preflight blocks invalid shell"
    [[ "$HOOK_STDERR" == *"Shell syntax error"* ]] || fail "preflight omitted its blocking diagnostic"

    run_hook "$cwd" "$command" '{"tool_name":"read_file","tool_input":{"path":"ignored.sh"}}'
    assert_eq 0 "$HOOK_STATUS" "preflight allows non-write tools"
}

test_validate_docs_hook() {
    local cwd="$1"
    local command="$2"
    local payload='{"tool_name":"replace_string_in_file","tool_input":{"filePath":"memory-viewers/doc-viewer/src/App.tsx"}}'

    run_hook "$cwd" "$command" "$payload"
    assert_eq 0 "$HOOK_STATUS" "documentation validation hook status"
    printf '%s' "$HOOK_STDOUT" | jq -e \
        '.hookSpecificOutput.hookEventName == "PostToolUse" and (.hookSpecificOutput.additionalContext | contains("mcp_docs-server_validate_docs"))' \
        >/dev/null || fail "documentation source edit did not produce its validation reminder"

    run_hook "$cwd" "$command" \
        '{"tool_name":"replace_string_in_file","tool_input":{"filePath":"README.md"}}'
    assert_eq 0 "$HOOK_STATUS" "documentation validation no-match status"
    assert_eq '{}' "$HOOK_STDOUT" "documentation validation no-match output"
}

test_terminal_pwd_hook() {
    local cwd="$1"
    local command="$2"

    run_hook "$cwd" "$command" \
        '{"tool_name":"run_in_terminal","tool_input":{"isBackground":false}}'
    assert_eq 0 "$HOOK_STATUS" "terminal reminder hook status"
    printf '%s' "$HOOK_STDOUT" | jq -e \
        '.hookSpecificOutput.hookEventName == "PostToolUse" and (.hookSpecificOutput.additionalContext | contains("Run pwd"))' \
        >/dev/null || fail "foreground terminal command did not produce the cwd reminder"

    run_hook "$cwd" "$command" \
        '{"tool_name":"run_in_terminal","tool_input":{"isBackground":true}}'
    assert_eq 0 "$HOOK_STATUS" "background terminal hook status"
    assert_eq '' "$HOOK_STDOUT" "background terminal hook output"

    run_hook "$cwd" "$command" '{"tool_name":"read_file"}'
    assert_eq 0 "$HOOK_STATUS" "non-terminal reminder hook status"
    assert_eq '' "$HOOK_STDOUT" "non-terminal reminder hook output"
}

tested=0
while IFS=$'\t' read -r config_path workspace_root cwd event command; do
    case "$command" in
        "session-capture-hook --from-hook-stdin")
            test_capture_hook "$cwd" "$command" "$event"
            ;;
        *run-configured-hook.sh*rtk-hook-copilot.sh*)
            test_rtk_hook "$cwd" "$command"
            ;;
        *run-configured-hook.sh*session-capture-hook*)
            test_capture_hook "$cwd" "$command" "$event"
            ;;
        *run-configured-hook.sh*preflight-write.sh*)
            test_preflight_hook "$cwd" "$command"
            ;;
        "bash "*)
            case "${command#bash }" in
                workflow-tools/tools/agent-hooks/rtk-hook-copilot.sh|../workflow-tools/tools/agent-hooks/rtk-hook-copilot.sh)
                    test_rtk_hook "$cwd" "$command"
                    ;;
                workflow-tools/tools/agent-hooks/preflight-write.sh|../workflow-tools/tools/agent-hooks/preflight-write.sh)
                    test_preflight_hook "$cwd" "$command"
                    ;;
                workflow-tools/tools/agent-hooks/validate-docs.sh|../workflow-tools/tools/agent-hooks/validate-docs.sh)
                    test_validate_docs_hook "$cwd" "$command"
                    ;;
                workflow-tools/tools/agent-hooks/terminal-pwd.sh|../workflow-tools/tools/agent-hooks/terminal-pwd.sh)
                    test_terminal_pwd_hook "$cwd" "$command"
                    ;;
                *)
                    fail "$config_path configures an untested hook command: $command"
                    ;;
            esac
            ;;
        *)
            fail "$config_path configures an untested hook command: $command"
            ;;
    esac

    tested=$((tested + 1))
    echo "PASS: $event - $command"
done < "$CONFIG_LIST"

(( tested > 0 )) || fail "no configured hook commands were tested"
echo "PASS: tested $tested active hook command entries across enabled workspace configurations"
