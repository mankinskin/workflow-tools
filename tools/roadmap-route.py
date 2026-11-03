"""Select a planning or approved-roadmap route without mutating any store."""

import argparse
import json
from pathlib import Path
import sys
from uuid import UUID

NATIVE_TOOLS = (
    "mission_intake_create", "mission_workspace_get", "mission_plan_get",
    "mission_plan_update", "mission_waypoint_create", "mission_waypoint_update",
    "mission_waypoint_remove", "mission_waypoint_reorder",
    "mission_waypoint_complete", "mission_plan_prepare_review",
    "mission_plan_approve", "mission_plan_replan", "mission_pause",
    "mission_resume", "mission_handoff", "mission_cancel", "mission_complete",
    "mission_review_results_add_waypoints",
    "mission_reopen_most_recent_completed",
    "mission_plan_record_review",
)

REVIEW_REQUIRED_FIELDS = (
    "workspace_root", "mission_id", "request_id", "expected_revision",
    "kind", "verdict", "reviewer", "plan_digest", "evidence_identity",
)


def fail(code: str, message: str) -> None:
    print(json.dumps({"code": code, "message": message, "fallback": None}),
          file=sys.stderr)
    sys.exit(2)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Select the source and action for an approved roadmap."
    )
    parser.add_argument("--mode", required=True,
                        choices=("classic", "mission-backed", "native-new-run"))
    parser.add_argument("--dossier")
    parser.add_argument("--workspace-root")
    parser.add_argument("--session-id")
    parser.add_argument("--tools-list", type=Path,
                        help="Fresh initialized Mission MCP tools/list result JSON")
    return parser.parse_args()


def schema_property(schema: dict, properties: dict, name: str) -> dict:
    value = properties.get(name)
    if not isinstance(value, dict):
        return {}
    reference = value.get("$ref")
    if isinstance(reference, str):
        if set(value) != {"$ref"} or not reference.startswith("#/$defs/"):
            return {}
        definitions = schema.get("$defs")
        if not isinstance(definitions, dict):
            return {}
        resolved = definitions.get(reference.removeprefix("#/$defs/"))
        return resolved if isinstance(resolved, dict) else {}
    return value


def native_route(args: argparse.Namespace) -> dict:
    if args.tools_list is None:
        fail("native_capability_unavailable",
             "Initialize Mission MCP and supply its fresh tools/list result "
             "with --tools-list; reconnect/install the named provider if missing.")
    try:
        listed = json.loads(args.tools_list.read_text())
    except (OSError, ValueError) as error:
        fail("native_capability_unavailable", f"Cannot read tools/list: {error}")
    tools = listed.get("tools") if isinstance(listed, dict) else None
    if not isinstance(tools, list):
        fail("native_capability_unavailable", "tools/list must contain a tools array.")
    for name in NATIVE_TOOLS:
        matches = [tool for tool in tools
                   if isinstance(tool, dict) and tool.get("name") == name]
        if len(matches) != 1:
            provider = ("review-recording provider"
                        if name == "mission_plan_record_review" else "W6 provider")
            fail("native_capability_unavailable",
                 f"Require exactly one named tool {name}; reconnect the {provider}.")
        tool = matches[0]
        description = tool.get("description")
        schema = tool.get("inputSchema")
        if (not isinstance(description, str)
                or "capability=mission.native-planning version=1" not in description
                or not isinstance(schema, dict)
                or schema.get("type") != "object"
                or not isinstance(schema.get("required"), list)
                or "workspace_root" not in schema["required"]):
            fail("native_capability_unavailable",
                 f"{name} must expose mission.native-planning version=1 "
                 "and the explicit-root object schema.")
        if name == "mission_plan_record_review":
            required = schema["required"]
            properties = schema.get("properties")
            fields = properties if isinstance(properties, dict) else {}
            expected_fields = set(REVIEW_REQUIRED_FIELDS) | {
                "session_root", "cold_review_id",
            }
            if (not all(isinstance(field, str) for field in required)
                    or len(required) != len(REVIEW_REQUIRED_FIELDS)
                    or set(required) != set(REVIEW_REQUIRED_FIELDS)
                    or schema.get("additionalProperties") is not False
                    or not isinstance(properties, dict)
                    or set(fields) != expected_fields
                    or any(schema_property(schema, fields, field).get("type") != "string"
                           for field in (
                               "workspace_root", "mission_id", "request_id",
                               "kind", "verdict", "reviewer", "plan_digest",
                               "evidence_identity",
                           ))
                    or schema_property(schema, fields, "expected_revision").get("type") != "integer"
                    or schema_property(schema, fields, "kind").get("enum")
                    != ["cold_walkthrough", "formal"]
                    or schema_property(schema, fields, "verdict").get("enum") != ["pass", "fail"]
                    or any(
                        schema_property(schema, fields, field).get("type") != "string"
                        or schema_property(schema, fields, field).get("nullable") is not True
                        for field in ("session_root", "cold_review_id")
                    )):
                fail(
                    "native_capability_unavailable",
                    "mission_plan_record_review must expose the strict review "
                    "input schema and cold_walkthrough/formal, pass/fail enums; "
                    "reconnect the review-recording provider.",
                )
    if args.dossier:
        fail("invalid_request", "native-new-run cannot replace an existing dossier route.")
    if not args.workspace_root:
        fail("workspace_unavailable", "Supply the explicit owning --workspace-root.")
    root = Path(args.workspace_root)
    if not root.is_absolute() or not root.is_dir():
        fail("workspace_unavailable", "--workspace-root must be an existing absolute directory.")
    try:
        UUID(args.session_id or "")
    except ValueError:
        fail("invalid_request", "Supply the active Session UUID with --session-id.")
    return {
        "mode": args.mode, "source": "session", "action": "plan-native",
        "workspace_root": str(root), "session_id": args.session_id,
        "capability": "mission.native-planning", "version": 1,
        "entry_tool": "mission_intake_create", "executable": False,
    }


def main() -> None:
    args = parse_args()
    if args.mode == "native-new-run":
        print(json.dumps(native_route(args)))
        return
    if not args.dossier:
        fail("invalid_request", f"{args.mode} requires --dossier.")
    route = {
        "mode": args.mode,
        "source": "dossier" if args.mode == "classic" else "mission",
        "dossier": args.dossier,
        "action": "execute-dossier" if args.mode == "classic" else "migrate-dossier",
    }
    print(json.dumps(route))


if __name__ == "__main__":
    main()
