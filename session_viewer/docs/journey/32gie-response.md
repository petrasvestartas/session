# 32gie · Deliver restored edit and Save results to the dock

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 14–27 minutes.** 23 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Route validated scene edits and original-precision downloads back to the captured command history.

**Follow:** Current completion → captured command name → complete → Changed or Saved → dock answer.

The browser receiver now calls complete instead of hydrating only the body. It records the command name from the captured intent before consuming the reply. Changed synchronizes the renderer; Saved downloads the bytes produced by the original-precision snapshot. A stale None does nothing. Errors use the same original command name.

Explicit Reload Sources still uses None intent and keeps its existing success message. save_result converts either the browser download failure or its success into one dock message. Actual automatic commands are connected next; this checkpoint prepares their result handling.

![Deliver the captured command result](../illustrations/journey-32gie.svg)

## Type the change

Continue from [Validate restoration before replaying the command](32gid-complete.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-32gie-response` (from `session_viewer`).

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

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Build and run the viewer. Explicit `Reload Sources` still restores without an Undo step; the browser can now consume scene or Save replies. Automatic command routing follows next.

**Verified checkpoint in Chrome.**

![Actual browser result: Deliver restored edit and Save results to the dock.](../screenshots/journey/32gie-response-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Use the current command field as the completion label. Type a camera command while a fetch is held and explain why that label would attach the result to the wrong operation.

</details>

## Explain the change

Why is the command name read from the reply before completion consumes it?

<details>
<summary>Compare your explanation</summary>

The reply owns the operation submitted before fetching. Later selection and typing do not identify that operation. Completion consumes its source bodies and intent, so capture the name first.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32gie-response
npm --prefix ../session_tests run course -- save 32gie-response
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

Asynchronous result ownership follows the submitted operation; neither the latest input nor later selection renames the completed command.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

Native completion checks still exercise captured Move/Delete/Save and stale or failed restoration. Chrome exercises explicit restoration through this new receiver, including the inherited fetch and cancellation checks. It does not yet claim an automatic browser edit.

Chrome checks explicit restoration through the result-aware receiver. Native tests verify captured edit and Save replies; browser command-triggered restoration follows next.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 32gie-response
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
