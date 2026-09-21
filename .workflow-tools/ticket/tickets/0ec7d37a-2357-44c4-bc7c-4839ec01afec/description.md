# Goal

Improve the `.rule/README.md` generated index so each entry is a single compact line ordered by section/slug tree rather than UUID, with collapsible slug-prefix groups.

## Problem

Current output has 5–7 line blocks per rule (provenance + H3 heading + summary paragraph + slug/section/tags/feedback/ref bullets). There are 200+ rule entries. The group key uses only the first two slug segments, so deeply nested slugs all cluster into one section. Summary paragraphs repeat body text the agent already has access to.

## Scope

**Refined per Q2-Q3 decisions:**

- Change `render_entry_block` in `rule-api/src/store_index.rs` so each entry is a single line:
  `- [slug-leaf](relative/path/to/rule.toml)`
  where `slug-leaf` is the last segment of the slug.

- **Q3 Link Format**: The entire entry line is a clickable link. No additional metadata (feedback count, low-rating badges, etc.) on the link line — just `[slug-leaf](path)`.

- **Q2 Collapsing**: Keep all groups open by default (expand all). Omit `<details>`/`<summary>` wrapping for clarity and simplicity in this version.

- Order within each group by slug lexicographically.
- Use three slug segments for the group key (not two) so deeply nested slugs get their own collapsible.
- Keep provenance comment unchanged.
- Update integration tests.

## Acceptance criteria

- Each rule entry is one line in format `- [slug-leaf](path)`.
- Groups use three-segment slug keys for section headers.
- All groups are open (expanded) by default.
- Each entry links to the relative `rule.toml`.
- Entries within a group are sorted lexicographically by full slug.
- `rule store-index --check` returns `drift:false`.

## Non-goals

- No change to sidecar format.
- No change to rule state machine.
- Low-rating visibility: surface low-rated rules as a separate filtered view later, not in the main index.
