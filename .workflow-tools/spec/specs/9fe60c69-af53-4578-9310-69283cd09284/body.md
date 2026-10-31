<!-- aligned-structure:v2 -->
# Requester Decision Authority and Revision-bound Execution Grant

## Target Code Location

- [mission/crates/mission-api/src/output_history.rs](../../../../mission/crates/mission-api/src/output_history.rs) — `code_ref: mission/crates/mission-api/src/output_history.rs`; position: `not-implemented` for this new contract (existing provider behavior is not reclassified).
- [mission/crates/mission-api/src/typed_source.rs](../../../../mission/crates/mission-api/src/typed_source.rs) — `code_ref: mission/crates/mission-api/src/typed_source.rs`; position: `not-implemented` for this new contract (existing provider behavior is not reclassified).
- [mission/crates/mission/src/lib.rs](../../../../mission/crates/mission/src/lib.rs) — `code_ref: mission/crates/mission/src/lib.rs`; position: `not-implemented` for this new contract (existing provider behavior is not reclassified).
- [session/crates/session-api/src/store/config/capture_query.rs](../../../../session/crates/session-api/src/store/config/capture_query.rs) — `code_ref: session/crates/session-api/src/store/config/capture_query.rs`; position: `not-implemented` for this new contract (existing provider behavior is not reclassified).

- [session/src/bin/session-capture-hook/args.rs](../../../../session/src/bin/session-capture-hook/args.rs) - `code_ref: session/src/bin/session-capture-hook/args.rs`; position: `not-implemented` for this new behavior.
- [session/src/bin/session-capture-hook/main.rs](../../../../session/src/bin/session-capture-hook/main.rs) - `code_ref: session/src/bin/session-capture-hook/main.rs`; position: `not-implemented` for this new behavior.
- [session/tests/copilot_capture_hook_e2e.rs](../../../../session/tests/copilot_capture_hook_e2e.rs) - `code_ref: session/tests/copilot_capture_hook_e2e.rs`; position: `not-implemented` for this new behavior.

## Naming Conventions

Separate PlanReview, RequesterDecision reference and ExecutionGrant concepts; free-text actor and generic reaction are not authority credentials. Acceptance criterion identifiers are `MN-W4-NN`.

## Requester Input

> Vertiefe die Planung und erstelle und ordne Tickets und Specs in der Roadmap zu.

The full source scope and earlier explicit lifecycle/trait decisions are preserved in
[merged clean intent](../../../../../transcripts/29-10-2026_mission-native-mcp-planning/inputs/merged.clean.md).

## Reading Order

1. [Composition root](../92c78a48-cad8-4c08-8bba-4a523b717842/body.md) — shared invariants and component map.
2. [Owning Part](../../../../../transcripts/29-10-2026_mission-native-mcp-planning/04-revision-bound-approval.md) — execution package, not a competing behavior authority.
3. [Mission Domain](../38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) — stable identity, serial order and foreign authority.

- Consumer W4 -> [provider W3](../c3385d21-983e-4fb5-92f7-2a6ad757677a/body.md) -> MN-W3-01, MN-W3-04, MN-W3-05.
- [Existing provider](../ec5f9329-041e-487e-b30b-2f27d5e06522/body.md) -> Whole-state history: coherent revisions and contract-linked reactions, not execution authority. Reuse form 3: link unchanged.

## Responsibility

Consumers can activate a Mission only when both current-revision reviews pass and an actual requester decision approves the exact Mission, plan revision and digest. This new behavior slice is not already guaranteed by its historical providers.

## Interfaces And Dependencies

Use the existing host `UserPromptSubmit` hook, not agent-authored actor text.
[args.rs](../../../../session/src/bin/session-capture-hook/args.rs)
reads stdin `prompt`; [main.rs](../../../../session/src/bin/session-capture-hook/main.rs)
builds the event and persists it before transcript flush, including main-checkout
mirroring. Existing hook events have no event/message ID: W4 must implement
durable identity, not assume one.
[capture_query.rs](../../../../session/crates/session-api/src/store/config/capture_query.rs)
and [existing hook E2E](../../../../session/tests/copilot_capture_hook_e2e.rs)
are the persistence and process-level test owners.
The [actual workspace hook registration](../../../../../.github/hooks/hooks.json)
already binds `UserPromptSubmit` to `session-capture-hook --from-hook-stdin`.
This is configured ingress, not proof of actual host delivery or human approval.
W8 records the exact installed capture-hook executable/version resolved by the
pilot host, not merely the Mission server version or a source-only test.

The rendered review gives the requester a complete line:
`mission-decision approve <mission-uuid> <revision> <plan-sha256> <review-snapshot-sha256> <challenge-id>`.
`reject` and `replan` replace `approve` with the same exact binding.
The requester submits that line as a new host user prompt. W4 creates and
persists a pending decision challenge before presenting it; it binds the
Mission, plan/review identities, explicit Mission/Session roots and active
requester Session. No guessed intent, generic "yes", reaction or tool argument
creates a decision.
Session owns pending challenges and immutable receipts under its managed
Session directory. Mission `prepare_review` calls the Session adapter to
register the binding, then journals the returned reference before exposing
the prompt. Mission owns reviews/grants. Interrupted registration, linkage
and consumption report the persisted pending subset and recover through the
cross-store operation journal. Retaining an old Session challenge never
bypasses Mission's current-review CAS check.

Planned Session owner:
`session/crates/session-api/src/requester_decision.rs`, re-exported by its
`lib.rs`, plus the existing capture-hook args/main and event persistence path.
It recognizes only a complete standalone line on the host user-prompt ingress,
stores immutable submitted bytes/digest and provenance, and resolves
`RequesterDecisionRef { session_id, decision_id, source_event_id }`.
Use an actual host message/event ID when present. When absent, allocate one
persisted source-event ID, and make the receipt idempotent on
`(session_id, challenge_id, decision, full binding)`; rereading/replaying the
same bytes returns that same receipt. Different decision/binding for an already
consumed challenge is a conflict. Persist event+receipt transactionally or with
an explicit recoverable journal; no receipt without its raw source event.
Main/worktree mirroring retains identity, rather than minting two decisions.
The adapter exposes read-only `resolve_requester_decision` to Mission; public
agent tools cannot manufacture a receipt from `actor`, arbitrary content or
an approval boolean. Hook protocol is a local trusted-host integration, **not
cryptographic proof against an actor with filesystem/process access**.

Planned Mission consumer: `mission/crates/mission/src/approval.rs`, consuming
`session-api` through an explicit new facade dependency (never the reverse).
Resolve the reference from the bound Session root, verify raw bytes, ingress,
challenge, requester Session, revision/digests and both passing review snapshots,
then CAS-append approval. Approval consumes the challenge; same operation replay
returns the accepted record, divergent replay conflicts. Finished Sessions may
be read for prior evidence but cannot mint a new decision; handoff requires
a fresh challenge bound to the active authorized requester Session. A failed
Mission write leaves a visible pending/reconcilable receipt, never approval.

W4 must extend G4 to `mission-api`, `mission`, `session-api` **and**
`session` with all features, covering hook subprocess stdin -> stored event ->
receipt resolution -> Mission gate, missing hook/ID, duplicate mirror, stale
binding, forged assistant/tool messages, reject/replan, finished Session and
partial persistence. Synthetic fixture stdin tests verify plumbing, not actual
human authorization. W8 separately records a **real host** prompt submission,
stored reference and exact read-back against installed binaries. If that host
does not deliver UserPromptSubmit, the pilot fails explicitly; no hand-authored
receipt or CLI approval substitutes for it.

- Consumer W4 -> [provider W3](../c3385d21-983e-4fb5-92f7-2a6ad757677a/body.md) -> MN-W3-01, MN-W3-04, MN-W3-05.
- [Existing provider](../ec5f9329-041e-487e-b30b-2f27d5e06522/body.md) -> Whole-state history: coherent revisions and contract-linked reactions, not execution authority. Reuse form 3: link unchanged.

Use explicit owning roots for each domain operation and same-root read-back.
Dependencies consume provider outcomes/criteria; they do not transfer provider ownership.

## Behavior

- **MN-W4-01**: Prepare review by recording a cold walkthrough then a distinct formal review on the same immutable plan revision/digest and named evidence snapshot. Missing/failed/stale verdicts cannot be presented as ready approval.
- **MN-W4-02**: Implement the host UserPromptSubmit ingress in session-capture-hook args/main and event persistence, Session receipt/resolver in session-api/src/requester_decision.rs (re-export lib.rs), and Mission consumer in mission/src/approval.rs with a one-way session-api facade dependency. A persisted pending challenge binds Mission/Session roots, active requester Session, revision, plan digest and review-snapshot digest. Accept only the complete standalone host prompt 'mission-decision approve|reject|replan <mission-uuid> <revision> <plan-sha256> <review-snapshot-sha256> <challenge-id>'. Preserve raw prompt bytes/digest and immutable provenance; return RequesterDecisionRef {session_id, decision_id, source_event_id}. Preserve a host event/message ID when available; otherwise persist an allocated event ID and idempotent receipt on Session/challenge/decision/full binding. Mirroring and replay retain one identity. Agent actor text, tool/assistant content, generic reactions and missing host ingress return approval_authority_unavailable, never a grant.
- **MN-W4-03**: Only matching affirmative requester approval creates an execution grant. Semantic plan changes invalidate reviews and grant; changes-requested does not grant execution. Read-only regeneration does not invalidate approval.
- **MN-W4-04**: Every activation, dispatch and resume entry invokes the same domain approval check and re-reads authoritative foreign prerequisites. Missing or stale approval returns approval_required or approval_stale, regardless of CLI/MCP/generic adapter.
- **MN-W4-05**: Append-only progress/results retain approved-plan binding. Resolve a receipt from its explicit Session root and verify stored raw event, ingress, binding and current reviews before CAS grant creation/challenge consumption. Same operation replay returns the accepted record; divergent reuse conflicts. Finished Sessions permit historical reads only, not a new receipt; authorized handoff requires a fresh challenge. Event/receipt persistence is transactional or recoverably journaled; failed Mission acceptance leaves a visible pending receipt, not approval. G4 covers the real hook subprocess path through Session persistence to Mission, missing IDs/hooks, mirror replay, stale/forged data, reject/replan, finished Session and injected partial writes. Synthetic tests are not human-approval evidence; W8 must prove actual installed host ingress.

## Boundaries And Failure Cases

Host-adapter absence is a failing supported-path gate, not permission to fabricate authority. Distinguish machine readiness, historical reaction and execution authorization. No product implementation or evidence of passing behavior is asserted by approval of this contract.

## Provider/Consumer Contract

The consumer may rely on the criterion IDs above only after their implementation evidence passes.

- Consumer W4 -> [provider W3](../c3385d21-983e-4fb5-92f7-2a6ad757677a/body.md) -> MN-W3-01, MN-W3-04, MN-W3-05.
- [Existing provider](../ec5f9329-041e-487e-b30b-2f27d5e06522/body.md) -> Whole-state history: coherent revisions and contract-linked reactions, not execution authority. Reuse form 3: link unchanged.

The next consumer links these IDs rather than copying their acceptance into its own authority.

## Examples

Plan digest A passes two reviews; the requester message approving A grants A only. Editing a requirement creates B and blocks execution until B is reviewed and approved. An automated fixture saying actor=requester cannot approve B.

## Evidence

Motivation: remove manual Mission intake/lifecycle chores while retaining requester control.
Dependent expectation: Consumers can activate a Mission only when both current-revision reviews pass and an actual requester decision approves the exact Mission, plan revision and digest.

Planned guard: **G4 requester-approval**, from
[ROADMAP validation gates](../../../../../transcripts/29-10-2026_mission-native-mcp-planning/ROADMAP.md#validation-gates).
Command from meta-workspace root:

```sh
cargo test --manifest-path workflow-tools/Cargo.toml --locked -p mission-api -p mission -p session-api -p session --all-features -j 1
```

This is a declared validation plan, not an already registered test-api ValidationSpec
or a passing execution. No test-store records are created during this planning-only
request. Execution must register the owning guard and criterion/Ticket references
through test tools before recording the new behavior's results. Approval of a
Spec does not mean implemented or verified. Positions/code_ref values are above.

Owning implementation [Ticket](../../../ticket/tickets/1d6816b5-92d6-4a08-858a-94df3a05b3f4/ticket.toml). State is planning until final review and supported lifecycle advancement; later read-back is in the dossier ledger.

## Scope

Excludes: Executing work, cryptographic identity claims or accepting a generic reaction as a grant. Implementation sequence and commit checkpoints remain in the
owning Ticket and roadmap; this Spec owns observable behavior only.
