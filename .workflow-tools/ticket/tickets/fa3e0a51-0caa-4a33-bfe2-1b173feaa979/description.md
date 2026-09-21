Implement fully automatic recursive store discovery across local and nested workspaces.

Scope:
- define traversal limits and deduplication semantics
- discover stores by known store markers
- persist discovered roots for indexing/reconciliation

Acceptance criteria:
- recursive discovery tests pass for nested workspace fixtures
- duplicate and loop-safe traversal behavior is documented
