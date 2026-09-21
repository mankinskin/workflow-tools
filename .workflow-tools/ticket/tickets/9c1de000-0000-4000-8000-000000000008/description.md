# Introduce the client-profile model and extend RenderTarget

## Context

`RenderTarget` in `memory-api/crates/rule-api/src/targets_model.rs` resolves to `{ name, repo_scope, file_kind, path_scope, section, state, nodes, output_path, source_config_path, source_output_root }`. There is **no** `headers` or `front_matter` field, and the only enums are `RenderTargetKind::{Root, Child}` and `RenderTargetNodeMode::{Replace, Append}`.

Selection is the 5-field `RenderTargetFilter` merged parent→child via `merged_with`; ordering is document order of `nodes` plus `order_key` within each node's `store.list()` result. That selection model is sound and is retained.

## Scope

- Define a `ClientProfile` config: profile id, output root, per-surface template mapping, per-surface output path pattern, and a metadata key-mapping table (neutral field → client frontmatter key, including casing differences such as `argument_hint` → `argument-hint`).
- Extend `RawRenderTarget`/`RenderTarget` with an optional profile reference and surface identifier.
- Make one logical target renderable under N profiles, producing N outputs, without duplicating the selection tree.
- Preserve backward compatibility for existing targets (`25-cline.yaml`, the README/AGENTS targets, the spec-doc targets) that declare no profile.

## Acceptance criteria

1. One target definition renders to multiple client outputs from a single selection tree.
2. All existing targets render byte-identical output with no profile declared.
3. `rule explain-target` reports which profiles a target renders under and where each output lands.
