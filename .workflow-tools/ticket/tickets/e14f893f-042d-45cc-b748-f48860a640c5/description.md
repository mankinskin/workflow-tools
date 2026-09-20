# BUG: ticket-viewer WGPU pipeline panics with `unreachable` when overlay is enabled

## Summary

Surfaced by the e2e test
`ticket-viewer — GPU overlay & theme settings › GPU overlay master toggle defaults ON and toggles off/on without errors`
(see [tools/viewer/e2e/tests/viewers.spec.ts](tools/viewer/e2e/tests/viewers.spec.ts#L233)).

When the master GPU toggle is enabled in the **ticket-viewer** Dioxus
frontend, a wasm trap (`pageerror: unreachable`) fires from the WgpuOverlay
render loop. Symptom is reliable in the e2e harness whenever the overlay
runs while ticket-viewer is on the home view (no `Graph3D` mounted).

The panic was previously masked because the master toggle defaulted to
**OFF**, so `render_frame()` was never invoked. With the new default of
**ON** (commit pending — feat: GPU overlay default ON; fully
GPU-accelerated viewer), the bug is now exposed on first paint.

## Reproduction

1. Start ticket-viewer (`viewer-ctl start ticket-viewer`).
2. Open `http://localhost:3002/` in Chromium with devtools open.
3. Observe the wasm trap in the browser console: `RuntimeError: unreachable executed`.

Or run:

```bash
cd tools/viewer/e2e
npx playwright test --grep "ticket-viewer.*defaults ON and toggles" --reporter=list
```

The test fails with:

```
expect(jsErrors).toEqual([])
+ Array [ "pageerror: unreachable" ]
```

## Hypothesis

`render_frame()` in
[tools/viewer/viewer-api/frontend/dioxus/src/effects/wgpu_overlay.rs](tools/viewer/viewer-api/frontend/dioxus/src/effects/wgpu_overlay.rs#L420)
likely hits an arithmetic, array, or `unwrap` trap on the very first
frame after `init_gpu()` finishes — probably one of:

- A buffer/uniform offset alignment mismatch.
- A canvas `client_width()`/`client_height()` of 0 producing a 0-size texture.
- A pipeline state assumption that holds in `log-viewer`/`doc-viewer` but
  not in the ticket-viewer DOM layout.

`spec-viewer` runs the same overlay code without panicking.

## Workaround

Disable the master toggle in ThemeSettings (persists to
`viewer-api-gpu-enabled=false` localStorage).

## Action

- Build a `--debug` wasm bundle with `console_error_panic_hook` enabled
  (the release build strips the panic message — only `unreachable`
  remains).
- Reproduce with devtools open and capture the panic site.
- Fix the underlying trap in `render_frame` / `init_gpu`.

Until fixed, the e2e test serves as a regression marker.
