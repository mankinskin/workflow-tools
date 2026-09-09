---
description: "Use when designing, debugging, or reviewing backend/frontend error handling, diagnostics, or user-facing failure states. Covers actionable error bubbling, trace-level detail, and frontend rendering obligations."
applyTo: "**/*.rs,**/*.ts,**/*.tsx,**/*.js,**/*.jsx"
---

# Error Boundary Handling

Every error boundary must preserve enough information for the next actor to resolve the failure. The next actor may be the end user, the operator, or the developer reading logs; choose the destination deliberately.

## Classify By Resolver

Classify each failure before choosing the response shape:

- User-resolvable: invalid credentials, missing authentication, wrong file count, invalid field names, oversized uploads, invalid text encoding, unsupported user input. Return a stable code, status, concise message, and concrete corrective detail to the frontend.
- Operator-resolvable: missing server environment variables, provider authentication setup, exhausted service capacity, expired retained diagnostics. Return a stable code and actionable operator instruction to the frontend when the user can contact or act as the operator.
- Developer-resolvable: parser inputs, raw model/provider responses, stack traces, internal state, dependency debug output. Write verbose context to trace/container logs and return only a concise pointer, request id, diagnostics capability, or parser summary to the frontend.

Do not collapse a known user- or operator-resolvable condition into a generic backend failure. Generic fallback errors are acceptable only after classification fails.

## Backend Response Contract

Backend error objects must be descriptive enough to stand alone:

- Include a stable machine code.
- Include the HTTP status or equivalent transport status.
- Include a human-readable message.
- Include details that explain the stage, cause class, and next action.
- Include a correlation handle when deeper logs exist, such as request id or diagnostics capability.

Do not return only a single-line `message` for an application error. Avoid leaking raw prompts, raw model responses, uploaded document content, bearer tokens, direct PATs, API keys, or stack traces in frontend-facing bodies.

## Frontend Rendering Contract

The frontend must render every frontend-facing error field the backend provides:

- Show the primary message.
- Show the stable code and status.
- Show every returned detail item.
- Fetch and display bounded diagnostics when the backend provides a diagnostics capability and the current credential mode is authorized to read that capability.
- If diagnostics retrieval fails, show that failure as an additional detail instead of silently discarding context.

A frontend alert that displays only a single line from a structured backend error is incomplete.

## Trace-Level Detail

When verbose context is needed but not frontend-safe, emit the context through tracing/logging only. For Dockerized services, verify the context with a bounded Docker log read such as:

```bash
docker logs <container-name> --tail 120
```

A frontend-safe error may say that raw model output, parser input, or stack traces were written to container logs. The raw content itself should remain in those logs unless the user explicitly requests maintainer-level disclosure and the content is known not to contain secrets or uploaded private data.

## Validation

Validate error-boundary work at the boundary that failed:

- Unit-test classification helpers for known user- or operator-resolvable cases.
- Integration-test response bodies for stable code, status, message, details, and correlation handles.
- Browser-test frontend rendering with the real UI path that a user follows.
- For containerized apps, rebuild the container and read bounded Docker logs when verbose context should be log-only.
