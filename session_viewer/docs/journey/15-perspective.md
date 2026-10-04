# 15 · Look through a perspective camera

**Typing: 55–109 minutes.** [Estimate](typing-load.md).

**Splitting required:** this verified checkpoint exceeds the one-hour typing target. Smaller runnable lessons are still being prepared.

Use a perspective camera looking from z = 3 towards z = 0, with a 60-degree vertical field of view. Farther objects appear smaller.

The view matrix puts positions relative to the eye; projection produces clip coordinates. The GPU divides by w. The existing shader matrix multiplication and `Less` depth test still work. Turquoise is now nearer from this viewpoint and wins the overlap.

## Type

Continue from [Run Undo and Redo from the command line](14-history.md). [Save or recover your work](recovery.md).

### 1. `src/camera.rs`

Replace the flat camera conversion with a perspective view and a clipped world-space ray. Retain the same pan, zoom, rotate and uniform entry points.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Camera {
    pub center: [f32; 2],
    pub scale: f32,
    pub angle: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self { center: [0.0, 0.0], scale: 1.0, angle: 0.0 }
    }
}

impl Camera {
    pub fn pan(&mut self, dx: f32, dy: f32) {
        self.center[0] += dx;
        self.center[1] += dy;
    }

    pub fn zoom(&mut self, factor: f32) {
        if factor.is_finite() && factor > 0.0 {
            self.scale = (self.scale * factor).clamp(0.1, 10.0);
        }
    }

    pub fn rotate(&mut self, radians: f32) {
        self.angle += radians;
    }

    pub fn world_from_screen(&self, point: [f32; 2]) -> [f32; 2] {
        let x = point[0] / self.scale;
        let y = point[1] / self.scale;
        let (sin, cos) = self.angle.sin_cos();
        [self.center[0] + x * cos - y * sin, self.center[1] + x * sin + y * cos]
    }

    pub fn uniform(&self) -> [f32; 16] {
        let a = self.scale * self.angle.cos();
        let b = self.scale * self.angle.sin();
        let [x, y] = self.center;
        // WGSL matrices are uploaded column by column.
        [
            a, -b, 0.0, 0.0,
            b,  a, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            -a * x - b * y, b * x - a * y, 0.0, 1.0,
        ]
    }
}

#[cfg(test)]
mod coordinate_tests {
    use super::Camera;

    #[test]
    fn moving_the_view_preserves_the_coordinate_round_trip() {
        let mut camera = Camera::default();
        camera.pan(0.1, -0.2);
        camera.zoom(0.5);
        camera.rotate(0.7);
        let world = [0.4, 0.0];
        let matrix = camera.uniform();
        let screen = [matrix[0] * world[0] + matrix[4] * world[1] + matrix[12],
            matrix[1] * world[0] + matrix[5] * world[1] + matrix[13]];
        let restored = camera.world_from_screen(screen);
        assert!((restored[0] - world[0]).abs() < 1.0e-6);
        assert!((restored[1] - world[1]).abs() < 1.0e-6);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panned_target_stays_at_the_screen_centre_when_zooming() {
        let mut camera = Camera::default();
        camera.pan(0.25, -0.5);
        camera.zoom(2.0);
        camera.rotate(std::f32::consts::FRAC_PI_4);
        let m = camera.uniform();
        let [x, y] = camera.center;
        assert!((m[0] * x + m[4] * y + m[12]).abs() < 1.0e-6);
        assert!((m[1] * x + m[5] * y + m[13]).abs() < 1.0e-6);
    }

    #[test]
    fn zoom_stays_positive_and_bounded() {
        let mut camera = Camera::default();
        camera.zoom(0.0001);
        assert_eq!(camera.scale, 0.1);
        camera.zoom(1_000.0);
        assert_eq!(camera.scale, 10.0);
        camera.zoom(f32::NAN);
        camera.zoom(-1.0);
        assert_eq!(camera.scale, 10.0);
    }
}
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/15-perspective-02.rs"
```

### 2. `src/picking.rs`

Replace the flat coverage query with ray–triangle intersections and update the visibility tests for the new camera.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::scene::{ObjectId, Scene};

pub fn pick(scene: &Scene, point: [f32; 2]) -> Option<ObjectId> {
    let mut nearest = 1.0;
    let mut hit = None;
    for object in scene.objects() {
        let vertices = object.mesh.vertices();
        for triangle in object.mesh.indices().chunks_exact(3) {
            let a = vertices[triangle[0] as usize];
            let b = vertices[triangle[1] as usize];
            let c = vertices[triangle[2] as usize];
            if let Some(depth) = triangle_depth(point, a, b, c) {
                if depth >= 0.0 && depth < nearest {
                    nearest = depth;
                    hit = Some(object.id);
                }
            }
        }
    }
    hit
}

fn triangle_depth(p: [f32; 2], a: [f32; 6], b: [f32; 6], c: [f32; 6]) -> Option<f32> {
    let cross = |u: [f32; 2], v: [f32; 2]| u[0] * v[1] - u[1] * v[0];
    let ab = [b[0] - a[0], b[1] - a[1]];
    let ac = [c[0] - a[0], c[1] - a[1]];
    let ap = [p[0] - a[0], p[1] - a[1]];
    let area = cross(ab, ac);
    if area.abs() < 1.0e-8 {
        return None;
    }
    let u = cross(ap, ac) / area;
    let v = cross(ab, ap) / area;
    if u >= 0.0 && v >= 0.0 && u + v <= 1.0 {
        Some((1.0 - u - v) * a[2] + u * b[2] + v * c[2])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera::Camera;

    #[test]
    fn overlap_chooses_the_visible_object_and_deletion_reveals_the_next() {
        let mut scene = Scene::demo();
        let near = scene.objects()[0].id;
        let far = scene.objects()[1].id;
        assert_eq!(pick(&scene, [0.0, 0.0]), Some(near));
        assert_eq!(pick(&scene, [0.4, 0.0]), Some(far));
        scene.remove(near);
        assert_eq!(pick(&scene, [0.0, 0.0]), Some(far));
        assert_eq!(pick(&scene, [-0.9, -0.9]), None);
    }

    #[test]
    fn picking_follows_a_panned_zoomed_and_rotated_camera() {
        let scene = Scene::demo();
        let mut camera = Camera::default();
        camera.pan(0.1, -0.2);
        camera.zoom(0.5);
        camera.rotate(0.7);
        let world = [0.4, 0.0];
        let m = camera.uniform();
        let screen = [m[0] * world[0] + m[4] * world[1] + m[12],
            m[1] * world[0] + m[5] * world[1] + m[13]];
        let restored = camera.world_from_screen(screen);
        assert!((restored[0] - world[0]).abs() < 1.0e-6);
        assert!((restored[1] - world[1]).abs() < 1.0e-6);
        assert_eq!(pick(&scene, restored), Some(scene.objects()[1].id));
    }

    #[test]
    fn an_edge_on_or_invalid_triangle_cannot_be_picked() {
        assert_eq!(triangle_depth([0.0, 0.0], [0.0; 6], [0.0; 6], [0.0; 6]), None);
        assert_eq!(pick(&Scene::demo(), [f32::NAN, 0.0]), None);
    }
}
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/15-perspective-03.rs"
```

### 3. `src/scene.rs`

Name the mesh by colour; which surface is nearer now depends on the camera.

<details>
<summary>Locate the existing block</summary>

```rust
        let near = Mesh::new(vec![
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/15-perspective-05.rs"
```

### 4. `src/scene.rs`

Give the other mesh a camera-independent name.

<details>
<summary>Locate the existing block</summary>

```rust
        let far = Mesh::new(vec![
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/15-perspective-06.rs"
```

### 5. `src/scene.rs`

Name the pink triangle in its validation error.

<details>
<summary>Locate the existing block</summary>

```rust
expect("Valid near triangle")
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/15-perspective-07.rs"
```

### 6. `src/scene.rs`

Name the turquoise triangle in its validation error.

<details>
<summary>Locate the existing block</summary>

```rust
expect("Valid far triangle")
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/15-perspective-08.rs"
```

### 7. `src/scene.rs`

Insert the same two mesh data sets in the same order. Their identities are unchanged.

<details>
<summary>Locate the existing block</summary>

```rust
        scene.insert(near).expect("ID available");
        scene.insert(far).expect("ID available");
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/15-perspective-09.rs"
```

### 8. `Cargo.toml`

Add the geometry kernel and browser-compatible random number support.

<details>
<summary>Locate the existing block</summary>

```toml
console_error_panic_hook = "=0.1.7"
wasm-bindgen-futures = "=0.4.78"
wgpu = "=29.0.4"

egui = { version = "=0.34.3", default-features = false }
egui-wgpu = { version = "=0.34.3", default-features = false }
```

</details>

Replace that block with:

```toml
--8<-- "journey/code/15-perspective-dock-01.toml"
```

### 9. `src/browser.rs`

Initialize camera aspect from the canvas size.

<details>
<summary>Locate the existing block</summary>

```rust
    let mut history = History::default();
    let mut selected = None;
    let mut background = Background::default();
    let mut camera = Camera::default();
    present(
        &surface,
        &renderer,
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/15-perspective-window-1.rs"
```

### 10. `src/browser.rs`

Build a camera ray for the clicked screen position and pick along it.

<details>
<summary>Locate the existing block</summary>

```rust
                        (2.0 * (event.client_x() as f64 - rect.left()) / rect.width() - 1.0) as f32,
                        (1.0 - 2.0 * (event.client_y() as f64 - rect.top()) / rect.height()) as f32,
                    ];
                    selected = crate::picking::pick(&scene, camera.world_from_screen(screen));
                    renderer.set_scene(&scene, selected);
                }
                "example triangle" => {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/15-perspective-window-2.rs"
```

### 11. `src/browser.rs`

Preserve aspect when resetting the camera.

<details>
<summary>Locate the existing block</summary>

```rust
                "pan left" => camera.pan(-0.25, 0.0),
                "pan right" => camera.pan(0.25, 0.0),
                "orbit right" => camera.rotate(std::f32::consts::FRAC_PI_4),
                "view reset" => camera = Camera::default(),
                _ => return,
            }
        }
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/15-perspective-window-3.rs"
```

### 12. `src/browser.rs`

Report that drawing and picking share the camera.

<details>
<summary>Locate the existing block</summary>

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Undo restores the document while the view stays put.");
    Ok(())
}
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/15-perspective-window-4.rs"
```

## Run and check

After typing the manifest, run this from `session_viewer` to select the fixed dependency versions. It updates Cargo.lock, preserves the previous lock, and installs any supplied binary font assets. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15-perspective
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Click the turquoise triangle at the overlap, type `Delete`, then `Undo`. Repeat after `Zoom In`. Picking must agree with the perspective drawing.

**Verified checkpoint in Chrome.**

![Actual browser result: Look through a perspective camera.](../screenshots/journey/15-perspective-browser.png)

[Verification scope](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Code explanation and diagram</summary>

Use the kernel's `Point`, `Vector` and `Xform` for double-precision calculations, then `to_f32` for the existing GPU uniform. Keep camera aspect equal to window width divided by height, including after reset.

Picking needs a ray: inverse-transform screen depth 0 and 1 to near and far points. Intersect that bounded segment with triangles and choose the nearest valid hit. Reject a near-zero determinant and hits outside the segment.

World point → view matrix → projection → divide by w → canvas; click → inverse projection → clipped ray → object ID.

![The eye sees a widening region between near and far planes; reversing the projection turns a clicked pixel into a segment through that region.](../illustrations/journey-15.svg)

Why can a larger world z value now belong to the nearer triangle?

World z is a scene coordinate, not distance from the camera. This camera is at positive z and looks toward z = 0, so the triangle at z = 0.75 is nearer than the one at z = 0.25. The view and projection matrices convert those world positions into depth values for the existing Less comparison.

Study estimate, including typing and experiments: 4–7 hours.

</details>

<details>
<summary>Optional experiment</summary>

Before running, predict which triangle is closer to an eye at z = 3. Check the overlap. Then select and delete turquoise: the pink surface beneath should become selectable. Undo, zoom in, and explain why the CPU ray and GPU depth test still agree despite the changed apparent sizes.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15-perspective
npm --prefix ../session_tests run course -- save 15-perspective
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

The full viewer uses this same geometry kernel, view-projection boundary and identity resolution. Its later camera adds world-up orbit, units, fit, floating origins and reversed depth for large scenes; those are extensions of the responsibilities introduced here.



[Full validation scope](release.md).

</details>
