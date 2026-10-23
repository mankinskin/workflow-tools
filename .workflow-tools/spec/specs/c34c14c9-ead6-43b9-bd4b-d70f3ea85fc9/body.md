<!-- aligned-structure:v2 -->
# Selective Git Guidance Retrieval

## Target Code Location

[Guidance implementation](../../../../install/install-ctl/src/guidance/mod.rs), [transport](../../../../install/install-ctl/src/guidance/remote/transport.rs), [planner](../../../../install/install-ctl/src/guidance/plan.rs), [dependency graph](../../../../install/install-ctl/src/guidance/graph.rs) and [operator contract](../../../../install/README.md).

## Naming Conventions

`guidance get` remains the public command. Criterion IDs are `SG-01` through `SG-06`. New acquisition modules are focused under `guidance/remote/`; persistent cache format is versioned `v1`. Git object IDs and credential-free canonical repository identity, not branch names alone, identify cached content.

## Requester Input

### Preserve general Git sources

> retain-general-git

### Check remote freshness and reuse content

> check-remote-reuse-content

### Reject unsupported selective transfer

> fail-explicitly

## Reading Order

1. [Source and decisions](../../../../../transcripts/22-10-2026_guidance-selective-fetch/ARTIFACTS.md).
2. [Existing command](../../../../install/install-ctl/src/guidance/mod.rs).
3. [Pure planner](../../../../install/install-ctl/src/guidance/plan.rs) and [closure](../../../../install/install-ctl/src/guidance/graph.rs).
4. [Original completed command ticket](../../../ticket/tickets/d062c9bd-0e3f-42b0-9448-f295e06f19f2/ticket.toml) is provenance, not new execution ownership.

## Responsibility

Installing a small selection must not require a complete shallow repository/submodule checkout. If this spec is implemented, dependents can rely on selective Git acquisition and verified retry reuse without a GitHub-only restriction or stale-data success fallback.

## Interfaces And Dependencies

Network acquisition is a distinct fallible phase, using typed blocking Git protocol capabilities and object retrieval. The filesystem planner and existing local plan/install commands remain network-free. The source workspace, code checkout, cache root, destination and entity-store root remain distinct. General HTTP(S), SSH, Git and local/file sources use supported transport configuration and credential helpers; unsupported protocol/capabilities fail explicitly.

## Behavior

SG-01: Before each fetch interaction, inspect negotiated capabilities. Require selective filtering for metadata packs (`blob:none`); request individual blob OIDs thereafter. Never run an unfiltered fetch/clone fallback or parse debug packet traces as an API. Unexpected objects or protocol failures are errors; guarantees assume a conforming peer, not protection against unsolicited hostile network bytes.

SG-02: Resolve remote HEAD to an immutable revision, load an explicit profile only if requested, validate selected IDs, and resolve all selected source roots from metadata before downloading guidance blobs. An explicit path may traverse only its owning submodule chain. Fetch only selected `.md`/`.toml` files, supported recipe inputs, local transitive closure and necessary metadata (including `.gitmodules` only on entry). Preserve existing external/non-guidance link classification. Full repository tree metadata is permitted; unrelated file-content transfer is not. Bare-filename lookup preserves deterministic selection and may require metadata traversal, including candidate submodule trees when necessary to resolve that selector.

SG-03: Traverse gitlinks lazily; use the recorded child commit, never mutable child HEAD. Resolve nested relative URLs using Git semantics. Required unavailable child revisions, mappings or transports block installation; no best-effort skipping.

SG-04: Store objects under `dirs::cache_dir()/workflow-tools/install-ctl/guidance/v1`, segregated by canonical repository identity and Git object format/OID. Resolve HEAD every call, reuse already verified object bytes across revisions, and fail on remote/authentication errors instead of using stale cache silently. Validate object hashes/types on reads; publish files atomically under per-repository process locks; interrupted writes cannot become valid objects. Recover corruption explicitly by diagnostic and refetch under lock. Preserve valid cache after selection errors. Do not store credentials in cache keys, URLs, manifests or logs.

SG-05: Preserve explicit profiles, copy recipes' source-to-output identity, directory `.md`/`.toml` expansion, lexical path guards, contained symlink semantics and destination policy. Reject escaping/cyclic symlinks explicitly. Temporary selected-source materialization is separate from the cache. Default cleanup removes only invocation-owned temporary materialization; `--keep-checkout` retains the selected source for inspection and prints its path, never deletes or exposes the shared cache as a retained checkout. Local plan/install behavior remains unchanged.

## Boundaries And Failure Cases

No GitHub-only API, offline mode, configurable refresh policy, automatic cross-repository search for a misspelled path, full-clone escape hatch, cache eviction UI or unrelated repository cleanup. No public upload of source content. An unsupported server, unreachable revision, bad profile/selection, read failure or required missing reference fails with actionable context and zero target writes before a valid complete plan. Required capability and format failures must not become success-shaped fallback results.

## Provider/Consumer Contract

The acquisition provider supplies a complete, verified, revision-consistent selected source snapshot to the existing planner/installer consumer. Consumers rely on SG-01 through SG-05; SG-06 supplies observable evidence. No independently owned sibling/provider spec is introduced by this single installer behavior slice.

## Examples

A root instruction links to one nested submodule instruction: acquire parent metadata, the root blob, required `.gitmodules` and the pinned child metadata/blob. Do not contact an unrelated sibling submodule. A missing explicit selector acquires only metadata required to establish absence, then fails without guidance blobs or target writes. A retry at unchanged HEAD rechecks the remote but receives zero already cached selected blob objects. A changed HEAD reuses unchanged OIDs while fetching changed selected contents.

## Evidence

Positions: [public selective get](../../../../install/install-ctl/src/guidance/get.rs)
is `implemented` and functionally validated; the eager clone path was removed.
The [selective transport](../../../../install/install-ctl/src/guidance/remote/transport.rs)
and [verified cache](../../../../install/install-ctl/src/guidance/remote/cache.rs)
are `implemented` and validated.
[Closure acquisition](../../../../install/install-ctl/src/guidance/remote/acquire.rs)
is `implemented` and validated. The operator contract and public CLI help
describe the delivered behavior.

Guard [vt-guidance-selective-transport](../../../test/specs/vt-guidance-selective-transport.json)
has a persisted, read-back
[passing execution](../../../test/executions/exec-vt-guidance-selective-transport-20261023-73748255.json).
The [W1 evidence](../../../../../transcripts/22-10-2026_guidance-selective-fetch/W1-RESULTS.md)
records the command, substantive test count, observed objects and request logs,
checkpoints and test-CLI linked-spec recomputation limitation.

Guard [vt-guidance-object-cache](../../../test/specs/vt-guidance-object-cache.json)
has a persisted, read-back
[passing execution](../../../test/executions/exec-vt-guidance-object-cache-20261023-73748255.json).
[W2 evidence](../../../../../transcripts/22-10-2026_guidance-selective-fetch/W2-RESULTS.md)
records verified retry reuse, immutable identities, interruption and corruption
recovery, concurrency, credentials checks, and actual cache artifact values.

Guard [vt-guidance-selective-closure](../../../test/specs/vt-guidance-selective-closure.json)
has a read-back [passing execution](../../../test/executions/exec-vt-guidance-selective-closure-20261023-73748255.json).
[W3 evidence](../../../../../transcripts/22-10-2026_guidance-selective-fetch/W3-RESULTS.md)
records exact received-object inventory, snapshot bytes, lazy nested submodules
and local planner compatibility.

Guard [vt-guidance-selective-cli](../../../test/specs/vt-guidance-selective-cli.json)
has a read-back [passing execution](../../../test/executions/exec-vt-guidance-selective-cli-20261023-73748255.json).
[W4 evidence](../../../../../transcripts/22-10-2026_guidance-selective-fetch/W4-RESULTS.md)
records 10 real CLI fixtures, the final full crate result (321 passed, 26
ignored), actual selected-object/request/installation read-back, and scoped
format/lint results. Unchanged retries check HEAD with zero fetch commands.

Strict all-target Clippy and whole-workspace format checks remain blocked by
existing baseline findings, not claimed green. The test-CLI linked-spec
recomputation defect still prevents claiming computed whole-spec verification;
all four guard execution records were persisted and read back successfully.

## Scope

One install-ctl behavior contract. Each roadmap waypoint is a bounded single-session implementation slice; no implementation ticket is created solely to duplicate those slices. Original completed work remains unchanged. Execution requires separate requester approval and route selection.
