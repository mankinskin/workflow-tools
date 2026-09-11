---
name: dioxus
description: >-
  Use when writing, debugging, or reviewing Dioxus (Rust WASM) frontend code — covers signals/state
  and component patterns, server functions and fullstack data flow, the WASM build/bundle toolchain
  (trunk / dx), integration with this repo's viewer-api managed viewers, and styling/asset handling.
  Trigger on any mention of dioxus, rsx!, use_signal, use_resource, #[component], #[server], trunk
  serve, dx serve, Dioxus.toml/Trunk.toml, or a `frontend/dioxus` viewer crate.
applyTo: "**/frontend/dioxus/**"
---

# Dioxus (Rust WASM Frontend)

Dioxus is a Rust UI framework that renders to the web via WebAssembly. This repo
pins **Dioxus 0.7** and builds the managed viewers (`doc-viewer`, `log-viewer`,
`spec-viewer`, `ticket-viewer`, and the shared `viewer-api` frontend) as
client-side WASM apps served by Trunk, talking to a separate Rust HTTP backend.

Use this skill for the five areas below. When in doubt, mirror the existing
`*/frontend/dioxus` crates rather than inventing new structure.

## 1. Signals / state + component patterns

- Components are `fn` returning `Element`, annotated `#[component]`, body is an `rsx! { ... }` macro.
- Reactive state uses **signals**. Read with `sig()`, write with `sig.set(..)` / `sig.write()`.
  - `use_signal(|| initial)` — local reactive state.
  - `use_memo(move || ...)` — derived value that recomputes when its read signals change.
  - `use_effect(move || ...)` — run side effects when tracked signals change.
  - `use_context::<T>()` / `use_context_provider(|| T)` — dependency-inject shared state
    (this repo uses a `ThemeProvider` context wrapping the app root).
- Props are the component function's typed parameters; pass them as attributes in `rsx!`.

```rust
use dioxus::prelude::*;

#[component]
fn Counter(start: i32) -> Element {
    let mut count = use_signal(|| start);
    let doubled = use_memo(move || count() * 2);

    rsx! {
        button { onclick: move |_| count += 1, "count: {count}" }
        p { "doubled: {doubled}" }
    }
}
```

## 2. Server functions + fullstack data flow

- Dioxus fullstack exposes `#[server]` functions: an `async fn` callable from the client that
  runs on the server; the macro generates the client stub + HTTP endpoint.

  ```rust
  #[server]
  async fn get_count(id: String) -> Result<i32, ServerFnError> {
      // server-only code (DB, filesystem) runs here
      Ok(load_count(&id).await?)
  }
  ```

- **This repo does not use `#[server]`.** The managed viewers are thin client-side WASM apps
  that fetch from a standalone Rust HTTP backend (`viewer-api` / each viewer's server crate)
  using `gloo_net::http::Request`. Follow that pattern for viewer work:

  ```rust
  use gloo_net::http::Request;

  async fn list_logs() -> Result<Vec<LogFileInfo>, ApiError> {
      let resp = Request::get("/api/logs").send().await?;
      if !resp.ok() { return Err(ApiError::HttpStatus(resp.status())); }
      Ok(resp.json().await?)
  }
  ```

- Drive async fetches from components with `use_resource(move || async move { ... })`, which
  returns a signal-like handle you match on (`None` = loading, `Some(Ok)` / `Some(Err)`).

## 3. WASM build/bundle + trunk / dx toolchain

- Toolchain: `wasm32-unknown-unknown` target + `wasm-bindgen`. Entry point calls
  `dioxus_web::launch::launch_cfg(App, dioxus_web::Config::default())` under `#[cfg(target_arch = "wasm32")]`.
- **Trunk** builds and serves the client bundle (this repo's convention):
  - `trunk serve` — dev server with hot reload (each viewer pins a port in `Trunk.toml`, e.g. `8092`).
  - `trunk build --release` — optimized bundle for distribution.
  - `Trunk.toml` holds `[build]` and `[serve]` (port, open) config; `index.html` is the Trunk entry.
- The Dioxus CLI `dx serve` / `dx build` is the framework-native alternative; prefer the repo's
  Trunk flow for the managed viewers so `viewer-ctl prepare <viewer>` stays authoritative.
- `Cargo.toml` gates browser-only deps under `[target.'cfg(target_arch = "wasm32")'.dependencies]`
  (`wasm-bindgen`, `wasm-bindgen-futures`, `js-sys`, `web-sys`, `gloo-*`).

## 4. Integration with this repo's viewer-api managed viewers

- Managed viewers live at `<viewer>/frontend/dioxus/` and share the `viewer-api` frontend crate
  for common components, theming, and the backend contract trait.
- Backend endpoints are defined as an `async` trait (e.g. `LogViewerBackend`) with a `gloo_net`
  client impl on the WASM side; keep new endpoints in that shared contract.
- Lifecycle is orchestrated by `viewer-ctl`: `viewer-ctl prepare <viewer>` builds the frontend,
  `viewer-ctl start <viewer>` runs it. Do not hand-roll bespoke build scripts — extend the
  managed flow.
- Browser verification follows [AGENTS.md](../../../AGENTS.md#quality-gates)'s mandatory external fullscreen Chromium-family browser check; add/extend the shared
  Playwright E2E suites under `viewer-api/viewer-api/frontend/dioxus/e2e/shared/`.

## 5. Styling / asset handling

- Static assets and stylesheets are wired through `index.html` / Trunk asset directives
  (`<link data-trunk rel="css" href="...">`, `rel="copy-dir"`, etc.); Trunk fingerprints and
  copies them into `dist/`.
- Prefer scoped, theme-aware styling: this repo injects a `ThemeProvider` context and uses
  CSS classes toggled from signal state rather than inline style soup.
- Reference assets by their Trunk-managed public path; never hardcode `dist/` hashes.

## Worked example — a viewer panel that fetches and renders backend data

```rust
use dioxus::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;

#[derive(Deserialize, Clone, PartialEq)]
struct LogFileInfo { name: String, size_bytes: u64 }

async fn fetch_logs() -> Result<Vec<LogFileInfo>, String> {
    let resp = Request::get("/api/logs").send().await.map_err(|e| e.to_string())?;
    if !resp.ok() { return Err(format!("status {}", resp.status())); }
    resp.json::<Vec<LogFileInfo>>().await.map_err(|e| e.to_string())
}

#[component]
fn LogList() -> Element {
    // use_resource drives the async fetch and re-renders on completion.
    let logs = use_resource(fetch_logs);
    // Local UI state via a signal.
    let mut filter = use_signal(String::new);

    rsx! {
        input {
            class: "log-filter",
            placeholder: "filter…",
            oninput: move |e| filter.set(e.value()),
        }
        match &*logs.read() {
            None => rsx! { p { "Loading…" } },
            Some(Err(err)) => rsx! { p { class: "error", "Failed: {err}" } },
            Some(Ok(files)) => rsx! {
                ul {
                    for f in files.iter().filter(|f| f.name.contains(&filter())) {
                        li { key: "{f.name}", "{f.name} ({f.size_bytes} bytes)" }
                    }
                }
            }
        }
    }
}
```

This exercises all five areas: signal + resource state (1), the repo's `gloo_net` fetch data
flow (2), a component built for the Trunk/WASM target (3), the viewer-api backend contract
shape (4), and class-based styling hooks (5).

## Pitfalls

- Do not read a signal outside a reactive scope (component body, `use_memo`, `use_effect`);
  reads elsewhere won't subscribe and won't re-render.
- Keep server-only deps out of the default dependency table — gate them under the
  `cfg(target_arch = "wasm32")` (client) or server target so WASM builds stay small.
- Always give list items a stable `key`.
- After any viewer change, run the Trunk build (`viewer-ctl prepare <viewer>`) and the Playwright
  E2E suite; verify visually in an external browser before marking work done.
