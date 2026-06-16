# session_tests

Vue 3 + TypeScript site: test viewer + `session_viewer` docs (Viewer tab).

```bash
npm install                          # once

npm run dev                          # docs/UI → localhost:8769/session/  (hot-reload .md + UI)
cd ../session_viewer && trunk serve  # the 3D viewer → localhost:8770  (only when editing it)

../bash/git_push.sh "msg"            # deploy: CI builds wasm + site → GitHub Pages
```

Chain:

- edit Rust → trunk recompiles wasm → trunk's WS reloads the iframe page
- Two separate reload systems, each owning its half:
  - Vite hot-reloads the .md/UI (the outer page).
  - Trunk hot-reloads the viewer (the inner iframe).
