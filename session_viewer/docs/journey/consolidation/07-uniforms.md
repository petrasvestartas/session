# 2.2 · Camera state and the world-to-clip transform

**Goal:** Rotate the flat view using a matrix, ready for the third dimension. · **Steps:** 3 · **Source:** `src/camera.rs,src/engine/gpu/render.rs,src/shaders/scene.wgsl`

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">3D World Space (x, y, z, 1)</text>
  <line x1="110" y1="136" x2="250" y2="136" stroke="#dc2626" stroke-width="2" marker-end="url(#arrR)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="262" y="140" font-family="monospace" font-size="10" font-weight="700" fill="#dc2626">+X</text>
  <line x1="110" y1="136" x2="110" y2="50" stroke="#2563eb" stroke-width="2" marker-end="url(#arrB)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="110" y="44" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#2563eb">+Z</text>
  <line x1="110" y1="136" x2="62" y2="164" stroke="#16a34a" stroke-width="2" marker-end="url(#arrG)"/>
  <polygon points="156,118 216,96 184,58" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <circle cx="80" cy="76" r="5" fill="#facc15" stroke="#111111" stroke-width="1.5"/>
  <line x1="85" y1="78" x2="176" y2="94" stroke="#111111" stroke-width="1.5" stroke-dasharray="3 3" marker-end="url(#arr)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="80" y="66" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">eye</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">@vertex: clip = M_proj * M_view * p</text>
  <polygon points="444,82 516,50 516,158 444,126" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="444" y1="82" x2="660" y2="46" stroke="#64748b" stroke-width="1.25" stroke-dasharray="3 3"/>
  <line x1="444" y1="126" x2="660" y2="162" stroke="#64748b" stroke-width="1.25" stroke-dasharray="3 3"/>
  <polygon points="600,56 668,46 668,162 600,148" fill="none" stroke="#111111" stroke-width="1.5"/>
  <polygon points="536,84 584,72 562,130" fill="#bfdbfe" stroke="#2563eb" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="164" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">ndc = clip.xyz / clip.w</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Viewer Perspective / Orbit Frame</text>
  <rect x="814" y="44" width="240" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <path d="M 858 124 A 78 28 0 1 1 1010 124" fill="none" stroke="#2563eb" stroke-width="1.75" stroke-dasharray="4 3" marker-end="url(#arrB)"/>
  <polygon points="934,60 886,128 986,136" fill="#93c5fd" stroke="#111111" stroke-width="2"/>
  <circle cx="934" cy="112" r="4" fill="#dc2626"/>
</svg>
</div>

## <a id="checkpoint-07-uniforms"></a>1. Send one view setting to every corner

**What this code does:** Scale and shift the diamond without changing its stored positions.

### 1. `src/triangle.wgsl`

Declare the uniform resource before the vertex function. Its group and binding identify a resource slot. Use the first pair as scale and the second pair as offset. The swizzles xy and zw select those pairs.

**Locate this existing block:**

```wgsl
@vertex
fn vertex(@location(0) position: vec2<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(position, 0.0, 1.0);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(0) @binding(0) var<uniform> transform: vec4<f32>;

@vertex
fn vertex(@location(0) position: vec2<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(position * transform.xy + transform.zw, 0.0, 1.0);
}
```

### 2. `src/renderer.rs`

Retain the uniform buffer and bind group. Allocate the uniform buffer and bind it at the shader’s resource slot. Bind the view settings before drawing.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                ..Default::default()
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_vertex_buffer(0, self.vertices.slice(..));
```

Replace it with:

```rust
                ..Default::default()
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            pass.set_vertex_buffer(0, self.vertices.slice(..));
```

**Locate this block:**

```rust
        &self,
        view: &wgpu::TextureView,
        background: &crate::background::Background,
    ) {
        let [r, g, b] = background.rgb();
        let mut encoder = self
            .device
```

Replace it with:

```rust
        &self,
        view: &wgpu::TextureView,
        background: &crate::background::Background,
        transform: &[f32; 4],
    ) {
        let bytes: Vec<u8> = transform
            .iter()
            .flat_map(|value| value.to_ne_bytes())
            .collect();
        self.queue
            .write_buffer(&self.uniform, 0, &bytes);
        let [r, g, b] = background.rgb();
        let mut encoder = self
            .device
```

**Locate this block:**

```rust
            multiview_mask: None,
            cache: None,
        });
        Self {
            device,
            queue,
            pipeline,
            vertices,
            indices,
        }
    }
```

Replace it with:

```rust
            multiview_mask: None,
            cache: None,
        });
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let view_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view settings"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        });
        Self {
            device,
            queue,
            pipeline,
            vertices,
            indices,
            uniform,
            view_group,
        }
    }
```

**Locate this block:**

```rust
    indices: wgpu::Buffer,
}

impl Renderer {
```

Replace it with:

```rust
    indices: wgpu::Buffer,
    uniform: wgpu::Buffer,
    view_group: wgpu::BindGroup,
}

impl Renderer {
```

### 3. `src/browser.rs`

Set the initial scale and translation for presentation. Redraw with the same transform after a command. Report the shared view setting. Accept the transform at the presentation boundary. Pass the transform to the renderer.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            format: Some(frame.texture.format().add_srgb_suffix()),
            ..Default::default()
        });
    renderer.draw(&view, background);
    panel.draw(renderer, &view);
```

Replace it with:

```rust
            format: Some(frame.texture.format().add_srgb_suffix()),
            ..Default::default()
        });
    renderer.draw(&view, background, transform);
    panel.draw(renderer, &view);
```

**Locate this block:**

```rust
    surface: &wgpu::Surface<'_>,
    renderer: &Renderer,
    background: &Background,
    panel: &mut crate::panel::Panel,
) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
```

Replace it with:

```rust
    surface: &wgpu::Surface<'_>,
    renderer: &Renderer,
    background: &Background,
    transform: &[f32; 4],
    panel: &mut crate::panel::Panel,
) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
```

**Locate this block:**

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Four shared corners make two triangles.");
    Ok(())
}
```

Replace it with:

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("One view setting moves all four corners.");
    Ok(())
}
```

**Locate this block:**

```rust
                ));
                return;
            }
            if let Err(error) = present(&surface, &renderer, &background, &mut panel) {
                report(&format!("Cannot redraw: {error:?}"));
            }
        });
```

Replace it with:

```rust
                ));
                return;
            }
            if let Err(error) = present(
                &surface,
                &renderer,
                &background,
                &transform,
                &mut panel,
            ) {
                report(&format!("Cannot redraw: {error:?}"));
            }
        });
```

**Locate this block:**

```rust
    let mut background = Background::default();
    present(&surface, &renderer, &background, &mut panel)?;
    let input_canvas = canvas.clone();
    let click =
        Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
```

Replace it with:

```rust
    let mut background = Background::default();
    let transform = [0.75, 0.75, 0.25, 0.15];
    present(
        &surface,
        &renderer,
        &background,
        &transform,
        &mut panel,
    )?;
    let input_canvas = canvas.clone();
    let click =
        Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
```

## <a id="checkpoint-08-camera"></a>2. Move the view, keep the geometry

**What this code does:** Pan, zoom and reset a flat view through camera state.

### 1. `src/camera.rs`

Create the camera state and its two tests. There are no browser or wgpu types in this file.

Create the file and type:

```rust
pub struct Camera {
    pub center: [f32; 2],
    pub scale: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            center: [0.0, 0.0],
            scale: 1.0,
        }
    }
}

impl Camera {
    pub fn pan(&mut self, dx: f32, dy: f32) {
        self.center[0] += dx;
        self.center[1] += dy;
    }

    pub fn zoom(&mut self, factor: f32) {
        if factor.is_finite() && factor > 0.0 {
            self.scale = (self.scale * factor).clamp(0.1, 10.0);
        }
    }

    pub fn uniform(&self) -> [f32; 4] {
        // Subtract the camera centre before scaling the world position.
        [
            self.scale,
            self.scale,
            -self.center[0] * self.scale,
            -self.center[1] * self.scale,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panned_target_stays_at_the_screen_centre_when_zooming() {
        let mut camera = Camera::default();
        camera.pan(0.25, -0.5);
        camera.zoom(2.0);
        let [sx, sy, ox, oy] = camera.uniform();
        assert_eq!(camera.center[0] * sx + ox, 0.0);
        assert_eq!(camera.center[1] * sy + oy, 0.0);
    }

    #[test]
    fn zoom_stays_positive_and_bounded() {
        let mut camera = Camera::default();
        camera.zoom(0.0001);
        assert_eq!(camera.scale, 0.1);
        camera.zoom(1_000.0);
        assert_eq!(camera.scale, 10.0);
        camera.zoom(f32::NAN);
        camera.zoom(-1.0);
        assert_eq!(camera.scale, 10.0);
    }
}
```

### 2. `src/lib.rs`

Register the camera module so the browser and native tests can use it.

**Locate this existing block:**

```rust
mod browser;
pub mod renderer;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod browser;
pub mod camera;
pub mod renderer;
```

### 3. `src/browser.rs`

Import the camera owner. Add camera commands and present its initial uniform. Apply the submitted camera command, then redraw with its uniform. Report that navigation leaves the mesh unchanged.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("One view setting moves all four corners.");
    Ok(())
```

Replace it with:

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Pan and zoom change the view, not the mesh.");
    Ok(())
```

**Locate this block:**

```rust
                &surface,
                &renderer,
                &background,
                &transform,
                &mut panel,
            ) {
                report(&format!("Cannot redraw: {error:?}"));
```

Replace it with:

```rust
                &surface,
                &renderer,
                &background,
                &camera.uniform(),
                &mut panel,
            ) {
                report(&format!("Cannot redraw: {error:?}"));
```

**Locate this block:**

```rust
                    return;
                }
            };
            if line.as_deref() == Some("background") {
                background.toggle();
            }
            if let Err(error) = panel.update(None, &input_canvas) {
                report(&format!(
```

Replace it with:

```rust
                    return;
                }
            };

            if let Some(line) = line {
                match line.as_str() {
                    "background" => background.toggle(),
                    "zoom in" => camera.zoom(2.0),
                    "zoom out" => camera.zoom(0.5),
                    "pan left" => camera.pan(-0.25, 0.0),
                    "pan right" => camera.pan(0.25, 0.0),
                    "view reset" => camera = Camera::default(),
                    _ => return,
                }
            }
            if let Err(error) = panel.update(None, &input_canvas) {
                report(&format!(
```

**Locate this block:**

```rust
    let mut panel = crate::panel::Panel::new(
        &renderer,
        config.format.add_srgb_suffix(),
        &["Help", "Background"],
    );
    panel.update(None, &canvas)?;
    let mut background = Background::default();
    let transform = [0.75, 0.75, 0.25, 0.15];
    present(
        &surface,
        &renderer,
        &background,
        &transform,
        &mut panel,
    )?;
    let input_canvas = canvas.clone();
```

Replace it with:

```rust
    let mut panel = crate::panel::Panel::new(
        &renderer,
        config.format.add_srgb_suffix(),
        &[
            "Help",
            "Background",
            "Zoom In",
            "Zoom Out",
            "Pan Left",
            "Pan Right",
            "View Reset",
        ],
    );
    panel.update(None, &canvas)?;
    let mut background = Background::default();
    let mut camera = Camera::default();
    present(
        &surface,
        &renderer,
        &background,
        &camera.uniform(),
        &mut panel,
    )?;
    let input_canvas = canvas.clone();
```

**Locate this block:**

```rust
use crate::background::Background;
use crate::renderer::Renderer;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
```

Replace it with:

```rust
use crate::background::Background;
use crate::camera::Camera;
use crate::renderer::Renderer;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
```

## <a id="checkpoint-09-matrices"></a>3. Let one matrix describe the view

**What this code does:** Rotate the flat view using a matrix, ready for the third dimension.

### 1. `src/camera.rs`

Keep the viewing angle in radians beside the centre and scale. Start without rotation and make Reset view restore that angle. Replace the four-number uniform with a column-major matrix and add rotation. Strengthen the camera test: its target must stay centred after both zooming and rotation.

**Locate this existing block:**

```rust
    pub scale: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            center: [0.0, 0.0],
            scale: 1.0,
        }
    }
}

impl Camera {
    pub fn pan(&mut self, dx: f32, dy: f32) {
        self.center[0] += dx;
        self.center[1] += dy;
    }

    pub fn zoom(&mut self, factor: f32) {
        if factor.is_finite() && factor > 0.0 {
            self.scale = (self.scale * factor).clamp(0.1, 10.0);
        }
    }

    pub fn uniform(&self) -> [f32; 4] {
        // Subtract the camera centre before scaling the world position.
        [
            self.scale,
            self.scale,
            -self.center[0] * self.scale,
            -self.center[1] * self.scale,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panned_target_stays_at_the_screen_centre_when_zooming() {
        let mut camera = Camera::default();
        camera.pan(0.25, -0.5);
        camera.zoom(2.0);
        let [sx, sy, ox, oy] = camera.uniform();
        assert_eq!(camera.center[0] * sx + ox, 0.0);
        assert_eq!(camera.center[1] * sy + oy, 0.0);
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub scale: f32,
    pub angle: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            center: [0.0, 0.0],
            scale: 1.0,
            angle: 0.0,
        }
    }
}

impl Camera {
    pub fn pan(&mut self, dx: f32, dy: f32) {
        self.center[0] += dx;
        self.center[1] += dy;
    }

    pub fn zoom(&mut self, factor: f32) {
        if factor.is_finite() && factor > 0.0 {
            self.scale = (self.scale * factor).clamp(0.1, 10.0);
        }
    }

    pub fn rotate(&mut self, radians: f32) {
        self.angle += radians;
    }

    pub fn uniform(&self) -> [f32; 16] {
        let a = self.scale * self.angle.cos();
        let b = self.scale * self.angle.sin();
        let [x, y] = self.center;
        // WGSL matrices are uploaded column by column.
        [
            a,
            -b,
            0.0,
            0.0,
            b,
            a,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            -a * x - b * y,
            b * x - a * y,
            0.0,
            1.0,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panned_target_stays_at_the_screen_centre_when_zooming() {
        let mut camera = Camera::default();
        camera.pan(0.25, -0.5);
        camera.zoom(2.0);
        camera.rotate(std::f32::consts::FRAC_PI_4);
        let m = camera.uniform();
        let [x, y] = camera.center;
        assert!((m[0] * x + m[4] * y + m[12]).abs() < 1.0e-6);
        assert!((m[1] * x + m[5] * y + m[13]).abs() < 1.0e-6);
    }
```

### 2. `src/triangle.wgsl`

Change the resource type to four columns of four floats. Multiply a world position by the matrix. Its final component 1 lets the translation column contribute.

**Locate this existing block:**

```wgsl
@group(0) @binding(0) var<uniform> transform: vec4<f32>;

@vertex
fn vertex(@location(0) position: vec2<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(position * transform.xy + transform.zw, 0.0, 1.0);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(0) @binding(0) var<uniform> transform: mat4x4<f32>;

@vertex
fn vertex(@location(0) position: vec2<f32>) -> @builtin(position) vec4<f32> {
    return transform * vec4<f32>(position, 0.0, 1.0);
}
```

### 3. `src/renderer.rs`

Allocate 64 bytes for sixteen f32 matrix entries. Upload all sixteen matrix entries before drawing.

**Locate this existing block:**

```rust
            label: Some("view transform"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let view_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view settings"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        });
        Self {
            device,
            queue,
            pipeline,
            vertices,
            indices,
            uniform,
            view_group,
        }
    }

    pub fn draw(
        &self,
        view: &wgpu::TextureView,
        background: &crate::background::Background,
        transform: &[f32; 4],
    ) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            label: Some("view transform"),
            size: 64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let view_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view settings"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        });
        Self {
            device,
            queue,
            pipeline,
            vertices,
            indices,
            uniform,
            view_group,
        }
    }

    pub fn draw(
        &self,
        view: &wgpu::TextureView,
        background: &crate::background::Background,
        transform: &[f32; 16],
    ) {
```

### 4. `src/browser.rs`

Add Orbit Right to the vocabulary. Rotate the camera when Orbit Right is submitted. Report the matrix transform. Pass the full matrix into presentation.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    surface: &wgpu::Surface<'_>,
    renderer: &Renderer,
    background: &Background,
    transform: &[f32; 4],
    panel: &mut crate::panel::Panel,
```

Replace it with:

```rust
    surface: &wgpu::Surface<'_>,
    renderer: &Renderer,
    background: &Background,
    transform: &[f32; 16],
    panel: &mut crate::panel::Panel,
```

**Locate this block:**

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Pan and zoom change the view, not the mesh.");
    Ok(())
}
```

Replace it with:

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("A matrix carries pan, scale and rotation.");
    Ok(())
}
```

**Locate this block:**

```rust
                    "zoom out" => camera.zoom(0.5),
                    "pan left" => camera.pan(-0.25, 0.0),
                    "pan right" => camera.pan(0.25, 0.0),
                    "view reset" => camera = Camera::default(),
                    _ => return,
                }
```

Replace it with:

```rust
                    "zoom out" => camera.zoom(0.5),
                    "pan left" => camera.pan(-0.25, 0.0),
                    "pan right" => camera.pan(0.25, 0.0),
                    "orbit right" => camera.rotate(std::f32::consts::FRAC_PI_4),
                    "view reset" => camera = Camera::default(),
                    _ => return,
                }
```

**Locate this block:**

```rust
            "Pan Right",
            "View Reset",
        ],
    );
```

Replace it with:

```rust
            "Pan Right",
            "Orbit Right",
            "View Reset",
        ],
    );
```

## Build & Run

![Recorded visual output for Lesson 2.2 (09-matrices)](../../screenshots/journey/09-matrices-browser.png)

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j1
CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 2.2
npm --prefix ../session_tests run course -- save 2.2
```

Continue with [2.3 · Depth, validated geometry and stable objects](10-depth.md).
