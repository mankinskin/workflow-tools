Extend the existing `audit` tool (crates: audit-api, audit-cli, audit-mcp under workflow-tools/audit/) with a new rule that parses Markdown relative links inside the agent guidance corpus (.agents/instructions/**, .agents/agents/**, .agents/prompts/**, .agents/skills/**/SKILL.md, AGENTS.md files) and reports any link whose resolved target path does not exist on disk.

Context / origin: dossier transcripts/09-09-2026_guidance-link-audit-hook/ROADMAP.md (waypoint W1). See ARTIFACTS.md in that dossier for two verified concrete examples of currently-broken links (generated-files.instructions.md -> ../../../AGENTS.md; multiple files -> .agents/instructions/orchestration/... which no longer exists after a workflow/ rename).

Acceptance criteria:
- `audit run .` (or the equivalent MCP call) reports every Markdown link in the guidance corpus whose relative target does not resolve, with source file + line + broken target in the output.
- External (http/https) links and anchor-only links (#section) are excluded from resolution checks (out of scope for this rule).
- The rule is covered by a unit/integration test using a small fixture set including at least one known-broken and one known-good link.
- No cross-repository link resolution is attempted (see dossier non-goal): a link whose target lives outside the current repo boundary is either skipped or reported separately as "unverifiable-cross-repo", not treated as broken.