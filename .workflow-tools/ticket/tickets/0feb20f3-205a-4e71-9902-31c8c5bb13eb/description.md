Adopt the new viewer-api widgets and stores in spec-viewer, replacing ad-hoc inline implementations.

## Deliverables

1. **Multi-spec tabs** — replace the single-spec right panel with `TabsStore<SpecId>` (P2). Selecting a spec from list/tree/graph opens or focuses a tab. Closing the last tab leaves the empty state.

2. **Breadcrumbs** — render `Breadcrumbs` (P1) above the spec detail tabs reflecting `category › name › section`. Each segment is clickable except the leaf.

3. **CategoryPage** — replace the spec list page hand-rolled cards with `CardGrid` + `Card` (P1). The graph page's empty/loading uses `empty-state` (already done) but the populated landing uses `CardSection` per category.

4. **Modal** — move the floating theme-settings panel (currently `theme-settings-floating`) into `Overlay` (P1) with backdrop dismiss.

5. **HeaderActions** — replace the manual nav buttons in `routes.rs` with `HeaderActions` (P3). Wire `on_home`, `on_filter_toggle`, `on_theme_toggle`.

6. **URL routing** — adopt `PathCodec` (P2). Active spec id syncs with `#/spec/<category>/<name>`. Navigating via URL opens the corresponding tab and expands tree ancestors.

7. **Prefetch** — wrap the spec-fetch API in `Prefetcher` (P2) so siblings of the active spec are eagerly loaded.

## Out of scope

- Filter panel adoption — requires P4 + a jq backend; tracked separately.
- Doc-viewer changes — separate ticket.

## Acceptance criteria

- `cargo check -p spec-viewer-dioxus --target wasm32-unknown-unknown` passes
- Browser verification per AGENTS.md: open spec-viewer, confirm tabs, breadcrumbs, theme overlay, URL roundtrip, and category landing all work
- Playwright e2e covering: open two specs in tabs, close one, navigate via URL, theme overlay open/close
- No remaining inline `style:` attributes in spec-viewer except the documented dynamic-only allowlist (`spec_graph.rs` per-node colours)
