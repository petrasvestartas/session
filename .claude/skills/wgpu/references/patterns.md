# wgpu viewer patterns — techniques, not signatures

Rendering techniques researched for this viewer: thick lines, point clouds,
colour-ID picking, gizmos, egui, instancing, wasm traps, kernel integration.

**Read `../SKILL.md` first, and treat every API snippet below as older than the
pin.** These were written against earlier wgpu and were not all re-verified;
they exist to explain *which approach* to take, not to be pasted. The current
signature comes from the vendored source, and the current working code comes
from `session_viewer/src/engine/`. Where the two disagree, this file loses.

## Core References

| Resource | URL |
|----------|-----|
| wgpu repo | https://github.com/gfx-rs/wgpu |
| wgpu web examples | https://wgpu.rs/examples/ |
| WebGPU spec (W3C) | https://www.w3.org/TR/webgpu/ |
| WGSL spec (W3C) | https://www.w3.org/TR/WGSL/ |
| WGSL alignment calculator | https://www.w3.org/2025/webgpu/wgsl-align.html |
| webgpufundamentals | https://webgpufundamentals.org/ |

learn-wgpu is deliberately absent — see the "Do not read" section of
`../SKILL.md`.

## Project Location
- `session_viewer/` — Rust wgpu viewer, wasm32 by default
- `session_viewer/src/engine/gpu/device.rs` — instance/adapter/device negotiation
- `session_viewer/src/engine/pipelines/mod.rs` — pipeline + layout construction
- `session_viewer/src/shaders/*.wgsl` — the nine shader lanes
- `session_viewer/examples/selftest.rs` — headless frame + ink count

## Build Tooling

Trunk. `wasm-pack` is far slower on Linux; `cargo-web` is abandoned.

```bash
trunk serve            # dev server, live reload, port 8770
trunk build --release
```

`session_viewer/Trunk.toml` sets `release = true` **for `serve` too**, and the
comment block in it holds the measured debug-vs-release load numbers — read it
before changing the build profile or quoting a load time.

`.cargo/config.toml` makes `wasm32-unknown-unknown` the default target, which
is why the source carries no `#[cfg(target_arch = "wasm32")]` gates and why
tests need the `cargo xtest` alias.

`Cargo.toml` needs `crate-type = ["cdylib", "rlib"]` — `cdylib` for wasm,
`rlib` so `examples/` can link the crate.

## Dependency notes

The manifest is the source of truth; read `session_viewer/Cargo.toml`. Two
standing constraints that a generic tutorial will violate:

- **No `webgl` feature.** The browser path is WebGPU-only
  (`Backends::BROWSER_WEBGPU`). Do not add a WebGL2 fallback or
  `Limits::downlevel_webgl2_defaults()`.
- **No external geometry or math crate.** Never `glam`, never `nalgebra` — the
  kernel's own types do this work.

## Data Loading

### A — In-Memory (Rust structs → GPU buffers)

```rust
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct GpuVertex {
    pub position: [f32; 3],
    pub _pad0:    f32,       // WGSL vec3f = 16 bytes, pad to avoid silent corruption
    pub normal:   [f32; 3],
    pub _pad1:    f32,
    pub color:    [f32; 4],
}

let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    label:    Some("Vertex Buffer"),
    contents: bytemuck::cast_slice(&vertices),
    usage:    wgpu::BufferUsages::VERTEX,
});
```

**WGSL alignment gotcha:** `vec3f` = 16-byte aligned, NOT 12. Always pad.
Check any host-shareable struct with https://www.w3.org/2025/webgpu/wgsl-align.html

### B — Protobuf (prost)

`prost` is wasm-compatible. Session proto schemas are in `session_proto/`.

```rust
use prost::Message;
let mesh = session_proto::Mesh::decode(bytes)?;
```

Fetch binary from server in wasm:
```rust
let resp         = JsFuture::from(window.fetch_with_str("/data/scene.pb")).await?;
let resp: web_sys::Response = resp.dyn_into()?;
let array_buffer = JsFuture::from(resp.array_buffer()?).await?;
let bytes        = js_sys::Uint8Array::new(&array_buffer).to_vec();
```

### C — JSON (serde_json)

`serde_json::from_str` / `from_slice` work in wasm unchanged.
For passing to/from JS use `serde_wasm_bindgen`:

```rust
#[wasm_bindgen]
pub fn load_scene_json(json_str: &str) -> Result<JsValue, JsValue> {
    let scene: SceneData = serde_json::from_str(json_str)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    serde_wasm_bindgen::to_value(&scene)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}
```

---

## Camera

The viewer has its own camera — read `session_viewer/src/` rather than the
older standalone https://github.com/petrasvestartas/wgpu_viewer, which is kept
only as a design record. The design decisions below still hold; the file names
and the uniform layout are from that older repo and no longer match.

### CameraUniform (GPU buffer layout)
```rust
struct CameraUniform {
    view_position: [f32; 4],       // camera world pos for lighting
    view_proj:     [[f32; 4]; 4],  // combined view-projection matrix
    aspect_ratio:  [f32; 4],       // viewport aspect ratio
}
```

### Controls
| Input | Action |
|-------|--------|
| Right mouse drag | Arcball orbit around target |
| Middle mouse drag | Pan (translate parallel to view plane) |
| Scroll wheel | Zoom, clamped [0.5, 100.0] |
| WASD / arrows | Keyboard pan |
| C | Reset to initial view `(0,10,10)` looking at origin |

### Key Design Decisions
- **Quaternion rotation** — no gimbal lock, continuously normalized
- **Pole stability** — tracks "last stable right vector" to prevent 180° flip at poles
- **Z-up** — matches Blender/Maya convention (not Y-up)
- **Turntable mode on by default** — maintains world-up orientation
- **Single bind group** — all geometry types (mesh, lines, points, pipes, polygons) share the same camera bind group
- **MSAA 4×** on all pipelines, depth format `Depth32Float`, compare `Less`
- OpenGL→wgpu coordinate system conversion applied in matrix computation


---

## Shaders

### Flat Shading (WGSL)

No geometry shaders in WebGPU. Two options:

**A — precompute face normals on CPU** (recommended): store per-vertex normal = face normal, pass as vertex attribute.

**B — screen-space derivatives** (no CPU work, less precise):
```wgsl
@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
    let face_normal = normalize(cross(dpdx(in.world_pos), dpdy(in.world_pos)));
    // use face_normal for lighting
}
```

### Blinn-Phong Lighting

```wgsl
fn blinn_phong(normal: vec3<f32>, view_dir: vec3<f32>, light_dir: vec3<f32>, light_color: vec3<f32>) -> vec3<f32> {
    let ambient  = 0.1 * light_color;
    let diffuse  = max(dot(normal, -light_dir), 0.0) * light_color;
    let halfway  = normalize(-light_dir + view_dir);
    let specular = pow(max(dot(normal, halfway), 0.0), 32.0) * light_color;
    return ambient + diffuse + specular;
}
```

---

## Line Rendering

No geometry shaders, no thick line primitive in WebGPU. `POLYGON_MODE_LINE` is native-only.

**Expanded lines** — triangle strip per segment (recommended):
- Vertex shader expands endpoints perpendicular to direction in screen space
- Reference: https://mattdesl.svbtle.com/drawing-lines-is-hard
- wgpu example: https://github.com/m-schuetz/webgpu_wireframe_thicklines

**lyon** — tessellates paths to triangles before GPU (wireframes, SVG strokes):
- https://github.com/nical/lyon (has wgpu example in `examples/wgpu`)

---

## Point Cloud

**Instanced billboards** (recommended — scales to millions):
- 1 quad per point; vertex shader billboards it to face camera
- Instance buffer: `[position: vec3, _pad: f32, color: vec4]`
- Reference: https://github.com/m-schuetz/webgpu_pointcloud

**Point primitive** (simple, 1px only):
```rust
topology: wgpu::PrimitiveTopology::PointList
```

---

## GPU Picking / Element Selection

**Color ID picking:**
1. Render to off-screen texture with object IDs encoded as RGB colors
2. On click: `copy_texture_to_buffer` → staging buffer → `map_async` → read pixel
3. Decode ID → selected object
4. Optimization: scissor rect to 1×1 pixel at cursor

```rust
let pick_texture = device.create_texture(&wgpu::TextureDescriptor {
    usage:  wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
    format: wgpu::TextureFormat::Rgba8Unorm,
    ..
});
```

References:
- https://webgpufundamentals.org/webgpu/lessons/webgpu-picking.html
- wgpu `render_to_texture` example: https://github.com/gfx-rs/wgpu/tree/trunk/examples

**Box selection**: read all pixels in drag rect from ID texture → accumulate into `HashSet<u32>`.

---

## Transform Gizmo

**`transform-gizmo`** — framework-agnostic, v0.9.0, uses `mint` types (glam/nalgebra compatible):
- https://github.com/urholaukkarinen/transform-gizmo
- Demo: https://urholaukkarinen.github.io/transform-gizmo/

**`transform-gizmo-egui`** — egui integration (renders on top of wgpu scene):
- https://crates.io/crates/transform-gizmo-egui

**`egui-gizmo`** — alternative (EmbarkStudios):
- https://github.com/EmbarkStudios/egui-gizmo

Math:
- Translation: ray-axis intersection (ray from camera through cursor, project onto axis)
- Rotation: arcball/trackball via quaternions — https://raw.org/code/trackball-rotation-using-quaternions/

---

## UI Overlay (egui)

```rust
let egui_ctx      = egui::Context::default();
let mut egui_state    = egui_winit::State::new(egui_ctx.clone(), ViewportId::default(), &event_loop, None, None, None);
let mut egui_renderer = egui_wgpu::Renderer::new(&device, surface_format, None, 1, false);

// Each frame:
let raw_input   = egui_state.take_egui_input(&window);
let full_output = egui_ctx.run(raw_input, |ctx| {
    egui::SidePanel::left("controls").show(ctx, |ui| { ui.label("Session Viewer"); });
});
egui_state.handle_platform_output(&window, full_output.platform_output);
let tris = egui_ctx.tessellate(full_output.shapes, full_output.pixels_per_point);
// upload + render via egui_renderer in a render pass
```

egui works in wasm unchanged. `fragile-send-sync-non-atomic-wasm` makes wgpu objects `Sync` on wasm.

---

## Buffer Optimization / Frustum Culling

**Instanced draws** — batch same geometry type:
```rust
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct Instance { transform: [[f32; 4]; 4], color: [f32; 4] }
// render_pass.draw_indexed(0..index_count, 0, 0..instance_count)
```
Working example: the instance buffers in `session_viewer/src/engine/`.

**Indirect draw + GPU frustum culling** (no CPU readback):
- https://github.com/toji/webgpu-bundle-culling

**Render bundles** (pre-record, replay without re-encoding):
- https://toji.dev/webgpu-best-practices/render-bundles.html

**Optimization guide:** https://webgpufundamentals.org/webgpu/lessons/webgpu-optimization.html

---

## WASM Gotchas

| Issue | Detail |
|-------|--------|
| `vec3f` alignment | 16 bytes in WGSL, not 12 — silent data corruption without error |
| `POLYGON_MODE_LINE` | native-only — unavailable on web |
| `crate-type` | must include `"cdylib"` or wasm build silently fails |
| SharedArrayBuffer | wgpu incompatible with wasm threads / rayon on web |
| WebGPU browser flag | Chrome: enable "Unsafe WebGPU" for local testing |
| Canvas context | `get_context("webgpu")` can fail silently — always error-check |
| wgpu features | this viewer is WebGPU-only: no `webgl` feature, no fallback |
| no logger | wgpu reports validation errors through `log` — without a sink a broken shader is just a black frame |
| hybrid GPUs | `PowerPreference::LowPower` on purpose: the discrete GPU renders but cannot share frames to the compositor, and the canvas stays black |

---

## Session Kernel Integration

- Session outputs: `Mesh`, `NurbsCurve`, `NurbsSurface`, `Line`, `Point`, `Color`
- Proto schemas: `session_proto/`
- JSON test data: `session_tests/public/testData.js`
- Render layers: mesh (flat shaded) + wireframe edges + naked edges + points + lines
- GPU vertex format maps from `session_proto::Mesh` vertices/faces
