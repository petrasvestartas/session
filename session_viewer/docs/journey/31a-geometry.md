# 31a · Give immutable GPU geometry one owner

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 19–38 minutes.** 48 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Separate vertex/index storage from each object’s uniform and retain its CPU source.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** CPU display Rc → GPU geometry Rc → object row with its own uniform → draw.

**Before you finish, explain:** What does cloning Rc<GpuGeometry> copy?

A GPU row currently owns vertices, indices and placement together. Extract the immutable part into GpuGeometry. It retains the exact Rc<Mesh> that produced its buffers; that source owner will also make identity safe when we introduce the cache.

![Several object rows can own one geometry allocation while retaining separate uniforms.](../illustrations/journey-31a.svg)

GpuMesh becomes one object’s draw state: a shared geometry owner and its own bind group. with_geometry accepts an existing geometry owner, while upload remains a convenient wrapper that creates one. The renderer still calls upload independently at this endpoint. Automatic reuse arrives next.

Follow the two draws. GpuMesh binds its object settings at group one, then GpuGeometry binds its vertex and index buffers. The render pass borrows both; it does not consume either.

The native GPU check constructs two rows from one geometry owner with different placement and selection. It checks shared allocation identity and release before drawing the ordinary lesson scene, whose independently selected rows verify the uniform boundary. This proves the ownership boundary, not automatic scene instancing; the later definition/instance lessons supply that document model.

## Type the change

Continue [Keep selection out of the vertex data](31-settings.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-31a-geometry`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/gpu_geometry.rs`

Create an owner for immutable buffers and the CPU display allocation that produced them.

Create the file and type:

```rust
--8<-- "journey/code/31a-geometry-01.rs"
```

### 2. `src/lib.rs`

Make geometry storage available to the draw row.

Find this exact block:

```rust
pub mod gpu_mesh;
```

Replace that block with:

```rust
--8<-- "journey/code/31a-geometry-02.rs"
```

### 3. `src/gpu_mesh.rs`

Use a shared geometry owner.

Find this exact block:

```rust
use crate::scene::Object;
```

Replace that block with:

```rust
--8<-- "journey/code/31a-geometry-03.rs"
```

### 4. `src/gpu_mesh.rs`

Keep only the geometry owner beside the object bind group.

Find this exact block:

```rust
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
    index_count: u32,
```

Replace that block with:

```rust
--8<-- "journey/code/31a-geometry-04.rs"
```

### 5. `src/gpu_mesh.rs`

Accept an existing geometry owner while keeping object settings independent.

Find this exact block:

```rust
        let mesh = &object.mesh;
        let mut bytes = Vec::with_capacity(mesh.vertices().len() * 24);
        for vertex in mesh.vertices() {
            for value in vertex {
                bytes.extend_from_slice(&value.to_ne_bytes());
            }
        }
        let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mesh vertices"),
            contents: &bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
        let bytes: Vec<u8> = mesh.indices().iter().flat_map(|index| index.to_ne_bytes()).collect();
        let indices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mesh indices"),
            contents: &bytes,
            usage: wgpu::BufferUsages::INDEX,
        });
```

Replace that block with:

```rust
--8<-- "journey/code/31a-geometry-05.rs"
```

### 6. `src/gpu_mesh.rs`

Return one draw row without copying its shared geometry.

Find this exact block:

```rust
        Self { vertices, indices, model_group, index_count: mesh.indices().len() as u32 }
```

Replace that block with:

```rust
--8<-- "journey/code/31a-geometry-06.rs"
```

### 7. `src/gpu_mesh.rs`

Delegate immutable bindings and indexed drawing to the geometry owner.

Find this exact block:

```rust
        pass.set_vertex_buffer(0, self.vertices.slice(..));
        pass.set_index_buffer(self.indices.slice(..), wgpu::IndexFormat::Uint16);
        pass.draw_indexed(0..self.index_count, 0, 0..1);
```

Replace that block with:

```rust
--8<-- "journey/code/31a-geometry-07.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Build and run the specimen. Selection, Move and Undo/Redo must keep the same appearance. Explain which fields could now be shared by two placements and which must remain independent.

**Actual Chrome screenshot.**

![Actual browser result: Give immutable GPU geometry one owner.](../screenshots/journey/31a-geometry-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Follow Rc::clone from a geometry owner into two GPU rows. Predict the strong owner count after the local owner is dropped and after each row is dropped. Do not confuse that count with a byte count.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

It adds an owner of the same geometry allocation. It does not upload another vertex or index buffer. Each GPU row still owns an independent placement/selection uniform, so objects can share geometry without sharing their settings.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 31a-geometry
npm --prefix ../session_tests run course -- save 31a-geometry
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

This is the ownership boundary needed for cached uploads and later definition instances. Production packed arenas and large-scene performance remain later work.

[Validation status and course release](release.md).
