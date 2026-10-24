# Direct Mission authoring and recording repair

## Objective

Implement the bounded direct CLI/MCP Mission operations specified by [Direct Mission Authoring and Recording](../../spec/specs/0e02e89c-aff1-47c8-a113-3c6b36688add/body.md), before any installation-pilot Mission is created.

## Boundaries

Do not author/import external native JSON bundles as a substitute. Preserve native-bundle compatibility, immutable accepted revisions, expected-current-revision checks, and derived Ticket/child-Mission state. Do not add scheduling, command execution, list/search, broad CRUD, activation, or pilot creation.
