Define the core shared contract crate for cross-store interaction primitives.

Scope:
- define shared trait and model primitives used across domains
- establish ownership and versioning boundaries
- document crate dependency constraints

Acceptance criteria:
- core contract crate compiles with no domain logic
- dependency DAG checks pass
- crate API baseline is documented
