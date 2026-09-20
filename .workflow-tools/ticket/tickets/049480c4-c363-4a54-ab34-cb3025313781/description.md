Composite ticket tracking three feature tracks for ticket-viewer:

1. **Sortable FileTree** — Add generic sorting to the shared FileTree component in viewer-api, then integrate it in ticket-viewer with last-modified, created, and title sort keys.
2. **GPU Pipeline Extraction** — Extract the WgpuOverlay rendering pipeline, shaders, effects, and 3D math from log-viewer into viewer-api as shared infrastructure. Build a generic Graph3DView component, then use it in ticket-viewer to render the dependency graph with GPU-accelerated 3D rendering (replacing the current SVG GraphView).
3. **Themes & Effects Sharing** — Make the theme/effects system (presets, particle effects, CRT overlays, smoke) available in ticket-viewer with its own palette and effects off by default.

## Dependency Graph

```
Composite (049480c4)
├─ Sorting (d7971816) ─── depends on ─── API: Add created_at (d3a8b66a)
├─ GPU Extraction (1789cdfa)
│  ├─ B1: Move WgpuOverlay (b3d250d5)
│  ├─ B2: Extract Graph3DView (c826869a) ─── depends on B1
│  └─ B3: GPU dep graph (6f71ca0b) ─── depends on B2
└─ Themes/Effects (4f399974) ─── depends on GPU Extraction
```

## Execution Order

- **Track A** (parallel, quick): d3a8b66a → d7971816
- **Track B** (critical path): b3d250d5 → c826869a → 6f71ca0b
- **Track C** (blocked on B): 4f399974

## Resolved Design Decisions

1. `created_at` added to TicketSummary HTTP response (not derived from UUID).
2. `ColorTheme` / `EffectTheme` as separate types; `ThemePreset` composes both.
3. `ThemeSettings` is a shared component in viewer-api; each viewer passes its own preset lists.
4. Ticket-viewer palette: Default, Paper, Scratchboard, Office, Calendar, Notepad, Signal.
5. Effects OFF by default; opt-in via effect theme, apply globally across the entire UI.
6. Sorting within state groups; fixed group order.
7. App schema system for element selectors (`AppSchema`, `MINIMAL_SCHEMA`, app-derived schemas).
8. Isolated rendering contexts — `createOverlayContext()` factory + `<OverlayProvider>` replaces overlay-api globals.
9. Both force-directed and hierarchical layout algorithms in Graph3DView (new generic implementations).
10. All HypergraphView hooks move to viewer-api; log-viewer only keeps the hypergraph model layer.
11. 6f71ca0b is hard-blocked on c826869a (no stubs).
12. SVG GraphView generalized as `Graph2DView` in viewer-api.
