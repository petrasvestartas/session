# 000 · Empty project to a wasm page

A browser runs JavaScript and WebAssembly, not Rust. WebAssembly (wasm) is a compact binary format that every modern browser runs at near-native speed. So we compile our Rust to wasm, and two tools wrap it: wasm-bindgen writes the JavaScript that loads the `.wasm` file, and Trunk builds the web page around both.

This lesson makes that crate: six small files and one function. Five of the files are final; you write them once and never open them again. The page and the manifest grow later by one line at a time.

![Four tools and four artefacts: cargo produces a .wasm a browser cannot load on its own, wasm-bindgen writes the JavaScript that can, Trunk assembles the page around it, and the browser runs the start function.](illustrations/toolchain.svg)

## Make the folder

```sh
cd session/session_viewer/docs/lessons
mkdir -p mine/src mine/.cargo
cd mine
```

Your crate sits beside the lesson crates, so the path `../../../../session_rust` in the manifest reaches the geometry kernel four folders up: the library of points, curves and meshes the viewer will draw. Every command below runs in `mine/`.

## Name the crate and its libraries

`lessons/000/Cargo.toml` · copy the file

`Cargo.toml` is the crate's manifest: its name and the libraries it uses. Two lines matter today. `crate-type = ["cdylib", ...]` asks for a library a browser can load, and `wgpu` is the library that talks to the GPU. The other libraries serve later lessons; listing them now means this file never changes again.

```toml
--8<-- "lessons/000/Cargo.toml"
```

## Build for the browser by default

`lessons/000/.cargo/config.toml` · type this

A target is the machine cargo compiles for. We make the browser, `wasm32-unknown-unknown`, the default, so a plain `cargo check` checks the browser build. Tests cannot run inside wasm, so the alias `cargo xtest` runs them on your own machine instead.

```toml
--8<-- "lessons/000/.cargo/config.toml"
```

## Tell Trunk how to build and serve

`lessons/000/Trunk.toml` · type this

Trunk builds the page into a `dist` folder and, with `trunk serve`, serves it at `http://127.0.0.1:8770` and rebuilds when a watched file changes. The watch list is one path per line: later lessons add their folders to it.

```toml
--8<-- "lessons/000/Trunk.toml"
```

## Keep build output out of git

`lessons/000/.gitignore` · copy the file

`target` and `dist` are rebuilt from the source, so git never stores them.

```text
--8<-- "lessons/000/.gitignore"
```

## Pin every library version

`lessons/000/Cargo.lock` · [download the file](https://github.com/petrasvestartas/session/blob/main/session_viewer/docs/lessons/000/Cargo.lock) into `mine/`

`Cargo.lock` records the exact version of every library, so your build uses the same code as ours. Never edit it by hand.

## The page the browser opens

`lessons/000/index.html` · type this

Three parts matter. The `<canvas>` fills the window; the viewer will draw into it. The line `<link data-trunk rel="rust" ...>` tells Trunk to build this crate and load it into the page. The hidden `viewer-error` box and the `viewer-status` line are where the viewer talks to you: an error with a reload button, a message in the corner. The style block removes the page margins and the focus ring, and hands every touch gesture to the canvas.

```html
--8<-- "lessons/000/index.html"
```

## The first function the browser runs

`lessons/000/src/lib.rs` · type this, new file

`#[wasm_bindgen(start)]` marks the function the JavaScript glue calls once the module has loaded. For now it does one thing: a Rust panic will print its message in the browser console instead of a bare "unreachable".

```rust
--8<-- "lessons/000/src/lib.rs:000-entry"
```

## Checkpoint

Run `cargo check`. It compiles without producing a file, so it is the fast way to find mistakes. The first run compiles every library once, which takes a few minutes; after that your crate checks in seconds.

Run `trunk serve` and open <http://127.0.0.1:8770>. You should see a light grey page and nothing else. That grey is the page's CSS background; the GPU has not drawn a single pixel yet. Open the browser console: it is empty. If cargo cannot find `session_rust`, your folder is not four levels below the one that holds it.

## Recap

The browser runs wasm; cargo makes it, wasm-bindgen writes the JavaScript that loads it, and Trunk builds the page. Your crate compiles for the browser by default and tests on your machine with `cargo xtest`. Next we give the page a window: the event loop that every later lesson plugs into.

Next: [001 · A window on the canvas](001-window.md)
