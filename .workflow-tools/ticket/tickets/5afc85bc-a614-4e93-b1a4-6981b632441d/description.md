# Waypoint 6: Ticket governance, recovery, and proportional satisfaction

## Governing specification

- `.spec/specs/f482eb83-5b47-4ea3-8d5b-b7baa0531333/body.md`

## Owned implementation surface

- `workflow-tools/spec/crates/spec-api/src/ticket_ref.rs`
- `workflow-tools/spec/src/cli/commands/validate_links.rs`
- ticket-api typed `SpecificationGate` persistence and validation-gate evaluation
- shared-journal integration points for ticket/spec cross-store recovery

Implement typed governing-spec relations, typed ticket gates, journaled cross-store recovery, and equal-weight proportional satisfaction. Consume W6.1 and the existing generic shared operation-journal ticket `73b2cd22`; integrate only after W6.5 health-policy contracts are available. This ticket must not author requirements, infer legacy generic relationships, add a first-class criterion index, select/persist executions or outcomes, or transition ticket lifecycle automatically.

## Acceptance criteria

1. Ticket and governing spec persist reciprocal typed identity; missing target, missing reverse relation, wrong store root, pre-spec planning, generic inference, empty/duplicate pairs, and invalid threshold are rejected.
2. Gates default to 100%, contain distinct `{validation_spec_id, acceptance_criterion_id}` pairs, query executions by validation spec, filter criterion links in the consumer, then select latest timestamp and lexicographically smallest id on ties.
3. Only passed contributes one; failed/blocked/absent contributes zero. A three-criterion 2/3 gate fails at 100 and passes at 60, while a newer failed/blocked shared result revokes every consumer without lifecycle change.
4. Cross-store interruption records planned/inverse writes, locks/collisions, recoverable drift, and explicit resume/rollback; it never silently repairs or requires a global transaction.

## Focused validation

- ticket-api/spec-api gate and link-validation tests
- gate-shape, tie-break, shared revocation, 100/60 threshold, reciprocal relation, interruption, resume, and rollback fixtures

## Done condition

Governed tickets can compute satisfaction from shared latest validation evidence and explicitly recover ticket/spec write interruptions.