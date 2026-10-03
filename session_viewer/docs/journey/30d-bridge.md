# 30d · Choose append or replace before opening the picker

**Typing: 27–53 minutes.** [Estimate](typing-load.md).

Add typed `Open Replace`, retaining `Open` for append. Opening either picker begins a ticket and stores its operation as a `Mode` enum.

The selected-file task captures that ticket and mode. `choose` reuses the pending ticket; a newer picker revokes earlier delivery before a file is chosen.

## Type

Continue from [Replace a document as one reversible change](30c-replace.md). [Save or recover your work](recovery.md).

### 1. `src/file_input.rs`

Capture a typed operation value with the selected file instead of consulting mutable state after reading.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn choose(event: &web_sys::Event, request: Rc<RefCell<crate::read_gate::ReadGate>>) {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30d-bridge-01.rs"
```

### 2. `src/file_input.rs`

A choice with no file revokes pending ownership without editing the scene.

<details>
<summary>Locate the existing block</summary>

```rust
    let Some(file) = input.files().and_then(|files| files.get(0)) else { return; };
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30d-bridge-02.rs"
```

### 3. `src/file_input.rs`

Use the ticket already issued by the Open command.

<details>
<summary>Locate the existing block</summary>

```rust
    let id = match request.borrow_mut().begin() {
        Ok(id) => id,
        Err(error) => {
            wasm_bindgen_futures::spawn_local(async move { failure(error); });
            return;
        }
    };
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30d-bridge-03.rs"
```

### 4. `src/file_input.rs`

Keep the captured mode beside the accepted bytes.

<details>
<summary>Locate the existing block</summary>

```rust
        let result = result.and_then(deliver);
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30d-bridge-04.rs"
```

### 5. `src/file_input.rs`

Include the selected operation in the private byte event.

<details>
<summary>Locate the existing block</summary>

```rust
fn deliver(buffer: JsValue) -> Result<(), JsValue> {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30d-bridge-05.rs"
```

### 6. `src/file_input.rs`

Use an explicit replace flag and byte array at the browser bridge.

<details>
<summary>Locate the existing block</summary>

```rust
    detail.set_detail(&js_sys::Uint8Array::new(&buffer));
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30d-bridge-06.rs"
```

### 7. `src/file_input.rs`

Translate checked browser payloads into ordinary editor actions.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn bytes(event: &web_sys::Event) -> Option<Vec<u8>> {
    let event = event.dyn_ref::<web_sys::CustomEvent>()?;
    let array = event.detail().dyn_into::<js_sys::Uint8Array>().ok()?;
    if array.length() as usize > crate::document::MAX_BYTES { return None; }
    Some(array.to_vec())
}
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30d-bridge-07.rs"
```

### 8. `src/browser.rs`

Offer replacement as a typed command.

<details>
<summary>Locate the existing block</summary>

```rust
            "Open",
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30d-bridge-08.rs"
```

### 9. `src/browser.rs`

Hold the picker operation in the event closure; each task copies it at selection.

<details>
<summary>Locate the existing block</summary>

```rust
    let request = std::rc::Rc::new(std::cell::RefCell::new(crate::read_gate::ReadGate::default()));
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30d-bridge-09.rs"
```

### 10. `src/browser.rs`

Revoke native picker cancellation and capture the operation when a file is selected.

<details>
<summary>Locate the existing block</summary>

```rust
        let action = if event.type_() == "change" {
            crate::file_input::choose(&event, std::rc::Rc::clone(&request));
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30d-bridge-10.rs"
```

### 11. `src/browser.rs`

Keep browser values outside the editor transaction.

<details>
<summary>Locate the existing block</summary>

```rust
            crate::file_input::bytes(&event).map(Action::Import)
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30d-bridge-11.rs"
```

### 12. `src/browser.rs`

A newer request revokes older work at picker opening, not only after file selection.

<details>
<summary>Locate the existing block</summary>

```rust
            } else if line == "open" {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30d-bridge-12.rs"
```

### 13. `src/browser.rs`

Remember which successful file action was committed before consuming it.

<details>
<summary>Locate the existing block</summary>

```rust
        if let Some(action) = action {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30d-bridge-13.rs"
```

### 14. `src/browser.rs`

Report the committed operation without overwriting another command.

<details>
<summary>Locate the existing block</summary>

```rust
                        report("File imported. Undo removes the entire import.");
                        panel.answer("Open", "File imported. Undo removes the entire import.");
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30d-bridge-14.rs"
```

### 15. `src/browser.rs`

Receive the native file-input cancellation event for the viewer lifetime.

<details>
<summary>Locate the existing block</summary>

```rust
    document.add_event_listener_with_callback("change", update.as_ref().unchecked_ref())?;
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30d-bridge-15.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Type `Open Replace` and choose `sample.pb`, then type `Undo`. The preceding document returns as one transaction. `Open Append` instead keeps existing objects.

**Verified checkpoint in Chrome.**

![Actual browser result: Choose append or replace before opening the picker.](../screenshots/journey/30d-bridge-browser.png)

[Verification scope](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Code explanation and diagram</summary>

The completion event carries a replace flag and `Uint8Array`. Validate shape and size, then translate to `Action::Import` or `Action::Replace`. Editor never receives browser values.

Picker cancellation revokes its ticket and appends an Open notice. Report append or replacement only after successful commit. A failed replacement retains the document; successful replacement is one Undo change.

Open or Open Replace → new ticket plus captured mode → file bytes → explicit editor action.

![The command starts a ticket and operation mode; the queued task captures both; its accepted bytes choose Import or Replace.](../illustrations/journey-30d.svg)

Why must a newer Open revoke old work before the new file is chosen?

Opening a new picker expresses a new request even if the user cancels it. If the ticket were issued only after choosing a file, an older pending read could still commit behind the newer picker. Begin at the command, capture its mode at selection, and consume that ticket before delivering an outcome.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Hold a previous read, open Open Replace, cancel the new picker and then let the old read finish. Predict why the old file must stay rejected despite the new picker having no chosen file.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 30d-bridge
npm --prefix ../session_tests run course -- save 30d-bridge
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Production scene replacement uses a generation started at request time and stages a whole scene before commit. This checkpoint applies that policy to the flat file picker; later command chapters align the full production vocabulary and loading paths.

Chrome opens a newer replacement picker while an old file promise is held, cancels the picker, then releases the old result. It checks that no rows arrive. It also verifies malformed replacement, successful replacement and pixel-identical Undo/Redo before capturing the replaced document.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 30d-bridge
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
