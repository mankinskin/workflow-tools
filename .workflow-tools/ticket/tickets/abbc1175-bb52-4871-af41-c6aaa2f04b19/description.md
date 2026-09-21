# Editor UX: Undo/Redo, Symmetry, Live Preview, Material Picker

## Problem

The voxel editor needs UX features for a productive editing workflow: undo/redo for all operations, symmetry modes for mirrored sculpting, live brush preview, and a material picker connected to the theme palette.

## Scope

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

Undo restores previous voxel state → marks chunks dirty → splats regenerated. Same 1-frame latency as any edit.

### Symmetry Modes

```rust
pub enum Symmetry {
    None,
    MirrorX,
    MirrorXZ,
    Radial(u32),
}
```

Each tool operation is duplicated across symmetry axes before being applied to the SVO.

### Live Brush Preview

While hovering (before clicking), show a preview of affected voxels as semi-transparent splats. Temporarily inject preview splats into the generation buffer with reduced opacity; remove on mouse move.

### Material Picker

```rust
fn material_picker_ui(editor: &mut EditorState, palette: &ThemePalette) {
    // Shows palette materials as selectable swatches
    // Selected material applied to painted voxels
}
```

Connected to ThemePalette (T5) — available materials come from the active theme.

### Upload Budget

For large edits (64³ region = 262K voxels), upload is throttled by `DoubleBufferParams.upload_budget_bytes`. Excess spreads across frames (progressive update).

## Dependencies
- T16a (core editor): EditorState, tool dispatch, VoxelHit
- T5 (theme): Material palette for picker
- T11 (params): DoubleBufferParams.upload_budget_bytes

## Acceptance Criteria
1. Undo reverses last edit; redo re-applies it
2. Symmetry modes duplicate edits across axes correctly
3. Live preview shows semi-transparent splats at brush position
4. Material picker displays theme palette materials
5. Large edits don't stall rendering (upload budget throttling)
6. Undo/redo stack has reasonable depth limit
