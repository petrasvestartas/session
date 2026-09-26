# 00 · Empty project to a WASM message

Seven files make a Rust crate that compiles to WebAssembly for the browser. The manifest and the page are already the final ones, so no later lesson edits them.

![Four tools and four artefacts: cargo produces a .wasm a browser cannot load on its own, wasm-bindgen writes the JavaScript that can, Trunk assembles the page around it, and the browser runs the start function.](illustrations/toolchain.svg)

## Make the folder

```sh
cd session/session_viewer/docs/lessons
mkdir -p mine/src mine/.cargo
cd mine
```

The folder sits beside the lesson crates, so the kernel path `../../../../session_rust` in Cargo.toml resolves; every command below runs here.

## Step 1 · Cargo.toml

The crate's name and every dependency the course uses, with the font stack pinned so only one ships in the wasm.

`lessons/00/Cargo.toml` · copy the file

```toml
--8<-- "lessons/00/Cargo.toml"
```

## Step 2 · .cargo/config.toml

Every cargo command builds for the browser, and `cargo xtest` runs the tests on this machine instead.

`lessons/00/.cargo/config.toml` · type this, new file

```toml
--8<-- "lessons/00/.cargo/config.toml"
```

## Step 3 · Trunk.toml

How Trunk builds the page, which files it watches, and where it serves the result.

`lessons/00/Trunk.toml` · copy the file

```toml
--8<-- "lessons/00/Trunk.toml"
```

## Step 4 · .gitignore

Build output and local datasets stay out of git; the lock file stays in.

`lessons/00/.gitignore` · copy the file

```text
--8<-- "lessons/00/.gitignore"
```

## Step 5 · Cargo.lock

The exact dependency versions, including the skrifa 0.40 pin that keeps a second font stack out of the wasm.

`lessons/00/Cargo.lock` · copy the file; never edit it by hand

## Step 6 · index.html

The page: a canvas, a hidden text field that opens the phone keyboard, a status line, and the scene prefetch.

`lessons/00/index.html` · copy the file

```html
--8<-- "lessons/00/index.html"
```

## Step 7 · src/lib.rs

The function the browser runs once the module has loaded; for now it only routes panics to the console.

`lessons/00/src/lib.rs` · type this, new file

```rust
--8<-- "lessons/00/src/lib.rs:entry"
```

Run `cargo check` in `lessons/00/`.

## Check

`cargo check` compiles every dependency once, which takes a few minutes, and then the crate itself in seconds. Nothing runs in the browser yet: `index.html` copies fonts, fixtures and a docs folder that later lessons add, so `trunk serve` comes later. If cargo cannot find `session_rust`, the folder is not four levels below the one that holds it.

## Next

[01 · First WebGPU frame](01-first-frame.md)
