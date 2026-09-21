# spec-api: list canonical component entities

## Problem

`SpecManifest` already carries a `component` field, and the spec HTTP layer can filter specs by component, but there is no first-class way to list the canonical component set from `spec-api`.

That leaves downstream tooling guessing. The current ticket store still contains tickets with `component=log-viewer-leptos`, even though that component has been removed. Without a canonical component inventory from specs, stale or invented component values are easy to insert and hard to audit.

## Goal

Expose a machine-readable component listing from `spec-api` so clients can enumerate the canonical component set instead of scraping it indirectly from individual specs.

## Notes

The preferred source is component-scope spec entities when they exist. If the current data model still relies on per-spec `component` strings, the ticket should define the fallback aggregation rules explicitly so callers do not have to reverse-engineer them.

## Acceptance Criteria

- `spec-api` exposes a component listing surface that returns the canonical component set.
- The output is stable and machine-readable, suitable for CLI, HTTP, MCP, or viewer consumers.
- The listing defines where each component came from, so component-scope entities and derived strings are distinguishable if both exist.
- Duplicate or inconsistent component naming is normalized or reported clearly.
- The resulting surface is sufficient to identify removed components such as `log-viewer-leptos` without manual repo-wide searches.
