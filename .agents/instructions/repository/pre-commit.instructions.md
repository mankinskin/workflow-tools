---
description: "Use when troubleshooting or configuring the repository pre-commit hook. Covers what the hook checks, how to resolve failures, and when bypassing is acceptable."
---

## What the pre-commit hook checks

Activate the repository hook path once per checkout:

```bash
git config core.hooksPath .githooks
```

The same activation applies to the meta-workspace root, `context-engine`,
`workflow-tools`, and each nested workflow-tools repository that carries a
`.githooks/pre-commit` entry point.

The hook runs checks on staged files (examples):

- `.vscode/tasks.json` / `.vscode/tasks.d/*.jsonc` — regenerate tasks and reject drift
- Markdown guidance under `.agents/**` or `AGENTS.md` — run the unified `audit hook` command and reject broken local targets

## Repository audit hook gate

When staged Markdown includes agent guidance, the hook runs:

```bash
audit hook <repository-root>
```

The gate is hard-blocking: a non-zero result names the source file, line, and
missing target. Repair the link and re-stage the changed Markdown before
committing. The hook runs the repository audit surface rather than only the
markdown link rule, so it covers the same full validation path as the canonical
audit command. The check intentionally skips external links, fragment-only
links, placeholders, cross-repository targets, and nested-repository targets
because those targets are outside the current repository's verifiable scope.

If the hook fails, the suggested repair command is:

```bash
install-ctl guidance autofix --repo-root <repository-root> --apply --yes
```

Use `--autofix` on the audit hook itself when you want the guard to attempt the
same remediation before exiting non-zero:

```bash
audit hook <repository-root> --autofix
```

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
