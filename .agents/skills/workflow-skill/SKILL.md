---
name: workflow-skill
description: Use when a consumer repository needs the workflow-tools ticket/spec/session/test/feedback bundle bootstrapped or operated — covers installing the CLI/MCP bundle via install.sh/install-ctl/bootstrap.sh and running the discover, plan, execute, feedback loop against ticket, spec, session, test, and feedback stores. Trigger on "set up workflow-tools", "bootstrap ticket/spec", "how do I track work as tickets", or "install the ticket CLI".
---

# Workflow-tools: bootstrap and operate the discover/plan/execute/feedback loop

`workflow-tools` (`github.com/mankinskin/workflow-tools` and its constituent
repositories) is the installable ticket/spec/session/rule/audit/test CLI and
MCP bundle that lets agents and humans track work as structured tickets and
specs instead of ad hoc markdown notes, and record validation/feedback
evidence instead of unverifiable prose claims.

## 1. What workflow-tools is

- **Ticket store** (`ticket` CLI / `ticket-mcp`) — structured work items with
  typed fields, state machine transitions, parts (objective, requirements,
  design, examples, acceptance criteria, review, validation), dependency
  links, board check-in/out, and full-text search.
- **Spec store** (`spec` CLI / `spec-mcp`) — hierarchical specifications
  (sections, cross-references) that tickets anchor to.
- **Session store** (`session-cli` / `session-mcp`) — session identity,
  worktree, and handoff bookkeeping.
- **Test store** (`test-cli` / `test-mcp`) — validation specs and executions
  (`record-spec`, `record`, `get`, `list`, `audit`) so validation evidence is
  queryable, not a narrative claim.
- **Feedback store** (`feedback-cli` / `feedback-mcp`) — surprising-behavior
  and drift reports (`feedback_ingest`, session summaries).
- **Audit** (`audit-cli` / `audit-mcp`) — repository health gate: file/line
  limits, compiler warnings, test success, markdown-link validation
  (`audit hook <repo>`).
- **Rule / peek** — generated-guidance synchronization and token-bounded
  inspection, used by guidance-authoring workflows, not by this skill's core
  loop.

Each store is a CLI binary and an MCP server over the same canonical
on-disk store (SQLite/Tantivy index + content files), so an agent host can
use either surface interchangeably against the same data.

## 2. Bootstrapping a consumer workspace

Three scripts compose the current, verified bootstrap path. Each is a
separate, independently invokable script — there is no interactive
selection step wiring them together yet (that wiring is tracked as later,
separate work; do not assume it exists).

1. **`install.sh`** — the public `curl | bash` entry point. Installs only
   `install-ctl` into a caller-supplied local directory (never a system bin
   path), pinned to a branch or revision:

   ```bash
   curl -fsSL https://raw.githubusercontent.com/mankinskin/install/main/install.sh \
     | bash -s -- --root "$HOME/.local/workflow-tools"
   ```

   Use `--dry-run` to preview the pinned `cargo install` command, or append
   `-- <install-ctl args>` to run `install-ctl` immediately after install
   (for example `-- --list`).

2. **`install-ctl`** — the installed binary. `install-ctl --list` enumerates
   every installable artifact by category (`cli`, `mcp`, `misc`, `service`),
   including `ticket`, `spec-cli`, `session-cli`, `test-cli`, `feedback-cli`,
   `audit-cli`, and their `*-mcp` counterparts. Install a selection with:

   ```bash
   install-ctl install ticket spec-cli session-cli test-cli feedback-cli audit-cli
   ```

   `install-ctl guidance plan|install|autofix|get` separately plans and
   installs `.agents/**` guidance corpora (not covered further by this
   skill).

3. **`bootstrap.sh`** (in the `workflow-tools` repository) — the minimal,
   version-pinned path to get only the ticket and spec stores running in one
   consumer workspace:

   ```bash
   bash bootstrap.sh \
     --root "$HOME/.local/workflow-tools" \
     --workspace /path/to/consumer
   ```

   It installs the `ticket` and `spec` binaries at pinned revisions into
   `--root` and runs `init` against `<workspace>/.ticket` and
   `<workspace>/.spec`. It never discovers a sibling consumer from a
   superproject working directory. Use `--dry-run` to inspect the exact
   pinned install/init commands first.

## 3. The discover / plan / execute / feedback loop

Once the stores are initialized, operate them through this cycle. Every
verb below has both a CLI form and an MCP tool form over the same store.

1. **Discover** — find existing, relevant work before creating new work.
   - `ticket search "<query>"` / `ticket list --state <state>` / `ticket board show`
   - `spec search "<query>"` / `spec tree <id>`
2. **Plan** — anchor new work to a spec and record intent as a ticket.
   - `ticket create` (objective/requirements/design/examples/acceptance parts)
   - `ticket link <from> <to>` to record dependencies
3. **Execute** — claim, transition, and implement.
   - `ticket claim <id> --agent <agent-id>` / `ticket update <id> --to-state <state>`
   - State machine is linear and explicit (for example `planning` →
     `ready` → `in-implementation` → `in-review` → `done`); read the
     rejection message from a failed `update` to find the allowed next
     states rather than guessing.
4. **Validate** — record evidence in the test store, not just prose.
   - `test record-spec --id <id> --title <title>` then
     `test record --id <id> --spec-id <spec-id> --outcome <passed|failed|blocked> --domain <d> --operation <op> --run-id <r> --ticket <id>`
   - `test get --id <id>` / `test list` / `test audit`
   - `audit hook <repo>` for repository-wide health gates (file limits,
     warnings, markdown-link validation).
5. **Feedback** — report drift or surprising tool behavior as you find it.
   - `feedback_ingest` (MCP) with `source: "agent"` and the relevant entity
   - `feedback_session_summary` for an end-of-turn visibility footer

## 4. Worked example: a clean-checkout ticket/spec operation

From a freshly bootstrapped consumer workspace, `--index-root`/`--workspace`
normalize to the canonical `.workflow-tools/<store>` location under the
workspace root you point at, so pass any path inside the workspace (for
example `.ticket`, `.spec`) and the CLI resolves the canonical store itself.
Every command below was run against a real scratch workspace and verified:

```bash
# Init (bootstrap.sh does this step for ticket + spec)
ticket --index-root .ticket init
spec --index-root .spec init

# Discover: is there already a ticket for this?
ticket --index-root .ticket search "healthz"

# Plan: create a ticket (anchor to a spec with `--field spec=<id>` if one exists)
ticket --index-root .ticket create --type task --title "Add /healthz endpoint" \
  --field component=api --field priority=medium
# -> id: <ticket-id>

# Execute: claim (an agent id is required) and progress through allowed states
ticket --index-root .ticket claim <ticket-id> --agent <agent-id>
ticket --index-root .ticket update <ticket-id> --to-state ready
ticket --index-root .ticket update <ticket-id> --to-state in-implementation

# Validate: record a spec, then a linked execution with provenance, then read back
test --workspace . record-spec --id healthz-check --title "healthz endpoint validation" \
  --command "curl -f /healthz"
test --workspace . record --id healthz-check-run1 --spec-id healthz-check \
  --outcome passed --detail "manual curl check" \
  --domain api --operation healthz --run-id run1 --ticket <ticket-id>
test --workspace . get --id healthz-check-run1
ticket --index-root .ticket get <ticket-id> --view review

# Feedback: only if something was surprising, not for routine blockers
```

Note: `test record` rejects an execution missing `--domain`/`--operation`/
`--run-id` or at least one linked ticket/spec/criterion — this is an
interoperability contract, not an optional nicety.

A reviewer following this section end to end, with no other document open,
reaches a working ticket/spec operation.

## 5. Related guidance (do not duplicate)

- `.agents/instructions/repository/workflow.instructions.md` — session and
  main-checkout vs. worktree discipline.
- `.agents/instructions/repository/submodule.instructions.md` —
  deepest-first commit ordering across nested repositories.
- `session/.agents/instructions/session/session-identity-and-handoff.instructions.md`
- `install/README.md` — full `install-ctl` command contract, profile schema,
  and guidance-corpus installer details.
