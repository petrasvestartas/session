# 00 · Start Rust in the browser

**Start:** the empty viewer directory created after the [course setup](README.md#prepare-one-workspace). **Finish:** a browser message written by Rust. No GPU is requested yet.

## The tools have different jobs

Cargo reads `Cargo.toml` and compiles Rust. `Cargo.lock` records exact dependency versions. The `wasm32-unknown-unknown` target produces WebAssembly instead of a native executable. Trunk prepares that module and its JavaScript bindings, copies browser assets and serves the page. The browser loads `index.html`, then calls the WASM start function.

```text
Cargo.toml + Cargo.lock + src/lib.rs
                 ↓ Cargo / wasm-bindgen / Trunk
        index.html + JavaScript + .wasm
                 ↓ browser
          Rust changes the page status
```

`Cargo.toml` is TOML configuration, not Rust code. This portion declares a library that can become browser WASM and can also be linked by native test tools:

```toml
[package]
name = "session_viewer"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib", "rlib"]
```

Read this as a configuration explanation. Write the **complete** checkpoint file from the file list below; it also includes dependencies and build settings.

## Read the first Rust function

Open the complete `src/lib.rs` listing. `use` brings names into scope. `pub fn start()` declares a public function with no parameters. The `#[wasm_bindgen(start)]` attribute tells the generated browser binding to call it when the module loads.

`let document = ...` binds a local value. Chained calls retrieve the browser window, its document and the status element. `expect("...")` stops with a useful message if a required value is absent. Later lessons use recoverable error handling for adapter and network failures. The semicolon ends a statement. Braces group a function body.

The final calls set both visible status text and a `data-checkpoint` attribute. The test checks that Rust changed the attribute, so a static HTML message cannot falsely pass.

This is the complete first Rust file, also supplied in the ordered file list:

<!-- include-code: 00 session_viewer/src/lib.rs -->

## Write the files

Open [Complete file changes for 00](../lessons/00/index.md). Create all six files in order. Copy `Cargo.lock` exactly; it is dependency data, not useful code to transcribe. Type `src/lib.rs` yourself and read the complete browser page once.

`.cargo/config.toml` chooses the WASM target for ordinary Cargo commands. `Trunk.toml` defines the optimized build. `index.html` supplies the element IDs expected by Rust. Spelling the status ID differently in HTML and Rust produces an initialization error.

## Checkpoint

```sh
cd "$COURSE_WORK/session_viewer"
cargo check --locked --lib
trunk serve --port 8780
```

Open <http://localhost:8780/?data=off&inspect=1>. You should see **Checkpoint 00: Rust/WASM ready**. The lack of a triangle is expected at this stage. Stop the server with Ctrl+C before lesson 01.

For an exact manual-source check after completing the files:

```sh
python3 "$COURSE_REPO/docs/reconstruction/replay.py" --output "$COURSE_WORK" --through 00 --adopt
```

If Cargo cannot find `../session_rust`, the initial extraction ran in a different workspace. If the status never changes, inspect the browser console and confirm you opened the Trunk address rather than an HTML file directly from disk.

**Before continuing:** explain why `Cargo.toml`, Rust and WGSL are different files. WGSL will first appear in [lesson 01](01-first-frame.md).
