## Summary
Deduplicated `spec-system.instructions.md` and added explicit precedence/exception handling aligned with the AGENTS instruction-precedence matrix.

## Changes
- Removed the duplicated second copy of the full spec-system guidance block.
- Added explicit precedence/exception bullets under Workflow Expectations:
  - path-scoped precedence over AGENTS defaults for matching files
  - same-specificity conflict handling
  - tooling-constraint fallback documentation requirement

## Validation
- Root-scoped `next` check for `e1d8be15` executed.
- Root-scoped health check for `e1d8be15` executed.
- Tracker dependency frontier/blockers remain structurally consistent after cleanup.
