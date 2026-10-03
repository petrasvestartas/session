# 20 · Keep a changing window in proportion

**Typing: 53–105 minutes.** [Estimate](typing-load.md).

**Splitting required:** this verified checkpoint exceeds the one-hour typing target. Smaller runnable lessons are still being prepared.

Keep the GPU image and camera proportional when the window changes. CSS pixels describe layout; drawing pixels equal layout size multiplied by display density.

The viewport helper caps both dimensions together at the device texture limit. `Some(size)` permits drawing; `None` means hidden or invalid measurements. Do not configure zero-sized textures.

## Type

Continue from [Give every action the same route](19-actions.md). [Save or recover your work](recovery.md).

### 1. `src/viewport.rs`

Convert layout dimensions to a safe drawing size, keeping the proportions when the GPU limit applies.

Create the file and type:

```rust
--8<-- "journey/code/20-resize-01.rs"
```

### 2. `src/editor.rs`

Reset the camera pose while retaining the current viewport proportions.

<details>
<summary>Locate the existing block</summary>

```rust
                    Action::ResetView => self.camera = Camera::default(),
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/20-resize-05.rs"
```

### 3. `src/editor.rs`

Catch a resize bug that would otherwise return whenever the learner presses Reset view.

<details>
<summary>Locate the existing block</summary>

```rust
    #[test]
    fn undo_changes_the_document_without_rewinding_the_view() {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/20-resize-06.rs"
```

### 4. `src/lib.rs`

Make the size calculation usable by browser code and Rust tests.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod picking;
pub mod history;
pub mod editor;
pub mod gpu_mesh;
pub mod renderer;
#[cfg(target_arch = "wasm32")]
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/20-resize-fullscreen-1.rs"
```

### 5. `src/renderer.rs`

Reuse the depth helper from lesson 10 with the measured Viewport dimensions.

<details>
<summary>Locate the existing block</summary>

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

</details>

Replace that block with:

```rust
--8<-- "journey/code/20-resize-fullscreen-8.rs"
```

### 6. `src/renderer.rs`

Reuse the depth helper from lesson 10 with the measured Viewport dimensions.

<details>
<summary>Locate the existing block</summary>

```rust
        depth_texture.create_view(&Default::default())
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.depth = Self::depth(&self.device, width, height);
    }

    pub fn draw(
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/20-resize-fullscreen-9.rs"
```

### 7. `src/browser.rs`

Import the viewport size type.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::background::Background;
use crate::editor::{Action, Change, Editor};
use crate::renderer::Renderer;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

pub fn report(message: &str) {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/20-resize-window-1.rs"
```

### 8. `src/browser.rs`

Size renderer attachments from the viewport.

<details>
<summary>Locate the existing block</summary>

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

</details>

Replace that block with:

```rust
--8<-- "journey/code/20-resize-window-2.rs"
```

### 9. `src/browser.rs`

Handle initial resizing and retain the window for later resize events.

<details>
<summary>Locate the existing block</summary>

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

</details>

Replace that block with:

```rust
--8<-- "journey/code/20-resize-window-3.rs"
```

### 10. `src/browser.rs`

Keep the event callback open for the resize path.

<details>
<summary>Locate the existing block</summary>

```rust
                return;
            }
        };

        let line = line.or_else(|| {
            let mouse = event.dyn_ref::<web_sys::MouseEvent>()?;
            (event.type_() == "click"
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/20-resize-window-4.rs"
```

### 11. `src/browser.rs`

Resize before handling input; refresh dock layout and drawing when dimensions change.

<details>
<summary>Locate the existing block</summary>

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

</details>

Replace that block with:

```rust
--8<-- "journey/code/20-resize-window-5.rs"
```

### 12. `src/browser.rs`

Register resize events and update canvas pixels, surface configuration, attachments and camera aspect together.

<details>
<summary>Locate the existing block</summary>

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

</details>

Replace that block with:

```rust
--8<-- "journey/code/20-resize-window-6.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Resize the browser from wide to tall. The canvas, depth image and camera must use the new dimensions; the scene must not stretch.

**Verified checkpoint in Chrome.**

![Actual browser result: Keep a changing window in proportion.](../screenshots/journey/20-resize-browser.png)

[Verification scope](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Code explanation and diagram</summary>

Before redraw, update canvas and surface dimensions, depth texture, and camera aspect together. Recreate textures only when size changes. The existing browser callback owns the editor and renderer and also handles resize events.

Camera reset must preserve the current aspect. Its test catches a reset that would stretch the scene after resizing. Dense displays use more drawing pixels while retaining the same visible geometry size.

CSS rectangle × display density → bounded pixel size → canvas and surface + depth image + camera aspect → draw.

![One measured pixel size feeds the surface, depth attachment and camera; all three must agree before the next draw.](../illustrations/journey-20.svg)

Why is changing only the canvas width insufficient?

The canvas drawing buffer, surface configuration and depth attachment must agree on their pixel dimensions. The camera also needs width divided by height so geometry keeps its proportions. Changing only one can cause a GPU validation error, a stretched view or an old blurry image scaled by CSS.

Study estimate, including typing and experiments: 3–5 hours.

</details>

<details>
<summary>Optional experiment</summary>

At density 2, predict the drawing size for a 768 × 384 CSS canvas: 1536 × 768. Now set the GPU limit in the viewport test to 1024. Both dimensions must shrink together to 1024 × 512. Run the test and explain why clamping each dimension independently would stretch the scene.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20-resize
npm --prefix ../session_tests run course -- save 20-resize
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

The final viewer uses the same size agreement for every attachment, including multisampling, picking and post-processing. Later panels need element resize observation as well as window events; this lesson handles window resizing and rechecks layout on each action.

The browser window has been resized. The drawing buffer, depth texture and camera aspect now follow the canvas dimensions.

[Full validation scope](release.md).

</details>
