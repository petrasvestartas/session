# Viewer

3D, TypeScript/Vue + wasm.

## Install & download

Needs [Node 20+](https://nodejs.org/) and Rust (above). Add the wasm target + trunk:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

## Run — same on all OS

Docs / UI → localhost:8769:

```bash
cd session_tests
npm install
npm run dev
```

3D viewer → localhost:8770:

```bash
cd session_viewer
trunk serve
```
