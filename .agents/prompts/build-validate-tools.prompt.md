---
agent: agent
description: "Build all tools from source and validate each one in the CLI or browser. Run after cross-cutting changes (new dep, API rename, toolchain upgrade, infra refactor)."
---

# Build and Validate All Tools

Use this workflow when you need to confirm that every deployable binary and
browser application compiles and responds correctly after a workspace-wide
change.

## Prerequisites

Install the lifecycle manager if not already present:

```bash
cargo make install-install-ctl
```

## Step 1 — Build everything

```bash
cargo make build-all
```

`build-all` runs two sub-targets in order:

| Sub-target | What it builds |
|---|---|
| `build-native-tools` | All Rust binaries: CLI tools, HTTP servers, MCP servers, viewer servers, install-ctl, misc utilities |
| `build-all-frontends` | All viewer frontend bundles: Vite (doc-viewer, log-viewer) and Trunk/WASM (ticket-viewer, spec-viewer) |

> **Partial rebuilds**: run `cargo make build-cli-tools`, `build-http-tools`,
> `build-mcp-tools`, `build-viewers`, or `build-all-frontends` independently.

If `build-all` fails, stop here. Fix the error before proceeding to validation.

## Step 2 — Validate CLI tools

Run a smoke check on each CLI binary. Any non-zero exit code is a failure.

```bash
# ticket CLI (most commonly used — run a real read-only command)
./target/release/ticket list --limit 3 --toon

# context CLI
./target/release/context-cli --help

# spec CLI
./target/release/spec --help

# audit CLI
./target/release/audit --help
```

Expected: each command prints structured JSON or a help message and exits 0.

## Step 3 — Validate HTTP servers (static smoke test)

These servers start up and listen on a port. Do a quick bind-and-exit check:

```bash
# Each server accepts --help or exits quickly when given an unknown flag.
./target/release/ticket-http  --help 2>&1 | head -5 || true
./target/release/context-http --help 2>&1 | head -5 || true
./target/release/spec-http    --help 2>&1 | head -5 || true
```

> For a deeper check, start the server in the background, hit a health
> endpoint, then kill it:
> ```bash
> ./target/release/ticket-http &; sleep 1; curl -s http://localhost:3003/api/health; kill %1
> ```

## Step 4 — Validate viewer apps in the browser

Use `install-ctl` to start each viewer, verify it returns HTTP 200, then stop it.
Run them one at a time (each blocks until killed).

```bash
# doc-viewer  — http://localhost:3001
install-ctl start doc-viewer --no-build &
sleep 2 && curl -sf http://localhost:3001/ -o /dev/null && echo "doc-viewer OK"
install-ctl viewer stop doc-viewer

# log-viewer  — http://localhost:3000
install-ctl start log-viewer --no-build &
sleep 2 && curl -sf http://localhost:3000/ -o /dev/null && echo "log-viewer OK"
install-ctl viewer stop log-viewer

# ticket-viewer — http://localhost:3002
install-ctl start ticket-viewer --no-build &
sleep 2 && curl -sf http://localhost:3002/ -o /dev/null && echo "ticket-viewer OK"
install-ctl viewer stop ticket-viewer

# spec-viewer — http://localhost:4002
install-ctl start spec-viewer --no-build &
sleep 2 && curl -sf http://localhost:4002/ -o /dev/null && echo "spec-viewer OK"
install-ctl viewer stop spec-viewer
```

For a manual in-browser check open each URL while the viewer is running.
The SPA must load without a blank page or console errors.

## Step 5 — Validate MCP servers (JSON-RPC handshake)

MCP servers communicate over stdio. A minimal smoke test sends the
`initialize` message and checks for a valid JSON response:

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"smoke-test","version":"0"}}}' \
  | timeout 3 ./target/release/ticket-mcp 2>/dev/null \
  | python3 -c "import sys,json; d=json.load(sys.stdin); print('ticket-mcp OK, server:', d['result']['serverInfo']['name'])"

echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"smoke-test","version":"0"}}}' \
  | timeout 3 ./target/release/context-mcp 2>/dev/null \
  | python3 -c "import sys,json; d=json.load(sys.stdin); print('context-mcp OK, server:', d['result']['serverInfo']['name'])"

echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"smoke-test","version":"0"}}}' \
  | timeout 3 ./target/release/spec-mcp 2>/dev/null \
  | python3 -c "import sys,json; d=json.load(sys.stdin); print('spec-mcp OK, server:', d['result']['serverInfo']['name'])"

echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"smoke-test","version":"0"}}}' \
  | timeout 3 ./target/release/audit-mcp 2>/dev/null \
  | python3 -c "import sys,json; d=json.load(sys.stdin); print('audit-mcp OK, server:', d['result']['serverInfo']['name'])"
```

## Step 6 — Run Playwright E2E tests in the browser

The Playwright suites validate that each viewer **actually works in Chromium** —
no console errors, no missing assets, and the UI renders after hydration.

Shared managed-viewer suites now live under
`viewer-api/viewer-api/frontend/dioxus/e2e/shared`.
The spec/doc/log entrypoints start their own managed viewer servers through
`install-ctl`; ticket-viewer release coverage remains separate.

```bash
# Install npm deps for spec/doc/log viewer suites (first time only)
cargo make e2e-install

# Run the spec/doc/log viewer suites
cargo make test-e2e

# Run ticket-viewer release coverage separately
cd memory-viewers/ticket-viewer/frontend/dioxus
npm run test:e2e:release
```

Or run manually from the viewer-specific package:

```bash
cd memory-viewers/spec-viewer/frontend/dioxus
npm run test:e2e:release

cd workflow-tools/doc/crates/doc-viewer/e2e
npm test

cd workflow-tools/log/crates/log-viewer/e2e
npm run test:managed
```

The suite tests each viewer for:
- **No console errors** — no `console.error()` or uncaught exceptions after load
- **No missing static assets** — no HTTP 404 for JS/CSS/WASM/font files
- **UI is visible** — the viewer-specific ready selector appears within timeout

> Viewer-specific ready selectors (signals that the app has fully rendered):
> | Viewer | Selector | Timeout |
> |---|---|---|
> | log-viewer | `.tab-bar` | 20 s |
> | doc-viewer | `.app` | 20 s |
> | ticket-viewer | `header.header` | 60 s (WASM) |
> | spec-viewer | `header.header` | 60 s (WASM) |

## Step 7 — Report results

Summarize in a checklist. Mark each tool as ✅ (passed) or ❌ (failed with
the error message).

```
Build
  [x] build-native-tools
  [x] build-all-frontends

CLI tools
  [x] ticket list
  [x] context-cli --help
  [x] spec --help
  [x] audit --help

HTTP servers
  [x] ticket-http
  [x] context-http
  [x] spec-http

Viewer apps (HTTP 200 + browser load)
  [x] doc-viewer   http://localhost:3001
  [x] log-viewer   http://localhost:3000
  [x] ticket-viewer http://localhost:3002
  [x] spec-viewer   http://localhost:4002

MCP servers (initialize handshake)
  [x] ticket-mcp
  [x] context-mcp
  [x] spec-mcp
  [x] audit-mcp

Playwright E2E (no console errors, no missing assets, UI renders)
  [x] log-viewer
  [x] doc-viewer
  [x] ticket-viewer
  [x] spec-viewer
```

Any ❌ item must be fixed before considering the change safe to merge.

## Cargo make equivalents

| Manual command | cargo make shortcut |
|---|---|
| `cargo build --release` (all native) | `cargo make build-native-tools` |
| `install-ctl viewer build <viewer>` (all) | `cargo make build-all-frontends` |
| Both | `cargo make build-all` |
| `install-ctl start <viewer>` | `cargo make start-<viewer>` |
| `install-ctl viewer stop <viewer>` | `cargo make stop-<viewer>` |
| TypeScript type generation | `cargo make gen-types` |
| VS Code extension install | `cargo make install-vscode-ext` |
| `npm install` in e2e dir | `cargo make e2e-install` |
| `npx playwright test` | `cargo make test-e2e` |
