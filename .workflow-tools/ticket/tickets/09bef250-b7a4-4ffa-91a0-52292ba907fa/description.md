# Problem

The current Chromium profile suite only proves that a trace file was written, while the WASM micro-benchmarks assert only that elapsed time is finite. Neither fails when performance regresses.

# Scope

- Build on `099ac71e` and the existing `withBrowserTrace`/`graph3d_bench.rs` implementation.
- Define deterministic benchmark workloads for graph load, first meaningful render, camera movement, drag, selection, texture/update work, and representative API-fed graph sizes.
- Measure warm-up separately from steady state.
- Report frame-time median/p95/p99, long frames, dropped-frame ratio, CPU task duration, selected WASM hot-path timings, and backend latency separately.
- Establish machine-qualified baselines and comparison tolerances rather than universal raw FPS claims.
- Emit machine-readable benchmark results suitable for test-api benchmark execution storage and historical comparison.
- Keep profile capture available for diagnosis when a budget fails.

# Acceptance criteria

- [ ] At least one WASM micro-benchmark and one release-browser workload fail against an intentionally tightened budget.
- [ ] Results include workload version, commit, browser, OS, CPU class, GPU/adapter class, viewport, build profile, sample count, and warm-up policy.
- [ ] Frame-time percentiles are used; smoothed FPS is not the sole gate.
- [ ] PR budgets are broad enough for stable software-rendered comparison and nightly/hardware budgets are stored separately.
- [ ] Baseline update requires an explicit reviewed artifact, not silent overwrite.
- [ ] A budget failure automatically retains the Chromium trace and correlated logs.
- [ ] Benchmark executions are queryable through the test/benchmark evidence surface.

# Implementation steps

1. Define benchmark schema and environment identity.
2. Add deterministic workloads and warm-up/sample collection.
3. Establish initial baselines on declared lanes.
4. Implement comparison and failure diagnostics.
5. Record benchmark evidence and document baseline review.