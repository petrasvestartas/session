# 01 Dependencies 

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

This section describes what this crate is such as name, version, Rust language edition.
At the time of writing I use `rustc 1.96.0`, which can be update by `rustup update`

## lib

This section describes what kind of output to produce. `cdylib` stands for "C dynamic library" this is what compiles to WebAssembly (.wasm) for the browser.
By C, I don't mean C language, but C ABI.

## dependencies

- session_rust - geometry kernel
- anyhow - easy erron handling e.g. `Result<T>` with `?`
- winit - creates the windows + event loop to resize, redraw, close a window
- wgpu - the GPU API - talks to WebGPU/WebGL
- log - hands of message
- console_log - actually prints in web development tools
- console_error_panic_hook - helps to debug Rust code in web
- wasm-bindgen - the bridge between Rust <-> Javascript
- wasm-bindgen-futures - run Rust async like State::new on the browser's event loop
- web-sys - typed bindings to browser APIs such as Event, Window, Element
- getrandom - random number support
- js-sys - Rust bindings to plain JavaScript like Array, Date, Math, read browser pixel ratio when resizin canvas
- bytemuck - send vertices to GPU
- pollster - run async in native test
- egui - draw toolbar
- egui-wgpu - paint the panel on the same wgpu frame as the 3D scene.
- egui-winit - enable clicking buttons in the panel
- egui_extras - show SVG tool icons.

## profile.release

How releas builds are tuned. When `strip = true` all the debug symbols are removed for smaller download.

## package.metadata.wasm-pack.profile.release

The option `wasm-opt` is a tool that shrinks the .wasm file after building. We used feature "bulk memoery" which crashed the application. Therefore it is turned off here.