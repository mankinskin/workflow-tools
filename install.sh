#!/usr/bin/env bash
# Public curl|bash entry point: installs install-ctl into a local workspace
# directory (never a system bin dir) at a pinned workflow-tools commit, then
# hands off to it. Ticket/spec bundle + consumer init is bootstrap.sh's job.
set -euo pipefail

WORKFLOW_TOOLS_REPOSITORY="https://github.com/mankinskin/workflow-tools"
WORKFLOW_TOOLS_REVISION="60a96726476e21d13664f4ce86eda886ab58cf6f"

usage() {
    cat <<'EOF'
Usage: install.sh --root <install-root> [--dry-run] [-- <install-ctl args>]

Installs install-ctl (pinned workflow-tools commit) into <install-root>/bin
and, unless --dry-run is given, execs it with any trailing arguments.

Example:
  curl -fsSL https://raw.githubusercontent.com/mankinskin/workflow-tools/<rev>/install.sh \
    | bash -s -- --root "$HOME/.local/workflow-tools"
EOF
}

install_root=""
dry_run=false
ctl_args=()

while [[ $# -gt 0 ]]; do
    case "$1" in
        --root)
            install_root="${2:-}"
            shift 2
            ;;
        --dry-run)
            dry_run=true
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        --)
            shift
            ctl_args=("$@")
            break
            ;;
        *)
            usage >&2
            exit 2
            ;;
    esac
done

if [[ -z "$install_root" ]]; then
    usage >&2
    exit 2
fi

command=(
    cargo install
    --git "$WORKFLOW_TOOLS_REPOSITORY"
    --rev "$WORKFLOW_TOOLS_REVISION"
    --bin install-ctl
    --root "$install_root"
    install-ctl
)

if "$dry_run"; then
    printf '+ '
    printf '%q ' "${command[@]}"
    printf '\n'
    exit 0
fi

"${command[@]}"

printf 'Installed install-ctl: %s/bin/install-ctl\n' "$install_root"

if [[ ${#ctl_args[@]} -gt 0 ]]; then
    exec "$install_root/bin/install-ctl" "${ctl_args[@]}"
fi
