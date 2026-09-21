# spec-api <-> ticket-api: link component entities and detect component drift

## Problem

Tickets still exist with stale component values such as `log-viewer-leptos`, even though that component has been removed. `ticket-api` currently treats `fields.component` as a free-form string, so outdated or newly invented component names can slip into the ticket store without any canonical validation.

At the same time, `spec-api` already carries component information and is the obvious source of truth for the component catalog. The two systems are not linked, so there is no automated detection of mismatches, insertions, or removed components.

## Goal

Link canonical component entities from `spec-api` to the component values used in `ticket-api`, then surface drift when tickets reference missing, stale, or unknown components.

## Scope

This follow-up should cover both validation and reporting. It is not enough to create a loose mapping table that nobody uses.

## Acceptance Criteria

- There is a defined mapping between canonical component entities from `spec-api` and ticket `fields.component` values in `ticket-api`.
- A validation or health-check path reports tickets whose component is missing, stale, unknown, or no longer present in the canonical component set.
- Newly inserted component values outside the canonical set are detected instead of silently accepted forever.
- The drift report includes enough context to identify concrete stale tickets, including current `log-viewer-leptos` examples.
- The implementation defines how mismatches are handled: report-only, auto-suggested rewrite, or explicit migration workflow.
