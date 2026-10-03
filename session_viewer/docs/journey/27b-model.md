# 27b · Apply object placement on the GPU

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 17–33 minutes.** 30 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Send a separate model matrix for each draw while retaining local vertex buffers.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Local vertex → model uniform → world point → camera uniform → clip position.

**Before you finish, explain:** Why do model and camera matrices need separate owners?

Bounds and picking now understand placement. Drawing must apply that same matrix, or the visible object and the selectable object would disagree.

Add a second uniform binding to the shader. Group 0 remains the shared camera transform. Group 1 contains one object’s model matrix. The vertex shader first produces a world position, then applies the camera. The fragment shader receives that world position so its derivative-based lighting follows the placed surface too.

![The camera binding is shared; the model binding changes for each object draw.](../illustrations/journey-27b.svg)

GpuMesh uploads the original local vertices, encodes the sixteen model values as floats, and creates the object bind group from the pipeline’s group-1 layout. The bind group retains its referenced buffer. Bind it immediately before drawing that object’s indices.

Finally change Example Box: create its shape around the local origin, then give the Object the translation that used to be baked into its vertices. Its world appearance stays consistent. Its local box coordinates are now reusable. The current renderer still rebuilds uploads on scene changes; later shared-storage lessons will update changed ranges.

## Type the change

Continue [Ask geometry questions in world coordinates](27a-world.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-27b-model`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/triangle.wgsl`

Keep camera and object placement in separate uniform groups.

Find this exact block:

```wgsl
@group(0) @binding(0) var<uniform> transform: mat4x4<f32>;
```

Replace that block with:

```wgsl
--8<-- "journey/code/27b-model-01.wgsl"
```

### 2. `src/triangle.wgsl`

Place the vertex before projecting it; use world position for lighting.

Find this exact block:

```wgsl
    output.position = transform * vec4<f32>(position, 1.0);
    output.colour = colour;
    output.world = position;
```

Replace that block with:

```wgsl
--8<-- "journey/code/27b-model-02.wgsl"
```

### 3. `src/gpu_mesh.rs`

Upload an object’s mesh and placement together.

Find this exact block:

```rust
use crate::mesh::Mesh;
```

Replace that block with:

```rust
--8<-- "journey/code/27b-model-03.rs"
```

### 4. `src/gpu_mesh.rs`

Retain the object bind group alongside its vertex/index buffers.

Find this exact block:

```rust
    index_count: u32,
```

Replace that block with:

```rust
--8<-- "journey/code/27b-model-04.rs"
```

### 5. `src/gpu_mesh.rs`

Borrow local mesh coordinates and the model from the same Object.

Find this exact block:

```rust
    pub fn upload(device: &wgpu::Device, mesh: &Mesh, selected: bool) -> Self {
```

Replace that block with:

```rust
--8<-- "journey/code/27b-model-05.rs"
```

### 6. `src/gpu_mesh.rs`

Encode the model’s column-major values and bind the uniform buffer.

Find this exact block:

```rust
        Self { vertices, indices, index_count: mesh.indices().len() as u32 }
```

Replace that block with:

```rust
--8<-- "journey/code/27b-model-06.rs"
```

### 7. `src/gpu_mesh.rs`

Choose this object’s placement before its indexed draw.

Find this exact block:

```rust
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
```

Replace that block with:

```rust
--8<-- "journey/code/27b-model-07.rs"
```

### 8. `src/renderer.rs`

Remove the upload that ran before the pipeline layout was available.

Find this exact block:

```rust
        let meshes = scene.objects().iter().map(|object| GpuMesh::upload(&device, &object.mesh, false)).collect();
```

Delete this block.

### 9. `src/renderer.rs`

Upload objects using the model layout inferred from the shader.

Find this exact block:

```rust
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
```

Replace that block with:

```rust
--8<-- "journey/code/27b-model-09.rs"
```

### 10. `src/renderer.rs`

Include each placement when a scene update rebuilds the current uploads.

Find this exact block:

```rust
    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.meshes = scene.objects().iter().map(|object| {
            GpuMesh::upload(&self.device, &object.mesh, selected == Some(object.id))
```

Replace that block with:

```rust
--8<-- "journey/code/27b-model-10.rs"
```

### 11. `src/scene.rs`

Keep the generated box centred in its local coordinates.

Find this exact block:

```rust
        source.transform(&session_rust::Xform::translation(0.9, 0.0, 0.4));
```

Delete this block.

### 12. `src/scene.rs`

Assign its world placement after inserting the local mesh.

Find this exact block:

```rust
        self.insert(Mesh::from_kernel(&source)?)
```

Replace that block with:

```rust
--8<-- "journey/code/27b-model-12.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run Example Box, Select Next three times, View Isometric and Fit Selected. The box must appear at its placed world position, remain selectable, and fit correctly. Its local coordinates stay centred around zero. Check the shader trace: model first, camera second.

**Actual Chrome screenshot.**

Apply object placement on the GPU. The actual command dock drives this checkpoint; the selected object is highlighted.

![Actual browser result: Apply object placement on the GPU.](../screenshots/journey/27b-model-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Change Example Box’s placement translation from (0.9, 0, 0.4) to (1.4, 0, 0.4). Predict where Fit Selected will aim and verify the visible box and its pick agree. Restore the original value. Do not move the box by rewriting its mesh.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

The object owns where its shape belongs; the camera owns how the scene is viewed. Binding the object model before each draw lets local mesh data stay unchanged while the same camera draws many placements.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 27b-model
npm --prefix ../session_tests run course -- save 27b-model
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

The maintained viewer shares local geometry across instances and supplies placement data to GPU draws. This checkpoint introduces the same ownership boundary with one bind group per object; batching and resource accounting follow later.

[Validation status and course release](release.md).
