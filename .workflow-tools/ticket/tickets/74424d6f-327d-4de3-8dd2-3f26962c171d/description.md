Move the floating theme settings panel (`theme-settings-floating`) into the viewer-api `Modal` overlay (P1).

## Scope

- Replace the floating-panel container with `Modal` props: `open`, `on_close`, backdrop dismiss, ESC handling.
- Anchor open/close state in a `Signal<bool>` exposed on the page level (or via existing theme store flag).
- Delete the now-unused `theme-settings-floating` CSS.

## Acceptance

- `cargo check -p spec-viewer-dioxus --target wasm32-unknown-unknown` passes.
- Browser smoke: open theme settings, click backdrop or press ESC — closes; `Tab` focus stays trapped while open.
- No leaked event listeners (verify via the `Signal<Option<EventListener>>` slot pattern already used in viewer-api `Modal`).
