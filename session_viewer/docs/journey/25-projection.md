# 25 · Choose how depth changes size

**Typing: 58–116 minutes.** [Estimate](typing-load.md).

**Splitting required:** this verified checkpoint exceeds the one-hour typing target. Smaller runnable lessons are still being prepared.

Add orthographic projection beside perspective. Perspective makes distant objects smaller; orthographic keeps equal lengths equally sized at different depths. Orbit direction remains independent of this choice.

Preserve screen size at the plane through the camera target. Perspective half-height there is `distance × tan(30°)`; give the orthographic rectangle that half-height. Geometry away from that plane changes apparent size when switching.

## Type

Continue from [Find the whole scene](24-fit.md). [Save or recover your work](recovery.md).

### 1. `src/camera.rs`

Name the projection choices with an enum; derive copying and equality for this small value.

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

Fit the sphere through a perspective half-angle or the smaller orthographic half-extent.

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

Choose the projection matrix; match orthographic size at the target plane and retain visible depth in 0–1.

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

Carry the requested projection in Action for the named view commands.

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

Change projection without moving the camera or creating a document history entry.

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

Check target-plane scale, parallel picking rays, fitting, close selection, and document history preservation.

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

Add the two named projection commands.

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

Map each projection command to its enum value.

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

Refresh the reported projection after applying an action.

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

Expose the current projection on the canvas for inspection.

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

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Open `sample.pb`. Type `View Isometric`, `View Orthographic`, `Fit`, then `View Perspective`. The target stays fixed while depth changes the relative sizes.

**Verified checkpoint in Chrome.**

![Actual browser result: Choose how depth changes size.](../screenshots/journey/25-projection-browser.png)

[Verification scope](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Code explanation and diagram</summary>

The depth buffer still chooses the nearest surface. Orthographic clipping may extend behind the notional eye, avoiding an eye-shaped cut when zoomed into geometry.

Camera owns projection and bounds. Editor changes the choice; drawing and picking consume the resulting matrix.

projection command → Editor → Camera matrix → drawing and inverse-matrix picking.

![Perspective rays spread from an eye; orthographic rays remain parallel.](../illustrations/journey-25.svg)

Why can the same ray-picking code work in both views even though their rays have different shapes?

Camera builds the selected projection and combines it with the same view transform. Drawing uses that matrix. Picking inverts it and maps a screen point at depths 0 and 1 back into the world. Those two points define the correct ray for either matrix. Perspective rays spread out; orthographic rays are parallel. Their different origins and directions come from the matrix, not a second picking routine.

Study estimate, including typing and experiments: 3–5 hours.

</details>

<details>
<summary>Optional experiment</summary>

Switch modes while looking squarely at the two original triangles. Which one changes size more? Their depths differ, and neither lies on the target plane. Now orbit: the mode should stay selected while the viewing direction changes. Explain why Isometric and Orthographic are not synonyms.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 25-projection
npm --prefix ../session_tests run course -- save 25-projection
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

The production camera uses this same target-plane scale relationship. Its tighter fitting, units, named views and camera-relative coordinates come next. Production also uses reversed depth for precision; this checkpoint keeps the existing 0-near/1-far depth convention. Switching modes deliberately preserves the view parameters rather than automatically refitting; run Fit if the new perspective clips or crops a nearby part.

The orange frame is fitted in orthographic view; the pink Orthographic command records the active choice. Chrome checks mouse and keyboard switching, exact perspective/orthographic round trips, visible-face selection in both modes, and fitting at wide and tall viewport sizes.

[Full validation scope](release.md).

</details>
