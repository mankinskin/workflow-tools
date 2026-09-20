# Composite: Ticket-Viewer Feature Bundle

**Ticket:** 049480c4-c363-4a54-ab34-cb3025313781
**Status:** Plan phase — all sub-tickets created and wired

---

## Overview

Three independent feature tracks for ticket-viewer, with the GPU pipeline extraction being the largest and most complex.

## Dependency Graph

```
Composite (049480c4)
├── depends_on → Sorting (d7971816) ........................ [depends on API change]
│   └── depends_on → API: Add created_at (d3a8b66a) ....... [no deps, low risk]
├── depends_on → GPU Extraction (1789cdfa) ................. [sequential, high risk]
│   ├── depends_on → B1: Move WgpuOverlay (b3d250d5) ...... [no deps]
│   ├── depends_on → B2: Extract Graph3DView (c826869a) ... [depends on B1]
│   └── depends_on → B3: GPU dep graph (6f71ca0b) ......... [depends on B2]
└── depends_on → Themes/Effects (4f399974) ................. [depends on GPU Extraction]
    └── depends_on → GPU Extraction (1789cdfa)
```

## Execution Order

**Parallel Track A** (can start immediately):
1. `d3a8b66a` — Add created_at to TicketSummary HTTP response
2. `d7971816` — Sortable FileTree with generic sorting header (blocked on A.1)

**Sequential Track B** (critical path):
1. `b3d250d5` — Move WgpuOverlay + shaders + effects to viewer-api
2. `c826869a` — Extract generic Graph3DView to viewer-api
3. `6f71ca0b` — GPU dependency graph in ticket-viewer

**Sequential Track C** (blocked on B1 at minimum):
1. `4f399974` — Share themes and effects to ticket-viewer

## Ticket Summary

| ID | Title | Risk | Component | Deps |
|----|-------|------|-----------|------|
| `d3a8b66a` | Add created_at to TicketSummary API | Low | ticket-http | none |
| `d7971816` | Sortable FileTree | Low | viewer-api | d3a8b66a |
| `b3d250d5` | Move WgpuOverlay to viewer-api | High | viewer-api | none |
| `c826869a` | Extract Graph3DView | High | viewer-api | B1 |
| `6f71ca0b` | GPU dep graph in ticket-viewer | Medium | ticket-viewer | B2 |
| `4f399974` | Themes/effects in ticket-viewer | Medium | ticket-viewer | GPU Extraction |

## Resolved Design Decisions

1. **created_at** added to TicketSummary HTTP response (not derived from UUID).
2. **ColorTheme / EffectTheme** are separate types in viewer-api; `ThemePreset` composes both.
3. **ThemeSettings** is a shared component in viewer-api; each viewer passes its own preset lists. Full feature set: presets + color editor + effect sliders + save/export/import + thumbnail capture.
4. **Ticket-viewer gets its own color/effect palettes** (not sharing log-viewer's 9 presets). Themes: Default, Paper, Scratchboard, Office, Calendar, Notepad, Signal.
5. **Effects are OFF by default** in ticket-viewer. Users opt-in by selecting an effect theme from the full ThemeSettings panel.
6. **Sorting (A1):** Applied **within** each state group. State group order is fixed (open → in-progress → review → validating → validated → done → blocked → cancelled → unknown).
7. **App schema system (A2):** Element selectors are replaced with a typed `AppSchema` (selectors + kinds + optional particle ranges). viewer-api ships a `MINIMAL_SCHEMA`. Log-viewer and ticket-viewer each define their own derived schema. `WgpuOverlay` accepts a `schema` prop.
8. **Isolated rendering contexts (A3):** Replace module-level globals in `overlay-api.ts` with `createOverlayContext()` factory + `<OverlayProvider>` + `useOverlayContext()`. Each app instance gets its own GPU context, signals, and callback registry. No shared mutable state.
9. **Multiple layout algorithms (A4):** Graph3DView ships with `force-directed` and `hierarchical` layouts, switchable via `layout` prop. Both are new generic implementations (not ports of the hypergraph-coupled `layout.ts`).
10. **All hooks move to viewer-api (A5):** viewer-api Graph3DView is fully self-contained (camera, interaction, touch, overlay renderer, animation, node layer, layout). Log-viewer adds only the hypergraph-specific layer.
11. **6f71ca0b blocked on c826869a (A6):** GPU dep graph in ticket-viewer is hard-blocked until Graph3DView is implemented. No stubs.
12. **Graph2DView moved to viewer-api (A7):** The SVG GraphView becomes a generic `Graph2DView` component in viewer-api. Used in ticket-viewer as a non-GPU path and available for doc-viewer or any lightweight embedding context.

Each sub-ticket has a detailed plan attached as `plan.md` or in its acceptance criteria.
