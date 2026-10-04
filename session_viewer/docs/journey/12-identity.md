# 12 · Name objects without depending on their row

**Typing: 42–83 minutes.** [Estimate](typing-load.md).

**Splitting required:** this verified checkpoint exceeds the one-hour typing target. Smaller runnable lessons are still being prepared.

Give each object an ID that survives changes in vector order. Remember the extra triangle's ID, so removing an earlier object cannot make its row point at the wrong mesh.

`ObjectId` wraps a number in a distinct type. Its private value and `Scene`'s private vector protect identity. `checked_add` rejects counter exhaustion; IDs never wrap or get reused.

## Type

Continue from [Give the scene an owner](11-scene.md). [Save or recover your work](recovery.md).

### 1. `src/scene.rs`

Replace the scene with stable object IDs and named insertion, removal and lookup operations. Keep the two existing demonstration meshes unchanged.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::mesh::Mesh;

pub struct Scene {
    pub meshes: Vec<Mesh>,
}

impl Scene {
    pub fn demo() -> Self {
        let near = Mesh::new(vec![
            [-0.7, -0.6, 0.25, 0.9, 0.25, 0.45],
            [ 0.5, -0.6, 0.25, 0.9, 0.25, 0.45],
            [-0.1,  0.6, 0.25, 0.9, 0.25, 0.45],
        ], vec![0, 1, 2]).expect("Valid near triangle");
        let far = Mesh::new(vec![
            [-0.4, -0.2, 0.75, 0.05, 0.7, 0.7],
            [ 0.8, -0.2, 0.75, 0.05, 0.7, 0.7],
            [ 0.2,  0.8, 0.75, 0.05, 0.7, 0.7],
        ], vec![0, 1, 2]).expect("Valid far triangle");
        Self { meshes: vec![near, far] }
    }

    pub fn toggle_extra(&mut self) {
        if self.meshes.len() == 3 {
            self.meshes.pop();
        } else {
            let extra = Mesh::new(vec![
                [-0.9, 0.3, 0.5, 0.2, 0.8, 0.3],
                [-0.4, 0.3, 0.5, 0.2, 0.8, 0.3],
                [-0.65, 0.9, 0.5, 0.2, 0.8, 0.3],
            ], vec![0, 1, 2]).expect("Valid extra triangle");
            self.meshes.push(extra);
        }
    }
}
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/12-identity-01.rs"
```

### 2. `src/gpu_mesh.rs`

Let the uploaded representation know whether to show selection.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn upload(device: &wgpu::Device, mesh: &Mesh) -> Self {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/12-identity-02.rs"
```

### 3. `src/gpu_mesh.rs`

Apply the yellow tint to an upload copy. Never overwrite the CPU mesh’s original colour.

<details>
<summary>Locate the existing block</summary>

```rust
        let bytes: Vec<u8> = mesh.vertices().iter().flatten()
            .flat_map(|value| value.to_ne_bytes()).collect();
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/12-identity-03.rs"
```

### 4. `src/renderer.rs`

Import ObjectId at the renderer boundary.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{gpu_mesh::GpuMesh, scene::Scene};

pub struct Renderer {
    pub device: wgpu::Device,
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/12-identity-fullscreen-5.rs"
```

### 5. `src/renderer.rs`

Upload each scene object without selection highlighting initially.

<details>
<summary>Locate the existing block</summary>

```rust
        format: wgpu::TextureFormat,
        scene: &Scene,
    ) -> Self {
        let meshes = scene.meshes.iter().map(|mesh| GpuMesh::upload(&device, mesh)).collect();
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(include_str!("triangle.wgsl").into()),
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/12-identity-fullscreen-6.rs"
```

### 6. `src/renderer.rs`

Highlight the uploaded object whose stable ID matches the selection.

<details>
<summary>Locate the existing block</summary>

```rust
        Self { device, queue, pipeline, meshes, uniform, view_group, depth }
    }

    pub fn set_scene(&mut self, scene: &Scene) {
        self.meshes = scene.meshes.iter().map(|mesh| GpuMesh::upload(&self.device, mesh)).collect();
    }

    fn depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/12-identity-fullscreen-7.rs"
```

### 7. `src/browser.rs`

Add Select Next and Delete to the vocabulary.

<details>
<summary>Locate the existing block</summary>

```rust
        &[
            "Help",
            "Example Triangle",
            "Background",
            "Zoom In",
            "Zoom Out",
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/12-identity-window-1.rs"
```

### 8. `src/browser.rs`

Start without a selected object.

<details>
<summary>Locate the existing block</summary>

```rust
        ],
    );
    panel.update(None, &canvas)?;
    let mut background = Background::default();
    let mut camera = Camera::default();
    present(
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/12-identity-window-2.rs"
```

### 9. `src/browser.rs`

Preserve valid selection, cycle by identity, and delete the selected object.

<details>
<summary>Locate the existing block</summary>

```rust
            match line.as_str() {
                "example triangle" => {
                    scene.toggle_extra();
                    renderer.set_scene(&scene);
                }
                "background" => background.toggle(),
                "zoom in" => camera.zoom(2.0),
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/12-identity-window-3.rs"
```

### 10. `src/browser.rs`

Report that selection follows object identity.

<details>
<summary>Locate the existing block</summary>

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Scene data owns the meshes; the renderer displays them.");
    Ok(())
}
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/12-identity-window-4.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Type `Select Next`, then `Delete`. The yellow object disappears. Run the identity checks below: the remaining object must keep its ID even when its row changes.

**Verified checkpoint in Chrome.**

![Actual browser result: Name objects without depending on their row.](../screenshots/journey/12-identity-browser.png)

[Verification scope](release.md).

Run the state checks from your project folder:

```sh
cargo test --lib --locked -j4
```

<details>
<summary>Code explanation and diagram</summary>

Selection is `Option<ObjectId>` in interaction state. When deletion removes that ID, clear selection. Camera changes preserve it.

For now, uploading a selected object uses yellow display vertices while retaining its original mesh colours. A later object-data buffer will avoid that whole-mesh upload.

ObjectId → scene lookup → selected display colour → uploaded mesh → yellow highlight.

![Deleting row zero shifts the second object into its seat while that object keeps ID 2; selection follows the ID.](../illustrations/journey-12.svg)

After deleting the first object, is the object now at row zero a new object?

No. Its storage row moved, but its ObjectId stayed the same. A row is a current location in a vector; an ID is the name we assigned when the object entered the scene. Selection retains that name, and a deleted name is never silently reused.

Study estimate, including typing and experiments: 3–5 hours.

</details>

<details>
<summary>Optional experiment</summary>

Add the third triangle, select the first object and delete it. Toggle the third triangle off. Only the original far triangle should remain. Read the identity test before running it: which ID moves to row zero, and why must it keep its old value?

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 12-identity
npm --prefix ../session_tests run course -- save 12-identity
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

The complete viewer distinguishes document identity, displayed object rows and source sub-elements. Picking and undo must cross those mappings explicitly. This lesson establishes the first rule: a storage index cannot serve as a permanent object name.



[Full validation scope](release.md).

</details>
