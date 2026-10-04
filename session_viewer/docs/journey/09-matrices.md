# 09 · Let one matrix describe the view

**Typing: 12–23 minutes.** [Estimate](typing-load.md).

Replace separate scale and offset uniforms with one matrix so the view can also rotate. The geometry and draw pipeline remain the same.

Read the four columns as transformed x, y and z steps, followed by the transformed origin. A position combines them using x, y, z and a final 1.

## Type

Continue from [Move the view, keep the geometry](08-camera.md). [Save or recover your work](recovery.md).

### 1. `src/camera.rs`

Keep the viewing angle in radians beside the centre and scale.

<details>
<summary>Locate the existing block</summary>

```rust
    pub scale: f32,
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/09-matrices-01.rs"
```

### 2. `src/camera.rs`

Start without rotation and make Reset view restore that angle.

<details>
<summary>Locate the existing block</summary>

```rust
        Self { center: [0.0, 0.0], scale: 1.0 }
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/09-matrices-02.rs"
```

### 3. `src/camera.rs`

Replace the four-number uniform with a column-major matrix and add rotation.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn uniform(&self) -> [f32; 4] {
        // Subtract the camera centre before scaling the world position.
        [self.scale, self.scale, -self.center[0] * self.scale, -self.center[1] * self.scale]
    }
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/09-matrices-03.rs"
```

### 4. `src/camera.rs`

Strengthen the camera test: its target must stay centred after both zooming and rotation.

<details>
<summary>Locate the existing block</summary>

```rust
        let [sx, sy, ox, oy] = camera.uniform();
        assert_eq!(camera.center[0] * sx + ox, 0.0);
        assert_eq!(camera.center[1] * sy + oy, 0.0);
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/09-matrices-04.rs"
```

### 5. `src/triangle.wgsl`

Change the resource type to four columns of four floats.

<details>
<summary>Locate the existing block</summary>

```wgsl
var<uniform> transform: vec4<f32>;
```

</details>

Replace that block with:

```wgsl
--8<-- "journey/code/09-matrices-05.wgsl"
```

### 6. `src/triangle.wgsl`

Multiply a world position by the matrix. Its final component 1 lets the translation column contribute.

<details>
<summary>Locate the existing block</summary>

```wgsl
    return vec4<f32>(position * transform.xy + transform.zw, 0.0, 1.0);
```

</details>

Replace that block with:

```wgsl
--8<-- "journey/code/09-matrices-06.wgsl"
```

### 7. `src/renderer.rs`

Allocate 64 bytes for sixteen f32 matrix entries.

<details>
<summary>Locate the existing block</summary>

```rust
        });
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/09-matrices-fullscreen-5.rs"
```

### 8. `src/renderer.rs`

Upload all sixteen matrix entries before drawing.

<details>
<summary>Locate the existing block</summary>

```rust
        &self,
        view: &wgpu::TextureView,
        background: &crate::background::Background,
        transform: &[f32; 4],
    ) {
        let bytes: Vec<u8> = transform.iter().flat_map(|value| value.to_ne_bytes()).collect();
        self.queue.write_buffer(&self.uniform, 0, &bytes);
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/09-matrices-fullscreen-6.rs"
```

### 9. `src/browser.rs`

Add Orbit Right to the vocabulary.

<details>
<summary>Locate the existing block</summary>

```rust
            "Zoom Out",
            "Pan Left",
            "Pan Right",
            "View Reset",
        ],
    );
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/09-matrices-window-1.rs"
```

### 10. `src/browser.rs`

Rotate the camera when Orbit Right is submitted.

<details>
<summary>Locate the existing block</summary>

```rust
                "zoom out" => camera.zoom(0.5),
                "pan left" => camera.pan(-0.25, 0.0),
                "pan right" => camera.pan(0.25, 0.0),
                "view reset" => camera = Camera::default(),
                _ => return,
            }
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/09-matrices-window-2.rs"
```

### 11. `src/browser.rs`

Report the matrix transform.

<details>
<summary>Locate the existing block</summary>

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Pan and zoom change the view, not the mesh.");
    Ok(())
}
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/09-matrices-window-3.rs"
```

### 12. `src/browser.rs`

Pass the full matrix into presentation.

<details>
<summary>Locate the existing block</summary>

```rust
    surface: &wgpu::Surface<'_>,
    renderer: &Renderer,
    background: &Background,
    transform: &[f32; 4],
    panel: &mut crate::panel::Panel,
) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/09-matrices-window-4.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Type `Orbit Right`. The diamond rotates by 45 degrees. Type `View Reset` to restore its angle. The vertex buffer stays unchanged.

**Verified checkpoint in Chrome.**

![Actual browser result: Let one matrix describe the view.](../screenshots/journey/09-matrices-browser.png)

[Verification scope](release.md).

Run the state checks from your project folder:

```sh
cargo test --lib --locked -j4
```

<details>
<summary>Code explanation and diagram</summary>

For camera angle θ, use `a = scale × cos(θ)` and `b = scale × sin(θ)`. Screen coordinates become `(a×x + b×y, −b×x + a×y)` plus translation. Subtract the rotated, scaled camera centre so that centre still lands at zero.

WGSL reads four consecutive `vec4<f32>` columns: 64 bytes. Rust supplies floats in that column order. The rotation test catches an incorrect translation column.

Camera centre, angle and scale → four matrix columns → uniform buffer → matrix × world position.

![The matrix columns describe transformed x, y and z steps and the translated origin; their weighted sum gives a screen position.](../illustrations/journey-09.svg)

Which matrix column moves a point, and why does its input position end with a 1?

The fourth column contains translation. Multiplication adds that column multiplied by the input’s last component. A position ends with 1, so translation affects it. A direction would end with 0, so translation would not move it. The first three columns describe how the coordinate axes are transformed.

Study estimate, including typing and experiments: 2–4 hours.

</details>

<details>
<summary>Optional experiment</summary>

Pan Right, then turn the view. Predict where the origin will appear before you run it. Reset, turn twice, and locate the original right-hand corner: a 90-degree camera turn places it below the centre. The position buffer must remain unchanged throughout.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 09-matrices
npm --prefix ../session_tests run course -- save 09-matrices
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Frame uniforms in the complete renderer carry view and projection matrices. Reading a matrix as transformed axes plus an origin helps diagnose a transposed upload, a wrong multiplication order or geometry that drifts while orbiting.



[Full validation scope](release.md).

</details>
