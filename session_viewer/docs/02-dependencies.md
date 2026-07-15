# 02 Dependencies

## Cargo.toml

```toml
[package]
name = "session_viewer"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
session_rust = { path = "../session_rust" }
anyhow = "1.0"
winit = { version = "0.30", features = ["android-native-activity"] }
wgpu = { version = "29.0", features = ["webgl"] }
log = "0.4"
console_error_panic_hook = "0.1.6"
console_log = "1.0"
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = { version = "0.3", features = ["Document", "Window", "Element", "HtmlCanvasElement", "EventTarget", "Event", "CanvasRenderingContext2d", "ImageData", "Location"] }
getrandom = { version = "0.2", features = ["js"] }
js-sys = "0.3"
bytemuck = { version = "1", features = ["derive"] }
egui         = { version = "0.34", default-features = false, features = ["default_fonts"] }
egui-wgpu    = { version = "0.34", default-features = false }
egui-winit   = { version = "0.34", default-features = false }
egui_extras  = { version = "0.34", default-features = false, features = ["svg"] }

[profile.release]
strip = true

[package.metadata.wasm-pack.profile.release]
wasm-opt = false
```

## package

Crate name, version, Rust edition. Written against `rustc 1.96.0` (update via `rustup update`).

## lib

The output kind. `cdylib` ("C dynamic library" — C ABI, not the C language) compiles to WebAssembly (.wasm) for the browser.

## dependencies

- session_rust — geometry kernel
- anyhow — `Result<T>` + `?` error handling
- winit — window and event loop: resize, redraw, close
- wgpu — the GPU API, talks to WebGPU/WebGL
- log — logging facade
- console_log — prints log messages to devtools
- console_error_panic_hook — surfaces Rust panics in the browser console
- wasm-bindgen — the Rust ↔ JavaScript bridge
- wasm-bindgen-futures — runs Rust async (e.g. `State::new`) on the browser's event loop
- web-sys — typed bindings to browser APIs (Event, Window, Element, …)
- getrandom — random number support
- js-sys — bindings to plain JS (Array, Date, Math); reads the browser pixel ratio on resize
- bytemuck — casts vertices into GPU byte buffers
- pollster — runs async in native tests
- egui — the toolbar UI
- egui-wgpu — paints the panel in the same wgpu frame as the 3D scene
- egui-winit — routes input events into egui
- egui_extras — SVG tool icons

## profile.release

Release-build tuning. `strip = true` drops debug symbols for a smaller download.

## package.metadata.wasm-pack.profile.release

`wasm-opt` shrinks the .wasm after build, but its "bulk memory" feature crashed the app — disabled here.