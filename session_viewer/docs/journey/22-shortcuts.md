# 22 · Give the keyboard a place to work

**Plan about 2–4 hours.** 200 lines to type, including comments and blank lines. Allow time to read, predict and experiment; this is an estimate, not a deadline.

**Today:** Zoom with the wheel and use focused keyboard shortcuts without stealing keys from the rest of the page.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Canvas focus → browser event → shortcuts → existing Editor action → scene or camera → frame.

**Before you finish, explain:** Why does scrolling need units, while a keyboard shortcut needs an owner?

Click the drawing, then press an arrow. Now Tab to a command and press an arrow again. The key is the same; its owner has changed. This boundary matters when we later add a command box: typing in that box must not move the camera or delete a selected object.

We add one small module, `shortcuts.rs`, beside `gesture.rs`. A gesture remembers a press. A shortcut needs no memory: input goes in and an optional Action comes out. Neither module owns a camera, history or mesh. Those still belong to Editor. Rust’s `Option<Action>` lets us say “this input has no viewer command” and leave the page alone.

A wheel reports pixels, lines or pages. We read its mode first and choose sixteen CSS pixels per line and one canvas height per page as our navigation policy. These are sensitivity choices, not physical measurements. An exponential turns the signed distance into a positive zoom factor: opposite small movements cancel, and ten small movements match one equal total movement. We cap a single event at 600 pixels; Camera keeps its existing distance limits.

![Wheel units and focused keys meet at the existing Action and Editor boundary.](../illustrations/journey-22.svg)

The browser rules come from [wheel delta modes](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaMode), [listener options](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener) and [text composition](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/isComposing). We request a non-passive wheel listener so a handled zoom can stop page scrolling.

## Type the change

Continue [Remember a press until it ends](21-gestures.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-22-shortcuts`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/shortcuts.rs`

Translate wheel units and key names into existing actions. A match arm ending in None means the viewer has no command for that input.

Create the file and type:

```rust
--8<-- "journey/code/22-shortcuts-01.rs"
```

### 2. `src/shortcut_tests.rs`

Compare equivalent wheel inputs, repeat behaviour and undo results. The tests exercise the same Editor that the page uses.

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
--8<-- "journey/code/22-shortcuts-dock-02.rs"
```

### 5. `src/browser.rs`

Connect give the keyboard a place to work to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

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
--8<-- "journey/code/22-shortcuts-dock-03.rs"
```

### 6. `src/browser.rs`

Connect give the keyboard a place to work to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

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
--8<-- "journey/code/22-shortcuts-dock-04.rs"
```

### 7. `src/browser.rs`

Connect give the keyboard a place to work to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

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
--8<-- "journey/code/22-shortcuts-dock-05.rs"
```

### 8. `index.html`

Keep the HTML page small. Feature input belongs to the command dock drawn inside the canvas.

Find this exact block:

```html
  <style>
    body { margin: 2rem auto; padding: 0 1rem; max-width: 960px; font: 18px/1.5 system-ui; color: #172238; }
    canvas { touch-action: none; display: block; width: 100%; height: min(60vh, 480px); background: #e9e9ec; outline: 1px solid #455b6b; }
  </style>
</head>
<body>
  <h1>My viewer</h1>
  <p id="status" role="status">Waiting for Rust…</p>
  <canvas id="canvas" tabindex="0" width="640" height="480" aria-label="Viewer drawing"></canvas>
  <p>Commands: Help · Example Box · Example Triangle · Select Next · Delete · Undo · Redo · Background · Zoom In · Zoom Out · Pan Left · Pan Right · Orbit Right · Orbit Up · View Isometric · View Reset. Type in the white Command field and press Enter.</p>
</body>
</html>
```

Replace that block with:

```html
--8<-- "journey/code/22-shortcuts-page-1.html"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run `Example Box` and `View Isometric`. Click the drawing, then try arrows, +/− and Home. Wheel over the drawing: it zooms without scrolling the page. Click the Command field and use arrow or Delete keys: they edit the command without moving or deleting geometry. Start a right drag and press Escape; further movement must not orbit.

**Actual Chrome screenshot.**

Wheel input zooms and focused arrow keys pan. The browser check also confirms that wheel zoom does not scroll the page and keys outside the canvas do not navigate.

![Actual browser result: Give the keyboard a place to work.](../screenshots/journey/22-shortcuts-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
cargo test --lib --locked -j4
```

## Try one small experiment

Change the wheel sensitivity from 0.002 to 0.001, predict the difference, and try it before restoring the value. Then explain why Ctrl/Cmd+Z still restores an object after several wheel movements: navigation never enters document history. Finally Tab away during a drag; the canvas blur listener should cancel it.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

A wheel delta can mean pixels, lines or pages, so its number alone is incomplete. A key has a different ambiguity: which part of the page should use it? We normalize wheel units and attach key handling only to the focusable canvas. Both routes then produce the same actions as our commands.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 22-shortcuts
npm --prefix ../session_tests run course -- save 22-shortcuts
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

The maintained viewer routes input through src/app/input.rs and command handling. We now have three input sources—commands, remembered gestures and stateless shortcuts—sharing Editor. Loading files and adding panels can extend that boundary without creating another undo stack. Zoom remains centred on the camera target here; zoom around the pointer and drawing-tool key maps come later.

[Validation status and course release](release.md).
