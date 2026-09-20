## Problem

`tools/viewer/ticket-viewer/frontend/dioxus/src/components/ticket_detail.rs` builds the right-side ticket detail panel from inline hardcoded hex colors. When the theme is set to PAPER (light) the panel still renders with `background: rgba(15, 15, 28, 0.93)` and text `#a0a0c8` — making it visually disconnected from the rest of the UI and ignoring the active theme.

Inventory of hardcoded colors in `ticket_detail.rs` (line numbers approximate):

| line | usage |
|---|---|
| 94, 110, 198, 224, 813 | `#1e1e2d`, `#13131f` panel surfaces |
| 123, 132, 242, 255 | `#3c3c5a`, `#ef4444`, `#3b82f6` button bgs |
| 739–740 | right-panel container `rgba(15,15,28,0.93)` + border `#2a2a45` |
| 754, 819, 851, 866 | `rgba(239,68,68,0.15)`, `rgba(99,102,241,0.15)` accents |

## Acceptance criteria

1. Every hardcoded color in `ticket_detail.rs` is replaced with a CSS variable (`var(--bg-secondary)`, `var(--text-primary)`, `var(--accent-blue)`, `var(--accent-red)`, …) or moved into a CSS class in a new file `tools/viewer/ticket-viewer/frontend/dioxus/public/ticket-detail.css`.
2. The right-panel container uses `Panel { placement: Right }` from `viewer-api-dioxus::components::layout` instead of an inline `<div style="width: 300px;…">`.
3. Conflict dialog reuses the shared `Modal` component (`viewer-api-dioxus::components::modal`) instead of inline overlays.
4. Manual screenshot verification under PAPER, SCRATCHBOARD, ARCADIA, DARK themes — all four screenshots attached to the PR.
5. No regressions in `tools/viewer/ticket-viewer/frontend/dioxus/e2e/` Playwright tests.

## Implementation notes

- Move repeated row layout to a `DetailRow` component to remove the wall of inline styles.
- The conflict dialog (`ConflictDialog`) is the only piece that intentionally uses high-emphasis colors (warning amber); it can keep semantic overrides through `var(--warning)` / `var(--error)`.

## Parent

Epic `4e2b2b0b-9f56-4786-991c-8f10e653f4c3`. Depends on `f00204fc-f33f-4cd6-9b5f-395071f4e118` (theme variables must be theme-aware first).
