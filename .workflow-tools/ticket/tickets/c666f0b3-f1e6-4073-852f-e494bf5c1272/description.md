# Summary

Expose the expectation and evidence model consistently through `spec-cli`, `spec-mcp`, and `spec-http`, with one shared parity contract.

# Why

The repository started this session with transport parity as a standing principle. The richer spec model will be worse than useless if one transport sees the contract and another silently drops it.

# Scope

- extend CLI, MCP, and HTTP surfaces to create, read, update, search, and validate the new spec contract data
- add parity tests that prove the same structured behavior across transports
- define stable response shapes for expectation, acceptance, evidence, and fulfillment state

# Assumptions To Prove

- one shared fixture set can drive parity checks across all three transports
- the new model can be represented without transport-specific reinterpretation
- search and health behavior can expose the richer model compatibly

# Acceptance Criteria

- CLI, MCP, and HTTP surfaces expose the same expectation, acceptance, evidence, and fulfillment state semantics.
- Parity tests cover create, get, update, search, and health for the richer spec model.
- No transport silently drops or rewrites the structured contract data.
- The transport contract is stable enough for downstream doc, test, log, and audit integration work.

# Validation

- Focused transport parity tests covering the touched surfaces.
- Narrow CLI and HTTP checks for the new fields when parity tests are not sufficient by themselves.