Parent epic: `0ee95228`. Spec: `2ccde9ee`. Depends on `89b0c64a` (toolchain with wasm plugins).

## Extraction source (decided)

Extract from **viewer-api's shared graph component** — the one consumed by ticket-viewer,
spec-viewer, and log-viewer — not from any single viewer's copy. Locate it under
`viewer-api/` and confirm it is the shared implementation before extracting.

## Scope

- Extract viewer-api's shared graph component into a standalone, parameterizable crate
  that compiles to WASM without a viewer server and without viewer-api's server-side
  data plumbing.
- Define a standalone data contract: the component accepts inline or bundled graph data
  so a slide has **no runtime dependency** on a running viewer.
- Provide a Slidev Vue wrapper component that loads the WASM module and accepts slide
  props, e.g. `<GraphSlide :data="..." :mode="2d" />`.
- Load heavy WASM in a Web Worker if it blocks slide transitions; slide navigation must
  stay smooth.
- Verify `slidev build` bundles the `.wasm` into `dist/assets/` and the static SPA works
  from a plain file host with no backend.
- Do not regress the existing viewers: the extraction must leave ticket-viewer,
  spec-viewer, and log-viewer building and passing their shared Playwright suites.

## Definition of done

- A sample slide renders a live graph from bundled data, fully offline.
- Playwright E2E asserts the graph renders and captures a screenshot.
- Existing viewer suites under `viewer-api/viewer-api/frontend/dioxus/e2e/shared/` still pass.
