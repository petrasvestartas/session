# 21 · Remember a press until it ends

**Plan about 3–5 hours.** 219 lines to type, including comments and blank lines. Allow time to read, predict and experiment; this is an estimate, not a deadline.

**Today:** Orbit with a right drag, pick with a left click, and stop safely when the pointer or window loses focus.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Pointer event → Gesture memory → Motion → Editor action → camera or selection → frame.

**Before you finish, explain:** Why do we need both browser pointer capture and our own active pointer ID?

Put your finger on the table. Move it, then lift it. Those are three events, but you understand them as one gesture because you remember the press. Our viewer needs that little piece of memory too.

A click waits until release. If a left press travels more than four CSS pixels, it is no longer a click—even if it comes back. A right press turns each movement into an orbit delta. We already know how to orbit; this lesson only supplies the changing angles.

Rust’s `Option<Drag>` means “either one active drag, or none.” `as_mut()` lets us update the remembered position without taking the drag out. `take()` removes it on release. Keeping this small state machine outside the browser lets us test interrupted gestures as ordinary Rust calls.

![A press remembers one pointer; moves update the view; release may pick; cancellation clears the memory.](../illustrations/journey-21.svg)

## Type the change

Continue [Keep a changing window in proportion](20-resize.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-21-gestures`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/gesture.rs`

Keep pointer state separate from the document. Motion carries intent; converting it to an Action uses CSS dimensions, never the GPU pixel count.

Create the file and type:

```rust
--8<-- "journey/code/21-gestures-01.rs"
```

### 2. `src/gesture.rs`

A drag remembers its pointer ID and previous position. Option lets an event produce no action without pretending it was a click.

Find this exact block:

```rust
pub struct Gesture {
    active: Option<Drag>,
}
```

Replace that block with:

```rust
--8<-- "journey/code/21-gestures-02.rs"
```

### 3. `src/gesture_tests.rs`

Test the event sequences that usually hide bugs: another finger, a cancelled press, and a drag returning to its start. These tests need no browser.

Create the file and type:

```rust
--8<-- "journey/code/21-gestures-03.rs"
```

### 4. `src/lib.rs`

Expose the gesture module to the browser and compile its tests only in the test build.

Find this exact block:

```rust
pub mod viewport;
```

Replace that block with:

```rust
--8<-- "journey/code/21-gestures-04.rs"
```

### 5. `Cargo.toml`

Enable the browser bindings for pointer IDs, buttons and pointer capture. The dependency versions stay unchanged.

Find this exact block:

```toml
"MouseEvent", "DomRect"
```

Replace that block with:

```toml
--8<-- "journey/code/21-gestures-05.toml"
```

### 6. `src/browser.rs`

The browser translates platform events; the gesture module decides what those events mean.

Find this exact block:

```rust
use crate::viewport::Viewport;
```

Replace that block with:

```rust
--8<-- "journey/code/21-gestures-06.rs"
```

### 7. `src/browser.rs`

Remove canvas picking from click events. Pointer release now decides whether the press was actually a click, so one release cannot select twice.

Find this exact block:

```rust
                "canvas" => {
                    let rect = canvas.get_bounding_client_rect();
                    if rect.width() <= 0.0 || rect.height() <= 0.0 {
                        return;
                    }
                    Action::Pick([
                        (2.0 * (event.client_x() as f64 - rect.left()) / rect.width() - 1.0) as f32,
                        (1.0 - 2.0 * (event.client_y() as f64 - rect.top()) / rect.height()) as f32,
                    ])
                }
```

Delete this block.

### 8. `src/browser.rs`

The same callback owns the gesture, editor and renderer. Clone the browser handle so registration can still use the canvas after the callback captures its copy.

Find this exact block:

```rust
    let browser_window = window.clone();
```

Replace that block with:

```rust
--8<-- "journey/code/21-gestures-08.rs"
```

### 9. `src/browser.rs`

Both buttons and pointer gestures will produce the same Action value.

Find this exact block:

```rust
        if event.type_() == "click" {
```

Replace that block with:

```rust
--8<-- "journey/code/21-gestures-09.rs"
```

### 10. `src/browser.rs`

Apply the action once, regardless of where it came from. A camera change still leaves scene buffers alone.

Find this exact block:

```rust
            match editor.apply(action) {
```

Replace that block with:

```rust
--8<-- "journey/code/21-gestures-10.rs"
```

### 11. `src/browser.rs`

Pointer capture keeps movement arriving when the cursor leaves the canvas. Losing capture or window focus cancels the gesture.

Find this exact block:

```rust
    controls.remove_attribute("disabled")?;
```

Replace that block with:

```rust
--8<-- "journey/code/21-gestures-11.rs"
```

### 12. `src/browser.rs`

Tell the learner which two gestures are available at this checkpoint.

Find this exact block:

```rust
    report("Canvas pixels, depth and camera resize together.");
```

Replace that block with:

```rust
--8<-- "journey/code/21-gestures-12.rs"
```

### 13. `src/browser.rs`

Browser pointer capture is delivery, not application state. Keep the two in step, then convert the motion through the current CSS rectangle.

Find this exact block:

```rust
fn resize(
```

Replace that block with:

```rust
--8<-- "journey/code/21-gestures-13.rs"
```

### 14. `index.html`

Declare that canvas gestures belong to the viewer, so the browser does not turn a touch into page scrolling midway through a gesture.

Find this exact block:

```html
canvas { display: block;
```

Replace that block with:

```html
--8<-- "journey/code/21-gestures-14.html"
```

### 15. `src/browser.rs`

Use the callback’s canvas handle when resizing too. The original handle remains available while we register its listeners.

Find this exact block:

```rust
resize(&browser_window, &canvas, &surface, &mut config, &mut renderer, &mut editor)
```

Replace that block with:

```rust
--8<-- "journey/code/21-gestures-15.rs"
```

### 16. `src/browser.rs`

An idle pointer move or cancellation has no picture to change. Return without drawing; a resize still needs a new frame.

Find this exact block:

```rust
            }
        }
        if resize(&browser_window, &pointer_canvas
```

Replace that block with:

```rust
--8<-- "journey/code/21-gestures-16.rs"
```

### 17. `src/browser.rs`

Borrow the click event instead of moving it. The callback still needs the original event afterward to distinguish resizing from idle movement.

Find this exact block:

```rust
let Ok(event) = event.dyn_into::<web_sys::MouseEvent>() else { return; };
```

Replace that block with:

```rust
--8<-- "journey/code/21-gestures-17.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run `trunk serve`. Add a box and choose Isometric. Right-drag across the canvas and beyond its edge, then release. Move again: the camera must stay still. Left-click a face to select it. A left drag must not select. Resize or switch windows during a press; returning must not leave a stuck drag. Buttons still work with keyboard focus and Enter.

**Actual Chrome screenshot.**

A right drag turns the scene. The browser check then releases the button and confirms that further pointer movement leaves the camera still.

![Actual browser result: Remember a press until it ends.](../screenshots/journey/21-gestures-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
cargo test --lib --locked -j4
```

## Try one small experiment

Change the four-pixel click threshold to twenty. Try a short left drag, then restore four. Explain why this distance is measured in CSS pixels: doubling display density should not make the same hand movement harder to classify. Finally undo adding the box after orbiting. The box should disappear while the camera stays where you put it.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

Capture keeps delivering events outside the canvas. The ID tells our application which pointer owns the gesture. Neither replaces the other: a second pointer must not finish the first gesture, and lost capture must clear our state.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 21-gestures
npm --prefix ../session_tests run course -- save 21-gestures
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

The maintained viewer keeps richer gesture state and cancellation listeners in `src/app/input.rs`. This checkpoint establishes one active pointer and cancellation; touch orbit, wheel zoom, keyboard shortcuts and drawing tools will extend the same route rather than editing geometry inside DOM callbacks.

[Validation status and course release](release.md).
