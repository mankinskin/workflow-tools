<!-- aligned-structure:v2 -->
# Mission CLI and MCP Installer Distribution

## Target Code Location

- [Installer registry](../../../../install/artifacts.toml) owns artifact declarations.
- [Installer control script](../../../../install/bin/install-ctl) owns selection, scoped installation, and catalog verification.
- [Mission CLI](../../../../mission/crates/mission/src/bin/mission.rs) and [Mission MCP](../../../../mission/crates/mission/src/bin/mission-mcp.rs) are registered outputs.
- [Installer registry contract](../97322185-2bda-4f25-85f7-f975a0b3fbad/body.md) owns adjacent registry rules.

## Naming Conventions

Artifact IDs are `mission-cli` and `mission-mcp`; executable names are `mission` and `mission-mcp`. Criteria use `mission-install-`. Scoped installation is `install-ctl install --root <ROOT> <SELECTION>...`.

## Requester Input

> teste eine mission anzulegen welche die Installation von mission-cli und mission-mcp via den install-ctl artefakten ermöglichen soll. Das ist ein ´Testlauf für die mission-api. Wenn Probleme bei der Verwendung der Tools auftreten, planen wir sie zu beheben und nutzen die klassischen Roadmaps. Das Ziel ist dass wir die vollständige Mission über die mission-mcp oder mission-cli tools betreiben können, ohne auf den "classic" transcript roadmap ordner zugreifen zu müssen.

## Reading Order

1. [Mission Domain](../38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) — parent map.
2. [Direct-authoring child spec](../0e02e89c-aff1-47c8-a113-3c6b36688add/body.md) — consumed provider criteria.
3. [Installer registry contract](../97322185-2bda-4f25-85f7-f975a0b3fbad/body.md) — registry constraints.
4. [Adjacent installer planning ticket](../../../../../.workflow-tools/ticket/tickets/c7becdaa-6939-4ab9-a8a5-29fbf8921584/ticket.toml) — context only.
5. [Mission installer planning ticket](../../../ticket/tickets/96db030d-019c-4745-b478-7e2fa26a2239/ticket.toml) — execution plan.
6. [Part 02](../../../../../transcripts/24-10-2026_mission-install-pilot/02-installer-distribution-repair.md) — scoped plan.

## Responsibility

This component registers and verifies exactly two distinct Mission artifacts. If implemented, consumers can select/install them into an isolated root and prove installed CLI and stdio MCP expose direct authoring.

## Interfaces And Dependencies

It consumes `mission-direct-create`, `mission-direct-readback`, and `mission-direct-publication`, plus registry schema. It provides installed-binary discovery; Mission semantics, execution, and scheduling are out of scope.

## Behavior

After approval one valid nonduplicate row exists per ID. Catalog checking and `install-ctl install --root /home/linus/.copilot/session-state/ae41c0ca-75fa-497e-b4ac-90e3fa04f6ec/files/mission-install-pilot-install mission-cli mission-mcp` select/install only those artifacts. Installed `mission --help` verifies CLI. The MCP executable has no help-based validation surface: run it with isolated workspace cwd `/home/linus/.copilot/session-state/ae41c0ca-75fa-497e-b4ac-90e3fa04f6ec/files/mission-install-pilot-workspace`, send `initialize` → `notifications/initialized` → `tools/list` → `tools/call`, assert structured outcomes, then clean up. Installed direct operations create/validate/publish/get by real stored ID/revision and read back persisted content/evidence with no manifest input.

## Boundaries And Failure Cases

No build/install/registry mutation occurs during planning. Missing/duplicate/invalid rows, catalog failure, absent binary, failed CLI help, failed MCP flow/cleanup, or failed stored-ID read-back blocks pilot publication. This component does not own authoring semantics, unrelated installer lifecycle work, execution, or scheduling. Current position is draft pending review; later supported approval/readiness is not prohibited.

## Provider/Consumer Contract

This consumer depends on `mission-direct-create`, `mission-direct-readback`, and `mission-direct-publication`; it provides `mission-install-registry` and `mission-install-verification` to the later pilot.

## Examples

After approval, the operator catalog-checks, installs both artifacts into the named root, verifies installed CLI and the MCP initialize/list/call flow with cleanup, and records install/verification evidence through Mission. Missing MCP stops the pilot; it is not replaced by repair-authoring or bundle import.

## Evidence

Positions: [Installer registry](../../../../install/artifacts.toml) — not-implemented for Mission rows; [install-ctl](../../../../install/bin/install-ctl) — implemented registry consumer; Mission binaries — partial.

| Criterion | Required evidence |
| --- | --- |
| `mission-install-registry` | Catalog identifies exactly the two valid nonduplicate Mission rows. |
| `mission-install-selection` | Exact scoped installation recognizes/installs both selected IDs. |
| `mission-install-cli` | Installed `mission --help` succeeds from isolated root. |
| `mission-install-mcp` | Installed MCP initialize → initialized → tools/list → tools/call passes structured assertions in isolated cwd and is cleaned up. |
| `mission-install-pilot-readback` | Installed direct create/validate/publish/get uses real stored ID/revision and returns content/evidence. |

Planned evidence after approval from `/home/linus/git/meta-workspace/workflow-tools`: `cargo test -p install-ctl --test install_parity --test prebuilt_install`; `install/bin/install-ctl catalog --check`; exact scoped install above; catalog output, root paths, MCP transcript/cleanup, and stored-ID read-back. No validation ran in this correction.

## Scope

Contract non-goals: implementation here, pre-approval registry mutation/installation, unrelated installer repair, authoring-semantic changes, and pilot creation/publication. State is draft pending review; supported later approval is not banned.
