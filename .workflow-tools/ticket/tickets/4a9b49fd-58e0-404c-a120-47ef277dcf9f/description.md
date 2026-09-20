# Goal

Keep the filtered explorer authoritative under overlapping requests, SSE updates, snapshot refreshes, and workspace switches, and lock the full redesign with focused tests.

# Scope

- preserve latest-request-wins for success and error paths
- ensure responses from a previous workspace cannot overwrite the current workspace list
- make `ticket.upsert` and `ticket.delete` filter-aware, or force a silent refetch instead of mutating a filtered list directly
- add a reusable regression matrix that spans storage, HTTP, and browser behavior

# Acceptance criteria

- filtered explorer state survives stale success and stale error races without corrupting list, loading, or error state
- non-matching SSE upserts do not appear in an actively filtered list
- workspace switches during overlapping requests leave the final list consistent with the active workspace
- the regression matrix covers deleted-on-disk drift, duplicate workspace names, unresolved search hits, and filtered live updates

# Required tests

- Playwright: stale success after fresh error does not overwrite the latest filtered state
- Playwright: stale error after fresh success does not overwrite the latest filtered state
- Playwright: SSE upsert for a non-matching ticket does not appear under an active state/query filter
- Playwright: switching workspaces during overlapping requests preserves the final workspace list
- focused unit/integration coverage is added near the owning storage and HTTP modules for the shared fixture matrix

# Rigorous validation requirements

- Browser tests must use deterministic delay or event injection so they prove race ordering intentionally rather than passing by luck.
- Assertions must check the final DOM state, selected workspace, and visible ticket ids; waiting on the network alone is not enough.
- Add explicit coverage for stale success after fresh error, stale error after fresh success, and both `ticket.upsert` and `ticket.delete` behavior under an active filter.
- If request coordination or filter matching moves into a helper, add focused unit tests for the helper instead of relying only on end-to-end coverage.
- Required command gate: `viewer-ctl prepare ticket-viewer` followed by focused release Playwright runs in `memory-viewers/ticket-viewer/frontend/dioxus`, plus headed Chromium-family verification whenever workspace-switch or route-visible behavior changes.
