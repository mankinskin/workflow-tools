## Validation — workflow-skill SKILL.md

Commands run and read back this session (root: `C:\Users\linus\git\3\meta-workspace`):

1. `audit hook workflow-tools` (baseline, before edits) — exit 0;
   `Markdown links: 187 files, 580 checked, 0 broken, 0 blocking, 487 skipped`.
2. Authored `.agents/skills/workflow-skill/SKILL.md` and added its row to
   `.agents/skills/README.md`.
3. `audit hook workflow-tools` (after edits) — exit 0;
   `Markdown links: 188 files, 581 checked, 0 broken, 0 blocking, 487 skipped`.
4. Scratch consumer walkthrough (`C:\Users\linus\wf-skill-scratch`, deleted
   after use — not inside this checkout):
   - `ticket --index-root .ticket init` -> `init ok`
   - `spec --index-root .spec init` -> `status: ok`
   - `ticket --index-root .ticket search "healthz"` -> `count: 0` (clean store)
   - `ticket --index-root .ticket create --type task --title "Add /healthz endpoint" --field component=api --field priority=medium` -> `create ok`, id `db28b547-79e0-4cfb-9879-2412bace89cc`
   - `ticket --index-root .ticket claim <id> --agent scratch-agent` -> `claim ok`
   - `ticket --index-root .ticket update <id> --to-state ready` -> `update ok`
   - `ticket --index-root .ticket update <id> --to-state in-implementation` -> `update ok`
   - `test --workspace . record-spec --id walkthrough-healthz --title "healthz endpoint validation" --command "curl -f /healthz"` -> `"status": "recorded"`
   - `test --workspace . record --id walkthrough-healthz-run1 --spec-id walkthrough-healthz --outcome passed --detail "manual curl check" --domain api --operation healthz --run-id walkthrough-run1 --ticket <id>` -> `"status": "recorded"`
   - `test --workspace . get --id walkthrough-healthz-run1` -> returns the recorded execution with correct `outcome`, `links.ticket_ids`, and `provenance`
   - `ticket --index-root .ticket get <id> --view review` -> `get ok`, state `in-implementation`

No simulated or hypothetical output above — every command is a real run
against real binaries (`workflow-tools/target/debug/ticket.exe`,
`workflow-tools/target/debug/test.exe`, `workflow-tools/target/debug/audit.exe`)
this session. The scratch directory was removed after the walkthrough and no
files outside `.agents/skills/workflow-skill/SKILL.md` and
`.agents/skills/README.md` were modified in this checkout.
