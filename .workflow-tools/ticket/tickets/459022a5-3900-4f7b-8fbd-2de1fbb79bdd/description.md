# Problem

Short FPS samples cannot detect retained event listeners, leaked wasm-bindgen closures, unbounded tracing buffers, DOM growth, texture/resource accumulation, or gradual frame-time degradation.

# Scope

- Add scheduled/on-demand soak scenarios that repeat load, graph interaction, navigation, theme/effect toggles, data refresh, and unload/remount cycles.
- Collect available JS heap, WASM linear-memory size, DOM node count, registered/observable listener proxies where feasible, WebGPU resource counters exposed by the application, client-log buffer/drop counters, long tasks, and frame-time percentiles over time.
- Add explicit test-only telemetry APIs behind a compile-time/runtime test flag; do not expose unrestricted production internals.
- Define stabilization windows and monotonic-growth rules that tolerate allocator high-water behavior.
- Capture heap/DevTools traces or equivalent artifacts on failure where supported.
- Ensure cleanup assertions cover document listeners, RAF callbacks, timers, streams, and wasm-bindgen closures.

# Acceptance criteria

- [ ] A bounded soak scenario runs at least 100 representative interaction/remount cycles without unbounded growth beyond declared tolerances.
- [ ] Metrics distinguish stable high-water allocation from continued monotonic growth.
- [ ] A deliberately retained listener/resource fixture is detected by the harness.
- [ ] Frame-time p95/p99 drift from early to late windows is measured and budgeted.
- [ ] Sink buffering and dropped-log counters remain bounded during the soak.
- [ ] Failure artifacts include metric time series, environment metadata, correlated logs, and browser traces where available.
- [ ] The soak lane is scheduled/on-demand and does not burden the fast PR lane.

# Implementation steps

1. Define test-only telemetry and threat model for leaks.
2. Build deterministic repeated lifecycle scenarios.
3. Add time-series collection and stabilization analysis.
4. Seed a known leak to prove detection.
5. Wire nightly execution and artifact retention.