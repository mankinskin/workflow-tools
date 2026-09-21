# Round-trip idempotence test

## Scope

Prove `render → sync-rules → render` is a fixed point across the whole store:

1. Render every profile/surface pair from the rule store.
2. Reverse-sync every generated file back.
3. Re-render.
4. Assert the second render is byte-identical to the first, and that the rule store is unchanged apart from expected timestamps.

Additional cases:

- Mutate prose in a generated file, reverse-sync, re-render, and assert the mutation survives exactly once.
- Mutate frontmatter in a Copilot output and assert it lands in structured metadata and re-renders identically.
- Round-trip through the lossy Cline profile and assert no metadata loss.

## Acceptance criteria

1. Full-store round-trip is idempotent.
2. Prose and metadata mutations survive exactly one round-trip without duplication or loss.
3. Lossy-profile round-trip preserves unobservable metadata.
