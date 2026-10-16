<!-- aligned-structure:v2 -->
# Actionable PreToolUse configured-hook denial diagnostics

## Target Code Location

- [`../context-engine/tools/agent-hooks/rtk-hook-copilot.sh`](../../../../../context-engine/tools/agent-hooks/rtk-hook-copilot.sh) — configured Copilot `PreToolUse` wrapper whose failed invocation must be diagnosable.
- [`../context-engine/tools/agent-hooks/run-configured-hook.sh`](../../../../../context-engine/tools/agent-hooks/run-configured-hook.sh) — captures command-boundary diagnostics for each configured `PreToolUse` hook.
- [`../.github/hooks/hooks.json`](../../../../../.github/hooks/hooks.json) — workspace `PreToolUse` command ordering and wrapper path.
- [`../context-engine/.github/hooks/hooks.json`](../../../../../context-engine/.github/hooks/hooks.json) — context-engine `PreToolUse` command ordering and wrapper path.
- [`../context-engine/tools/agent-hooks/test-active-hooks.sh`](../../../../../context-engine/tools/agent-hooks/test-active-hooks.sh) — deterministic configured-hook harness that must remain deterministic.

## Naming Conventions

- Component: `agent-hooks`; slug: `agent-hooks/pretooluse-denial-diagnostics`.
- Criterion IDs use the `PTH-` prefix: `PTH-1` through `PTH-4`.
- The focused regression file is [`context-engine/tools/agent-hooks/test-pretooluse-denial.sh`](../../../../../context-engine/tools/agent-hooks/test-pretooluse-denial.sh); its validation-spec ID is `val-pretooluse-denial-diagnostics`.
- Diagnostic fields are named `failed path`, `failed command`, `stderr`, and `exit status`; attribution labels are `wrapper`, `rtk hook copilot`, and `another configured hook`.

## Requester Input

> start fixing bug ticket a7e69f7a for me and consider this as a practical example of a bug with the hooks in our workspace.

## Reading Order

1. [Bug ticket `a7e69f7a-0aa7-4781-9a44-202ecd856407`](../../../ticket/tickets/a7e69f7a-0aa7-4781-9a44-202ecd856407/ticket.toml) — incident, scope, and executable work item.
2. [W1 actionable-hook-denial part](../../../../../transcripts/15-10-2026_hook-testing-observability/01-actionable-hook-denial.md) — approved outcome, non-goals, and validation commands.
3. [`rtk-hook-copilot.sh`](../../../../../context-engine/tools/agent-hooks/rtk-hook-copilot.sh) — wrapper contract.
4. [Workspace hook manifest](../../../../../.github/hooks/hooks.json) — workspace configured-hook chain.
5. [Context-engine hook manifest](../../../../../context-engine/.github/hooks/hooks.json) — context-engine configured-hook chain.
6. [`run-configured-hook.sh`](../../../../../context-engine/tools/agent-hooks/run-configured-hook.sh) — configured-command diagnostics.
7. [`test-active-hooks.sh`](../../../../../context-engine/tools/agent-hooks/test-active-hooks.sh) — deterministic harness.
8. [`test-pretooluse-denial.sh`](../../../../../context-engine/tools/agent-hooks/test-pretooluse-denial.sh) — focused regression.

## Responsibility

This spec owns the narrow diagnostic contract for a failed configured VS Code/Copilot `PreToolUse` hook. Motivation: the present generic `hook errored` denial blocks maintainers from identifying the failing layer without local reproduction.

If this spec is implemented, dependents can rely on every denial within this configured chain being accompanied by enough factual diagnostic output to identify the failing configured path/command, its useful stderr, its exit status, and the failure origin as wrapper, `rtk hook copilot`, or another configured hook.

## Interfaces And Dependencies

The configured commands and their order are the `PreToolUse` entries in the [workspace manifest](../../../../../.github/hooks/hooks.json) and [context-engine manifest](../../../../../context-engine/.github/hooks/hooks.json). The diagnostic runner wraps each configured command and reports its observed path, command, stderr, and exit status. The RTK wrapper additionally reports whether its delegate failed; attribution must not claim an unobserved root cause.

Positions:

| Position | Status | Code reference |
| --- | --- | --- |
| Wrapper delegation failure propagation | implemented | [`rtk-hook-copilot.sh`](../../../../../context-engine/tools/agent-hooks/rtk-hook-copilot.sh) |
| Configured-command failure diagnostics | implemented | [`run-configured-hook.sh`](../../../../../context-engine/tools/agent-hooks/run-configured-hook.sh) |
| Configured-chain command declaration | implemented | [workspace manifest](../../../../../.github/hooks/hooks.json) and [context-engine manifest](../../../../../context-engine/.github/hooks/hooks.json) |
| Deterministic active-hook coverage | implemented | [`test-active-hooks.sh`](../../../../../context-engine/tools/agent-hooks/test-active-hooks.sh) |
| Focused failure-diagnostic regression | implemented | [`test-pretooluse-denial.sh`](../../../../../context-engine/tools/agent-hooks/test-pretooluse-denial.sh) |

## Behavior

| ID | Acceptance criterion | Planned proof |
| --- | --- | --- |
| PTH-1 | For a failing configured `PreToolUse` invocation, diagnostics identify the factual origin as the wrapper, `rtk hook copilot`, or another configured hook; attribution must follow the observed failing boundary and must not claim an unobserved root cause. | `bash context-engine/tools/agent-hooks/test-pretooluse-denial.sh` simulates each relevant boundary. |
| PTH-2 | The denial diagnostics include the failed path and configured command, useful stderr, and numeric exit status for the failed invocation. | `bash context-engine/tools/agent-hooks/test-pretooluse-denial.sh` asserts each required field on a nonzero failure. |
| PTH-3 | A maintainer can read the emitted denial diagnostics and distinguish the configured wrapper from its `rtk hook copilot` delegate and from a different configured hook without reproducing the incident. | Focused regression asserts attribution plus the four diagnostic fields for representative configured-chain failures. |
| PTH-4 | The existing active-hook harness remains deterministic and continues to validate the enabled manifests and wrapper behavior. | `bash context-engine/tools/agent-hooks/test-active-hooks.sh`. |

## Boundaries And Failure Cases

Only configured VS Code/Copilot `PreToolUse` denial diagnostics are in scope. A missing executable, a nonzero delegate result, and another configured command's failure must expose only factual invocation evidence. This slice does not alter general hook UX, Cline, other hook systems, CI wiring, manifest scope beyond what is required for diagnosis, or speculative root-cause attribution.

## Provider/Consumer Contract

The configured `PreToolUse` chain is the provider of invocation outcomes; maintainers consuming denial output rely on `PTH-1`, `PTH-2`, and `PTH-3`. The active-hook harness consumes the manifests and wrapper behavior and protects `PTH-4`. No separate spec component provides a contract to this single-component slice.

## Examples

Given a `PreToolUse` request whose configured wrapper invokes `rtk hook copilot` and the delegate exits `23` after writing stderr, the denial diagnostic identifies `rtk hook copilot` as the failed origin, names the wrapper path and configured command, includes the useful stderr, and reports exit status `23`. Given a later configured command fails, the diagnostic identifies that command as `another configured hook` rather than blaming the wrapper or delegate.

## Evidence

The test-api `ValidationSpec` `val-pretooluse-denial-diagnostics` runs `bash context-engine/tools/agent-hooks/test-active-hooks.sh` and `bash context-engine/tools/agent-hooks/test-pretooluse-denial.sh`; both passed with deterministic coverage of 22 enabled hook entries in [execution `w1-pretooluse-denial-20261016-02`](../../../test/executions/w1-pretooluse-denial-20261016-02.json), with captured output in [log `w1-pretooluse-denial-20261016-02-log`](../../../log/default/captures/w1-pretooluse-denial-20261016-02-log.json). The execution links this spec and [ticket `a7e69f7a-0aa7-4781-9a44-202ecd856407`](../../../ticket/tickets/a7e69f7a-0aa7-4781-9a44-202ecd856407/ticket.toml); the guard covers `PTH-1` through `PTH-4`.

## Scope

Internal planning contract for the approved W1 bug fix. It neither changes code nor transitions the ticket.