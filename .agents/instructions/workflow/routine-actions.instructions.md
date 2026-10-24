---
description: "Use when deciding whether to narrate or execute routine operations. Covers eliminating explanatory self-talk for obvious next steps."
---

## Routine Action Discipline

Do not spend reasoning budget on actions whose next step is already obvious from the current local hypothesis.

Examples:
- If the touched slice has one relevant test, run it instead of narrating why that is probably the right test.
- If a command failed because of cwd drift, rerun it from the correct directory instead of exploring multiple explanations.
- If the correct tool is already loaded and known, call it instead of searching for it again.

Rules:
- Prefer direct execution over explanatory self-talk for routine operations.
- Avoid repeating unchanged state checks such as `git status`, board reads, or ticket fetches unless a write or external change occurred.
- After a long command spills output, inspect the spill artifact directly instead of re-running the command.
- Convert retries into one-line findings in subsequent summaries.
