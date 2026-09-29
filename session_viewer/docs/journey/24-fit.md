# 24 · Find the whole scene

**Plan about 3–5 hours.** 169 lines to type, including comments and blank lines. Allow time to read, predict and experiment; this is an estimate, not a deadline.

**Today:** Frame all current objects without rotating them or changing the document.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Fit command → Editor → Scene bounds → Camera target, distance and clipping → existing Renderer.

**Before you finish, explain:** Why does Fit need both the scene bounds and the window shape, but no new GPU mesh?

Imagine opening a model drawn far from the origin. The file loaded correctly, but the canvas looks empty. More clicking is not a reliable way to find it. Let us give the viewer a command that asks: “How much space do all these objects occupy?”

![Scene bounds and a camera fitting the enclosing sphere.](../illustrations/journey-24.svg)

**Keep the ownership clear.** Scene can measure its vertices. Camera can decide where to look. Editor connects those two decisions. Renderer already knows how to draw with a new camera matrix, so we leave its code alone.

A box is easy to measure; a sphere around that box is easy to fit from any direction. The sphere can leave extra empty space around a long, thin object. That is a deliberate first fit: predictable, small, and testable. A later tighter fit can project the eight box corners without changing the command or renderer.

Think of the viewing angle as a pair of scissors opened at the eye. In a tall window, the horizontal opening is narrower. We must fit through that opening too. The diagram shows the right triangle behind the one-line distance calculation.

![The radius is opposite the half-angle in a right triangle from the eye to the sphere.](../illustrations/journey-24-fit.svg)

## Type the change

Continue [Keep the document behind the picture](23-import.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-24-fit`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/bounds.rs`

A box needs only its smallest and largest coordinate on each axis. Its centre is halfway between them. Half its diagonal is the radius of a sphere containing every corner. array::from_fn fills the three entries by calling the small expression once per index.

Create the file and type:

```rust
--8<-- "journey/code/24-fit-01.rs"
```

### 2. `src/scene.rs`

Walk the displayed positions once. The first vertex starts the box; later vertices enlarge it. None means there was no vertex at all. We use f64 for the calculation even though the GPU mesh stores f32.

Find this exact block:

```rust
    pub fn objects(&self) -> &[Object] {
```

Replace that block with:

```rust
--8<-- "journey/code/24-fit-03.rs"
```

### 3. `src/camera.rs`

Keep the fitted scene scale with the camera. Distance says where the eye is; radius says how large the scene is.

Find this exact block:

```rust
    pub distance: f64,
```

Replace that block with:

```rust
--8<-- "journey/code/24-fit-04.rs"
```

### 4. `src/camera.rs`

The original small demonstration scene starts with a one-unit radius. Fit will measure the real bounds.

Find this exact block:

```rust
            distance: 3.0,
```

Replace that block with:

```rust
--8<-- "journey/code/24-fit-05.rs"
```

### 5. `src/camera.rs`

The smaller half-angle limits the view: horizontal in a tall window, vertical in a wide one. A tangent from the eye to the enclosing sphere gives sin(angle) = radius / distance. Multiply by 1.1 for a little breathing room. A single point has no size, so give it a one-unit viewing scale.

Find this exact block:

```rust
    pub fn pan(&mut self, dx: f32, dy: f32) {
```

Replace that block with:

```rust
--8<-- "journey/code/24-fit-06.rs"
```

### 6. `src/camera.rs`

Keep the existing zoom limits proportional to the measured scene. Otherwise the first wheel event after fitting a large model would jump back to a distance of 50 units.

Find this exact block:

```rust
            self.distance = (self.distance / factor as f64).clamp(0.2, 50.0);
```

Replace that block with:

```rust
--8<-- "journey/code/24-fit-07.rs"
```

### 7. `src/camera.rs`

Move the clipping planes with the viewing scale too. The near plane stays in front of the eye; the far plane reaches beyond the fitted sphere. A fixed far plane at 100 would erase a large fitted scene. Zooming inside geometry can still clip it, as it should.

Find this exact block:

```rust
        let projection = Xform::perspective(60.0_f64.to_radians(), self.aspect, 0.01, 100.0);
```

Replace that block with:

```rust
--8<-- "journey/code/24-fit-08.rs"
```

### 8. `src/editor.rs`

Fit is an explicit action, like Isometric. Import still changes the document; Fit only changes how you look at it.

Find this exact block:

```rust
    ResetView,
```

Replace that block with:

```rust
--8<-- "journey/code/24-fit-09.rs"
```

### 9. `src/editor.rs`

Read the current scene, including imported meshes. An empty scene leaves the camera alone. This branch returns Change::View, so it neither uploads meshes nor creates a history entry.

Find this exact block:

```rust
                    Action::Isometric => self.camera.isometric(),
```

Replace that block with:

```rust
--8<-- "journey/code/24-fit-10.rs"
```

### 10. `src/fit_tests.rs`

Test the promise rather than one distance: every box corner must survive projection and clipping. Also check that Fit preserves selection, leaves the import as one undoable action, and handles an empty scene or one point.

Create the file and type:

```rust
--8<-- "journey/code/24-fit-13.rs"
```

### 11. `src/lib.rs`

Register the bounds module beside the camera. It owns numbers, not GPU resources.

Find this exact block:

```rust
pub mod background;
pub mod camera;
pub mod mesh;
pub mod scene;
pub mod picking;
```

Replace that block with:

```rust
--8<-- "journey/code/24-fit-dock-01.rs"
```

### 12. `src/lib.rs`

Register the bounds module beside the camera. It owns numbers, not GPU resources.

Find this exact block:

```rust
#[cfg(test)]
mod document_tests;
#[cfg(test)]
mod shortcut_tests;
#[cfg(test)]
mod gesture_tests;
```

Replace that block with:

```rust
--8<-- "journey/code/24-fit-dock-02.rs"
```

### 13. `src/browser.rs`

Connect find the whole scene to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
            "Orbit Right",
            "Orbit Up",
            "View Isometric",
            "View Reset",
        ],
    );
```

Replace that block with:

```rust
--8<-- "journey/code/24-fit-dock-03.rs"
```

### 14. `src/browser.rs`

Connect find the whole scene to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
                    "orbit right" => Action::Orbit(std::f64::consts::FRAC_PI_4, 0.0),
                    "orbit up" => Action::Orbit(0.0, std::f64::consts::FRAC_PI_6),
                    "view isometric" => Action::Isometric,
                    "view reset" => Action::ResetView,
                    _ => return,
                };
```

Replace that block with:

```rust
--8<-- "journey/code/24-fit-dock-04.rs"
```

### 15. `index.html`

Keep the HTML page small. Feature input belongs to the command dock drawn inside the canvas.

Find this exact block:

```html
    Home resets. Delete removes the selection. Ctrl/Cmd+Z undoes; Shift adds redo.</p>
  <canvas id="canvas" tabindex="0" width="640" height="480"
    aria-label="Viewer drawing" aria-describedby="navigation"></canvas>
  <p>Commands: Help · Open · Example Box · Example Triangle · Select Next · Delete · Undo · Redo · Background · Zoom In · Zoom Out · Pan Left · Pan Right · Orbit Right · Orbit Up · View Isometric · View Reset. Type in the white Command field and press Enter.</p>
  <input id="open" type="file" accept=".pb" hidden>
</body>
</html>
```

Replace that block with:

```html
--8<-- "journey/code/24-fit-page-1.html"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Create `sample.pb` as in lesson 23. Run `Open` and choose it, then `View Isometric`, `Pan Right` three times, and `Fit`. The whole scene returns with a margin; the isometric orientation stays.

Run `Undo`: the import disappears although Fit came later. `Redo` restores it. Narrow the browser and run `Fit` again. Resizing keeps the distance; Fit recalculates it for the new window shape.

**Actual Chrome screenshot.**

The orange frame and both triangles are centred together after three pans and Fit scene. Chrome also checks that a second Fit leaves the drawing unchanged, that Fit recovers after another pan, and that the imported file remains one undoable action.

![Actual browser result: Find the whole scene.](../screenshots/journey/24-fit-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
cargo test --lib --locked -j4
```

## Try one small experiment

Before running Fit, predict whether a tall window needs the eye farther away. Resize, fit and compare. Then temporarily change the 1.1 margin to 1.4: the scene should become smaller, not larger. Restore 1.1 when you finish.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

Scene walks the displayed vertices to find a box. Camera aims at its centre and backs away until the enclosing sphere fits the smaller view angle. The window aspect decides which angle is smaller. Radius also scales zoom limits and clipping. Only the camera uniform changes; object IDs, mesh buffers and history stay as they were.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 24-fit
npm --prefix ../session_tests run course -- save 24-fit
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Production also measures scene bounds and keeps fitting out of document history. Its corner-based fit is tighter and adds units, selection-only bounds and orthographic views. This checkpoint fits all displayed meshes and assumes ordinary coordinates that f32 can represent accurately. Very large offsets with tiny details need a later coordinate-origin strategy; fitting alone cannot restore precision lost in mesh storage. Pan commands still move by a fixed world-unit amount.

[Validation status and course release](release.md).
