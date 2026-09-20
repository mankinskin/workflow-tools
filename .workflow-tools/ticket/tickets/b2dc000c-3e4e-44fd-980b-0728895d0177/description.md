Replace the hand-rolled spec list cards with viewer-api `CardGrid`/`Card`/`CardSection` (P1).

## Scope

- `CategoryPage` (spec list landing): one `CardSection` per category, with a `CardGrid` of `Card`s for each spec.
- Card body: name, short summary, status chip if available.
- Card click → push `Route::SpecDetailPage { id }`.
- Remove now-dead inline `style:` attributes and bespoke CSS for the old card layout.

## Acceptance

- `cargo check -p spec-viewer-dioxus --target wasm32-unknown-unknown` passes.
- Browser smoke: list page renders one section per category, click navigates to detail.
- No new inline styles; only design tokens used.
