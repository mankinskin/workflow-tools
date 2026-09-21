# Integrate vendored-skill installation into guidance install

## Context

`skills-lock.json` is the existing vendoring mechanism: `{ version: 1, skills: { <name>: { source, sourceType: "github", skillPath, computedHash } } }`. `.agents/skills/` currently holds 116 markdown files across vendored and repo-local skills.

Skill installation is a *fetch* operation, not a *render* operation, so it needs a distinct code path inside the same installer.

## Scope

- Drive `skills-lock.json` resolution from `rule install`, selectable via `--skills`.
- Verify `computedHash` after fetch; refuse to install on mismatch.
- **Security:** fetch only over HTTPS from the pinned `source`; never execute fetched content during install; treat every fetched file as untrusted data.
- Render repo-local skills from the rule store while passing vendored skills through unchanged.
- Record both kinds in the selection lockfile so `--check` covers them.

## Acceptance criteria

1. Selected vendored skills are fetched, hash-verified, and placed under the client's skill path.
2. A tampered hash aborts the install with a clear error.
3. Repo-local skills render from the rule store; vendored skills are byte-preserved.
4. `--check` detects a modified vendored skill.
