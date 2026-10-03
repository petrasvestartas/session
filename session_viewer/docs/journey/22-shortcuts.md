# 22 · Keep navigation on the mouse and commands in the dock

**Combined study estimate: 2–4 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 42–84 minutes.** 111 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Splitting required:** this verified checkpoint exceeds the one-hour typing target. Smaller runnable lessons are still being prepared.

**Today:** Normalize wheel input and keep every keyboard feature command in the command dock.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Wheel units → navigation::wheel → Editor action → camera → frame; typed text → command dock → Editor action.

**Before you finish, explain:** Why does a wheel need units, while typed text needs one owner?

Click the drawing and type `Pan Right`. The first letter opens the dock and stays in the field. Enter runs the complete command. Arrow and Delete keys edit text when the field owns focus; they never become camera or document shortcuts.

We add `navigation.rs` beside `gesture.rs`. A gesture remembers a press. The wheel translator has no memory: a number, its unit and the canvas height become an optional Action. Neither module owns a camera, history or mesh. Those still belong to Editor. `Option<Action>` expresses that an invalid or zero movement has no navigation action.

A wheel reports pixels, lines or pages. We choose sixteen CSS pixels per line and one canvas height per page. These are sensitivity choices, not physical measurements. An exponential turns signed distance into a positive zoom factor: opposite small movements cancel, and ten small movements match one equal total movement. We cap a single event at 600 pixels; Camera keeps its existing distance limits.

![Wheel navigation and typed commands meet at the existing Editor boundary.](../illustrations/journey-22.svg)

The dock already receives printable keys from lesson 03d. The browser navigation dispatcher handles only the wheel and pointer gestures. Escape cancels an active gesture; it is a lifecycle key, not a feature shortcut. This keeps one route for Undo, Delete and view commands.

The browser rules come from [wheel delta modes](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaMode) and [listener options](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener). A non-passive wheel listener lets a handled zoom stop page scrolling.

## Type the change

Continue [Remember a press until it ends](21-gestures.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-22-shortcuts`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/navigation.rs`

Normalize wheel units into the existing Zoom action. Zero or invalid input returns None.

Create the file and type:

```rust
--8<-- "journey/code/22-shortcuts-01.rs"
```

### 2. `src/navigation_tests.rs`

Compare equivalent wheel inputs, bound large jumps, and show that navigation preserves document history.

Create the file and type:

```rust
--8<-- "journey/code/22-shortcuts-02.rs"
```

### 3. `Cargo.toml`

Enable the browser event bindings used by the command dock. serde records the drawn field for browser verification; the same code still receives real keyboard events.

Find this exact block:

```toml

[dependencies]
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "FocusOptions", "HtmlElement"] }
console_error_panic_hook = "=0.1.7"
wasm-bindgen-futures = "=0.4.78"
wgpu = "=29.0.4"
```

Replace that block with:

```toml
--8<-- "journey/code/22-shortcuts-dock-01.toml"
```

### 4. `src/lib.rs`

Register the small input translator and its tests.

Find this exact block:

```rust
pub mod editor;
pub mod viewport;
pub mod gesture;
#[cfg(test)]
mod gesture_tests;
pub mod gpu_mesh;
```

Replace that block with:

```rust
--8<-- "journey/code/22-shortcuts-fullscreen-1.rs"
```

### 5. `src/browser.rs`

Route wheel and pointer events through navigation. Leave keyboard feature commands with the dock.

Find this exact block:

```rust
            };
            Some(action)
        } else if !panel.consumed {
            pointer_action(&event, &pointer_canvas, &mut gesture)
        } else {
            None
        };
```

Replace that block with:

```rust
--8<-- "journey/code/22-shortcuts-window-1.rs"
```

### 6. `src/browser.rs`

Route wheel and pointer events through navigation. Leave keyboard feature commands with the dock.

Find this exact block:

```rust
    window.add_event_listener_with_callback("blur", update.as_ref().unchecked_ref())?;
    // Both event sources retain this one callback for the lifetime of the page.
    update.forget();
    report("Right-drag to orbit. Left-click to select. A cancelled drag stops immediately.");
    Ok(())
}

fn pointer_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
```

Replace that block with:

```rust
--8<-- "journey/code/22-shortcuts-window-2.rs"
```

### 7. `src/browser.rs`

Route wheel and pointer events through navigation. Leave keyboard feature commands with the dock.

Find this exact block:

```rust
        "pointerdown" => {
            if pointer.is_primary() && gesture.press(id, pointer.button(), position) {
                event.prevent_default();
                if canvas.set_pointer_capture(id).is_err() {
                    gesture.cancel();
                }
            }
```

Replace that block with:

```rust
--8<-- "journey/code/22-shortcuts-window-3.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run `Example Box` and `View Isometric`. Wheel over the drawing: it zooms without scrolling the page. Click the drawing and type `Pan Right`, then Enter. Type a single letter without Enter: it must appear in the dock without changing the picture. Arrow and Delete keys must never move or delete geometry. Start a right drag and press Escape; further movement must not orbit.

**Actual Chrome screenshot.**

Wheel input zooms and the Pan Right command pans. Typing after clicking the drawing activates the dock without invoking keyboard feature shortcuts.

![Actual browser result: Keep navigation on the mouse and commands in the dock.](../screenshots/journey/22-shortcuts-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
cargo test --lib --locked -j4
```

## Try one small experiment

Change wheel sensitivity from 0.002 to 0.001, predict the difference, and try it before restoring the value. Run `Undo` after several wheel movements: it removes the box while preserving the camera. Run `Redo` to restore it. Explain why navigation does not enter document history. Finally Tab away during a drag; the canvas blur listener should cancel it.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

A wheel delta may represent pixels, lines or pages. Normalize those units before zooming. Typed text belongs to the command dock, even after clicking the drawing. There is no second keyboard feature map to accidentally move the camera or delete geometry.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 22-shortcuts
npm --prefix ../session_tests run course -- save 22-shortcuts
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

The maintained viewer routes mouse and phone navigation through src/app/input.rs. Printable input activates the command dock immediately; named commands share the existing state and history. This checkpoint keeps wheel zoom centred on the camera target. Pointer-centred zoom and phone gestures will extend navigation without introducing keyboard feature shortcuts.

[Validation status and course release](release.md).
