# Path-rendering campaign inventory

This tool consumes the common Docker driver's copied-source JSON attestation,
not a grep result or a container checkout with hidden Git metadata. It accounts
for every recursive repository, copied Rust file, and excluded Rust source policy.
Paths inside its TOML/JSON schemas and Cargo arguments are deliberately portable
workspace-relative identifiers, not human native filesystem displays.

`scan --root /source --source /validation-results/source.json --output ...`
creates a new, entirely pending ledger. It never overwrites an existing ledger.
`batches --ledger ... --stage inventory` emits deterministic exact selectors.
`validate` is intended for the common container driver, not host Cargo execution.

The syntactic candidate visitor covers conversions, path/string/error wrappers,
formatting macros, Display/Debug/serde implementations and generated derives.
It is **not a semantic completeness proof**: every file has an unresolved
whole-file semantic-review gap, including files with zero syntactic candidates.
Parsing failures, missing owners and excluded/generated Rust inputs have explicit
additional gaps. Human review must account for hidden conversions and add their
occurrences before resolving those gaps.

## Ledger contract

Schema 1 separates repositories, file reviews, occurrences, test owners and
immutable publication. Stable candidate IDs depend on the portable file identity,
candidate kind, source-line anchor and duplicate index, not a global row counter.
Every row starts unclassified and unimplemented. Classification requires a
rationale, verified whole-file inspection, resolved coverage gaps and a concrete
semantic `rendering_context` separate from the immutable source-line `context`,
Cargo owner or justified generated/exception policy. `no-occurrence` is a human
disposition, never a parser fallback. New files/repositories, missing detected
candidates, duplicate IDs, unowned tests and changed source hashes fail checks.

Each owner specifies manifest, package manifest, package, full locked Cargo
arguments and dependency recipes. The initial supported Linux recipe is
`rust-bookworm-openssl`; unsupported native prerequisites fail explicitly and
require a real recipe extension, not an exemption. Windows applicability starts
`pending`; a later verified assignment provides native tests/recipe or a specific
inapplicability rationale.

Batches inspect at most 25 files or 100 occurrences, including splitting a large
file across consecutive batches. Selectors are `batch:<group>:<nnn>` or
`windows-batch:<group>:<nnn>`, with groups `workflow-tools`, `context-engine` and
`remaining`. Classification and migration use separate stage parameters.
Selections explicitly name their repository census reviews. Each group's first
batch includes its zero-Rust repositories; a repository-only group still emits
a review batch. No repository disappears merely because it has no Rust files.

## Completion gates

* `inventory-complete` and `inventory-group:<group>:complete`: complete semantic
  classification and repository census; planned migrations may remain.
* `group:<group>:complete`: implemented eligible occurrences and current owning
  Linux tests.
* `linux-migration-complete`: all groups migrated, current Linux evidence and
  resolved Windows applicability for each owner.
* `final-both-platform-complete`: no unresolved rows/gaps, latest immutable
  publication reflected in independent consumers, current Linux evidence and
  native Windows evidence for every applicable owner.

Passed common-driver Docker receipts supply owner proofs. Owner identities cover
reachable local source/tests, workspace Cargo configuration, relevant lockfile
sections, concrete test arguments and harness recipe inputs. Unrelated owner or
lock-section changes do not invalidate them. Actual tests must return success;
only the real container OS can generate that platform's proof.

Source metadata carries immutable recipe digests per platform. A host adapter
preserves the other platform's digest only while the shared recipe inputs remain
unchanged. Missing or changed base/toolchain identities invalidate owner evidence;
the terminal Windows adapter must attest its own digest rather than reuse Linux.
Generation receipts include the complete deterministic batch-selector list.
Final runs also verify actual consumer manifest pins, including aliases and
workspace inheritance, and reject renderer changes after the published revision.

After changing a classified source file, explicitly reconcile its rows and hashes
with a new scan. Never preserve old classifications merely by updating the
ledger's global source digest.
