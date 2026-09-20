# Interview: Build Tooling

**Date:** 2026-04-08
**Applies to:** `7346feae` (viewer-api-dioxus scaffold), `44d22e8f` (ticket-viewer scaffold)

## Question

context-editor uses `trunk serve` for dev. Options: Trunk (proven) or `dx serve` (Dioxus CLI, hot-reload)?

## Answer

**Use `trunk serve`** — `dx` (Dioxus CLI) was rejected due to wasm-opt crashes on Windows (`0xc0000409`) that
prevented reliable builds. Trunk is already proven in this repo (`log-viewer-leptos`, `context-editor`).

## Implications

- Use `trunk serve` for development
- Requires `trunk` installed (`cargo install trunk`)
- Configuration via `Trunk.toml` + `index.html` — consistent with `log-viewer-leptos`
- `trunk build --release` for production WASM builds
- CSS assets handled by `<link data-trunk rel="css" .../>` in `index.html`
