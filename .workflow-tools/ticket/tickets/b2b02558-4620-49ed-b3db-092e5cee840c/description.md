Render the viewer-api `Breadcrumbs` (P1) above the spec detail tabs.

## Scope

- Compute `Vec<BreadcrumbItem>` from the active tab's spec id: `category › name › section?`.
- Each non-leaf segment is clickable: navigates the spec list scoped to that category (or the parent spec).
- The leaf segment is rendered as plain text per `Breadcrumbs` API.

## Acceptance

- `cargo check -p spec-viewer-dioxus --target wasm32-unknown-unknown` passes.
- Browser smoke: open a spec, see `Category › Name`, click the category to filter the list.
- Tabs ↔ breadcrumbs stay in sync when the user switches tabs.
