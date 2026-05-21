# session_viewer

## Buffers

Instead of vertex buffer use point class in session_rust. 
Be sure that mesh vertex contians points too.
Be sure we use f32.


```
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}
```

I would also pass the buffers as session to wgpu.

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
   cmd //c "taskkill /F /IM trunk.exe"     
   trunk serve  
   ```

done. fire good.


## Tutorials

https://sotrh.github.io/learn-wgpu/beginner/tutorial3-pipeline/
https://github.com/sotrh/learn-wgpu/tree/master/code/beginner/tutorial3-pipeline
