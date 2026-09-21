Build a generator that reads the spec store (spec-api) and emits a hierarchical markdown folder tree under `.spec/`, with `.spec/index.toon` as the machine-readable TOON sidecar. The purpose is to give agents a compact, navigable map of the spec hierarchy without scanning raw spec.toml files.

## Scope
- Implement a `store-index` subcommand (or extend `spec-cli`) that traverses the spec tree depth-first to **full depth** (D3) — there is no default depth cap.
- Output is written under `.spec/` as a directory tree: one markdown file per spec entry, with a canonical-named folder per entry holding its child entries, using relative links (e.g., `[My Child](./child/xyz/README.md)`).
- For each spec entry: emit title, component, state, summary, acceptance criteria excerpt, and an `IndexRef`.
- Conforms to the `IndexEntry` schema (`0dba399a`); outputs `.spec/index.toon` as the primary machine-readable format (TOON, D8).
- Emit an `.agents/` agent-hook entry pointing agents at the spec tree root (D1).
- Regenerates and commits all indices to git (D5) when spec.toml changes are staged in the pre-commit hook (D2).

## Acceptance criteria
- The entire spec tree maps to a full-depth relative markdown folder hierarchy under `.spec/` (one file per entry, one canonical folder per entry's children).
- Root `.spec/README.md` carries the table of contents.
- Sibling and child navigation uses relative markdown links.
- Co-located `.spec/index.toon` sidecar is generated.
- Digest is identical on unchanged inputs.

## Non-goals
- No central store folder outside `.spec/`.
- Does not duplicate full spec bodies.

## Resolved design decisions
- D3: full depth, one file per entry, one canonical-named folder per entry for its children, relative file links.
- D8: TOON sidecar primary. D2: pre-commit hook. D5: committed to git.

## Review Pushback 2026-07-03

Pushed back from `in-review` to `in-implementation` because the generated hierarchy is not current/digest-stable.

Validation run:

- Failed: `rtk cargo run --manifest-path memory-api/tools/cli/spec-cli/Cargo.toml -- store-index --check`
- Error: `spec store-index is out of date`
- Drifted outputs include `.agents/spec-catalog.md`, `.spec/README.md`, `.spec/index.toon`, `.spec/tree/memory-api/8074d6f7/workspace/ae5ef697/README.md`, child workspace tree pages including `path-normalization-kernel/b4833ecc`, plus `.spec/tree/runtime-logging/aa769a27/README.md` and `.spec/tree/transport-layer-e2e-matrix/76da5f2d/README.md`.
- Validation spec recorded: `vt-review-spec-store-index-check-20260703`.

Required before review: regenerate the spec store index outputs/tree, rerun `spec store-index --check` to `drift:false`, and keep the agent catalog synchronized.