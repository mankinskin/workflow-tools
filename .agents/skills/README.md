# Agent Skills

Hand-owned agent skills for this repository. Skills are loaded on demand by
matching a task to a skill's `description` (by-description loading), not by
rendering monolithic rule-store files.

Governing spec: `agents/skill-infrastructure` (`a9b7ef39-ace4-4a12-a171-80f54548fc65`).

## Skill Directory Contract

- **One folder per skill.** Every skill lives at `.agents/skills/<skill-name>/SKILL.md`.
  The root `NAME.SKILL.md` scheme is retired — no file may match
  `.agents/skills/*.SKILL.md` at the skills root.
- **Frontmatter is required.** Each `SKILL.md` starts with YAML frontmatter:
  - `name` — the skill's unique slug (matches the folder name).
  - `description` — the by-description loading trigger. Written so an agent can
    decide from the description alone whether the skill applies to the current
    task. Compatible with VS Code Copilot skill loading (name + description present).
  - `applyTo` *(optional)* — a glob for path-scoped activation.
- **Hand-owned, never generated.** Skills must not carry a
  `<!-- rule-api:file generated=true -->` header. Generated artifacts do not
  live in the skills tree.
- **Supporting files** (examples, scripts, reference docs) may live alongside
  `SKILL.md` in the skill's folder and be referenced by relative path.
- **Vendored skills** record their upstream provenance in the skill folder
  (see each skill's `PROVENANCE.md`) so normalized copies stay auditable.

## Master Index

### Hand-owned skills

| Skill | Description trigger |
|---|---|
| [find-skills](./find-skills/SKILL.md) | Discover and install agent skills from the open ecosystem when the user asks "how do I do X", "find a skill for X", or wants to extend capabilities. |
| [token-optimized-agentic-engineering](./token-optimized-agentic-engineering/SKILL.md) | Maximize agent efficiency, minimize token cost, and preserve context-window capacity via token discipline, prefix-cache preservation, state externalization, and multi-tier model routing. |
| [dioxus](./dioxus/SKILL.md) | Write/debug/review Dioxus (Rust WASM) frontends — signals & components, server functions & data flow, trunk/dx WASM toolchain, viewer-api managed-viewer integration, and styling/assets. |

### Vendored skills (normalized copies; provenance in each folder's `PROVENANCE.md`)

| Skill | Domain | Upstream | Description trigger |
|---|---|---|---|
| [rust-async-patterns](./rust-async-patterns/SKILL.md) | Rust | wshobson/agents | Master async Rust with Tokio, async traits, error handling, and concurrent patterns. |
| [rust-best-practices](./rust-best-practices/SKILL.md) | Rust | apollographql/skills | Idiomatic Rust: ownership vs cloning, `Result` error handling, and review/refactor guidance. |
| [playwright-cli](./playwright-cli/SKILL.md) | Browser | microsoft/playwright-cli | Automate browser interactions and run Playwright tests via the playwright-cli tool. |
| [playwright-best-practices](./playwright-best-practices/SKILL.md) | Browser | currents-dev/playwright-best-practices-skill | Write/fix Playwright tests, debug flakiness, POM, CI/CD, mocking, accessibility, and E2E patterns. |
| [typegpu](./typegpu/SKILL.md) | WebGPU | software-mansion-labs/skills | Type-safe WebGPU in TypeScript with TypeGPU: `use gpu` shaders, `tgpu.fn`, buffers, pipelines, layouts. |
| [webgpu-threejs-tsl](./webgpu-threejs-tsl/SKILL.md) | WebGPU/3D | dgreenheck/webgpu-claude-skill | Build WebGPU Three.js apps with TSL node materials, compute shaders, and post-processing. |
| [customer-interviews](./customer-interviews/SKILL.md) | Interviewing | refoundai/lenny-skills | Conduct high-impact user/customer interviews that surface root frustrations and causal triggers. |
| [doc-coauthoring](./doc-coauthoring/SKILL.md) | Authoring | anthropics/skills | Structured workflow for co-authoring docs, proposals, specs, and decision documents. |

When adding a hand-owned skill, create its folder + `SKILL.md`, then add a row
above with the skill's `name` and a one-line summary of its `description` trigger.
When vendoring an upstream skill, copy its folder under `.agents/skills/<name>/`,
add a `PROVENANCE.md`, record it in the root `skills-lock.json`, and add a row here.
