# Summary
Session events capture tool start/complete envelopes but omit structured tool result payloads, reducing post-session debuggability for tool-calling failures.

## Reproduction context
Session: `a0228f9f-bbac-4c82-b1e6-8a628aa91ec1`
- `events.json` contains many `tool.execution_start` / `tool.execution_complete` events with `success` booleans.
- No `tool.execution_result` events were recorded.
- Practical impact: investigation required correlating partial metadata and transcript prose instead of querying normalized tool outcomes.

## Expected
Session artifacts should retain compact structured result summaries for each tool call (status, key fields, truncation pointer, error class).

## Actual
Only start/complete envelopes are reliably persisted; detailed outcomes are dropped or inaccessible in event log schema.

## Suggested fixes
1. Add `tool.execution_result` event with normalized tuple:
- `scope | command | result | blocker | pointer`
2. Store compact payload only (bounded):
- for large outputs, include spill-file pointer + key excerpts hash.
3. Add queryability:
- index `tool_name`, `result_code`, `error_type`, `has_spill`, `duration_ms`.
4. Validation:
- regression test: replay known session and assert result events exist for all tool calls.

## Related specs
- `09f96d83-4795-4f19-9259-64ad0d452387`
- `f5e0df47-d0ec-4456-b268-689f2a41ecd7`
