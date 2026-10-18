<!-- aligned-structure:v2 -->
# Mission Domain

## Motivation

The workflow-tools repository needs a durable, queryable mission entity that replaces transcript dossiers as the shared planning object across sessions. The mission entity aggregates the objective, a fully ordered roadmap, requirements, linked workflow entities, artifacts, and attached sessions without copying session-local workflow state. Structured JSON authoring must let a user collect and validate these inputs before publishing a predictable Markdown view.

If this specification is implemented, dependents can rely on UUID-addressed mission records as the accepted source of roadmap state; strict validation and deterministic rendering will produce contiguous, uniquely executable roadmap documents while preserving stable waypoint identity and historical references across revisions.

## Target Code Location

- [workflow-tools workspace manifest](../../../../Cargo.toml) owns the future `mission-api` and `mission` workspace members.
- [Session pin routing](../../../../session/crates/session-api/src/store_routing_types.rs) owns the legal URI store enumeration that will gain `mission`.
- [Domain crate contract](../../../../../context-engine/WORKFLOW_TOOLS_DOMAIN_CRATE_CONTRACT.md) governs the public mission facade and transport boundaries.
- [Roadmap authoring rule](../../../../.agents/instructions/workflow/roadmap-authoring.instructions.md) governs the narrowly scoped numbering exception for mission-compiled roadmaps.

## Naming Conventions

- The internal crate is `mission-api`; the public facade crate is `mission`.
- A mission identifier is a UUID and its canonical reference is `ce://<workspace>/mission/<mission-id>`.
- A persisted mission is a UUID directory under `.workflow-tools/mission/missions/<mission-id>/` containing the accepted manifest and revision history.
- Every waypoint has an immutable mission-local `waypoint_id`; dependencies and current-waypoint selection reference this stable ID, never a display label.
- Rendered labels are contiguous `W1` through `Wn` within one published revision only. A revision-qualified label-to-waypoint map preserves historical references.
- Mission acceptance criteria use stable identifiers with the `mission-` prefix.

## Requester Input

> Aufgaben und Artefakte und Anforderungen und alle möglichen Entitäten oder auch Agentensitzungen ... oder eben kurz Missionen zu speichern und zu verwalten.

## Reading Order

1. [Current mission roadmap](../../../../../transcripts/01-10-2026_roadmap-state-integration/ROADMAP.md) - strictly serial W1-W9 execution route.
2. [Structured-roadmap architecture](../../../../../transcripts/01-10-2026_roadmap-state-integration/STRUCTURED-ROADMAP.md) - JSON authoring, validation, rendering, and identity decisions.
3. [Mission research inventory](../../../../../transcripts/01-10-2026_roadmap-state-integration/ARTIFACTS.md) - store discovery and established architecture.
4. [Workflow-tools domain crate contract](../../../../../context-engine/WORKFLOW_TOOLS_DOMAIN_CRATE_CONTRACT.md) - required public/private crate boundary.
5. [Session workflow guidance](../../../../session/.agents/instructions/session/session-workflow.instructions.md) - per-session graph boundary that missions must not duplicate.
6. [Session pin routing](../../../../session/crates/session-api/src/store_routing_types.rs) - extension point for mission attachment URNs.
7. [Roadmap authoring rule](../../../../.agents/instructions/workflow/roadmap-authoring.instructions.md) - versioned mission-label exception and baseline authoring constraints.
8. [mission-api implementation ticket](../../../ticket/tickets/1486c2f9-301e-4709-a8d0-cae3c4e6f5d4/ticket.toml) - persistence, model, validation, and renderer work.
9. [mission transport implementation ticket](../../../ticket/tickets/c616418e-8c27-4954-975e-490fc22c408f/ticket.toml) - CLI/MCP import and publication work.
10. [session attachment ticket](../../../ticket/tickets/465f31b9-2329-4918-930c-0dfc3fc15b5c/ticket.toml) - session reference boundary.
11. [dossier migration ticket](../../../ticket/tickets/dc314dc3-40e9-4e6e-93db-5bbab48cf224/ticket.toml) - legacy conversion and historical reference mapping.

## Responsibility

The mission domain owns durable cross-session planning records. A record captures mission identity, title, objective, lifecycle state, requirements, artifacts, ordered waypoints, waypoint dependencies, canonical references to tickets/specifications/validation records/sessions, and migration provenance. The mission model owns one explicit total execution order for its waypoints.

Mission JSON bundles are versioned authoring/interchange inputs, not a second runtime store. The accepted mission manifest is authoritative. `ROADMAP.md` and per-waypoint Part files are deterministic generated projections. Manual edits to generated files are detected as drift and are not imported implicitly.

The mission domain preserves foreign entities as canonical identifiers or URNs. It does not duplicate foreign payloads, histories, state machines, or the per-session workflow graph.

## Interfaces And Dependencies

`mission-api` owns typed versioned bundle/model types, parsing, validation, normalization, mission persistence, and a pure Markdown renderer. The public `mission` facade re-exports that API and provides feature-gated CLI and MCP operations through `transport-harness`, following the [domain crate contract](../../../../../context-engine/WORKFLOW_TOOLS_DOMAIN_CRATE_CONTRACT.md). CLI and MCP must use the same domain functions and return equivalent structured diagnostics.

The session domain remains the owner of session manifests and `session_workflow_*`; sessions attach to a shared mission by URN. Ticket, specification, test, feedback, and artifact domains remain owners of their referenced entities. Ticket-backed waypoint state is read from the ticket authority and included in a declared snapshot, never duplicated as mutable mission status.

## Behavior

### Structured authoring bundle

A manifest declares `schema_version`, monotonic `revision`, mission metadata, an explicit manifest-ordered list of waypoint fragment paths, one `execution_order` array of stable waypoint IDs, requirements, artifacts, validation gates, and notes. Each waypoint fragment declares stable `waypoint_id`, title, prerequisite IDs, mode, scope or canonical ticket reference, prompt, artifacts, requirement IDs, non-goals, validation commands, commit checkpoint, and Part Markdown. The W3-aligned JSON Schema is strict: unknown keys, duplicate paths/IDs, missing required values, unsupported versions, absolute paths, traversal, and symlink escapes are rejected.

The manifest's `execution_order` must contain every waypoint ID exactly once. Every declared prerequisite must exist and appear before its dependent. The order is total: each waypoint after the first is also blocked by its immediate predecessor's checkpoint. The validator rejects partial orders, cycles, forward dependencies, omitted waypoints, and ticket dependency graphs inconsistent with this serial chain. It does not schedule agents or execute validation commands.

### Revision identity and numbering

Stable waypoint IDs are the only dependency/current-waypoint identities. For every accepted revision, the renderer numbers the declared execution order contiguously from `W1` through `Wn`. Inserting, removing, or reordering an item updates every label and current-document reference in that new revision. A revision-qualified mapping records each historical label and stable ID; an unqualified old `W<n>` is never silently resolved against another revision. Existing dossier `W<n>` labels are migration provenance, not immutable identities.

The mission compiler uses a scoped exception to the ordinary roadmap rule that labels are never reused across revisions. The exception applies only to explicitly versioned, mission-owned generated roadmaps; it preserves the previous rendered revision and the label-to-ID mapping. It does not renumber historical snapshots or ordinary manually authored roadmaps.

### Validation and diagnostics

Validation is read-only and returns stable diagnostic codes with source path, JSON Pointer, waypoint ID when applicable, and actionable text. Before import or publication, it checks:

- schema/version, required values, safe bundle-relative paths, unique mission/waypoint/reference IDs, and complete fragment collection;
- complete total order, unique stable IDs, known prerequisites, prerequisite precedence, immediate-predecessor serial edges, acyclicity, and consistency with ticket-backed dependencies;
- scope/mode constraints, one Part per waypoint, requirement ownership or explicit deferral, and gate variables with exactly one prior producer;
- canonical ticket/spec references, resolution from explicitly selected stores, ticket status ownership, and a declared external-status snapshot;
- output sections, escaped pipes, resolvable links relative to each generated file, unique headings, Part navigation, contiguous labels, and deterministic output digest/renderer version.

Missing external references or status resolution failures block import/publication and name the unresolved canonical reference. Automated validation proves declared structural and semantic properties; it does not prove prose truth, estimate quality, complete intent preservation, or that external commands pass.

### Import, rendering, and publication

The facade provides validate/collect preview, import with an expected current revision, render preview, publish, and check-generated operations over CLI and MCP. Validate and preview do not mutate the store. Import rejects stale revisions. Published `ROADMAP.md` and Part files live together under `.workflow-tools/mission/missions/<uuid>/generated/`. Publication renders every output before replacement and preserves the prior accepted manifest and generated file set if validation or publication fails. Check-generated is read-only: it recomputes the projection for the declared mission/status snapshot and reports any byte drift without rewriting documents or executing gates.

The same accepted model and snapshot must render byte-identically. The published output includes provenance identifying mission ID, accepted revision, external-status snapshot, and renderer version. Every successful mutation is read back from the canonical store.

### Cross-session attachment and legacy migration

The first release extends legal session pin-URN stores with `mission`. Any session may attach the same mission URN without embedding its record or transferring ownership. Migration inventories existing dossier directories at run time, imports only each dossier's current `ROADMAP.md` as mission revision 1, and records the source path and each excluded `ROADMAP.vN.md` snapshot in provenance notes. Excluded snapshots remain unchanged in the source dossier and do not create historical mission revisions or label mappings. Migration creates or maps each mission idempotently, reports migrated/skipped/blocked records, offers dry-run/recovery, and never deletes source dossiers. When a legacy waypoint has only a positional W-label and title, migration derives a deterministic stable ID from the title; unsafe or colliding derived IDs block that dossier rather than being guessed.

## Nested Mission Waypoints

A waypoint may reference a child mission. Classic dossier roadmaps and
mission-generated roadmaps support these references recursively, within the
same workspace. A mission-generated roadmap remains a projection of its
accepted bundle and must not be hand-edited.

Before accepting or publishing a mission that references child missions,
resolve each reference from the explicitly selected workspace's mission
store. Reject missing or unaccepted targets, cross-workspace references,
missions with no waypoints, and direct or transitive cycles in the reachable
mission graph. Validate recursively. On failure, preserve the previously
accepted parent record and generated files.

An accepted mission must contain at least one waypoint. Its status is derived:
the mission is `completed` only when every waypoint in its accepted
`execution_order` is `completed`; otherwise, the first non-completed waypoint
in that order supplies the mission's state. Ticket-backed waypoint state
continues to be read from the ticket store. A parent nested-mission waypoint
reflects the referenced child's derived state, recursively.

Nested missions do not introduce parallel or simultaneous execution. Preserve
the total serial order and immediate-predecessor dependency rule. Do not add a
separate dependency when the serial edge already supplies the required
ordering. This extension does not add cross-workspace traversal,
session-based transcript ingestion, automatic mission creation, or
multi-agent scheduling.

## Boundaries And Failure Cases

- The first release has no HTTP transport, viewer, agent assignment, execution lease, or multi-agent scheduling.
- A mission roadmap is strictly serial even when functional domains could otherwise execute independently; a later approved feature may define parallel execution.
- Invalid input, unresolved references, inconsistent ticket dependencies, stale revision, or publication failure leaves the previous accepted mission and generated documents intact.
- Session-local graphs remain session-local. The mission domain does not move, mirror, or mutate `session_workflow_*` state.
- Migration preserves ambiguous source records and reports them blocked rather than guessing or deleting data.
- External status is a time/revision-scoped snapshot, not a promise that ticket state remains fresh indefinitely.

## Provider/Consumer Contract

The mission domain provides versioned, strictly validated roadmap models and deterministic generated documents to CLI/MCP clients, sessions, roadmap guidance, and migration tooling. Consumers rely on `mission-roadmap-bundle`, `mission-roadmap-order`, `mission-roadmap-numbering`, `mission-roadmap-render`, `mission-roadmap-publication`, and `mission-roadmap-migration` criteria. The mission domain consumes only stable IDs/URNs and explicit status snapshots from ticket, specification, test, feedback, artifact, and session domains.

## Examples

A revision-1 manifest collects fragments whose stable IDs are `mission-store`, `mission-transports`, and `mission-migration`; its `execution_order` lists those IDs in that order. The renderer emits W1/W2/W3 and enforces the same serial chain. In revision 2 a prerequisite is inserted before `mission-store`: the new Markdown labels are recomputed contiguously, but existing dependencies still identify their stable waypoint IDs. The accepted revision retains a mapping from each old W-label to its stable ID, and the prior rendered files remain as the revision-1 snapshot.

## Evidence

Positions:

- [Mission facade](../../../../mission/crates/mission/src/lib.rs) - implemented;
  external ticket snapshots are resolved read-only for import, publication,
  get, rendering and drift checks, including recursive child missions.
- [mission-api bundle/model/validator/store/renderer](../../../../mission/crates/mission-api/src/lib.rs) -
  implemented, including same-workspace nested validation and derived state.
- [Mission CLI](../../../../mission/crates/mission/src/bin/mission.rs) -
  implemented; help returns success and invalid arguments fail.
- `session-api` mission URN routing - not-implemented.
- [Dossier migration](../../../../mission/crates/mission/src/migration.rs) -
  implemented; nested references, ordered gates and authored Parts survive migration.

Related planning work:

- [Ticket c5d40e2a](../../../../../.workflow-tools/ticket/tickets/c5d40e2a-aaa8-4996-b7df-d281e6c75989/ticket.toml) owns the migration-heading and classic-execution handoff criteria.
- [Ticket 7d5521c0](../../../ticket/tickets/7d5521c0-1e2c-40c1-a555-8136568707cf/ticket.toml) owns mission CLI help exit behavior.
- [Ticket 97ad0658](../../../../.workflow-tools/ticket/tickets/97ad0658-3ca7-40e4-b7c7-aec27ea03b96/ticket.toml) owns recursive nested mission waypoints.

The governing rule is [Roadmap State And Waypoint Ownership](../../../../.agents/instructions/workflow/roadmap-authoring.instructions.md). This specification remains coming-soon until the acceptance guards below pass.

Planned executable guards, to be registered in the test store before implementation:

- `cargo test -p mission-api --test roadmap_bundle` covers `manifest_rejects_unknown_field`, `manifest_rejects_unsafe_fragment_path`, `order_requires_each_waypoint_once`, `order_requires_prerequisites_before_dependents`, `order_requires_immediate_predecessor_chain`, `revision_numbers_waypoints_contiguously`, `stable_waypoint_ids_survive_reorder`, and `failed_import_preserves_accepted_revision`.
- `cargo test -p mission-api --test roadmap_render` covers `render_is_byte_deterministic`, `render_escapes_pipes_and_resolves_links`, `render_parts_follow_execution_order`, and `revision_mapping_preserves_old_label_identity`.
- `cargo test -p mission --test roadmap_commands` covers `validate_preview_does_not_mutate`, `cli_and_mcp_return_equivalent_diagnostics`, `publish_failure_preserves_previous_file_set`, `check_generated_detects_document_drift`, and `stale_revision_import_is_rejected`.
- `cargo test -p mission --test dossier_migration` covers `migration_is_idempotent`, `migration_imports_current_roadmap_only_and_preserves_history_sources`, and `ambiguous_source_is_reported_blocked`.

Each named integration-test target must execute at least one test; CI must fail if a required target or named test is absent. Read back the accepted manifest and generated ROADMAP/Part files in fixtures; a successful exit status alone is insufficient.

### Final nested-mission review, 2026-10-19

All eight acceptance criteria of
[ticket 97ad0658](../../../ticket/tickets/97ad0658-3ca7-40e4-b7c7-aec27ea03b96/ticket.toml)
were reviewed against implementation and observed outputs. The review found and
fixed a snapshot-state defect: a local waypoint with omitted state was skipped,
allowing a later state or a false completion to win. Omitted local state now
remains pending, including when propagated through a parent.

The four predeclared guards passed and their executions were read back from the
meta-workspace's canonical test store:

- [Bundle execution](../../../../../.workflow-tools/test/executions/exec-mission-nested-bundle-20261019.json):
  14 tests, including missing/empty children and publication preservation.
- [Render execution](../../../../../.workflow-tools/test/executions/exec-mission-nested-render-20261019.json):
  8 tests, including the failing-before/passing-after pending-state regression.
- [Command execution](../../../../../.workflow-tools/test/executions/exec-mission-nested-commands-20261019.json):
  10 tests, covering recursive external snapshots and read-only drift.
- [Migration execution](../../../../../.workflow-tools/test/executions/exec-mission-nested-migration-20261019.json):
  7 tests, covering classic migration and accepted/generated artifact read-back.

The combined mission package suite with CLI and MCP features passed 46 tests;
the classic route selector passed 3 tests. Both public help commands returned
exit 0. Formatting and package Clippy (`--no-deps -- -D warnings`) passed.
Both repository audit hooks passed with zero broken/blocking Markdown links.
Audit coverage collection remains unavailable; existing size/complexity
advisories and a ticket-api dependency dead-code warning are not evidence of
a clean global audit or a full dependency-lint pass.

A public CLI fixture additionally published a blocked child and its parent.
Read-back of the generated parent ROADMAP showed `State = blocked`, whereas
the accepted parent contained `state = null` and the canonical child reference;
`check-generated` returned `ok`. The render execution records the exact
session-local fixture path. No production Mission revision was changed by
this review. Specification verification remains guard-derived, not hand-set.

## Scope

This specification owns the first-release mission and structured-roadmap contract. It does not own ticket/spec/test/session payloads, authorize implementation outside the linked tickets, or include HTTP, a viewer, or multi-agent scheduling.