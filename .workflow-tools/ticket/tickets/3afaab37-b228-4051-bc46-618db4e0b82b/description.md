# Bug: `update_ticket` state regression + `transition_states` no-op

## Severity
High — silently corrupts ticket lifecycle state. Any agent that edits a ticket's `description` or `fields` via the update API loses the ticket's current state (reset to `new`), and the documented multi-step `transition_states` path does not transition at all. This was hit repeatedly during the session-bootstrap planning work and required a manual workaround.

## Affected surface
- API: `ticket-api` update path (the operation behind `update_ticket`).
- Exposed through: ticket-mcp `update_ticket`, and likely ticket-cli `update` / ticket-http update endpoint (verify all three transports).

## Symptom 1 — state reset on field/description patch
When `update_ticket` is called with `description` and/or `fields`/`field_map` but **without** `to_state`, the ticket's `state` is reset to the initial state (`new`), discarding the prior state (e.g. `ready`, `in-implementation`).

## Symptom 2 — `transition_states` silently no-ops
When `update_ticket` is called with the `transition_states` array (the documented multi-step transition path), the call returns success (`ok`) but the ticket state is **unchanged** — no transition is applied and no error is raised.

## Working workaround (current)
Single-step `to_state` works correctly and returns a `state_transition: { from, to }` confirmation object. Field/description edits must be done in a **separate** call from the transition, and the transition must be the **last** call, never combined with `description`/`fields` in the same request.

## Reproduction
Workspace: `default`. Use any `tracker-improvement` ticket currently in a non-initial state (e.g. `ready`).

1. Create + advance a ticket to `ready`:
   - `create_ticket(type=tracker-improvement, title=\"repro\")` → note id, state `new`.
   - `update_ticket(id, to_state=ready)` → returns `state_transition {from: new, to: ready}`; `get_ticket(id)` shows `state=ready`. ✅
2. **Trigger Symptom 1:** `update_ticket(id, description=\"changed\")` (no `to_state`).
   - Observed: `get_ticket(id)` shows `state=new` (regressed). ❌ Expected: `state=ready` preserved.
3. **Trigger Symptom 2:** re-advance to `ready`, then `update_ticket(id, transition_states=[\"ready\",\"in-implementation\"])`.
   - Observed: call returns `ok`, `get_ticket(id)` shows unchanged state, no `state_transition` in response. ❌ Expected: state advanced (or an explicit error if the path is unsupported).

## Acceptance criteria (test-validatable)
1. **State preservation:** `update_ticket` with `description` and/or `fields`/`field_map` and no `to_state` MUST NOT change `state`. Add a regression test: create → `to_state=ready` → patch description only → assert `state == ready`.
2. **transition_states honored or rejected:** `transition_states` MUST either (a) apply the full transition sequence and return a `state_transition`/transition record, or (b) return an explicit validation error. It MUST NOT return success while leaving state unchanged. Add a test asserting the resulting state equals the final element of `transition_states` (or that an error is returned).
3. **Combined edit + transition:** `update_ticket` with both a field/description patch and `to_state` in one call MUST apply the patch AND the transition (final `state == to_state`), with no regression to `new`. Add a test.
4. **Transport parity:** equivalent regression tests (or at least manual verification notes) for ticket-cli `update` and ticket-http update, confirming the fix is in the shared API layer, not just the MCP adapter.
5. **No required_states bypass:** confirm the fix still enforces `required_states` (e.g. cannot reach `done` without visiting `in-review`).

## Investigation notes
- Likely the update handler reconstructs the ticket from patched fields and re-derives `state` from a default rather than preserving the persisted current state when `to_state` is absent.
- `transition_states` handling likely parses the array but never drives the state machine (dead parameter), or applies it before the field patch overwrites it.
- Root-cause in the shared `ticket-api` layer is preferred so all transports inherit the fix.