<!-- aligned-structure:v2 -->
# Dedicated Mission Review and Completeness Role

## Motivation

Source requirements R1-008–009 request a dedicated workflow-aware quality and completeness reviewer entry while treating audit replacement only as a possibility. The existing Audit Agent is general-purpose and scoped to context-engine; bounded roster/guidance discovery found no existing Mission-specific reviewer owner. The Guidance Lifecycle Agent owns any future authoring of a workflow-tools-local Mission role.

If implemented, reviewers can use the generated entry to check promises, artifact reachability and evidence gaps without hidden context, while the existing audit role remains compatible.

## Target Code Location

- [Mission API documentation](../../../../mission/README.md)
- [Existing Audit Agent](../../../../audit/.agents/agents/audit.agent.md)
- [Workflow-tools agent directory](../../../../.agents/agents/)
- [Guidance Lifecycle Agent](../../../../.agents/agents/guidance-lifecycle.agent.md) — future authoring owner.

## Naming Conventions

Use the role name `Mission Review Agent` and criteria `mission-review-role-*`. Its future guidance belongs in the workflow-tools-owned agent directory, not in this planning change.

## Reading Order

1. [Generated review-entry contract](../87d20edd-240d-4533-b7ba-711b06bf8296/body.md) — reviewer input surface.
2. [Existing Audit Agent](../../../../audit/.agents/agents/audit.agent.md) — adjacent compatibility boundary.
3. [Mission documentation](../../../../mission/README.md) — Mission-specific terminology.
4. [Guidance Lifecycle Agent](../../../../.agents/agents/guidance-lifecycle.agent.md) — canonical future guidance author.

## Responsibility

Define the behavior and handoff contract of a Mission-specific completeness reviewer that operates on generated review entry links and reports evidence gaps.

## Interfaces And Dependencies

Consumes the generated review entry, Mission source/rendered artifacts, acceptance expectations and available validation receipts. It returns evidence-linked findings, unavailable facts and residual risks. It does not own product validation execution.

## Behavior

The reviewer checks promised-versus-delivered reachability, schema/reference consistency and declared completeness, and cites evidence for each finding. It states unknowns explicitly and works from the entry without requiring undocumented context. It may invoke or refer to repository audit checks but does not replace or silently change their authority.

## Boundaries And Failure Cases

Do not create guidance files in W8, modify the Audit Agent, claim product readiness from planning records, fabricate missing evidence, or make audit replacement a requirement. If the generated entry is missing or incomplete, report that as a blocker rather than reconstructing invisible state.

## Provider/Consumer Contract

Consumes the generated review-entry spec's navigation contract and existing Audit Agent's general checks as an adjacent provider. It provides a Mission-specific review result; no replacement edge or ownership transfer is defined.

## Examples

Given a review entry that omits one declared Part and has no current validation receipt, the role reports two evidence-linked completeness gaps, states that readiness cannot be concluded, and leaves repository audit ownership unchanged.

## Acceptance Criteria

- The Mission-specific role has a clear invocation/input/output contract and works from the generated entry without hidden context.
- It cites delivered artifacts and validation evidence for each finding and distinguishes missing, contradictory and unavailable information.
- It does not execute product tests or claim readiness in the absence of evidence.
- Existing audit checks remain callable and unchanged; audit replacement is not required.
- The Guidance Lifecycle Agent owns any future role authoring; all actual guidance changes remain outside W8.

## Guards

`w8-mission-reviewer-role` is the declared test-api ValidationSpec. Its validation is a future manual role walkthrough against complete, missing and contradictory-entry examples; no guidance or product tests were run in W8.

## Positions

- `../../../../mission/README.md` — `partial`: Mission terminology and operations are documented; a dedicated reviewer entry is not present.
- `../../../../audit/.agents/agents/audit.agent.md` — `implemented`: general audit role exists; its context-engine scope does not establish a Mission-specific reviewer.

## Evidence

The guard is a manual workflow simulation documented in `w8-mission-reviewer-role`; evaluate complete, missing and contradictory entries and verify evidence citations, unknown handling and audit compatibility.

## Scope

Owns the Mission reviewer role contract and future role entry. It does not create or edit guidance during W8 and does not replace repository audit.