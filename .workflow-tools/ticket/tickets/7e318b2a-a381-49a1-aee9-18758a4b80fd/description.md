Support absent-then-present store integration and late onboarding reconciliation.

Scope:
- handle references to stores not present yet
- reconcile when store appears later without destructive rebuilds
- surface per-store integration status in scan/index outputs

Acceptance criteria:
- absent-then-present integration tests pass
- scan reports include discovered/integrated/diagnostic store-level fields
