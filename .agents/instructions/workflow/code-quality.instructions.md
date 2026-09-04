---
description: "Use when implementation, validation, audit output, compiler output, or lint output reveals a code-quality finding. Covers bounded proactive remediation and follow-up ticket capture."
applyTo: "**"
---

## Code Quality Findings

Treat code as a domain hierarchy of small modules, functions, types, and files
with one clear responsibility each. Rust code must be easy to understand,
search, and adapt without navigating mixed responsibilities.

Cheap, always-available signals include oversized files; files mixing unrelated
responsibilities; compiler warnings; Clippy warnings; and findings from the
repository `audit` tooling.

| Finding scope | Required action |
| --- | --- |
| Current unit touches the code or directly depends on the code | Block completion. Proactively fix the finding, validate the affected unit, then continue. This is in-scope work, not a broad refactor. |
| Current unit neither touches nor depends on the code | Create a follow-up ticket and continue the current unit without changing the unrelated code. Use `ticket.exe create --type task --title "<signal>: <path>" --body-file <finding.md> --workspace .`; the body records the signal, path, impact, and proposed remediation. |

A related fix may extract or separate the smallest cohesive responsibility needed
to remove the finding. Do not use a related finding to reorganize unrelated
modules, rename broad APIs, or clean repository-wide warnings. Follow the
owning [ticket workflow](../ticket/workflow.instructions.md) for ticket state
and completion evidence.

`memory-api/crates/session-capture-hook/src/main.rs` is an exemplar: a module
combining provisioning, capture-store resolution, spill statistics, feedback,
follow-up ticket synthesis, and metrics rollup signals a need to separate
domain responsibilities when a current unit touches or depends on the module.