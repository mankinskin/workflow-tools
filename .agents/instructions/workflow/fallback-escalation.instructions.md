---
description: "Use when compact tooling is unavailable or insufficient. Covers documenting tooling gaps and fallback patterns."
applyTo: "**/*.sh,**/*.ps1"
---

## Fallback Escalation

When compact tooling is unavailable or insufficient:
1. Note the limitation in the ticket/spec status summary.
2. Use the next-best available tool (e.g., bounded grep instead of full file read).
3. Do not silently fall back to full-file pulls — record the gap explicitly so the tooling can be improved.
