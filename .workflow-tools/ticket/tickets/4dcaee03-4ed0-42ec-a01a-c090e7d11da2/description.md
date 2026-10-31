Apply install and autofix link rewrites by exact span (spec `0c5727b1`, waypoint W4).

Scope: `guidance/install.rs` edits only located spans (deduplicated, applied back-to-front), emits `encode_destination` output, and records a non-blocking plan warning for unlocatable rewrites; `guidance/autofix.rs` re-parses the current source and rewrites every located span whose destination equals the raw destination, refusing when none is located. Regression tests: code-fence/code-span text untouched, reference-style rewrite, encoded destination, unlocatable warn/refuse.
