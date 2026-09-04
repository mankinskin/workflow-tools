---
description: "Use when turning a request into production work or reporting validated results. Covers the closed-loop production cycle: request -> spec -> tickets -> tests -> implementation -> validated response -> next iteration."
applyTo: "**/*.md,**/*.toml"
---

## Core Cycle

Use this named production cycle to make work traceable from a request through
the user's next judgment. It complements [phase separation](phase-separation.instructions.md), which owns discovery versus implementation boundaries, and [loop closure](loop-closure.instructions.md), which owns Review -> Interview -> Commit -> Handoff.

| Stage | Enter when | Exit when | Handoff artifact |
| --- | --- | --- | --- |
| 1. Request | A user ask or raw transcript exists. | The intended outcome and open questions are captured. | The request or refined dossier. |
| 2. Spec | The work needs a durable goal and definition of success. | The spec defines the goal, owned acceptance criteria, and required traceability. | A reviewed spec and criterion references. |
| 3. Tickets | The spec is ready to plan implementation. | Tickets reference the spec and define executable slices. | Ticket plan, dependencies, and spec references. |
| 4. Tests | Tickets expose acceptance criteria to measure. | Validation evidence is recorded, or the criterion is explicitly documented without executable validation. | Test/validation records linked to the ticket, spec, and applicable criteria. |
| 5. Implementation | A planned ticket and its validation approach are available. | The scoped change and validation results are ready for the ticket review gate. | Implementation, documentation, and validation evidence. |
| 6. Validated response | Review and validation evidence are available. | The user receives an evidence-backed result they can judge. | Response with the relevant validation and traceability. |
| 7. Next iteration | The user has judged the response. | The judgment closes the loop or becomes the next request. | Recorded satisfaction or follow-up request. |

## Contract Rules

- Author the spec before tickets when the work requires a spec. Tickets plan implementation and reference the governing spec; they do not author or restate spec content.
- Each component owns its outward-facing acceptance criteria. When a component requires a child spec, [spec-system.instructions.md](../spec/spec-system.instructions.md#component-hierarchy) requires that separate entity; "component" never means an in-body record. Consumers reference the provider-owned criteria instead of duplicating them.
- An acceptance criterion without executable validation remains a valid documented criterion. Add automation when feasible, but do not block review on a missing `validated_by` record.
- Test evidence links the cycle's measurable validation stage: [validation evidence](../testing/validation-evidence.instructions.md) records `ticket_ids`, `spec_ids`, and applicable `acceptance_criterion_ids`.

## Owning Workflows

- [phase-separation.instructions.md](phase-separation.instructions.md) owns request discovery, planning, and implementation boundaries.
- [spec.prompt.md](../../prompts/spec.prompt.md) owns spec authoring.
- [ticket workflow](../ticket/workflow.instructions.md) and [ticket lifecycle](../ticket/lifecycle.instructions.md) own ticket planning, transitions, review, and closing.
- [loop-closure.instructions.md](loop-closure.instructions.md) owns the final Review -> Interview -> Commit -> Handoff iteration workflow.