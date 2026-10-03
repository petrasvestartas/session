# 16 · Walk around the model

**Combined study estimate: 2–4 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 28–56 minutes.** 61 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Orbit and tilt a perspective camera while keeping its target in place.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** View action → orientation quaternion → eye and up vectors → view-projection matrix → drawing and pick ray.

**Before you finish, explain:** Which camera value should stay fixed while you orbit around the model?

Imagine walking around a small sculpture while keeping your eyes on one point. Your position changes, but the point you are watching stays put. That point is the camera target.

We extend the camera from a flat centre and one angle to a 3D target and an orientation. The orientation tells us where the camera's right, forward and up axes point. The eye lies one viewing distance behind the target along the forward axis.

![A fixed target and viewing distance define the orbit; orientation determines the camera’s right, forward and up directions.](../illustrations/journey-16.svg)

A **quaternion** is the geometry kernel's compact representation of a rotation. For this lesson, read its operations as actions: create a rotation around an axis, combine rotations, rotate a vector, and normalise the result. Normalisation keeps the orientation a pure rotation after many updates; it does not change the viewing distance.

Orbit uses two axes. Turning sideways rotates around world z, the scene's vertical direction. Tilting rotates around the camera's current right direction. That right direction changes as we move, so we calculate it from the current orientation before making the tilt rotation.

Order matters. In `yaw × (pitch × orientation)`, apply the existing orientation, then the tilt, then the turn around world up. The multiplication does not mean “add three angles”. Our test repeats many small turns and checks that the three resulting axes remain perpendicular and unit length.

The default orientation preserves the previous lesson's straight-down view: forward points down −z, up points along +y. Isometric changes only orientation. The target, distance, scene, history and selection stay in their existing owners. Drawing and picking both use the same updated matrix, so neither needs a special orbit path.

## Type the change

Continue [Look through a perspective camera](15-perspective.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-16-orbit`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/camera.rs`

Use the kernel’s rotation representation alongside its points, vectors and matrices.

Find this exact block:

```rust
use session_rust::{Point, Vector, Xform};
```

Replace that block with:

```rust
--8<-- "journey/code/16-orbit-01.rs"
```

### 2. `src/camera.rs`

Store a 3D target and orientation instead of a flat centre and one angle.

Find this exact block:

```rust
    pub center: [f64; 2],
    pub distance: f64,
    pub angle: f64,
    pub aspect: f64,
```

Replace that block with:

```rust
--8<-- "journey/code/16-orbit-02.rs"
```

### 3. `src/camera.rs`

Keep the same initial eye and up directions as the perspective lesson.

Find this exact block:

```rust
        Self { center: [0.0, 0.0], distance: 3.0, angle: 0.0, aspect: 640.0 / 480.0 }
```

Replace that block with:

```rust
--8<-- "journey/code/16-orbit-03.rs"
```

### 4. `src/camera.rs`

Move the target in the camera’s current right/up plane, including its world-z component.

Find this exact block:

```rust
    pub fn pan(&mut self, dx: f32, dy: f32) {
        let (sin, cos) = self.angle.sin_cos();
        self.center[0] += dx as f64 * cos - dy as f64 * sin;
        self.center[1] += dx as f64 * sin + dy as f64 * cos;
    }
```

Replace that block with:

```rust
--8<-- "journey/code/16-orbit-04.rs"
```

### 5. `src/camera.rs`

Add world-up orbit and a named pose. Keep rotate as the existing Turn view action.

Find this exact block:

```rust
    pub fn rotate(&mut self, radians: f32) {
        self.angle += radians as f64;
    }
```

Replace that block with:

```rust
--8<-- "journey/code/16-orbit-05.rs"
```

### 6. `src/camera.rs`

Calculate eye and up from the orientation, then reuse the existing view and projection construction.

Find this exact block:

```rust
        let eye = Point::new(self.center[0], self.center[1], self.distance);
        let target = Point::new(self.center[0], self.center[1], 0.0);
        let (sin, cos) = self.angle.sin_cos();
        let up = Vector::new(-sin, cos, 0.0);
```

Replace that block with:

```rust
--8<-- "journey/code/16-orbit-06.rs"
```

### 7. `src/camera.rs`

Extend the target-centering test to include a 3D orbit.

Find this exact block:

```rust
        let target = Point::new(camera.center[0], camera.center[1], 0.0);
```

Replace that block with:

```rust
--8<-- "journey/code/16-orbit-07.rs"
```

### 8. `src/camera.rs`

Check the rotation invariant after many small updates, not just one convenient angle.

Find this exact block:

```rust
    #[test]
    fn a_projected_point_lies_on_its_unprojected_ray() {
```

Replace that block with:

```rust
--8<-- "journey/code/16-orbit-08.rs"
```

### 9. `src/browser.rs`

Connect walk around the model to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
            "Pan Left",
            "Pan Right",
            "Orbit Right",
            "View Reset",
        ],
    );
```

Replace that block with:

```rust
--8<-- "journey/code/16-orbit-window-1.rs"
```

### 10. `src/browser.rs`

Connect walk around the model to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
                "pan left" => camera.pan(-0.25, 0.0),
                "pan right" => camera.pan(0.25, 0.0),
                "orbit right" => camera.rotate(std::f32::consts::FRAC_PI_4),
                "view reset" => camera = Camera { aspect: camera.aspect, ..Camera::default() },
                _ => return,
            }
```

Replace that block with:

```rust
--8<-- "journey/code/16-orbit-window-2.rs"
```

### 11. `src/browser.rs`

Connect walk around the model to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Perspective and picking use the same camera.");
    Ok(())
}
```

Replace that block with:

```rust
--8<-- "journey/code/16-orbit-window-3.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run `Orbit Up` to see the triangles at different heights. `Orbit Right` moves around the target. `View Isometric` chooses a repeatable oblique view. Click a visible surface after each change and check the selection.

**Actual Chrome screenshot.**

Look at the two triangles from Isometric: the changed overlap comes from the camera, not edited vertex positions.

![Actual browser result: Walk around the model.](../screenshots/journey/16-orbit-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
cargo test --lib --locked -j4
```

## Try one small experiment

Type `View Isometric` and press Enter, pan, then tilt. Predict whether the point at the screen centre will stay centred during the tilt. It should: pan chose a new target, and orbit keeps that target fixed. Delete a selected surface and undo; the camera should keep its current orientation.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

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

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

The maintained camera uses target, distance, orientation and world-up in the same way. Later gesture lessons turn pointer deltas into these operations; unit conversion, fit-to-scene and large-coordinate precision extend the camera without moving source geometry.

[Validation status and course release](release.md).
