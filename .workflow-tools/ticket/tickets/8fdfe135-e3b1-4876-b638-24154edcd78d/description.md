## Objective
Implement the schema-modernization contract established in [e9c38d24 Schema modernization lifecycle and migration](.spec/specs/e9c38d24-42cc-4044-8b2c-6811b918530f/spec.toml).

## Decision Record
- Features are owned by specs; the new work-item types are `epic`, `bug`, `research`, `planning`, `implementation`, `review`, `interview`, and `testing`.
- Runtime schemas use strict single-parent inheritance and directed lifecycle graphs. Relation graphs remain separate ticket-edge graphs.
- Work-item lifecycle categories are `plan`, `act`, and `verify`; each concrete type must provide a path through all three.
- Shipped ticket schemas move to JSON; custom TOML and JSON remain supported with identical semantics. Type IDs are unique per schema kind; every collision is a full atomic-load error.
- Legacy records migrate through approved, idempotent transactional batches. Track 6 repairs only forward; it never reverts committed batches.

## Child Tracks
1. Engine and inheritance semantics
2. Dual-format schema loader
3. Resolved catalog and JSON built-ins
4. CLI and VS Code integration
5. Inventory and transactional migration
6. Cross-interface validation and release gate

## Done
All child tickets are done, linked validation evidence passes, and the linked spec is review-ready.


## Completion Evidence
Done requires every binding final-interview decision to be traceable to this spec and an owning acceptance/validation record, Track 5 migration-completion evidence, and an all-pass Track 6 validation-matrix artifact.


## Workflow Role Contract
Until Track 3 introduces target schema types, legacy storage types map through `target_schema_type` and `workflow_role`. Each implementation/testing slice must have completed target-role `research` and `planning` prerequisites; final completion also requires target-role `review`. The full sequence is recorded in spec e9c38d24's Implementation Workflow and Role Mapping section.

## Track Readiness Review — 2026-08-06

**VERDICT: RETURN-TRACK** (pre-implementation readiness review of Track 1; not the release-evidence gate owned by e03eb731)

Per-ticket:
- `8fdfe135` RETURN — live dependencies omit Track 1 and the epic acceptance criterion cannot verify all six child tracks.
- `85012858` RETURN — absent component is acceptable for cross-component research, but the required implementation brief has no durable target path or owner.
- `9e450826` RETURN — absent component is acceptable for cross-component planning, but the implementation plan has no required artifact location or review evidence.
- `7ef3f8db` RETURN — the implementation contract names no target files, focused test names, or executable validation commands.

**Graph finding:** the live graph is authoritative at 9 nodes / 8 edges; the asserted "19-ticket single chain" is not present. The `7ef3f8db -> 9e450826 -> 85012858` ordering does correctly prevent implementation from bypassing research and planning, and `8fdfe135 -> e03eb731` correctly makes release evidence a prerequisite for epic completion. The break is missing Track 1 linkage into the epic traversal, not edge orientation.

Findings:
- C1-C2 | 8fdfe135 | Epic dependency traversal excludes `85012858`, `9e450826`, `7ef3f8db`, contradicting the claimed single-chain normalization. | Reconcile the graph to the authoritative sequence and add the missing Track 1 edges.
- C4 | 8fdfe135 | "All six child tracks are validated" names neither the child tickets nor evidence artifacts. | Replace with an enumerated child-track/evidence checklist plus the required release-matrix artifact.
- C3-C4 | 85012858 | The research brief has no required durable location, format, or acceptance evidence for the planning ticket to consume. | Name the brief path or ticket part, its owning maintainer/component, and an inspection command.
- C3-C4 | 9e450826 | The planning output has no durable location or approval record, so `7ef3f8db` cannot consume a fixed contract. | Name the plan artifact location, required approval evidence, and the exact target-file/test-command fields it must populate.
- C4-C5 | 7ef3f8db | Requires "focused" validation but names no test files, functions, or commands; target implementation files are also absent. | Add an explicit target-file list, focused test seams/names, and runnable validation commands before dispatch.
- C5 | 7ef3f8db | The governing policy-rule requirement does not identify the existing rule artifact or validation location. | Name the rule file/entity to update and the focused rule-resolution validation command.

**Blocking before implementation:**
1. Reconcile `8fdfe135` dependency edges so Track 1 is included in the authoritative execution sequence.
2. Persist the `85012858` brief and `9e450826` plan at named artifact locations.
3. Add target files, focused test seams, and exact validation commands to `7ef3f8db`.

**Readiness of 7ef3f8db: NOT-READY** — lacks the concrete files and executable validation contract required for an Implement Agent to start without discovery.
