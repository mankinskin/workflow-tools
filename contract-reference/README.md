# contract-reference

Minimal reference implementation of the workflow-tools domain-crate contract.
Each piece demonstrates the smallest viable shape for a given transport or
integration surface; none of them carry domain logic beyond `example-api`'s
single `domain_name()` function.

## Crates

- [crates/example-api](crates/example-api) — the domain-neutral library. Owns
  the one piece of "business logic" the reference exposes.
- [crates/example](crates/example) — re-exports `example-api` and hosts the
  `cli`/`mcp`/`http` transport binaries via `transport-harness` (see
  [memory-kernel](../memory-kernel/README.md)).
- [crates/example-viewer](crates/example-viewer) — the smallest viable viewer:
  a single binary built on `viewer-api`'s `run_server` helper, with no
  dedicated frontend build step.

## example-vscode

[example-vscode](example-vscode) is the smallest viable VS Code extension
shape: it activates on startup and registers one command
(`example-viewer.openBrowser`) that opens the `example-viewer` HTTP server in
a browser. Compare with `workflow-tools/ticket/ticket-vscode` for the full
production shape (WASM core, tree views, webviews, bundling).

```bash
cd example-vscode
npm install
npm run compile
```
