## Motivation
Explicit feedback was recorded against `ce://default/spec/73aa2bce-cf09-4319-b9c5-758599898724` during session `6eba60b8-71f7-439b-bad9-c1fd76af01f3` (tool call `call_Um7XJ5cRYTPTcY818deZ0EYR`).

## Feedback
- Rating: `mixed`
- Note: High: ROADMAP.md says there is no planning-scope blocker and sends W5 to /tickets, but 05-delivery-plan.md requires deciding whether context-agent owns APIs or consumes workflow-tools/viewer APIs before tickets can be derived. Add accepted API ownership as a W5 entry condition, then map each AC in spec 73aa2bce to one execution ticket and one validation command before ticket derivation.

## Backtrace
- Session: `6eba60b8-71f7-439b-bad9-c1fd76af01f3`
- Tool call: `call_Um7XJ5cRYTPTcY818deZ0EYR`
- Event id: `daba56f9-25a1-4e78-b62e-16f1402628a8`
- Dedupe key: `feedback-followup/6eba60b8-71f7-439b-bad9-c1fd76af01f3/call_Um7XJ5cRYTPTcY818deZ0EYR`
- FeedbackEntry: the live `feedback_ingest` call already persisted its own entry for `ce://default/spec/73aa2bce-cf09-4319-b9c5-758599898724`; cross-reference it via `feedback_inbox`/`entries_for(target)` filtered to this session and tool call (today's `feedback_ingest` transport does not yet echo back the created entry's id for direct linking here).

## Verification
Record a validation execution (test-api) confirming the reported issue is addressed before moving this ticket past `in-review`.
