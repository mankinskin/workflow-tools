# [spec-viewer] GPU-accelerated spec browser — Dioxus SPA + spec-http backend

A single-process, GPU-accelerated web application for **reading and navigating** the
specification database.  Follows the exact same architecture as `ticket-viewer`:
Axum backend embedding `spec-http` + Dioxus/WASM SPA on port **4002**.

**Spec:** `spec-viewer` (slug: `spec-viewer`)

---

## Acceptance Criteria

- [ ] `cargo run -p spec-viewer` starts the server on port 4002 without error.
- [ ] `GET /` serves the Dioxus SPA (or static fallback).
- [ ] `GET /api/specs` proxied to spec-http responds with the spec list.
- [ ] `SpecListPage` renders all specs with correct title/state/slug.
- [ ] `SpecTreePage` renders hierarchical parent-child tree; collapse/expand works.
- [ ] `SpecDetailPage` renders body.md as Markdown with correct heading hierarchy.
- [ ] **Sections tab** lists all sections; clicking loads section body.
- [ ] **CodeRefs tab** shows file, symbol, kind, start/end line for each ref.
- [ ] **Health tab** calls `/api/specs/:id/health` and displays issues.
- [ ] `SearchBar` calls `/api/specs/search?q=<term>` and updates the list.
- [ ] `StateBadge` displays the correct colour for each state.
- [ ] SSE subscription on `SpecListPage` refreshes the list on `spec.updated` events.
- [ ] Keyboard navigation: `Tab` through tree nodes, `Enter` to open detail.
- [ ] `cargo check --target wasm32-unknown-unknown -p spec-viewer-dioxus` passes.
- [ ] `trunk build --release` in `tools/viewer/spec-viewer/frontend/dioxus` produces `dist/` output.

---

## Architecture

### Backend (`tools/viewer/spec-viewer/`)

```toml
[dependencies]
spec-api    = { path = "../../../crates/spec-api" }
spec-http   = { path = "../../http/spec-http" }
viewer-api  = { path = "../viewer-api" }
tokio       = { version = "1", features = ["full"] }
axum        = "0.8"
```

`src/main.rs`: parse `--port` (4002), `--workspace`, `--index-root`, `--static-dir`;
build `SpecStore`; mount `spec-http` router at `/api/`; serve SPA static files via
`viewer-api::with_static_files`.

### Frontend (`tools/viewer/spec-viewer/frontend/dioxus/`)

Key source files:
- `src/main.rs` — `dioxus::launch(App)` wrapping `ViewerShell { Router::<Route>{} }`
- `src/routes.rs` — `Route` enum (SpecListPage, SpecTreePage, SpecDetailPage)
- `src/api.rs` — `gloo-net` HTTP client
- `src/store.rs` — `SpecListStore` reactive state
- `src/sse.rs` — `use_sse` hook
- `src/types.rs` — `SpecSummary`, `SpecDetail`, `SectionMeta`, `CodeRefEntry`
- `src/components/spec_tree.rs` — collapsible hierarchy tree
- `src/components/spec_detail.rs` — body + sections + CodeRefs + health tabs
- `src/components/spec_card.rs` — compact row in list/tree
- `src/components/code_ref_list.rs` — CodeRef table
- `src/components/state_badge.rs` — coloured state pill
- `src/components/search_bar.rs` — search input
- `src/components/health_panel.rs` — health issues display

---

## Out of scope (defer to spec-editor ticket)

- Any write operations (editing body, sections, CodeRefs, state transitions).
- 3-D WebGPU tree visualisation (`SpecGraph3D`) — deferred to a follow-up ticket.