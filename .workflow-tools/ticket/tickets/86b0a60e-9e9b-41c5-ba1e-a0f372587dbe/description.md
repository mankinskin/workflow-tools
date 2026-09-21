# Core Voxel Editor: Paint, Carve, and Ray-Octree Intersection

## Problem

The minimum viable world editor: paint voxels onto surfaces, carve voxels away, and ray-cast to find which voxel the cursor is pointing at. Edits flow through VoxelWorld → double buffer → splat regeneration for instant visual feedback.

## Scope

### Editor State

```rust
#[derive(Resource)]
pub struct EditorState {
    pub active_tool: EditorTool,
    pub brush_size: u32,
    pub current_material: MaterialDef,
}

pub enum EditorTool {
    Paint,
    Carve,
}
```

### Ray-Octree Intersection

```rust
fn editor_raycast(
    mouse: Res<MousePosition>,
    camera: Query<(&Transform, &Projection), With<Camera3d>>,
    svo: Res<VoxelWorld>,
) -> Option<VoxelHit> {
    let ray = camera_ray(&mouse, &camera);
    svo.raycast(ray.origin, ray.direction, 200.0)
}

pub struct VoxelHit {
    pub position: IVec3,
    pub normal: IVec3,
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
        let center = hit.position + hit.normal;
        for offset in sphere_offsets(editor.brush_size) {
            svo.set_voxel(center + offset, editor.current_material);
        }
    }
}
```

### Carve Tool

Same as paint but calls `svo.remove_voxel()` for each position in the brush sphere.

### Edit Pipeline

```
Mouse click → ray-octree hit → apply paint/carve to SVO
  → dirty ranges → double-buffer upload (T7b) → splat regen (T6a)
  → visual update next frame
```

## Dependencies
- T7a (VoxelWorld API): set_voxel, remove_voxel, raycast
- T7b (double buffer upload): dirty upload for visual feedback
- T8 (character): camera for ray construction

## Acceptance Criteria
1. Paint tool adds voxels → splats appear next frame
2. Carve tool removes voxels → splats disappear next frame
3. Ray-octree intersection finds correct voxel surface
4. Brush size scales spherical paint/carve region
5. Continuous painting while mouse is held down
