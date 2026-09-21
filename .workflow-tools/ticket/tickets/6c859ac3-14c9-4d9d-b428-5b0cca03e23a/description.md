# Goal

Generalize the move kernel's journaling concept into a reusable operation journal contract for memory stores.

## Scope

- Define an `OperationJournal` envelope with ids, schema version, operation kind, preflight inputs, blockers, ordered steps, phases, touched entities/files, inverse operations, recovery instructions, and links.
- Represent reversibility explicitly: replayable, rollbackable, manual_recovery.
- Define read-only preflight, apply, resume, rollback, and replay semantics.
- Decide storage layout and index ownership for journal metadata.

## Implementation evidence

Implemented in owning observability architecture spec:

- `.spec/specs/aa769a27-2721-4b9d-880c-5c4e2f8136a7/body.md`
  - Added `OperationJournal envelope (v1)` section with required top-level schema fields and modeling requirements.
  - Added explicit `Storage and index ownership decision` section:
    - journal artifacts stored under `.log/<workspace_slug>/journals/`
    - `log-api` owns metadata indexing/query for journals
    - domain stores keep mutation authority
  - Added explicit resolution note in Open Decisions section for journal storage/index and schema envelope version.
  - Added traceability link to this ticket.

## Acceptance criteria status

- Schema models existing move-journal recovery details without loss: covered by explicit envelope requirement and compatibility statement.
- Non-mutating graph/search journals and rollbackable mutations are representable: covered by modeling requirements.
- Contract explicitly avoids treating trace logs as rollback journals: covered explicitly in envelope requirements.
- Storage layout and index ownership are now decided: covered by storage/index decision section.

## Depends-on context

Parent architecture boundary tracker: `84673399` (done).
Related profiling tracker dependency path: `ff6637f5`.