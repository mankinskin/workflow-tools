Build a generator that reads the audit-api and emits a compact markdown summary of the current audit status at `.audit/README.md` along with its TOON sidecar at `.audit/index.toon`. The purpose is to give agents a committed, scannable audit snapshot without re-running the full audit on every read.

## Scope
- Implement a `store-index` subcommand (or extend `audit-cli`) that runs `audit run .` and formats the AuditReport into `.audit/README.md` and `.audit/index.toon` (TOON primary, D8).
- Include overall rating, severities table, and active findings summaries.
- Each finding is an `IndexEntry` with `ContentKind::audit_finding`; cross-references use `IndexRef`.
- Conforms to the `IndexEntry` schema (`0dba399a`); committed to git (D5).
- Emit an `.agents/` agent-hook entry pointing agents at the audit summary (D1).
- Highly performant; can be run during a profiled git pre-commit hook (D2) or on demand.

## Acceptance criteria
- Running the generator writes outputs under `.audit/` plus the `.agents/` hook.
- Findings are mapped as `IndexRef`s with severities and stable identifiers.
- Re-running with unchanged code is digest-stable.

## Non-goals
- No central store folder outside `.audit/`.
- Does not add new audit checks.

## Resolved design decisions
- D5: committed to git. D8: TOON sidecar. D2: profiled git hook / on-demand. D1: `.agents/` hook + `.audit/` workspace index.

## Review Pushback 2026-07-03

Pushed back from `in-review` to `in-implementation` because the digest-stability/current-output acceptance criterion is not met.

Validation run:

- Failed: `rtk cargo run --manifest-path memory-api/tools/cli/audit-cli/Cargo.toml -- store-index --check`
- Error: `audit store-index is out of date; regenerate and re-stage: .audit/README.md, .audit/index.toon`
- Validation spec recorded: `vt-review-audit-store-index-check-20260703`.

Required before review: regenerate the audit store index outputs, ensure `.agents/audit-catalog.md` remains synchronized, then rerun `audit store-index --check` to `drift:false`.