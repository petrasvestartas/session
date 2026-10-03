# 30 · Report the result that actually committed

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 15–30 minutes.** 29 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Keep failed reads and imports visible in command history without claiming success.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** File read result → browser event → validated editor action → success or error in the command dock.

**Before you finish, explain:** Why must import success be reported inside the successful action branch?

Our native import tests already prove malformed bytes leave the document intact. The browser feedback has a separate bug: after Editor::apply returns an error, the event handler still reports File imported. The final status therefore contradicts what happened.

Move success reporting into the successful Scene branch. Failed decoding keeps the old scene and records its error beside the command. There is no new history path.

![A read delivers bytes or an error; only a successful editor transaction reports that the file entered the scene.](../illustrations/journey-30.svg)

Asynchronous read errors need the same command-history route. The file adapter cannot borrow the panel held by the main event closure. Instead it dispatches viewer-file-error with a String detail. The main closure receives that event, reports the status and calls panel.result.

Oversized files take that route in the queued task, before reading. Dispatching another event from inside the currently borrowed input callback would re-enter the same FnMut closure and fail. Waiting until the task runs releases that borrow first. A rejected read promise takes it after the existing latest-read check, so an old read still cannot publish a late error over a newer choice. The next lessons replace the loose counter with explicit tickets and cancellation.

File status stays hidden while the command dock carries user feedback. A startup failure still exposes the HTML status because the GPU dock could not be created. A temporary read failure must not add a banner over the full-window drawing. Chrome checks the actual history text, unchanged scene pixels, preserved Redo, an oversize file and a rejected read promise.

## Type the change

Continue [Download the editable document from the command line](29d-save.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-30-feedback`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/browser.rs`

Keep a startup failure visible when the GPU dock could not be created; file errors stay in command history.

Find this exact block:

```rust
            if message.starts_with("Cannot") { let _ = status.remove_attribute("hidden"); }
```

Replace that block with:

```rust
--8<-- "journey/code/30-feedback-01.rs"
```

### 2. `src/file_input.rs`

Route the size refusal through the panel’s main event closure.

Find this exact block:

```rust
        super::browser::report("This checkpoint accepts files up to 4 MiB");
```

Replace that block with:

```rust
--8<-- "journey/code/30-feedback-02.rs"
```

### 3. `src/file_input.rs`

Deliver a read error only after the existing latest-read check.

Find this exact block:

```rust
            super::browser::report(&format!("Cannot read file: {error:?}"));
```

Replace that block with:

```rust
--8<-- "journey/code/30-feedback-03.rs"
```

### 4. `src/file_input.rs`

Pass asynchronous errors through a small custom event rather than borrowing the panel.

Find this exact block:

```rust
fn deliver(buffer: JsValue) -> Result<(), JsValue> {
```

Replace that block with:

```rust
--8<-- "journey/code/30-feedback-04.rs"
```

### 5. `src/browser.rs`

Receive read failures in the same closure that owns command history.

Find this exact block:

```rust
        } else if event.type_() == "viewer-file" {
```

Replace that block with:

```rust
--8<-- "journey/code/30-feedback-05.rs"
```

### 6. `src/browser.rs`

A successful transaction is the only branch that may announce an imported file.

Find this exact block:

```rust
                Ok(Change::Scene) => renderer.set_scene(&editor.scene, editor.selected),
```

Replace that block with:

```rust
--8<-- "journey/code/30-feedback-06.rs"
```

### 7. `src/browser.rs`

Remove the unconditional success that overwrote decoding errors.

Find this exact block:

```rust
        if event.type_() == "viewer-file" {
            report("File imported. Undo removes the entire import.");
        }
```

Delete this block.

### 8. `src/browser.rs`

Keep the error event connected for the viewer lifetime.

Find this exact block:

```rust
    window.add_event_listener_with_callback("viewer-file", update.as_ref().unchecked_ref())?;
```

Replace that block with:

```rust
--8<-- "journey/code/30-feedback-08.rs"
```

### 9. `src/file_input.rs`

Move the size refusal out of the currently borrowed input callback.

Find this exact block:

```rust
    if file.size() > crate::document::MAX_BYTES as f64 {
        failure("This checkpoint accepts files up to 4 MiB");
        return;
    }
```

Delete this block.

### 10. `src/file_input.rs`

Check size in the queued task before reading; error delivery can now enter the main callback safely.

Find this exact block:

```rust
    wasm_bindgen_futures::spawn_local(async move {
```

Replace that block with:

```rust
--8<-- "journey/code/30-feedback-10.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Open your sample, Example Box, Select Next six times, Move -0.2,-0.8,0.1, View Isometric and Fit. Try opening a malformed file: the error must stay in command history, the drawing must stay unchanged, and Redo must remain available after an undone edit. A valid Open afterward still succeeds.

**Actual Chrome screenshot.**

![Actual browser result: Report the result that actually committed.](../screenshots/journey/30-feedback-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
cargo test --lib --locked -j4
```

## Try one small experiment

Temporarily move the success report back outside the action match. Predict the final status after malformed bytes, reproduce the contradiction, then restore the successful-branch report.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

Receiving bytes does not mean they decoded or entered the scene. Editor::apply can reject a file while preserving scene, selection and history. Only a successful Scene change proves the import committed; an unconditional message after the match would erase the error.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 30-feedback
npm --prefix ../session_tests run course -- save 30-feedback
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

The production loader stages work and reports failure while keeping the last valid scene visible. This lesson separates read delivery from a committed document change; replacement and cancellation follow.

[Validation status and course release](release.md).
