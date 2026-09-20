Parent epic: `0ee95228`. Spec: `2ccde9ee` (R14/AC14). Depends on `1500a9e6` for deck/slide slot shape only — this ticket does NOT depend on `693763fc`'s real extraction data; preset/legend design proceeds from the specified typed node/edge kinds, not live projections.

## Scope

Define the topology visual preset contract required before any flagship structural (Git/Cargo topology) slide ships: a required legend, named node and edge roles matching the typed projections in `693763fc` (git_containment, cargo_membership, cargo_dependency), density limits for readable rendering, and baseline screenshots for regression comparison. This is a preset-descriptor contract (consumed the same way `60222b57`'s presets are), not the topology extraction or rendering implementation itself.

## Definition of done

- Preset descriptor(s) define legend requirements, node/edge role vocabulary, and density thresholds.
- Baseline screenshots exist for at least one synthetic topology fixture.
- AC14 is satisfied: the contract exists before any flagship structural slide is authored.