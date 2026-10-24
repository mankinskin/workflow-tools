<!-- aligned-structure:v2 -->
# Mission CLI and MCP Installer Distribution

## Target Code Location

- [Installer registry](../../../../install/artifacts.toml) owns artifact declarations.
- [Installer control script](../../../../install/bin/install-ctl) owns selection, scoped installation and catalog verification.
- [Mission CLI](../../../../mission/crates/mission/src/bin/mission.rs) and [Mission MCP](../../../../mission/crates/mission/src/bin/mission-mcp.rs) are registered outputs.
- [Focused registry/selection tests](../../../../install/install-ctl/tests/mission_artifacts.rs) cover both artifact mappings and separate/combined selection.

## Naming Conventions

Artifact IDs are `mission-cli` and `mission-mcp`; executables are `mission` and
`mission-mcp`. Criteria use `mission-install-`. Scoped installation is
`install-ctl install --root <ROOT> <SELECTION>...`.

## Requester Input

> teste eine mission anzulegen welche die Installation von mission-cli und mission-mcp via den install-ctl artefakten ermöglichen soll. Das ist ein ´Testlauf für die mission-api. Wenn Probleme bei der Verwendung der Tools auftreten, planen wir sie zu beheben und nutzen die klassischen Roadmaps. Das Ziel ist dass wir die vollständige Mission über die mission-mcp oder mission-cli tools betreiben können, ohne auf den "classic" transcript roadmap ordner zugreifen zu müssen.

The requester subsequently selected `same-mission`: existing bootstrap tools
create the installation Mission before installation; installed tools manage
that same Mission. A second installed-create fixture is not required.

## Reading Order

1. [Mission Domain](../38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) - parent map.
2. [Direct-authoring provider](../0e02e89c-aff1-47c8-a113-3c6b36688add/body.md) - native authoring/read-back/publication.
3. [Installer registry contract](../97322185-2bda-4f25-85f7-f975a0b3fbad/body.md) - registry constraints.
4. [Installer ticket](../../../ticket/tickets/96db030d-019c-4745-b478-7e2fa26a2239/ticket.toml) - authoritative execution state and acceptance amendment.

## Responsibility

Register and verify exactly two distinct Mission artifacts. Dependents can
select/install both into an isolated root and use installed CLI/MCP to manage
native Mission content, progress, evidence and publication by stored ID.

## Interfaces And Dependencies

Consume `mission-direct-create`, `mission-direct-readback` and
`mission-direct-publication` from the provider. Bootstrap creation precedes
installation; installed management uses the same workspace and Mission UUID.
The installer does not implement Mission semantics or automatic execution.

## Behavior

One valid nonduplicate registry row exists per ID. `mission-cli` maps to bin
`mission`, feature `cli`; `mission-mcp` maps to bin `mission-mcp`, feature `mcp`;
both use source `mission/crates/mission`. Separate and combined selections are
recognized. Catalog checking succeeds and isolated installation yields both
executables. Installed CLI help succeeds.

Run installed MCP with the pilot workspace as cwd: initialize, initialized
notification, tools/list, tools/call. Check structured errors and `isError`,
cross-read CLI/MCP edits, reject a stale revision without changing accepted
data, and terminate/reap the owned server. Installed validation/publication/get
and generated checks use the real stored UUID/revision without manifest input.

## Boundaries And Failure Cases

Missing/duplicate/invalid rows, failed catalog/install, missing binaries, failed
protocol/cleanup, stale-write acceptance or missing result read-back block
completion. Never overwrite stable/home installations or substitute import for
native operations. Unexpected tool failure retains partial state and returns
to classic repair planning. Tests need no Python or interpreter-denial layer;
reuse the completed W3 single run rather than repeat a validation matrix.

## Provider/Consumer Contract

The direct-authoring provider owns create/read-back/publication semantics.
This consumer provides `mission-install-registry`, `mission-install-selection`,
`mission-install-cli`, `mission-install-mcp` and `mission-install-pilot-readback`.
The installer ticket owns distribution completion; Mission-local state tracks
the broader trial without overriding ticket lifecycle.

## Examples

Create an installation Mission using bootstrap CLI. From its stored commands,
install `mission-cli mission-mcp` once into the isolated root. Switch to the
installed CLI/MCP while keeping the same workspace/UUID; record installation
and protocol evidence, then validate/publish and inspect generated output.

## Evidence

Observed 2026-10-24, WSL2 Ubuntu/Linux. Windows/macOS runtime proof is not claimed.

| code_ref | Position | Observed evidence |
| --- | --- | --- |
| [Registry](../../../../install/artifacts.toml) | implemented | Exactly two Mission entries; list/catalog check exit 0. |
| [Focused tests](../../../../install/install-ctl/tests/mission_artifacts.rs) | implemented | Existing genuine red/green registry/selection proof retained; current W3 run includes 2 passing tests. |
| [Installer](../../../../install/bin/install-ctl) | implemented | Actual isolated install exit 0; both binaries present and reported matching freshly built outputs. |
| [Mission CLI/MCP](../../../../mission/crates/mission/src/bin/mission.rs) | implemented | Installed cross-transport management, unchanged order and stale-rejection/no-change checks pass. |

Guard: [real installation/transport ValidationSpec](../../../test/specs/mission-install-real-pilot.json).
Execution: `mission-install-real-pilot-20261024`, with actual log/JSON source
pointers in the owning test store. The earlier W2 failure notes are historical;
W3's single terminal run recorded 359 passed, zero failed, one existing ignored.

Pilot UUID `e6e9c515-4f2c-40eb-968d-c3e1aa345dc2` was created before installation.
Installed publication read-back at revision 15 returns status `ok`, empty
diagnostics and a matching generated projection. Final pilot completion is
recorded separately through that Mission.

| Criterion | Required evidence |
| --- | --- |
| `mission-install-registry` | Exactly two valid nonduplicate catalog entries. |
| `mission-install-selection` | Scoped install recognizes both IDs and produces both binaries. |
| `mission-install-cli` | Installed CLI help and stored-ID operations succeed. |
| `mission-install-mcp` | Real initialize/initialized/list/call, cross-transport edits and server cleanup succeed. |
| `mission-install-pilot-readback` | Installed tools manage the existing bootstrap-created Mission, preserve stored content/results, validate/publish/get and check generated output by UUID/revision. |

## Scope

Contract remains approved; distribution implementation and actual installed
proof are recorded above. Computed verification is owned by validation guards,
not a manually assigned verified state. Non-goals: unrelated installer repair,
automatic execution, a second fixture Mission or new authoring semantics.
