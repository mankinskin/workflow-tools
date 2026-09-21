## Objective
Extend schema loading for the contract in [e9c38d24 Schema modernization lifecycle and migration](.spec/specs/e9c38d24-42cc-4044-8b2c-6811b918530f/spec.toml).

## Requirements
- Parse external/custom `.toml` and `.json` schemas into the same model with identical inheritance and validation semantics.
- Sort the combined set of eligible TOML and JSON paths lexically before parsing and diagnostics.
- Treat every duplicate `type_id` in the same schema-kind registry, including built-in/custom collisions, as a complete atomic-load failure; report all colliding paths in lexical order.
- Preserve the previous valid registry generation when a reload fails.
- Add TOML/JSON fixture pairs that compare resolved-manifest output.

## Acceptance Criteria
Tests cover mixed-format success, cross-format collisions, multi-file collisions, invalid JSON/TOML diagnostics, deterministic ordering, and prior-generation retention.


## Permanent Compatibility Guarantee
External/custom TOML and JSON loading is a permanent supported contract after shipped built-ins convert to JSON. Add a regression fixture proving mixed-format external loads remain valid after built-in conversion.