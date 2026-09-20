# Plan: Share Themes and Effects from log-viewer to ticket-viewer

**Ticket:** 4f399974-0ad6-4da6-848a-954cccea4943
**Component:** viewer-api (shared), log-viewer (source), ticket-viewer (consumer)
**Risk:** Medium
**Depends on:** 1789cdfa (GPU pipeline extraction)

---

## Goal

Make the theme/effects system (currently only in log-viewer) available in ticket-viewer, so ticket-viewer gets themed presets, particle effects, CRT overlays, smoke, and other visual effects powered by the shared WgpuOverlay pipeline.

## Current State

### What's already shared (viewer-api)

`viewer-api/frontend/src/store/theme.ts` exports:
- `ThemeColors` interface (backgrounds, text, borders, accents, log levels, particles, cinders, smoke)
- `ThemePreset` interface (`{ name, description, colors }`)
- `createThemeStore()` factory → returns `ThemeStore` with signals + localStorage persistence
- Color utility functions (`hexToVec3`, `vec3ToHex`, `hexLuminance`, `hexToRgba`, `brightenHex`, `saturateHex`)
- `DEFAULT_THEME` (light "Arcadia")

### What's log-viewer-only

`log-viewer/frontend/src/store/theme.ts` adds:
- `EffectSettings` interface (30+ tuneable params: sparks, embers, beams, glitter, cinder, smoke, CRT, grain, vignette, underglow)
- `DEFAULT_EFFECT_SETTINGS` (balanced defaults)
- `LogViewerPreset` type extending `ThemePreset` with optional `effects`
- 9 themed presets (Cinder, Frost, Blood Moon, Verdant, Void, Amber Terminal, Ocean Abyss, Elysium, Sakura) — each with tuned color palettes AND effect profiles
- `effectSettings` signal + localStorage persistence
- Preset-to-effect mapping on theme switch

### What log-viewer components consume effects

- `WgpuOverlay/gpu-buffers.ts`: Reads `effectSettings` signal to update GPU uniform buffer each frame (CRT params, smoke intensity, particle speeds, etc.)
- `WgpuOverlay/gpu-render-loop.ts`: Uses effect toggles (sparks/embers/beams/glitter enabled) to decide what to render
- `effects/palette.ts`: `buildPaletteBuffer(colors)` converts `ThemeColors` particle/cinder/smoke fields to GPU float array
- `ThemeSettings` component: UI for choosing presets and tuning individual effect params

## Design

### Step 1: Introduce ColorTheme and EffectTheme as separate types

Move to `viewer-api/frontend/src/store/theme.ts`:

```typescript
/** Color-only theme — backgrounds, text, borders, accents, particles. */
export interface ColorTheme {
  name: string;
  description: string;
  colors: ThemeColors;
}

/** Effect-only theme — particle, smoke, CRT, grain, vignette settings. */
export interface EffectTheme {
  name: string;
  description: string;
  effects: EffectSettings;
}

export interface EffectSettings {
  sparksEnabled: boolean; sparkCount: number; sparkSize: number; sparkSpeed: number;
  embersEnabled: boolean; emberCount: number; emberSize: number; emberSpeed: number;
  beamsEnabled: boolean; beamCount: number; beamHeight: number; beamSpeed: number; beamDrift: number;
  glitterEnabled: boolean; glitterCount: number; glitterSize: number; glitterSpeed: number;
  cinderEnabled: boolean; cinderSize: number;
  smokeEnabled: boolean; smokeIntensity: number; smokeSpeed: number;
  crtEnabled: boolean; crtScanlinesH: number; crtScanlinesV: number; crtEdgeShadow: number; crtFlicker: number;
  grainIntensity: number;
  vignetteStrength: number;
  underglowStrength: number;
}

export const DEFAULT_EFFECT_SETTINGS: EffectSettings = { /* all disabled/zero */ };

/** Combined preset — one color theme + optional effect theme. */
export interface ThemePreset {
  colorTheme: ColorTheme;
  effectTheme?: EffectTheme;
}
```

This separation allows viewers to mix-and-match: a viewer can offer color-only presets, effect-only presets, or combined presets.

Also move `buildPaletteBuffer()` and palette WGSL to viewer-api (done in GPU extraction ticket).

### Step 2: Shared ThemeSettings component in viewer-api

The `ThemeSettings` component is extracted to viewer-api as a **configurable** shared component. Each viewer passes its own preset list:

```typescript
// viewer-api/frontend/src/components/ThemeSettings/ThemeSettings.tsx
export interface ThemeSettingsProps {
  colorThemes: ColorTheme[];
  effectThemes?: EffectTheme[];
  currentColorTheme: Signal<string>;   // name of active color theme
  currentEffectTheme?: Signal<string>; // name of active effect theme
  colors: Signal<ThemeColors>;
  effectSettings?: Signal<EffectSettings>;
  onApplyColorTheme: (theme: ColorTheme) => void;
  onApplyEffectTheme?: (theme: EffectTheme) => void;
  onUpdateColor?: (key: keyof ThemeColors, value: string) => void;
  onUpdateEffect?: <K extends keyof EffectSettings>(key: K, value: EffectSettings[K]) => void;
}
```

The component renders:
- Color preset grid (with swatch previews)
- Effect preset grid (if effectThemes provided)
- Collapsible color editor sections
- Collapsible effect toggle/slider sections (if effectSettings provided)

### Step 3: Create ticket-viewer's own palette

Ticket-viewer defines its own color and effect themes. **Aesthetic direction (A10):** Light, clean workspace themes — paper-like, document-oriented, with signal colors for ticket states.

```typescript
// ticket-viewer/frontend/src/store/theme.ts

const TICKET_COLOR_THEMES: ColorTheme[] = [
  {
    name: 'Default',
    description: 'Clean light workspace',
    colors: { /* light neutral — white bg, soft greys, blue accent */ },
  },
  {
    name: 'Paper',
    description: 'Off-white paper tones with warm ink',
    colors: { /* #f8f5f0 bg, #2c2a26 text, sepia borders */ },
  },
  {
    name: 'Scratchboard',
    description: 'Dark board with chalk-white text',
    colors: { /* very dark bg #1a1a1a, off-white text, rough chalk accent */ },
  },
  {
    name: 'Office',
    description: 'Corporate neutral, easy on the eyes',
    colors: { /* white bg, cool grey borders, #0078d4 accent */ },
  },
  {
    name: 'Calendar',
    description: 'Bright dates, crisp lines',
    colors: { /* clean white, #e74c3c red accent, #3498db blue for events */ },
  },
  {
    name: 'Notepad',
    description: 'Ruled-line warmth, pencil greys',
    colors: { /* light yellow bg #fffde7, #424242 text, ruled-line border */ },
  },
  {
    name: 'Signal',
    description: 'High-contrast state colors for triage',
    colors: { /* dark bg, ticket state colors as primary accents: open=blue, in-progress=amber, done=green, blocked=red */ },
  },
];
```

Each `ColorTheme` should define meaningful values for all `ThemeColors` fields, including particle/cinder/smoke colors for when effects are enabled.

Effect themes cover the full range of GPU capabilities (A11 — effects can be enabled everywhere):

```typescript
const TICKET_EFFECT_THEMES: EffectTheme[] = [
  {
    name: 'None',
    description: 'No visual effects (default)',
    effects: DEFAULT_EFFECT_SETTINGS,  // all disabled
  },
  {
    name: 'Subtle',
    description: 'Gentle glitter and soft vignette',
    effects: { glitterEnabled: true, glitterCount: 30, glitterSize: 80, vignetteStrength: 0.15, /* rest off */ },
  },
  {
    name: 'Office Haze',
    description: 'Soft smoke and grain, no particles',
    effects: { smokeEnabled: true, smokeIntensity: 0.3, smokeSpeed: 0.2, grainIntensity: 0.08, /* rest off */ },
  },
  {
    name: 'Forge',
    description: 'Dark industrial — sparks, embers, cinder, smoke',
    effects: { sparksEnabled: true, embersEnabled: true, cinderEnabled: true, smokeEnabled: true, underglowStrength: 0.3 },
  },
  {
    name: 'Terminal',
    description: 'CRT scanlines, grain, phosphor glow',
    effects: { crtEnabled: true, crtScanlinesH: 1.2, crtFlicker: 0.03, grainIntensity: 0.12, vignetteStrength: 0.25 },
  },
];
```

**Effects are OFF by default** in ticket-viewer. Users opt-in by selecting an effect theme. Future effect themes can be added freely — full GPU/shader capability is available.

### Step 4: Wire effects in ticket-viewer

1. Import `createThemeStore` from viewer-api, initialize in ticket-viewer store
2. Add `effectSettings` signal (persisted to localStorage, defaults to `DEFAULT_EFFECT_SETTINGS` — all off)
3. WgpuOverlay reads `effectSettings` via `useOverlayContext()` to drive its render loop — CPU/GPU wiring flows through the shared context
4. Import **full `ThemeSettings` component** from viewer-api (A9 — same feature set as log-viewer: presets + color editor + effect sliders + save/export/import + thumbnail capture)
5. Pass ticket-viewer's `TICKET_COLOR_THEMES` and `TICKET_EFFECT_THEMES` as props to `ThemeSettings`

**Effects scope (A11):** Effects are **globally available across the entire ticket-viewer UI** — not just the 3D dependency graph. The element scanner (via `TICKET_VIEWER_SCHEMA`) detects DOM elements throughout the sidebar, ticket tree, content pane, and any state-colored nodes. Sparks, smoke, CRT, glow effects can apply everywhere. This uses the full GPU and shader capabilities, with more effects to be added in the future.

### Step 5: Ticket-viewer element selectors for effects

The WgpuOverlay element scanner needs CSS selectors for ticket-viewer DOM elements to apply border glows, hover effects, etc.:

```typescript
const TICKET_ELEMENT_SELECTORS: ElementSelectorConfig[] = [
  { selector: '.header', kind: KIND_STRUCTURAL, hue: 0.58 },
  { selector: '.sidebar', kind: KIND_STRUCTURAL, hue: 0.55 },
  { selector: '.ticket-content', kind: KIND_STRUCTURAL, hue: 0.52 },
  { selector: '.ticket-tree .tree-item-row.selected', kind: KIND_SELECTED, hue: 0.12 },
  { selector: '.ticket-tree .tree-item-row:hover', kind: KIND_STRUCTURAL, hue: 0.60 },
  // State-colored ticket nodes in 3D graph
  { selector: '[data-ticket-state="open"]', kind: KIND_INFO, hue: 0.60 },
  { selector: '[data-ticket-state="in-progress"]', kind: KIND_WARN, hue: 0.10 },
  { selector: '[data-ticket-state="done"]', kind: KIND_DEBUG, hue: 0.35 },
  { selector: '[data-ticket-state="blocked"]', kind: KIND_ERROR, hue: 0.0 },
];
```

### Step 6: ThemeSettings UI

Either:
- (a) Extract the log-viewer `ThemeSettings` component to viewer-api as a shared component with preset list + effect sliders, OR
- (b) Create a simpler version in ticket-viewer with just preset picker + a few toggles

**RESOLVED:** Extract `ThemeSettings` to viewer-api as a shared component. Color and effect themes are separate sections. Each viewer passes its own preset lists via props.

## Implementation Steps

1. Define `ColorTheme`, `EffectTheme`, `EffectSettings`, `ThemePreset` types in viewer-api theme store.
2. Export `DEFAULT_EFFECT_SETTINGS` (all off) from viewer-api.
3. Extract `ThemeSettings` component to viewer-api with `ThemeSettingsProps` accepting color/effect theme lists.
4. Update log-viewer to import shared types + use shared ThemeSettings component, passing its 9 presets as props.
5. Add `effectSettings` signal (defaults all-off) to ticket-viewer store.
6. Define ticket-viewer's own `TICKET_COLOR_THEMES` and `TICKET_EFFECT_THEMES`.
7. Pass ticket-specific `elementSelectors` to `WgpuOverlay` in ticket-viewer `App.tsx`.
8. Import shared `ThemeSettings` in ticket-viewer, pass ticket-viewer presets.
9. Wire CSS variable injection from `ColorTheme.colors` in ticket-viewer.
10. Verify: both viewers typecheck, effects render in ticket-viewer, theme switching works.

## Files Changed

| File | Change |
|------|--------|
| `tools/viewer-api/frontend/src/store/theme.ts` | Add ColorTheme, EffectTheme, EffectSettings, ThemePreset types |
| `tools/viewer-api/frontend/src/index.ts` | Re-export new types |
| `tools/log-viewer/frontend/src/store/theme.ts` | Import EffectSettings from viewer-api instead of local definition |
| `tools/ticket-viewer/frontend/src/store.ts` | Add effectSettings signal, theme presets |
| `tools/ticket-viewer/frontend/src/App.tsx` | Add WgpuOverlay + ThemeSettings |
| `tools/ticket-viewer/frontend/src/styles.css` | Add data-ticket-state classes, GPU-active styles |

## Resolved Questions

1. **RESOLVED:** Ticket-viewer gets its own preset palette. Log-viewer's presets are NOT shared.
2. **RESOLVED:** ThemeSettings is a shared component in viewer-api. Each viewer provides its own preset lists via props.
3. **RESOLVED:** Effects are OFF by default in ticket-viewer. Users opt-in by choosing an effect theme.
4. **RESOLVED:** Color and effect themes are cleanly separated into `ColorTheme` and `EffectTheme` types. `ThemePreset` composes both.
5. **RESOLVED (A9):** Full ThemeSettings component — same feature set as log-viewer (presets + color editor + all effect sliders + save/export/import + thumbnail capture).
6. **RESOLVED (A10):** Ticket-viewer color palette: Default, Paper, Scratchboard, Office, Calendar, Notepad, Signal. Document/workspace aesthetic with light themes dominant.
7. **RESOLVED (A11):** Effects apply to the **entire ticket-viewer UI** (not just the 3D graph). Element scanner covers sidebar, ticket tree, content pane, state nodes. Full GPU/shader capability. More effects to be added in future.
