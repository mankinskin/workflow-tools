<!-- aligned-structure:v2 -->
# Canonical Mission and Session operational guidance

## Target Code Location
- [mission/README.md](../../../../mission/README.md)
- [session/README.md](../../../../session/README.md)
- [.agents/instructions/workflow/roadmap-authoring.instructions.md](../../../../.agents/instructions/workflow/roadmap-authoring.instructions.md)

## Naming Conventions
Criterion prefix: `DOC`. Public operations and carrier names are the concrete contract in Behavior and the linked execution package.

## Requester Input
> update the ticket and create all planning entities required. continue refinement until all blockers are solved


## Reading Order
1. [Recovery composition](../9f8d9f34-764f-4821-9b14-b6d8773ec2ae/body.md)
2. [authority provider](../0a8b3ea4-a3ce-4f26-90cc-1ce34474fb6c/body.md)
3. [workspace provider](../eb4444fd-1349-471f-b911-26b8c7cdf5d0/body.md)
4. [repair provider](../44f100c4-215e-445a-b452-1a8a969d96cc/body.md)
5. [move provider](../2cd4db32-3987-4d6f-a2dc-774c356b69ad/body.md)
6. [session-set provider](../af04e1d2-0a89-4720-b03e-ec1c23c53235/body.md)
7. [session-schema provider](../cef24f88-a24b-4d1c-9a9c-5c249e9991f9/body.md)
8. [migration provider](../53e78fcb-df6f-4d74-91fa-163dbcb0e1a8/body.md)
9. [Historical or adjacent contract](../dee92faf-b649-4577-8016-e6e465e6724a/body.md)
10. [W9 execution package](../../../../../transcripts/24-10-2026_mission-typed-authoring/09-guidance.md)
11. [Owning ticket](../../../ticket/tickets/f71a08aa-938c-4590-9b09-006ce69c48fc/ticket.toml)

## Responsibility
Motivation: Canonical Mission and Session operational guidance closes the observed canonical-recovery gap without annexing adjacent historical work. If this spec is implemented, dependents can rely on every criterion below, not merely a successful command exit.

## Interfaces And Dependencies
Consumes only the directed provider criteria listed below.
Domain APIs own persistence and lifecycle; transports preserve structured outcomes. Explicit entity-store roots remain distinct from command cwd and checkout.

## Behavior
- DOC-1: Update only canonical Mission/Session READMEs, superproject Mission tooling instructions and changed roadmap-authoring instructions; generated projections remain read-only.
- DOC-2: Document API-owned typed authoring/UUID/bootstrap, independent JSON Part sources and generated Markdown output, save versus publish, explicit/configured/cwd roots and canonical workspace.
- DOC-3: Document delivered repair/move journal IDs, full fingerprints/CAS, no-op/restart/compensation and approval boundaries, plus Session set-move and production-v1 schema contracts.
- DOC-4: Examples and final status correspond to actual W7 live results and W8 independent verification. Incomplete migration cannot be described as completed; canonical destination and current execution-Mission root are explicit.
- DOC-5: Validate changed links and actual CLI/MCP example fields against existing focused evidence, plus repository-specific whitespace checks. Do not repeat unchanged provider suites or use keyword grep as semantic proof.

## Boundaries And Failure Cases
An example/field/owner mismatch is corrected in its canonical source. New unrelated generator configuration is not guessed, generated copies are not hand-edited, and failing live work is not papered over by successful docs.

## Provider/Consumer Contract
- Consumer mission.typed-authoring.guidance -> [mission.canonical-recovery.authority](../0a8b3ea4-a3ce-4f26-90cc-1ce34474fb6c/body.md) -> `AUTH-1`, `AUTH-2`, `AUTH-3`, `AUTH-4`, `AUTH-5`.
- Consumer mission.typed-authoring.guidance -> [mission.canonical-recovery.workspace](../eb4444fd-1349-471f-b911-26b8c7cdf5d0/body.md) -> `ROOT-1`, `ROOT-2`, `ROOT-3`, `ROOT-4`.
- Consumer mission.typed-authoring.guidance -> [mission.canonical-recovery.repair](../44f100c4-215e-445a-b452-1a8a969d96cc/body.md) -> `REPAIR-1`, `REPAIR-2`, `REPAIR-3`, `REPAIR-4`, `REPAIR-5`, `REPAIR-6`.
- Consumer mission.typed-authoring.guidance -> [mission.canonical-recovery.move](../2cd4db32-3987-4d6f-a2dc-774c356b69ad/body.md) -> `MOVE-1`, `MOVE-2`, `MOVE-3`, `MOVE-4`, `MOVE-5`, `MOVE-6`.
- Consumer mission.typed-authoring.guidance -> [session.canonical-recovery.set-transports](../af04e1d2-0a89-4720-b03e-ec1c23c53235/body.md) -> `SET-1`, `SET-2`, `SET-3`, `SET-4`.
- Consumer mission.typed-authoring.guidance -> [session.canonical-recovery.schema](../cef24f88-a24b-4d1c-9a9c-5c249e9991f9/body.md) -> `SCHEMA-1`, `SCHEMA-2`, `SCHEMA-3`, `SCHEMA-4`, `SCHEMA-5`.
- Consumer mission.typed-authoring.guidance -> [mission.typed-authoring.migration](../53e78fcb-df6f-4d74-91fa-163dbcb0e1a8/body.md) -> `LIVE-1`, `LIVE-2`, `LIVE-3`, `LIVE-4`, `LIVE-5`, `LIVE-6`, `LIVE-7`.

## Examples
A guide distinguishes deterministic planned proposal identity from actual journal J, shows fresh post-repair move preflight and explicit canonical destination, and states that production Session v1 plan/apply is a journal-free no-op.

## Evidence
- Position: `partial` existing surface; the new criteria are `not-implemented`. code_ref: [mission/README.md](../../../../mission/README.md).
- Position: `partial` existing surface; the new criteria are `not-implemented`. code_ref: [session/README.md](../../../../session/README.md).
- Position: `partial` existing surface; the new criteria are `not-implemented`. code_ref: [.agents/instructions/workflow/roadmap-authoring.instructions.md](../../../../.agents/instructions/workflow/roadmap-authoring.instructions.md).
Planned ValidationSpec guard ID: `mission-recovery-guidance`, not yet registered or executed. Its owning authorized execution package registers and reads back this guard through test-api before recording results; no test entity is created during planning. Approval is not verification and no guard success is claimed.
- `rtk git diff --check -- .agents/instructions/workflow/mission-tooling.instructions.md`
- `rtk git -C workflow-tools diff --check -- mission/README.md session/README.md .agents/instructions/workflow/roadmap-authoring.instructions.md`
- `Semantic link and CLI/MCP example comparison against W1-W8 recorded results`
Commands run only during separately authorized execution, each focused selector once per owning changed revision with nonzero intended cases. Negative cases require exact diagnostic and unchanged-byte assertions. Runtime evidence belongs in the explicit workflow-tools test store and links this spec/owning ticket; a result write is read back by ID.
Traceability: [actual owning ticket](../../../ticket/tickets/f71a08aa-938c-4590-9b09-006ce69c48fc/ticket.toml).
The current content is a planning contract, not an implemented/verified claim. For reused approved migration/guidance specs, the supported lifecycle has no approved-to-draft edge; body revision preserves the existing state/history and requires the new route reviews before handoff.

## Scope
Product behavior changes, generated guidance edits, repeated broad suites or live operations disguised as docs.
