# 6.1.10 · Record the ID draw and aligned image copy

**Chapter 6 · Picking and selection · Section 6.1 · GPU picking** · [Contents](contents.md)

**Typing: 21–42 minutes.** [Estimate](typing-load.md).

Record the same placed geometry into the integer image. End the render pass, then copy only the cursor window into aligned rows.

## Type

Continue from [Bound asynchronous picking ownership](42h-owner.md). [Save or recover your work](recovery.md).

### 1. `src/pick_encode.rs`

Record depth-correct IDs and copy the bounded cursor region.

Create the file and type:

```rust
--8<-- "journey/code/42i-copy-01.rs"
```

### 2. `src/lib.rs`

Register the new picking component.

<details>
<summary>Locate the existing block</summary>

```rust
mod pick_image;

pub mod pick_gpu;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/42i-copy-02.rs" skip=35
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Run the checks. ID recording and copy are prepared; the next step connects asynchronous mapping.

### Recorded visual result

**Previous native render — current verification pending.**

The inherited grid and geometry remain visible; copied integer readback checks verify the new picking behavior.

![Native renderer output for record the id draw and aligned image copy.](../screenshots/journey/41f-input.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [5.7.8 · Inspect the ground grid through actual commands](41f-input.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



mesh depth → visible ink → halo render → cursor-only aligned copy.

![Submit surfaces and ink to a bounded image before copying its cursor region.](../illustrations/journey-42i.svg)

Why draw a halo but copy a smaller region?

The halo preserves nearby rasterization support. Only pixels inside the cursor window are needed to choose the hit.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Follow the encoder scope: its render-pass borrow must end before the texture copy and encoder finish.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 6.1.10
npm --prefix ../session_tests run course -- save 6.1.10
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This is preparation within the GPU picking subject. The final input endpoint connects and checks actual canvas clicks; sub-object selection follows in44.

Existing normals commands remain available while the picking components are prepared.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 6.1.10
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
