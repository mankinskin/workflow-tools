#!/usr/bin/env bash
set -euo pipefail

TICKET_REPOSITORY="https://github.com/mankinskin/ticket"
TICKET_REVISION="a02cb0773ea9e63e3ce979cb43da9a92579dad16"
SPEC_REPOSITORY="https://github.com/mankinskin/spec"
SPEC_REVISION="6dddad9dc8ad4f79b62b5d4a10b1008a9a52f6b7"

usage() {
    cat <<'EOF'
Usage: bootstrap.sh --root <install-root> --workspace <consumer-root> [--workflow-tools-rev <full-sha>] [--dry-run]

Installs the minimal workflow-tools CLI bundle into <install-root>/bin and
initializes ticket and spec stores only in the selected consumer workspace.

Pass --workflow-tools-rev to re-execute this script from that exact
workflow-tools commit before installing the bundle.
EOF
}

install_root=""
workspace=""
workflow_tools_rev=""
dry_run=false
forward_args=()

while [[ $# -gt 0 ]]; do
    case "$1" in
        --root)
            install_root="${2:-}"
            forward_args+=("$1" "$install_root")
            shift 2
            ;;
        --workspace)
            workspace="${2:-}"
            forward_args+=("$1" "$workspace")
            shift 2
            ;;
        --workflow-tools-rev)
            workflow_tools_rev="${2:-}"
            shift 2
            ;;
        --dry-run)
            dry_run=true
            forward_args+=("$1")
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            usage >&2
            exit 2
            ;;
    esac
done

if [[ -n "$workflow_tools_rev" ]]; then
    if [[ ! "$workflow_tools_rev" =~ ^[0-9a-fA-F]{40}$ ]]; then
        printf '%s\n' '--workflow-tools-rev must be a full 40-character commit SHA' >&2
        exit 2
    fi

    bootstrap_file="$(mktemp "${TMPDIR:-/tmp}/workflow-tools-bootstrap.XXXXXX")"
    cleanup_bootstrap_file() {
        rm -f "$bootstrap_file"
    }
    trap cleanup_bootstrap_file EXIT

    if ! curl -fsSL --output "$bootstrap_file" \
        "https://raw.githubusercontent.com/mankinskin/workflow-tools/$workflow_tools_rev/bootstrap.sh"; then
        printf '%s\n' "failed to fetch bootstrap.sh at $workflow_tools_rev" >&2
        exit 1
    fi

    bash "$bootstrap_file" "${forward_args[@]}"
    exit $?
fi

if [[ -z "$install_root" || -z "$workspace" ]]; then
    usage >&2
    exit 2
fi

if [[ ! -d "$workspace" ]]; then
    printf 'workspace does not exist: %s\n' "$workspace" >&2
    exit 2
fi

install_tool() {
    local name="$1"
    local repository="$2"
    local revision="$3"

    local command=(
        cargo install
        --git "$repository"
        --rev "$revision"
        --features cli
        --bin "$name"
        --root "$install_root"
        "$name"
    )

    if "$dry_run"; then
        printf '+ '
        printf '%q ' "${command[@]}"
        printf '\n'
        return
    fi

    "${command[@]}"
}

initialize_store() {
    local binary="$1"
    local store_dir="$2"
    local command=("$install_root/bin/$binary" --index-root "$store_dir" init)

    if "$dry_run"; then
        printf '+ '
        printf '%q ' "${command[@]}"
        printf '\n'
        return
    fi

    "${command[@]}"
}

install_tool ticket "$TICKET_REPOSITORY" "$TICKET_REVISION"
install_tool spec "$SPEC_REPOSITORY" "$SPEC_REVISION"
initialize_store ticket "$workspace/.ticket"
initialize_store spec "$workspace/.spec"

if ! "$dry_run"; then
    cat <<EOF
Installed workflow-tools CLI bundle: $install_root/bin
Initialized consumer workspace: $workspace
EOF
fi