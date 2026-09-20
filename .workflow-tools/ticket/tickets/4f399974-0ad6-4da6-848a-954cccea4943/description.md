Make the theme and effects system available in ticket-viewer so it can use themed presets, particle effects, CRT overlays, smoke, and other visual effects from the shared GPU pipeline.

## Design Decisions

- **ColorTheme / EffectTheme separation:** Two distinct types in viewer-api. `ColorTheme` covers backgrounds, text, borders, accents, particle colors. `EffectTheme` covers spark/ember/beam/glitter/cinder/smoke/CRT/grain/vignette settings. `ThemePreset` composes both.
- **Full ThemeSettings component (A9):** Shared from viewer-api with the complete feature set: preset grid, per-color editors, per-effect sliders, save/export/import, thumbnail capture. Not a stripped-down version.
- **Ticket-viewer palette (A10):** 7 own color themes — Default, Paper, Scratchboard, Office, Calendar, Notepad, Signal. Document/workspace aesthetic, mostly light themes.
- **Effect themes:** None (default), Subtle, Office Haze, Forge, Terminal. Future themes can always be added.
- **Effects scope (A11):** Effects apply to the **entire ticket-viewer UI** — sidebar, ticket tree, content pane, state-colored nodes. The element scanner (via `TICKET_VIEWER_SCHEMA`) covers the full DOM. Effects are OFF by default; users opt-in. Full GPU/shader capability used.

## Changes Required

1. **viewer-api:** Add `EffectSettings` type + `DEFAULT_EFFECT_SETTINGS` (all disabled). Add `ColorTheme`, `EffectTheme` interfaces. Extend `ThemePreset` to compose both. Move `ThemeSettings` UI component from log-viewer, parameterized by preset list.
2. **log-viewer:** Import `EffectSettings`, `ThemeSettings` from viewer-api instead of local definitions. Keep its 9 presets defined locally.
3. **ticket-viewer:** Create ticket-viewer-specific presets (own palette). Wire `ThemeSettings` into the sidebar. Initialize `effectSettings` signal with all effects disabled.

## Depends On

1789cdfa — The GPU pipeline (WgpuOverlay) must be in viewer-api for effects to render at all.
