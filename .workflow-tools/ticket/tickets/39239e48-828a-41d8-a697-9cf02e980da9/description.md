# Shared Domain-Store Workspace API

## Objective
Coordinate implementation of the existing workspace contract and a kernel-owned domain-store interface. The governing workspace Spec is attached through this ticket's `spec_refs` field.

## Scope
- Define the `memory-kernel` `DomainStore` core trait and typed operation capabilities.
- Adapt Ticket, Spec, Test, Session, Feedback, Audit, and Log APIs only to operations they already support, preserving domain types and extensions.
- Align the relevant CLI/MCP/HTTP and capture consumers with canonical selection, legacy read compatibility, diagnostics, explicit selector normalization, owner-root references, discovery/write-base separation, and read-only behavior.
- Reuse existing selector, resolver, Spec/Audit transport, and TestSpec work; complete only uncovered API adapters and cross-domain validation.

## Exclusions
Rule APIs, stores, transports, and Rule-specific work; entity schema rewrites; production data migration; entity distribution; viewer UI.

## Completion criteria
All active child tickets reach terminal states; G1-G8 and G10 are satisfied with read-back evidence; the workspace Spec, child tickets, validation guard, and execution records agree on the implemented contract. Do not close the tracker before its required review and validation gates pass.