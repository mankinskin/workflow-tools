# Dioxus theme settings: Theme Presets grid + header actions

## Problem

The canonical theme settings spec (§2 row 1, default open) requires a Theme Presets grid
of `.theme-preset-card`s, each showing 5 swatches + name + description, and a header
action bar with Reset / Randomize / Save / Export / Import buttons. The Dioxus port
currently has only a basic preset selector and no header action row.

## Acceptance criteria

1. Header (`.theme-settings-header`) renders title, subtitle, and the action row
   `.theme-settings-actions` with these buttons in this order:
   - Reset to Default (`btn btn-primary`)
   - 🎲 Randomize (`btn btn-primary`) — shown only when `store.randomizeTheme` exists
   - 💾 Save Theme (`btn btn-primary`) — opens inline name input + Save / ✕
   - 📤 Export (`btn btn-secondary`) — downloads `<name>.json`
   - 📂 Import (`btn btn-secondary`) — opens hidden file picker; errors render inline.
2. Theme Presets section is default open and renders a grid of cards. Each card shows
   5 swatches (`bgPrimary`, `accentOrange`, `accentBlue`, `levelError`, `cinderEmber`)
   plus the preset name + description.
3. Clicking a card calls `store.applyPreset(preset)` and updates the active theme.
4. CSS classes match `.theme-presets-grid`, `.theme-preset-card`, `.theme-preset-swatches`,
   `.theme-preset-swatch`, `.theme-preset-info` (already in `viewer-api.css`).

## Implementation notes

- TS reference: `ThemeSettings.tsx` lines 290–320 (header + preset grid).
- `ThemeStore` needs to expose `presets: Vec<ThemePreset>` (port from `theme.ts`).
- Save/Export/Import buttons: this ticket only covers the *UI buttons*; the underlying
  save/export/import logic is tracked by ticket `17358907…`.
