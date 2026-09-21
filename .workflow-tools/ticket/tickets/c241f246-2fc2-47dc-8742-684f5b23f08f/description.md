# VoxelWorld API: Octree Data Structure, Manipulation, and Dirty-Range Tracking

## Problem

All rendering, physics, and editing flows through the VoxelWorld resource — the Sparse Voxel Octree that stores world structure. This ticket implements the core data structure, manipulation API, and dirty-region tracking. It does NOT handle GPU upload (T7b) or Rapier collision (T7c).

## Scope

### VoxelWorld Resource

```rust
#[derive(Resource)]
pub struct VoxelWorld {
    pub nodes: Vec<OctreeNode>,
    pub root_index: u32,
    pub max_depth: u32,
    pub dirty_ranges: Vec<(usize, usize)>,
    pub material_refs: Vec<(usize, MaterialRef)>,
}
```

### Manipulation API

```rust
impl VoxelWorld {
    pub fn set_voxel(&mut self, pos: IVec3, material: VoxelMaterial) {
        let (node_idx, depth) = self.descend_to(pos);
        self.nodes[node_idx].color_data = material.pack();
        self.mark_dirty(node_idx, depth);
    }

    pub fn remove_voxel(&mut self, pos: IVec3) { /* ... */ }

    pub fn apply_sdf_brush(&mut self, center: Vec3, radius: f32, material: VoxelMaterial) -> u32 {
        /* Iterate sphere region, set voxels, return count */
    }

    pub fn carve_sdf_brush(&mut self, center: Vec3, radius: f32) -> u32 {
        /* Iterate sphere region, remove voxels, return count */
    }
}
```

### Dirty-Range Tracking

```rust
impl VoxelWorld {
    fn mark_dirty(&mut self, node_idx: usize, _depth: u32) {
        let byte_start = node_idx * std::mem::size_of::<OctreeNode>();
        let byte_end = byte_start + std::mem::size_of::<OctreeNode>();
        self.dirty_ranges.push((byte_start, byte_end));
    }

    pub fn take_dirty_ranges(&mut self) -> Vec<(usize, usize)> {
        self.dirty_ranges.sort_by_key(|r| r.0);
        let merged = merge_ranges(&self.dirty_ranges);
        self.dirty_ranges.clear();
        merged
    }
}
```

### Octree Queries

- `descend_to(pos)`: traverse from root to target voxel coordinate
- `raycast(origin, dir, max_dist)`: ray-octree intersection (CPU-side)
- Position iteration: sphere, chunk, AABB helpers

## Dependencies
- T1 (scaffold): OctreeNode struct, VoxelWorld resource registration

## Acceptance Criteria
1. `set_voxel()` modifies the octree and marks the region dirty
2. `carve_sdf_brush()` removes voxels in a sphere
3. `apply_sdf_brush()` paints voxels in a sphere
4. `take_dirty_ranges()` returns merged, sorted ranges and clears internal state
5. `raycast()` finds correct voxel surface (position + normal)
6. Octree traversal handles max_depth correctly
