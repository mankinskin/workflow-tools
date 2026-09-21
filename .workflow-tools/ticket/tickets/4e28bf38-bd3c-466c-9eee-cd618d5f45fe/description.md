# [AOH] Epic: Agent Orchestration Harness — Complete Agentic Workflow System

## Objective

Design and implement a full-stack agent orchestration harness that covers the full development lifecycle: user-driven research, ticket refinement, autonomous parallel implementation, local review/merge flow, session revival, and archival.

The current goal is to reach **implementation-ready ticket quality** before starting execution work.

## Locked v1 Decisions

- **Sandbox**: container-based Browser-as-a-Service using Docker/Podman; not microVMs for browser workloads
- **Messaging**: Telegram first, then Discord and Slack
- **Git**: GitHub remote with local-first PR management; no automatic remote push during implementation
- **UI**: ratatui TUI in v1; VS Code extension deferred
- **Agent API**: GitHub Copilot only in v1
- **Identity**: reusable nature-vocabulary personas with LRU assignment
- **Revival**: summary-injected archive-based revival
- **Budgeting**: configurable soft/hard token + time limits

## Lifecycle

```
User prompt
  → orchestrator research / interview / ticket refinement
  → ticket promoted to ready
  → orchestrator provisions isolated session
  → agent implements, validates, reports
  → local PR record opened
  → user approves or requests changes
  → merge/archive or revive session
```

## Major Subsystems

### Orchestrator core
- research + interview flow
- ready-ticket detection
- session scheduling
- conflict detection
- budget watchdog
- results ingestion

### Session execution
- per-session container runtime
- per-session git worktree/branch
- per-session MCP access
- persona-bound git identity

### Review + PR management
- local PR records
- local diff/review flow
- optional remote push / GitHub PR on explicit trigger
- revival from change requests

### Notifications + operator control
- Telegram / Discord / Slack notifier adapters
- ratatui operator console
- review, retry, terminate, and budget actions

## Existing Foundations

This epic builds on the existing Phase 2 execution-layer plan and its implementation tickets under `d5ced7e2`.

It is also now blocked on three AOH-specific refinement/design tickets:
- `02412b9a` — reconcile AOH planning with existing Phase 2 implementation tickets
- `db784443` — operator authorization, secret lifecycle, and trust boundaries
- `ffa5361a` — session archive, artifact retention, and revival schema

## Key Risks

| Risk | Severity | Mitigation |
|---|---|---|
| Duplicate implementation ownership between AOH and Phase 2 tickets | High | reconcile decomposition before implementation |
| Credential leakage in logs or notifications | Critical | trust-boundary and redaction design before implementation |
| Unsafe messenger approvals | High | explicit operator auth model |
| Ambiguous revival semantics | High | archive/revival schema contract |
| Branch/PR identity mismatch across tickets | High | naming reconciliation before implementation |

## Acceptance Criteria

- [ ] Ticket graph is implementation-ready with no major unresolved contradictions
- [ ] Existing Phase 2 implementation tickets are reconciled with AOH planning
- [ ] Security / authorization model is defined for operator and messenger actions
- [ ] Archive and revival schema is defined
- [ ] At least one canonical implementation ticket exists for each major subsystem
- [ ] Full implementation can start without reopening core architectural questions