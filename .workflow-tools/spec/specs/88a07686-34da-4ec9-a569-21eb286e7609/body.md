<!-- aligned-structure:v2 -->
# Optional Waypoint Ticket, Specification, and Test Relationships

## Motivation

Source requirements R1-027–030 and R1-036 ask for optional Ticket promotion,
Specification-owned acceptance criteria, test-execution links, and
entity-driven Mission roadmaps. Existing Mission Domain and authoring specs
are the contract owners; this child makes the cross-domain relationship
acceptance explicit without duplicating their criteria.

If implemented, an unpromoted waypoint remains valid and a promoted waypoint
can reference real Tickets, Specification-owned criteria and test executions.

## Target Code Location

- [Mission API](../../../../mission/crates/mission-api/src/lib.rs)
- [Ticket model](../../../../ticket/crates/ticket-api/src/model/ticket.rs)
- [Test API](../../../../test/crates/test-api/src/lib.rs)

## Naming Conventions

Use `mission-waypoint-link-*` criteria. `promotion` means adding an optional
reference to an existing Ticket, not creating, taking over or activating it.

## Reading Order

1. [Mission Domain](../38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) — parent invariants.
2. [Direct authoring](../0e02e89c-aff1-47c8-a113-3c6b36688add/body.md) — authoring/provider contract.
3. [Typed file-backed authoring](../dee92faf-b649-4577-8016-e6e465e6724a/body.md) — source composition.
4. [Ticket model](../../../../ticket/crates/ticket-api/src/model/ticket.rs) — canonical Ticket identity.
5. [Test API](../../../../test/crates/test-api/src/lib.rs) — canonical validation execution links.

## Responsibility

Specify the optional cross-domain links required when a waypoint is promoted
or references externally owned acceptance and validation evidence.

## Interfaces And Dependencies

Mission consumes canonical Ticket, Specification/criterion and test-execution
identities through domain APIs. The Specification system owns criteria; the
test store owns execution outcomes. Mission may render links but does not
become a second authority.

## Behavior

Waypoints without Ticket links validate and render normally. A promoted
waypoint links to an existing canonical Ticket; Ticket acceptance criteria
resolve to the Specification system. Validation executions can link to the
waypoint and optionally an individual criterion. Roadmap generation reads
current Ticket/Specification records and identifies the owning root.

## Boundaries And Failure Cases

Never require every waypoint to have a Ticket, create a Ticket as a side
effect, take over active/completed Tickets, copy criteria into Mission, or
duplicate test execution storage. Missing, wrong-root or dangling entities
produce explicit validation errors and preserve the source record.

## Provider/Consumer Contract

Consumes canonical Ticket, Specification and test-store identities; provides
resolved display data to Mission roadmap generation. These interface links do
not imply ticket `depends_on` edges or implementation order.

## Examples

An unpromoted waypoint with a complete prompt and checkpoint remains valid.
A promoted waypoint points to an existing Ticket and its Spec-owned criterion;
one passed execution links to the waypoint, with criterion linkage optional.
A stale UUID or wrong owning root returns a diagnostic instead of a partial
roadmap.

## Acceptance Criteria

- Both promoted and unpromoted waypoint cases validate.
- Promotion references existing Tickets only; no create, activation or
  ownership change occurs implicitly.
- Criteria resolve through the Specification owner and executions through the
  test store; optional criterion linkage is accepted.
- Missing, wrong-root and dangling links fail explicitly and leave source data
  unchanged.
- Generated roadmaps read current linked entity content and preserve canonical
  IDs and root context.

## Guards

`w8-mission-waypoint-interoperability` is the declared test-api
ValidationSpec. No product test or execution was run during W8.

## Positions

- [Mission API](../../../../mission/crates/mission-api/src/lib.rs) — `partial`: Mission
  waypoint authoring exists; complete Ticket/specification/execution linkage
  is not demonstrated.
- [Ticket model](../../../../ticket/crates/ticket-api/src/model/ticket.rs) — `implemented`:
  canonical Ticket model exists.
- [Test API](../../../../test/crates/test-api/src/lib.rs) — `implemented`:
  validation execution store exists; Mission waypoint linkage is not
  established by that fact.

## Evidence

The guard command is
`cargo test -p mission-api --test waypoint_interoperability_contract`. It must
cover both optionality paths, correct-root reads, dangling/wrong-root failures
and preservation after failed resolution.

## Scope

Owns Mission waypoint relationships only. Ticket criteria and test execution
semantics remain with their authoritative stores.
