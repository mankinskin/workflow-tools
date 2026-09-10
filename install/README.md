# install-ctl

`install-ctl` installs workspace tool binaries, runs the viewer lifecycle,
renders the artifact catalog, and plans/installs/repairs guidance corpora
under `.agents/**`. This file is the canonical operator contract for the
`guidance` command family; `../README.md` and `COMMANDS.md` link here rather
than repeating the same rules.

## Artifact registry and catalog

`artifacts.toml` is the runtime-read registry of installable binaries and
hooks. `COMMANDS.md` at the repository root is its generated projection:

```bash
cargo run -p install-ctl -- catalog --check
```

`guidance plan`/`install`/`autofix` are subcommands of the already-registered
`install-ctl` binary artifact, not separate artifacts. A user-authored
guidance profile (a `.toml` file naming a corpus and a destination) is
runtime input to those subcommands, never a registry entry — the catalog
projects installable binaries, not profiles a caller supplies on the command
line.

## `guidance` command family

```bash
install-ctl guidance plan    --source <repo> --profile <profile.toml> --select <id>... --target <repo> [--destination-scope repo|user|system|explicit] [--destination-path <path>] [--json]
install-ctl guidance install --source <repo> --profile <profile.toml> --select <id>... --target <repo> [--destination-scope ...] [--destination-path ...] [--json]
install-ctl guidance autofix --repo-root <repo> [--scope <path>] [--rewrite old=new]... (--plan | --apply --yes) [--json]
```

- `plan` computes a read-only installation plan: no writes, no recipe
  execution, no network access. It fails with a non-zero exit and prints
  every diagnostic when the plan is blocking.
- `install` recomputes the same plan, then writes only if the plan has zero
  diagnostics. Every artifact's final bytes (including any link rewrite) are
  staged in memory first; a read failure on any one artifact leaves the
  destination completely untouched, and per-file writes are atomic
  (temp file + rename). A second `install` run is idempotent and reports
  unchanged files instead of rewriting them.
- `autofix` targets existing blocking guidance-audit findings (missing
  target, non-guidance target, unsupported dependency, unsafe path,
  unreadable artifact). `--plan` is read-only. `--apply` is the only mutation
  path and requires `--yes`. Only a finding whose raw link destination
  exactly matches an explicit `--rewrite old=new` pair becomes a candidate
  operation — autofix never invents a transformation. Before writing, apply
  re-hashes each source file against the hash captured at plan time and
  refuses (does not write) any operation whose source changed since
  (`stale plan`). After applying, it reruns the guidance audit; if any
  touched source still has a blocking finding for the same destination,
  every write from this call is rolled back from its in-memory backup and
  the result reports `rolled_back: true`.

## Profile schema

A profile is one TOML file; there is no global profile registry:

```toml
[profile]
id = "example"

[[corpus]]
id = "readme"
paths = ["workflow-tools/README.md"]      # direct corpus item

[[corpus]]
id = "generated-doc"
[[corpus.recipe.step]]
kind = "copy"
from = "workflow-tools/README.md"          # repo-relative, source root
to = "workflow-tools/GENERATED.md"          # repo-relative, destination-side id

[destination]
scope = "repo"                              # repo | user | system | explicit
# path = "..."                              # required only when scope = "explicit"
```

- A **direct** corpus item lists repo-relative source paths.
- A **recipe** corpus item has typed steps. Only `kind = "copy"` is
  implemented; any other step `kind` is recorded as an `UnsupportedRecipeStep`
  blocking diagnostic and is never executed (no network access, no shell-out)
  — recipes are declarative, not scripts.
- `to` for a copy step must end in `.md` or `.toml`; a non-guidance target
  is an `UnsafeTarget` diagnostic, not a silent write.

## Dependency closure and diagnostics

Starting from the `--select`ed roots, the planner extracts Markdown links
and classifies each one:

| Link shape | Class | Closure behavior |
|---|---|---|
| `http(s)://`, `mailto:`, `#fragment` | External | Skipped, never added to the closure |
| ends in `.md` | Guidance | Added to the closure (`GuidanceReference` edge) |
| ends in `.toml` | Profile | Added to the closure (`ProfileReference` edge) |
| other relative path | Non-guidance | Skipped, recorded, never installed |
| absolute or drive-qualified path | Absolute source | `AbsoluteSourcePath` diagnostic |

Typed dependency edges (`GuidanceReference`, `ProfileReference`,
`GeneratedInput` for recipe copies, `SubmoduleOwnership`, `Unsupported`,
`Unsafe`) and their status (`Resolved`, `Missing`, `Blocked`) are included in
the plan output for inspection.

A plan is **blocking** (`is_blocking() == true`, non-zero exit, no writes)
when it contains any diagnostic: `MissingDependency`, `Cycle`,
`DuplicateDestination`, `AmbiguousOwnership`, `PathTraversal`,
`AbsoluteSourcePath`, `SymlinkEscape`, `UnsupportedRecipeStep`, or
`UnsafeTarget`. A symlink is resolved to its real path before the
source-root containment check, so a symlink escaping the source root is
rejected the same way a `..` traversal is.

## Destination scopes

| Scope | Root | Notes |
|---|---|---|
| `repo` | `--target` | Prefers an existing initialized submodule (has a `.git` *file*, not directory) that owns the artifact's `.agents/`-relative id; falls back to the canonical `.agents/...` path under the target root when no matching submodule is initialized. Intra-artifact relative links are rewritten to stay valid at the final location. |
| `user` | platform config dir (e.g. `~/.config/install-ctl/guidance`) | Injected in tests via `DestinationPaths`; never a caller's real home directory during a test run. |
| `system` | `/etc/install-ctl/guidance` (Unix) / `C:/ProgramData/install-ctl/guidance` (Windows) | Same injection contract as `user`. |
| `explicit` | `--destination-path` or the profile's `destination.path` | Required for this scope; a path containing a leading `..` that would escape itself is a `PathTraversal` diagnostic, not a partial write. |

`--destination-scope`/`--destination-path` override whatever the profile
declares; the profile's own `[destination]` table is the default.

## Fixture, Docker, and network validation

- `guidance-fixtures/README.md` and `guidance-fixtures/v1/manifest.toml` are
  the deterministic fixture contract: every fixture uses temporary source,
  target, user, and system roots and never touches a real developer
  environment or entity store.
  ```bash
  cargo test -p install-ctl guidance
  cargo test -p audit-api markdown_links
  ```
- `docker-validation/run-docker-guidance-fixtures.sh` builds the validation
  image and runs the same deterministic contract inside a fresh container,
  overriding the image's default entrypoint:
  ```bash
  bash install/docker-validation/run-docker-guidance-fixtures.sh
  ```
- `docker-validation/run-docker-validation.sh` (via `run-in-container.sh`) is
  a **separate, network-dependent smoke path**. It is supplementary evidence
  only and never substitutes for the deterministic fixture contract above.

## Quickstart: bootstrap a consumer

See `../README.md` for `bootstrap.sh` and the public `install.sh` entry
point; those cover initial CLI/MCP bundle installation, not guidance corpus
installation.
