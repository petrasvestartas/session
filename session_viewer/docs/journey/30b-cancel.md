# 30b · Cancel reads without accepting their late result

**Typing: 16–31 minutes.** [Estimate](typing-load.md).

Use `Rc<RefCell<ReadGate>>` to share a ticket gate between the command callback and read tasks. Keep mutable borrows short and release them before dispatching events.

A chosen file begins a ticket. Check it before reading; consume it once with `finish` after completion. Stale or cancelled tasks deliver neither bytes nor errors.

## Type

Continue from [Give a pending read an explicit ticket](30a-tickets.md). [Save or recover your work](recovery.md).

### 1. `src/file_input.rs`

Share the native gate through short interior mutable borrows.

<details>
<summary>Locate the existing block</summary>

```rust
use std::{cell::Cell, rc::Rc};
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30b-cancel-01.rs"
```

### 2. `src/file_input.rs`

Receive the same pending-read owner as the command callback.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn choose(event: &web_sys::Event, request: Rc<Cell<u64>>) {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30b-cancel-02.rs"
```

### 3. `src/file_input.rs`

Issue a bounded ticket; even an exhaustion error must be delivered after the input callback returns.

<details>
<summary>Locate the existing block</summary>

```rust
    let id = request.get() + 1;
    request.set(id);
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30b-cancel-03.rs"
```

### 4. `src/file_input.rs`

Avoid work for a ticket already superseded or cancelled before its queued task starts.

<details>
<summary>Locate the existing block</summary>

```rust
        if request.get() != id { return; }
        if file.size()
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30b-cancel-04.rs"
```

### 5. `src/file_input.rs`

Consume the current ticket before reporting a size refusal, releasing the RefCell borrow before dispatch.

<details>
<summary>Locate the existing block</summary>

```rust
            failure("This checkpoint accepts files up to 4 MiB");
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30b-cancel-05.rs"
```

### 6. `src/file_input.rs`

Only a current one-shot completion may deliver bytes or a read error.

<details>
<summary>Locate the existing block</summary>

```rust
        if request.get() != id { return; }
        let result = result.and_then(deliver);
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30b-cancel-06.rs"
```

### 7. `src/browser.rs`

Expose the initial row count through hidden diagnostics.

<details>
<summary>Locate the existing block</summary>

```rust
    let mut editor = Editor::default();
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30b-cancel-07.rs"
```

### 8. `src/browser.rs`

Offer read cancellation as a command rather than a keyboard feature shortcut.

<details>
<summary>Locate the existing block</summary>

```rust
            "Open",
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30b-cancel-08.rs"
```

### 9. `src/browser.rs`

Let the input callback and queued tasks share one native gate.

<details>
<summary>Locate the existing block</summary>

```rust
    let request = std::rc::Rc::new(std::cell::Cell::new(0));
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30b-cancel-09.rs"
```

### 10. `src/browser.rs`

An asynchronous read error gets its own history entry instead of rewriting a later command.

<details>
<summary>Locate the existing block</summary>

```rust
                panel.result(&message);
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30b-cancel-10.rs"
```

### 11. `src/browser.rs`

Revoke pending delivery without touching the scene or edit history.

<details>
<summary>Locate the existing block</summary>

```rust
            if line == "open" {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30b-cancel-11.rs"
```

### 12. `src/browser.rs`

Record asynchronous success separately from subsequent typed commands.

<details>
<summary>Locate the existing block</summary>

```rust
                        report("File imported. Undo removes the entire import.");
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30b-cancel-12.rs"
```

### 13. `src/browser.rs`

Distinguish a file result from a synchronous error on the current command.

<details>
<summary>Locate the existing block</summary>

```rust
                    report(error);
                    panel.result(error);
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30b-cancel-13.rs"
```

### 14. `src/browser.rs`

Update hidden diagnostics after every handled event.

<details>
<summary>Locate the existing block</summary>

```rust
show_projection(&editor.camera)
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30b-cancel-14.rs"
```

### 15. `src/browser.rs`

Keep scene count and projection in one diagnostic boundary.

<details>
<summary>Locate the existing block</summary>

```rust
fn show_projection(camera: &crate::camera::Camera) -> Result<(), JsValue> {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30b-cancel-15.rs"
```

### 16. `src/browser.rs`

Report rows independently of overlapping drawing pixels.

<details>
<summary>Locate the existing block</summary>

```rust
    canvas.set_attribute("data-projection", &format!("{:?}", camera.projection))
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30b-cancel-16.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Start `Open`, then type `Cancel Open` before its read completes. A late result must not enter the document or replace a newer command’s history entry.

**Verified checkpoint in Chrome.**

![Actual browser result: Cancel reads without accepting their late result.](../screenshots/journey/30b-cancel-browser.png)

[Verification scope](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Code explanation and diagram</summary>

Typed `Cancel Open` revokes ownership without editing the document. It does not physically abort `File.arrayBuffer`; its eventual result is ignored.

Append asynchronous feedback as an Open history entry so it cannot overwrite a newer command's result. A hidden object-count attribute lets Chrome detect duplicate insertion even when its pixels overlap perfectly.

Selected file → ticket-owned task → consume current ticket → deliver outcome; Cancel Open revokes ownership.

![The task keeps its ticket; Cancel Open revokes it; only a matching one-shot completion can deliver a result.](../illustrations/journey-30b.svg)

Does cancelling a read mean its promise stops running?

No. File.arrayBuffer has no abort operation here. Cancellation revokes permission to deliver its result. The task may finish later, but finish rejects its ticket and neither scene nor command feedback changes. A separate result entry also prevents a read outcome from rewriting a later typed command.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Hold a read in a debugger, type View Isometric and Cancel Open, then let the read finish. Predict why neither the object count nor command history should change when that cancelled task returns.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 30b-cancel
npm --prefix ../session_tests run course -- save 30b-cancel
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Production loader generations prevent stale work from mutating the active scene. This checkpoint expresses cancellation as delivery ownership; listener and resource lifetime cleanup remains a later lesson.

Chrome holds real File.arrayBuffer promises, types other commands, cancels reads and releases old completions. It checks both scene pixels and object count, and confirms an old failure cannot overwrite a newer request or command.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 30b-cancel
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
