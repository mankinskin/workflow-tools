<!-- aligned-structure:v2 -->
# Bounded Mission/Ticket Schema, Requirements, and Entity Rendering

## Motivation

Source requirements R1-022–026, R1-035, and R1-037–038 ask for explicit
schema/version meaning, allowed values, a bounded Mission/Ticket overlap,
requirements linked to canonical specifications, and composable entity text
renderers. Native Mission v1, typed source v2, Part source v1 and Ticket
manifest prior art already exist; this contract does not claim that schemas or
rendering are wholly absent.

If implemented, consumers can validate the demonstrated shared schema
intersection and compose entity renderings without duplicating authoritative
facts.

## Target Code Location

- [Mission model](../../../../mission/crates/mission-api/src/lib.rs)
- [Typed Mission source](../../../../mission/crates/mission-api/src/typed_source.rs)
- [Ticket model](../../../../ticket/crates/ticket-api/src/model/ticket.rs)
- [Ticket entity manifest](../../../../memory-kernel/src/model/entity.rs)

## Naming Conventions

Use `mission-shared-schema-*`, `mission-requirement-*`, and
`mission-entity-render-*` criteria. Version values are explicit and unknown
versions never silently coerce.

## Reading Order

1. [Mission Domain](../38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) — owning Mission schema and requirement model.
2. [Typed file-backed authoring](../dee92faf-b649-4577-8016-e6e465e6724a/body.md) — structured source and composition.
3. [Direct authoring](../0e02e89c-aff1-47c8-a113-3c6b36688add/body.md) — current authoring behavior.
4. [Mission source model](../../../../mission/crates/mission-api/src/typed_source.rs) — versioned data representation.
5. [Ticket model](../../../../ticket/crates/ticket-api/src/model/ticket.rs) — demonstrated Ticket prior art.

## Responsibility

Define only the shared Mission/Ticket contract supported by observed
requirements, formalize Requirement-to-Specification references, and specify
entity-specific renderers that can be composed by Mission output.

## Interfaces And Dependencies

Mission and Ticket remain separate authoritative domains. A bounded common
representation carries only the proven shared identity/version/typed-attribute
semantics; Mission requirements reference canonical specification identities.
Renderers accept typed entities and return deterministic embeddable text.

## Behavior

Schema versions, attribute meaning and allowed values are documented and
validated. Unknown, malformed and unsupported-version data is rejected or
reported with an explicit migration path. Requirements have stable IDs and
typed links to canonical specification criteria. Mission, roadmap, Ticket,
specification and criterion renderers compose without copying authoritative
facts, and preserve deterministic escaping/reference behavior.

## Boundaries And Failure Cases

Do not create a generic all-workflow entity backend, migrate unrelated
domains, create parallel criteria/test authorities, or require a single
implementation language. Shared attributes unsupported by observed Ticket and
Mission models remain domain-owned. Migration failure preserves the accepted
source version and data.

## Provider/Consumer Contract

Consumes the existing Mission Domain, typed lifecycle and Ticket model
contracts. It provides versioned validated values to Mission authoring and
composable renderers to Mission output consumers. It does not require that
these components ship serially.

## Examples

A version-1 Mission with a Requirement linked to a canonical Spec renders a
roadmap snippet and a standalone Requirement block from the same typed facts.
An unsupported schema version produces a structured error; it is not treated
as version 1.

## Acceptance Criteria

- Mission/Ticket common fields are explicitly enumerated with documented
  versioning, meanings, allowed values, validation and migration behavior.
- Unsupported versions and invalid values are rejected with source-preserving
  diagnostics.
- Requirements reference canonical specification/criterion IDs and dangling
  or wrong-root references fail.
- Entity renderers produce deterministic composable text without duplicating
  facts or changing authoritative state.
- No all-workflow backend or competing criteria/test authority is introduced.

## Guards

`w8-mission-schema-renderers` is the declared test-api ValidationSpec. No
product tests or migrations were run during W8.

## Positions

- [Typed Mission source](../../../../mission/crates/mission-api/src/typed_source.rs) — `partial`:
  versioned Mission source exists; bounded Mission/Ticket schema overlap and
  complete Requirement links remain unestablished.
- [Ticket model](../../../../ticket/crates/ticket-api/src/model/ticket.rs) — `implemented`:
  Ticket-specific typed model prior art; no shared Mission contract is claimed.
- [Entity manifest](../../../../memory-kernel/src/model/entity.rs) — `partial`: generic
  manifest exists; shared version semantics are not established.

## Evidence

The guard command is
`cargo test -p mission-api --test schema_requirement_renderer_contract`.
It must exercise valid/invalid attributes, unknown versions, migration
preservation, canonical references, deterministic composition and escaping.

## Scope

Owns bounded schema overlap, Mission requirement references and composable
entity rendering only. Broader workflow-entity architecture remains deferred.
