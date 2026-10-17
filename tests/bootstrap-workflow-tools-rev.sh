#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
bootstrap="$repo_root/bootstrap.sh"
sha="0123456789abcdef0123456789abcdef01234567"
temp_dir="$(mktemp -d)"
trap 'rm -rf "$temp_dir"' EXIT
mkdir -p "$temp_dir/bin" "$temp_dir/workspace with spaces"

cat >"$temp_dir/pinned-bootstrap.sh" <<'EOF'
#!/usr/bin/env bash
printf '<%s>\n' "$@" >"$FORWARDED_ARGS"
EOF
chmod +x "$temp_dir/pinned-bootstrap.sh"

cat >"$temp_dir/bin/curl" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
while [[ $# -gt 0 ]]; do
    case "$1" in
        --output) output="$2"; shift 2 ;;
        *) shift ;;
    esac
done
if [[ "${CURL_FIXTURE_MODE:-}" == fail ]]; then
    printf 'partial payload must not execute\n' >"$output"
    exit 22
fi
cp "$PINNED_BOOTSTRAP" "$output"
EOF
chmod +x "$temp_dir/bin/curl"

export PATH="$temp_dir/bin:$PATH"
export PINNED_BOOTSTRAP="$temp_dir/pinned-bootstrap.sh"
export FORWARDED_ARGS="$temp_dir/forwarded.txt"

"$bootstrap" --root "$temp_dir/root with spaces" --workspace "$temp_dir/workspace with spaces" \
    --workflow-tools-rev "$sha" --dry-run
grep -Fx "<--root>" "$FORWARDED_ARGS"
grep -Fx "<$temp_dir/root with spaces>" "$FORWARDED_ARGS"
grep -Fx "<--workspace>" "$FORWARDED_ARGS"
grep -Fx "<$temp_dir/workspace with spaces>" "$FORWARDED_ARGS"
grep -Fx "<--dry-run>" "$FORWARDED_ARGS"

rm -f "$FORWARDED_ARGS"
if CURL_FIXTURE_MODE=fail "$bootstrap" --root "$temp_dir/root" --workspace "$temp_dir/workspace with spaces" \
    --workflow-tools-rev "$sha" --dry-run; then
    printf '%s\n' 'expected failed pinned download to fail' >&2
    exit 1
fi
[[ ! -e "$FORWARDED_ARGS" ]]
