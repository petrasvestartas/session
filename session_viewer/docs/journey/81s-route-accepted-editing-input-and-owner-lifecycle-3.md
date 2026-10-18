# 16.1.27 · Restore source selection when an edit is cancelled

**Chapter 16 · Geometry editing and measurement · Section 16.1 · Projection and Split inputs** · [Contents](contents.md)

**Typing: 7–13 minutes.** [Estimate](typing-load.md).

Cancellation restores the selection captured by projection, extension or gather. Drop the draft and its previews, then report whether the GPU preview needs rebuilding. This completes the owner lifecycle established in [the input-routing lesson](81q-route-accepted-editing-input-and-owner-lifecycle-1.md).

## Type

Continue from [Finish a pending edit through one commit path](81r-route-accepted-editing-input-and-owner-lifecycle-2.md). [Save or recover your work](recovery.md).

### 1. `src/editor.rs`

Restore captured selection and release preview ownership on Cancel.

<details>
<summary>Locate the existing block</summary>

```rust
            Action::ToolPoint(point) => return self.place_tool_point(point),
            Action::ToolHover(point) => { if let Some(draft) = self.draft.as_mut() { draft.snapped=None; draft.hover(point)?; } return self.update_edit_preview(); }
            Action::CancelTool => { if let Some(before)=self.draft.as_ref().and_then(|d|d.gather.as_ref()).map(|g|g.before.clone()){self.select_objects(before,false);}let changed=self.edit_preview.take().is_some()|self.transform_preview.take().is_some(); self.draft=None;self.plane_menu=false;self.plane_only=false; self.tool_status="Tool cancelled.".into(); return Ok(if changed { Change::Preview } else { Change::View }); }
            Action::Picked(id) => self.select_objects(id, false),
            Action::PickedMany { target, additive } => {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/81s-route-accepted-editing-input-and-owner-lifecycle-3-01.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.1.27
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.



**Visual review: issue found.** This inherited network reference has faint or interrupted original input curves. Its aggregate pixel checks do not establish curve continuity or contrast. These are input curves, not a full NURBS isocurve display; visual-quality acceptance is incomplete.

[Visual review findings](visual-review.md).

### Recorded visual result

**Previous native render — current verification pending.**

The original network and four-corner patch remain visible.

![Native renderer output for restore source selection when an edit is cancelled.](../screenshots/journey/80d-exercise-networks-corner-retries-and-exact-original-archives.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.4.5 · Exercise networks corner retries and exact original archives](80d-exercise-networks-corner-retries-and-exact-original-archives.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



restore source selection when an edit is cancelled.



Which selection should cancellation restore?

The selection captured when projection, extension or gather began. Drop the draft and temporary previews before rebuilding their display state.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Begin an edit with selected originals, create a preview and cancel. Compare the restored source selection and confirm that temporary highlights disappear.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.1.27
npm --prefix ../session_tests run course -- save 16.1.27
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The preceding network/patch contract remains authored and unrun. The final editing contract adds projected originals, atomic Three Point retry, interval extension and active distance/cancellation. Actual browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 16.1.27
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
