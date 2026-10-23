# 22.3 · Original boundary pens, joins and vertex markers

**Section 10: Shared GPU Arenas, Floating-Origin Precision, Multisampling, and Production Parity** · **Lesson 22.3** (Redraft `10.18`) · **4 checkpoints (`96zzzzzzn-respect-original-boundary-pens` → `96zzzzzzq-taper-crowded-original-mesh-vertices`)** · **Study time: 4–8 h**

> **What You Will Learn & Build:** Render original edge widths and colors, hide authored zero-width mesh edges, and omit coplanar seams without changing source face keys. By the final checkpoint (`96zzzzzzq-taper-crowded-original-mesh-vertices`), you will keep dense source vertices distinguishable while retaining explicit point sizes, editing controls and original ownership.

## 1. Visual WebGPU Explanation & Architecture (Black & White Schematics)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg width="840" height="256" viewBox="0 0 840 256" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrow-hollow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="4" y="4" width="832" height="248" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Section 10 Visual Explanation: Shared GPU Storage Arena, f64 Floating Origin, Reversed-Z, 4x MSAA & Matrix Shader</text>
  <text x="20" y="45" font-size="11" fill="#111111">Left: Growable storage arena + vertex pulling. Center: Camera-relative f64→f32 floating origin & Reversed-Z. Right: 4x MSAA + Matrix.</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    <rect x="18" y="64" width="255" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="145" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">A. Shared GPU Storage Arena</text>
    <rect x="34" y="100" width="222" height="32" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <rect x="34" y="100" width="65" height="32" fill="url(#hatch)" stroke="#111111" stroke-width="1.2"/>
    <rect x="145" y="100" width="55" height="32" fill="url(#hatch)" stroke="#111111" stroke-width="1.2"/>
    <text x="122" y="120" text-anchor="middle" font-size="9" font-weight="700" fill="#111111">Free Span</text>
    <text x="145" y="156" text-anchor="middle" font-size="9.5" font-weight="700" fill="#111111">Vertex Pulling (@builtin(vertex_index))</text>
    <text x="145" y="176" text-anchor="middle" font-size="9.5" fill="#111111">8-byte Draw Slots -&gt; ObjectRecord</text>
    <text x="145" y="194" text-anchor="middle" font-size="9.5" fill="#111111">slot_table.wgsl resolves owner IDs</text>

    <line x1="273" y1="154" x2="295" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="297" y="64" width="265" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="429" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">B. f64 Floating Origin &amp; Reversed-Z</text>
    <rect x="313" y="98" width="233" height="54" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
    <text x="429" y="116" text-anchor="middle" font-size="10" font-weight="700" fill="#111111">rel_f32 = (world_f64 - anchor_f64) as f32</text>
    <text x="429" y="134" text-anchor="middle" font-size="9.5" fill="#111111">Eliminates f32 vertex jitter at 10^12 m!</text>
    <rect x="313" y="162" width="233" height="64" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="429" y="180" text-anchor="middle" font-size="10" font-weight="700" fill="#111111">Reversed-Z Depth (GreaterEqual)</text>
    <text x="429" y="198" text-anchor="middle" font-size="9.5" fill="#111111">Near Plane = 1.0  →  Far Plane = 0.0</text>
    <text x="429" y="214" text-anchor="middle" font-size="9" fill="#111111">Preserves live camera across .pb reload</text>

    <line x1="562" y1="154" x2="584" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="586" y="64" width="236" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="704" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">C. 4x MSAA &amp; Matrix Shader</text>
    <rect x="602" y="98" width="204" height="128" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="704" y="118" text-anchor="middle" font-size="10" font-weight="700" fill="#111111">Coherent 1x / 4x Sample Targets</text>
    <text x="614" y="138" font-size="9.5" fill="#111111">• Color + Depth32Float at 4x MSAA</text>
    <text x="614" y="156" font-size="9.5" fill="#111111">• Rg32Uint Pick Target stays 1x</text>
    <text x="614" y="176" font-size="9.5" font-weight="700" fill="#111111">• matrix.wgsl procedural cells:</text>
    <text x="614" y="192" font-size="9" fill="#111111">  3-decimal digits, 0 blank, plate-less</text>
    <text x="614" y="207" font-size="9" fill="#111111">  world title &amp; yellow selected border</text>
    
</svg>
</div>

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg width="840" height="214" viewBox="0 0 840 214" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrow-hollow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="4" y="4" width="832" height="206" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Lesson 22.3 Pipeline Schematic: Original boundary pens, joins and vertex markers</text>
  <text x="20" y="45" font-size="11" fill="#111111">Step-by-step data flow you implement in this lesson (4 Checkpoints: 96zzzzzzn-respect-original-boundary-pens → 96zzzzzzq-taper-crowded-original-mesh-vertices)</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="89" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="90" x2="151" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="89" y="108" text-anchor="middle" font-size="11" fill="#111111">Original topology</text>
    <text x="89" y="124" text-anchor="middle" font-size="11" fill="#111111">and style order</text>
    <line x1="161" y1="108" x2="181" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="183" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="254" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="193" y1="90" x2="316" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="254" y="108" text-anchor="middle" font-size="11" fill="#111111">shared prepared</text>
    <text x="254" y="124" text-anchor="middle" font-size="11" fill="#111111">numeric pens</text>
    <line x1="326" y1="108" x2="346" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="348" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="358" y1="90" x2="481" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">existing boundary</text>
    <text x="419" y="124" text-anchor="middle" font-size="11" fill="#111111">stroke lane</text>
    <line x1="491" y1="108" x2="511" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="513" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="584" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="523" y1="90" x2="646" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="584" y="108" text-anchor="middle" font-size="11" fill="#111111">actual source edge</text>
    <text x="584" y="124" text-anchor="middle" font-size="11" fill="#111111">queries</text>
    <line x1="656" y1="108" x2="676" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="678" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="749" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 5</text>
    <line x1="688" y1="90" x2="811" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="749" y="108" text-anchor="middle" font-size="11" fill="#111111">Render bounds and</text>
    <text x="749" y="124" text-anchor="middle" font-size="11" fill="#111111">source vertex count</text>
    
    <rect x="18" y="158" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="177" font-size="11" font-weight="700" fill="#111111">Files Modified in Lesson 22.3:</text>
    <text x="235" y="177" font-size="11" fill="#111111">src/mesh_pens.rs  ·  src/lib.rs  ·  src/mesh_parts.rs  ·  src/bounds_original.rs  ·  src/pick_sources.rs  ·  src/source_pen_tests.rs</text>
    <text x="32" y="194" font-size="10.5" fill="#111111">4 Checkpoints: 96zzzzzzn-respect-original-boundary-pens → 96zzzzzzq-taper-crowded-original-mesh-vertices — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
</svg>
</div>

## 2. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

Before writing code, review how the three layers in this chapter talk to each other:

| Layer | Construct | Plain-English Explanation for Students |
| :--- | :--- | :--- |
| **WGSL (Vertex Pull)** | `@group(0) @binding(b) var<storage, read> arena_verts: array<u32>;` | Fetches packed vertex positions, normals, and colors directly from a single growable storage buffer using `@builtin(vertex_index)`. |
| **Rust/WGSL (`f64`->`f32`)** | `let rel: Vec3 = (world_f64 - anchor_f64).as_vec3();` | Subtracts the camera-area `f64` anchor on the CPU before converting to `f32`, preventing 32-bit float mantissa exhaustion far from `(0,0,0)`. |
| **WGSL (`matrix.wgsl`)** | 16-word header + 3 words/cell procedural digits | Renders matrix heatmaps and 3-decimal numeric digits procedurally inside the fragment shader with zero texture uploads per cell. |
| **Rust (`ink_ranges.rs`)** | Merged contiguous `Range<u32>` instance draws | Toggles point/line/edge/CAD-wire family visibility without rebuilding GPU buffers or invalidating original owner indices. |

**Authoritative production source (`session_viewer/src`):** [engine/gpu/render.rs](../../../../src/engine/gpu/render.rs), [engine/gpu/pass.rs](../../../../src/engine/gpu/pass.rs), [engine/gpu/targets.rs](../../../../src/engine/gpu/targets.rs), [engine/gpu/frame.rs](../../../../src/engine/gpu/frame.rs), [engine/gpu/view.rs](../../../../src/engine/gpu/view.rs), [engine/pipelines/mod.rs](../../../../src/engine/pipelines/mod.rs), [shaders/triangle.wgsl](../../../../src/shaders/triangle.wgsl), [shaders/ribbon.wgsl](../../../../src/shaders/ribbon.wgsl), [shaders/vector.wgsl](../../../../src/shaders/vector.wgsl), [shaders/ink_visibility.wgsl](../../../../src/shaders/ink_visibility.wgsl), [app/scene.rs](../../../../src/app/scene.rs), [build.rs](../../../../build.rs).

## 3. Checkpoint Roadmap (Jump to Any Notebook Step)

| Step | Checkpoint | What You Build | Files Modified | Pipeline Trace |
| ---: | :--- | :--- | :--- | :--- |
| **1** | [`96zzzzzzn-respect-original-boundary-pens`](#checkpoint-96zzzzzzn-respect-original-boundary-pens) | Render original edge widths and colors, hide authored zero-width mesh edges, and omit coplanar seams without changing source face keys. | `src/mesh_pens.rs`, `src/lib.rs`, `src/mesh_parts.rs`, `src/bounds_original.rs` | `Original topology and style order → shared prepared numeric pens → existing boundary stroke lane → actual source edge queries` |
| **2** | [`96zzzzzzo-close-original-boundary-joins`](#checkpoint-96zzzzzzo-close-original-boundary-joins) | Close wide original boundary corners and preserve actual source queries at round ends and flat near-plane cuts. | `src/stroke_quad.wgsl`, `src/finite_project_source.rs`, `src/stroke.wgsl`, `src/finite_stroke.wgsl` | `Original endpoints → shared near-clipped screen quad → distance coverage → matching integer query` |
| **3** | [`96zzzzzzp-draw-default-mesh-vertex-dots`](#checkpoint-96zzzzzzp-draw-default-mesh-vertex-dots) | Render the maintained default and authored mesh vertex dots without duplicating source meshes or endpoint positions. | `src/mesh_pens.rs`, `src/mesh_parts.rs`, `src/bounds_original.rs`, `src/plane_original.rs` | `Original nonzero edge incidence → endpoint address → anchored marker → existing point upload` |
| **4** | [`96zzzzzzq-taper-crowded-original-mesh-vertices`](#checkpoint-96zzzzzzq-taper-crowded-original-mesh-vertices) | Keep dense source vertices distinguishable while retaining explicit point sizes, editing controls and original ownership. | `src/mesh.rs`, `src/marker.rs`, `src/pick_sources.rs`, `src/stroke_gpu.rs` | `Render bounds and source vertex count → one shared spacing scalar → signed GPU size → common radius and query coverage` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Study the **Visual Step Trace**, read the **Concept Walkthrough** and **Self-Check Q&A**, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-96zzzzzzn-respect-original-boundary-pens"></a>

### Step 1 of 4 · Respect original boundary pens (`96zzzzzzn-respect-original-boundary-pens`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 1/4 Visual Trace (96zzzzzzn-respect-original-boundary-pens): Respect original boundary pens</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/mesh_pens.rs, src/lib.rs, src/mesh_parts.rs, src/bounds_original.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">Original topology and</text>
    <text x="111" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">style…</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">shared prepared numeric</text>
    <text x="316" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">pens</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">existing boundary stroke</text>
    <text x="521" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">lane</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">actual source edge queries</text>
    
</svg>
</div>

- **Goal:** Render original edge widths and colors, hide authored zero-width mesh edges, and omit coplanar seams without changing source face keys.
- **Data Flow Trace:** `Original topology and style order → shared prepared numeric pens → existing boundary stroke lane → actual source edge queries`
- **Files You Touch:** `src/mesh_pens.rs` (*Prepare only numeric pens using the already-taught topology and original encounter order.*) · `src/lib.rs` (*Connect the prepared style producer to the independent learner runtime.*) · `src/mesh_parts.rs` (*Keep lightweight pens beside the existing shared source-key display.*) · `src/bounds_original.rs` (*Existing analytic bounds edges retain their default black pen.*) · `src/pick_sources.rs` (*Keep source endpoint keys, boundary addresses and pen slots aligned.*) · `src/source_pen_tests.rs` (*Supply real GPU/source/archive, sparse style-order and source-unload controls.*) · `src/renderer.rs` (*Connect Native validation outside the browser runtime.*) · `src/plane_original.rs` (*Existing analytic plane boundaries retain their default black pen.*) · `src/mesh_arena_tests.rs` (*Keep the supplied stale-binding GPU probe inside its actual bound arrays; preserve all pixel, identity and write-count assertions.*)

A mesh with an authored seven-pixel pen currently looks identical to its one-pixel version. Prepare pens in the topology producer’s edge order, then feed the existing stroke lane. Use the same preparation to suppress coplanar seams while preserving source face identity.

A Mesh width of zero hides an edge; a single zero width denotes an undecorated printed fill. BRep and NURBS widths instead use the default pen for nonpositive values. Dense meshes above the maintained triangle budget also keep their faces without edge decoration. Store only numeric pens on the existing shared display so source unloading does not change appearance.

> **Self-Check Question:** *Why must edge colors and widths use topology encounter order instead of a sorted edge list?*  
> **Answer:** The source stores edge style streams in the order seeded while walking faces and their rings. Sorting endpoint pairs changes that order and assigns styles to different original edges. Preserve actual endpoint keys for source queries.

- **What You Should See:** The authored pen changes the actual GPU border. Sparse original keys and complete source records remain unchanged; coplanar seams and hidden mesh edges do not become visible ink.
- **Hands-On Lab Experiment:** Compare default and seven-pixel borders, a hidden sparse edge, original per-edge colors, a coplanar face pair and BRep/NURBS boundary pens. Check source unload, shared placement and exact restoration.

<details>
<summary>Implementation 96zzzzzzn-respect-original-boundary-pens: Respect original boundary pens</summary>

Render original edge widths and colors, hide authored zero-width mesh edges, and omit coplanar seams without changing source face keys.

Data flow: Original topology and style order → shared prepared numeric pens → existing boundary stroke lane → actual source edge queries.

### 1. `src/mesh_pens.rs`

Prepare only numeric pens using the already-taught topology and original encounter order.

Create the file and type:

```rust
--8<-- "journey/code/96zzzzzzn-respect-original-boundary-pens-01.rs"
```

### 2. `src/lib.rs`

Connect the prepared style producer to the independent learner runtime.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod mesh_parts;
```

</details>

After that block, type:

```rust
pub mod mesh_pens;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzn-respect-original-boundary-pens-02.rs"
```

</details>

### 3. `src/mesh_parts.rs`

Keep lightweight pens beside the existing shared source-key display. Reuse one topology preparation for styling and coplanar edge omission. Retain pens after the editable source is unloaded. Decode packed original colors only when preparing the existing stroke stream. Count actual retained numeric pen capacity.

<details>
<summary>Locate the existing block</summary>

```rust
#[derive(Clone,Default)]
pub struct Parts {
    pub faces: Vec<usize>,
    pub triangles: Vec<u32>,
    pub edges: Vec<(usize, usize)>,
    pub endpoints: Vec<[[f64; 3]; 2]>,
}

impl Parts {
    // Render triangles can split one source polygon; retain its original key rather than treating each display triangle as a new face.
    pub fn new(source: &session_rust::Mesh, count: usize) -> Result<Self, &'static str> {
        let faces = source.faces(); let mut triangles = Vec::new();
        for (slot, &face) in faces.iter().enumerate() {
            let slot = u32::try_from(slot).map_err(|_| "Too many source faces")?;
            let cached = source.get_triangulation().get(&face).filter(|tris| !tris.is_empty());
            let indices = if let Some(cached) = cached { cached.clone() } else {
                let vertices = source.face_vertices(face).ok_or("Missing source face")?;
                (1..vertices.len().saturating_sub(1)).map(|i| [vertices[0], vertices[i], vertices[i + 1]]).collect()
            };
            for tri in indices { if tri.iter().all(|&key| source.vertex_point(key).is_some()) { triangles.push(slot); } }
        }
        if triangles.len() != count { return Err("Source faces do not match the rendered triangles"); }
        let edges = source.edges(); let mut endpoints = Vec::new();
        for &(a, b) in &edges {
            let a = source.vertex_point(a).ok_or("Missing edge endpoint")?;
            let b = source.vertex_point(b).ok_or("Missing edge endpoint")?;
            endpoints.push([[a[0], a[1], a[2]], [b[0], b[1], b[2]]]);
        }
        Ok(Self { faces, triangles, edges, endpoints })
    }

    pub fn bytes(&self) -> usize {
        self.faces.capacity() * std::mem::size_of::<usize>() + self.triangles.capacity() * 4
            + self.edges.capacity() * std::mem::size_of::<(usize, usize)>() + self.endpoints.capacity() * 48
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
#[derive(Clone,Default)]
pub struct Parts {
    pub pens: Vec<crate::mesh_pens::Pen>,
    pub faces: Vec<usize>,
    pub triangles: Vec<u32>,
    pub edges: Vec<(usize, usize)>,
    pub endpoints: Vec<[[f64; 3]; 2]>,
}

impl Parts {
    // Render triangles can split one source polygon; retain its original key rather than treating each display triangle as a new face.
    pub fn new(source: &session_rust::Mesh, count: usize) -> Result<Self, &'static str> {
        let faces = source.faces(); let mut triangles = Vec::new();
        for (slot, &face) in faces.iter().enumerate() {
            let slot = u32::try_from(slot).map_err(|_| "Too many source faces")?;
            let cached = source.get_triangulation().get(&face).filter(|tris| !tris.is_empty());
            let indices = if let Some(cached) = cached { cached.clone() } else {
                let vertices = source.face_vertices(face).ok_or("Missing source face")?;
                (1..vertices.len().saturating_sub(1)).map(|i| [vertices[0], vertices[i], vertices[i + 1]]).collect()
            };
            for tri in indices { if tri.iter().all(|&key| source.vertex_point(key).is_some()) { triangles.push(slot); } }
        }
        if triangles.len() != count { return Err("Source faces do not match the rendered triangles"); }
        let (edges,pens)=crate::mesh_pens::prepare(source,count); let mut endpoints = Vec::new();
        for &(a, b) in &edges {
            let a = source.vertex_point(a).ok_or("Missing edge endpoint")?;
            let b = source.vertex_point(b).ok_or("Missing edge endpoint")?;
            endpoints.push([[a[0], a[1], a[2]], [b[0], b[1], b[2]]]);
        }
        Ok(Self { pens,faces, triangles, edges, endpoints })
    }

    pub fn pen(&self,index:usize)->(f32,[f32;4]) {
        let &(width,colour)=self.pens.get(if self.pens.len()==1 {0} else {index}).unwrap_or(&(1.,crate::source_topology_colour::BLACK));
        (width,colour.to_le_bytes().map(|v|v as f32/255.))
    }

    pub fn bytes(&self) -> usize {
        self.pens.capacity()*std::mem::size_of::<crate::mesh_pens::Pen>()+self.faces.capacity() * std::mem::size_of::<usize>() + self.triangles.capacity() * 4
            + self.edges.capacity() * std::mem::size_of::<(usize, usize)>() + self.endpoints.capacity() * 48
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzn-respect-original-boundary-pens-03.rs"
```

```text
--8<-- "journey/code/96zzzzzzn-respect-original-boundary-pens-04.rs"
```

```text
--8<-- "journey/code/96zzzzzzn-respect-original-boundary-pens-05.rs"
```

```text
--8<-- "journey/code/96zzzzzzn-respect-original-boundary-pens-06.rs"
```

```text
--8<-- "journey/code/96zzzzzzn-respect-original-boundary-pens-07.rs"
```

</details>

### 4. `src/bounds_original.rs`

Existing analytic bounds edges retain their default black pen.

<details>
<summary>Locate the existing block</summary>

```rust
        endpoints,
```

</details>

After that block, type:

```rust
--8<-- "journey/code/96zzzzzzn-respect-original-boundary-pens-08.rs" skip=19
```

### 5. `src/pick_sources.rs`

Keep source endpoint keys, boundary addresses and pen slots aligned. Respect each original geometry’s width semantics before emitting visible ink. Preserve source edge colors unless the existing display override wins. Use the existing CSS-pixel stroke path rather than another render lane.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            let flag=if planes.contains(&row.id){4096}else{0};
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
            let mut surface_edges=row.surface.as_ref().map(|surface|surface.edge_ids());
            for (&(a, b), ends) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints) {
                let part=cad_edges.as_mut().and_then(|edges|edges.next()).map(|edge|Part::BRepEdge(*edge)).unwrap_or_else(||surface_edges.as_mut().and_then(|edges|edges.next()).map_or(Part::MeshEdge(a,b),Part::SurfaceEdge));
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
                let ends=ends.map(|p|model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());
                sources.mesh_ink.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: scene.view.edges.get(&row.id).map_or([0.,0.,0.,1.],|rgb|[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]), width: 1. });
                sources.mesh_ids.push(id(row.id, ids[1]));
                sources.mesh_metadata.extend([flag,facing]);
            }
```

</details>

Replace it with:

```rust
            let flag=if planes.contains(&row.id){4096}else{0};
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
            let mut surface_edges=row.surface.as_ref().map(|surface|surface.edge_ids());
            for (index,(&(a,b),ends)) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints).enumerate() {
                let part=cad_edges.as_mut().and_then(|edges|edges.next()).map(|edge|Part::BRepEdge(*edge)).unwrap_or_else(||surface_edges.as_mut().and_then(|edges|edges.next()).map_or(Part::MeshEdge(a,b),Part::SurfaceEdge));
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
                let (width,colour)=row.mesh.parts.pen(index);
                let width=boundary_width.unwrap_or(width);
                if width==0. {continue;}
                let ends=ends.map(|p|model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());
                sources.mesh_ink.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: edge_colour.unwrap_or(colour), width });
                sources.mesh_ids.push(id(row.id, ids[1]));
                sources.mesh_metadata.extend([flag,facing]);
            }
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{scene::{ObjectId, Scene}, sub_id::{self, Kind}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part { Object, Face(usize), SurfaceEdge(usize), BRepEdge(usize), MeshEdge(usize, usize), PathEdge(usize), Curve, Control(usize), SourceControl(crate::source_controls::ControlId), CloudPoint(u32), SheetEntity(u32) }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Target { pub owner: ObjectId, pub part: Part }

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u32)]
pub enum Mode { #[default] Object, Face, Edge, Control }

#[derive(Default)]
pub struct Sources {
    pub sheet_bases:std::collections::HashMap<(ObjectId,usize),u32>,
    pub faces: Vec<Target>,
    pub edges: Vec<Target>,
    pub controls: Vec<Target>,
    pub mesh_bases: Vec<(ObjectId, u32)>,
    pub mesh_ink: Vec<crate::stroke::Stroke>,
    pub mesh_ids: Vec<[u32; 2]>,
    pub mesh_metadata:Vec<u32>,
    pub line_ids: Vec<[u32; 2]>,
    pub path_ids: Vec<[u32; 2]>,
    pub marker_ids: Vec<[u32; 2]>,
}

fn append(rows: &mut Vec<Target>, kind: Kind, target: Target) -> Result<[u32; 2], &'static str> {
    let tag = sub_id::encode(kind, rows.len())?; rows.push(target); Ok([target.owner.value(), tag])
}

impl Sources {
    // A source table translates compact GPU addresses to original keys; addresses belong to this scene snapshot, not the document's saved identity.
    pub fn new(scene:&Scene)->Result<Self,&'static str> {Self::anchored(scene,[0.;3])}
    pub fn anchored(scene:&Scene,anchor:[f64;3])->Result<Self,&'static str> {
        let mut sources = Self::default();
        let id = |owner: ObjectId, tag| [crate::feature_owner::display_owner(scene, owner).map_or(0, ObjectId::value),
            if scene.forest.derived(owner) { 0 } else { tag }];
        let planes=crate::clipping_rows::owners(scene);
        for row in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let model=crate::render_space::placed(&row.model,anchor);
            sources.mesh_bases.push((row.id, u32::try_from(sources.faces.len()).map_err(|_| "Too many source faces")?));
            if !scene.forest.derived(row.id) {
                for &key in &row.mesh.parts.faces {
                    append(&mut sources.faces,Kind::Face,Target{owner:row.id,part:Part::Face(key)})?;
                }
            }
            let mut facing=crate::packed_vertex::NO_NORMAL;
            if row.mesh.alphas().first().is_some_and(|a|(*a-1./255.).abs()<0.5/255.) {
                let n=row.mesh.normals()[0];let m=crate::normals::matrix(&row.model)?;
                facing=crate::packed_vertex::oct32(std::array::from_fn(|i|(0..3).map(|j|m[j*4+i]*n[j]).sum()));
            }
            let flag=if planes.contains(&row.id){4096}else{0};
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
            let mut surface_edges=row.surface.as_ref().map(|surface|surface.edge_ids());
            for (index,(&(a,b),ends)) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints).enumerate() {
                let part=cad_edges.as_mut().and_then(|edges|edges.next()).map(|edge|Part::BRepEdge(*edge)).unwrap_or_else(||surface_edges.as_mut().and_then(|edges|edges.next()).map_or(Part::MeshEdge(a,b),Part::SurfaceEdge));
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
                let (width,colour)=row.mesh.parts.pen(index);
                let width=boundary_width.unwrap_or(width);
                if width==0. {continue;}
                let ends=ends.map(|p|model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());
                sources.mesh_ink.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: edge_colour.unwrap_or(colour), width });
                sources.mesh_ids.push(id(row.id, ids[1]));
                sources.mesh_metadata.extend([flag,facing]);
            }
        }
        for row in scene.lines().iter().filter(|row| scene.visible(row.id)) {
            let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part: Part::Curve })?;
            sources.line_ids.push(id(row.id, ids[1]));
        }
        for row in scene.paths().iter().filter(|row| scene.visible(row.id)) {
            let points:Vec<_>=row.points_at(anchor).iter().map(session_rust::Point::to_f32).collect();
            for (index, _) in points.windows(2).enumerate().filter(|(_, pair)| pair[0] != pair[1]) {
                let part = if matches!(row.prepared.source, crate::chain::Source::Polyline(_)) { Part::PathEdge(index) } else { Part::Curve };
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
                sources.path_ids.push(id(row.id, ids[1]));
            }
        }
        for row in scene.points().iter().filter(|row| scene.visible(row.id)) { sources.marker_ids.push(id(row.id, 1)); }
        for (owner, stroke) in scene.control_nets_at(anchor) {
            sources.mesh_ink.push(stroke);
            sources.mesh_ids.push([owner.value(), 0]);
            sources.mesh_metadata.extend([0,crate::packed_vertex::NO_NORMAL]);
        }
        for row in scene.control_markers_at(anchor) {
            let ids = append(&mut sources.controls, Kind::Control, Target { owner: row.parent, part: row.part })?;
            sources.marker_ids.push(id(row.parent, ids[1]));
        }
        for row in scene.sheets().iter().filter(|row|scene.visible(row.id)) {for page in row.prepared.pages() {sources.sheet_bases.insert((row.id,page.key()),u32::try_from(sources.edges.len()).map_err(|_|"Too many sheet pick addresses")?);for entity in &page.entities {append(&mut sources.edges,Kind::Edge,Target {owner:row.id,part:if *entity==u32::MAX {Part::Object} else {Part::SheetEntity(*entity)}})?;}}}
        Ok(sources)
    }

    pub fn bytes(&self) -> usize {
        (self.faces.capacity() + self.edges.capacity() + self.controls.capacity()) * std::mem::size_of::<Target>()
            + self.mesh_bases.capacity() * std::mem::size_of::<(ObjectId, u32)>()
            + self.mesh_metadata.capacity()*4
            + self.mesh_ink.capacity() * std::mem::size_of::<crate::stroke::Stroke>()
            + (self.mesh_ids.capacity() + self.line_ids.capacity() + self.path_ids.capacity() + self.marker_ids.capacity()) * 8
    }

    pub fn resolve(&self, scene: &Scene, pixel: [u32; 2], mode: Mode) -> Option<Target> {
        let owner = scene.pick_id(pixel[0])?;
        if mode == Mode::Object { return Some(Target { owner, part: Part::Object }); }
        let (kind, address) = sub_id::decode(pixel[1])?;
        if mode==Mode::Control && kind==Kind::Control && (scene.clouds().iter().any(|r|r.id==owner)||scene.streams().iter().any(|r|r.id==owner)) {
            return crate::cloud_pick::resolve(scene,owner,address);
        }
        let target = match (mode, kind) {
            (Mode::Face, Kind::Face) => self.faces.get(address),
            (Mode::Edge, Kind::Edge) => self.edges.get(address),
            (Mode::Control, Kind::Control) => self.controls.get(address), _ => None,
        }?;
        (target.owner == owner && target.valid(scene)).then_some(*target)
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzn-respect-original-boundary-pens-09.rs"
```

```text
--8<-- "journey/code/96zzzzzzn-respect-original-boundary-pens-10.rs"
```

```text
--8<-- "journey/code/96zzzzzzn-respect-original-boundary-pens-11.rs"
```

```text
--8<-- "journey/code/96zzzzzzn-respect-original-boundary-pens-12.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/source_pen_tests.rs`

Supply real GPU/source/archive, sparse style-order and source-unload controls.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzn-respect-original-boundary-pens-13.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/renderer.rs`

Connect Native validation outside the browser runtime.

<details>
<summary>Locate the existing block</summary>

```text
mod source_face_policy_tests;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzn-respect-original-boundary-pens-14.rs" skip=29
```

</details>

### 8. `src/plane_original.rs`

Existing analytic plane boundaries retain their default black pen.

<details>
<summary>Locate the existing block</summary>

```rust
    let endpoints=edges.iter().map(|&(a,b)|[&points[a],&points[b]].map(|p|[p[0],p[1],p[2]])).collect();display.parts=crate::mesh_parts::Parts{faces:Vec::new(),triangles:Vec::new(),edges,endpoints};
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    let endpoints=edges.iter().map(|&(a,b)|[&points[a],&points[b]].map(|p|[p[0],p[1],p[2]])).collect();display.parts=crate::mesh_parts::Parts{pens:Vec::new(),faces:Vec::new(),triangles:Vec::new(),edges,endpoints};
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzn-respect-original-boundary-pens-15.rs"
```

</details>

### 9. `src/pick_sources.rs`

Resolve immutable mesh-row ownership and explicit display color once. Share the existing source-face eligibility classification. Resolve the original surface/BRep scalar pen once per row, outside its boundary segments. Retain identical original pick addresses without repeated per-segment hierarchy lookups.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
                if width==0. {continue;}
                let ends=ends.map(|p|model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());
                sources.mesh_ink.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: edge_colour.unwrap_or(colour), width });
                sources.mesh_ids.push(id(row.id, ids[1]));
                sources.mesh_metadata.extend([flag,facing]);
            }
        }
```

</details>

Replace it with:

```rust
                if width==0. {continue;}
                let ends=ends.map(|p|model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());
                sources.mesh_ink.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: edge_colour.unwrap_or(colour), width });
                sources.mesh_ids.push([owner,if derived{0}else{ids[1]}]);
                sources.mesh_metadata.extend([flag,facing]);
            }
        }
```

<details>
<summary>Locate this block</summary>

```rust
                facing=crate::packed_vertex::oct32(std::array::from_fn(|i|(0..3).map(|j|m[j*4+i]*n[j]).sum()));
            }
            let flag=if planes.contains(&row.id){4096}else{0};
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
            let mut surface_edges=row.surface.as_ref().map(|surface|surface.edge_ids());
            for (index,(&(a,b),ends)) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints).enumerate() {
```

</details>

Replace it with:

```rust
                facing=crate::packed_vertex::oct32(std::array::from_fn(|i|(0..3).map(|j|m[j*4+i]*n[j]).sum()));
            }
            let flag=if planes.contains(&row.id){4096}else{0};
            let boundary_width=row.cad.as_ref().map(|c|c.source.width).or_else(||row.surface.as_ref().map(|s|s.source.width)).map(crate::mesh_pens::width);
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
            let mut surface_edges=row.surface.as_ref().map(|surface|surface.edge_ids());
            for (index,(&(a,b),ends)) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints).enumerate() {
```

<details>
<summary>Locate this block</summary>

```rust
            if scene.forest.derived(owner) { 0 } else { tag }];
        let planes=crate::clipping_rows::owners(scene);
        for row in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let model=crate::render_space::placed(&row.model,anchor);
            sources.mesh_bases.push((row.id, u32::try_from(sources.faces.len()).map_err(|_| "Too many source faces")?));
            if !scene.forest.derived(row.id) {
                for &key in &row.mesh.parts.faces {
                    append(&mut sources.faces,Kind::Face,Target{owner:row.id,part:Part::Face(key)})?;
                }
```

</details>

Replace it with:

```rust
            if scene.forest.derived(owner) { 0 } else { tag }];
        let planes=crate::clipping_rows::owners(scene);
        for row in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let derived=scene.forest.derived(row.id);
            let owner=crate::feature_owner::display_owner(scene,row.id).map_or(0,ObjectId::value);
            let edge_colour=scene.view.edges.get(&row.id).map(|rgb|[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]);
            let model=crate::render_space::placed(&row.model,anchor);
            sources.mesh_bases.push((row.id, u32::try_from(sources.faces.len()).map_err(|_| "Too many source faces")?));
            if !derived {
                for &key in &row.mesh.parts.faces {
                    append(&mut sources.faces,Kind::Face,Target{owner:row.id,part:Part::Face(key)})?;
                }
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{scene::{ObjectId, Scene}, sub_id::{self, Kind}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part { Object, Face(usize), SurfaceEdge(usize), BRepEdge(usize), MeshEdge(usize, usize), PathEdge(usize), Curve, Control(usize), SourceControl(crate::source_controls::ControlId), CloudPoint(u32), SheetEntity(u32) }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Target { pub owner: ObjectId, pub part: Part }

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u32)]
pub enum Mode { #[default] Object, Face, Edge, Control }

#[derive(Default)]
pub struct Sources {
    pub sheet_bases:std::collections::HashMap<(ObjectId,usize),u32>,
    pub faces: Vec<Target>,
    pub edges: Vec<Target>,
    pub controls: Vec<Target>,
    pub mesh_bases: Vec<(ObjectId, u32)>,
    pub mesh_ink: Vec<crate::stroke::Stroke>,
    pub mesh_ids: Vec<[u32; 2]>,
    pub mesh_metadata:Vec<u32>,
    pub line_ids: Vec<[u32; 2]>,
    pub path_ids: Vec<[u32; 2]>,
    pub marker_ids: Vec<[u32; 2]>,
}

fn append(rows: &mut Vec<Target>, kind: Kind, target: Target) -> Result<[u32; 2], &'static str> {
    let tag = sub_id::encode(kind, rows.len())?; rows.push(target); Ok([target.owner.value(), tag])
}

impl Sources {
    // A source table translates compact GPU addresses to original keys; addresses belong to this scene snapshot, not the document's saved identity.
    pub fn new(scene:&Scene)->Result<Self,&'static str> {Self::anchored(scene,[0.;3])}
    pub fn anchored(scene:&Scene,anchor:[f64;3])->Result<Self,&'static str> {
        let mut sources = Self::default();
        let id = |owner: ObjectId, tag| [crate::feature_owner::display_owner(scene, owner).map_or(0, ObjectId::value),
            if scene.forest.derived(owner) { 0 } else { tag }];
        let planes=crate::clipping_rows::owners(scene);
        for row in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let derived=scene.forest.derived(row.id);
            let owner=crate::feature_owner::display_owner(scene,row.id).map_or(0,ObjectId::value);
            let edge_colour=scene.view.edges.get(&row.id).map(|rgb|[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]);
            let model=crate::render_space::placed(&row.model,anchor);
            sources.mesh_bases.push((row.id, u32::try_from(sources.faces.len()).map_err(|_| "Too many source faces")?));
            if !derived {
                for &key in &row.mesh.parts.faces {
                    append(&mut sources.faces,Kind::Face,Target{owner:row.id,part:Part::Face(key)})?;
                }
            }
            let mut facing=crate::packed_vertex::NO_NORMAL;
            if row.mesh.alphas().first().is_some_and(|a|(*a-1./255.).abs()<0.5/255.) {
                let n=row.mesh.normals()[0];let m=crate::normals::matrix(&row.model)?;
                facing=crate::packed_vertex::oct32(std::array::from_fn(|i|(0..3).map(|j|m[j*4+i]*n[j]).sum()));
            }
            let flag=if planes.contains(&row.id){4096}else{0};
            let boundary_width=row.cad.as_ref().map(|c|c.source.width).or_else(||row.surface.as_ref().map(|s|s.source.width)).map(crate::mesh_pens::width);
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
            let mut surface_edges=row.surface.as_ref().map(|surface|surface.edge_ids());
            for (index,(&(a,b),ends)) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints).enumerate() {
                let part=cad_edges.as_mut().and_then(|edges|edges.next()).map(|edge|Part::BRepEdge(*edge)).unwrap_or_else(||surface_edges.as_mut().and_then(|edges|edges.next()).map_or(Part::MeshEdge(a,b),Part::SurfaceEdge));
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
                let (width,colour)=row.mesh.parts.pen(index);
                let width=boundary_width.unwrap_or(width);
                if width==0. {continue;}
                let ends=ends.map(|p|model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());
                sources.mesh_ink.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: edge_colour.unwrap_or(colour), width });
                sources.mesh_ids.push([owner,if derived{0}else{ids[1]}]);
                sources.mesh_metadata.extend([flag,facing]);
            }
        }
        for row in scene.lines().iter().filter(|row| scene.visible(row.id)) {
            let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part: Part::Curve })?;
            sources.line_ids.push(id(row.id, ids[1]));
        }
        for row in scene.paths().iter().filter(|row| scene.visible(row.id)) {
            let points:Vec<_>=row.points_at(anchor).iter().map(session_rust::Point::to_f32).collect();
            for (index, _) in points.windows(2).enumerate().filter(|(_, pair)| pair[0] != pair[1]) {
                let part = if matches!(row.prepared.source, crate::chain::Source::Polyline(_)) { Part::PathEdge(index) } else { Part::Curve };
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
                sources.path_ids.push(id(row.id, ids[1]));
            }
        }
        for row in scene.points().iter().filter(|row| scene.visible(row.id)) { sources.marker_ids.push(id(row.id, 1)); }
        for (owner, stroke) in scene.control_nets_at(anchor) {
            sources.mesh_ink.push(stroke);
            sources.mesh_ids.push([owner.value(), 0]);
            sources.mesh_metadata.extend([0,crate::packed_vertex::NO_NORMAL]);
        }
        for row in scene.control_markers_at(anchor) {
            let ids = append(&mut sources.controls, Kind::Control, Target { owner: row.parent, part: row.part })?;
            sources.marker_ids.push(id(row.parent, ids[1]));
        }
        for row in scene.sheets().iter().filter(|row|scene.visible(row.id)) {for page in row.prepared.pages() {sources.sheet_bases.insert((row.id,page.key()),u32::try_from(sources.edges.len()).map_err(|_|"Too many sheet pick addresses")?);for entity in &page.entities {append(&mut sources.edges,Kind::Edge,Target {owner:row.id,part:if *entity==u32::MAX {Part::Object} else {Part::SheetEntity(*entity)}})?;}}}
        Ok(sources)
    }

    pub fn bytes(&self) -> usize {
        (self.faces.capacity() + self.edges.capacity() + self.controls.capacity()) * std::mem::size_of::<Target>()
            + self.mesh_bases.capacity() * std::mem::size_of::<(ObjectId, u32)>()
            + self.mesh_metadata.capacity()*4
            + self.mesh_ink.capacity() * std::mem::size_of::<crate::stroke::Stroke>()
            + (self.mesh_ids.capacity() + self.line_ids.capacity() + self.path_ids.capacity() + self.marker_ids.capacity()) * 8
    }

    pub fn resolve(&self, scene: &Scene, pixel: [u32; 2], mode: Mode) -> Option<Target> {
        let owner = scene.pick_id(pixel[0])?;
        if mode == Mode::Object { return Some(Target { owner, part: Part::Object }); }
        let (kind, address) = sub_id::decode(pixel[1])?;
        if mode==Mode::Control && kind==Kind::Control && (scene.clouds().iter().any(|r|r.id==owner)||scene.streams().iter().any(|r|r.id==owner)) {
            return crate::cloud_pick::resolve(scene,owner,address);
        }
        let target = match (mode, kind) {
            (Mode::Face, Kind::Face) => self.faces.get(address),
            (Mode::Edge, Kind::Edge) => self.edges.get(address),
            (Mode::Control, Kind::Control) => self.controls.get(address), _ => None,
        }?;
        (target.owner == owner && target.valid(scene)).then_some(*target)
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzn-respect-original-boundary-pens-16.rs"
```

```text
--8<-- "journey/code/96zzzzzzn-respect-original-boundary-pens-17.rs"
```

```text
--8<-- "journey/code/96zzzzzzn-respect-original-boundary-pens-18.rs"
```

```text
--8<-- "journey/code/96zzzzzzn-respect-original-boundary-pens-19.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 10. `src/mesh_arena_tests.rs`

Keep the supplied stale-binding GPU probe inside its actual bound arrays; preserve all pixel, identity and write-count assertions.

<details>
<summary>Locate the existing block</summary>

```text
  let v=vertices[indices[occurrence]];return Out(vec4<f32>(packed_position(v),1.),unpack4x8unorm(v.color));
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzn-respect-original-boundary-pens-20.rs"
```

</details>

</details>

<a id="checkpoint-96zzzzzzo-close-original-boundary-joins"></a>

### Step 2 of 4 · Close original boundary joins (`96zzzzzzo-close-original-boundary-joins`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 2/4 Visual Trace (96zzzzzzo-close-original-boundary-joins): Close original boundary joins</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/stroke_quad.wgsl, src/finite_project_source.rs, src/stroke.wgsl, src/finite_stroke.wgsl</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">Original endpoints</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">shared near-clipped screen</text>
    <text x="316" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">quad</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">distance coverage</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">matching integer query</text>
    
</svg>
</div>

- **Goal:** Close wide original boundary corners and preserve actual source queries at round ends and flat near-plane cuts.
- **Data Flow Trace:** `Original endpoints → shared near-clipped screen quad → distance coverage → matching integer query`
- **Files You Touch:** `src/stroke_quad.wgsl` (*Share near-clipped round-stroke projection without new records or buffers.*) · `src/finite_project_source.rs` (*Expand the one shared projection helper in both consumers.*) · `src/stroke.wgsl` (*Use one shared stroke shape for ordinary and finite rendering.*) · `src/finite_stroke.wgsl` (*Use one shared stroke shape for ordinary and finite rendering.*) · `src/stroke_coverage.wgsl` (*Measure distance to the finite segment; keep near cuts flat.*) · `src/stroke_pick.wgsl` (*Keep color and integer queries on the same end policy.*) · `src/finite_stroke_pick.wgsl` (*Keep color and integer queries on the same end policy.*) · `src/round_caps_tests.rs` (*Copy actual GPU validation fixtures.*) · `src/renderer.rs` (*Register Native validation only.*) · `src/stroke_gpu.rs` (*Exercise the actual existing legacy pipeline in Native fixtures.*) · `src/cap_gpu_tests.rs` (*Sample the unoccluded original back face, away from the now-visible end-on construction-plane marker. Keep the zero section-fill assertion.*) · `src/finite_visibility_tests.rs` (*Retain exact clipped headed-chain comparison; distinguish genuine round source caps with exact body, flat-cut and bounded-cap pixel assertions.*)

A wide source border leaves a white gap outside each corner. Expand the existing stroke quad along its length, then measure fragment distance to the finite segment. Share this footprint with the existing query shaders.

Original endpoints receive round caps; a camera near-plane cut stays flat. A nonzero line viewed end-on uses its nearest reverse-depth endpoint. Identical source endpoints remain hidden. Both existing finite and legacy lanes consume the same helper without new geometry owners, buffers or metadata.

> **Self-Check Question:** *Why must a near-plane cut remain flat even when original line endpoints are round?*  
> **Answer:** Clipping creates a camera boundary rather than another original endpoint. A round cap there would invent ink beyond the visible source segment and allow queries on that invented shape.

- **What You Should See:** Original borders close at wide corners; end-on original lines remain visible and queryable. Near cuts stay flat, complete source records remain unchanged, and shared geometry survives placement and source unload.
- **Hands-On Lab Experiment:** Inspect actual original Mesh, NURBS and BRep corners, free ends, end-on lines, near cuts and a shared mirrored pair at both densities and projections. Compare integer source queries with the rendered footprint.

<details>
<summary>Implementation 96zzzzzzo-close-original-boundary-joins: Close original boundary joins</summary>

Close wide original boundary corners and preserve actual source queries at round ends and flat near-plane cuts.

Data flow: Original endpoints → shared near-clipped screen quad → distance coverage → matching integer query.

### 1. `src/stroke_quad.wgsl`

Share near-clipped round-stroke projection without new records or buffers.

Create the file and type:

```wgsl
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-01.rs"
```

### 2. `src/finite_project_source.rs`

Expand the one shared projection helper in both consumers.

<details>
<summary>Locate the existing block</summary>

```rust
        .replace("#include \"stroke_coverage.wgsl\"", include_str!("stroke_coverage.wgsl"))
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-02.rs"
```

### 3. `src/stroke.wgsl`

Use one shared stroke shape for ordinary and finite rendering. Distinguish genuine round ends from flat near-plane cuts. Carry only screen coordinates needed by existing fragment coverage. Replace duplicate projection with the shared clipped capsule.

<details>
<summary>Locate the existing block</summary>

```wgsl
#include "near_clip.wgsl"
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32 }
@group(0) @binding(0) var<uniform> view: View;

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) side: f32,
    @location(2) @interpolate(flat) half_width: f32,
}

fn hidden(colour: vec4<f32>) -> Output {
    return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, 0.0, 0.0);
}

fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, index: u32) -> Output {
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    if (near_distance(a) < 0.0 && near_distance(b) < 0.0) { return hidden(colour); }
    a=near_endpoint(a,b);
    b=near_endpoint(b,a);
    if (a.w <= 0.0 || b.w <= 0.0) { return hidden(colour); }
    let delta = (b.xy / b.w - a.xy / a.w) * view.size;
    let length2 = dot(delta, delta);
    if (length2 < 0.000001) { return hidden(colour); }
    let normal = vec2<f32>(-delta.y, delta.x) * inverseSqrt(length2);
    let corners = array<vec2<f32>, 6>(vec2<f32>(0.0, -1.0), vec2<f32>(1.0, -1.0),
        vec2<f32>(0.0, 1.0), vec2<f32>(0.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let corner = corners[index];
    let half_width = width * view.density * 0.5;
    let extent = half_width + 0.70711;
    var position = select(a, b, corner.x > 0.5);
    position = vec4<f32>(position.xy + normal * extent * 2.0 / view.size * corner.y * position.w, position.zw);
    return Output(position, colour, corner.y * extent, half_width);
}

#include "stroke_coverage.wgsl"

@vertex
fn vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32) -> Output { return geometry(start, end, colour, width, index); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> { return colour(input); }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
#include "near_clip.wgsl"
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32 }
@group(0) @binding(0) var<uniform> view: View;
#include "stroke_quad.wgsl"

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(flat) flat_ends:u32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(9) @interpolate(flat) finite_axis_value:vec4f,
    @location(11) @interpolate(linear) finite_pixel:vec2f,
}

fn geometry(start:vec3f,end:vec3f,colour:vec4f,width:f32,index:u32)->Output {
    let s=stroke_quad(view.transform*vec4f(start,1.),view.transform*vec4f(end,1.),width,index);
    return Output(s.position,colour,s.flat_ends,s.radius,s.axis,s.pixel);
}

#include "stroke_coverage.wgsl"

@vertex
fn vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32) -> Output { return geometry(start, end, colour, width, index); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> { return colour(input); }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-03.rs"
```

```text
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-04.rs"
```

```text
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-05.rs"
```

```text
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-06.rs"
```

</details>

### 4. `src/finite_stroke.wgsl`

Use one shared stroke shape for ordinary and finite rendering. Distinguish genuine round ends from flat near-plane cuts. Let the shared projection own hidden, near-clipped and end-on cases. Retain contact rise and reuse the shared screen-space shape.

<details>
<summary>Locate the existing block</summary>

```wgsl
#include "near_clip.wgsl"
#include "contact_depth.wgsl"
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32, display:vec4<f32>, eye:vec4<f32> }
@group(0) @binding(0) var<uniform> view: View;

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) side: f32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
    @location(12) @interpolate(flat) rise:f32,
}

fn hidden(colour: vec4<f32>,flags:u32) -> Output {
    return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, 0.0, 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0),vec2<f32>(0.0),0.);
}

fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, index: u32,metadata:InkRecord) -> Output {
    let words=ink_words(metadata);let flags=words.x;
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    let z=vec2<f32>(a.z,b.z);let n=oct32_decode(words.y);
    a=contact_layer(a,start,n,view.transform,view.size,view.eye);
    b=contact_layer(b,end,n,view.transform,view.size,view.eye);
    let rise=max(0.,max((a.z-z.x)/max(a.w,1e-20),(b.z-z.y)/max(b.w,1e-20)));
    if (near_distance(a) < 0.0 && near_distance(b) < 0.0) { return hidden(colour,flags); }
    a=near_endpoint(a,b);
    b=near_endpoint(b,a);
    if (a.w <= 0.0 || b.w <= 0.0) { return hidden(colour,flags); }
    let delta = (b.xy / b.w - a.xy / a.w) * view.size;
    let length2 = dot(delta, delta);
    if (length2 < 0.000001) { return hidden(colour,flags); }
    let normal = vec2<f32>(-delta.y, delta.x) * inverseSqrt(length2);
    let corners = array<vec2<f32>, 6>(vec2<f32>(0.0, -1.0), vec2<f32>(1.0, -1.0),
        vec2<f32>(0.0, 1.0), vec2<f32>(0.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let corner = corners[index];
    let half_width = width * view.density * 0.5;
    let extent = half_width + 0.70711;
    var position = select(a, b, corner.x > 0.5);
    position = vec4<f32>(position.xy + normal * extent * 2.0 / view.size * corner.y * position.w, position.zw);
    return Output(position, colour, corner.y * extent, half_width,flags,finite_axis(a,b),vec2<f32>(a.z/a.w,b.z/b.w),finite_screen(position),rise);
}

#include "stroke_coverage.wgsl"

@vertex
fn vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32,@location(8) metadata:InkRecord) -> Output { return geometry(start, end, colour, width, index,metadata); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,false,vec2<f32>(input.rise,0.));if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
#include "finite_visibility.wgsl"
fn finite_depth_pixel(pixel:vec2<f32>)->vec2<i32>{return vec2<i32>(pixel);}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
#include "near_clip.wgsl"
#include "contact_depth.wgsl"
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32, display:vec4<f32>, eye:vec4<f32> }
@group(0) @binding(0) var<uniform> view: View;
#include "stroke_quad.wgsl"

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(flat) flat_ends:u32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
    @location(12) @interpolate(flat) rise:f32,
}

fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, index: u32,metadata:InkRecord) -> Output {
    let words=ink_words(metadata);let flags=words.x;
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    let z=vec2<f32>(a.z,b.z);let n=oct32_decode(words.y);
    a=contact_layer(a,start,n,view.transform,view.size,view.eye);
    b=contact_layer(b,end,n,view.transform,view.size,view.eye);
    let rise=max(0.,max((a.z-z.x)/max(a.w,1e-20),(b.z-z.y)/max(b.w,1e-20)));
    let s=stroke_quad(a,b,width,index);
    return Output(s.position,colour,s.flat_ends,s.radius,flags,s.axis,s.depths,s.pixel,rise);
}

#include "stroke_coverage.wgsl"

@vertex
fn vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32,@location(8) metadata:InkRecord) -> Output { return geometry(start, end, colour, width, index,metadata); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,false,vec2<f32>(input.rise,0.));if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
#include "finite_visibility.wgsl"
fn finite_depth_pixel(pixel:vec2<f32>)->vec2<i32>{return vec2<i32>(pixel);}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-07.rs"
```

```text
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-08.rs"
```

```text
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-10.rs"
```

</details>

### 5. `src/stroke_coverage.wgsl`

Measure distance to the finite segment; keep near cuts flat.

<details>
<summary>Locate the existing block</summary>

```wgsl
fn colour(input: Output) -> vec4<f32> {
    let coverage = clamp(input.half_width + 0.5 - abs(input.side), 0.0, 1.0);
    return vec4<f32>(input.colour.rgb, input.colour.a * coverage);
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-11.rs"
```

### 6. `src/stroke_pick.wgsl`

Keep color and integer queries on the same end policy. Carry the actual clipping-end policy into the cropped request. Reuse the exact color coverage in integer picking. Use full-viewport pixel coordinates for cropped coverage. Cropping changes clip position, not the original screen-space footprint. Pass the shared footprint to the ordinary pick fragment.

<details>
<summary>Locate the existing block</summary>

```wgsl
@group(1) @binding(0) var<uniform> crop: Crop;
struct PickOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) side: f32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
}
@vertex
fn pick_vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>) -> PickOutput {
    let source = geometry(start, end, colour, width, index);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.side = source.side;
    output.half_width = source.half_width;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.side, input.half_width);
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(source).a <= 0.0) { discard; }
    return input.id;
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(1) @binding(0) var<uniform> crop: Crop;
struct PickOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(flat) flat_ends:u32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(9) @interpolate(flat) finite_axis_value:vec4f,
    @location(11) @interpolate(linear) finite_pixel:vec2f,
    @location(7) @interpolate(flat) id: vec2<u32>,
}
@vertex
fn pick_vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>) -> PickOutput {
    let source = geometry(start, end, colour, width, index);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.flat_ends = source.flat_ends;
    output.half_width = source.half_width;
    output.finite_axis_value=source.finite_axis_value;output.finite_pixel=source.finite_pixel;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.flat_ends, input.half_width,input.finite_axis_value,input.finite_pixel);
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(source).a <= 0.0) { discard; }
    return input.id;
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-12.rs"
```

```text
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-13.rs"
```

```text
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-14.rs"
```

```text
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-15.rs"
```

```text
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-16.rs"
```

```text
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-17.rs"
```

</details>

### 7. `src/finite_stroke_pick.wgsl`

Keep color and integer queries on the same end policy. Carry the actual clipping-end policy into the cropped request. Reuse the exact color coverage in integer picking.

<details>
<summary>Locate the existing block</summary>

```wgsl
@group(1) @binding(0) var<uniform> crop: Crop;
struct PickOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) side: f32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
    @location(12) @interpolate(flat) rise:f32,
}
@vertex
fn pick_vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>,@location(8) metadata:InkRecord) -> PickOutput {
    let source = geometry(start, end, colour, width, index,metadata);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.side = source.side;
    output.half_width = source.half_width;
    output.clip_flags=source.clip_flags;
    output.finite_axis_value=source.finite_axis_value;output.finite_depths=source.finite_depths;output.finite_pixel=source.finite_pixel;
    output.rise=source.rise;output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.side, input.half_width,input.clip_flags,input.finite_axis_value,input.finite_depths,input.finite_pixel,input.rise);
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(source).a <= 0.0) { discard; }
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,true,vec2<f32>(input.rise,0.))<=0.){discard;}
    return input.id;
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(1) @binding(0) var<uniform> crop: Crop;
struct PickOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(flat) flat_ends:u32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
    @location(12) @interpolate(flat) rise:f32,
}
@vertex
fn pick_vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>,@location(8) metadata:InkRecord) -> PickOutput {
    let source = geometry(start, end, colour, width, index,metadata);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.flat_ends = source.flat_ends;
    output.half_width = source.half_width;
    output.clip_flags=source.clip_flags;
    output.finite_axis_value=source.finite_axis_value;output.finite_depths=source.finite_depths;output.finite_pixel=source.finite_pixel;
    output.rise=source.rise;output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.flat_ends, input.half_width,input.clip_flags,input.finite_axis_value,input.finite_depths,input.finite_pixel,input.rise);
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(source).a <= 0.0) { discard; }
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,true,vec2<f32>(input.rise,0.))<=0.){discard;}
    return input.id;
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-18.rs"
```

```text
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-19.rs"
```

```text
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-20.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 8. `src/round_caps_tests.rs`

Copy actual GPU validation fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-21.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 9. `src/renderer.rs`

Register Native validation only.

<details>
<summary>Locate the existing block</summary>

```text
mod source_pen_tests;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-22.rs" skip=21
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 10. `src/stroke_gpu.rs`

Exercise the actual existing legacy pipeline in Native fixtures.

<details>
<summary>Locate the existing block</summary>

```text
impl Lane {
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-23.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 11. `src/cap_gpu_tests.rs`

Sample the unoccluded original back face, away from the now-visible end-on construction-plane marker. Keep the zero section-fill assertion.

<details>
<summary>Locate the existing block</summary>

```text
use crate::{scene::Scene,renderer::Renderer,viewport::Viewport};
use std::rc::Rc;
pub(crate) fn source(open:bool)->session_rust::Mesh{
 let corners=(0..8).map(|i|session_rust::Point::new(if i%4==1||i%4==2{0.6}else{-0.6},if i%4>=2{0.6}else{-0.6},if i>=4{-0.2}else{-0.8})).collect();let mut faces=vec![vec![0,3,2,1],vec![0,1,5,4],vec![2,3,7,6],vec![0,4,7,3],vec![1,2,6,5]];if !open{faces.push(vec![4,5,6,7]);}let mut mesh=session_rust::Mesh::from_vertices_and_faces(corners,faces);mesh.set_objectcolor(session_rust::Color::new(1.,0.,0.,1.));mesh
}
pub(crate) fn scene(open:bool)->(Scene,crate::scene::ObjectId){let mut scene=Scene::empty();let owner=scene.insert(crate::prepared::PreparedMesh::new(source(open)).unwrap()).unwrap();let plane=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Xy,&[[0.,0.,-0.5]],0.9).unwrap();crate::plane_original::insert(&mut scene,Rc::new(plane)).unwrap();(scene,owner)}
pub(crate) fn matrix()->[f32;16]{session_rust::Xform::orthographic(-1.,1.,-1.,1.,1.,0.).m.map(|v|v as f32)}
pub(crate) fn renderer(scene:&Scene)->Renderer{let instance=wgpu::Instance::new(wgpu::InstanceDescriptor{backends:wgpu::Backends::VULKAN,flags:Default::default(),memory_budget_thresholds:Default::default(),backend_options:Default::default(),display:None});let adapter=pollster::block_on(instance.request_adapter(&Default::default())).unwrap();let(device,queue)=pollster::block_on(adapter.request_device(&Default::default())).unwrap();device.on_uncaptured_error(std::sync::Arc::new(|e|panic!("actual original cap: {e}")));let mut renderer=Renderer::new(device,queue,wgpu::TextureFormat::Rgba8Unorm,scene);renderer.resize(Viewport{width:320,height:240});renderer}
pub(crate) fn wait(r:&Renderer){r.device.poll(wgpu::PollType::Wait{submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();}
pub(crate) fn pixels(r:&Renderer)->Vec<u8>{pixels_at(r,matrix())}
pub(crate) fn pixels_at(r:&Renderer,transform:[f32;16])->Vec<u8>{let texture=r.device.create_texture(&wgpu::TextureDescriptor{label:Some("original cap pixel fixture"),size:wgpu::Extent3d{width:320,height:240,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::COPY_SRC,view_formats:&[]});r.draw(&texture.create_view(&Default::default()),&Default::default(),&transform);let pitch=1280;let read=r.device.create_buffer(&wgpu::BufferDescriptor{label:None,size:pitch*240,usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let mut encoder=r.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&read,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(pitch as u32),rows_per_image:Some(240)}},texture.size());r.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|v|tx.send(v).unwrap());wait(r);rx.recv().unwrap().unwrap();let data=read.slice(..).get_mapped_range().to_vec();read.unmap();data}
pub(crate) fn at(data:&[u8],x:usize,y:usize)->&[u8]{&data[(y*320+x)*4..(y*320+x+1)*4]}
#[test]
fn actual_closed_original_sections_draw_grey_hatch_and_selected_color_then_release_large_images(){
 let(mut scene,owner)=scene(false);let mut r=renderer(&scene);let solid=pixels(&r);let center=at(&solid,208,96);assert!(center[..3].iter().all(|v|(150..=155).contains(v)),"solid section: {center:?}");assert_eq!(r.cap_usage()[1],1);assert_eq!(r.cap_usage()[2],1);assert_eq!(r.cap_usage()[0],16+16+320*240*2);let uploads=r.stats()[0];r.set_clipping(true,false);let hatch=pixels(&r);assert_ne!(hatch,solid);assert!(hatch.chunks_exact(4).filter(|p|p[..3].iter().all(|v|*v<20)).count()>400,"hatch has actual black ink");r.set_clipping(true,true);r.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&r);let center=at(&selected,208,96);assert!(center[0]>240&&center[1]>240&&center[2]<20,"selected original section: {center:?}");assert_eq!(r.stats()[0],uploads);assert_eq!(r.cap_usage()[2],1,"selection retains original closedness cache");let stored=selected.clone();r.use_uniform_surface_records();assert_eq!(pixels(&r),stored,"Exact uniform and shared cap crossing pixels");r.set_scene(&scene,None);r.select_objects(&scene,&[owner]).unwrap();assert_eq!(pixels(&r),stored);assert_eq!(r.stats()[0],uploads);scene.clear();r.set_scene(&scene,None);pixels(&r);assert_eq!(r.cap_usage()[0],16);assert_eq!(r.cap_usage()[1],0);
}
#[test]
fn actual_open_originals_get_no_section_fill(){let(scene,_)=scene(true);let r=renderer(&scene);let image=pixels(&r);assert_eq!(r.cap_usage()[1],0);let center=at(&image,160,120);assert!(center[0]>center[1]+80&&center[0]>center[2]+80,"open back face remains red: {center:?}");}

pub(crate) fn reversed(mut matrix:[f32;16])->[f32;16]{for column in 0..4{matrix[column*4+2]=matrix[column*4+3]-matrix[column*4+2];}matrix}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-24.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 12. `src/finite_visibility_tests.rs`

Retain exact clipped headed-chain comparison; distinguish genuine round source caps with exact body, flat-cut and bounded-cap pixel assertions.

<details>
<summary>Locate the existing block</summary>

```text
use crate::{scene::{Scene,ObjectId},renderer::Renderer,viewport::Viewport};
use std::rc::Rc;
fn scene(depth:f64,kind:u8,sloped:bool)->(Scene,ObjectId,ObjectId){
 let mut scene=Scene::empty();let points=if sloped{[[-0.8,-0.8,0.1],[0.8,-0.8,0.1],[0.,0.8,0.9]]}else{[[-0.8,-0.8,0.5],[0.8,-0.8,0.5],[0.,0.8,0.5]]};let solid=scene.insert(crate::prepared::PreparedMesh::triangle(points,[0.1,0.5,0.8]).unwrap()).unwrap();let color=session_rust::Color::new(1.,0.,0.,1.);
 let ink=if kind==2{let mut p=session_rust::Point::new(0.,0.,depth);p.width=10.;p.pointcolor=color;scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(p)).unwrap()).unwrap()}else{let mut line=session_rust::Line::new(-0.25,0.,depth,0.25,0.,depth);line.width=4.;line.linecolor=color;if kind==1{line.arrowhead=session_rust::Arrowhead::END;scene.insert_path(crate::chain::PreparedChain::line(Rc::new(line)).unwrap()).unwrap()}else{scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(line)).unwrap()).unwrap()}};(scene,solid,ink)
}
fn renderer(scene:&Scene,size:[u32;2],density:f32)->Renderer{renderer_limit(scene,size,density,None)}
fn renderer_limit(scene:&Scene,size:[u32;2],density:f32,storage:Option<u64>)->Renderer{let instance=wgpu::Instance::new(wgpu::InstanceDescriptor{backends:wgpu::Backends::VULKAN,flags:Default::default(),memory_budget_thresholds:Default::default(),backend_options:Default::default(),display:None});let adapter=pollster::block_on(instance.request_adapter(&Default::default())).unwrap();let mut descriptor=wgpu::DeviceDescriptor::default();if let Some(bytes)=storage{descriptor.required_limits.max_storage_buffer_binding_size=bytes;}let(device,queue)=pollster::block_on(adapter.request_device(&descriptor)).unwrap();device.on_uncaptured_error(std::sync::Arc::new(|error|panic!("actual finite visibility: {error}")));let mut renderer=Renderer::new(device,queue,wgpu::TextureFormat::Rgba8Unorm,scene);renderer.resize(Viewport{width:size[0],height:size[1]});renderer.set_density(density);renderer}
fn wait(renderer:&Renderer){renderer.device.poll(wgpu::PollType::Wait{submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();}
fn red_pixels(renderer:&Renderer,size:[u32;2])->usize{red_frame(renderer,size,crate::cap_gpu_tests::reversed(session_rust::Xform::identity().m.map(|v|v as f32)))}
fn red_frame(renderer:&Renderer,size:[u32;2],matrix:[f32;16])->usize{let texture=renderer.device.create_texture(&wgpu::TextureDescriptor{label:Some("finite visibility pixels"),size:wgpu::Extent3d{width:size[0],height:size[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::COPY_SRC,view_formats:&[]});renderer.draw(&texture.create_view(&Default::default()),&Default::default(),&matrix);let pitch=(size[0]*4).div_ceil(256)*256;let read=renderer.device.create_buffer(&wgpu::BufferDescriptor{label:None,size:u64::from(pitch)*u64::from(size[1]),usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let mut encoder=renderer.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&read,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(size[1])}},texture.size());renderer.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|r|tx.send(r).unwrap());wait(renderer);rx.recv().unwrap().unwrap();let bytes=read.slice(..).get_mapped_range();let count=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..size[0]as usize*4].chunks_exact(4)).filter(|p|p[0]>200&&p[1]<50&&p[2]<50).count();drop(bytes);read.unmap();count}
fn picked(renderer:&mut Renderer,at:[f32;2])->Option<[u32;2]>{pick_frame(renderer,at,crate::cap_gpu_tests::reversed(session_rust::Xform::identity().m.map(|v|v as f32)))}
fn pick_frame(renderer:&mut Renderer,at:[f32;2],matrix:[f32;16])->Option<[u32;2]>{renderer.request_pick(at,matrix).unwrap();wait(renderer);renderer.take_pick(matrix).unwrap().unwrap()}
#[test]
fn original_lines_headed_chains_and_points_share_actual_color_and_integer_occlusion(){
 for density in [1u32,2]{let size=[320*density,240*density];for kind in 0..3{
  let(hidden,solid,_)=scene(0.7,kind,false);let mut renderer=renderer(&hidden,size,density as f32);
  assert_eq!(picked(&mut renderer,[0.,0.]).unwrap()[0],solid.value(),"a hidden original cannot win a pick before the first color frame");assert_eq!(red_pixels(&renderer,size),0,"hidden family{kind}, density{density}");
  let(front,_,ink)=scene(0.3,kind,false);renderer.set_scene(&front,None);assert!(red_pixels(&renderer,size)>30,"actual visible family{kind}, density{density}");assert_eq!(picked(&mut renderer,[0.,0.]).unwrap()[0],ink.value());
 }}
 for density in [1u32,2]{let size=[320*density,240*density];for kind in 0..2{
  let make=|cropped:bool|{let mut scene=Scene::empty();let mut line=session_rust::Line::new(if cropped{0.}else{-0.8},0.,if cropped{0.}else{-0.2},0.8,0.,0.2);line.width=4.;line.linecolor=session_rust::Color::new(1.,0.,0.,1.);if kind==1&&!cropped{line.arrowhead=session_rust::Arrowhead::START;}let source=Rc::new(line);let id=if kind==1{scene.insert_path(crate::chain::PreparedChain::line(source.clone()).unwrap()).unwrap()}else{scene.insert_line(crate::stroke::PreparedLine::new(source.clone()).unwrap()).unwrap()};(scene,id,source)};
  let(original,id,source)=make(false);let mut gpu=renderer(&original,size,density as f32);
  assert_eq!(picked(&mut gpu,[-0.4,0.]),None,"near-clipped original ink cannot win a pick");
  assert_eq!(picked(&mut gpu,[0.4,0.]).unwrap()[0],id.value());
  let pixels=red_pixels(&gpu,size);assert!(pixels>100);
  if kind==0{assert!(Rc::ptr_eq(&original.lines()[0].prepared.source,&source));}else{assert!(matches!(&original.paths()[0].prepared.source,crate::chain::Source::Line(actual) if Rc::ptr_eq(actual,&source)));}
  let(cropped,_,_)=make(true);gpu.set_scene(&cropped,None);
  assert_eq!(red_pixels(&gpu,size),pixels,"homogeneous near clipping matches the explicit segment and invents no arrowhead at DPR{density}");
 }}
}
#[test]
fn a_line_on_a_sloping_original_face_keeps_its_full_css_width(){
 for density in [1u32,2]{let size=[320*density,240*density];let(front,_,_)=scene(0.3,0,false);let mut renderer=renderer(&front,size,density as f32);let flat=red_pixels(&renderer,size);let(sloped,_,ink)=scene(0.5,0,true);renderer.set_scene(&sloped,None);let grazing=red_pixels(&renderer,size);assert_eq!(grazing,flat,"finite center-line visibility preserves both sides of the four-CSS-pixel ribbon");assert_eq!(picked(&mut renderer,[0.,0.]).unwrap()[0],ink.value());}
}

#[test]
fn shifted_view_frames_keep_original_color_and_crop_ownership(){
 let size=[320,240];let mut matrix=crate::cap_gpu_tests::reversed(session_rust::Xform::identity().m.map(|v|v as f32));matrix[12]=0.35;matrix[13]=-0.25;
 for kind in 0..3{let(hidden,solid,_)=scene(0.7,kind,false);let mut renderer=renderer(&hidden,size,1.);assert_eq!(red_frame(&renderer,size,matrix),0);assert_eq!(pick_frame(&mut renderer,[0.35,-0.25],matrix).unwrap()[0],solid.value());let(front,_,ink)=scene(0.3,kind,false);renderer.set_scene(&front,None);assert!(red_frame(&renderer,size,matrix)>30);assert_eq!(pick_frame(&mut renderer,[0.35,-0.25],matrix).unwrap()[0],ink.value());}
}
#[test]
fn a_device_limit_retains_physical_color_and_pick_occlusion(){
 let size=[320,240];let(hidden,solid,_)=scene(0.7,0,false);let mut renderer=renderer_limit(&hidden,size,1.,Some(64*1024));assert_eq!(red_pixels(&renderer,size),0);assert_eq!(renderer.tile_usage()[..2],[96,16]);assert_eq!(picked(&mut renderer,[0.,0.]).unwrap()[0],solid.value());let(front,_,ink)=scene(0.3,0,false);renderer.set_scene(&front,None);assert!(red_pixels(&renderer,size)>30);assert_eq!(picked(&mut renderer,[0.,0.]).unwrap()[0],ink.value());
}
#[test]
fn an_unchanged_dense_view_consumes_actual_overflow_and_stops_rebuilding(){
 let mut scene=Scene::empty();for _ in 0..8{scene.insert(crate::prepared::PreparedMesh::triangle([[-10.,-10.,0.5],[10.,-10.,0.5],[0.,10.,0.5]],[0.1,0.5,0.8]).unwrap()).unwrap();}let size=[320,240];let renderer=renderer(&scene,size,1.);let mut matrix=crate::cap_gpu_tests::reversed(session_rust::Xform::identity().m.map(|v|v as f32));
 let texture=renderer.device.create_texture(&wgpu::TextureDescriptor{label:None,size:wgpu::Extent3d{width:320,height:240,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});let view=texture.create_view(&Default::default());renderer.draw(&view,&Default::default(),&matrix);let initial=renderer.tile_usage()[4];matrix[12]=0.01;renderer.draw(&view,&Default::default(),&matrix);wait(&renderer);
 for _ in 0..6{renderer.draw(&view,&Default::default(),&matrix);wait(&renderer);}let settled=renderer.tile_usage();assert!(settled[4]>initial,"actual overlapping triangles require a larger shared pool");assert!(settled[6]<=6,"actual growth converges");for _ in 0..3{renderer.draw(&view,&Default::default(),&matrix);wait(&renderer);}assert_eq!(renderer.tile_usage(),settled,"a resting view has no redraw or report-copy loop");
}

#[test]
fn coplanar_construction_grid_is_hidden_without_hiding_original_ink(){
 let size=[320,240];let(scene,_,_)=scene(0.3,0,false);let mut renderer=renderer(&scene,size,1.);let baseline=red_pixels(&renderer,size);let plane=crate::cplane::CPlane::Frame{origin:[0.,0.,0.5],x:[1.,0.,0.],y:[0.,1.,0.]};renderer.set_grid_on(Some(crate::grid::Settings{spacing:0.1,half_cells:1,..Default::default()}),plane).unwrap();assert_eq!(red_pixels(&renderer,size),baseline,"coplanar red construction grid cannot paint over the solid");let first=renderer.tile_usage();renderer.resize(crate::viewport::Viewport{width:319,height:239});let _=red_pixels(&renderer,[319,239]);assert_eq!(renderer.tile_usage()[5],first[5]+1,"physical viewport change invalidates projection even with the same tile dimensions");
}

#[test]
fn a_point_on_a_sloping_original_face_keeps_its_disc_and_first_pick(){
 for density in [1u32,2]{
  let size=[320*density,240*density];let(front,_,_)=scene(0.3,2,false);let mut renderer=renderer(&front,size,density as f32);let flat=red_pixels(&renderer,size);
  let(sloped,_,point)=scene(0.5,2,true);renderer.set_scene(&sloped,None);
  assert_eq!(picked(&mut renderer,[0.,0.]).unwrap()[0],point.value(),"surface-fitted point owns the first pick before color");
  assert_eq!(red_pixels(&renderer,size),flat,"a coplanar point keeps the same full CSS disc on a sloping face");
  let(hidden,solid,_)=scene(0.7,2,true);renderer.set_scene(&hidden,None);assert_eq!(picked(&mut renderer,[0.,0.]).unwrap()[0],solid.value());assert_eq!(red_pixels(&renderer,size),0);
 }
}

#[test]
fn translucent_faces_reveal_original_ink_without_giving_it_pick_ownership(){
 for density in [1u32,2]{let size=[320*density,240*density];for kind in 0..3{
  let(hidden,solid,_)=scene(0.7,kind,false);let mut renderer=renderer(&hidden,size,density as f32);renderer.set_opacity(0.1);
  assert_eq!(picked(&mut renderer,[0.,0.]).unwrap()[0],solid.value(),"occluded family{kind} cannot win the first pick through glass at DPR{density}");
  assert!(red_pixels(&renderer,size)>30,"hidden family{kind} still contributes its through-glass color");
  assert_eq!(picked(&mut renderer,[0.,0.]).unwrap()[0],solid.value(),"drawing the translucent frame does not change pick ownership");
 }}
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-25.rs"
```

</details>

</details>

<a id="checkpoint-96zzzzzzp-draw-default-mesh-vertex-dots"></a>

### Step 3 of 4 · Draw default mesh vertex dots (`96zzzzzzp-draw-default-mesh-vertex-dots`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 3/4 Visual Trace (96zzzzzzp-draw-default-mesh-vertex-dots): Draw default mesh vertex dots</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/mesh_pens.rs, src/mesh_parts.rs, src/bounds_original.rs, src/plane_original.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">Original nonzero edge</text>
    <text x="111" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">incidence</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">endpoint address</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">anchored marker</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">existing point upload</text>
    
</svg>
</div>

- **Goal:** Render the maintained default and authored mesh vertex dots without duplicating source meshes or endpoint positions.
- **Data Flow Trace:** `Original nonzero edge incidence → endpoint address → anchored marker → existing point upload`
- **Files You Touch:** `src/mesh_pens.rs` (*Prepare numeric endpoint references and only authored point colors on the existing display owner.*) · `src/mesh_parts.rs` (*Retain four-byte references into existing f64 endpoints rather than duplicate positions or original meshes.*) · `src/bounds_original.rs` (*Bounding frames do not invent mesh vertex markers.*) · `src/plane_original.rs` (*Original planes retain their own frame glyph policy.*) · `src/pick_sources.rs` (*Build ordinary point instances with object ownership and no new editable component domain.*) · `src/renderer.rs` (*Upload through the existing point buffer once, releasing temporary CPU instances.*) · `src/mesh_marker_tests.rs` (*Exercise actual GPU dots, source keys, authored colors, incidence and shared source unloading.*) · `src/source_pen_tests.rs` (*Extend the existing real 200000-triangle budget case without duplicating its dense fixture.*) · `src/global_outline_tests.rs` (*This existing fixture manually isolates surface-buffer rebasing; suppress both original ink and new default dots while preserving every surface/grid/cap/pick/source assertion. Production complete-lane marker rebasing is separately exercised at a billion units.*) · `src/scene_origin_tests.rs` (*Keep the original point/control count exact after the new eight default Box vertex markers; verify all marker values, owners, ordering and large-origin equality independently.*)

The original mesh border has ink but its default vertex dots are missing. Reuse its prepared topology to reference eligible endpoints, then upload their markers through the existing point lane.

A nonzero authored edge makes its endpoint vertices eligible even when a coplanar seam is omitted from the ink. Printed and dense fills have no dots. Preserve exact authored point colors when requested; otherwise use the maintained dark default. These dots select their object and remain separate from active editing controls.

> **Self-Check Question:** *Why retain an endpoint address rather than another original mesh or a second list of vertex positions?*  
> **Answer:** The existing shared display owner already retains precise endpoints after source unloading. A compact address reuses those positions and keeps transformed instances independent without introducing another geometry owner.

- **What You Should See:** Eligible original mesh vertices have their dark default or authored dot colors, original source queries remain correct, and shared placed geometry survives unloading and restoration.
- **Hands-On Lab Experiment:** Inspect actual sparse original vertices, authored and ignored point colors, zero widths, coplanar seam incidence, source unload and a mirrored shared pair. Check both projections and pixel densities.

<details>
<summary>Implementation 96zzzzzzp-draw-default-mesh-vertex-dots: Draw default mesh vertex dots</summary>

Render the maintained default and authored mesh vertex dots without duplicating source meshes or endpoint positions.

Data flow: Original nonzero edge incidence → endpoint address → anchored marker → existing point upload.

### 1. `src/mesh_pens.rs`

Prepare numeric endpoint references and only authored point colors on the existing display owner. Printed and dense fills have no default vertex decoration. Reuse the same source topology and sparse slot map once. A nonzero authored source edge makes its endpoints eligible, including a removed coplanar seam. Default dark color needs no per-vertex color allocation; common authored colors retain one entry.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::source_mesh_topology::{SlotMap,mesh_topology};
use session_rust::Mesh;
pub type Pen=(f32,u32);

pub fn width(value:f64)->f32 {
    let value=value as f32;
    if value.is_finite()&&value>0. {value} else {1.}
}

pub fn prepare(source:&Mesh,count:usize)->(Vec<(usize,usize)>,Vec<Pen>) {
    if count>200_000||source.widths()==[0.] {return (Vec::new(),Vec::new());}
    let keys=source.vertices();
    let points=keys.iter().map(|key|{let p=&source.vertex[key];[p.x,p.y,p.z]}).collect::<Vec<_>>();
    let topology=mesh_topology(source,&keys,&points,&SlotMap::new(&keys));
    let mut pens=Vec::with_capacity(topology.edges.len());
    for (i,&(_,_,colour)) in topology.edges.iter().enumerate() {
        let widths=source.widths();
        let value=if widths.len()==1 {widths[0]} else {widths.get(i).copied().unwrap_or(1.)};
        let pair=topology.edge_faces[i].map(|face|topology.normals.get(face as usize).copied().flatten());
        let flat=match pair {
            [Some(a),Some(b)]=>a.iter().zip(b).map(|(a,b)|a*b).sum::<f64>()*if topology.opposed[i]{1.}else{-1.}>=1.-1e-9,
            _=>false,
        };
        pens.push((if value==0.||flat {0.} else {width(value)},if topology.edges.len()>=10_000 {crate::source_topology_colour::BLACK} else {colour}));
    }
    if pens.first().is_some_and(|first|pens.iter().all(|pen|pen==first)) {pens=vec![pens[0]];}
    (topology.edges.into_iter().map(|(a,b,_)|(a,b)).collect(),pens)
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::source_mesh_topology::{SlotMap,mesh_topology};
use session_rust::Mesh;
pub type Pen=(f32,u32);

pub fn width(value:f64)->f32 {
    let value=value as f32;
    if value.is_finite()&&value>0. {value} else {1.}
}

pub fn prepare(source:&Mesh,count:usize)->(Vec<(usize,usize)>,Vec<Pen>,Vec<u32>,Vec<[f32;4]>) {
    if count>200_000||source.widths()==[0.] {return (Vec::new(),Vec::new(),Vec::new(),Vec::new());}
    let keys=source.vertices();
    let points=keys.iter().map(|key|{let p=&source.vertex[key];[p.x,p.y,p.z]}).collect::<Vec<_>>();
    let slots=SlotMap::new(&keys);
    let topology=mesh_topology(source,&keys,&points,&slots);
    let mut vertices=vec![u32::MAX;keys.len()];
    let mut pens=Vec::with_capacity(topology.edges.len());
    for (i,&(_,_,colour)) in topology.edges.iter().enumerate() {
        let widths=source.widths();
        let value=if widths.len()==1 {widths[0]} else {widths.get(i).copied().unwrap_or(1.)};
        if value!=0. {for(side,key)in [topology.edges[i].0,topology.edges[i].1].into_iter().enumerate(){let dot=&mut vertices[slots.slot(key)];if *dot==u32::MAX {*dot=(i*2+side)as u32;}}}
        let pair=topology.edge_faces[i].map(|face|topology.normals.get(face as usize).copied().flatten());
        let flat=match pair {
            [Some(a),Some(b)]=>a.iter().zip(b).map(|(a,b)|a*b).sum::<f64>()*if topology.opposed[i]{1.}else{-1.}>=1.-1e-9,
            _=>false,
        };
        pens.push((if value==0.||flat {0.} else {width(value)},if topology.edges.len()>=10_000 {crate::source_topology_colour::BLACK} else {colour}));
    }
    if pens.first().is_some_and(|first|pens.iter().all(|pen|pen==first)) {pens=vec![pens[0]];}
    let mut dots=Vec::new();let mut colours=Vec::new();let pc=source.get_pointcolors();
    let coloured=source.color_mode==session_rust::mesh::ColorMode::POINTCOLORS&&pc.len()==keys.len();
    for(slot,edge)in vertices.into_iter().enumerate(){if edge!=u32::MAX{dots.push(edge);if coloured{colours.push(pc[slot].to_f32());}}}
    if colours.first().is_some_and(|c|colours.iter().all(|v|v==c)){colours=vec![colours[0]];}
    (topology.edges.into_iter().map(|(a,b,_)|(a,b)).collect(),pens,dots,colours)
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-01.rs"
```

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-02.rs"
```

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-03.rs"
```

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-04.rs"
```

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-05.rs"
```

</details>

### 2. `src/mesh_parts.rs`

Retain four-byte references into existing f64 endpoints rather than duplicate positions or original meshes. Carry the prepared numerical decoration through source release. Keep the original shared display owner. Read authored point colors or the maintained default exactly, without byte quantization. Count actual retained numeric decoration capacity.

<details>
<summary>Locate the existing block</summary>

```rust
#[derive(Clone,Default)]
pub struct Parts {
    pub pens: Vec<crate::mesh_pens::Pen>,
    pub faces: Vec<usize>,
    pub triangles: Vec<u32>,
    pub edges: Vec<(usize, usize)>,
    pub endpoints: Vec<[[f64; 3]; 2]>,
}

impl Parts {
    // Render triangles can split one source polygon; retain its original key rather than treating each display triangle as a new face.
    pub fn new(source: &session_rust::Mesh, count: usize) -> Result<Self, &'static str> {
        let faces = source.faces(); let mut triangles = Vec::new();
        for (slot, &face) in faces.iter().enumerate() {
            let slot = u32::try_from(slot).map_err(|_| "Too many source faces")?;
            let cached = source.get_triangulation().get(&face).filter(|tris| !tris.is_empty());
            let indices = if let Some(cached) = cached { cached.clone() } else {
                let vertices = source.face_vertices(face).ok_or("Missing source face")?;
                (1..vertices.len().saturating_sub(1)).map(|i| [vertices[0], vertices[i], vertices[i + 1]]).collect()
            };
            for tri in indices { if tri.iter().all(|&key| source.vertex_point(key).is_some()) { triangles.push(slot); } }
        }
        if triangles.len() != count { return Err("Source faces do not match the rendered triangles"); }
        let (edges,pens)=crate::mesh_pens::prepare(source,count); let mut endpoints = Vec::new();
        for &(a, b) in &edges {
            let a = source.vertex_point(a).ok_or("Missing edge endpoint")?;
            let b = source.vertex_point(b).ok_or("Missing edge endpoint")?;
            endpoints.push([[a[0], a[1], a[2]], [b[0], b[1], b[2]]]);
        }
        Ok(Self { pens,faces, triangles, edges, endpoints })
    }

    pub fn pen(&self,index:usize)->(f32,[f32;4]) {
        let &(width,colour)=self.pens.get(if self.pens.len()==1 {0} else {index}).unwrap_or(&(1.,crate::source_topology_colour::BLACK));
        (width,colour.to_le_bytes().map(|v|v as f32/255.))
    }

    pub fn bytes(&self) -> usize {
        self.pens.capacity()*std::mem::size_of::<crate::mesh_pens::Pen>()+self.faces.capacity() * std::mem::size_of::<usize>() + self.triangles.capacity() * 4
            + self.edges.capacity() * std::mem::size_of::<(usize, usize)>() + self.endpoints.capacity() * 48
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
#[derive(Clone,Default)]
pub struct Parts {
    pub dots:Vec<u32>,
    pub dot_colours:Vec<[f32;4]>,
    pub pens: Vec<crate::mesh_pens::Pen>,
    pub faces: Vec<usize>,
    pub triangles: Vec<u32>,
    pub edges: Vec<(usize, usize)>,
    pub endpoints: Vec<[[f64; 3]; 2]>,
}

impl Parts {
    // Render triangles can split one source polygon; retain its original key rather than treating each display triangle as a new face.
    pub fn new(source: &session_rust::Mesh, count: usize) -> Result<Self, &'static str> {
        let faces = source.faces(); let mut triangles = Vec::new();
        for (slot, &face) in faces.iter().enumerate() {
            let slot = u32::try_from(slot).map_err(|_| "Too many source faces")?;
            let cached = source.get_triangulation().get(&face).filter(|tris| !tris.is_empty());
            let indices = if let Some(cached) = cached { cached.clone() } else {
                let vertices = source.face_vertices(face).ok_or("Missing source face")?;
                (1..vertices.len().saturating_sub(1)).map(|i| [vertices[0], vertices[i], vertices[i + 1]]).collect()
            };
            for tri in indices { if tri.iter().all(|&key| source.vertex_point(key).is_some()) { triangles.push(slot); } }
        }
        if triangles.len() != count { return Err("Source faces do not match the rendered triangles"); }
        let (edges,pens,dots,dot_colours)=crate::mesh_pens::prepare(source,count); let mut endpoints = Vec::new();
        for &(a, b) in &edges {
            let a = source.vertex_point(a).ok_or("Missing edge endpoint")?;
            let b = source.vertex_point(b).ok_or("Missing edge endpoint")?;
            endpoints.push([[a[0], a[1], a[2]], [b[0], b[1], b[2]]]);
        }
        Ok(Self { dots,dot_colours,pens,faces, triangles, edges, endpoints })
    }

    pub fn pen(&self,index:usize)->(f32,[f32;4]) {
        let &(width,colour)=self.pens.get(if self.pens.len()==1 {0} else {index}).unwrap_or(&(1.,crate::source_topology_colour::BLACK));
        (width,colour.to_le_bytes().map(|v|v as f32/255.))
    }

    pub fn dot_colour(&self,index:usize)->[f32;4]{
        self.dot_colours.get(if self.dot_colours.len()==1{0}else{index}).copied().unwrap_or([0.1,0.1,0.1,1.])
    }

    pub fn bytes(&self) -> usize {
        self.dots.capacity()*4+self.dot_colours.capacity()*16+self.pens.capacity()*std::mem::size_of::<crate::mesh_pens::Pen>()+self.faces.capacity() * std::mem::size_of::<usize>() + self.triangles.capacity() * 4
            + self.edges.capacity() * std::mem::size_of::<(usize, usize)>() + self.endpoints.capacity() * 48
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-06.rs"
```

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-07.rs"
```

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-08.rs"
```

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-09.rs"
```

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-10.rs"
```

</details>

### 3. `src/bounds_original.rs`

Bounding frames do not invent mesh vertex markers.

<details>
<summary>Locate the existing block</summary>

```rust
        pens:Vec::new(),
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        dots:Vec::new(),dot_colours:Vec::new(),pens:Vec::new(),
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-11.rs"
```

</details>

### 4. `src/plane_original.rs`

Original planes retain their own frame glyph policy.

<details>
<summary>Locate the existing block</summary>

```rust
    let endpoints=edges.iter().map(|&(a,b)|[&points[a],&points[b]].map(|p|[p[0],p[1],p[2]])).collect();display.parts=crate::mesh_parts::Parts{pens:Vec::new(),faces:Vec::new(),triangles:Vec::new(),edges,endpoints};
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    let endpoints=edges.iter().map(|&(a,b)|[&points[a],&points[b]].map(|p|[p[0],p[1],p[2]])).collect();display.parts=crate::mesh_parts::Parts{dots:Vec::new(),dot_colours:Vec::new(),pens:Vec::new(),faces:Vec::new(),triangles:Vec::new(),edges,endpoints};
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-12.rs"
```

</details>

### 5. `src/pick_sources.rs`

Build ordinary point instances with object ownership and no new editable component domain. Transform numerical endpoints after origin anchoring; BRep, NURBS and derived contact shapes keep their own display policy. Account for temporary marker preparation; move it into the existing upload.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    pub fn bytes(&self) -> usize {
        (self.faces.capacity() + self.edges.capacity() + self.controls.capacity()) * std::mem::size_of::<Target>()
            + self.mesh_bases.capacity() * std::mem::size_of::<(ObjectId, u32)>()
            + self.mesh_metadata.capacity()*4
            + self.mesh_ink.capacity() * std::mem::size_of::<crate::stroke::Stroke>()
            + (self.mesh_ids.capacity() + self.line_ids.capacity() + self.path_ids.capacity() + self.marker_ids.capacity()) * 8
```

</details>

Replace it with:

```rust
    pub fn bytes(&self) -> usize {
        (self.faces.capacity() + self.edges.capacity() + self.controls.capacity()) * std::mem::size_of::<Target>()
            + self.mesh_bases.capacity() * std::mem::size_of::<(ObjectId, u32)>()
            + self.mesh_markers.capacity()*std::mem::size_of::<crate::marker::Marker>()
            + self.mesh_metadata.capacity()*4
            + self.mesh_ink.capacity() * std::mem::size_of::<crate::stroke::Stroke>()
            + (self.mesh_ids.capacity() + self.line_ids.capacity() + self.path_ids.capacity() + self.marker_ids.capacity()) * 8
```

<details>
<summary>Locate this block</summary>

```rust
                let n=row.mesh.normals()[0];let m=crate::normals::matrix(&row.model)?;
                facing=crate::packed_vertex::oct32(std::array::from_fn(|i|(0..3).map(|j|m[j*4+i]*n[j]).sum()));
            }
            let flag=if planes.contains(&row.id){4096}else{0};
            let boundary_width=row.cad.as_ref().map(|c|c.source.width).or_else(||row.surface.as_ref().map(|s|s.source.width)).map(crate::mesh_pens::width);
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
```

</details>

Replace it with:

```rust
                let n=row.mesh.normals()[0];let m=crate::normals::matrix(&row.model)?;
                facing=crate::packed_vertex::oct32(std::array::from_fn(|i|(0..3).map(|j|m[j*4+i]*n[j]).sum()));
            }
            if row.cad.is_none()&&row.surface.is_none()&&!derived {
                for(i,&address)in row.mesh.parts.dots.iter().enumerate(){
                    let p=row.mesh.parts.endpoints[address as usize/2][address as usize%2];
                    sources.mesh_markers.push(crate::marker::Marker{center:model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32(),diameter:1.,colour:edge_colour.unwrap_or_else(||row.mesh.parts.dot_colour(i))});
                    sources.marker_ids.push([owner,0]);
                }
            }
            let flag=if planes.contains(&row.id){4096}else{0};
            let boundary_width=row.cad.as_ref().map(|c|c.source.width).or_else(||row.surface.as_ref().map(|s|s.source.width)).map(crate::mesh_pens::width);
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
```

<details>
<summary>Locate this block</summary>

```rust
    pub mesh_metadata:Vec<u32>,
    pub line_ids: Vec<[u32; 2]>,
    pub path_ids: Vec<[u32; 2]>,
    pub marker_ids: Vec<[u32; 2]>,
}
```

</details>

Replace it with:

```rust
    pub mesh_metadata:Vec<u32>,
    pub line_ids: Vec<[u32; 2]>,
    pub path_ids: Vec<[u32; 2]>,
    pub mesh_markers:Vec<crate::marker::Marker>,
    pub marker_ids: Vec<[u32; 2]>,
}
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{scene::{ObjectId, Scene}, sub_id::{self, Kind}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part { Object, Face(usize), SurfaceEdge(usize), BRepEdge(usize), MeshEdge(usize, usize), PathEdge(usize), Curve, Control(usize), SourceControl(crate::source_controls::ControlId), CloudPoint(u32), SheetEntity(u32) }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Target { pub owner: ObjectId, pub part: Part }

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u32)]
pub enum Mode { #[default] Object, Face, Edge, Control }

#[derive(Default)]
pub struct Sources {
    pub sheet_bases:std::collections::HashMap<(ObjectId,usize),u32>,
    pub faces: Vec<Target>,
    pub edges: Vec<Target>,
    pub controls: Vec<Target>,
    pub mesh_bases: Vec<(ObjectId, u32)>,
    pub mesh_ink: Vec<crate::stroke::Stroke>,
    pub mesh_ids: Vec<[u32; 2]>,
    pub mesh_metadata:Vec<u32>,
    pub line_ids: Vec<[u32; 2]>,
    pub path_ids: Vec<[u32; 2]>,
    pub mesh_markers:Vec<crate::marker::Marker>,
    pub marker_ids: Vec<[u32; 2]>,
}

fn append(rows: &mut Vec<Target>, kind: Kind, target: Target) -> Result<[u32; 2], &'static str> {
    let tag = sub_id::encode(kind, rows.len())?; rows.push(target); Ok([target.owner.value(), tag])
}

impl Sources {
    // A source table translates compact GPU addresses to original keys; addresses belong to this scene snapshot, not the document's saved identity.
    pub fn new(scene:&Scene)->Result<Self,&'static str> {Self::anchored(scene,[0.;3])}
    pub fn anchored(scene:&Scene,anchor:[f64;3])->Result<Self,&'static str> {
        let mut sources = Self::default();
        let id = |owner: ObjectId, tag| [crate::feature_owner::display_owner(scene, owner).map_or(0, ObjectId::value),
            if scene.forest.derived(owner) { 0 } else { tag }];
        let planes=crate::clipping_rows::owners(scene);
        for row in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let derived=scene.forest.derived(row.id);
            let owner=crate::feature_owner::display_owner(scene,row.id).map_or(0,ObjectId::value);
            let edge_colour=scene.view.edges.get(&row.id).map(|rgb|[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]);
            let model=crate::render_space::placed(&row.model,anchor);
            sources.mesh_bases.push((row.id, u32::try_from(sources.faces.len()).map_err(|_| "Too many source faces")?));
            if !derived {
                for &key in &row.mesh.parts.faces {
                    append(&mut sources.faces,Kind::Face,Target{owner:row.id,part:Part::Face(key)})?;
                }
            }
            let mut facing=crate::packed_vertex::NO_NORMAL;
            if row.mesh.alphas().first().is_some_and(|a|(*a-1./255.).abs()<0.5/255.) {
                let n=row.mesh.normals()[0];let m=crate::normals::matrix(&row.model)?;
                facing=crate::packed_vertex::oct32(std::array::from_fn(|i|(0..3).map(|j|m[j*4+i]*n[j]).sum()));
            }
            if row.cad.is_none()&&row.surface.is_none()&&!derived {
                for(i,&address)in row.mesh.parts.dots.iter().enumerate(){
                    let p=row.mesh.parts.endpoints[address as usize/2][address as usize%2];
                    sources.mesh_markers.push(crate::marker::Marker{center:model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32(),diameter:1.,colour:edge_colour.unwrap_or_else(||row.mesh.parts.dot_colour(i))});
                    sources.marker_ids.push([owner,0]);
                }
            }
            let flag=if planes.contains(&row.id){4096}else{0};
            let boundary_width=row.cad.as_ref().map(|c|c.source.width).or_else(||row.surface.as_ref().map(|s|s.source.width)).map(crate::mesh_pens::width);
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
            let mut surface_edges=row.surface.as_ref().map(|surface|surface.edge_ids());
            for (index,(&(a,b),ends)) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints).enumerate() {
                let part=cad_edges.as_mut().and_then(|edges|edges.next()).map(|edge|Part::BRepEdge(*edge)).unwrap_or_else(||surface_edges.as_mut().and_then(|edges|edges.next()).map_or(Part::MeshEdge(a,b),Part::SurfaceEdge));
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
                let (width,colour)=row.mesh.parts.pen(index);
                let width=boundary_width.unwrap_or(width);
                if width==0. {continue;}
                let ends=ends.map(|p|model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());
                sources.mesh_ink.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: edge_colour.unwrap_or(colour), width });
                sources.mesh_ids.push([owner,if derived{0}else{ids[1]}]);
                sources.mesh_metadata.extend([flag,facing]);
            }
        }
        for row in scene.lines().iter().filter(|row| scene.visible(row.id)) {
            let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part: Part::Curve })?;
            sources.line_ids.push(id(row.id, ids[1]));
        }
        for row in scene.paths().iter().filter(|row| scene.visible(row.id)) {
            let points:Vec<_>=row.points_at(anchor).iter().map(session_rust::Point::to_f32).collect();
            for (index, _) in points.windows(2).enumerate().filter(|(_, pair)| pair[0] != pair[1]) {
                let part = if matches!(row.prepared.source, crate::chain::Source::Polyline(_)) { Part::PathEdge(index) } else { Part::Curve };
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
                sources.path_ids.push(id(row.id, ids[1]));
            }
        }
        for row in scene.points().iter().filter(|row| scene.visible(row.id)) { sources.marker_ids.push(id(row.id, 1)); }
        for (owner, stroke) in scene.control_nets_at(anchor) {
            sources.mesh_ink.push(stroke);
            sources.mesh_ids.push([owner.value(), 0]);
            sources.mesh_metadata.extend([0,crate::packed_vertex::NO_NORMAL]);
        }
        for row in scene.control_markers_at(anchor) {
            let ids = append(&mut sources.controls, Kind::Control, Target { owner: row.parent, part: row.part })?;
            sources.marker_ids.push(id(row.parent, ids[1]));
        }
        for row in scene.sheets().iter().filter(|row|scene.visible(row.id)) {for page in row.prepared.pages() {sources.sheet_bases.insert((row.id,page.key()),u32::try_from(sources.edges.len()).map_err(|_|"Too many sheet pick addresses")?);for entity in &page.entities {append(&mut sources.edges,Kind::Edge,Target {owner:row.id,part:if *entity==u32::MAX {Part::Object} else {Part::SheetEntity(*entity)}})?;}}}
        Ok(sources)
    }

    pub fn bytes(&self) -> usize {
        (self.faces.capacity() + self.edges.capacity() + self.controls.capacity()) * std::mem::size_of::<Target>()
            + self.mesh_bases.capacity() * std::mem::size_of::<(ObjectId, u32)>()
            + self.mesh_markers.capacity()*std::mem::size_of::<crate::marker::Marker>()
            + self.mesh_metadata.capacity()*4
            + self.mesh_ink.capacity() * std::mem::size_of::<crate::stroke::Stroke>()
            + (self.mesh_ids.capacity() + self.line_ids.capacity() + self.path_ids.capacity() + self.marker_ids.capacity()) * 8
    }

    pub fn resolve(&self, scene: &Scene, pixel: [u32; 2], mode: Mode) -> Option<Target> {
        let owner = scene.pick_id(pixel[0])?;
        if mode == Mode::Object { return Some(Target { owner, part: Part::Object }); }
        let (kind, address) = sub_id::decode(pixel[1])?;
        if mode==Mode::Control && kind==Kind::Control && (scene.clouds().iter().any(|r|r.id==owner)||scene.streams().iter().any(|r|r.id==owner)) {
            return crate::cloud_pick::resolve(scene,owner,address);
        }
        let target = match (mode, kind) {
            (Mode::Face, Kind::Face) => self.faces.get(address),
            (Mode::Edge, Kind::Edge) => self.edges.get(address),
            (Mode::Control, Kind::Control) => self.controls.get(address), _ => None,
        }?;
        (target.owner == owner && target.valid(scene)).then_some(*target)
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-13.rs"
```

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-14.rs"
```

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-15.rs"
```

</details>

### 6. `src/renderer.rs`

Upload through the existing point buffer once, releasing temporary CPU instances. Match default mesh, original point and active control ownership order.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
        self.mesh_ink.set(&self.device, self.sources.mesh_ink.iter().copied()).expect("Prepared source edges");
        self.mesh_ink.set_ids(&self.device, self.sources.mesh_ids.iter().copied()).expect("Source edge IDs match");
        self.mesh_ink.set_records(&self.device,std::mem::take(&mut self.sources.mesh_metadata)).expect("Original ink metadata");
        self.markers.set_points(&self.device, scene.points().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut marker=row.marker_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {marker.colour=colour;}marker}).chain(scene.control_markers_at(anchor).into_iter().map(|c| c.display)))
            .expect("Prepared point placement fits the display range");
        self.paths.set_chains(&self.device, scene.paths().iter().filter(|row| scene.visible(row.id)).flat_map(|row| {let mut segments=row.segments_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {for segment in &mut segments {segment.stroke.colour=colour;}}segments}))
            .expect("Prepared path placement fits the display range");
```

</details>

Replace it with:

```rust
        self.mesh_ink.set(&self.device, self.sources.mesh_ink.iter().copied()).expect("Prepared source edges");
        self.mesh_ink.set_ids(&self.device, self.sources.mesh_ids.iter().copied()).expect("Source edge IDs match");
        self.mesh_ink.set_records(&self.device,std::mem::take(&mut self.sources.mesh_metadata)).expect("Original ink metadata");
        self.markers.set_points(&self.device, std::mem::take(&mut self.sources.mesh_markers).into_iter().chain(scene.points().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut marker=row.marker_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {marker.colour=colour;}marker})).chain(scene.control_markers_at(anchor).into_iter().map(|c| c.display)))
            .expect("Prepared point placement fits the display range");
        self.paths.set_chains(&self.device, scene.paths().iter().filter(|row| scene.visible(row.id)).flat_map(|row| {let mut segments=row.segments_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {for segment in &mut segments {segment.stroke.colour=colour;}}segments}))
            .expect("Prepared path placement fits the display range");
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{gpu_mesh::GpuMesh, scene::{ObjectId, Scene}};

pub struct Renderer {
    surface_options:std::cell::Cell<[bool;2]>,
    world_bounds:std::cell::RefCell<Option<crate::bounds::Bounds>>,
    space:crate::render_space::Origin,
    opacity:std::cell::Cell<f32>,
    ambient_enabled:std::cell::Cell<bool>,
    ambient:std::cell::RefCell<Option<crate::ambient_gpu::Ambient>>,
    ambient_rows:crate::ambient_rows::Rows,
    pyramid:std::cell::RefCell<Option<crate::pyramid_gpu::Pyramid>>,
    pyramid_revision:u64,
    caps:std::cell::RefCell<crate::caps_gpu::Caps>,
    clipping:std::cell::RefCell<crate::clipping_gpu::Clipping>,
    tiles:std::cell::RefCell<crate::tile_gpu::Tiles>,
    guides:crate::guide_gpu::Guide,
    measurement:crate::measure_gpu::Measurement,
    sheets:crate::sheet_gpu::Lane,
    clouds:std::cell::RefCell<Option<crate::cloud_gpu::Lane>>,
    cloud_view:std::cell::Cell<crate::cloud_gpu::View>,
    cloud_error:std::cell::RefCell<Option<String>>,
    outline:std::cell::RefCell<Option<crate::outline_gpu::Outline>>,
    global_outline:std::cell::RefCell<Option<crate::outline_gpu::Outline>>,
    outlines_enabled:std::cell::Cell<bool>,
    outline_composite:std::cell::RefCell<Option<crate::outline_composite::Composite>>,
    text:std::cell::RefCell<Option<crate::scene_text_gpu::Lane>>,
    text_error:std::cell::RefCell<Option<String>>,
    full_text_fonts:std::cell::RefCell<Option<Vec<Vec<u8>>>>,
    presentation_text:std::cell::RefCell<Vec<crate::text::TextLabel>>,
    scene_text:std::cell::RefCell<Vec<crate::text::TextLabel>>,
    format:wgpu::TextureFormat,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    instances:crate::instance_gpu::Lane,
    instance_rows:std::collections::HashSet<ObjectId>,
    pipeline: wgpu::RenderPipeline,
    highlight_pipeline: wgpu::RenderPipeline,
    highlight: crate::highlight::Highlight,
    preview: crate::tool_preview::Preview,
    widget: crate::widget::Widget,
    meshes: Vec<GpuMesh>,
    geometry: crate::geometry_cache::GeometryCache,
    settings_allocations: usize,
    settings_writes: usize,
    settings_bytes: usize,
    uniform: wgpu::Buffer,
    view_group: wgpu::BindGroup,
    depth: wgpu::TextureView,
    primitive:crate::primitive_image::Image,
    strokes: crate::stroke_gpu::Lane,
    paths: crate::stroke_gpu::Lane,
    markers: crate::stroke_gpu::Lane,
    grid: crate::stroke_gpu::Lane,
    grid_settings: Option<crate::grid::Settings>,
    grid_plane:crate::cplane::CPlane,grid_anchor:[f64;3],
    size: [u32; 2],
    density: f32,
    picker: crate::pick_gpu::Picker,
    sources: crate::pick_sources::Sources,
    faces: crate::face_pick::Faces,
    mesh_ink: crate::stroke_gpu::Lane,
    pick_mode:crate::pick_sources::Mode,
    pick_cursor:Option<([f32;2],[u64;3])>,
}

impl Renderer {
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        format: wgpu::TextureFormat,
        scene: &Scene,
    ) -> Self {
        crate::shader_check::check(&device, crate::clipping_gpu::source(include_str!("cloud_points.wgsl"),2,0), "Cloud points");
        crate::shader_check::check(&device, include_str!("cloud_resolve.wgsl").into(), "Cloud resolve");
        crate::shader_check::check(&device, crate::finite_project_source::source(), "Finite projection");
        crate::shader_check::check(&device, crate::finite_project_source::tiles(), "Finite tiles");
        crate::shader_check::check(&device, crate::finite_project_source::scan(), "Finite scan");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0).into()),
        });
        let view_layout = crate::surface_view::layout(&device);
        let model_layout = crate::gpu_layout::uniform(&device, crate::object_record::BYTES as u64);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mesh settings"), bind_group_layouts: &[Some(&view_layout), Some(&model_layout),Some(&crate::clipping_gpu::layout(&device)),Some(&crate::gpu_geometry::GpuGeometry::storage_layout(&device))], immediate_size: 0 });
        let mesh_pipeline = |depth,primitive_write| device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[crate::draw_slots::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState{format,blend:Some(wgpu::BlendState::ALPHA_BLENDING),write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(primitive_write))],
            }),
            primitive: Default::default(),
            depth_stencil: Some(depth),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let pipeline = mesh_pipeline(crate::depth::surface(),true);
        let tiles=std::cell::RefCell::new(crate::tile_gpu::Tiles::new(&device,&view_layout));
        let instances=crate::instance_gpu::Lane::new(&device,&queue,format,&view_layout);
        let highlight_pipeline = mesh_pipeline(crate::depth::ink(),false);
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
        let widget = crate::widget::Widget::new(&device, format);
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform and normal diagnostic"),
            size: crate::surface_view::BYTES,
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
        let depth = Self::depth(&device, 640, 480);
        let primitive=crate::primitive_image::Image::new(&device,[640,480]);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
        let grid = crate::stroke_gpu::Lane::grid(&device, format);
        let picker = crate::pick_gpu::Picker::new(&device, &view_layout, &model_layout);
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets=crate::sheet_gpu::Lane::new(&device,format);
        let measurement=crate::measure_gpu::Measurement::new(&device,format);let guides=crate::guide_gpu::Guide::new(&device,format);
        let mut renderer = Self {surface_options:Default::default(),world_bounds:Default::default(),space:Default::default(),opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, primitive, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy,grid_anchor:[0.;3], size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode:Default::default(),pick_cursor:None };
        renderer.mesh_ink.enable_finite(&renderer.device,format,true);
        for lane in [&mut renderer.strokes,&mut renderer.paths,&mut renderer.markers]{lane.enable_finite(&renderer.device,format,false);}
        renderer.highlight.enable_finite(&renderer.device,format);
        renderer.set_scene(scene, None);
        renderer
    }

    pub fn anchor(&self)->[f64;3] {self.space.anchor()}

    fn depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("opaque depth"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: crate::depth::FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        depth_texture.create_view(&Default::default())
    }

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        if self.size != [size.width, size.height] { self.cancel_pick(); }
        self.size = [size.width, size.height];
        self.depth = Self::depth(&self.device, size.width, size.height);
        self.primitive=crate::primitive_image::Image::new(&self.device,self.size);
    }

    pub fn set_density(&mut self, density: f32) {
        if density.is_finite() && density > 0.0 && density != self.density { self.cancel_pick(); self.density = density; }
    }

    pub fn stroke_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.strokes.usage();
        [count, bytes, self.strokes.uploads, self.strokes.uploaded_bytes]
    }

    pub fn path_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.paths.usage();
        [count, bytes, self.paths.uploads, self.paths.uploaded_bytes]
    }

    pub fn point_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.markers.usage();
        [count, bytes, self.markers.uploads, self.markers.uploaded_bytes]
    }

    #[cfg(test)]
    pub(crate) fn use_uniform_surface_records(&mut self) {self.instances.ordinary.clear();}

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        let anchor=self.anchor();
        self.ambient_rows.set(scene);if let Some(ambient)=self.ambient.borrow_mut().as_mut(){if self.ambient_enabled.get(){ambient.sources.set(&self.device,&self.queue,&self.ambient_rows.specs);}}
        self.pyramid_revision=self.pyramid_revision.wrapping_add(1);
        self.clipping.borrow_mut().set(scene);
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        self.cancel_pick(); self.faces.clear();
        self.preview.set(&self.device, None).expect("Empty tool preview");
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, None).expect("Empty selection");
        self.set_source_ink(scene);
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let matching = old.iter().position(|row| row.id == object.id
                && std::rc::Rc::ptr_eq(&row.geometry.source, &object.mesh));
            let row = if let Some(index) = matching {
                let mut row = old.swap_remove(index);
                let [calls, bytes] = row.update(&self.queue, object,style(scene,object.id,selected==Some(object.id)));
                self.settings_writes += calls;
                self.settings_bytes += bytes;
                row
            } else {
                let geometry = self.geometry.get(&crate::text_gpu_support::GpuCtx::new(&self.device,&self.queue), &object.mesh);
                self.settings_allocations += 1;
                GpuMesh::with_geometry_at(&self.device,&layout,object,
                    style(scene,object.id,selected==Some(object.id)),geometry,anchor)
            };
            self.meshes.push(row);
        }
        drop(old);
        self.caps.borrow_mut().set_scene(&self.queue,scene,&mut self.meshes,&self.clipping.borrow().rows);
        for mesh in &mut self.meshes{mesh.set_xray(&self.queue,self.opacity.get()==0.);}
        let radii=self.ambient_rows.specs.iter().copied().collect::<std::collections::HashMap<_,_>>();for mesh in &mut self.meshes {mesh.set_ambient_radius(&self.queue,radii.get(&mesh.id.value()).copied().unwrap_or(0.));}
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,true).expect("Validated instance records fit device limits");
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
        self.sheets.set_anchored(&self.device,&self.queue,scene,selected.as_slice(),&self.sources.sheet_bases,anchor).expect("Prepared original sheet batches fit device limits");
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        if let Err(error)=self.set_clouds(scene,selected.as_slice()) {*self.cloud_error.borrow_mut()=Some(error.to_string());}
        if let Err(error)=self.set_scene_text(scene,selected.as_slice()) {*self.text_error.borrow_mut()=Some(error.to_string());}
        *self.world_bounds.borrow_mut()=scene.bounds();
    }

    pub fn grow_camera(&self,camera:&mut crate::camera::Camera) {
        if let Some(bounds)=self.world_bounds.borrow().as_ref() {camera.grow_extent(bounds);}
    }

    pub fn camera_matrix(&self,camera:&crate::camera::Camera)->[f32;16] {camera.view_projection_anchored(self.anchor()).to_f32()}
    pub fn rebase_delay(&self,now:f64)->Option<u32> {self.space.retry_after_ms(now)}
    pub fn prepare_origin(&mut self,camera:&crate::camera::Camera,scene:&Scene,target:Option<crate::pick_sources::Target>,ids:&[ObjectId],now:f64)->Result<crate::render_space::Rebase,&'static str> {
        let previous=self.anchor();let mut frame=self.space.update(camera.target,camera.distance,now).ok_or("Invalid camera origin")?;
        frame.moved&=previous.map(f64::to_bits)!=frame.anchor.map(f64::to_bits);
        if frame.moved {
            self.pyramid_revision=self.pyramid_revision.wrapping_add(1);
            for mesh in &mut self.meshes {let[calls,bytes]=mesh.rebase(&self.queue,frame.anchor);self.settings_writes+=calls;self.settings_bytes+=bytes;}
            self.set_source_ink(scene);
            if !self.highlight.rebase_face(&self.queue,frame.anchor,target){self.set_highlight(scene,target)?;}
            self.select_objects(scene,ids)?;
            self.set_grid_on(self.grid_settings,self.grid_plane)?;
        }
        self.cloud_camera(camera);Ok(frame)
    }

    fn set_source_ink(&mut self,scene:&Scene) {
        let anchor=self.anchor();
        self.sources = crate::pick_sources::Sources::anchored(scene,anchor).expect("Prepared source topology is bounded");
        self.mesh_ink.set(&self.device, self.sources.mesh_ink.iter().copied()).expect("Prepared source edges");
        self.mesh_ink.set_ids(&self.device, self.sources.mesh_ids.iter().copied()).expect("Source edge IDs match");
        self.mesh_ink.set_records(&self.device,std::mem::take(&mut self.sources.mesh_metadata)).expect("Original ink metadata");
        self.markers.set_points(&self.device, std::mem::take(&mut self.sources.mesh_markers).into_iter().chain(scene.points().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut marker=row.marker_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {marker.colour=colour;}marker})).chain(scene.control_markers_at(anchor).into_iter().map(|c| c.display)))
            .expect("Prepared point placement fits the display range");
        self.paths.set_chains(&self.device, scene.paths().iter().filter(|row| scene.visible(row.id)).flat_map(|row| {let mut segments=row.segments_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {for segment in &mut segments {segment.stroke.colour=colour;}}segments}))
            .expect("Prepared path placement fits the display range");
        self.strokes.set(&self.device, scene.lines().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut stroke=row.stroke_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {stroke.colour=colour;}stroke}))
            .expect("Prepared line placement fits the display range");
        self.markers.set_ids(&self.device, self.sources.marker_ids.iter().copied()).expect("Marker owners match");
        self.paths.set_ids(&self.device, self.sources.path_ids.iter().copied()).expect("Path owners match");
        self.strokes.set_ids(&self.device, self.sources.line_ids.iter().copied()).expect("Line owners match");
    }

    pub fn request_pick(&mut self, screen: [f32; 2], transform: [f32; 16]) -> Result<(), &'static str> {
        let window = crate::pick_window::Window::new(screen, self.size, self.density).ok_or("Invalid pick cursor")?;
        let origin=self.anchor();self.picker.queue_anchored(crate::pick_gpu::Request{window,transform,density:self.density,mode:self.pick_mode},origin)?;
        self.pick_cursor=Some((screen,origin.map(f64::to_bits)));self.pump_pick(transform);Ok(())
    }

    fn pump_pick(&mut self, transform: [f32; 16]) {
        let Ok(view) = crate::pick_view::View::anchored(transform,self.size,self.density,self.anchor()) else { self.cancel_pick(); return; };
        if self.pick_mode == crate::pick_sources::Mode::Face && !self.faces.ready(self.meshes.len()) {
            self.faces.set(&self.device, &self.meshes, &self.sources.mesh_bases,&self.clipping.borrow().buffer);
        }
        if let Some(lane)=self.text.borrow_mut().as_mut() {if let Err(error)=lane.prepare(&self.text_context(),&self.text_frame(transform)) {*self.text_error.borrow_mut()=Some(error.to_string());return;}}
        self.clipping.borrow_mut().frame_anchored(&self.queue,transform,self.anchor(),self.size[1],self.density);
        self.prepare_clouds(transform);
        if let Some(request) = self.picker.next(view) {
            let mut clipping=self.clipping.borrow_mut();clipping.frame_anchored(&self.queue,transform,self.anchor(),self.size[1],self.density);
            let bytes=crate::surface_view::bytes(transform,[0.;4],self.size,self.density);self.queue.write_buffer(&self.uniform,0,&bytes);
            let tables={let mut tiles=self.tiles.borrow_mut();tiles.prepare(&self.device,&self.queue,self.size);let mut encoder=self.device.create_command_encoder(&Default::default());tiles.encode(&self.device,&mut encoder,&self.view_group,transform);self.queue.submit([encoder.finish()]);tiles.map_report();crate::finite_ink::Tables{projected:tiles.projected.clone(),tiles:tiles.buffer.clone(),primitive:self.primitive.view.clone()}};
            self.picker.submit_finite(&self.device, &self.queue, request,
            &self.meshes, &[&self.mesh_ink, &self.strokes, &self.paths, &self.markers], &self.faces,self.text.borrow().as_ref(),self.clouds.borrow().as_ref(),Some(&self.sheets),Some(&tables),&clipping.group,&clipping.buffer,Some(&mut self.caps.borrow_mut()),Some(&self.instances)); }
    }

    pub fn take_pick(&mut self, transform: [f32; 16]) -> Option<Result<Option<[u32; 2]>, &'static str>> {
        let view = match crate::pick_view::View::anchored(transform,self.size,self.density,self.anchor()) { Ok(view) => view, Err(error) => {
            self.cancel_pick(); return Some(Err(error));
        } };
        match self.picker.take(view) {
            Some(Ok(crate::pick_gpu::Reply::Hit(hit)))=>{self.pick_cursor=None;Some(Ok(hit))},
            Some(Ok(crate::pick_gpu::Reply::Stale))=>{
                if let Some((screen,_))=self.pick_cursor.filter(|(_,origin)|*origin!=self.anchor().map(f64::to_bits)) {
                    if let Err(error)=self.request_pick(screen,transform){self.cancel_pick();return Some(Err(error));}
                } else {
                    self.pump_pick(transform);
                    if self.picker.usage()[0]==0&&!self.picker.queued(){self.pick_cursor=None;}
                }
                None
            },
            Some(Err(error))=>{self.cancel_pick();Some(Err(error))},None=>None,
        }
    }

    pub fn set_pick_mode(&mut self, mode: crate::pick_sources::Mode) {
        if self.pick_mode != mode { self.cancel_pick(); self.pick_mode = mode; }
    }

    pub fn resolve_pick(&self, scene: &Scene, pixel: [u32; 2]) -> Option<crate::pick_sources::Target> {
        self.sources.resolve(scene, pixel, self.pick_mode)
    }

    pub fn part_usage(&self) -> [u64; 4] { let [count, bytes] = self.mesh_ink.usage(); [count, bytes, self.faces.bytes(), self.sources.bytes() as u64] }

    pub fn cancel_pick(&mut self) {self.pick_cursor=None;self.picker.cancel();}

    pub fn pick_usage(&self) -> [u64; 5] {
        let [active, pending, bytes] = self.picker.usage();
        [active, pending, bytes + self.strokes.pick_bytes() + self.paths.pick_bytes() + self.markers.pick_bytes() + self.mesh_ink.pick_bytes() + self.mesh_ink.usage()[1] + self.faces.bytes(),
            self.picker.submitted, self.picker.coalesced]
    }

    pub fn geometry_owners(&self) -> impl Iterator<Item = &std::rc::Rc<crate::gpu_geometry::GpuGeometry>> {
        self.meshes.iter().map(|row| &row.geometry)
    }

    pub fn usage(&self)->[u64;4]{let mut usage=crate::memory::gpu(&self.meshes);usage[1]+=self.geometry.allocated_bytes();usage}
    pub fn arena_cpu_bytes(&self)->usize{self.geometry.cpu_bytes()}

    pub fn uploaded_bytes(&self) -> u64 {
        self.instances.written_bytes + self.geometry.uploaded_bytes + self.settings_allocations as u64 * crate::object_record::BYTES as u64 + self.settings_bytes as u64
            + self.strokes.uploaded_bytes + self.paths.uploaded_bytes + self.markers.uploaded_bytes + self.grid.uploaded_bytes
    }

    pub fn set_grid(&mut self,settings:Option<crate::grid::Settings>)->Result<(),&'static str>{self.set_grid_on(settings,crate::cplane::CPlane::Xy)}
    pub fn set_grid_on(&mut self,settings:Option<crate::grid::Settings>,plane:crate::cplane::CPlane)->Result<(),&'static str>{
        let anchor=self.anchor();
        if settings==self.grid_settings&&plane==self.grid_plane&&anchor==self.grid_anchor{return Ok(())}
        let strokes=settings.map(|s|s.strokes_on_at(plane,anchor)).transpose()?.unwrap_or_default();
        self.grid.set(&self.device,strokes)?;self.grid_settings=settings;self.grid_plane=plane;self.grid_anchor=anchor;Ok(())
    }

    pub fn grid_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.grid.usage(); [count, bytes, self.grid.uploads, self.grid.uploaded_bytes]
    }

    pub fn set_highlight(&mut self, scene: &Scene, target: Option<crate::pick_sources::Target>) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        let anchor=self.anchor();self.highlight.set_at(&self.device,&self.pipeline.get_bind_group_layout(1),scene,target,anchor)
    }

    pub fn select_objects(&mut self, scene: &Scene, ids: &[ObjectId]) -> Result<(), &'static str> {
        let anchor=self.anchor();
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        for mesh in &mut self.meshes { if let Some(row) = scene.objects().iter().find(|r| r.id == mesh.id) {
            let [calls, bytes] = mesh.update(&self.queue, row,style(scene,row.id,ids.contains(&row.id)));
            self.settings_writes += calls; self.settings_bytes += bytes;
        } }
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,false)?;
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        self.set_clouds(scene,ids)?;self.sheets.set_anchored(&self.device,&self.queue,scene,ids,&self.sources.sheet_bases,anchor)?;
        if !ids.is_empty() { self.highlight.objects_at(&self.device,scene,ids,anchor)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
    }

    pub fn set_guides(&mut self,overlay:&crate::guide_gpu::Overlay)->Result<(),&'static str>{if self.guides.set(&self.device,overlay)?{self.set_scene_measure_text().map_err(|_|"Cannot prepare editing guide label")?}Ok(())}
    pub fn guide_bytes(&self)->u64{self.guides.bytes()}
    pub fn set_measurement(&mut self,mark:Option<&crate::measure_mark::Mark>)->Result<(),&'static str>{let anchor=self.anchor();if self.measurement.set_at(&self.device,mark,anchor)?{self.set_scene_measure_text().map_err(|_|"Cannot prepare measurement annotation")?;}Ok(())}
    pub fn measurement_bytes(&self)->u64{self.measurement.bytes()}
    fn merge_measurement(&self,mut labels:Vec<crate::text::TextLabel>)->Vec<crate::text::TextLabel>{if let Some(mark)=&self.measurement.mark{labels.extend(mark.labels(&labels));}labels.extend(self.guides.overlay.labels(&labels));labels}
    fn set_scene_measure_text(&self)->anyhow::Result<()>{let mut labels=self.scene_text.borrow().clone();labels.extend(self.presentation_text.borrow().clone());self.adopt_text(self.merge_measurement(labels))}
    pub fn set_preview(&mut self,draft:Option<&crate::tool::Draft>)->Result<(),&'static str>{let anchor=self.anchor();self.preview.set_at(&self.device,draft,anchor)}
    pub fn preview_bytes(&self) -> u64 { self.preview.bytes() }

    pub fn set_gizmo(&mut self, camera:&crate::camera::Camera, origin:Option<[f64;3]>, active:f32) {
        let anchor=self.anchor();let matrix=camera.view_projection_anchored(anchor);
        let layout=origin.map(|p|std::array::from_fn(|i|p[i]-anchor[i])).and_then(|p|crate::widget_layout::world_per_pixel(&matrix,p,self.size[1] as f64/self.density as f64)
            .and_then(|scale|crate::widget_layout::layout(&matrix,p,scale,self.size,active)));
        self.widget.prepare(&self.device,&self.queue,layout);
    }
    pub fn gizmo_usage(&self) -> [u64;4] { self.widget.usage() }
    pub fn gizmo_extent(&self) -> Option<[u32;2]> {self.widget.extent()}

    fn text_context(&self)->crate::text_gpu_support::GpuCtx {crate::text_gpu_support::GpuCtx::new(&self.device,&self.queue)}
    fn text_frame(&self,transform:[f32;16])->crate::scene_text_gpu::TextFrame {crate::scene_text_gpu::TextFrame {mvp:transform,origin:self.anchor(),framebuffer:self.size,logical:[self.size[0] as f64/self.density as f64,self.size[1] as f64/self.density as f64],ortho_half_height:0.,clip:{let clip=self.clipping.borrow();let mut planes=[[0.;4];6];if clip.enabled {for (p,row) in planes.iter_mut().zip(&clip.rows){*p=[row.plane.normal[0],row.plane.normal[1],row.plane.normal[2],row.plane.offset];}}planes}}}
    fn adopt_text(&self,labels:Vec<crate::text::TextLabel>)->anyhow::Result<()> {
        if labels.is_empty() {self.text.borrow_mut().take();self.text_error.borrow_mut().take();return Ok(());}
        let mut lane=self.text.borrow_mut();if lane.is_none() {let mut new=crate::scene_text_gpu::Lane::new(&self.text_context(),crate::text_gpu_support::Target {format:self.format,samples:1});if let Some(fonts)=self.full_text_fonts.borrow().as_ref() {new.document.replace_fonts(fonts.clone())?;}*lane=Some(new);}lane.as_mut().unwrap().set_labels(labels)
    }
    pub fn set_text(&self,mut labels:Vec<crate::text::TextLabel>)->anyhow::Result<()> {
        for (i,label) in labels.iter_mut().enumerate() {label.id=u32::MAX-i as u32;label.object=None;}
        let mut merged=self.scene_text.borrow().clone();merged.extend(labels.clone());self.adopt_text(self.merge_measurement(merged))?;*self.presentation_text.borrow_mut()=labels;Ok(())
    }
    fn set_scene_text(&self,scene:&Scene,selected:&[ObjectId])->anyhow::Result<()> {
        let labels=scene.text_labels(selected).map_err(anyhow::Error::msg)?;let mut merged=labels.clone();merged.extend(self.presentation_text.borrow().clone());self.adopt_text(self.merge_measurement(merged))?;*self.scene_text.borrow_mut()=labels;Ok(())
    }
    pub fn refresh_text_bounds(&self,scene:&mut Scene)->bool {
        let mut changed=false;
        if let Some(lane)=self.text.borrow().as_ref() {for row in scene.texts_mut() {if let Some(run)=lane.document.runs.iter().find(|run|run.label.id==row.id.value()) {
            let extent=run.buffer.layout_runs().fold([0f64;2],|[w,h],line|[w.max(line.line_w as f64),h.max((line.line_top+line.line_height) as f64)]);
            if row.prepared.extent!=extent {row.prepared.extent=extent;changed=true;}
        }}}
        if changed {*self.world_bounds.borrow_mut()=scene.bounds();}
        changed
    }
    pub fn text_stats(&self)->Option<crate::scene_text_gpu::TextStats> {self.text.borrow().as_ref().map(|lane|lane.stats.clone())}
    pub fn text_scene_usage(&self)->[u64;4] {self.text.borrow().as_ref().map_or([0;4],|lane|[lane.allocated_bytes(),lane.texture_bytes(),lane.stats.world_plane_rasterizations,lane.document.runs.len() as u64])}
    pub fn replace_text_fonts(&self,fonts:Vec<Vec<u8>>)->anyhow::Result<()> {if let Some(lane)=self.text.borrow_mut().as_mut() {lane.document.replace_fonts(fonts.clone())?;} else {let mut document=crate::text::TextDocument::new();document.replace_fonts(fonts.clone())?;}*self.full_text_fonts.borrow_mut()=Some(fonts);Ok(())}
    pub fn clear_text_fonts(&self) {self.full_text_fonts.borrow_mut().take();}

    pub fn text_error(&self)->Option<String> {self.text_error.borrow().clone()}
    pub fn text_usage(&self)->[u64;6] {self.text.borrow().as_ref().map_or([0;6],|lane|{[lane.document.shape_count,lane.stats.preparations,lane.stats.skipped_preparations,lane.stats.raster_images as u64,lane.stats.raster_image_capacity_bytes as u64,lane.stats.atlas_resets]})}
    pub fn text_demo(&self)->anyhow::Result<()> {self.set_text(vec![crate::text_example::label()])}

    pub fn cloud_camera(&self,camera:&crate::camera::Camera) {self.cloud_view.set(crate::cloud_gpu::View::anchored(camera,self.anchor()));self.sheets.camera(camera);}
    pub fn cloud_usage(&self)->[u64;5] {self.clouds.borrow().as_ref().map_or([0;5],|lane|lane.usage())}
    pub fn cloud_error(&self)->Option<String> {self.cloud_error.borrow().clone()}
    fn set_clouds(&self,scene:&Scene,selected:&[ObjectId])->Result<(),&'static str> {
        if scene.clouds().is_empty()&&scene.streams().is_empty() {self.clouds.borrow_mut().take();self.cloud_error.borrow_mut().take();return Ok(());}
        let mut owner=self.clouds.borrow_mut();let lane=owner.get_or_insert_with(||crate::cloud_gpu::Lane::new(&self.text_context(),self.format,&self.clipping.borrow().group));lane.set_scene(&self.text_context(),scene,selected)
    }
    fn prepare_clouds(&self,transform:[f32;16]) {
        if let Some(lane)=self.clouds.borrow_mut().as_mut() {lane.set_clipping(self.clipping.borrow().writes);if let Err(error)=lane.prepare(&self.text_context(),transform,self.size,self.density,self.cloud_view.get()) {lane.failed();*self.cloud_error.borrow_mut()=Some(error.to_string());} else {self.cloud_error.borrow_mut().take();}}
    }
    pub fn set_opacity(&mut self,value:f32){if !value.is_finite(){return;}let value=value.clamp(0.,1.);if self.opacity.get()==value{return;}self.opacity.set(value);self.cancel_pick();self.pyramid_revision=self.pyramid_revision.wrapping_add(1);for mesh in &mut self.meshes{mesh.set_xray(&self.queue,value==0.);}if let Some(o)=self.outline.borrow_mut().as_mut(){o.invalidate();}if let Some(o)=self.global_outline.borrow_mut().as_mut(){o.invalidate();}}
    pub fn opacity(&self)->f32{self.opacity.get()}
    pub fn set_outlines(&self,on:bool){self.outlines_enabled.set(on);if !on{self.global_outline.borrow_mut().take();self.outline_composite.borrow_mut().take();}}
    pub fn global_outline_usage(&self)->[u64;4]{self.global_outline.borrow().as_ref().map_or([0;4],|o|o.usage())}
    pub fn outline_usage(&self)->[u64;4] {self.outline.borrow().as_ref().map_or([0;4],|outline|outline.usage())}

    pub fn highlight_bytes(&self) -> u64 { self.highlight.bytes() }

    pub fn sheet_camera(&self,camera:&crate::camera::Camera) {self.sheets.camera(camera);}
    #[cfg(test)]
    pub(crate) fn tile_buffers(&self)->[wgpu::Buffer;2]{let tiles=self.tiles.borrow();[tiles.projected.clone(),tiles.buffer.clone()]}
    pub fn set_clipping(&self,enabled:bool,solid:bool){let mut clip=self.clipping.borrow_mut();clip.enabled=enabled;clip.solid=solid;self.caps.borrow_mut().enabled=enabled;}
    pub fn set_lighting(&self,on:bool,backfaces:bool){self.surface_options.set([on,backfaces]);}
    pub fn set_ambient(&self,enabled:bool){if self.ambient_enabled.replace(enabled)==enabled{return;}let mut ambient=self.ambient.borrow_mut();if enabled{let owner=ambient.get_or_insert_with(||crate::ambient_gpu::Ambient::new(&self.device,self.format));owner.sources.set(&self.device,&self.queue,&self.ambient_rows.specs);}else if let Some(owner)=ambient.as_mut(){owner.clear(&self.device,&self.queue);}}
    pub fn ambient_usage(&self)->[u64;5]{self.ambient.borrow().as_ref().map_or([0;5],crate::ambient_gpu::Ambient::usage)}
    pub fn ambient_error(&self)->Option<&'static str>{self.ambient.borrow().as_ref().and_then(|owner|owner.error)}
    pub fn set_depth_pyramid(&self,enabled:bool){let mut owner=self.pyramid.borrow_mut();if enabled {if owner.is_none(){*owner=Some(crate::pyramid_gpu::Pyramid::new(&self.device));}}else{*owner=None;}}
    pub fn depth_pyramid_usage(&self)->[u64;4]{self.pyramid.borrow().as_ref().map_or([0;4],crate::pyramid_gpu::Pyramid::usage)}
    pub fn depth_pyramid_error(&self)->Option<&'static str>{self.pyramid.borrow().as_ref().and_then(|p|p.error)}
    pub fn cap_error(&self)->Option<&'static str>{self.caps.borrow().pick_error}
    pub fn cap_usage(&self)->[u64;5]{self.caps.borrow().usage()}
    pub fn clipping_usage(&self)->[u64;5]{let clip=self.clipping.borrow();[clip.rows.len()as u64,u64::from(clip.enabled),u64::from(clip.solid),clip.writes,clip.buffer.size()]}
    pub fn clipping_ink_bytes(&self)->[u64;2]{[&self.mesh_ink,&self.strokes,&self.paths,&self.markers,&self.grid].into_iter().map(crate::stroke_gpu::Lane::clipping_bytes).fold(self.highlight.clipping_bytes(),|a,b|[a[0]+b[0],a[1]+b[1]])}
    pub fn clipping_error(&self)->Option<&'static str>{self.clipping.borrow().error}
    pub fn tile_usage(&self)->[u64;7]{self.tiles.borrow().usage()}
    pub fn sheet_usage(&self)->[u64;5] {self.sheets.usage()}
    pub fn instance_usage(&self)->[u64;5] {self.instances.usage()}
    pub fn instance_allocations(&self)->[u64;3] {self.instances.allocation_parts()}
    pub fn stats(&self) -> [usize; 4] {
        [self.geometry.uploads, self.settings_allocations, self.settings_writes, self.settings_bytes]
    }

    pub fn draw(
        &self,
        view: &wgpu::TextureView,
        background: &crate::background::Background,
        transform: &[f32; 16],
    ) { self.draw_mode(view, background, transform, false); }

    pub fn draw_mode(&self, view: &wgpu::TextureView, background: &crate::background::Background,
        transform: &[f32; 16], normals: bool,
    ) {
        let (clip_group,clip_revision)={let mut clip=self.clipping.borrow_mut();clip.frame_anchored(&self.queue,*transform,self.anchor(),self.size[1],self.density);(clip.group.clone(),clip.writes)};
        self.caps.borrow_mut().prepare(&self.device,&self.queue,self.format,self.size,{let clip=self.clipping.borrow();clip.enabled&&!clip.rows.is_empty()});
        self.prepare_clouds(*transform);
        let options=self.surface_options.get();
        let display=[f32::from(normals),self.opacity.get(),if self.ambient_enabled.get(){2.}else{f32::from(options[0])},f32::from(options[1])];
        let bytes=crate::surface_view::bytes(*transform,display,self.size,self.density);
        self.queue.write_buffer(&self.uniform, 0, &bytes);
        self.sheets.view(&self.queue,transform,self.size,self.density);
        let [r, g, b] = if self.ambient_enabled.get() { [0.94; 3] } else { background.rgb() };
        let mut encoder = self.device.create_command_encoder(&Default::default());
        let mut tiles=self.tiles.borrow_mut();tiles.prepare(&self.device,&self.queue,self.size);tiles.encode(&self.device,&mut encoder,&self.view_group,*transform);
        let tables=crate::finite_ink::Tables{projected:tiles.projected.clone(),tiles:tiles.buffer.clone(),primitive:self.primitive.view.clone()};for lane in [&self.mesh_ink,&self.strokes,&self.paths,&self.markers]{lane.finite_frame(&self.device,&tables,&self.depth);}self.highlight.finite_frame(&self.device,&tables,&self.depth);
        {
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("canvas"),
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(crate::depth::FAR as f32),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r, g, b, a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                }),Some(wgpu::RenderPassColorAttachment{
                    view:&self.primitive.view,depth_slice:None,resolve_target:None,
                    ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store},
                })],
                ..Default::default()
            });
            self.grid.draw(&mut pass,&self.view_group);
            pass.set_bind_group(2,&clip_group,&[]);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            for mesh in self.meshes.iter().filter(|row|self.opacity.get()<1.||!self.instance_rows.contains(&row.id)) {if !self.instances.draw_ordinary(&mut pass,mesh){pass.set_pipeline(&self.pipeline);self.instances.bind_own(&mut pass);mesh.draw_pulled(&mut pass);}}
            if self.opacity.get()>=1.{self.instances.draw(&mut pass);}
            pass.set_pipeline(&self.highlight_pipeline); pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            self.highlight.mesh(&mut pass);
        }
        self.caps.borrow_mut().draw(&self.device,&mut encoder,view,&self.primitive.view,&self.depth,&self.view_group,&clip_group,&self.meshes,if self.clipping.borrow().enabled{self.clipping.borrow().rows.len()as u32}else{0},Some(&self.instances));
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("cloud depth composite"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});if let Some(clouds)=self.clouds.borrow().as_ref(){clouds.draw(&mut pass);}}
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite scene ink"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:None,stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});
            pass.set_bind_group(2,&clip_group,&[]);
            self.highlight.ink(&mut pass,&self.view_group);
            self.mesh_ink.draw(&mut pass,&self.view_group);
            self.strokes.draw(&mut pass,&self.view_group);self.sheets.draw(&mut pass);
            self.paths.draw(&mut pass,&self.view_group);
            self.markers.draw(&mut pass,&self.view_group);
            self.highlight.points(&mut pass,&self.view_group);
            self.preview.draw(&mut pass,&self.view_group);
        }
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("measurement overlay"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});self.measurement.draw(&mut pass,&self.view_group);self.guides.draw(&mut pass,&self.view_group);}
        if self.outlines_enabled.get()&&!self.meshes.is_empty(){
            let ctx=self.text_context();
            let mut global=self.global_outline.borrow_mut();let global=global.get_or_insert_with(||crate::outline_gpu::Outline::new(&ctx,self.format));
            global.encode_masks(&ctx,&mut encoder,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision,true,Some(&self.primitive.view));
            let mut selected=self.outline.borrow_mut();
            if self.meshes.iter().any(GpuMesh::selected){let selected=selected.get_or_insert_with(||crate::outline_gpu::Outline::new(&ctx,self.format));selected.encode_masks(&ctx,&mut encoder,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision,false,Some(&self.primitive.view));}else{selected.take();}
            if let Some(alpha)=global.alpha_view(){self.outline_composite.borrow_mut().get_or_insert_with(||crate::outline_composite::Composite::new(&self.device,self.format)).draw(&self.device,&mut encoder,view,alpha,selected.as_ref().and_then(|s|s.alpha_view()));}
        }else{
            self.global_outline.borrow_mut().take();self.outline_composite.borrow_mut().take();
            if self.meshes.iter().any(GpuMesh::selected){let mut outline=self.outline.borrow_mut();let outline=outline.get_or_insert_with(||crate::outline_gpu::Outline::new(&self.text_context(),self.format));outline.encode(&self.text_context(),&mut encoder,view,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision);}else{self.outline.borrow_mut().take();}
        }
        self.widget.draw(&mut encoder,view);
        self.text_error.borrow_mut().take();
        if let Some(lane)=self.text.borrow_mut().as_mut() {
            match lane.prepare(&self.text_context(),&self.text_frame(*transform)) {
            Ok(())=>{
                let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("scene glyphs"),depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment {view:&self.depth,depth_ops:Some(wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),color_attachments:&[Some(wgpu::RenderPassColorAttachment {view,depth_slice:None,resolve_target:None,ops:wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],..Default::default()});if let Err(error)=lane.draw(&mut pass) {*self.text_error.borrow_mut()=Some(error.to_string());}
            }
            Err(error)=>{*self.text_error.borrow_mut()=Some(error.to_string());}
            }
        }
        if self.ambient_enabled.get(){let clip=self.clipping.borrow();if let Some(ambient)=self.ambient.borrow_mut().as_mut(){ambient.draw(&self.device,&self.queue,&mut encoder,view,&self.depth,&self.view_group,&clip_group,&clip.buffer,&self.primitive.view,&self.meshes,*transform,self.size,self.density,self.ambient_rows.ground_at(self.anchor()),[self.pyramid_revision,clip_revision],Some(&self.instances));}}
        if let Some(pyramid)=self.pyramid.borrow_mut().as_mut(){pyramid.encode(&self.device,&self.queue,&mut encoder,&self.depth,*transform,self.size,self.density,[self.pyramid_revision,clip_revision]);}
        self.queue.submit([encoder.finish()]);tiles.map_report();
    }
}

fn style(scene:&Scene,id:ObjectId,selected:bool)->crate::gpu_mesh::Style {crate::gpu_mesh::Style {source_faces:!scene.forest.derived(id),selected,owner:crate::feature_owner::display_owner(scene,id).map_or(0,ObjectId::value),colour:scene.face_rgba(id)}}

fn rgba(rgb:[u8;3])->[f32;4] {[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]}

#[cfg(all(test,not(target_arch="wasm32")))]
impl Renderer {
 pub fn outline_alpha(&self)->Vec<u8>{self.read_outline_alpha(false)}
 pub fn global_outline_alpha(&self)->Vec<u8>{self.read_outline_alpha(true)}
 fn read_outline_alpha(&self,global:bool)->Vec<u8> {
  let owner=if global{self.global_outline.borrow()}else{self.outline.borrow()};let Some(outline)=owner.as_ref() else {return Vec::new();};let Some(texture)=outline.alpha_texture() else {return Vec::new();};let [width,height]=self.size;let pitch=width.div_ceil(256)*256;
  let buffer=self.device.create_buffer(&wgpu::BufferDescriptor {label:Some("actual outline alpha readback"),size:pitch as u64*height as u64,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::MAP_READ,mapped_at_creation:false});let mut encoder=self.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo {buffer:&buffer,layout:wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},wgpu::Extent3d {width,height,depth_or_array_layers:1});self.queue.submit([encoder.finish()]);let (tx,rx)=std::sync::mpsc::channel();buffer.slice(..).map_async(wgpu::MapMode::Read,move|r|tx.send(r).unwrap());self.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();rx.recv().unwrap().unwrap();let bytes=buffer.slice(..).get_mapped_range();let pixels=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize].iter().copied()).collect();drop(bytes);buffer.unmap();pixels
 }
}

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="pyramid_gpu_tests.rs"]
mod pyramid_gpu_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="ambient_gpu_tests.rs"]
mod ambient_gpu_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="global_outline_tests.rs"]
mod global_outline_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="opacity_gpu_tests.rs"]
mod opacity_gpu_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="camera_extent_tests.rs"]
mod camera_extent_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="surface_lighting_tests.rs"]
mod surface_lighting_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="surface_controls_gpu_tests.rs"]
mod surface_controls_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="shared_view_tests.rs"]
mod shared_view_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="contact_feature_tests.rs"]
mod contact_feature_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="ink_metadata_tests.rs"]
mod ink_metadata_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="contact_boundary_tests.rs"]
mod contact_boundary_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="source_face_policy_tests.rs"]
mod source_face_policy_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="source_pen_tests.rs"]
mod source_pen_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="round_caps_tests.rs"]
mod round_caps_tests;
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-16.rs"
```

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-17.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/mesh_marker_tests.rs`

Exercise actual GPU dots, source keys, authored colors, incidence and shared source unloading.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-18.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 8. `src/renderer.rs`

Register the three actual mesh marker GPU checks.

<details>
<summary>Locate the existing block</summary>

```text
mod round_caps_tests;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-19.rs" skip=21
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 9. `src/source_pen_tests.rs`

Extend the existing real 200000-triangle budget case without duplicating its dense fixture.

<details>
<summary>Locate the existing block</summary>

```text
assert!(scene.objects()[0].mesh.parts.pens.is_empty());
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-20.rs" skip=55
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 10. `src/global_outline_tests.rs`

This existing fixture manually isolates surface-buffer rebasing; suppress both original ink and new default dots while preserving every surface/grid/cap/pick/source assertion. Production complete-lane marker rebasing is separately exercised at a billion units.

<details>
<summary>Locate the existing block</summary>

```text
fn actual_precise_object_rebase_matches_local_pixels_records_selection_and_owners(){
 let mut local=Scene::empty();let owner=local.insert(PreparedMesh::triangle([[-0.2,-0.3,-0.4],[0.2,-0.3,-0.4],[0.,0.3,-0.4]],[0.1,0.3,0.9]).unwrap()).unwrap();local.translate(owner,[0.125,-0.25,0.]).unwrap();let mut gpu=renderer(&local);gpu.set_opacity(1.);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let baseline=pixels(&gpu);assert!(baseline.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(50)).count()>100);let source=local.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 let anchor=[1e12,-1e12,1e12];let mut far=local.clone();far.translate(owner,anchor).unwrap();gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[1,12]);assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[0,0]);gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64);assert_eq!(&bytes[48..60],&[0.125f32,-0.25,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(&bytes[..crate::object_record::BYTES],&gpu.meshes[0].packed());
 gpu.select_objects(&far,&[owner]).unwrap();assert_ne!(pixels(&gpu),baseline);gpu.request_pick([0.125,-0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());gpu.select_objects(&far,&[]).unwrap();assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[0,0]);assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);
 far.translate(owner,[0.001,0.,0.]).unwrap();gpu.set_scene(&far,None);let expected=crate::render_space::relative(far.objects()[0].model.m[12..15].try_into().unwrap(),anchor);assert_eq!(&gpu.meshes[0].packed()[48..60],&expected.into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());assert!(expected[0]>0.125);assert_eq!(gpu.stats()[0],uploads);
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-21.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 11. `src/global_outline_tests.rs`

This existing fixture manually isolates surface-buffer rebasing; suppress both original ink and new default dots while preserving every surface/grid/cap/pick/source assertion. Production complete-lane marker rebasing is separately exercised at a billion units.

<details>
<summary>Locate the existing block</summary>

```text
fn actual_derived_frame_origin_keeps_cap_pixels_world_text_clips_and_precise_ground(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.5,0.5,0.5)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,-0.5);let owner=scene.insert(prepared).unwrap();let plane=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Xy,&[[0.,0.,-0.5]],1.).unwrap();let plane_id=crate::plane_original::insert(&mut scene,Rc::new(plane)).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let baseline=pixels(&gpu);assert!(crate::cap_gpu_tests::at(&baseline,160,120)[..3].iter().all(|v|(150..=156).contains(v)));let ground=gpu.ambient_rows.ground_at([0.;3]);let source=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in [owner,plane_id]{far.translate(id,anchor).unwrap();}gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();assert!(gpu.space.update(anchor,1.,0.).unwrap().moved);for mesh in &mut gpu.meshes {mesh.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(gpu.anchor(),anchor);assert_eq!(gpu.ambient_rows.ground_at(anchor),ground);assert_eq!(gpu.ambient_rows.ground[0]-anchor[2],-0.75);let frame=gpu.text_frame(matrix());assert_eq!(frame.origin,anchor);assert!(!frame.cut([anchor[0],anchor[1],anchor[2]-0.75]));assert!(frame.cut([anchor[0],anchor[1],anchor[2]-0.25]));assert_eq!(pixels(&gpu),baseline);let writes=gpu.clipping.borrow().writes;assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.clipping.borrow().writes,writes);assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-22.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 12. `src/global_outline_tests.rs`

This existing fixture manually isolates surface-buffer rebasing; suppress both original ink and new default dots while preserving every surface/grid/cap/pick/source assertion. Production complete-lane marker rebasing is separately exercised at a billion units.

<details>
<summary>Locate the existing block</summary>

```text
fn actual_source_grid_origin_cache_keeps_surface_pixels_owners_and_release(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.25,-0.25,-0.5],[0.25,-0.25,-0.5],[0.,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let source=scene.objects()[0].geometry().unwrap().to_proto();let mut gpu=renderer(&scene);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let settings=Some(crate::grid::Settings{spacing:0.1,half_cells:5});let plane=|origin|crate::cplane::CPlane::Frame{origin,x:[1.,0.,0.],y:[0.,1.,0.]};gpu.set_grid_on(settings,plane([0.,0.,-0.75])).unwrap();let mut camera=crate::camera_defaults_tests::top_fixture();controlled_translation_plane(&mut camera,[0.;3]);camera.projection=crate::camera::Projection::Orthographic;camera.distance=2.;camera.aspect=4./3.;let transform=camera.uniform();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);let uploads=gpu.stats()[0];let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();far.translate(owner,anchor).unwrap();gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();gpu.space.update(anchor,2.,0.).unwrap();for mesh in &mut gpu.meshes{mesh.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));let far_plane=plane([anchor[0],anchor[1],anchor[2]-0.75]);gpu.set_grid_on(settings,far_plane).unwrap();camera.target=anchor;controlled_translation_plane(&mut camera,anchor);let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);let grid_uploads=gpu.grid_usage()[2];gpu.set_grid_on(settings,far_plane).unwrap();assert_eq!(gpu.grid_usage()[2],grid_uploads);
 let moved=[anchor[0]+2000.,anchor[1],anchor[2]];gpu.space.update(moved,2.,300.).unwrap();camera.target=moved;for mesh in &mut gpu.meshes{mesh.rebase(&gpu.queue,moved);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.set_grid_on(settings,far_plane).unwrap();assert!(gpu.grid_usage()[2]>grid_uploads);assert_eq!(camera.view_projection_anchored(moved).to_f32(),local);assert!(crate::cap_gpu_tests::pixels_at(&gpu,local).chunks_exact(4).all(|p|p==&[255;4]));gpu.space.update(anchor,2.,600.).unwrap();for mesh in &mut gpu.meshes{mesh.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.set_grid_on(settings,far_plane).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);gpu.request_pick([0.,0.],local).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(local).unwrap().unwrap().unwrap()[0],owner.value());assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);gpu.set_grid(None).unwrap();gpu.set_scene(&Scene::empty(),None);assert_eq!(&gpu.grid_usage()[..2],&[0,0]);assert_eq!(gpu.usage(),[0;4]);
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-23.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 13. `src/scene_origin_tests.rs`

Keep the original point/control count exact after the new eight default Box vertex markers; verify all marker values, owners, ordering and large-origin equality independently.

<details>
<summary>Locate the existing block</summary>

```text
assert_eq!(current.marker_ids.len(),far.points().len()+far.control_markers_at(anchor).len());
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-24.rs"
```

</details>

</details>

<a id="checkpoint-96zzzzzzq-taper-crowded-original-mesh-vertices"></a>

### Step 4 of 4 · Taper crowded original mesh vertices (`96zzzzzzq-taper-crowded-original-mesh-vertices`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 4/4 Visual Trace (96zzzzzzq-taper-crowded-original-mesh-vertices): Taper crowded original mesh vertices</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/mesh.rs, src/marker.rs, src/pick_sources.rs, src/stroke_gpu.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">Render bounds and source</text>
    <text x="111" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">vertex…</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">one shared spacing scalar</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">signed GPU size</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">common radius and query</text>
    <text x="726" y="81" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">coverage</text>
    
</svg>
</div>

- **Goal:** Keep dense source vertices distinguishable while retaining explicit point sizes, editing controls and original ownership.
- **Data Flow Trace:** `Render bounds and source vertex count → one shared spacing scalar → signed GPU size → common radius and query coverage`
- **Files You Touch:** `src/mesh.rs` (*Retain one prepared mesh spacing scalar after original source release.*) · `src/marker.rs` (*Name the render-only size mode explicitly; original point widths and editing-control diameters remain unchanged.*) · `src/pick_sources.rs` (*Reuse the existing32byte point instance, endpoint addresses and upload; no distance array, extra metadata or buffer.*) · `src/stroke_gpu.rs` (*Accept the explicit finite mesh-spacing size mode while rejecting zero and nonfinite display data.*) · `src/marker_radius.wgsl` (*Project one instance spacing at each vertex depth; retain the actual three-diameter taper and half-physical-pixel minimum.*) · `src/finite_project_source.rs` (*Expand one shared radius helper for both color/pick and existing finite/legacy paths.*) · `src/marker.wgsl` (*Use the same radius rule in the existing marker paths.*) · `src/finite_marker.wgsl` (*Use the same radius rule in the existing marker paths.*) · `src/mesh_density_tests.rs` (*Exercise actual dense physical footprints, explicit point/control independence, correct rear-edge visibility, source release and shared geometry.*) · `src/renderer.rs` (*Register the three actual source-bound density GPU checks.*) · `src/marker_coverage.wgsl` (*Measure the circle at the actual physical fragment pixel relative to its fixed centre; keep one shared colour/query coverage function.*) · `src/marker_pick.wgsl` (*Carry a fixed screen centre instead of interpolating the expanded quad offset.*) · `src/finite_marker_pick.wgsl` (*Reuse the existing fixed axis centre; drop the redundant interpolated offset.*)

Dense original mesh dots overlap until the blue face becomes an opaque dark sheet. Estimate spacing once, then taper each default dot at its projected depth while keeping a half-physical-pixel minimum.

Reuse the existing 32-byte point layout. Positive GPU sizes retain explicit CSS diameters; negative values carry prepared mesh spacing in millimetres. A named constructor makes this render-only mode explicit, and original source widths remain unchanged. One helper serves both existing finite and legacy color/query paths. The original mesh owner retains one scalar after source release; no per-vertex distance array, extra buffer or neighbor-normal table is added.

Measure each disc from the fragment’s physical pixel and a fixed screen centre. Interpolating a quad offset shifts dense-dot coverage with rasterizer rounding. Crop queries must use the same pixel origin as colour.

The half-pixel floor can still crowd a distant mesh. The readable and extreme views show that limit; the taper does not remove it.

> **Self-Check Question:** *Why apply the half-physical-pixel minimum after the crowding taper?*  
> **Answer:** Crowded default dots must become smaller before the lower bound is applied. This preserves a visible footprint without keeping an unnecessarily large radius at a higher pixel density.

- **What You Should See:** Crowding-aware dots retain more of the face at readable zoom levels. At extreme zoom-out, the half-physical-pixel minimum can still overlap. Explicit point sizes, active controls, original ownership and shared geometry remain stable.
- **Hands-On Lab Experiment:** Compare the actual crowded original grid with its preceding dark-sheet render and the maintained taper reference. Inspect both projections and densities, explicit points and controls, source unload, camera changes and shared mirrored placement. Compare the readable and extreme-zoom views: the minimum physical footprint intentionally limits further thinning.

<details>
<summary>Implementation 96zzzzzzq-taper-crowded-original-mesh-vertices: Taper crowded original mesh vertices</summary>

Keep dense source vertices distinguishable while retaining explicit point sizes, editing controls and original ownership.

Data flow: Render bounds and source vertex count → one shared spacing scalar → signed GPU size → common radius and query coverage.

### 1. `src/mesh.rs`

Retain one prepared mesh spacing scalar after original source release. Only actual original default-dot meshes need a spacing estimate. Use actual render bounds and original vertex count once, matching the maintained per-mesh estimate; skip dense/printed fills without dots.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
        }
        let normals = vec![[0.0; 3]; vertices.len()];
        let alphas = vec![1.; vertices.len()];
        Ok(Self { vertices, normals, alphas, indices, parts: Default::default() })
    }

    pub fn from_kernel(mesh: &session_rust::Mesh) -> Result<Self, &'static str> {
        let render = mesh.to_render();
        let parts = crate::mesh_parts::Parts::new(mesh, render.indices.len() / 3)?;
        let mut display = Self::from_render(render)?;
        display.parts = parts; Ok(display)
    }
```

</details>

Replace it with:

```rust
        }
        let normals = vec![[0.0; 3]; vertices.len()];
        let alphas = vec![1.; vertices.len()];
        Ok(Self { vertices, normals, alphas, indices, spacing:0.,parts: Default::default() })
    }

    pub fn from_kernel(mesh: &session_rust::Mesh) -> Result<Self, &'static str> {
        let render = mesh.to_render();
        let parts = crate::mesh_parts::Parts::new(mesh, render.indices.len() / 3)?;
        let mut display = Self::from_render(render)?;
        if !parts.dots.is_empty(){
            let mut b=session_rust::AABB::empty();for p in display.vertices(){b.union_with_point(p[0]as f64,p[1]as f64,p[2]as f64);}
            display.spacing=(b.diagonal().min(f32::MAX as f64)as f32)/(mesh.number_of_vertices()as f32).sqrt();
        }
        display.parts = parts; Ok(display)
    }
```

<details>
<summary>Locate this block</summary>

```rust
    normals: Vec<[f32; 3]>,
    alphas: Vec<f32>,
    indices: Vec<u32>,
    pub parts: crate::mesh_parts::Parts,
}
```

</details>

Replace it with:

```rust
    normals: Vec<[f32; 3]>,
    alphas: Vec<f32>,
    indices: Vec<u32>,
    pub spacing:f32,
    pub parts: crate::mesh_parts::Parts,
}
```

<details>
<summary>Complete updated block reference</summary>

```rust
#[derive(Clone)]
pub struct Mesh {
    vertices: Vec<[f32; 6]>,
    normals: Vec<[f32; 3]>,
    alphas: Vec<f32>,
    indices: Vec<u32>,
    pub spacing:f32,
    pub parts: crate::mesh_parts::Parts,
}

impl Mesh {
    pub fn new(vertices: Vec<[f32; 6]>, indices: Vec<u32>) -> Result<Self, &'static str> {
        if indices.len() % 3 != 0 || indices.len() > u32::MAX as usize {
            return Err("Indices must describe complete triangles within the draw limit");
        }
        if indices.iter().any(|&index| index as usize >= vertices.len()) {
            return Err("A triangle refers to a missing vertex");
        }
        if vertices.iter().flatten().any(|value| !value.is_finite()) {
            return Err("Vertex values must be finite");
        }
        let normals = vec![[0.0; 3]; vertices.len()];
        let alphas = vec![1.; vertices.len()];
        Ok(Self { vertices, normals, alphas, indices, spacing:0.,parts: Default::default() })
    }

    pub fn from_kernel(mesh: &session_rust::Mesh) -> Result<Self, &'static str> {
        let render = mesh.to_render();
        let parts = crate::mesh_parts::Parts::new(mesh, render.indices.len() / 3)?;
        let mut display = Self::from_render(render)?;
        if !parts.dots.is_empty(){
            let mut b=session_rust::AABB::empty();for p in display.vertices(){b.union_with_point(p[0]as f64,p[1]as f64,p[2]as f64);}
            display.spacing=(b.diagonal().min(f32::MAX as f64)as f32)/(mesh.number_of_vertices()as f32).sqrt();
        }
        display.parts = parts; Ok(display)
    }

    pub fn from_render(render:session_rust::render_mesh::RenderMesh)->Result<Self,&'static str>{
        let vertices = render.vertices.iter().map(|vertex| {
            let [x, y, z] = vertex.position;
            let [r, g, b, _] = vertex.color;
            [x, y, z, r, g, b]
        }).collect();
        let normals = render.vertices.iter().map(|vertex| vertex.normal).collect();
        let alphas = render.vertices.iter().map(|vertex| vertex.color[3]).collect();
        Self::new(vertices, render.indices)?.with_normals(normals)?.with_alphas(alphas)
    }

    pub fn with_normals(mut self, normals: Vec<[f32; 3]>) -> Result<Self, &'static str> {
        if normals.len() != self.vertices.len() { return Err("Each display vertex needs one normal"); }
        self.normals = normals.into_iter().map(crate::normals::unit).collect::<Result<_, _>>()?; Ok(self)
    }

    pub fn with_alphas(mut self, alphas:Vec<f32>)->Result<Self,&'static str>{
        if alphas.len()!=self.vertices.len()||alphas.iter().any(|v|!v.is_finite()){
            return Err("Each display vertex needs finite alpha");
        }
        self.alphas=alphas;Ok(self)
    }
    pub fn alphas(&self)->&[f32]{&self.alphas}
    pub fn normals(&self) -> &[[f32; 3]] { &self.normals }

    pub fn payload_bytes(&self) -> usize {
        self.vertices.capacity() * std::mem::size_of::<[f32; 6]>()
            + self.indices.capacity() * std::mem::size_of::<u32>()
            + self.normals.capacity() * std::mem::size_of::<[f32; 3]>()
            + self.alphas.capacity() * std::mem::size_of::<f32>() + self.parts.bytes()
    }

    pub fn vertices(&self) -> &[[f32; 6]] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_kernel_box_becomes_eight_vertices_and_twelve_triangles() {
        let source = session_rust::Mesh::create_box(0.8, 0.8, 0.8);
        let mesh = Mesh::from_kernel(&source).unwrap();
        assert_eq!(mesh.vertices().len(), 8);
        assert_eq!(mesh.indices().len(), 36);
    }

    #[test]
    fn invalid_connections_and_nonfinite_vertices_are_rejected() {
        assert!(Mesh::new(vec![[0.0; 6]; 3], vec![0, 1, 3]).is_err());
        assert!(Mesh::new(vec![[0.0; 6]; 3], vec![0, 1]).is_err());
        assert!(Mesh::new(vec![[f32::NAN; 6]; 3], vec![0, 1, 2]).is_err());
        assert!(Mesh::new(vec![[0.0; 6]; 3], vec![0, 1, 2]).is_ok());
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-01.rs"
```

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-02.rs"
```

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-03.rs"
```

</details>

### 2. `src/marker.rs`

Name the render-only size mode explicitly; original point widths and editing-control diameters remain unchanged.

<details>
<summary>Locate the existing block</summary>

```rust
impl Marker {
```

</details>

After that block, type:

```rust
    // The signed GPU size is a CSS diameter, or negative mesh spacing in mm.
    pub fn mesh_dot(center:[f32;3],colour:[f32;4],spacing:f32)->Self{
        Self{center,colour,diameter:if spacing>0.{-spacing}else{1.}}
    }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-04.rs"
```

</details>

### 3. `src/pick_sources.rs`

Reuse the existing32byte point instance, endpoint addresses and upload; no distance array, extra metadata or buffer.

<details>
<summary>Locate the existing block</summary>

```rust
                    sources.mesh_markers.push(crate::marker::Marker{center:model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32(),diameter:1.,colour:edge_colour.unwrap_or_else(||row.mesh.parts.dot_colour(i))});
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
                    sources.mesh_markers.push(crate::marker::Marker::mesh_dot(model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32(),edge_colour.unwrap_or_else(||row.mesh.parts.dot_colour(i)),row.mesh.spacing));
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-05.rs"
```

</details>

### 4. `src/stroke_gpu.rs`

Accept the explicit finite mesh-spacing size mode while rejecting zero and nonfinite display data.

<details>
<summary>Locate the existing block</summary>

```rust
            if marker.diameter <= 0.0 || marker.values().iter().any(|v| !v.is_finite()) { return Err("Invalid marker display data"); }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
            if marker.diameter == 0.0 || marker.values().iter().any(|v| !v.is_finite()) { return Err("Invalid marker display data"); }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-06.rs"
```

</details>

### 5. `src/marker_radius.wgsl`

Project one instance spacing at each vertex depth; retain the actual three-diameter taper and half-physical-pixel minimum.

Create the file and type:

```wgsl
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-07.rs"
```

### 6. `src/finite_project_source.rs`

Expand one shared radius helper for both color/pick and existing finite/legacy paths.

<details>
<summary>Locate the existing block</summary>

```rust
        .replace("#include \"marker_coverage.wgsl\"", include_str!("marker_coverage.wgsl"))
```

</details>

After that block, type:

```rust
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-08.rs" skip=92
```

### 7. `src/marker.wgsl`

Use the same radius rule in the existing marker paths. Color and integer picking retain the same projected marker footprint.

<details>
<summary>Locate the existing block</summary>

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) offset: vec2<f32>,
    @location(2) @interpolate(flat) radius: f32,
}
fn geometry(center: vec3<f32>, diameter: f32, colour: vec4<f32>, index: u32) -> Output {
    var position = view.transform * vec4<f32>(center, 1.0);
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, vec2<f32>(0.0), 0.0);
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = diameter * view.density * 0.5;
    let offset = corners[index] * (radius + 0.70711);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, offset, radius);
}
#include "marker_coverage.wgsl"

@vertex
fn vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32) -> Output { return geometry(center, diameter, colour, index); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> { return colour(input); }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) offset: vec2<f32>,
    @location(2) @interpolate(flat) radius: f32,
}
#include "marker_radius.wgsl"
fn geometry(center: vec3<f32>, diameter: f32, colour: vec4<f32>, index: u32) -> Output {
    var position = view.transform * vec4<f32>(center, 1.0);
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, vec2<f32>(0.0), 0.0);
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = marker_radius(position,diameter);
    let offset = corners[index] * (radius + 0.70711);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, offset, radius);
}
#include "marker_coverage.wgsl"

@vertex
fn vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32) -> Output { return geometry(center, diameter, colour, index); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> { return colour(input); }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-09.rs"
```

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-10.rs"
```

</details>

### 8. `src/finite_marker.wgsl`

Use the same radius rule in the existing marker paths. Color and integer picking retain the same projected marker footprint.

<details>
<summary>Locate the existing block</summary>

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) offset: vec2<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
}
fn geometry(center: vec3<f32>, diameter: f32, colour: vec4<f32>, index: u32,flags:u32) -> Output {
    var position = view.transform * vec4<f32>(center, 1.0);
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, vec2<f32>(0.0), 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0),vec2<f32>(0.0));
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = diameter * view.density * 0.5;
    let offset = corners[index] * (radius + 0.70711);
    let axis=finite_axis(position,position);let depths=vec2<f32>(position.z/position.w);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, offset, radius,flags,axis,depths,finite_screen(position));
}
#include "marker_coverage.wgsl"

@vertex
fn vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32,@location(8) flags:u32) -> Output { return geometry(center, diameter, colour, index,flags); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_disc_alpha(input.finite_pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,false);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
#include "finite_visibility.wgsl"
fn finite_depth_pixel(pixel:vec2<f32>)->vec2<i32>{return vec2<i32>(pixel);}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) offset: vec2<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
}
#include "marker_radius.wgsl"
fn geometry(center: vec3<f32>, diameter: f32, colour: vec4<f32>, index: u32,flags:u32) -> Output {
    var position = view.transform * vec4<f32>(center, 1.0);
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, vec2<f32>(0.0), 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0),vec2<f32>(0.0));
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = marker_radius(position,diameter);
    let offset = corners[index] * (radius + 0.70711);
    let axis=finite_axis(position,position);let depths=vec2<f32>(position.z/position.w);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, offset, radius,flags,axis,depths,finite_screen(position));
}
#include "marker_coverage.wgsl"

@vertex
fn vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32,@location(8) flags:u32) -> Output { return geometry(center, diameter, colour, index,flags); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_disc_alpha(input.finite_pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,false);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
#include "finite_visibility.wgsl"
fn finite_depth_pixel(pixel:vec2<f32>)->vec2<i32>{return vec2<i32>(pixel);}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-11.rs"
```

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-12.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 9. `src/mesh_density_tests.rs`

Exercise actual dense physical footprints, explicit point/control independence, correct rear-edge visibility, source release and shared geometry.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-13.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 10. `src/renderer.rs`

Register the three actual source-bound density GPU checks.

<details>
<summary>Locate the existing block</summary>

```text
mod mesh_marker_tests;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-14.rs" skip=22
```

</details>

### 11. `src/marker_coverage.wgsl`

Measure the circle at the actual physical fragment pixel relative to its fixed centre; keep one shared colour/query coverage function.

<details>
<summary>Locate the existing block</summary>

```wgsl
fn colour(input: Output) -> vec4<f32> {
    let coverage = clamp(input.radius + 0.5 - length(input.offset), 0.0, 1.0);
    return vec4<f32>(input.colour.rgb, input.colour.a * coverage);
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-physical-centre-01.wgsl"
```

### 12. `src/marker.wgsl`

Carry a fixed screen centre instead of interpolating the expanded quad offset.

<details>
<summary>Locate the existing block</summary>

```wgsl
    @location(1) @interpolate(linear) offset: vec2<f32>,
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    @location(1) @interpolate(flat) centre: vec2<f32>,
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-physical-centre-02.wgsl"
```

</details>

### 13. `src/marker_pick.wgsl`

Carry a fixed screen centre instead of interpolating the expanded quad offset.

<details>
<summary>Locate the existing block</summary>

```wgsl
    @location(1) @interpolate(linear) offset: vec2<f32>,
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    @location(1) @interpolate(flat) centre: vec2<f32>,
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-physical-centre-03.wgsl"
```

</details>

### 14. `src/marker.wgsl`

Compute the centre before expanding the existing quad; no new vertex buffer or radius policy. Pass the fixed screen centre through the existing location. Use the fragment built-in pixel rather than interpolated quad coordinates.

<details>
<summary>Locate the existing block</summary>

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(flat) centre: vec2<f32>,
    @location(2) @interpolate(flat) radius: f32,
}
#include "marker_radius.wgsl"
fn geometry(center: vec3<f32>, diameter: f32, colour: vec4<f32>, index: u32) -> Output {
    var position = view.transform * vec4<f32>(center, 1.0);
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, vec2<f32>(0.0), 0.0);
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = marker_radius(position,diameter);
    let offset = corners[index] * (radius + 0.70711);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, offset, radius);
}
#include "marker_coverage.wgsl"

@vertex
fn vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32) -> Output { return geometry(center, diameter, colour, index); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> { return colour(input); }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(flat) centre: vec2<f32>,
    @location(2) @interpolate(flat) radius: f32,
}
#include "marker_radius.wgsl"
fn geometry(center: vec3<f32>, diameter: f32, colour: vec4<f32>, index: u32) -> Output {
    var position = view.transform * vec4<f32>(center, 1.0);
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, vec2<f32>(0.0), 0.0);
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = marker_radius(position,diameter);
    let centre=(position.xy/position.w*vec2<f32>(0.5,-0.5)+0.5)*view.size;
    let offset = corners[index] * (radius + 0.70711);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, centre, radius);
}
#include "marker_coverage.wgsl"

@vertex
fn vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32) -> Output { return geometry(center, diameter, colour, index); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> { return colour(input.colour,input.radius,input.position.xy-input.centre); }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-physical-centre-04.wgsl"
```

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-physical-centre-05.wgsl"
```

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-physical-centre-06.wgsl"
```

</details>

### 15. `src/marker_pick.wgsl`

Give the cropped query the same centre in its local physical-pixel coordinates. Remove the unnecessary reconstructed colour-output record. Apply the same analytic disc coverage to integer queries.

<details>
<summary>Locate the existing block</summary>

```wgsl
@group(1) @binding(0) var<uniform> crop: Crop;
struct PickOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(flat) centre: vec2<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
}
@vertex
fn pick_vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>) -> PickOutput {
    let source = geometry(center, diameter, colour, index);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.offset = source.offset;
    output.radius = source.radius;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.offset, input.radius);
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(source).a <= 0.0) { discard; }
    return input.id;
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(1) @binding(0) var<uniform> crop: Crop;
struct PickOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(flat) centre: vec2<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
}
@vertex
fn pick_vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>) -> PickOutput {
    let source = geometry(center, diameter, colour, index);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.centre = source.centre-vec2<f32>(vec2<u32>(crop._a,crop._b));
    output.radius = source.radius;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(input.colour,input.radius,input.position.xy-input.centre).a <= 0.0) { discard; }
    return input.id;
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-physical-centre-07.wgsl"
```

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-physical-centre-09.wgsl"
```

</details>

### 16. `src/finite_marker.wgsl`

Reuse the existing fixed axis centre; drop the redundant interpolated offset. Use the actual fragment pixel and remove the second redundant varying.

<details>
<summary>Locate the existing block</summary>

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) offset: vec2<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
}
#include "marker_radius.wgsl"
fn geometry(center: vec3<f32>, diameter: f32, colour: vec4<f32>, index: u32,flags:u32) -> Output {
    var position = view.transform * vec4<f32>(center, 1.0);
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, vec2<f32>(0.0), 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0),vec2<f32>(0.0));
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = marker_radius(position,diameter);
    let offset = corners[index] * (radius + 0.70711);
    let axis=finite_axis(position,position);let depths=vec2<f32>(position.z/position.w);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, offset, radius,flags,axis,depths,finite_screen(position));
}
#include "marker_coverage.wgsl"

@vertex
fn vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32,@location(8) flags:u32) -> Output { return geometry(center, diameter, colour, index,flags); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_disc_alpha(input.finite_pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,false);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
#include "finite_visibility.wgsl"
fn finite_depth_pixel(pixel:vec2<f32>)->vec2<i32>{return vec2<i32>(pixel);}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
}
#include "marker_radius.wgsl"
fn geometry(center: vec3<f32>, diameter: f32, colour: vec4<f32>, index: u32,flags:u32) -> Output {
    var position = view.transform * vec4<f32>(center, 1.0);
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, vec2<f32>(0.0), 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0),vec2<f32>(0.0));
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = marker_radius(position,diameter);
    let offset = corners[index] * (radius + 0.70711);
    let axis=finite_axis(position,position);let depths=vec2<f32>(position.z/position.w);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, offset, radius,flags,axis,depths,finite_screen(position));
}
#include "marker_coverage.wgsl"

@vertex
fn vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32,@location(8) flags:u32) -> Output { return geometry(center, diameter, colour, index,flags); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_disc_alpha(input.finite_pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,false);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
#include "finite_visibility.wgsl"
fn finite_depth_pixel(pixel:vec2<f32>)->vec2<i32>{return vec2<i32>(pixel);}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

</details>

### 17. `src/finite_marker_pick.wgsl`

Reuse the existing fixed axis centre; drop the redundant interpolated offset. Use the actual fragment pixel and remove the second redundant varying.

<details>
<summary>Locate the existing block</summary>

```wgsl
@group(1) @binding(0) var<uniform> crop: Crop;
struct PickOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) offset: vec2<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
}
@vertex
fn pick_vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>,@location(8) flags:u32) -> PickOutput {
    let source = geometry(center, diameter, colour, index,flags);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.offset = source.offset;
    output.radius = source.radius;
    output.clip_flags=source.clip_flags;
    output.finite_axis_value=source.finite_axis_value;output.finite_depths=source.finite_depths;output.finite_pixel=source.finite_pixel;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.offset, input.radius,input.clip_flags,input.finite_axis_value,input.finite_depths,input.finite_pixel);
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(source).a <= 0.0) { discard; }
    if(finite_disc_alpha(input.finite_pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,true)<=0.){discard;}
    return input.id;
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(1) @binding(0) var<uniform> crop: Crop;
struct PickOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
}
@vertex
fn pick_vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>,@location(8) flags:u32) -> PickOutput {
    let source = geometry(center, diameter, colour, index,flags);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.offset = source.offset;
    output.radius = source.radius;
    output.clip_flags=source.clip_flags;
    output.finite_axis_value=source.finite_axis_value;output.finite_depths=source.finite_depths;output.finite_pixel=source.finite_pixel;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.offset, input.radius,input.clip_flags,input.finite_axis_value,input.finite_depths,input.finite_pixel);
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(source).a <= 0.0) { discard; }
    if(finite_disc_alpha(input.finite_pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,true)<=0.){discard;}
    return input.id;
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

</details>

### 18. `src/finite_marker.wgsl`

Keep the rejected-vertex result aligned with the smaller output. Reuse existing fixed source centre/depth fields and omit redundant pixel interpolation. Read visibility at the actual full-frame physical pixel. Evaluate colour coverage from the existing flat centre.

<details>
<summary>Locate the existing block</summary>

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
}
#include "marker_radius.wgsl"
fn geometry(center: vec3<f32>, diameter: f32, colour: vec4<f32>, index: u32,flags:u32) -> Output {
    var position = view.transform * vec4<f32>(center, 1.0);
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, vec2<f32>(0.0), 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0),vec2<f32>(0.0));
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = marker_radius(position,diameter);
    let offset = corners[index] * (radius + 0.70711);
    let axis=finite_axis(position,position);let depths=vec2<f32>(position.z/position.w);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, offset, radius,flags,axis,depths,finite_screen(position));
}
#include "marker_coverage.wgsl"

@vertex
fn vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32,@location(8) flags:u32) -> Output { return geometry(center, diameter, colour, index,flags); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_disc_alpha(input.finite_pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,false);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
#include "finite_visibility.wgsl"
fn finite_depth_pixel(pixel:vec2<f32>)->vec2<i32>{return vec2<i32>(pixel);}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
}
#include "marker_radius.wgsl"
fn geometry(center: vec3<f32>, diameter: f32, colour: vec4<f32>, index: u32,flags:u32) -> Output {
    var position = view.transform * vec4<f32>(center, 1.0);
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0));
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = marker_radius(position,diameter);
    let offset = corners[index] * (radius + 0.70711);
    let axis=finite_axis(position,position);let depths=vec2<f32>(position.z/position.w);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, radius,flags,axis,depths);
}
#include "marker_coverage.wgsl"

@vertex
fn vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32,@location(8) flags:u32) -> Output { return geometry(center, diameter, colour, index,flags); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_disc_alpha(input.position.xy,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,false);if(alpha<=0.){discard;}let c=colour(input.colour,input.radius,input.position.xy-input.finite_axis_value.xy);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
#include "finite_visibility.wgsl"
fn finite_depth_pixel(pixel:vec2<f32>)->vec2<i32>{return vec2<i32>(pixel);}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-physical-centre-14.wgsl"
```

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-physical-centre-15.wgsl"
```

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-physical-centre-16.wgsl"
```

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-physical-centre-17.wgsl"
```

</details>

### 19. `src/finite_marker_pick.wgsl`

Remove the deleted output varying from the query vertex. Keep the existing fixed axis/depth values without another varying. Restore the query fragment to the full physical frame once, then share it with coverage and visibility. Keep the integer-query disc identical to colour coverage. Use the same restored full-frame pixel for depth/section eligibility.

<details>
<summary>Locate the existing block</summary>

```wgsl
@group(1) @binding(0) var<uniform> crop: Crop;
struct PickOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
}
@vertex
fn pick_vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>,@location(8) flags:u32) -> PickOutput {
    let source = geometry(center, diameter, colour, index,flags);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.offset = source.offset;
    output.radius = source.radius;
    output.clip_flags=source.clip_flags;
    output.finite_axis_value=source.finite_axis_value;output.finite_depths=source.finite_depths;output.finite_pixel=source.finite_pixel;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.offset, input.radius,input.clip_flags,input.finite_axis_value,input.finite_depths,input.finite_pixel);
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(source).a <= 0.0) { discard; }
    if(finite_disc_alpha(input.finite_pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,true)<=0.){discard;}
    return input.id;
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(1) @binding(0) var<uniform> crop: Crop;
struct PickOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
}
@vertex
fn pick_vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>,@location(8) flags:u32) -> PickOutput {
    let source = geometry(center, diameter, colour, index,flags);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.radius = source.radius;
    output.clip_flags=source.clip_flags;
    output.finite_axis_value=source.finite_axis_value;output.finite_depths=source.finite_depths;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let pixel=input.position.xy+vec2<f32>(vec2<u32>(crop._a,crop._b));
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(input.colour,input.radius,pixel-input.finite_axis_value.xy).a <= 0.0) { discard; }
    if(finite_disc_alpha(pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,true)<=0.){discard;}
    return input.id;
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-physical-centre-20.wgsl"
```

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-physical-centre-21.wgsl"
```

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-physical-centre-22.wgsl"
```

</details>

</details>

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 96zzzzzzq-taper-crowded-original-mesh-vertices
npm --prefix ../session_tests run course -- check 96zzzzzzq-taper-crowded-original-mesh-vertices
npm --prefix ../session_tests run course -- save 96zzzzzzq-taper-crowded-original-mesh-vertices
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

Crowding-aware dots retain more of the face at readable zoom levels. At extreme zoom-out, the half-physical-pixel minimum can still overlap. Explicit point sizes, active controls, original ownership and shared geometry remain stable.

- **Final Lesson Experiment:** Compare the actual crowded original grid with its preceding dark-sheet render and the maintained taper reference. Inspect both projections and densities, explicit points and controls, source unload, camera changes and shared mirrored placement. Compare the readable and extreme-zoom views: the minimum physical footprint intentionally limits further thinning.

Continue with [Original ambient inputs and real pass ordering](../../consolidation/96zzzzzzr-share-geometric-ambient-inputs.md).
