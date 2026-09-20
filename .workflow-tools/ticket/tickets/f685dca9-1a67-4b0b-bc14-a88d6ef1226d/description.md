## Goal
Prevent reactive re-renders (e.g. SSE `ticket.upsert`, hover changes) from resetting an in-progress drag or camera interaction.

## Scope
File: `viewer-api/viewer-api/frontend/dioxus/src/graph3d/mod.rs` (`sync_render_state`, mod.rs:604)

Problem: node drag mutates `state.base_layout` to the dragged coordinates (`viewer-api/viewer-api/frontend/dioxus/src/graph3d/interaction/handlers.rs:363`) while `props.layout` still holds the original cache coordinates. On any re-render mid-interaction, the `state.base_layout != *layout` branch resets `state.layout`/`target_layout` from props and sets `dirty_layout`, snapping back the drag.

Change:
- Track an "interaction active" flag (drag/orbit/pan in progress) in `RenderState`.
- In `sync_render_state`, skip the layout-reset branch (or defer it) while an interaction is active, applying it on interaction end instead.
- Ensure visual-only prop changes (state color/title via node_view_transform / dirty_edges) still apply without resetting positions.

## Acceptance Criteria
- [ ] A `ticket.upsert` arriving mid-drag does not snap the dragged node back
- [ ] Camera orbit/pan is not interrupted by reactive re-renders
- [ ] Visual node-state updates still apply during interaction without repositioning
- [ ] Existing same-topology preservation behavior unchanged when idle

## Validation
- wasm-pack graph3d tests
- Playwright: drag a node while emitting a ticket update; assert no snap-back