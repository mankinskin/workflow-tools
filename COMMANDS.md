# Command & Hook Registry

Generated from `install/artifacts.toml` (schema version 1). Do not edit by hand.

## install-ctl

- Category: Cli
- Kind: RustBinary
- Source: `install/install-ctl`
- Owner: workflow-tools
- Safety: ApprovalRequired
- Bin: `install-ctl`
- Lifecycle: Install, Inspect

## mcp-toolmon

- Category: Mcp
- Kind: RustBinary
- Source: `session/crates/mcp-toolmon`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `mcp-toolmon`
- Lifecycle: Install, Inspect

## session-capture-hook

- Category: Misc
- Kind: RustBinary
- Source: `session`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `session-capture-hook`
- Lifecycle: Install, Inspect

## session-record-merge

- Category: Misc
- Kind: RustBinary
- Source: `session/crates/session-record-merge`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `session-record-merge`
- Lifecycle: Install, Inspect

## ticket

- Category: Cli
- Kind: RustBinary
- Source: `ticket`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `ticket`
- Lifecycle: Install, Inspect

## ticket-record-merge

- Category: Misc
- Kind: RustBinary
- Source: `ticket`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `ticket-record-merge`
- Lifecycle: Install, Inspect

## spec-cli

- Category: Cli
- Kind: RustBinary
- Source: `spec`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `spec`
- Lifecycle: Install, Inspect

## audit-cli

- Category: Cli
- Kind: RustBinary
- Source: `audit/crates/audit-cli`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `audit`
- Lifecycle: Install, Inspect

## rule-cli

- Category: Cli
- Kind: RustBinary
- Source: `rule`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `rule`
- Lifecycle: Install, Inspect

## feedback-cli

- Category: Cli
- Kind: RustBinary
- Source: `feedback/crates/feedback-cli`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `feedback`
- Lifecycle: Install, Inspect

## session-cli

- Category: Cli
- Kind: RustBinary
- Source: `session`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `session`
- Lifecycle: Install, Inspect

## peek-cli

- Category: Cli
- Kind: RustBinary
- Source: `peek/crates/peek-cli`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `peek`
- Lifecycle: Install, Inspect

## test-cli

- Category: Cli
- Kind: RustBinary
- Source: `test/crates/test-cli`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `test`
- Lifecycle: Install, Inspect

## ticket-mcp

- Category: Mcp
- Kind: RustBinary
- Source: `ticket`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `ticket-mcp`
- Lifecycle: Install, Inspect

## spec-mcp

- Category: Mcp
- Kind: RustBinary
- Source: `spec`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `spec-mcp`
- Lifecycle: Install, Inspect

## test-mcp

- Category: Mcp
- Kind: RustBinary
- Source: `test/crates/test-mcp`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `test-mcp`
- Lifecycle: Install, Inspect

## feedback-mcp

- Category: Mcp
- Kind: RustBinary
- Source: `feedback/crates/feedback-mcp`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `feedback-mcp`
- Lifecycle: Install, Inspect

## session-mcp

- Category: Mcp
- Kind: RustBinary
- Source: `session`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `session-mcp`
- Lifecycle: Install, Inspect

## peek-mcp

- Category: Mcp
- Kind: RustBinary
- Source: `peek/crates/peek-mcp`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `peek-mcp`
- Lifecycle: Install, Inspect

## rule-mcp

- Category: Mcp
- Kind: RustBinary
- Source: `rule`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `rule-mcp`
- Lifecycle: Install, Inspect

## audit-mcp

- Category: Mcp
- Kind: RustBinary
- Source: `audit/crates/audit-mcp`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `audit-mcp`
- Lifecycle: Install, Inspect

## doc-viewer

- Category: Service
- Kind: RustBinary
- Source: `doc/crates/doc-viewer`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `doc-viewer`
- Lifecycle: Install, Start, Stop, Restart, Uninstall, Inspect

## log-viewer

- Category: Service
- Kind: RustBinary
- Source: `log/crates/log-viewer`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `log-viewer`
- Lifecycle: Install, Start, Stop, Restart, Uninstall, Inspect

## spec-viewer

- Category: Service
- Kind: RustBinary
- Source: `spec/spec-viewer`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `spec-viewer`
- Lifecycle: Install, Start, Stop, Restart, Uninstall, Inspect

## ticket-viewer

- Category: Service
- Kind: RustBinary
- Source: `ticket/ticket-viewer`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `ticket-viewer`
- Lifecycle: Install, Start, Stop, Restart, Uninstall, Inspect

## worktree-ctl

- Category: Misc
- Kind: RustBinary
- Source: `session/crates/worktree-ctl`
- Owner: tooling
- Safety: Safe
- Bin: `worktree-ctl`
- Lifecycle: Inspect

## hook-copilot-capture

- Category: Hook
- Kind: Hook
- Source: `session`
- Owner: tooling
- Safety: Safe
- Bin: `session-capture-hook --from-hook-stdin`
- Lifecycle: Inspect
