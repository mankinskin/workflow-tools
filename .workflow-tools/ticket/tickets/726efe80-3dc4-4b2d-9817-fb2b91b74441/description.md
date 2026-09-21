Adopt extended error envelope in HTTP surfaces and trace correlation.

Scope:
- align ticket-http/spec-http/peer handlers to envelope contract
- ensure request_id and cause_chain propagation
- include actionable hint and remediation_id in relevant failures

Acceptance criteria:
- HTTP error payloads include required envelope fields
- integration tests verify request_id and causal context propagation
