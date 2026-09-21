# Goal

Add incremental indexing/search over active JSONL logs and operation journal metadata.

## Scope

- Track file offsets/checkpoints and tolerate partial records while a process is writing.
- Index log entry fields, log session metadata, journal metadata, and cross-store links.
- Filter by level, target/module, span name, operation, request id, run id, journal id, graph operation id, ticket/spec/test/session links, time range, and structured fields.
- Support compact CLI/MCP output plus HTTP use by log-viewer.

## Acceptance criteria

- Active logs can be tailed/indexed without waiting for process exit.
- Query results can join log entries to session and journal metadata.
- Corrupt or partial trailing records do not poison the index.