<!-- aligned-structure:v2 -->
# Mission Waypoint Planned Outcomes and Advisory Handling

## Target Code Location

- [Native Mission bundle model, validation, and rendering](../../../../mission/crates/mission-api/src/lib.rs)
- [Typed Mission source model and validation](../../../../mission/crates/mission-api/src/typed_source.rs)
- [Native-to-typed migration](../../../../mission/crates/mission-api/src/typed_migration.rs)
- [Actual operator-result model and rendering](../../../../mission/crates/mission-api/src/operator_results.rs)

## Naming Conventions

Use `WaypointOutcomePlan` with `expected_outcome` and `alternative_outcomes`. Each `AlternativeOutcome` has an `outcome` and one or more `acceptable_handling` text options. Store the plan as `outcome_plan` on native `WaypointFragment` and typed `TypedWaypoint` values. Keep this plan-time vocabulary distinct from `OperatorResult`.

## Requester Input

> For each waypoint, include expected outcomes, alternative possible outcomes, and possible acceptable handling steps.
>
> Store acceptable handling for alternative outcomes as advisory human guidance: one or more descriptive options, not binding acceptance criteria and not an automatic progress change or branch.
>
> Keep implemented alternative branches and mutually exclusive roadmap paths out of scope.

## Reading Order

1. [Mission Domain](../38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) — owning Mission schema and ordered-roadmap contract.
2. [Bounded Mission/Ticket Schema, Requirements, and Entity Rendering](../b2c52f3b-5061-441e-9c7b-dc8dc8d8eec8/body.md) — provider criteria for explicit schema versions, migration preservation, and deterministic renderers.
3. [Direct Mission Authoring and Recording](../0e02e89c-aff1-47c8-a113-3c6b36688add/body.md) — actual operator-result semantics that remain separate.
4. [W7 implementation Ticket 921335e1](../../../ticket/tickets/921335e1-ac41-4217-b86c-e582eb0762b9/ticket.toml) — execution plan for this contract.
5. [Native Mission bundle](../../../../mission/crates/mission-api/src/lib.rs) — `WaypointFragment`, validation, and output.
6. [Typed Mission source](../../../../mission/crates/mission-api/src/typed_source.rs) — `TypedWaypoint` and persisted typed records.
7. [Typed migration](../../../../mission/crates/mission-api/src/typed_migration.rs) — native-to-typed conversion behavior.
8. [Operator results](../../../../mission/crates/mission-api/src/operator_results.rs) — distinct actual outcomes.

## Responsibility

Define and validate plan-time expected and alternative waypoint outcomes, carry them losslessly through both Mission schema representations, and render them as descriptive roadmap and Part content. This contract does not own actual execution results or progress.

## Interfaces And Dependencies

The new Mission implementation consumes W3's schema-version, migration-preservation, and deterministic-renderer foundation. W7 Ticket [921335e1](../../../ticket/tickets/921335e1-ac41-4217-b86c-e582eb0762b9/ticket.toml) depends on completion of W3 Ticket [7ecdd680](../../../ticket/tickets/7ecdd680-e1aa-4d78-b0bb-656a58757b10/ticket.toml); no other W1-W6 Ticket dependency is implied. The new fields are carried by native and typed Mission representations and rendered into both generated views. `OperatorResult` remains the authority for actual operator-reported results.

## Behavior

Native bundle schema v2 adds `outcome_plan` to each `WaypointFragment`; typed source schema v3 adds the same plan to each `TypedWaypoint`. A plan contains a non-empty `expected_outcome` and at least one alternative. Each alternative has a non-empty `outcome` and a non-empty list of non-empty `acceptable_handling` options. New v2/v3 waypoints require a complete plan.

Continue to read, validate, and render native schema v1 and typed source v2 records with no outcome plan. Preserve their current rendering and data. Preserve native v1 to typed v2 migration without synthesizing outcomes; convert native v2 plans losslessly to typed v3. An explicit legacy-data upgrade to the new versions requires requester-authored plans for every waypoint. Unsupported versions and invalid plans fail without changing accepted source data.

Render the plan deterministically in the generated roadmap and waypoint Part. Clearly label expected outcomes, alternative outcomes, and acceptable handling as descriptive manual guidance. Do not treat alternatives as acceptance criteria, state transitions, prerequisite edges, scheduling instructions, or dispatch commands. Do not change or merge actual `operator_results`.

## Boundaries And Failure Cases

Reject missing plans on new-schema waypoints, blank expected or alternative outcomes, empty handling lists, blank handling options, and unsupported schema versions with source-preserving diagnostics. A failed parse, validation, conversion, or publication must leave the accepted Mission and its existing generated artifacts unchanged. Legacy versions remain readable without backfill. Do not implement branch execution, automatic recovery, progress mutation, or retrospective Mission edits.

## Provider/Consumer Contract

This outcome contract consumes W3 Ticket [7ecdd680](../../../ticket/tickets/7ecdd680-e1aa-4d78-b0bb-656a58757b10/ticket.toml) and its approved Spec criteria for explicit schema/version meaning, preservation on migration failure, and deterministic escaped composition. It provides validated outcome plans to Mission roadmap and Part renderers. The consumer-to-provider gate is W7 Ticket `depends_on` W3 Ticket; it does not create dependencies among the existing W1-W6 Tickets.

## Examples

A new native v2 waypoint may contain:

```json
{
  "outcome_plan": {
    "expected_outcome": "The accepted Mission revision and its generated roadmap agree.",
    "alternative_outcomes": [
      {
        "outcome": "Validation detects a mismatch before publication.",
        "acceptable_handling": [
          "Preserve the accepted revision, inspect the diagnostic, and correct the draft before retrying."
        ]
      }
    ]
  }
}
```

The renderer displays this text but does not publish, repair, or change waypoint progress because an alternative is present.

## Acceptance Criteria

- Native v2 and typed v3 schemas represent `WaypointOutcomePlan` on every new waypoint and reject missing or malformed plan content.
- Native v1 and typed v2 records remain readable and renderable without fabricated plans; native v1 to typed v2 migration preserves the legacy absence, and native v2 to typed v3 migration preserves plan values exactly.
- Unsupported versions and invalid plans return explicit diagnostics and preserve the prior accepted source and generated artifact set.
- Roadmap and Part output render the plan deterministically with escaping; legacy output remains unchanged when no plan exists.
- Alternatives and handling text never affect acceptance, dependency edges, state, progress, scheduling, or execution.
- Actual `operator_results` remain separate and retain their current validation and rendering semantics.

## Guards

Planned test-api ValidationSpec: `mission-waypoint-outcome-plan-contract`.
The execution-time guard command is `cargo test --manifest-path workflow-tools/Cargo.toml --locked -p mission-api --test waypoint_outcome_plan_contract`. Register this ValidationSpec before implementation; it has not been created or executed during planning.

## Positions

- [Native `WaypointFragment`](../../../../mission/crates/mission-api/src/lib.rs) — `partial`: no outcome plan exists; native schema v1 is strict.
- [Typed `TypedWaypoint`](../../../../mission/crates/mission-api/src/typed_source.rs) — `partial`: no outcome plan exists; typed source v2 is strict.
- [Typed migration](../../../../mission/crates/mission-api/src/typed_migration.rs) — `partial`: converts native bundles but has no plan-preserving v2-to-v3 path.
- [Operator-result model](../../../../mission/crates/mission-api/src/operator_results.rs) — `implemented`: actual results are separate and must remain unchanged.

## Evidence

The guard must exercise legacy reads, schema-version routing, complete and invalid plans, lossless migration, deterministic escaped rendering, preservation after failure, and proof that descriptive alternatives do not alter progress or dependencies. No product implementation or validation has occurred for this contract.

## Scope

Owns only plan-time waypoint outcome schema, migration compatibility, validation, and rendering. Existing W1-W6 acceptance contracts remain unchanged; broader workflow backends, automatic branches, actual result recording, and historical Mission backfill remain out of scope.
