# 16.1.23 · Connect boundary picks repeated end clicks and cancellation

**Chapter 16 · Geometry editing and measurement · Section 16.1 · Projection and Split inputs** · [Contents](contents.md)

**Typing: 28–55 minutes.** [Estimate](typing-load.md).

Keep boundary picks outside ordinary selection. Commit each checked end click once, then refresh that source for another click.

## Type

Continue from [Connect checked projection to one editor transaction](81n-connect-checked-projection-to-one-editor-transaction.md). [Save or recover your work](recovery.md).

### 1. `src/editor.rs`

Connect complete source-editing methods to one checked history transaction.

<details>
<summary>Locate the existing block</summary>

```rust
    }
    fn project_points(&mut self,added:&[[f64;3]])->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.project.clone()).ok_or("No projection tool is running")?;let plane=next.accept(added).map_err(|message|self.primitive_failure(message))?;if let Some(plane)=plane{return self.commit_project(next,plane)}let draft=self.draft.as_mut().unwrap();draft.points=next.points.clone();draft.project=Some(next);draft.cursor=None;draft.snapped=None;self.tool_status=draft.prompt();Ok(Change::View)}
    fn tool_text(&mut self,text:&str) -> Result<Change,&'static str> {
        if self.draft.as_ref().is_some_and(|d|d.gather.is_some()&&d.plane_points.is_none()){return self.gather_text(text);}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/81o-connect-boundary-picks-repeated-end-clicks-and-cancellation-01.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.1.23
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

![Native renderer output for connect boundary picks repeated end clicks and cancellation.](../screenshots/journey/80d-exercise-networks-corner-retries-and-exact-original-archives.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.4.5 · Exercise networks corner retries and exact original archives](80d-exercise-networks-corner-retries-and-exact-original-archives.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



temporary picks → fresh hover → one replacement → repeated click.

![Connect boundary picks repeated end clicks and cancellation.](../illustrations/81o-connect-boundary-picks-repeated-end-clicks-and-cancellation.svg)

How many Undo steps do two successful end clicks make?

Two. Each click changes one original in its own transaction; hovering or missing the curve makes none.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Inspect boundary pick, positive-distance answer, current camera screen conversion, click freshness and selection restoration at Done.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.1.23
npm --prefix ../session_tests run course -- save 16.1.23
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The preceding network/patch contract remains authored and unrun. The final editing contract adds projected originals, atomic Three Point retry, interval extension and active distance/cancellation. Actual browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 16.1.23
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
