# [spec-editor] Interactive spec authoring — Dioxus SPA with body/section/coderef editing

A single-process, GPU-accelerated web application for **authoring and editing** specs.
Same architecture as `spec-viewer` (Axum + Dioxus/WASM) on port **4003**, with
additional backend routes for the workspace file/symbol picker and state-advance
endpoint.

**Spec:** `spec-editor` (slug: `spec-editor`)
**Sub-specs:** `spec-editor/body-editor`, `spec-editor/coderef-editor`,
`spec-editor/state-machine`, `spec-editor/section-editor`

**Depends on:** spec-viewer ticket (06399bb2) — shares component patterns and viewer
infrastructure.

---

## Acceptance Criteria

- [ ] `cargo run -p spec-editor` starts on port 4003 without error.
- [ ] `SpecListPage` has a `[+ New Spec]` button that opens `NewSpecPage`.
- [ ] `NewSpecPage` form creates a spec via `POST /api/specs` and navigates to its
      edit page on success; slug is auto-generated from title (with `_` → `-`).
- [ ] `SpecEditPage` loads the spec manifest and body; all manifest fields are
      displayed in editable form inputs (`title`, `slug`, `component`, `scope`,
      `parent`).
- [ ] **Body editor** — split-pane Markdown editor; autosaves 2 s after last keystroke
      via `PATCH /api/specs/:id/body`; `Ctrl+S`/`⌘S` saves immediately.
- [ ] **Preview pane** renders body Markdown via `pulldown-cmark` (WASM); updates
      within 150 ms of keystroke.
- [ ] `DirtyBanner` shows unsaved-changes indicator; "Discard" reloads from server.
- [ ] Browser `beforeunload` warning fires when `dirty == true`.
- [ ] Undo ring buffer: `Ctrl+Z`/`⌘Z` restores previous body state (≥10 levels).
- [ ] **Section panel** lists sections; "Add" creates a new section (name must match
      `[a-z0-9-]+`); "Delete" removes with confirmation.
- [ ] `SectionEditor` autosaves via `PUT /api/specs/:id/sections/:name`.
- [ ] **CodeRef editor** picker: Step 1 file browser, Step 2 symbol list, Step 3
      confirm and append to manifest via `PATCH /api/specs/:id`.
- [ ] CodeRef validation runs after add/delete; broken refs shown inline.
- [ ] **State machine stepper** shows current state; "Advance" button triggers
      pre-flight health check; confirmation modal before transition fires
      `POST /api/specs/:id/advance`.
- [ ] Advance blocked (advisory warning) when health issues exist per health-gate
      table in `spec-editor/state-machine` spec.
- [ ] `cargo check --target wasm32-unknown-unknown -p spec-editor-dioxus` passes.
- [ ] `trunk build --release` in `tools/viewer/spec-editor/frontend/dioxus` produces `dist/` output.

---

## New Backend Routes (beyond spec-http)

These are added by `spec-editor/src/main.rs` as axum routes before mounting
`spec-http`:

| Method | Path | Purpose |
|---|---|---|
| `GET` | `/api/workspace/files` | List `.rs` files in workspace (for CodeRef picker) |
| `GET` | `/api/workspace/files/:path/symbols` | `syn`-extract public symbols from file |
| `POST` | `/api/specs/:id/advance` | Advance state machine by one step |

---

## Architecture

### Backend (`tools/viewer/spec-editor/`)

```toml
[dependencies]
spec-api    = { path = "../../../crates/spec-api" }
spec-http   = { path = "../../http/spec-http" }
viewer-api  = { path = "../viewer-api" }
syn         = { version = "2", features = ["full", "extra-traits"] }
walkdir     = "2"
tokio       = { version = "1", features = ["full"] }
axum        = "0.8"
```

### Frontend (`tools/viewer/spec-editor/frontend/dioxus/`)

Key source files (in addition to those shared with spec-viewer):
- `src/components/spec_form.rs` — manifest field editor
- `src/components/body_editor.rs` — split-pane Markdown editor
- `src/components/section_panel.rs` — section list with CRUD
- `src/components/section_editor.rs` — per-section Markdown editor
- `src/components/coderef_editor.rs` — three-step picker
- `src/components/state_transition.rs` — state machine stepper
- `src/components/dirty_banner.rs` — unsaved-changes indicator
- `src/components/health_inline.rs` — inline health issue display

Additional WASM dependency:
```toml
pulldown-cmark = { version = "0.12", default-features = false }
```

---

## Phased delivery

**Phase 1 (v1):** Backend server scaffold + manifest form + body editor + autosave +
section panel. State stepper display only (no advance button).

**Phase 2:** CodeRef picker (requires `/api/workspace/files` and `.../symbols`
endpoints). State advance button with pre-flight.

**Phase 3:** GPU ambient effects tuned for editor context; 3-D spec graph from
spec-viewer embedded as read-only sidebar.