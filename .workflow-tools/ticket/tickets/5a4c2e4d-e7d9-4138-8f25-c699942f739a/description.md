# Summary

Add a first-class `test-api` for validation specifications and validation results in the memory system.

# Why

The corrected workflow architecture needs native storage and identity for validation plans, executions, and outcomes. A wrapper CLI artifact format is not the intended long-term storage model.

# Scope

- add a `test-api` crate to the memory-system tool stack
- define first-class entities for validation specifications, executions, and outcomes such as `passed`, `failed`, and `blocked`
- support native links to tickets, specs, docs, and logs
- support configuration-driven default workflow capture from existing tool surfaces instead of a separate wrapper tool
- define the minimal CLI/MCP/HTTP follow-up surfaces needed after the API exists

# Acceptance criteria

- A `test-api` crate exists for validation specifications and validation results.
- The API provides native identifiers and metadata for validation plans, executions, and outcomes.
- The API supports first-class links to tickets, specs, docs, and logs.
- The design replaces the standalone workflow artifact direction with default shared-library behavior integrated into the memory-system tools.
