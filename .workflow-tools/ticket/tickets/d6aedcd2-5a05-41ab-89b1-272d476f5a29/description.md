Previous Part: [07-waypoint-outcome-plan.md](07-waypoint-outcome-plan.md)  
Next Part: none

# Part 08 — Representative generated Mission fixtures

## Outcome

Provide realistic, permanently inspectable Mission examples inspired by real
transcript roadmaps, and exercise the same corpus as automated test fixtures.
Replace the representativeness gap, not the useful small W5 negative cases.

The requester explicitly asked to append this work on 2026-10-29 because the
existing examples do not represent production scope. This Part plans that
extension; it does not execute it or inherit W1-W7 execution approval.

## Source scope and representativeness

Use three locally available roadmaps as read-only inspiration:

| Example | Source | Required scale and content |
|---|---|---|
| Documentation workflow | [Doc-comment standardization](../../03-09-2026_doc-comment-standardization/ROADMAP.md) | All four phases: contract, health checks, code-map extraction, semantic candidate analysis; retain distinct outcomes, validation and human-review boundaries. |
| Mission integration | [Mission domain integration](../../01-10-2026_roadmap-state-integration/ROADMAP.md) | All nine phases; specification, implementation, transports, session attachment, migration and closure provide realistic multi-surface content and relationships. |
| Ticket reliability and migration | [Ticket-part reliability](../../12-10-2026_ticket-part-empty-files/ROADMAP.md) | All seventeen phases; preserve the distinction between completed work, migration preflight, blocked apply and remaining integration, including separate operation approval. |

Before authoring, read each selected source's index and cited Parts, not only
its roadmap summary. Record dated source paths, SHA-256 hashes, source-to-fixture
waypoint mappings and every adaptation. Count alone is insufficient: preserve
the distinct objectives, constraints, validation methods, non-goals, realistic
Part detail and blocked/approval semantics. Do not pad the count with repeated
placeholder waypoints or flatten the source to a one-waypoint illustration.

These are synthetic adaptations, not imports or faithful runtime replicas of
the original entities. Keep original IDs and observations as provenance only;
use deterministic fixture-only identities for executable references. Any
authored expected/alternative outcomes absent from the source must be labelled
fixture-authored guidance, not attributed to the original requester.
Document how the Mission's serial order represents the source; do not invent
cross-mission dependencies or claim that a source's independent work was
originally serial.

## Expected post-state

- One discoverable review index links all three examples, their source
  mappings, regeneration command, and intended review focus. Each example
  has its own generated entry, roadmap, waypoint map and all Parts.
- Commit canonical native-v2/typed-v3 fixture authoring inputs and the actual
  output produced by the final Mission APIs. Do not hand-author or patch
  generated README, roadmap, Parts, provenance, contracts or history.
- Every waypoint has complete planned expected/alternative outcomes and
  acceptable manual handling. Across the corpus, include local fixture
  Ticket, Spec/criterion and Test/evidence relationships, and a legitimate
  no-Ticket waypoint. Link only to isolated synthetic records, never mutable
  live records as a prerequisite for regeneration.
- At least one example has two coherent published revisions, changed input
  and a preserved continuation, whole-state archives, a verifiable contract
  inventory and a separate contract-bound reaction. Label reactions and
  validation receipts synthetic; no genuine human approval or authentication
  may be fabricated. Include an actual operator-result example separately
  from its planned outcomes.
- Both the compact reviewer corpus and the new production-scale corpus remain
  available. Add controlled missing-reference, output-drift and malformed-plan
  test variants derived from the new sources without corrupting the canonical
  reviewable examples.
- Store the corpus under the owning Mission test surface, planned
  `mission/crates/mission-api/tests/fixtures/representative-missions/`,
  with a nearby review index and regeneration recipe. Tests use these same
  inputs and expected outputs, not a second simplified parallel corpus.

## Validation and review

Implement `representative_mission_fixtures` as an integration-test target.
Its regeneration mode must use explicit temporary workspace roots and real
Mission operations; normal tests compare without rewriting committed fixtures.
Use fixed fixture-authored UUIDs for import/publish inputs. The current
implemented approval-event path generates `Uuid::new_v4()` and
`SystemTime::now()` in
[`output_history.rs`](../../workflow-tools/mission/crates/mission-api/src/output_history.rs);
there is no existing controllable time/event-identity seam. Therefore W8 must
first add one narrowly scoped, test-only `MissionStore` event-value provider
(fixed event UUID plus Unix-millisecond timestamp) used only by the fixture
harness, while production callers retain the current random/clock provider.
The integration test must prove that two fresh isolated roots using the fixed
provider produce byte-identical generated trees, archives, contracts, and
event bindings. Do not strip arbitrary fields, patch generated output, or
bless regenerated drift to conceal nondeterminism.

The G8 gate verifies:

1. Three examples and exact waypoint counts 4/9/17; full source coverage and
   substantive, distinct Parts; no invented source approval or completion.
2. Accepted native/typed schemas and exact round-trip plan preservation.
3. Generated file-set and byte equality against committed examples, including
   provenance, contracts, archives and event bindings where present.
4. Same-store read-back, `check-generated` and history verification pass for
   canonical examples. Local relative destinations resolve in the documented
   publication context; any archive-link limitation remains explicit.
5. Negative variants fail with the intended diagnostic and preserve accepted
   source/output bytes. Their failures do not turn canonical examples red.
6. Original transcript dossiers and live stores remain unchanged; the harness
   neither migrates nor executes any source roadmap or its validation commands.
7. Manual walkthrough from the review index reaches each roadmap, Parts,
   outcomes, references and history. Record observations separately from the
   requester's later review feedback; delivering examples is not human approval.

Run from the meta-workspace root:

```text
cargo test --manifest-path workflow-tools/Cargo.toml --locked -p mission-api --test representative_mission_fixtures
cargo test --manifest-path workflow-tools/Cargo.toml --locked -p mission-api -p mission --all-features -j 1
```

The target and corpus are planned outputs, not existing tools or passing
evidence. Record the implemented regeneration command and actual validation
results before marking W8 done. This is one ticket-backed package. After the
parent accepts this plan, the execution owner must prepare the single Ticket
through the domain tools, read it back by its actual ID, and then attach that
ID here and in the roadmap before implementation starts. This dossier creates
no entity and records no fake ID; no separate Spec is planned unless that
preparation finds a contract that cannot be bounded by the Ticket and tests.

## Boundaries and failure handling

No product-feature expansion, live Mission publication, historical repair,
installed-tool update or source-roadmap execution. Preserve unrelated dirty
files and all completed W1-W7 evidence. Refresh ownership before edits.
If source coverage, reproducibility or isolation cannot be established, leave
W8 incomplete and report the exact discrepancy; do not reduce it to toy cases
or silently rewrite source meaning.

Checkpoint after validation and acceptance review:
`test(mission-api): add representative generated mission fixtures`.
Integrate nested repositories before parent gitlinks if affected.
