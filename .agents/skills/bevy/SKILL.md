---
name: bevy
description: >-
  Use when writing, debugging, or reviewing Bevy 0.18 ECS/plugin/render code, WGSL shaders, or the
  Dioxus-to-Bevy state/event boundary in the context-editor kernel
  (`context-engine/context-stack/tools/context-editor/kernel`) and its `sandbox-app` host. Covers
  ECS/plugin/resource/system conventions, the SVO/render-graph module boundaries, GPU double-buffer
  upload, WGSL shader-uniform layout, and the Criterion benchmark / visual-capture testing patterns
  used for this GPU-heavy surface. Trigger on any mention of Bevy, bevy::prelude, WGSL, wgpu, render
  graph nodes, SVO ray march, context-editor kernel, or sandbox-app.
applyTo: "context-engine/context-stack/tools/context-editor/**"
---

# Bevy (context-editor kernel)

This skill documents Bevy 0.18 patterns as they are actually used in the
context-editor kernel crate (`context-editor-kernel`, pinned in
[kernel/Cargo.toml](../../../../context-engine/context-stack/tools/context-editor/kernel/Cargo.toml))
and its `sandbox-app` host. It is grounded in the module-boundary
characterization from dossier waypoint W1
([01-characterize-architecture.md](../../../../transcripts/18-09-2026_context-editor-sandbox-architecture/01-characterize-architecture.md))
and direct source reads of this pass. Every pattern below cites the kernel
file that demonstrates it — do not extend a pattern beyond what its citation
shows.

This is a distinct topic from the
[dioxus](../dioxus/SKILL.md) skill (Dioxus component/signal/WASM-build
patterns for the managed viewers). Use `dioxus` for the DOM/component half of
`sandbox-app`'s UI overlay and this skill for the Bevy ECS/render/WGSL half;
see "Dioxus-to-Bevy boundary" below for where the two meet.

## 1. ECS / plugin / resource / system conventions

The kernel assembles ~28 `Plugin` impls into one `bevy::prelude::App` in
`build_kernel_app()`
([kernel/src/lib.rs](../../../../context-engine/context-stack/tools/context-editor/kernel/src/lib.rs)),
gated `#[cfg(target_arch = "wasm32")]` — the native build compiles a no-op
stub for this function and for `launch()`, `apply_registered_preset()`,
`register_world_presets()`, and `world_preset_names()`. Treat any claim about
`build_kernel_app()`'s runtime behavior as **wasm32-only**; a native
`cargo test` run never exercises it (confirmed: W1's native test run did not
compile the wasm32 arm).

- **One plugin per domain module, one `mod tests` per file.** Every
  behavioral module (`gpu`, `physics`, `render`, `splat`, `svo`, `editor`,
  `multiplayer`, `simulation`, `ui`, `world`) owns its own `Plugin` struct(s)
  and inline `#[cfg(test)] mod tests`. Example:
  `InteractionBridgePlugin` in
  [kernel/src/ui/interaction.rs](../../../../context-engine/context-stack/tools/context-editor/kernel/src/ui/interaction.rs)
  registers exactly one resource pair (`InteractionQueue`, `InteractionHits`)
  and one `Update`-schedule system (`process_interactions`) — keep a plugin's
  `build()` this small: resource init + system registration, no inline logic.
- **Resources are the cross-system contract, not events.** GPU buffers
  (`CameraUniformBuffer`, `SvoPageTableBuffer`, `SvoTransformBuffer`,
  `DoubleBindGroups`) are held as Bevy `Resource`s created once at startup in
  [kernel/src/gpu/mod.rs](../../../../context-engine/context-stack/tools/context-editor/kernel/src/gpu/mod.rs);
  the module doc comment states plainly "the hot render-loop path never
  allocates." Mirror this: allocate GPU-backed resources in a startup system,
  mutate them in place per frame, never in a hot per-frame system.
- **Crate-root re-exports are two kinds, and the difference matters.**
  `kernel/src/lib.rs` re-exports most submodules directly (`pub use
  render::runtime_params;`, `pub use splat::{force_compute,
  particle_splat};`) — these are **structural**, load-bearing paths used
  directly in `build_kernel_app()`. `editor`'s `pub use core::*` / `ux::*`
  wildcard inside `editor/mod.rs` is explicitly called out in its own doc
  comment as a **backward-compat shim** kept "so `crate::editor::Foo`
  continues to resolve as before." When adding a new submodule, re-export it
  by name at the crate root (the structural pattern); do not add a new
  wildcard shim.
- **`VoxelWorld` is the single source of truth for world geometry**, per its
  own module doc comment in
  [kernel/src/svo/mod.rs](../../../../context-engine/context-stack/tools/context-editor/kernel/src/svo/mod.rs).
  Every domain module that touches geometry (`physics`, `splat`, `render`,
  `gpu`, `editor`, `world::svo_lod`/`world::theme`) reads or mutates this one
  resource directly — there is no abstraction layer between them. A new
  system that needs geometry should depend on `Res<VoxelWorld>` /
  `ResMut<VoxelWorld>` directly rather than introducing an intermediate
  event or cache.

## 2. Architecture boundaries (grounded in W1)

These are the load-bearing module boundaries W1 characterized; treat them as
fixed contracts, not internal implementation detail to casually cross:

- **SVO authority** — [kernel/src/svo/mod.rs](../../../../context-engine/context-stack/tools/context-editor/kernel/src/svo/mod.rs)'s
  `VoxelWorld` owns `nodes: Vec<OctreeNode>`, `dirty_ranges`, and the
  `set_voxel`/`remove_voxel`/`apply_sdf_brush`/`carve_sdf_brush` mutation API.
  `take_dirty_ranges()` (line ~223) drains pending dirty byte ranges and MUST
  be called exactly once per frame — its own unit tests (`svo/upload.rs`
  lines ~258–280) assert the drain empties the queue, which is the regression
  guard for this invariant.
- **GPU upload / double buffer** — `svo::upload::SvoUploadPlugin`
  ([kernel/src/svo/upload.rs](../../../../context-engine/context-stack/tools/context-editor/kernel/src/svo/upload.rs))
  runs `svo_resize_system → svo_paged_upload_system →
  double_buffer_swap_system` in `PostUpdate`, draining `take_dirty_ranges()`
  and writing the BACK buffer via `gpu::DoubleBindGroups`
  ([kernel/src/gpu/mod.rs](../../../../context-engine/context-stack/tools/context-editor/kernel/src/gpu/mod.rs)).
  The swap is a pointer flip documented as "< 0.01 ms" — never write directly
  to the FRONT buffer or skip the swap system.
- **Render graph / depth / wireframe** —
  [kernel/src/render/mod.rs](../../../../context-engine/context-stack/tools/context-editor/kernel/src/render/mod.rs)
  fixes the node order as `BufferSwap → ParticleCompute → SvoRayMarch →
  DepthBridge → UiComposite → WireframeOverlay` via the `ContextEditorLabel`
  `RenderLabel` enum. This order is structural: `DepthBridge` depends on
  `SvoRayMarch`'s NDC-depth storage-buffer output, and `WireframeOverlay` is
  drawn last with depth testing against the composited result. A new render
  node must be inserted into this fixed sequence, not appended after
  `WireframeOverlay` by default.
- **Editor mutation** —
  [kernel/src/editor/sdf_cutting.rs](../../../../context-engine/context-stack/tools/context-editor/kernel/src/editor/sdf_cutting.rs)'s
  `sdf_csg()` implements standard CSG boolean ops on signed-distance values
  (`union = min`, `intersection = max`, `subtraction = max(a, -b)`); paint/carve
  itself is `VoxelWorld::apply_sdf_brush`/`carve_sdf_brush` in `svo/mod.rs`
  (above). Editor tools mutate `VoxelWorld` directly — they do not go through
  a separate command queue.
- **Dioxus bridge** — see §3 below.
- **World palette / LOD** —
  [kernel/src/world/theme.rs](../../../../context-engine/context-stack/tools/context-editor/kernel/src/world/theme.rs)'s
  `ThemePalette` resource converts `MaterialDef` → `VoxelMaterial::pack()` →
  `OctreeNode.color_data`, and exposes `to_standard_material()`, which
  `sandbox-app`'s `bootstrap.rs::sync_palette_materials` calls to rewrite
  `Assets<StandardMaterial>` when the palette changes — a kernel→app
  boundary, not an internal kernel contract. `world::svo_lod` reads
  `VoxelWorld` directly for LOD color propagation.

## 3. Dioxus-to-Bevy state/event boundary (and its evidence gap)

`ui::interaction::InteractionQueue`
([kernel/src/ui/interaction.rs](../../../../context-engine/context-stack/tools/context-editor/kernel/src/ui/interaction.rs))
is a `Mutex<Vec<KernelEvent>>` Bevy `Resource`: `push()` is documented "safe
to call from any thread" for the Dioxus/DOM side, and `drain()` is called
once per frame by the `process_interactions` system (registered by
`InteractionBridgePlugin`), which ray-casts against `VoxelWorld` and
populates the `InteractionHits` resource the render pipeline reads.

**Known evidence gap — do not assume more than this:** the only call site of
`InteractionQueue::push()` found in this kernel crate is its own unit test
(`ui/interaction.rs`, `push(KernelEvent { ... })` inside `mod tests`). No
production call from Dioxus DOM/`web_sys` event code into `push()` was found
in this pass, and `ui::bridge::UiPanelList`
([kernel/src/ui/bridge.rs](../../../../context-engine/context-stack/tools/context-editor/kernel/src/ui/bridge.rs))
only documents the reverse direction (Dioxus DOM → `UiPanelList` → GPU
composite buffer), not an event queue. Treat the Dioxus→Bevy enqueue path as
**unverified in production code**, not as a live bidirectional event bus —
this repeats and does not resolve the gap W1 already flagged. Do not describe
or design against a bidirectional mechanism this kernel has not been shown to
implement.

## 4. WGSL shader layout, validation, and testing conventions

- **Compact bit-packed material encoding.**
  [kernel/src/render/pbr_material.wgsl](../../../../context-engine/context-stack/tools/context-editor/kernel/src/render/pbr_material.wgsl)
  packs an entire PBR material into one `u32` (`OctreeNode.color_data`): 8
  bits each for R/G/B, 5 bits roughness, 1 bit metallic, 2 reserved. Mirror
  this pattern (`pack_material`/`unpack_material` pairs) for any new
  per-voxel shader attribute rather than adding a second wide buffer.
- **Explicit byte-offset comments on uniform structs.** Every uniform struct
  field in
  [kernel/src/render/svo_ray_march.wgsl](../../../../context-engine/context-stack/tools/context-editor/kernel/src/render/svo_ray_march.wgsl)
  carries an inline `// N bytes` comment and the struct closes with a
  `// Total: N bytes` comment; padding fields (`_pad1`, `_pad3`) are explicit,
  not implicit compiler-inserted alignment. The matching Rust-side struct is
  named in a comment (`matches Rust SvoTransformData`) — keep WGSL and Rust
  uniform layouts in sync by hand and comment the correspondence explicitly,
  since there is no shared codegen between them in this kernel.
- **Shaders are validated by compiling, not by a separate linter.** There is
  no standalone WGSL syntax checker in this kernel; a WGSL file's correctness
  is exercised only when the owning Rust module's pipeline is built (native
  `cargo test` does not compile shaders at all — pipeline creation is
  wasm32/GPU-backend gated). Treat a WGSL edit as validated only after a
  `cargo check --target wasm32-unknown-unknown -p context-editor-kernel` (or
  a full `trunk build` of `sandbox-app`) has actually compiled the pipeline
  that includes it.

## 5. Testing and benchmark patterns

- **Native unit tests** — `cargo test -p context-editor-kernel` runs the
  36 in-file `#[cfg(test)] mod tests` blocks across the 11 top-level modules
  (389 passed at W1's pass). This exercises only the native/no-op arms of
  any `#[cfg(target_arch = "wasm32")]`-gated code — it is not evidence about
  `build_kernel_app()`, `launch()`, or any GPU/render-graph behavior.
- **wasm check** — `cargo check --target wasm32-unknown-unknown -p
  context-editor-kernel` is the cheapest way to confirm the wasm32 arms
  (including `build_kernel_app()`'s 28-plugin assembly and every WGSL-owning
  render module) still compile, without running a full `trunk build`.
- **Trunk build** — `trunk build` from
  [sandbox-app](../../../../context-engine/context-stack/tools/context-editor/sandbox-app)
  (config in that directory's `Trunk.toml`) is the integration build that
  actually links the kernel's wasm32 target into the Dioxus host; use it as
  the final compile gate for any change touching `build_kernel_app()`,
  render-graph nodes, or WGSL shaders.
- **Validated visual capture and benchmarks** — dossier waypoint W3
  ([03-build-validation-foundation.md](../../../../transcripts/18-09-2026_context-editor-sandbox-architecture/03-build-validation-foundation.md))
  established a deterministic capture harness at
  `sandbox-app/e2e/verify-visual-baseline.mjs` that triggers the exact
  per-frame `take_dirty_ranges()` sequence before capturing, plus a Criterion
  `benches/` suite for SVO upload, splat preparation, and ray-march uniform
  preparation. Use the W3 ticket evidence and baseline artifacts when
  comparing later changes.
- **Criterion calibration and timeout discipline** — for the W3 `benches/`
  suite exists, follow
  [benchmarks-criterion-calibration.instructions.md](../../../test/.agents/instructions/testing/benchmarks-criterion-calibration.instructions.md)
  for per-scenario `measurement_time` calibration across a heterogeneous SVO
  upload/splat/ray-march matrix (shrink `measurement_time`, not
  `sample_size`, which has a hard floor of 10), and
  [benchmarks-timeout.instructions.md](../../../test/.agents/instructions/testing/benchmarks-timeout.instructions.md)
  for estimating wall time and setting a hard timeout before any `cargo
  bench` invocation.

## 6. Failure diagnostics and regression evidence for visual/GPU work

- **A green native `cargo test` is not evidence for GPU or wasm32 behavior.**
  State explicitly in validation evidence whether a claim about
  `build_kernel_app()`, a render-graph node, or a WGSL shader was checked via
  `cargo check --target wasm32-unknown-unknown`, `trunk build`, or an actual
  wasm32 execution — a native-only test pass is a distinct, weaker claim.
- **Distinguish compile failure from a failing assertion.** A WGSL or Rust
  pipeline compile error surfaces at `cargo check --target
  wasm32-unknown-unknown` or `trunk build` time, not at `cargo test` time;
  don't conflate a shader compile error with a unit-test assertion failure
  when reporting a regression.
- **The `take_dirty_ranges()` once-per-frame invariant is the primary
  regression guard for the upload path** — see
  [kernel/src/svo/upload.rs](../../../../context-engine/context-stack/tools/context-editor/kernel/src/svo/upload.rs)'s
  own tests. Any change touching the upload/double-buffer path should assert
  this invariant still holds (drain empties the queue) rather than only
  checking that `cargo test` passes overall.
- **A visual/GPU regression needs a captured artifact, not a described one.**
  Once W3's capture harness exists, cite the actual baseline capture path
  from that waypoint's evidence rather than describing an expected visual
  result from source reading alone — this mirrors the general rule in
  [data-capture-verification.instructions.md](../../../test/.agents/instructions/testing/data-capture-verification.instructions.md)
  that a capture claim requires reading the artifact back.

## Related skills

- [dioxus](../dioxus/SKILL.md) — the DOM/component half of `sandbox-app`'s UI
  overlay; use it for `rsx!`/signal/WASM-build questions and this skill for
  the Bevy ECS/render/WGSL half.
- [playwright-best-practices](../playwright-best-practices/SKILL.md) and
  [playwright-cli](../playwright-cli/SKILL.md) — informs the future visual
  capture harness in §5; no Playwright wiring exists for `sandbox-app` yet.
- [rust-best-practices](../rust-best-practices/SKILL.md) — general Rust
  idioms; this skill only covers what is Bevy/WGSL/context-editor-specific.
