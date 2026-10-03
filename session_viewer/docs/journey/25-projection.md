# 25 · Choose how depth changes size

**Combined study estimate: 3–5 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 58–116 minutes.** 132 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Splitting required:** this verified checkpoint exceeds the one-hour typing target. Smaller runnable lessons are still being prepared.

**Today:** Switch between perspective and orthographic views while keeping fitting, zoom and picking coherent.

**Follow:** projection command → Editor → Camera matrix → drawing and inverse-matrix picking.

Add orthographic projection beside perspective. Perspective makes distant objects smaller; orthographic keeps equal lengths equally sized at different depths. Orbit direction remains independent of this choice.

Preserve screen size at the plane through the camera target. Perspective half-height there is `distance × tan(30°)`; give the orthographic rectangle that half-height. Geometry away from that plane changes apparent size when switching.

The depth buffer still chooses the nearest surface. Orthographic clipping may extend behind the notional eye, avoiding an eye-shaped cut when zoomed into geometry.

Camera owns projection and bounds. Editor changes the choice; drawing and picking consume the resulting matrix.

![Perspective rays spread from an eye; orthographic rays remain parallel.](../illustrations/journey-25.svg)

## Type the change

Continue from [Find the whole scene](24-fit.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-25-projection` (from `session_viewer`).

### 1. `src/camera.rs`

Name the two choices instead of passing an unexplained true or false. Copy lets us pass this small enum by value; PartialEq and Eq let us compare it. HALF_FOV is the same 30-degree half-angle used by the existing camera.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Ray {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/25-projection-01.rs"
```

### 2. `src/camera.rs`

Projection belongs to the view, alongside its orientation and size. It is not a property of an object.

<details>
<summary>Locate the existing block</summary>

```rust
    pub aspect: f64,
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/25-projection-02.rs"
```

### 3. `src/camera.rs`

Start with the perspective view you already know. Reset view will restore this default too.

<details>
<summary>Locate the existing block</summary>

```rust
            aspect: 640.0 / 480.0,
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/25-projection-03.rs"
```

### 4. `src/camera.rs`

Use the shared angle so fitting and drawing cannot quietly use different fields of view.

<details>
<summary>Locate the existing block</summary>

```rust
        let half_y = 30.0_f64.to_radians();
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/25-projection-04.rs"
```

### 5. `src/camera.rs`

Perspective keeps the enclosing-sphere calculation from lesson 24. Orthographic fits that sphere inside a rectangle: half-height is distance × tan(30°), and half-width is half-height × aspect. The smaller side must hold the radius and margin.

<details>
<summary>Locate the existing block</summary>

```rust
        self.distance = self.radius * 1.1 / half_x.min(half_y).sin();
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/25-projection-05.rs"
```

### 6. `src/camera.rs`

The orthographic rectangle matches the perspective view at the target plane. Its depth interval extends behind and ahead of the eye; this lets parallel rays reach the scene even when a close zoom puts the eye inside it. Both matrices still map visible depth to 0–1.

<details>
<summary>Locate the existing block</summary>

```rust
        let projection = Xform::perspective(60.0_f64.to_radians(), self.aspect, near, far);
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/25-projection-06.rs"
```

### 7. `src/editor.rs`

An action carries the requested projection. One route serves both commands and any future keyboard shortcut.

<details>
<summary>Locate the existing block</summary>

```rust
    Fit,
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/25-projection-07.rs"
```

### 8. `src/editor.rs`

Changing projection preserves target, distance and orientation. It returns Change::View and leaves document history alone. Isometric remains an orientation choice; it does not silently choose a projection.

<details>
<summary>Locate the existing block</summary>

```rust
                    Action::Isometric => self.camera.isometric(),
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/25-projection-08.rs"
```

### 9. `src/projection_tests.rs`

Check the geometric promises: equal scale on the target plane, depth-independent orthographic size, parallel pick rays, complete fitting, and selection at close zoom. The final test also checks that switching modes never becomes an undoable document edit.

Create the file and type:

```rust
--8<-- "journey/code/25-projection-14.rs"
```

### 10. `src/lib.rs`

Register the new checks without changing the production browser entry point.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod fit_tests;
#[cfg(test)]
mod navigation_tests;
#[cfg(test)]
mod gesture_tests;
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/25-projection-fullscreen-1.rs"
```

### 11. `src/browser.rs`

Connect choose how depth changes size to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

<details>
<summary>Locate the existing block</summary>

```rust
            "Orbit Right",
            "Orbit Up",
            "View Isometric",
            "Fit",
            "View Reset",
        ],
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/25-projection-window-1.rs"
```

### 12. `src/browser.rs`

Connect choose how depth changes size to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

<details>
<summary>Locate the existing block</summary>

```rust
                    "orbit right" => Action::Orbit(std::f64::consts::FRAC_PI_4, 0.0),
                    "orbit up" => Action::Orbit(0.0, std::f64::consts::FRAC_PI_6),
                    "view isometric" => Action::Isometric,
                    "fit" => Action::Fit,
                    "view reset" => Action::ResetView,
                    _ => return,
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/25-projection-window-2.rs"
```

### 13. `src/browser.rs`

Connect choose how depth changes size to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

<details>
<summary>Locate the existing block</summary>

```rust
                }
            }
        }
        if event.type_() == "viewer-file" {
            report("File imported. Undo removes the entire import.");
        }
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/25-projection-window-3.rs"
```

### 14. `src/browser.rs`

Connect choose how depth changes size to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

<details>
<summary>Locate the existing block</summary>

```rust
    Ok(())
}

fn navigation_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/25-projection-window-4.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Open `sample.pb`. Type `View Isometric`, `View Orthographic`, `Fit`, then `View Perspective`. The target stays fixed while depth changes the relative sizes.

**Verified checkpoint in Chrome.**

![Actual browser result: Choose how depth changes size.](../screenshots/journey/25-projection-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Switch modes while looking squarely at the two original triangles. Which one changes size more? Their depths differ, and neither lies on the target plane. Now orbit: the mode should stay selected while the viewing direction changes. Explain why Isometric and Orthographic are not synonyms.

</details>

## Explain the change

Why can the same ray-picking code work in both views even though their rays have different shapes?

<details>
<summary>Compare your explanation</summary>

Camera builds the selected projection and combines it with the same view transform. Drawing uses that matrix. Picking inverts it and maps a screen point at depths 0 and 1 back into the world. Those two points define the correct ray for either matrix. Perspective rays spread out; orthographic rays are parallel. Their different origins and directions come from the matrix, not a second picking routine.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 25-projection
npm --prefix ../session_tests run course -- save 25-projection
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

The production camera uses this same target-plane scale relationship. Its tighter fitting, units, named views and camera-relative coordinates come next. Production also uses reversed depth for precision; this checkpoint keeps the existing 0-near/1-far depth convention. Switching modes deliberately preserves the view parameters rather than automatically refitting; run Fit if the new perspective clips or crops a nearby part.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

The orange frame is fitted in orthographic view; the pink Orthographic command records the active choice. Chrome checks mouse and keyboard switching, exact perspective/orthographic round trips, visible-face selection in both modes, and fitting at wide and tall viewport sizes.

[Full validation scope](release.md).

</details>
