# 32gie · Deliver restored edit and Save results to the dock

**Typing: 14–27 minutes.** [Estimate](typing-load.md).

Connect completed source replies to drawing and Save downloads. Report success or failure under the original command name.

## Type

Continue from [Validate restoration before replaying the command](32gid-complete.md). [Save or recover your work](recovery.md).

### 1. `src/edit_intent.rs`

Name the captured operation independently of the current input field or selection.

<details>
<summary>Locate the existing block</summary>

```rust
impl Intent {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32gie-response-01.rs"
```

### 2. `src/browser.rs`

Consume validated edit or Save results and associate success or failure with the original command.

<details>
<summary>Locate the existing block</summary>

```rust
            if let Some(reply) = reload_delivery.borrow_mut().take() {
                match reply.result.and_then(|values| editor.hydrate(values).map_err(str::to_owned)) {
                    Ok(true) => {
                        renderer.set_scene(&editor.scene, editor.selected);
                        let message = "Editable sources restored; display retained.";
                        report(message); panel.answer("Reload Sources", message);
                    }
                    Ok(false) => {}
                    Err(error) => { report(&error); panel.answer("Reload Sources", &error); }
                }
            }
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32gie-response-02.rs"
```

### 3. `src/browser.rs`

Keep download handling shared by immediate and restored Save results.

<details>
<summary>Locate the existing block</summary>

```rust
pub async fn run() -> Result<(), JsValue> {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32gie-response-03.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Build and run the viewer. Explicit `Reload Sources` still restores without an Undo step; the browser can now consume scene or Save replies. Automatic command routing follows next.

**Verified checkpoint in Chrome.**

![Actual browser result: Deliver restored edit and Save results to the dock.](../screenshots/journey/32gie-response-browser.png)

[Verification scope](release.md).

Run the state checks on your computer from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked --target host-tuple -j4
```

<details>
<summary>Code explanation and diagram</summary>

The browser receiver now calls complete instead of hydrating only the body. It records the command name from the captured intent before consuming the reply. Changed synchronizes the renderer; Saved downloads the bytes produced by the original-precision snapshot. A stale None does nothing. Errors use the same original command name.

Explicit Reload Sources still uses None intent and keeps its existing success message. save_result converts either the browser download failure or its success into one dock message. Actual automatic commands are connected next; this checkpoint prepares their result handling.

Current completion → captured command name → complete → Changed or Saved → dock answer.

![Deliver the captured command result](../illustrations/journey-32gie.svg)

Why is the command name read from the reply before completion consumes it?

The reply owns the operation submitted before fetching. Later selection and typing do not identify that operation. Completion consumes its source bodies and intent, so capture the name first.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Use the current command field as the completion label. Type a camera command while a fetch is held and explain why that label would attach the result to the wrong operation.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 32gie-response
npm --prefix ../session_tests run course -- save 32gie-response
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Asynchronous result ownership follows the submitted operation; neither the latest input nor later selection renames the completed command.

Native completion checks still exercise captured Move/Delete/Save and stale or failed restoration. Chrome exercises explicit restoration through this new receiver, including the inherited fetch and cancellation checks. It does not yet claim an automatic browser edit.

Chrome checks explicit restoration through the result-aware receiver. Native tests verify captured edit and Save replies; browser command-triggered restoration follows next.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 32gie-response
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
