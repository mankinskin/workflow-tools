---
description: "Use when creating, testing, rolling back, or promoting an isolated prototype version while an existing implementation remains the default."
applyTo: "**"
---

# Rapid Prototyping And Version Transitions

Use this workflow when a new implementation must be developed separately while
an existing version remains the default for tools and users. The workflow was
rehearsed in the `template/` repository with commit `b3b6c9f`.

## Fixed Boundaries

- Keep the stable implementation as the default target until explicit promotion approval.
- Put the experimental implementation in a separate version directory or sibling package.
- Use one explicit selector with only the documented values; the tested template selector is `TEMPLATE_TARGET=v1|v2` and defaults to `v1` when unset.
- Keep development data separate from stable data. For the Session API design, v2 uses `SESSION_API_STORE_ROOT` with `.workflow-tools/session-v2` as the default and treats `.workflow-tools/session/` as read-only reference data.
- Do not change stable configuration, APIs, runtime records, or launch behavior merely to make the prototype easier to test.

## Prototype Setup

1. Record the stable baseline commit and working-tree status before creating a prototype version.
2. Create separate `v1` and `v2` version boundaries.
3. Add a selector that rejects unknown values explicitly and verifies the selected version directory or package exists.
4. Test the default with the selector unset; the default must resolve to v1.
5. Test the experimental version by setting the selector to v2 without changing the default configuration.

The template rehearsal uses:

```bash
bash scripts/select-prototype-target.sh
TEMPLATE_TARGET=v2 bash scripts/select-prototype-target.sh
TEMPLATE_TARGET=v1 bash scripts/select-prototype-target.sh
```

## Validation And Rollback

Record all four observations:

- default selection resolves to v1;
- explicit v2 selection resolves to the isolated v2 boundary;
- v2 validation completes without changing stable data or configuration;
- selecting v1 again restores the stable path.

An invalid selector must fail with a non-zero status and an actionable
 diagnostic. Rollback is a selector operation: stop v2 callers, select v1,
 verify stable reads/resume behavior, compare stable data before and after, and
 retain the v2 output for analysis. Do not merge v2 data into the stable store
 during rollback.

## Promotion Gate

Promotion is a separate approved operation, not an automatic consequence of a
passing prototype test. Before promotion, collect reproducible evidence for the
library/package contract, CLI, MCP server, capture hook, handoffs, runtime
store, lifecycle behavior, rollback, and every launch configuration. Require a
human approval checkpoint and preserve the stable version as the rollback
reference.

For the Session API rewrite, do not create `session-api-v2`, change
`SESSION_API_TARGET`'s default, write v1 runtime records, or begin the rewrite
until the approved roadmap's W1-W5 gates and W6 handoff are complete.

## Required Evidence

Each transition record must name the baseline, selector values, selected paths,
commands run, outputs, changed files, stable-data no-write check, rollback
result, commit id, and approval status. An untested selector, an ambiguous
version boundary, a dirty unexplained baseline, or a missing rollback result
blocks promotion.
