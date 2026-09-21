# [context-editor][SDF-DAG] Phase 2: DAG-Persistent Edit Operations with Hash Consing

## Problem

The current SVO uses a mutable flat `Vec<OctreeNode>` where every edit directly mutates nodes
in-place. This has two structural problems:

1. **Duplication**: brush operations that paint identical sub-scenes at multiple locations allocate
   redundant independent node trees. A forest of 100 identical trees stores 100× the node data.

2. **No structural consistency guarantee**: the duplication-resistance property must be ensured
   by the data structure itself, not by caller discipline. Currently there is no mechanism to
   detect and share identical subtrees — they are silently duplicated.

## Design

### Content-Addressed Node Pool

Replace `VoxelWorld.nodes: Vec<OctreeNode>` with a hash-consed pool. Two nodes with identical
content (same `child_pointer` bits and same `atom_ref`) occupy exactly one slot.

```rust
// kernel/src/dag/pool.rs
pub struct NodePool {
    /// Flat array of all allocated nodes. Slot index = stable identity.
    pub nodes: Vec<OctreeNode>,
    /// Content hash → slot index.
    /// Hash = FxHash64(child_pointer_bits XOR (atom_ref rotl 32))
    pub hash_to_slot: FxHashMap<u64, u32>,
    /// Reference count per slot. When ref_count[slot] drops to 0, slot is freed.
    pub ref_counts: Vec<u32>,
    /// Free slots available for reuse (reclaimed from ref_count == 0 slots).
    pub free_slots: Vec<u32>,
}

impl NodePool {
    /// Insert or find a node by content. Returns the canonical slot index.
    /// If an identical node already exists, its slot is returned unchanged.
    pub fn intern(&mut self, node: OctreeNode) -> u32;
    /// Increment ref count — called when a new parent references this slot.
    pub fn retain(&mut self, slot: u32);
    /// Decrement ref count; reclaim into free_slots if count reaches zero.
    pub fn release(&mut self, slot: u32);
}
```

**Key invariant**: `hash_to_slot.len() == nodes.iter().filter(|n| ref_counts[i] > 0).count()`.
No two distinct live slots carry identical content.

### VoxelDag: Root-Versioned DAG

```rust
// kernel/src/dag/mod.rs
#[derive(Resource)]
pub struct VoxelDag {
    pub pool: NodePool,
    pub atom_pool: AtomPool,        // from Phase 1
    pub root: u32,                  // current root slot
    pub max_depth: u32,
}

impl VoxelDag {
    /// Set atom at pos. Produces a new root via COW traversal.
    /// After return, the DAG is fully consistent regardless of prior state.
    pub fn set_atom(&mut self, pos: IVec3, atom: AtomDescriptor) -> u32;

    /// Remove atom at pos. Returns new root.
    pub fn remove_atom(&mut self, pos: IVec3) -> u32;

    /// Batch set. Sorts ops by path prefix and performs a single-pass COW,
    /// forking each internal node at most once per batch.
    pub fn set_atoms_batch(&mut self, ops: &[(IVec3, AtomDescriptor)]);

    /// CPU-side raycast (still works after migration to DAG).
    pub fn raycast(&self, origin: Vec3, dir: Vec3, max_dist: f32) -> Option<(Vec3, Vec3)>;
}
```

### Copy-on-Write Traversal

Every path from root to a modified leaf is cloned, re-interned, and the old root is released.
Siblings not on the modified path are referenced without copying — their ref_count increases.

```rust
fn cow_descend(
    pool: &mut NodePool,
    node_slot: u32,
    pos: UVec3,
    depth: u32,
    max_depth: u32,
    atom: Option<(u8, u32)>,  // (type_id, pool_idx) from AtomPool::insert
) -> u32 {
    if depth == max_depth {
        let leaf = OctreeNode::leaf(atom.map(|(t, i)| (t as u32) << 24 | i).unwrap_or(0));
        return pool.intern(leaf);
    }
    let old_node = pool.nodes[node_slot as usize];
    let child_bit = path_bit(pos, depth, max_depth);
    // Fork: clone old node, update child pointer to new child slot, re-intern.
    let new_child = cow_descend(pool, child_slot, pos, depth + 1, max_depth, atom);
    let mut new_node = old_node;
    new_node.set_child(child_bit, new_child);
    let new_slot = pool.intern(new_node);
    pool.release(node_slot);
    new_slot
}
```

### Batch COW for Brush Operations

Naïve sequential `set_atom` calls on N voxels create O(N × depth) COW copies of internal nodes.
Batched COW forks each internal node **at most once** per batch:

```rust
pub fn set_atoms_batch(&mut self, ops: &[(IVec3, AtomDescriptor)]) {
    // Sort ops by Z-order (Morton code) to maximise prefix sharing.
    // Single top-down recursive COW: at each depth, group ops by child quadrant,
    // recurse into each occupied quadrant exactly once, re-intern parent once.
    let atom_refs: Vec<_> = ops.iter().map(|(_, a)| self.atom_pool.insert(a.clone())).collect();
    self.root = batch_cow_descend(&mut self.pool, self.root, ops, &atom_refs, 0, self.max_depth);
}
```

A sphere brush with radius 5 (≈ 500 leaf edits) at depth 8 creates ≤ 8 × 8 = 64 new internal
nodes instead of 500 × 8 = 4000 with naïve sequential calls.

### GPU Upload: Diff-Based

Replace dirty-range tracking with a structural diff between old and new root subtrees:

```rust
/// Returns (slot, node) pairs that exist in `new_root` but not in `old_root`.
/// These are the only slots that need GPU upload — zero bytes for unchanged branches.
pub fn diff_roots(old_root: u32, new_root: u32, pool: &NodePool) -> Vec<(u32, OctreeNode)>;
```

GPU slot layout is unchanged — slot index is still the offset into the GPU storage buffer.
Freed slots are overwritten lazily when their free_slot is reused by a future `intern` call.

### Backward Compatibility

`VoxelWorld` becomes a thin wrapper over `VoxelDag`:

```rust
pub type VoxelWorld = VoxelDag;
// All existing call sites work unchanged:
// world.set_voxel(pos, mat)  → world.set_atom(pos, AtomDescriptor::Legacy(mat))
// world.apply_sdf_brush(..)  → world.set_atoms_batch(..)
// world.raycast(..)          → VoxelDag::raycast(..)
```

## Implementation Plan

1. **`kernel/src/dag/pool.rs`** (new): `NodePool` with `intern`, `retain`, `release`.
   Hash function: `FxHash64` of `(child_pointer as u64) ^ ((atom_ref as u64) << 32)`.
   Free-list reuse: slots with `ref_counts[i] == 0` are reclaimed via `free_slots` deque.
2. **`kernel/src/dag/mod.rs`** (new): `VoxelDag` resource. Implements `set_atom`, `remove_atom`,
   `set_atoms_batch`, `raycast`, `compute_node_positions` (for GPU node_positions buffer).
3. **`kernel/src/svo/mod.rs`**: Replace `VoxelWorld { nodes: Vec<OctreeNode>, .. }` with
   `pub type VoxelWorld = VoxelDag`. Migrate `descend_and_allocate` to `cow_descend`.
4. **`kernel/src/svo/upload.rs`**: Replace `take_dirty_ranges` + `write_buffer` loop with
   `diff_roots` → upload only changed slots. Add slot-to-byte-offset conversion.
5. **`kernel/src/svo_lod.rs`**: Migrate `propagate_lod_colors` to read from `NodePool`.
   LOD color accumulation is still bottom-up DFS, now on pool slot indices.
6. **`kernel/src/sdf_cutting.rs`**: Route `apply_sdf_brush` and CSG operations through
   `set_atoms_batch` for deduplication-resistant batch edits.
7. **Tests**:
   - Property test: after N random `set_atom`/`remove_atom` calls,
     `pool.hash_to_slot.len() == live_slot_count` (no duplicate content ever).
   - Structural sharing: painting 1000 identical atoms at different positions allocates
     exactly 1 atom pool entry.
   - Identical subtrees: 2 identical 4×4×4 voxel blocks placed at different world positions
     share all 64 leaf node slots.
   - Batch efficiency: 500-voxel sphere brush allocates ≤ max_depth × 8 new internal nodes.
   - `raycast` output is identical before and after migration.

## Acceptance Criteria

1. After any `set_atom`, `remove_atom`, or `set_atoms_batch` call the DAG is fully consistent:
   traversable from `root` to all expected leaves with no orphaned or duplicate nodes.
2. `pool.hash_to_slot.len() == live_slot_count` holds as an invariant at all times.
3. Painting 1000 identical `AtomDescriptor::Sphere` atoms allocates exactly 1 atom pool entry.
4. Two identical `8×8×8` voxel block copies placed at different world locations share all
   internal node slots — only their parent pointers differ.
5. `diff_roots(old, new)` emits zero upload bytes for unchanged spatial regions.
6. No GPU upload occurs in frames where `set_atom` was not called.
7. `raycast` and `apply_sdf_brush` produce pixel-identical results before and after migration.
8. Brush batch (500 voxel sphere) allocates ≤ `max_depth × 8` new internal nodes.

## Dependencies

- Phase 1 (Per-Voxel SDF Atom Type System) must be complete.
  The `atom_ref` bit layout (type_id[8] | pool_idx[24]) is part of the node hash input.
  DAG node equality is only well-defined once atom encoding is stable.
