# 06 · Share a corner between triangles

**Plan about 1–2 hours.** 24 lines to type, including comments and blank lines. Allow time to read, predict and experiment; this is an estimate, not a deadline.

**Today:** Draw a diamond from four positions and six small index numbers.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Six indices → four shared positions → vertex shader → two joined triangles.

**Before you finish, explain:** Why do we draw six indices when there are only four positions?

Imagine numbering the corners of a paper diamond. To name its bottom triangle, say “0, 1, 2”. To name its top triangle, say “0, 2, 3”. The numbers tell us how to connect corners; they do not contain coordinates themselves.

That is an **index buffer**. The position buffer answers “where is corner 2?” The index buffer answers “which corners make this triangle?” Changing one shared position now changes both triangles consistently.

![Two triangles refer to four numbered positions, sharing positions zero and two along their common edge.](../illustrations/journey-06.svg)

We use `u16`, an unsigned 16-bit integer, for these four indices. The draw call must read the buffer as `Uint16`. Each index takes two bytes; each position still takes eight. Larger meshes may need `u32` indices, but the idea is unchanged.

In `draw_indexed`, the first range chooses index entries. The middle argument, zero, is added to each index before looking up a vertex; later a shared buffer can use that base to locate another mesh. The last range still draws one instance.

The shader does not need an “indexed” version. It receives a position in exactly the same layout as before. We change the Rust resources and draw command that supply those positions.

## Type the change

Continue [Let Rust supply the corners](05-vertices.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-06-indices`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/renderer.rs`

Replace the positions and add their connections. The diamond makes the four distinct corners easy to identify.

Find this exact block:

```rust
const POSITIONS: [[f32; 2]; 6] = [
    [-0.6, -0.5], [0.6, -0.5], [0.6, 0.6],
    [-0.6, -0.5], [0.6, 0.6], [-0.6, 0.6],
];
```

Replace that block with:

```rust
--8<-- "journey/code/06-indices-01.rs"
```

### 2. `src/renderer.rs`

Keep the index buffer beside the position buffer. They serve different roles.

Find this exact block:

```rust
    vertices: wgpu::Buffer,
```

Replace that block with:

```rust
--8<-- "journey/code/06-indices-02.rs"
```

### 3. `src/renderer.rs`

Upload the index numbers. Reusing the name bytes starts a new binding; the previous upload is already complete.

Find this exact block:

```rust
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
```

Replace that block with:

```rust
--8<-- "journey/code/06-indices-03.rs"
```

### 4. `src/renderer.rs`

Transfer ownership of both buffers to the renderer.

Find this exact block:

```rust
        Self { device, queue, pipeline, vertices }
```

Replace that block with:

```rust
--8<-- "journey/code/06-indices-04.rs"
```

### 5. `src/renderer.rs`

Select the index format and draw six index entries. The existing vertex-buffer binding still supplies positions.

Find this exact block:

```rust
            pass.draw(0..POSITIONS.len() as u32, 0..1);
```

Replace that block with:

```rust
--8<-- "journey/code/06-indices-05.rs"
```

### 6. `src/browser.rs`

Connect share a corner between triangles to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Six uploaded corners make one rectangle.");
    Ok(())
}
```

Replace that block with:

```rust
--8<-- "journey/code/06-indices-dock-01.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

The rectangle becomes a diamond. Its upper and lower triangles meet along the horizontal diagonal. Run `Background` to check that the command path still works.

**Actual Chrome screenshot.**

![Actual browser result: Share a corner between triangles.](../screenshots/journey/06-indices-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

## Try one small experiment

Move position 0 from [-0.6, 0.0] to [-0.9, 0.0]. Predict which triangles change. Both should move their left corner together, leaving no crack. Then restore the coordinate and temporarily draw only 0..3 indices: only the lower triangle should remain. Restore 0..6 before comparing.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

Each triangle needs three corner references. The first triangle reads positions 0, 1 and 2; the second reads 0, 2 and 3. Positions 0 and 2 are shared. Six index entries describe two triangles while only four coordinate pairs are stored.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 06-indices
npm --prefix ../session_tests run course -- save 06-indices
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Mesh topology uses indices to share vertices. The production mesh lane uses the same distinction between a vertex range and an index range; its shared allocations add offsets without changing what an index means.

[Validation status and course release](release.md).
