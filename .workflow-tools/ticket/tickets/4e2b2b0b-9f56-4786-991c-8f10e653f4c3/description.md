## Problem

Visual inspection of the running ticket-viewer (http://localhost:3002) revealed four classes of UI defects, ranging from a CSS variable bug to missing platform-level infrastructure. This epic groups the related sub-tickets so they can be sequenced.

Computed style snapshot (PAPER theme active):

| selector | bg | text |
|---|---|---|
| `.header` | `rgba(20,20,24,0.75)` | `#2c2a26` |
| `.sidebar` | `rgba(20,20,24,0.55)` | `#2c2a26` |
| `.sidebar-header` (uses `--bg-tertiary`) | `#f2efe8` | `#2c2a26` |
| `TicketDetail` (right panel — inline) | `rgba(15,15,28,0.93)` | `#a0a0c8` |

The dark `--panel-bg` is hardcoded independent of the active theme palette, so light themes (PAPER, SCRATCHBOARD) still render dark translucent panels with dark text → unreadable. The right-side `TicketDetail` is built from inline hardcoded hex colors and ignores theme variables. Action buttons in the header use bespoke inline styles that drift from the shared `.btn` system.

## Sub-tickets

1. Bug: theme variable inconsistency — `--panel-bg` independent of theme palette.
2. Bug: TicketDetail hardcoded colors — port inline styles to CSS variables.
3. UI: transparent context-adaptive header & sidebar action buttons.
4. Refactor: extract `viewer-theme` and `viewer-widgets` crates from `viewer-api-dioxus`.
5. Feature: tiling + tabbed panel system — replace flat `Sidebar`/`Panel` primitives with a draggable tile tree.

## Acceptance criteria

- All five sub-tickets created with implementation-ready scope.
- Sub-tickets 1, 2, 3 marked `ready` (quick fixes landing this iteration).
- Sub-ticket 4 marked `new` with research notes.
- Sub-ticket 5 marked `new` and explicitly deferred.

## Notes

User confirmed scope at refinement time:
- Implement quick fixes for tickets 1–3 immediately.
- Crate extraction (#4) → tickets only (broad refactor).
- Tiling panel system (#5) → defer; build from scratch when picked up.
