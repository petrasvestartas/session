# 20.3 · Shared arenas, object records and source preparation

**Section 10: Shared GPU Arenas, Floating-Origin Precision, Multisampling, and Production Parity** · **Lesson 20.3** (Redraft `10.7`) · **14 checkpoints (`96zzg-grow-buffers-without-losing-rows` → `96zzt-share-the-current-selection-yellow`)** · **Study time: 10–14 h**

> **What You Will Learn & Build:** Preserve uploaded GPU records when a shared buffer grows. By the final checkpoint (`96zzt-share-the-current-selection-yellow`), you will use the current selection yellow for surfaces and temporary highlights without changing original colors.

## 1. Visual Pipeline & WebGPU Architecture (Black & White Schematics)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 220" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="212" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Lesson 20.3 Execution Flow: Shared arenas, object records and source preparation</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">Concrete data pipeline you build in this lesson (14 Checkpoints: 96zzg-grow-buffers-without-losing-rows → 96zzt-share-the-current-selection-yellow)</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="68" width="184" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="110" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="92" x2="192" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="110" y="108" text-anchor="middle" font-size="11" fill="#111111">Follow existing rows</text>
    <text x="110" y="124" text-anchor="middle" font-size="11" fill="#111111">through buffer</text>
    <text x="110" y="140" text-anchor="middle" font-size="11" fill="#111111">replacement,…</text>
    <line x1="202" y1="108" x2="222" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="224" y="68" width="184" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="234" y1="92" x2="398" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="316" y="108" text-anchor="middle" font-size="11" fill="#111111">Session palette</text>
    <line x1="408" y1="108" x2="428" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="430" y="68" width="184" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="522" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="440" y1="92" x2="604" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="522" y="108" text-anchor="middle" font-size="11" fill="#111111">CPU highlight data and</text>
    <text x="522" y="124" text-anchor="middle" font-size="11" fill="#111111">shared WGSL</text>
    <line x1="614" y1="108" x2="634" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="636" y="68" width="184" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="728" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="646" y1="92" x2="810" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="728" y="108" text-anchor="middle" font-size="11" fill="#111111">selected display</text>
    
    <rect x="18" y="162" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="181" font-size="11" font-weight="700" fill="#111111">Files You Edit in Lesson 20.3:</text>
    <text x="215" y="181" font-size="11" fill="#111111">src/text_gpu_support.rs  ·  src/growing_buffer_tests.rs  ·  src/lib.rs  ·  src/arena_rows.rs  ·  src/arena_rows_tests.rs  ·  src/mesh_arena.rs</text>
    <text x="32" y="198" font-size="10.5" fill="#111111">14 Checkpoints: 96zzg-grow-buffers-without-losing-rows → 96zzt-share-the-current-selection-yellow — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
</svg>
</div>

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 240" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="232" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Section 10 Production Architecture: Shared GPU Storage Arena, f64 Floating Origin, Reversed-Z, 4x MSAA & Matrix Shader</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">How the maintained viewer pulls vertices from one growable GPU arena, eliminates jitter at 10^12 units, and renders procedural Matrices.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    <rect x="18" y="68" width="250" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="143" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">1. Shared GPU Arena (Ch 19–20)</text>
    <text x="28" y="108" font-size="10.5" fill="#111111">• 1 Growable Storage Buffer Arena</text>
    <text x="28" y="126" font-size="10.5" font-weight="600" fill="#111111">• Vertex Pulling (@builtin(vertex_index))</text>
    <text x="28" y="144" font-size="10.5" fill="#111111">• 8-byte Draw Slots -&gt; ObjectRecord</text>
    <text x="28" y="162" font-size="10.5" fill="#111111">• slot_table.wgsl maps Rg32Uint</text>
    <text x="28" y="178" font-size="10.5" fill="#111111">  primitive IDs to Element/Face owners</text>
    <text x="28" y="194" font-size="10" fill="#111111">• Free-list span reuse after GPU lease</text>

    <line x1="268" y1="138" x2="298" y2="138" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="300" y="68" width="265" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="432" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">2. f64 Floating Origin &amp; Reversed-Z (Ch 21)</text>
    <text x="312" y="108" font-size="10.5" font-weight="600" fill="#111111">• rel_f32 = (world_f64 - anchor_f64) as f32</text>
    <text x="312" y="125" font-size="10" fill="#111111">  Zero vertex/grid jitter even at 10^12 m!</text>
    <text x="312" y="144" font-size="10.5" font-weight="600" fill="#111111">• Reversed-Z Depth (GreaterEqual):</text>
    <text x="312" y="160" font-size="10" fill="#111111">  Near plane = 1.0, Far plane = 0.0</text>
    <text x="312" y="178" font-size="10.5" fill="#111111">• Held world base grid &amp; preserved</text>
    <text x="312" y="193" font-size="10.5" fill="#111111">  live camera during Session reload</text>

    <line x1="565" y1="138" x2="595" y2="138" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="597" y="68" width="223" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="708" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">3. 4x MSAA &amp; Matrix Shader (Ch 22)</text>
    <text x="609" y="108" font-size="10.5" fill="#111111">• Coherent 1x/4x MSAA sample targets</text>
    <text x="609" y="126" font-size="10.5" font-weight="600" fill="#111111">• matrix.wgsl (16w header + 3w/cell):</text>
    <text x="609" y="142" font-size="10" fill="#111111">  Heatmap + procedural 3-decimal digits</text>
    <text x="609" y="157" font-size="10" fill="#111111">  (0 cell blank; plate-less title; yellow</text>
    <text x="609" y="172" font-size="10" fill="#111111">  selected border; Rg32Uint whole pick)</text>
    <text x="609" y="190" font-size="10" fill="#111111">• Coalesced single-flight frame scheduler</text>
    
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

**Authoritative production source (`session_viewer/src`):** [engine/gpu/arena.rs](../../../../src/engine/gpu/arena.rs), [engine/gpu/buffers.rs](../../../../src/engine/gpu/buffers.rs), [engine/gpu/objects.rs](../../../../src/engine/gpu/objects.rs), [engine/gpu/slots.rs](../../../../src/engine/gpu/slots.rs), [engine/gpu/instanced.rs](../../../../src/engine/gpu/instanced.rs), [engine/gpu/faces.rs](../../../../src/engine/gpu/faces.rs), [engine/gpu/clip.rs](../../../../src/engine/gpu/clip.rs), [engine/gpu/triangle_tiles.rs](../../../../src/engine/gpu/triangle_tiles.rs), [app/scene_sync/tombs.rs](../../../../src/app/scene_sync/tombs.rs), [build.rs](../../../../build.rs).

## 3. Checkpoint Roadmap (Jump to Any Notebook Step)

| Step | Checkpoint | What You Build | Files Modified | Pipeline Trace |
| ---: | :--- | :--- | :--- | :--- |
| **1** | [`96zzg-grow-buffers-without-losing-rows`](#checkpoint-96zzg-grow-buffers-without-losing-rows) | Preserve uploaded GPU records when a shared buffer grows. | `src/text_gpu_support.rs`, `src/growing_buffer_tests.rs`, `src/lib.rs` | `Follow existing rows through buffer replacement, an ordered GPU copy and the next append.` |
| **2** | [`96zzh-prepare-shared-compact-mesh-ranges`](#checkpoint-96zzh-prepare-shared-compact-mesh-ranges) | Prepare shared packed records and stable ranges without pinning display sources. | `src/arena_rows.rs`, `src/arena_rows_tests.rs`, `src/lib.rs` | `Follow a local mesh index into its arena vertex offset and a reused range for shared placements.` |
| **3** | [`96zzi-upload-compact-ranges-to-shared-gpu-storage`](#checkpoint-96zzi-upload-compact-ranges-to-shared-gpu-storage) | Upload shared compact ranges and refresh immutable bindings after buffer growth. | `src/arena_rows.rs`, `src/mesh_arena.rs`, `src/lib.rs`, `src/growing_buffer_tests.rs` | `Follow an appended range into buffer growth and the replacement shared bind group.` |
| **4** | [`96zzj-share-the-object-record-layout`](#checkpoint-96zzj-share-the-object-record-layout) | Share the host object-record size across uniform and instance consumers. | `src/object_record.rs`, `src/lib.rs`, `src/gpu_mesh.rs`, `src/instance_gpu.rs` | `packed object bytes → uniform binding or instance array stride → WGSL object fields` |
| **5** | [`96zzk-borrow-the-current-arena-buffers`](#checkpoint-96zzk-borrow-the-current-arena-buffers) | Use one geometry interface for standalone and current shared storage. | `src/gpu_geometry.rs`, `src/face_pick.rs`, `src/tile_gpu.rs`, `src/caps_gpu.rs` | `geometry lease → arena owner → current vertex, index and binding handles` |
| **6** | [`96zzl-read-shared-ranges-through-object-records`](#checkpoint-96zzl-read-shared-ranges-through-object-records) | Read an arena range without changing original owners or local triangle identity. | `src/object_record.rs`, `src/gpu_mesh.rs`, `src/triangle.wgsl`, `src/cap_count.wgsl` | `object range start → shared index → shared packed vertex → original draw and triangle owner` |
| **7** | [`96zzm-connect-the-renderer-to-one-mesh-arena`](#checkpoint-96zzm-connect-the-renderer-to-one-mesh-arena) | Render prepared source meshes through one shared arena and release its owner when unused. | `src/geometry_cache.rs`, `src/renderer.rs`, `src/browser.rs`, `src/global_outline_tests.rs` | `prepared mesh → cached geometry lease → shared arena → object range → existing renderer passes` |
| **8** | [`96zzn-describe-eight-byte-draw-slots`](#checkpoint-96zzn-describe-eight-byte-draw-slots) | Store an object row and primitive ID base in one instance-step vertex record. | `src/draw_slots.rs`, `src/lib.rs`, `src/draw_slots_tests.rs` | `object row + primitive ID base → eight-byte slot → instance-step vertex input` |
| **9** | [`96zzo-read-instance-properties-through-draw-slots`](#checkpoint-96zzo-read-instance-properties-through-draw-slots) | Connect compact draw slots to the existing instance color pipeline. | `src/draw_slots.rs`, `src/instance_gpu.rs`, `src/object_record_tests.rs`, `src/pick_gpu_tests.rs` | `instance slot → property record + ID base → shared mesh range → color and physical triangle` |
| **10** | [`96zzp-classify-contact-fill-shading`](#checkpoint-96zzp-classify-contact-fill-shading) | Read the contact marker from packed RGBA and keep that display fill opaque and unlit. | `src/triangle.wgsl`, `src/face_pick.wgsl`, `src/global_outline_tests.rs` | `derived RGBA → packed vertex → contact classification → opaque unlit display` |
| **11** | [`96zzq-open-files-through-one-measured-session-reader`](#checkpoint-96zzq-open-files-through-one-measured-session-reader) | Prepare browser file imports once through the measured full original-document reader. | `src/load_measure.rs`, `src/document_source.rs`, `src/source_phase_tests.rs`, `src/lib.rs` | `file bytes → decode → full validation → kernel → display walk → atomic history adoption` |
| **12** | [`96zzr-route-current-close-and-opacity-commands`](#checkpoint-96zzr-route-current-close-and-opacity-commands) | Finish an active curve with Close and read the current Opacity argument convention. | `src/commands.rs`, `src/editor.rs`, `src/opacity_command_tests.rs`, `src/current_command_tests.rs` | `command words → existing editor action → draft or display state` |
| **13** | [`96zzs-reuse-full-source-preparation-for-editor-and-reload`](#checkpoint-96zzs-reuse-full-source-preparation-for-editor-and-reload) | Reuse the full Session reader for byte actions and released mesh sources. | `src/editor.rs`, `src/document.rs`, `src/full_source_action_tests.rs`, `src/lib.rs` | `source bytes → shared full reader → prepared scene or restored source` |
| **14** | [`96zzt-share-the-current-selection-yellow`](#checkpoint-96zzt-share-the-current-selection-yellow) | Use the current selection yellow for surfaces and temporary highlights without changing original colors. | `src/colour.rs`, `src/highlight.rs`, `src/tool_preview.rs`, `src/clipping_gpu.rs` | `Session palette → CPU highlight data and shared WGSL → selected display` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Read the **Concept Walkthrough** and **Self-Check Q&A** first, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-96zzg-grow-buffers-without-losing-rows"></a>

### Step 1 of 14 · Grow buffers without losing rows (`96zzg-grow-buffers-without-losing-rows`)

- **Goal:** Preserve uploaded GPU records when a shared buffer grows.
- **Data Flow Trace:** `Follow existing rows through buffer replacement, an ordered GPU copy and the next append.`
- **Files You Touch:** `src/text_gpu_support.rs` (*Grow the existing owner in rows, preserve live bytes on the GPU and share one append algorithm.*) · `src/growing_buffer_tests.rs` (*Supply real mapped GPU bytes for nonempty growth, empty appends, reuse, release and the existing float adapter.*) · `src/lib.rs` (*Register the supplied GPU-growth readback fixture.*)

Growing a shared GPU arena must keep rows already uploaded.

Extend the existing text buffer owner to append any aligned record bytes, preserve live rows with a GPU copy and respect device limits. Text keeps the same owner; the compact mesh arena will reuse it next. Mesh storage is still per geometry at this preparatory checkpoint.

> **Self-Check Question:** *Why submit the old-row copy before writing the appended rows?*  
> **Answer:** Queue order preserves earlier writes before the copy and places the new append after it. No CPU readback is needed for growth.

- **What You Should See:** One existing GPU buffer owner preserves old records, supports aligned packed bytes and bounds growth by device limits.
- **Hands-On Lab Experiment:** Create and move original text repeatedly, then clear it and add it again. Check the existing text counters and pixels. The supplied GPU fixture separately verifies the exact bytes through multiple nonempty growths and reset/release.

<details>
<summary>Implementation 96zzg-grow-buffers-without-losing-rows: Grow buffers without losing rows</summary>

Preserve uploaded GPU records when a shared buffer grows.

Data flow: Follow existing rows through buffer replacement, an ordered GPU copy and the next append..

### 1. `src/text_gpu_support.rs`

Grow the existing owner in rows, preserve live bytes on the GPU and share one append algorithm.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct GrowBuf {pub buf:wgpu::Buffer,stride:u64,len:u32,label:&'static str,usage:wgpu::BufferUsages}
impl GrowBuf {
 pub fn new(ctx:&GpuCtx,label:&'static str,stride:u64,usage:wgpu::BufferUsages)->Self {Self {buf:ctx.device.create_buffer(&wgpu::BufferDescriptor {label:Some(label),size:stride,usage,mapped_at_creation:false}),stride,len:0,label,usage}}
 pub fn append<const N:usize>(&mut self,ctx:&GpuCtx,vertices:&[[f32;N]]) {assert_eq!(self.stride,N as u64*4);if vertices.is_empty() {return;}
  let count=u32::try_from(vertices.len()).expect("Bounded text vertices");let size=(self.len as u64+count as u64)*self.stride;
  if size>self.buf.size() {assert_eq!(self.len,0,"Text upload starts at an empty buffer");self.buf=ctx.device.create_buffer(&wgpu::BufferDescriptor {label:Some(self.label),size:size.next_power_of_two(),usage:self.usage,mapped_at_creation:false});}
  let bytes:Vec<u8>=vertices.iter().flatten().flat_map(|n|n.to_ne_bytes()).collect();ctx.queue.write_buffer(&self.buf,self.len as u64*self.stride,&bytes);self.len+=count;
 }
 pub fn reset(&mut self) {self.len=0;}
 pub fn release(&mut self,ctx:&GpuCtx) {*self=Self::new(ctx,self.label,self.stride,self.usage);}
 pub fn len(&self)->u32 {self.len}
 pub fn is_empty(&self)->bool {self.len==0}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96zzg-grow-buffers-without-losing-rows-1.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/growing_buffer_tests.rs`

Supply real mapped GPU bytes for nonempty growth, empty appends, reuse, release and the existing float adapter.

Copy this supplied file:

```text
--8<-- "journey/code/96zzg-grow-buffers-without-losing-rows-2.rs"
```

</details>

### 3. `src/lib.rs`

Register the supplied GPU-growth readback fixture.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod text_gpu_support;
```

</details>

After that block, type:

```rust
#[cfg(test)]
mod growing_buffer_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzg-grow-buffers-without-losing-rows-3.rs"
```

</details>

</details>

<a id="checkpoint-96zzh-prepare-shared-compact-mesh-ranges"></a>

### Step 2 of 14 · Prepare shared compact mesh ranges (`96zzh-prepare-shared-compact-mesh-ranges`)

- **Goal:** Prepare shared packed records and stable ranges without pinning display sources.
- **Data Flow Trace:** `Follow a local mesh index into its arena vertex offset and a reused range for shared placements.`
- **Files You Touch:** `src/arena_rows.rs` (*Use the existing twenty-byte encoder, rebase validated local indices and cache weak display identities.*) · `src/arena_rows_tests.rs` (*Supply exact packed bytes, shared owner ranges, global indices, weak retention and reset/release fixtures.*) · `src/lib.rs` (*Register the preparatory arena records and supplied fixtures.*)

Shared geometry needs one range of vertices and indices, regardless of how many placed rows draw it.

Prepare compact mesh records with the existing encoder. Rebase local indices into their shared vertex range and reuse ranges for the same display owner. Keep weak source references so derived arena records do not keep removed sources alive. This is preparation; the renderer still uploads per geometry.

> **Self-Check Question:** *Why add the vertex offset to indices rather than changing source positions?*  
> **Answer:** The range changes where storage lives. Positions remain in the original object frame; placements remain separate draw state.

- **What You Should See:** Prepared arena records preserve source positions and share ranges by display identity.
- **Hands-On Lab Experiment:** Inspect placed copies of an original CAD surface and compare their shared geometry counts. Rendering is unchanged at this preparatory checkpoint; the supplied fixture verifies exact compact bytes, index offsets and weak source retention.

<details>
<summary>Implementation 96zzh-prepare-shared-compact-mesh-ranges: Prepare shared compact mesh ranges</summary>

Prepare shared packed records and stable ranges without pinning display sources.

Data flow: Follow a local mesh index into its arena vertex offset and a reused range for shared placements..

### 1. `src/arena_rows.rs`

Use the existing twenty-byte encoder, rebase validated local indices and cache weak display identities.

Create the file and type:

```rust
--8<-- "journey/code/96zzh-prepare-shared-compact-mesh-ranges-1.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/arena_rows_tests.rs`

Supply exact packed bytes, shared owner ranges, global indices, weak retention and reset/release fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/96zzh-prepare-shared-compact-mesh-ranges-2.rs"
```

</details>

### 3. `src/lib.rs`

Register the preparatory arena records and supplied fixtures.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod packed_vertex;
```

</details>

After that block, type:

```rust
pub mod arena_rows;
#[cfg(test)]
mod arena_rows_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzh-prepare-shared-compact-mesh-ranges-3.rs"
```

</details>

</details>

<a id="checkpoint-96zzi-upload-compact-ranges-to-shared-gpu-storage"></a>

### Step 3 of 14 · Upload compact ranges to shared GPU storage (`96zzi-upload-compact-ranges-to-shared-gpu-storage`)

- **Goal:** Upload shared compact ranges and refresh immutable bindings after buffer growth.
- **Data Flow Trace:** `Follow an appended range into buffer growth and the replacement shared bind group.`
- **Files You Touch:** `src/arena_rows.rs` (*Share one index serializer for full and appended ranges.*) · `src/mesh_arena.rs` (*Upload appended records through the existing owners and rebuild bindings only when growth replaces resources.*) · `src/lib.rs` (*Register the preparatory shared GPU storage owner.*) · `src/growing_buffer_tests.rs` (*Reuse the existing supplied mapped-buffer reader in the arena fixtures.*) · `src/mesh_arena_tests.rs` (*Supply real packed-byte reads and two rendered arena triangles through fresh and stale bindings.*)

A shared arena binds one vertex buffer and one index buffer.

Upload only the new ranges through the existing growth owner. Rebuild the shared bind group when growth replaces either buffer: bind groups retain the resources they were created with. Reuse the CPU range cache and index encoder. The application renderer still uses per-geometry uploads at this preparatory checkpoint.

> **Self-Check Question:** *Why does growing a buffer require a new bind group?*  
> **Answer:** The old bind group refers to the old GPU buffer. Updating a Rust field does not retarget that resource binding.

- **What You Should See:** One shared GPU owner uploads appended records, preserves earlier ranges and rebuilds bindings when resources change.
- **Hands-On Lab Experiment:** Inspect existing placed CAD copies and their upload counters; the application remains unchanged here. In the supplied GPU exercise, omit rebuilding the group and predict which of the two arena triangles disappears. Restore the rebuild before continuing.

<details>
<summary>Implementation 96zzi-upload-compact-ranges-to-shared-gpu-storage: Upload compact ranges to shared GPU storage</summary>

Upload shared compact ranges and refresh immutable bindings after buffer growth.

Data flow: Follow an appended range into buffer growth and the replacement shared bind group..

### 1. `src/arena_rows.rs`

Share one index serializer for full and appended ranges.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn index_bytes(&self)->Vec<u8>{self.indices.iter().flat_map(|i|i.to_le_bytes()).collect()}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn index_bytes(&self)->Vec<u8>{self.index_bytes_from(0)}
    pub fn index_bytes_from(&self,start:usize)->Vec<u8>{self.indices[start..].iter().flat_map(|i|i.to_le_bytes()).collect()}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzi-upload-compact-ranges-to-shared-gpu-storage-1.rs"
```

</details>

### 2. `src/mesh_arena.rs`

Upload appended records through the existing owners and rebuild bindings only when growth replaces resources.

Create the file and type:

```rust
--8<-- "journey/code/96zzi-upload-compact-ranges-to-shared-gpu-storage-2.rs"
```

### 3. `src/lib.rs`

Register the preparatory shared GPU storage owner.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod arena_rows;
```

</details>

After that block, type:

```rust
pub mod mesh_arena;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzi-upload-compact-ranges-to-shared-gpu-storage-3.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/growing_buffer_tests.rs`

Reuse the existing supplied mapped-buffer reader in the arena fixtures.

<details>
<summary>Locate the existing block</summary>

```text
fn read(ctx:
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzi-upload-compact-ranges-to-shared-gpu-storage-4.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/mesh_arena_tests.rs`

Supply real packed-byte reads and two rendered arena triangles through fresh and stale bindings.

Copy this supplied file:

```text
--8<-- "journey/code/96zzi-upload-compact-ranges-to-shared-gpu-storage-5.rs"
```

</details>

### 6. `src/lib.rs`

Register the supplied arena GPU fixtures.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod mesh_arena;
```

</details>

After that block, type:

```rust
#[cfg(test)]
mod mesh_arena_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzi-upload-compact-ranges-to-shared-gpu-storage-6.rs"
```

</details>

</details>

<a id="checkpoint-96zzj-share-the-object-record-layout"></a>

### Step 4 of 14 · Share the object record layout (`96zzj-share-the-object-record-layout`)

- **Goal:** Share the host object-record size across uniform and instance consumers.
- **Data Flow Trace:** `packed object bytes → uniform binding or instance array stride → WGSL object fields`
- **Files You Touch:** `src/object_record.rs` (*Name the shared record as two matrices and two four-component blocks.*) · `src/lib.rs` (*Register the shared host layout.*) · `src/gpu_mesh.rs` (*Use the same record type for packing and retained previous bytes.*) · `src/instance_gpu.rs` (*Share the packed type and stride with the instance lane.*) · `src/renderer.rs` (*Declare the object binding from the shared host size.*) · `src/face_pick.rs` (*Declare the object binding from the shared host size.*) · `src/cap_pipeline.rs` (*Declare the object binding from the shared host size.*) · `src/cap_pick_pipeline.rs` (*Declare the object binding from the shared host size.*) · `src/ambient_source_pipeline.rs` (*Declare the object binding from the shared host size.*) · `src/outline_gpu.rs` (*Declare the object binding from the shared host size.*) · `src/object_record_tests.rs` (*Supply a check against the actual assembled WGSL layouts, not a repeated Rust size assertion.*)

One object record contains a model matrix, four integer settings, a normal matrix and a color override. Uniforms and instance arrays must agree on its byte size.

Name the host record and its size once. Reuse them in packing, instance capacity, pipeline bindings and upload accounting. Existing shader fields and browser behavior remain unchanged; this prepares the record for shared arena offsets.

> **Self-Check Question:** *Why does a correct uniform binding size not prove that an instance array has the correct stride?*  
> **Answer:** A uniform reads one object. A storage array locates subsequent objects by its element stride; both must match the WGSL struct layout.

- **Hands-On Lab Experiment:** Add the byte sizes of the two matrices and two four-component settings blocks. Compare uniform and storage-array shader declarations. Predict the effect of an undersized instance stride, then rebuild the unchanged browser scene.

<details>
<summary>Implementation 96zzj-share-the-object-record-layout: Share the object record layout</summary>

Share the host object-record size across uniform and instance consumers.

Data flow: packed object bytes → uniform binding or instance array stride → WGSL object fields.

### 1. `src/object_record.rs`

Name the shared record as two matrices and two four-component blocks.

Create the file and type:

```rust
--8<-- "journey/code/96zzj-share-the-object-record-layout-1.rs"
```

### 2. `src/lib.rs`

Register the shared host layout.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod mesh_arena;
```

</details>

After that block, type:

```rust
pub mod object_record;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzj-share-the-object-record-layout-2.rs"
```

</details>

### 3. `src/gpu_mesh.rs`

Use the same record type for packing and retained previous bytes. Use the shared host record size without changing field offsets.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    pub fn update(&mut self, queue: &wgpu::Queue, object: &Object, style:Style) -> [usize; 2] {
        let next = settings(object, style,self.cap_flags|if self.xray&&self.cap_flags&8192!=0{32768}else{0},self.cap_token,self.primitive_base);
        let mut writes = [0, 0];
        for (start, end) in [(0, 64), (64, 80), (80, 144), (144,160)] {
            if next[start..end] != self.previous[start..end] {
                queue.write_buffer(&self.model, start as u64, &next[start..end]);
                writes[0] += 1;
```

</details>

Replace it with:

```rust
    pub fn update(&mut self, queue: &wgpu::Queue, object: &Object, style:Style) -> [usize; 2] {
        let next = settings(object, style,self.cap_flags|if self.xray&&self.cap_flags&8192!=0{32768}else{0},self.cap_token,self.primitive_base);
        let mut writes = [0, 0];
        for (start, end) in [(0, 64), (64, 80), (80, 144), (144,BYTES)] {
            if next[start..end] != self.previous[start..end] {
                queue.write_buffer(&self.model, start as u64, &next[start..end]);
                writes[0] += 1;
```

<details>
<summary>Locate this block</summary>

```rust
    }

    pub(crate) fn model_buffer(&self)->&wgpu::Buffer{&self.model}
    pub(crate) fn packed(&self)->[u8;160] {self.previous}
    pub fn selected(&self)->bool {u32::from_ne_bytes(self.previous[64..68].try_into().unwrap())&1!=0}

    pub fn set_primitive_base(&mut self,queue:&wgpu::Queue,base:u32)->Result<(),&'static str>{
```

</details>

Replace it with:

```rust
    }

    pub(crate) fn model_buffer(&self)->&wgpu::Buffer{&self.model}
    pub(crate) fn packed(&self)->Record {self.previous}
    pub fn selected(&self)->bool {u32::from_ne_bytes(self.previous[64..68].try_into().unwrap())&1!=0}

    pub fn set_primitive_base(&mut self,queue:&wgpu::Queue,base:u32)->Result<(),&'static str>{
```

<details>
<summary>Locate this block</summary>

```rust
    model_group: wgpu::BindGroup,
    pub id: crate::scene::ObjectId,
    model: wgpu::Buffer,
    previous: [u8; 160],primitive_base:u32,xray:bool,cap_flags:u32,cap_token:u32,
}

#[derive(Clone,Copy)]
pub struct Style {pub selected:bool,pub pickable:bool,pub colour:Option<[f32;4]>}

fn settings(object: &Object, style:Style,cap_flags:u32,cap_token:u32,primitive_base:u32) -> [u8; 160] {
    let mut bytes = [0; 160];
    for (index, value) in object.model.m.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
```

</details>

Replace it with:

```rust
    model_group: wgpu::BindGroup,
    pub id: crate::scene::ObjectId,
    model: wgpu::Buffer,
    previous: Record,primitive_base:u32,xray:bool,cap_flags:u32,cap_token:u32,
}

#[derive(Clone,Copy)]
pub struct Style {pub selected:bool,pub pickable:bool,pub colour:Option<[f32;4]>}

fn settings(object: &Object, style:Style,cap_flags:u32,cap_token:u32,primitive_base:u32) -> Record {
    let mut bytes = [0; BYTES];
    for (index, value) in object.model.m.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
```

<details>
<summary>Locate this block</summary>

```rust
use crate::{gpu_geometry::GpuGeometry, scene::Object};
use std::rc::Rc;
use wgpu::util::DeviceExt;

pub struct GpuMesh {
```

</details>

Replace it with:

```rust
use crate::{gpu_geometry::GpuGeometry, scene::Object};
use std::rc::Rc;
use crate::object_record::{BYTES, Record};
use wgpu::util::DeviceExt;

pub struct GpuMesh {
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{gpu_geometry::GpuGeometry, scene::Object};
use std::rc::Rc;
use crate::object_record::{BYTES, Record};
use wgpu::util::DeviceExt;

pub struct GpuMesh {
    pub geometry: Rc<GpuGeometry>,
    model_group: wgpu::BindGroup,
    pub id: crate::scene::ObjectId,
    model: wgpu::Buffer,
    previous: Record,primitive_base:u32,xray:bool,cap_flags:u32,cap_token:u32,
}

#[derive(Clone,Copy)]
pub struct Style {pub selected:bool,pub pickable:bool,pub colour:Option<[f32;4]>}

fn settings(object: &Object, style:Style,cap_flags:u32,cap_token:u32,primitive_base:u32) -> Record {
    let mut bytes = [0; BYTES];
    for (index, value) in object.model.m.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
    bytes[64..68].copy_from_slice(&((primitive_base<<1)|u32::from(style.selected)).to_ne_bytes());
    let id=if style.pickable {object.id.value()} else {0};
    bytes[68..72].copy_from_slice(&id.to_ne_bytes());
    let flags=if matches!(&object.original,Some(session_rust::Geometry::Plane(p)) if crate::clipping_geometry::is_clipping(p)){4096u32}else{0};bytes[72..76].copy_from_slice(&(flags|cap_flags).to_ne_bytes());bytes[76..80].copy_from_slice(&cap_token.to_ne_bytes());
    let normals = crate::normals::matrix(&object.model).expect("Scene placements are checked before insertion or editing");
    for (index, value) in normals.iter().enumerate() {
        bytes[80 + index * 4..84 + index * 4].copy_from_slice(&value.to_ne_bytes());
    }
    if let Some(colour)=style.colour {for (index,value) in colour[..3].iter().copied().chain([1.]).enumerate() {bytes[144+index*4..148+index*4].copy_from_slice(&value.to_ne_bytes());}}
    bytes
}

impl GpuMesh {
    pub fn upload(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, object: &Object, selected: bool) -> Self {
        let geometry = Rc::new(GpuGeometry::upload(device, Rc::clone(&object.mesh)));
        Self::with_geometry(device, layout, object, Style {selected,pickable:!object.metadata.is_locked,colour:None}, geometry)
    }

    pub fn with_geometry(device: &wgpu::Device, layout: &wgpu::BindGroupLayout,
        object: &Object, style:Style, geometry: Rc<GpuGeometry>) -> Self
    {
        let bytes = settings(object, style,0,0,0);
        let model = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("object placement"),
            contents: &bytes,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let model_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("object placement"),
            layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: model.as_entire_binding() }],
        });
        Self { geometry, model_group, id: object.id, model, previous: bytes,primitive_base:0,xray:false,cap_flags:0,cap_token:0 }
    }

    pub(crate) fn model_buffer(&self)->&wgpu::Buffer{&self.model}
    pub(crate) fn packed(&self)->Record {self.previous}
    pub fn selected(&self)->bool {u32::from_ne_bytes(self.previous[64..68].try_into().unwrap())&1!=0}

    pub fn set_primitive_base(&mut self,queue:&wgpu::Queue,base:u32)->Result<(),&'static str>{
        if base>0x7fff_ffff{return Err("Primitive base exceeds its packed range");}
        if self.primitive_base!=base{
            let word=(base<<1)|u32::from(self.selected());
            self.previous[64..68].copy_from_slice(&word.to_ne_bytes());
            queue.write_buffer(&self.model,64,&self.previous[64..68]);self.primitive_base=base;
        }
        Ok(())
    }

    pub fn set_xray(&mut self,queue:&wgpu::Queue,on:bool){if self.xray==on{return;}self.xray=on;let flags=u32::from_ne_bytes(self.previous[72..76].try_into().unwrap());let flags=(flags&!32768)|if on&&flags&8192!=0{32768}else{0};self.previous[72..76].copy_from_slice(&flags.to_ne_bytes());queue.write_buffer(&self.model,72,&self.previous[72..76]);}
    pub fn closed(&self)->bool{!self.xray&&self.cap_flags&crate::solid_orientation::CLOSED!=0}
    pub fn pickable(&self)->bool{u32::from_ne_bytes(self.previous[68..72].try_into().unwrap())!=0}
    pub fn set_caps(&mut self,queue:&wgpu::Queue,flags:u32,token:u32){
        if self.cap_flags==flags&&self.cap_token==token{return;}let plane=u32::from_ne_bytes(self.previous[72..76].try_into().unwrap())&4096;self.previous[72..76].copy_from_slice(&(plane|flags|if self.xray&&flags&8192!=0{32768}else{0}).to_ne_bytes());self.previous[76..80].copy_from_slice(&token.to_ne_bytes());queue.write_buffer(&self.model,72,&self.previous[72..80]);self.cap_flags=flags;self.cap_token=token;
    }
    pub fn settings_bytes(&self) -> u64 { self.model.size() }

    pub fn update(&mut self, queue: &wgpu::Queue, object: &Object, style:Style) -> [usize; 2] {
        let next = settings(object, style,self.cap_flags|if self.xray&&self.cap_flags&8192!=0{32768}else{0},self.cap_token,self.primitive_base);
        let mut writes = [0, 0];
        for (start, end) in [(0, 64), (64, 80), (80, 144), (144,BYTES)] {
            if next[start..end] != self.previous[start..end] {
                queue.write_buffer(&self.model, start as u64, &next[start..end]);
                writes[0] += 1;
                writes[1] += end - start;
            }
        }
        self.previous = next;
        writes
    }

    pub fn bind_model(&self, pass: &mut wgpu::RenderPass<'_>) { pass.set_bind_group(1, &self.model_group, &[]); }

    pub fn draw_pulled(&self, pass: &mut wgpu::RenderPass<'_>) {
        self.bind_model(pass);
        self.geometry.draw_pulled(pass, 1);
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_bind_group(1, &self.model_group, &[]);
        self.geometry.draw(pass);
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzj-share-the-object-record-layout-3.rs"
```

```text
--8<-- "journey/code/96zzj-share-the-object-record-layout-4.rs"
```

```text
--8<-- "journey/code/96zzj-share-the-object-record-layout-5.rs"
```

```text
--8<-- "journey/code/96zzj-share-the-object-record-layout-6.rs"
```

```text
--8<-- "journey/code/96zzj-share-the-object-record-layout-7.rs"
```

```text
--8<-- "journey/code/96zzj-share-the-object-record-layout-8.rs"
```

</details>

### 4. `src/instance_gpu.rs`

Share the packed type and stride with the instance lane. Use the same size for minimum binding, capacity and cached instance records.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh};
use std::{collections::{BTreeMap,HashMap},rc::Rc};
use wgpu::util::DeviceExt;

struct Batch {geometry:Rc<GpuGeometry>,records:wgpu::Buffer,group:wgpu::BindGroup,bytes:Vec<u8>,count:u32}
pub struct Lane {pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<(usize,usize),Batch>,pub writes:u64,pub written_bytes:u64}
impl Lane {
    pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("shared instance records"),entries:&[wgpu::BindGroupLayoutEntry {binding:0,visibility:wgpu::ShaderStages::VERTEX,ty:wgpu::BindingType::Buffer {ty:wgpu::BufferBindingType::Storage {read_only:true},has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(160)},count:None}]});
        let source=include_str!("triangle.wgsl")
            .replace("var<uniform> object_settings: ObjectSettings;", "var<storage, read> objects: array<ObjectSettings>;")
            .replace("return object_settings;", "return objects[index];");
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(&source,2,0).into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(format.into()),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        Self {pipeline,layout,batches:BTreeMap::new(),writes:0,written_bytes:0}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=(u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size)/160) as usize;
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:HashMap<usize,(Rc<GpuGeometry>,Vec<[u8;160]>)>=HashMap::new();
        for row in rows {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push(row.packed());}
        let mut old=std::mem::take(&mut self.batches);let mut batches=BTreeMap::new();
        for (source,(geometry,records)) in groups {
            for (part,records) in records.chunks(limit).enumerate() {
                let key=(source,part);let bytes=records.iter().flatten().copied().collect::<Vec<_>>();
                let batch=if let Some(mut batch)=old.remove(&key).filter(|b|Rc::ptr_eq(&b.geometry,&geometry)&&b.records.size()>=bytes.len() as u64) {
                    if batch.bytes!=bytes {queue.write_buffer(&batch.records,0,&bytes);self.writes+=1;self.written_bytes+=bytes.len() as u64;batch.bytes=bytes;}batch.count=records.len() as u32;batch
                } else {
                    let buffer=device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:Some("independent source placements"),contents:&bytes,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST});let group=crate::gpu_layout::group(device,&self.layout,&buffer);self.writes+=1;self.written_bytes+=bytes.len() as u64;Batch {geometry:geometry.clone(),records:buffer,group,bytes,count:records.len() as u32}
                };batches.insert(key,batch);
            }
        }
        self.batches=batches;Ok(())
    }
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        for batch in self.batches.values() {pass.set_bind_group(1,&batch.group,&[]);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.batches.values().map(|b|b.records.size()).sum(),self.writes,self.written_bytes]}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh};
use std::{collections::{BTreeMap,HashMap},rc::Rc};
use wgpu::util::DeviceExt;
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,records:wgpu::Buffer,group:wgpu::BindGroup,bytes:Vec<u8>,count:u32}
pub struct Lane {pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<(usize,usize),Batch>,pub writes:u64,pub written_bytes:u64}
impl Lane {
    pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("shared instance records"),entries:&[wgpu::BindGroupLayoutEntry {binding:0,visibility:wgpu::ShaderStages::VERTEX,ty:wgpu::BindingType::Buffer {ty:wgpu::BufferBindingType::Storage {read_only:true},has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(BYTES as u64)},count:None}]});
        let source=include_str!("triangle.wgsl")
            .replace("var<uniform> object_settings: ObjectSettings;", "var<storage, read> objects: array<ObjectSettings>;")
            .replace("return object_settings;", "return objects[index];");
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(&source,2,0).into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(format.into()),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        Self {pipeline,layout,batches:BTreeMap::new(),writes:0,written_bytes:0}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=(u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size)/BYTES as u64) as usize;
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:HashMap<usize,(Rc<GpuGeometry>,Vec<Record>)>=HashMap::new();
        for row in rows {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push(row.packed());}
        let mut old=std::mem::take(&mut self.batches);let mut batches=BTreeMap::new();
        for (source,(geometry,records)) in groups {
            for (part,records) in records.chunks(limit).enumerate() {
                let key=(source,part);let bytes=records.iter().flatten().copied().collect::<Vec<_>>();
                let batch=if let Some(mut batch)=old.remove(&key).filter(|b|Rc::ptr_eq(&b.geometry,&geometry)&&b.records.size()>=bytes.len() as u64) {
                    if batch.bytes!=bytes {queue.write_buffer(&batch.records,0,&bytes);self.writes+=1;self.written_bytes+=bytes.len() as u64;batch.bytes=bytes;}batch.count=records.len() as u32;batch
                } else {
                    let buffer=device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:Some("independent source placements"),contents:&bytes,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST});let group=crate::gpu_layout::group(device,&self.layout,&buffer);self.writes+=1;self.written_bytes+=bytes.len() as u64;Batch {geometry:geometry.clone(),records:buffer,group,bytes,count:records.len() as u32}
                };batches.insert(key,batch);
            }
        }
        self.batches=batches;Ok(())
    }
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        for batch in self.batches.values() {pass.set_bind_group(1,&batch.group,&[]);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.batches.values().map(|b|b.records.size()).sum(),self.writes,self.written_bytes]}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzj-share-the-object-record-layout-9.rs"
```

```text
--8<-- "journey/code/96zzj-share-the-object-record-layout-10.rs"
```

```text
--8<-- "journey/code/96zzj-share-the-object-record-layout-11.rs"
```

```text
--8<-- "journey/code/96zzj-share-the-object-record-layout-12.rs"
```

</details>

### 5. `src/renderer.rs`

Declare the object binding from the shared host size.

<details>
<summary>Locate the existing block</summary>

```rust
        let model_layout = crate::gpu_layout::uniform(&device, 160);
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        let model_layout = crate::gpu_layout::uniform(&device, crate::object_record::BYTES as u64);
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzj-share-the-object-record-layout-13.rs"
```

</details>

### 6. `src/face_pick.rs`

Declare the object binding from the shared host size.

<details>
<summary>Locate the existing block</summary>

```rust
        let model = crate::gpu_layout::uniform(device, 160); let crop = crate::gpu_layout::uniform(device, 80);
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        let model = crate::gpu_layout::uniform(device, crate::object_record::BYTES as u64); let crop = crate::gpu_layout::uniform(device, 80);
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzj-share-the-object-record-layout-14.rs"
```

</details>

### 7. `src/cap_pipeline.rs`

Declare the object binding from the shared host size.

<details>
<summary>Locate the existing block</summary>

```rust
  let view=crate::gpu_layout::uniform(device,80);let model=crate::gpu_layout::uniform(device,160);let clip=crate::clipping_gpu::layout(device);let frame=crate::gpu_layout::uniform(device,16);let counts=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("section crossing texture"),entries:&[crate::text_gpu_support::texture_entry(0,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Float{filterable:false},false)]});
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
  let view=crate::gpu_layout::uniform(device,80);let model=crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64);let clip=crate::clipping_gpu::layout(device);let frame=crate::gpu_layout::uniform(device,16);let counts=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("section crossing texture"),entries:&[crate::text_gpu_support::texture_entry(0,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Float{filterable:false},false)]});
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzj-share-the-object-record-layout-15.rs"
```

</details>

### 8. `src/cap_pick_pipeline.rs`

Declare the object binding from the shared host size.

<details>
<summary>Locate the existing block</summary>

```rust
  let frame=crate::gpu_layout::uniform(device,32);let clip=crate::clipping_gpu::layout(device);let view=crate::gpu_layout::uniform(device,80);let model=crate::gpu_layout::uniform(device,160);
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
  let frame=crate::gpu_layout::uniform(device,32);let clip=crate::clipping_gpu::layout(device);let view=crate::gpu_layout::uniform(device,80);let model=crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64);
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzj-share-the-object-record-layout-16.rs"
```

</details>

### 9. `src/ambient_source_pipeline.rs`

Declare the object binding from the shared host size.

<details>
<summary>Locate the existing block</summary>

```rust
  let view=crate::gpu_layout::uniform(device,80);let model=crate::gpu_layout::uniform(device,160);let clip=crate::clipping_gpu::layout(device);let radius=crate::gpu_layout::uniform(device,16);let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("ambient original source metadata"),bind_group_layouts:&[Some(&view),Some(&model),Some(&clip),Some(&radius)],immediate_size:0});let module=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("ambient original normals and radii"),source:wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(include_str!("ambient_sources.wgsl"),2,0).into())});
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
  let view=crate::gpu_layout::uniform(device,80);let model=crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64);let clip=crate::clipping_gpu::layout(device);let radius=crate::gpu_layout::uniform(device,16);let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("ambient original source metadata"),bind_group_layouts:&[Some(&view),Some(&model),Some(&clip),Some(&radius)],immediate_size:0});let module=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("ambient original normals and radii"),source:wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(include_str!("ambient_sources.wgsl"),2,0).into())});
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzj-share-the-object-record-layout-17.rs"
```

</details>

### 10. `src/outline_gpu.rs`

Declare the object binding from the shared host size.

<details>
<summary>Locate the existing block</summary>

```rust
  let model = crate::gpu_layout::uniform(device, 160);
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
  let model = crate::gpu_layout::uniform(device, crate::object_record::BYTES as u64);
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzj-share-the-object-record-layout-18.rs"
```

</details>

### 11. `src/renderer.rs`

Count allocated object records using their actual shared size.

<details>
<summary>Locate the existing block</summary>

```rust
        self.instances.written_bytes + self.geometry.uploaded_bytes + self.settings_allocations as u64 * 160 + self.settings_bytes as u64
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        self.instances.written_bytes + self.geometry.uploaded_bytes + self.settings_allocations as u64 * crate::object_record::BYTES as u64 + self.settings_bytes as u64
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzj-share-the-object-record-layout-19.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 12. `src/object_record_tests.rs`

Supply a check against the actual assembled WGSL layouts, not a repeated Rust size assertion.

Copy this supplied file:

```text
--8<-- "journey/code/96zzj-share-the-object-record-layout-20.rs"
```

</details>

### 13. `src/lib.rs`

Register the supplied shader-layout fixture.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod object_record;
```

</details>

After that block, type:

```rust
#[cfg(test)]
mod object_record_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzj-share-the-object-record-layout-21.rs"
```

</details>

</details>

<a id="checkpoint-96zzk-borrow-the-current-arena-buffers"></a>

### Step 5 of 14 · Borrow the current arena buffers (`96zzk-borrow-the-current-arena-buffers`)

- **Goal:** Use one geometry interface for standalone and current shared storage.
- **Data Flow Trace:** `geometry lease → arena owner → current vertex, index and binding handles`
- **Files You Touch:** `src/gpu_geometry.rs` (*Borrow one shared arena owner instead of retaining an old buffer generation.*) · `src/face_pick.rs` (*Build source-face bindings from the current shared handles.*) · `src/tile_gpu.rs` (*Invalidate projected rows when the shared buffer handles change.*) · `src/caps_gpu.rs` (*Reuse the indexed range draw while preserving the plane instance.*) · `src/cap_pick_gpu.rs` (*Reuse the indexed range draw while preserving the plane instance.*) · `src/global_outline_tests.rs` (*Keep the supplied actual compact-buffer allocation assertion through the common accessor.*) · `src/geometry_storage_tests.rs` (*Supply a real GPU growth, current-handle and weak-source ownership exercise.*) · `src/lib.rs` (*Register the supplied shared-storage ownership fixture.*)

A geometry can own standalone preview buffers or borrow the shared arena. Keep one interface for both. Borrow its current buffers and bind group when drawing, so growth does not pin an obsolete generation. Shared range addressing connects in the following lesson.

> **Self-Check Question:** *Why can a geometry retain its source without pinning an old GPU buffer generation?*  
> **Answer:** It retains the original mesh and a shared arena owner. Buffer and binding handles are read from that owner when needed.

- **Hands-On Lab Experiment:** Inspect the unchanged browser scene. In the supplied storage exercise, compare an earlier geometry’s current buffer handles before and after appending a second source. Offset pulled draws connect next.

<details>
<summary>Implementation 96zzk-borrow-the-current-arena-buffers: Borrow the current arena buffers</summary>

Use one geometry interface for standalone and current shared storage.

Data flow: geometry lease → arena owner → current vertex, index and binding handles.

### 1. `src/gpu_geometry.rs`

Borrow one shared arena owner instead of retaining an old buffer generation. Retain a source range separately from its storage owner. Keep standalone preview storage and shared source storage in one geometry interface. Keep the existing standalone upload path at range zero. Upload through the existing arena and read its latest handles. Count shared storage at its owner, not once per geometry. Read the current shared binding on every pulled draw. Use one indexed-range draw for ordinary passes and per-plane cap instances.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::mesh::Mesh;
use std::rc::Rc;
use wgpu::util::DeviceExt;

pub struct GpuGeometry {
    pub source: Rc<Mesh>,
    pub(crate) vertices: wgpu::Buffer,
    pub(crate) indices: wgpu::Buffer,
    pub(crate) index_count: u32,
    pull_group: wgpu::BindGroup,
}

impl GpuGeometry {
    pub fn storage_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        let entries = [0, 2].map(|binding| wgpu::BindGroupLayoutEntry {
            binding, visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false, min_binding_size: std::num::NonZeroU64::new(if binding==0 {20}else{4}),
            }, count: None,
        });
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("shared geometry storage"), entries: &entries,
        })
    }

    pub fn upload(device: &wgpu::Device, source: Rc<Mesh>) -> Self {
        fn buffer(device: &wgpu::Device, label: &str, mut bytes: Vec<u8>, usage: wgpu::BufferUsages) -> wgpu::Buffer {
            if bytes.is_empty() { bytes.resize(4, 0); }
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(label), contents: &bytes, usage: usage | wgpu::BufferUsages::STORAGE,
            })
        }
        let mut bytes = crate::packed_vertex::bytes(&source);
        if bytes.is_empty() { bytes.resize(20, 0); }
        let vertices = buffer(device, "shared mesh vertices", bytes, wgpu::BufferUsages::VERTEX);
        let bytes = source.indices().iter().flat_map(|v| v.to_ne_bytes()).collect();
        let indices = buffer(device, "shared mesh indices", bytes, wgpu::BufferUsages::INDEX);
        let entries = [(0, &vertices), (2, &indices)].map(|(binding, buffer)| wgpu::BindGroupEntry {
            binding, resource: buffer.as_entire_binding(),
        });
        let pull_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("shared vertex reads"), layout: &Self::storage_layout(device), entries: &entries,
        });
        let index_count = source.indices().len() as u32;
        Self { source, vertices, indices, index_count, pull_group }
    }

    pub fn allocated_bytes(&self) -> u64 { self.vertices.size() + self.indices.size() }

    pub fn draw_pulled(&self, pass: &mut wgpu::RenderPass<'_>, count: u32) {
        if self.index_count == 0 || count == 0 { return; }
        pass.set_bind_group(3, &self.pull_group, &[]);
        pass.draw(0..self.index_count, 0..count);
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {self.draw_instances(pass,1);}

    pub fn draw_instances(&self, pass: &mut wgpu::RenderPass<'_>,count:u32) {
        if self.index_count==0||count==0{return;}
        pass.set_vertex_buffer(0, self.vertices.slice(..));
        pass.set_index_buffer(self.indices.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.index_count, 0, 0..count);
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::mesh::Mesh;
use std::{cell::RefCell,rc::Rc};
use wgpu::util::DeviceExt;

pub struct GpuGeometry {
    pub source: Rc<Mesh>,
    pub(crate) index_count: u32,
    pub(crate) index_start: u32,
    storage: Storage,
}

enum Storage {
    Owned {vertices:wgpu::Buffer,indices:wgpu::Buffer,group:wgpu::BindGroup},
    Shared(Rc<RefCell<crate::mesh_arena::Arena>>),
}

impl GpuGeometry {
    pub fn storage_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        let entries = [0, 2].map(|binding| wgpu::BindGroupLayoutEntry {
            binding, visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false, min_binding_size: std::num::NonZeroU64::new(if binding==0 {20}else{4}),
            }, count: None,
        });
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("shared geometry storage"), entries: &entries,
        })
    }

    pub fn upload(device: &wgpu::Device, source: Rc<Mesh>) -> Self {
        fn buffer(device: &wgpu::Device, label: &str, mut bytes: Vec<u8>, usage: wgpu::BufferUsages) -> wgpu::Buffer {
            if bytes.is_empty() { bytes.resize(4, 0); }
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(label), contents: &bytes, usage: usage | wgpu::BufferUsages::STORAGE,
            })
        }
        let mut bytes = crate::packed_vertex::bytes(&source);
        if bytes.is_empty() { bytes.resize(20, 0); }
        let vertices = buffer(device, "shared mesh vertices", bytes, wgpu::BufferUsages::VERTEX);
        let bytes = source.indices().iter().flat_map(|v| v.to_ne_bytes()).collect();
        let indices = buffer(device, "shared mesh indices", bytes, wgpu::BufferUsages::INDEX);
        let entries = [(0, &vertices), (2, &indices)].map(|(binding, buffer)| wgpu::BindGroupEntry {
            binding, resource: buffer.as_entire_binding(),
        });
        let pull_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("shared vertex reads"), layout: &Self::storage_layout(device), entries: &entries,
        });
        let index_count = source.indices().len() as u32;
        Self {source,index_count,index_start:0,storage:Storage::Owned {vertices,indices,group:pull_group}}
    }

    pub fn shared(ctx:&crate::text_gpu_support::GpuCtx,source:Rc<Mesh>,arena:Rc<RefCell<crate::mesh_arena::Arena>>)->Result<Self,&'static str>{
        let range=arena.borrow_mut().upload(ctx,&source)?;
        Ok(Self {source,index_count:range.indices.len()as u32,index_start:range.indices.start,storage:Storage::Shared(arena)})
    }
    pub(crate) fn buffers(&self)->(wgpu::Buffer,wgpu::Buffer){
        match &self.storage {
            Storage::Owned {vertices,indices,..}=>(vertices.clone(),indices.clone()),
            Storage::Shared(arena)=>{let arena=arena.borrow();(arena.vertices.buf.clone(),arena.indices.buf.clone())},
        }
    }
    fn group(&self)->wgpu::BindGroup{
        match &self.storage {Storage::Owned {group,..}=>group.clone(),Storage::Shared(arena)=>arena.borrow().group.clone()}
    }
    pub fn allocated_bytes(&self)->u64{
        match &self.storage {Storage::Owned {vertices,indices,..}=>vertices.size()+indices.size(),Storage::Shared(_)=>0}
    }

    pub fn draw_pulled(&self, pass: &mut wgpu::RenderPass<'_>, count: u32) {
        if self.index_count == 0 || count == 0 { return; }
        pass.set_bind_group(3, &self.group(), &[]);
        pass.draw(0..self.index_count, 0..count);
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {self.draw_instances(pass,1);}

    pub fn draw_instances(&self,pass:&mut wgpu::RenderPass<'_>,count:u32){self.draw_range(pass,0..count);}
    pub(crate) fn draw_range(&self,pass:&mut wgpu::RenderPass<'_>,instances:std::ops::Range<u32>){
        if self.index_count==0||instances.is_empty(){return;}
        let(vertices,indices)=self.buffers();
        pass.set_vertex_buffer(0,vertices.slice(..));pass.set_index_buffer(indices.slice(..),wgpu::IndexFormat::Uint32);
        pass.draw_indexed(self.index_start..self.index_start+self.index_count,0,instances);
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzk-borrow-the-current-arena-buffers-1.rs"
```

```text
--8<-- "journey/code/96zzk-borrow-the-current-arena-buffers-2.rs"
```

```text
--8<-- "journey/code/96zzk-borrow-the-current-arena-buffers-3.rs"
```

```text
--8<-- "journey/code/96zzk-borrow-the-current-arena-buffers-4.rs"
```

```text
--8<-- "journey/code/96zzk-borrow-the-current-arena-buffers-5.rs"
```

```text
--8<-- "journey/code/96zzk-borrow-the-current-arena-buffers-6.rs"
```

```text
--8<-- "journey/code/96zzk-borrow-the-current-arena-buffers-7.rs"
```

</details>

### 2. `src/face_pick.rs`

Build source-face bindings from the current shared handles.

<details>
<summary>Locate the existing block</summary>

```rust
            let buffers = [&geometry.vertices, &geometry.indices, &faces, &settings,clipping];
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
            let(vertices,indices)=geometry.buffers();
            let buffers = [&vertices, &indices, &faces, &settings,clipping];
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzk-borrow-the-current-arena-buffers-8.rs"
```

</details>

### 3. `src/tile_gpu.rs`

Invalidate projected rows when the shared buffer handles change. Use current shared buffers when preparing projection rows.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{gpu_mesh::GpuMesh,tile_math::{TileLayout,PROJECTED_BYTES,projected_records,next_pool_words,project_groups},tile_pipelines::{Pipelines,group},tile_report::PoolReport};
use wgpu::util::DeviceExt;
type Witness=(wgpu::Buffer,wgpu::Buffer,wgpu::Buffer,[u8;64]);
struct Row{vertices:wgpu::Buffer,indices:wgpu::Buffer,model:wgpu::Buffer,count:wgpu::Buffer,triangles:u32,group:Option<wgpu::BindGroup>}
pub struct Tiles{pub projected:wgpu::Buffer,pub buffer:wgpu::Buffer,pipes:Pipelines,line:wgpu::Buffer,line_group:wgpu::BindGroup,raster:wgpu::BindGroup,scan:wgpu::BindGroup,rows:Vec<Row>,witness:Vec<Witness>,triangles:u32,viewport:[u32;2],layout:Option<TileLayout>,target:Option<wgpu::TextureView>,pool:u64,report:PoolReport,key:Option<[f32;16]>,pub projections:u64,pub bins:u64,reported:Option<u64>,pub fallback:bool}
fn buffer(device:&wgpu::Device,label:&str,size:u64)->wgpu::Buffer{device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::COPY_SRC,mapped_at_creation:false})}
impl Tiles{
 pub fn new(device:&wgpu::Device,view:&wgpu::BindGroupLayout)->Self{
  let pipes=Pipelines::new(device,view);let projected=buffer(device,"finite projected placeholder",PROJECTED_BYTES);let buffer=buffer(device,"finite tile placeholder",16);
  let line=device.create_buffer(&wgpu::BufferDescriptor{label:Some("finite view dimensions"),size:32,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});
  let line_group=group(device,&pipes.line,&[&line]);let raster=group(device,&pipes.raster_layout,&[&projected,&buffer]);let scan=group(device,&pipes.scan_layout,&[&buffer]);
  Self{projected,buffer,pipes,line,line_group,raster,scan,rows:Vec::new(),witness:Vec::new(),triangles:0,viewport:[0;2],layout:None,target:None,pool:0,report:PoolReport::new(device),key:None,projections:0,bins:0,reported:None,fallback:false}
 }
 pub fn set(&mut self,device:&wgpu::Device,meshes:&[GpuMesh]){
  let limit=u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size);
  let count=meshes.iter().try_fold(0u32,|count,row|count.checked_add(row.geometry.index_count/3));
  self.fallback=count.is_none_or(|count|u64::from(count)*PROJECTED_BYTES>limit);
  if self.fallback{self.rows.clear();self.witness.clear();self.triangles=0;self.key=None;return}
  let witness=meshes.iter().filter(|row|row.geometry.index_count>0).map(|row|(row.geometry.vertices.clone(),row.geometry.indices.clone(),row.model_buffer().clone(),row.packed()[..64].try_into().unwrap())).collect::<Vec<_>>();
  if witness==self.witness{return}
  self.rows.clear();let mut offset=0;
  for row in meshes.iter().filter(|row|row.geometry.index_count>0){let triangles=row.geometry.index_count/3;
   let bytes=[triangles,offset,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();
   let count=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("finite original triangle interval"),contents:&bytes,usage:wgpu::BufferUsages::UNIFORM});
   self.rows.push(Row{vertices:row.geometry.vertices.clone(),indices:row.geometry.indices.clone(),model:row.model_buffer().clone(),count,triangles,group:None});offset+=triangles;
  }
  self.witness=witness;self.triangles=count.unwrap();self.key=None;
 }
 fn rebind(&mut self,device:&wgpu::Device){self.raster=group(device,&self.pipes.raster_layout,&[&self.projected,&self.buffer]);self.scan=group(device,&self.pipes.scan_layout,&[&self.buffer]);for row in &mut self.rows{row.group=None;}}
 fn release(&mut self,device:&wgpu::Device){if self.layout.take().is_none()&&self.projected.size()==PROJECTED_BYTES&&self.buffer.size()==16{return}self.projected=buffer(device,"finite projected placeholder",PROJECTED_BYTES);self.buffer=buffer(device,"finite tile placeholder",16);self.pool=0;self.target=None;self.key=None;self.rebind(device);}
 pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,size:[u32;2]){
  if self.triangles==0||size.contains(&0){self.release(device);return}
  let layout=TileLayout::new((size[0],size[1]));let limit=u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size);
  let report=self.report.poll().filter(|_|self.layout==Some(layout));let capacity=self.buffer.size()/4;
  let ceiling=layout.max_pool_words().min(limit.saturating_sub(layout.header_records()*16)/4);
  let pool=next_pool_words(self.pool,layout.initial_pool_words(self.triangles),capacity,report,ceiling);
  if pool<layout.initial_pool_words(self.triangles)||layout.header_records()*16>limit{self.fallback=true;self.release(device);return}
  let mut moved=false;
  if let Some(records)=projected_records(u64::from(self.triangles),self.projected.size()/PROJECTED_BYTES,limit/PROJECTED_BYTES){self.projected=buffer(device,"finite projected triangles",records*PROJECTED_BYTES);moved=true;}
  if self.layout!=Some(layout)||pool>self.pool{self.buffer=buffer(device,"finite triangle tile lists",layout.buffer_bytes(pool));self.pool=pool;moved=true;}
  if self.layout!=Some(layout){self.target=Some(device.create_texture(&wgpu::TextureDescriptor{label:Some("finite tile raster"),size:wgpu::Extent3d{width:layout.dimensions()[0],height:layout.dimensions()[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::R8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]}).create_view(&Default::default()));}
  if self.viewport!=size{let bytes=[0.,0.,0.,size[1]as f32,size[0]as f32,0.,0.,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();queue.write_buffer(&self.line,0,&bytes);self.viewport=size;self.key=None;}
  self.layout=Some(layout);if moved{self.key=None;self.rebind(device);}
 }
 pub fn encode(&mut self,device:&wgpu::Device,encoder:&mut wgpu::CommandEncoder,view:&wgpu::BindGroup,matrix:[f32;16]){
  let Some(layout)=self.layout else{return};if self.key==Some(matrix){if self.reported!=Some(self.bins)&&self.report.copy(encoder,&self.buffer){self.reported=Some(self.bins)}return}
  for row in &mut self.rows{let bindings=row.group.get_or_insert_with(||group(device,&self.pipes.project_layout,&[&row.vertices,&row.indices,&row.model,&self.projected,&row.count]));let mut pass=encoder.begin_compute_pass(&wgpu::ComputePassDescriptor{label:Some("finite original projection"),timestamp_writes:None});pass.set_pipeline(&self.pipes.project);pass.set_bind_group(0,view,&[]);pass.set_bind_group(1,&self.line_group,&[]);pass.set_bind_group(2,&*bindings,&[]);let(x,y)=project_groups(row.triangles);pass.dispatch_workgroups(x,y,1);}
  self.projections+=1;encoder.clear_buffer(&self.buffer,0,Some(layout.header_records()*16));self.bin(encoder,&self.pipes.count);
  let blocks=layout.count().div_ceil(256);
  for(pipeline,count)in self.pipes.scans.iter().zip([blocks,blocks.div_ceil(256),blocks]){let mut pass=encoder.begin_compute_pass(&wgpu::ComputePassDescriptor{label:Some("finite prefix scan"),timestamp_writes:None});pass.set_pipeline(pipeline);pass.set_bind_group(0,&self.line_group,&[]);pass.set_bind_group(1,&self.scan,&[]);pass.dispatch_workgroups(count,1,1);}
  self.bin(encoder,&self.pipes.fill);self.bins+=1;if self.report.copy(encoder,&self.buffer){self.reported=Some(self.bins)}self.key=Some(matrix);
 }
 fn bin(&self,encoder:&mut wgpu::CommandEncoder,pipeline:&wgpu::RenderPipeline){let Some(target)=&self.target else{return};let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite tile lists"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:target,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Discard}})],depth_stencil_attachment:None,timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});pass.set_pipeline(pipeline);pass.set_bind_group(0,&self.line_group,&[]);pass.set_bind_group(1,&self.raster,&[]);pass.draw(0..4,0..self.triangles);}
 pub fn map_report(&mut self){self.report.map();}
 pub fn usage(&self)->[u64;7]{[self.projected.size(),self.buffer.size(),32+16+16*self.rows.len()as u64,self.triangles as u64,self.pool,self.projections,self.bins]}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{gpu_mesh::GpuMesh,tile_math::{TileLayout,PROJECTED_BYTES,projected_records,next_pool_words,project_groups},tile_pipelines::{Pipelines,group},tile_report::PoolReport};
use wgpu::util::DeviceExt;
type Witness=(wgpu::Buffer,wgpu::Buffer,wgpu::Buffer,[u8;64]);
struct Row{vertices:wgpu::Buffer,indices:wgpu::Buffer,model:wgpu::Buffer,count:wgpu::Buffer,triangles:u32,group:Option<wgpu::BindGroup>}
pub struct Tiles{pub projected:wgpu::Buffer,pub buffer:wgpu::Buffer,pipes:Pipelines,line:wgpu::Buffer,line_group:wgpu::BindGroup,raster:wgpu::BindGroup,scan:wgpu::BindGroup,rows:Vec<Row>,witness:Vec<Witness>,triangles:u32,viewport:[u32;2],layout:Option<TileLayout>,target:Option<wgpu::TextureView>,pool:u64,report:PoolReport,key:Option<[f32;16]>,pub projections:u64,pub bins:u64,reported:Option<u64>,pub fallback:bool}
fn buffer(device:&wgpu::Device,label:&str,size:u64)->wgpu::Buffer{device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::COPY_SRC,mapped_at_creation:false})}
impl Tiles{
 pub fn new(device:&wgpu::Device,view:&wgpu::BindGroupLayout)->Self{
  let pipes=Pipelines::new(device,view);let projected=buffer(device,"finite projected placeholder",PROJECTED_BYTES);let buffer=buffer(device,"finite tile placeholder",16);
  let line=device.create_buffer(&wgpu::BufferDescriptor{label:Some("finite view dimensions"),size:32,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});
  let line_group=group(device,&pipes.line,&[&line]);let raster=group(device,&pipes.raster_layout,&[&projected,&buffer]);let scan=group(device,&pipes.scan_layout,&[&buffer]);
  Self{projected,buffer,pipes,line,line_group,raster,scan,rows:Vec::new(),witness:Vec::new(),triangles:0,viewport:[0;2],layout:None,target:None,pool:0,report:PoolReport::new(device),key:None,projections:0,bins:0,reported:None,fallback:false}
 }
 pub fn set(&mut self,device:&wgpu::Device,meshes:&[GpuMesh]){
  let limit=u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size);
  let count=meshes.iter().try_fold(0u32,|count,row|count.checked_add(row.geometry.index_count/3));
  self.fallback=count.is_none_or(|count|u64::from(count)*PROJECTED_BYTES>limit);
  if self.fallback{self.rows.clear();self.witness.clear();self.triangles=0;self.key=None;return}
  let witness=meshes.iter().filter(|row|row.geometry.index_count>0).map(|row|{let(vertices,indices)=row.geometry.buffers();(vertices,indices,row.model_buffer().clone(),row.packed()[..64].try_into().unwrap())}).collect::<Vec<_>>();
  if witness==self.witness{return}
  self.rows.clear();let mut offset=0;
  for row in meshes.iter().filter(|row|row.geometry.index_count>0){let triangles=row.geometry.index_count/3;
   let bytes=[triangles,offset,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();
   let count=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("finite original triangle interval"),contents:&bytes,usage:wgpu::BufferUsages::UNIFORM});
   self.rows.push(Row{vertices:row.geometry.buffers().0,indices:row.geometry.buffers().1,model:row.model_buffer().clone(),count,triangles,group:None});offset+=triangles;
  }
  self.witness=witness;self.triangles=count.unwrap();self.key=None;
 }
 fn rebind(&mut self,device:&wgpu::Device){self.raster=group(device,&self.pipes.raster_layout,&[&self.projected,&self.buffer]);self.scan=group(device,&self.pipes.scan_layout,&[&self.buffer]);for row in &mut self.rows{row.group=None;}}
 fn release(&mut self,device:&wgpu::Device){if self.layout.take().is_none()&&self.projected.size()==PROJECTED_BYTES&&self.buffer.size()==16{return}self.projected=buffer(device,"finite projected placeholder",PROJECTED_BYTES);self.buffer=buffer(device,"finite tile placeholder",16);self.pool=0;self.target=None;self.key=None;self.rebind(device);}
 pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,size:[u32;2]){
  if self.triangles==0||size.contains(&0){self.release(device);return}
  let layout=TileLayout::new((size[0],size[1]));let limit=u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size);
  let report=self.report.poll().filter(|_|self.layout==Some(layout));let capacity=self.buffer.size()/4;
  let ceiling=layout.max_pool_words().min(limit.saturating_sub(layout.header_records()*16)/4);
  let pool=next_pool_words(self.pool,layout.initial_pool_words(self.triangles),capacity,report,ceiling);
  if pool<layout.initial_pool_words(self.triangles)||layout.header_records()*16>limit{self.fallback=true;self.release(device);return}
  let mut moved=false;
  if let Some(records)=projected_records(u64::from(self.triangles),self.projected.size()/PROJECTED_BYTES,limit/PROJECTED_BYTES){self.projected=buffer(device,"finite projected triangles",records*PROJECTED_BYTES);moved=true;}
  if self.layout!=Some(layout)||pool>self.pool{self.buffer=buffer(device,"finite triangle tile lists",layout.buffer_bytes(pool));self.pool=pool;moved=true;}
  if self.layout!=Some(layout){self.target=Some(device.create_texture(&wgpu::TextureDescriptor{label:Some("finite tile raster"),size:wgpu::Extent3d{width:layout.dimensions()[0],height:layout.dimensions()[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::R8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]}).create_view(&Default::default()));}
  if self.viewport!=size{let bytes=[0.,0.,0.,size[1]as f32,size[0]as f32,0.,0.,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();queue.write_buffer(&self.line,0,&bytes);self.viewport=size;self.key=None;}
  self.layout=Some(layout);if moved{self.key=None;self.rebind(device);}
 }
 pub fn encode(&mut self,device:&wgpu::Device,encoder:&mut wgpu::CommandEncoder,view:&wgpu::BindGroup,matrix:[f32;16]){
  let Some(layout)=self.layout else{return};if self.key==Some(matrix){if self.reported!=Some(self.bins)&&self.report.copy(encoder,&self.buffer){self.reported=Some(self.bins)}return}
  for row in &mut self.rows{let bindings=row.group.get_or_insert_with(||group(device,&self.pipes.project_layout,&[&row.vertices,&row.indices,&row.model,&self.projected,&row.count]));let mut pass=encoder.begin_compute_pass(&wgpu::ComputePassDescriptor{label:Some("finite original projection"),timestamp_writes:None});pass.set_pipeline(&self.pipes.project);pass.set_bind_group(0,view,&[]);pass.set_bind_group(1,&self.line_group,&[]);pass.set_bind_group(2,&*bindings,&[]);let(x,y)=project_groups(row.triangles);pass.dispatch_workgroups(x,y,1);}
  self.projections+=1;encoder.clear_buffer(&self.buffer,0,Some(layout.header_records()*16));self.bin(encoder,&self.pipes.count);
  let blocks=layout.count().div_ceil(256);
  for(pipeline,count)in self.pipes.scans.iter().zip([blocks,blocks.div_ceil(256),blocks]){let mut pass=encoder.begin_compute_pass(&wgpu::ComputePassDescriptor{label:Some("finite prefix scan"),timestamp_writes:None});pass.set_pipeline(pipeline);pass.set_bind_group(0,&self.line_group,&[]);pass.set_bind_group(1,&self.scan,&[]);pass.dispatch_workgroups(count,1,1);}
  self.bin(encoder,&self.pipes.fill);self.bins+=1;if self.report.copy(encoder,&self.buffer){self.reported=Some(self.bins)}self.key=Some(matrix);
 }
 fn bin(&self,encoder:&mut wgpu::CommandEncoder,pipeline:&wgpu::RenderPipeline){let Some(target)=&self.target else{return};let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite tile lists"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:target,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Discard}})],depth_stencil_attachment:None,timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});pass.set_pipeline(pipeline);pass.set_bind_group(0,&self.line_group,&[]);pass.set_bind_group(1,&self.raster,&[]);pass.draw(0..4,0..self.triangles);}
 pub fn map_report(&mut self){self.report.map();}
 pub fn usage(&self)->[u64;7]{[self.projected.size(),self.buffer.size(),32+16+16*self.rows.len()as u64,self.triangles as u64,self.pool,self.projections,self.bins]}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzk-borrow-the-current-arena-buffers-9.rs"
```

```text
--8<-- "journey/code/96zzk-borrow-the-current-arena-buffers-10.rs"
```

</details>

### 4. `src/caps_gpu.rs`

Reuse the indexed range draw while preserving the plane instance.

<details>
<summary>Locate the existing block</summary>

```rust
   {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original closed-solid crossings"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&counts.image.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&pipes.count);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clipping,&[]);for mesh in meshes.iter().filter(|m|m.closed()&&self.targets.get(&m.id.value()).is_some_and(|mask|mask&(1<<plane)!=0)){mesh.bind_model(&mut pass);pass.set_vertex_buffer(0,mesh.geometry.vertices.slice(..));pass.set_index_buffer(mesh.geometry.indices.slice(..),wgpu::IndexFormat::Uint32);pass.draw_indexed(0..mesh.geometry.index_count,0,plane..plane+1);self.draws+=1;}}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
   {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original closed-solid crossings"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&counts.image.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&pipes.count);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clipping,&[]);for mesh in meshes.iter().filter(|m|m.closed()&&self.targets.get(&m.id.value()).is_some_and(|mask|mask&(1<<plane)!=0)){mesh.bind_model(&mut pass);mesh.geometry.draw_range(&mut pass,plane..plane+1);self.draws+=1;}}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzk-borrow-the-current-arena-buffers-11.rs"
```

</details>

### 5. `src/cap_pick_gpu.rs`

Reuse the indexed range draw while preserving the plane instance.

<details>
<summary>Locate the existing block</summary>

```rust
  for pipeline in [&self.pipes.count,&self.pipes.nearest]{let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section count and nearest owner"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&target.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(pipeline);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clipping,&[]);pass.set_bind_group(3,self.record_group.as_ref().unwrap(),&[]);for plane in 0..planes{for mesh in meshes.iter().filter(|m|m.closed()&&targets.get(&m.id.value()).is_some_and(|mask|mask&(1<<plane)!=0)){mesh.bind_model(&mut pass);pass.set_vertex_buffer(0,mesh.geometry.vertices.slice(..));pass.set_index_buffer(mesh.geometry.indices.slice(..),wgpu::IndexFormat::Uint32);pass.draw_indexed(0..mesh.geometry.index_count,0,plane..plane+1);}}}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
  for pipeline in [&self.pipes.count,&self.pipes.nearest]{let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section count and nearest owner"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&target.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(pipeline);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clipping,&[]);pass.set_bind_group(3,self.record_group.as_ref().unwrap(),&[]);for plane in 0..planes{for mesh in meshes.iter().filter(|m|m.closed()&&targets.get(&m.id.value()).is_some_and(|mask|mask&(1<<plane)!=0)){mesh.bind_model(&mut pass);mesh.geometry.draw_range(&mut pass,plane..plane+1);}}}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzk-borrow-the-current-arena-buffers-12.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/global_outline_tests.rs`

Keep the supplied actual compact-buffer allocation assertion through the common accessor.

<details>
<summary>Locate the existing block</summary>

```text
gpu.meshes[0].geometry.vertices.size()
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzk-borrow-the-current-arena-buffers-13.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/geometry_storage_tests.rs`

Supply a real GPU growth, current-handle and weak-source ownership exercise.

Copy this supplied file:

```text
--8<-- "journey/code/96zzk-borrow-the-current-arena-buffers-14.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 8. `src/lib.rs`

Register the supplied shared-storage ownership fixture.

<details>
<summary>Locate the existing block</summary>

```text
pub mod object_record;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzk-borrow-the-current-arena-buffers-15.rs" skip=22
```

</details>

</details>

<a id="checkpoint-96zzl-read-shared-ranges-through-object-records"></a>

### Step 6 of 14 · Read shared ranges through object records (`96zzl-read-shared-ranges-through-object-records`)

- **Goal:** Read an arena range without changing original owners or local triangle identity.
- **Data Flow Trace:** `object range start → shared index → shared packed vertex → original draw and triangle owner`
- **Files You Touch:** `src/object_record.rs` (*Append an aligned four-word range block after the existing object fields.*) · `src/gpu_mesh.rs` (*Pack the range after the existing model, flags, normal matrix and override.*) · `src/triangle.wgsl` (*Match the appended aligned host range block.*) · `src/cap_count.wgsl` (*Match the appended aligned host range block.*) · `src/outline_mask.wgsl` (*Keep the object layout consistent in indexed surface consumers.*) · `src/ambient_sources.wgsl` (*Keep the object layout consistent in indexed surface consumers.*) · `src/finite_project.wgsl` (*Read the same object range in physical triangle projection.*) · `src/triangle_pick.wgsl` (*Read the same shared range for first surface picks.*) · `src/face_pick.wgsl` (*Keep original face metadata local while pulling shared positions.*) · `src/object_record_tests.rs` (*Match the new range field in both supplied surface shader layout assertions.*) · `src/global_outline_tests.rs` (*Supply an actual nonzero-range renderer comparison for surface color, original face picks, projection, outlines and clipped caps.*) · `src/pick_gpu_tests.rs` (*Keep the exact supplied GPU allocation check: two 176-byte object records occupy 352 bytes.*)

An arena index addresses a global vertex, while a draw still names its own local triangle. Keep the first index in the object record and use it only for mesh reads.

Let geometry borrow the arena owner instead of retaining buffer generations. Read its latest buffers and bind group after growth. Surface and pick shaders add the range start; indexed caps, outlines and ambient drawing use the matching index interval. The application cache remains per geometry until the next connection.

> **Self-Check Question:** *Why must an old geometry read the arena’s latest binding after growth?*  
> **Answer:** A retained binding reads the old buffers. Borrowing the shared owner gives every draw the same current buffers, including earlier ranges preserved by the growth helper.

- **Hands-On Lab Experiment:** Trace local occurrence zero with a nonzero first index. Compare its storage address with its local primitive number. Inspect the browser scene; the application cache still uses standalone uploads here.

<details>
<summary>Implementation 96zzl-read-shared-ranges-through-object-records: Read shared ranges through object records</summary>

Read an arena range without changing original owners or local triangle identity.

Data flow: object range start → shared index → shared packed vertex → original draw and triangle owner.

### 1. `src/object_record.rs`

Append an aligned four-word range block after the existing object fields.

<details>
<summary>Locate the existing block</summary>

```rust
pub const BYTES: usize = 64 + 16 + 64 + 16;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub const RANGE: usize = 64 + 16 + 64 + 16;
pub const BYTES: usize = RANGE + 16;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzl-read-shared-ranges-through-object-records-1.rs"
```

</details>

### 2. `src/gpu_mesh.rs`

Pack the range after the existing model, flags, normal matrix and override. Keep the range address independent from the cap token and selected primitive base. Write the first shared index into its own integer field. Pack the geometry range when creating an object row. Preserve its range while updating placement and display settings. Compare and upload the color and range independently.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    pub fn settings_bytes(&self) -> u64 { self.model.size() }

    pub fn update(&mut self, queue: &wgpu::Queue, object: &Object, style:Style) -> [usize; 2] {
        let next = settings(object, style,self.cap_flags|if self.xray&&self.cap_flags&8192!=0{32768}else{0},self.cap_token,self.primitive_base);
        let mut writes = [0, 0];
        for (start, end) in [(0, 64), (64, 80), (80, 144), (144,BYTES)] {
            if next[start..end] != self.previous[start..end] {
                queue.write_buffer(&self.model, start as u64, &next[start..end]);
                writes[0] += 1;
```

</details>

Replace it with:

```rust
    pub fn settings_bytes(&self) -> u64 { self.model.size() }

    pub fn update(&mut self, queue: &wgpu::Queue, object: &Object, style:Style) -> [usize; 2] {
        let next = settings(object, style,self.cap_flags|if self.xray&&self.cap_flags&8192!=0{32768}else{0},self.cap_token,self.primitive_base,self.geometry.index_start);
        let mut writes = [0, 0];
        for (start, end) in [(0, 64), (64, 80), (80, 144), (144,RANGE),(RANGE,BYTES)] {
            if next[start..end] != self.previous[start..end] {
                queue.write_buffer(&self.model, start as u64, &next[start..end]);
                writes[0] += 1;
```

<details>
<summary>Locate this block</summary>

```rust
    pub fn with_geometry(device: &wgpu::Device, layout: &wgpu::BindGroupLayout,
        object: &Object, style:Style, geometry: Rc<GpuGeometry>) -> Self
    {
        let bytes = settings(object, style,0,0,0);
        let model = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("object placement"),
            contents: &bytes,
```

</details>

Replace it with:

```rust
    pub fn with_geometry(device: &wgpu::Device, layout: &wgpu::BindGroupLayout,
        object: &Object, style:Style, geometry: Rc<GpuGeometry>) -> Self
    {
        let bytes = settings(object, style,0,0,0,geometry.index_start);
        let model = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("object placement"),
            contents: &bytes,
```

<details>
<summary>Locate this block</summary>

```rust
        bytes[80 + index * 4..84 + index * 4].copy_from_slice(&value.to_ne_bytes());
    }
    if let Some(colour)=style.colour {for (index,value) in colour[..3].iter().copied().chain([1.]).enumerate() {bytes[144+index*4..148+index*4].copy_from_slice(&value.to_ne_bytes());}}
    bytes
}
```

</details>

Replace it with:

```rust
        bytes[80 + index * 4..84 + index * 4].copy_from_slice(&value.to_ne_bytes());
    }
    if let Some(colour)=style.colour {for (index,value) in colour[..3].iter().copied().chain([1.]).enumerate() {bytes[144+index*4..148+index*4].copy_from_slice(&value.to_ne_bytes());}}
    bytes[RANGE..RANGE+4].copy_from_slice(&index_start.to_ne_bytes());
    bytes
}
```

<details>
<summary>Locate this block</summary>

```rust
#[derive(Clone,Copy)]
pub struct Style {pub selected:bool,pub pickable:bool,pub colour:Option<[f32;4]>}

fn settings(object: &Object, style:Style,cap_flags:u32,cap_token:u32,primitive_base:u32) -> Record {
    let mut bytes = [0; BYTES];
    for (index, value) in object.model.m.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
```

</details>

Replace it with:

```rust
#[derive(Clone,Copy)]
pub struct Style {pub selected:bool,pub pickable:bool,pub colour:Option<[f32;4]>}

fn settings(object: &Object, style:Style,cap_flags:u32,cap_token:u32,primitive_base:u32,index_start:u32) -> Record {
    let mut bytes = [0; BYTES];
    for (index, value) in object.model.m.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
```

<details>
<summary>Locate this block</summary>

```rust
use crate::{gpu_geometry::GpuGeometry, scene::Object};
use std::rc::Rc;
use crate::object_record::{BYTES, Record};
use wgpu::util::DeviceExt;

pub struct GpuMesh {
```

</details>

Replace it with:

```rust
use crate::{gpu_geometry::GpuGeometry, scene::Object};
use std::rc::Rc;
use crate::object_record::{BYTES, Record, RANGE};
use wgpu::util::DeviceExt;

pub struct GpuMesh {
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{gpu_geometry::GpuGeometry, scene::Object};
use std::rc::Rc;
use crate::object_record::{BYTES, Record, RANGE};
use wgpu::util::DeviceExt;

pub struct GpuMesh {
    pub geometry: Rc<GpuGeometry>,
    model_group: wgpu::BindGroup,
    pub id: crate::scene::ObjectId,
    model: wgpu::Buffer,
    previous: Record,primitive_base:u32,xray:bool,cap_flags:u32,cap_token:u32,
}

#[derive(Clone,Copy)]
pub struct Style {pub selected:bool,pub pickable:bool,pub colour:Option<[f32;4]>}

fn settings(object: &Object, style:Style,cap_flags:u32,cap_token:u32,primitive_base:u32,index_start:u32) -> Record {
    let mut bytes = [0; BYTES];
    for (index, value) in object.model.m.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
    bytes[64..68].copy_from_slice(&((primitive_base<<1)|u32::from(style.selected)).to_ne_bytes());
    let id=if style.pickable {object.id.value()} else {0};
    bytes[68..72].copy_from_slice(&id.to_ne_bytes());
    let flags=if matches!(&object.original,Some(session_rust::Geometry::Plane(p)) if crate::clipping_geometry::is_clipping(p)){4096u32}else{0};bytes[72..76].copy_from_slice(&(flags|cap_flags).to_ne_bytes());bytes[76..80].copy_from_slice(&cap_token.to_ne_bytes());
    let normals = crate::normals::matrix(&object.model).expect("Scene placements are checked before insertion or editing");
    for (index, value) in normals.iter().enumerate() {
        bytes[80 + index * 4..84 + index * 4].copy_from_slice(&value.to_ne_bytes());
    }
    if let Some(colour)=style.colour {for (index,value) in colour[..3].iter().copied().chain([1.]).enumerate() {bytes[144+index*4..148+index*4].copy_from_slice(&value.to_ne_bytes());}}
    bytes[RANGE..RANGE+4].copy_from_slice(&index_start.to_ne_bytes());
    bytes
}

impl GpuMesh {
    pub fn upload(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, object: &Object, selected: bool) -> Self {
        let geometry = Rc::new(GpuGeometry::upload(device, Rc::clone(&object.mesh)));
        Self::with_geometry(device, layout, object, Style {selected,pickable:!object.metadata.is_locked,colour:None}, geometry)
    }

    pub fn with_geometry(device: &wgpu::Device, layout: &wgpu::BindGroupLayout,
        object: &Object, style:Style, geometry: Rc<GpuGeometry>) -> Self
    {
        let bytes = settings(object, style,0,0,0,geometry.index_start);
        let model = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("object placement"),
            contents: &bytes,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let model_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("object placement"),
            layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: model.as_entire_binding() }],
        });
        Self { geometry, model_group, id: object.id, model, previous: bytes,primitive_base:0,xray:false,cap_flags:0,cap_token:0 }
    }

    pub(crate) fn model_buffer(&self)->&wgpu::Buffer{&self.model}
    pub(crate) fn packed(&self)->Record {self.previous}
    pub fn selected(&self)->bool {u32::from_ne_bytes(self.previous[64..68].try_into().unwrap())&1!=0}

    pub fn set_primitive_base(&mut self,queue:&wgpu::Queue,base:u32)->Result<(),&'static str>{
        if base>0x7fff_ffff{return Err("Primitive base exceeds its packed range");}
        if self.primitive_base!=base{
            let word=(base<<1)|u32::from(self.selected());
            self.previous[64..68].copy_from_slice(&word.to_ne_bytes());
            queue.write_buffer(&self.model,64,&self.previous[64..68]);self.primitive_base=base;
        }
        Ok(())
    }

    pub fn set_xray(&mut self,queue:&wgpu::Queue,on:bool){if self.xray==on{return;}self.xray=on;let flags=u32::from_ne_bytes(self.previous[72..76].try_into().unwrap());let flags=(flags&!32768)|if on&&flags&8192!=0{32768}else{0};self.previous[72..76].copy_from_slice(&flags.to_ne_bytes());queue.write_buffer(&self.model,72,&self.previous[72..76]);}
    pub fn closed(&self)->bool{!self.xray&&self.cap_flags&crate::solid_orientation::CLOSED!=0}
    pub fn pickable(&self)->bool{u32::from_ne_bytes(self.previous[68..72].try_into().unwrap())!=0}
    pub fn set_caps(&mut self,queue:&wgpu::Queue,flags:u32,token:u32){
        if self.cap_flags==flags&&self.cap_token==token{return;}let plane=u32::from_ne_bytes(self.previous[72..76].try_into().unwrap())&4096;self.previous[72..76].copy_from_slice(&(plane|flags|if self.xray&&flags&8192!=0{32768}else{0}).to_ne_bytes());self.previous[76..80].copy_from_slice(&token.to_ne_bytes());queue.write_buffer(&self.model,72,&self.previous[72..80]);self.cap_flags=flags;self.cap_token=token;
    }
    pub fn settings_bytes(&self) -> u64 { self.model.size() }

    pub fn update(&mut self, queue: &wgpu::Queue, object: &Object, style:Style) -> [usize; 2] {
        let next = settings(object, style,self.cap_flags|if self.xray&&self.cap_flags&8192!=0{32768}else{0},self.cap_token,self.primitive_base,self.geometry.index_start);
        let mut writes = [0, 0];
        for (start, end) in [(0, 64), (64, 80), (80, 144), (144,RANGE),(RANGE,BYTES)] {
            if next[start..end] != self.previous[start..end] {
                queue.write_buffer(&self.model, start as u64, &next[start..end]);
                writes[0] += 1;
                writes[1] += end - start;
            }
        }
        self.previous = next;
        writes
    }

    pub fn bind_model(&self, pass: &mut wgpu::RenderPass<'_>) { pass.set_bind_group(1, &self.model_group, &[]); }

    pub fn draw_pulled(&self, pass: &mut wgpu::RenderPass<'_>) {
        self.bind_model(pass);
        self.geometry.draw_pulled(pass, 1);
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_bind_group(1, &self.model_group, &[]);
        self.geometry.draw(pass);
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzl-read-shared-ranges-through-object-records-2.rs"
```

```text
--8<-- "journey/code/96zzl-read-shared-ranges-through-object-records-3.rs"
```

```text
--8<-- "journey/code/96zzl-read-shared-ranges-through-object-records-4.rs"
```

```text
--8<-- "journey/code/96zzl-read-shared-ranges-through-object-records-5.rs"
```

```text
--8<-- "journey/code/96zzl-read-shared-ranges-through-object-records-6.rs"
```

```text
--8<-- "journey/code/96zzl-read-shared-ranges-through-object-records-7.rs"
```

</details>

### 3. `src/triangle.wgsl`

Match the appended aligned host range block.

<details>
<summary>Locate the existing block</summary>

```wgsl
    face_colour:vec4<f32>,
```

</details>

After that block, type:

```wgsl
    range:vec4<u32>,
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzl-read-shared-ranges-through-object-records-8.wgsl"
```

</details>

### 4. `src/cap_count.wgsl`

Match the appended aligned host range block.

<details>
<summary>Locate the existing block</summary>

```wgsl
    face_colour:vec4<f32>,
```

</details>

After that block, type:

```wgsl
    range:vec4<u32>,
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzl-read-shared-ranges-through-object-records-9.wgsl"
```

</details>

### 5. `src/outline_mask.wgsl`

Keep the object layout consistent in indexed surface consumers.

<details>
<summary>Locate the existing block</summary>

```wgsl
struct Object {model:mat4x4<f32>,selection:vec4<u32>,normal_matrix:mat4x4<f32>,face_colour:vec4<f32>}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct Object {model:mat4x4<f32>,selection:vec4<u32>,normal_matrix:mat4x4<f32>,face_colour:vec4<f32>,range:vec4<u32>}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzl-read-shared-ranges-through-object-records-10.wgsl"
```

</details>

### 6. `src/ambient_sources.wgsl`

Keep the object layout consistent in indexed surface consumers.

<details>
<summary>Locate the existing block</summary>

```wgsl
struct Object{model:mat4x4<f32>,selection:vec4<u32>,normal_matrix:mat4x4<f32>,face_colour:vec4<f32>}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct Object{model:mat4x4<f32>,selection:vec4<u32>,normal_matrix:mat4x4<f32>,face_colour:vec4<f32>,range:vec4<u32>}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzl-read-shared-ranges-through-object-records-11.wgsl"
```

</details>

### 7. `src/finite_project.wgsl`

Read the same object range in physical triangle projection.

<details>
<summary>Locate the existing block</summary>

```wgsl
struct ObjectSettings {model:mat4x4<f32>,selected:u32,id:u32,pad:vec2<u32>,normals:mat4x4<f32>,colour:vec4<f32>}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct ObjectSettings {model:mat4x4<f32>,selected:u32,id:u32,pad:vec2<u32>,normals:mat4x4<f32>,colour:vec4<f32>,range:vec4<u32>}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzl-read-shared-ranges-through-object-records-12.wgsl"
```

</details>

### 8. `src/triangle.wgsl`

Keep a local occurrence separate from the shared storage offset. Offset only the storage read; local triangle numbering stays unchanged. Use each instance’s own object row when pulling shared vertices.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```wgsl
@group(3) @binding(0) var<storage, read> mesh_values: array<PackedVertex>;
@group(3) @binding(2) var<storage, read> mesh_indices: array<u32>;
struct MeshInput { position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>, }
fn mesh_vertex(occurrence:u32)->MeshInput{
 let vertex=mesh_values[mesh_indices[occurrence]];
 return MeshInput(packed_position(vertex),unpack4x8unorm(vertex.color).rgb,oct32_decode(vertex.normal));
}

@vertex
fn vertex(@builtin(vertex_index) occurrence: u32, @builtin(instance_index) instance: u32) -> VertexOutput {
    let input = mesh_vertex(occurrence);
    let object=object_at(instance);
    var output=geometry(input.position,input.colour,input.normal,object);
    output.primitive=(object.selection.x>>1u)+occurrence/3u+1u;
    return output;
```

</details>

Replace it with:

```wgsl
@group(3) @binding(0) var<storage, read> mesh_values: array<PackedVertex>;
@group(3) @binding(2) var<storage, read> mesh_indices: array<u32>;
struct MeshInput { position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>, }
fn mesh_vertex(occurrence:u32,first:u32)->MeshInput{
 let vertex=mesh_values[mesh_indices[occurrence+first]];
 return MeshInput(packed_position(vertex),unpack4x8unorm(vertex.color).rgb,oct32_decode(vertex.normal));
}

@vertex
fn vertex(@builtin(vertex_index) occurrence: u32, @builtin(instance_index) instance: u32) -> VertexOutput {
    let object=object_at(instance);
    let input=mesh_vertex(occurrence,object.range.x);
    var output=geometry(input.position,input.colour,input.normal,object);
    output.primitive=(object.selection.x>>1u)+occurrence/3u+1u;
    return output;
```

<details>
<summary>Complete updated block reference</summary>

```wgsl
struct ViewSettings { transform: mat4x4<f32>, display: vec4<f32>, }
@group(0) @binding(0) var<uniform> view: ViewSettings;
struct ObjectSettings {
    model: mat4x4<f32>,
    selection: vec4<u32>,
    normal_matrix: mat4x4<f32>,
    face_colour:vec4<f32>,
    range:vec4<u32>,
}
@group(1) @binding(0) var<uniform> object_settings: ObjectSettings;
fn object_at(index: u32) -> ObjectSettings { return object_settings; }

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec3<f32>,
    @location(1) world: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) @interpolate(flat) clip_flags:u32,
    @location(4) @interpolate(flat) mirrored:u32,
    @location(5) @interpolate(flat) primitive:u32,
}

fn geometry(position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>, object: ObjectSettings) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = view.transform * world;
    let displayed=select(colour,object.face_colour.xyz,object.face_colour.w>0.5);
    output.colour = select(displayed, vec3<f32>(1.0, 0.75, 0.05), (object.selection.x & 1u) != 0u);
    output.mirrored=select(0u,1u,determinant(mat3x3<f32>(object.model[0].xyz,object.model[1].xyz,object.model[2].xyz))<0.);
    output.world = world.xyz;output.clip_flags=object.selection.z;
    output.normal = (object.normal_matrix * vec4<f32>(normal, 0.0)).xyz;
    return output;
}

fn shade(input: VertexOutput, front: bool) -> vec4<f32> {
    // Derivatives require neighbouring fragments; calculate the sharp face direction before choosing a potentially smooth interpolated source direction.
    let derivative = cross(dpdx(input.world), dpdy(input.world));
    // Screen derivatives ignore vertex order; front-facing restores the original face orientation for reversed winding and world-normal inspection.
    let face = select(derivative, -derivative, front);
    if(clip_active()&&clip_cut(input.clip_flags,input.world)){discard;}
    let closed=(input.clip_flags&8192u)!=0u;
    let source_front=(front!=(input.mirrored!=0u))!=((input.clip_flags&16384u)!=0u);
    if((input.clip_flags&32768u)!=0u || (closed&&!source_front&&view.display.y>0.&&view.display.y<1.)){discard;}
    let direction = select(face, input.normal, dot(input.normal, input.normal) > 0.0);
    let scale = max(max(abs(direction.x), abs(direction.y)), abs(direction.z));
    let bounded = direction / max(scale, 1e-30);
    let normal = bounded / max(length(bounded), 1e-30);
    if view.display.x > 0.5 { return vec4<f32>(normal * 0.5 + vec3<f32>(0.5), select(1.,view.display.y,closed)); }
    let light = normalize(vec3<f32>(0.4, -0.6, 1.0));
    // abs lights both sides of an open face; 0.3 keeps unlit faces readable.
    let brightness = 0.3 + 0.7 * abs(dot(normal, light));
    return vec4<f32>(input.colour * brightness, select(1.,view.display.y,closed));
}

struct FragmentOutput{
    @location(0) color:vec4<f32>,
    @location(1) primitive:vec2<u32>,
}
@fragment
fn fragment(input:VertexOutput,@builtin(front_facing) front:bool)->FragmentOutput{
    return FragmentOutput(shade(input,front),physical_primitive(input.primitive));
}

@group(3) @binding(0) var<storage, read> mesh_values: array<PackedVertex>;
@group(3) @binding(2) var<storage, read> mesh_indices: array<u32>;
struct MeshInput { position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>, }
fn mesh_vertex(occurrence:u32,first:u32)->MeshInput{
 let vertex=mesh_values[mesh_indices[occurrence+first]];
 return MeshInput(packed_position(vertex),unpack4x8unorm(vertex.color).rgb,oct32_decode(vertex.normal));
}

@vertex
fn vertex(@builtin(vertex_index) occurrence: u32, @builtin(instance_index) instance: u32) -> VertexOutput {
    let object=object_at(instance);
    let input=mesh_vertex(occurrence,object.range.x);
    var output=geometry(input.position,input.colour,input.normal,object);
    output.primitive=(object.selection.x>>1u)+occurrence/3u+1u;
    return output;
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzl-read-shared-ranges-through-object-records-13.wgsl"
```

```text
--8<-- "journey/code/96zzl-read-shared-ranges-through-object-records-14.wgsl"
```

```text
--8<-- "journey/code/96zzl-read-shared-ranges-through-object-records-15.wgsl"
```

</details>

### 9. `src/triangle_pick.wgsl`

Read the same shared range for first surface picks.

<details>
<summary>Locate the existing block</summary>

```wgsl
    let input=mesh_vertex(occurrence);let placed=geometry(input.position,input.colour,input.normal,object_settings);
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    let input=mesh_vertex(occurrence,object_settings.range.x);let placed=geometry(input.position,input.colour,input.normal,object_settings);
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzl-read-shared-ranges-through-object-records-16.wgsl"
```

</details>

### 10. `src/face_pick.wgsl`

Keep original face metadata local while pulling shared positions.

<details>
<summary>Locate the existing block</summary>

```wgsl
    let vertex = positions[indices[index]];
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    let vertex = positions[indices[index+object_settings.range.x]];
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzl-read-shared-ranges-through-object-records-17.wgsl"
```

</details>

### 11. `src/finite_project.wgsl`

Project the same shared source indices into the local physical interval.

<details>
<summary>Locate the existing block</summary>

```wgsl
 let point=packed_position(physical_vertices[physical_indices[index]]);
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
 let point=packed_position(physical_vertices[physical_indices[index+object.range.x]]);
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzl-read-shared-ranges-through-object-records-18.wgsl"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 12. `src/object_record_tests.rs`

Match the new range field in both supplied surface shader layout assertions.

<details>
<summary>Locate the existing block</summary>

```text
(uniform, vec![0, 64, 80, 144]), (instances, vec![0, 64, 80, 144])
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzl-read-shared-ranges-through-object-records-19.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 13. `src/object_record_tests.rs`

Match the same range field in the supplied compute shader layout assertion.

<details>
<summary>Locate the existing block</summary>

```text
vec![0, 64, 68, 72, 80, 144]
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzl-read-shared-ranges-through-object-records-20.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 14. `src/global_outline_tests.rs`

Supply an actual nonzero-range renderer comparison for surface color, original face picks, projection, outlines and clipped caps.

<details>
<summary>Locate the existing block</summary>

```text
#[test]
fn actual_compact_surface_upload_uses_twenty_bytes_per_vertex_and_releases(){
 let scene=Scene::demo();let mut gpu=renderer(&scene);let source=scene.objects()[0].mesh.clone();let original=source.vertices().to_vec();
 assert_eq!(gpu.meshes[0].geometry.buffers().0.size(),source.vertices().len()as u64*20);
 assert_eq!(gpu.meshes[0].geometry.allocated_bytes(),source.vertices().len()as u64*20+source.indices().len()as u64*4);
 let mut cache=crate::geometry_cache::GeometryCache::default();let uploaded=cache.get(&gpu.device,&source);
 assert_eq!(cache.uploaded_bytes,uploaded.allocated_bytes(),"Nonempty compact upload statistics match actual GPU buffers");
 let written=cache.uploaded_bytes;cache.get(&gpu.device,&source);assert_eq!(cache.uploads,1);assert_eq!(cache.uploaded_bytes,written);
 let empty=crate::gpu_geometry::GpuGeometry::upload(&gpu.device,std::rc::Rc::new(crate::mesh::Mesh::new(vec![],vec![]).unwrap()));assert_eq!(empty.allocated_bytes(),24);
 pixels(&gpu);assert_eq!(source.vertices(),original);let uploads=gpu.stats();pixels(&gpu);assert_eq!(gpu.stats(),uploads);
 gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzl-read-shared-ranges-through-object-records-21.rs" skip=1162
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 15. `src/pick_gpu_tests.rs`

Keep the exact supplied GPU allocation check: two 176-byte object records occupy 352 bytes.

<details>
<summary>Locate the existing block</summary>

```text
renderer.instance_usage()[..3],&[1,2,320]
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzl-read-shared-ranges-through-object-records-instance-bytes-renderer.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 16. `src/pick_gpu_tests.rs`

Keep the exact supplied GPU allocation check: two 176-byte object records occupy 352 bytes.

<details>
<summary>Locate the existing block</summary>

```text
gpu.instance_usage()[..3],&[1,2,320]
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzl-read-shared-ranges-through-object-records-instance-bytes-gpu.rs"
```

</details>

</details>

<a id="checkpoint-96zzm-connect-the-renderer-to-one-mesh-arena"></a>

### Step 7 of 14 · Connect the renderer to one mesh arena (`96zzm-connect-the-renderer-to-one-mesh-arena`)

- **Goal:** Render prepared source meshes through one shared arena and release its owner when unused.
- **Data Flow Trace:** `prepared mesh → cached geometry lease → shared arena → object range → existing renderer passes`
- **Files You Touch:** `src/geometry_cache.rs` (*Keep one shared arena alongside the existing weak geometry cache.*) · `src/renderer.rs` (*Connect visible source objects to the shared arena cache.*) · `src/browser.rs` (*Include retained packed upload rows in the browser’s CPU payload diagnostic.*) · `src/global_outline_tests.rs` (*Supply exact shared-owner accounting, cached-source reuse, preserved standalone placeholder and document-release checks.*)

Connect the existing renderer cache to the shared arena. Keep weak geometry leases for definition reuse, while the arena owns one vertex buffer and one index buffer.

Upload a source range once. Each object record carries its start; earlier geometry reads the current handles after growth. Count arena capacity once, include retained CPU upload rows, and drop the owner when the last geometry lease disappears.

> **Self-Check Question:** *Why are geometry leases weak while the arena itself has one strong owner?*  
> **Answer:** Object rows keep live geometry leases. The cache must not keep removed geometry alive; it owns shared storage only while a live lease remains.

- **Hands-On Lab Experiment:** Compare geometry uploads before and after moving and selecting an existing placed copy. Close the document and inspect the GPU geometry byte count: the shared owner must be released.

<details>
<summary>Implementation 96zzm-connect-the-renderer-to-one-mesh-arena: Connect the renderer to one mesh arena</summary>

Render prepared source meshes through one shared arena and release its owner when unused.

Data flow: prepared mesh → cached geometry lease → shared arena → object range → existing renderer passes.

### 1. `src/geometry_cache.rs`

Keep one shared arena alongside the existing weak geometry cache. Allocate shared storage lazily when a prepared source is first requested. Use the existing GPU context to upload through the arena. Reuse the shared source-range encoder and count only newly prepared ranges and uploaded bytes. Remove the per-geometry upload accounting now that the arena records actual appended bytes. Release the owner after the last active geometry lease is removed. Report shared GPU capacity and retained CPU upload-row capacity once.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{gpu_geometry::GpuGeometry, mesh::Mesh};
use std::{collections::HashMap, rc::{Rc, Weak}};

#[derive(Default)]
pub struct GeometryCache {
    entries: HashMap<*const Mesh, Weak<GpuGeometry>>,
    pub uploads: usize,
    pub uploaded_bytes: u64,
}

impl GeometryCache {
    pub fn get(&mut self, device: &wgpu::Device, source: &Rc<Mesh>) -> Rc<GpuGeometry> {
        let key = Rc::as_ptr(source);
        if let Some(geometry) = self.entries.get(&key).and_then(Weak::upgrade) {
            return geometry;
        }
        let geometry = Rc::new(GpuGeometry::upload(device, Rc::clone(source)));
        self.entries.insert(key, Rc::downgrade(&geometry));
        self.uploads += 1;
        self.uploaded_bytes += (source.vertices().len() * 20 + source.indices().len() * 4) as u64;
        geometry
    }

    pub fn prune(&mut self) {
        self.entries.retain(|_, geometry| geometry.strong_count() > 0);
    }

    pub fn entries(&self) -> usize { self.entries.len() }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{gpu_geometry::GpuGeometry, mesh::Mesh};
use std::{cell::RefCell,collections::HashMap, rc::{Rc, Weak}};

#[derive(Default)]
pub struct GeometryCache {
    entries: HashMap<*const Mesh, Weak<GpuGeometry>>,
    arena:Option<Rc<RefCell<crate::mesh_arena::Arena>>>,
    pub uploads: usize,
    pub uploaded_bytes: u64,
}

impl GeometryCache {
    pub fn get(&mut self, ctx:&crate::text_gpu_support::GpuCtx, source: &Rc<Mesh>) -> Rc<GpuGeometry> {
        let key = Rc::as_ptr(source);
        if let Some(geometry) = self.entries.get(&key).and_then(Weak::upgrade) {
            return geometry;
        }
        let arena=self.arena.get_or_insert_with(||Rc::new(RefCell::new(crate::mesh_arena::Arena::new(ctx)))).clone();
        let (sources,bytes)={let arena=arena.borrow();(arena.rows.source_count(),arena.rows.vertices.len()+arena.rows.indices.len()*4)};
        let geometry=Rc::new(GpuGeometry::shared(ctx,Rc::clone(source),arena.clone()).expect("Prepared shared mesh ranges"));
        let arena=arena.borrow();
        self.uploads+=usize::from(arena.rows.source_count()!=sources);
        self.uploaded_bytes+=(arena.rows.vertices.len()+arena.rows.indices.len()*4-bytes)as u64;
        self.entries.insert(key, Rc::downgrade(&geometry));

        geometry
    }

    pub fn prune(&mut self) {
        self.entries.retain(|_, geometry| geometry.strong_count() > 0);
        if self.entries.is_empty(){self.arena=None;}
    }

    pub fn entries(&self) -> usize { self.entries.len() }
    pub fn allocated_bytes(&self)->u64{self.arena.as_ref().map_or(0,|arena|arena.borrow().allocated_bytes())}
    pub fn cpu_bytes(&self)->usize{self.arena.as_ref().map_or(0,|arena|{let arena=arena.borrow();arena.rows.vertices.capacity()+arena.rows.indices.capacity()*4})}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzm-connect-the-renderer-to-one-mesh-arena-1.rs"
```

```text
--8<-- "journey/code/96zzm-connect-the-renderer-to-one-mesh-arena-2.rs"
```

```text
--8<-- "journey/code/96zzm-connect-the-renderer-to-one-mesh-arena-3.rs"
```

```text
--8<-- "journey/code/96zzm-connect-the-renderer-to-one-mesh-arena-4.rs"
```

```text
--8<-- "journey/code/96zzm-connect-the-renderer-to-one-mesh-arena-6.rs"
```

```text
--8<-- "journey/code/96zzm-connect-the-renderer-to-one-mesh-arena-7.rs"
```

</details>

### 2. `src/renderer.rs`

Connect visible source objects to the shared arena cache. Add the arena owner once instead of charging its buffers to every source geometry.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
        self.meshes.iter().map(|row| &row.geometry)
    }

    pub fn usage(&self) -> [u64; 4] { crate::memory::gpu(&self.meshes) }

    pub fn uploaded_bytes(&self) -> u64 {
        self.instances.written_bytes + self.geometry.uploaded_bytes + self.settings_allocations as u64 * crate::object_record::BYTES as u64 + self.settings_bytes as u64
```

</details>

Replace it with:

```rust
        self.meshes.iter().map(|row| &row.geometry)
    }

    pub fn usage(&self)->[u64;4]{let mut usage=crate::memory::gpu(&self.meshes);usage[1]+=self.geometry.allocated_bytes();usage}
    pub fn arena_cpu_bytes(&self)->usize{self.geometry.cpu_bytes()}

    pub fn uploaded_bytes(&self) -> u64 {
        self.instances.written_bytes + self.geometry.uploaded_bytes + self.settings_allocations as u64 * crate::object_record::BYTES as u64 + self.settings_bytes as u64
```

<details>
<summary>Locate this block</summary>

```rust
                self.settings_bytes += bytes;
                row
            } else {
                let geometry = self.geometry.get(&self.device, &object.mesh);
                self.settings_allocations += 1;
                GpuMesh::with_geometry(&self.device, &layout, object,
                    style(scene,object.id,selected==Some(object.id)), geometry)
```

</details>

Replace it with:

```rust
                self.settings_bytes += bytes;
                row
            } else {
                let geometry = self.geometry.get(&crate::text_gpu_support::GpuCtx::new(&self.device,&self.queue), &object.mesh);
                self.settings_allocations += 1;
                GpuMesh::with_geometry(&self.device, &layout, object,
                    style(scene,object.id,selected==Some(object.id)), geometry)
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{gpu_mesh::GpuMesh, scene::{ObjectId, Scene}};

pub struct Renderer {
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
    grid_plane:crate::cplane::CPlane,
    size: [u32; 2],
    density: f32,
    picker: crate::pick_gpu::Picker,
    sources: crate::pick_sources::Sources,
    faces: crate::face_pick::Faces,
    mesh_ink: crate::stroke_gpu::Lane,
    pick_mode: crate::pick_sources::Mode,
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
        let view_layout = crate::gpu_layout::uniform(&device, 80);
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
                buffers: &[],
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
        let instances=crate::instance_gpu::Lane::new(&device,format,&view_layout);
        let highlight_pipeline = mesh_pipeline(crate::depth::ink(),false);
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
        let widget = crate::widget::Widget::new(&device, format);
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform and normal diagnostic"),
            size: 80,
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
        let mut renderer = Self {opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, primitive, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy, size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode: Default::default() };
        for lane in [&mut renderer.mesh_ink,&mut renderer.strokes,&mut renderer.paths,&mut renderer.markers,&mut renderer.grid]{lane.enable_finite(&renderer.device,format);}
        renderer.highlight.enable_finite(&renderer.device,format);
        renderer.set_scene(scene, None);
        renderer
    }

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
        if self.size != [size.width, size.height] { self.picker.cancel(); }
        self.size = [size.width, size.height];
        self.depth = Self::depth(&self.device, size.width, size.height);
        self.primitive=crate::primitive_image::Image::new(&self.device,self.size);
    }

    pub fn set_density(&mut self, density: f32) {
        if density.is_finite() && density > 0.0 && density != self.density { self.picker.cancel(); self.density = density; }
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

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.ambient_rows.set(scene);if let Some(ambient)=self.ambient.borrow_mut().as_mut(){if self.ambient_enabled.get(){ambient.sources.set(&self.device,&self.queue,&self.ambient_rows.specs);}}
        self.pyramid_revision=self.pyramid_revision.wrapping_add(1);
        self.clipping.borrow_mut().set(scene);
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        self.picker.cancel(); self.faces.clear();
        self.preview.set(&self.device, None).expect("Empty tool preview");
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, None).expect("Empty selection");
        self.sources = crate::pick_sources::Sources::new(scene).expect("Prepared source topology is bounded");
        self.mesh_ink.set(&self.device, self.sources.mesh_ink.iter().copied()).expect("Prepared source edges");
        self.mesh_ink.set_ids(&self.device, self.sources.mesh_ids.iter().copied()).expect("Source edge IDs match");
        let planes=crate::clipping_rows::owners(scene);let flags=self.sources.mesh_ids.iter().map(|id|crate::sub_id::decode(id[1]).and_then(|(_,address)|self.sources.edges.get(address)).map_or(0,|target|if planes.contains(&target.owner){4096}else{0})).collect();self.mesh_ink.set_flags(&self.device,flags).expect("Original plane ink flags match source addresses");
        self.markers.set_points(&self.device, scene.points().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut marker=row.marker();if let Some(colour)=scene.face_rgba(row.id) {marker.colour=colour;}marker}).chain(scene.control_markers().into_iter().map(|c| c.display)))
            .expect("Prepared point placement fits the display range");
        self.paths.set_chains(&self.device, scene.paths().iter().filter(|row| scene.visible(row.id)).flat_map(|row| {let mut segments=row.segments();if let Some(colour)=scene.face_rgba(row.id) {for segment in &mut segments {segment.stroke.colour=colour;}}segments}))
            .expect("Prepared path placement fits the display range");
        self.strokes.set(&self.device, scene.lines().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut stroke=row.stroke();if let Some(colour)=scene.face_rgba(row.id) {stroke.colour=colour;}stroke}))
            .expect("Prepared line placement fits the display range");
        self.markers.set_ids(&self.device, self.sources.marker_ids.iter().copied()).expect("Marker owners match");
        self.paths.set_ids(&self.device, self.sources.path_ids.iter().copied()).expect("Path owners match");
        self.strokes.set_ids(&self.device, self.sources.line_ids.iter().copied()).expect("Line owners match");
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
                GpuMesh::with_geometry(&self.device, &layout, object,
                    style(scene,object.id,selected==Some(object.id)), geometry)
            };
            self.meshes.push(row);
        }
        drop(old);
        self.caps.borrow_mut().set_scene(&self.queue,scene,&mut self.meshes,&self.clipping.borrow().rows);
        for mesh in &mut self.meshes{mesh.set_xray(&self.queue,self.opacity.get()==0.);}
        let mut base=0u32;
        for mesh in &mut self.meshes{
            mesh.set_primitive_base(&self.queue,base).expect("Bounded projected triangle base");
            base=base.checked_add(mesh.geometry.index_count/3).filter(|n|*n<=0x7fff_ffff).expect("Bounded projected triangle range");
        }
        self.tiles.borrow_mut().set(&self.device,&self.meshes);
        self.instance_rows=scene.instances.keys().copied().collect();
        self.instances.set(&self.device,&self.queue,self.meshes.iter().filter(|row|self.instance_rows.contains(&row.id))).expect("Validated instance records fit device limits");
        self.sheets.set(&self.device,&self.queue,scene,selected.as_slice(),&self.sources.sheet_bases).expect("Prepared original sheet batches fit device limits");
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        if let Err(error)=self.set_clouds(scene,selected.as_slice()) {*self.cloud_error.borrow_mut()=Some(error.to_string());}
        if let Err(error)=self.set_scene_text(scene,selected.as_slice()) {*self.text_error.borrow_mut()=Some(error.to_string());}
    }

    pub fn request_pick(&mut self, screen: [f32; 2], transform: [f32; 16]) -> Result<(), &'static str> {
        let window = crate::pick_window::Window::new(screen, self.size, self.density).ok_or("Invalid pick cursor")?;
        self.picker.queue(crate::pick_gpu::Request { window, transform, density: self.density, mode: self.pick_mode })?;
        self.pump_pick(transform); Ok(())
    }

    fn pump_pick(&mut self, transform: [f32; 16]) {
        let Ok(view) = crate::pick_view::View::new(transform, self.size, self.density) else { self.picker.cancel(); return; };
        if self.pick_mode == crate::pick_sources::Mode::Face && !self.faces.ready(self.meshes.len()) {
            self.faces.set(&self.device, &self.meshes, &self.sources.mesh_bases,&self.clipping.borrow().buffer);
        }
        if let Some(lane)=self.text.borrow_mut().as_mut() {if let Err(error)=lane.prepare(&self.text_context(),&self.text_frame(transform)) {*self.text_error.borrow_mut()=Some(error.to_string());return;}}
        self.clipping.borrow_mut().frame(&self.queue,transform,self.size[1],self.density);
        self.prepare_clouds(transform);
        if let Some(request) = self.picker.next(view) {
            let mut clipping=self.clipping.borrow_mut();clipping.frame(&self.queue,transform,self.size[1],self.density);
            let mut bytes=transform.iter().flat_map(|v|v.to_ne_bytes()).collect::<Vec<_>>();bytes.resize(80,0);self.queue.write_buffer(&self.uniform,0,&bytes);
            let tables={let mut tiles=self.tiles.borrow_mut();tiles.prepare(&self.device,&self.queue,self.size);let mut encoder=self.device.create_command_encoder(&Default::default());tiles.encode(&self.device,&mut encoder,&self.view_group,transform);self.queue.submit([encoder.finish()]);tiles.map_report();crate::finite_ink::Tables{projected:tiles.projected.clone(),tiles:tiles.buffer.clone(),primitive:self.primitive.view.clone()}};
            self.picker.submit_finite(&self.device, &self.queue, request,
            &self.meshes, &[&self.mesh_ink, &self.strokes, &self.paths, &self.markers], &self.faces,self.text.borrow().as_ref(),self.clouds.borrow().as_ref(),Some(&self.sheets),Some(&tables),&clipping.group,&clipping.buffer,Some(&mut self.caps.borrow_mut())); }
    }

    pub fn take_pick(&mut self, transform: [f32; 16]) -> Option<Result<Option<[u32; 2]>, &'static str>> {
        let view = match crate::pick_view::View::new(transform, self.size, self.density) { Ok(view) => view, Err(error) => {
            self.picker.cancel(); return Some(Err(error));
        } };
        match self.picker.take(view) {
            Some(Ok(crate::pick_gpu::Reply::Hit(hit))) => Some(Ok(hit)),
            Some(Ok(crate::pick_gpu::Reply::Stale)) => { self.pump_pick(transform); None },
            Some(Err(error)) => Some(Err(error)), None => None,
        }
    }

    pub fn set_pick_mode(&mut self, mode: crate::pick_sources::Mode) {
        if self.pick_mode != mode { self.picker.cancel(); self.pick_mode = mode; }
    }

    pub fn resolve_pick(&self, scene: &Scene, pixel: [u32; 2]) -> Option<crate::pick_sources::Target> {
        self.sources.resolve(scene, pixel, self.pick_mode)
    }

    pub fn part_usage(&self) -> [u64; 4] { let [count, bytes] = self.mesh_ink.usage(); [count, bytes, self.faces.bytes(), self.sources.bytes() as u64] }

    pub fn cancel_pick(&mut self) { self.picker.cancel(); }

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
        if settings==self.grid_settings&&plane==self.grid_plane{return Ok(())}
        let strokes=settings.map(|s|s.strokes_on(plane)).transpose()?.unwrap_or_default();
        self.grid.set(&self.device,strokes)?;self.grid_settings=settings;self.grid_plane=plane;Ok(())
    }

    pub fn grid_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.grid.usage(); [count, bytes, self.grid.uploads, self.grid.uploaded_bytes]
    }

    pub fn set_highlight(&mut self, scene: &Scene, target: Option<crate::pick_sources::Target>) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, target)
    }

    pub fn select_objects(&mut self, scene: &Scene, ids: &[ObjectId]) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        for mesh in &mut self.meshes { if let Some(row) = scene.objects().iter().find(|r| r.id == mesh.id) {
            let [calls, bytes] = mesh.update(&self.queue, row,style(scene,row.id,ids.contains(&row.id)));
            self.settings_writes += calls; self.settings_bytes += bytes;
        } }
        self.instances.set(&self.device,&self.queue,self.meshes.iter().filter(|row|self.instance_rows.contains(&row.id)))?;
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        self.set_clouds(scene,ids)?;self.sheets.set(&self.device,&self.queue,scene,ids,&self.sources.sheet_bases)?;
        if !ids.is_empty() { self.highlight.objects(&self.device, scene, ids)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
    }

    pub fn set_guides(&mut self,overlay:&crate::guide_gpu::Overlay)->Result<(),&'static str>{if self.guides.set(&self.device,overlay)?{self.set_scene_measure_text().map_err(|_|"Cannot prepare editing guide label")?}Ok(())}
    pub fn guide_bytes(&self)->u64{self.guides.bytes()}
    pub fn set_measurement(&mut self,mark:Option<&crate::measure_mark::Mark>)->Result<(),&'static str>{if self.measurement.mark.as_ref()==mark{return Ok(())}self.measurement.set(&self.device,mark)?;self.set_scene_measure_text().map_err(|_|"Cannot prepare measurement annotation")}
    pub fn measurement_bytes(&self)->u64{self.measurement.bytes()}
    fn merge_measurement(&self,mut labels:Vec<crate::text::TextLabel>)->Vec<crate::text::TextLabel>{if let Some(mark)=&self.measurement.mark{labels.extend(mark.labels(&labels));}labels.extend(self.guides.overlay.labels(&labels));labels}
    fn set_scene_measure_text(&self)->anyhow::Result<()>{let mut labels=self.scene_text.borrow().clone();labels.extend(self.presentation_text.borrow().clone());self.adopt_text(self.merge_measurement(labels))}
    pub fn set_preview(&mut self, draft: Option<&crate::tool::Draft>) -> Result<(), &'static str> { self.preview.set(&self.device, draft) }
    pub fn preview_bytes(&self) -> u64 { self.preview.bytes() }

    pub fn set_gizmo(&mut self, camera:&crate::camera::Camera, origin:Option<[f64;3]>, active:f32) {
        let matrix=camera.view_projection();
        let layout=origin.and_then(|p|crate::widget_layout::world_per_pixel(&matrix,p,self.size[1] as f64/self.density as f64)
            .and_then(|scale|crate::widget_layout::layout(&matrix,p,scale,self.size,active)));
        self.widget.prepare(&self.device,&self.queue,layout);
    }
    pub fn gizmo_usage(&self) -> [u64;4] { self.widget.usage() }
    pub fn gizmo_extent(&self) -> Option<[u32;2]> {self.widget.extent()}

    fn text_context(&self)->crate::text_gpu_support::GpuCtx {crate::text_gpu_support::GpuCtx::new(&self.device,&self.queue)}
    fn text_frame(&self,transform:[f32;16])->crate::scene_text_gpu::TextFrame {crate::scene_text_gpu::TextFrame {mvp:transform,origin:[0.;3],framebuffer:self.size,logical:[self.size[0] as f64/self.density as f64,self.size[1] as f64/self.density as f64],ortho_half_height:0.,clip:{let clip=self.clipping.borrow();let mut planes=[[0.;4];6];if clip.enabled {for (p,row) in planes.iter_mut().zip(&clip.rows){*p=[row.plane.normal[0],row.plane.normal[1],row.plane.normal[2],row.plane.offset];}}planes}}}
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
    pub fn refresh_text_bounds(&self,scene:&mut Scene) {
        if let Some(lane)=self.text.borrow().as_ref() {for row in scene.texts_mut() {if let Some(run)=lane.document.runs.iter().find(|run|run.label.id==row.id.value()) {row.prepared.extent=run.buffer.layout_runs().fold([0f64;2],|[w,h],line|[w.max(line.line_w as f64),h.max((line.line_top+line.line_height) as f64)]);}}}
    }
    pub fn text_stats(&self)->Option<crate::scene_text_gpu::TextStats> {self.text.borrow().as_ref().map(|lane|lane.stats.clone())}
    pub fn text_scene_usage(&self)->[u64;4] {self.text.borrow().as_ref().map_or([0;4],|lane|[lane.allocated_bytes(),lane.texture_bytes(),lane.stats.world_plane_rasterizations,lane.document.runs.len() as u64])}
    pub fn replace_text_fonts(&self,fonts:Vec<Vec<u8>>)->anyhow::Result<()> {if let Some(lane)=self.text.borrow_mut().as_mut() {lane.document.replace_fonts(fonts.clone())?;} else {let mut document=crate::text::TextDocument::new();document.replace_fonts(fonts.clone())?;}*self.full_text_fonts.borrow_mut()=Some(fonts);Ok(())}
    pub fn clear_text_fonts(&self) {self.full_text_fonts.borrow_mut().take();}

    pub fn text_error(&self)->Option<String> {self.text_error.borrow().clone()}
    pub fn text_usage(&self)->[u64;6] {self.text.borrow().as_ref().map_or([0;6],|lane|{[lane.document.shape_count,lane.stats.preparations,lane.stats.skipped_preparations,lane.stats.raster_images as u64,lane.stats.raster_image_capacity_bytes as u64,lane.stats.atlas_resets]})}
    pub fn text_demo(&self)->anyhow::Result<()> {self.set_text(vec![crate::text_example::label()])}

    pub fn cloud_camera(&self,camera:&crate::camera::Camera) {self.cloud_view.set(crate::cloud_gpu::View::camera(camera));self.sheets.camera(camera);}
    pub fn cloud_usage(&self)->[u64;5] {self.clouds.borrow().as_ref().map_or([0;5],|lane|lane.usage())}
    pub fn cloud_error(&self)->Option<String> {self.cloud_error.borrow().clone()}
    fn set_clouds(&self,scene:&Scene,selected:&[ObjectId])->Result<(),&'static str> {
        if scene.clouds().is_empty()&&scene.streams().is_empty() {self.clouds.borrow_mut().take();self.cloud_error.borrow_mut().take();return Ok(());}
        let mut owner=self.clouds.borrow_mut();let lane=owner.get_or_insert_with(||crate::cloud_gpu::Lane::new(&self.text_context(),self.format,&self.clipping.borrow().group));lane.set_scene(&self.text_context(),scene,selected)
    }
    fn prepare_clouds(&self,transform:[f32;16]) {
        if let Some(lane)=self.clouds.borrow_mut().as_mut() {lane.set_clipping(self.clipping.borrow().writes);if let Err(error)=lane.prepare(&self.text_context(),transform,self.size,self.density,self.cloud_view.get()) {lane.failed();*self.cloud_error.borrow_mut()=Some(error.to_string());} else {self.cloud_error.borrow_mut().take();}}
    }
    pub fn set_opacity(&mut self,value:f32){if !value.is_finite(){return;}let value=value.clamp(0.,1.);if self.opacity.get()==value{return;}self.opacity.set(value);self.picker.cancel();self.pyramid_revision=self.pyramid_revision.wrapping_add(1);for mesh in &mut self.meshes{mesh.set_xray(&self.queue,value==0.);}for lane in [&self.mesh_ink,&self.strokes,&self.paths,&self.markers,&self.grid]{lane.set_opacity(value);}self.highlight.set_opacity(value);if let Some(o)=self.outline.borrow_mut().as_mut(){o.invalidate();}if let Some(o)=self.global_outline.borrow_mut().as_mut(){o.invalidate();}}
    pub fn opacity(&self)->f32{self.opacity.get()}
    pub fn set_outlines(&self,on:bool){self.outlines_enabled.set(on);if !on{self.global_outline.borrow_mut().take();self.outline_composite.borrow_mut().take();}}
    pub fn global_outline_usage(&self)->[u64;4]{self.global_outline.borrow().as_ref().map_or([0;4],|o|o.usage())}
    pub fn outline_usage(&self)->[u64;4] {self.outline.borrow().as_ref().map_or([0;4],|outline|outline.usage())}

    pub fn highlight_bytes(&self) -> u64 { self.highlight.bytes() }

    pub fn sheet_camera(&self,camera:&crate::camera::Camera) {self.sheets.camera(camera);}
    #[cfg(test)]
    pub(crate) fn tile_buffers(&self)->[wgpu::Buffer;2]{let tiles=self.tiles.borrow();[tiles.projected.clone(),tiles.buffer.clone()]}
    pub fn set_clipping(&self,enabled:bool,solid:bool){let mut clip=self.clipping.borrow_mut();clip.enabled=enabled;clip.solid=solid;self.caps.borrow_mut().enabled=enabled;}
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
        let (clip_group,clip_revision)={let mut clip=self.clipping.borrow_mut();clip.frame(&self.queue,*transform,self.size[1],self.density);(clip.group.clone(),clip.writes)};
        self.caps.borrow_mut().prepare(&self.device,&self.queue,self.format,self.size,{let clip=self.clipping.borrow();clip.enabled&&!clip.rows.is_empty()});
        self.prepare_clouds(*transform);
        let mut bytes: Vec<u8> = transform.iter().flat_map(|value| value.to_ne_bytes()).collect();
        let display = [if normals { 1.0f32 } else { 0.0 }, self.opacity.get(), 0.0, 0.0];
        bytes.extend(display.iter().flat_map(|value| value.to_ne_bytes()));
        self.queue.write_buffer(&self.uniform, 0, &bytes);
        self.strokes.view(&self.queue, transform, self.size, self.density);
        self.sheets.view(&self.queue,transform,self.size,self.density);
        self.paths.view(&self.queue, transform, self.size, self.density);
        self.markers.view(&self.queue, transform, self.size, self.density);
        self.grid.view(&self.queue, transform, self.size, self.density);
        self.mesh_ink.view(&self.queue, transform, self.size, self.density);
        self.highlight.view(&self.queue, transform, self.size, self.density);
        self.preview.view(&self.queue, transform, self.size, self.density);
        self.measurement.view(&self.queue,transform,self.size,self.density);self.guides.view(&self.queue,transform,self.size,self.density);
        let [r, g, b] = background.rgb();
        let mut encoder = self.device.create_command_encoder(&Default::default());
        let mut tiles=self.tiles.borrow_mut();tiles.prepare(&self.device,&self.queue,self.size);tiles.encode(&self.device,&mut encoder,&self.view_group,*transform);
        let tables=crate::finite_ink::Tables{projected:tiles.projected.clone(),tiles:tiles.buffer.clone(),primitive:self.primitive.view.clone()};for lane in [&self.mesh_ink,&self.strokes,&self.paths,&self.markers,&self.grid]{lane.finite_frame(&self.device,&tables,&self.depth);}self.highlight.finite_frame(&self.device,&tables,&self.depth);
        {
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("canvas"),
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
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
            pass.set_bind_group(2,&clip_group,&[]);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            for mesh in self.meshes.iter().filter(|row|self.opacity.get()<1.||!self.instance_rows.contains(&row.id)) {mesh.draw_pulled(&mut pass);}
            if self.opacity.get()>=1.{self.instances.draw(&mut pass);}
            pass.set_pipeline(&self.highlight_pipeline); pass.set_bind_group(0, &self.view_group, &[]);
            self.highlight.mesh(&mut pass);
        }
        self.caps.borrow_mut().draw(&mut encoder,view,&self.primitive.view,&self.depth,&self.view_group,&clip_group,&self.meshes,if self.clipping.borrow().enabled{self.clipping.borrow().rows.len()as u32}else{0});
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("cloud depth composite"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});if let Some(clouds)=self.clouds.borrow().as_ref(){clouds.draw(&mut pass);}}
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite scene ink"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:None,stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});
            pass.set_bind_group(2,&clip_group,&[]);
            self.highlight.ink(&mut pass);
            self.mesh_ink.draw(&mut pass);
            self.grid.draw(&mut pass);
            self.strokes.draw(&mut pass);self.sheets.draw(&mut pass);
            self.paths.draw(&mut pass);
            self.markers.draw(&mut pass);
            self.highlight.points(&mut pass);
            self.preview.draw(&mut pass);
        }
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("measurement overlay"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});self.measurement.draw(&mut pass);self.guides.draw(&mut pass);}
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
        if self.ambient_enabled.get(){let clip=self.clipping.borrow();if let Some(ambient)=self.ambient.borrow_mut().as_mut(){ambient.draw(&self.device,&self.queue,&mut encoder,view,&self.depth,&self.view_group,&clip_group,&clip.buffer,&self.primitive.view,&self.meshes,*transform,self.size,self.density,self.ambient_rows.ground,[self.pyramid_revision,clip_revision]);}}
        if let Some(pyramid)=self.pyramid.borrow_mut().as_mut(){pyramid.encode(&self.device,&self.queue,&mut encoder,&self.depth,*transform,self.size,self.density,[self.pyramid_revision,clip_revision]);}
        self.queue.submit([encoder.finish()]);tiles.map_report();
    }
}

fn style(scene:&Scene,id:ObjectId,selected:bool)->crate::gpu_mesh::Style {crate::gpu_mesh::Style {selected,pickable:scene.pick_id(id.value()).is_some(),colour:scene.face_rgba(id)}}

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
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzm-connect-the-renderer-to-one-mesh-arena-8.rs"
```

```text
--8<-- "journey/code/96zzm-connect-the-renderer-to-one-mesh-arena-9.rs"
```

</details>

### 3. `src/browser.rs`

Include retained packed upload rows in the browser’s CPU payload diagnostic.

<details>
<summary>Locate the existing block</summary>

```rust
    let cpu = crate::memory::cpu(editor.presentation_scenes(), renderer.geometry_owners().map(|owner| &owner.source));
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    let mut cpu = crate::memory::cpu(editor.presentation_scenes(), renderer.geometry_owners().map(|owner| &owner.source));
    cpu[4]+=renderer.arena_cpu_bytes();
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzm-connect-the-renderer-to-one-mesh-arena-10.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/global_outline_tests.rs`

Supply exact shared-owner accounting, cached-source reuse, preserved standalone placeholder and document-release checks.

<details>
<summary>Locate the existing block</summary>

```text
fn actual_compact_surface_upload_uses_twenty_bytes_per_vertex_and_releases(){
 let scene=Scene::demo();let mut gpu=renderer(&scene);let source=scene.objects()[0].mesh.clone();let original=source.vertices().to_vec();
 assert_eq!(gpu.meshes[0].geometry.buffers().0.size(),source.vertices().len()as u64*20);
 assert_eq!(gpu.meshes[0].geometry.allocated_bytes(),source.vertices().len()as u64*20+source.indices().len()as u64*4);
 let mut cache=crate::geometry_cache::GeometryCache::default();let uploaded=cache.get(&gpu.device,&source);
 assert_eq!(cache.uploaded_bytes,uploaded.allocated_bytes(),"Nonempty compact upload statistics match actual GPU buffers");
 let written=cache.uploaded_bytes;cache.get(&gpu.device,&source);assert_eq!(cache.uploads,1);assert_eq!(cache.uploaded_bytes,written);
 let empty=crate::gpu_geometry::GpuGeometry::upload(&gpu.device,std::rc::Rc::new(crate::mesh::Mesh::new(vec![],vec![]).unwrap()));assert_eq!(empty.allocated_bytes(),24);
 pixels(&gpu);assert_eq!(source.vertices(),original);let uploads=gpu.stats();pixels(&gpu);assert_eq!(gpu.stats(),uploads);
 gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzm-connect-the-renderer-to-one-mesh-arena-11.rs"
```

</details>

</details>

<a id="checkpoint-96zzn-describe-eight-byte-draw-slots"></a>

### Step 8 of 14 · Describe eight-byte draw slots (`96zzn-describe-eight-byte-draw-slots`)

- **Goal:** Store an object row and primitive ID base in one instance-step vertex record.
- **Data Flow Trace:** `object row + primitive ID base → eight-byte slot → instance-step vertex input`
- **Files You Touch:** `src/draw_slots.rs` (*Type the compact integer layout and reuse the existing growing buffer for slot ownership.*) · `src/lib.rs` (*Register the shared slot owner before connecting the instance pipeline.*) · `src/draw_slots_tests.rs` (*Supply a real GPU readback fixture for exact large integer IDs, growth, reuse, sentinel and release.*)

Separate instance selection from object properties. A draw slot contains two integers: the object row to read and the primitive ID base. Reserve slot zero for an ordinary draw that keeps its own object row. Reuse the growing buffer and preserve that sentinel through reset and release.

> **Self-Check Question:** *Why does a draw slot use an instance-step buffer?*  
> **Answer:** Every vertex of one placed instance reads the same slot; the next instance advances to another object row and ID base.

- **Hands-On Lab Experiment:** Trace slots for two placed copies. Predict the uploaded bytes and explain why slot zero remains available after clearing the instances.

<details>
<summary>Implementation 96zzn-describe-eight-byte-draw-slots: Describe eight-byte draw slots</summary>

Store an object row and primitive ID base in one instance-step vertex record.

Data flow: object row + primitive ID base → eight-byte slot → instance-step vertex input.

### 1. `src/draw_slots.rs`

Type the compact integer layout and reuse the existing growing buffer for slot ownership.

Create the file and type:

```rust
--8<-- "journey/code/96zzn-describe-eight-byte-draw-slots-1.rs"
```

### 2. `src/lib.rs`

Register the shared slot owner before connecting the instance pipeline.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn validate_source_json(text: &str) -> Result<(), JsValue> {
    source_validate::json(text).map_err(|error| JsValue::from_str(&error))
}

mod shader_check;
#[cfg(test)]
mod cloud_shader_contract;
```

</details>

After that block, type:

```rust

mod draw_slots;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzn-describe-eight-byte-draw-slots-2.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/draw_slots_tests.rs`

Supply a real GPU readback fixture for exact large integer IDs, growth, reuse, sentinel and release.

Copy this supplied file:

```text
--8<-- "journey/code/96zzn-describe-eight-byte-draw-slots-3.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/lib.rs`

Register the optional GPU fixture.

<details>
<summary>Locate the existing block</summary>

```text
mod draw_slots;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzn-describe-eight-byte-draw-slots-4.rs" skip=15
```

</details>

</details>

<a id="checkpoint-96zzo-read-instance-properties-through-draw-slots"></a>

### Step 9 of 14 · Read instance properties through draw slots (`96zzo-read-instance-properties-through-draw-slots`)

- **Goal:** Connect compact draw slots to the existing instance color pipeline.
- **Data Flow Trace:** `instance slot → property record + ID base → shared mesh range → color and physical triangle`
- **Files You Touch:** `src/draw_slots.rs` (*Keep one comparison shadow in the slot owner.*) · `src/instance_gpu.rs` (*Keep slots beside the existing batch property records.*) · `src/object_record_tests.rs` (*Validate the actual shader assembled by the instance pipeline.*) · `src/pick_gpu_tests.rs` (*Retain exact two-record allocation checks and include the three eight-byte slots.*)

Bind an eight-byte slot beside the shared source mesh. Its object row chooses a full property record, and its ID base names the placed triangle. Keep the shader’s local triangle address separate from the original owner ID. Reuse unchanged slot bytes when only object properties change.

> **Self-Check Question:** *Why must placed copies have separate primitive ID bases?*  
> **Answer:** They share source triangles but occupy distinct places. Physical visibility must distinguish those occurrences while picking retains each original object owner.

- **Hands-On Lab Experiment:** Create two placed copies, move one, toggle normal diagnostics and select it. Check the source upload count stays fixed and Undo restores the same image.

<details>
<summary>Implementation 96zzo-read-instance-properties-through-draw-slots: Read instance properties through draw slots</summary>

Connect compact draw slots to the existing instance color pipeline.

Data flow: instance slot → property record + ID base → shared mesh range → color and physical triangle.

### 1. `src/draw_slots.rs`

Keep one comparison shadow in the slot owner. Start with no uploaded slots. Skip unchanged slot uploads. Update the shadow after uploading the complete logical slot rows.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::text_gpu_support::{GpuCtx,GrowBuf};
pub const OWN_ROW:u32=u32::MAX;
pub type Slot=[u32;2];
const ATTRIBUTES:[wgpu::VertexAttribute;2]=[
 wgpu::VertexAttribute {offset:0,shader_location:0,format:wgpu::VertexFormat::Uint32},
 wgpu::VertexAttribute {offset:4,shader_location:1,format:wgpu::VertexFormat::Uint32},
];
pub fn layout()->wgpu::VertexBufferLayout<'static>{wgpu::VertexBufferLayout {array_stride:8,step_mode:wgpu::VertexStepMode::Instance,attributes:&ATTRIBUTES}}
pub struct Slots {pub(crate) buffer:GrowBuf}
impl Slots {
 pub fn new(ctx:&GpuCtx)->Self {
  let mut slots=Self {buffer:GrowBuf::new(ctx,"instance draw slots",8,wgpu::BufferUsages::VERTEX)};
  slots.set(ctx,&[]);slots
 }
 pub fn set(&mut self,ctx:&GpuCtx,rows:&[Slot]){
  let bytes:Vec<u8>=std::iter::once([OWN_ROW,0]).chain(rows.iter().copied()).flatten().flat_map(u32::to_ne_bytes).collect();
  self.buffer.reset();self.buffer.append_bytes(ctx,&bytes);
 }
 pub fn clear(&mut self,ctx:&GpuCtx,free:bool){if free {self.buffer.release(ctx);}self.set(ctx,&[]);}
 pub fn bind(&self,pass:&mut wgpu::RenderPass<'_>,slot:u32,first:u32){
  assert!(first<self.buffer.len());pass.set_vertex_buffer(slot,self.buffer.buf.slice(u64::from(first)*8..));
 }
 pub fn instances(&self)->u32{self.buffer.len()-1}
 pub fn allocated_bytes(&self)->u64{self.buffer.buf.size()}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::text_gpu_support::{GpuCtx,GrowBuf};
pub const OWN_ROW:u32=u32::MAX;
pub type Slot=[u32;2];
const ATTRIBUTES:[wgpu::VertexAttribute;2]=[
 wgpu::VertexAttribute {offset:0,shader_location:0,format:wgpu::VertexFormat::Uint32},
 wgpu::VertexAttribute {offset:4,shader_location:1,format:wgpu::VertexFormat::Uint32},
];
pub fn layout()->wgpu::VertexBufferLayout<'static>{wgpu::VertexBufferLayout {array_stride:8,step_mode:wgpu::VertexStepMode::Instance,attributes:&ATTRIBUTES}}
pub struct Slots {pub(crate) buffer:GrowBuf,previous:Vec<Slot>}
impl Slots {
 pub fn new(ctx:&GpuCtx)->Self {
  let mut slots=Self {buffer:GrowBuf::new(ctx,"instance draw slots",8,wgpu::BufferUsages::VERTEX),previous:Vec::new()};
  slots.set(ctx,&[]);slots
 }
 pub fn set(&mut self,ctx:&GpuCtx,rows:&[Slot])->bool{
  if self.buffer.len()>0&&self.previous==rows{return false;}
  let bytes:Vec<u8>=std::iter::once([OWN_ROW,0]).chain(rows.iter().copied()).flatten().flat_map(u32::to_ne_bytes).collect();
  self.buffer.reset();self.buffer.append_bytes(ctx,&bytes);
  self.previous.clear();self.previous.extend_from_slice(rows);true
 }
 pub fn clear(&mut self,ctx:&GpuCtx,free:bool){if free {self.buffer.release(ctx);}self.set(ctx,&[]);}
 pub fn bind(&self,pass:&mut wgpu::RenderPass<'_>,slot:u32,first:u32){
  assert!(first<self.buffer.len());pass.set_vertex_buffer(slot,self.buffer.buf.slice(u64::from(first)*8..));
 }
 pub fn instances(&self)->u32{self.buffer.len()-1}
 pub fn allocated_bytes(&self)->u64{self.buffer.buf.size()}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzo-read-instance-properties-through-draw-slots-1.rs"
```

```text
--8<-- "journey/code/96zzo-read-instance-properties-through-draw-slots-2.rs"
```

```text
--8<-- "journey/code/96zzo-read-instance-properties-through-draw-slots-3.rs"
```

```text
--8<-- "journey/code/96zzo-read-instance-properties-through-draw-slots-4.rs"
```

</details>

### 2. `src/instance_gpu.rs`

Keep slots beside the existing batch property records. Use one assembled instance shader for the pipeline and its validation fixture. Select a property row and placed primitive ID base from instance-step integer attributes. Avoid assembling the clipping and packed helpers twice. Declare the eight-byte instance-step slot in the actual pipeline. Keep each batch-local object row and its existing globally assigned primitive base. Upload slots only when their row or primitive address changes. Create the sentinel and instance slots when a batch is first allocated. Skip sentinel slot zero when drawing actual instances. Count both property storage and actual slot-buffer capacity.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh};
use std::{collections::{BTreeMap,HashMap},rc::Rc};
use wgpu::util::DeviceExt;
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,records:wgpu::Buffer,group:wgpu::BindGroup,bytes:Vec<u8>,count:u32}
pub struct Lane {pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<(usize,usize),Batch>,pub writes:u64,pub written_bytes:u64}
impl Lane {
    pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("shared instance records"),entries:&[wgpu::BindGroupLayoutEntry {binding:0,visibility:wgpu::ShaderStages::VERTEX,ty:wgpu::BindingType::Buffer {ty:wgpu::BufferBindingType::Storage {read_only:true},has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(BYTES as u64)},count:None}]});
        let source=include_str!("triangle.wgsl")
            .replace("var<uniform> object_settings: ObjectSettings;", "var<storage, read> objects: array<ObjectSettings>;")
            .replace("return object_settings;", "return objects[index];");
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(&source,2,0).into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(format.into()),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        Self {pipeline,layout,batches:BTreeMap::new(),writes:0,written_bytes:0}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=(u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size)/BYTES as u64) as usize;
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:HashMap<usize,(Rc<GpuGeometry>,Vec<Record>)>=HashMap::new();
        for row in rows {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push(row.packed());}
        let mut old=std::mem::take(&mut self.batches);let mut batches=BTreeMap::new();
        for (source,(geometry,records)) in groups {
            for (part,records) in records.chunks(limit).enumerate() {
                let key=(source,part);let bytes=records.iter().flatten().copied().collect::<Vec<_>>();
                let batch=if let Some(mut batch)=old.remove(&key).filter(|b|Rc::ptr_eq(&b.geometry,&geometry)&&b.records.size()>=bytes.len() as u64) {
                    if batch.bytes!=bytes {queue.write_buffer(&batch.records,0,&bytes);self.writes+=1;self.written_bytes+=bytes.len() as u64;batch.bytes=bytes;}batch.count=records.len() as u32;batch
                } else {
                    let buffer=device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:Some("independent source placements"),contents:&bytes,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST});let group=crate::gpu_layout::group(device,&self.layout,&buffer);self.writes+=1;self.written_bytes+=bytes.len() as u64;Batch {geometry:geometry.clone(),records:buffer,group,bytes,count:records.len() as u32}
                };batches.insert(key,batch);
            }
        }
        self.batches=batches;Ok(())
    }
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        for batch in self.batches.values() {pass.set_bind_group(1,&batch.group,&[]);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.batches.values().map(|b|b.records.size()).sum(),self.writes,self.written_bytes]}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh};
use std::{collections::{BTreeMap,HashMap},rc::Rc};
use wgpu::util::DeviceExt;
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,records:wgpu::Buffer,group:wgpu::BindGroup,bytes:Vec<u8>,count:u32,slots:crate::draw_slots::Slots}
pub struct Lane {pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<(usize,usize),Batch>,pub writes:u64,pub written_bytes:u64}
pub(crate) fn source()->String {
 let source=include_str!("triangle.wgsl")
  .replace("var<uniform> object_settings: ObjectSettings;","var<storage, read> objects: array<ObjectSettings>;")
  .replace("return object_settings;","return objects[index];")
  .replace("@builtin(instance_index) instance: u32","@location(0) row:u32,@location(1) primitive_base:u32")
  .replace("let object=object_at(instance);","let object=object_at(row);")
  .replace("(object.selection.x>>1u)+occurrence/3u+1u","primitive_base+occurrence/3u+1u");
 crate::clipping_gpu::source(&source,2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("shared instance records"),entries:&[wgpu::BindGroupLayoutEntry {binding:0,visibility:wgpu::ShaderStages::VERTEX,ty:wgpu::BindingType::Buffer {ty:wgpu::BufferBindingType::Storage {read_only:true},has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(BYTES as u64)},count:None}]});
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(format.into()),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        Self {pipeline,layout,batches:BTreeMap::new(),writes:0,written_bytes:0}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=(u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size)/BYTES as u64) as usize;
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:HashMap<usize,(Rc<GpuGeometry>,Vec<Record>)>=HashMap::new();
        for row in rows {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push(row.packed());}
        let mut old=std::mem::take(&mut self.batches);let mut batches=BTreeMap::new();
        for (source,(geometry,records)) in groups {
            for (part,records) in records.chunks(limit).enumerate() {
                let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
                let slots:Vec<crate::draw_slots::Slot>=records.iter().enumerate().map(|(row,record)|[row as u32,u32::from_ne_bytes(record[64..68].try_into().unwrap())>>1]).collect();
                let key=(source,part);let bytes=records.iter().flatten().copied().collect::<Vec<_>>();
                let batch=if let Some(mut batch)=old.remove(&key).filter(|b|Rc::ptr_eq(&b.geometry,&geometry)&&b.records.size()>=bytes.len() as u64) {
                    if batch.slots.set(&ctx,&slots){self.writes+=1;self.written_bytes+=(slots.len()as u64+1)*8;}
                    if batch.bytes!=bytes {queue.write_buffer(&batch.records,0,&bytes);self.writes+=1;self.written_bytes+=bytes.len() as u64;batch.bytes=bytes;}batch.count=records.len() as u32;batch
                } else {
                    let buffer=device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:Some("independent source placements"),contents:&bytes,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST});let group=crate::gpu_layout::group(device,&self.layout,&buffer);self.writes+=1;self.written_bytes+=bytes.len() as u64;{let mut slot_buffer=crate::draw_slots::Slots::new(&ctx);slot_buffer.set(&ctx,&slots);self.writes+=2;self.written_bytes+=8+(slots.len()as u64+1)*8;Batch {geometry:geometry.clone(),records:buffer,group,bytes,count:records.len() as u32,slots:slot_buffer}}
                };batches.insert(key,batch);
            }
        }
        self.batches=batches;Ok(())
    }
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        for batch in self.batches.values() {pass.set_bind_group(1,&batch.group,&[]);batch.slots.bind(pass,0,1);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.batches.values().map(|b|b.records.size()+b.slots.allocated_bytes()).sum(),self.writes,self.written_bytes]}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzo-read-instance-properties-through-draw-slots-5.rs"
```

```text
--8<-- "journey/code/96zzo-read-instance-properties-through-draw-slots-6.rs"
```

```text
--8<-- "journey/code/96zzo-read-instance-properties-through-draw-slots-7.rs"
```

```text
--8<-- "journey/code/96zzo-read-instance-properties-through-draw-slots-8.rs"
```

```text
--8<-- "journey/code/96zzo-read-instance-properties-through-draw-slots-9.rs"
```

```text
--8<-- "journey/code/96zzo-read-instance-properties-through-draw-slots-10.rs"
```

```text
--8<-- "journey/code/96zzo-read-instance-properties-through-draw-slots-11.rs"
```

```text
--8<-- "journey/code/96zzo-read-instance-properties-through-draw-slots-12.rs"
```

```text
--8<-- "journey/code/96zzo-read-instance-properties-through-draw-slots-13.rs"
```

```text
--8<-- "journey/code/96zzo-read-instance-properties-through-draw-slots-14.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/object_record_tests.rs`

Validate the actual shader assembled by the instance pipeline.

<details>
<summary>Locate the existing block</summary>

```text
    let instances = uniform.replace("var<uniform> object_settings: ObjectSettings;", "var<storage, read> objects: array<ObjectSettings>;")
        .replace("return object_settings;", "return objects[index];");
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzo-read-instance-properties-through-draw-slots-15.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/pick_gpu_tests.rs`

Retain exact two-record allocation checks and include the three eight-byte slots.

<details>
<summary>Locate the existing block</summary>

```text
renderer.instance_usage()[..3],&[1,2,352]
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzo-read-instance-properties-through-draw-slots-16.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/pick_gpu_tests.rs`

Retain exact two-record allocation checks and include the three eight-byte slots.

<details>
<summary>Locate the existing block</summary>

```text
gpu.instance_usage()[..3],&[1,2,352]
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzo-read-instance-properties-through-draw-slots-17.rs"
```

</details>

</details>

<a id="checkpoint-96zzp-classify-contact-fill-shading"></a>

### Step 10 of 14 · Classify contact fill shading (`96zzp-classify-contact-fill-shading`)

- **Goal:** Read the contact marker from packed RGBA and keep that display fill opaque and unlit.
- **Data Flow Trace:** `derived RGBA → packed vertex → contact classification → opaque unlit display`
- **Files You Touch:** `src/triangle.wgsl` (*Keep packed alpha available when placing a surface vertex.*) · `src/face_pick.wgsl` (*Reuse the same RGBA placement input in source-face picking.*) · `src/global_outline_tests.rs` (*Supply an actual rendered closed-source fixture for contact color, opacity/x-ray exemptions, selection, original picks, source preservation and release.*)

The derived display already retains the kernel contact marker: alpha is one byte out of 255. Carry that byte through the shared vertex decoder. Classify it once when placing the vertex, preserve its own color before selection, and exempt the fill from solid opacity and x-ray. Keep the original object owner and primitive address.

> **Self-Check Question:** *Why is this alpha a marker instead of transparency?*  
> **Answer:** The derived contact display uses this reserved value to identify a fill. Its presentation is opaque; applying the marker as opacity would nearly erase it.

- **Hands-On Lab Experiment:** Trace the contact marker through RGBA packing and compare its fragment with an ordinary face. Predict the result at opacity zero and when its original owner is selected.

<details>
<summary>Implementation 96zzp-classify-contact-fill-shading: Classify contact fill shading</summary>

Read the contact marker from packed RGBA and keep that display fill opaque and unlit.

Data flow: derived RGBA → packed vertex → contact classification → opaque unlit display.

### 1. `src/triangle.wgsl`

Keep packed alpha available when placing a surface vertex. Recognize the reserved byte and retain contact color before the existing selection override. Carry classification in the existing flat flags while exempting contact fills from closed-solid and x-ray behavior. Read the placed classification in the fragment. Paint the contact fill opaque and unlit after clipping; ordinary faces retain the existing normal and lighting path. Pass all RGBA components through the one surface decoder. Decode the already packed marker without a second upload or vertex format.

<details>
<summary>Locate the existing block</summary>

```wgsl
struct ViewSettings { transform: mat4x4<f32>, display: vec4<f32>, }
@group(0) @binding(0) var<uniform> view: ViewSettings;
struct ObjectSettings {
    model: mat4x4<f32>,
    selection: vec4<u32>,
    normal_matrix: mat4x4<f32>,
    face_colour:vec4<f32>,
    range:vec4<u32>,
}
@group(1) @binding(0) var<uniform> object_settings: ObjectSettings;
fn object_at(index: u32) -> ObjectSettings { return object_settings; }

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec3<f32>,
    @location(1) world: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) @interpolate(flat) clip_flags:u32,
    @location(4) @interpolate(flat) mirrored:u32,
    @location(5) @interpolate(flat) primitive:u32,
}

fn geometry(position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>, object: ObjectSettings) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = view.transform * world;
    let displayed=select(colour,object.face_colour.xyz,object.face_colour.w>0.5);
    output.colour = select(displayed, vec3<f32>(1.0, 0.75, 0.05), (object.selection.x & 1u) != 0u);
    output.mirrored=select(0u,1u,determinant(mat3x3<f32>(object.model[0].xyz,object.model[1].xyz,object.model[2].xyz))<0.);
    output.world = world.xyz;output.clip_flags=object.selection.z;
    output.normal = (object.normal_matrix * vec4<f32>(normal, 0.0)).xyz;
    return output;
}

fn shade(input: VertexOutput, front: bool) -> vec4<f32> {
    // Derivatives require neighbouring fragments; calculate the sharp face direction before choosing a potentially smooth interpolated source direction.
    let derivative = cross(dpdx(input.world), dpdy(input.world));
    // Screen derivatives ignore vertex order; front-facing restores the original face orientation for reversed winding and world-normal inspection.
    let face = select(derivative, -derivative, front);
    if(clip_active()&&clip_cut(input.clip_flags,input.world)){discard;}
    let closed=(input.clip_flags&8192u)!=0u;
    let source_front=(front!=(input.mirrored!=0u))!=((input.clip_flags&16384u)!=0u);
    if((input.clip_flags&32768u)!=0u || (closed&&!source_front&&view.display.y>0.&&view.display.y<1.)){discard;}
    let direction = select(face, input.normal, dot(input.normal, input.normal) > 0.0);
    let scale = max(max(abs(direction.x), abs(direction.y)), abs(direction.z));
    let bounded = direction / max(scale, 1e-30);
    let normal = bounded / max(length(bounded), 1e-30);
    if view.display.x > 0.5 { return vec4<f32>(normal * 0.5 + vec3<f32>(0.5), select(1.,view.display.y,closed)); }
    let light = normalize(vec3<f32>(0.4, -0.6, 1.0));
    // abs lights both sides of an open face; 0.3 keeps unlit faces readable.
    let brightness = 0.3 + 0.7 * abs(dot(normal, light));
    return vec4<f32>(input.colour * brightness, select(1.,view.display.y,closed));
}

struct FragmentOutput{
    @location(0) color:vec4<f32>,
    @location(1) primitive:vec2<u32>,
}
@fragment
fn fragment(input:VertexOutput,@builtin(front_facing) front:bool)->FragmentOutput{
    return FragmentOutput(shade(input,front),physical_primitive(input.primitive));
}

@group(3) @binding(0) var<storage, read> mesh_values: array<PackedVertex>;
@group(3) @binding(2) var<storage, read> mesh_indices: array<u32>;
struct MeshInput { position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>, }
fn mesh_vertex(occurrence:u32,first:u32)->MeshInput{
 let vertex=mesh_values[mesh_indices[occurrence+first]];
 return MeshInput(packed_position(vertex),unpack4x8unorm(vertex.color).rgb,oct32_decode(vertex.normal));
}

@vertex
fn vertex(@builtin(vertex_index) occurrence: u32, @builtin(instance_index) instance: u32) -> VertexOutput {
    let object=object_at(instance);
    let input=mesh_vertex(occurrence,object.range.x);
    var output=geometry(input.position,input.colour,input.normal,object);
    output.primitive=(object.selection.x>>1u)+occurrence/3u+1u;
    return output;
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct ViewSettings { transform: mat4x4<f32>, display: vec4<f32>, }
@group(0) @binding(0) var<uniform> view: ViewSettings;
struct ObjectSettings {
    model: mat4x4<f32>,
    selection: vec4<u32>,
    normal_matrix: mat4x4<f32>,
    face_colour:vec4<f32>,
    range:vec4<u32>,
}
@group(1) @binding(0) var<uniform> object_settings: ObjectSettings;
fn object_at(index: u32) -> ObjectSettings { return object_settings; }

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec3<f32>,
    @location(1) world: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) @interpolate(flat) clip_flags:u32,
    @location(4) @interpolate(flat) mirrored:u32,
    @location(5) @interpolate(flat) primitive:u32,
}

const CONTACT_FLAG:u32=65536u;
fn geometry(position: vec3<f32>, colour: vec4<f32>, normal: vec3<f32>, object: ObjectSettings) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = view.transform * world;
    let contact=abs(colour.a-1./255.)<0.5/255.;
    let displayed=select(colour.rgb,object.face_colour.xyz,object.face_colour.w>0.5&&!contact);
    output.colour = select(displayed, vec3<f32>(1.0, 0.75, 0.05), (object.selection.x & 1u) != 0u);
    output.mirrored=select(0u,1u,determinant(mat3x3<f32>(object.model[0].xyz,object.model[1].xyz,object.model[2].xyz))<0.);
    output.world = world.xyz;
    output.clip_flags=select(object.selection.z,(object.selection.z&~(8192u|32768u))|CONTACT_FLAG,contact);
    output.normal = (object.normal_matrix * vec4<f32>(normal, 0.0)).xyz;
    return output;
}

fn shade(input: VertexOutput, front: bool) -> vec4<f32> {
    // Derivatives require neighbouring fragments; calculate the sharp face direction before choosing a potentially smooth interpolated source direction.
    let derivative = cross(dpdx(input.world), dpdy(input.world));
    // Screen derivatives ignore vertex order; front-facing restores the original face orientation for reversed winding and world-normal inspection.
    let face = select(derivative, -derivative, front);
    if(clip_active()&&clip_cut(input.clip_flags,input.world)){discard;}
    let contact=(input.clip_flags&CONTACT_FLAG)!=0u;
    let closed=(input.clip_flags&8192u)!=0u;
    let source_front=(front!=(input.mirrored!=0u))!=((input.clip_flags&16384u)!=0u);
    if((input.clip_flags&32768u)!=0u || (closed&&!source_front&&view.display.y>0.&&view.display.y<1.)){discard;}
    if(contact){return vec4<f32>(input.colour,1.);}
    let direction = select(face, input.normal, dot(input.normal, input.normal) > 0.0);
    let scale = max(max(abs(direction.x), abs(direction.y)), abs(direction.z));
    let bounded = direction / max(scale, 1e-30);
    let normal = bounded / max(length(bounded), 1e-30);
    if view.display.x > 0.5 { return vec4<f32>(normal * 0.5 + vec3<f32>(0.5), select(1.,view.display.y,closed)); }
    let light = normalize(vec3<f32>(0.4, -0.6, 1.0));
    // abs lights both sides of an open face; 0.3 keeps unlit faces readable.
    let brightness = 0.3 + 0.7 * abs(dot(normal, light));
    return vec4<f32>(input.colour * brightness, select(1.,view.display.y,closed));
}

struct FragmentOutput{
    @location(0) color:vec4<f32>,
    @location(1) primitive:vec2<u32>,
}
@fragment
fn fragment(input:VertexOutput,@builtin(front_facing) front:bool)->FragmentOutput{
    return FragmentOutput(shade(input,front),physical_primitive(input.primitive));
}

@group(3) @binding(0) var<storage, read> mesh_values: array<PackedVertex>;
@group(3) @binding(2) var<storage, read> mesh_indices: array<u32>;
struct MeshInput { position: vec3<f32>, colour: vec4<f32>, normal: vec3<f32>, }
fn mesh_vertex(occurrence:u32,first:u32)->MeshInput{
 let vertex=mesh_values[mesh_indices[occurrence+first]];
 return MeshInput(packed_position(vertex),unpack4x8unorm(vertex.color),oct32_decode(vertex.normal));
}

@vertex
fn vertex(@builtin(vertex_index) occurrence: u32, @builtin(instance_index) instance: u32) -> VertexOutput {
    let object=object_at(instance);
    let input=mesh_vertex(occurrence,object.range.x);
    var output=geometry(input.position,input.colour,input.normal,object);
    output.primitive=(object.selection.x>>1u)+occurrence/3u+1u;
    return output;
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzp-classify-contact-fill-shading-1.rs"
```

```text
--8<-- "journey/code/96zzp-classify-contact-fill-shading-2.rs"
```

```text
--8<-- "journey/code/96zzp-classify-contact-fill-shading-3.rs"
```

```text
--8<-- "journey/code/96zzp-classify-contact-fill-shading-4.rs"
```

```text
--8<-- "journey/code/96zzp-classify-contact-fill-shading-5.rs"
```

```text
--8<-- "journey/code/96zzp-classify-contact-fill-shading-6.rs"
```

```text
--8<-- "journey/code/96zzp-classify-contact-fill-shading-7.rs"
```

</details>

### 2. `src/face_pick.wgsl`

Reuse the same RGBA placement input in source-face picking.

<details>
<summary>Locate the existing block</summary>

```wgsl
    let placed = geometry(packed_position(vertex), unpack4x8unorm(vertex.color).rgb, vec3<f32>(0.0),object_settings);
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    let placed = geometry(packed_position(vertex), unpack4x8unorm(vertex.color), vec3<f32>(0.0),object_settings);
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzp-classify-contact-fill-shading-8.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/global_outline_tests.rs`

Supply an actual rendered closed-source fixture for contact color, opacity/x-ray exemptions, selection, original picks, source preservation and release.

<details>
<summary>Locate the existing block</summary>

```text
pected,"Shared source addressing preserves original picks");
        }
        assert!(r.tiles.borrow().usage()[3]>0);
        if clipped {assert_eq!(r.cap_usage()[1],1);}
    }
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzp-classify-contact-fill-shading-9.rs" skip=180
```

</details>

</details>

<a id="checkpoint-96zzq-open-files-through-one-measured-session-reader"></a>

### Step 11 of 14 · Open files through one measured Session reader (`96zzq-open-files-through-one-measured-session-reader`)

- **Goal:** Prepare browser file imports once through the measured full original-document reader.
- **Data Flow Trace:** `file bytes → decode → full validation → kernel → display walk → atomic history adoption`
- **Files You Touch:** `src/load_measure.rs` (*Let the existing timer observe String errors without duplicating its clock and completion algorithm.*) · `src/document_source.rs` (*Expose observations on the existing full reader and share its browser callback.*) · `src/source_phase_tests.rs` (*Supply fixtures against actual original Element/interaction bytes, exact phase order and failure boundaries, and retained source locations.*) · `src/lib.rs` (*Register the optional full-source observation fixtures.*) · `src/scene_document.rs` (*Reuse original-document import while allowing the browser file origin to retain its reload location.*) · `src/publication.rs` (*Remove the older mesh reader and unmeasured fallback; preserve the existing archive path.*)

Use one full Session reader for browser files. Reuse the phase timer for decode, validation, kernel reconstruction and the actual display walk. Retain the source reload location, and adopt the candidate only after preparation succeeds. Keep archive opening on its existing path.

> **Self-Check Question:** *Why must timing and file adoption use the same full reader?*  
> **Answer:** Measurements must describe the work that prepares the accepted document. A decode, validation or display error must leave the visible document and its undo/redo history intact.

- **Hands-On Lab Experiment:** Open an Element Session with interactions, download Report, then try malformed bytes. Inspect the four successful phases, the failed decode, and unchanged accepted drawing. Undo and Redo must preserve original owners.

<details>
<summary>Implementation 96zzq-open-files-through-one-measured-session-reader: Open files through one measured Session reader</summary>

Prepare browser file imports once through the measured full original-document reader.

Data flow: file bytes → decode → full validation → kernel → display walk → atomic history adoption.

### 1. `src/load_measure.rs`

Let the existing timer observe String errors without duplicating its clock and completion algorithm.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn run<T>(name: &str, bytes: u64, clock: &mut impl FnMut() -> f64,
    observe: &mut impl FnMut(Completed<'_>), operation: impl FnOnce() -> Result<T, &'static str>,
) -> Result<T, &'static str> {
```

</details>

After that block, type:

```rust
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-1.rs" skip=200
```

### 2. `src/document_source.rs`

Expose observations on the existing full reader and share its browser callback. Record the actual decode and a stable malformed-payload error. Start validation around the existing complete schema, identities, tree, placements and prepared-sheet checks. Finish validation and retain the original reload location in the one source origin. Measure the actual kernel reconstruction once.

<details>
<summary>Locate the existing block</summary>

```rust
use session_rust::{proto,Session,Xform};
use prost::Message;
use std::{collections::{HashMap,HashSet},rc::Rc};

pub(crate) struct Source {pub document:Rc<Session>,pub origin:Rc<crate::origin::Origin>,pub tree:Option<proto::TreeNode>,pub world:HashMap<String,Xform>,pub sheets:Vec<Rc<crate::sheet_source::Prepared>>}
pub(crate) fn prepare(bytes:&[u8],place:&Xform,point_px:f32)->Result<Source,String> {
    if bytes.len()>crate::archive::LIMIT {return Err("Session exceeds the 512 MiB file limit".into());}
    let message=proto::Session::decode(bytes).map_err(|e|e.to_string())?;
    crate::source_validate::session(&message)?;
    supported(&message)?;
    if !crate::placement::valid(&place.m)||!point_px.is_finite() {return Err("Invalid document placement or point size".into());}
    let tree=message.tree.as_ref().and_then(|tree|tree.root.clone());
    let mut guids=HashSet::new();let objects=message.objects.as_ref().ok_or("Session has no objects")?;
    macro_rules! identity {($field:ident)=>{for object in &objects.$field {if object.guid.is_empty()||!guids.insert(object.guid.as_str()) {return Err("Source GUIDs must be distinct within a document".into());}}};}
    identity!(components);identity!(elements);identity!(planes);identity!(bboxes);identity!(meshes);identity!(points);identity!(lines);identity!(polylines);identity!(nurbscurves);identity!(nurbssurfaces);identity!(breps);identity!(pointclouds);identity!(instances);identity!(sheets);
    if let Some(root)=tree.as_ref() {crate::hierarchy::validate(root,&guids).map_err(str::to_owned)?;}
    let mut placed=HashSet::new();for entry in &message.xforms {if !placed.insert(entry.guid.as_str())||!crate::placement::valid(&entry.xform.as_ref().ok_or("Missing placement matrix")?.matrix) {return Err("Invalid source placement".into());}}
    let sheets=objects.sheets.iter().cloned().map(|sheet|crate::sheet_source::Prepared::new(Rc::new(sheet)).map(Rc::new).map_err(str::to_owned)).collect::<Result<Vec<_>,_>>()?;
    let origin=Rc::new(crate::origin::Origin::new(&message,bytes));
    let document=Rc::new(Session::from_proto(message).map_err(|e|e.to_string())?);
    let world=crate::nested_placement::prepare(&document,place).map_err(str::to_owned)?;
    Ok(Source {document,origin,tree,world,sheets})
}

fn supported(source:&proto::Session)->Result<(),String> {
    crate::instance_plan::validate(source)?;
    Ok(())
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use session_rust::{proto,Session,Xform};
use prost::Message;
use std::{collections::{HashMap,HashSet},rc::Rc};

pub(crate) struct Source {pub document:Rc<Session>,pub origin:Rc<crate::origin::Origin>,pub tree:Option<proto::TreeNode>,pub world:HashMap<String,Xform>,pub sheets:Vec<Rc<crate::sheet_source::Prepared>>}
pub(crate) fn observe(phase:crate::load_measure::Completed<'_>){
 #[cfg(target_arch="wasm32")] crate::browser_phase::completed(phase,"document");
 #[cfg(not(target_arch="wasm32"))] let _=phase;
}
pub(crate) fn prepare(bytes:&[u8],place:&Xform,point_px:f32)->Result<Source,String>{prepare_at(bytes,place,point_px,None)}
pub(crate) fn prepare_at(bytes:&[u8],place:&Xform,point_px:f32,location:Option<Rc<crate::reload_url::ReloadUrl>>)->Result<Source,String>{
 prepare_observed(bytes,place,point_px,location,&mut crate::load_measure::now,&mut observe)
}
pub(crate) fn prepare_observed(bytes:&[u8],place:&Xform,point_px:f32,location:Option<Rc<crate::reload_url::ReloadUrl>>,
 clock:&mut impl FnMut()->f64,observe:&mut impl FnMut(crate::load_measure::Completed<'_>))->Result<Source,String>{
 use crate::load_measure::run_result;
 let size=bytes.len()as u64;
    if bytes.len()>crate::archive::LIMIT {return Err("Session exceeds the 512 MiB file limit".into());}
    let message=run_result("decode",size,clock,observe,||proto::Session::decode(bytes).map_err(|_|"Invalid session protobuf".to_owned()))?;
    let (tree,sheets)=run_result("validation",size,clock,observe,||->Result<_,String>{
    crate::source_validate::session(&message)?;
    supported(&message)?;
    if !crate::placement::valid(&place.m)||!point_px.is_finite() {return Err("Invalid document placement or point size".into());}
    let tree=message.tree.as_ref().and_then(|tree|tree.root.clone());
    let mut guids=HashSet::new();let objects=message.objects.as_ref().ok_or("Session has no objects")?;
    macro_rules! identity {($field:ident)=>{for object in &objects.$field {if object.guid.is_empty()||!guids.insert(object.guid.as_str()) {return Err("Source GUIDs must be distinct within a document".into());}}};}
    identity!(components);identity!(elements);identity!(planes);identity!(bboxes);identity!(meshes);identity!(points);identity!(lines);identity!(polylines);identity!(nurbscurves);identity!(nurbssurfaces);identity!(breps);identity!(pointclouds);identity!(instances);identity!(sheets);
    if let Some(root)=tree.as_ref() {crate::hierarchy::validate(root,&guids).map_err(str::to_owned)?;}
    let mut placed=HashSet::new();for entry in &message.xforms {if !placed.insert(entry.guid.as_str())||!crate::placement::valid(&entry.xform.as_ref().ok_or("Missing placement matrix")?.matrix) {return Err("Invalid source placement".into());}}
    let sheets=objects.sheets.iter().cloned().map(|sheet|crate::sheet_source::Prepared::new(Rc::new(sheet)).map(Rc::new).map_err(str::to_owned)).collect::<Result<Vec<_>,_>>()?;
    Ok((tree,sheets))})?;
    let mut origin=crate::origin::Origin::new(&message,bytes);origin.location=location;let origin=Rc::new(origin);
    let document=run_result("kernel",size,clock,observe,||Session::from_proto(message).map(Rc::new).map_err(|e|e.to_string()))?;
    let world=crate::nested_placement::prepare(&document,place).map_err(str::to_owned)?;
    Ok(Source {document,origin,tree,world,sheets})
}

fn supported(source:&proto::Session)->Result<(),String> {
    crate::instance_plan::validate(source)?;
    Ok(())
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-2.rs"
```

```text
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-3.rs"
```

```text
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-4.rs"
```

```text
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-5.rs"
```

```text
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-6.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/source_phase_tests.rs`

Supply fixtures against actual original Element/interaction bytes, exact phase order and failure boundaries, and retained source locations.

Copy this supplied file:

```text
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-7.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/lib.rs`

Register the optional full-source observation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
ce_validate::json(text).map_err(|error| JsValue::from_str(&error))
}

mod shader_check;
#[cfg(test)]
mod cloud_shader_contract;

mod draw_slots;
#[cfg(test)] mod draw_slots_tests;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-8.rs" skip=180
```

</details>

### 5. `src/scene_document.rs`

Reuse original-document import while allowing the browser file origin to retain its reload location. Use the measured full reader once. Time the existing original geometry, hierarchy and feature preparation as the actual display walk. Adopt the complete candidate only after a successful measured walk.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
        }
    }
    let colours=candidate.forest.colours(key);Rc::make_mut(&mut candidate.view).faces.extend(colours);
    scene.restore(candidate);Ok(key)
}
```

</details>

Replace it with:

```rust
        }
    }
    let colours=candidate.forest.colours(key);Rc::make_mut(&mut candidate.view).faces.extend(colours);
    Ok(key)})?;
    scene.restore(candidate);Ok(key)
}
```

<details>
<summary>Locate this block</summary>

```rust
use {session_rust::proto,prost::Message,std::collections::HashSet};

// Prepare every original source in a temporary scene; one successful adoption prevents partially imported files from replacing the document.
pub fn import(scene:&mut Scene,bytes:&[u8],name:&str,place:&Xform,point_px:f32)->Result<u32,String> {
    let source=crate::document_source::prepare(bytes,place,point_px)?;
    let mut candidate=scene.clone();let forest=candidate.forest.clone();let created=candidate.created;let current=candidate.view.current.clone();
    candidate.forest=Default::default();candidate.created=None;Rc::make_mut(&mut candidate.view).current=None;
    let key=candidate.reserve_document_key().map_err(str::to_owned)?;let owners=crate::document_geometry::append(&mut candidate,&source,place,point_px)?;
    candidate.forest=forest;candidate.created=created;Rc::make_mut(&mut candidate.view).current=current;
```

</details>

Replace it with:

```rust
use {session_rust::proto,prost::Message,std::collections::HashSet};

// Prepare every original source in a temporary scene; one successful adoption prevents partially imported files from replacing the document.
pub fn import(scene:&mut Scene,bytes:&[u8],name:&str,place:&Xform,point_px:f32)->Result<u32,String>{import_at(scene,bytes,name,place,point_px,None)}
pub fn import_at(scene:&mut Scene,bytes:&[u8],name:&str,place:&Xform,point_px:f32,location:Option<Rc<crate::reload_url::ReloadUrl>>)->Result<u32,String>{
    let source=crate::document_source::prepare_at(bytes,place,point_px,location)?;
    let mut candidate=scene.clone();
    let key=crate::load_measure::run_result("display walk",bytes.len()as u64,&mut crate::load_measure::now,&mut crate::document_source::observe,||->Result<u32,String>{
    let forest=candidate.forest.clone();let created=candidate.created;let current=candidate.view.current.clone();
    candidate.forest=Default::default();candidate.created=None;Rc::make_mut(&mut candidate.view).current=None;
    let key=candidate.reserve_document_key().map_err(str::to_owned)?;let owners=crate::document_geometry::append(&mut candidate,&source,place,point_px)?;
    candidate.forest=forest;candidate.created=created;Rc::make_mut(&mut candidate.view).current=current;
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::scene::Scene;
use session_rust::Xform;
use std::rc::Rc;
#[cfg(test)]
use {session_rust::proto,prost::Message,std::collections::HashSet};

// Prepare every original source in a temporary scene; one successful adoption prevents partially imported files from replacing the document.
pub fn import(scene:&mut Scene,bytes:&[u8],name:&str,place:&Xform,point_px:f32)->Result<u32,String>{import_at(scene,bytes,name,place,point_px,None)}
pub fn import_at(scene:&mut Scene,bytes:&[u8],name:&str,place:&Xform,point_px:f32,location:Option<Rc<crate::reload_url::ReloadUrl>>)->Result<u32,String>{
    let source=crate::document_source::prepare_at(bytes,place,point_px,location)?;
    let mut candidate=scene.clone();
    let key=crate::load_measure::run_result("display walk",bytes.len()as u64,&mut crate::load_measure::now,&mut crate::document_source::observe,||->Result<u32,String>{
    let forest=candidate.forest.clone();let created=candidate.created;let current=candidate.view.current.clone();
    candidate.forest=Default::default();candidate.created=None;Rc::make_mut(&mut candidate.view).current=None;
    let key=candidate.reserve_document_key().map_err(str::to_owned)?;let owners=crate::document_geometry::append(&mut candidate,&source,place,point_px)?;
    candidate.forest=forest;candidate.created=created;Rc::make_mut(&mut candidate.view).current=current;
    candidate.forest.add(key,if name.is_empty() {source.document.name.clone()} else {name.into()},source.tree,&owners);
    candidate.forest.original_placements(key,&source.document,place,&source.world);
    Rc::make_mut(candidate.forest.documents.last_mut().unwrap()).header=Some(source.origin);
    let elements = owners.iter().filter_map(|(guid, id, _)|
        (matches!(source.document.lookup.get(guid), Some(session_rust::Geometry::Element(_)))
            || candidate.instances.contains_key(id)).then_some(*id))
        .collect::<std::collections::HashSet<_>>();
    candidate.forest.classify_elements(key, &elements);
    for (guid, id, _) in &owners {
        if let Some(session_rust::Geometry::Element(element)) = source.document.lookup.get(guid) {
            crate::feature_display::append(&mut candidate, *id, &source.document, element)
                .map_err(str::to_owned)?;
        }
    }
    for (_, id, _) in &owners {
        if let Some(owner) = candidate.instances.get(id).cloned() {
            crate::instance_features::append(&mut candidate, *id, &owner).map_err(str::to_owned)?;
        }
    }
    let colours=candidate.forest.colours(key);Rc::make_mut(&mut candidate.view).faces.extend(colours);
    Ok(key)})?;
    scene.restore(candidate);Ok(key)
}

pub fn open(scene:&mut Scene,bytes:&[u8])->Result<(),String> {
    let bundle=crate::archive::Bundle::decode(bytes)?;
    let mut candidate=scene.clone();candidate.clear();let mut keys=Vec::new();
    for (source,meta) in bundle.documents.iter().zip(&bundle.metadata.documents) {keys.push(import(&mut candidate,source,&meta.name,&Xform::from_matrix(meta.place),meta.point_px)?);}
    crate::archive_restore_texts::restore(&mut candidate,&bundle.metadata)?;
    crate::archive_restore_flags::restore(&mut candidate,&bundle.metadata,&keys)?;
    scene.restore(candidate);Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mixed_save_open_restores_original_sources_placements_layers_and_presentation() {
        let mut scene=Scene::demo();scene.clear();
        let cad=scene.insert_cad(Rc::new(crate::cad_display::example())).unwrap();
        let surface=scene.insert_surface(Rc::new(crate::surface::example())).unwrap();
        let cloud=scene.insert_cloud(crate::cloud::Prepared::new(Rc::new(crate::cloud::example())).unwrap()).unwrap();
        let copy=scene.copy_row(cloud,false).unwrap();scene.place_cloud(copy,Xform::translation(1_000_000_000.125,0.,0.)).unwrap();
        scene.cloud_point_size(cloud,5.).unwrap();scene.cloud_point_size(copy,5.).unwrap();
        let point=scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(0.123456789012345,0.,0.))).unwrap()).unwrap();
        let line=scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(-1.,0.,0.,1.,0.,0.))).unwrap()).unwrap();
        let label=crate::scene_label::example(crate::text::TextPlacement::WorldBillboard {world:[0.,0.,0.],world_height:0.1});
        let text=scene.insert_text(crate::scene_label::Prepared::new(Rc::new(label)).unwrap()).unwrap();scene.place_text(text,Xform::translation(0.123456789012345,0.,0.)).unwrap();
        let root=&scene.forest.documents[0];let current=format!("{}/{}",root.key,root.nodes[0].source.guid);
        let view=Rc::make_mut(&mut scene.view);view.current=Some(current);view.hide(&[cad,text],true);view.lock(&[surface,line,text],true);view.faces.insert(cad,[2,5,7]);view.edges.insert(cad,[0;3]);
        let mut restored=Scene::demo();let old_ids=restored.objects().iter().map(|row|row.id.value()).collect::<Vec<_>>();let bytes=crate::scene_archive::save(&scene).unwrap();open(&mut restored,&bytes).unwrap();
        assert_eq!(restored.objects().len(),2);assert_eq!(restored.objects().iter().filter(|row|row.cad.is_some()).count(),1);assert_eq!(restored.objects().iter().filter(|row|row.surface.is_some()).count(),1);
        assert_eq!(restored.clouds().len(),2);assert_eq!(restored.clouds()[1].model.m[12],1_000_000_000.125);assert_eq!(restored.clouds()[0].prepared.source.to_proto().coords,scene.clouds()[0].prepared.source.to_proto().coords);assert!(restored.clouds().iter().all(|row|row.point_px==5.&&row.prepared.source.point_size==3.));
        assert_eq!(restored.points()[0].prepared.source[0],scene.points()[0].prepared.source[0]);assert_eq!(restored.points()[0].prepared.source[0],0.123456789012345);let _=point;
        assert_eq!(restored.texts().len(),1);assert_eq!(restored.texts()[0].label(false,false).unwrap().placement,scene.texts()[0].label(false,false).unwrap().placement);assert!(!restored.visible(restored.texts()[0].id));assert!(restored.view.locked.contains(&restored.texts()[0].id));
        assert!(restored.view.current.is_some());assert!(restored.created.is_some());assert_eq!(restored.view.hidden.len(),2);assert_eq!(restored.view.locked.len(),3);assert_eq!(restored.view.faces[&restored.objects().iter().find(|row|row.cad.is_some()).unwrap().id],[2,5,7]);assert_eq!(restored.view.edges[&restored.objects().iter().find(|row|row.cad.is_some()).unwrap().id],[0;3]);assert!(restored.objects().iter().all(|row|!old_ids.contains(&row.id.value())));
        let again=crate::archive::Bundle::decode(&crate::scene_archive::save(&restored).unwrap()).unwrap();assert_eq!(again.documents.len(),2);assert_eq!(again.metadata.hidden.len(),2);assert_eq!(again.metadata.locked.len(),3);
    }
    #[test]
    fn repeated_guid_documents_reopen_with_independent_flags_and_original_meshes() {
        let mut scene=Scene::demo();scene.clear();let raw=crate::specimen::bytes();scene.import(crate::document::load(&raw).unwrap()).unwrap();let n=scene.objects().len();scene.import(crate::document::load(&raw).unwrap()).unwrap();let a=scene.objects()[0].id;let b=scene.objects()[n].id;Rc::make_mut(&mut scene.view).hide(&[a],true);Rc::make_mut(&mut scene.view).lock(&[b],true);
        let saved=crate::scene_archive::save(&scene).unwrap();let mut restored=Scene::demo();open(&mut restored,&saved).unwrap();assert_eq!(restored.objects().len(),2*n);let a=&restored.objects()[0];let b=&restored.objects()[n];assert_ne!(a.guid,b.guid);assert_eq!(a.geometry().unwrap().guid(),b.geometry().unwrap().guid());assert!(!restored.visible(a.id));assert!(restored.visible(b.id));assert!(!restored.view.locked.contains(&a.id));assert!(restored.view.locked.contains(&b.id));
        let document=Rc::downgrade(&a.source().unwrap().document);let ids=restored.objects().iter().take(n).map(|row|row.id).collect::<Vec<_>>();for id in ids {assert!(restored.remove(id));}assert!(document.upgrade().is_none());
    }
    #[test]
    fn rejected_files_and_late_display_preparation_errors_leave_the_scene_unchanged() {
        let mut scene=Scene::demo();let ids=scene.objects().iter().map(|row|row.id).collect::<Vec<_>>();assert!(open(&mut scene,b"broken").is_err());assert_eq!(scene.objects().iter().map(|row|row.id).collect::<Vec<_>>(),ids);
        let mut source=proto::Session::decode(crate::specimen::bytes().as_slice()).unwrap();source.tree=None;source.objects.as_mut().unwrap().points.push(session_rust::Point::new(1e100,0.,0.).to_proto());let bytes=source.encode_to_vec();assert!(import(&mut scene,&bytes,"too large",&Xform::identity(),0.).is_err());assert_eq!(scene.objects().iter().map(|row|row.id).collect::<Vec<_>>(),ids);assert!(scene.points().is_empty());assert!(scene.forest.documents.iter().all(|doc|doc.label!="too large"));
        source.objects.as_mut().unwrap().points.clear();source.definitions=Some(proto::Objects {instances:vec![Default::default()],..Default::default()});assert!(import(&mut scene,&source.encode_to_vec(),"unsupported",&Xform::identity(),0.).unwrap_err().contains("definition"));assert_eq!(scene.objects().iter().map(|row|row.id).collect::<Vec<_>>(),ids);
    }
    #[test]
    fn document_placement_is_composed_without_mutating_original_source_coordinates() {
        let mut scene=Scene::demo();scene.clear();let cloud=crate::cloud::example();let _=cloud.guid();let original=cloud.to_proto();let source=proto::Session {name:"precise cloud".into(),objects:Some(proto::Objects {pointclouds:vec![original.clone()],..Default::default()}),..Default::default()};let raw=source.encode_to_vec();let place=Xform::translation(1_000_000_000.125,0.,0.);import(&mut scene,&raw,"placed",&place,7.).unwrap();assert_eq!(scene.clouds()[0].model.m[12],place.m[12]);assert_eq!(scene.clouds()[0].prepared.source.to_proto(),original);assert_eq!(scene.clouds()[0].point_px,7.);
    }
    #[test]
    fn repeated_roundtrips_keep_tree_depth_current_layer_source_header_and_precise_pose() {
        let mut source=proto::Session::decode(crate::specimen::bytes().as_slice()).unwrap();source.guid="00000000-0000-0000-0000-000000000123".into();let raw=source.encode_to_vec();let mut scene=Scene::empty();import(&mut scene,&raw,"placed",&Xform::translation(0.123456789012345,0.,0.),0.).unwrap();let doc=&scene.forest.documents[0];let current=format!("{}/{}",doc.key,doc.nodes[0].source.guid);Rc::make_mut(&mut scene.view).current=Some(current);let nodes=doc.nodes.len();let depth=doc.nodes.iter().map(|node|node.depth).max().unwrap();
        for _ in 0..6 {let saved=crate::scene_archive::save(&scene).unwrap();let bundle=crate::archive::Bundle::decode(&saved).unwrap();assert_eq!(proto::Session::decode(bundle.documents[0].as_slice()).unwrap().guid,source.guid);open(&mut scene,&saved).unwrap();assert_eq!(scene.forest.documents[0].nodes.len(),nodes);assert_eq!(scene.forest.documents[0].nodes.iter().map(|node|node.depth).max().unwrap(),depth);assert_eq!(scene.objects()[0].model.m[12],0.123456789012345);assert!(scene.view.current.is_some());}
    }
    #[test]
    fn moving_repeated_source_objects_into_one_layer_saves_distinct_editable_records() {
        let raw=crate::specimen::bytes();let mut scene=Scene::empty();import(&mut scene,&raw,"first",&Xform::identity(),0.).unwrap();let n=scene.objects().len();import(&mut scene,&raw,"second",&Xform::identity(),0.).unwrap();let a=scene.objects()[0].id;let b=scene.objects()[n].id;let doc=&scene.forest.documents[0];let key=format!("{}/{}",doc.key,doc.nodes[0].source.guid);crate::layer_edit::apply(&mut scene,&key,crate::layer_edit::Edit::Move(vec![b])).unwrap();
        let saved=crate::scene_archive::save(&scene).unwrap();let bundle=crate::archive::Bundle::decode(&saved).unwrap();let source=proto::Session::decode(bundle.documents[0].as_slice()).unwrap();let meshes=source.objects.unwrap().meshes;assert_eq!(meshes.len(),n+1);let guids=meshes.iter().map(|mesh|&mesh.guid).collect::<HashSet<_>>();assert_eq!(guids.len(),n+1);assert_eq!(scene.objects().iter().find(|row|row.id==a).unwrap().geometry().unwrap().guid(),scene.objects().iter().find(|row|row.id==b).unwrap().geometry().unwrap().guid());let mut restored=Scene::empty();open(&mut restored,&saved).unwrap();assert_eq!(restored.objects().len(),2*n);
    }

}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-9.rs"
```

```text
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-10.rs"
```

```text
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-11.rs"
```

```text
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-12.rs"
```

</details>

### 6. `src/publication.rs`

Remove the older mesh reader and unmeasured fallback; preserve the existing archive path.

<details>
<summary>Locate the existing block</summary>

```rust
    let url=location.as_ref().map(|location|location.value().to_owned());
    match crate::document::load_at(bytes,location) {
        Ok(loaded)=>candidate.import(loaded).map_err(str::to_owned)?,
        Err(_)=>{let previous=candidate.all_ids();crate::scene_document::import(&mut candidate,bytes,"",&session_rust::Xform::identity(),0.)?;if let Some(url)=url {candidate.sheet_import_url(&url,&previous);}}
    }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-13.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/publication.rs`

Supply full original Element/interaction preparation and atomic malformed-file checks with retained reload origin.

<details>
<summary>Locate the existing block</summary>

```text
t(),1);assert_eq!(editor.camera.uniform(),camera);assert_eq!(editor.scene.clouds().len(),1);editor.apply(Action::Undo).unwrap();assert_eq!(editor.scene.clouds().len(),1);
    }

}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-14.rs" skip=180
```

</details>

### 8. `src/publication.rs`

Reuse the existing sheet URL initializer for newly imported originals; preserve earlier sheets’ metadata bases.

<details>
<summary>Locate the existing block</summary>

```rust
    crate::scene_document::import_at(&mut candidate,bytes,"",&session_rust::Xform::identity(),0.,location)?;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    let previous=candidate.all_ids();let url=location.as_ref().map(|location|location.value().to_owned());
    crate::scene_document::import_at(&mut candidate,bytes,"",&session_rust::Xform::identity(),0.,location)?;
    if let Some(url)=url {candidate.sheet_import_url(&url,&previous);}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzq-open-files-through-one-measured-session-reader-15.rs"
```

</details>

</details>

<a id="checkpoint-96zzr-route-current-close-and-opacity-commands"></a>

### Step 12 of 14 · Route current Close and Opacity commands (`96zzr-route-current-close-and-opacity-commands`)

- **Goal:** Finish an active curve with Close and read the current Opacity argument convention.
- **Data Flow Trace:** `command words → existing editor action → draft or display state`
- **Files You Touch:** `src/commands.rs` (*Route the public command through the existing curve-closing action.*) · `src/editor.rs` (*Protect other tools and construction-plane picks before building a closed curve.*) · `src/opacity_command_tests.rs` (*Update the original invalid-input fixture for the required current parser behavior.*) · `src/current_command_tests.rs` (*Supply original-document, one-undo, retained-draft and precision-boundary fixtures.*) · `src/lib.rs` (*Register the optional command fixtures.*) · `src/inventory_tests.rs` (*Update the comparison fixture to require the now-matched first-argument convention and its exact value.*)

Route Close through the existing curve builder and one history transaction, protecting other tools and construction-plane picks. Read the first Opacity value as a finite double before narrowing for the shader. Later words follow the current parser convention.

> **Self-Check Question:** *Why keep document release separate from Close?*  
> **Answer:** Close finishes an unfinished curve. A document lifetime action owns release and history cleanup; a drawing command must not erase existing geometry.

- **Hands-On Lab Experiment:** With existing geometry visible, run Close without a draft and confirm it remains. Begin Polyline, enter three points separately, run Close, and Undo once. Try Opacity 0.5 extra, then Opacity 1.000000001 extra; only the first is valid.

<details>
<summary>Implementation 96zzr-route-current-close-and-opacity-commands: Route current Close and Opacity commands</summary>

Finish an active curve with Close and read the current Opacity argument convention.

Data flow: command words → existing editor action → draft or display state.

### 1. `src/commands.rs`

Route the public command through the existing curve-closing action.

<details>
<summary>Locate the existing block</summary>

```rust
    ("Close", || Action::Close),
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    ("Close", || Action::CloseTool),
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzr-route-current-close-and-opacity-commands-1.rs"
```

</details>

### 2. `src/editor.rs`

Protect other tools and construction-plane picks before building a closed curve.

<details>
<summary>Locate the existing block</summary>

```rust
                if draft.gather.is_some()||draft.project.is_some()||draft.extend.is_some(){return Err("Close works on a polyline or curve")}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
                if self.plane_only || draft.plane_points.is_some() || !matches!(draft.tool.name(),"Polyline"|"Curve"){return Err("Close works on a polyline or curve")}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzr-route-current-close-and-opacity-commands-2.rs"
```

</details>

### 3. `src/commands.rs`

Validate the first argument in the same precision as the current command parser.

<details>
<summary>Locate the existing block</summary>

```rust
    if name=="Opacity"{let [word]=&words[count..]else{return Err("Opacity 0..1 · Example: Opacity 0.5".into())};let value=word.parse::<f32>().ok().filter(|v|v.is_finite()&&(0.0..=1.0).contains(v)).ok_or("Opacity takes a value from 0 to 1")?;return Ok(Command::Editor(Action::Opacity(value)));}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    if name=="Opacity"{let word=words.get(count).ok_or("Opacity 0..1 · Example: Opacity 0.5")?;let value=word.parse::<f64>().ok().filter(|v|v.is_finite()&&(0.0..=1.0).contains(v)).ok_or("Opacity takes a value from 0 to 1")?;return Ok(Command::Editor(Action::Opacity(value as f32)));}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzr-route-current-close-and-opacity-commands-3.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/opacity_command_tests.rs`

Update the original invalid-input fixture for the required current parser behavior.

<details>
<summary>Locate the existing block</summary>

```text
"Opacity NaN","Opacity 0.5 extra"
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzr-route-current-close-and-opacity-commands-4.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/current_command_tests.rs`

Supply original-document, one-undo, retained-draft and precision-boundary fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/96zzr-route-current-close-and-opacity-commands-5.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/lib.rs`

Register the optional command fixtures.

<details>
<summary>Locate the existing block</summary>

```text
&error))
}

mod shader_check;
#[cfg(test)]
mod cloud_shader_contract;

mod draw_slots;
#[cfg(test)] mod draw_slots_tests;

#[cfg(test)] mod source_phase_tests;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzr-route-current-close-and-opacity-commands-6.rs" skip=160
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/inventory_tests.rs`

Update the comparison fixture to require the now-matched first-argument convention and its exact value.

<details>
<summary>Locate the existing block</summary>

```text
assert!(crate::commands::parse("Opacity 0.4 extra").is_err());
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzr-route-current-close-and-opacity-commands-7.rs"
```

</details>

</details>

<a id="checkpoint-96zzs-reuse-full-source-preparation-for-editor-and-reload"></a>

### Step 13 of 14 · Reuse full source preparation for editor and reload (`96zzs-reuse-full-source-preparation-for-editor-and-reload`)

- **Goal:** Reuse the full Session reader for byte actions and released mesh sources.
- **Data Flow Trace:** `source bytes → shared full reader → prepared scene or restored source`
- **Files You Touch:** `src/editor.rs` (*Prepare every byte action once through the existing full candidate reader.*) · `src/document.rs` (*Validate the same full original file before recovering a released mesh source.*) · `src/full_source_action_tests.rs` (*Supply all four editor actions, original Element payloads, shared locations, one-undo and mixed-file reload fixtures.*) · `src/lib.rs` (*Register the optional full-source action fixtures.*)

Route all editor byte imports through the existing atomic candidate preparation. Reuse that reader when restoring a released mesh from a mixed Session file, checking its file version first. Keep source locations and original payloads; avoid a second geometry-type whitelist.

> **Self-Check Question:** *Why must a mesh reload accept the entire original file?*  
> **Answer:** A mesh can share its Session file with points, Elements and other originals. Reloading its source must validate that complete file, then recover the mesh by its original GUID.

- **Hands-On Lab Experiment:** Trace an imported mixed file through candidate preparation, original source release and reload. Identify which owner keeps the reload location and which check rejects changed bytes.

<details>
<summary>Implementation 96zzs-reuse-full-source-preparation-for-editor-and-reload: Reuse full source preparation for editor and reload</summary>

Reuse the full Session reader for byte actions and released mesh sources.

Data flow: source bytes → shared full reader → prepared scene or restored source.

### 1. `src/editor.rs`

Prepare every byte action once through the existing full candidate reader. Route the nested example through the same original-document boundary.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            Action::AddInstances=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare original definition instances"))?;},
            Action::AddCad=>{let source=std::rc::Rc::new(crate::cad_display::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_cad(source).map(|_|()))?;},
            Action::AddSurface=>{let source=std::rc::Rc::new(crate::surface::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_surface(source).map(|_|()))?;},
            Action::AddNested=>{let loaded=crate::document::load(&crate::tree_specimen::nested(true))?;self.history.try_edit(&mut self.scene,|scene|scene.import(loaded))?;},
            Action::Live(ready)=>{self.history.clear();self.scene.restore(*ready);},
            Action::Adopt {scene:ready,..}=>{self.history.edit(&mut self.scene,|scene|scene.restore(*ready));},
            Action::Replace(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.replace(loaded))?;
            }
            Action::Import(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ImportAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ReplaceAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history.try_edit(&mut self.scene, |scene| scene.replace(loaded))?;
            }
            Action::AddPolyline | Action::AddArrow => {
                let prepared = if matches!(action, Action::AddPolyline) {
                    let mut line = session_rust::Polyline::new(vec![session_rust::Point::new(-0.6, -0.4, 0.0),
```

</details>

Replace it with:

```rust
            Action::AddInstances=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare original definition instances"))?;},
            Action::AddCad=>{let source=std::rc::Rc::new(crate::cad_display::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_cad(source).map(|_|()))?;},
            Action::AddSurface=>{let source=std::rc::Rc::new(crate::surface::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_surface(source).map(|_|()))?;},
            Action::AddNested=>{let ready=crate::publication::prepare(&self.scene,&crate::tree_specimen::nested(true),false,None).map_err(|_|"Cannot prepare nested originals")?;self.history.edit(&mut self.scene,|scene|scene.restore(ready));},
            Action::Live(ready)=>{self.history.clear();self.scene.restore(*ready);},
            Action::Adopt {scene:ready,..}=>{self.history.edit(&mut self.scene,|scene|scene.restore(*ready));},
            file @ (Action::Replace(_) | Action::Import(_) | Action::ImportAt(_,_) | Action::ReplaceAt(_,_)) => {
                let (bytes,replace,location)=match file {
                    Action::Replace(bytes)=>(bytes,true,None),
                    Action::Import(bytes)=>(bytes,false,None),
                    Action::ImportAt(bytes,location)=>(bytes,false,Some(location)),
                    Action::ReplaceAt(bytes,location)=>(bytes,true,Some(location)),
                    _=>unreachable!(),
                };
                let ready=crate::publication::prepare(&self.scene,&bytes,replace,location).map_err(|_|"Cannot prepare original document")?;
                self.history.edit(&mut self.scene,|scene|scene.restore(ready));
            }
            Action::AddPolyline | Action::AddArrow => {
                let prepared = if matches!(action, Action::AddPolyline) {
                    let mut line = session_rust::Polyline::new(vec![session_rust::Point::new(-0.6, -0.4, 0.0),
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{background::Background, camera::Camera, history::History, scene::{ObjectId, Scene}};

pub enum Action {
    ElementBegin{kind:crate::element_input::Kind,text:String},ElementText(String),ElementPick(Option<ObjectId>),ElementAttributes(Option<bool>),ElementInteractions(Option<bool>),
    Opacity(f32),
    Arctic(Option<bool>),
    ViewOutline(Option<bool>),
    Clipping(crate::clipping_input::Request),
    SelectLasso,LassoPress{at:[f64;2],size:[f64;2],density:f64,pointer:i32},LassoMove{at:[f64;2],pointer:i32},LassoRelease{at:[f64;2],pointer:i32,add:bool,remove:bool},LassoCancel,
    AddGroup(String),AddEdge,
    StreamRefresh,
    Trim,TrimPick(Option<ObjectId>),TrimScreen{screen:[f32;2],size:[f64;2],commit:bool},CancelTrim,
    TextBegin(String),
    Explode,
    Query(crate::selection_query::Query),Arrowhead(session_rust::Arrowhead),Escape,
    NamedView(crate::camera::View),
    Measure(crate::measure_mark::Kind),MeasureDistance(String),BoundingBox,
    Split,SplitPick(Option<ObjectId>),CancelSplit,
    Project(Vec<String>),ExtendBegin{distance:Option<f64>,asking:bool},ExtendPick(Option<ObjectId>),Interval(crate::interval_geometry::Interval),
    GatherBegin{recipe:&'static crate::gather_input::Recipe,text:String},GatherPick(Option<ObjectId>),
    BeginTransformation(crate::transform_draft::Kind),
    ScaleFactor(f64),RotateWorld{axis:crate::gizmo::Axis,degrees:f64},
    Transformation {kind:crate::transform_draft::Kind,delta:session_rust::Xform},
    PrimitiveBegin {shape:&'static crate::primitive_shape::Shape,option:usize,text:String},
    ToolSides(usize),
    ConstructionPlane(Option<crate::cplane::CPlane>), PlaneMenu, Plane3,
    Draw {shape:crate::tool::Shape,text:String}, ToolText(String), FinishTool, CloseTool,
    CanvasStart {from:[f32;2],to:[f32;2],size:[f64;2],modified:bool,pointer:i32},
    CanvasMove {screen:[f32;2],size:[f64;2]},
    CanvasRelease {screen:[f32;2],size:[f64;2],add:bool,remove:bool},
    CanvasHit(Option<ObjectId>),CanvasCancel,
    SelectRows {ids:Vec<ObjectId>,add:bool,remove:bool},
    PanScreen {delta:[f64;2],size:[f64;2]},ZoomAt {factor:f32,screen:[f32;2]},
    Touch(Vec<Action>),
    LayerEdit {key:String,edit:crate::layer_edit::Edit},
    LayerProperty {key:String,property:crate::layer_properties::Property},
    TreePick {ids:Vec<ObjectId>,additive:bool},
    GizmoPress {pointer:i32,screen:[f32;2],size:[f64;2],modified:bool},
    GizmoTouch {pointer:i32,screen:[f32;2],size:[f64;2]},
    GizmoMove {pointer:i32,screen:[f32;2],size:[f64;2],shift:bool},
    GizmoRelease {pointer:i32,screen:[f32;2],size:[f64;2],shift:bool},
    GizmoCancel,
    GizmoNumber(String),
    Snap(crate::snap::Setting),
    ToolScreen { screen: [f32; 2], size: [f64; 2], commit: bool },
    BeginTool(crate::tool::Shape),
    ToolPoint([f64; 3]),
    ToolHover(Option<[f64; 3]>),
    CancelTool,
    Pick([f32; 2]),
    Picked(Option<ObjectId>),
    PickedTarget(Option<crate::pick_sources::Target>),
    PickedMany { target: Option<crate::pick_sources::Target>, additive: bool },
    SelectAll,
    PickMode(crate::pick_sources::Mode),
    Live(Box<Scene>),
    Adopt {scene:Box<Scene>,replace:bool},
    Import(Vec<u8>),
    Replace(Vec<u8>),
    ImportAt(Vec<u8>, std::rc::Rc<crate::reload_url::ReloadUrl>),
    ReplaceAt(Vec<u8>, std::rc::Rc<crate::reload_url::ReloadUrl>),
    Close,
    UnloadSources,
    Hide,
    Show,
    AddBox,
    AddNested,
    AddCloudLod,
    AddCloud,
    AddCad,
    AddInstances,
    AddShapeReuse,
    AddNestedInstances,
    AddSheet,
    AddText(crate::text::TextPlacement),
    AddSurface,
    AddLine,
    AddPoint,
    AddCurve,
    AddColours(crate::colour::Mode),
    NormalView(bool),
    Grid(Option<crate::grid::Settings>),
    AddNormals(crate::normal_example::Mode),
    EnableControls,
    Controls(bool),
    AddPolyline,
    AddArrow,
    Translate([f64; 3]),
    ToggleExtra,
    SelectNext,
    Delete,
    Undo,
    Redo,
    Background,
    Zoom(f32),
    Pan(f32, f32),
    Orbit(f64, f64),
    Isometric,
    ResetView,
    Fit,
    FitSelected,
    Projection(crate::camera::Projection),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Change {
    View,
    Scene,
    Preview,
}

pub struct Editor {
    pub element:Option<crate::element_draft::Draft>,
    pub element_attributes:bool,
    pub element_interactions:bool,
    pub opacity:f32,
    pub arctic:bool,
    pub outlines:bool,
    pub clipping_enabled:bool,pub clipping_solid:bool,
    pub interaction_epoch:u64,
    pub lasso:Option<crate::lasso::Lasso>,
    pub trim:Option<crate::trim_draft::Draft>,
    pub last_text_height:f64,
    pub mark:Option<crate::measure_mark::Mark>,pub unit:crate::measure_mark::Unit,
    pub split:Option<crate::split_draft::Draft>,
    pub canvas_drag:Option<crate::canvas_drag::Drag>,
    pub gizmo_edit:Option<crate::gizmo_edit::Edit>,
    gizmo_hover:Option<crate::gizmo::Handle>,
    pub edit_preview: Option<crate::edit_preview::Preview>,
    pub transform_preview:Option<crate::gizmo_edit::Preview>,
    pub snapping: crate::snap::Settings,
    snap_index: Option<crate::snap_index::Index>,
    pub draft: Option<crate::tool::Draft>,
    pub construction_plane:Option<crate::cplane::CPlane>,
    pub plane_menu:bool,
    pub plane_only:bool,
    pub tool_status: String,
    pub scene: Scene,
    pub selected: Option<ObjectId>,
    pub selected_part: Option<crate::pick_sources::Part>,
    pub pick_mode: crate::pick_sources::Mode,
    objects: crate::object_selection::Objects,
    pub camera: Camera,
    pub background: Background,
    pub normal_view: bool,
    history: History,
    last_release: u64,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            element:None,element_attributes:false,element_interactions:true,opacity:1.,arctic:false,outlines:false,clipping_enabled:true,clipping_solid:true,interaction_epoch:0,lasso:None,trim:None,last_text_height:0.,mark:None,unit:crate::measure_mark::Unit::Millimeters,split:None,canvas_drag:None, gizmo_edit:None, gizmo_hover:None, edit_preview: None, snapping: Default::default(), snap_index: None,
            transform_preview:None,draft: None, tool_status: String::new(), construction_plane:None, plane_menu:false, plane_only:false,
            scene: Scene::demo(),
            selected: None, selected_part: None, pick_mode: Default::default(), objects: Default::default(),
            camera: Camera::default(),
            background: Background::default(),
            normal_view: false,
            history: History::default(),
            last_release: 0,
        }
    }
}

impl Editor {
    pub fn display_scene(&self) -> &Scene {
        if let Some(scene)=self.trim.as_ref().and_then(|t|t.preview.as_ref()){return scene}
        self.canvas_drag.as_ref().and_then(|d|d.preview.as_ref()).map_or_else(||self.gizmo_edit.as_ref().map_or_else(||self.transform_preview.as_ref().map_or_else(||self.edit_preview.as_ref().map_or(&self.scene,|p| &p.scene),|p|p.scene()),|g|g.preview.scene()),|p|&p.scene)
    }

    pub fn gizmo_origin(&self)->Option<[f64;3]> {
        if self.trim.is_some()||self.split.is_some()||self.draft.is_some()||self.canvas_drag.is_some() {return None;}
        if self.scene.clouds().iter().any(|r|r.controls_visible)||self.scene.streams().iter().any(|r|r.controls_visible) {return None;}
        if self.pick_mode==crate::pick_sources::Mode::Control && self.selected_part.is_none() {return None;}
        if let Some(edit)=self.gizmo_edit.as_ref() {return Some(edit.origin());}
        if let Some(target)=self.selection().filter(|_|self.selected_part.is_some()) {return crate::part_preview::origin(&self.scene,target);}
        self.scene.bounds_of(&self.selected_objects()).map(|b|b.centre())
    }

    pub fn gizmo_hit(&self,screen:[f32;2],size:[f64;2])->Option<crate::gizmo::Handle> {self.gizmo_hit_radius(screen,size,8.)}

    pub fn gizmo_hit_radius(&self,screen:[f32;2],size:[f64;2],radius:f64)->Option<crate::gizmo::Handle> {
        if size.iter().any(|v| !v.is_finite() || *v<=0.) {return None;}
        let origin=self.gizmo_origin()?;
        let scale=crate::widget_layout::world_per_pixel(&self.camera.view_projection(),origin,size[1])?;
        let ray=self.camera.ray(screen)?;
        crate::gizmo::Gizmo::new(session_rust::Point::new(origin[0],origin[1],origin[2])).hit_with_radius(&ray.origin,&ray.direction,scale,radius)
    }

    pub fn number_prompt(&self,size:[f64;2])->Option<crate::number_box::NumberPrompt> {
        let edit=self.gizmo_edit.as_ref()?;let handle=edit.gizmo.typing?;
        let origin=edit.origin();let matrix=self.camera.view_projection();let scale=crate::widget_layout::world_per_pixel(&matrix,origin,size[1])?;
        let point=edit.gizmo.handle_point(handle,scale);
        let at=crate::snap_math::Screen {matrix:matrix.m,size}.point([point[0],point[1],point[2]])?;
        let unit=handle.labels().2;
        Some(crate::number_box::NumberPrompt {handle,title:handle.title(),unit,hint:if unit=="factor" {"1"} else {"0"},at:at.map(|v|v as f32)})
    }

    pub fn gizmo_active(&self)->f32 {crate::gizmo_edit::code(self.gizmo_edit.as_ref().and_then(|e|e.active()).or(self.gizmo_hover))}

    pub fn hover_gizmo(&mut self,screen:Option<([f32;2],[f64;2])>) {
        self.gizmo_hover=if self.gizmo_edit.is_some() {None} else {screen.and_then(|(point,size)|self.gizmo_hit(point,size))};
    }

    fn gizmo_press(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],modified:bool)->Result<Change,&'static str> {
        self.gizmo_press_radius(pointer,screen,size,modified,8.)
    }

    fn gizmo_press_radius(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],modified:bool,radius:f64)->Result<Change,&'static str> {
        if modified || self.draft.is_some() || self.gizmo_edit.as_ref().is_some_and(|g|g.pointer.is_some()) {return Ok(Change::View);}
        let Some(handle)=self.gizmo_hit_radius(screen,size,radius) else {return Ok(Change::View);};
        let origin=self.gizmo_origin().ok_or("No editable selection")?;
        let preview=crate::gizmo_edit::Preview::new(&self.scene,self.selected_objects(),self.selection().filter(|_|self.selected_part.is_some()))?;
        let ray=self.camera.ray(screen).ok_or("Invalid handle ray")?;
        let gizmo=crate::gizmo::Gizmo::new(session_rust::Point::new(origin[0],origin[1],origin[2]));
        self.gizmo_edit=crate::gizmo_edit::Edit::begin(gizmo,handle,&ray,preview,pointer,screen);Ok(Change::View)
    }

    fn gizmo_move(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],shift:bool)->Result<Change,&'static str> {
        if size.iter().any(|v| !v.is_finite() || *v<=0.) {return Err("Invalid handle viewport");}
        let Some(ray)=self.camera.ray(screen) else {return Err("Invalid handle ray");};
        let Some(edit)=self.gizmo_edit.as_mut() else {return Ok(Change::View);};
        Ok(if edit.update(pointer,screen,size,&ray,shift)? {Change::Preview} else {Change::View})
    }

    fn gizmo_release(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],shift:bool)->Result<Change,&'static str> {
        if self.gizmo_edit.as_ref().is_none_or(|g|g.pointer!=Some(pointer)) {return Ok(Change::View);}
        self.gizmo_move(pointer,screen,size,shift)?;
        let edit=self.gizmo_edit.as_mut().unwrap();
        if edit.click() {self.tool_status=format!("{}: type a value in {}, Enter applies, Esc closes",edit.active().unwrap().title(),edit.active().unwrap().labels().2);return Ok(Change::Preview);}
        let changed=edit.preview.commit(&mut self.scene,&mut self.history)?; self.gizmo_edit=None;
        self.tool_status="Handle edit completed.".into();Ok(if changed {Change::Scene} else {Change::Preview})
    }

    fn gizmo_number(&mut self,text:&str)->Result<Change,&'static str> {
        let edit=self.gizmo_edit.as_mut().ok_or("No handle number is being requested")?;
        let handle=edit.gizmo.typing.ok_or("No handle number is being requested")?;
        let value=crate::gizmo::typed_value(handle,text).map_err(|_|"Enter a finite number; scale needs a positive factor")?;
        let Some(value)=value else {self.gizmo_edit=None;return Ok(Change::Preview);};
        edit.preview.set(edit.gizmo.typed(handle,value))?;
        let changed=edit.preview.commit(&mut self.scene,&mut self.history)?;self.gizmo_edit=None;
        self.tool_status=format!("{} {value} {}",handle.title(),handle.labels().2);Ok(if changed {Change::Scene} else {Change::Preview})
    }

    fn update_edit_preview(&mut self) -> Result<Change, &'static str> {
        if let Some(transform)=self.draft.as_ref().and_then(|draft|draft.transform.as_ref()) {
            let matrix=transform.preview(self.draft.as_ref().and_then(|draft|draft.cursor)).unwrap_or_else(session_rust::Xform::identity);
            if let Some(preview)=self.transform_preview.as_mut(){preview.set(matrix)?;return Ok(Change::Preview);}return Ok(Change::View);
        }
        let Some(preview)=self.edit_preview.as_mut() else { return Ok(Change::View); };
        let offset=self.draft.as_ref().and_then(|d| d.points.first().zip(d.cursor)).map(|(a,b)| std::array::from_fn::<_,3,_>(|i| b[i]-a[i])).unwrap_or([0.;3]);
        preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?; Ok(Change::Preview)
    }

    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }

    pub fn presentation_scenes(&self)->impl Iterator<Item=&Scene> {
        self.scenes().chain(self.transform_preview.iter().flat_map(|p|p.scenes())).chain(self.canvas_drag.iter().filter_map(|d|d.preview.as_ref()).flat_map(|p|p.scenes())).chain(self.edit_preview.iter().flat_map(|p|p.scenes())).chain(self.gizmo_edit.iter().flat_map(|g|g.preview.scenes()))
    }

    pub fn reload_keys(&self) -> Vec<crate::rehydrate::ReloadKey> {
        let mut seen = std::collections::HashSet::new();
        self.scene.objects().iter().filter_map(crate::rehydrate::ReloadKey::of)
            .filter(|key| seen.insert(key.origin.id)).collect()
    }

    fn scenes_mut(&mut self) -> impl Iterator<Item = &mut Scene> {
        std::iter::once(&mut self.scene).chain(self.history.scenes_mut())
    }

    pub fn hydrate(&mut self, values: Vec<(crate::rehydrate::ReloadKey, Vec<u8>)>) -> Result<bool, &'static str> {
        if values.is_empty() { return Ok(false); }
        let mut seen = std::collections::HashSet::new();
        for (key, _) in &values {
            if !seen.insert(key.origin.id) || !self.scene.objects().iter().any(|row| key.matches(row)) {
                return Ok(false);
            }
        }
        let prepared: Vec<_> = values.into_iter().map(|(key, bytes)|
            crate::rehydrate::PreparedReload::new(key, &bytes)).collect::<Result<_, _>>()?;
        for plan in &prepared {
            if self.scenes().flat_map(|scene| scene.objects()).filter(|row| plan.key.matches(row))
                .any(|row| !plan.validates(row)) { return Err("Reloaded source GUID or metadata does not match"); }
        }
        for scene in self.scenes_mut() {
            for plan in &prepared { scene.hydrate(plan); }
        }
        if self.edit_preview.take().is_some() { self.draft=None; } self.gizmo_edit=None; self.snap_index = None; Ok(true)
    }

    fn unload_sources(&mut self) -> Result<(), &'static str> {
        let mut origins: std::collections::HashSet<_> = self.scenes().flat_map(|scene| scene.objects())
            .filter(|row| row.can_unload()).filter_map(|row| row.origin().map(|origin| origin.id)).collect();
        for row in self.scenes().flat_map(|scene| scene.objects()) {
            if let Some(origin) = row.origin() {
                if !row.can_unload() { origins.remove(&origin.id); }
            }
        }
        if origins.is_empty() { return Err("No eligible imported sources to unload"); }
        let epoch = self.last_release.checked_add(1).ok_or("Release epochs exhausted")?;
        for scene in self.scenes_mut() { scene.unload(&origins, epoch); }
        self.last_release = epoch;
        Ok(())
    }

    pub fn move_object(&mut self, id: ObjectId, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|value| value.is_finite()) { return Err("Move needs finite coordinates"); }
        if !self.scene.contains(id) {return Err("Object not found");}
        let object=self.scene.objects().iter().find(|row|row.id==id);
        if self.scene.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        if object.is_some_and(|object|object.geometry().is_none()) {return Err("Reload editable sources before Move");}
        if offset.iter().all(|&value| value == 0.0) { return Ok(Change::View); }
        let ids=self.scene.with_descendants(&[id]);let delta=session_rust::Xform::translation(offset[0],offset[1],offset[2]);
        self.history.try_edit(&mut self.scene,|scene| {for id in ids {let matrix=&delta*&scene.placement(id).ok_or("Object not found")?;scene.place_following(id,matrix)?;}Ok(())})?;
        if self.edit_preview.take().is_some() { self.draft=None; }
        self.gizmo_edit=None; self.snap_index = None; Ok(Change::Scene)
    }

    pub fn delete_object(&mut self, id: ObjectId) -> Result<Change, &'static str> {
        if !self.scene.contains(id) {return Err("Object not found");}
        if self.scene.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        if self.scene.objects().iter().find(|row| row.id == id).is_some_and(|row| row.geometry().is_none()) {
            return Err("Reload editable sources before Delete");
        }
        let ids=self.scene.with_descendants(&[id]);
        self.history.edit(&mut self.scene, |scene| {for id in ids {scene.remove(id);}});
        if self.edit_preview.take().is_some() { self.draft=None; }
        self.objects.retain(&self.scene);
        if self.selected == Some(id) { self.selected = self.objects.primary(); self.selected_part = None; }
        self.gizmo_edit=None; self.snap_index = None; Ok(Change::Scene)
    }

    pub fn selected_objects(&self) -> Vec<ObjectId> {
        if self.selected_part.is_some() { return Vec::new(); }
        if self.objects.primary() == self.selected { self.objects.rows().to_vec() }
        else { self.selected.into_iter().filter(|id| self.scene.pick_id(id.value()).is_some()).collect() }
    }

    pub fn select_objects(&mut self, rows: impl IntoIterator<Item = ObjectId>, additive: bool) {
        if additive { let old = self.selected_objects(); self.objects.set(&self.scene, old, false); }
        let rows=crate::graph_edit::expand(&self.scene,rows);
        self.selected_part = None; self.objects.set(&self.scene, rows, additive); self.selected = self.objects.primary();
    }

    pub fn select_rows(&mut self,ids:Vec<ObjectId>,add:bool,remove:bool) {
        if remove {let old=self.selected_objects();self.objects.set(&self.scene,old,false);self.objects.remove(&self.scene,ids);self.selected=self.objects.primary();self.selected_part=None;}
        else {self.select_objects(ids,add);}
    }

    fn finish_canvas(&mut self)->Result<Change,&'static str> {
        let Some(mut drag)=self.canvas_drag.take() else {return Ok(Change::View);};
        let (add,remove)=drag.released.unwrap_or((false,false));
        if let Some(preview)=drag.preview.take() {let changed=preview.commit(&mut self.scene,&mut self.history)?;if changed {self.snap_index=None;}return Ok(if changed {Change::Scene} else {Change::Preview});}
        if drag.missed {if let Some(rectangle)=drag.rectangle() {let ids=crate::rect_select::found(&self.scene,&self.camera.view_projection().m,drag.size,rectangle);self.select_rows(ids,add,remove);}}
        Ok(Change::View)
    }

    fn repair_selection(&mut self) {
        if self.objects.primary() == self.selected && self.selected_part.is_none() {
            self.objects.retain(&self.scene); self.selected = self.objects.primary();
        } else { self.objects.clear(); }
    }

    fn move_selection(&mut self, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|n| n.is_finite()) { return Err("Move needs finite coordinates"); }
        let ids = self.selected_objects(); if ids.is_empty() { return Err("Select an object before Move"); }
        if offset.iter().all(|&n| n == 0.) { return Ok(Change::View); }
        let mut preview=crate::edit_preview::Preview::new(&self.scene,ids)?;
        preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?;
        preview.commit(&mut self.scene,&mut self.history)?;Ok(Change::Scene)
    }

    fn delete_selection(&mut self) -> Result<Change, &'static str> {
        if self.selected_part.is_some() { return Err("Use Select Object before Delete"); }
        let ids = self.selected_objects(); if ids.is_empty() { return Ok(Change::View); }
        if self.scene.objects().iter().any(|r| ids.contains(&r.id) && r.geometry().is_none()) {
            return Err("Reload editable sources before Delete");
        }
        let ids=self.scene.with_descendants(&ids);
        self.history.edit(&mut self.scene, |scene| { for id in ids { scene.remove(id); } });
        self.objects.clear(); self.selected = None; Ok(Change::Scene)
    }

    pub fn selection(&self) -> Option<crate::pick_sources::Target> {
        self.selected.map(|owner| crate::pick_sources::Target { owner, part: self.selected_part.unwrap_or(crate::pick_sources::Part::Object) })
    }

    pub fn whole_selection(&self) -> Option<ObjectId> { self.selected.filter(|_| self.selected_part.is_none()) }

    pub fn drawing_plane(&self)->crate::cplane::CPlane{self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())))}
    pub fn tool_line(&self, text: &str) -> Option<Result<Action, &'static str>> {
        if self.element.is_some(){return Some(Ok(if text.trim().eq_ignore_ascii_case("Escape"){Action::CancelTool}else{Action::ElementText(text.into())}))}
        if self.draft.as_ref().is_some_and(|d|d.tool.name()=="Clipping Plane"){if let Some(mode)=crate::clipping_geometry::Mode::parse(text.trim()){return Some(Ok(Action::Clipping(crate::clipping_input::Request::Begin(mode))))}}
        if self.trim.is_some(){return match text.trim(){""=>Some(Ok(Action::Trim)),v if v.eq_ignore_ascii_case("Trim")=>Some(Ok(Action::Trim)),v if v.eq_ignore_ascii_case("Escape")||v.eq_ignore_ascii_case("Esc")=>Some(Ok(Action::CancelTrim)),_=>None}}
        if self.mark.is_some()&&text.trim().eq_ignore_ascii_case("Escape"){return Some(Ok(Action::CancelTool))}
        if self.split.is_some(){return match text.trim(){""=>Some(Ok(Action::Split)),v if v.eq_ignore_ascii_case("Split")=>Some(Ok(Action::Split)),v if v.eq_ignore_ascii_case("Escape")||v.eq_ignore_ascii_case("Esc")=>Some(Ok(Action::CancelSplit)),_=>None}}
        if self.gizmo_edit.as_ref().is_some_and(|e|e.gizmo.typing.is_some() || text.trim().eq_ignore_ascii_case("Escape")) {
            return Some(Ok(if text.trim().eq_ignore_ascii_case("Escape") {Action::GizmoCancel} else {Action::GizmoNumber(text.to_owned())}));
        }
        if self.plane_menu {return Some(crate::drawing_input::choice(text).ok_or("Choose XY, XZ, YZ, 3 Point, View or Cancel"));}
        let draft=self.draft.as_ref()?;
        if draft.plane_points.is_none(){if let Some(label)=&draft.text{if label.asking||text.split_whitespace().next().is_some_and(|w|w.eq_ignore_ascii_case("Height")){return Some(Ok(Action::ToolText(text.into())))}}}

        if let Some(choice)=crate::drawing_input::choice(text){return Some(Ok(choice));}
        if draft.extend.is_some()&&draft.plane_points.is_none(){let word=text.trim();if word.is_empty()||word.split_whitespace().next().is_some_and(|w|w.eq_ignore_ascii_case("Distance")||w.parse::<f64>().is_ok()){return Some(Ok(Action::ToolText(text.into())))}return None}
        if let Some(choice)=crate::drawing_input::choice(text) {return Some(Ok(choice));}
        if draft.plane_points.is_none(){if let Some(gather)=&draft.gather{let first=text.split_whitespace().next();if text.trim().is_empty()||first.is_some_and(|word|word.parse::<f64>().is_ok()||word.contains(',')||word.starts_with('w')||word.starts_with('W')||gather.recipe.option_in(&[word]).is_some()||gather.recipe.choices().any(|choice|crate::primitive_options::compact(choice).starts_with(&crate::primitive_options::compact(word)))||!gather.typed.is_empty()){return Some(Ok(Action::ToolText(text.into())))}return None}}
        if draft.plane_points.is_none() && draft.primitive.is_some() {
            let words:Vec<_>=text.split_whitespace().collect();let primitive=draft.primitive.as_ref().unwrap();
            if text.trim().is_empty() || primitive.shape.option_in(&words).is_some() || words.first().is_some_and(|word|word.parse::<f64>().is_ok() || crate::drawing_coords::parse_world(word).is_some() || word.contains(',') || word.contains('<') || word.starts_with('@')) {return Some(Ok(Action::ToolText(text.to_owned())));}
            return None;
        }
        if draft.plane_points.is_none() && draft.transform.is_some(){
            let first=text.split_whitespace().next();if text.trim().is_empty()||first.is_some_and(|word|word.parse::<f64>().is_ok()||word.contains(',')||word.contains('<')||word.starts_with('@')||word.starts_with('w')||word.starts_with('W')||["1D","2D","3D"].iter().any(|mode|word.eq_ignore_ascii_case(mode))){return Some(Ok(Action::ToolText(text.into())))}return None;
        }
        if text.trim().eq_ignore_ascii_case("Close") && draft.tool.count()==0 {return Some(Ok(Action::CloseTool));}
        if text.trim().is_empty() {return Some(Ok(if draft.tool.count()==0 && !self.plane_only {Action::FinishTool}else{Action::CancelTool}));}
        let word=text.trim();
        if word.split_whitespace().next().is_some_and(|s|s.eq_ignore_ascii_case("Sides")) && draft.tool.sides().is_some() {
            let parts:Vec<_>=word.split_whitespace().collect();
            return Some(match parts.as_slice() {[_name,value]=>value.parse::<usize>().ok().filter(|n|(3..crate::tool::MAX_POINTS).contains(n)).map(Action::ToolSides).ok_or("Polygon sides must be between 3 and 4095"),_=>Err("Use Sides 6")});
        }
        let coordinate=crate::drawing_coords::parse_world(word).is_some() || word.contains(',') || word.contains('<') || word.starts_with('@') || word.starts_with('w') || word.starts_with('W') || word.split_whitespace().next().is_some_and(|p|crate::drawing_coords::parse_world(p).is_some());
        coordinate.then(|| {
            let points=draft.plane_points.as_deref().unwrap_or(&draft.points);
            crate::drawing_input::points(word,self.construction_plane,draft.frame,points,draft.cursor).map(|_|Action::ToolText(word.to_owned()))
        })
    }

    fn place_tool_point(&mut self, point: [f64; 3]) -> Result<Change, &'static str> {
        if self.draft.as_ref().is_some_and(|d|d.project.is_some()&&d.plane_points.is_none()){return self.project_points(&[point])}
        if self.draft.as_ref().is_some_and(|d|d.extend.is_some()&&d.plane_points.is_none()){return Err("Extend uses curve-end clicks")}
        if self.draft.as_ref().is_some_and(|d|d.gather.is_some()&&d.plane_points.is_none()){return self.gather_point(point);}
        if self.draft.as_ref().is_some_and(|d|d.transform.is_some()&&d.plane_points.is_none()){return self.transform_point(point);}

        let draft=self.draft.as_mut().ok_or("No point tool is running")?;
        if let Some(points)=draft.plane_points.as_mut() {
            let point=crate::tool_coords::checked(point)?;let mut picked=points.clone();picked.push(point);
            if picked.len()==3 {
                let p:Vec<_>=picked.iter().map(|p|session_rust::Point::new(p[0],p[1],p[2])).collect();
                let plane=crate::cplane::CPlane::from_3_points(&p[0],&p[1],&p[2]).ok_or("The three points are in line; pick the third again")?;
                draft.plane_points=None;self.apply(Action::ConstructionPlane(Some(plane)))?;
                if self.plane_only {self.draft=None;self.plane_only=false;}return Ok(Change::View);
            }
            *points=picked;self.tool_status=draft.prompt();return Ok(Change::View);
        }
        if draft.primitive.is_some() {return self.primitive_answer(session_rust::Point::new(point[0],point[1],point[2]));}
        crate::drawing_coords::set_plane(self.construction_plane);
        let Some(built)=draft.submit(point)? else {self.tool_status=draft.prompt();self.snap_index=None;return self.update_edit_preview();};
        self.commit_tool(built)
    }

    fn commit_tool(&mut self,built:crate::tool::Built) -> Result<Change,&'static str> {
        if let crate::tool::Built::Text(point)=built{return self.commit_text(point)}
        if let crate::tool::Built::Measured(points)=built{return self.distance_mark(points)}
        let name=self.draft.as_ref().ok_or("No point tool is running")?.tool.name();
        if let crate::tool::Built::Move(offset)=built {
            let preview=self.edit_preview.as_mut().ok_or("No placement preview is running")?;
            preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?;
            let changed=preview.commit(&mut self.scene,&mut self.history)?;
            self.edit_preview=None;self.draft=None;self.tool_status="Move completed.".into();return Ok(if changed {Change::Scene}else{Change::Preview});
        }
        self.history.try_edit(&mut self.scene,|scene|match built {
            crate::tool::Built::Point(point)=>scene.insert_point(point).map(|_|()),
            crate::tool::Built::Line(line)=>scene.insert_line(line).map(|_|()),
            crate::tool::Built::Polyline(line)=>scene.insert_path(line).map(|_|()),
            crate::tool::Built::Geometry(geometry)=>crate::primitive_geometry::insert(scene,geometry).map(|_|()),
            crate::tool::Built::Text(_)=>unreachable!("Text adopts its original label owner"),
            crate::tool::Built::Measured(_)=>unreachable!("Measurement changes no original geometry"),
            crate::tool::Built::Move(_)=>unreachable!("Move uses its placement preview"),
        })?;
        if name=="Clipping Plane"{if let Some(row)=self.scene.objects().last(){self.select_rows(vec![row.id],false,false);}}
        self.draft=None;self.tool_status=format!("{name} created.");Ok(Change::Scene)
    }

    fn primitive_failure(&mut self,message:String)->&'static str {self.tool_status=message;"Shape input invalid"}
    fn primitive_answer(&mut self,point:session_rust::Point)->Result<Change,&'static str> {
        let draft=self.draft.as_mut().ok_or("No shape tool is running")?;
        let result=draft.primitive.as_mut().ok_or("No shape tool is running")?.accept(crate::primitive_shape::Answer::Point(point.clone()));
        let geometry=result.map_err(|message|self.primitive_failure(message))?;
        let draft=self.draft.as_mut().unwrap();draft.points.push([point[0],point[1],point[2]]);draft.cursor=None;draft.snapped=None;
        if let Some(geometry)=geometry {return self.commit_tool(crate::tool::Built::Geometry(geometry));}
        self.tool_status=draft.prompt();Ok(Change::View)
    }
    fn primitive_text(&mut self,text:&str)->Result<Change,&'static str> {
        let draft=self.draft.as_mut().ok_or("No shape tool is running")?;
        let result=draft.primitive.as_mut().ok_or("No shape tool is running")?.text(text,draft.cursor,self.construction_plane);
        let geometry=result.map_err(|message|self.primitive_failure(message))?;
        let draft=self.draft.as_mut().unwrap();draft.points=draft.primitive.as_ref().unwrap().answers.iter().filter_map(|answer|match answer {crate::primitive_shape::Answer::Point(p)=>Some([p[0],p[1],p[2]]),_=>None}).collect();draft.cursor=None;draft.snapped=None;
        if let Some(geometry)=geometry {return self.commit_tool(crate::tool::Built::Geometry(geometry));}
        self.tool_status=draft.prompt();Ok(Change::View)
    }
    fn apply_transform(&mut self,kind:crate::transform_draft::Kind,delta:&session_rust::Xform,ids:&[ObjectId],target:Option<crate::pick_sources::Target>)->Result<Change,&'static str> {
        if !crate::placement::valid(&delta.m){return Err("Transform must be finite, affine and invertible");}
        if kind==crate::transform_draft::Kind::Copy {
            if target.is_some(){return Err("Copy works on whole objects");}
            self.history.try_edit(&mut self.scene,|scene|crate::layer_edit::copy_objects(scene,ids,delta).map(|_|()))?;return Ok(Change::Scene);
        }
        let mut preview=crate::gizmo_edit::Preview::new(&self.scene,ids.to_vec(),target)?;preview.set(delta.clone())?;let changed=preview.commit(&mut self.scene,&mut self.history)?;Ok(if changed{Change::Scene}else{Change::Preview})
    }
    fn begin_transform(&mut self,kind:crate::transform_draft::Kind)->Result<Change,&'static str> {
        let owners=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());
        let preview=if kind==crate::transform_draft::Kind::Copy {if target.is_some(){return Err("Copy works on whole objects");}let mut candidate=self.scene.clone();crate::layer_edit::copy_objects(&mut candidate,&owners,&session_rust::Xform::identity())?;None}else{Some(crate::gizmo_edit::Preview::new(&self.scene,owners.clone(),target)?)};
        let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
        let mut draft=crate::tool::Draft::new(crate::tool::Shape::Transform(kind));draft.frame=frame;let transform=draft.transform.as_mut().unwrap();transform.frame=frame;transform.owners=owners;transform.target=target;
        self.edit_preview=None;self.transform_preview=preview;self.draft=Some(draft);self.snap_index=None;self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::Preview)
    }
    fn transform_outcome(&mut self,next:crate::transform_draft::Draft,outcome:crate::transform_draft::Outcome)->Result<Change,&'static str> {
        // Validate the entire typed batch against a temporary scene before recording its separate Copy target transactions.
        let mut candidate=self.scene.clone();
        for delta in &outcome.transforms {if next.kind==crate::transform_draft::Kind::Copy {crate::layer_edit::copy_objects(&mut candidate,&next.owners,delta)?;}else{let mut preview=crate::gizmo_edit::Preview::new(&candidate,next.owners.clone(),next.target)?;preview.set(delta.clone())?;let mut history=crate::history::History::default();preview.commit(&mut candidate,&mut history)?;}}
        for delta in &outcome.transforms {self.apply_transform(next.kind,delta,&next.owners,next.target)?;}
        let changed=!outcome.transforms.is_empty();
        if outcome.finished {self.draft=None;self.transform_preview=None;self.tool_status=format!("{} completed.",next.kind.name());return Ok(if changed{Change::Scene}else{Change::Preview});}
        let draft=self.draft.as_mut().ok_or("No transform tool is running")?;draft.points=next.points.clone();draft.transform=Some(next);draft.cursor=None;draft.snapped=None;self.tool_status=draft.prompt();
        if changed {self.snap_index=None;return Ok(Change::Scene);}self.update_edit_preview()
    }
    fn transform_point(&mut self,point:[f64;3])->Result<Change,&'static str> {
        let mut next=self.draft.as_ref().and_then(|draft|draft.transform.clone()).ok_or("No transform tool is running")?;
        let transform=next.accept(crate::primitive_shape::Answer::Point(session_rust::Point::new(point[0],point[1],point[2]))).map_err(|message|self.primitive_failure(message))?;
        let finished=transform.is_some()&&next.kind!=crate::transform_draft::Kind::Copy;self.transform_outcome(next,crate::transform_draft::Outcome{transforms:transform.into_iter().collect(),finished})
    }
    fn transform_text(&mut self,text:&str)->Result<Change,&'static str> {
        let draft=self.draft.as_ref().ok_or("No transform tool is running")?;let mut next=draft.transform.clone().ok_or("No transform tool is running")?;
        let outcome=next.text(text,draft.cursor,self.construction_plane).map_err(|message|self.primitive_failure(message))?;self.transform_outcome(next,outcome)
    }
    pub fn highlighted_objects(&self)->Vec<ObjectId>{if let Some(e)=&self.element{return e.ids()}if let Some(trim)=&self.trim{return if trim.phase==crate::trim_draft::Phase::Parts{Vec::new()}else if trim.phase==crate::trim_draft::Phase::Targets{trim.targets.iter().map(|o|o.id).collect()}else{trim.cutters.iter().map(|o|o.id).collect()}}if let Some(split)=&self.split{return std::iter::once(split.target.id).chain(split.cutters.iter().map(|o|o.id)).collect()}if let Some(extend)=self.draft.as_ref().and_then(|d|d.extend.as_ref()){return extend.boundaries.iter().map(|o|o.id).collect()}self.draft.as_ref().and_then(|d|d.gather.as_ref()).map_or_else(||crate::feature_owner::highlighted(&self.scene,&self.selected_objects()),|g|{g.progress.picked().iter().filter_map(|row|g.curves.iter().find(|(id,_)|id.value()==*row).map(|(id,_)|*id)).collect()})}
    pub fn gathering_curves(&self)->bool{self.draft.as_ref().is_some_and(|d|d.plane_points.is_none()&&d.gather.as_ref().is_some_and(|g|g.picks()))}
    fn gather_adopt(&mut self,next:crate::gather_draft::Draft,made:Option<crate::gather_input::Made>)->Result<Change,&'static str>{
        next.valid(&self.scene).map_err(|message|self.primitive_failure(message))?;
        if let Some(made)=made{let mut ids=Vec::new();self.history.try_edit(&mut self.scene,|scene|{ids=made.geometries.into_iter().map(|geometry|crate::primitive_geometry::insert(scene,geometry)).collect::<Result<Vec<_>,_>>()?;Ok(())})?;self.select_objects(ids,false);self.draft=None;self.tool_status=made.message;return Ok(Change::Scene)}
        let draft=self.draft.as_mut().ok_or("No gathering tool is running")?;draft.points=next.answers.iter().filter_map(|answer|if let crate::gather_input::Answer::Point(p)=answer{Some([p[0],p[1],p[2]])}else{None}).collect();draft.gather=Some(next);draft.cursor=None;draft.snapped=None;self.tool_status=draft.prompt();Ok(Change::View)
    }
    fn begin_gather(&mut self,recipe:&'static crate::gather_input::Recipe,text:&str)->Result<Change,&'static str>{
        crate::drawing_coords::set_plane(self.construction_plane);let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
        let next=crate::gather_draft::Draft::new(recipe,text,&self.scene,self.selected_objects(),frame).map_err(|message|self.primitive_failure(message))?;let made=next.build().map_err(|message|self.primitive_failure(message))?;
        if made.is_some(){return self.gather_adopt(next,made)}
        self.apply(Action::BeginTool(crate::tool::Shape::Gather(recipe)))?;self.select_objects([],false);self.gather_adopt(next,made)
    }
    fn gather_point(&mut self,point:[f64;3])->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.gather.clone()).ok_or("No gathering tool is running")?;next.valid(&self.scene).map_err(|message|self.primitive_failure(message))?;let made=next.point(point).map_err(|message|self.primitive_failure(message))?;self.gather_adopt(next,made)}
    fn gather_text(&mut self,text:&str)->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.gather.clone()).ok_or("No gathering tool is running")?;next.valid(&self.scene).map_err(|message|self.primitive_failure(message))?;let made=next.text(text,self.construction_plane).map_err(|message|self.primitive_failure(message))?;self.gather_adopt(next,made)}
    fn gather_pick(&mut self,id:Option<ObjectId>)->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.gather.clone()).ok_or("No gathering tool is running")?;next.valid(&self.scene).map_err(|message|self.primitive_failure(message))?;let made=next.pick(&self.scene,id).map_err(|message|self.primitive_failure(message))?;self.gather_adopt(next,made)}
    pub fn editing_boundaries(&self)->bool{self.draft.as_ref().is_some_and(|d|d.plane_points.is_none()&&d.extend.as_ref().is_some_and(|e|!e.curves))}
    fn commit_project(&mut self,project:crate::project_draft::Draft,plane:session_rust::Plane)->Result<Change,&'static str>{let(edits,message)=project.prepare(&self.scene,&plane).map_err(|message|self.primitive_failure(message))?;self.history.try_edit(&mut self.scene,|scene|{for(id,geometry)in edits{scene.replace_original(id,geometry)?}Ok(())})?;self.select_objects(project.before,false);self.draft=None;self.tool_status=message;Ok(Change::Scene)}
    fn begin_project(&mut self,words:&[String])->Result<Change,&'static str>{
        crate::drawing_coords::set_plane(self.construction_plane);let plan=crate::project_geometry::parse("Project To Plane",&words.iter().map(String::as_str).collect::<Vec<_>>()).map_err(|message|self.primitive_failure(message))?;let mut project=crate::project_draft::Draft::new(&self.scene,self.selected_objects())?;
        if plan.plane=="3Point"{let plane=project.accept(&plan.points).map_err(|message|self.primitive_failure(message))?;if let Some(plane)=plane{return self.commit_project(project,plane)}self.apply(Action::BeginTool(crate::tool::Shape::Project))?;self.draft.as_mut().unwrap().project=Some(project);self.tool_status=self.draft.as_ref().unwrap().prompt();return Ok(Change::View)}
        let frame=match plan.plane{"XY"=>crate::cplane::CPlane::Xy,"YZ"=>crate::cplane::CPlane::Yz,"ZX"=>crate::cplane::CPlane::Xz,_=>crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis()))};let(x,y)=frame.axes();self.commit_project(project,session_rust::Plane::from_point_normal(session_rust::Point::new(0.,0.,0.),x.cross(&y),None))
    }
    fn project_points(&mut self,added:&[[f64;3]])->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.project.clone()).ok_or("No projection tool is running")?;let plane=next.accept(added).map_err(|message|self.primitive_failure(message))?;if let Some(plane)=plane{return self.commit_project(next,plane)}let draft=self.draft.as_mut().unwrap();draft.points=next.points.clone();draft.project=Some(next);draft.cursor=None;draft.snapped=None;self.tool_status=draft.prompt();Ok(Change::View)}
    fn begin_extend(&mut self,distance:Option<f64>,asking:bool)->Result<Change,&'static str>{let next=crate::extend_draft::Draft::new(&self.scene,self.selected_objects(),distance,asking).map_err(|message|self.primitive_failure(message))?;self.apply(Action::BeginTool(crate::tool::Shape::Extend))?;self.select_objects([],false);self.draft.as_mut().unwrap().extend=Some(next);self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn extend_pick(&mut self,id:Option<ObjectId>)->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.extend.clone()).ok_or("No extension tool is running")?;next.pick(&self.scene,id).map_err(|message|self.primitive_failure(message))?;self.draft.as_mut().unwrap().extend=Some(next);self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn extend_text(&mut self,text:&str)->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.extend.clone()).ok_or("No extension tool is running")?;let finished=next.text(&self.scene,text).map_err(|message|self.primitive_failure(message))?;if finished{self.select_objects(next.before,false);self.draft=None;self.tool_status="Extend done".into();return Ok(Change::View)}self.draft.as_mut().unwrap().extend=Some(next);self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn extend_screen(&mut self,screen:[f32;2],size:[f64;2],commit:bool)->Result<Change,&'static str>{
        let mut next=self.draft.as_ref().and_then(|d|d.extend.clone()).ok_or("No extension tool is running")?;let view=crate::snap_math::Screen{matrix:self.camera.view_projection().m,size};next.follow(&self.scene,&view,[(screen[0]as f64+1.)*size[0]/2.,(1.-screen[1]as f64)*size[1]/2.]).map_err(|message|self.primitive_failure(message))?;
        if commit{let hover=next.hover.clone().ok_or_else(||self.primitive_failure(if next.error.is_empty(){"Click near a curve end".into()}else{next.error.clone()}))?;hover.owner.valid(&self.scene)?;self.history.try_edit(&mut self.scene,|scene|scene.replace_original(hover.owner.id,hover.geometry))?;let added=hover.tail.windows(2).map(|p|(0..3).map(|i|(p[1][i]-p[0][i]).powi(2)).sum::<f64>().sqrt()).sum::<f64>();next.refreshed(&self.scene,hover.owner.id).map_err(|message|self.primitive_failure(message))?;self.tool_status=format!("Extended by {added:.3} · Undo reverts it");self.draft.as_mut().unwrap().extend=Some(next);return Ok(Change::Scene)}self.draft.as_mut().unwrap().extend=Some(next);self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)
    }
    fn edit_interval(&mut self,interval:crate::interval_geometry::Interval)->Result<Change,&'static str>{let ids=self.selected_objects();let[id]=ids.as_slice()else{return Err("Select one original curve")};let owner=crate::source_edit::Owner::capture(&self.scene,*id)?;let geometry=crate::interval_geometry::edited(&owner.source,interval).map_err(|message|self.primitive_failure(message))?;owner.valid(&self.scene)?;self.history.try_edit(&mut self.scene,|scene|scene.replace_original(*id,geometry))?;self.draft=None;self.tool_status="Geometry updated".into();Ok(Change::Scene)}
    fn split_command(&mut self)->Result<Change,&'static str>{
        if self.split.as_ref().is_some_and(|s|s.cutters.is_empty()){return Err("Select at least one cutter curve, then press Enter")}
        if let Some(next)=self.split.take(){let(pieces,regions)=next.prepare(&self.scene).map_err(|message|self.primitive_failure(message))?;if regions<2{self.tool_status="No division: cutters must cross the curve or lie on the selected face".into();return Ok(Change::View)}self.history.try_edit(&mut self.scene,|scene|crate::split_draft::adopt(scene,next.target.id,pieces))?;self.select_objects([next.target.id],false);self.tool_status=format!("Split into {regions} regions · cutters retained · Undo restores the original");return Ok(Change::Scene)}
        let id=self.selection().map(|t|t.owner).or(self.selected).ok_or("Select a curve or one surface face, then run Split")?;
        let face=match self.selected_part{Some(crate::pick_sources::Part::Face(face))=>Some(face as usize),_=>None};
        let next=crate::split_draft::Draft::new(&self.scene,id,face).map_err(|message|self.primitive_failure(message))?;
        self.draft=None;self.gizmo_edit=None;self.edit_preview=None;self.transform_preview=None;self.tool_status=next.prompt();self.split=Some(next);Ok(Change::View)
    }
    fn split_pick(&mut self,id:Option<ObjectId>)->Result<Change,&'static str>{let mut next=self.split.clone().ok_or("Start Split first")?;next.pick(&self.scene,id).map_err(|message|self.primitive_failure(message))?;self.tool_status=next.prompt();self.split=Some(next);Ok(Change::View)}
    fn measured_ids(&self)->Vec<ObjectId>{if self.selected_part.is_some(){self.selected.into_iter().filter(|id|self.scene.contains(*id)).collect()}else{self.selected_objects()}}
    fn measure_selection(&mut self,kind:crate::measure_mark::Kind)->Result<Change,&'static str>{let face=match(self.selected,self.selected_part){(Some(id),Some(crate::pick_sources::Part::Face(face)))=>Some((id,face)),_=>None};let(mark,message)=crate::measure_mark::selection(&self.scene,&self.measured_ids(),face,kind,self.unit).map_err(|message|self.primitive_failure(message))?;self.mark=Some(mark);self.tool_status=message;Ok(Change::View)}
    fn distance_mark(&mut self,points:[[f64;3];2])->Result<Change,&'static str>{let(mark,message)=crate::measure_mark::distance(points[0],points[1],self.unit);self.mark=Some(mark);self.tool_status=message;self.draft=None;Ok(Change::View)}
    fn begin_measure_distance(&mut self,text:&str)->Result<Change,&'static str>{crate::drawing_coords::set_plane(self.construction_plane);let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));let points=if text.trim().is_empty(){Vec::new()}else{crate::drawing_input::points(text,self.construction_plane,frame,&[],None)?};if points.len()>2{return Err("Measure Distance takes two points")};self.apply(Action::BeginTool(crate::tool::Shape::Measure))?;self.snapping.objects=true;if !points.is_empty(){let built=self.draft.as_mut().unwrap().submit_many(&points)?;if let Some(crate::tool::Built::Measured(points))=built{return self.distance_mark(points)}}self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn bounding_box(&mut self)->Result<Change,&'static str>{let ids=self.measured_ids();for id in &ids{if self.scene.objects().iter().any(|r|r.id==*id&&r.geometry().is_none()){return Err("Reload original sources before BoundingBox")}}let bounds=crate::measure_mark::selected_bounds(&self.scene,&ids).map_err(|message|self.primitive_failure(message))?;let mut original=crate::bounds_original::box_geometry(&bounds);original.name="bounding box".into();let mut id=None;self.history.try_edit(&mut self.scene,|scene|{id=Some(crate::bounds_original::insert(scene,std::rc::Rc::new(original))?);Ok(())})?;self.select_objects(id,false);self.tool_status="Created a world axis aligned bounding box".into();Ok(Change::Scene)}
    pub fn guide_overlay(&self)->crate::guide_gpu::Overlay{
        if let Some(trim)=&self.trim{return trim.overlay()}
        let mut overlay=crate::guide_gpu::Overlay::default();let red=[210./255.,40./255.,40./255.,1.];
        let Some(extend)=self.draft.as_ref().and_then(|d|d.extend.as_ref())else{return overlay};
        let label=if let Some(hover)=&extend.hover{
            overlay.wire(&hover.tail,red,true);hover.tail.last().map(|point|{let added=hover.tail.windows(2).map(|p|(p[1][0]-p[0][0]).hypot(p[1][1]-p[0][1]).hypot(p[1][2]-p[0][2])).sum::<f64>();(*point,format!("+{added:.3}"))})
        }else if !extend.error.is_empty(){extend.last.and_then(|(id,end)|{
            let owner=&extend.candidates.iter().find(|c|c.0.id==id)?.0;let (tip,_)=crate::extend_reach::tip(&owner.source,end)?;let p=tip.transformed(&owner.model);Some(([p[0],p[1],p[2]],extend.error.clone()))
        })}else{None};
        if let Some((point,word))=label{overlay.marks.push(crate::marker::Marker{center:point.map(|v|v as f32),diameter:6.,colour:red});overlay.label=Some((point,word));}overlay
    }

    pub fn trimming_picks(&self)->bool{self.trim.as_ref().is_some_and(|t|t.picks())}
    pub fn trimming_parts(&self)->bool{self.trim.as_ref().is_some_and(|t|!t.picks())}
    fn trim_command(&mut self)->Result<Change,&'static str>{
        if self.trim.is_none(){if self.selected_part.is_some(){return Err("Trim starts from whole original objects")}
            let next=crate::trim_draft::Draft::new(&self.scene,self.selected_objects()).map_err(|message|self.primitive_failure(message))?;self.apply(Action::CancelTool)?;self.select_objects([],false);self.tool_status=next.prompt();self.trim=Some(next);return Ok(Change::View)}
        if self.trimming_parts(){return self.commit_trim()}
        let mut trim=self.trim.take().unwrap();let result=trim.prepare(&self.scene,&self.camera);self.tool_status=trim.prompt();self.trim=Some(trim);result.map_err(|message|self.primitive_failure(message))?;Ok(Change::View)
    }
    fn commit_trim(&mut self)->Result<Change,&'static str>{
        let trim=self.trim.take().ok_or("Start Trim first")?;let before=trim.before.clone();if !trim.changed(){self.select_objects(before,false);self.tool_status="Trim cancelled · nothing changed".into();return Ok(Change::Preview)}
        let edits=trim.prepared_edits(&self.scene).map_err(|message|{self.select_objects(before.clone(),false);self.primitive_failure(message)})?;
        let result=self.history.try_edit(&mut self.scene,|scene|{for(id,pieces)in edits{crate::split_draft::adopt(scene,id,pieces)?}Ok(())});self.select_objects([],false);self.snap_index=None;result?;self.tool_status="Trimmed originals · one Undo restores them".into();Ok(Change::Scene)
    }
    fn trim_screen(&mut self,screen:[f32;2],size:[f64;2],commit:bool)->Result<Change,&'static str>{
        let mut trim=self.trim.take().ok_or("Start Trim first")?;let hit=trim.part_at(&self.camera,screen,size,commit);
        if !commit{trim.hover=hit;self.trim=Some(trim);return Ok(Change::View)}
        let result=hit.ok_or_else(||"Click a remaining trim part".to_owned()).and_then(|(cut,part)|trim.remove(cut,part));
        let finish=match result{Ok(finish)=>finish,Err(message)=>{self.trim=Some(trim);return Err(self.primitive_failure(message))}};
        if !finish{if let Err(message)=trim.refresh_preview(&self.scene){self.trim=Some(trim);return Err(self.primitive_failure(message))}}
        self.trim=Some(trim);if finish{self.commit_trim()}else{Ok(Change::Preview)}
    }
    fn explode_selected(&mut self)->Result<Change,&'static str>{if self.selected_part.is_some(){return Err("Explode works on whole objects")}
        let plan=crate::explode_edit::Plan::prepare(&self.scene,&self.selected_objects()).map_err(|message|self.primitive_failure(message))?;let mut made=None;self.history.try_edit(&mut self.scene,|scene|{made=Some(plan.adopt(scene)?);Ok(())})?;self.select_objects(made.unwrap(),false);self.snap_index=None;Ok(Change::Scene)
    }
    fn curve_heads(&mut self,head:session_rust::Arrowhead)->Result<Change,&'static str>{
        let mut edits=Vec::new();let mut curves=0;
        for id in self.measured_ids(){let owner=crate::source_edit::Owner::capture(&self.scene,id)?;if let Some((next,changed))=crate::curve_heads::headed(&owner.source,head){curves+=1;if changed{edits.push((id,next))}}}
        if curves==0{return Err("Arrowhead needs a selected line, polyline or curve")}if edits.is_empty(){return Ok(Change::View)}
        self.history.try_edit(&mut self.scene,|scene|{for(id,source)in edits{scene.replace_original(id,source)?;}Ok(())})?;self.snap_index=None;Ok(Change::Scene)
    }
    fn begin_text(&mut self,words:String)->Result<Change,&'static str>{let text=crate::text_draft::Draft::new(words,self.camera.distance,self.last_text_height)?;self.apply(Action::BeginTool(crate::tool::Shape::Text))?;self.draft.as_mut().unwrap().text=Some(text);self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn commit_text(&mut self,point:[f64;3])->Result<Change,&'static str>{let draft=self.draft.as_ref().ok_or("Start Text first")?;let text=draft.text.as_ref().ok_or("Start Text first")?;let prepared=crate::scene_label::Prepared::new(std::rc::Rc::new(text.source(point,draft.frame,&self.camera)))?;let height=text.height;self.history.try_edit(&mut self.scene,|scene|scene.insert_text(prepared).map(|_|()))?;self.last_text_height=height;self.draft=None;self.tool_status="Created original scene text. Undo removes it.".into();Ok(Change::Scene)}
    fn clipping_begin(&mut self,mode:crate::clipping_geometry::Mode)->Result<Change,&'static str>{
        if crate::clipping_rows::owners(&self.scene).len()>=6{return Err("At most six clipping planes")}
        self.apply(Action::CancelTool)?;let mut draft=crate::tool::Draft::new(crate::tool::Shape::Line);draft.frame=self.drawing_plane();draft.tool=Box::new(crate::clipping_tool::Clipping{mode,half:crate::clipping_rows::half_size(&self.scene)});self.tool_status=draft.prompt();self.draft=Some(draft);self.snap_index=None;Ok(Change::View)
    }
    fn clipping_request(&mut self,request:crate::clipping_input::Request)->Result<Change,&'static str>{
        use crate::clipping_input::Request;
        match request{
            Request::Begin(mode)=>self.clipping_begin(mode),
            Request::Create(mode,points)=>{let tool=crate::clipping_tool::Clipping{mode,half:crate::clipping_rows::half_size(&self.scene)};let built=crate::tool::Tool::build(&tool,&points)?;self.clipping_begin(mode)?;self.commit_tool(built)},
            Request::Switch(on)=>{self.clipping_enabled=on;self.tool_status=format!("Clipping Plane {}",if on{"On"}else{"Off"});Ok(Change::View)},
            Request::Fill(solid)=>{self.clipping_solid=solid.unwrap_or(!self.clipping_solid);self.tool_status=format!("Clipping Plane Fill {}",if self.clipping_solid{"Solid"}else{"Hatch"});Ok(Change::View)},
            Request::Flip=>{let ids=self.selected_objects();let owners=crate::clipping_rows::owners(&self.scene);let ids=ids.into_iter().filter(|id|owners.contains(id)).collect::<Vec<_>>();if ids.is_empty(){return Err("Select a clipping plane to flip")}
                self.history.try_edit(&mut self.scene,|scene|{for id in &ids{let plane=scene.objects().iter().find(|r|r.id==*id).and_then(|r|r.original.as_ref());let Some(session_rust::Geometry::Plane(plane))=plane else{return Err("Missing original clipping plane")};let flipped=crate::clipping_geometry::flipped(plane);scene.replace_original(*id,session_rust::Geometry::Plane(std::rc::Rc::new(flipped)))?;}Ok(())})?;self.tool_status="Flipped clipping planes. Undo restores their original sides.".into();Ok(Change::Scene)}
        }
    }
    fn tool_text(&mut self,text:&str) -> Result<Change,&'static str> {
        if let Some(label)=self.draft.as_mut().filter(|d|d.plane_points.is_none()).and_then(|d|d.text.as_mut()){if label.asking||text.split_whitespace().next().is_some_and(|w|w.eq_ignore_ascii_case("Height")){label.height(text)?;self.tool_status=label.prompt();return Ok(Change::View)}}
        if self.draft.as_ref().is_some_and(|d|d.extend.is_some()&&d.plane_points.is_none()){return self.extend_text(text)}
        if self.draft.as_ref().is_some_and(|d|d.gather.is_some()&&d.plane_points.is_none()){return self.gather_text(text);}
        if self.draft.as_ref().is_some_and(|d|d.transform.is_some()&&d.plane_points.is_none()){return self.transform_text(text);}

        if self.draft.as_ref().is_some_and(|d|d.primitive.is_some() && d.plane_points.is_none()) {return self.primitive_text(text);}
        let draft=self.draft.as_ref().ok_or("No point tool is running")?;
        let points=draft.plane_points.as_deref().unwrap_or(&draft.points);
        let added=crate::drawing_input::points(text,self.construction_plane,draft.frame,points,draft.cursor)?;
        if draft.plane_points.is_some() {
            if added.len()!=1 {return Err("Supply one construction-plane point at a time");}return self.place_tool_point(added[0]);
        }
        if self.draft.as_ref().is_some_and(|d|d.project.is_some()){return self.project_points(&added)}
        let draft=self.draft.as_mut().unwrap();
        let Some(built)=draft.submit_many(&added)? else {self.tool_status=draft.prompt();self.snap_index=None;return self.update_edit_preview();};
        self.commit_tool(built)
    }

    pub fn snap_point(&mut self, screen: [f32; 2], size: [f64; 2]) -> Option<crate::snap::Hit> {
        if let Some(primitive)=self.draft.as_ref()?.primitive.as_ref() {if primitive.next().is_some_and(|ask|ask.kind==crate::primitive_shape::Kind::Height) {return primitive.on_axis(&self.camera,screen).map(|point|crate::snap::Hit {point,kind:crate::snap::Kind::Near,owner:None});}}
        if let Some(gather)=self.draft.as_ref()?.gather.as_ref(){if self.draft.as_ref()?.plane_points.is_none(){match gather.current(){Some(crate::gather_input::Step::Curves{..}|crate::gather_input::Step::Number{..})=>return None,Some(crate::gather_input::Step::Distance(_))=>return gather.on_axis(&self.camera,screen).map(|point|crate::snap::Hit{point,kind:crate::snap::Kind::Near,owner:None}),_=>{}}}}
        let ray = self.camera.ray(screen)?;
        let from = self.draft.as_ref()?.points.last().copied();
        let hit = if self.snapping.objects {
            let exclude=self.edit_preview.as_ref().filter(|_| from.is_some()).map_or_else(Vec::new,|p| p.ids().to_vec());
            let index = self.snap_index.get_or_insert_with(|| crate::snap_index::Index::new(&self.scene).excluding(&exclude));
            let viewport = crate::snap_math::Screen { matrix: self.camera.view_projection().m, size };
            index.query(viewport, &ray, [(screen[0] as f64+1.)*size[0]/2., (1.-screen[1] as f64)*size[1]/2.], self.snapping.modes, from)
        } else { None };
        let own=self.draft.as_ref().filter(|d|d.primitive.is_none() && d.transform.is_none() && d.gather.is_none() && d.project.is_none() && d.extend.is_none() && d.tool.count()==0 && d.points.len()>=2 && self.snapping.objects && self.snapping.modes&crate::snap::END!=0).and_then(|d|{
            let point=*d.points.first()?;let viewport=crate::snap_math::Screen {matrix:self.camera.view_projection().m,size};let p=viewport.point(point)?;
            let cursor=[(screen[0] as f64+1.)*size[0]/2.,(1.-screen[1] as f64)*size[1]/2.];
            ((p[0]-cursor[0]).hypot(p[1]-cursor[1])<=12.).then_some(crate::snap::Hit {point,kind:crate::snap::Kind::End,owner:None})
        });
        own.or(hit).or_else(|| {
            let draft=self.draft.as_ref()?;
            let frame=self.construction_plane.unwrap_or(draft.frame);
            let through=if self.construction_plane.is_none() {draft.points.last().copied()}else{None};
            let point=crate::drawing_input::on_at(&self.camera,screen,frame,through)?;
            let (x,y)=frame.axes();let o=frame.origin();
            let point=if self.snapping.grid {crate::snap_math::grid(point,[o[0],o[1],o[2]],[[x[0],x[1],x[2]],[y[0],y[1],y[2]]],self.snapping.step)?}else{point};
            Some(crate::snap::Hit {point,kind:if self.snapping.grid {crate::snap::Kind::Grid}else{crate::snap::Kind::Near},owner:None})
        })
    }

    pub fn snap_usage(&self) -> [u64; 2] { self.snap_index.as_ref().map_or([0,0], |index| [index.builds,index.visits as u64]) }

    fn element_commit(&mut self,next:crate::element_draft::Draft)->Result<Change,&'static str>{
        let made=next.build(&self.scene).map_err(|m|self.primitive_failure(m))?;let consumed=next.ids();let mut ids=Vec::new();let show=self.element_attributes;let interactions=self.element_interactions;self.history.try_edit(&mut self.scene,|scene|{ids=crate::element_adopt::adopt(scene,made,&consumed)?;let features=crate::element_adopt::attribute_ids(scene);std::rc::Rc::make_mut(&mut scene.view).hide(&features,!show);let ids=crate::element_adopt::interaction_ids(scene);std::rc::Rc::make_mut(&mut scene.view).hide(&ids,!interactions);Ok(())})?;self.element=None;self.select_objects(ids,false);self.snap_index=None;self.tool_status=format!("Created {} from its sources · Undo restores them",next.kind.name());Ok(Change::Scene)
    }
    fn element_begin(&mut self,kind:crate::element_input::Kind,text:&str)->Result<Change,&'static str>{
        let(next,typed)=crate::element_draft::Draft::new(kind,text,&self.scene,self.selected_objects()).map_err(|m|self.primitive_failure(m))?;
        self.apply(Action::CancelTool)?;if typed&&!next.picked.is_empty(){return self.element_commit(next)}self.select_objects([],false);self.tool_status=next.prompt();self.element=Some(next);Ok(Change::View)
    }
    fn element_text(&mut self,text:&str)->Result<Change,&'static str>{
        let mut next=self.element.clone().ok_or("Start an Element tool first")?;next.valid(&self.scene).map_err(|m|self.primitive_failure(m))?;let finish=next.text(text).map_err(|m|self.primitive_failure(m))?;if finish{return self.element_commit(next)}self.tool_status=next.prompt();self.element=Some(next);Ok(Change::View)
    }
    fn element_pick(&mut self,id:Option<ObjectId>)->Result<Change,&'static str>{
        let mut next=self.element.clone().ok_or("Start an Element tool first")?;next.valid(&self.scene).map_err(|m|self.primitive_failure(m))?;next.pick(&self.scene,id).map_err(|m|self.primitive_failure(m))?;self.tool_status=next.prompt();self.element=Some(next);Ok(Change::View)
    }
    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        let action=match action{Action::Touch(actions)=>{let mut change=Change::View;for action in actions{let next=self.apply(action)?;if next!=Change::View{change=next;}}return Ok(change)},action=>action};
        if let Action::Opacity(value)=&action{if !value.is_finite()||!(0.0..=1.0).contains(value){return Err("Opacity takes a value from0 to1")}self.opacity=*value;self.tool_status=format!("Opacity {value}");return Ok(Change::View)}
        if let Action::ViewOutline(on)=&action{self.outlines=on.unwrap_or(!self.outlines);self.tool_status=format!("View Outline {}",if self.outlines{"On"}else{"Off"});return Ok(Change::View)}
        if let Action::Arctic(on)=&action{self.arctic=on.unwrap_or(!self.arctic);if self.arctic{self.outlines=true;}self.tool_status=format!("Arctic {}",if self.arctic{"On"}else{"Off"});return Ok(Change::View)}
        if !matches!(&action,Action::Zoom(_)|Action::ZoomAt{..}|Action::PanScreen{..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::NamedView(_)|Action::Isometric|Action::ResetView|Action::Background|Action::NormalView(_)|Action::Grid(_)|Action::CanvasCancel|Action::ToolScreen{commit:false,..}|Action::TrimScreen{commit:false,..}|Action::ToolHover(_)){self.interaction_epoch=self.interaction_epoch.wrapping_add(1);}
        if matches!(&action,Action::Escape){return if self.lasso.is_some(){self.apply(Action::LassoCancel)}else if self.trim.is_some(){self.apply(Action::CancelTrim)}else if self.split.is_some(){self.apply(Action::CancelSplit)}else if self.gizmo_edit.is_some(){self.apply(Action::GizmoCancel)}else if self.draft.is_some()||self.plane_menu{self.apply(Action::CancelTool)}else if self.pick_mode==crate::pick_sources::Mode::Control{self.apply(Action::Controls(false))}else{self.apply(Action::Picked(None))}}
        if !matches!(&action,Action::Zoom(_)|Action::ZoomAt{..}|Action::PanScreen{..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::NamedView(_)|Action::Isometric|Action::ResetView|Action::Background|Action::NormalView(_)|Action::Grid(_)|Action::CanvasCancel){self.mark=None;}
        match action {
            Action::LassoPress{at,size,density,pointer}=>{if size.iter().any(|v|!v.is_finite()||*v<=0.)||!density.is_finite()||density<=0.||size.iter().any(|v|v*density>32768.)||at.iter().any(|v|!v.is_finite()){return Err("Invalid lasso viewport")}let lasso=self.lasso.as_mut().ok_or("Start Select Lasso first")?;lasso.abandoned();lasso.size=size;lasso.density=density;lasso.pointer=Some(pointer);lasso.spacing=2.;lasso.points.push((at[0]*density,at[1]*density));return Ok(Change::View)},
            Action::LassoMove{at,pointer}=>{if let Some(lasso)=self.lasso.as_mut().filter(|l|l.pointer==Some(pointer)){if at.iter().all(|v|v.is_finite()){let at=[at[0].clamp(0.,lasso.size[0]),at[1].clamp(0.,lasso.size[1])];lasso.follow((at[0]*lasso.density,at[1]*lasso.density));}}return Ok(Change::View)},
            Action::LassoRelease{at,pointer,add,remove}=>{self.apply(Action::LassoMove{at,pointer})?;let Some(lasso)=self.lasso.as_mut().filter(|l|l.pointer==Some(pointer))else{return Ok(Change::View)};lasso.pointer=None;if lasso.points.len()<3{lasso.points.clear();return Ok(Change::View)}let ids=lasso.found(&self.scene,&self.camera);lasso.points.clear();let ids=ids?;self.select_rows(ids,add&&!remove,remove);self.lasso=None;self.tool_status=format!("{} selected inside the lasso",self.selected_objects().len());return Ok(Change::View)},
            Action::StreamRefresh=>{self.snap_index=None;return Ok(Change::Scene);},
            Action::CanvasStart {from,to,size,modified,pointer}=>{self.canvas_drag=crate::canvas_drag::Drag::new(from,to,size,modified);if let Some(drag)=self.canvas_drag.as_mut() {drag.pointer=pointer;}return Ok(Change::View);},
            Action::CanvasMove {screen,size}=>{if let Some(drag)=self.canvas_drag.as_mut() {if drag.released.is_none()&&drag.size==size&&screen.iter().all(|v|v.is_finite()) {drag.to=screen;drag.update(&self.camera)?;}if drag.preview.is_some() {return Ok(Change::Preview);}}return Ok(Change::View);},
            Action::CanvasRelease {screen,size,add,remove}=>{if let Some(drag)=self.canvas_drag.as_mut() {if drag.size!=size {self.canvas_drag=None;return Ok(Change::Preview);}drag.to=screen;drag.released=Some((add,remove));drag.update(&self.camera)?;if drag.preview.is_some()||drag.missed {return self.finish_canvas();}}return Ok(Change::View);},
            Action::CanvasHit(hit)=>{
                let Some(drag)=self.canvas_drag.as_ref() else {return Ok(Change::View);};let ids=hit.filter(|id|!drag.modified&&self.scene.pick_id(id.value()).is_some());
                if let Some(id)=ids {let mut rows=self.selected_objects();if !rows.contains(&id) {self.select_objects([id],false);rows=vec![id];}if let Err(error)=self.canvas_drag.as_mut().unwrap().grab(&self.scene,rows,&self.camera) {self.canvas_drag=None;return Err(error);}}
                else {self.canvas_drag.as_mut().unwrap().missed=true;}
                if self.canvas_drag.as_ref().unwrap().released.is_some() {return self.finish_canvas();}return Ok(Change::Preview);
            },
            Action::CanvasCancel=>{let preview=self.canvas_drag.take().is_some_and(|d|d.preview.is_some());return Ok(if preview {Change::Preview} else {Change::View});},
            Action::SelectRows {ids,add,remove}=>{if self.trimming_picks(){for id in ids{self.apply_action(Action::TrimPick(Some(id)))?;}return Ok(Change::View)}if self.trimming_parts(){return Err("Trim now needs a part click")}if self.split.is_some(){for id in ids{self.split_pick(Some(id))?;}return Ok(Change::View)}self.select_rows(ids,add,remove);return Ok(Change::View);},
            _=>{}
        }
        self.canvas_drag=None;
        let keep_gizmo=matches!(&action,Action::GizmoPress {..}|Action::GizmoTouch {..}|Action::GizmoMove {..}|Action::GizmoRelease {..}|Action::GizmoCancel|Action::GizmoNumber(_)|Action::Zoom(_)|Action::ZoomAt {..}|Action::PanScreen {..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::NamedView(_)|Action::Isometric|Action::ResetView|Action::Background|Action::NormalView(_)|Action::Grid(_)|Action::Snap(_));
        let keep_edit=matches!(&action, Action::GizmoPress {..} | Action::GizmoTouch {..} | Action::GizmoMove {..} | Action::GizmoRelease {..} | Action::GizmoCancel | Action::GizmoNumber(_) | Action::BeginTool(_) | Action::Draw {..} | Action::ToolText(_) | Action::FinishTool | Action::CloseTool | Action::ConstructionPlane(_) | Action::PlaneMenu | Action::Plane3 | Action::ToolSides(_) | Action::BeginTransformation(_) | Action::Transformation {..} | Action::PrimitiveBegin {..} | Action::GatherBegin{..} | Action::GatherPick(_) | Action::Project(_) | Action::ExtendBegin{..} | Action::ExtendPick(_) | Action::Interval(_) | Action::ToolPoint(_) | Action::ToolScreen {..} | Action::ToolHover(_) | Action::CancelTool | Action::Snap(_) | Action::Zoom(_) | Action::ZoomAt {..} | Action::PanScreen {..} | Action::Pan(_,_) | Action::Orbit(_,_) | Action::Fit | Action::FitSelected | Action::Projection(_) | Action::NamedView(_) | Action::Isometric | Action::ResetView | Action::Background | Action::NormalView(_) | Action::Grid(_));
        let mut change=self.apply_action(action)?;
        self.gizmo_hover=None;
        if !keep_gizmo && self.gizmo_edit.take().is_some() && change==Change::View {change=Change::Preview;}
        if !keep_edit && self.draft.as_ref().is_some_and(|d|d.project.is_some()||d.extend.is_some()){self.apply_action(Action::CancelTool)?;}
        if !keep_edit && self.draft.as_ref().is_some_and(|d|d.gather.is_some()){self.apply_action(Action::CancelTool)?;}
        if !keep_edit && self.draft.as_ref().is_some_and(|d|d.transform.is_some()){self.transform_preview=None;self.draft=None;if change==Change::View{change=Change::Preview;}}
        if !keep_edit && self.edit_preview.take().is_some() { self.draft=None; if change==Change::View { change=Change::Preview; } }
        if change==Change::Scene { self.snap_index=None; }
        Ok(change)
    }

    fn apply_action(&mut self, action: Action) -> Result<Change, &'static str> {
        if self.lasso.is_some()&&!matches!(&action,Action::SelectLasso|Action::LassoCancel|Action::Zoom(_)|Action::ZoomAt{..}|Action::PanScreen{..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::NamedView(_)|Action::Isometric|Action::Projection(_)|Action::Fit|Action::FitSelected|Action::ResetView|Action::Grid(_)|Action::ConstructionPlane(_)){self.lasso=None;}
        if self.trim.is_some()&&!matches!(&action,Action::Trim|Action::TrimPick(_)|Action::TrimScreen{..}|Action::CancelTrim|Action::TreePick{..}|Action::ResetView|Action::Zoom(_)|Action::ZoomAt{..}|Action::PanScreen{..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::NamedView(_)|Action::Isometric|Action::Projection(_)|Action::Fit|Action::FitSelected|Action::Grid(_)|Action::NormalView(_)|Action::Background|Action::ConstructionPlane(_)|Action::PlaneMenu|Action::Plane3){let trim=self.trim.take().unwrap();self.select_objects(trim.before,false);}

        if matches!(&action, Action::Close | Action::Undo | Action::Redo | Action::Adopt {..} | Action::Live(_) | Action::Import(_) | Action::Replace(_) | Action::ImportAt(_, _) | Action::ReplaceAt(_, _)) { self.draft = None;self.plane_menu=false;self.plane_only=false; }
        if self.split.is_some()&&!matches!(&action,Action::Split|Action::SplitPick(_)|Action::CancelSplit|Action::TreePick{..}|Action::Zoom(_)|Action::ZoomAt{..}|Action::PanScreen{..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::NamedView(_)|Action::Isometric|Action::ResetView|Action::Background|Action::NormalView(_)|Action::Grid(_)|Action::ConstructionPlane(_)|Action::PlaneMenu|Action::Plane3){self.split=None;}
        match action {
            Action::ElementBegin{kind,text}=>return self.element_begin(kind,&text),
            Action::ElementText(text)=>return self.element_text(&text),
            Action::ElementPick(id)=>return self.element_pick(id),
            Action::ElementAttributes(on)|Action::ElementInteractions(on)=>{
                let (shown,ids,label)=if matches!(action,Action::ElementInteractions(_)) {
                    (&mut self.element_interactions,crate::element_adopt::interaction_ids(&self.scene),"Interactions")
                }else{(&mut self.element_attributes,crate::element_adopt::attribute_ids(&self.scene),"Attributes")};
                *shown=on.unwrap_or(!*shown);
                std::rc::Rc::make_mut(&mut self.scene.view).hide(&ids,!*shown);
                self.tool_status=format!("Element {label} {}",if *shown{"On"}else{"Off"});
                return Ok(Change::Scene)
            },
            Action::Opacity(_)=>unreachable!("Opacity presentation returned before document mutation"),
            Action::ViewOutline(_)=>unreachable!("Outline presentation returned before editor mutation"),
            Action::Arctic(_)=>unreachable!("Arctic presentation returned before editor mutation"),
            Action::Clipping(request)=>{return self.clipping_request(request)},
            Action::SelectLasso=>{self.lasso=Some(Default::default());self.tool_status="Select Lasso · drag a loop · Shift adds · Ctrl removes · Esc cancels".into();return Ok(Change::View)},
            Action::LassoCancel=>{self.lasso=None;self.tool_status="Select Lasso cancelled; selection unchanged".into();return Ok(Change::View)},
            Action::LassoPress{..}|Action::LassoMove{..}|Action::LassoRelease{..}=>unreachable!(),
            Action::AddGroup(name)=>{let ids=self.selected_objects();let mut label=String::new();self.history.try_edit(&mut self.scene,|scene|{label=crate::layer_edit::group(scene,&ids,&name)?;Ok(())})?;self.tool_status=format!("Grouped {} objects as {label}; a click selects all; Undo removes the group",ids.len());return Ok(Change::Scene)},
            Action::AddEdge=>{let ids=self.selected_objects();let mut message=String::new();self.history.try_edit(&mut self.scene,|scene|{message=crate::graph_edit::connect(scene,&ids)?;Ok(())})?;self.tool_status=message;return Ok(Change::Scene)},
            Action::Explode=>return self.explode_selected(),
            Action::TextBegin(words)=>return self.begin_text(words),
            Action::Trim=>return self.trim_command(),
            Action::TrimPick(id)=>{let mut trim=self.trim.take().ok_or("Start Trim first")?;let result=trim.pick(&self.scene,id);self.tool_status=trim.prompt();self.trim=Some(trim);result.map_err(|message|self.primitive_failure(message))?;return Ok(Change::View)},
            Action::TrimScreen{screen,size,commit}=>return self.trim_screen(screen,size,commit),
            Action::CancelTrim=>{if let Some(trim)=self.trim.take(){self.select_objects(trim.before,false)}self.tool_status="Trim cancelled · nothing changed".into();return Ok(Change::Preview)},
            Action::Query(query)=>{let ids=query.found(&self.scene).map_err(|message|self.primitive_failure(message))?;self.select_objects(ids,false);return Ok(Change::View)},
            Action::Arrowhead(head)=>return self.curve_heads(head),
            Action::Escape=>{if self.element.is_some()||self.draft.is_some()||self.split.is_some()||self.gizmo_edit.is_some(){self.apply_action(Action::GizmoCancel)?;return self.apply_action(Action::CancelTool)}self.select_objects([],false);return Ok(Change::View)},
            Action::Measure(kind)=>return self.measure_selection(kind),
            Action::MeasureDistance(text)=>return self.begin_measure_distance(&text),
            Action::BoundingBox=>return self.bounding_box(),
            Action::Split=>return self.split_command(),
            Action::SplitPick(id)=>return self.split_pick(id),
            Action::CancelSplit=>{self.split=None;self.tool_status="Split cancelled".into();return Ok(Change::View)},
            Action::Project(words)=>return self.begin_project(&words),
            Action::ExtendBegin{distance,asking}=>return self.begin_extend(distance,asking),
            Action::ExtendPick(id)=>return self.extend_pick(id),
            Action::Interval(interval)=>return self.edit_interval(interval),
            Action::GatherBegin{recipe,text}=>return self.begin_gather(recipe,&text),
            Action::GatherPick(id)=>return self.gather_pick(id),
            Action::ScaleFactor(factor)=>{
                if !factor.is_finite()||factor<=0.{return Err("Scale wants a finite factor above zero");}
                let origin=self.gizmo_origin().unwrap_or([0.;3]);let delta=session_rust::Xform::scale_uniform(&session_rust::Point::new(origin[0],origin[1],origin[2]),factor);let ids=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());return self.apply_transform(crate::transform_draft::Kind::Scale,&delta,&ids,target);
            },
            Action::RotateWorld{axis,degrees}=>{
                if !degrees.is_finite(){return Err("Rotate needs finite degrees");}let origin=self.gizmo_origin().unwrap_or([0.;3]);let(o,p)=(session_rust::Xform::translation(origin[0],origin[1],origin[2]),session_rust::Xform::translation(-origin[0],-origin[1],-origin[2]));let turn=match axis{crate::gizmo::Axis::X=>session_rust::Xform::rotation_x(degrees,true),crate::gizmo::Axis::Y=>session_rust::Xform::rotation_y(degrees,true),crate::gizmo::Axis::Z=>session_rust::Xform::rotation_z(degrees,true)};let delta=&(&o*&turn)*&p;let ids=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());return self.apply_transform(crate::transform_draft::Kind::Rotate,&delta,&ids,target);
            },
            Action::BeginTransformation(kind)=>return self.begin_transform(kind),
            Action::Transformation {kind,delta}=>{let ids=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());return self.apply_transform(kind,&delta,&ids,target);},
            Action::PrimitiveBegin {shape,option,text}=>{
                let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                let mut primitive=crate::primitive_draft::Draft::new(shape,frame,option).map_err(|message|self.primitive_failure(message))?;
                let geometry=if text.trim().is_empty() {None}else{primitive.text(&text,None,self.construction_plane).map_err(|message|self.primitive_failure(message))?};
                self.apply(Action::BeginTool(crate::tool::Shape::Primitive(shape)))?;
                let draft=self.draft.as_mut().unwrap();draft.primitive=Some(primitive);draft.points=draft.primitive.as_ref().unwrap().answers.iter().filter_map(|answer|match answer {crate::primitive_shape::Answer::Point(p)=>Some([p[0],p[1],p[2]]),_=>None}).collect();
                if let Some(geometry)=geometry {return self.commit_tool(crate::tool::Built::Geometry(geometry));}
                self.tool_status=draft.prompt();return Ok(Change::View);
            },
            Action::ToolSides(sides)=>{
                let draft=self.draft.as_mut().ok_or("Sides works while drawing a polygon")?;draft.tool.set_sides(sides)?;self.tool_status=draft.prompt();return Ok(Change::View);
            },
            Action::ConstructionPlane(plane)=>{
                let frame=plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(primitive)=self.draft.as_mut().and_then(|draft|draft.primitive.as_mut()) {primitive.set_plane(frame).map_err(|message|{self.tool_status=message;"Shape input invalid"})?;}
                if let Some(transform)=self.draft.as_mut().and_then(|d|d.transform.as_mut()){transform.frame=frame;}
                if let Some(gather)=self.draft.as_mut().and_then(|d|d.gather.as_mut()){gather.frame=frame;}
                self.camera.grid_plane=plane;self.construction_plane=plane;crate::drawing_coords::set_plane(plane);self.plane_menu=false;
                let frame=plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(draft)=self.draft.as_mut() {draft.frame=frame;draft.cursor=None;draft.snapped=None;}
                self.tool_status="Construction plane changed.".into();return Ok(Change::View);
            },
            Action::PlaneMenu=>{self.plane_menu=true;self.tool_status="CPlane · XY XZ YZ 3 Point View Cancel".into();return Ok(Change::View);},
            Action::Plane3=>{
                self.plane_menu=false;self.plane_only=self.draft.is_none();
                if self.plane_only {self.apply(Action::BeginTool(crate::tool::Shape::Point))?;}
                let draft=self.draft.as_mut().unwrap();draft.plane_points=Some(Vec::new());draft.frame=crate::cplane::CPlane::Xy;
                self.tool_status=draft.prompt();return Ok(Change::View);
            },
            Action::Draw {shape,text}=>{
                let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                let added=crate::drawing_input::points(&text,self.construction_plane,frame,&[],None)?;
                let mut draft=crate::tool::Draft::new(shape);draft.frame=frame;
                let built=draft.submit_many(&added)?;
                let built=if built.is_none() && draft.tool.count()==0 && added.len()>=2 {Some(draft.finish()?)}else{built};
                self.apply(Action::BeginTool(shape))?;self.draft=Some(draft);
                if let Some(built)=built {return self.commit_tool(built);}self.tool_status=self.draft.as_ref().unwrap().prompt();return Ok(Change::View);
            },
            Action::ToolText(text)=>return self.tool_text(&text),
            Action::CloseTool=>{
                let draft=self.draft.as_ref().ok_or("No point tool is running")?;
                if self.plane_only || draft.plane_points.is_some() || !matches!(draft.tool.name(),"Polyline"|"Curve"){return Err("Close works on a polyline or curve")}
                if draft.tool.count()!=0 || draft.points.len()<3 {return Err("Close needs at least three polyline points");}
                if draft.points.len()>=crate::tool::MAX_POINTS {return Err("Drawing commands accept at most 4096 points");}
                let mut points=draft.points.clone();points.push(points[0]);
                crate::drawing_coords::set_plane(self.construction_plane);
                let built=draft.build_world(&points)?;return self.commit_tool(built);
            },
            Action::FinishTool=>{if self.draft.as_ref().is_some_and(|d|d.extend.is_some()){return self.extend_text("")}if self.draft.as_ref().is_some_and(|d|d.project.is_some()){return Err("Project 3Point needs three plane points")}if self.draft.as_ref().is_some_and(|d|d.gather.is_some()){return self.gather_text("")}crate::drawing_coords::set_plane(self.construction_plane);let built=self.draft.as_ref().ok_or("No point tool is running")?.finish()?;return self.commit_tool(built);},
            Action::LayerEdit {key,edit}=>{
                if matches!(edit,crate::layer_edit::Edit::Current) {crate::layer_edit::apply(&mut self.scene,&key,edit)?;return Ok(Change::View);}
                self.history.try_edit(&mut self.scene,|scene|crate::layer_edit::apply(scene,&key,edit))?;self.repair_selection();
            },
            Action::LayerProperty {key,property}=>{crate::layer_properties::apply(&mut self.scene,&key,property)?;self.repair_selection();if self.selected.is_some_and(|id|self.scene.pick_id(id.value()).is_none()) {self.selected=None;self.selected_part=None;}self.snap_index=None;return Ok(Change::Preview);},
            Action::Hide=>{let ids=self.scene.with_descendants(&self.selected_objects());std::rc::Rc::make_mut(&mut self.scene.view).hide(&ids,true);self.select_objects([],false);self.snap_index=None;return Ok(Change::Preview);},
            Action::Show=>{std::rc::Rc::make_mut(&mut self.scene.view).show_all();self.snap_index=None;return Ok(Change::Preview);},
            Action::TreePick {ids,additive}=>{if self.trimming_picks(){for id in ids{self.apply_action(Action::TrimPick(Some(id)))?;}return Ok(Change::View)}if self.trimming_parts(){return Err("Trim now needs a part click")}if self.split.is_some(){for id in ids{self.split_pick(Some(id))?;}return Ok(Change::View)}let eligible=ids.into_iter().filter(|id|self.scene.pick_id(id.value())==Some(*id)).collect::<Vec<_>>();if eligible.is_empty() {return Err("This tree row has no visible editable objects");}self.select_objects(eligible,additive);self.apply(Action::FitSelected)?;return Ok(Change::Preview);},
            Action::GizmoTouch {pointer,screen,size}=>return self.gizmo_press_radius(pointer,screen,size,false,18.),
            Action::GizmoPress {pointer,screen,size,modified}=>return self.gizmo_press(pointer,screen,size,modified),
            Action::GizmoMove {pointer,screen,size,shift}=>return self.gizmo_move(pointer,screen,size,shift),
            Action::GizmoRelease {pointer,screen,size,shift}=>return self.gizmo_release(pointer,screen,size,shift),
            Action::GizmoCancel=>{self.gizmo_edit=None;self.tool_status="Handle edit cancelled.".into();return Ok(Change::Preview);},
            Action::GizmoNumber(text)=>return self.gizmo_number(&text),
            Action::Snap(setting) => {
                self.tool_status = self.snapping.apply(setting)?;
                if let Some(draft) = self.draft.as_mut() { draft.cursor = None; draft.snapped = None; }
                return self.update_edit_preview();
            }
            Action::ToolScreen { screen, size, commit } => {
                if self.draft.as_ref().is_some_and(|d|d.extend.is_some()&&d.plane_points.is_none()){return self.extend_screen(screen,size,commit)}
                let hit = self.snap_point(screen,size);
                if let Some(draft) = self.draft.as_mut() { draft.snapped = hit.filter(|h| h.owner.is_some() || h.kind == crate::snap::Kind::Grid).map(|h| h.kind); }
                if commit {
                    if hit.is_some_and(|h|self.draft.as_ref().is_some_and(|d|d.primitive.is_none() && d.transform.is_none() && d.gather.is_none() && d.project.is_none() && d.extend.is_none() && d.tool.count()==0 && d.points.len()>=2 && d.points.first().is_some_and(|p|(0..3).all(|i|(p[i]-h.point[i]).abs()<=1e-12)))) {return self.apply(Action::CloseTool);}
                    return hit.map_or(Ok(Change::View), |hit| self.place_tool_point(hit.point));
                }
                if let Some(draft) = self.draft.as_mut() { draft.hover(hit.map(|h| h.point))?; }
                return self.update_edit_preview();
            }
            Action::BeginTool(shape) => {
                if matches!(shape,crate::tool::Shape::Move)&&self.selected_part.is_some(){return self.begin_transform(crate::transform_draft::Kind::Move);}
                self.transform_preview=None;
                let preview=if matches!(shape,crate::tool::Shape::Move) {
                    if self.selected_part.is_some() { return Err("Use Select Object before Move"); }
                    Some(crate::edit_preview::Preview::new(&self.scene,self.selected_objects())?)
                } else { None };
                let changed=preview.is_some() || self.edit_preview.is_some(); self.edit_preview=preview;
                self.snap_index=None;let mut draft=crate::tool::Draft::new(shape);
                draft.frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(primitive)=draft.primitive.as_mut() {primitive.set_plane(draft.frame).map_err(|message|{self.tool_status=message;"Shape input invalid"})?;}
                self.draft=Some(draft);self.tool_status=self.draft.as_ref().unwrap().prompt();
                return Ok(if changed { Change::Preview } else { Change::View });
            }
            Action::ToolPoint(point) => return self.place_tool_point(point),
            Action::ToolHover(point) => { if let Some(draft) = self.draft.as_mut() { draft.snapped=None; draft.hover(point)?; } return self.update_edit_preview(); }
            Action::CancelTool => {self.element=None;if let Some(before)=self.draft.as_ref().and_then(|d|d.project.as_ref()).map(|p|p.before.clone()).or_else(||self.draft.as_ref().and_then(|d|d.extend.as_ref()).map(|e|e.before.clone())){self.select_objects(before,false);}if let Some(before)=self.draft.as_ref().and_then(|d|d.gather.as_ref()).map(|g|g.before.clone()){self.select_objects(before,false);}let changed=self.edit_preview.take().is_some()|self.transform_preview.take().is_some(); self.draft=None;self.plane_menu=false;self.plane_only=false; self.tool_status="Tool cancelled.".into(); return Ok(if changed { Change::Preview } else { Change::View }); }
            Action::Picked(id) => self.select_objects(id, false),
            Action::PickedMany { target, additive } => {
                if self.pick_mode != crate::pick_sources::Mode::Object { return self.apply(Action::PickedTarget(target)); }
                self.select_objects(target.filter(|t| t.part == crate::pick_sources::Part::Object && t.valid(&self.scene)).map(|t| t.owner), additive);
            }
            Action::SelectAll => {
                let ids: Vec<_> = self.scene.objects().iter().map(|r| r.id).chain(self.scene.lines().iter().map(|r| r.id))
                    .chain(self.scene.paths().iter().map(|r| r.id)).chain(self.scene.points().iter().map(|r| r.id)).chain(self.scene.texts().iter().map(|r|r.id)).chain(self.scene.clouds().iter().map(|r|r.id)).chain(self.scene.streams().iter().map(|r|r.id)).chain(self.scene.sheets().iter().map(|r|r.id)).collect();
                self.select_objects(ids, false);return Ok(Change::View);
            }
            Action::PickedTarget(target) => {
                let target = target.filter(|target| target.valid(&self.scene));
                self.objects.clear();
                self.selected = target.map(|target| target.owner);
                if let Some(target) = target.filter(|t| t.part == crate::pick_sources::Part::Object) { self.objects.set(&self.scene, [target.owner], false); }
                self.selected_part = target.map(|target| target.part).filter(|part| *part != crate::pick_sources::Part::Object);
            }
            Action::PickMode(mode) => { if mode!=crate::pick_sources::Mode::Control {self.scene.cloud_controls(None);} self.pick_mode = mode; self.selected = None; self.selected_part = None; self.objects.clear(); }
            Action::Pick(screen) => {
                let id = self.camera.ray(screen).and_then(|ray| crate::picking::pick(&self.scene, &ray));
                self.select_objects(id, false);
            }
            Action::UnloadSources => self.unload_sources()?,
            Action::Close => {
                self.scene.clear();
                self.history.clear();
                self.objects.clear(); self.selected = None;
            }
            Action::AddText(placement)=>{let source=std::rc::Rc::new(crate::scene_label::example(placement));let prepared=crate::scene_label::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_text(prepared).map(|_|()))?;},
            Action::AddCloudLod=>{let source=std::rc::Rc::new(crate::cloud::large_example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddCloud=>{let source=std::rc::Rc::new(crate::cloud::example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddSheet=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::sheet_example::bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare original drawing sheet"))?;},
            Action::AddNestedInstances=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::nested_example::bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare nested original placements"))?;},
            Action::AddShapeReuse=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::cad_shape_example::bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare congruent original CAD"))?;},
            Action::AddInstances=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare original definition instances"))?;},
            Action::AddCad=>{let source=std::rc::Rc::new(crate::cad_display::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_cad(source).map(|_|()))?;},
            Action::AddSurface=>{let source=std::rc::Rc::new(crate::surface::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_surface(source).map(|_|()))?;},
            Action::AddNested=>{let ready=crate::publication::prepare(&self.scene,&crate::tree_specimen::nested(true),false,None).map_err(|_|"Cannot prepare nested originals")?;self.history.edit(&mut self.scene,|scene|scene.restore(ready));},
            Action::Live(ready)=>{self.history.clear();self.scene.restore(*ready);},
            Action::Adopt {scene:ready,..}=>{self.history.edit(&mut self.scene,|scene|scene.restore(*ready));},
            file @ (Action::Replace(_) | Action::Import(_) | Action::ImportAt(_,_) | Action::ReplaceAt(_,_)) => {
                let (bytes,replace,location)=match file {
                    Action::Replace(bytes)=>(bytes,true,None),
                    Action::Import(bytes)=>(bytes,false,None),
                    Action::ImportAt(bytes,location)=>(bytes,false,Some(location)),
                    Action::ReplaceAt(bytes,location)=>(bytes,true,Some(location)),
                    _=>unreachable!(),
                };
                let ready=crate::publication::prepare(&self.scene,&bytes,replace,location).map_err(|_|"Cannot prepare original document")?;
                self.history.edit(&mut self.scene,|scene|scene.restore(ready));
            }
            Action::AddPolyline | Action::AddArrow => {
                let prepared = if matches!(action, Action::AddPolyline) {
                    let mut line = session_rust::Polyline::new(vec![session_rust::Point::new(-0.6, -0.4, 0.0),
                        session_rust::Point::new(0.0, 0.0, 0.0), session_rust::Point::new(0.6, -0.4, 0.0)]);
                    line.width = 6.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                    crate::chain::PreparedChain::polyline(std::rc::Rc::new(line))?
                } else {
                    let mut line = session_rust::Line::new(-0.6, 0.3, 0.0, 0.6, 0.3, 0.0);
                    line.width = 5.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                    line.arrowhead = session_rust::Arrowhead::BOTH;
                    crate::chain::PreparedChain::line(std::rc::Rc::new(line))?
                };
                self.history.try_edit(&mut self.scene, |scene| scene.insert_path(prepared).map(|_| ()))?;
            }
            Action::EnableControls => {
                let parent=self.selected.ok_or("Select one object before Controls")?;
                if self.pick_mode==crate::pick_sources::Mode::Control && (self.scene.control_markers().iter().any(|m|m.parent==parent) || self.scene.clouds().iter().any(|r|r.id==parent&&r.controls_visible) || self.scene.streams().iter().any(|r|r.id==parent&&r.controls_visible)) {return Ok(Change::View);}
                if self.selected_objects().len()!=1 {return Err("Select one object before Controls");}
                self.scene.owner_controls(parent)?;
                self.pick_mode=crate::pick_sources::Mode::Control;
                self.objects.clear();self.selected_part=None;
            },
            Action::Controls(visible) => {
                let parent=if visible {self.selected.filter(|id|self.scene.clouds().iter().any(|r|r.id==*id)||self.scene.streams().iter().any(|r|r.id==*id))} else {None};
                if parent.is_some() && self.pick_mode==crate::pick_sources::Mode::Control && (self.scene.clouds().iter().any(|r|Some(r.id)==parent && r.controls_visible)||self.scene.streams().iter().any(|r|Some(r.id)==parent&&r.controls_visible)) {return Ok(Change::View);}
                self.scene.cloud_controls(parent);
                self.scene.show_controls(visible);
                if parent.is_some() {self.pick_mode=crate::pick_sources::Mode::Control;self.objects.clear();self.selected_part=None;}
                else if !visible && self.pick_mode==crate::pick_sources::Mode::Control {self.pick_mode=crate::pick_sources::Mode::Object;self.selected_part=None;self.select_objects(self.selected,false);}
            },
            Action::AddNormals(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(crate::normal_example::specimen(mode))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert(prepared).map(|_| ()))?;
            }
            Action::AddColours(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(crate::colour::specimen(mode))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert(prepared).map(|_| ()))?;
            }
            Action::AddCurve => {
                let mut curve = session_rust::NurbsCurve::create(false, 2, &[session_rust::Point::new(-0.6, -0.4, 0.0),
                    session_rust::Point::new(0.0, 0.6, 0.0), session_rust::Point::new(0.6, -0.4, 0.0)]);
                curve.width = 5.0; curve.linecolors = vec![session_rust::Color::new(0.0, 0.0, 0.0, 0.5)];
                let prepared = crate::chain::PreparedChain::curve(std::rc::Rc::new(curve))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_path(prepared).map(|_| ()))?;
            }
            Action::AddPoint => {
                let mut point = session_rust::Point::new(0.0, 0.0, 0.0);
                point.width = 12.0; point.pointcolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                let prepared = crate::marker::PreparedPoint::new(std::rc::Rc::new(point))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_point(prepared).map(|_| ()))?;
            }
            Action::AddLine => {
                let mut line = session_rust::Line::new(-0.6, 0.0, 0.0, 0.6, 0.0, 0.0);
                line.width = 5.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                let prepared = crate::stroke::PreparedLine::new(std::rc::Rc::new(line))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_line(prepared).map(|_| ()))?;
            }
            Action::AddBox => {
                self.history.try_edit(&mut self.scene, |scene| scene.add_box().map(|_| ()))?;
            }
            Action::Translate(offset) => {
                if self.selected_part.is_some(){let target=self.selection();return self.apply_transform(crate::transform_draft::Kind::Move,&session_rust::Xform::translation(offset[0],offset[1],offset[2]),&[],target);}
                return self.move_selection(offset);
            },
            Action::ToggleExtra => self.history.edit(&mut self.scene, Scene::toggle_extra),
            Action::SelectNext => { let id = self.scene.next(self.selected); self.select_objects(id, false); }
            Action::Delete => return self.delete_selection(),
            Action::Undo => { self.history.undo(&mut self.scene); }
            Action::Redo => { self.history.redo(&mut self.scene); }
            view => {
                match view {
                    Action::Background => self.background.toggle(),
                    Action::NormalView(visible) => self.normal_view = visible,
                    Action::Grid(settings) => {
                        if let Some(grid) = settings { grid.half()?; grid.reach(self.camera.target)?; }
                        self.camera.grid = settings;
                    }
                    Action::Zoom(factor) => self.camera.zoom(factor),
                    Action::ZoomAt {factor,screen}=>self.camera.zoom_at(factor,screen),
                    Action::PanScreen {delta,size}=>self.camera.pan_screen(delta,size),
                    Action::Pan(dx, dy) => self.camera.pan(dx, dy),
                    Action::Orbit(yaw, pitch) => self.camera.orbit(yaw, pitch),
                    Action::Isometric => self.camera.set_view(crate::camera::View::Iso),
                    Action::NamedView(view)=>self.camera.set_view(view),
                    Action::Projection(mode) => self.camera.projection = mode,
                    Action::Fit => {
                        if let Some(bounds) = self.display_scene().bounds_of(&self.selected_objects()).or_else(||self.display_scene().bounds()) {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::FitSelected => {
                        if let Some(bounds) = self.display_scene().bounds_of(&self.selected_objects()).or_else(|| self.selected_part.and(self.selected).and_then(|id| self.scene.selected_bounds(id)))
                        {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::ResetView => {
                        let aspect = self.camera.aspect;
                        self.camera = Camera { aspect, grid: self.camera.grid,grid_plane:self.camera.grid_plane,frame:self.camera.frame, ..Camera::default() };
                    }
                    _ => unreachable!("Document actions were handled above"),
                }
                return Ok(Change::View);
            }
        }
        // Deletion and history travel can invalidate selection; repair it in one place.
        self.repair_selection();
        self.selected = self.selected.filter(|id| self.scene.contains(*id));
        if let (Some(owner), Some(part)) = (self.selected, self.selected_part) {
            if !(crate::pick_sources::Target { owner, part }).valid(&self.scene) { self.selected = None; }
        }
        if self.selected.is_none() { self.selected_part = None; }
        Ok(Change::Scene)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_modified_history_row_protects_only_its_own_import_origin() {
        use std::rc::Rc;
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let first = Rc::new(crate::reload_url::ReloadUrl::new("test:first".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), first)).unwrap();
        let origin = editor.scene.objects()[0].origin().unwrap().id;
        let original = Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        let second = Rc::new(crate::reload_url::ReloadUrl::new("test:second".into()));
        editor.apply(Action::ImportAt(bytes, second)).unwrap();
        let copy = Rc::downgrade(editor.scene.objects()[3].geometry().unwrap());
        editor.apply(Action::SelectNext).unwrap(); editor.apply(Action::Translate([0.25, 0.0, 0.0])).unwrap();
        editor.apply(Action::Undo).unwrap();
        let saved = editor.history.scenes_mut().last().unwrap();
        assert_eq!(saved.objects()[0].origin().unwrap().id, origin);
        Rc::make_mut(&mut saved.objects_mut()[0].metadata).is_locked = true;
        assert!(editor.scene.objects()[0].can_unload());
        editor.apply(Action::UnloadSources).unwrap();
        assert!(original.upgrade().is_some()); assert!(copy.upgrade().is_none());
        assert!(editor.scene.objects()[0].geometry().is_some());
        assert!(editor.scene.objects()[3].geometry().is_none());
        assert_eq!(editor.last_release, 1);
        assert!(editor.apply(Action::UnloadSources).is_err()); assert_eq!(editor.last_release, 1);
    }

    #[test]
    fn release_epochs_do_not_wrap_or_reset_on_close() {
        use std::rc::Rc;
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let url = Rc::new(crate::reload_url::ReloadUrl::new("test:epoch".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), Rc::clone(&url))).unwrap();
        editor.last_release = u64::MAX;
        assert_eq!(editor.apply(Action::UnloadSources), Err("Release epochs exhausted"));
        assert!(editor.scene.objects()[0].geometry().is_some());
        editor.last_release = 0; editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(editor.scene.objects()[0].release_epoch(), Some(1));
        editor.apply(Action::Close).unwrap();
        editor.apply(Action::ImportAt(bytes, url)).unwrap();
        editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(editor.scene.objects()[0].release_epoch(), Some(2));
    }

    #[test]
    fn close_ends_history_and_keeps_issued_ids_and_view_settings() {
        let mut editor = Editor::default();
        editor.apply(Action::Import(crate::specimen::bytes())).unwrap();
        editor.apply(Action::AddBox).unwrap(); editor.apply(Action::Undo).unwrap();
        let old: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter().map(|row| row.id)).collect();
        editor.apply(Action::SelectNext).unwrap();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap(); editor.apply(Action::Background).unwrap();
        let view = editor.camera.uniform(); let background = editor.background.rgb();
        editor.apply(Action::Close).unwrap();
        assert!(editor.scene.objects().is_empty() && editor.selected.is_none());
        assert_eq!(editor.scenes().count(), 1);
        editor.apply(Action::Undo).unwrap(); editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.objects().is_empty());
        assert_eq!(editor.camera.uniform(), view); assert_eq!(editor.background.rgb(), background);
        editor.apply(Action::AddBox).unwrap();
        assert!(!old.contains(&editor.scene.objects()[0].id));
    }

    #[test]
    fn repeated_close_is_empty_and_drops_sources_without_gpu_owners() {
        let mut editor = Editor::default();
        let display = std::rc::Rc::downgrade(&editor.scene.objects()[0].mesh);
        let source = std::rc::Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        editor.apply(Action::Close).unwrap(); editor.apply(Action::Close).unwrap();
        assert!(display.upgrade().is_none() && source.upgrade().is_none());
        assert_eq!(crate::memory::cpu(editor.scenes(), std::iter::empty()), [0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn close_releases_imported_history_documents_and_reopening_uses_fresh_ids() {
        use std::rc::Rc;
        let mut editor = Editor::default();
        let bytes = crate::specimen::bytes();
        editor.apply(Action::Import(bytes.clone())).unwrap();
        editor.apply(Action::Replace(bytes.clone())).unwrap();
        let displays: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .map(|row| Rc::downgrade(&row.mesh))).collect();
        let sources: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .map(|row| Rc::downgrade(row.geometry().unwrap()))).collect();
        let documents: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .filter_map(|row| row.source().map(|source| Rc::downgrade(&source.document)))).collect();
        let old: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter().map(|row| row.id)).collect();
        assert!(!documents.is_empty());
        editor.apply(Action::Close).unwrap();
        assert!(displays.iter().all(|owner| owner.upgrade().is_none()));
        assert!(sources.iter().all(|owner| owner.upgrade().is_none()));
        assert!(documents.iter().all(|owner| owner.upgrade().is_none()));
        editor.apply(Action::Import(bytes)).unwrap();
        assert!(editor.scene.objects().iter().all(|row| !old.contains(&row.id)));
    }

    #[test]
    fn resetting_the_view_preserves_its_current_aspect() {
        let mut editor = Editor::default();
        editor.camera.aspect = 2.0;
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::ResetView).unwrap();
        assert_eq!(editor.camera.aspect, 2.0);
        assert_eq!(editor.camera.distance, 3.0);
    }

    #[test]
    fn undo_changes_the_document_without_rewinding_the_view() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::Background).unwrap();
        editor.apply(Action::Undo).unwrap();
        assert_eq!(editor.scene.objects().len(), 2);
        assert_eq!(editor.camera.distance, 1.5);
        assert_eq!(editor.background.rgb(), [0.9, 0.9, 0.9]);
    }

    #[test]
    fn undo_repairs_selection_and_empty_delete_preserves_redo() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        for _ in 0..3 {
            editor.apply(Action::SelectNext).unwrap();
        }
        let box_id = editor.selected.unwrap();
        editor.apply(Action::Undo).unwrap();
        assert!(editor.selected.is_none());
        editor.apply(Action::Delete).unwrap();
        editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.contains(box_id));
    }

    #[test]
    fn picking_a_moved_view_uses_the_editors_current_camera() {
        let mut editor = Editor::default();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap();
        let point = session_rust::Point::new(0.2, 0.3, 0.75);
        let screen = editor.camera.view_projection().transform_point(&point);
        assert_eq!(editor.apply(Action::Pick([screen[0] as f32, screen[1] as f32])).unwrap(), Change::Scene);
        assert_eq!(editor.selected, Some(editor.scene.objects()[1].id));
        assert_eq!(editor.apply(Action::Zoom(2.0)).unwrap(), Change::View);
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzs-reuse-full-source-preparation-for-editor-and-reload-1.rs"
```

```text
--8<-- "journey/code/96zzs-reuse-full-source-preparation-for-editor-and-reload-2.rs"
```

</details>

### 2. `src/document.rs`

Validate the same full original file before recovering a released mesh source. Remove the restricted decode helper that reload no longer calls.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    proto::Session::decode(bytes).map_err(|_| "Invalid session protobuf")
}

fn decode(bytes: &[u8]) -> Result<proto::Session, &'static str> {
    let message = decode_raw(bytes)?;
    validate(&message)?;
    Ok(message)
}

pub(crate) fn restore(bytes: &[u8], origin: &crate::origin::Origin) -> Result<Rc<Session>, &'static str> {
    if bytes.len() > MAX_BYTES { return Err("This checkpoint accepts files up to 4 MiB"); }
    if crate::origin::FileVersion::of(bytes) != origin.version { return Err("Reloaded file version changed"); }
    Session::from_proto(decode(bytes)?).map(Rc::new).map_err(|_| "Cannot restore source session")
}

pub fn snapshot(scene: &crate::scene::Scene) -> Result<Vec<u8>, &'static str> {
```

</details>

Replace it with:

```rust
    proto::Session::decode(bytes).map_err(|_| "Invalid session protobuf")
}

pub(crate) fn restore(bytes: &[u8], origin: &crate::origin::Origin) -> Result<Rc<Session>, &'static str> {
    if crate::origin::FileVersion::of(bytes) != origin.version { return Err("Reloaded file version changed"); }
    crate::document_source::prepare_at(bytes,&session_rust::Xform::identity(),0.,origin.location.clone())
        .map(|source|source.document).map_err(|_|"Cannot restore source session")
}

pub fn snapshot(scene: &crate::scene::Scene) -> Result<Vec<u8>, &'static str> {
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::prepared::PreparedMesh;
use prost::Message;
use session_rust::{proto, Session};
use std::{collections::HashSet, rc::Rc};

pub const MAX_BYTES: usize = 4 * 1024 * 1024;

#[derive(Clone)]
pub struct Source {
    pub document: Rc<Session>,
    pub guid: String,
    pub origin: Rc<crate::origin::Origin>,
}

pub struct Loaded {
    pub document: Rc<Session>,
    pub meshes: Vec<PreparedMesh>,
    pub tree:Option<proto::TreeNode>,
}

pub fn load(bytes: &[u8]) -> Result<Loaded, &'static str> { load_at(bytes, None) }

pub fn load_at(bytes: &[u8], location: Option<Rc<crate::reload_url::ReloadUrl>>) -> Result<Loaded, &'static str> {
    load_observed(bytes, location, &mut crate::load_measure::now, &mut |phase| {
        #[cfg(target_arch = "wasm32")]
        crate::browser_phase::completed(phase, "document");
        #[cfg(not(target_arch = "wasm32"))]
        let _ = phase;
    })
}

pub fn load_observed(bytes: &[u8], location: Option<Rc<crate::reload_url::ReloadUrl>>,
    clock: &mut impl FnMut() -> f64, observe: &mut impl FnMut(crate::load_measure::Completed<'_>),
) -> Result<Loaded, &'static str> {
    use crate::load_measure::run;
    let size = bytes.len() as u64;
    let message = run("decode", size, clock, observe, || decode_raw(bytes))?;
    run("validation", size, clock, observe, || validate(&message))?;
    let mut origin = crate::origin::Origin::new(&message, bytes); origin.location = location;
    let origin = Rc::new(origin);
    let tree=message.tree.as_ref().and_then(|tree|tree.root.clone());
    let document = run("kernel", size, clock, observe, || {
        Session::from_proto(message).map(Rc::new).map_err(|_| "Cannot construct session")
    })?;
    let meshes = run("display walk", size, clock, observe, || {
        let mut meshes = Vec::new();
        for mesh in document.objects.meshes.iter() {
            let mut prepared = PreparedMesh::from_shared(Rc::clone(mesh))?;
            prepared.model = crate::hierarchy::world(tree.as_ref(),&document.xforms,mesh.guid());
            prepared.source = Some(Source {
                document: Rc::clone(&document), guid: mesh.guid().to_owned(), origin: Rc::clone(&origin),
            });
            meshes.push(prepared);
        }
        Ok(meshes)
    })?;
    Ok(Loaded { document, meshes, tree })
}

fn decode_raw(bytes: &[u8]) -> Result<proto::Session, &'static str> {
    if bytes.len() > MAX_BYTES { return Err("This checkpoint accepts files up to 4 MiB"); }
    proto::Session::decode(bytes).map_err(|_| "Invalid session protobuf")
}

pub(crate) fn restore(bytes: &[u8], origin: &crate::origin::Origin) -> Result<Rc<Session>, &'static str> {
    if crate::origin::FileVersion::of(bytes) != origin.version { return Err("Reloaded file version changed"); }
    crate::document_source::prepare_at(bytes,&session_rust::Xform::identity(),0.,origin.location.clone())
        .map(|source|source.document).map_err(|_|"Cannot restore source session")
}

pub fn snapshot(scene: &crate::scene::Scene) -> Result<Vec<u8>, &'static str> {
    if scene.objects().iter().any(|row|row.surface.is_some()||row.cad.is_some()) {return Err("This checkpoint cannot save CAD surface objects yet");}
    if !scene.sheets().is_empty() {return Err("Use the viewer archive to save original sheets");}
    if !scene.clouds().is_empty() {return Err("This checkpoint cannot save cloud objects yet");}
    if !scene.texts().is_empty() {return Err("This checkpoint cannot save scene text objects yet");}
    if !scene.points().is_empty() { return Err("This checkpoint cannot save point objects yet"); }
    if !scene.lines().is_empty() || !scene.paths().is_empty() { return Err("This checkpoint cannot save line objects yet"); }
    let mut objects = proto::Objects::default();
    let mut xforms = Vec::new();
    let (mut children,placed)=scene.forest.snapshot(scene,&mut xforms)?;
    for object in scene.objects() {
        let geometry = object.geometry().ok_or("Reload editable sources before saving")?;
        let mut mesh = geometry.to_proto();
        mesh.guid = object.guid.clone();
        objects.meshes.push(mesh);
        if !placed.contains(&object.id) {children.push(proto::TreeNode { name: object.guid.clone(), ..Default::default() });}
        if !placed.contains(&object.id) && object.model.m != session_rust::Xform::identity().m {
            xforms.push(proto::XformEntry {
                guid: object.guid.clone(), xform: Some(object.model.to_proto()),
            });
        }
    }
    let message = proto::Session {
        name: "Viewer document".into(), objects: Some(objects),
        tree: Some(proto::Tree { root: Some(proto::TreeNode {
            name: "Objects".into(), children, ..Default::default()
        }), ..Default::default() }), xforms, ..Default::default()
    };
    validate(&message)?;
    let bytes = message.encode_to_vec();
    if bytes.len() > MAX_BYTES { return Err("The saved document exceeds 4 MiB"); }
    Ok(bytes)
}

fn validate(message: &proto::Session) -> Result<(), &'static str> {
    let objects = message.objects.as_ref().ok_or("Session has no objects")?;
    if message.definitions.is_some() || !message.interactions.is_empty() {
        return Err("Definitions and interactions arrive in later checkpoints");
    }
    let other = objects.points.len() + objects.lines.len() + objects.planes.len()
        + objects.bboxes.len() + objects.polylines.len() + objects.pointclouds.len()
        + objects.nurbscurves.len() + objects.nurbssurfaces.len() + objects.breps.len()
        + objects.elements.len() + objects.components.len() + objects.sheets.len() + objects.instances.len();
    if other != 0 || objects.meshes.is_empty() || objects.meshes.len() > 64 {
        return Err("This checkpoint accepts 1–64 meshes and no other geometry types");
    }
    let mut guids = HashSet::new();
    for mesh in &objects.meshes {
        if mesh.guid.is_empty() || !guids.insert(mesh.guid.as_str()) {
            return Err("Every mesh needs a distinct source GUID");
        }
        validate_mesh(mesh)?;
    }
    let mut placed = HashSet::new();
    for entry in &message.xforms {
        let matrix = &entry.xform.as_ref().ok_or("Missing placement matrix")?.matrix;
        if !guids.contains(entry.guid.as_str()) || !placed.insert(entry.guid.as_str())
            || !crate::placement::valid(matrix)
        {
            return Err("Invalid mesh placement");
        }
    }
    if let Some(root) = message.tree.as_ref().and_then(|tree| tree.root.as_ref()) {
        crate::hierarchy::validate(root,&guids)?;
    }
    Ok(())
}

fn validate_mesh(mesh: &proto::Mesh) -> Result<(), &'static str> {
    if mesh.vertices.len() > 10_000 || mesh.faces.is_empty() || mesh.faces.len() > 10_000 {
        return Err("A mesh needs faces and at most 10,000 vertices and faces");
    }
    if !(0..=3).contains(&mesh.color_mode) { return Err("Unknown mesh colour mode"); }
    for colours in [&mesh.pointcolors_rgba, &mesh.facecolors_rgba, &mesh.linecolors_rgba] {
        if colours.len() % 4 != 0 || colours.len() > 160_000 || colours.iter().any(|v| !v.is_finite()) {
            return Err("Mesh colours need bounded complete finite RGBA records");
        }
    }
    if mesh.objectcolor.as_ref().is_some_and(|c| [c.r, c.g, c.b, c.a].iter().any(|v| !v.is_finite())) {
        return Err("Mesh object colour must be finite");
    }
    for (&key, vertex) in &mesh.vertices {
        if key > u32::MAX as u64 || [vertex.x, vertex.y, vertex.z].iter().any(|v| !v.is_finite()) {
            return Err("Invalid vertex key or coordinate");
        }
    }
    for (&key, face) in &mesh.faces {
        if key > u32::MAX as u64 || !(3..=4).contains(&face.vertices.len()) || !face.holes.is_empty()
            || face.vertices.iter().any(|key| !mesh.vertices.contains_key(key)) {
            return Err("This checkpoint needs triangles or quads with existing vertices and no holes");
        }
    }
    for (key, triangles) in &mesh.triangulation {
        if !mesh.faces.contains_key(key) || triangles.vertices.len() % 3 != 0
            || triangles.vertices.iter().any(|key| !mesh.vertices.contains_key(key)) {
            return Err("Invalid stored triangulation");
        }
    }
    Ok(())
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzs-reuse-full-source-preparation-for-editor-and-reload-3.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/full_source_action_tests.rs`

Supply all four editor actions, original Element payloads, shared locations, one-undo and mixed-file reload fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/96zzs-reuse-full-source-preparation-for-editor-and-reload-5.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/lib.rs`

Register the optional full-source action fixtures.

<details>
<summary>Locate the existing block</summary>

```text
]
mod cloud_shader_contract;

mod draw_slots;
#[cfg(test)] mod draw_slots_tests;

#[cfg(test)] mod source_phase_tests;

#[cfg(test)]
mod current_command_tests;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzs-reuse-full-source-preparation-for-editor-and-reload-6.rs" skip=160
```

</details>

</details>

<a id="checkpoint-96zzt-share-the-current-selection-yellow"></a>

### Step 14 of 14 · Share the current selection yellow (`96zzt-share-the-current-selection-yellow`)

- **Goal:** Use the current selection yellow for surfaces and temporary highlights without changing original colors.
- **Data Flow Trace:** `Session palette → CPU highlight data and shared WGSL → selected display`
- **Files You Touch:** `src/colour.rs` (*Define selection presentation once from the existing Session palette.*) · `src/highlight.rs` (*Use the shared palette in the existing temporary overlay owner.*) · `src/tool_preview.rs` (*Use the shared palette in the existing temporary overlay owner.*) · `src/clipping_gpu.rs` (*Include the same generated selection constant in the shared shader assembly.*) · `src/triangle.wgsl` (*Read selection presentation from the shared shader constant.*) · `src/global_outline_tests.rs` (*Update the existing exact contact-selection pixel assertion to the current palette.*) · `src/pick_gpu_tests.rs` (*Require current yellow in the original surface and temporary-preview GPU fixtures; preserve ownership and upload checks.*)

Read selection yellow from the Session color palette. Reuse it in temporary CPU highlights and generate the shared WGSL constant from those same components. Surface selection changes presentation; it must preserve authored colors, original owners and GPU storage.

> **Self-Check Question:** *Why generate the shader constant from the CPU palette?*  
> **Answer:** Both sides must use the same components. A shared definition prevents surface selection and temporary overlays from drifting apart, without adding a uniform or changing the object record.

- **Hands-On Lab Experiment:** Select a surface, then select one of its original edges or controls. Compare their yellow. Clear selection and confirm the original colors return; Undo should still describe geometry edits.

<details>
<summary>Implementation 96zzt-share-the-current-selection-yellow: Share the current selection yellow</summary>

Use the current selection yellow for surfaces and temporary highlights without changing original colors.

Data flow: Session palette → CPU highlight data and shared WGSL → selected display.

### 1. `src/colour.rs`

Define selection presentation once from the existing Session palette.

<details>
<summary>Locate the existing block</summary>

```rust
        Mode::Points => mesh.set_pointcolors(vec![red.clone(), green.clone(), blue.clone(), red, green, blue]),
        Mode::Faces => mesh.set_facecolors(vec![red, blue]),
    }
    mesh
}
```

</details>

After that block, type:

```rust

pub fn selection()->[f32;4]{Color::yellow().to_unified_array()}
pub fn selection_shader()->String{
    let [r,g,b,_]=selection();
    format!("const SELECTION_COLOR: vec3<f32> = vec3<f32>({r:?}, {g:?}, {b:?});")
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzt-share-the-current-selection-yellow-1.rs"
```

</details>

### 2. `src/highlight.rs`

Use the shared palette in the existing temporary overlay owner.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    }

    pub fn objects(&mut self, device: &wgpu::Device, scene: &Scene, ids: &[crate::scene::ObjectId]) -> Result<(), &'static str> {
        self.mesh = None; let yellow = [1., 0.75, 0.05, 1.];
        self.strokes.set(device, scene.lines().iter().filter(|r| ids.contains(&r.id) && scene.visible(r.id)).map(|r|
            crate::stroke::Stroke { colour: yellow, width: r.prepared.display.width + 2., ..r.stroke() }).chain(scene.objects().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)&&r.mesh.indices().is_empty()).flat_map(|r|r.mesh.parts.endpoints.iter().map(move|pair|{let ends=pair.map(|p|r.model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());crate::stroke::Stroke{start:ends[0],end:ends[1],colour:yellow,width:3.}}))))?;
        let planes=crate::clipping_rows::owners(scene);let flags=scene.lines().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)).map(|_|0).chain(scene.objects().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)&&r.mesh.indices().is_empty()).flat_map(|r|std::iter::repeat_n(if planes.contains(&r.id){4096}else{0},r.mesh.parts.endpoints.len()))).collect();self.strokes.set_flags(device,flags)?;
```

</details>

Replace it with:

```rust
    }

    pub fn objects(&mut self, device: &wgpu::Device, scene: &Scene, ids: &[crate::scene::ObjectId]) -> Result<(), &'static str> {
        self.mesh = None; let yellow = crate::colour::selection();
        self.strokes.set(device, scene.lines().iter().filter(|r| ids.contains(&r.id) && scene.visible(r.id)).map(|r|
            crate::stroke::Stroke { colour: yellow, width: r.prepared.display.width + 2., ..r.stroke() }).chain(scene.objects().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)&&r.mesh.indices().is_empty()).flat_map(|r|r.mesh.parts.endpoints.iter().map(move|pair|{let ends=pair.map(|p|r.model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());crate::stroke::Stroke{start:ends[0],end:ends[1],colour:yellow,width:3.}}))))?;
        let planes=crate::clipping_rows::owners(scene);let flags=scene.lines().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)).map(|_|0).chain(scene.objects().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)&&r.mesh.indices().is_empty()).flat_map(|r|std::iter::repeat_n(if planes.contains(&r.id){4096}else{0},r.mesh.parts.endpoints.len()))).collect();self.strokes.set_flags(device,flags)?;
```

<details>
<summary>Locate this block</summary>

```rust
    pub fn finite_frame(&self,device:&wgpu::Device,tables:&crate::finite_ink::Tables,depth:&wgpu::TextureView){for lane in [&self.strokes,&self.paths,&self.markers]{lane.finite_frame(device,tables,depth)}}
    pub fn set(&mut self, device: &wgpu::Device, layout: &wgpu::BindGroupLayout, scene: &Scene, target: Option<Target>) -> Result<(), &'static str> {
        self.mesh = None; let mut strokes = Vec::new(); let mut paths = Vec::new(); let mut markers = Vec::new();
        let yellow = [1., 0.75, 0.05, 1.];
        if let Some(target) = target.filter(|t| t.valid(scene)) { match target.part {
            Part::Face(key) => {
                let row = scene.objects().iter().find(|r| r.id == target.owner).unwrap();
```

</details>

Replace it with:

```rust
    pub fn finite_frame(&self,device:&wgpu::Device,tables:&crate::finite_ink::Tables,depth:&wgpu::TextureView){for lane in [&self.strokes,&self.paths,&self.markers]{lane.finite_frame(device,tables,depth)}}
    pub fn set(&mut self, device: &wgpu::Device, layout: &wgpu::BindGroupLayout, scene: &Scene, target: Option<Target>) -> Result<(), &'static str> {
        self.mesh = None; let mut strokes = Vec::new(); let mut paths = Vec::new(); let mut markers = Vec::new();
        let yellow = crate::colour::selection();
        if let Some(target) = target.filter(|t| t.valid(scene)) { match target.part {
            Part::Face(key) => {
                let row = scene.objects().iter().find(|r| r.id == target.owner).unwrap();
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{gpu_mesh::GpuMesh, pick_sources::{Part, Target}, scene::Scene, stroke_gpu::Lane};
use std::rc::Rc;

pub struct Highlight { mesh: Option<GpuMesh>, strokes: Lane, paths: Lane, markers: Lane }

impl Highlight {
    pub fn set_opacity(&self,value:f32){for lane in [&self.strokes,&self.paths,&self.markers]{lane.set_opacity(value);}}

    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self { mesh: None, strokes: Lane::new(device, format), paths: Lane::joined(device, format), markers: Lane::markers(device, format) }
    }

    pub fn enable_finite(&mut self,device:&wgpu::Device,format:wgpu::TextureFormat){for lane in [&mut self.strokes,&mut self.paths,&mut self.markers]{lane.enable_finite(device,format)}}
    pub fn finite_frame(&self,device:&wgpu::Device,tables:&crate::finite_ink::Tables,depth:&wgpu::TextureView){for lane in [&self.strokes,&self.paths,&self.markers]{lane.finite_frame(device,tables,depth)}}
    pub fn set(&mut self, device: &wgpu::Device, layout: &wgpu::BindGroupLayout, scene: &Scene, target: Option<Target>) -> Result<(), &'static str> {
        self.mesh = None; let mut strokes = Vec::new(); let mut paths = Vec::new(); let mut markers = Vec::new();
        let yellow = crate::colour::selection();
        if let Some(target) = target.filter(|t| t.valid(scene)) { match target.part {
            Part::Face(key) => {
                let row = scene.objects().iter().find(|r| r.id == target.owner).unwrap();
                let face = row.mesh.parts.faces.iter().position(|&f| f == key).unwrap() as u32;
                // Highlights own derived display data; changing selection changes presentation without mutating source geometry or adding a document history transaction.
                let mut vertices = Vec::new(); let mut normals = Vec::new(); let mut indices = Vec::new();
                for (tri, &source) in row.mesh.indices().chunks_exact(3).zip(&row.mesh.parts.triangles) { if source == face {
                    for &index in tri { indices.push(vertices.len() as u32); vertices.push(row.mesh.vertices()[index as usize]); normals.push(row.mesh.normals()[index as usize]); }
                } }
                let display = crate::mesh::Mesh::new(vertices, indices)?.with_normals(normals)?;
                let mut object = row.clone(); object.mesh = Rc::new(display);
                self.mesh = Some(GpuMesh::upload(device, layout, &object, true));
            }
            Part::BRepEdge(edge)=>{
                let row=scene.objects().iter().find(|r|r.id==target.owner).unwrap();let points=&row.cad.as_ref().unwrap().edges.iter().find(|border|border.edge==edge).unwrap().points;
                for pair in points.windows(2) {let ends=[pair[0],pair[1]].map(|p|row.model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());strokes.push(crate::stroke::Stroke {start:ends[0],end:ends[1],colour:yellow,width:3.});}
            }
            Part::SurfaceEdge(edge)=>{
                let row=scene.objects().iter().find(|r|r.id==target.owner).unwrap();let surface=row.surface.as_ref().unwrap();let boundary=surface.grid.boundaries.iter().find(|b|b.edge==edge).unwrap();
                for ends in &boundary.segments {let ends=ends.map(|index|{let p=surface.grid.samples[index as usize].point;row.model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32()});strokes.push(crate::stroke::Stroke {start:ends[0],end:ends[1],colour:yellow,width:3.});}
            }
            Part::MeshEdge(a, b) => {
                let row = scene.objects().iter().find(|r| r.id == target.owner).unwrap();
                let index = row.mesh.parts.edges.iter().position(|&edge| edge == (a, b)).unwrap();
                let ends = row.mesh.parts.endpoints[index].map(|p| row.model.transform_point(&session_rust::Point::new(p[0], p[1], p[2])).to_f32());
                strokes.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: yellow, width: 3. });
            }
            Part::PathEdge(index) => {
                let row = scene.paths().iter().find(|r| r.id == target.owner).unwrap(); let points = row.points();
                strokes.push(crate::stroke::Stroke { start: points[index].to_f32(), end: points[index + 1].to_f32(), colour: yellow, width: row.prepared.width + 2. });
            }
            Part::Curve => {
                if let Some(row) = scene.lines().iter().find(|r| r.id == target.owner) {
                    strokes.push(crate::stroke::Stroke { colour: yellow, width: row.prepared.display.width + 2., ..row.stroke() });
                } else if let Some(row) = scene.paths().iter().find(|r| r.id == target.owner) {
                    paths.extend(row.segments().into_iter().map(|mut segment| { segment.stroke.colour = yellow; segment.stroke.width += 2.; segment }));
                }
            }
            Part::Control(_) | Part::SourceControl(_) => {
                let marker = scene.control_markers().into_iter().find(|m| m.parent == target.owner && m.part == target.part).unwrap();
                markers.push(crate::marker::Marker { colour: [1., 1., 0., 1.], ..marker.display });
            }
            Part::CloudPoint(id)=>{
                let p=crate::cloud_pick::position(scene,target.owner,id).ok_or("Cloud point is missing")?;
                markers.push(crate::marker::Marker {center:p.to_f32(),diameter:11.,colour:yellow});
            }
            Part::SheetEntity(_)|Part::Object => {}
        } }
        let flag=target.filter(|t|t.valid(scene)).map_or(0,|t|if crate::clipping_rows::owners(scene).contains(&t.owner){4096}else{0});let count=strokes.len();self.strokes.set(device, strokes)?;self.strokes.set_flags(device,vec![flag;count])?; self.paths.set_chains(device, paths)?; self.markers.set_points(device, markers)?; Ok(())
    }

    pub fn objects(&mut self, device: &wgpu::Device, scene: &Scene, ids: &[crate::scene::ObjectId]) -> Result<(), &'static str> {
        self.mesh = None; let yellow = crate::colour::selection();
        self.strokes.set(device, scene.lines().iter().filter(|r| ids.contains(&r.id) && scene.visible(r.id)).map(|r|
            crate::stroke::Stroke { colour: yellow, width: r.prepared.display.width + 2., ..r.stroke() }).chain(scene.objects().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)&&r.mesh.indices().is_empty()).flat_map(|r|r.mesh.parts.endpoints.iter().map(move|pair|{let ends=pair.map(|p|r.model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());crate::stroke::Stroke{start:ends[0],end:ends[1],colour:yellow,width:3.}}))))?;
        let planes=crate::clipping_rows::owners(scene);let flags=scene.lines().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)).map(|_|0).chain(scene.objects().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)&&r.mesh.indices().is_empty()).flat_map(|r|std::iter::repeat_n(if planes.contains(&r.id){4096}else{0},r.mesh.parts.endpoints.len()))).collect();self.strokes.set_flags(device,flags)?;
        self.paths.set_chains(device, scene.paths().iter().filter(|r| ids.contains(&r.id) && scene.visible(r.id)).flat_map(|r|
            r.segments().into_iter().map(|mut s| { s.stroke.colour = yellow; s.stroke.width += 2.; s })))?;
        self.markers.set_points(device, scene.points().iter().filter(|r| ids.contains(&r.id) && scene.visible(r.id)).map(|r|
            crate::marker::Marker { colour: yellow, diameter: r.prepared.display.diameter + 4., ..r.marker() }))?;
        Ok(())
    }

    pub fn view(&self, queue: &wgpu::Queue, transform: &[f32; 16], size: [u32; 2], density: f32) {
        for lane in [&self.strokes, &self.paths, &self.markers] { lane.view(queue, transform, size, density); }
    }

    pub fn mesh(&self, pass: &mut wgpu::RenderPass<'_>) { if let Some(mesh) = &self.mesh { mesh.draw_pulled(pass); } }
    pub fn ink(&self, pass: &mut wgpu::RenderPass<'_>) { self.strokes.draw(pass); self.paths.draw(pass); }
    pub fn points(&self, pass: &mut wgpu::RenderPass<'_>) { self.markers.draw(pass); }
    pub fn clipping_bytes(&self)->[u64;2]{[&self.strokes,&self.paths,&self.markers].into_iter().map(Lane::clipping_bytes).fold([0;2],|a,b|[a[0]+b[0],a[1]+b[1]])}
    pub fn bytes(&self) -> u64 { self.mesh.as_ref().map_or(0, |m| m.geometry.allocated_bytes() + m.settings_bytes())
        + self.strokes.usage()[1] + self.paths.usage()[1] + self.markers.usage()[1] }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzt-share-the-current-selection-yellow-2.rs"
```

```text
--8<-- "journey/code/96zzt-share-the-current-selection-yellow-3.rs"
```

</details>

### 3. `src/tool_preview.rs`

Use the shared palette in the existing temporary overlay owner.

<details>
<summary>Locate the existing block</summary>

```rust
        let yellow = [1., 0.75, 0.05, 1.]; let mut strokes = Vec::new(); let mut markers = Vec::new();
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96zzt-share-the-current-selection-yellow-4.rs"
```

### 4. `src/clipping_gpu.rs`

Include the same generated selection constant in the shared shader assembly.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn source(base:&str,group:u32,binding:u32)->String{format!("@group({group}) @binding({binding}) var<uniform> clipping:ClipUniform;\n{}\n{base}",concat!(include_str!("packed_vertex.wgsl"),"\n",include_str!("physical.wgsl"),"\n",include_str!("clip.wgsl")))}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn source(base:&str,group:u32,binding:u32)->String{format!("{}\n@group({group}) @binding({binding}) var<uniform> clipping:ClipUniform;\n{}\n{base}",crate::colour::selection_shader(),concat!(include_str!("packed_vertex.wgsl"),"\n",include_str!("physical.wgsl"),"\n",include_str!("clip.wgsl")))}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzt-share-the-current-selection-yellow-5.rs"
```

</details>

### 5. `src/triangle.wgsl`

Read selection presentation from the shared shader constant.

<details>
<summary>Locate the existing block</summary>

```wgsl
    output.colour = select(displayed, vec3<f32>(1.0, 0.75, 0.05), (object.selection.x & 1u) != 0u);
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    output.colour = select(displayed, SELECTION_COLOR, (object.selection.x & 1u) != 0u);
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzt-share-the-current-selection-yellow-6.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/global_outline_tests.rs`

Update the existing exact contact-selection pixel assertion to the current palette.

<details>
<summary>Locate the existing block</summary>

```text
&[255,191,13]
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzt-share-the-current-selection-yellow-7.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/global_outline_tests.rs`

Supply actual selected pixels, original picks, exact restoration, stable uploads and release assertions.

<details>
<summary>Locate the existing block</summary>

```text
sert_eq!(gpu.stats()[0],uploads);assert_eq!(source.alphas(),vec![1./255.;vertices]);
 scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzt-share-the-current-selection-yellow-8.rs" skip=160
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 8. `src/pick_gpu_tests.rs`

Require current yellow in the original surface and temporary-preview GPU fixtures; preserve ownership and upload checks.

<details>
<summary>Locate the existing block</summary>

```text
assert!(p[0] > p[1] && p[1] > p[2]);
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzt-share-the-current-selection-yellow-9.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 9. `src/pick_gpu_tests.rs`

Require current yellow in the original surface and temporary-preview GPU fixtures; preserve ownership and upload checks.

<details>
<summary>Locate the existing block</summary>

```text
assert!(p[0]>p[1] && p[1]>p[2], "The preview is yellow: {p:?}");
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzt-share-the-current-selection-yellow-10.rs"
```

</details>

</details>

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 96zzt-share-the-current-selection-yellow
npm --prefix ../session_tests run course -- check 96zzt-share-the-current-selection-yellow
npm --prefix ../session_tests run course -- save 96zzt-share-the-current-selection-yellow
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

- **Final Lesson Experiment:** Select a surface, then select one of its original edges or controls. Compare their yellow. Clear selection and confirm the original colors return; Undo should still describe geometry edits.

Continue with [Slots, independent instance owners and stable record updates](../../consolidation/96zzu-separate-primitive-addresses-from-selection.md).
