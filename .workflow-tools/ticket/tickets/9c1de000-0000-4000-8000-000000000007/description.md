# Add minijinja and sandboxed template loading to rule-api

## Context

There is no templating engine anywhere in the workspace — `grep` for `handlebars|tera|minijinja|askama|liquid` across every `Cargo.toml` returns zero matches. `memory-api/crates/rule-api/Cargo.toml` depends only on `memory-api`, `feedback-api`, `serde`, `serde_json`, `serde_yaml`, `toml`, `chrono`, `uuid`, `thiserror`, `tracing`.

Rendering is `String::push_str` concatenation via `memory_api::generated_markdown::render_markdown_file`.

## Decision

`minijinja` — Jinja2 syntax, minimal transitive dependencies, sandboxed by default.

## Scope

- Add `minijinja` to `rule-api`.
- Introduce a template environment that loads `.j2` templates from a client-profile directory.
- **Security:** run with the sandbox enabled; templates must not be able to read the filesystem, open network connections, or escape the provided context. Template loading paths must be validated against the profile root to prevent traversal.
- Define the template context struct: rule entries with their structured metadata, resolved target metadata, output path, and provenance constants (`GENERATED_FILE_COMMENT`, `GENERATED_ENTRY_PREFIX`).
- Keep the existing concatenation renderer available for non-templated targets during transition.

## Acceptance criteria

1. A trivial template renders a rule set to markdown.
2. A template attempting filesystem or network access fails, with a test proving it.
3. A template path escaping the profile root is rejected.
4. Template syntax errors surface with the template name and line number.
