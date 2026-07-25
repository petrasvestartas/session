# 01 Run

The viewer embeds in a TypeScript Vue app. For local development:

```bash
npm install                          # once
npm run dev                          # docs/UI → localhost:8769/session/  (hot-reload .md + UI)
```

Then start the viewer:


```bash
cd ../session_viewer && trunk serve  # the 3D viewer → localhost:8770  (only when editing it)
```

Chain:

- edit Rust → trunk recompiles wasm → trunk's WS reloads the iframe page

## Offline

Both the docs and the viewer run with no internet:

- **Docs** — `python serve.py` (this folder → localhost:8771). marked + highlight.js are vendored in `vendor/`, so nothing loads from a CDN.
- **Viewer** — `trunk serve --offline`. Needs `wasm-bindgen` on `PATH` matching `Cargo.lock` (trunk's `--offline` mode only checks `PATH`, not its download cache):

  ```bash
  cargo install wasm-bindgen-cli --version 0.2.122   # once; version must match Cargo.lock
  ```

  Cargo deps resolve from the local registry cache, so the build needs no network.
