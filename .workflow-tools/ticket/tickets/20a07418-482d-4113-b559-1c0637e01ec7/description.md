## Observed Defect

`context-engine/.spec/specs/585aa074-356a-4169-b08b-4e3aba659a72/` holds a complete, valid, non-corrupt spec record (`spec.toml` + `body.md`, "Session Worktree Lifecycle Rewrite") on disk, yet:

- `spec.exe get 585aa074-... --workspace context-engine` returns `spec error: spec not found`.
- `spec.exe move 585aa074-... --workspace context-engine --to-workspace-root . --dry-run` returns `blocked` / `MissingSourceEntity`.
- `spec.exe scan --workspace context-engine --force` reports `integrated:1` but the integrated record is a *different* id (`e5294ae5-...`) that itself has no directory on disk (`context-engine/.spec/specs/e5294ae5-.../` does not exist), i.e. the entities.db / search index contains a stale/ghost row unrelated to the filesystem contents, and the real record on disk (`585aa074`) is never picked up by scan.
- Repeated forced scans are inconsistent: one run reported `pruned:78`, an immediately following identical run reported `pruned:0`.

## Reproduction

```
cd meta-workspace
./workflow-tools/target/release/spec.exe get 585aa074-356a-4169-b08b-4e3aba659a72 --workspace context-engine --json
./workflow-tools/target/release/spec.exe scan --workspace context-engine --force --json
./workflow-tools/target/release/spec.exe list --workspace context-engine --json
```

## Impact

This is a spec-api/spec-cli indexing defect, not spec content corruption. It was previously mischaracterized in `transcripts/30-08-2026_spec-system-improvement-planning/migration-execution.md` as a "pre-existing corrupt/incomplete legacy record" — that characterization is incorrect and should be read alongside this ticket. During the workflow-tools migration, the affected record's content was recreated at the destination store via `spec.exe create` (new id `1930fb77-774b-46ba-8a94-470c9ae45f20`, ticket_ids relinked to `5e6cf4f8-120c-4674-95de-d7b79c99f5b3`, body verified byte-identical) rather than moved, since the move primitive could not resolve the source entity.

## Suggested Remediation

Investigate why `spec.exe scan --force` does not reconcile entities.db with the actual `specs/<uuid>/` directory tree in a legacy `context-engine/.spec` store that has undergone heavy record migration (253 of 254 records moved out). Likely candidates: `scan_fingerprints.json` caching stale state, or a rescan pass that only compares against the search index rather than a full filesystem walk.
