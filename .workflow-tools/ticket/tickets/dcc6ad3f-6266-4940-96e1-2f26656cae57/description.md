# Advanced Voxel Tools: Fill, Smooth, Extrude, Clone

## Problem

Beyond basic paint/carve, the world editor needs advanced manipulation tools: flood-fill enclosed regions, smooth surfaces (averaging neighbors), extrude faces outward, and clone/stamp regions.

## Scope

### Additional Tools

```rust
pub enum EditorTool {
    Paint,    // (T16a)
    Carve,    // (T16a)
    Fill,     // flood-fill enclosed region
    Smooth,   // average neighbor voxel properties
    Extrude,  // push face outward by N voxels
    Clone,    // copy region, place at new location
}
```

### Fill Tool

Flood-fill from a hit point, filling all connected empty voxels bounded by existing voxels. Stops at max region size to prevent runaway fills.

### Smooth Tool

Averages neighboring voxel colors/materials. Effect on splats:
- Averaged colors → smoother PBR material parameters
- Can increase half-extent for a soft/blurred look at surfaces

### Extrude Tool

Select a voxel face → drag to push it outward, duplicating voxels along the normal direction.

### Clone Tool

Select a region (AABB) → copy voxel data → place at new position. Uses a ghost preview (semi-transparent splats) before confirming placement.

## Dependencies
- T16a (core editor): EditorState, VoxelHit, basic tool dispatch
- T7a (VoxelWorld API): set_voxel, remove_voxel, region iteration

## Acceptance Criteria
1. Fill flood-fills enclosed empty regions correctly
2. Fill has max region cap to prevent runaway
3. Smooth visibly softens hard voxel edges (smoother splats)
4. Extrude pushes faces outward by drag distance
5. Clone copies and places voxel regions at new positions
6. All tools respect brush_size where applicable
