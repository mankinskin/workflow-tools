Acceptance criteria:

- Native v2 and typed v3 waypoint outcome plans validate required fields and reject empty or missing content.
- Native v1 and typed v2 data remain readable, valid, and unchanged without synthesized outcomes; migration preserves legacy absence and new plan values.
- Unsupported versions and invalid plans return explicit diagnostics and preserve accepted source and generated artifacts.
- Roadmap and Part renderers include the plan deterministically with escaping; legacy output stays unchanged when the plan is absent.
- Alternative outcomes and handling never change acceptance, state, progress, dependency edges, scheduling, or execution.
- Actual operator_results retain their current independent validation and rendering semantics.