# Problem

Current suites include useful screenshots, keyboard checks, and isolated responsive assertions, but there is no shared accessibility scanner or maintained visual baseline. Screenshot attachments alone cannot detect regressions.

# Scope

- Add shared Axe-based accessibility checks for representative routes and transient surfaces.
- Define keyboard-only and deterministic-focus contracts for navigation, drawers, dialogs, graph controls, and async route/data transitions.
- Define desktop, tablet, and narrow-mobile viewport projects or a bounded equivalent matrix.
- Add maintained `toHaveScreenshot` baselines for stable shared shell states and selected viewer-specific critical states.
- Cover loading, empty, success, recoverable error, and overlay-open states.
- Establish deterministic data, fonts, animation disabling, color profile, and screenshot masking rules to control flake.
- Keep canvas/WebGPU validation separate: DOM accessibility plus targeted pixel/canvas assertions and screenshots.

# Acceptance criteria

- [ ] Ticket-viewer and spec-viewer representative routes pass the agreed Axe severity threshold with documented exceptions.
- [ ] Shared shell keyboard/focus tests cover modal open/close, drawer navigation, route changes, and graph toolbar controls.
- [ ] Responsive checks run at declared desktop, tablet, and mobile sizes without text overlap or inaccessible controls.
- [ ] Visual baselines are assertions, not report-only attachments, and have a documented review/update workflow.
- [ ] Loading, empty, success, and error states are testable through deterministic fixtures or route interception.
- [ ] WebGPU/canvas checks verify nonblank rendered pixels where applicable and attach screenshots.
- [ ] Flake controls and retry policy are documented; retries do not hide persistent diffs.

# Implementation steps

1. Select stable representative routes/states and add deterministic fixtures.
2. Add shared Axe and focus helpers.
3. Add bounded viewport projects.
4. Establish initial visual baselines and canvas checks.
5. Run in release-browser CI and record evidence.