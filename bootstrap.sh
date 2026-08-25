#!/usr/bin/env bash
set -euo pipefail

TICKET_REPOSITORY="https://github.com/mankinskin/ticket"
TICKET_REVISION="bd826dc9ddaa8dbf44e0e0d371618309c67409a6"
SPEC_REPOSITORY="https://github.com/mankinskin/spec"
SPEC_REVISION="f9f661f0ef876b7298fdaa976bd397531df27a01"

usage() {
    cat <<'EOF'
Usage: bootstrap.sh --root <install-root> --workspace <consumer-root> [--dry-run]

Installs the minimal workflow-tools CLI bundle into <install-root>/bin and
initializes ticket and spec stores only in the selected consumer workspace.
EOF
}

install_root=""
workspace=""
dry_run=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        --root)
            install_root="${2:-}"
            shift 2
            ;;
        --workspace)
            workspace="${2:-}"
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
        *)
            usage >&2
            exit 2
            ;;
    esac
done

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