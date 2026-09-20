# Summary
`run_in_terminal` in `mode=sync` moved a command to background despite no explicit timeout and no clear completion signal, then required repeated polling and eventual manual termination.

## Reproduction context
Session: `a0228f9f-bbac-4c82-b1e6-8a628aa91ec1`
- Command started: `rtk cargo run -p rule-cli --bin rule -- sync-targets --config rule-targets.yaml`
- Event evidence: `.session/sessions/a0228f9f-bbac-4c82-b1e6-8a628aa91ec1/events.json` around lines ~2235 (start), ~2272 (complete), ~2300 and ~2442 (`get_terminal_output` follow-ups), ~2748 (`kill_terminal`).
- User-visible symptom from tool output: command reported moved to background due to extended no-output and never produced a final success/failure confirmation before manual kill.

## Expected
For `mode=sync`, completion should resolve deterministically with either:
1) final exit and buffered output, or
2) explicit interactive-input needed state, or
3) explicit timeout only when a user-specified timeout elapsed.

## Actual
A long-running non-interactive build-like command entered ambiguous state (`background` + repeated unchanged output), causing uncertain completion semantics and forced manual termination.

## Suggested fixes
1. Adjust sync-idle heuristic:
- Distinguish `no stdout yet` from `stuck` by checking process liveness and child process tree activity.
- Require stronger stuck criteria before backgrounding sync calls without timeout.
2. Add completion markers:
- Persist and return terminal-side process state transitions (`started`, `running`, `exited`, `reaped`).
- Include last known PID / exit status availability in `get_terminal_output` metadata.
3. Improve UX contract:
- If sync is backgrounded automatically, include actionable reason code and recommended next step (`wait`, `cancel`, `retry async`).
4. Validation:
- Add integration tests covering long compile with sparse output and ensure eventual `exited=0` is surfaced without manual kill.

## Related specs
- `f5e0df47-d0ec-4456-b268-689f2a41ecd7`
- `09f96d83-4795-4f19-9259-64ad0d452387`
