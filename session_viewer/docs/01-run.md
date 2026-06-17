# 00 Run

The viewer is embedded into TypeScript Vue application. 

For local development we first need to:

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
