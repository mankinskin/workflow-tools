# Overwrite protection in sync-targets

## Context

Per the `14c0995c` reconciliation, `sync_targets_payload` (in `memory-api/tools/cli/rule-cli/src/cli/rendering.rs`) now treats a record as an orphaned generated artifact only when the output file exists **and** starts with `GENERATED_FILE_COMMENT`. Marker-free outputs are pruned from tracking state without touching the file.

The dangerous consequence: re-adding a target whose output path points at a hand-owned file will **silently overwrite it** on the first sync, with no conflict warning. This track re-adds targets for ~197 currently hand-owned files, so this is a live data-loss hazard, not a hypothetical.

## Scope

- Before writing, check whether the output file exists and lacks the provenance marker. If so, refuse and report.
- Add `--force` to override, and `--adopt` to import the existing file content into the store before generating over it.
- Apply the same guard to `rule install`.
- Cover the guard with a regression test that would fail against today's behaviour.

## Acceptance criteria

1. Generating over a marker-free existing file fails by default with a clear message naming the file.
2. `--force` overwrites; `--adopt` imports first, then generates.
3. The regression test fails on the pre-fix implementation.
