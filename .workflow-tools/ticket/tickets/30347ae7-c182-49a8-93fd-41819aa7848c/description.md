Goal: Add opt-in migration and re-render support for legacy handoff records (preserve historical handoff `a9519525`).

Acceptance Criteria:
- Sandbox regeneration: a CLI/mechanism to regenerate handoff `a9519525` into an isolated sandbox or ephemeral store, documented steps.
- Current-schema JSON: regenerated artifacts must include a current-schema JSON output (no live ticket-store mutation).
- Markdown derivation: a Markdown rendering must be derivable from the regenerated JSON programmatically.
- Zero live-ticket-store mutation: the implementation must not alter existing live tickets; all migrations run in sandbox unless explicitly opted-in by a user.

Notes:
- Preserve original handoff id `a9519525` as historical record reference. Store replacement proof in sandbox-only stores.
- Traceability: reference spec `5e52039d` for spec-cleanup coordination.