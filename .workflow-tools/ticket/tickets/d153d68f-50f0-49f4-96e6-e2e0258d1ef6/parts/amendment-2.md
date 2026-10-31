# Effective design - cold-read correction

This supersedes part `7cd9270b-1662-4e2e-a1bd-cf3dbef75adf` for W7. Implement [the amended behavior authority](../../../../spec/specs/ec2f0709-180f-4ccf-b692-601796f33e05/body.md) with unchanged criterion IDs and [current execution package](../../../../../../transcripts/29-10-2026_mission-native-mcp-planning/07-native-workflow-switch.md). Frozen original text remains historical, not a competing plan.

## Concrete interface and provider contract

New-run native planning and explicit classic migration are separate routes. Add mission_native_routing as a future integration test target.

- **MN-W7-01**: Edit parent .agents/prompts/{refine-ingest,execute-ingest,execute-roadmap}.prompt.md and product .agents/instructions/workflow/{prompt-ingestion,roadmap-authoring,roadmap-execution}.instructions.md plus tools/roadmap-route.py and install/README.md. Both .claude/commands links alias .agents/prompts; do not generate phantom .github guidance or a rule-target registry. Installed bodies are runtime guidance-install closure, not COMMANDS.md or artifact-catalog projections. Preserve planning-before-approval and no-execution-before-approval.
- **MN-W7-02**: Retain input cleaning/ambiguity clarification, bounded research, cold walkthrough, distinct formal review and exact requester decision gates. Route source capture through Session intake and use Mission-owned managed source/projection locations instead of manually scaffolded transcript folders.
- **MN-W7-03**: Require W6 capability/version discovery. Missing native capabilities or requester authority return actionable failures with no silent classic fallback or hand-written lifecycle script requirement.
- **MN-W7-04**: Preserve existing classic dossiers and active routes unchanged as provenance; mass migration, archive deletion and unsolicited active-route replacement are excluded. Explicit migration uses its existing validated separate route.
- **MN-W7-05**: Create mission/crates/mission/tests/fixtures/mission-native-guidance.toml with item mission-native-route selecting the six canonical Markdown paths from the explicit meta source root, and mission/crates/mission/tests/mission_native_routing.rs. Exercise install-ctl guidance plan/install from its separate manifest against disposable repo-scope targets both with initialized product submodule and without one. Require nonblocking closure, correct ownership destinations, rewritten-link integrity, source/installed routing parity, unchanged second install and alias correctness where present. Existing roadmap-route.py accepts classic/mission-backed only; add native-new-run and test all three modes without altering existing modes. Cover fresh native entry, stop-before-approval, missing capability and no manual fallback. Commit product sources/tests/profile first, then parent prompts and gitlink; no runtime installation bodies committed.

The [Spec interface contract](../../../../spec/specs/ec2f0709-180f-4ccf-b692-601796f33e05/body.md#interfaces-and-dependencies) and [correction inventory](../../../../../../transcripts/29-10-2026_mission-native-mcp-planning/ARTIFACTS.md) name the complete owner, authority, persistence, host/installer ingress and failure contracts. Do not replace them with guessed adapter or generated-output paths.

Planned guard: `cargo test --manifest-path workflow-tools/Cargo.toml --locked -p mission --all-features --test mission_native_routing -j 1`.

Checkpoint: `feat(workflow): default new intake to Mission-native planning` after the guard. No execution or implementation evidence is asserted.
