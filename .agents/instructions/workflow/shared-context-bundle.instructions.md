---
description: "Use at the start of and throughout every session, whenever spawning a sub-agent: compiled delegation prompt protocol that gives sub-agents a compact, self-contained brief instead of raw artifact dumps. Covers prompt composition, named-artifact referencing, progressive compilation across a delegation chain, and parallel fan-out."
applyTo: "**"
---

## Purpose

Sub-agents spawn with zero inherited context and must rediscover artifacts independently if the orchestrator names them but does not explain them. The fix is NOT to paste every artifact's full content into the prompt — that inflates orchestrator output tokens and duplicates content the sub-agent's own tools can fetch directly. The fix is a **compiled prompt**: the orchestrator's own distillation of prior context, objective, plan, and boundaries, with artifacts referenced by resolved path/id for the sub-agent to fetch itself when it needs more than the brief.

This is also how a fresh session started after a Worker's one-step-then-terminate dispatch receives its context — see [write-and-die.instructions.md](write-and-die.instructions.md).

## Measured Cost

Cross-agent duplicate reads from analyzed sessions (`tmp/subagent_cost_probe.py`):

| artifact | distinct sub-agents reading | total reads |
|---|---|---|
| handoff package JSON | 10 sub-agents | 14 reads |
| `compact-terminal-mcp/src/server.rs` | 6 sub-agents | 21 reads |
| MCP config files | 3 sub-agents | 10 reads |
| spec body.md | 3 sub-agents | 3 reads |

Within-agent redundancy is worse: one sub-agent read `subagent_rollup.rs` 6 times, `body.md` 4 times, `lib.rs` 4 times.

Parallel fan-out siblings issued byte-identical command sequences: same `ticket.exe get`, `spec.exe get`, `spec.exe search`, and file globs.

These numbers motivate compiling a good prompt, not inlining full content: a sub-agent that fetches `spec.exe get <id>` once because the compiled prompt named it precisely is cheap; the redundancy above came from sub-agents groping for artifacts the orchestrator never named, not from artifacts named but not pasted.

## Compiled Prompt Composition

A compiled prompt is the orchestrator's own summary, written fresh for this delegation — not a template you fill with pasted content. It names every artifact the sub-agent needs by resolved path or id and states what the sub-agent should conclude from it, then trusts the sub-agent's own tools (ticket-mcp, spec-mcp, peek-mcp, file reads) to fetch full content on demand.

**Required sections, in order**:

0. **Execution identity** (always first): resolve the target repository from the active VS Code workspace root before selecting a command directory. Keep workspace scope, code location, entity-store location, and command location separate. The generic word `workspace` is insufficient for these values.
   ```
   execution_identity:
     session_id: <copilot-session-uuid>
     workspace_root: <absolute active VS Code workspace root>
     code_worktree: <absolute assigned worktree path>
     git_toplevel: <expected git top-level path>
     branch: <expected feature branch>
     entity_store_root: <absolute canonical store root>
     command_cwd: <absolute directory for the delegated command>
   ```
   Every sub-agent MUST attest these six values before its first task command.
   A mismatch is a blocker, not a path to repair or work around. Never reuse a
   worktree path from another session. Never infer `entity_store_root` from
   `command_cwd`, and never create or use a worktree-local `.ticket`, `.spec`,
   `.test`, or `.session` shadow store unless the prompt explicitly declares
   that store as the canonical target.

1. **Available prior context** — named, not pasted. Reference the ticket/spec ids and titles, prior decisions, and findings compiled from earlier units in this chain, in prose or a short list:
   ```
   prior_context:
     ticket: <short-id> "<title>" (state: <state>)
     spec: <short-id> "<slug>"
     decisions: ["<decision 1>", "<decision 2>"]
     prior_findings: ["<compiled finding from unit N>", ...]
   ```
   The sub-agent fetches `ticket_get <short-id>` or `spec_get <short-id>` itself if it needs the full body — do not paste it here.

2. **Core objective and validation metrics** — the one outcome this unit must produce and the exact commands/criteria to check during and after the work:
   ```
   objective: "<single-sentence outcome>"
   validation:
     - "cargo test -p compact-terminal-cli"
     - "cargo test -p compact-terminal-mcp"
   ```

3. **Exact planned steps** — the ordered steps already worked out for this unit:
   ```
   steps:
     1. "<step>"
     2. "<step>"
   ```

4. **Non-goals, constraints, boundaries**:
   ```
   non_goals: ["<out-of-scope item>"]
   constraints: ["<hard constraint, e.g. branch/worktree/tier>"]
   ```

**Named artifacts**: every ticket, spec, or file the sub-agent needs is named by its resolved path or id — `memory-api/crates/session-api/src/model/handoff.rs`, ticket short-id, spec slug — never pasted in full. The sub-agent has the same ticket-mcp/spec-mcp/peek-mcp tools the orchestrator used to learn the reference and fetches it directly.

**Prompt size guidance**: target 1k-2k tokens for the compiled prompt. If you are pasting more than a short excerpt of any single artifact, you are inlining instead of compiling — name it and let the sub-agent fetch it.

## Parallel Fan-Out Optimization

When spawning parallel siblings (independent READ-ONLY probes dispatched concurrently), compute the **shared prior-context summary** ONCE in the orchestrator, and reuse it verbatim across EACH child prompt.

**Pattern**:

1. Identify the shared reference set (e.g., all siblings act against the same ticket/spec)
2. Resolve and summarize those references ONCE
3. Reuse the identical "available prior context" section in every sibling's prompt
4. Each sibling still gets a unique objective, steps, and non-goals — only the prior-context summary is shared text

**Cost model**: reusing a short shared summary across siblings avoids each one independently exploring to reconstruct the same context, while staying far smaller than inlining full artifact content per sibling.

## Read Deduplication Within a Sub-Agent

Sub-agents must not re-read files they already have in their own transcript.

**Rule for sub-agent templates**: Before issuing a file read, check if the file was already read in this session. If the file content is unchanged (same path, no intervening edit), reference the prior turn instead of re-reading.

**Implementation**: Add to sub-agent guidance (Implement, Research, Testing, etc.):

```markdown
## Read Deduplication

Before reading a file, check if you already read it in this session. If you did and the file has not been edited since, DO NOT re-read it — reference the prior turn number instead.

Reading the same unchanged file twice in one session is ALWAYS waste.
```

## Eliminating Agent Template Reads

Sub-agents currently read `.agents/agents/*.agent.md` files to understand their role or the delegation system. This is pure waste — the orchestrator already knows the contract and should state it inline.

**Fix**: When delegating, the orchestrator prompt MUST include the relevant contract excerpt from the target agent's template. Do not make the sub-agent read the template itself.

**Example** (in orchestrator delegation prompt):

```markdown
You are dispatched as an Implement Agent. Your contract:
- Act on the compiled prompt below; fetch any named ticket/spec/file yourself if you need more than the summary given
- Make the smallest correct change that satisfies the objective
- Validate immediately after the first substantive edit, using the listed validation commands
- Return: implementation target, edits made, validation run, remaining risk

Compiled prompt:
<prior context / objective+validation / steps / non-goals — see Compiled Prompt Composition above>
```

## Session-Scoped Artifact Cache (Future)

**Scope for future work**: A session-scoped cache keyed by `(path, content-hash)` could return a cheap "unchanged, see turn N" marker for repeat reads. This is NOT in scope for this ticket — the immediate fix is prompt discipline: compile a prior-context summary, name artifacts, and let sub-agents fetch what they need.

## Integration with Orchestrator Template

See the "Compiled Delegation Prompts" section of [orchestrator.agent.md](../../agents/orchestrator.agent.md) for the canonical statement of this contract.

## Integration with Delegation Instructions

See item 4 of the delegation contract in [orchestrator-delegation.instructions.md](orchestrator-delegation.instructions.md), which points here for the compiled-prompt composition rules.

## Validation

This is prose-only guidance that cannot be mechanically tested. The acceptance check is:
1. Compiled prompt composition is defined with exact section structure (prior context, objective+validation, steps, non-goals)
2. Named-artifact referencing (path/id, not pasted content) is the stated default
3. Parallel fan-out reuse of a shared prior-context summary is documented
4. Progressive compilation of prior findings across a delegation chain is documented
5. Read deduplication rule for sub-agents is stated
6. Prompt size target (1k-2k tokens) is specified

## Relation to Benchmark

Benchmark ticket `10d21210` includes a scenario with parallel siblings needing the same artifact. With compiled prompts applied:
- The shared prior-context summary is written ONCE by the orchestrator
- Reused verbatim across every sibling prompt (cheap input cost)
- Artifacts are named, not pasted — siblings fetch them directly only if the summary is insufficient
- The count of artifacts read redundantly by >2 distinct sub-agents drops versus a raw-dump baseline, without inflating orchestrator output tokens
