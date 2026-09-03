#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")"

suite_timeout="${MOVE_BENCH_SUITE_TIMEOUT_SECS:-590}"
if ! [[ "$suite_timeout" =~ ^[0-9]+$ ]] || (( suite_timeout < 1 )); then
    printf 'MOVE_BENCH_SUITE_TIMEOUT_SECS must be a positive integer\n' >&2
    exit 2
fi
if (( suite_timeout > 590 )); then
    suite_timeout=590
fi

export MOVE_BENCH_DEADLINE_SECS="${MOVE_BENCH_DEADLINE_SECS:-540}"
export MOVE_BENCH_FILTER="${MOVE_BENCH_FILTER:-}"
export MOVE_BENCH_PACKAGES="${MOVE_BENCH_PACKAGES:-ticket-api spec-api session-api rule-api audit-api}"

set +e
timeout --foreground --kill-after=1s "${suite_timeout}s" bash -c '
    set -euo pipefail
    read -r -a packages <<< "$MOVE_BENCH_PACKAGES"
    for package in "${packages[@]}"; do
        printf "=== %s ===\n" "$package"
        if [[ -n "$MOVE_BENCH_FILTER" ]]; then
            cargo bench -p "$package" --bench move_health -- "$MOVE_BENCH_FILTER"
        else
            cargo bench -p "$package" --bench move_health
        fi
    done
'
status=$?
set -e

if (( status == 124 )); then
    printf 'move benchmark suite reached its %ss wall-time cap\n' "$suite_timeout" >&2
fi
exit "$status"
