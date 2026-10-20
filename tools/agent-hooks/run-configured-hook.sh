#!/usr/bin/env bash
set -euo pipefail

if (($# < 4)); then
    echo "usage: $0 <origin> <configured-command> -- <command> [args...]" >&2
    exit 2
fi

origin="$1"
configured_command="$2"
shift 2

if [[ "$1" != "--" ]]; then
    echo "usage: $0 <origin> <configured-command> -- <command> [args...]" >&2
    exit 2
fi
shift

if (($# == 0)); then
    echo "configured hook command is empty" >&2
    exit 2
fi

if [[ "$1" == "bash" || "$1" == "sh" ]]; then
    failed_path="${2:-$1}"
else
    failed_path="$(command -v "$1" 2>/dev/null || printf '%s' "$1")"
fi

stderr_file="$(mktemp)"
trap 'rm -f "$stderr_file"' EXIT

if "$@" 2>"$stderr_file"; then
    if [[ -s "$stderr_file" ]]; then
        cat "$stderr_file" >&2
    fi
    exit 0
else
    status=$?
fi

{
    echo "[configured-hook] PreToolUse hook denial"
    printf '[configured-hook] origin: %s\n' "$origin"
    printf '[configured-hook] failed path: %s\n' "$failed_path"
    printf '[configured-hook] configured command: %s\n' "$configured_command"
    echo "[configured-hook] stderr:"
    if [[ -s "$stderr_file" ]]; then
        cat "$stderr_file"
        printf '\n'
    else
        echo "(no stderr output)"
    fi
    printf '[configured-hook] exit status: %s\n' "$status"
} >&2

exit "$status"
