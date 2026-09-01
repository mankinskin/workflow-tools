#!/usr/bin/env bash
# Instruction-link validator wrapper (ticket 5bc9fede, Waypoint 4).
#
# Usage:
#   bash workflow-tools/tools/validate-instruction-links.sh \
#     --manifest <instruction-distribution.md> \
#     --baseline <instruction-link-baseline.txt> \
#     [--exceptions <instruction-link-exceptions.txt>]
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

manifest=""
baseline=""
exceptions=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --manifest) manifest="$2"; shift 2 ;;
    --baseline) baseline="$2"; shift 2 ;;
    --exceptions) exceptions="$2"; shift 2 ;;
    *) printf 'unknown argument: %s\n' "$1" >&2; exit 2 ;;
  esac
done

if [[ -z "$manifest" || -z "$baseline" ]]; then
  printf 'usage: %s --manifest <path> --baseline <path> [--exceptions <path>]\n' "$0" >&2
  exit 2
fi

args=(--manifest "$manifest" --baseline "$baseline")
if [[ -n "$exceptions" ]]; then
  args+=(--exceptions "$exceptions")
fi

exec python3 "$script_dir/validate_instruction_links.py" "${args[@]}"
