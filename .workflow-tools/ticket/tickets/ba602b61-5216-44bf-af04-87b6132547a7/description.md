Goal: Create a coordination-only ticket to reconcile overlapping scope across tickets `e4f84414`, `25b5f3e7`, `742dbc65`, and `ba8f5528` without blocking their current progress.

Acceptance Criteria:
- Explicitly document overlapping areas and propose non-blocking scope boundaries.
- Provide suggested communication plan and ownership recommendations so work can proceed in parallel.
- Provide a sandbox-based plan for any shared artifacts; do not mutate live tickets or code.
- Include a risk matrix indicating potential conflicts and mitigation steps.

Notes:
- This ticket is coordination-only and must not block the other tickets; do not add `depends_on` edges to existing tickets.
- Link this ticket with `linked` edges to each of the four existing tickets listed above (non-blocking linkage).