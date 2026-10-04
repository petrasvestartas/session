# 08 · Move the view, keep the geometry

**Typing: 25–50 minutes.** [Estimate](typing-load.md).

Give view state its own `Camera`: a centre and scale. Typed pan and zoom commands update it, then redraw with the existing geometry.

The camera formula is `(position - center) * scale`. Our shader already accepts scale and offset, so supply `offset = -center * scale`. With centre 0.25 and scale 2, offset −0.5 puts world x = 0.25 at screen x = 0.

## Type

Continue from [Send one view setting to every corner](07-uniforms.md). [Save or recover your work](recovery.md).

### 1. `src/camera.rs`

Create the camera state and its two tests. There are no browser or wgpu types in this file.

Create the file and type:

```rust
--8<-- "journey/code/08-camera-02.rs"
```

### 2. `src/lib.rs`

Register the camera module so the browser and native tests can use it.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod background;
pub mod renderer;
#[cfg(target_arch = "wasm32")]
mod browser;
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/08-camera-fullscreen-1.rs"
```

### 3. `src/browser.rs`

Import the camera owner.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::background::Background;
use crate::renderer::Renderer;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/08-camera-window-1.rs"
```

### 4. `src/browser.rs`

Add camera commands and present its initial uniform.

<details>
<summary>Locate the existing block</summary>

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

</details>

Replace that block with:

```rust
--8<-- "journey/code/08-camera-window-2.rs"
```

### 5. `src/browser.rs`

Apply the submitted camera command, then redraw with its uniform.

<details>
<summary>Locate the existing block</summary>

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

</details>

Replace that block with:

```rust
--8<-- "journey/code/08-camera-window-3.rs"
```

### 6. `src/browser.rs`

Report that navigation leaves the mesh unchanged.

<details>
<summary>Locate the existing block</summary>

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("One view setting moves all four corners.");
    Ok(())
}
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/08-camera-window-4.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Type `Zoom Out`, then `View Reset`. The diamond shrinks, then returns to its initial size. The camera changed; the geometry did not.

**Verified checkpoint in Chrome.**

![Actual browser result: Move the view, keep the geometry.](../screenshots/journey/08-camera-browser.png)

[Verification scope](release.md).

Run the state checks on your computer from your project folder:

```sh
cargo test --lib --locked --target host-tuple -j4
```

<details>
<summary>Code explanation and diagram</summary>

The browser translates a submitted command into camera changes. The renderer receives four numbers and does not know how the user chose them. Camera tests verify the centre stays fixed during zoom and reject invalid or out-of-range scale.

command → camera centre or scale → four uniform values → existing renderer → unchanged mesh in a different view.

![commands change camera state; the camera calculates uniforms while the vertex and index buffers remain unchanged.](../illustrations/journey-08.svg)

When the camera centre moves right, why does the diamond move left on the screen?

The shader receives an offset equal to minus the camera centre times the scale. It therefore draws each world position relative to the camera centre. Looking farther right places the stationary diamond farther left in our view; its stored positions have not moved.

Study estimate, including typing and experiments: 2–3 hours.

</details>

<details>
<summary>Optional experiment</summary>

Zoom Out once, Pan Right once, then Pan Left once. Predict what View Reset will change. Now change the background and reset again: the background should stay as you chose it. This checks that camera state and background state have separate responsibilities.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 08-camera
npm --prefix ../session_tests run course -- save 08-camera
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

A full 3D camera also owns a target and a viewing scale or distance. Its orientation and projection add the third dimension. We will extend this view model before connecting orbit gestures, while preserving the separation between camera state, document geometry and GPU resources.



[Full validation scope](release.md).

</details>
