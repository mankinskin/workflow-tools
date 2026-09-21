Goal: Implement visible, non-fatal reporting and retry/alerting for ticket-mirror failures.

Acceptance Criteria:
- Failures are surfaced in a dashboard or log view without causing the store-write operation to abort/crash (non-fatal).
- Retry strategy: configurable backoff and retry limits for mirror writes, with observable metrics.
- Alerting: integration points for paging/email/Slack alerts when retries exceed threshold.
- Migration-safe: must not mutate live tickets; any corrective writes must be explicit and opt-in.

Notes:
- Mirror failures should be logged with enough context to reproduce: original request, target ticket id, error detail, and sandboxed replacement artifacts where applicable.
- Link to coordination ticket for overlaps and to spec `5e52039d` as traceability.