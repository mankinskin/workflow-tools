# Optional waypoint Ticket, Specification, and test relationships

## Objective

Support optional Ticket promotion and canonical Specification/test-execution
relationships for Mission waypoints.

## Requirements

- Unpromoted waypoints remain valid; promotion references an existing Ticket
  and never creates, activates or takes over one.
- Ticket acceptance criteria remain owned by the Specification system.
- Test executions link to waypoints and may optionally link to a criterion.
- Roadmaps resolve current Ticket and Specification content with owning-root
  context.
- Missing, dangling or wrong-root references fail without changing source.

## Design and boundaries

Reuse the Mission Domain, direct/file-backed authoring, Ticket and test APIs.
Do not copy acceptance criteria or create another test store. Do not add a
ticket dependency edge solely because a link exists.

## Acceptance criteria

- Both unpromoted and promoted waypoints pass their respective validation.
- Promoted Ticket references resolve to the exact canonical root and ID.
- Specification-owned criteria and test executions are read through their
  owning APIs; criterion-specific execution links remain optional.
- Missing/wrong-root references produce explicit diagnostics and preserve
  accepted Mission data.

## Validation plan

Future guard `w8-mission-waypoint-interoperability`:
`cargo test -p mission-api --test waypoint_interoperability_contract`.
No product validation was run during this planning handoff.

## Sources and non-goals

R1-027–030 and R1-036; A24. Reuse Mission Domain, Direct Mission Authoring
and Typed File-Backed Mission Authoring. No Mission or Ticket mutation in W8.
