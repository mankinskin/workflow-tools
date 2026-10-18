# Make `mission --help` exit successfully

## Problem

The public Mission CLI prints usage for help requests but returns exit code 1 and prints `Error: Arguments("Usage: ...")`. This occurs for both the top-level help command and the `validate-preview` subcommand.

## Reproduction

```text
cargo run -p mission --features cli --bin mission -- --help
cargo run -p mission --features cli --bin mission -- validate-preview --help
```

Both commands print usage, followed by the `Arguments("Usage: ...")` error, and exit with status 1. The top-level behavior was reproduced through Cargo against the current workflow-tools checkout; the `validate-preview` subcommand failure was also reproduced during W1 execution and recorded in the [dossier evidence](../../../../../transcripts/13-10-2026_mission-api-problems/ARTIFACTS.md).

The Mission CLI propagates the shared transport-harness argument-parsing error. The shared parser maps Clap help-display errors to `HarnessError::Arguments`; the ticket CLI handles help as successful output. See [mission.rs](../../../../mission/crates/mission/src/bin/mission.rs), [transport-harness lib.rs](../../../../memory-kernel/crates/transport-harness/src/lib.rs), and [ticket CLI](../../../../ticket/src/bin/ticket-cli.rs).

## Expected behavior

Both `mission --help` and `mission validate-preview --help` should print usage and exit successfully. Invalid command-line arguments must continue to produce a nonzero exit status.

## Acceptance criteria

- `mission --help` prints usage and returns exit code 0.
- `mission validate-preview --help` prints usage and returns exit code 0.
- Invalid command-line arguments still return a nonzero exit code.
- Add an integration regression test in `mission/tests/mission_cli.rs` that launches the public binary and checks top-level help success, `validate-preview` help success, and invalid-argument failure.

## Scope

This ticket covers the two reproduced Mission CLI help failures and the invalid-argument regression only. It does not change Mission storage, the dossier migration contract, or nested-waypoint behavior, and it does not duplicate the separate migration/workflow issue tracked by [ticket c5d40e2a](../../../../../.workflow-tools/ticket/tickets/c5d40e2a-aaa8-4996-b7df-d281e6c75989/ticket.toml).

## Related contract

See the [Mission Domain spec](../../../../.workflow-tools/spec/specs/38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/spec.toml) for the adjacent Mission CLI and roadmap behavior contract.