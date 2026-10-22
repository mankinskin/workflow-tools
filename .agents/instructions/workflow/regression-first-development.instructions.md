---
description: "Use when fixing a bug, a tool failure, or implementing a feature with a clear expected behavior. Covers reproducing the exact failure in an isolated fixture test first, enumerating sibling edge cases, fixing every layer that duplicates the logic, and recording read-back evidence."
applyTo: "**"
---

# Regression-First Development

Write the test that proves the problem before you write the fix. A fix without
a failing reproducer is a guess; a reproducer without edge cases protects only
the one input that was observed.

## Procedure

1. **Gather the exact failure.** Start from durable evidence: feedback entries,
   the session record, error text, and the failing input. Record the exact
   error message and the input that triggered it. Do not paraphrase the failure
   into a generic one.
2. **Reproduce it in an isolated fixture.** Build the scenario in a sandboxed
   temporary directory or in-memory store that the test owns (for example
   `tempfile::TempDir`). Never depend on the developer's real stores, home
   directory, network, or another session's state. Name the test after the
   observed failure.
3. **Watch it fail for the observed reason.** Run the test and confirm the
   failure matches the recorded error, not a compile error or an unrelated
   panic. A non-zero exit from a manifest or build error (for example Cargo
   exit `101` with `failed to load manifest`) is a build blocker, not a red
   test: fix the blocker first, then rerun until the real assertion fails.
4. **Fix minimally.** Change the smallest owning code path that makes the
   reproducer pass. Do not weaken or delete the reproducer to get green.
5. **Enumerate sibling edge cases.** For each fixed issue, list the inputs that
   share its shape and add a test for each: missing vs. empty vs. malformed
   files, nonexistent parents, deleted directories, every accepted and rejected
   scheme or variant, boundary paths (`..`, `.`, outside root, inside root),
   and error messages that must name the failing path. Expect some edge-case
   tests to expose further bugs; fix those the same way.
6. **Update tests that encoded the old behavior.** When an existing test
   asserted the buggy behavior, change it deliberately and say why in the
   change summary. Do not leave contradictory tests.
7. **Fix every layer that duplicates the logic.** Search for consumers, policy
   layers, proxies, and docs that mirror the changed validation (for example a
   tool-monitor proxy that re-checks filesystem path arguments). Add a
   reproducer there too, and update tool descriptions and docs so callers see
   the new contract.
8. **Validate at the crate level.** Run the full test target of every changed
   crate, not just the new tests, and inspect the summary lines for failures.
9. **Record and read back evidence.** Record the validation spec and execution
   in the test store with the red-run and green-run counts, then read the
   execution back by id before reporting success. See
   [validation-evidence.instructions.md](../../../test/.agents/instructions/testing/validation-evidence.instructions.md).

## Out-of-scope findings

Issues found during investigation that are outside the requested fix are logged
with `feedback_ingest` rather than fixed silently or dropped, following
[feedback-turn-visibility.instructions.md](feedback-turn-visibility.instructions.md).

## Features

The same loop applies to a new feature with a clear expected behavior: write
the acceptance test from the stated behavior first, confirm it fails because
the behavior is absent, implement, then add the boundary cases.
