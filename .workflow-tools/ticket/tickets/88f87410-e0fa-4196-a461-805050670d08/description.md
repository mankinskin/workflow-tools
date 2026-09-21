Integrate the four graph improvements into spec-viewer:

1. **Selection focus/deselection**: Update `spec_graph/page.rs` to use Graph3D with `on_deselect` handler, implement outside-click clearing
2. **Property-based rendering tiers**: Ensure spec graph nodes use the 5-level LOD system (Point/Sphere → Icon → Label → Compact → Rich) with hover promotion
3. **Panel-aware framing**: Configure viewport insets to keep spec graph nodes behind UI panels in spec-viewer layout
4. **2D mode/keyframing**: Add Fixed2D camera mode support to spec-viewer graph, enable presentation keyframing

**Files to update:**
- `memory-viewers/spec-viewer/frontend/dioxus/src/components/spec_graph/page.rs`
- Any spec-viewer specific graph wrapper components

**Acceptance Criteria:**
1. spec-viewer graph page uses updated Graph3D component with all four improvements
2. Browser verification confirms:
   - Clicking outside spec graph nodes clears selection
   - Node rendering detail changes with distance and hover state
   - Graph nodes stay behind spec preview panels
   - 2D mode switching works in spec-viewer context
3. Playwright E2E coverage for spec-viewer graph interactions
4. No regression in existing spec-viewer graph functionality