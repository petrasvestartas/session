# 32gif · Automatically restore sources for Move, Delete and Save

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 25–49 minutes.** 36 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Connect cold-source edits and Save to captured reload and validated replay without requiring Reload Sources first.

**Follow:** Typed command → capture original intent → required cold keys → fetch → validated completion → one edit or download.

restore_for finds only the captured operation’s required cold source keys. Move and Delete need the original target’s source; Save needs all active cold sources. It cancels older pending work, returns false for a warm operation, and otherwise starts the existing abortable flight with Some(intent).

The shared action path captures Move/Delete before applying them. Its Result<Option<Change>, String> distinguishes an immediate change (Some), a waiting operation (None), and a failure (Err). map(Some) wraps an immediate editor result; and_then carries an earlier error forward without applying the action. A pending restoration leaves drawing, placement and edit history untouched. Camera and selection commands still run immediately; completed replay uses the captured target and the current placement, while preserving the later view and selection. Save uses the same decision and downloads only after completion has restored every required source.

Zero Move and Delete with no selection request no fetch. Warm commands retain their normal behavior. Close, Undo, Redo, source unload and document replacement keep the cancellation taught earlier. A newer real edit or Save supersedes an older pending intent even when the newer operation is warm.

![Automatic cold-source commands](../illustrations/journey-32gif.svg)

The final proof placement moves the post clear of the beam. Keeping two front faces in exactly the same plane can produce depth competition; this demonstration separates the solids instead of claiming the later rendering-quality lessons are already implemented.

## Type the change

Continue from [Deliver restored edit and Save results to the dock](32gie-response.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-32gif-auto` (from `session_viewer`).

### 1. `src/browser_reload.rs`

A new edit supersedes older pending work. Warm edits need no fetch; cold edits capture their required keys and operation.

Find this exact block:

```rust
pub fn start(shared: Shared, keys: Vec<ReloadKey>, delivery: Delivery) -> Result<(), String> {
```

Replace that block with:

```rust
--8<-- "journey/code/32gif-auto-01.rs"
```

### 2. `src/browser.rs`

Save reloads every active cold source before producing its original-precision download; warm Save remains immediate.

Find this exact block:

```rust
                match crate::document::snapshot(&editor.scene) {
                    Ok(bytes) => match crate::file_output::download(&bytes) {
                        Ok(()) => panel.result(&format!("Saved editable document ({} bytes)", bytes.len())),
                        Err(error) => panel.result(&format!("Save failed: {error:?}")),
                    },
                    Err(error) => panel.result(error),
                }
```

Replace that block with:

```rust
--8<-- "journey/code/32gif-auto-02.rs"
```

### 3. `src/browser.rs`

Capture Move/Delete before fetching; camera and selection commands remain immediate while pending work keeps its original target.

Find this exact block:

```rust
            match editor.apply(action) {
                Ok(Change::Scene) => {
```

Replace that block with:

```rust
--8<-- "journey/code/32gif-auto-03.rs"
```

### 4. `src/browser.rs`

A deferred command has no scene change yet; failures retain normal dock reporting.

Find this exact block:

```rust
                Ok(Change::View) => {}
                Err(error) => {
                    report(error);
                    if event.type_() == "viewer-file" { panel.answer("Open", error); }
                    else { panel.result(error); }
                }
```

Replace that block with:

```rust
--8<-- "journey/code/32gif-auto-04.rs"
```

### 5. `src/browser.rs`

Expose read-only placement, selection and camera values so Chrome checks the original target and later view exactly. These values add no controls.

Find this exact block:

```rust
    canvas.set_attribute("data-object-count", &editor.scene.objects().len().to_string())?;
```

Replace that block with:

```rust
--8<-- "journey/code/32gif-auto-05.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Open `sample.pb`, select an object and type `Unload Sources`, then `Move 0.25,0,0.15`. Move restores its source automatically and runs once. `Undo` returns the object to its previous placement.

**Verified checkpoint in Chrome.**

![Actual browser result: Automatically restore sources for Move, Delete and Save.](../screenshots/journey/32gif-auto-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Hold a source response, submit Move, select another object and orbit, then release it. Explain why replay must use the captured object identity but preserve the later selection and camera.

</details>

## Explain the change

Which commands should keep working while source restoration is pending?

<details>
<summary>Compare your explanation</summary>

Camera and selection commands execute immediately. The pending operation retains its original target and arguments. A new real edit or Save supersedes earlier pending work; Close, Undo, Redo, Unload Sources and replacement already cancel its authority.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32gif-auto
npm --prefix ../session_tests run course -- save 32gif-auto
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

Automatic editable-source restoration obeys command ownership, document-context revocation, original targets, current placement, original precision and ordinary history boundaries.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

Chrome uses held real source responses to check original-target Move and Delete, later selection and camera, one-step Undo, automatic Save download and cancellation. These controlled waits test ordering, not phone performance. Exact source-coordinate preservation remains covered by native Save/replay checks; the following acceptance lesson broadens browser failures and precision.

Visible Chrome types real cold-source Move/Delete/Save commands, holds fetch responses, changes selection and camera, then checks replay, download, Undo and cancellation. Controlled waits establish ordering, not device speed.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 32gif-auto
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
