# 00 · Empty project to a WASM message

Five files and one command. At the end, the browser shows a line of text written by Rust. No GPU yet.

![Four tools and four artefacts: cargo produces a .wasm a browser cannot load on its own, wasm-bindgen writes the JavaScript that can, Trunk assembles the page around it, and the browser runs start().](illustrations/toolchain.svg)

`cargo` compiles to `.wasm`, Trunk wraps and serves it, the browser runs `start()`.

## Make the folder

```sh
cd ~/session            # the folder that holds session_rust
mkdir -p session_view/src session_view/.cargo
cd session_view
```

Every command below runs here.

## Step 1 · `Cargo.toml`

The crate's name and every dependency it will ever need, listed once.

`lessons/00/Cargo.toml` · 72 lines · type this, new file

```toml
--8<-- "lessons/00/Cargo.toml"
```

## Step 2 · `.cargo/config.toml`

Makes the browser the default target, and `cargo xtest` runs tests natively.

`lessons/00/.cargo/config.toml` · 11 lines · type this, new file

```toml
--8<-- "lessons/00/.cargo/config.toml"
```

## Step 3 · `Trunk.toml`

Release build, page-relative assets, and the dev server address.

`lessons/00/Trunk.toml` · 8 lines · type this, new file

```toml
--8<-- "lessons/00/Trunk.toml"
```

## Step 4 · `Cargo.lock`

Cargo's exact dependency versions; copy it, never edit it.

```sh
cargo generate-lockfile
```

## Step 5 · `index.html`

The page: Trunk fills the `rust` link, Rust writes into `status`.

`lessons/00/index.html` · 11 lines · type this, new file

```html
--8<-- "lessons/00/index.html"
```

## Step 6 · `src/lib.rs`

Runs when the module loads and writes one line into the page.

`lessons/00/src/lib.rs` · 18 lines · type this, new file

```rust
--8<-- "lessons/00/src/lib.rs"
```

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
- The page stays on *Loading WASM*: open `localhost:8780`, not the file, and wait for the build.

## What changed

```text
lessons/00/session_viewer/
├── .cargo/
│   └── config.toml  +
├── src/
│   └── lib.rs  +
├── Cargo.toml  +
├── Trunk.toml  +
└── index.html  +
```

`+` new in this lesson · `~` changed in this lesson

Every file at this point: `lessons/00/`.

## Next

[01 · First WebGPU frame](01-first-frame.md): one triangle.

## Expected viewer result

Checkpoint 00 in Chrome: the status line written by Rust.

[![Full viewer result for 00 environment](screenshots/00.png)](screenshots/00.png)
