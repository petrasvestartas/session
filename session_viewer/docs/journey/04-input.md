# 04 · Make a choice change the picture

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 12–24 minutes.** 38 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Use a command to switch backgrounds without changing the triangle.

**Follow:** Typed Background → Panel returns a line → Background toggles → Renderer redraws.

We can already type in the viewer’s own command line. Today one submitted word changes the picture. Type Background and press Enter: one boolean flips, and the existing renderer draws again.

The browser owns Background beside Panel and Renderer. Panel reads the line; Background owns the choice; Renderer turns that choice into pixels. No new device or pipeline is created when you run the command.

![The command changes one background value, then redraws with the existing renderer.](../illustrations/journey-04.svg)

`Option<String>` means a submitted line may or may not be present. Most events only move a caret or update completion. Only the recognised Background line changes the background. The match reads that result without inventing a second input path.

## Type the change

Continue from [Type into the real command dock](03d-input.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-04-input` (from `session_viewer`).

### 1. `src/background.rs`

Create the small piece of application state. It knows colours, but knows nothing about HTML or GPU resources.

Create the file and type:

```rust
--8<-- "journey/code/04-input-02.rs"
```

### 2. `src/lib.rs`

Register the state module so both the browser and the renderer can use it.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod renderer;
#[cfg(target_arch = "wasm32")]
mod browser;
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/04-input-fullscreen-1.rs"
```

### 3. `src/renderer.rs`

The renderer now reads a background passed by its caller. & borrows it for this call; drawing does not take ownership or change the choice.

<details>
<summary>Locate the existing block</summary>

```rust
        Self { device, queue, pipeline }
    }

    pub fn draw(&self, view: &wgpu::TextureView) {
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/04-input-fullscreen-6.rs"
```

### 4. `src/renderer.rs`

The renderer now reads a background passed by its caller. & borrows it for this call; drawing does not take ownership or change the choice.

<details>
<summary>Locate the existing block</summary>

```rust
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0, g: 1.0, b: 1.0, a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/04-input-fullscreen-7.rs"
```

### 5. `src/browser.rs`

Import the background state.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::renderer::Renderer;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/04-input-window-1.rs"
```

### 6. `src/browser.rs`

Own the background and toggle it when the dock submits Background.

<details>
<summary>Locate the existing block</summary>

```rust
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let renderer = Renderer::new(device, queue, config.format.add_srgb_suffix());
    let mut panel = crate::panel::Panel::new(&renderer, config.format.add_srgb_suffix(), &["Help"]);
    panel.update(None, &canvas)?;
    present(&surface, &renderer, &mut panel)?;
    let input_canvas = canvas.clone();
    let click = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let _line = match panel.update(Some(&event), &input_canvas) {
            Ok(line) => line,
            Err(error) => {
                report(&format!("Cannot read command: {error:?}"));
                return;
            }
        };
        if let Err(error) = panel.update(None, &input_canvas) {
            report(&format!("Cannot lay out commands: {error:?}"));
            return;
        }
        if let Err(error) = present(&surface, &renderer, &mut panel) {
            report(&format!("Cannot redraw: {error:?}"));
        }
    });
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/04-input-window-2.rs"
```

### 7. `src/browser.rs`

Pass the background into presentation and update the status message.

<details>
<summary>Locate the existing block</summary>

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Type Help in the command field and press Enter.");
    Ok(())
}

fn present(
    surface: &wgpu::Surface<'_>,
    renderer: &Renderer,
    panel: &mut crate::panel::Panel,
) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/04-input-window-3.rs"
```

### 8. `src/browser.rs`

Draw the scene with the current background.

<details>
<summary>Locate the existing block</summary>

```rust
        format: Some(frame.texture.format().add_srgb_suffix()),
        ..Default::default()
    });
    renderer.draw(&view);
    panel.draw(renderer, &view);
    frame.present();
    Ok(())
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/04-input-window-4.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Type `Background` twice. The first command changes the background; the second restores white. The triangle keeps the same shape and position.

**Verified checkpoint in Chrome.**

![Actual browser result: Make a choice change the picture.](../screenshots/journey/04-input-browser.png)

[What this screenshot checks](release.md).

<details>
<summary>Optional experiment</summary>

Before submitting Background twice, predict the final picture. Then trace the one boolean changed by toggle. Find the Renderer constructor and explain why neither command calls it again.

</details>

## Explain the change

Does submitting Background create a new GPU pipeline?

<details>
<summary>Compare your explanation</summary>

No. The browser keeps the existing Renderer and changes one boolean in Background. The next draw reuses the device, queue and pipeline.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 04-input
npm --prefix ../session_tests run course -- save 04-input
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

In the full viewer, input similarly requests a change before a new frame reads the result. This background is view state, not geometry. Moving an object will instead change the document and refresh its display data.

This uses the production dock and styling. Background is a course practice command; later chapters build the production vocabulary. The command names below each checkpoint tell you exactly what it currently accepts.

</details>
