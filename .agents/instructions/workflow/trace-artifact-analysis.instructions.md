---
description: "Use when analyzing agent traces together with generated artifacts. Covers evidence pairing, scenario-specific review, privacy, defect classification, and algorithm-improvement handoffs."
applyTo: "**"
---

# Trace and Artifact Analysis

Use this workflow when an agent run has produced an execution trace, one or
more generated artifacts, or both. The trace explains how the result was
produced; the artifact shows what a user or downstream process received.
Neither is sufficient alone.

## Evidence Model

Treat every run as an evidence bundle with explicit identities:

```text
run_id | agent/model | input references | trace reference | artifact references | environment | validation scope
```

Resolve the run, trace, and artifacts before interpreting quality. Prefer
durable files, stored records, or bounded CLI/MCP output over raw session
transcripts. Keep raw traces and large artifacts as named references; extract
only the windows or fields needed for the current question.

Pair evidence by stage and artifact identity:

| Evidence | Answers | Does not prove |
| --- | --- | --- |
| Input or latent representation | What the agent was given | That the agent used it correctly |
| System instruction and prompt | What the stage was asked to do | That the model followed the contract |
| Model response | What the stage attempted to produce | That the output is useful or safe |
| Final artifact | What the consumer received | Why the artifact has its defects |
| Validation result | Whether a stated property held | Unstated quality properties |

Never call a run successful from a prompt, a green test name, or a model
response alone. Read back the actual produced artifact and connect it to the
trace message that produced it.

## Review Workflow

Run the following steps in order. Stop and record a blocker when an identity,
artifact, or required trace segment cannot be resolved.

1. **Establish scope.** Record the run id, agent, model, input artifacts,
   requested outcome, environment, and the exact review question. Distinguish
   a production run, test fixture, replay, and synthetic example.
2. **Inventory outputs.** List every declared and produced artifact by stable
   id, name, format, and path. Check for missing, extra, duplicated, empty, or
   overwritten outputs.
3. **Build the trace map.** Summarize messages by sequence, stage, message
   type, artifact id, and outcome. Confirm stage transitions, prompt/response
   pairing, retries, errors, truncation, and terminal state.
4. **Check contract fidelity.** Compare the relevant instruction and prompt
   with the artifact's schema, language, section order, metadata boundary,
   grounding requirements, and refusal or safety rules.
5. **Read back artifacts.** Inspect the actual output, not only metadata. Check
   factual grounding, completeness, usefulness, audience fit, tone, structure,
   and whether recommendations are actionable rather than generic.
6. **Triangulate findings.** For every defect, cite the artifact location and
   the trace stage or input fact that explains it. Classify the defect as
   input, instruction, orchestration, model response, renderer/serialization,
   validation, or environment.
7. **Separate verdicts.** Report contract status, content quality, safety /
   privacy status, observability status, and reproducibility separately. A
   passing contract does not imply a high-quality artifact.
8. **Create the improvement handoff.** Convert confirmed findings into a
   small change hypothesis, an expected observable effect, a regression case,
   and a validation command or fixture comparison. Do not tune prompts from a
   single subjective preference without identifying the failed property.

## Scenario-Specific Practice

Choose the narrowest review profile that matches the run, then add only the
checks relevant to that profile.

| Scenario | Primary question | Required checks |
| --- | --- | --- |
| Single-agent generation | Did the agent produce the requested result? | Input grounding, instruction compliance, artifact schema, usefulness, unsupported claims |
| Multi-stage pipeline | Did information survive each stage? | Stage boundaries, latent/structured representation, field loss, artifact-to-stage association, ordering |
| Delegated sub-agent | Did the worker stay within its handoff? | Self-contained context, target path, allowed tools, one-step scope, return contract, blocker reporting |
| Planner/worker chain | Did planning improve execution without scope drift? | Compiled prompt, dependency handoff, worker result, retry count, escalation after repeated failure |
| Browser or CLI workflow | Did a user-observable workflow complete? | Auth boundary, visible state, downloaded artifact, API status, trace capability, redaction, no backend-only proxy evidence |
| Failure or partial run | Is the failure actionable and honest? | Stable error code/status, stage and cause, correlation handle, diagnostics, preserved partial state, no fabricated success |
| Prompt or algorithm experiment | Did the change improve the intended property? | Fixed input fixture, before/after trace, artifact diff, targeted metric, regression coverage, unchanged non-goals |
| Security/privacy review | Did observability avoid disclosure? | Credential and header scan, uploaded-content scope, capability ownership, TTL/retention, logs and frontend exposure |

For subjective quality, use a fixed rubric and compare equivalent runs. For
deterministic contract properties, prefer executable assertions. For live model
behavior, record model/version, prompt or instruction revision, temperature or
other relevant settings, and known nondeterminism; do not present one run as a
statistical conclusion.

## Trace Reading Rules

- Read a bounded skeleton or summary first, then targeted message windows.
- Preserve message sequence and stage names when summarizing; do not reorder
  messages to fit a narrative.
- Distinguish prompts, model responses, tool calls, execution errors, retries,
  and renderer output. Do not treat a prompt containing a desired value as
  evidence that the model produced that value.
- Check that each generation response has exactly one intended artifact and
  that the final artifact is the response actually persisted or downloaded.
- Treat `truncated`, missing terminal events, timeouts, and incomplete spans as
  observability blockers, not successful empty results.
- Redact credentials, authorization headers, capability secrets, private input
  content, and provider tokens from review notes unless disclosure is explicitly
  authorized and the material is demonstrably safe.

## Artifact Reading Rules

- Read the final serialized artifact through the same boundary a consumer uses
  when the review claims user-visible behavior.
- Compare declared and actual headings, item counts, language, formats, links,
  identifiers, and ordering.
- Mark facts as `source`, `inference`, `recommendation`, or `unsupported` when
  the distinction matters. Unsupported claims are defects even when they sound
  plausible.
- Check for generic advice that ignores the input, accidental leakage of
  internal metadata, prompt injection carried through source content, and
  recommendations that contradict explicit constraints.
- Record content defects with a short excerpt and a stable path or section;
  avoid copying whole private artifacts into reports.

## Improvement Handoff

Every algorithm-improvement finding should use this compact shape:

```text
scope: <stage, agent, artifact, or boundary>
finding: <observable defect or strength>
evidence: <trace sequence + artifact path/section + input reference>
hypothesis: <small instruction, orchestration, or algorithm change>
expected_effect: <observable change>
regression_fixture: <fixed input and expected property>
validation: <exact command or deterministic comparison>
non_goals: <properties intentionally unchanged>
```

Prefer one hypothesis per finding. Keep the captured trace and artifact
unchanged; create a new run or derived comparison for the experiment so the
baseline remains reproducible. If a finding cannot distinguish prompt,
orchestration, model, and renderer causes, label the uncertainty and collect
the smallest additional evidence needed before changing the algorithm.

## Completion Criteria

A trace/artifact review is complete only when:

- run identity and evidence references resolve;
- declared versus produced artifacts are reconciled;
- the trace has a stage-aware summary and terminal-state verdict;
- final artifacts were read back at the claimed consumer boundary;
- findings cite both trace and artifact evidence where applicable;
- privacy, observability, and reproducibility status are explicit; and
- each confirmed algorithm finding has a bounded experiment or follow-up
  handoff rather than an ungrounded prompt rewrite.

Cross-reference [tool-output.instructions.md](tool-output.instructions.md) for
output reduction, [data-capture-verification.instructions.md](../../../test/.agents/instructions/testing/data-capture-verification.instructions.md)
for artifact read-back, [end-to-end-testing.instructions.md](../../../test/.agents/instructions/testing/end-to-end-testing.instructions.md)
for user-observable evidence, and
[evidence-grounded-refinement.instructions.md](evidence-grounded-refinement.instructions.md)
for critique and refinement loops.