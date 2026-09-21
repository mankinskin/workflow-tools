## Objective
Refine spec `9f0b9e30` with deterministic model-rejection guidance.

## Context
Owner decision: “N=5. If the model is NOT FOUND → nearest matches by edit distance. If the model is TOO EXPENSIVE → list viable models highest-tier-first, cheapest within each tier. Explicitly: suggest the cheapest model within the highest tier that can use the tool, NOT the cheapest model overall.”
This sharpening was escalated because the spec lacked N and a comparator.

## Acceptance criteria
- Specify N=5 and edit-distance matching.
- Specify tier ordering and within-tier comparator.
- Define no-match and no-viable-model outcomes.

## Out of scope
- Implementing guidance generation.