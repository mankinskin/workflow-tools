Wrap the spec-fetch API in the viewer-api `Prefetcher` (P2) so that siblings of the active spec are eagerly loaded.

## Scope

- Define a fetch task: given a `SpecId`, call the existing `api::get_spec` and store result in the in-memory cache.
- On tab focus, schedule prefetch for siblings (same category) and the next/previous spec in tree order.
- Bound concurrency per the `Prefetcher` defaults; cancel pending tasks when the user navigates away.

## Acceptance

- `cargo check -p spec-viewer-dioxus --target wasm32-unknown-unknown` passes.
- Browser smoke: open a spec, watch network panel — sibling specs are fetched in the background; switching to a sibling renders without a spinner.
- No duplicate concurrent fetches for the same id.

## Depends on

- P5.1 (TabsStore) — prefetch is keyed off active tab.
