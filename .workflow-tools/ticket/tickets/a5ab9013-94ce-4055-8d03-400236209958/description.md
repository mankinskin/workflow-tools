# Rapier Collision Bridge: SVO → Chunk Colliders for Physics

## Problem

bevy_rapier3d needs collision shapes derived from the SVO for character physics and rigid body dynamics. This ticket implements the bridge that converts dirty SVO chunks into Rapier colliders using greedy box merging.

## Scope

### VoxelChunkCollider Component

```rust
#[derive(Component)]
pub struct VoxelChunkCollider {
    pub chunk_pos: IVec3,
    pub dirty: bool,
}
```

### Chunk Collider Rebuild

```rust
fn rapier_rebuild_system(
    voxel_world: Res<VoxelWorld>,
    mut chunks: Query<(&mut VoxelChunkCollider, &mut Collider)>,
) {
    for (mut chunk, mut collider) in chunks.iter_mut() {
        if chunk.dirty {
            *collider = rebuild_chunk_collider(&voxel_world, chunk.chunk_pos, CHUNK_SIZE);
            chunk.dirty = false;
        }
    }
}
```

### Greedy Box Merging

`rebuild_chunk_collider()` scans occupied voxels in a chunk and merges adjacent voxels into larger box colliders to reduce the collision shape count. A 16³ chunk of solid voxels becomes a single box collider instead of 4096.

### Rapier Plugin Configuration

```rust
app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
   .add_systems(PostUpdate, rapier_rebuild_system);
```

### Dirty Chunk Detection

When `VoxelWorld::set_voxel()` is called, the system determines which chunk was affected and marks its `VoxelChunkCollider` dirty.

## Dependencies
- T7a (VoxelWorld API): octree data + voxel query for chunk scanning
- T1 (scaffold): bevy_rapier3d dependency

## Acceptance Criteria
1. Dirty chunks rebuild Rapier colliders automatically
2. Greedy merging reduces collision shape count (compare merged vs naive)
3. Character (T8) stands on voxel surface via Rapier
4. Rigid body dropped into scene lands on voxels correctly
5. Chunk rebuild < 2ms for a 16³ chunk
6. Only dirty chunks are rebuilt (others untouched)
