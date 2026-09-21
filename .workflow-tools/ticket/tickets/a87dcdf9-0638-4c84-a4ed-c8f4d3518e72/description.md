Define operational rollout controls for scaffold automation, including feature flags, safe defaults, and rollback handling when regression gates fail.

Acceptance criteria:
- scaffold rollout has explicit progressive enablement stages
- failed drift/replay/e2e gates trigger deterministic rollback path
- operator-facing failure diagnostics and next-step guidance are documented