# [profiling] Testing + benchmark matrix index doc and run commands

Child of tracker `ef3f4a91`. Author the single index document that ties the
whole profiling/benchmark track together: what exists, where it lives, and the
exact commands to run each piece. This is the discoverability deliverable.

## Content

- The browser profiling pipeline (feature flag, `profile_scope!`,
  `withBrowserTrace`, `graph3d-profiling-suite`, `tests/graph3d_bench.rs`) and
  how to capture a trace + read `blink.user_timing` marks.
- The native Criterion bench matrix (`6a19ae5f`) and `cargo bench` commands.
- The CLI/HTTP/MCP E2E matrix (`c37ea985`) and `cargo test` commands.
- The transport throughput/latency benchmarks (`2d59b99c`).
- A copy-paste run-command table for each, with the correct working dir.

## Acceptance Criteria

- [ ] One index doc exists (location agreed: doc-api source or a top-level
      `*.md` under the profiling area) covering all four sibling slices.
- [ ] Every command block lists its required working directory and any feature
      flags (`--features profile-browser`, `--target wasm32-unknown-unknown`).
- [ ] Doc links the trace and bench evidence artifacts produced by the sibling
      tickets.
- [ ] If published through doc-api, doc validation passes; otherwise note the
      substitute check used.

## Notes

- Highest-value when written last (after siblings land), but the run-command
  skeleton can be drafted early from the sibling tickets' Commands sections.
- This ticket is the natural place to record the final tracker close-out
  summary.