# Dioxus theme settings: Color rows for Backgrounds / Text / Borders / Accents / Log levels / Spans

## Problem

The canonical theme settings spec (§2 rows 2–8) requires color rows for every entry in
`ThemeColors`:

- Backgrounds (5)
- Text & Fonts (3)
- Borders (2)
- Accent Colors (5)
- Log Level Colors (5)
- Log Level Text Colors (5)
- Span Badge Colors (2)

Each row needs: label, optional description, native color picker, hex text input
(validated `^#[0-9a-fA-F]{6}$`), and reset-to-default button.

The current Dioxus port renders a small subset of color tokens (the existing
`__color-token-row` blocks). Bring it to full parity.

## Acceptance criteria

1. All 27 ColorRows above render in the correct sections in the correct order.
2. Picker + hex input + reset all function and persist via `viewer-api-theme` localStorage.
3. Live changes immediately update the corresponding CSS variables on `:root`.
4. Section headers use the canonical icons (▧ A □ ◈ ▤ T →) and wording.

## Implementation notes

- TS reference: `ThemeSettings.tsx` lines 320–380.
- Add a single `ColorRow(label, desc, key)` Dioxus component to avoid repetition.
- `defaultTheme` lookup goes through `ThemeStore::default_theme()`.
