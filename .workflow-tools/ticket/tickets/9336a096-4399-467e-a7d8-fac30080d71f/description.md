Build a generator that reads the rule store (rule-api) and emits a grouped catalog at `.rule/README.md` with its TOON sidecar at `.rule/index.toon`. The purpose is to give agents a compact, browsable map of all guidance rules, replacing the need to scan raw rule entry files and eliminating repetitive rule-layout descriptions from agent instruction files.

## Scope
- Implement a `store-index` subcommand (or extend `rule-cli`) that reads all rule entries.
- Group rules by slug prefix segments (D4) — e.g. `shared/agent-rules/`. No new `category` field is introduced for now.
- For each rule entry: emit slug, section, title, summary, tags, feedback-rating if available, and an `IndexRef` to the canonical rule entry.
- Conforms to the `IndexEntry` schema (`0dba399a`); co-located under `.rule/`.
- Emit an `.agents/` agent-hook entry pointing agents at the rules catalog (D1).
- Both index files plus the `.agents/` hook are committed to git (D5) and regenerated during the rule-file pre-commit hook (D2).

## Acceptance criteria
- Catalog output written to `.rule/README.md` and `.rule/index.toon`.
- Rules categorized by the segment hierarchy derived from the slug.
- A visual badge/indicator surfaces low-rated rules.
- Re-running with unchanged rule data is digest-stable.

## Non-goals
- No global store folder outside `.rule/`.
- Does not replace rule-api rule formatting.
- Does not add a `category` field to rule-api (slug-derived grouping only, D4).

## Resolved design decisions
- D4: group by slug prefix only. D8: TOON sidecar. D2: pre-commit. D5: committed.