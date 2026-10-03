# 08 · Move the view, keep the geometry

**Combined study estimate: 2–3 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 25–50 minutes.** 93 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Pan, zoom and reset a flat view through camera state.

**Follow:** command → camera centre or scale → four uniform values → existing renderer → unchanged mesh in a different view.

Place a drawing under a small window cut in a sheet of paper. Slide the window right. The drawing appears to move left inside it, although the drawing stayed on the table. A camera changes the place from which we see the scene.

We begin with a flat view so that one subtraction explains pan. `Camera.center` says which world point appears at the middle of the picture. `scale` says how large distances appear. The camera converts those choices into the four uniform values we already know how to upload.

![commands change camera state; the camera calculates uniforms while the vertex and index buffers remain unchanged.](../illustrations/journey-08.svg)

Follow the formula: `(position - center) * scale`. Our shader already accepts `position * scale + offset`, so the camera supplies `offset = -center * scale`. A centre of 0.25 and scale of 2 produce an offset of -0.5. The world point 0.25 then appears at screen x = 0, exactly where it should.

The first edit adds several commands inside one disabled `fieldset`. Once setup succeeds, enabling that fieldset enables all its controls. Their click events bubble to a single listener. That listener owns the background, camera, renderer and surface; we do not need several callbacks sharing mutable camera data.

The listener inspects the clicked element's id, changes the appropriate state, and requests a frame. An unrecognised target does nothing. The renderer still accepts four numbers rather than a browser element or a Camera: it has no reason to know how the user chose this view.

Two tests protect the meaning of that state. One proves that the point we look at remains at the centre after zooming. The other keeps zoom positive and bounded, including invalid input. These checks exercise the camera without creating a window or opening a GPU.

## Type the change

Continue from [Send one view setting to every corner](07-uniforms.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-08-camera` (from `session_viewer`).

### 1. `src/camera.rs`

Create the camera state and its two tests. There are no browser or wgpu types in this file.

Create the file and type:

```rust
--8<-- "journey/code/08-camera-02.rs"
```

### 2. `src/lib.rs`

Register the camera module so the browser and native tests can use it.

Find this exact block:

```rust
pub mod background;
pub mod renderer;
#[cfg(target_arch = "wasm32")]
mod browser;
```

Replace that block with:

```rust
--8<-- "journey/code/08-camera-fullscreen-1.rs"
```

### 3. `src/browser.rs`

Connect move the view, keep the geometry to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
use crate::background::Background;
use crate::renderer::Renderer;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
```

Replace that block with:

```rust
--8<-- "journey/code/08-camera-window-1.rs"
```

### 4. `src/browser.rs`

Connect move the view, keep the geometry to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
    let mut panel = crate::panel::Panel::new(
        &renderer,
        config.format.add_srgb_suffix(),
        &["Help", "Background"],
    );
    panel.update(None, &canvas)?;
    let mut background = Background::default();
    let transform = [0.75, 0.75, 0.25, 0.15];
    present(&surface, &renderer, &background, &transform, &mut panel)?;
    let input_canvas = canvas.clone();
    let click = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let line = match panel.update(Some(&event), &input_canvas) {
```

Replace that block with:

```rust
--8<-- "journey/code/08-camera-window-2.rs"
```

### 5. `src/browser.rs`

Connect move the view, keep the geometry to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
                return;
            }
        };
        if line.as_deref() == Some("background") {
            background.toggle();
        }
        if let Err(error) = panel.update(None, &input_canvas) {
            report(&format!("Cannot lay out commands: {error:?}"));
            return;
        }
        if let Err(error) = present(&surface, &renderer, &background, &transform, &mut panel) {
            report(&format!("Cannot redraw: {error:?}"));
        }
    });
```

Replace that block with:

```rust
--8<-- "journey/code/08-camera-window-3.rs"
```

### 6. `src/browser.rs`

Connect move the view, keep the geometry to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("One view setting moves all four corners.");
    Ok(())
}
```

Replace that block with:

```rust
--8<-- "journey/code/08-camera-window-4.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Type `Zoom Out`, then `View Reset`. The diamond shrinks, then returns to its initial size. The camera changed; the geometry did not.

**Verified checkpoint in Chrome.**

![Actual browser result: Move the view, keep the geometry.](../screenshots/journey/08-camera-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Zoom Out once, Pan Right once, then Pan Left once. Predict what View Reset will change. Now change the background and reset again: the background should stay as you chose it. This checks that camera state and background state have separate responsibilities.

</details>

## Explain the change

When the camera centre moves right, why does the diamond move left on the screen?

<details>
<summary>Compare your explanation</summary>

The shader receives an offset equal to minus the camera centre times the scale. It therefore draws each world position relative to the camera centre. Looking farther right places the stationary diamond farther left in our view; its stored positions have not moved.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 08-camera
npm --prefix ../session_tests run course -- save 08-camera
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

A full 3D camera also owns a target and a viewing scale or distance. Its orientation and projection add the third dimension. We will extend this view model before connecting orbit gestures, while preserving the separation between camera state, document geometry and GPU resources.

</details>
