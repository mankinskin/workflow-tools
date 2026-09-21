## Objective
Provide the release gate and forward-repair protocol for schema modernization.

## Requirements
- Require focused unit/contract checks, CLI/MCP/HTTP parity, TOML/JSON resolved-manifest fixtures, migration dry-run/idempotence/rollback/live-cutover/remediation checks, and Playwright client coverage.
- Track 6 blocks the entire release on any failed validation or forward-repair batch.
- Never roll back committed migration batches or restore a legacy exemption after migration completion.
- On failure, run one transactional corrective migration batch from current state. If the retry fails, keep release blocked and require a linked human-approved remediation review ticket.
- Preserve original immutable migration evidence. Corrective work appends an immutable `corrective-migration` part linked by `supersedes`; the latest approved superseding part is authoritative.

## Acceptance Criteria
The validation matrix is executable and release-blocking. Corrective migration, retry cap, review escalation, immutable evidence chaining, and repair-forward behavior have automated tests.


## Interface Matrix
Name CLI, MCP, HTTP, VS Code, every affected viewer, and search/indexing as schema-consumer surfaces. For each surface, require an executable compatibility check or an explicit, justified non-applicability record. Playwright evidence alone is not a substitute for the complete interface matrix.


## Recovered Release Gate Contract
- Track 6 starts only after a Track 5 migration-completion record exists.
- Maintain an executable, release-blocking interface matrix for CLI, MCP, HTTP, VS Code, every affected viewer, and search/indexing. Each surface has an executable check or an explicit justified non-applicability record. Persist an all-pass matrix artifact before release.
- Persist and enforce the one-corrective-retry counter. Resolve corrective authority through the latest approved immutable `supersedes` chain.
- After a second failure, a human-approved remediation review ticket is required. A rejected remediation review closes that approval path; any later corrective batch requires a new remediation review ticket and fresh human approval. Release remains blocked, completed batches are never rolled back, and all corrective work remains idempotent forward repair from current state.

## Additional Acceptance Criteria
Tests cover Track 5 completion gating, every interface-matrix surface or justified exclusion, all-pass matrix persistence, retry-counter enforcement, latest-approved supersedes selection, rejected-review closure, fresh-review requirement, and blocked-release behavior.