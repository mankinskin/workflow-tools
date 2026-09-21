# Migrate frontmatter out of body.md into structured metadata

## Context

`GeneratedMarkdownConfig { file_comment, entry_prefix, skip_provenance_for_yaml_frontmatter }` in `memory-api/crates/memory-api/src/generated_markdown.rs` splits a leading `---` block off the first entry body and emits it above the provenance comment. `parse_generated_artifact` re-attaches it on the way back. The behaviour is covered by `render_markdown_file_keeps_frontmatter_first_and_emits_provenance` in `memory-api/crates/rule-api/src/render.rs`.

This special case is exactly what makes a fragment client-specific. Once client profiles render frontmatter from structured metadata, the hoisting path becomes both redundant and actively harmful.

## Scope

- Migrate the 27+ bodies that begin with `---` into structured metadata fields, leaving pure prose in `body.md`.
- Retire `skip_provenance_for_yaml_frontmatter` and its hoisting path.
- Update `parse_generated_artifact` so reverse-sync reconstructs metadata into fields rather than back into the body.
- Update the existing render test to assert metadata-driven frontmatter instead of hoisted frontmatter.

## Acceptance criteria

1. No `body.md` in any `.rule` store begins with a YAML frontmatter block.
2. Rendering a Copilot instruction file produces byte-identical frontmatter to the pre-migration file.
3. The hoisting code path is deleted, not merely bypassed.
