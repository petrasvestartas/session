# 16 · Walk around the model

**Combined study estimate: 2–4 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 28–56 minutes.** 61 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Orbit and tilt a perspective camera while keeping its target in place.

**Follow:** View action → orientation quaternion → eye and up vectors → view-projection matrix → drawing and pick ray.

Orbit around a fixed target. Camera orientation determines its right, up and forward directions; viewing distance places the eye behind the target.

Use the kernel's quaternion to represent orientation. Rotate sideways around world z and tilt around the camera's current right axis. In `yaw × (pitch × orientation)`, the existing orientation is followed by tilt, then yaw. Order matters.

Normalise after updates to keep a pure rotation. The repeated-turn test checks that the resulting axes remain unit length and perpendicular.

Isometric changes orientation while retaining target and distance. Drawing and picking consume the same updated camera matrix. The scene, history and selection keep their existing owners.

![A fixed target and viewing distance define the orbit; orientation determines the camera’s right, forward and up directions.](../illustrations/journey-16.svg)

## Type the change

Continue from [Look through a perspective camera](15-perspective.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-16-orbit` (from `session_viewer`).

### 1. `src/camera.rs`

Use the kernel’s rotation representation alongside its points, vectors and matrices.

<details>
<summary>Locate the existing block</summary>

```rust
use session_rust::{Point, Vector, Xform};
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/16-orbit-01.rs"
```

### 2. `src/camera.rs`

Store a 3D target and orientation instead of a flat centre and one angle.

<details>
<summary>Locate the existing block</summary>

```rust
    pub center: [f64; 2],
    pub distance: f64,
    pub angle: f64,
    pub aspect: f64,
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/16-orbit-02.rs"
```

### 3. `src/camera.rs`

Keep the same initial eye and up directions as the perspective lesson.

<details>
<summary>Locate the existing block</summary>

```rust
        Self { center: [0.0, 0.0], distance: 3.0, angle: 0.0, aspect: 640.0 / 480.0 }
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/16-orbit-03.rs"
```

### 4. `src/camera.rs`

Move the target in the camera’s current right/up plane, including its world-z component.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn pan(&mut self, dx: f32, dy: f32) {
        let (sin, cos) = self.angle.sin_cos();
        self.center[0] += dx as f64 * cos - dy as f64 * sin;
        self.center[1] += dx as f64 * sin + dy as f64 * cos;
    }
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/16-orbit-04.rs"
```

### 5. `src/camera.rs`

Add world-up orbit and a named pose. Keep rotate as the existing Turn view action.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn rotate(&mut self, radians: f32) {
        self.angle += radians as f64;
    }
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/16-orbit-05.rs"
```

### 6. `src/camera.rs`

Calculate eye and up from the orientation, then reuse the existing view and projection construction.

<details>
<summary>Locate the existing block</summary>

```rust
        let eye = Point::new(self.center[0], self.center[1], self.distance);
        let target = Point::new(self.center[0], self.center[1], 0.0);
        let (sin, cos) = self.angle.sin_cos();
        let up = Vector::new(-sin, cos, 0.0);
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/16-orbit-06.rs"
```

### 7. `src/camera.rs`

Extend the target-centering test to include a 3D orbit.

<details>
<summary>Locate the existing block</summary>

```rust
        let target = Point::new(camera.center[0], camera.center[1], 0.0);
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/16-orbit-07.rs"
```

### 8. `src/camera.rs`

Check the rotation invariant after many small updates, not just one convenient angle.

<details>
<summary>Locate the existing block</summary>

```rust
    #[test]
    fn a_projected_point_lies_on_its_unprojected_ray() {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/16-orbit-08.rs"
```

### 9. `src/browser.rs`

Connect walk around the model to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

<details>
<summary>Locate the existing block</summary>

```rust
            "Pan Left",
            "Pan Right",
            "Orbit Right",
            "View Reset",
        ],
    );
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/16-orbit-window-1.rs"
```

### 10. `src/browser.rs`

Connect walk around the model to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

<details>
<summary>Locate the existing block</summary>

```rust
                "pan left" => camera.pan(-0.25, 0.0),
                "pan right" => camera.pan(0.25, 0.0),
                "orbit right" => camera.rotate(std::f32::consts::FRAC_PI_4),
                "view reset" => camera = Camera { aspect: camera.aspect, ..Camera::default() },
                _ => return,
            }
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/16-orbit-window-2.rs"
```

### 11. `src/browser.rs`

Connect walk around the model to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

<details>
<summary>Locate the existing block</summary>

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Perspective and picking use the same camera.");
    Ok(())
}
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/16-orbit-window-3.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Type `View Isometric`, then `Orbit Right`. The eye moves around the same target. Click a visible face to confirm picking still agrees with the new view.

**Verified checkpoint in Chrome.**

![Actual browser result: Walk around the model.](../screenshots/journey/16-orbit-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Type `View Isometric` and press Enter, pan, then tilt. Predict whether the point at the screen centre will stay centred during the tilt. It should: pan chose a new target, and orbit keeps that target fixed. Delete a selected surface and undo; the camera should keep its current orientation.

</details>

## Explain the change

Which camera value should stay fixed while you orbit around the model?

<details>
<summary>Compare your explanation</summary>

The target stays fixed. Orientation changes the direction from the eye toward that target, and distance controls how far the eye is from it. Recomputing eye = target − forward × distance moves the eye around the target without editing any mesh positions.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 16-orbit
npm --prefix ../session_tests run course -- save 16-orbit
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

The maintained camera uses target, distance, orientation and world-up in the same way. Later gesture lessons turn pointer deltas into these operations; unit conversion, fit-to-scene and large-coordinate precision extend the camera without moving source geometry.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

Look at the two triangles from Isometric: the changed overlap comes from the camera, not edited vertex positions.

[Full validation scope](release.md).

</details>
