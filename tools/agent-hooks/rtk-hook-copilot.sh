#!/usr/bin/env bash
# tools/agent-hooks/rtk-hook-copilot.sh
#
# Check if stdin has data. If not, exit immediately to prevent hang.
# Otherwise, pipe stdin to rtk hook copilot.

set -euo pipefail

SCRIPT_PATH="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)/$(basename -- "${BASH_SOURCE[0]}")"
RTK_BIN=""
for candidate in \
    "${RTK_BIN_OVERRIDE:-}" \
    "${HOME:-}/.local/rtk/bin/rtk.exe" \
    "${HOME:-}/.local/rtk/bin/rtk" \
    "${HOME:-}/.cargo/bin/rtk.exe" \
    "${HOME:-}/.cargo/bin/rtk" \
    "${USERPROFILE:-}\\.cargo\\bin\\rtk.exe" \
    "${USERPROFILE:-}/.cargo/bin/rtk.exe" \
    "/c/Users/${USERNAME:-}/.cargo/bin/rtk.exe" \
    "/c/Users/${USER:-}/.cargo/bin/rtk.exe"
do
    if [[ -n "$candidate" && -f "$candidate" ]]; then
        RTK_BIN="$candidate"
        break
    fi
done

if [[ -z "$RTK_BIN" ]]; then
    RTK_BIN="$(command -v rtk 2>/dev/null || true)"
fi

if IFS= read -r -N 1 -t 0.1 first_char; then
    if [[ -n "$RTK_BIN" ]]; then
        STDERR_FILE="$(mktemp)"
        trap 'rm -f "$STDERR_FILE"' EXIT

        if { printf '%s' "$first_char"; cat; } | "$RTK_BIN" hook copilot 2>"$STDERR_FILE"; then
            cat "$STDERR_FILE" >&2
            status=0
        else
            pipeline_status=("${PIPESTATUS[@]}")
            if ((pipeline_status[1] != 0)); then
                origin="rtk hook copilot"
                status="${pipeline_status[1]}"
                failed_command="$RTK_BIN hook copilot"
            else
                origin="wrapper"
                status="${pipeline_status[0]}"
                failed_command="stdin forwarding pipeline"
            fi
            {
                echo "[rtk-hook-copilot] PreToolUse hook denial"
                printf '[rtk-hook-copilot] origin: %s\n' "$origin"
                printf '[rtk-hook-copilot] failed path: %s\n' "$SCRIPT_PATH"
                printf '[rtk-hook-copilot] configured command: bash %s\n' "$SCRIPT_PATH"
                printf '[rtk-hook-copilot] failed command: %s\n' "$failed_command"
                echo "[rtk-hook-copilot] stderr:"
                if [[ -s "$STDERR_FILE" ]]; then
                    cat "$STDERR_FILE"
                    printf '\n'
                else
                    echo "(no stderr output)"
                fi
                printf '[rtk-hook-copilot] exit status: %s\n' "$status"
            } >&2
            exit "$status"
        fi
    else
        echo "[rtk-hook-copilot] WARN: rtk not found; skipping rtk hook copilot" >&2
    fi
fi
