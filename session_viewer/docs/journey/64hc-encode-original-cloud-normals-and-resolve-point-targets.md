# 11.2.12 · Encode original cloud normals and resolve point targets

**Chapter 11 · Outlines and point clouds · Section 11.2 · Point clouds and detail levels** · [Contents](contents.md)

**Typing: 17–33 minutes.** [Estimate](typing-load.md).

Encode source normals into the sixteen-bit octahedral format expected by the shader. Preserve the absent-normal case instead of normalizing a zero vector. This adapter uploads original data; it does not replace or mutate the source cloud.

## Type

Continue from [Rasterize source point discs with integer ownership](64hb-rasterize-source-point-discs-with-integer-ownership.md). [Save or recover your work](recovery.md).

### 1. `src/cloud_normals.rs`

Copy current production oct-normal packing.

Create the file and type:

```rust
--8<-- "journey/code/64hc-encode-original-cloud-normals-and-resolve-point-targets-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod cloud_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/64hc-encode-original-cloud-normals-and-resolve-point-targets-02.rs" skip=30
```

### 3. `src/cloud_resolve.wgsl`

Resolve cached forward depth and original whole-owner IDs through a bounded pick-window origin.

Create the file and type:

```wgsl
--8<-- "journey/code/64hc-encode-original-cloud-normals-and-resolve-point-targets-03.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 11.2.12
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

The selected CAD bore and black original and silhouette contours remain visible.

![Native renderer output for encode original cloud normals and resolve point targets.](../screenshots/journey/63h-outline.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [11.1.10 · Check real contours widths reuse picks and cleanup](63h-outline.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

The resolve shader reads the private point depth, color and owner attachments. A forward-depth clear is empty; valid point depth is written into the main depth attachment. Apply the crop origin for picks and return the original object owner only in object mode. Keep both resolves on the same source pixels.

encode original cloud normals and resolve point targets.



Why does the integer resolve add its crop origin while the color resolve does not?

The color pass covers the whole target. The pick pass renders a cropped window, so its local fragment coordinate must be translated back to the source point target.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a zero normal, a reflected normal and a pick at crop origin(40,20). Predict the packed normal, empty-depth behavior and source pixel address. After connection, compare a point pixel and its picked original owner.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 11.2.12
npm --prefix ../session_tests run course -- save 11.2.12
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The inherited silhouette browser contract remains authored and unrun. Cloud acceptance connects at the final endpoint; native software rendering is not Lenovo timing.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 11.2.12
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
