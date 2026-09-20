Replace the manual nav buttons in `tools/viewer/spec-viewer/frontend/dioxus/src/routes.rs` with the viewer-api `HeaderActions` component (P3).

## Scope

- Wire `on_home`, `on_filter_toggle`, `on_theme_toggle`.
  - `on_home` → push `Route::SpecListPage`.
  - `on_filter_toggle` → toggle a `Signal<bool>` consumed by the existing filter UI (or stub to no-op until P5 filter ticket lands).
  - `on_theme_toggle` → open the new theme `Modal` (depends on P5.4).
- Remove now-dead inline button markup and CSS.

## Acceptance

- `cargo check -p spec-viewer-dioxus --target wasm32-unknown-unknown` passes.
- Browser smoke: header buttons render, all three handlers fire and update state.
