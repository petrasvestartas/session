# 16.5.10 · Draw cached guides through the overlay owner

**Chapter 16 · Geometry editing and measurement · Section 16.5 · Text, styles and command integration** · [Contents](contents.md)

**Typing: 9–18 minutes.** [Estimate](typing-load.md).

Draw cached guides through the overlay owner.

## Type

Continue from [Cache guide buffers and actual allocation](85h-cache-guide-buffers-and-actual-allocation.md). [Save or recover your work](recovery.md).

### 1. `src/guide_gpu.rs`

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn labels(&self,existing:&[crate::text::TextLabel])->Vec<crate::text::TextLabel>{self.label.as_ref().map_or_else(Vec::new,|(point,label)|crate::measure_mark::Mark{points:vec![*point],label:label.clone()}.labels(existing))}
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/85i-draw-cached-guides-through-the-overlay-owner-01.rs" skip=233
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.5.10
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.



### Recorded visual result

**Previous native render — current verification pending.**

The preceding solid, wire bounding box and temporary distance annotation remain visible.

![Native renderer output for draw cached guides through the overlay owner.](../screenshots/journey/84m-exercise-original-boxes-and-temporary-distance-annotations.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.4.14 · Exercise original boxes and temporary distance annotations](84m-exercise-original-boxes-and-temporary-distance-annotations.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



unchanged overlay → reused strokes and text → color frame.

![Draw cached guides through the overlay owner.](../illustrations/85i-draw-cached-guides-through-the-overlay-owner.svg)

What happens when the overlay becomes empty?

The guide lane releases its used strokes and removes only its temporary labels.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare actual used and allocated bytes before and after clearing the overlay.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.5.10
npm --prefix ../session_tests run course -- save 16.5.10
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Draw cached guides through the overlay owner.

The inherited measurement browser contract remains authored and unrun. The current-command browser contract connects at85ay and still requires real browser execution.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 16.5.10
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
