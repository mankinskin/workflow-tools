---
description: "Use when debugging, validating, or reading logs from Dockerized local services and containers. Covers rebuild verification, docker logs, and container-vs-browser evidence boundaries."
applyTo: "**/Dockerfile,**/docker-compose*.yml,**/docker-compose*.yaml,**/scripts/*.sh,**/scripts/*.ps1"
---

# Docker Container Debugging

Use Docker evidence when a local service runs inside a container or when a browser/API failure may differ from the host build.

## Rebuild And Runtime Boundary

When validating a containerized app, prove both sides of the boundary:

- Run the repository-owned startup or build script, not an ad-hoc command, unless the script is the suspected defect.
- Confirm the script rebuilt the layers that contain source code. A healthy container can still be stale when Docker reused a source-copy or builder layer.
- Confirm the running container is the one just rebuilt with `docker ps` or the script's own output before live validation.
- Treat host `cargo`, `npm`, or frontend checks as compile evidence only. Browser/API behavior must be checked against the container that serves the app.

## Logs And Verbose Trace

Verbose backend context belongs in container logs, not in frontend-facing API payloads, when the context is useful for maintainers but not directly actionable by the user.

Use bounded Docker log reads:

```bash
docker logs <container-name> --tail 120
```

Prefer a bounded `--tail` over dumping full logs. If the service emits structured fields such as request id, event kind, capability id, or trace id, use those fields to search or filter the log. Do not print secrets from environment variables, authorization headers, uploaded documents, or credential fields.

## Evidence Split

Use this split when reporting or debugging a containerized failure:

- Browser evidence: visible UI state, user-facing API status, user-facing error body, downloaded artifacts.
- API evidence: status code, stable error code, concise details, diagnostics capability or request id.
- Docker evidence: verbose model/provider responses, parser inputs, stack traces, dependency logs, and maintainer-only trace context.

A live browser pass is incomplete until the served container path and any relevant Docker logs have both been checked when the failure is container-specific.
