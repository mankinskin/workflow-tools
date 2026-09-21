## Objective

Bounded research spike: verify the actual state of candidate pure-Rust PDF crates before any implementation ticket locks in an API. No web access was available when this epic/track was authored, so every crate fact below is currently unverified.

## Target Files

- `tmp/pdf-crate-spike-findings.md` (scratch findings doc; NOT committed as a permanent doc — summarize conclusions back into this ticket's description via `mcp_ticket-mcp_update_ticket` and into T1-T9 as needed instead of leaving a stray file, per AGENTS.md "ephemeral notes" rule — delete the scratch file once findings are folded back into tickets).

## Design

Investigate (using whatever access is available — `cargo search`, `cargo info`, crates.io, docs.rs, lockfiles of other repos, or local knowledge) for each of: `lopdf`, `pdf-extract`, `printpdf`, and any other pure-Rust PDF crate found that has no C/C++ binding requirement (explicitly excluding `pdfium-render` and any wrapper around a native PDF renderer):

1. Latest published version on crates.io.
2. Maintenance status (last release date, open issue/PR velocity if determinable, whether it looks abandoned).
3. License (must resolve to MIT or Apache-2.0, or a compatible permissive license; reject anything GPL/AGPL or unclear).
4. Which of the six v1 capabilities it actually covers, concretely:
   - text extraction
   - embedded image extraction
   - page operations (merge/split/reorder/delete)
   - metadata read/write
   - PDF creation (programmatic)
   - none (typst-cli path does not need a Rust PDF crate; typst-cli itself produces PDF output directly)
5. Any hard native/C dependency hidden transitively (e.g. via optional features) that would violate the pure-Rust policy.

Produce a decision: which crate(s) are selected for which capability (a single crate may not cover all six; combining `lopdf` for structural edits/merge/split/metadata with `pdf-extract` for text and a dedicated image-extraction approach is expected, but do not assume — confirm).

## Acceptance Criteria

- [ ] Every candidate crate's latest version, license, and maintenance status is recorded and license-checked as MIT/Apache-2.0 compatible.
- [ ] A capability coverage matrix (crate x capability) is produced.
- [ ] A final crate selection decision is written back into this ticket's description (or a linked section) covering all six v1 capabilities, with no unresolved "TBD".
- [ ] If no pure-Rust crate adequately covers embedded image extraction, that is documented explicitly (T9 already treats image extraction as cuttable, so this is an acceptable spike outcome, not a blocker).
- [ ] No crate with a GPL/AGPL or unclear license is selected.
- [ ] No crate requiring pdfium or other native C/C++ PDF rendering bindings is selected.
- [ ] Findings are folded into T1 (Cargo.toml dependency versions) and any capability ticket (T3-T5, T9) whose acceptance criteria reference crate-specific behavior.
- [ ] Any scratch spike notes file is deleted once findings are folded back into tickets (no stray uncommitted research docs left behind).

## Validation Plan

Manual review: the crate selection decision recorded in this ticket is reviewed against the license and pure-Rust constraints before T1 starts. No automated test applies to a research spike; validation is the presence of a complete, unambiguous decision record with no unresolved TBDs.