## Review — workflow-skill SKILL.md

Scope: `.agents/skills/workflow-skill/SKILL.md` + master-index row in
`.agents/skills/README.md`. No publication/install-ctl TUI changes (reserved
for W9/W10), per the ticket's explicit non-goal.

### Criterion-by-criterion

**AC-1: `SKILL.md` exists with valid frontmatter and passes any repo-level
skill lint/validation used for other skills in this workspace.**
- Frontmatter has `name: workflow-skill` and a specific `description` trigger,
  matching the shape of every sibling skill (`find-skills`,
  `token-optimized-agentic-engineering`, etc.).
- No `rule-api:file generated=true` header — hand-owned, per the skill
  directory contract (ticket `5f217947`).
- Repo-level check: `audit hook workflow-tools` run from the root before and
  after authoring. Before: `Markdown links: 187 files, 580 checked, 0 broken,
  0 blocking, 487 skipped`, exit 0. After: `Markdown links: 188 files, 581
  checked, 0 broken, 0 blocking, 487 skipped`, exit 0. The new skill file and
  its cross-references parse clean; no new broken/blocking links introduced.
  (The known unrelated `cargo test --no-run` coverage pre-build failure
  inside the same `audit hook` run is a pre-existing, separately-tracked
  signal — distinct from the markdown-link/frontmatter gate — and is
  unaffected by this change; the overall `audit hook` process still exits 0.)

**AC-2: A reviewer can follow the skill's instructions from a clean checkout
through a working ticket/spec operation without consulting other docs.**
- Walked section 4 ("Worked example") verbatim in a scratch consumer
  directory (`C:\Users\linus\wf-skill-scratch`, deleted after the walkthrough):
  `ticket init` / `spec init`, `ticket search`, `ticket create`, `ticket claim
  --agent`, two `ticket update --to-state` transitions, `test record-spec`,
  `test record` (with required `--domain`/`--operation`/`--run-id`/`--ticket`
  provenance), `test get --id`, and a final `ticket get --view review`
  read-back. Every command as documented executed successfully against real
  binaries (`ticket.exe`, `test.exe`) with no doc beyond the skill file open.
  Fixed two inaccuracies discovered only by actually running the commands:
  `ticket claim` requires `--agent <agent-id>`; `test record` requires
  `--domain`/`--operation`/`--run-id` and at least one linked
  ticket/spec/criterion — both now called out explicitly in the skill text.

### Non-goals respected
- No `install.sh`, `install-ctl`, or `bootstrap.sh` modifications.
- No skills.sh publication, `skills-lock.json`, or install-ctl TUI changes.
- `install-ctl --list`/`guidance --help` inspected read-only to ground the
  bootstrap narrative in verified current behavior; confirmed install-ctl has
  no interactive bootstrap-selection TUI yet, so the skill text states the
  three scripts (`install.sh`, `install-ctl`, `bootstrap.sh`) are currently
  independent, not already wired together — avoiding preempting W9/W10.
