# [AOH][Refinement] Reconcile AOH Architecture with Existing Phase 2 Execution Tickets

## Objective

Normalize the AOH planning tree so there is **one canonical implementation decomposition**, not a second parallel tree beside the existing Phase 2 execution-layer tickets.

This ticket exists because the current AOH planning set has introduced valuable design/refinement work, but the repository already contains implementation tickets under `d5ced7e2` for sandbox manager, assignment runner, notifier, TUI, review coordinator, and E2E execution. Starting implementation without reconciliation would create duplicate ownership and inconsistent done conditions.

## Resolved Decisions

> **Locked — do not reopen without new evidence.**

### Canonical Component → Implementation Ticket Mapping

| AOH Component | Impl Ticket Owner | Research/Design Inputs | Status |
|---|---|---|---|
| **Copilot API client** | `8c185de3` — Execution Provider Contracts + Copilot API Auth Client | `cba080b5` (Copilot API research, **done**) | Well-aligned. Ticket already references ADR-5. |
| **Container sandbox** | `51471c3e` — Sandbox Manager | `49d6fe2e` (Container BaaS, **done**), `7cf1044a` (Sandbox isolation, **done**), `65d8e6c7` (Cloud Hypervisor dead-end, **done**) | Well-aligned. Ticket has `ContainerRuntime` trait, bollard, GPU flags, ADR-1 mapping. |
| **Assignment runner** | `a8632357` — Assignment Runner and Progress Watcher | `09b68366` (Multi-agent coordination, **done**), `d45826cd` (Persona store, **done**) | Aligned but needs ADR-6 clarification (see below). |
| **Review coordinator + PR manager** | `d0cc3c8b` — Review Coordinator | `d3f76335` (Local-first git, **done**), `f3c6ed90` (GitHub API, **done**), `ffa5361a` (Session archive, **done**) | Aligned. Already includes local PR records, merge execution, archive trigger. |
| **Notifications + messaging** | `8db8ef2f` — Notifier Adapters | `89701593` (Messaging APIs, **done**) | Well-aligned. Includes `Notifier` trait, Telegram/Discord/Slack, `CommandListener`, ADR-2/ADR-12 mapping. |
| **Terminal UI** | `5af54f6c` — Terminal UI | `34bc4938` (Architecture) | Aligned. ratatui TUI per ADR-4. |
| **E2E testing** | `0135d961` — E2E Integration | All impl tickets | Aligned. Final validation gate. |
| **Orchestrator core daemon** | **No existing ticket** | All design tickets, `34bc4938` | **GAP**: The glue crate (`orchestrator-core`) that wires sandbox-manager, runner, reviewer, notifier, TUI, ticket-api, and persona-store. Needs a new ticket. |
| **Persona store** | **No existing ticket** | `d45826cd` (Persona store design, **done**) | **GAP**: The `agent-identity` crate. Needs a new implementation ticket. |

### Identified Gaps (Require New Implementation Tickets)

1. **`orchestrator-core` daemon** — The main process that:
   - Loads `orchestrator.toml` configuration
   - Initializes persona store, sandbox manager, assignment runner, review coordinator, notifier adapters
   - Starts the secret server endpoint (ADR-13)
   - Runs the TUI event loop
   - Manages session scheduling and the session state machine
   - Orchestrates the full lifecycle from ticket assignment to archive
   
   *Recommendation*: Create as child of `d5ced7e2` (Phase 2 plan). This is Track B (orchestration) from the original plan.

2. **`agent-identity` crate** — Persona store implementation:
   - `PersonaStore::load()` from `personas.toml`
   - LRU assignment algorithm
   - Revival path (same persona for same ticket)
   - Git identity configuration per worktree
   
   *Recommendation*: Create as child of `d5ced7e2`. Small, self-contained crate. Can be implemented early in Track A.

### ADR-6 Clarification: tokio::broadcast vs Polling

**Finding**: Four Phase 2 implementation tickets reference `tokio::broadcast` for real-time event delivery:
- `a8632357` (runner) — "Publish real-time events via `tokio::broadcast` channels"
- `d0cc3c8b` (reviewer) — "review events broadcast via `tokio::broadcast`"
- `8db8ef2f` (notifier) — "Notifiers subscribe to `tokio::broadcast` progress events"
- `5af54f6c` (TUI) — "TUI receives events via `tokio::broadcast`"

**ADR-6** says: "ticket-api (tickets + draftboard) as sole coordination layer in v1. No in-process event bus."

**Resolution**: ADR-6 governs **cross-agent coordination** (how parallel agent sessions learn about each other's state). The user's intent was to prevent complexity from an agent-to-agent broadcast protocol. **Intra-orchestrator process communication** (runner → TUI, runner → notifier within the same Rust process) is a different concern:
- Using `tokio::mpsc` or `tokio::broadcast` for internal event routing is an implementation detail of the orchestrator daemon, not a coordination protocol.
- Without in-process channels, the TUI would need to poll ticket-api at high frequency for stdout streaming — architecturally worse.
- **Decision**: Allow `tokio::mpsc` (not broadcast) for intra-orchestrator event routing. ticket-api remains the durable source of truth and the only layer agents interact with. Implementation tickets should update their language from "broadcast" to "mpsc" to signal this is a unidirectional internal channel, not a pub-sub bus.

### Branch Naming Convention — Fully Normalized

**ADR-11**: `aoh/{agent-id}/{ticket-slug}` — already applied to:
- `51471c3e` (sandbox manager) — worktree creation path ✅
- `f3c6ed90` (GitHub API) — branch operations section ✅
- `d3f76335` (local-first git) — all examples use canonical naming ✅
- `d0cc3c8b` (review coordinator) — branch name in LocalPR struct ✅

No remaining inconsistencies.

### AOH Ticket Classification

| Ticket | ID | Classification | Status |
|---|---|---|---|
| Interview: Requirements | `f345b954` | Research (input) | **done** |
| Cloud Hypervisor (dead-end) | `65d8e6c7` | Research (input) | **done** |
| Messaging APIs | `89701593` | Research (input) | **done** |
| Sandbox isolation | `7cf1044a` | Research (input) | **done** |
| GitHub API | `f3c6ed90` | Research (input) | **done** |
| Multi-agent coordination | `09b68366` | Research (input) | **done** |
| Container BaaS | `49d6fe2e` | Research (input) | **done** |
| Copilot API + MCP | `cba080b5` | Research (input) | **done** |
| Frameworks survey | `1b681754` | Research (input) | **done** |
| Local-first git | `d3f76335` | Design (input) | **done** |
| Persona store | `d45826cd` | Design (input) | **done** |
| Operator authorization | `db784443` | Design (input) | **done** |
| Session archive | `ffa5361a` | Design (input) | **done** |
| **This ticket** | `02412b9a` | Refinement (process) | **closing** |
| Architecture | `34bc4938` | Design (synthesizes all) | Needs final update |
| Phase 2 plan | `d5ced7e2` | Implementation-owning (parent) | Active |
| Copilot client | `8c185de3` | Implementation-owning | Ready |
| Sandbox manager | `51471c3e` | Implementation-owning | Ready |
| Assignment runner | `a8632357` | Implementation-owning | Needs ADR-6 language update |
| Review coordinator | `d0cc3c8b` | Implementation-owning | Ready |
| Notifier adapters | `8db8ef2f` | Implementation-owning | Needs ADR-6 language update |
| Terminal UI | `5af54f6c` | Implementation-owning | Needs ADR-6 language update |
| E2E integration | `0135d961` | Implementation-owning | Ready |

### Phase 2 Implementation Ticket Enrichment Status

All 7 existing Phase 2 implementation tickets are already well-enriched:
- Each has detailed component boundaries (in-scope / out-of-scope)
- Each has key data types with Rust code sketches
- Each has ADR mapping tables
- Each has specific acceptance criteria

**No thin tickets remain.** The original concern about under-specified Phase 2 tickets has been addressed by the enrichment work done during AOH design.

### Implementation Track Assignment (from Phase 2 plan)

| Track | Tickets | Dependencies |
|---|---|---|
| **A (Platform)** | `8c185de3` (Copilot client), `51471c3e` (Sandbox mgr), NEW (Persona store) | None (bootstrap T1-T5 assumed ready) |
| **B (Orchestration)** | `a8632357` (Runner), NEW (Orchestrator core) | Track A |
| **C (Operator UX)** | `8db8ef2f` (Notifier), `5af54f6c` (TUI), `d0cc3c8b` (Review coordinator) | Track B |
| **D (Quality)** | `0135d961` (E2E) | Tracks A+B+C |

## Concrete Problems to Resolve

### 1. Duplicate decomposition risk
The AOH design ticket (`34bc4938`) currently describes new Phase A–E implementation tracks, while `d5ced7e2` already has implementation children:
- `8c185de3` — execution provider contracts + Copilot API auth client
- `51471c3e` — sandbox manager
- `a8632357` — assignment runner
- `d0cc3c8b` — review coordinator
- `8db8ef2f` — notifier adapters
- `5af54f6c` — terminal UI
- `0135d961` — E2E integration / fault injection

### 2. Inconsistent naming/contracts across tickets
Examples already present:
- Branch naming differs between tickets (`aoh/{agent-id}/{ticket-slug}` vs `agent/{agent-id}/{ticket-id}/{slug}`)
- Some AOH research/design tickets still embed implementation acceptance criteria instead of design/research outputs
- Epic/design tickets and Phase 2 plan use different wording for the same components

### 3. Missing canonical mapping
There is not yet a single place that answers:
- Which existing implementation ticket owns which AOH component?
- Which AOH design/research ticket feeds which implementation ticket?
- Which tickets should be updated vs superseded vs closed as planning-only?

## Deliverables

### Canonical mapping matrix
Produce a table mapping:
- AOH design component
- Existing implementation ticket owner
- Required input tickets (research/design prerequisites)
- Remaining gaps needing new implementation tickets

### Naming and scope normalization
Decide and document one canonical form for:
- Branch naming
- Local PR identifier format
- Session/run naming
- Which tickets are planning-only vs implementation-owning

### Ticket hygiene pass
For the affected AOH tickets:
- remove stale contradictions to final ADRs
- rewrite planning tickets whose acceptance criteria currently require implementation work
- enrich the thin Phase 2 implementation tickets with AOH-specific descriptions and clear acceptance criteria

## Recommended Resolution Strategy

1. **Reuse existing Phase 2 implementation tickets** as the canonical implementation tree.
2. Treat current AOH tickets as research/design/refinement inputs that feed those implementation tickets.
3. Add or keep only genuinely new tickets where AOH introduces scope not covered by Phase 2.
4. Avoid creating a second Phase A–E implementation subtree.

## Acceptance Criteria

- [x] Canonical component→implementation ticket mapping is documented
- [x] One branch naming convention is selected and applied consistently across AOH tickets
- [x] All AOH planning tickets are classified as research, design, refinement, or implementation-owning
- [x] Thin Phase 2 implementation tickets are either enriched with descriptions or explicitly superseded
- [x] No duplicate implementation ownership remains between AOH tickets and the `d5ced7e2` subtree
- [x] Epic (`4e28bf38`) and architecture ticket (`34bc4938`) reference the reconciled implementation decomposition