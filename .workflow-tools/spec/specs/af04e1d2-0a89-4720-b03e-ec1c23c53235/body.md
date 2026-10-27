<!-- aligned-structure:v2 -->
# Session set-move CLI and MCP transport parity

## Target Code Location
- [session/crates/session-api/src/move_domain.rs](../../../../session/crates/session-api/src/move_domain.rs)
- [session/src/cli.rs](../../../../session/src/cli.rs)
- [session/src/mcp/server.rs](../../../../session/src/mcp/server.rs)

## Naming Conventions
Criterion prefix: `SET`. Public operations and carrier names are the concrete contract in Behavior and the linked execution package.

## Requester Input
> update the ticket and create all planning entities required. continue refinement until all blockers are solved


## Reading Order
1. [Recovery composition](../9f8d9f34-764f-4821-9b14-b6d8773ec2ae/body.md)
2. [W5 execution package](../../../../../transcripts/24-10-2026_mission-typed-authoring/05-guidance-evidence.md)

## Responsibility
Motivation: Session set-move CLI and MCP transport parity closes the observed canonical-recovery gap without annexing adjacent historical work. If this spec is implemented, dependents can rely on every criterion below, not merely a successful command exit.

## Interfaces And Dependencies
Uses existing domain/shared-kernel infrastructure; no dependency on unrelated Mission/Session work is fabricated.
Domain APIs own persistence and lifecycle; transports preserve structured outcomes. Explicit entity-store roots remain distinct from command cwd and checkout.

## Behavior
- SET-1: Add CLI session move-set preflight/apply/resume/rollback and MCP session_move_set_* over existing plan_move_set/execute_move_set/resume_move_set/rollback_move_set.
- SET-2: Preserve existing single-ID interfaces and API normalization, explicit roots, finite selection, per-ID blockers/results, set eligibility, no-op and partial semantics exactly.
- SET-3: API, actual CLI JSON and initialized MCP return equivalent structured results and recovery identity. Adapter failure after API success requires read-back, not replay.
- SET-4: Unsafe/missing IDs, reference/containment conflicts and journal/restart failures are surfaced without transport-owned fallback. Only minimal DTO exposure is allowed if existing API data needs transport access.

## Boundaries And Failure Cases
Any one-transport mismatch or single-ID compatibility regression keeps the inline package incomplete. A genuinely missing lifecycle engine is new scope, not permission to duplicate it in an adapter.

## Provider/Consumer Contract
No new sibling provider prerequisite. Existing implementation reuse is not a dependency on a new sibling outcome.

## Examples
A two-ID preflight with one blocked source returns the API's same set status and both dispositions through CLI and MCP; resume forwards the same actual kernel journal and roots.

## Evidence
- Position: `partial` existing surface; the new criteria are `not-implemented`. code_ref: [session/crates/session-api/src/move_domain.rs](../../../../session/crates/session-api/src/move_domain.rs).
- Position: `partial` existing surface; the new criteria are `not-implemented`. code_ref: [session/src/cli.rs](../../../../session/src/cli.rs).
- Position: `partial` existing surface; the new criteria are `not-implemented`. code_ref: [session/src/mcp/server.rs](../../../../session/src/mcp/server.rs).
Planned ValidationSpec guard ID: `session-recovery-set-transports`, not yet registered or executed. Its owning authorized execution package registers and reads back this guard through test-api before recording results; no test entity is created during planning. Approval is not verification and no guard success is claimed.
- `rtk cargo test -p session-api --lib move_domain::tests`
- `rtk cargo test -p session --features cli --lib move_set_`
- `rtk cargo test -p session --features mcp --lib move_set_`
Commands run only during separately authorized execution, each focused selector once per owning changed revision with nonzero intended cases. Negative cases require exact diagnostic and unchanged-byte assertions. Runtime evidence belongs in the explicit workflow-tools test store and links this spec/owning ticket; a result write is read back by ID.
Inline single-session package; no redundant planning ticket. Its native Mission checkpoint and validation evidence own execution progress.
The current content is a planning contract, not an implemented/verified claim. For reused approved migration/guidance specs, the supported lifecycle has no approved-to-draft edge; body revision preserves the existing state/history and requires the new route reviews before handoff.

## Scope
A new move engine or a dependency on Mission's move implementation.
