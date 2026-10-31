<!-- aligned-structure:v2 -->
# Mission-native New-run Workflow Routing and Guidance

## Target Code Location

- [tools/roadmap-route.py](../../../../tools/roadmap-route.py) — `code_ref: tools/roadmap-route.py`; position: `not-implemented` for this new contract (existing provider behavior is not reclassified).
- [.agents/instructions/workflow/prompt-ingestion.instructions.md](../../../../.agents/instructions/workflow/prompt-ingestion.instructions.md) — `code_ref: .agents/instructions/workflow/prompt-ingestion.instructions.md`; position: `not-implemented` for this new contract (existing provider behavior is not reclassified).
- [.agents/instructions/workflow/roadmap-authoring.instructions.md](../../../../.agents/instructions/workflow/roadmap-authoring.instructions.md) — `code_ref: .agents/instructions/workflow/roadmap-authoring.instructions.md`; position: `not-implemented` for this new contract (existing provider behavior is not reclassified).
- [.agents/instructions/workflow/roadmap-execution.instructions.md](../../../../.agents/instructions/workflow/roadmap-execution.instructions.md) — `code_ref: .agents/instructions/workflow/roadmap-execution.instructions.md`; position: `not-implemented` for this new contract (existing provider behavior is not reclassified).

## Naming Conventions

New-run native planning and explicit classic migration are separate routes. Add mission_native_routing as a future integration test target. Acceptance criterion identifiers are `MN-W7-NN`.

## Requester Input

> Vertiefe die Planung und erstelle und ordne Tickets und Specs in der Roadmap zu.

The full source scope and earlier explicit lifecycle/trait decisions are preserved in
[merged clean intent](../../../../../transcripts/29-10-2026_mission-native-mcp-planning/inputs/merged.clean.md).

## Reading Order

1. [Composition root](../92c78a48-cad8-4c08-8bba-4a523b717842/body.md) — shared invariants and component map.
2. [Owning Part](../../../../../transcripts/29-10-2026_mission-native-mcp-planning/07-native-workflow-switch.md) — execution package, not a competing behavior authority.
3. [Mission Domain](../38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) — stable identity, serial order and foreign authority.

- Consumer W7 -> [provider W6](../551422d7-8a0f-4711-8642-a4c367649361/body.md) -> MN-W6-01, MN-W6-02, MN-W6-03, MN-W6-04, MN-W6-05.
- Consumer W7 -> [provider W4](../9fe60c69-af53-4578-9310-69283cd09284/body.md) -> MN-W4-01, MN-W4-02, MN-W4-03, MN-W4-04.
- [Existing provider](../38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) -> Mission Domain: generated projections and durable cross-session authority. Reuse form 3: link unchanged.

## Responsibility

New intake defaults to Session-to-Mission MCP planning before exact requester approval while existing classic sources remain readable and opt-in migration stays explicit. This new behavior slice is not already guaranteed by its historical providers.

## Interfaces And Dependencies

Verified inventory from the bounded guidance research:

| Canonical owner | Repo-scope consumer materialization |
|---|---|
| Parent `.agents/prompts/refine-ingest.prompt.md` | `.agents/prompts/refine-ingest.prompt.md` |
| Parent `.agents/prompts/execute-ingest.prompt.md` | `.agents/prompts/execute-ingest.prompt.md` |
| Parent `.agents/prompts/execute-roadmap.prompt.md` | `.agents/prompts/execute-roadmap.prompt.md` |
| Product `.agents/instructions/workflow/prompt-ingestion.instructions.md` | `.agents/instructions/workflow/prompt-ingestion.instructions.md` in initialized owning product submodule, otherwise consumer root |
| Product `.agents/instructions/workflow/roadmap-authoring.instructions.md` | same relative instruction path and ownership rule |
| Product `.agents/instructions/workflow/roadmap-execution.instructions.md` | same relative instruction path and ownership rule |
| Product `tools/roadmap-route.py` | code, not a generated guidance copy |

Both repositories' `.claude/commands` are symlinks to `../.agents/prompts`.
They are aliases, not separate files to regenerate. The three route prompts
exist in the parent, not the product prompt directory. No tracked `.github`
guidance copies or configured rule-target registry were found; do not invent
such outputs. `install/artifacts.toml` owns binaries/hooks; `COMMANDS.md` is
its projection, not a guidance-body consistency check.

The owning [installer contract](../../../../install/README.md#guidance-command-family)
has `guidance plan/install/get`. `plan` is read-only; `install/get` materialize
transitive Markdown closure and rewrite relative links. Installed outputs are
runtime destinations, not checked-in generated bodies.

W7 must add `mission/crates/mission/tests/fixtures/mission-native-guidance.toml` with direct
corpus item ID `mission-native-route`, the six Markdown source paths above
(product paths prefixed `workflow-tools/` from source root), and repo destination
scope. It must add `mission/crates/mission/tests/mission_native_routing.rs`, which resolves the
meta source root explicitly, uses a disposable target and builds/runs install-ctl
via its separate manifest. Test both initialized-submodule and consumer-root
layouts; never write user/system install locations. Planned exact operations:

```text
install-ctl guidance plan --source <meta-root> --profile <workflow-tools>/mission/crates/mission/tests/fixtures/mission-native-guidance.toml --select mission-native-route --target <disposable-target> --json
install-ctl guidance install --source <meta-root> --profile <workflow-tools>/mission/crates/mission/tests/fixtures/mission-native-guidance.toml --select mission-native-route --target <disposable-target> --json
```

W7 creates that profile/test; they are not current artifacts. Tests fail on
blocking diagnostics, duplicate destinations, missing selected bodies,
unresolved rewritten links or inconsistent routing. Compare installed content
against the install plan's rewrites, not raw byte equality across relocated
files; a second install must be unchanged. Check aliases where present,
explicit source hashes, required installed transitive closure, legacy read
fallback, the native new-run route, exact-review/approval boundary and failure
when capabilities are missing. Run the existing route checker too. Review the
orchestrator and mission-planning templates only for affected routing links;
do not copy the whole corpus or repair unrelated guidance.
Product sources/profile/tests and installer invocation belong in the product
checkpoint first; parent route prompts and parent gitlink are checkpointed
after it. No installed guidance is committed. W7 owns any selected-closure link
fix required to make its installer plan nonblocking; broader cleanup is excluded. — Mission-native MCP planning

Inventory date: 2026-10-29.  This is a planning inventory, not evidence that
the proposed surface is installed or available.

- Consumer W7 -> [provider W6](../551422d7-8a0f-4711-8642-a4c367649361/body.md) -> MN-W6-01, MN-W6-02, MN-W6-03, MN-W6-04, MN-W6-05.
- Consumer W7 -> [provider W4](../9fe60c69-af53-4578-9310-69283cd09284/body.md) -> MN-W4-01, MN-W4-02, MN-W4-03, MN-W4-04.
- [Existing provider](../38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) -> Mission Domain: generated projections and durable cross-session authority. Reuse form 3: link unchanged.

Use explicit owning roots for each domain operation and same-root read-back.
Dependencies consume provider outcomes/criteria; they do not transfer provider ownership.

## Behavior

- **MN-W7-01**: Edit parent .agents/prompts/{refine-ingest,execute-ingest,execute-roadmap}.prompt.md and product .agents/instructions/workflow/{prompt-ingestion,roadmap-authoring,roadmap-execution}.instructions.md plus tools/roadmap-route.py and install/README.md. Both .claude/commands links alias .agents/prompts; do not generate phantom .github guidance or a rule-target registry. Installed bodies are runtime guidance-install closure, not COMMANDS.md or artifact-catalog projections. Preserve planning-before-approval and no-execution-before-approval.
- **MN-W7-02**: Retain input cleaning/ambiguity clarification, bounded research, cold walkthrough, distinct formal review and exact requester decision gates. Route source capture through Session intake and use Mission-owned managed source/projection locations instead of manually scaffolded transcript folders.
- **MN-W7-03**: Require W6 capability/version discovery. Missing native capabilities or requester authority return actionable failures with no silent classic fallback or hand-written lifecycle script requirement.
- **MN-W7-04**: Preserve existing classic dossiers and active routes unchanged as provenance; mass migration, archive deletion and unsolicited active-route replacement are excluded. Explicit migration uses its existing validated separate route.
- **MN-W7-05**: Create mission/crates/mission/tests/fixtures/mission-native-guidance.toml with item mission-native-route selecting the six canonical Markdown paths from the explicit meta source root, and mission/crates/mission/tests/mission_native_routing.rs. Exercise install-ctl guidance plan/install from its separate manifest against disposable repo-scope targets both with initialized product submodule and without one. Require nonblocking closure, correct ownership destinations, rewritten-link integrity, source/installed routing parity, unchanged second install and alias correctness where present. Existing roadmap-route.py accepts classic/mission-backed only; add native-new-run and test all three modes without altering existing modes. Cover fresh native entry, stop-before-approval, missing capability and no manual fallback. Commit product sources/tests/profile first, then parent prompts and gitlink; no runtime installation bodies committed.

## Boundaries And Failure Cases

No contradictory planning-prohibition copies, implicit migration or claim that a tool-list alone proves installed acceptance. No product implementation or evidence of passing behavior is asserted by approval of this contract.

## Provider/Consumer Contract

The consumer may rely on the criterion IDs above only after their implementation evidence passes.

- Consumer W7 -> [provider W6](../551422d7-8a0f-4711-8642-a4c367649361/body.md) -> MN-W6-01, MN-W6-02, MN-W6-03, MN-W6-04, MN-W6-05.
- Consumer W7 -> [provider W4](../9fe60c69-af53-4578-9310-69283cd09284/body.md) -> MN-W4-01, MN-W4-02, MN-W4-03, MN-W4-04.
- [Existing provider](../38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) -> Mission Domain: generated projections and durable cross-session authority. Reuse form 3: link unchanged.

The next consumer links these IDs rather than copying their acceptance into its own authority.

## Examples

A fresh execute-ingest captures input through Session intake, discovers its Mission directory and prepares the plan through named MCP calls. It stops before execution for the exact approval; missing capabilities stop visibly rather than creating a classic folder.

## Evidence

Motivation: remove manual Mission intake/lifecycle chores while retaining requester control.
Dependent expectation: New intake defaults to Session-to-Mission MCP planning before exact requester approval while existing classic sources remain readable and opt-in migration stays explicit.

Planned guard: **G7 native-routing**, from
[ROADMAP validation gates](../../../../../transcripts/29-10-2026_mission-native-mcp-planning/ROADMAP.md#validation-gates).
Command from meta-workspace root:

```sh
cargo test --manifest-path workflow-tools/Cargo.toml --locked -p mission --all-features --test mission_native_routing -j 1
```

This is a declared validation plan, not an already registered test-api ValidationSpec
or a passing execution. No test-store records are created during this planning-only
request. Execution must register the owning guard and criterion/Ticket references
through test tools before recording the new behavior's results. Approval of a
Spec does not mean implemented or verified. Positions/code_ref values are above.

Owning implementation [Ticket](../../../ticket/tickets/d153d68f-50f0-49f4-96e6-e2e0258d1ef6/ticket.toml). State is planning until final review and supported lifecycle advancement; later read-back is in the dossier ledger.

## Scope

Excludes: Mass archive migration, production Mission mutation or changing an already active route. Implementation sequence and commit checkpoints remain in the
owning Ticket and roadmap; this Spec owns observable behavior only.
