# Goal

Improve the `.spec/README.md` generated index so each entry is a compact single-line link rendered in true hierarchy order with collapsible component groups.

## Problem

Current output has 6–9 line blocks per spec (provenance comment + H3 heading + multi-sentence summary + slug/scope/tag/children/ref bullets). With ~200 specs and full multi-paragraph metadata this makes the file too long to scan. Ordering is alphabetical by component and then by UUID, which does not reflect the parent/child hierarchy. There is no collapsing.

## Scope

**Refined per Q2, Q4-Q5 decisions:**

- Change `render_entry_block` in `spec-api/src/store_index.rs` so each entry is a single line:
  `- [indent] [slug](relative/path/to/spec.toml)`
  where `[indent]` is `  ` (2 spaces) per depth level in the parent/child tree.
  
- **Q4 Parent-Child Structure**: Children specs are indented on their own line directly under their parent. Each child appears at `depth_level * 2` spaces (2 spaces per level). Tree traversal is DFS (depth-first search).

- **Q2 Collapsing**: Keep all component groups open by default (expand all). Omit `<details>`/`<summary>` wrapping for clarity and simplicity in this version.

- **Q3 Link Format**: The entire entry line is a clickable link. No additional metadata on the link line — just `- [indent] [slug](path)`.

- **Q5 Spec Cross-Links**: Within specs, eagerly link to affected tickets and related specs using inline list format. For example, in the description section: "Related specs: [spec-a](path), [spec-b](path) | Affected tickets: [t1](path), [t2](path)".

- Order entries within each component group hierarchically: root specs first, children indented below their parent, DFS traversal order.
- Summary suffix should be the first sentence only (truncated at 120 chars), not the full block.
- Keep provenance comment unchanged.
- Update integration tests.

## Acceptance criteria

- Each spec entry occupies one line in the rendered README.
- Children appear indented directly under their parent in DFS order.
- Indentation is 2 spaces per depth level.
- The link target is the relative path to `spec.toml`.
- All component sections are open (expanded) by default.
- Related specs and tickets are linked with inline list format where applicable.
- Re-running `spec store-index --check` returns `drift:false`.

## Non-goals

- No change to sidecar format.
- No change to spec state machine.
