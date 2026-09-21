# Purge machine-specific paths from generated-target state records

## Context

`.rule/entities/` holds 109 generated-target state records. Their `config_path` and `output_path` values are absolute Windows UNC paths of the form `//?/C:/Users/linus/git/graph_app/context-engine/...`, and the machine-specific path is **embedded in the slug** (`generated-targets///-/c-/users/linus/git/...`). These records are committed.

Additionally:

- **17** records point at `.../rule-targets/30-agents-prompts.yaml`, a file deleted in `4697eb3f`.
- **50** records use the directory form `.../rule-targets`, duplicating the `rule-targets.yaml` records.
- Duplicate `output_path` counts of 2–3 per file indicate three overlapping generations of tracking state.

Install-time generation on arbitrary machines cannot work with this addressing, and the duplication makes drift detection unreliable.

## Scope

- Change generated-target state addressing to repo-root-relative paths with forward slashes.
- Derive slugs from the relative path so they are machine-independent.
- Write a one-shot migration that rewrites existing records across all four `.rule` stores (root, memory-api, memory-viewers, viewer-api).
- Delete records whose `config_path` no longer exists.
- Collapse the directory-form duplicates against their canonical entry.

## Acceptance criteria

1. No record in any `.rule/entities/**` contains an absolute path or a drive letter.
2. No record references a non-existent config file.
3. Exactly one active record exists per `(config, target, output)` triple.
4. A regression test asserts that generated-target records round-trip through a different checkout root unchanged.
