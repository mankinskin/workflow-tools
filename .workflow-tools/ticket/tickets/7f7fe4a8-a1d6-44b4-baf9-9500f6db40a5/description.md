## Problem

`IndexEntry` and `IndexSidecar` already define a generic digest algorithm, but the generator tickets do not define how each domain produces the stable input fields that feed that digest. In particular, `summary`, `keywords`, `tags`, relation sets, synthetic entry IDs, ordering, and normalization rules are unspecified for ticket, spec, rule, audit, and workspace generators. Without this contract, `digest stability` is a requirement with no executable plan.

## Goal

Define the domain-level digest input contract for generated memory-index entries so every generator derives stable `IndexEntry` payloads before calling `seal()` or sidecar validation.

## Scope

- Define, per domain, the canonical source fields used to derive `title`, `summary`, `keywords`, `tags`, and relation sets.
- Define whitespace, newline, path, and ordering normalization before digest sealing.
- Define how synthetic entries such as `workspace_summary`, `index`, and `agent_hook` get stable IDs across runs.
- Define how relations are sorted or normalized before they are emitted, even though relations are excluded from the digest, so generator output remains diff-stable.
- Define the excerpt/summary rules for ticket descriptions, spec bodies, rule bodies, audit findings, and workspace summaries.
- Define the keyword and tag extraction rules, including lower-casing, deduplication, stop conditions, and ordering.
- Add golden examples or focused tests that prove identical source inputs produce identical digests across runs.
- Update the five generator tickets so they depend on this contract instead of assuming the domain normalization implicitly.

## Acceptance Criteria

- A per-domain normalization contract exists for ticket, spec, rule, audit, and workspace generators.
- The contract names the exact source fields and normalization rules used before `IndexEntry::compute_digest()` / `seal()`.
- Stable-ID rules are documented for synthetic entries so agent-hook and workspace-summary digests do not drift spuriously.
- At least one focused validation artifact or test fixture proves unchanged source inputs yield unchanged digests.
- The contract is precise enough that generator implementers do not need to guess how to produce `summary`, `keywords`, `tags`, or ordering.

## Non-goals

- Does not wire git hooks.
- Does not implement the generator binaries themselves.
- Does not change the SHA-256 digest algorithm already implemented in `IndexEntry`.

## Resolved decisions carried into this ticket

- The generic digest algorithm already exists in `memory-api`; this ticket owns the missing domain normalization layer above it.
- Digest stability must be proven at the generator input level, not assumed from the generic schema alone.
