## Objective
Test fs-api, fs-cli, and fs-mcp symlink and TOCTOU behavior.

## Context
Owner decision: “permit symlinks that resolve inside the root; reject escapes.”

## Acceptance criteria
- Cover inside-root success and escape rejection across transports.
- Cover specified per-platform race behavior.

## Out of scope
- Filesystem API redesign.