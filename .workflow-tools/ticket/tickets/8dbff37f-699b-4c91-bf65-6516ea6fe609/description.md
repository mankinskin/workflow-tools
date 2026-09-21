Plan the audit and operator-enforcement layer for validation-aware ticket graphs.

Scope:
- extend audit health beyond orphan/convergence checks to dependency validation readiness
- warn on check-in when a ticket lacks required validation requirements or depends on unmet review/test gates
- report dependency risks involving in-review prerequisites, failed dependency evidence, and missing validation links

Acceptance criteria:
- spec defines audit findings, board warnings, and severity mapping
- plan reuses ticket-api derived graph/validation state instead of duplicating heuristics
- validation plan covers repo audit, ticket health, and board check-in warning paths
