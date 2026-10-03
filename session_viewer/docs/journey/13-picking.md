# 13 · Ask which object is under the pointer

**Combined study estimate: 2–4 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 47–93 minutes.** 116 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Splitting required:** this verified checkpoint exceeds the one-hour typing target. Smaller runnable lessons are still being prepared.

**Today:** Click a visible triangle to select its stable object ID, including after camera movement.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Browser click → canvas-relative coordinates → inverse camera → triangle coverage and depth → ObjectId → highlight.

**Before you finish, explain:** Why must picking undo the camera transform before testing the stored triangles?

We can already select an object by its place in a list. Now let us select the object we point at. The first job is to make sure the pointer and geometry are speaking the same coordinate language.

A mouse event gives a position in the browser viewport, measured in CSS pixels. Subtract the canvas rectangle's left and top to get a position within the canvas. Divide by its displayed width and height, then map to −1 through +1. Browser y increases downward; our drawing y increases upward, so we reverse that axis.

![A browser click becomes a canvas coordinate, then a world point; triangle coverage and nearest depth produce a stable object ID.](../illustrations/journey-13.svg)

The camera must undo its display transformation next. Divide by scale, rotate back, then add the camera centre. This is the inverse of the transformation we wrote in the matrix lesson. The test sends a known point through both directions and checks that it comes back to the same place.

Our current camera leaves depth unchanged and looks straight through a flat arrangement of triangles. For this case we can test coverage on the CPU. A triangle point can be written as `a + u×(b−a) + v×(c−a)`. It is inside when u and v are nonnegative and their sum is at most 1. The little cross-product calculation finds those two weights. The same weights interpolate depth, letting us choose the nearest covered triangle.

We return an ObjectId, not the triangle number or vector row. Selection therefore continues to work after an earlier object is deleted. Clicking empty space returns None and clears the highlight.

This small CPU query teaches coordinate conversion and visibility. It does not reproduce every rasterizer edge rule, and it visits every triangle. The full viewer's GPU picking will provide precise rendered IDs for large scenes and a 3D camera; it will keep this same boundary between an input location and a resolved object identity.

## Type the change

Continue [Name objects without depending on their row](12-identity.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-13-picking`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/camera.rs`

Add the inverse conversion for our flat camera. Rust’s sin_cos returns the sine and cosine as a pair.

Find this exact block:

```rust
    pub fn uniform(&self) -> [f32; 16] {
```

Replace that block with:

```rust
--8<-- "journey/code/13-picking-03.rs"
```

### 2. `src/picking.rs`

Create the CPU query and tests. It reads scene data and returns identity without changing selection or GPU resources.

Create the file and type:

```rust
--8<-- "journey/code/13-picking-04.rs"
```

### 3. `Cargo.toml`

Enable the browser event bindings used by the command dock. serde records the drawn field for browser verification; the same code still receives real keyboard events.

Find this exact block:

```toml

[dependencies]
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "Event", "PointerEvent", "MouseEvent", "KeyboardEvent", "WheelEvent", "FocusOptions", "DomRect", "HtmlElement"] }
console_error_panic_hook = "=0.1.7"
wasm-bindgen-futures = "=0.4.78"
wgpu = "=29.0.4"
```

Replace that block with:

```toml
--8<-- "journey/code/13-picking-dock-01.toml"
```

### 4. `src/lib.rs`

Register the query module.

Find this exact block:

```rust
pub mod camera;
pub mod mesh;
pub mod scene;
pub mod gpu_mesh;
pub mod renderer;
#[cfg(target_arch = "wasm32")]
```

Replace that block with:

```rust
--8<-- "journey/code/13-picking-fullscreen-1.rs"
```

### 5. `src/browser.rs`

Connect ask which object is under the pointer to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
            }
        };

        if let Some(line) = line {
            match line.as_str() {
                "example triangle" => {
                    scene.toggle_extra();
                    selected = selected.filter(|id| scene.contains(*id));
```

Replace that block with:

```rust
--8<-- "journey/code/13-picking-window-1.rs"
```

### 6. `src/browser.rs`

Connect ask which object is under the pointer to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
        "keydown",
        "keyup",
        "blur",
    ] {
        canvas.add_event_listener_with_callback(name, click.as_ref().unchecked_ref())?;
    }
```

Replace that block with:

```rust
--8<-- "journey/code/13-picking-window-2.rs"
```

### 7. `src/browser.rs`

Connect ask which object is under the pointer to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Selection follows object identity, even when rows move.");
    Ok(())
}
```

Replace that block with:

```rust
--8<-- "journey/code/13-picking-window-3.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Click the pink triangle, then the visible turquoise part. Each becomes yellow. Click their overlap: the nearer object must win. Click empty space to clear selection. Use `Pan Right`, `Zoom Out` and `Orbit Right`, then repeat the picks.

**Actual Chrome screenshot.**

![Actual browser result: Ask which object is under the pointer.](../screenshots/journey/13-picking-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
cargo test --lib --locked -j4
```

## Try one small experiment

Select the front triangle at the overlap and delete it. Click that same place again. The far triangle should now be selected with its original ID. Then move the camera and repeat: explain the coordinate conversions rather than memorising a pixel position.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

The click describes a place on the displayed image, while the mesh vertices describe positions in the scene. Undoing camera rotation, scale and translation brings the click into the same coordinate system as those vertices. Comparing them without that conversion would select the wrong place after moving the view.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 13-picking
npm --prefix ../session_tests run course -- save 13-picking
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Production picking renders integer identifiers, reads a small result asynchronously, rejects stale results and resolves the displayed row back to source identity. The conversion and ownership questions introduced here remain the same when the implementation becomes more capable.

[Validation status and course release](release.md).
