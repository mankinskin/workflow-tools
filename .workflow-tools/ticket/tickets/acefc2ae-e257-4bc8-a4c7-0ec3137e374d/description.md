Plan how ticket dependencies can declare required validation items whose satisfaction is resolved through test-api evidence rather than ad hoc ticket text.

Scope:
- extend dependency metadata with validation requirements and acceptance targeting
- derive dependency satisfaction and risk from ValidationSpec and ValidationExecution records
- surface reusable health and next/flow-graph signals from one shared ticket-api model

Acceptance criteria:
- spec defines dependency-level validation requirement semantics and failure states
- plan identifies test-api integration points using current ValidationSpec/ValidationExecution links
- validation plan covers passed, failed, blocked, stale, and missing evidence cases
