---
description: "Use when troubleshooting or configuring the repository pre-commit hook. Covers what the hook checks, how to resolve failures, and when bypassing is acceptable."
---

## What the pre-commit hook checks

The hook runs checks on staged files (examples):

- `.vscode/tasks.json` / `.vscode/tasks.d/*.jsonc` — regenerate tasks and reject drift

## Resolving failures

Regenerate the failing output and stage it before re-committing:

```bash
git commit -m "chore: regenerate generated output"
```

### Whitespace-only churn

Discard whitespace-only diffs in tracked source files before staging; never commit line-ending-only churn. `.gitattributes` normalizes tracked source files, complementing ticket `f76169f7`'s generated-output policy.

## Bypass (rare)

Only use `--no-verify` when you can justify the bypass in the commit message and the failure is a confirmed false positive:

```bash
git commit --no-verify
```
