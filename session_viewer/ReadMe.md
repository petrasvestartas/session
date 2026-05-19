# session_viewer

## Dependecies

Trunk is a build tool specifically for Rust + WebAssembly web apps.

What it does:
- Compiles your Rust code to .wasm using wasm-bindgen automatically
- Bundles it with your index.html into a deployable web app in dist/
- Runs a dev server with hot reload
- Optimizes the wasm binary with wasm-opt

```
cargo install trunk
rustup target add wasm32-unknown-unknown
```

## Guide

1. open terminal
2. go to folder:
   ```
   cd session_viewer
   cargo clean    
   ```
3. build app:
   ```
   trunk serve  
   ```

done. fire good.


## Tutorials

https://sotrh.github.io/learn-wgpu/beginner/tutorial1-window/