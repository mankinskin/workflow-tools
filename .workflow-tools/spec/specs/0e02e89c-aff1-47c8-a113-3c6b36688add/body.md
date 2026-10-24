<!-- aligned-structure:v2 -->
# Direct Mission Authoring and Recording

## Target Code Location

- [Mission CLI](../../../../mission/crates/mission/src/bin/mission.rs) owns direct command parsing and workspace routing.
- [Mission MCP server](../../../../mission/crates/mission/src/bin/mission-mcp.rs) owns the stdio JSON-RPC tool surface.
- [Mission facade](../../../../mission/crates/mission/src/lib.rs) owns shared transport operations.
- [Direct authoring](../../../../mission/crates/mission/src/direct.rs) owns local waypoint edits.
- [Mission API](../../../../mission/crates/mission-api/src/lib.rs) owns persisted revisions and derived Ticket/child-Mission authority.
- [Operator results](../../../../mission/crates/mission-api/src/operator_results.rs) owns native evidence validation and rendering.
- [Real transport tests](../../../../mission/crates/mission/tests/direct_transports.rs) and [direct regressions](../../../../mission/crates/mission/tests/direct_authoring.rs) own direct-authoring coverage.

## Naming Conventions

Operations are `create_mission`, `get_mission`, `edit_mission_objective`, `add_waypoint`, `edit_waypoint`, `order_waypoints`, `record_progress`, `record_validation_result`, `validate_mission`, and `publish_mission`; CLI commands are kebab-case and MCP tools match. Existing-record writes use `expected_current_revision`; accepted revisions and waypoint IDs are stable. Criteria use `mission-direct-`. `get` is the compatibility read spelling; `get-mission`/`get_mission` are aliases.

## Requester Input

> teste eine mission anzulegen welche die Installation von mission-cli und mission-mcp via den install-ctl artefakten ermöglichen soll. Das ist ein ´Testlauf für die mission-api. Wenn Probleme bei der Verwendung der Tools auftreten, planen wir sie zu beheben und nutzen die klassischen Roadmaps. Das Ziel ist dass wir die vollständige Mission über die mission-mcp oder mission-cli tools betreiben können, ohne auf den "classic" transcript roadmap ordner zugreifen zu müssen.

## Reading Order

1. [Mission Domain](../38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) - shared invariants and component map.
2. [Mission usage](../../../../mission/README.md) - inline content, revision protection, ordering and transport recipes.
3. [Mission CLI](../../../../mission/crates/mission/src/bin/mission.rs) and [MCP](../../../../mission/crates/mission/src/bin/mission-mcp.rs) - transport providers.
4. [Mission API](../../../../mission/crates/mission-api/src/lib.rs) - record authority.
5. [Installer-distribution child spec](../9e714b78-ef31-46e5-8103-a944e7b1a278/body.md) - consumer of `mission-direct-create`, `mission-direct-readback`, and `mission-direct-publication`.
6. [Direct-authoring ticket](../../../ticket/tickets/57161cbd-58fc-44a3-9528-baa4cd1fceac/ticket.toml) - execution plan.
7. [Part 01](../../../../../transcripts/24-10-2026_mission-install-pilot/01-direct-authoring-repair.md) - approved planning boundary.

## Responsibility

This component directly creates and changes valid nonempty Missions through CLI and stdio MCP, records only operator-supplied local results, and validates, publishes, and reads stored records by returned ID. Consumers can rely on actual accepted ID/revision/content/evidence without a dossier or external manifest input.

## Interfaces And Dependencies

CLI and MCP use the same facade/domain operations and equivalent success/diagnostic shapes. Create accepts title, nonblank objective, and the complete ordered nonempty waypoint set atomically. Later operations use only stored ID/current expected revision. Derived Ticket/child-Mission state remains authoritative where applicable.

## Behavior

`create_mission` either creates one valid record at revision 1 with stable waypoint IDs or creates nothing. Objective/waypoint writes require `expected_current_revision`; unknown IDs, stale revisions, blank/invalid content, invalid evidence, or invalid order/dependencies leave accepted data unchanged. Result recording requires actual outcome, operator command, and evidence pointer and never runs that command. `validate_mission`, `publish_mission`, and `get_mission` are required stored-ID operations with no dossier/manifest input; read-back returns actual accepted ID, revision, content, validation/publication state, and evidence.

MCP validation starts with an explicitly selected isolated workspace cwd, then performs JSON-RPC `initialize` -> `notifications/initialized` -> `tools/list` -> `tools/call` with structured assertions and process cleanup. The MCP executable has no help-based validation surface.

## Boundaries And Failure Cases

Native bundle compatibility remains covered, but bundle/dossier/external-JSON input is not direct-authoring evidence. No list/search, deletion, scheduling, command/test/install execution, agent launch, or pilot Mission creation is required by this component.

## Provider/Consumer Contract

Installer distribution consumes `mission-direct-create`, `mission-direct-readback`, and `mission-direct-publication`; installer distribution cannot redefine revision protection or derived-state authority.

## Examples

An operator creates one Mission with ordered `install`, `verify-cli-mcp`, and `record-evidence` waypoints. The returned ID `M`, revision 1, and waypoint IDs are retained. The operator separately installs, verifies CLI/MCP, and records actual outcomes, commands, and evidence pointers. A stale write is unchanged; validate, publish, and `get_mission(M)` read back accepted content/evidence. No operation repairs authoring or launches installation.

## Evidence

Positions: the linked CLI, MCP, direct authoring module, native result validator and renderer are implemented. [Mission usage](../../../../mission/README.md) documents the complete operation contract.

Guard: [mission-w1-direct-authoring](../../../test/specs/mission-w1-direct-authoring.json). [Passing execution ae41c0ca-w1-green-verified](../../../test/executions/ae41c0ca-w1-green-verified.json) records the actual full targets: Mission 34 tests, Mission API 26 tests, cargo check, fmt check, and a persistent real CLI/MCP fixture. Tests read generated bytes and accepted records; runtime transport evidence was also read independently.

| Criterion | Observed evidence |
| --- | --- |
| `mission-direct-create` | CLI and MCP create full waypoint content; blank/missing fields create no record. |
| `mission-direct-content-edit` | Both adapters edit objective/content; add/edit flags and atomic order/dependency reversal preserve IDs. |
| `mission-direct-revision` | Stale writes, invalid order/maps, malformed content and unknown IDs preserve accepted data. |
| `mission-direct-recording` | Native validation rejects invalid result references/outcomes/commands/pointers; actual result fields were read back. |
| `mission-direct-readback` | Persistent fixture ID `2189b7b0-f28c-492f-a187-4b39bb1adfa4`, revision 5, local/derived state completed. |
| `mission-direct-publication` | Stored-ID publication writes real projections including results; get reports current publication and detects changed generated bytes as drift. |
| `mission-direct-parity` | Four real subprocess tests cover both CLI and initialized MCP stdio; owned MCP processes are terminated and waited. |
| `mission-direct-compatibility` | Full API and Mission targets retain native bundles, migration and derived Ticket/child-state tests. |

Genuine red evidence: [three direct assertion failures](../../../test/executions/ae41c0ca-w1-red-direct.json), [native result-validation failure](../../../test/executions/ae41c0ca-w1-red-api-results.json), [CLI ordering failure](../../../test/executions/ae41c0ca-w1-red-order.json), and [publication-state failure](../../../test/executions/ae41c0ca-w1-red-publication.json). Compilation/argument setup failures and zero-test runs are not red-test evidence.

External local runtime artifact, observed 2026-10-24: `/home/linus/.copilot/session-state/ae41c0ca-75fa-497e-b4ac-90e3fa04f6ec/files/mission-w1-transport-workspace/.workflow-tools/mission/missions/2189b7b0-f28c-492f-a187-4b39bb1adfa4/accepted.json`. Actual operator result: `outcome=passed`, command is the real `mission --workspace <fixture> validate-mission 2189b7b0-f28c-492f-a187-4b39bb1adfa4` invocation, and evidence points to the fixture's persistent `transport-evidence.json`. The generated projection contains the same command and evidence pointer. The fixture is not the installation pilot.

The validation CLI's automatic cross-store spec verifier remains degraded: the tool initially confused ValidationSpec and product-spec IDs and cannot resolve this approved product spec while same-root `spec get` succeeds. Execution links were corrected through `test record` and read back. No computed verified state is claimed; passing command evidence and the explicit guard are the evidence.

## Scope

This component excludes installer registration, the real installation pilot, stable installation changes, dossier migration as direct-authoring fulfillment, and autonomous execution.
