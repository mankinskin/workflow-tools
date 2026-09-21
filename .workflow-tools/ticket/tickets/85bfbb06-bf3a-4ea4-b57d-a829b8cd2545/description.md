# Problem

An orchestrator-mode session delegates every unit of work to sub-agents, but the repository can currently observe almost nothing about those sub-agents.

Measured evidence:

- `.session/local/hook-captures/` contains only `SessionStart.json`, `UserPromptSubmit.json`, `PreToolUse.json`, `PostToolUse.json`, and `Stop.json`. It contains neither `SubagentStart.json` nor `SubagentStop.json`, even though `SubagentStart` and `SubagentStop` are documented Copilot hook events carrying `agent_id` and `agent_type` in `.agents/instructions/session/worktree-provisioning.instructions.md`.
- `.github/hooks/hooks.json` registers no handler for either sub-agent event.
- No session record exists for analyzed session `b9020ba2-df5d-426a-b1b9-228ef159cad1` in `.session/sessions/` or `.worktrees/b9020ba2-df5d-426a-b1b9-228ef159cad1/workflow-tools-restructure/.session/sessions/`, so `session.exe subagent-rollups` has nothing to report.
- The only surviving sub-agent internals are raw VS Code chat transcript JSONL records under `C:/Users/linus/AppData/Roaming/Code/User/workspaceStorage/<hash>/GitHub.copilot-chat/transcripts/<chat-id>.jsonl`. The transcript contains child user messages, internal turns, tool start/completion events, and final sub-agent messages, but is neither indexed nor repo-local nor queryable through repository tooling.

# Goal

Make delegated sub-agent runs first-class, queryable session data so delegation quality can be measured instead of reconstructed by hand.

# Requirements

- Register `SubagentStart` and `SubagentStop` in `.github/hooks/hooks.json` through `tools/agent-hooks/capture-hook-stdin.sh` so payloads land in `.session/local/hook-captures/`.
- Persist every sub-agent run in the session store, keyed by parent session id and `agent_id`, with at least `agent_type`, dispatch timestamp, stop timestamp, and outcome.
- Investigate and specify whether raw chat transcript JSONL can be ingested to recover per-sub-agent turns and tool calls, or whether hook payloads alone are sufficient.
- Determine why a session record is absent for session `b9020ba2-df5d-426a-b1b9-228ef159cad1`: capture/identity linkage is broken or the record was written to a location the CLI does not read. State the root cause.
- Extend `session.exe subagent-rollups` to report per-dispatch outcome as well as token and turn counts.