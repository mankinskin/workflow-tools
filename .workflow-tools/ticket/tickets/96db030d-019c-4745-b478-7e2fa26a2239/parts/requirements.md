1. After approval, register distinct `mission-cli` and `mission-mcp` artifacts in the canonical installer registry; retain unrelated installer work, including `install/install.sh`.
2. Verify registry selection/catalog behavior and exact scoped installation: `install-ctl install --root <ROOT> <SELECTION>...`; for this pilot use root `/home/linus/.copilot/session-state/ae41c0ca-75fa-497e-b4ac-90e3fa04f6ec/files/mission-install-pilot-install` and selections `mission-cli mission-mcp`.
3. Verify installed `mission --help`. Verify installed `mission-mcp` only through stdio JSON-RPC in isolated workspace cwd `/home/linus/.copilot/session-state/ae41c0ca-75fa-497e-b4ac-90e3fa04f6ec/files/mission-install-pilot-workspace`: `initialize` → `notifications/initialized` → `tools/list` → `tools/call`, followed by process cleanup; do not use a help-based MCP check.
4. Installed direct operations must create, validate, publish, and get by the real stored ID/current revision, with persisted content and typed result evidence read-back and no external manifest/dossier input.

## Acceptance criteria

- Registry/catalog validation reports exactly both IDs with no duplicate or missing entry; `catalog --check` passes.
- `install-ctl` recognizes both selected IDs and exact scoped installation yields both binaries under the isolated root.
- Installed CLI help succeeds; MCP protocol flow returns structured success/error assertions and cleanup evidence.
- Installed direct-operation fixture reads back actual ID, revision, content, validation/publication state, and result evidence; any failure blocks pilot publication.
- W2 evidence runs `cargo test -p install-ctl --test install_parity --test prebuilt_install`, catalog check, and the exact scoped install.
