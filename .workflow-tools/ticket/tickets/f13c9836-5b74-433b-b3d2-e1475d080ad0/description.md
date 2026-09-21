## Scope
Batches the tickets that define the Planner/Worker dispatch contract: the architecture spec plus the three instruction-level policies that depend on its vocabulary. Closes when all four children are done.

## Children (depends_on)
- Spec: two-tier Planner/Worker model routing architecture (do first, unlocks the other three)
- Retry-limit escalation policy for worker-tier test failures
- Write-and-die pattern for worker sub-agent dispatch
- Split-responsibility testing: frontier-authored tests, worker-only implementation

## Sequencing note
The spec ticket hard-blocks (`depends_on`) the three policy tickets since they use "worker-tier" terminology the spec formally defines. The three policy tickets are otherwise independent of each other and can run in parallel once the spec lands.
All four children done: feb5784c (spec), 1a240fdc (retry cap), 7563ce30 (write-and-die), 44c6cc5c (split-responsibility testing). Epic acceptance criteria satisfied.