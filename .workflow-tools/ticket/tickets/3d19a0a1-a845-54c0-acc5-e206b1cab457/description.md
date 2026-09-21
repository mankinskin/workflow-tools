## Motivation
Explicit feedback was recorded against `ce://default/spec/96bc688a-62ac-4083-923f-e507f2bb19fe` during session `6f9208a4-c40e-4010-abf8-023505b4bf97` (tool call `call_Dw8Ts8bDIwMgdLvrpN1JqRDp`).

## Feedback
- Rating: `mixed`
- Note: Implementation review 2026-07-25: actionable errors, batches, CLI/MCP source parity, and hand-owned guidance passed. Anchor/category ticket AC1 needs a literal MCP schema example `kind="task", category="<your-label>"`; follow-up 6cc88405-62f2-4e36-b95e-a37f498175eb. Spec remains active/changes-requested until that gap closes.

## Backtrace
- Session: `6f9208a4-c40e-4010-abf8-023505b4bf97`
- Tool call: `call_Dw8Ts8bDIwMgdLvrpN1JqRDp`
- Event id: `unknown`
- Dedupe key: `feedback-followup/6f9208a4-c40e-4010-abf8-023505b4bf97/call_Dw8Ts8bDIwMgdLvrpN1JqRDp`
- FeedbackEntry: the live `feedback_ingest` call already persisted its own entry for `ce://default/spec/96bc688a-62ac-4083-923f-e507f2bb19fe`; cross-reference it via `feedback_inbox`/`entries_for(target)` filtered to this session and tool call (today's `feedback_ingest` transport does not yet echo back the created entry's id for direct linking here).

## Verification
Record a validation execution (test-api) confirming the reported issue is addressed before moving this ticket past `in-review`.
