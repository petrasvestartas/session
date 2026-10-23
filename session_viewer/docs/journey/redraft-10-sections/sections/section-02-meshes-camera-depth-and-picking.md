# Section 2: 3D Meshes, Camera Matrices, Depth, and Ray Picking

> **Gemini Notebook Student Guide** · **Absorbs:** Chapter 2 (*Geometry, camera and selection*) · **Lessons:** 6 (`2.1`–`2.6`) · **Checkpoints:** 19 (`05-vertices` → `16-orbit`)

## 1. Learning Outcome

Upload CPU vertex and index arrays into GPU buffers (`wgpu::Buffer`), bind a 4x4 camera `view_proj` uniform matrix (`@group(0) @binding(0)`), enable hardware Z-buffering (`Depth32Float`) so nearer surfaces occlude farther ones, cast 3D perspective viewing rays from mouse coordinates to pick nearest mesh triangles on the CPU, record Undo/Redo history, and orbit/pan/zoom the 3D camera.

## 2. WebGPU & Architecture Schematic (First-Year Student Guide)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 240" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="232" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Chapter 2 WebGPU Pipeline: 3D Vertex/Index Buffers, Camera Uniform (mat4x4), Depth32Float & Ray Picking</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">How 3D world coordinates project onto the 2D canvas with hardware Z-buffering—and how a 2D mouse click unprojects back into a 3D ray.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    <!-- CPU Buffers -->
    <rect x="18" y="68" width="185" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="110" y="88" text-anchor="middle" font-size="12" font-weight="700" fill="#111111">1. Rust CPU Data (#[repr(C)])</text>
    <text x="28" y="108" font-size="11" fill="#111111">• vertices: Vec&lt;[f32; 3]&gt;</text>
    <text x="28" y="125" font-size="11" fill="#111111">• indices:  Vec&lt;u32&gt; (shared!)</text>
    <text x="28" y="142" font-size="11" fill="#111111">• camera.view_proj: [[f32;4];4]</text>
    <text x="28" y="162" font-size="10.5" font-weight="600" fill="#111111">bytemuck::cast_slice(&amp;data)</text>
    <text x="28" y="180" font-size="10.5" fill="#111111">• Undo / Redo Snapshot Stack</text>
    <text x="28" y="197" font-size="10.5" fill="#111111">• Orbit / Pan / Zoom Camera</text>

    <line x1="203" y1="115" x2="233" y2="115" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <!-- Vertex Shader -->
    <rect x="235" y="68" width="205" height="90" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="337" y="87" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">2. @vertex Shader Stage</text>
    <text x="245" y="105" font-size="10.5" fill="#111111">@location(0) pos: vec3&lt;f32&gt;</text>
    <text x="245" y="122" font-size="10.5" fill="#111111">@group(0) @binding(0) view_proj</text>
    <text x="245" y="142" font-size="11" font-weight="700" fill="#111111">clip = view_proj * vec4(pos, 1.0)</text>

    <line x1="440" y1="115" x2="470" y2="115" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <!-- Raster + Depth32Float -->
    <rect x="472" y="68" width="185" height="90" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="564" y="87" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">3. Depth32Float Z-Test</text>
    <text x="482" y="105" font-size="10.5" fill="#111111">NDC = clip.xyz / clip.w</text>
    <text x="482" y="122" font-size="10.5" font-weight="600" fill="#111111">if z_fragment &lt;= z_buffer[x,y]:</text>
    <text x="482" y="139" font-size="10.5" fill="#111111">  write pixel &amp; update z_buffer</text>

    <line x1="657" y1="115" x2="685" y2="115" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <!-- Screen & Reverse Ray Pick -->
    <rect x="687" y="68" width="133" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="753" y="87" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">4. Canvas &amp; Pick</text>
    <text x="696" y="106" font-size="10.5" fill="#111111">• Nearer mesh</text>
    <text x="696" y="120" font-size="10.5" fill="#111111">  occludes rear</text>
    <text x="696" y="142" font-size="10.5" font-weight="600" fill="#111111">• Click (px, py):</text>
    <text x="696" y="157" font-size="10" fill="#111111">  inv_view_proj</text>
    <text x="696" y="172" font-size="10" fill="#111111">  -&gt; 3D Ray(O, D)</text>
    <text x="696" y="187" font-size="10" fill="#111111">  -&gt; min t hit ID!</text>

    <!-- Bottom Reverse Arrow for Ray Picking -->
    <rect x="235" y="168" width="422" height="45" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="446" y="187" text-anchor="middle" font-size="11" font-weight="600" fill="#111111">CPU Ray Picking (Inverse Path): Screen (px, py) → NDC (-1..+1) → inv(view_proj) → World Ray (O + t·D)</text>
    <text x="446" y="203" text-anchor="middle" font-size="10.5" fill="#111111">Tests every triangle via Möller–Trumbore; smallest positive t wins (matching hardware Depth32Float occlusion!)</text>
    
</svg>
</div>

```
+---------------------------------------------------------------------------------------------------+
|                        THE 3D COORDINATE & DEPTH PIPELINE (CPU -> GPU)                            |
+---------------------------------------------------------------------------------------------------+
  CPU Mesh (Rust)                 Uniform Buffer (@group(0) @binding(0))
  vertices: Vec<[f32; 3]> ----+   camera.view_proj: [[f32; 4]; 4]
  indices:  Vec<u32>      --+ |                 |
                            | |                 v
                            v v    +--------------------------------------------+
                      [Vertex Stage: clip_pos = view_proj * vec4(world_pos, 1.0)]
                                                |
                                   Perspective Divide (xyz / w)
                                                |
                                                v
                      [Rasterizer + Depth32Float Z-Buffer Test (z_new <= z_old)]
                                                |
                           +--------------------+--------------------+
                           | Pass: Write new z & fragment color      | Fail: Discard occluded pixel
                           v                                         v
                    [sRGB Canvas View]                          (Hidden Surface)
```

### Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `Browser`)

| Layer | Construct | What It Does in Plain English |
| :--- | :--- | :--- |
| **Rust (`#[repr(C)]`)** | `bytemuck::cast_slice(&vertices)` | Reinterprets typed Rust `f32` arrays as raw `&[u8]` bytes with guaranteed C memory layout for GPU upload. |
| **WGSL (Inputs)** | `@location(0) pos: vec3<f32>` | Reads 12 bytes (`Float32x3`) per vertex from slot 0 according to `VertexBufferLayout`. |
| **WGSL (Uniform)** | `@group(0) @binding(0) var<uniform> view: mat4x4<f32>;` | Reads the 64-byte column-major camera matrix shared by all vertices in the draw call. |
| **Math (Rays)** | `ray = unproject(ndc_x, ndc_y, inv_view_proj)` | Maps a 2D mouse click in CSS pixels back into a 3D world-space ray `(origin, dir)` for CPU triangle picking. |

## 3. Existing Chapters & Lessons Absorbed

| Redraft Lesson | Existing Lesson | Title | Checkpoints | Range (`first` → `last`) | Consolidated Lesson Page |
| :--- | :--- | :--- | ---: | :--- | :--- |
| `2.1` | `2.1` | **Buffers that describe a shared mesh** | 2 | `05-vertices` → `06-indices` | [05-vertices.md](../../consolidation/05-vertices.md) · [Redraft](../lessons/05-vertices.md) |
| `2.2` | `2.2` | **Camera state and the world-to-clip transform** | 3 | `07-uniforms` → `09-matrices` | [07-uniforms.md](../../consolidation/07-uniforms.md) · [Redraft](../lessons/07-uniforms.md) |
| `2.3` | `2.3` | **Depth, validated geometry and stable objects** | 4 | `10-depth` → `11a-identity` | [10-depth.md](../../consolidation/10-depth.md) · [Redraft](../lessons/10-depth.md) |
| `2.4` | `2.4` | **Selection that agrees with the displayed view** | 4 | `12-identity` → `13-picking` | [12-identity.md](../../consolidation/12-identity.md) · [Redraft](../lessons/12-identity.md) |
| `2.5` | `2.5` | **Document history and perspective rays** | 4 | `13a-history` → `14b-ray` | [13a-history.md](../../consolidation/13a-history.md) · [Redraft](../lessons/13a-history.md) |
| `2.6` | `2.6` | **Perspective picking and orbit navigation** | 2 | `15-perspective` → `16-orbit` | [15-perspective.md](../../consolidation/15-perspective.md) · [Redraft](../lessons/15-perspective.md) |

## 4. Rendering, Geometry, Interaction, and Runtime Features Covered

### Rendering & Shaders
- Vertex buffer layouts (`wgpu::VertexBufferLayout`, `array_stride`, `@location(0)`) and indexed drawing (`draw_indexed`, `IndexFormat::Uint32`)
- Uniform buffers (`wgpu::BufferUsages::UNIFORM | COPY_DST`), bind group layouts, and `@group(0) @binding(0) var<uniform>`
- Hardware depth attachment (`wgpu::TextureFormat::Depth32Float`, `CompareFunction::LessEqual`, `depth_write_enabled: true`)
- Selected vs unselected object color uniforms and per-object draw calls

### Geometry & Data Structures
- Indexed 3D triangle meshes, shared corner vertices, and topological validation (rejecting out-of-bounds indices and degenerate/non-finite triangles)
- Right-handed 3D coordinate transformations: `Local -> World -> View (Camera) -> Clip -> NDC -> Canvas Pixels`
- Perspective vs Orthographic projection matrices and 3D viewing rays (`origin + t * direction`)

### User Interaction & Selection
- Mouse click ray-casting against scene triangles (Möller–Trumbore ray-triangle intersection returning minimum positive `t`)
- Document Undo / Redo command history stacks (`Vec<SceneSnapshot>`)
- Interactive Orbit, Pan, and dolly Zoom camera navigation

### Browser Runtime & Production Authority (`session_viewer/src`)
- Stable object `Identity` / IDs across scene edits and `queue.write_buffer` per-frame uniform updates without reallocating GPU buffers
- Production authority: `src/camera.rs`, `src/app/walk/mesh.rs`, `src/engine/gpu/upload.rs`, `src/engine/pipelines/layouts.rs`

## 5. Practical Exercises & Expected Visible Results

| Checkpoint(s) | Practical Student Exercise | Expected Visible Result |
| :--- | :--- | :--- |
| `06-indices` | Move shared vertex 0 from `[-0.6, 0.0]` to `[-0.9, 0.0]`, then change `draw_indexed(0..6, ...)` to `0..3`. | Moving vertex 0 stretches both triangles together without a seam. Drawing `0..3` renders only the lower triangle. |
| `11a-identity / 13-picking` | Click the overlap region where the front and back meshes intersect. | The depth buffer draws the nearer surface in front, and the ray picker selects that exact same nearer object ID. |
| `16-orbit` | Drag to orbit the camera around the two meshes, click to select the rear mesh once uncovered, then run `Undo` and `Redo`. | Orbiting updates `view_proj` smoothly; clicking selects the newly visible mesh; `Undo` restores the prior selection state. |

## 6. Verified Work vs. Remaining Implementation or Validation

### Verified in Repository Evidence
- [x] All 6 consolidated lessons (`2.1`–`2.6`) and 19 checkpoints (`05-vertices`..`16-orbit`) are integrated and verified in `checkpoints.test.ts` and the strict Vite docs build.
- [x] Lockfile transitions (`15-perspective.lock`) and dependency checks pass.

### Remaining Implementation or Validation
- [ ] Chapter-specific accepted visual review image record (1 of 19 chapters awaiting fresh image inspection) and final 1,361-checkpoint release gate remain open.
