## Problem

`sessions_for_ticket` fails hard (aborts the entire scan) when the live `.session` store contains a malformed entry. Two such entries currently exist in the real store:

- `.session/sessions/6a51a1af-6812-4dfc-80d7-0e4f56b4af4f` — missing its `session.json` file.
- `.session/sessions/structured-ticket-entities-iteration` — a stray non-UUID-named directory (not a valid session entry at all).

A full-store scan by `sessions_for_ticket` errors out on hitting either of these, rather than skipping them. As a direct consequence, the dogfood run for ticket bba9b313-ff13-4fd1-91d4-6485a6c2f4de could not run against the real `.session` store at all and had to be run against a scrubbed scratch copy at `/tmp/session-dogfood` instead — meaning the shipped query has never actually been validated against the real, messy store it will run against in production.

This is a robustness defect in code merged this session (bba9b313), not a data problem to fix by deleting the two bad directories.

## Acceptance Criteria

- AC1: A malformed or incomplete session directory (missing `session.json`, non-UUID directory name, unparseable JSON, or any other structurally invalid entry) is skipped during a `sessions_for_ticket` scan with a logged warning, not an aborted/failed scan.
- AC2: A regression test seeds a fixture store containing at least one corrupt/malformed session entry (e.g. a directory missing `session.json`) alongside valid session entries, runs `sessions_for_ticket`, and asserts the scan completes successfully and returns the correct valid matches (i.e. the corrupt entry is silently skipped, not silently swallowing valid results too).
- AC3: A second regression test case covers a non-UUID-named stray directory under `sessions/` and asserts it is likewise skipped without aborting the scan.
- AC4: Do not delete or otherwise modify the two known-corrupt entries in the live `.session` store as part of this ticket's fix — those are exercised as a real-world case, not remediated data.

## Evidence

- `.session/sessions/6a51a1af-6812-4dfc-80d7-0e4f56b4af4f`: missing `session.json`.
- `.session/sessions/structured-ticket-entities-iteration`: stray non-UUID directory.
- Dogfood workaround: had to run against scrubbed copy at `/tmp/session-dogfood` (see ticket 33463861-ffba-4ead-905e-5d867b707936) because a full-store scan aborted on these two entries.
