# Objective

Track the next graph-viewer interaction and presentation upgrade for ticket-viewer: property-based node rendering, stronger selection semantics, panel-aware framing, an optional fixed 2D camera mode, and presentation keyframing.

# Scope

- refine selection focus semantics, selected-state effects, and deselection behavior
- replace the current card-first rendering assumption with property-based level-of-detail rendering
- keep graph projections behind UI panels while adapting framing and layout to panel-occupied screen space
- add an optional fixed 2D graph camera plus matching 2D grid styling
- support temporary selection-driven presentation layouts through keyframed transitions, with configurable auto-trigger behavior
- strengthen release Playwright and browser validation planning for graph node tiers, effects, and presentation transitions

# Non-goals

- replacing the entire viewer shell or generic tiling panel system
- removing the existing 3D mode entirely
- broad unrelated explorer or ticket-document work
