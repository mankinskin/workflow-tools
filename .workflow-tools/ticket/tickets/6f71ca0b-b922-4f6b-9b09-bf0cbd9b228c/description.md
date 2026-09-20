Build a GPU-rendered dependency graph in ticket-viewer using the shared `Graph3DView` component from viewer-api, replacing the current SVG-based `GraphView.tsx`.

## Current State

`ticket-viewer/frontend/src/components/GraphView.tsx` renders the dependency graph as SVG:
- Calls `getSubgraph()` for ticket dependency data
- Groups nodes by depth, horizontal layout (190px spacing)
- SVG circles (11px radius) colored by ticket state
- Simple bezier-curve edges

## Target State

A new `DependencyGraph` component that:
1. Imports `Graph3DView` from `@context-engine/viewer-api-frontend`
2. Maps ticket subgraph data (nodes with state/title, edges with kind) to `Graph3DNode[]` / `Graph3DEdge[]`
3. Colors nodes by ticket state (open, in-progress, done, cancelled)
4. Visually distinguishes edge kinds (depends_on, blocks, linked)
5. Highlights the root ticket
6. Navigates to ticket on node click
7. Handles loading/error/empty states

## Fallback

`Graph2DView` (moved to viewer-api from the old SVG GraphView, as part of b3d250d5) is available as the non-WebGPU fallback. Both can coexist — detect WebGPU on mount and choose renderer.

## Depends On

c826869a — **hard block**. Graph3DView must be fully implemented before this ticket begins. No stubs.
