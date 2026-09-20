# 00 · Empty project to a WASM message

<!-- locator: off -->

Five files and one command. At the end, the browser shows a line of text written by Rust. No GPU yet.

![Four tools and four artefacts: cargo produces a .wasm a browser cannot load on its own, wasm-bindgen writes the JavaScript that can, Trunk assembles the page around it, and the browser runs start().](illustrations/toolchain.svg)

`cargo` compiles the Rust to a `.wasm` file. `wasm-bindgen` writes the JavaScript that loads it; Trunk runs it for you. `trunk` serves the page. The browser runs `start()`.

## Make the folder

```sh
cd ~/session            # the folder that holds session_rust
mkdir -p session_view/src session_view/.cargo
cd session_view
```

Every command below runs here.

<!-- step-status: start -->

**Does it compile yet?** `cargo check` passes after step 6; steps 1–5 fail and build again at step 6.

<!-- step-status: end -->

## Step 1 · `Cargo.toml`

What the crate is and what it uses. This lesson needs only `wasm-bindgen`, `web-sys` and `console_error_panic_hook`; the rest is for later lessons, listed once so the file never changes. Keep the package name `session_viewer` whatever the folder is called.

<!-- file: 00 session_viewer/Cargo.toml type -->

## Step 2 · `.cargo/config.toml`

Makes the browser the default target of every `cargo` command, so you never type `--target`. `xtest` runs the tests natively; wasm has no test runner.

<!-- file: 00 session_viewer/.cargo/config.toml type -->

## Step 3 · `Trunk.toml`

Optimized build, assets relative to the page, the dev server address. Lessons pass `--port 8780` on the command line so a maintained viewer on 8770 can run at the same time.

<!-- file: 00 session_viewer/Trunk.toml type -->

## Step 4 · `Cargo.lock`

Cargo picks an exact version of every dependency and records it here. Never typed, never edited.

```sh
cargo generate-lockfile
```

## Step 5 · `index.html`

The page. `<link data-trunk rel="rust">` is where Trunk puts the loader. `<output id="status">` is the element Rust overwrites.

<!-- file: 00 session_viewer/index.html type -->

## Step 6 · `src/lib.rs`

`#[wasm_bindgen(start)]` runs when the module has loaded. It turns panics into console messages, finds the element by id, writes the text and marks it with `data-checkpoint`.

<!-- file: 00 session_viewer/src/lib.rs type -->

## Check

```sh
cargo check --lib
trunk serve --port 8780
```

The first check compiles every dependency and takes a few minutes. Open <http://localhost:8780/>: the page shows **Checkpoint 00: Rust/WASM ready**. Stop with Ctrl+C.

![Checkpoint 00 in Chrome: the status line written by Rust.](screenshots/00.png)

If it fails:

- *failed to load manifest for dependency `session_rust`*: the folder is not next to `session_rust`.
- An error naming a crate or feature: compare your `Cargo.toml` with step 1.
- The page stays on *Loading WASM*: you opened the file from disk instead of `localhost:8780`, or the build has not finished. A misspelled id panics instead, and the console shows the `expect` message.

## What changed

<!-- tree: 00 session_viewer -->

Every file at this point: [source at checkpoint 00](../lessons/00/index.md).

## Next

[01 · First WebGPU frame](01-first-frame.md): one triangle.

## Expected viewer result

Checkpoint 00 in Chrome: the status line written by Rust.

[![Full viewer result for 00 environment](screenshots/00.png)](screenshots/00.png)
