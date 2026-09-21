## Objective
Replace shipped ticket-schema TOML sources with generated JSON built-ins backed by the resolved registry manifest in [e9c38d24 Schema modernization lifecycle and migration](.spec/specs/e9c38d24-42cc-4044-8b2c-6811b918530f/spec.toml).

## Requirements
- Convert all shipped ticket built-ins (`tracker-improvement`, `task`, `feature`, `bug`, `epic`) to JSON and delete shipped TOML copies only after the dual loader passes.
- Generate a typed resolved-registry manifest with model version, source hashes, parent chains, resolved hashes, nodes, transitions, terminals, type metadata, and local graph membership.
- Define new concrete work-item schema types: `epic`, `bug`, `research`, `planning`, `implementation`, `review`, `interview`, `testing`.
- Ensure every concrete type refines the universal lifecycle and has an explicit path through plan, act, and verify. `epic` act means orchestration; `bug` act means diagnosis/fix.

## Acceptance Criteria
Generated artifacts are reproducible, manifest schema is validated, built-ins load as JSON, external TOML remains accepted, and every concrete lifecycle passes graph validation.


## Manifest Contract
The generated resolved-registry manifest must expose top-level `model_version`, `sources`, `schemas`, and `graphs`. Schema records include IDs, parent chains, hashes, resolved nodes, transitions, terminals, and type metadata. Graph records include entity kind and schema IDs.

## Additional Acceptance Criteria
Manifest fixtures validate every required top-level and nested record field, every generated concrete type has a valid three-category path, and shipped built-in TOML deletion occurs only after Track 2 dual-loader validation passes.