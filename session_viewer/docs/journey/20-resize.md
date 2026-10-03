# 20 · Keep a changing window in proportion

**Combined study estimate: 3–5 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 53–105 minutes.** 149 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Splitting required:** this verified checkpoint exceeds the one-hour typing target. Smaller runnable lessons are still being prepared.

**Today:** Resize the drawing buffer, depth attachment and camera together, including on dense displays.

**Follow:** CSS rectangle × display density → bounded pixel size → canvas and surface + depth image + camera aspect → draw.

Our canvas has filled the window since lesson 01, and the GPU has used the startup window size since lesson 02. Now narrow the window after opening it: we need to update the image dimensions as well as the CSS layout. Today we add that response and support sharper drawing on dense displays.

There are two measurements. **CSS pixels** describe page layout. **Drawing pixels** describe the GPU image. A canvas 768 CSS pixels wide on a display with a density of 2 needs 1536 drawing pixels across. Geometry should look the same size on the page, just sharper.

![One measured pixel size feeds the surface, depth attachment and camera; all three must agree before the next draw.](../illustrations/journey-20.svg)

The viewport helper multiplies layout size by display density, then caps both dimensions together if the image would exceed the GPU's limit. Its result is an `Option`: `Some(size)` means we can draw; `None` means the canvas is hidden or the measurements are invalid. A zero-sized GPU texture is not a useful placeholder.

The browser will update three things together: the canvas and surface, the depth image, and the camera's aspect ratio. Here **aspect** means width divided by height. A wide canvas needs a wide view, not a stretched box.

Clicks and window resize events can share one callback. Browser events arrive one at a time; the callback already owns our mutable editor and renderer. The window clone is another handle to the same browser window, not another window. We check size before every redraw, but recreate textures only when the dimensions change.

Keep the aspect while resetting the camera pose, just as we did when introducing perspective. This rule now belongs in Editor, where all actions meet. The new test catches this, so resizing cannot quietly stop working after a reset.

## Type the change

Continue from [Give every action the same route](19-actions.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-20-resize` (from `session_viewer`).

### 1. `src/viewport.rs`

Convert layout dimensions to a safe drawing size, keeping the proportions when the GPU limit applies.

Create the file and type:

```rust
--8<-- "journey/code/20-resize-01.rs"
```

### 2. `src/editor.rs`

Reset the camera pose while retaining the current viewport proportions.

Find this exact block:

```rust
                    Action::ResetView => self.camera = Camera::default(),
```

Replace that block with:

```rust
--8<-- "journey/code/20-resize-05.rs"
```

### 3. `src/editor.rs`

Catch a resize bug that would otherwise return whenever the learner presses Reset view.

Find this exact block:

```rust
    #[test]
    fn undo_changes_the_document_without_rewinding_the_view() {
```

Replace that block with:

```rust
--8<-- "journey/code/20-resize-06.rs"
```

### 4. `src/lib.rs`

Make the size calculation usable by browser code and Rust tests.

Find this exact block:

```rust
pub mod picking;
pub mod history;
pub mod editor;
pub mod gpu_mesh;
pub mod renderer;
#[cfg(target_arch = "wasm32")]
```

Replace that block with:

```rust
--8<-- "journey/code/20-resize-fullscreen-1.rs"
```

### 5. `src/renderer.rs`

Reuse the depth helper from lesson 10 with the measured Viewport dimensions.

Find this exact block:

```rust
        Self { device, queue, pipeline, meshes, uniform, view_group, depth }
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.meshes = scene.objects().iter().map(|object| {
            GpuMesh::upload(&self.device, &object.mesh, selected == Some(object.id))
        }).collect();
    }

    fn depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("opaque depth"),
```

Replace that block with:

```rust
--8<-- "journey/code/20-resize-fullscreen-8.rs"
```

### 6. `src/renderer.rs`

Reuse the depth helper from lesson 10 with the measured Viewport dimensions.

Find this exact block:

```rust
        depth_texture.create_view(&Default::default())
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.depth = Self::depth(&self.device, width, height);
    }

    pub fn draw(
```

Replace that block with:

```rust
--8<-- "journey/code/20-resize-fullscreen-9.rs"
```

### 7. `src/browser.rs`

Connect keep a changing window in proportion to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
use crate::background::Background;
use crate::editor::{Action, Change, Editor};
use crate::renderer::Renderer;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

pub fn report(message: &str) {
```

Replace that block with:

```rust
--8<-- "journey/code/20-resize-window-1.rs"
```

### 8. `src/browser.rs`

Connect keep a changing window in proportion to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let mut editor = Editor::default();
    editor.camera.aspect = width as f64 / height as f64;
    let mut renderer = Renderer::new(
        device,
        queue,
        config.format.add_srgb_suffix(),
        &editor.scene,
    );
    renderer.resize(width, height);
    let mut panel = crate::panel::Panel::new(
        &renderer,
        config.format.add_srgb_suffix(),
```

Replace that block with:

```rust
--8<-- "journey/code/20-resize-window-2.rs"
```

### 9. `src/browser.rs`

Connect keep a changing window in proportion to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
        ],
    );
    panel.update(None, &canvas)?;
    present(
        &surface,
        &renderer,
        &editor.background,
        &editor.camera.uniform(),
        &mut panel,
    )?;
    let input_canvas = canvas.clone();
    let click = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let line = match panel.update(Some(&event), &input_canvas) {
            Ok(line) => line,
            Err(error) => {
```

Replace that block with:

```rust
--8<-- "journey/code/20-resize-window-3.rs"
```

### 10. `src/browser.rs`

Connect keep a changing window in proportion to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
                return;
            }
        };

        let line = line.or_else(|| {
            let mouse = event.dyn_ref::<web_sys::MouseEvent>()?;
            (event.type_() == "click"
```

Replace that block with:

```rust
--8<-- "journey/code/20-resize-window-4.rs"
```

### 11. `src/browser.rs`

Connect keep a changing window in proportion to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
                }
            }
        }
        if let Err(error) = panel.update(None, &input_canvas) {
            report(&format!("Cannot lay out commands: {error:?}"));
            return;
        }
        if let Err(error) = present(
            &surface,
            &renderer,
            &editor.background,
            &editor.camera.uniform(),
            &mut panel,
        ) {
            report(&format!("Cannot redraw: {error:?}"));
        }
    });
    for name in [
```

Replace that block with:

```rust
--8<-- "journey/code/20-resize-window-5.rs"
```

### 12. `src/browser.rs`

Connect keep a changing window in proportion to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
        "blur",
        "click",
    ] {
        canvas.add_event_listener_with_callback(name, click.as_ref().unchecked_ref())?;
    }
    let options = web_sys::AddEventListenerOptions::new();
    options.set_passive(false);
    canvas.add_event_listener_with_callback_and_add_event_listener_options(
        "wheel",
        click.as_ref().unchecked_ref(),
        &options,
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Every document action follows the same editor path.");
    Ok(())
}

fn present(
    surface: &wgpu::Surface<'_>,
    renderer: &Renderer,
```

Replace that block with:

```rust
--8<-- "journey/code/20-resize-window-6.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Resize the browser from wide to tall. The canvas, depth image and camera must use the new dimensions; the scene must not stretch.

**Verified checkpoint in Chrome.**

![Actual browser result: Keep a changing window in proportion.](../screenshots/journey/20-resize-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

At density 2, predict the drawing size for a 768 × 384 CSS canvas: 1536 × 768. Now set the GPU limit in the viewport test to 1024. Both dimensions must shrink together to 1024 × 512. Run the test and explain why clamping each dimension independently would stretch the scene.

</details>

## Explain the change

Why is changing only the canvas width insufficient?

<details>
<summary>Compare your explanation</summary>

The canvas drawing buffer, surface configuration and depth attachment must agree on their pixel dimensions. The camera also needs width divided by height so geometry keeps its proportions. Changing only one can cause a GPU validation error, a stretched view or an old blurry image scaled by CSS.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 20-resize
npm --prefix ../session_tests run course -- save 20-resize
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

The final viewer uses the same size agreement for every attachment, including multisampling, picking and post-processing. Later panels need element resize observation as well as window events; this lesson handles window resizing and rechecks layout on each action.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

The browser window has been resized. The drawing buffer, depth texture and camera aspect now follow the canvas dimensions.

[Full validation scope](release.md).

</details>
