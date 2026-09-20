Parent epic: `0ee95228`. Spec: `2ccde9ee`. This is the system's real-use validation.

## Scope

Produce the first real deck: an introduction and overview of the workflow-tools suite
(ticket-api, spec-api, rule-api, test-api, doc-api, log-api, feedback-api, audit-api,
the transport-harness contract, and the managed viewers).

- Authored end-to-end through the Presentation agent mode using the vendored skills.
- Stored as a deck entity in `.presentation/` (`deck.toml` + `slides.md`).
- Uses charts, at least one Mermaid architecture diagram, and at least one live embedded
  graph slide (WASM).
- `[trace]` links to the specs and tickets the deck presents.
- Served by `presentation-viewer` at `/deck/{id}`.

## Definition of done

- Deck builds as a static SPA.
- Playwright E2E passes with a screenshot per slide.
- Manual verification in an external fullscreen Chromium browser; record the window resolution.
- Deck state set to `published`, trace validation clean (no dangling refs).
- Retrospective note on token cost: confirm the authoring pass was achievable by a mid-tier
  model; if not, file follow-up against the skills ticket.
