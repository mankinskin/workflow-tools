Define extended cross-channel error envelope schema and mapping rules.

Scope:
- define required fields: code, message, request_id, details, cause_chain, hint, remediation_id
- specify channel mapping policy for CLI, MCP, HTTP
- define stability/versioning policy for machine consumers

Acceptance criteria:
- schema contract is documented and referenced by adoption tickets
- serialization/corruption examples are mapped with required context fields
