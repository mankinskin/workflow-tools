# World Editor: Voxel Paint/Carve with Live Splat Regeneration

> **Coordinator ticket** — this ticket has been decomposed into focused sub-tickets.
> Implementation work happens in the children; this ticket tracks overall completion.
>
> **Sub-tickets:**
> - **T16a** Core Voxel Editor — Paint, Carve & Ray-Octree — `86b0a60e-9e9b-41c5-ba1e-a0f372587dbe`
> - **T16b** Advanced Voxel Tools — Fill, Smooth, Extrude & Clone — `dcc6ad3f-6266-4940-96e1-2f26656cae57`
> - **T16c** Editor UX — Undo/Redo, Symmetry, Preview & Material Picker — `abbc1175-bb52-4871-af41-c6aaa2f04b19`
>
> This ticket is done when all three sub-tickets are closed.

---

## Problem

The world editor lets users paint, carve, and sculpt voxels in the SVO. Each edit dirties the SVO, which is uploaded via the double buffer (T7), triggering splat regeneration on GPU (T6). The user sees visual feedback within one frame — new/removed voxels appear/disappear as splats.

## Architecture

### Edit Tools

```rust
#[derive(Resource)]
pub struct EditorState {
    pub active_tool: EditorTool,
    pub brush_size: u32,         // voxel radius
    pub current_material: MaterialDef,
    pub symmetry: Symmetry,
}

pub enum EditorTool {
    Paint,          // set voxel to current material
    Carve,          // remove voxels
    Fill,           // flood-fill enclosed region
    Smooth,         // average neighbors (adjusts splat extent)
    Extrude,        // push face outward
    Clone,          // copy region
}

pub enum Symmetry {
    None,
    MirrorX,
    MirrorXZ,
    Radial(u32),  // N-fold rotational
}
```

### Edit → SVO → Double Buffer → splats Pipeline

```
User action (mouse click/drag)
  → Ray cast through camera (screen → world)
  → Hit SVO voxel surface (ray-octree traversal on CPU)
  → Apply tool to SVO region
  → Mark dirty chunks in SVO
  → svo_upload_system writes dirty chunks to BACK buffer
  → double_buffer_swap_system swaps FRONT↔BACK
  → voxel_splat_kernel compute pass reads new FRONT buffer
  → New splats reflect edits
  → Tiled rasterizer renders updated scene
```

Total latency: 1 frame (edit on frame N, visible on frame N+1). With double buffering, the edit never stalls the current frame's rendering — the GPU continues reading the old FRONT buffer while WASM writes to BACK.

### Ray-Octree Intersection

```rust
fn editor_raycast(
    mouse: Res<MousePosition>,
    camera: Query<(&Transform, &Projection), With<Camera3d>>,
    svo: Res<VoxelWorld>,
) -> Option<VoxelHit> {
    let ray = camera_ray(mouse, camera);
    svo.raycast(ray.origin, ray.direction, 200.0)  // max distance
}

pub struct VoxelHit {
    pub position: IVec3,      // voxel coordinate
    pub normal: IVec3,        // face normal (for placement on surface)
    pub distance: f32,
}
```

### Paint Tool

```rust
fn paint_tool_system(
    input: Res<InputState>,
    editor: Res<EditorState>,
    mut svo: ResMut<VoxelWorld>,
    hit: Option<Res<VoxelHit>>,
) {
    if !input.mouse_left || editor.active_tool != EditorTool::Paint { return; }
    if let Some(hit) = hit {
        let center = hit.position + hit.normal; // place on surface
        for offset in sphere_offsets(editor.brush_size) {
            let pos = center + offset;
            svo.set_voxel(pos, editor.current_material);
            // This marks the chunk as dirty automatically
        }
    }
}
```

### Smooth Tool

Smoothing averages neighboring voxel properties, which affects the splat generation:
- Averaged colors → smoother PBR material parameters on generated splats
- Averaged positions → splats shift slightly, creating smoother surfaces
- Can increase half-extent (larger splats) for a soft/blurred look

### Material Picker

```rust
fn material_picker_ui(editor: &mut EditorState, palette: &ThemePalette) {
    // Shows palette materials (T5) as selectable swatches
    // Each swatch shows: base_color, roughness, metallic preview
    // Selected material applied to painted voxels
    // PBR material parameters computed by pack_material() in splat generation
}
```

### Undo/Redo

```rust
#[derive(Resource)]
pub struct EditHistory {
    pub undos: Vec<EditSnapshot>,
    pub redos: Vec<EditSnapshot>,
}

pub struct EditSnapshot {
    pub changed_voxels: Vec<(IVec3, Option<VoxelData>, Option<VoxelData>)>, // pos, old, new
}
```

Undo restores previous voxel state → marks chunks dirty → double buffer upload → splats regenerated. Same pipeline, same 1-frame latency.

### Live Preview

While dragging a brush, show a preview of affected voxels as semi-transparent splats. Implementation: temporarily add preview splats to the generation buffer with reduced opacity, remove them if the user cancels.

### Performance Budget

For large edits (e.g., filling a 64³ region = 262,144 voxels):
- SVO update: ~5ms (batch set_voxel)
- Upload to BACK buffer: limited by DoubleBufferParams.upload_budget_bytes (4MB default)
- If edit exceeds budget: upload spreads across multiple frames (progressive update)
- splat regeneration: automatic, 1 frame after upload completes

## Dependencies
- T7 (physics+world): VoxelWorld SVO, double-buffered upload, dirty chunk tracking
- T6 (3D scene): splat generation reads SVO FRONT buffer
- T5 (theme): Material palette for paint tool
- T8 (character): Camera ray for ray-octree intersection
- T11 (params): DoubleBufferParams.upload_budget_bytes for large edit throttling

## Acceptance Criteria
1. Paint tool adds voxels → splats appear next frame
2. Carve tool removes voxels → splats disappear next frame
3. Fill tool flood-fills enclosed regions
4. Smooth tool averages voxel properties → softer splats
5. Material picker uses theme palette (T5)
6. Undo/redo works with full voxel state restoration
7. Symmetry modes (mirror, radial) work for all tools
8. Large edits don't stall rendering (double buffer + upload budget)
9. Brush preview shows semi-transparent splats while dragging
10. Ray-octree intersection finds correct voxel surface
