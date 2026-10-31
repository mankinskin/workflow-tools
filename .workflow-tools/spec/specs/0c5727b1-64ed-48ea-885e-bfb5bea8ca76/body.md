# Shared Markdown link parsing (`markdown-links`)

Dossier: `transcripts/31-10-2026_shared-markdown-links/` (decisions D1–D11 in REVIEW.md).

## Purpose

One CommonMark-based implementation of Markdown link syntax for audit and
install-ctl. It owns *syntax*: finding real links, their positions, anchors,
destination shape, and path resolution. Consumers own *rules*: which targets
are guidance, what blocks, and how findings or rewrites are reported.

## Location and dependencies

- Crate `workflow-tools/audit/crates/markdown-links/` (package `markdown-links`, lib `markdown_links`).
- Only dependency: `pulldown-cmark 0.12`, `default-features = false`.
- `audit-api` re-exports it as `audit_api::markdown`.
- install-ctl depends on it directly (git `https://github.com/mankinskin/audit`, branch `main`).
- Every source file ≤500 lines.

## Contract

### `extract(text) -> Vec<LinkRef>`

`LinkRef { destination: String, line: usize, span: Option<Range<usize>> }`, in source order.

- Includes inline, reference (full/collapsed/shortcut), autolink and image links.
- Excludes anything inside fenced/indented code blocks and code spans.
- `destination` is pulldown's parsed `dest_url` (escapes/entities processed, no percent-decoding).
- `line` is the 1-based line of the link start.
- `span` is the byte range of `destination` verbatim in the source: within the link's own source range for inline links; within the matching reference definition's span for reference-style links; `None` when not found verbatim (angle-bracket form, escapes, entities).
- Links sharing one definition return the same span.

### `anchors(text) -> Anchors`

GitHub-style heading slugs with duplicate suffixes and HTML `id`/`name`
values; `Anchors::matches(fragment)` is case-tolerant as in audit today.
Fragment helpers: `checkable_fragment(destination)` (percent-decoded, `None`
for empty or GitHub line anchors such as `L10`, `L10-L20`, `L10C2-L12C4`).

### `classify_destination(destination) -> DestinationShape`

| Shape | Rule (path part before `#`) |
|---|---|
| `FragmentOnly` | empty path |
| `OtherExternal` | `mailto:` or `data:` prefix |
| `Web` | `http://` or `https://` prefix |
| `Unsupported` | `//…`, other `scheme://`, or contains `<`/`>` |
| `Absolute(path)` | leading `/`, leading `\`, or drive letter (`X:`) |
| `Local(path)` | everything else |

### `resolve_local(base_dir, path) -> Option<String>`

Percent-decodes `path`, joins it with `/`-separated `base_dir`, resolves `.`
and `..` lexically, returns the `/`-joined repository-relative result, or
`None` when it escapes above the root.

### `encode_destination(path) -> String`

Percent-encodes space, `%`, `(`, `)`, `<`, `>`; all other characters unchanged.
`resolve_local(base, &encode_destination(p))` round-trips for relative `p`.
Percent-decoding operates on bytes and builds the result as UTF-8 (`from_utf8_lossy` for invalid sequences), so non-ASCII paths round-trip.

## Consumer obligations

- Audit maps `Absolute` to its unsupported-dependency finding. Only behavior delta: `\`-leading and drive-letter destinations are classified unsupported instead of missing-target (blocking either way). Every other finding, metric and test result is unchanged.
- install-ctl maps `Local` by extension on the decoded path, `Absolute` to `AbsoluteSourcePath`, `Web`/`OtherExternal`/`FragmentOnly`/`Unsupported` to skipped references.
- Rewrites edit only located spans. install: an unlocatable rewrite is a non-blocking plan warning and the link stays unchanged. autofix: an unlocatable operation is refused.

## Implementation tickets

- W1 `audit/.workflow-tools/ticket/tickets/777f9cc6-4a28-4d9e-8620-d94009d3c0b5/` — create the crate
- W2 `audit/.workflow-tools/ticket/tickets/bb847e22-f703-4c21-89b4-3a2b21f46073/` — migrate audit and split the module
- W3 `.workflow-tools/ticket/tickets/800237c4-dbe5-4d48-94ea-e8a1f906e582/` — install-ctl closure and plan
- W4 `.workflow-tools/ticket/tickets/4dcaee03-4ed0-42ec-a01a-c090e7d11da2/` — span-based rewrites

Paths are relative to the workflow-tools repository root. Roadmap: Meta-workspace `transcripts/31-10-2026_shared-markdown-links/ROADMAP.md`.
