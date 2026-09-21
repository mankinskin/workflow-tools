## Problem
Current orchestration allows a multi-tier chain (large model -> smaller model -> smaller model) via orchestrator-delegation.instructions.md and model-routing.instructions.md. Each extra hop dilutes instructions and adds structural token overhead, and iteration cost from small-model mistakes offsets their cheaper per-token price.

## Goal
Write a spec defining a flattened two-tier architecture:
- Planner/Architect (frontier model): reads ticket + repo schema, outputs an immutable, rigid step-by-step execution plan (structured format, e.g. JSON) intended for direct execution rather than further re-planning by intermediate agents.
- Worker (fast/cheap model): executes exactly one isolated step from that plan against a specific target file/scope, then stops.
The spec should reconcile this with the existing orchestrator-delegation.instructions.md and model-routing.instructions.md tier ladder, stating explicitly what changes and what is preserved.

## Acceptance criteria
- Spec created under the spec store, linked to this ticket and to the current model-routing/orchestrator-delegation instruction files it modifies.
- Spec defines the plan schema, the boundary of what a Worker may/may not do, and how this differs from current multi-tier delegation.
- Open questions (e.g. how plans get validated before dispatch) are captured explicitly rather than assumed.

## Source
Derived from AGENT_WORKFLOW_OPTIMIZATIONS.md conversation, "Step 1: Tooling Restructure" and "The Two-Tier Architecture".

## Status
Spec authored at .spec/specs/1b654f30-d1a4-4cb4-ab2e-8355dfe5a758/spec.toml. Acceptance criteria met: 3/3. Validation: spec-cli store-index --check reports pre-existing unrelated store-wide drift. The live ticket store rejected a typed spec link, so the spec reference is recorded here as plain text.
Final review re-confirmed at HEAD after commit 975454f8: all 3 ACs met (spec created/linked, plan schema + Worker boundary defined, 6 open questions captured explicitly — capture-only AC, resolution deliberately deferred to follow-up tickets under epic f13c9836). No attributable validation failures. Closing.