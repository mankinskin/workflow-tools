# .presentation/

Slidev-based presentational entry point for `workflow-tools`. Builds
standalone; also composed into the `context-engine` root deck via Slidev's
`src:` include (see `context-engine/.presentation/slides.md`).

```bash
npm install
npm run dev    # hot reload
npm run build  # static SPA in dist/
```

See `context-engine/.presentation/README.md` for the full composition pattern
and instructions for replicating this scaffold into another repository.
