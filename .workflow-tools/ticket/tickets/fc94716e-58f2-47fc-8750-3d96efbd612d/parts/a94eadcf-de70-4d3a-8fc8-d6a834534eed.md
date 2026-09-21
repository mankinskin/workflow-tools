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

