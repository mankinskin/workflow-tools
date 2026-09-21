## Objective
Build fixture sanitization and `tests/fixtures/<surface>/<scenario>-v1/` convention.

## Context
Audit `tmp/test-coverage-audit/05-gap-analysis.md`: fixtures are essentially synthetic/hand-authored.
Hard requirement: strip prompts, source text, credentials, tokens, signed URLs, remotes, hosts, and email; preserve typed shape, lengths, Unicode, and newline properties.

## Acceptance criteria
- Define redaction and preservation contract.
- Define versioned fixture layout and validation.

## Out of scope
- Surface-specific fixture recording.