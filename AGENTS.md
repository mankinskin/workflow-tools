# Agent Rules

Global working rules for this repository. Keep this file small and stable.

## Operating Principles

- Gather context before coding. Do not guess.
- Read existing tests to infer expected behavior.
- Tickets track large work that spans multiple sessions or multiple files/components — not small refactorings or single-file fixes. Create or update the relevant ticket(s) before editing code only when the work meets that bar; see Task Routing below for the exact threshold.
- For new or changed requirements and goals, create or update the relevant spec before implementation proceeds. A small, self-contained change does not require a spec update unless it changes a documented requirement or goal.
- Keep the ticket, spec, validation, and documentation trail current so review and status summaries stay accurate.
- Commit every completed change on its owning feature branch; uncommitted completed work is not a valid handoff.
- Prefer bash commands over PowerShell/cmd.
- Use Unix-style paths (`/`) in commands and docs.
- Read test logs in `target/test-logs/` for debugging instead of relying on truncated test stdout.
- Keep scope tight: do not add extra features or broad refactors unless requested.
- Declare the session identity and execution checkout at the start of each session, then repeat both in the final response; use the main checkout unless the session has an explicitly registered worktree. Follow the [agent world-model narrative](../workflow-tools/.agents/instructions/workflow/agent-world-model.instructions.md) for the lifecycle and return contract.

## Checkout Resolution

The active VS Code workspace root is the starting scope for every task. Before
running a repository command, resolve the task target repository explicitly
from that workspace root and verify the candidate with `git -C <candidate>
rev-parse --show-toplevel`. If the request does not name a nested repository,
the workspace root is the candidate. An instruction-file physical location, an
absolute path embedded in prompt metadata, a pasted artifact path, and the
shell inherited working directory are evidence about artifacts only; none
selects the target repository or an execution checkout.

In this guidance, “main checkout” means the verified target repository main
checkout. A command may run from a nested directory, but that command directory
does not reselect the target repository. Do not create, look up, or register a
worktree until checkout resolution identifies the target repository and the
task has independently met the worktree criteria below.

## Discovery Protocol (Before Editing)

Use live sources first:

1. Documentation: use doc-viewer MCP tools to locate relevant module docs.
2. Known issues/plans: use ticket-mcp tools before duplicating work.
3. Board state: check active WIP, stale entries, and file ownership before touching
   implementation files — `mcp_ticket-mcp_board_show` with `{"workspace": "default"}` or:
   ```bash
   ./target/debug/ticket.exe board show --toon
   ```
4. Test failures: use log-viewer MCP tools (`get_log`, `search_all_logs`, `query_logs`).
5. Graph/workspace behavior: use context-mcp tools for context-engine operations.

Use static references as support:

1. Relevant crate `README.md` and `HIGH_LEVEL_GUIDE.md` for design context and
  API patterns.
2. Existing tests for usage examples, assertions, and type-level behavior.

## Task Routing

- **Ticket threshold**: a ticket is required only when the work is large enough to plausibly span multiple sessions — a feature, a cross-cutting refactor, a multi-file/multi-component change, or anything with unclear scope. A ticket is NOT required for a small, self-contained change: one existing file, or the addition of one new file, completable in the current session. Do not create a ticket for a small refactor just because it touches guidance, docs, or config rather than application code — the file type does not change the size threshold.
- Small, self-contained change (no ticket needed): may be made in the main checkout without a ticket or spec update. Verify that no active board entry owns the path, stage only the changed path, and validate before committing. It does not require worktree provisioning, `session_check_in`, or `board_check_in`.
- Simple fix (1-2 files, behavior requirements unchanged): gather context, implement, validate, and update docs directly — no ticket required.
- Bug fix: if it stays within the small-change threshold above, fix directly; otherwise create the tracking ticket first and capture the reproduction and validation evidence in the ticket.
- Feature or refactor (>5 files, >100 LOC, spans multiple sessions, or unclear scope): create or update the spec first, then establish implementation ticket(s) that reference the spec and plan how to reach it. Do not create a ticket to author spec content; a ticket describes how to get to the goal, not the goal itself, and referencing the spec (not restating it) is the ticket's responsibility.
- Worktrees are an opt-in isolation tool, not a ticket-size threshold. Default to the main checkout after checking the board for overlapping ownership. Create a worktree only when concurrent edits would conflict, the requester requires branch isolation, or the planned Git operation requires an independent branch. A ticket, a multi-file change, a submodule, or a risky change alone does not require a worktree. For worktree-backed work, follow the [repository workflow](../workflow-tools/.agents/instructions/repository/workflow.instructions.md). Rebase and integrate affected submodules before the superproject.
- Unfamiliar module or unclear behavior: follow the [research prompt](../workflow-tools/.agents/prompts/research.prompt.md) when available before locking the spec or implementation plan.
- Planning and implementation order are owned by [phase-separation.instructions.md](../workflow-tools/.agents/instructions/workflow/phase-separation.instructions.md); this section keeps only the ticket and spec size thresholds.

## Quality Gates

- Relevant validation must pass before completion. If a required check repeatedly fails, stop expanding scope and record the failing command, log or manual result, and blocker clearly in the ticket/spec status summary.
- Before a ticket moves to `in-review`, ensure the relevant spec is updated for the changed requirements or goals and links the related tickets, updated docs, and test or validation results.
- **Browser verification is mandatory** for any change to a server interface or frontend feature:
  open the affected viewer in an external fullscreen Chromium-family browser, not VS Code's integrated browser, and confirm the feature works visually before marking work done.
- Record the browser window or display resolution used for manual visual validation whenever layout, rendering, or responsive behavior could affect the result.
- **Write Playwright end-to-end tests** for all browser-facing features and server interface changes.
  When executing browser-hosted frontend checks, first try the MCP Playwright/browser tools. Fall back to repo-local Playwright commands only when the MCP surface is unavailable or cannot cover the scenario.
  Capture screenshots during Playwright verification for UI-facing changes so the rendered state is visually confirmed, not inferred only from DOM assertions.
  For modals, overlays, drawers, popovers, menus, and similar transient surfaces, include at least one screenshot with the surface open and, when useful, a before/after pair.
  Shared managed-viewer suites live under `viewer-api/viewer-api/frontend/dioxus/e2e/shared/`.
  Spec-viewer release E2E lives under `memory-viewers/spec-viewer/frontend/dioxus/`; run it with `npm run test:e2e:release`.
  Ticket-viewer release E2E lives under `memory-viewers/ticket-viewer/frontend/dioxus/`; run it with `npm run test:e2e:release`.
  Doc-viewer and log-viewer keep local Playwright wrappers under `memory-viewers/doc-viewer/e2e/` and `memory-viewers/log-viewer/e2e/`, importing shared suites from `memory-viewers/viewer-api`.
- For tracing-based tests, use:

```rust
let _tracing = init_test_tracing!(&graph);
```

- If public behavior or docs changed, update the docs and run doc validation workflows.
- When dedicated test, doc, or cross-store-link tooling is missing or partial, use the strongest available command or manual check and call out the limitation explicitly in the status summary and spec traceability.
- Follow `.github/hooks/` reminders when they fire.
- Scratch notes belong in temporary files only; do not commit ephemeral notes.
- Follow the closed-loop iteration workflow: Review→Interview→Commit→Handoff. See [loop-closure.instructions.md](../workflow-tools/.agents/instructions/workflow/loop-closure.instructions.md).
- Follow the production workflow cycle: request → spec → tickets → tests → implementation → validated response → next iteration. See [core-cycle.instructions.md](../workflow-tools/.agents/instructions/workflow/core-cycle.instructions.md).
- When a handoff package is incomplete or requirements are ambiguous, escalate rather than clarifying inline during implementation. See [escalation-gate.instructions.md](../workflow-tools/.agents/instructions/workflow/escalation-gate.instructions.md).
- Never commit directly to `main` for worktree-backed work — all such commits land on the feature branch. After the branch is rebased clean and validation passes, the session merges its own feature branch into `main` (bottom-up: rebase every affected submodule then the superproject onto updated `main`, resolve conflicts on the feature branch, then fast-forward each `main`). A validated main-checkout task may commit its explicitly staged paths directly to `main`. See the [repository workflow](../workflow-tools/.agents/instructions/repository/workflow.instructions.md) for commit and integration rules.

## Feedback Workflow

- Record feedback in the entity feedback store today. Use the canonical entity URN for the target, for example `ce://default/spec/<spec-id>` or `ce://default/ticket/<ticket-id>`.
- When feedback came from a specific hand-maintained instruction or prompt file, target the entity URN for the spec or ticket that owns that guidance instead of the file path.
- Record or inspect feedback with the feedback-api transports:
  - CLI: `feedback ingest|inbox|summary --store-root <path-to-.feedback> --workspace-slug <slug> --target <ce://...>` with `--source`, optional `--rating`, `--note`, `--note-kind`, `--session-id`, and `--author` on `ingest`.
  - MCP: `feedback_ingest`, `feedback_inbox` or `feedback_query`, `feedback_summary`, and `feedback_mine`.
- Use `feedback_summary` or `feedback summary` when you need the current low-signal state for an entity; use `feedback_inbox` or `feedback inbox` when you need the raw stored entries that explain why follow-up is needed.
- If feedback implies a contract or workflow change, open or update the corresponding spec or ticket and link the exact entity that received the feedback instead of leaving the signal stranded in chat only.

## Escalation Rules

Clarification and ambiguity blocking are owned by [escalation-gate.instructions.md](../workflow-tools/.agents/instructions/workflow/escalation-gate.instructions.md); question construction is owned by [question-quality.instructions.md](../workflow-tools/.agents/instructions/workflow/question-quality.instructions.md).

- In multi-agent workspaces, treat unrelated workspace changes as expected background activity and continue.
- Before worktree-backed editing, claim ownership of the files you will touch; commit only your owned changes, and release ownership when done. Before a small main-checkout change, inspect the board and do not touch a path actively owned by another agent.
- Ignore unrelated changes by default; do not interrupt work solely because they exist.
- Escalate only when unrelated changes create a real conflict with your owned scope (for example merge conflicts, overlapping owned paths, or failures directly caused by those changes).
- Never revert, stage, or commit unrelated changes created by other agents.

## Token-Efficient Output

See the canonical [workflow instructions](../workflow-tools/.agents/instructions/workflow/) for compact output, bounded file inspection, tool output handling, and model-cost-aware routing. For ticket reads, default to the narrowest `--view` profile (`summary` to orient, `plan` to implement, `review` to verify).

## Clickable Reference Policy

Render every reference to a workspace entity (ticket, spec, doc, log, or one of their files) as a clickable markdown link in **all** agent and prompt responses. This entry is the single canonical owner of reference formatting for the repository: the "Formatting conflict policy" note in the Instruction Precedence section defers here, and switching the reference format is done here once.

**Scope.** These rules govern the reference token you emit in a response — the markdown link and the path inside it. They do not govern ordinary prose that merely names a file, nor backticked shell commands. The anti-backtick rule below applies to the emitted reference, not to illustrative prose in this policy.

Emit a reference in exactly one of three forms, selected by the active reference mode (default: manifest):

1. viewer — a deep link to the domain viewer server that opens the entity. Routes that exist today:
   - ticket-viewer: http://localhost:3002/workspace/{workspace}/ticket/{id}
   - spec-viewer: http://localhost:4002/specs/{id}
   - log-viewer: http://localhost:3000/#/file/{url-encoded-log-name} (append /stats or /hypergraph for those tabs)
   - doc-viewer (port 3001) has no stable per-entity deep-link route yet — its artifacts are keyed by package::target, not a URL. Use manifest or description mode for docs until a route exists.
2. manifest — a relative link to the entity's manifest file: a ticket's ticket.toml, a spec's spec.toml, or the equivalent manifest.
3. description — a relative link to the entity's top description/body file: a spec's body.md, or a ticket's rendered description.md.

Link text is always "{short-id} {title}", where {short-id} is the first 8 characters of the authoritative entity id and {title} is the authoritative entity title.

Path normalization for every emitted reference:
- Use forward-slash (unix) paths only; convert any Windows backslashes.
- Use repo-root-relative paths; never emit a drive-letter absolute path (no C:/… form).
- Assume a mingw (Git Bash) or WSL shell, so a repo-root-relative unix path resolves for both file links and terminal use.
- Do not wrap the emitted reference — its link text or its path — in backticks.

Resolve manifest and description paths from the owning API (ticket-api, spec-api, and so on), not from a template. If the first response omits the folder path, run a follow-up call (for example ticket get {id} --json and read the payload's ticket path) before composing the reference.
