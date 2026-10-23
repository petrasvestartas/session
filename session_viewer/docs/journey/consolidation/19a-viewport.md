# 3.2 · One drawing size and one captured gesture

**Goal:** Normalize wheel input and keep every keyboard feature command in the command dock. · **Steps:** 8 · **Source:** `src/app/decode.rs,src/app/loader.rs,src/state.rs`

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">DOM Resize / Visibility Events</text>
  <rect x="64" y="54" width="108" height="72" rx="4" fill="#f8fafc" stroke="#111111" stroke-width="1.5" stroke-dasharray="3 3"/>
  <rect x="64" y="54" width="224" height="98" rx="4" fill="none" stroke="#2563eb" stroke-width="2"/>
  <line x1="172" y1="90" x2="276" y2="90" stroke="#2563eb" stroke-width="2" marker-end="url(#arrB)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="224" y="140" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Resize (W, H, DPR)</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Matched Color + Depth32Float Attachments</text>
  <rect x="444" y="56" width="104" height="84" rx="4" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="496" y="102" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Surface</text>
  <rect x="572" y="56" width="104" height="84" rx="4" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="624" y="102" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Depth32</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="156" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">aspect = W / H</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Unstretched Aspect-Correct View</text>
  <rect x="802" y="44" width="264" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="934,58 994,84 934,110 874,84" fill="#93c5fd" stroke="#111111" stroke-width="1.75"/>
  <polygon points="874,84 934,110 934,150 874,124" fill="#60a5fa" stroke="#111111" stroke-width="1.75"/>
  <polygon points="934,110 994,84 994,124 934,150" fill="#2563eb" stroke="#111111" stroke-width="1.75"/>
</svg>
</div>

## <a id="checkpoint-19a-viewport"></a>1. Measure a safe drawing size

**What this code does:** Convert CSS dimensions and display density to finite, bounded GPU pixel dimensions.

### 1. `src/viewport.rs`

Convert layout dimensions to a safe drawing size, keeping the proportions when the GPU limit applies.

Create the file and type:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
}

impl Viewport {
    pub fn from_css(width: f64, height: f64, ratio: f64, limit: u32) -> Option<Self> {
        if [width, height, ratio]
            .iter()
            .any(|value| !value.is_finite() || *value <= 0.0)
            || limit == 0
        {
            return None;
        }
        let width = width * ratio;
        let height = height * ratio;
        if !width.is_finite() || !height.is_finite() {
            return None;
        }
        // Reduce both dimensions together when the GPU cannot hold the requested image.
        let scale = (limit as f64 / width.max(height)).min(1.0);
        Some(Self {
            width: (width * scale)
                .round()
                .clamp(1.0, limit as f64) as u32,
            height: (height * scale)
                .round()
                .clamp(1.0, limit as f64) as u32,
        })
    }

    pub fn aspect(self) -> f64 {
        self.width as f64 / self.height as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_density_changes_pixels_without_stretching_the_view() {
        let normal = Viewport::from_css(768.0, 384.0, 1.0, 4096).unwrap();
        let dense = Viewport::from_css(768.0, 384.0, 2.0, 4096).unwrap();
        assert_eq!(dense.width, normal.width * 2);
        assert_eq!(dense.height, normal.height * 2);
        assert_eq!(dense.aspect(), normal.aspect());
        let capped = Viewport::from_css(768.0, 384.0, 4.0, 1024).unwrap();
        assert_eq!(
            capped,
            Viewport {
                width: 1024,
                height: 512
            }
        );
    }

    #[test]
    fn a_hidden_canvas_has_no_renderable_size() {
        assert!(Viewport::from_css(640.0, 0.0, 1.0, 4096).is_none());
        assert!(Viewport::from_css(f64::NAN, 480.0, 1.0, 4096).is_none());
        assert!(Viewport::from_css(640.0, 480.0, 0.0, 4096).is_none());
    }
}
```

### 2. `src/lib.rs`

Make the size calculation usable by browser code and Rust tests.

**Locate this existing block:**

```rust
pub mod scene;

use wasm_bindgen::prelude::*;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod scene;
pub mod viewport;

use wasm_bindgen::prelude::*;
```

## <a id="checkpoint-20-resize"></a>2. Resize canvas, depth and camera together

**What this code does:** Resize the drawing buffer, depth attachment and camera together, including on dense displays.

### 1. `src/renderer.rs`

Keep depth texture allocation in the existing depth helper. Recreate the depth view for the measured viewport dimensions.

**Locate this existing block:**

```rust
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.meshes = scene
            .objects()
            .iter()
            .map(|object| {
                GpuMesh::upload(
                    &self.device,
                    &object.mesh,
                    selected == Some(object.id),
                )
            })
            .collect();
    }

    fn depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("opaque depth"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        depth_texture.create_view(&Default::default())
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.depth = Self::depth(&self.device, width, height);
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    fn depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("opaque depth"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        depth_texture.create_view(&Default::default())
    }

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        self.depth = Self::depth(&self.device, size.width, size.height);
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.meshes = scene
            .objects()
            .iter()
            .map(|object| {
                GpuMesh::upload(
                    &self.device,
                    &object.mesh,
                    selected == Some(object.id),
                )
            })
            .collect();
    }
```

### 2. `src/browser.rs`

Import the viewport size type. Size renderer attachments from the viewport. Handle initial resizing and retain the window for later resize events. Keep the event callback open for the resize path. Resize before handling input; refresh dock layout and drawing when dimensions change. Register resize events and update canvas pixels, surface configuration, attachments and camera aspect together.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        "click",
    ] {
        canvas
            .add_event_listener_with_callback(name, click.as_ref().unchecked_ref())?;
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
```

Replace it with:

```rust
        "click",
    ] {
        canvas
            .add_event_listener_with_callback(name, update.as_ref().unchecked_ref())?;
    }
    let options = web_sys::AddEventListenerOptions::new();
    options.set_passive(false);
    canvas.add_event_listener_with_callback_and_add_event_listener_options(
        "wheel",
        update.as_ref().unchecked_ref(),
        &options,
    )?;
    window
        .add_event_listener_with_callback("resize", update.as_ref().unchecked_ref())?;
    // Both event sources retain this one callback for the lifetime of the page.
    update.forget();
    report("Canvas pixels, depth and camera resize together.");
    Ok(())
}

fn resize(
    window: &web_sys::Window,
    canvas: &web_sys::HtmlCanvasElement,
    surface: &wgpu::Surface<'_>,
    config: &mut wgpu::SurfaceConfiguration,
    renderer: &mut Renderer,
    editor: &mut Editor,
) -> bool {
    let rect = canvas.get_bounding_client_rect();
    let limit = renderer
        .device
        .limits()
        .max_texture_dimension_2d;
    let Some(size) = Viewport::from_css(
        rect.width(),
        rect.height(),
        window.device_pixel_ratio(),
        limit,
    ) else {
        return false;
    };
    if size.width != config.width || size.height != config.height {
        canvas.set_width(size.width);
        canvas.set_height(size.height);
        config.width = size.width;
        config.height = size.height;
        surface.configure(&renderer.device, config);
        renderer.resize(size);
    }
    editor.camera.aspect = size.aspect();
    true
}
```

**Locate this block:**

```rust
                    }
                }
            }
            if let Err(error) = panel.update(None, &input_canvas) {
                report(&format!(
                    "Cannot lay out commands: {error:?}"
                ));
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

Replace it with:

```rust
                    }
                }
            }
            if resize(
                &browser_window,
                &input_canvas,
                &surface,
                &mut config,
                &mut renderer,
                &mut editor,
            ) {
                if let Err(error) = panel.update(None, &input_canvas) {
                    report(&format!(
                        "Cannot lay out commands: {error:?}"
                    ));
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
            }
        });
    for name in [
```

**Locate this block:**

```rust
                    return;
                }
            };

            let line = line.or_else(|| {
                let mouse = event.dyn_ref::<web_sys::MouseEvent>()?;
                (event.type_() == "click"
```

Replace it with:

```rust
                    return;
                }
            };
            let line = line.or_else(|| {
                let mouse = event.dyn_ref::<web_sys::MouseEvent>()?;
                (event.type_() == "click"
```

**Locate this block:**

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
    let click =
        Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
            let line = match panel.update(Some(&event), &input_canvas) {
                Ok(line) => line,
```

Replace it with:

```rust
        ],
    );
    panel.update(None, &canvas)?;
    if resize(
        &window,
        &canvas,
        &surface,
        &mut config,
        &mut renderer,
        &mut editor,
    ) {
        panel.update(None, &canvas)?;
        present(
            &surface,
            &renderer,
            &editor.background,
            &editor.camera.uniform(),
            &mut panel,
        )?;
    }
    let input_canvas = canvas.clone();
    let browser_window = window.clone();
    let update =
        Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
            let line = match panel.update(Some(&event), &input_canvas) {
                Ok(line) => line,
```

**Locate this block:**

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

Replace it with:

```rust
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let mut editor = Editor::default();
    let mut renderer = Renderer::new(
        device,
        queue,
        config.format.add_srgb_suffix(),
        &editor.scene,
    );
    renderer.resize(Viewport { width, height });
    let mut panel = crate::panel::Panel::new(
        &renderer,
        config.format.add_srgb_suffix(),
```

**Locate this block:**

```rust
use crate::renderer::Renderer;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

pub fn report(message: &str) {
```

Replace it with:

```rust
use crate::renderer::Renderer;
use crate::viewport::Viewport;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

pub fn report(message: &str) {
```

## <a id="checkpoint-20a-press"></a>3. Remember the pointer that starts a drag

**What this code does:** Retain one pointer and produce movement relative to its previous position.

### 1. `src/gesture.rs`

Retain one pointer, track its starting position and return relative right-drag movement.

Create the file and type:

```rust
#[derive(Debug, PartialEq)]
pub enum Motion {
    Pick([f64; 2]),
    Orbit([f64; 2]),
}

struct Drag {
    id: i32,
    button: i16,
    start: [f64; 2],
    last: [f64; 2],
    moved: bool,
}

#[derive(Default)]
pub struct Gesture {
    active: Option<Drag>,
}

impl Gesture {
    pub fn press(&mut self, id: i32, button: i16, position: [f64; 2]) -> bool {
        if self.active.is_some()
            || !matches!(button, 0 | 2)
            || position.iter().any(|n| !n.is_finite())
        {
            return false;
        }
        self.active = Some(Drag {
            id,
            button,
            start: position,
            last: position,
            moved: false,
        });
        true
    }

    pub fn move_to(&mut self, id: i32, position: [f64; 2]) -> Option<Motion> {
        let drag = self
            .active
            .as_mut()
            .filter(|drag| drag.id == id)?;
        if position.iter().any(|n| !n.is_finite()) {
            return None;
        }
        let delta = [
            position[0] - drag.last[0],
            position[1] - drag.last[1],
        ];
        drag.last = position;
        let distance = (position[0] - drag.start[0]).hypot(position[1] - drag.start[1]);
        // Once a press becomes a drag, returning to its start must not turn it into a click.
        drag.moved |= distance > 4.0;
        (drag.button == 2 && delta != [0.0, 0.0]).then_some(Motion::Orbit(delta))
    }
}
```

### 2. `src/gesture_tests.rs`

Optional authoring fixture. Check rejected presses, pointer ownership and movement from the previous position.

Create the file and type:

```rust
use crate::gesture::{Gesture, Motion};

#[test]
fn a_press_retains_one_pointer_and_reports_relative_motion() {
    let mut gesture = Gesture::default();
    assert!(!gesture.press(1, 1, [0.0, 0.0]));
    assert!(!gesture.press(1, 0, [f64::NAN, 0.0]));
    assert!(gesture.press(1, 2, [10.0, 20.0]));
    assert!(!gesture.press(2, 0, [10.0, 20.0]));
    assert_eq!(gesture.move_to(2, [30.0, 25.0]), None);
    assert_eq!(
        gesture.move_to(1, [20.0, 25.0]),
        Some(Motion::Orbit([10.0, 5.0]))
    );
    assert_eq!(
        gesture.move_to(1, [24.0, 28.0]),
        Some(Motion::Orbit([4.0, 3.0]))
    );
}
```

### 3. `src/lib.rs`

Expose the gesture module to the browser and compile its tests only in the test build.

**Locate this existing block:**

```rust
pub mod editor;
pub mod gpu_mesh;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod editor;
pub mod gesture;
#[cfg(test)]
mod gesture_tests;
pub mod gpu_mesh;
```

## <a id="checkpoint-20b-release"></a>4. Finish or cancel a drag

**What this code does:** Release an owned pointer or cancel it without inventing a click.

### 1. `src/gesture.rs`

Finish the owned pointer, preserve drag classification and clear cancelled input.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn release(&mut self, id: i32, position: [f64; 2]) -> Option<Motion> {
        if self.active.as_ref()?.id != id {
            return None;
        }
        let last_motion = self.move_to(id, position);
        let drag = self.active.take()?;
        if drag.button == 2 {
            return last_motion;
        }
        (!drag.moved && position.iter().all(|n| n.is_finite()))
            .then_some(Motion::Pick(position))
    }

    pub fn cancel(&mut self) {
        self.active = None;
    }

    pub fn cancel_pointer(&mut self, id: i32) {
        if self
            .active
            .as_ref()
            .is_some_and(|drag| drag.id == id)
        {
            self.cancel();
        }
    }
}
```

### 2. `src/gesture_tests.rs`

Optional authoring fixture. Check left-click classification, final motion, pointer ownership and cancellation.

**Locate this existing block:**

```rust
#[test]
fn a_press_retains_one_pointer_and_reports_relative_motion() {
    let mut gesture = Gesture::default();
    assert!(!gesture.press(1, 1, [0.0, 0.0]));
    assert!(!gesture.press(1, 0, [f64::NAN, 0.0]));
    assert!(gesture.press(1, 2, [10.0, 20.0]));
    assert!(!gesture.press(2, 0, [10.0, 20.0]));
    assert_eq!(gesture.move_to(2, [30.0, 25.0]), None);
    assert_eq!(
        gesture.move_to(1, [20.0, 25.0]),
        Some(Motion::Orbit([10.0, 5.0]))
    );
    assert_eq!(
        gesture.move_to(1, [24.0, 28.0]),
        Some(Motion::Orbit([4.0, 3.0]))
    );
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[test]
fn a_click_selects_but_a_left_drag_does_not() {
    let mut input = Gesture::default();
    assert!(input.press(1, 0, [10.0, 20.0]));
    assert_eq!(
        input.release(1, [12.0, 20.0]),
        Some(Motion::Pick([12.0, 20.0]))
    );
    input.press(1, 0, [10.0, 20.0]);
    assert_eq!(input.move_to(1, [20.0, 20.0]), None);
    assert_eq!(input.release(1, [10.0, 20.0]), None);
    input.press(1, 0, [10.0, 20.0]);
    assert_eq!(input.release(1, [30.0, 20.0]), None);
}

#[test]
fn only_the_captured_pointer_can_move_or_finish_the_gesture() {
    let mut input = Gesture::default();
    assert!(input.press(1, 2, [0.0, 0.0]));
    assert!(!input.press(2, 0, [0.0, 0.0]));
    assert_eq!(input.move_to(2, [100.0, 50.0]), None);
    assert_eq!(input.release(2, [0.0, 0.0]), None);
    input.cancel_pointer(2);
    assert_eq!(
        input.move_to(1, [10.0, 5.0]),
        Some(Motion::Orbit([10.0, 5.0]))
    );
    assert_eq!(
        input.move_to(1, [30.0, 8.0]),
        Some(Motion::Orbit([20.0, 3.0]))
    );
    assert_eq!(input.release(1, [30.0, 8.0]), None);
    assert_eq!(input.move_to(1, [40.0, 8.0]), None);
    input.press(3, 2, [0.0, 0.0]);
    assert_eq!(
        input.release(3, [5.0, 2.0]),
        Some(Motion::Orbit([5.0, 2.0]))
    );
}

#[test]
fn cancellation_never_selects_and_allows_a_fresh_press() {
    let mut input = Gesture::default();
    input.press(1, 0, [0.0, 0.0]);
    input.cancel_pointer(1);
    assert_eq!(input.release(1, [0.0, 0.0]), None);
    assert!(input.press(2, 2, [0.0, 0.0]));
    input.cancel();
    assert_eq!(input.move_to(2, [20.0, 0.0]), None);
    assert!(!input.press(1, 1, [0.0, 0.0]));
    assert!(!input.press(1, 0, [f64::NAN, 0.0]));
}
```

## <a id="checkpoint-20c-motion"></a>5. Convert pointer motion to editor actions

**What this code does:** Convert CSS pointer coordinates and movement into existing Pick and Orbit actions.

### 1. `src/gesture.rs`

Import the editor action used by motion conversion. Convert CSS position or motion to an existing editor action after validating the canvas rectangle.

**Locate this existing block:**

```rust
#[derive(Debug, PartialEq)]
pub enum Motion {
    Pick([f64; 2]),
    Orbit([f64; 2]),
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::editor::Action;

#[derive(Debug, PartialEq)]
pub enum Motion {
    Pick([f64; 2]),
    Orbit([f64; 2]),
}

impl Motion {
    pub fn action(self, rect: [f64; 4]) -> Option<Action> {
        let [left, top, width, height] = rect;
        if rect.iter().any(|n| !n.is_finite()) || width <= 0.0 || height <= 0.0 {
            return None;
        }
        Some(match self {
            Self::Pick([x, y]) => Action::Pick([
                (2.0 * (x - left) / width - 1.0) as f32,
                (1.0 - 2.0 * (y - top) / height) as f32,
            ]),
            Self::Orbit([dx, dy]) => Action::Orbit(
                -std::f64::consts::PI * dx / width,
                -std::f64::consts::PI * dy / height,
            ),
        })
    }
}
```

### 2. `src/gesture_tests.rs`

Optional authoring fixture. Import editor actions for motion and history checks. Optional authoring fixture. Check CSS coordinate conversion and camera movement outside document history.

Replace the entire file with:

```rust
use crate::{
    editor::{Action, Change, Editor},
    gesture::{Gesture, Motion},
};

#[test]
fn a_click_selects_but_a_left_drag_does_not() {
    let mut input = Gesture::default();
    assert!(input.press(1, 0, [10.0, 20.0]));
    assert_eq!(
        input.release(1, [12.0, 20.0]),
        Some(Motion::Pick([12.0, 20.0]))
    );
    input.press(1, 0, [10.0, 20.0]);
    assert_eq!(input.move_to(1, [20.0, 20.0]), None);
    assert_eq!(input.release(1, [10.0, 20.0]), None);
    input.press(1, 0, [10.0, 20.0]);
    assert_eq!(input.release(1, [30.0, 20.0]), None);
}

#[test]
fn only_the_captured_pointer_can_move_or_finish_the_gesture() {
    let mut input = Gesture::default();
    assert!(input.press(1, 2, [0.0, 0.0]));
    assert!(!input.press(2, 0, [0.0, 0.0]));
    assert_eq!(input.move_to(2, [100.0, 50.0]), None);
    assert_eq!(input.release(2, [0.0, 0.0]), None);
    input.cancel_pointer(2);
    assert_eq!(
        input.move_to(1, [10.0, 5.0]),
        Some(Motion::Orbit([10.0, 5.0]))
    );
    assert_eq!(
        input.move_to(1, [30.0, 8.0]),
        Some(Motion::Orbit([20.0, 3.0]))
    );
    assert_eq!(input.release(1, [30.0, 8.0]), None);
    assert_eq!(input.move_to(1, [40.0, 8.0]), None);
    input.press(3, 2, [0.0, 0.0]);
    assert_eq!(
        input.release(3, [5.0, 2.0]),
        Some(Motion::Orbit([5.0, 2.0]))
    );
}

#[test]
fn cancellation_never_selects_and_allows_a_fresh_press() {
    let mut input = Gesture::default();
    input.press(1, 0, [0.0, 0.0]);
    input.cancel_pointer(1);
    assert_eq!(input.release(1, [0.0, 0.0]), None);
    assert!(input.press(2, 2, [0.0, 0.0]));
    input.cancel();
    assert_eq!(input.move_to(2, [20.0, 0.0]), None);
    assert!(!input.press(1, 1, [0.0, 0.0]));
    assert!(!input.press(1, 0, [f64::NAN, 0.0]));
}

#[test]
fn css_coordinates_reach_picking_without_a_device_pixel_ratio() {
    let Some(Action::Pick(screen)) =
        Motion::Pick([420.0, 290.0]).action([100.0, 50.0, 640.0, 480.0])
    else {
        panic!("Expected a pick");
    };
    assert_eq!(screen, [0.0, 0.0]);
    assert!(
        Motion::Orbit([1.0, 1.0])
            .action([0.0; 4])
            .is_none()
    );
}

#[test]
fn a_drag_changes_the_view_without_editing_the_document() {
    let mut editor = Editor::default();
    editor.apply(Action::AddBox).unwrap();
    let before = editor.camera.uniform();
    let mut input = Gesture::default();
    input.press(1, 2, [0.0, 0.0]);
    let action = input
        .move_to(1, [100.0, 50.0])
        .unwrap()
        .action([0.0, 0.0, 640.0, 480.0])
        .unwrap();
    assert_eq!(editor.apply(action).unwrap(), Change::View);
    assert_ne!(editor.camera.uniform(), before);
    let moved = editor.camera.uniform();
    editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.scene.objects().len(), 2);
    assert_eq!(editor.camera.uniform(), moved);
}
```

## <a id="checkpoint-21-gestures"></a>6. Connect captured pointers to the editor

**What this code does:** Orbit with a right drag, pick with a left click, and stop safely when the pointer or window loses focus.

### 1. `Cargo.toml`

Enable the browser event bindings used by the command dock.

**Locate this existing block:**

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "Event", "PointerEvent", "MouseEvent", "KeyboardEvent", "WheelEvent", "FocusOptions", "DomRect", "HtmlElement"] }
console_error_panic_hook = "=0.1.7"
```

Edit the existing block to match this code; keep unchanged lines:

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "FocusOptions", "HtmlElement"] }
console_error_panic_hook = "=0.1.7"
```

### 2. `src/browser.rs`

Import the gesture state. Retain the canvas and one gesture across pointer events. Begin choosing between submitted commands and pointer gestures. Use pointer gestures only when the dock has not consumed input; apply any resulting action. Use the retained pointer canvas for resizing. Keep the callback open for pointer event registration. Track pointer presses, capture and release; cancel on blur or resize and suppress the browser context menu.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    )?;
    window
        .add_event_listener_with_callback("resize", update.as_ref().unchecked_ref())?;
    // Both event sources retain this one callback for the lifetime of the page.
    update.forget();
    report("Canvas pixels, depth and camera resize together.");
    Ok(())
}
```

Replace it with:

```rust
    )?;
    window
        .add_event_listener_with_callback("resize", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("blur", update.as_ref().unchecked_ref())?;
    // Both event sources retain this one callback for the lifetime of the page.
    update.forget();
    report(
        "Right-drag to orbit. Left-click to select. A cancelled drag stops immediately.",
    );
    Ok(())
}

fn pointer_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
    gesture: &mut Gesture,
) -> Option<Action> {
    match event.type_().as_str() {
        "contextmenu" => {
            event.prevent_default();
            return None;
        }
        "blur" | "resize" => {
            gesture.cancel();
            return None;
        }
        _ => {}
    }
    let pointer = event.dyn_ref::<web_sys::PointerEvent>()?;
    let id = pointer.pointer_id();
    let position = [
        pointer.client_x() as f64,
        pointer.client_y() as f64,
    ];
    let motion = match event.type_().as_str() {
        "pointerdown" => {
            if pointer.is_primary() && gesture.press(id, pointer.button(), position) {
                event.prevent_default();
                if canvas.set_pointer_capture(id).is_err() {
                    gesture.cancel();
                }
            }
            None
        }
        "pointermove" => gesture.move_to(id, position),
        "pointerup" => {
            let motion = gesture.release(id, position);
            let _ = canvas.release_pointer_capture(id);
            motion
        }
        "pointercancel" | "lostpointercapture" => {
            gesture.cancel_pointer(id);
            None
        }
        _ => None,
    }?;
    let rect = canvas.get_bounding_client_rect();
    motion.action([
        rect.left(),
        rect.top(),
        rect.width(),
        rect.height(),
    ])
}
```

**Locate this block:**

```rust
        "keydown",
        "keyup",
        "blur",
        "click",
    ] {
        canvas
            .add_event_listener_with_callback(name, update.as_ref().unchecked_ref())?;
```

Replace it with:

```rust
        "keydown",
        "keyup",
        "blur",
    ] {
        canvas
            .add_event_listener_with_callback(name, update.as_ref().unchecked_ref())?;
```

**Locate this block:**

```rust
            }
            if resize(
                &browser_window,
                &input_canvas,
                &surface,
                &mut config,
                &mut renderer,
```

Replace it with:

```rust
            }
            if resize(
                &browser_window,
                &pointer_canvas,
                &surface,
                &mut config,
                &mut renderer,
```

**Locate this block:**

```rust
                    "view reset" => Action::ResetView,
                    _ => return,
                };
                match editor.apply(action) {
                    Ok(Change::Scene) => {
                        renderer.set_scene(&editor.scene, editor.selected)
```

Replace it with:

```rust
                    "view reset" => Action::ResetView,
                    _ => return,
                };
                Some(action)
            } else if !panel.consumed {
                pointer_action(&event, &pointer_canvas, &mut gesture)
            } else {
                None
            };
            if let Some(action) = action {
                match editor.apply(action) {
                    Ok(Change::Scene) => {
                        renderer.set_scene(&editor.scene, editor.selected)
```

**Locate this block:**

```rust
                    return;
                }
            };
            let line = line.or_else(|| {
                let mouse = event.dyn_ref::<web_sys::MouseEvent>()?;
                (event.type_() == "click"
                    && !panel.consumed
                    && mouse
                        .target()
                        .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
                        .is_some_and(|target| target.id() == "canvas"))
                .then(|| "canvas".into())
            });
            if let Some(line) = line {
                let action = match line.as_str() {
                    "canvas" => {
                        let Some(event) = event.dyn_ref::<web_sys::MouseEvent>() else {
                            return;
                        };
                        let rect = input_canvas.get_bounding_client_rect();
                        if rect.width() <= 0.0 || rect.height() <= 0.0 {
                            return;
                        }
                        Action::Pick([
                            (2.0 * (event.client_x() as f64 - rect.left())
                                / rect.width()
                                - 1.0) as f32,
                            (1.0 - 2.0 * (event.client_y() as f64 - rect.top())
                                / rect.height()) as f32,
                        ])
                    }
                    "example box" => Action::AddBox,
                    "example triangle" => Action::ToggleExtra,
                    "select next" => Action::SelectNext,
```

Replace it with:

```rust
                    return;
                }
            };
            let action = if let Some(line) = line {
                let action = match line.as_str() {
                    "example box" => Action::AddBox,
                    "example triangle" => Action::ToggleExtra,
                    "select next" => Action::SelectNext,
```

**Locate this block:**

```rust
    }
    let input_canvas = canvas.clone();
    let browser_window = window.clone();
    let update =
        Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
            let line = match panel.update(Some(&event), &input_canvas) {
```

Replace it with:

```rust
    }
    let input_canvas = canvas.clone();
    let browser_window = window.clone();
    let pointer_canvas = canvas.clone();
    let mut gesture = Gesture::default();
    let update =
        Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
            let line = match panel.update(Some(&event), &input_canvas) {
```

**Locate this block:**

```rust
use crate::editor::{Action, Change, Editor};
use crate::renderer::Renderer;
use crate::viewport::Viewport;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
```

Replace it with:

```rust
use crate::editor::{Action, Change, Editor};
use crate::gesture::Gesture;
use crate::renderer::Renderer;
use crate::viewport::Viewport;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
```

## <a id="checkpoint-21a-wheel"></a>7. Convert wheel units to camera zoom

**What this code does:** Normalize pixels, lines and pages to bounded camera zoom without adding document history.

### 1. `src/navigation.rs`

Normalize wheel units into the existing Zoom action. Zero or invalid input returns None.

Create the file and type:

```rust
use crate::editor::Action;

pub fn wheel(delta: f64, mode: u32, height: f64) -> Option<Action> {
    if !delta.is_finite() || delta == 0.0 || !height.is_finite() || height <= 0.0 {
        return None;
    }
    let unit = match mode {
        0 => 1.0,
        1 => 16.0,
        2 => height,
        _ => return None,
    };
    let pixels = (delta * unit).clamp(-600.0, 600.0);
    Some(Action::Zoom((-0.002 * pixels).exp() as f32))
}
```

### 2. `src/navigation_tests.rs`

Optional authoring fixture. Compare equivalent wheel inputs, bound large jumps, and show that navigation preserves document history.

Create the file and type:

```rust
use crate::{
    editor::{Action, Change, Editor},
    navigation::wheel,
};

fn distance(delta: f64, mode: u32) -> f64 {
    let mut editor = Editor::default();
    assert_eq!(
        editor
            .apply(wheel(delta, mode, 480.0).unwrap())
            .unwrap(),
        Change::View
    );
    editor.camera.distance
}

#[test]
fn wheel_units_agree_and_small_movements_accumulate() {
    assert!((distance(48.0, 0) - distance(3.0, 1)).abs() < 1.0e-6);
    assert!((distance(48.0, 0) - distance(0.1, 2)).abs() < 1.0e-6);
    let mut editor = Editor::default();
    for _ in 0..10 {
        editor
            .apply(wheel(-12.0, 0, 480.0).unwrap())
            .unwrap();
    }
    assert!((editor.camera.distance - distance(-120.0, 0)).abs() < 1.0e-5);
    editor
        .apply(wheel(120.0, 0, 480.0).unwrap())
        .unwrap();
    assert!((editor.camera.distance - 3.0).abs() < 1.0e-5);
}

#[test]
fn wheel_rejects_invalid_input_and_bounds_large_jumps() {
    for delta in [0.0, f64::NAN, f64::INFINITY] {
        assert!(wheel(delta, 0, 480.0).is_none());
    }
    for height in [0.0, -1.0, f64::NAN] {
        assert!(wheel(10.0, 0, height).is_none());
    }
    assert!(wheel(10.0, 99, 480.0).is_none());
    assert_eq!(distance(f64::MAX, 2), distance(600.0, 0));
    assert_eq!(distance(-f64::MAX, 2), distance(-600.0, 0));
}

#[test]
fn navigation_preserves_document_history() {
    let mut editor = Editor::default();
    editor.apply(Action::AddBox).unwrap();
    editor
        .apply(wheel(-120.0, 0, 480.0).unwrap())
        .unwrap();
    let zoomed = editor.camera.distance;
    editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.scene.objects().len(), 2);
    assert_eq!(editor.camera.distance, zoomed);
    editor.apply(Action::Redo).unwrap();
    assert_eq!(editor.scene.objects().len(), 3);
    editor.apply(Action::SelectNext).unwrap();
    let selected = editor.selected.unwrap();
    editor.apply(Action::Delete).unwrap();
    assert!(!editor.scene.contains(selected));
    editor.apply(Action::Undo).unwrap();
    assert!(editor.scene.contains(selected));
}
```

### 3. `src/lib.rs`

Register the small input translator and its tests.

**Locate this existing block:**

```rust
pub mod mesh;
pub mod picking;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod mesh;
pub mod navigation;
#[cfg(test)]
mod navigation_tests;
pub mod picking;
```

## <a id="checkpoint-22-shortcuts"></a>8. Deliver wheel input without keyboard feature shortcuts

**What this code does:** Normalize wheel input and keep every keyboard feature command in the command dock.

### 1. `Cargo.toml`

Enable the browser event bindings used by the command dock.

**Locate this existing block:**

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "FocusOptions", "HtmlElement"] }
console_error_panic_hook = "=0.1.7"
```

Edit the existing block to match this code; keep unchanged lines:

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "FocusOptions"] }
console_error_panic_hook = "=0.1.7"
```

### 2. `src/browser.rs`

Route unconsumed canvas input through navigation_action. Retain the shared callback and dispatch wheel or pointer navigation. Focus the canvas and capture accepted pointer presses; cancel if either operation fails.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        "pointerdown" => {
            if pointer.is_primary() && gesture.press(id, pointer.button(), position) {
                event.prevent_default();
                if canvas.set_pointer_capture(id).is_err() {
                    gesture.cancel();
```

Replace it with:

```rust
        "pointerdown" => {
            if pointer.is_primary() && gesture.press(id, pointer.button(), position) {
                event.prevent_default();
                let options = web_sys::FocusOptions::new();
                options.set_prevent_scroll(true);
                if canvas.focus_with_options(&options).is_err()
                    || canvas.set_pointer_capture(id).is_err()
                {
                    gesture.cancel();
```

**Locate this block:**

```rust
    // Both event sources retain this one callback for the lifetime of the page.
    update.forget();
    report(
        "Right-drag to orbit. Left-click to select. A cancelled drag stops immediately.",
    );
    Ok(())
}

fn pointer_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
```

Replace it with:

```rust
    // Both event sources retain this one callback for the lifetime of the page.
    update.forget();
    report(
        "Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.",
    );
    Ok(())
}

fn navigation_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
    gesture: &mut Gesture,
) -> Option<Action> {
    let action = match event.type_().as_str() {
        "wheel" => {
            let wheel = event.dyn_ref::<web_sys::WheelEvent>()?;
            let mode = wheel.delta_mode();
            crate::navigation::wheel(
                wheel.delta_y(),
                mode,
                canvas.get_bounding_client_rect().height(),
            )
        }
        "keydown" => {
            let key = event.dyn_ref::<web_sys::KeyboardEvent>()?;
            if key.is_composing() || key.alt_key() {
                return None;
            }
            if key.key() == "Escape" {
                gesture.cancel();
                event.prevent_default();
                return None;
            }
            return None;
        }
        _ => return pointer_action(event, canvas, gesture),
    }?;
    event.prevent_default();
    Some(action)
}

fn pointer_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
```

**Locate this block:**

```rust
            } else if !panel.consumed {
                pointer_action(&event, &pointer_canvas, &mut gesture)
            } else {
                None
            };
```

Replace it with:

```rust
            } else if !panel.consumed {
                navigation_action(&event, &pointer_canvas, &mut gesture)
            } else {
                None
            };
```

## Build & Run

![Recorded visual output for Lesson 3.2 (22-shortcuts)](../../screenshots/journey/22-shortcuts-browser.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 3.2
npm --prefix ../session_tests run course -- save 3.2
```

Continue with [3.3 · Validated files, bounds and useful projections](22a-records.md).
