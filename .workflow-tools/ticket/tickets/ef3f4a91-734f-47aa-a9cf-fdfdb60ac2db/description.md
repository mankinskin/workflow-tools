# [profiling] Performance profiling & benchmark matrix (tracker)

Parent tracker for performance profiling and benchmarking across context-engine and the viewer platform. It combines native storage/transport measurement with browser/WASM rendering evidence without treating those environments as interchangeable.

# Existing implementation

Phase 1 landed in viewer-api Dioxus:

- `profile-browser` cargo feature and zero-cost `profile_scope!` macro;
- `graph3d::render_frame` instrumentation;
- tracing-wasm timeline marks;
- Playwright Chromium trace capture;
- browser-hosted WASM graph math micro-benchmarks.

Compilation checks passed previously; browser evidence remains owned by `099ac71e`.

# Existing child slices

- `099ac71e` validate trace capture and WASM browser benchmarks.
- `6a19ae5f` native Criterion benchmark matrix.
- `c37ea985` CLI/HTTP/MCP end-to-end matrix.
- `2d59b99c` CLI/HTTP/MCP latency and throughput benchmarks.
- `d8d18128` testing/benchmark command index.

# Added viewer-platform slices

- `26a73130` define reproducible software and hardware browser/GPU profiles.
- `09bef250` turn profile capture and micro-benchmark output into machine-qualified regression budgets.
- `459022a5` add long-running browser/WASM soak and resource-leak detection.

# Acceptance criteria

- [ ] Native, browser software-rendered, and browser hardware-rendered results are stored as distinct environment classes.
- [ ] Browser measurements include warm-up, deterministic workload identity, frame-time percentiles, and environment metadata.
- [ ] At least one browser and one native performance budget fail against an intentionally tightened threshold.
- [ ] Soak runs detect a seeded retained resource and report metric time series.
- [ ] Diagnostic traces and correlated logs are retained on performance failure.
- [ ] CLI/HTTP/MCP and browser benchmark outputs are represented in the queryable test/benchmark evidence store.
- [ ] The command/index documentation names local, CI, nightly, and hardware lanes.

# Execution order

1. Validate existing browser profiling pipeline and establish GPU environment identity.
2. Add deterministic budgets and baseline review rules.
3. Add leak/soak telemetry.
4. Complete native and transport matrices.
5. Publish the consolidated command/evidence index.