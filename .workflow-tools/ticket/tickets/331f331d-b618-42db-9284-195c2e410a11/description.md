## Summary
Implement v1 reverse-sync for generated rule artifacts using canonical `rule-api:entry` ids so generated-file edits can update existing rule bodies in place.

## Scope
- Add `rule sync-rules --file <path>` command and args plumbing.
- Parse generated artifact format (`rule-api:file generated=true` + `rule-api:entry id=...`) into ordered `(id, body)` entries.
- Apply updates by id via store get/update_body with drift-only modes.
- Reject non-generated files and spec-doc artifacts.

## Out of Scope
- `--config` + `--target` convenience selector (follow-up).
- Reverse-sync for spec-doc artifacts (follow-up).

## Acceptance Criteria
1. Command exists: `rule sync-rules --file <path> [--dry-run] [--check]`.
2. Non-generated file inputs fail early with guard error.
3. Spec-doc file inputs fail with unsupported reverse-sync error.
4. Sync updates only existing rules referenced by entry ids.
5. Unknown entry ids produce a clear orphan-id failure.
6. Marker `id` is authoritative and marker `slug` is non-authoritative metadata; edited slug values do not redirect updates by slug.
7. `--dry-run` reports per-entry changed status and performs no writes.
8. `--check` exits non-zero when drift exists.
9. Frontmatter is round-tripped for generated artifacts that hoist YAML frontmatter above the generated marker by re-attaching it to the first entry body during reverse-sync.
10. Reverse-sync preserves forward normalization semantics for trim and line endings so regenerate checks do not fail on normalization-only diffs.
11. Apply semantics are atomic at command scope: any entry failure prevents all writes.
12. Round-trip regression uses a frontmatter-bearing generated artifact (roast agent target): generate -> edit artifact -> sync-rules -> generate --check clean.

## Traceability
- Spec: rule-system/single-target-reverse-sync
- Spec id: a969562b-6920-4fa6-a757-d317e7d442df

## Validation Plan
- `cargo test -p rule-cli` focused command/parser tests.
- Add/extend tests in command and generate-target suites for frontmatter path, trim/line-ending normalization, id-authoritative marker parsing, and atomic failure behavior.