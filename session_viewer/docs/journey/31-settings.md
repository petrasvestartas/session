# 31 · Keep selection out of the vertex data

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 8–15 minutes.** 19 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Put placement and selection in an explicitly packed object uniform.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Object placement and selection → eighty uniform bytes → vertex shader; source colours stay unchanged.

**Before you finish, explain:** Why must selection move before we can share one vertex buffer?

Our renderer currently copies gold into every selected vertex during upload. That makes selection part of geometry storage. Before sharing any buffers, separate those two responsibilities.

The first sixty-four bytes still contain the placement matrix. The next sixteen bytes form a vec4 selection field: its first float is one or zero and its remaining floats are zero. We pack all eighty bytes explicitly rather than depending on the layout of a Rust struct.

![Immutable vertex colours and one object uniform meet in the vertex shader.](../illustrations/journey-31.svg)

The [WGSL layout rules](https://www.w3.org/TR/WGSL/#alignment-and-size) align both a matrix and a vec4 to sixteen bytes. A four-column matrix occupies sixty-four bytes, so the following field begins at byte sixty-four and the complete uniform occupies eighty.

The shader multiplies by object.model and uses object.selection.x to choose the original colour or our existing gold. Lighting still runs after that choice. Source geometry and its colour values do not change when selection changes.

This endpoint still rebuilds GPU rows after every scene change. It establishes the data boundary; the following lessons change ownership and reuse.

## Type the change

Continue [Prove recovery at the transaction and display boundaries](30e-recovery.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-31-settings`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/gpu_mesh.rs`

Upload original vertex colours regardless of which object is selected.

Find this exact block:

```rust
            let mut display = *vertex;
            if selected {
                display[3..].copy_from_slice(&[1.0, 0.75, 0.05]);
            }
            for value in display {
```

Replace that block with:

```rust
--8<-- "journey/code/31-settings-01.rs"
```

### 2. `src/gpu_mesh.rs`

Use one encoder for the complete object uniform.

Find this exact block:

```rust
        let bytes: Vec<u8> = object.model.m.iter()
            .flat_map(|value| (*value as f32).to_ne_bytes()).collect();
```

Replace that block with:

```rust
--8<-- "journey/code/31-settings-02.rs"
```

### 3. `src/gpu_mesh.rs`

Encode the matrix, selection and padding at their shader offsets.

Find this exact block:

```rust
}

impl GpuMesh {
```

Replace that block with:

```rust
--8<-- "journey/code/31-settings-03.rs"
```

### 4. `src/triangle.wgsl`

Describe the same eighty-byte object layout in WGSL.

Find this exact block:

```wgsl
@group(1) @binding(0) var<uniform> model: mat4x4<f32>;
```

Replace that block with:

```wgsl
--8<-- "journey/code/31-settings-04.wgsl"
```

### 5. `src/triangle.wgsl`

Read placement from the object settings structure.

Find this exact block:

```wgsl
    let world = model * vec4<f32>(position, 1.0);
```

Replace that block with:

```wgsl
--8<-- "journey/code/31-settings-05.wgsl"
```

### 6. `src/triangle.wgsl`

Choose the displayed colour per object without rewriting vertices.

Find this exact block:

```wgsl
    output.colour = colour;
```

Replace that block with:

```wgsl
--8<-- "journey/code/31-settings-06.wgsl"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Open Replace the specimen, select the right post and Move it. Cycle selection through all three rows and return to the same post. The picture must return exactly; source colours remain available when a row is deselected.

**Actual Chrome screenshot.**

![Actual browser result: Keep selection out of the vertex data.](../screenshots/journey/31-settings-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Temporarily put the selection flag at byte sixty-eight while leaving the shader field unchanged. Predict why the first selection component remains zero. Restore byte sixty-four before continuing.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

Two objects can use the same geometry while only one is selected. A gold colour written into their shared vertices would affect both. Keep immutable source colours in the vertex buffer and choose the displayed colour from each object’s uniform.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 31-settings
npm --prefix ../session_tests run course -- save 31-settings
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Object presentation must stay separate from shared geometry in the production renderer, including selection, opacity and other display flags. This uniform introduces that boundary without claiming the later rendering modes are complete.

[Validation status and course release](release.md).
