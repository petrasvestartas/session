# 32fd · Attach one origin to an imported document

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 22–44 minutes.** 36 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Share one geometry-free origin across imported rows and history.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Share one geometry-free origin across imported rows and history..

**Before you finish, explain:** Does retaining an Origin after Close keep the imported Session alive?

Construct an Origin after decoding and validation, before moving the protobuf into the kernel Session. Every prepared row shares the same Rc<Origin> through its Source. A second import gets a different Origin even when its file bytes are identical.

The native check records Weak observers, retains only the Origin, and closes the editor. The Session and kernel value must disappear while the original header remains readable. Another check follows one shared origin through Move and Undo.

Chrome exposes origin identity and the byte fingerprint in a hidden attribute. It checks that three rows of one import share an origin, duplicate imports have different origins but the same version, and history preserves them. No reload location or unload command exists yet.

![Share one geometry-free origin across imported rows and history.](../illustrations/journey-32fd.svg)

## Type the change

Continue [Record a reload version without retaining geometry](32fc-version.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-32fd-origin`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/document.rs`

Retain reload metadata separately from the kernel Session.

Find this exact block:

```rust
    pub guid: String,
```

Replace that block with:

```rust
--8<-- "journey/code/32fd-origin-01.rs"
```

### 2. `src/document.rs`

Record accepted original bytes and a geometry-free header before kernel construction.

Find this exact block:

```rust
    let document = Rc::new(Session::from_proto(message).map_err(|_| "Cannot construct session")?);
```

Replace that block with:

```rust
--8<-- "journey/code/32fd-origin-02.rs"
```

### 3. `src/document.rs`

Share one import origin among all its source rows.

Find this exact block:

```rust
            document: Rc::clone(&document), guid: mesh.guid().to_owned(),
```

Replace that block with:

```rust
--8<-- "journey/code/32fd-origin-03.rs"
```

### 4. `src/lib.rs`

Check source ownership independently of drawing.

Find this exact block:

```rust
pub mod origin;
```

Replace that block with:

```rust
--8<-- "journey/code/32fd-origin-04.rs"
```

### 5. `src/origin_tests.rs`

Observe true kernel expiration while retaining only header/version metadata.

Create the file and type:

```rust
--8<-- "journey/code/32fd-origin-05.rs"
```

### 6. `src/browser.rs`

Inspect shared import identity and exact versions without adding controls.

Find this exact block:

```rust
    canvas.set_attribute("data-row-metadata", &serde_json::to_string(&metadata)
```

Replace that block with:

```rust
--8<-- "journey/code/32fd-origin-06.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Open the same specimen twice and compare import IDs and fingerprints. Move and Undo without changing either. Run the Weak ownership checks.

**Actual Chrome screenshot.**

![Actual browser result: Attach one origin to an imported document.](../screenshots/journey/32fd-origin-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Clone Source instead of only Origin in the ownership check and predict which Weak observations remain alive.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

No. It owns copied header values and a version, not the Session Rc or any kernel mesh Rc. Weak checks can prove the Session and its meshes are dropped while the retained origin remains readable.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32fd-origin
npm --prefix ../session_tests run course -- save 32fd-origin
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Origin identity is separate from saved row GUID and original source GUID. This prevents a later completion for one duplicate import from being adopted by another.

[Validation status and course release](release.md).
