# Problem

Research against the current Dioxus ticket-viewer shows the explorer still has three concrete gaps:

1. Combined search + state filtering is incorrect. `GET /api/tickets` drops `state` whenever `query` is present, so the sidebar explorer cannot reliably narrow search results under an active state filter.
2. State chips are still single-select. The current store persists `state_filter` as a single string and the sidebar header renders one active chip at a time.
3. Ticket navigation is still mouse-first. The sidebar ticket tree and the quick-search overlay do not provide consistent arrow-key focus movement plus Enter activation.

## Evidence

- `memory-viewers/ticket-viewer/frontend/dioxus/src/store.rs` persists `state_filter` as `String`.
- `memory-viewers/ticket-viewer/frontend/dioxus/src/routes/list/page.rs` sends only one `state` value to `list_tickets(...)`.
- `memory-viewers/ticket-viewer/frontend/dioxus/src/components/ticket_tree/header.rs` compares chips against a single active value.
- `memory-api/tools/http/ticket-http/src/serve/handlers/tickets/read.rs` takes the `query` branch and ignores `params.state` entirely.
- `memory-viewers/ticket-viewer/frontend/dioxus/src/components/search/results.rs` supports hover/click only; `Enter` does not activate a keyboard-focused result because no keyboard focus model exists.

## Scope

This parent tracks the concrete explorer hardening work only:

- fix combined query + state filtering
- support multi-select state chips in the explorer
- add keyboard navigation for the sidebar tree and quick-search result list

Viewer-wide keyboard support beyond those local flows is tracked separately.

## Deliverables

1. The ticket list API returns correct results for combined query and state filters.
2. The sidebar explorer supports selecting multiple states at once with clear active/reset behavior.
3. Users can move through tickets with Up/Down and open the focused ticket with Enter in the sidebar tree and quick-search results.
4. Focus behavior remains visible and predictable after filtering, refresh, and search-result changes.
5. Regression coverage exists at the API layer and the browser UX layer.

## Execution Notes

- Treat the filter bug as the root-cause fix before or alongside the multi-state work.
- Keep the broader shortcut system out of this ticket so implementation stays reviewable.
- Browser verification and Playwright coverage are required because this changes a browser-facing viewer flow.

## Coordinated execution plan

This ticket is the Track C coordination point in the broader ticket-viewer shell plan.

1. Track A covers header actions cleanup in `C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/workflow-tools/.workflow-tools/ticket/tickets/6ea2c97c-0b41-4b90-91db-f0de9e8e4b8e`.
2. Track B covers sidebar/tree parity and long-list scrolling in `C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/memory-viewers/.ticket/tickets/c10cc92e-03b5-423b-a7ef-93879c253f7d`.
3. This explorer track may run in parallel with those shell tracks, but should not absorb their shell-level scope.
4. Shared helper extraction is acceptable only after the shell-level behavior is stable enough that the helper can be validated in one place.
5. Final integration should rerun the shell and explorer validation slices together before review.

## Rigorous test design required before implementation

1. Before any implementation begins on the remaining explorer sub-work, each concrete child slice must record a pre-implementation test design.
2. The test design MUST separate API-layer checks from browser-UX checks so filter semantics and keyboard semantics can fail independently.
3. The test design MUST identify the existing unrelated failing tests, if any, and classify them as blockers or background noise before new code lands.
4. Multi-state filtering work MUST declare its exact request format and expected backend query semantics before implementation.
5. Keyboard-navigation work MUST declare the expected focus model, active-row visibility rules, and Enter-activation behavior before implementation.
6. Browser-facing explorer changes MUST include at least one focused release Playwright test and one external headed Chromium-family browser verification step.
7. The relevant child ticket should not move to `in-implementation` until that test design has been recorded in the ticket or a linked asset.