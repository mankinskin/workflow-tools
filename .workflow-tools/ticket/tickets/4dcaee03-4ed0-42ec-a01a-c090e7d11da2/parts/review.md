# W4 review — span-based install and autofix rewrites

Commits:
- install b14848e: `span_edit::replace_destinations`; encoded destinations and decoded "unchanged" comparison in `rewrite_link`; non-blocking `UnlocatableRewrite` with per-occurrence locatability; `DiagnosticCode::is_blocking` and `GuidancePlan::blocking_diagnostics`, used by install.rs and get.rs; render_text split into warnings and blocking; autofix uses span edits and refuses unlocated destinations; autofix tests moved to autofix/tests.rs.
- workflow-tools ccddb27: install gitlink.

Gates:
- G-INST-GUIDANCE: lib 169 pass; integration binaries 16 + 1 + 3 + 15 pass.
- G-AUDITCLI-LINKS: 5 pass.
- G-LINES-GUIDANCE (full gate): pass; autofix.rs is 337 lines.
- Submodule branch policy: PASS.

Code review (1f658cd..b14848e): no significant issues. Covered `rewrite_link` callers, span dedup and ordering, per-occurrence locatability, the blocking set, autofix refusal and the test split.
