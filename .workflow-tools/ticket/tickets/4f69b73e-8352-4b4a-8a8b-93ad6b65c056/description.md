Adopt the viewer-api `PathCodec`/`expand_path_to` (P2) so the active spec id roundtrips through the URL.

## Scope

- URL format: `#/spec/<category>/<name>` decoded via `PathCodec` (use the `ColonSegmented` codec or define a thin spec-id codec on top).
- Navigating via URL opens (or focuses) the corresponding tab — must integrate with P5.1.
- `expand_path_to` expands the spec tree to reveal the active spec on URL load.
- Updating tabs (open / focus / close-last) writes back the URL.

## Acceptance

- `cargo check -p spec-viewer-dioxus --target wasm32-unknown-unknown` passes.
- Browser smoke: paste a `#/spec/<category>/<name>` URL into a fresh tab — the tree expands and the matching spec opens in a tab.
- Browser/forward navigation in the browser history switches the active tab.

## Depends on

- P5.1 (TabsStore) — needed to focus via URL.
