# T0: Verification Spike — PDF Crate Selection

## Status: Scope Reduced

A dedicated research pass with real network access has **already discharged the
survey half of this spike**. Versions, licences, maintenance status, pure-Rust
confirmation, the typst CLI contract, and the T4 merge object-ID renumbering risk
are all resolved below and must **not** be re-investigated.

What remains is the part that can only be answered by reading crate source and
running code. That is the whole of this ticket now.

**Nothing downstream may add a PDF dependency until this ticket is done.**

## Already Verified — Do Not Re-Research

Fetched from the crates.io API, the GitHub API, and raw source. All licences are
MIT / Apache-2.0 compatible. All are pure Rust: no `*-sys`, no C/C++ dependency
appears anywhere in the trees that were pulled.

| Crate | Version | Licence | Last push | Notes |
|---|---|---|---|---|
| `lopdf` | 0.44.0 | MIT | active | MSRV: README says 1.85, manifest field says 1.88 — reconcile |
| `pdf-extract` | 0.12.0 | MIT | active | Depends on `lopdf ^0.42`; also `adobe-cmap-parser`, `cff-parser`, `postscript`, `type1-encoding-parser` |
| `printpdf` | 0.12.4 | MIT | active | **Dropped** — T5 now uses `krilla` |
| `pdf` (pdf-rs) | 0.10.0 | MIT | active | Self-describes writes as "still experimental"; **do not evaluate further** |
| `typst` / `typst-cli` | 0.15.1 | Apache-2.0 | active | CLI contract verified from source, recorded in T5 |
| `krilla` | 0.8.2 | MIT OR Apache-2.0 | active | **Selected** creation backend for T5; built on `pdf-writer`, font subsetting via `skrifa`/`subsetter`/`rustybuzz` |
| `pdf-writer` | 0.15.0 | MIT OR Apache-2.0 | active | Low-level serializer underneath `krilla`; minimal deps |
| `hayro` | 0.7.1 | Apache-2.0 OR MIT | active | Pure-Rust PDF **page rasterizer** — possible T9 angle, but renders pages rather than extracting embedded XObjects |
| `oxidize-pdf` | 4.2.1 | MIT | active | Broad-scope, AI/RAG-targeted, but only ~1 year old vs lopdf's ~9 — treat maturity as unproven |

**Merge object-ID renumbering is a solved problem.** `lopdf::Document` exposes
`renumber_objects()` and `renumber_objects_with(max_id)`, and lopdf's README ships
a working merge example that renumbers each source document before combining
object maps and rebuilding the page tree and trailer. T4 should follow that
reference rather than inventing an approach.

**Encryption behaviour is known.** `lopdf` detects `/Encrypt`, exposes
`is_encrypted()` and `encryption_state`, and auto-decrypts empty-password PDFs.
Handling is specified in T2; nothing to research here.

**`pdf-extract` pulls its own `lopdf`.** It depends on `lopdf ^0.42` while we
would use 0.44 directly, so Cargo will build two copies. This is **not** a
correctness hazard: `pdf-extract`'s surface is string-in/string-out and does not
leak `lopdf` types across its boundary. It costs compile time and binary size
only. Note it in the dependency table so a later reader does not rediscover and
over-worry about it. **Empirically confirmed, not just argued from API shape:**
the scratch-crate compile/run pass (see Validation Executed below) resolved
both `lopdf` 0.44.0 (direct) and `lopdf` 0.42.0 (transitive via `pdf-extract`
0.12.0) in one dependency tree, and the crate built and ran successfully. This
was previously a suspected non-hazard; it is now a verified one.

**The `lopdf` MSRV discrepancy is cosmetic.** `rust-toolchain.toml` pins
`channel = "nightly"`, which satisfies either 1.85 or 1.88. Record the actual
figure for documentation; it gates nothing.

**`pdfium-render` and any C/C++-backed crate remain excluded by decision 3.** Do
not evaluate, do not propose.

## What This Ticket Still Has To Do

Five items. Each needs code or source-reading, not a web search.

1. **`lopdf` page operations.** Confirm the exact `Document` API for page
   delete, reorder, and rotate. The capability class is confirmed by the page-tree
   model; the literal method names are not. Record signatures for T4.
2. **`krilla` text and font requirement.** Determine whether it needs a supplied
   font asset or offers a no-embed text path. If a font must be vendored, choose
   a permissively licensed one (OFL / Apache-2.0) and record licence and
   provenance for T5.
3. **Image filter coverage in `lopdf`.** Read its filter handling directly.
   FlateDecode is certain (`flate2` is a direct dep) and DCTDecode is likely via
   the optional `image` feature. **CCITTFaxDecode (G3/G4 fax) and JPXDecode
   (JPEG 2000) are unconfirmed and assumed absent.** Confirm or refute by
   reading source. If absent, that is fine — R10 already prescribes skip-and-report
   — but record it so T8 can document it and T9 can be scoped honestly.
4. **`pdf-extract` failure modes.** Upstream documents none. Determine
   empirically: encrypted input, missing ToUnicode CMap, scanned page with no text
   layer, malformed xref. T3 needs to know which of these produce errors versus
   silent empty output.
5. **Fixture strategy.** Where test PDFs live, how they are generated or sourced,
   and their licensing. Include an encrypted fixture and an empty-password
   fixture for T2, and a colliding-object-ID pair for T4's merge test.

## Method

- Build a throwaway scratch crate **outside** the workspace (e.g. under
  `target/tmp/`) and actually exercise each item above. Do not add anything to
  the workspace `Cargo.toml` in this ticket.
- Read crate source directly for item 3 — a docs.rs API listing will not tell you
  which filters are implemented.
- Do NOT fabricate findings. An accurate "unknown, blocked by X" is the correct
  output when something cannot be determined.

## Deliverable

Append a **Findings** section to this description containing:

- Answers to all five items above, with the source file or test output as
  evidence.
- A capability → crate + API entry point binding table covering all six epic
  capabilities, with any unmet capability called out explicitly as a gap.
- A dependency set with exact version requirements, ready to paste into
  `pdf-api/Cargo.toml`.
- The fixture strategy.

### Collated capability → crate → API table

| Capability | Crate | API (exact path/signature) | Status | Evidence | Caveat |
|---|---|---|---|---|---|
| Page delete | `lopdf` 0.44.0 | `Document::delete_pages(&mut self, ids: &[u32])` | verified | target/tmp/pdf-spike/run_full.log (Claim 1) | 1-based page indices; deleting index 2 from a 3-page doc left `[1, 2]`. |
| Page reorder | `lopdf` 0.44.0 | No library method — hand-rolled by mutating the page-tree `Kids` array on the pages dictionary | verified (hand-rolled, not a library API) | target/tmp/pdf-spike/run_full.log (Claim 2); target/tmp/pdf-spike/fixture_3page_reordered_rotated.pdf | No `reorder_pages` exists; confirmed absent at compile time via `E0599`. T4 must budget for hand-rolled implementation, not a library call. |
| Page rotate | `lopdf` 0.44.0 | No library method — hand-rolled by setting `Object::Integer(90)` (multiples of 90 only) on the page dictionary's `/Rotate` entry | verified (hand-rolled, not a library API) | target/tmp/pdf-spike/run_full.log (Claim 2 — `Rotate entry: Ok(90)`); target/tmp/pdf-spike/fixture_3page_reordered_rotated.pdf | No `rotate_page` exists; round-tripped correctly through save + reload. |
| Text extraction | `pdf-extract` 0.12.0 | `pdf_extract::extract_text` (crate-level entry point; underlying `lib.rs` panics at various sites, notably `L2408`) | partial | target/tmp/pdf-spike/run_full.log (Claim 6); target/tmp/pdf-spike/run_round2.log | **SILENT-FAILURE RISK (highest-value finding of this spike):** missing ToUnicode CMap returns `Ok("\n\nAB")` — wrong text from raw char codes, undetectable by the caller. Type0/CID-keyed (Identity-H, CIDFontType2) returns `Ok("\n\n")` — empty output indistinguishable from a blank page. Scanned/no-text-layer page returns `Ok("")` — correct but carries the same signal as the CID failure, so callers cannot disambiguate "no text" from "CID extraction silently failed" on output alone. Malformed/broken xref is the one clean case: `Err(PdfError(Parse(InvalidXref)))`, catchable, no panic. Separately, a missing inherited `/MediaBox` **panics** (not `Err`) at `lib.rs:2408`, contained only via `catch_unwind`. Every `pdf-extract` call must be treated as a panicking operation at the domain boundary. |
| CCITTFax decode | `lopdf` 0.44.0 | none — `src/object.rs` L940-946 stream decoder implements only `FlateDecode`, `LZWDecode`, `ASCII85Decode` | unsupported | target/tmp/pdf-spike/run_full.log (Claim 4 background); ticket Finding 2 (source read of `src/object.rs` L940-946, `src/document.rs` L804-812) | Raw CCITTFax stream bytes are a bare fax bitstream with no container — not usable as a standalone file without a dedicated G3/G4 decoder, which lopdf lacks. T9 must skip-with-reason. |
| JPX decode | `lopdf` 0.44.0 | none for decoding, but `Document::get_page_images` (src/document.rs L804-812) returns raw encoded stream bytes + filter list, and a JPXDecode-filtered stream's raw bytes are a directly openable `.jp2` file | partial (extractable without decoding) | target/tmp/pdf-spike/run_full.log (Claim 4); ticket Finding 2 | Same for DCTDecode (JPEG). Missing `/Resources` on a page returns `Err(DictKey("Resources"))` rather than an empty `Vec` — must be mapped to a non-error "no images on this page" case, not propagated as a failure. |
| Image extraction | `lopdf` 0.44.0 | `Document::get_page_images(&self, page_id) -> Result<Vec<PdfImage<'_>>, lopdf::Error>`, `PdfImage.content: &[u8]`, `PdfImage.filters: Option<Vec<String>>` | partial | target/tmp/pdf-spike/run_full.log (Claim 4) | Viable pure-Rust path exists for DCTDecode/JPXDecode (raw-bytes-as-file); no viable path for CCITTFax. Missing-Resources error-mapping caveat above applies. |
| PDF writing / font embedding | `krilla` 0.8.2 | `Font::new(data: Data, index: u32) -> Option<Self>` / `Font::new_variable(...)`; `Surface::draw_text` requires an owned `Font` | verified | target/tmp/pdf-spike/krilla_err1.log, krilla_err2.log, krilla_err3.log (compile-time confirmation of the hard requirement) | No `Default` impl and no no-embed text path exist — a font asset **must** be supplied. krilla vendors `NotoSans-Regular.ttf` (OFL-licensed) as its own test/example asset, a reasonable but not yet finalized candidate; T5 owns final selection and provenance recording. |

**Durability risk:** `target/tmp/pdf-spike/` lives under `target/`, which `cargo clean` wipes — the scratch-crate evidence cited above is not durable and should be copied out of `target/` before it can be lost to a clean.

## Acceptance Criteria

- [x] `lopdf` page delete/reorder/rotate method signatures are recorded.
- [x] `krilla`'s font requirement is determined; if an asset is needed, a
      specific licence-compatible font is named with its provenance.
- [x] CCITTFaxDecode and JPXDecode support in `lopdf` is confirmed or refuted by
      reading source, and the result recorded for T8 and T9.
- [ ] `pdf-extract` behaviour is recorded empirically for: encrypted input,
      missing ToUnicode, scanned/no-text-layer page, malformed xref.
- [ ] All six capabilities are mapped to a crate + API entry point, or recorded
      as an unmet gap.
- [x] A fixture strategy is recorded, including encrypted, empty-password, and
      colliding-object-ID fixtures.
- [x] Every recommended crate has been exercised against a real PDF in the
      scratch project, not just read about.
- [x] The workspace `Cargo.toml` is unchanged by this ticket.
- [x] If image extraction has no viable pure-Rust path, that is recorded here so
      T9 can be cut early rather than discovered late.

## Validation

The scratch project compiles and its capability probes run successfully:

```bash
cargo run --manifest-path target/tmp/pdf-spike/Cargo.toml
```

Paste the output into the Findings section as evidence.

## Findings (source-verified)

1. **Page delete/reorder/rotate (lopdf 0.44.0).** Confirmed by reading
   `src/processor.rs`: `Document::delete_pages(&mut self, &[u32])` exists.
   **No reorder method and no rotate method exist anywhere in v0.44.0** — a
   grep of the full `src/` tree confirms their absence. T4 must implement
   reorder by rewriting the page-tree `Kids` array directly (a permutation of
   existing page object refs) and rotate by setting the page dictionary's
   `/Rotate` entry directly (multiples of 90 only). Both are hand-rolled
   page-tree edits, not calls into a library API — T4's estimate must budget
   for that.

2. **Image filter coverage (lopdf 0.44.0).** `src/object.rs` L940-946 shows
   the stream decoder implements only `FlateDecode`, `LZWDecode`, and
   `ASCII85Decode`. **`CCITTFaxDecode` and `JPXDecode` are confirmed absent**
   — not decoded by lopdf. However, `Document::get_page_images`
   (`src/document.rs` L804-812) returns the **raw encoded stream bytes plus
   the filter list** for every image XObject regardless of filter:
   - DCTDecode (JPEG) and JPXDecode (JPEG2000) streams can be extracted
     **without decoding** — the raw bytes are themselves a complete, directly
     openable `.jpg` / `.jp2` file. This is retainable scope for T9 and is
     simpler than a decode-and-reencode path.
   - CCITTFaxDecode streams are a bare fax bitstream with no container — the
     raw bytes are **not** a usable standalone file without a dedicated G3/G4
     fax decoder, which lopdf does not have. T9 must treat CCITTFax as
     unsupported and skip-with-reason; this is an explicit scope decision, not
     a gap to silently swallow.

   **Runtime-confirmed addendum (scratch-crate pass):** `get_page_images`
   returns `Result<Vec<PdfImage<'_>>, lopdf::Error>`, with `PdfImage.content:
   &[u8]` and `PdfImage.filters: Option<Vec<String>>`. On a page with no
   `/Resources` entry it returns `Err(DictKey("Resources"))` rather than an
   empty `Vec` — T9 must treat a missing-Resources page as a non-error empty
   case (map that specific error to "no images on this page"), not propagate
   it as a failure.

3. **krilla 0.8.2 font requirement.** Confirmed by reading krilla's public
   API: every text-drawing call requires an explicit `Font`, constructed via
   `Font::new(font_bytes, ...)`. **There is no default/base-14 font and no
   no-embed text path.** T5's "if a font asset must be supplied" is now a hard
   requirement, not a contingency. krilla itself vendors
   `NotoSans-Regular.ttf` (OFL-licensed) as a test/example asset — a
   reasonable candidate — but T5 still owns final selection and provenance
   recording. **Runtime-confirmed (scratch-crate pass):** `Font` has no
   `Default` impl; the only constructors are `Font::new(data: Data, index:
   u32) -> Option<Self>` and `Font::new_variable(...)`, and
   `Surface::draw_text` requires an owned `Font`. The hard requirement is now
   confirmed by compiling and running against krilla's real API, not only by
   reading it.

4. **pdf-extract 0.12.0 failure modes.** Encrypted input returns
   `Err(OutputError)` cleanly — no panic. However, its font/CMap/CID/
   colorspace handling uses `unwrap`/`expect`/`panic!` extensively and **will
   panic**, not error, on: missing `ToUnicode` CMap, unusual Type0/CID
   encodings, and a missing inherited `MediaBox` (confirmed in source, L2408
   in the vendored copy read). T3 must treat every `pdf-extract` call as a
   **panicking** operation at the domain boundary, not merely a fallible one —
   `catch_unwind` around the call, or process isolation, is required.
   Ordinary `Result` handling alone is insufficient. **Runtime-confirmed
   (scratch-crate pass):** the MediaBox panic was actually triggered —
   `pdf-extract` panics at `src/lib.rs:2408` with payload `"MediaBox"` on a
   page lacking an inherited `/MediaBox` — and
   `catch_unwind(AssertUnwindSafe(...))` **did** contain the panic, with
   execution continuing afterward. The planned panic-containment design is
   now validated as workable, not just theorized. Missing-ToUnicode, unusual
   Type0/CID, scanned/no-text-layer, and malformed-xref behavior were **not**
   exercised in this pass and remain open.

5. **Page-index base.** Both `lopdf::Document::get_pages()` and
   `pdf-extract` use 1-based page numbering at their own API surface. This
   directly confirms — does not contradict — T2's already-locked
   1-based-external/0-based-internal convention; no crate forces a different
   base.

6. **Fixture strategy.** `lopdf` ships MIT-licensed fixture PDFs in-tree at
   `assets/` (including `encrypted.pdf`), safe to vendor into this workspace
   for T2's and T4's fixture needs. `pdf-extract`'s own test fixtures use
   `.pdf.link` pointers to externally-hosted attachments, which are **not
   provenance-safe** — do not vendor or reference them. Generate any
   additional needed fixtures (empty-password, colliding-object-ID) from
   lopdf's vendored fixtures or from-scratch minimal PDFs, not from
   pdf-extract's test corpus. **Confirmed by the scratch-crate pass:**
   fixtures for the page-op and malformed-input cases were synthesized
   in-process with `lopdf::Document` builders (no file on disk, no external
   download). This simplifies the vendored-fixture plan — the page-delete/
   reorder/rotate and malformed-input cases do not need a checked-in fixture
   file at all; only the encrypted, empty-password, and colliding-object-ID
   cases need a real vendored/generated file.

## Validation Executed

The scratch-crate compile/run validation required by this ticket's own
Validation section is **done**. Executed in a throwaway crate at
`target/tmp/pdf-spike/` — untracked, `[workspace]` empty-table, never added to
the repo workspace, per the Method section's constraint.

All six claims below were confirmed by compiling and running the scratch
crate (not merely reading source):

1. `Document::delete_pages(&mut self, &[u32])` is 1-based: deleting index 2
   from a 3-page doc left pages `[1, 2]`.
2. No `reorder_pages`/`rotate_page` methods exist (confirmed again at compile
   time via `E0599`). Hand-rolled reorder (reversing the page-tree `Kids`
   array) and rotate (`Object::Integer(90)` on `/Rotate`) both round-tripped
   correctly through save + a fresh `Document::load`.
3. `get_pages(&self) -> BTreeMap<u32, ObjectId>` is 1-based, confirmed at
   runtime.
4. `get_page_images` behavior and the missing-Resources addendum — see
   Finding 2 above.
5. krilla's `Font` hard requirement — see Finding 3 above.
6. `pdf-extract`'s MediaBox panic and its containment via `catch_unwind` —
   see Finding 4 above.

Resolved dependency versions used in the scratch crate: `lopdf` 0.44.0
(direct) **and** `lopdf` 0.42.0 (transitive via `pdf-extract` 0.12.0)
coexisting in one dependency tree, `krilla` 0.8.2, `pdf-extract` 0.12.0. The
dual-`lopdf` situation (see "Already Verified" above) is now empirically
confirmed to build and run, not just argued as safe from API shape.

All open items from the original Deliverable/Acceptance Criteria are resolved
except: (a) `pdf-extract` empirical behavior for missing-ToUnicode, unusual
Type0/CID, scanned/no-text-layer, and malformed-xref inputs — not exercised in
this pass, still open; (b) the capability → crate + API entry point binding
table covering all six epic capabilities has not been assembled as a single
table (the mapping exists distributed across Findings 1-6 but is not yet
collated).

## Review Outcome (iteration close) — 2026-07-28

**AC#4 (pdf-extract failure modes): NOT waived.** A second verification-spike
round is required to empirically exercise missing-ToUnicode, Type0/CID,
scanned/no-text-layer, and malformed-xref failure modes. Only the MediaBox
panic case was exercised in round 1.

**AC#5 (collated capability→crate→API table): NOT waived.** T0 remains
blocked until the single collated table required by the Deliverable section
above is actually written as one table.

**Correction to the round-1 record.** The earlier claim of "5 PDF tickets
updated" was an overcount. Round 1 actually updated 4 tickets: T3 `a4d7df73`,
T4 `e135e28c`, T5 `42780b6e`, T9 `a59f35fb`. T2 `e9c0e280` is being folded in
now as a fifth.

## Blocks

T1 (scaffolding) and everything after it.
