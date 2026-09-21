# [AOH][Research] Messaging Service APIs for Async User Interaction

## Status: COMPLETE — All decisions locked 2026-04-13

## Context

**Updated 2026-04-09**: WhatsApp removed from candidates — requires paid Meta Business account. Active candidates: **Telegram** (primary), **Discord**, **Slack**.

---

## Resolved Decisions (2026-04-13)

| Decision | Resolution |
|---|---|
| **Adapter implementation order** | Telegram first (MVP), Discord second, Slack third. Matches ADR-2. `teloxide` is the best Rust bot crate with zero setup cost. |
| **MultiNotifier routing policy (v1)** | `PrimaryOnly` — Telegram only in v1. Trait design supports `Broadcast` and `ByEventType` for v2. |
| **Rate limiting defaults** | Max 1 notification per 10s per channel, burst up to 5 after 60s idle, digest mode when >5 events pending within 5 min. Locked as configurable defaults. |

---

## Candidates

### Telegram (Primary Candidate)
- Bot API: `sendMessage`, inline keyboards, callback queries
- Webhook or long-polling for incoming messages
- Rust: `teloxide` crate — async, actively maintained, idiomatic
- Setup: @BotFather → instant bot; zero-cost
- Interactive: `InlineKeyboardMarkup` buttons on any message
- Free-text commands: user sends any text; bot parses
- **Assessment**: fastest setup, excellent Rust support, zero cost

### Discord
- Bot API: REST + WebSocket Gateway (or HTTP Interactions for slash commands)
- Slash commands + component interactions (buttons, select menus, modals)
- Rust: `serenity` (full, active) or `twilight` (lower-level)
- Setup: Discord Developer Portal; bot invite link; zero-cost
- Interactive: button/select components in message embeds
- Thread support: native thread creation per notification
- **Assessment**: excellent Rust crates; dev-team-native; good embed formatting

### Slack
- Web API + Events API (webhook) or Socket Mode (WebSocket, no public URL needed)
- Block Kit for interactive messages (buttons, overflow menus, date pickers)
- Rust: `slack-morphism` crate (async, maintained)
- Setup: Slack App creation; free tier sufficient for bot messages
- Interactive: Block Kit buttons → action callbacks
- Thread support: `thread_ts` reply threading
- **Assessment**: most common in enterprise dev teams; richest formatting; Socket Mode avoids needing a public HTTP endpoint

---

## Feature Comparison Matrix

| Feature | Telegram | Discord | Slack |
|---|---|---|---|
| Interactive buttons | ✓ | ✓ | ✓ |
| Thread replies | ✓ (reply_to) | ✓ (native threads) | ✓ (thread_ts) |
| Code blocks | ✓ | ✓ (markdown) | ✓ (mrkdwn) |
| Rust crate | `teloxide` ✓ | `serenity`/`twilight` ✓ | `slack-morphism` ✓ |
| Free tier | ✓ | ✓ | ✓ (10k msg history) |
| Public URL needed | No (polling) | No (gateway WS) | No (Socket Mode) |
| Mobile app | ✓ | ✓ | ✓ |
| Setup complexity | Very low | Low | Low-medium |

---

## Recommended Adapter Order

1. **Telegram** — MVP adapter (fastest to implement; `teloxide` is the best Rust bot crate)
2. **Discord** — Second adapter (`serenity` is mature; dev teams already on Discord)
3. **Slack** — Third adapter (Socket Mode avoids public URL; adds enterprise reach)

Build all three behind the `Notifier` trait — same interface, swappable adapters.

---

## Notifier Trait Design

```rust
pub trait Notifier: Send + Sync + 'static {
    async fn notify(&self, event: &OrchestratorEvent) -> Result<MessageHandle>;
    async fn poll_commands(&self) -> Result<Vec<UserCommand>>;
    async fn update(&self, handle: &MessageHandle, event: &OrchestratorEvent) -> Result<()>;
}

pub struct MultiNotifier {
    adapters: Vec<Box<dyn Notifier>>,
    routing: RoutingPolicy,
}

pub enum RoutingPolicy {
    Broadcast,           // all events → all adapters
    PrimaryOnly,         // first adapter; others are fallback
    ByEventType(HashMap<EventType, usize>), // route specific events to specific adapters
}
```

---

## Notification Event Types

| Event | Message format | Interactive? |
|---|---|---|
| Session started | Text notification | No |
| PR ready for review | PR metadata + diff summary | Buttons: Approve / Request Changes |
| Session failed | Error summary | Buttons: Retry / Skip |
| Budget warning | Token/time usage | Buttons: Continue / Terminate |
| Budget: agent self-assessment | Agent's own analysis | Buttons: Approve budget / Terminate |
| Conflict detected | Overlapping files + affected agents | Buttons: Pause A / Pause B |
| Session revived | Summary of revival context | No |
| Digest (periodic) | X sessions active, Y PRs waiting | No |

---

## Command Parsing Grammar

Inbound free-text commands from user (any adapter):
```
approve <pr-number>
reject <pr-number> [<reason>]
request-changes <pr-number> <message>
retry <session-id>
skip <session-id>
extend-budget <session-id>
terminate <session-id>
status
list sessions
list prs
priority <ticket-id> high|medium|low
```

Commands from buttons: button payload encodes `(action, target-id)` — no parsing needed.

---

## Rate Limiting

- Soft: max 1 notification per 10 seconds per channel
- Burst: up to 5 back-to-back if idle for 60+ seconds
- Digest mode: when >5 events pending within 5 minutes → batch into summary

---

## Acceptance Criteria

- [ ] `Notifier` trait defined with `MessageHandle`, `OrchestratorEvent`, `UserCommand` types
- [ ] `MultiNotifier` routing implemented with Broadcast and PrimaryOnly policies
- [ ] Telegram adapter implemented with `teloxide`: send, button interactions, command parse
- [ ] Discord adapter implemented with `serenity`: send embed, button interactions, command parse
- [ ] Slack adapter implemented with `slack-morphism`: Socket Mode, Block Kit, action callbacks
- [ ] All 8 notification event types formatted for Telegram (other adapters extend from this)
- [ ] Command deduplication (same command from two adapters within 30s) implemented
- [ ] Rate limiting documented and enforced in `MultiNotifier`
- [ ] Integration test: send test notification and receive simulated command response on Telegram