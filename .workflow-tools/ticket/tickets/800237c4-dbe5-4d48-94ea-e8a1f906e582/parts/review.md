# W3 review — install-ctl closure and plan use markdown-links

Commits:
- install 58b0d6b: pure move of guidance/tests.rs into guidance/tests/*.
- install 1f658cd: graph.rs uses extract/classify_destination/resolve_local; plan rewrites moved to plan/rewrites.rs; links.rs, resolve_relative and the pulldown-cmark dependency removed; four new closure tests in tests/link_closure.rs.
- workflow-tools b123a4c: gitlink and Cargo.lock.

Gates:
- G-INST-INV-BASE and G-INST-INV-CHECK: 384 test names before and after, empty diff.
- G-INST-GUIDANCE: pass.
- G-INST-ACQUIRE: 12 pass.
- G-LINES-GUIDANCE: the only offender is the pre-existing 501-line autofix.rs, which W4 owns. All W3-touched files are ≤464 lines. Recorded in REVIEW.md "Execution deviations".
- Submodule branch policy: PASS.

Code review (09fbd4a..1f658cd): no significant issues.
