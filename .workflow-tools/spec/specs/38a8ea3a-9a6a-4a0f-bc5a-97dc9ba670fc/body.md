<!-- aligned-structure:v2 -->
# Mission Domain

## Motivation

The workflow-tools repository needs a durable, queryable mission entity that replaces transcript dossiers as the shared planning object across sessions. The mission entity aggregates an objective, roadmap state, waypoints, linked workflow entities, artifacts, requirements, and attached sessions without copying session-local workflow state.

If this specification is implemented, dependents can rely on a UUID-addressed mission record remaining a shared source of planning context across sessions while its referenced tickets, specifications, validation records, artifacts, and sessions retain their owning-domain identities.

## Target Code Location

- [workflow-tools workspace manifest](../../Cargo.toml) owns the future `mission-api` and `mission` workspace members.
- [Session pin routing](../../session/crates/session-api/src/store_routing_types.rs) owns the legal URI store enumeration that will gain `mission`.
- [Domain crate contract](../../../context-engine/WORKFLOW_TOOLS_DOMAIN_CRATE_CONTRACT.md) governs the future public mission facade and transport boundaries.

## Naming Conventions

- The internal crate is `mission-api`; the public facade crate is `mission`.
- A mission identifier is a UUID and its canonical session reference is `ce://<workspace>/mission/<mission-id>`.
- A persisted mission is a UUID directory under `.workflow-tools/mission/missions/<mission-id>/` containing a manifest and durable history.
- Mission criteria use the `mission-*` prefix.

## Requester Input

> Aufgaben und Artefakte und Anforderungen und alle möglichen Entitäten oder auch Agentensitzungen ... oder eben kurz Missionen zu speichern und zu verwalten.

## Reading Order

1. [Mission-domain roadmap](ROADMAP.md) - approved W5-W12 execution route.
2. [Mission research inventory](ARTIFACTS.md) - discovery evidence and established domain precedents.
3. [Mission review decision](REVIEW.md) - confirmed scope, non-goals, and dossier-replacement decision.
4. [Workflow-tools domain crate contract](../../context-engine/WORKFLOW_TOOLS_DOMAIN_CRATE_CONTRACT.md) - required public/private crate boundary.
5. [Session workflow guidance](../../workflow-tools/.agents/instructions/session/session-workflow.instructions.md) - per-session graph boundary that missions must not duplicate.
6. [Session pin routing](../../workflow-tools/session/crates/session-api/src/store_routing_types.rs) - extension point for mission attachment URNs.

## Responsibility

The mission domain owns durable cross-session planning records. A record must capture:

- mission identity, title, objective, and lifecycle state;
- roadmap and ordered waypoints with dependencies and current state;
- canonical references to tickets, specifications, validation evidence, artifacts, requirements, and sessions; and
- migration provenance when a record originates from a legacy `transcripts/` dossier.

The mission domain must preserve references as identifiers or URNs. A mission record must not duplicate the authoritative payload, history, state machine, or workflow graph of any referenced domain entity.

## Interfaces And Dependencies

`mission-api` provides the internal model, persistence, integrity validation, and mutation/read-back API. The public `mission` facade re-exports that API and later supplies feature-gated CLI and MCP transports through `transport-harness`, following the [domain crate contract](../../context-engine/WORKFLOW_TOOLS_DOMAIN_CRATE_CONTRACT.md).

The session domain remains the owner of session manifests and `session_workflow_*`. The session domain consumes mission URNs only to attach a session to a shared mission. Ticket, specification, test, feedback, and artifact domains remain owners of their referenced entities.

## Behavior

### Mission persistence

The mission store root is `.workflow-tools/mission/`. Each mission manifest is UUID-addressed, atomically persisted, and read back after every successful mutation. A read returns the mission model and its canonical references without resolving or copying foreign entity content.

### Integrity

Mission mutations reject malformed UUIDs, duplicate reference identities, invalid canonical URNs, and waypoint edges that reference unknown waypoints or create cycles. A failed mutation leaves the last durable manifest unchanged. Store reads report malformed or incomplete manifests as actionable errors rather than silently dropping fields.

### Cross-session attachment

The first release extends the legal session pin-URN stores with `mission`. Any session may attach the same `ce://<workspace>/mission/<mission-id>` reference. A session attachment is not a transfer of ownership and does not embed the mission record into `session.json`.

### Legacy-dossier migration

Migration inventories the 69 legacy `transcripts/` dossier folders, creates or maps one mission record per source folder idempotently, preserves source-path provenance, and reports migrated, skipped, and blocked records. Migration provides dry-run behavior and never deletes source dossiers. The current roadmap dossier is migrated first and becomes the `$MISSION_ID` record used by later validation.

## Boundaries And Failure Cases

- The first release has no HTTP transport, viewer, or multi-agent scheduling.
- A mission waypoint is planning metadata; scheduling agents, assigning execution leases, and parallel execution policy remain outside the model.
- A missing foreign entity does not permit destructive repair of the mission record. The read or validation surface reports the missing reference with its canonical identity.
- Migration blockers preserve the source dossier and report the failure; migration does not partially rewrite a legacy dossier.
- Session-local graphs remain session-local. The mission domain does not move, mirror, or mutate `session_workflow_*` state.

## Provider/Consumer Contract

The mission domain provides durable shared planning context to the session domain, prompt-ingestion guidance, roadmap execution guidance, and migration tooling. Those consumers rely on `mission-*` criteria for durable storage, canonical references, session attachment, and idempotent migration. The mission domain consumes only stable identifiers from ticket, specification, test, feedback, artifact, and session domains.

## Examples

A mission created from `transcripts/01-10-2026_roadmap-state-integration/` stores the objective and W5-W12 waypoint state, references the W5 specification and W7-W10 tickets by canonical identifiers, and records the source dossier path. Two implementation sessions can each pin the same mission URN while keeping separate `session.json` workflow graphs.

## Evidence

Positions:

- `workflow-tools/mission/` - not-implemented.
- `mission-api` crate - not-implemented.
- `mission` facade crate - not-implemented.
- `session-api` mission URN routing - not-implemented.
- Dossier migration command - not-implemented.

The governing rule is [Roadmap State And Waypoint Ownership](../../workflow-tools/.agents/instructions/workflow/roadmap-authoring.instructions.md), which introduces mission-level planning context. The specification is coming soon: no dependent may assume the contract is implemented until the validation guards below pass.

Planned validation guards:

- `cargo test -p mission-api` proves persistence and integrity behavior.
- `cargo test -p mission` proves public facade and transport composition behavior.
- `cargo test -p session-api -- mission` proves mission session-attachment behavior.
- `cargo run -p mission --features cli -- mission migrate-dossiers --dry-run` proves migration reporting without source deletion.

## Scope

This specification owns the first-release mission-domain contract. It does not authorize implementation before linked tickets are created, does not own ticket/spec/test/session payloads, and excludes HTTP, a viewer, and multi-agent scheduling.