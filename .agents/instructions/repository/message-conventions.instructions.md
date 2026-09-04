---
description: "Use when drafting or reviewing commit messages and multi-commit batch strategies. Covers conventional commit types, scope guidance, and body format."
---

## Conventional commit types

Use the conventional commit format: <type>(<scope>): <imperative summary>

Common types: feat, fix, chore, refactor, docs, test, perf

## Scope and examples

Use crate name or subsystem as scope: `feat(token-efficiency): add peek-cli`

## Body format

For non-trivial commits include a short body with bullet points and changed paths.

## Ticket checkpoint suggestions

Suggest a commit checkpoint when a ticket transitions to `done`, multiple related tickets finish, or a dependency graph changes materially.
