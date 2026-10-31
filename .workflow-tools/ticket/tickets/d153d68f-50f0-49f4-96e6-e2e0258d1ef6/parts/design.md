Implement [the authoritative contract](../../../../spec/specs/ec2f0709-180f-4ccf-b692-601796f33e05/body.md), criteria MN-W7-01 through MN-W7-05, using the [execution package](../../../../../../transcripts/29-10-2026_mission-native-mcp-planning/07-native-workflow-switch.md).

Owning code: tools/roadmap-route.py, .agents/instructions/workflow/prompt-ingestion.instructions.md, .agents/instructions/workflow/roadmap-authoring.instructions.md, .agents/instructions/workflow/roadmap-execution.instructions.md.

Example: A fresh execute-ingest captures input through Session intake, discovers its Mission directory and prepares the plan through named MCP calls. It stops before execution for the exact approval; missing capabilities stop visibly rather than creating a classic folder.

Failure boundary: No contradictory planning-prohibition copies, implicit migration or claim that a tool-list alone proves installed acceptance.

Non-goal: Mass archive migration, production Mission mutation or changing an already active route.

Checkpoint after G7 native-routing: `feat(workflow): default new intake to Mission-native planning`. Provider criteria and entry/exit conditions are linked in the execution package; a passed predecessor checkpoint and explicit execution approval are required before activation.