# Implement recursive nested mission waypoints

## Objective

Support recursively nested mission waypoints in classic dossier roadmaps and
mission-generated roadmaps, using the agreed Mission Domain contract.

## Scope

- Represent a waypoint's child-mission reference distinctly from ticket
  ownership and execution scope.
- Resolve referenced missions only from the explicitly selected same-workspace
  mission store.
- Reject missing, unaccepted, cross-workspace, or empty child missions and
  direct or transitive cycles before accepting or publishing a parent mission.
- Reject acceptance of any mission with zero waypoints.
- Derive completion recursively: a mission is complete only when all its
  accepted waypoints are complete. Otherwise, the first non-completed
  waypoint in execution order determines the mission state. Ticket-backed
  waypoint status remains owned by the ticket store. The parent nested-mission
  waypoint reflects the child's derived state.
- Support equivalent nested-mission references in classic and
  mission-generated roadmap inputs while keeping generated files read-only
  projections.
- Preserve strict serial execution and the immediate-predecessor dependency
  rule. Do not add parallel execution or redundant dependencies.

## Acceptance criteria

1. Mission bundles can represent a nested-mission waypoint without treating
   its mission reference as a ticket reference or ticket-status owner.
2. Recursive resolution works only within the selected workspace and rejects
   missing, unaccepted, cross-workspace, and empty missions.
3. Validation rejects direct and transitive cycles at every supported nesting
   depth. A failed acceptance or publication leaves the accepted parent and
   generated files unchanged.
4. Empty missions are rejected rather than treated as vacuously complete.
5. A child mission is complete iff all of its accepted waypoints are complete;
   otherwise, the first non-completed waypoint in execution order supplies its
   state. Ticket-backed waypoint state is read from the ticket store, and the
   parent nested-mission waypoint reflects the child's derived state.
6. Classic dossier roadmaps and mission-generated roadmaps accept equivalent
   nested-mission references; generated roadmaps remain projections of the
   accepted bundle.
7. Nested execution remains strictly serial. No cross-workspace traversal,
   parallel mission execution, session-based transcript ingestion, automatic
   mission creation, or multi-agent scheduling is introduced.
8. Focused tests cover model representation, recursive resolution, all
   rejection cases, state derivation, both roadmap inputs, serial execution,
   and preservation of accepted/generated state on failure.

## Validation

- `cargo test -p mission-api --test roadmap_bundle`
- `cargo test -p mission-api --test roadmap_render`
- `cargo test -p mission --test roadmap_commands`
- `cargo test -p mission --test dossier_migration`

## Related contract

[Mission Domain spec](../../../spec/specs/38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/spec.toml).
