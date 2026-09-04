---
description: "Use when adding or verifying a Cargo dependency owned by another repository."
applyTo: "{Cargo.toml,Cargo.lock,**/Cargo.toml}"
---

## Committed Source Identity

Declare a cross-repository Cargo dependency with its canonical `git` URL and
`branch = \"main\"`. Do not replace the committed dependency with a local path:
the same source identity must resolve from the extracted repository and from
context-engine.

## Local Patch Override

Treat the root `[patch]` section as development-workspace composition only.
The root patch for `https://github.com/mankinskin/memory-kernel` replaces only
`memory-kernel` and `transport-harness` with their checked-out submodule paths;
it must not replace domain APIs or unrelated sources. Domain manifests retain
their canonical git declarations.

## Remote Resolution Proof

Before completing a migration, disable the root patch, run
`cargo build --workspace`, and confirm `Cargo.lock` records the dependency as
`git+https://...#<commit>`. A migration remains incomplete until the dependency
is pushed to remote `main` and the patch-free lockfile proof exists.

After disabling a patch, `cargo update -p <pkg>` can fail to select the
dependency. Do not treat that command as a repair or as proof of remote
resolution; the patch-free build and lockfile source are the proof.

This repository-wide Cargo policy belongs here, rather than in
[core-crates.instructions.md](../engine/core-crates.instructions.md), whose
scope is limited to `context-stack` core crates. It derives from R1 and R2 of
[182940eb repository architecture dependency policies](../../../.spec/specs/182940eb-0df3-4fa0-8aff-2abce6095708/body.md).