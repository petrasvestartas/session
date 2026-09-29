# 23 · Keep the document behind the picture

**Plan about 5–8 hours.** 258 lines to type, including comments and blank lines. Allow time to read, predict and experiment; this is an estimate, not a deadline.

**Today:** Import a real mesh-session file, keep its source identity, and undo the whole import as one action.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** File picker → bytes → validated session → prepared display meshes → one history transaction → GPU upload.

**Before you finish, explain:** Why do we retain the session after creating the arrays that the renderer needs?

So far every object was born inside our example. Now let a real file enter. Think of the source session as the labelled drawing in a folder, and the GPU mesh as the picture projected on a screen. We need both: the screen is quick to draw, while the folder remembers what the object means.

A .pb file contains protobuf bytes, not Rust source. The geometry kernel already supplies the encoder and decoder. You will type a tiny specimen builder and run it to create a real file; there is no binary content to copy by hand.

The new boundary has three steps: read bytes, prepare a complete import, then commit it. Nothing enters the scene during decoding. If the third mesh is broken, the first two do not sneak into the document. Our existing History already knows how to undo one committed change.

This checkpoint accepts small, flat sessions containing triangle or quad meshes with object colours. Other geometry, placements, instances and face holes receive an explicit error; we will add them as their rendering lessons arrive. This is a stage in the full viewer course, not its final file support.

![A source session stays alongside its display meshes; validation happens before the history transaction.](../illustrations/journey-23.svg)

`Rc<Session>` means several displayed objects can share one session without making copies. `Source` pairs that shared session with a source GUID. The local ObjectId answers “which row in this viewer?”; the GUID answers “which mesh in this source document?”

`Cell<u64>` holds one changeable request number; `Rc` lets the callback and the waiting read share that number. File reading pauses at `await`, so the file adapter sends a completion event instead of holding a mutable editor borrow while waiting. A request number rejects an old completion after a newer file was chosen. The editor remains owned by the same callback you already understand.

## Type the change

Continue [Give the keyboard a place to work](22-shortcuts.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-23-import`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/document.rs`

Decode into a protobuf record before constructing kernel geometry. Validate the subset we can draw, then prepare every display mesh before changing the scene. Source keeps the complete session and its original mesh GUID.

Create the file and type:

```rust
--8<-- "journey/code/23-import-01.rs"
```

### 2. `src/specimen.rs`

Build a small three-piece frame with the real kernel. This is our file specimen, not a second renderer or a new file format.

Create the file and type:

```rust
--8<-- "journey/code/23-import-02.rs"
```

### 3. `examples/sample.rs`

Write the specimen as an actual session file. You will choose this file in the browser.

Create the file and type:

```rust
--8<-- "journey/code/23-import-03.rs"
```

### 4. `src/document_tests.rs`

Check data preservation, whole-file undo, repeated imports and error atomicity. Broken files must fail before kernel construction or document history changes.

Create the file and type:

```rust
--8<-- "journey/code/23-import-04.rs"
```

### 5. `src/lib.rs`

Register the reader, specimen and native checks.

Find this exact block:

```rust
pub mod shortcuts;
```

Replace that block with:

```rust
--8<-- "journey/code/23-import-05.rs"
```

### 6. `src/scene.rs`

An imported display object remembers the document and source GUID it came from. Demo objects have no source document.

Find this exact block:

```rust
    pub mesh: Rc<Mesh>,
```

Replace that block with:

```rust
--8<-- "journey/code/23-import-06.rs"
```

### 7. `src/scene.rs`

Keep the existing insertion path for demo objects.

Find this exact block:

```rust
self.objects.push(Object { id, mesh: Rc::new(mesh) });
```

Replace that block with:

```rust
--8<-- "journey/code/23-import-07.rs"
```

### 8. `src/scene.rs`

All imported rows share one retained source session. History will wrap this entire insertion in one transaction.

Find this exact block:

```rust
    pub fn remove(&mut self, id: ObjectId) -> bool {
```

Replace that block with:

```rust
--8<-- "journey/code/23-import-08.rs"
```

### 9. `src/editor.rs`

Make file import an ordinary document action.

Find this exact block:

```rust
    Pick([f32; 2]),
```

Replace that block with:

```rust
--8<-- "journey/code/23-import-09.rs"
```

### 10. `src/editor.rs`

Prepare first, commit second. A decode or mesh error cannot clear redo or leave half an import in the scene.

Find this exact block:

```rust
            Action::AddBox => {
```

Replace that block with:

```rust
--8<-- "journey/code/23-import-10.rs"
```

### 11. `Cargo.toml`

Use the kernel’s protobuf implementation and the already pinned JavaScript bindings directly.

Find this exact block:

```toml
console_error_panic_hook = "=0.1.7"
```

Replace that block with:

```toml
--8<-- "journey/code/23-import-11.toml"
```

### 12. `Cargo.toml`

Enable the file picker, asynchronous bytes and the browser message that delivers a completed read.

Find this exact block:

```toml
"HtmlElement"
```

Replace that block with:

```toml
--8<-- "journey/code/23-import-12.toml"
```

### 13. `src/file_input.rs`

Read asynchronously without borrowing Editor across await. A numbered request prevents a slow earlier file from replacing a newer choice. Deliver bytes to the callback that already owns Editor.

Create the file and type:

```rust
--8<-- "journey/code/23-import-13.rs"
```

### 14. `src/lib.rs`

Keep browser file APIs out of native builds.

Find this exact block:

```rust
mod browser;
```

Replace that block with:

```rust
--8<-- "journey/code/23-import-14.rs"
```

### 15. `src/browser.rs`

Retain only a request number across asynchronous file reads. The editor itself still has one owner.

Find this exact block:

```rust
    let mut gesture = Gesture::default();
```

Replace that block with:

```rust
--8<-- "journey/code/23-import-15.rs"
```

### 16. `src/browser.rs`

File selection starts a read; its later completion becomes Import. Both events enter the existing callback.

Find this exact block:

```rust
        let action = if event.type_() == "click" {
```

Replace that block with:

```rust
--8<-- "journey/code/23-import-16.rs"
```

### 17. `src/browser.rs`

Announce success only after the editor accepted every mesh and committed the transaction.

Find this exact block:

```rust
        if resize(&browser_window, &pointer_canvas, &surface, &mut config, &mut renderer, &mut editor) {
```

Replace that block with:

```rust
--8<-- "journey/code/23-import-17.rs"
```

### 18. `src/browser.rs`

Register file choice and read completion alongside existing input.

Find this exact block:

```rust
    document.add_event_listener_with_callback("click", update.as_ref().unchecked_ref())?;
```

Replace that block with:

```rust
--8<-- "journey/code/23-import-18.rs"
```

### 19. `index.html`

Expose a labelled native file picker. The extension is a hint; the reader still validates the actual bytes.

Find this exact block:

```html
    <legend>View</legend>
```

Replace that block with:

```html
--8<-- "journey/code/23-import-19.html"
```

## Run and look

After typing the manifest, run this from `session_viewer` to select the fixed dependency versions. It updates only Cargo.lock and preserves the previous lock:

```sh
npm --prefix ../session_tests run course -- dependencies 23-import
```

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

In a second terminal inside workspace/journey, run `cargo run --example sample --locked --target x86_64-unknown-linux-gnu -j4` to create sample.pb (use your native target on another platform). Choose that file with Open mesh session, then Isometric. The three orange pieces should join the existing triangles. Undo once: all three disappear together. Redo: all three return. Choose the same file again: it imports a second independent set. Try choosing a text file renamed to .pb: the scene and its undo history must stay intact.

**Actual Chrome screenshot.**

The file picker imports all three orange pieces. The browser check removes the entire import with one Undo, restores it with Redo, and then chooses Isometric.

![Actual browser result: Keep the document behind the picture.](../screenshots/journey/23-import-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
cargo test --lib --locked -j4
```

## Try one small experiment

Change the specimen’s top beam width from 1.8 to 2.2, regenerate sample.pb, and import it. You have changed the source, not the renderer. Undo the import, restore 1.8, and regenerate. Then explain why the same source GUID may occur in two imports while their local ObjectIds must differ.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

The GPU arrays contain positions, colours and indices. They do not retain the session name, source mesh identities, tree or graph. Each imported object keeps a source reference: the complete session plus its original mesh GUID. Its local ObjectId remains separate, so opening the same file twice does not confuse selection or undo.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 23-import
npm --prefix ../session_tests run course -- save 23-import
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

The maintained viewer keeps source sessions in FileDoc and builds display data from them. Its reader also handles hierarchy, placement, CAD, points, curves and large files. This first reader intentionally refuses those cases. Future lessons extend the retained document boundary instead of trying to recover lost information from GPU buffers.

[Validation status and course release](release.md).
