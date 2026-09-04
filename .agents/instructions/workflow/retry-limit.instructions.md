---
description: "Use throughout every session, during execution of any dispatched worker-tier step: the mid-execution retry cap for test failures. Covers the exact retry count, the required escalation action on a second failure, and how this differs from the pre-dispatch fail-fast gate."
applyTo: "**"
---

## Purpose

Resolves Open Question 2 ("Mid-execution failure handling") from spec [1b654f30 Two-tier Planner/Worker model routing architecture](../../../.spec/specs/1b654f30-d1a4-4cb4-ab2e-8355dfe5a758/body.md): *when a Worker's step fails `done_criteria`, does the whole Plan invalidate back to the Planner, or can the Step be patched in place?* This instruction is that answer: **a Worker gets exactly one self-fix retry after a failing test; a second failure on the same step escalates instead of retrying again.**

## Classify Compile Failures Before Applying the Cap

Compile/build failures are not test failures and do not consume the test
self-fix retry. Run a separate compile command when needed (for example,
`cargo test -p <crate> --no-run`), persist its output to a log, and make local
mechanical repairs until the test executable can be built or a concrete blocker
requires escalation. An interrupt, timeout, or empty result before tests run is
a compile/execution diagnostic, not a failed test; do not abandon partially
applied work merely because that diagnostic occurred twice.

Only a test that actually runs and fails enters the one-self-fix-retry rule
below.

## Execution Must Be Terminal Before Retry

A timeout, backgrounded execution, interrupt, or empty result does not establish
that the command failed or stopped. The Worker MUST inspect the existing
execution and obtain a terminal status before starting another attempt. If the
process is still active, wait for the tool's completion notification or cancel
the recorded execution and confirm termination. Never launch a duplicate Cargo,
npm, Playwright, or fetch process to discover whether the first process is hung.

An identical command may be retried only after recording the changed input or
environmental condition. Without such a change, reuse the prior output or
return a blocker instead of spending the retry.

## The Rule (exact)

**A Worker (or any worker-tier sub-agent) that fails a test on a step gets exactly one self-fix retry. If the retried attempt also fails a test on that same step, the Worker MUST stop and escalate — it must not attempt a third fix.**

The self-fix retry happens **inside the one Worker session dispatched for that step** — it is intra-session iteration (attempt, observe failure, fix, attempt again) before the Worker returns its declared result and terminates, not a second spawn. This composes with, and does not violate, the one-step-then-terminate contract in [write-and-die.instructions.md](write-and-die.instructions.md): the Worker still terminates after this one step, having made at most two attempts inside it.

Escalation on the second failure means: the Worker returns `{pass: false, blocker: "<step-id> failed done_criteria twice: <first failure> / <second failure>"}` and stops. The dispatcher (Planner/Architect or orchestrator) then does one of:
- patch the Step in place (narrow fix, same `target_path`, same objective) and re-dispatch once, or
- invalidate the Step back to the Planner for re-planning if the failure indicates the Plan itself was wrong (wrong `target_path`, missing dependency, unmet hidden precondition).

Silently re-dispatching the same Worker a third time against the same step without one of these two actions is the exact failure mode this rule exists to close.

## Mid-Execution Cap vs. Pre-Dispatch Fail-Fast (distinguished)

This cap and the pre-dispatch gate in [pre-dispatch-gates.instructions.md](pre-dispatch-gates.instructions.md) apply at different points and catch different failures:

- **Pre-dispatch fail-fast** (`pre-dispatch-gates.instructions.md`) runs **before** a sub-agent is spawned at all — it blocks dispatch when a *precondition* is unmet (ticket not dispatchable, no spec coverage, missing validation commands). It never sees a test result because the work hasn't started.
- **This mid-execution retry cap** runs **after** dispatch, once the Worker has already executed its step and a test has actually failed. It bounds how many times the *same dispatched Worker* may self-fix *that same step* before control must return to the dispatcher.

A unit can pass every pre-dispatch gate and still hit this cap; the two controls are sequential, not overlapping.

## Reconciliation with the Existing "One Retry, Step Up One Band" Rule

[model-routing.instructions.md](model-routing.instructions.md) "Failure Path" already states: *"One retry, then step up exactly one band. T3 failure goes to T2. Jumping a cheap unit straight to T1/T0 is how low-tier routing turns into a net loss."*

This instruction is the **detailed, test-failure-specific form of that same rule**, not a second, conflicting policy:

- The existing rule's "one retry" **is** this rule's one self-fix retry.
- The existing rule's "step up one band" **is** this rule's escalation action, narrowed for the worker-tier test-failure case: escalating to the Planner/Architect (or re-dispatching at the next tier band per the ladder) instead of a third same-tier attempt.

Apply both together: retry once at the current tier; on the second test failure, escalate per this file's escalation action, which composes with — and does not replace — the ladder's one-band step-up when the escalation target is a re-dispatch rather than a Plan-level patch.

## Cross-Reference

See [pre-dispatch-gates.instructions.md](pre-dispatch-gates.instructions.md) for the pre-dispatch fail-fast gate this cap is distinguished from, and [model-routing.instructions.md](model-routing.instructions.md) "Failure Path" for the general one-retry/step-up-one-band rule this file specializes.
