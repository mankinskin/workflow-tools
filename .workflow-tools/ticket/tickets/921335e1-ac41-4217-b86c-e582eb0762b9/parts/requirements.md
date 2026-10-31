Requirements:

- Native bundle schema v2 and typed source schema v3 represent a complete outcome plan for each new waypoint.
- The plan has a non-empty expected outcome and at least one alternative outcome; each alternative has a non-empty outcome and one or more non-empty acceptable-handling options.
- Legacy native v1 and typed v2 records remain readable and renderable without fabricated plans. Preserve native v1-to-typed-v2 migration without backfill; preserve native v2 plans losslessly in typed v3.
- Version-specific validation rejects missing or malformed new-version plans and unsupported versions without changing accepted data.
- Render outcome guidance deterministically in the generated roadmap and Part. Keep actual operator_results unchanged and separate.
- Alternative text is advisory only; it does not change acceptance, progress, state, dependencies, scheduling, or dispatch.