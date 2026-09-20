Replace the single-spec right panel in spec-viewer with `TabsStore<SpecId>` (from viewer-api P2).

## Scope

- Wire `TabsStore<SpecId>` into `store.rs`; provide via context.
- Selecting a spec from list/tree/graph opens a new tab or focuses an existing one (use `open_or_focus`).
- Render a `TabBar` above the right panel that dispatches close events back into the store.
- Closing the last tab restores the existing empty-state placeholder.

## Acceptance

- `cargo check -p spec-viewer-dioxus --target wasm32-unknown-unknown` passes.
- Browser smoke (per AGENTS.md): open three specs from the list, close the middle tab, confirm focus jumps to a sibling.
- No regression of the existing single-spec URL route (`/specs/:id`) — opening a URL still focuses or creates the matching tab.

## Out of scope

- URL ↔ tabs roundtrip (covered by P5.6 routing ticket).
- Prefetching siblings (covered by P5.7).
