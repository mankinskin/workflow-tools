<!-- aligned-structure:v2 -->
# Portable explicit and configured Mission launch roots

## Target Code Location
- [mission/crates/mission/src/lib.rs](../../../../mission/crates/mission/src/lib.rs)
- [mission/crates/mission/src/bin/mission.rs](../../../../mission/crates/mission/src/bin/mission.rs)
- [mission/crates/mission/src/bin/mission-mcp.rs](../../../../mission/crates/mission/src/bin/mission-mcp.rs)

## Naming Conventions
Criterion prefix: `ROOT`. Public operations and carrier names are the concrete contract in Behavior and the linked execution package.

## Requester Input
> update the ticket and create all planning entities required. continue refinement until all blockers are solved


## Reading Order
1. [Recovery composition](../9f8d9f34-764f-4821-9b14-b6d8773ec2ae/body.md)
2. [authority provider](../0a8b3ea4-a3ce-4f26-90cc-1ce34474fb6c/body.md)
3. [Historical or adjacent contract](../9413a0a2-0d53-407d-b088-3953074393fb/body.md)
4. [W2 execution package](../../../../../transcripts/24-10-2026_mission-typed-authoring/02-typed-persistence.md)

## Responsibility
Motivation: Portable explicit and configured Mission launch roots closes the observed canonical-recovery gap without annexing adjacent historical work. If this spec is implemented, dependents can rely on every criterion below, not merely a successful command exit.

## Interfaces And Dependencies
Consumes only the directed provider criteria listed below.
Domain APIs own persistence and lifecycle; transports preserve structured outcomes. Explicit entity-store roots remain distinct from command cwd and checkout.

## Behavior
- ROOT-1: CLI/MCP select explicit --workspace, then optional MISSION_WORKSPACE, then process cwd; relative selected paths resolve against process cwd, never inferred checkout.
- ROOT-2: Valid explicit root wins over lower-priority empty/invalid configuration. A selected empty configured value errors explicitly; no hidden fallback or cross-store write.
- ROOT-3: API receives only explicit root and never reads launch environment. Portable canonical launch configuration selects workflow-tools/mission without embedding a developer absolute path.
- ROOT-4: Bare no-config CLI/MCP retain cwd compatibility. Read-only operations never initialize a missing store; unsafe root/symlink/containment cases follow shared resolver rejection.

## Boundaries And Failure Cases
Invalid selected roots and launch configuration produce actionable errors. A stale installed binary is not accepted as proof of checkout behavior.

## Provider/Consumer Contract
- Consumer mission.canonical-recovery.workspace -> [mission.canonical-recovery.authority](../0a8b3ea4-a3ce-4f26-90cc-1ce34474fb6c/body.md) -> `AUTH-1`, `AUTH-2`, `AUTH-3`, `AUTH-4`, `AUTH-5`.

## Examples
With cwd /work and MISSION_WORKSPACE=/canonical, --workspace relative selects /work/relative. With no explicit root, an empty environment value is rejected rather than silently selecting cwd.

## Evidence
- Position: `partial` existing surface; the new criteria are `not-implemented`. code_ref: [mission/crates/mission/src/lib.rs](../../../../mission/crates/mission/src/lib.rs).
- Position: `partial` existing surface; the new criteria are `not-implemented`. code_ref: [mission/crates/mission/src/bin/mission.rs](../../../../mission/crates/mission/src/bin/mission.rs).
- Position: `partial` existing surface; the new criteria are `not-implemented`. code_ref: [mission/crates/mission/src/bin/mission-mcp.rs](../../../../mission/crates/mission/src/bin/mission-mcp.rs).
Planned ValidationSpec guard ID: `mission-recovery-workspace`, not yet registered or executed. Its owning authorized execution package registers and reads back this guard through test-api before recording results; no test entity is created during planning. Approval is not verification and no guard success is claimed.
- `rtk cargo test -p mission --features cli,mcp --test direct_transports workspace_`
Commands run only during separately authorized execution, each focused selector once per owning changed revision with nonzero intended cases. Negative cases require exact diagnostic and unchanged-byte assertions. Runtime evidence belongs in the explicit workflow-tools test store and links this spec/owning ticket; a result write is read back by ID.
Inline single-session package; no redundant planning ticket. Its native Mission checkpoint and validation evidence own execution progress.
The current content is a planning contract, not an implemented/verified claim. For reused approved migration/guidance specs, the supported lifecycle has no approved-to-draft edge; body revision preserves the existing state/history and requires the new route reviews before handoff.

## Scope
Global configuration frameworks or changing bare cwd semantics.
