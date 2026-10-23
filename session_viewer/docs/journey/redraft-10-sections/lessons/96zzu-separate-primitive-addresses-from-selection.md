# 20.4 · Slots, independent instance owners and stable record updates

**Section 10: Shared GPU Arenas, Floating-Origin Precision, Multisampling, and Production Parity** · **Lesson 20.4** (Redraft `10.8`) · **24 checkpoints (`96zzu-separate-primitive-addresses-from-selection` → `96zzzr-share-transparent-instance-records`)** · **Study time: 19.400000000000006–24 h**

> **What You Will Learn & Build:** Keep full primitive addresses in the range record while selection remains an independent flag. By the final checkpoint (`96zzzr-share-transparent-instance-records`), you will draw transparent placements from the same shared records in scene order.

## 1. Visual Pipeline & WebGPU Architecture (Black & White Schematics)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 220" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="212" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Lesson 20.4 Execution Flow: Slots, independent instance owners and stable record updates</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">Concrete data pipeline you build in this lesson (24 Checkpoints: 96zzu-separate-primitive-addresses-from-selection → 96zzzr-share-transparent-instance-records)</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="89" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="92" x2="151" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="89" y="108" text-anchor="middle" font-size="11" fill="#111111">range record</text>
    <line x1="161" y1="108" x2="181" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="183" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="254" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="193" y1="92" x2="316" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="254" y="108" text-anchor="middle" font-size="11" fill="#111111">ordinary shader or</text>
    <text x="254" y="124" text-anchor="middle" font-size="11" fill="#111111">instance slot</text>
    <line x1="326" y1="108" x2="346" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="348" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="358" y1="92" x2="481" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">physical primitive</text>
    <text x="419" y="124" text-anchor="middle" font-size="11" fill="#111111">image</text>
    <line x1="491" y1="108" x2="511" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="513" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="584" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="523" y1="92" x2="646" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="584" y="108" text-anchor="middle" font-size="11" fill="#111111">Original object ID</text>
    <line x1="656" y1="108" x2="676" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="678" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="749" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 5</text>
    <line x1="688" y1="92" x2="811" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="749" y="108" text-anchor="middle" font-size="11" fill="#111111">existing shared</text>
    <text x="749" y="124" text-anchor="middle" font-size="11" fill="#111111">record and slot</text>
    
    <rect x="18" y="162" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="181" font-size="11" font-weight="700" fill="#111111">Files You Edit in Lesson 20.4:</text>
    <text x="215" y="181" font-size="11" fill="#111111">src/primitive_image.rs  ·  src/gpu_mesh.rs  ·  src/triangle.wgsl  ·  src/triangle_pick.wgsl  ·  src/instance_gpu.rs  ·  src/renderer.rs</text>
    <text x="32" y="198" font-size="10.5" fill="#111111">24 Checkpoints: 96zzu-separate-primitive-addresses-from-selection → 96zzzr-share-transparent-instance-records — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
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
| **1** | [`96zzu-separate-primitive-addresses-from-selection`](#checkpoint-96zzu-separate-primitive-addresses-from-selection) | Keep full primitive addresses in the range record while selection remains an independent flag. | `src/primitive_image.rs`, `src/gpu_mesh.rs`, `src/triangle.wgsl`, `src/triangle_pick.wgsl` | `range record → ordinary shader or instance slot → physical primitive image` |
| **2** | [`96zzv-bind-the-own-row-slot-for-ordinary-surfaces`](#checkpoint-96zzv-bind-the-own-row-slot-for-ordinary-surfaces) | Use one integer vertex input for ordinary surfaces and placed-instance surfaces. | `src/draw_slots.rs`, `src/clipping_gpu.rs`, `src/triangle.wgsl`, `src/instance_gpu.rs` | `eight-byte draw slot → own uniform or instance row → shared surface vertex` |
| **3** | [`96zzw-reload-scenes-through-the-existing-publication-owner`](#checkpoint-96zzw-reload-scenes-through-the-existing-publication-owner) | Expose scene replacement through the existing reader, including empty-manifest document release. | `src/publication_route.rs`, `src/browser_publication.rs`, `src/browser_runtime.rs`, `src/reload_complete_tests.rs` | `reload_scene → publication owner → validated candidate → Live adoption` |
| **4** | [`96zzx-update-only-changed-slot-runs`](#checkpoint-96zzx-update-only-changed-slot-runs) | Patch changed instance slots without rewriting their unchanged neighbours. | `src/text_gpu_support.rs`, `src/draw_slots.rs`, `src/instance_gpu.rs`, `src/draw_slots_tests.rs` | `changed rows → contiguous runs → queue.write_buffer → unchanged draw slots` |
| **5** | [`96zzy-share-one-slot-buffer-across-surface-draws`](#checkpoint-96zzy-share-one-slot-buffer-across-surface-draws) | Bind ordinary surfaces and every instance batch from one slot buffer. | `src/instance_gpu.rs`, `src/renderer.rs`, `src/global_outline_tests.rs` | `one slot owner → sentinel or batch range → shared surface vertex entry` |
| **6** | [`96zzz-prepare-the-integer-primitive-slot-texture`](#checkpoint-96zzz-prepare-the-integer-primitive-slot-texture) | Store slot lookup records in an integer texture with bounded partial uploads. | `src/primitive_slot_table.rs`, `src/lib.rs`, `src/primitive_slot_table_tests.rs` | `slot index → texture row and column → four exact integer words` |
| **7** | [`96zzza-describe-slot-and-triangle-runs`](#checkpoint-96zzza-describe-slot-and-triangle-runs) | Represent reserved slot capacity separately from occupied members and triangle IDs. | `src/slot_space.rs`, `src/lib.rs` | `definition → reserved slots and IDs → used members` |
| **8** | [`96zzzb-read-slot-runs-and-lookup-texels`](#checkpoint-96zzzb-read-slot-runs-and-lookup-texels) | Derive GPU slot values and lookup texels from the same CPU run. | `src/slot_space.rs` | `CPU run → vertex slot and fragment lookup texel` |
| **9** | [`96zzzc-reuse-and-merge-free-slot-ranges`](#checkpoint-96zzzc-reuse-and-merge-free-slot-ranges) | Allocate fitting holes, retain unused room and merge released neighbours. | `src/slot_space.rs` | `free range → fitting allocation → retained remainder` |
| **10** | [`96zzzd-follow-arena-growth-with-slot-headroom`](#checkpoint-96zzzd-follow-arena-growth-with-slot-headroom) | Preserve instance IDs while arena growth remains inside reserved headroom. | `src/slot_space.rs` | `arena count → reserved headroom → header update or renumber` |
| **11** | [`96zzze-report-only-changed-instance-slots`](#checkpoint-96zzze-report-only-changed-instance-slots) | Update touched members and growing definitions while reporting bounded dirty ranges. | `src/slot_space.rs` | `touched members → reused runs → coalesced dirty ranges` |
| **12** | [`96zzzf-project-triangles-at-their-physical-addresses`](#checkpoint-96zzzf-project-triangles-at-their-physical-addresses) | Keep finite triangle records at the same addresses written into the primitive image. | `src/gpu_mesh.rs`, `src/tile_gpu.rs`, `src/global_outline_tests.rs` | `object primitive base → projected record interval → finite tile references` |
| **13** | [`96zzzg-clear-the-current-arctic-background`](#checkpoint-96zzzg-clear-the-current-arctic-background) | Use the current Arctic background without another draw or geometry upload. | `src/renderer.rs`, `src/ambient_gpu_tests.rs` | `Arctic setting → existing colour attachment clear → sRGB target → stored background pixel` |
| **14** | [`96zzzh-share-changed-record-runs`](#checkpoint-96zzzh-share-changed-record-runs) | Reuse the existing contiguous-change scan for slot and object records. | `src/draw_slots.rs` | `previous records → changed contiguous runs → existing bounded buffer writes` |
| **15** | [`96zzzi-prepare-one-shared-object-record-table`](#checkpoint-96zzzi-prepare-one-shared-object-record-table) | Prepare one growable storage table without duplicating slot upload logic. | `src/object_rows.rs`, `src/lib.rs` | `packed object records → changed runs → growing storage buffer → current immutable binding` |
| **16** | [`96zzzj-share-instance-object-records-across-definitions`](#checkpoint-96zzzj-share-instance-object-records-across-definitions) | Draw distinct definitions from one shared placement-record buffer. | `src/instance_gpu.rs`, `src/global_outline_tests.rs` | `original placements → shared object records → global slot row → unchanged pulled surface shader` |
| **17** | [`96zzzk-reuse-checked-curve-control-coordinates`](#checkpoint-96zzzk-reuse-checked-curve-control-coordinates) | Use one checked rational-control decoder for sampling and control markers. | `src/controls.rs`, `src/curve.rs` | `original curve and index → shared checked control coordinate → sampling or marker display` |
| **18** | [`96zzzl-publish-allocator-texels-through-the-integer-table`](#checkpoint-96zzzl-publish-allocator-texels-through-the-integer-table) | Connect allocator changes to the existing exact integer texture writer. | `src/primitive_slot_table.rs`, `src/primitive_slot_table_tests.rs` | `SlotSpace change → exact allocator texels → full or bounded texture write → current texture view` |
| **19** | [`96zzzm-resolve-physical-triangles-with-integer-slots`](#checkpoint-96zzzm-resolve-physical-triangles-with-integer-slots) | Resolve a physical triangle ID back to its definition triangle and object row. | `src/primitive_slot_lookup.wgsl`, `src/primitive_slot_table_tests.rs` | `One-based physical ID → unsigned table search → definition triangle and object row` |
| **20** | [`96zzzn-batch-shared-geometry-with-original-ownership`](#checkpoint-96zzzn-batch-shared-geometry-with-original-ownership) | Reuse instance drawing for copied originals sharing GPU geometry. | `src/renderer.rs`, `src/pick_gpu_tests.rs`, `src/instance_gpu.rs`, `src/object_rows.rs` | `Visible original rows → shared GPU geometry → object-record batch → original IDs` |
| **21** | [`96zzzo-shrink-record-ranges-without-rewriting-live-prefixes`](#checkpoint-96zzzo-shrink-record-ranges-without-rewriting-live-prefixes) | Reuse one logical buffer truncate operation for object records and draw slots. | `src/text_gpu_support.rs`, `src/object_rows.rs`, `src/draw_slots.rs`, `src/draw_slots_tests.rs` | `Shorter live range → same allocation → changed live runs only → current draw count` |
| **22** | [`96zzzp-share-opaque-ordinary-surface-records`](#checkpoint-96zzzp-share-opaque-ordinary-surface-records) | Draw ordinary opaque surfaces from the same object-record table as instances. | `src/instance_gpu.rs`, `src/renderer.rs`, `src/pick_gpu_tests.rs`, `src/global_outline_tests.rs` | `Ordinary visible mesh → shared packed record and draw slot → existing storage shader → original pixels and primitive IDs` |
| **23** | [`96zzzq-blend-ordinary-surface-records`](#checkpoint-96zzzq-blend-ordinary-surface-records) | Use the shared ordinary record table at the current translucent default. | `src/instance_gpu.rs`, `src/renderer.rs`, `src/global_outline_tests.rs` | `Shared ordinary record → alpha-blended color plus unchanged integer primitive → earlier scene order` |
| **24** | [`96zzzr-share-transparent-instance-records`](#checkpoint-96zzzr-share-transparent-instance-records) | Draw transparent placements from the same shared records in scene order. | `src/instance_gpu.rs`, `src/global_outline_tests.rs` | `Original object ID → existing shared record and slot → one blended draw in scene order` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Read the **Concept Walkthrough** and **Self-Check Q&A** first, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-96zzu-separate-primitive-addresses-from-selection"></a>

### Step 1 of 24 · Separate primitive addresses from selection (`96zzu-separate-primitive-addresses-from-selection`)

- **Goal:** Keep full primitive addresses in the range record while selection remains an independent flag.
- **Data Flow Trace:** `range record → ordinary shader or instance slot → physical primitive image`
- **Files You Touch:** `src/primitive_image.rs` (*Name the beginning of the existing reserved cap interval.*) · `src/gpu_mesh.rs` (*Keep selection independent of the address.*) · `src/triangle.wgsl` (*Read the same range word in color and first picking.*) · `src/triangle_pick.wgsl` (*Read the same range word in color and first picking.*) · `src/instance_gpu.rs` (*Keep the instance shader adaptation attached to the new ordinary expression.*) · `src/renderer.rs` (*Bound cumulative visible ranges below reserved cap markers.*) · `src/global_outline_tests.rs` (*Read the independent base in the existing ownership fixture.*)

Selection is presentation, not part of a triangle address. Move the primitive base into the existing range record. Both color and first-pick shaders read that word, and instance slots reuse it. Keep the reserved cap interval outside ordinary geometry.

> **Self-Check Question:** *Why check the end of the range rather than the base alone?*  
> **Answer:** Every emitted address adds the triangle index and one. The last address must remain below the reserved cap interval, including when integer addition would overflow.

- **Hands-On Lab Experiment:** Select two placed copies, clear selection, hide one and pick the other. Their original identities must remain stable even when visible primitive ranges are rebuilt.

<details>
<summary>Implementation 96zzu-separate-primitive-addresses-from-selection: Separate primitive addresses from selection</summary>

Keep full primitive addresses in the range record while selection remains an independent flag.

Data flow: range record → ordinary shader or instance slot → physical primitive image.

### 1. `src/primitive_image.rs`

Name the beginning of the existing reserved cap interval.

<details>
<summary>Locate the existing block</summary>

```rust
pub(crate) const FORMAT:wgpu::TextureFormat=wgpu::TextureFormat::Rg16Uint;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub(crate) const CAP_START:u32=0xfffffff0;
pub(crate) const FORMAT:wgpu::TextureFormat=wgpu::TextureFormat::Rg16Uint;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzu-separate-primitive-addresses-from-selection-1.rs"
```

</details>

### 2. `src/gpu_mesh.rs`

Keep selection independent of the address. Use the already aligned range record for the full primitive base. Validate the entire emitted interval and upload only its address word.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
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
```

</details>

Replace it with:

```rust
    pub fn selected(&self)->bool {u32::from_ne_bytes(self.previous[64..68].try_into().unwrap())&1!=0}

    pub fn set_primitive_base(&mut self,queue:&wgpu::Queue,base:u32)->Result<(),&'static str>{
        if base.checked_add(self.geometry.index_count/3).is_none_or(|end|end>=crate::primitive_image::CAP_START){return Err("Primitive range overlaps reserved caps");}
        if self.primitive_base!=base{
            self.previous[RANGE+4..RANGE+8].copy_from_slice(&base.to_ne_bytes());
            queue.write_buffer(&self.model,(RANGE+4)as u64,&self.previous[RANGE+4..RANGE+8]);self.primitive_base=base;
        }
        Ok(())
    }
```

<details>
<summary>Locate this block</summary>

```rust
    }
    if let Some(colour)=style.colour {for (index,value) in colour[..3].iter().copied().chain([1.]).enumerate() {bytes[144+index*4..148+index*4].copy_from_slice(&value.to_ne_bytes());}}
    bytes[RANGE..RANGE+4].copy_from_slice(&index_start.to_ne_bytes());
    bytes
}
```

</details>

Replace it with:

```rust
    }
    if let Some(colour)=style.colour {for (index,value) in colour[..3].iter().copied().chain([1.]).enumerate() {bytes[144+index*4..148+index*4].copy_from_slice(&value.to_ne_bytes());}}
    bytes[RANGE..RANGE+4].copy_from_slice(&index_start.to_ne_bytes());
    bytes[RANGE+4..RANGE+8].copy_from_slice(&primitive_base.to_ne_bytes());
    bytes
}
```

<details>
<summary>Locate this block</summary>

```rust
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
    bytes[64..68].copy_from_slice(&((primitive_base<<1)|u32::from(style.selected)).to_ne_bytes());
    let id=if style.pickable {object.id.value()} else {0};
    bytes[68..72].copy_from_slice(&id.to_ne_bytes());
    let flags=if matches!(&object.original,Some(session_rust::Geometry::Plane(p)) if crate::clipping_geometry::is_clipping(p)){4096u32}else{0};bytes[72..76].copy_from_slice(&(flags|cap_flags).to_ne_bytes());bytes[76..80].copy_from_slice(&cap_token.to_ne_bytes());
```

</details>

Replace it with:

```rust
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
    bytes[64..68].copy_from_slice(&u32::from(style.selected).to_ne_bytes());
    let id=if style.pickable {object.id.value()} else {0};
    bytes[68..72].copy_from_slice(&id.to_ne_bytes());
    let flags=if matches!(&object.original,Some(session_rust::Geometry::Plane(p)) if crate::clipping_geometry::is_clipping(p)){4096u32}else{0};bytes[72..76].copy_from_slice(&(flags|cap_flags).to_ne_bytes());bytes[76..80].copy_from_slice(&cap_token.to_ne_bytes());
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
    bytes[64..68].copy_from_slice(&u32::from(style.selected).to_ne_bytes());
    let id=if style.pickable {object.id.value()} else {0};
    bytes[68..72].copy_from_slice(&id.to_ne_bytes());
    let flags=if matches!(&object.original,Some(session_rust::Geometry::Plane(p)) if crate::clipping_geometry::is_clipping(p)){4096u32}else{0};bytes[72..76].copy_from_slice(&(flags|cap_flags).to_ne_bytes());bytes[76..80].copy_from_slice(&cap_token.to_ne_bytes());
    let normals = crate::normals::matrix(&object.model).expect("Scene placements are checked before insertion or editing");
    for (index, value) in normals.iter().enumerate() {
        bytes[80 + index * 4..84 + index * 4].copy_from_slice(&value.to_ne_bytes());
    }
    if let Some(colour)=style.colour {for (index,value) in colour[..3].iter().copied().chain([1.]).enumerate() {bytes[144+index*4..148+index*4].copy_from_slice(&value.to_ne_bytes());}}
    bytes[RANGE..RANGE+4].copy_from_slice(&index_start.to_ne_bytes());
    bytes[RANGE+4..RANGE+8].copy_from_slice(&primitive_base.to_ne_bytes());
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
        if base.checked_add(self.geometry.index_count/3).is_none_or(|end|end>=crate::primitive_image::CAP_START){return Err("Primitive range overlaps reserved caps");}
        if self.primitive_base!=base{
            self.previous[RANGE+4..RANGE+8].copy_from_slice(&base.to_ne_bytes());
            queue.write_buffer(&self.model,(RANGE+4)as u64,&self.previous[RANGE+4..RANGE+8]);self.primitive_base=base;
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
--8<-- "journey/code/96zzu-separate-primitive-addresses-from-selection-2.rs"
```

```text
--8<-- "journey/code/96zzu-separate-primitive-addresses-from-selection-3.rs"
```

```text
--8<-- "journey/code/96zzu-separate-primitive-addresses-from-selection-4.rs"
```

</details>

### 3. `src/triangle.wgsl`

Read the same range word in color and first picking.

<details>
<summary>Locate the existing block</summary>

```wgsl
    output.primitive=(object.selection.x>>1u)+occurrence/3u+1u;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    output.primitive=object.range.y+occurrence/3u+1u;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzu-separate-primitive-addresses-from-selection-5.rs"
```

</details>

### 4. `src/triangle_pick.wgsl`

Read the same range word in color and first picking.

<details>
<summary>Locate the existing block</summary>

```wgsl
    return MeshPickOutput(crop.transform*placed.position,placed.world,placed.clip_flags,(object_settings.selection.x>>1u)+occurrence/3u+1u);
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    return MeshPickOutput(crop.transform*placed.position,placed.world,placed.clip_flags,object_settings.range.y+occurrence/3u+1u);
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzu-separate-primitive-addresses-from-selection-6.rs"
```

</details>

### 5. `src/instance_gpu.rs`

Keep the instance shader adaptation attached to the new ordinary expression. Build instance slots from the same full address.

<details>
<summary>Locate the existing block</summary>

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
  .replace("object.range.y+occurrence/3u+1u","primitive_base+occurrence/3u+1u");
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
                let slots:Vec<crate::draw_slots::Slot>=records.iter().enumerate().map(|(row,record)|[row as u32,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]).collect();
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
--8<-- "journey/code/96zzu-separate-primitive-addresses-from-selection-7.rs"
```

```text
--8<-- "journey/code/96zzu-separate-primitive-addresses-from-selection-8.rs"
```

</details>

### 6. `src/renderer.rs`

Bound cumulative visible ranges below reserved cap markers.

<details>
<summary>Locate the existing block</summary>

```rust
            base=base.checked_add(mesh.geometry.index_count/3).filter(|n|*n<=0x7fff_ffff).expect("Bounded projected triangle range");
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
            base=base.checked_add(mesh.geometry.index_count/3).filter(|n|*n<crate::primitive_image::CAP_START).expect("Bounded projected triangle range");
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzu-separate-primitive-addresses-from-selection-9.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/global_outline_tests.rs`

Read the independent base in the existing ownership fixture.

<details>
<summary>Locate the existing block</summary>

```text
u32::from_ne_bytes(row.packed()[64..68].try_into().unwrap())>>1
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzu-separate-primitive-addresses-from-selection-10.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 8. `src/global_outline_tests.rs`

Reject reserved addresses while keeping the original unchanged-state assertion.

<details>
<summary>Locate the existing block</summary>

```text
set_primitive_base(&gpu.queue,0x8000_0000).is_err()
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzu-separate-primitive-addresses-from-selection-11.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 9. `src/global_outline_tests.rs`

Supply full high-bit GPU readback, selection independence, cap collision and overflow assertions.

<details>
<summary>Locate the existing block</summary>

```text
()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzu-separate-primitive-addresses-from-selection-12.rs" skip=160
```

</details>

</details>

<a id="checkpoint-96zzv-bind-the-own-row-slot-for-ordinary-surfaces"></a>

### Step 2 of 24 · Bind the own-row slot for ordinary surfaces (`96zzv-bind-the-own-row-slot-for-ordinary-surfaces`)

- **Goal:** Use one integer vertex input for ordinary surfaces and placed-instance surfaces.
- **Data Flow Trace:** `eight-byte draw slot → own uniform or instance row → shared surface vertex`
- **Files You Touch:** `src/draw_slots.rs` (*Generate the shader sentinel from the existing integer owner.*) · `src/clipping_gpu.rs` (*Include the same own-row sentinel in shared shader assembly.*) · `src/triangle.wgsl` (*Read the shared eight-byte vertex slot in ordinary and instance pipelines.*) · `src/instance_gpu.rs` (*Retain only record-owner adaptation; the vertex entry and slots are now shared.*) · `src/renderer.rs` (*Own one permanent default slot with the renderer pipeline resources.*) · `src/global_outline_tests.rs` (*Supply actual ordinary/instance pixels, placed-face highlights, primitive preservation, original sources and permanent sentinel checks.*)

Ordinary surfaces use their own uniform record; placed instances use a storage row. Bind the reserved own-row slot for ordinary and temporary surface passes. Both pipelines now share one vertex entry and slot layout.

> **Self-Check Question:** *Why bind the own-row slot again before a temporary face highlight?*  
> **Answer:** An instance batch may have changed the vertex binding. The next pass must choose its own record and primitive base explicitly.

- **Hands-On Lab Experiment:** Compare opaque instance drawing with a partly transparent view, which uses ordinary draws. Select an original face after the instance pass and check that its highlight stays on the same placed surface.

<details>
<summary>Implementation 96zzv-bind-the-own-row-slot-for-ordinary-surfaces: Bind the own-row slot for ordinary surfaces</summary>

Use one integer vertex input for ordinary surfaces and placed-instance surfaces.

Data flow: eight-byte draw slot → own uniform or instance row → shared surface vertex.

### 1. `src/draw_slots.rs`

Generate the shader sentinel from the existing integer owner.

<details>
<summary>Locate the existing block</summary>

```rust
pub const OWN_ROW:u32=u32::MAX;
```

</details>

After that block, type:

```rust
pub fn sentinel_shader()->String{format!("const OWN_ROW:u32={OWN_ROW}u;")}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzv-bind-the-own-row-slot-for-ordinary-surfaces-1.rs"
```

</details>

### 2. `src/clipping_gpu.rs`

Include the same own-row sentinel in shared shader assembly.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn source(base:&str,group:u32,binding:u32)->String{format!("{}\n@group({group}) @binding({binding}) var<uniform> clipping:ClipUniform;\n{}\n{base}",crate::colour::selection_shader(),concat!(include_str!("packed_vertex.wgsl"),"\n",include_str!("physical.wgsl"),"\n",include_str!("clip.wgsl")))}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn source(base:&str,group:u32,binding:u32)->String{format!("{}\n{}\n@group({group}) @binding({binding}) var<uniform> clipping:ClipUniform;\n{}\n{base}",crate::colour::selection_shader(),crate::draw_slots::sentinel_shader(),concat!(include_str!("packed_vertex.wgsl"),"\n",include_str!("physical.wgsl"),"\n",include_str!("clip.wgsl")))}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzv-bind-the-own-row-slot-for-ordinary-surfaces-2.rs"
```

</details>

### 3. `src/triangle.wgsl`

Read the shared eight-byte vertex slot in ordinary and instance pipelines. Choose the own-record base for ordinary surfaces and the slot base for instances. Emit the selected independent primitive base.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```wgsl
}

@vertex
fn vertex(@builtin(vertex_index) occurrence: u32, @builtin(instance_index) instance: u32) -> VertexOutput {
    let object=object_at(instance);
    let input=mesh_vertex(occurrence,object.range.x);
    var output=geometry(input.position,input.colour,input.normal,object);
    output.primitive=object.range.y+occurrence/3u+1u;
    return output;
}
```

</details>

Replace it with:

```wgsl
}

@vertex
fn vertex(@builtin(vertex_index) occurrence: u32, @location(0) row:u32,@location(1) slot_base:u32) -> VertexOutput {
    let object=object_at(row);
    let primitive_base=select(slot_base,object.range.y,row==OWN_ROW);
    let input=mesh_vertex(occurrence,object.range.x);
    var output=geometry(input.position,input.colour,input.normal,object);
    output.primitive=primitive_base+occurrence/3u+1u;
    return output;
}
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

const CONTACT_FLAG:u32=65536u;
fn geometry(position: vec3<f32>, colour: vec4<f32>, normal: vec3<f32>, object: ObjectSettings) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = view.transform * world;
    let contact=abs(colour.a-1./255.)<0.5/255.;
    let displayed=select(colour.rgb,object.face_colour.xyz,object.face_colour.w>0.5&&!contact);
    output.colour = select(displayed, SELECTION_COLOR, (object.selection.x & 1u) != 0u);
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
fn vertex(@builtin(vertex_index) occurrence: u32, @location(0) row:u32,@location(1) slot_base:u32) -> VertexOutput {
    let object=object_at(row);
    let primitive_base=select(slot_base,object.range.y,row==OWN_ROW);
    let input=mesh_vertex(occurrence,object.range.x);
    var output=geometry(input.position,input.colour,input.normal,object);
    output.primitive=primitive_base+occurrence/3u+1u;
    return output;
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzv-bind-the-own-row-slot-for-ordinary-surfaces-3.rs"
```

```text
--8<-- "journey/code/96zzv-bind-the-own-row-slot-for-ordinary-surfaces-4.rs"
```

```text
--8<-- "journey/code/96zzv-bind-the-own-row-slot-for-ordinary-surfaces-5.rs"
```

</details>

### 4. `src/instance_gpu.rs`

Retain only record-owner adaptation; the vertex entry and slots are now shared.

<details>
<summary>Locate the existing block</summary>

```rust
  .replace("return object_settings;","return objects[index];")
  .replace("@builtin(instance_index) instance: u32","@location(0) row:u32,@location(1) primitive_base:u32")
  .replace("let object=object_at(instance);","let object=object_at(row);")
  .replace("object.range.y+occurrence/3u+1u","primitive_base+occurrence/3u+1u");
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
  .replace("return object_settings;","return objects[index];");
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzv-bind-the-own-row-slot-for-ordinary-surfaces-6.rs"
```

</details>

### 5. `src/renderer.rs`

Own one permanent default slot with the renderer pipeline resources. Declare the same integer vertex layout for ordinary surface pipelines. Create the eight-byte own-row sentinel once. Carry the existing slot owner into the renderer. Bind the own-row input before ordinary surface draws. Restore the own-row input after any instance batch changes it.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            pass.set_bind_group(2,&clip_group,&[]);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            for mesh in self.meshes.iter().filter(|row|self.opacity.get()<1.||!self.instance_rows.contains(&row.id)) {mesh.draw_pulled(&mut pass);}
            if self.opacity.get()>=1.{self.instances.draw(&mut pass);}
            pass.set_pipeline(&self.highlight_pipeline); pass.set_bind_group(0, &self.view_group, &[]);
            self.highlight.mesh(&mut pass);
        }
        self.caps.borrow_mut().draw(&mut encoder,view,&self.primitive.view,&self.depth,&self.view_group,&clip_group,&self.meshes,if self.clipping.borrow().enabled{self.clipping.borrow().rows.len()as u32}else{0});
```

</details>

Replace it with:

```rust
            pass.set_bind_group(2,&clip_group,&[]);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.own_slot.bind(&mut pass,0,0);
            for mesh in self.meshes.iter().filter(|row|self.opacity.get()<1.||!self.instance_rows.contains(&row.id)) {mesh.draw_pulled(&mut pass);}
            if self.opacity.get()>=1.{self.instances.draw(&mut pass);}
            pass.set_pipeline(&self.highlight_pipeline); pass.set_bind_group(0, &self.view_group, &[]);
            self.own_slot.bind(&mut pass,0,0);
            self.highlight.mesh(&mut pass);
        }
        self.caps.borrow_mut().draw(&mut encoder,view,&self.primitive.view,&self.depth,&self.view_group,&clip_group,&self.meshes,if self.clipping.borrow().enabled{self.clipping.borrow().rows.len()as u32}else{0});
```

<details>
<summary>Locate this block</summary>

```rust
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets=crate::sheet_gpu::Lane::new(&device,format);
        let measurement=crate::measure_gpu::Measurement::new(&device,format);let guides=crate::guide_gpu::Guide::new(&device,format);
        let mut renderer = Self {opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, primitive, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy, size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode: Default::default() };
        for lane in [&mut renderer.mesh_ink,&mut renderer.strokes,&mut renderer.paths,&mut renderer.markers,&mut renderer.grid]{lane.enable_finite(&renderer.device,format);}
```

</details>

Replace it with:

```rust
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets=crate::sheet_gpu::Lane::new(&device,format);
        let measurement=crate::measure_gpu::Measurement::new(&device,format);let guides=crate::guide_gpu::Guide::new(&device,format);
        let mut renderer = Self {opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,own_slot,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, primitive, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy, size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode: Default::default() };
        for lane in [&mut renderer.mesh_ink,&mut renderer.strokes,&mut renderer.paths,&mut renderer.markers,&mut renderer.grid]{lane.enable_finite(&renderer.device,format);}
```

<details>
<summary>Locate this block</summary>

```rust
        let pipeline = mesh_pipeline(crate::depth::surface(),true);
        let tiles=std::cell::RefCell::new(crate::tile_gpu::Tiles::new(&device,&view_layout));
        let instances=crate::instance_gpu::Lane::new(&device,format,&view_layout);
        let highlight_pipeline = mesh_pipeline(crate::depth::ink(),false);
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
```

</details>

Replace it with:

```rust
        let pipeline = mesh_pipeline(crate::depth::surface(),true);
        let tiles=std::cell::RefCell::new(crate::tile_gpu::Tiles::new(&device,&view_layout));
        let instances=crate::instance_gpu::Lane::new(&device,format,&view_layout);
        let own_slot=crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(&device,&queue));
        let highlight_pipeline = mesh_pipeline(crate::depth::ink(),false);
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
```

<details>
<summary>Locate this block</summary>

```rust
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
```

</details>

Replace it with:

```rust
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[crate::draw_slots::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
```

<details>
<summary>Locate this block</summary>

```rust
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    instances:crate::instance_gpu::Lane,
    instance_rows:std::collections::HashSet<ObjectId>,
    pipeline: wgpu::RenderPipeline,
    highlight_pipeline: wgpu::RenderPipeline,
```

</details>

Replace it with:

```rust
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    instances:crate::instance_gpu::Lane,
    own_slot:crate::draw_slots::Slots,
    instance_rows:std::collections::HashSet<ObjectId>,
    pipeline: wgpu::RenderPipeline,
    highlight_pipeline: wgpu::RenderPipeline,
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
    own_slot:crate::draw_slots::Slots,
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
        let instances=crate::instance_gpu::Lane::new(&device,format,&view_layout);
        let own_slot=crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(&device,&queue));
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
        let mut renderer = Self {opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,own_slot,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
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
            base=base.checked_add(mesh.geometry.index_count/3).filter(|n|*n<crate::primitive_image::CAP_START).expect("Bounded projected triangle range");
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
            self.own_slot.bind(&mut pass,0,0);
            for mesh in self.meshes.iter().filter(|row|self.opacity.get()<1.||!self.instance_rows.contains(&row.id)) {mesh.draw_pulled(&mut pass);}
            if self.opacity.get()>=1.{self.instances.draw(&mut pass);}
            pass.set_pipeline(&self.highlight_pipeline); pass.set_bind_group(0, &self.view_group, &[]);
            self.own_slot.bind(&mut pass,0,0);
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
--8<-- "journey/code/96zzv-bind-the-own-row-slot-for-ordinary-surfaces-7.rs"
```

```text
--8<-- "journey/code/96zzv-bind-the-own-row-slot-for-ordinary-surfaces-8.rs"
```

```text
--8<-- "journey/code/96zzv-bind-the-own-row-slot-for-ordinary-surfaces-9.rs"
```

```text
--8<-- "journey/code/96zzv-bind-the-own-row-slot-for-ordinary-surfaces-10.rs"
```

```text
--8<-- "journey/code/96zzv-bind-the-own-row-slot-for-ordinary-surfaces-11.rs"
```

```text
--8<-- "journey/code/96zzv-bind-the-own-row-slot-for-ordinary-surfaces-12.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/global_outline_tests.rs`

Supply actual ordinary/instance pixels, placed-face highlights, primitive preservation, original sources and permanent sentinel checks.

<details>
<summary>Locate the existing block</summary>

```text
));assert_eq!(gpu.meshes[0].packed(),before);}
 }
 assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzv-bind-the-own-row-slot-for-ordinary-surfaces-13.rs" skip=160
```

</details>

</details>

<a id="checkpoint-96zzw-reload-scenes-through-the-existing-publication-owner"></a>

### Step 3 of 24 · Reload scenes through the existing publication owner (`96zzw-reload-scenes-through-the-existing-publication-owner`)

- **Goal:** Expose scene replacement through the existing reader, including empty-manifest document release.
- **Data Flow Trace:** `reload_scene → publication owner → validated candidate → Live adoption`
- **Files You Touch:** `src/publication_route.rs` (*Share named-scene resolution between startup and explicit reload.*) · `src/browser_publication.rs` (*Keep one stable publication slot for browser callbacks.*) · `src/browser_runtime.rs` (*Export the current replacement entry point and start reading after the runtime borrow ends.*) · `src/reload_complete_tests.rs` (*Supply actual history, original-owner and manifest-cache release assertions.*)

The reader already adopts complete live scenes and clears their history. Add the current reload_scene entry point instead of another loader. Update its stable control, cancel obsolete work and begin the existing candidate transaction. An empty manifest releases the document.

> **Self-Check Question:** *Why start the read after releasing the runtime borrow?*  
> **Answer:** The publication checks which runtime owns its device. Starting it while that runtime is mutably borrowed would reenter the same RefCell.

- **Hands-On Lab Experiment:** Create `empty.yaml` containing `items: []` and temporarily add `<link data-trunk rel="copy-file" href="empty.yaml">` to `index.html`. Call `window.wasmBindings.reload_scene(new URL("empty.yaml",location.href).href)` in the browser console. Compare valid, malformed and empty manifests. Empty reload must clear the drawing and its Undo history.

<details>
<summary>Implementation 96zzw-reload-scenes-through-the-existing-publication-owner: Reload scenes through the existing publication owner</summary>

Expose scene replacement through the existing reader, including empty-manifest document release.

Data flow: reload_scene → publication owner → validated candidate → Live adoption.

### 1. `src/publication_route.rs`

Share named-scene resolution between startup and explicit reload. Reuse the existing data-base rules. Use the same path rule at startup without a second algorithm.

<details>
<summary>Locate the existing block</summary>

```rust
pub const DATA_BASE:&str="https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev/";
pub const DEFAULT_SOURCE:&str="https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev/scenes/view_live.yaml";

#[derive(Clone)]
pub struct Route {pub manifest:String,pub base:String,pub poll_ms:Option<i32>,pub notify:Option<String>}
pub fn local(url:&str)->bool {url.starts_with("http://localhost:")||url.starts_with("http://127.0.0.1:")||url.starts_with("http://[::1]:")}
pub fn safe_scene(value:&str)->bool {!value.is_empty()&&!value.starts_with('/')&&!value.contains("//")&&!value.contains(':')&&!value.split('/').any(|segment|segment=="..")}
pub fn path_scene(path:&str)->Option<&str> {let last=path.rsplit('/').next()?;(!last.is_empty()&&!last.ends_with(".html")&&!last.contains(':')&&!last.starts_with('.')).then_some(last)}
pub fn join(base:&str,file:&str)->String {if file.starts_with("https://")||file.starts_with("http://") {file.into()} else {format!("{base}{}",file.trim_start_matches("./"))}}
pub fn directory(url:&str)->String {url.rsplit_once('/').map_or_else(String::new,|(prefix,_)|format!("{prefix}/"))}

pub fn select(scene:Option<&str>,path:&str,data:Option<&str>,live:Option<&str>,poll:Option<&str>,notify:Option<&str>)->Option<Route> {
    let base=match data {Some("off"|"")=>String::new(),Some(value) if value.starts_with("https://")||local(value)=>{if value.ends_with('/') {value.into()} else {format!("{value}/")}},_=>DATA_BASE.into()};
    let named=scene.filter(|value|safe_scene(value)).or_else(||path_scene(path));
    let watched=match live {Some("off"|"0")=>None,Some(value) if value.starts_with("https://")||local(value)=>Some(value.to_owned()),Some(_)=>Some(DEFAULT_SOURCE.into()),None if named==Some("view_live")=>Some(DEFAULT_SOURCE.into()),_=>None};
    if let Some(manifest)=watched {
        let source_base=if !base.is_empty()&&manifest.starts_with(&base) {base} else {directory(&manifest)};
        let seconds=poll.and_then(|v|v.parse::<f64>().ok()).filter(|v|v.is_finite()&&*v>=1.).unwrap_or(5.);
        let notify=match notify {Some("off"|"0")=>None,Some(value) if value.starts_with("https://")||local(value)=>Some(value.into()),_=>None};
        return Some(Route {manifest,base:source_base,poll_ms:Some((seconds*1000.).min(i32::MAX as f64) as i32),notify});
    }
    let name=named?;let mut name=if name.contains('.') {name.to_owned()} else {format!("{name}.yaml")};if !name.contains('/') {name=format!("scenes/{name}");}
    Some(Route {manifest:join(&base,&name),base,poll_ms:None,notify:None})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn named_routes_keep_bucket_relative_files_and_refuse_parent_traversal() {
        assert!(!safe_scene("../outside"));assert!(!safe_scene("folder/../outside"));assert!(!safe_scene("https://other/scene"));assert!(!safe_scene("/scene"));
        let route=select(Some("view_floor"),"/",None,None,None,None).unwrap();assert_eq!(route.manifest,format!("{DATA_BASE}scenes/view_floor.yaml"));assert_eq!(join(&route.base,"pb/floor.pb"),format!("{DATA_BASE}pb/floor.pb"));
        let route=select(Some("fixtures/scene.yaml"),"/",Some("off"),Some("off"),None,None).unwrap();assert_eq!(route.manifest,"fixtures/scene.yaml");assert_eq!(join(&route.base,"./fixtures/a.pb"),"fixtures/a.pb");assert_eq!(path_scene("/view_floor"),Some("view_floor"));assert!(path_scene("/index.html").is_none());assert!(select(None,"/",Some("off"),None,None,None).is_none());
    }
    #[test]
    fn live_routes_keep_local_directories_poll_bounds_and_explicit_notification_urls() {
        let route=select(None,"/",Some("off"),Some("http://localhost:8770/live/scene.yaml"),Some("0.1"),Some("off")).unwrap();assert_eq!(route.base,"http://localhost:8770/live/");assert_eq!(route.poll_ms,Some(5000));assert!(route.notify.is_none());
        let route=select(Some("view_floor"),"/",None,Some("https://example.test/live.yaml"),Some("1"),Some("https://example.test/events")).unwrap();assert_eq!(route.poll_ms,Some(1000));assert_eq!(route.notify.as_deref(),Some("https://example.test/events"));assert_eq!(route.base,"https://example.test/");
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub const DATA_BASE:&str="https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev/";
pub const DEFAULT_SOURCE:&str="https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev/scenes/view_live.yaml";

#[derive(Clone)]
pub struct Route {pub manifest:String,pub base:String,pub poll_ms:Option<i32>,pub notify:Option<String>}
pub fn local(url:&str)->bool {url.starts_with("http://localhost:")||url.starts_with("http://127.0.0.1:")||url.starts_with("http://[::1]:")}
pub fn safe_scene(value:&str)->bool {!value.is_empty()&&!value.starts_with('/')&&!value.contains("//")&&!value.contains(':')&&!value.split('/').any(|segment|segment=="..")}
pub fn path_scene(path:&str)->Option<&str> {let last=path.rsplit('/').next()?;(!last.is_empty()&&!last.ends_with(".html")&&!last.contains(':')&&!last.starts_with('.')).then_some(last)}
pub fn join(base:&str,file:&str)->String {if file.starts_with("https://")||file.starts_with("http://") {file.into()} else {format!("{base}{}",file.trim_start_matches("./"))}}
pub fn directory(url:&str)->String {url.rsplit_once('/').map_or_else(String::new,|(prefix,_)|format!("{prefix}/"))}

fn data_base(data:Option<&str>)->String {match data {Some("off"|"")=>String::new(),Some(value) if value.starts_with("https://")||local(value)=>{if value.ends_with('/') {value.into()} else {format!("{value}/")}},_=>DATA_BASE.into()}}
pub fn named(path:&str,data:Option<&str>)->Route {
 let base=data_base(data);let mut path=if path.contains('.') {path.to_owned()} else {format!("{path}.yaml")};
 if !path.contains('/') {path=format!("scenes/{path}");}
 Route {manifest:join(&base,&path),base,poll_ms:None,notify:None}
}

pub fn select(scene:Option<&str>,path:&str,data:Option<&str>,live:Option<&str>,poll:Option<&str>,notify:Option<&str>)->Option<Route> {
    let base=data_base(data);
    let named=scene.filter(|value|safe_scene(value)).or_else(||path_scene(path));
    let watched=match live {Some("off"|"0")=>None,Some(value) if value.starts_with("https://")||local(value)=>Some(value.to_owned()),Some(_)=>Some(DEFAULT_SOURCE.into()),None if named==Some("view_live")=>Some(DEFAULT_SOURCE.into()),_=>None};
    if let Some(manifest)=watched {
        let source_base=if !base.is_empty()&&manifest.starts_with(&base) {base} else {directory(&manifest)};
        let seconds=poll.and_then(|v|v.parse::<f64>().ok()).filter(|v|v.is_finite()&&*v>=1.).unwrap_or(5.);
        let notify=match notify {Some("off"|"0")=>None,Some(value) if value.starts_with("https://")||local(value)=>Some(value.into()),_=>None};
        return Some(Route {manifest,base:source_base,poll_ms:Some((seconds*1000.).min(i32::MAX as f64) as i32),notify});
    }
    Some(self::named(named?,data))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn named_routes_keep_bucket_relative_files_and_refuse_parent_traversal() {
        assert!(!safe_scene("../outside"));assert!(!safe_scene("folder/../outside"));assert!(!safe_scene("https://other/scene"));assert!(!safe_scene("/scene"));
        let route=select(Some("view_floor"),"/",None,None,None,None).unwrap();assert_eq!(route.manifest,format!("{DATA_BASE}scenes/view_floor.yaml"));assert_eq!(join(&route.base,"pb/floor.pb"),format!("{DATA_BASE}pb/floor.pb"));
        let route=select(Some("fixtures/scene.yaml"),"/",Some("off"),Some("off"),None,None).unwrap();assert_eq!(route.manifest,"fixtures/scene.yaml");assert_eq!(join(&route.base,"./fixtures/a.pb"),"fixtures/a.pb");assert_eq!(path_scene("/view_floor"),Some("view_floor"));assert!(path_scene("/index.html").is_none());assert!(select(None,"/",Some("off"),None,None,None).is_none());
    }
    #[test]
    fn live_routes_keep_local_directories_poll_bounds_and_explicit_notification_urls() {
        let route=select(None,"/",Some("off"),Some("http://localhost:8770/live/scene.yaml"),Some("0.1"),Some("off")).unwrap();assert_eq!(route.base,"http://localhost:8770/live/");assert_eq!(route.poll_ms,Some(5000));assert!(route.notify.is_none());
        let route=select(Some("view_floor"),"/",None,Some("https://example.test/live.yaml"),Some("1"),Some("https://example.test/events")).unwrap();assert_eq!(route.poll_ms,Some(1000));assert_eq!(route.notify.as_deref(),Some("https://example.test/events"));assert_eq!(route.base,"https://example.test/");
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-1.rs"
```

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-2.rs"
```

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-3.rs"
```

</details>

### 2. `src/browser_publication.rs`

Keep one stable publication slot for browser callbacks. Let callback clones observe the current owner through the same slot. Retain a stable control slot even when startup has no scene route. Reuse one publication state constructor for startup and explicit replacement. Keep route-less reload calls as no-ops, matching the current entry point. Keep the existing callback identity when a scene is replaced. Read the latest state through the stable weak control slot. Cancel the old reader and move its replacement into the same slot.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    timer:Option<i32>,tick:Option<Closure<dyn FnMut()>>,
    notify:Option<web_sys::EventSource>,message:Option<Closure<dyn FnMut(web_sys::MessageEvent)>>,
}
pub struct Publication(Option<Rc<RefCell<State>>>);
#[derive(Clone)]
pub struct Control(Option<Weak<RefCell<State>>>);
impl Publication {
    pub fn new(document:crate::drawing_document::Shared,fault:crate::gpu_fault::Fault)->Result<Self,JsValue> {
        let window=web_sys::window().ok_or("No browser window")?;let url=web_sys::Url::new(&window.location().href()?)?;let q=url.search_params();
        let scene=q.get("scene");let data=q.get("data");let live=q.get("live");let poll=q.get("poll");let notify=q.get("notify");
        let Some(route)=crate::publication_route::select(scene.as_deref(),&window.location().pathname()?,data.as_deref(),live.as_deref(),poll.as_deref(),notify.as_deref()) else {return Ok(Self(None));};
        let index=Index::new(route.base.clone());Ok(Self(Some(Rc::new(RefCell::new(State {active:true,inflight:false,again:false,pending_sheets:Vec::new(),sheets:Vec::new(),pending_streams:Vec::new(),streams:Vec::new(),cancelled:Rc::new(Cell::new(false)),gate:Default::default(),abort:None,route,index:Some(index),document,fault,delivery:None,last_error:None,timer:None,tick:None,notify:None,message:None})))))
    }
    pub fn control(&self)->Control {Control(self.0.as_ref().map(Rc::downgrade))}
}
impl Drop for Publication {fn drop(&mut self) {self.control().cancel();}}
impl Control {
    fn state(&self)->Option<Rc<RefCell<State>>> {self.0.as_ref()?.upgrade()}
    pub fn begin(&self) {
        let Some(state)=self.state() else {return;};let Some(window)=web_sys::window() else {return;};
        let route=state.borrow().route.clone();
```

</details>

Replace it with:

```rust
    timer:Option<i32>,tick:Option<Closure<dyn FnMut()>>,
    notify:Option<web_sys::EventSource>,message:Option<Closure<dyn FnMut(web_sys::MessageEvent)>>,
}
type Slot=RefCell<Option<Rc<RefCell<State>>>>;
pub struct Publication(Rc<Slot>);
#[derive(Clone)]
pub struct Control(Weak<Slot>);
impl Publication {
    pub fn new(document:crate::drawing_document::Shared,fault:crate::gpu_fault::Fault)->Result<Self,JsValue> {
        let window=web_sys::window().ok_or("No browser window")?;let url=web_sys::Url::new(&window.location().href()?)?;let q=url.search_params();
        let scene=q.get("scene");let data=q.get("data");let live=q.get("live");let poll=q.get("poll");let notify=q.get("notify");
        let Some(route)=crate::publication_route::select(scene.as_deref(),&window.location().pathname()?,data.as_deref(),live.as_deref(),poll.as_deref(),notify.as_deref()) else {return Ok(Self(Rc::new(RefCell::new(None))));};
        Ok(Self::from_route(document,fault,route))
    }
    pub fn at(document:crate::drawing_document::Shared,fault:crate::gpu_fault::Fault,path:&str)->Result<Self,JsValue>{
        let window=web_sys::window().ok_or("No browser window")?;let url=web_sys::Url::new(&window.location().href()?)?;
        Ok(Self::from_route(document,fault,crate::publication_route::named(path,url.search_params().get("data").as_deref())))
    }
    fn from_route(document:crate::drawing_document::Shared,fault:crate::gpu_fault::Fault,route:Route)->Self{
        let index=Index::new(route.base.clone());Self(Rc::new(RefCell::new(Some(Rc::new(RefCell::new(State {active:true,inflight:false,again:false,pending_sheets:Vec::new(),sheets:Vec::new(),pending_streams:Vec::new(),streams:Vec::new(),cancelled:Rc::new(Cell::new(false)),gate:Default::default(),abort:None,route,index:Some(index),document,fault,delivery:None,last_error:None,timer:None,tick:None,notify:None,message:None}))))))
    }
    pub fn replace(&mut self,next:Self){
        self.control().cancel();*self.0.borrow_mut()=next.0.borrow_mut().take();
    }
    pub fn has_route(&self)->bool{self.0.borrow().is_some()}
    pub fn control(&self)->Control {Control(Rc::downgrade(&self.0))}
}
impl Drop for Publication {fn drop(&mut self) {self.control().cancel();}}
impl Control {
    fn state(&self)->Option<Rc<RefCell<State>>> {self.0.upgrade()?.borrow().clone()}
    pub fn begin(&self) {
        let Some(state)=self.state() else {return;};let Some(window)=web_sys::window() else {return;};
        let route=state.borrow().route.clone();
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{publication_index::Index,publication_route::Route};
use std::{cell::{Cell,RefCell},collections::HashSet,rc::{Rc,Weak}};
use wasm_bindgen::{JsCast,JsValue,closure::Closure};

struct State {
    active:bool,inflight:bool,again:bool,
    pending_sheets:Vec<crate::sheet_stream::Cursor>,sheets:Vec<Weak<crate::sheet_source::Prepared>>,
    pending_streams:Vec<crate::stream_load::Cursor>,streams:Vec<std::rc::Weak<crate::range_cloud::Prepared>>,cancelled:Rc<Cell<bool>>,
    gate:crate::read_gate::ReadGate,
    abort:Option<web_sys::AbortController>,
    route:Route,index:Option<Index>,
    document:crate::drawing_document::Shared,fault:crate::gpu_fault::Fault,
    delivery:Option<Result<Box<crate::scene::Scene>,String>>,last_error:Option<String>,
    timer:Option<i32>,tick:Option<Closure<dyn FnMut()>>,
    notify:Option<web_sys::EventSource>,message:Option<Closure<dyn FnMut(web_sys::MessageEvent)>>,
}
type Slot=RefCell<Option<Rc<RefCell<State>>>>;
pub struct Publication(Rc<Slot>);
#[derive(Clone)]
pub struct Control(Weak<Slot>);
impl Publication {
    pub fn new(document:crate::drawing_document::Shared,fault:crate::gpu_fault::Fault)->Result<Self,JsValue> {
        let window=web_sys::window().ok_or("No browser window")?;let url=web_sys::Url::new(&window.location().href()?)?;let q=url.search_params();
        let scene=q.get("scene");let data=q.get("data");let live=q.get("live");let poll=q.get("poll");let notify=q.get("notify");
        let Some(route)=crate::publication_route::select(scene.as_deref(),&window.location().pathname()?,data.as_deref(),live.as_deref(),poll.as_deref(),notify.as_deref()) else {return Ok(Self(Rc::new(RefCell::new(None))));};
        Ok(Self::from_route(document,fault,route))
    }
    pub fn at(document:crate::drawing_document::Shared,fault:crate::gpu_fault::Fault,path:&str)->Result<Self,JsValue>{
        let window=web_sys::window().ok_or("No browser window")?;let url=web_sys::Url::new(&window.location().href()?)?;
        Ok(Self::from_route(document,fault,crate::publication_route::named(path,url.search_params().get("data").as_deref())))
    }
    fn from_route(document:crate::drawing_document::Shared,fault:crate::gpu_fault::Fault,route:Route)->Self{
        let index=Index::new(route.base.clone());Self(Rc::new(RefCell::new(Some(Rc::new(RefCell::new(State {active:true,inflight:false,again:false,pending_sheets:Vec::new(),sheets:Vec::new(),pending_streams:Vec::new(),streams:Vec::new(),cancelled:Rc::new(Cell::new(false)),gate:Default::default(),abort:None,route,index:Some(index),document,fault,delivery:None,last_error:None,timer:None,tick:None,notify:None,message:None}))))))
    }
    pub fn replace(&mut self,next:Self){
        self.control().cancel();*self.0.borrow_mut()=next.0.borrow_mut().take();
    }
    pub fn has_route(&self)->bool{self.0.borrow().is_some()}
    pub fn control(&self)->Control {Control(Rc::downgrade(&self.0))}
}
impl Drop for Publication {fn drop(&mut self) {self.control().cancel();}}
impl Control {
    fn state(&self)->Option<Rc<RefCell<State>>> {self.0.upgrade()?.borrow().clone()}
    pub fn begin(&self) {
        let Some(state)=self.state() else {return;};let Some(window)=web_sys::window() else {return;};
        let route=state.borrow().route.clone();
        if let Some(ms)=route.poll_ms {
            let weak=Rc::downgrade(&state);let callback=Closure::<dyn FnMut()>::new(move ||{if let Some(state)=weak.upgrade() {poll(state);}});
            match window.set_interval_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(),ms) {Ok(id)=>{let mut state=state.borrow_mut();state.timer=Some(id);state.tick=Some(callback);},Err(error)=>crate::browser::report(&format!("Cannot schedule live polling: {error:?}"))}
            if let Some(url)=route.notify {if let Ok(source)=web_sys::EventSource::new(&url) {let weak=Rc::downgrade(&state);let callback=Closure::<dyn FnMut(web_sys::MessageEvent)>::new(move |_|{if let Some(state)=weak.upgrade() {poll(state);}});source.set_onmessage(Some(callback.as_ref().unchecked_ref()));let mut state=state.borrow_mut();state.notify=Some(source);state.message=Some(callback);}}
        }
        poll(state);
    }
    pub fn cancel(&self) {
        let Some(state)=self.state() else {return;};let mut state=state.borrow_mut();state.active=false;state.cancelled.set(true);for source in state.streams.drain(..).filter_map(|s|s.upgrade()) {source.generation.borrow_mut().close();}for source in state.sheets.drain(..).filter_map(|s|s.upgrade()) {if let Some(stream)=&source.streamed {stream.generation.borrow_mut().close();}}state.pending_sheets.clear();state.pending_streams.clear();crate::stream::reset_range_gate();state.gate.cancel();state.delivery=None;state.index=None;
        if let Some(abort)=state.abort.take() {abort.abort();}if let Some(id)=state.timer.take() {if let Some(window)=web_sys::window() {window.clear_interval_with_handle(id);}}
        if let Some(source)=state.notify.take() {source.set_onmessage(None);source.close();}state.message=None;state.tick=None;
    }
    pub fn take(&self)->Result<Option<crate::editor::Action>,String> {
        let Some(state)=self.state() else {return Ok(None);};let mut owner=state.borrow_mut();if !owner.active {owner.delivery=None;return Ok(None);}
        let scene=owner.delivery.take().transpose()?;
        if let Some(next)=scene.as_ref() {
            owner.streams.retain(|weak|weak.upgrade().is_some_and(|source| {if next.streams().iter().any(|row|Rc::ptr_eq(&row.prepared,&source)) {true} else {source.generation.borrow_mut().close();crate::range_fetch::cancel_url(&source.source.url);false}}));
            owner.sheets.retain(|weak|weak.upgrade().is_some_and(|source| {if next.sheets().iter().any(|row|Rc::ptr_eq(&row.prepared,&source)) {true} else {if let Some(stream)=&source.streamed {stream.generation.borrow_mut().close();crate::range_fetch::cancel_url(&stream.source.url);}false}}));
            let sheet_cursors=std::mem::take(&mut owner.pending_sheets);
            let cursors=std::mem::take(&mut owner.pending_streams);let cancelled=owner.cancelled.clone();drop(owner);
            for cursor in sheet_cursors {let weak=Rc::downgrade(&state);let current=Rc::new(move |source:&Rc<crate::sheet_source::Prepared>|weak.upgrade().is_some_and(|state| {let owner=state.borrow();owner.active&&crate::browser_runtime::owns(&owner.fault)&&owner.fault.message().is_none()&&owner.document.borrow().scene.sheets().iter().any(|row|Rc::ptr_eq(&row.prepared,source))}));let changed=Rc::new(|| {if let Some(window)=web_sys::window() {if let Ok(event)=web_sys::Event::new("viewer-stream-page") {let _=window.dispatch_event(&event);}}});let error=Rc::new(|message:String|crate::browser::report(&format!("Drawing page read stopped: {message}")));crate::sheet_stream::continue_read(cursor,current,changed,error,cancelled.clone());}
            for cursor in cursors {
                let weak=Rc::downgrade(&state);let current=Rc::new(move |source:&Rc<crate::range_cloud::Prepared>|weak.upgrade().is_some_and(|state| {let owner=state.borrow();owner.active&&crate::browser_runtime::owns(&owner.fault)&&owner.fault.message().is_none()&&owner.document.borrow().scene.streams().iter().any(|row|Rc::ptr_eq(&row.prepared,source))}));
                let changed=Rc::new(|| {if let Some(window)=web_sys::window() {if let Ok(event)=web_sys::Event::new("viewer-stream-page") {let _=window.dispatch_event(&event);}}});
                let error=Rc::new(|message:String|crate::browser::report(&format!("Source page read stopped: {message}")));
                crate::stream_load::continue_read(cursor,current,changed,error,cancelled.clone());
            }
        }
        Ok(scene.map(crate::editor::Action::Live))
    }

}

fn poll(state:Rc<RefCell<State>>) {
    let mut owner=state.borrow_mut();if !owner.active||!crate::browser_runtime::owns(&owner.fault)||owner.fault.message().is_some() {return;}
    if owner.inflight {owner.again=true;return;}
    let Ok(abort)=web_sys::AbortController::new() else {return;};let Ok(ticket)=owner.gate.begin() else {return;};let Some(mut index)=owner.index.take() else {return;};
    owner.inflight=true;owner.abort=Some(abort.clone());let route=owner.route.clone();drop(owner);
    wasm_bindgen_futures::spawn_local(async move {
        let result=read(&route,&mut index,&abort.signal()).await;
        let mut owner=state.borrow_mut();let current=owner.active&&owner.gate.finish(ticket)&&crate::browser_runtime::owns(&owner.fault)&&owner.fault.message().is_none();
        owner.inflight=false;owner.abort=None;if !current {return;}
        let outcome=result.and_then(|(cursors,sheets)| {for cursor in &sheets {owner.sheets.push(Rc::downgrade(&cursor.prepared));}owner.pending_sheets.extend(sheets);for cursor in &cursors {owner.streams.push(Rc::downgrade(&cursor.prepared));}owner.pending_streams.extend(cursors);index.replacement(&owner.document.borrow().scene)});owner.index=Some(index);
        let changed=match outcome {Ok(Some(scene))=>{owner.last_error=None;owner.delivery=Some(Ok(Box::new(scene)));true},Ok(None)=>false,Err(error)=>{if owner.last_error.as_ref()==Some(&error) {false} else {let _=crate::browser_report::observe("publication",&error);owner.last_error=Some(error.clone());owner.delivery=Some(Err(error));true}}};
        let again=std::mem::take(&mut owner.again);drop(owner);
        if changed {if let Some(window)=web_sys::window() {if let Ok(event)=web_sys::Event::new("viewer-publication") {let _=window.dispatch_event(&event);}}}
        if again {poll(state);}
    });
}

async fn read(route:&Route,index:&mut Index,signal:&web_sys::AbortSignal)->Result<(Vec<crate::stream_load::Cursor>,Vec<crate::sheet_stream::Cursor>),String> {
    let mut sheets=Vec::new();let mut segments=index.streamed_segments();let mut cursors=Vec::new();let mut points=index.streamed_points();let budget=crate::page_cache::Budget::new(crate::stream_budget::PAGE_BYTES);
    let started=crate::browser_phase::now();let mut response=crate::publication_fetch::get(&route.manifest,index.etag(&route.manifest),4*1024*1024,signal).await?;
    if response.status==404&&route.manifest.ends_with(".toml") {let yaml=format!("{}.yaml",route.manifest.trim_end_matches(".toml"));response=crate::publication_fetch::get(&yaml,index.etag(&route.manifest),4*1024*1024,signal).await?;}
    match response.status {304 if index.has_manifest()=>{},200..=299=>{index.manifest(&route.manifest,response.etag,&response.bytes)?;crate::browser_phase::finish("manifest",started,response.bytes.len() as u64,&route.manifest);},status=>return Err(format!("Manifest HTTP {status}"))}
    let mut seen=HashSet::new();
    for url in index.urls() {
        if !seen.insert(url.clone())||crate::manifest::immutable_key(&url)&&index.cached(&url) {continue;}
        let encoding=index.encoding(&url)?;
        if encoding.name.is_empty()&&url.split('?').next().is_some_and(|path|path.ends_with(".pb")) {
            if let Some(probe)=crate::stream::probe(&url).await {
                if crate::sheet_range::identity(&probe.bytes).is_some() {
                    let source=crate::sheet_range::Source::from_probe(url.clone(),&probe).await?;
                    if source.fields.count>crate::sheet_stream::PREFIX {
                        if index.cached(&url)&&source.fields.revision.is_some()&&index.etag(&url)==source.fields.revision {continue;}if signal.aborted() {return Err("Drawing source read cancelled".into());}segments=segments.saturating_sub(index.source_segments(&url));let remaining=crate::sheet_stream::CEILING.saturating_sub(segments);let cursor=crate::sheet_stream::start(Rc::new(source),budget.clone(),remaining).await?;if signal.aborted() {cursor.prepared.streamed.as_ref().unwrap().generation.borrow_mut().close();return Err("Drawing source read cancelled".into());}segments=segments.saturating_add(cursor.from);index.stream_sheet(&url,cursor.prepared.clone())?;sheets.push(cursor);continue;
                    }
                }
                if crate::range_shape::single_cloud(&probe.bytes)&&crate::stream::cloud_identity(&probe.bytes).is_some() {
                    let source=crate::range_source::Source::from_probe(url.clone(),&probe).await?;
                    if source.fields.count>crate::stream_budget::PREFIX_POINTS||source.fields.coords_len>=crate::stream_budget::MIN_BYTES {
                        if index.cached(&url)&&source.fields.revision.is_some()&&index.etag(&url)==source.fields.revision {continue;}
                        if signal.aborted() {return Err("Source read cancelled".into());}
                        points=points.saturating_sub(index.source_points(&url));
                        let remaining=crate::stream_budget::MAX_POINTS.saturating_sub(points);
                        if remaining==0 {return Err("Manifest sources exceed the point ceiling".into());}
                        let cursor=crate::stream_load::start(Rc::new(source),budget.clone(),remaining).await?;
                        if signal.aborted() {cursor.prepared.generation.borrow_mut().close();return Err("Source read cancelled".into());}
                        points=points.saturating_add(cursor.loaded);index.stream(&url,cursor.prepared.clone())?;cursors.push(cursor);continue;
                    }
                }
            }
        }
        let response=crate::publication_fetch::get(&url,index.etag(&url),crate::archive::LIMIT,signal).await?;
        match response.status {
            304 if index.cached(&url)=>{},
            200..=299=>{let started=crate::browser_phase::now();let bytes=crate::source_unpack::decode(response.bytes,&encoding,signal).await?;index.source(&url,response.etag,&bytes)?;crate::browser_phase::finish("decode/validation/display",started,bytes.len() as u64,&url);},
            status=>{index.forget(&url);return Err(format!("Source HTTP {status}: {url}"));}
        }
    }
    let extra=crate::stream_budget::MAX_POINTS.saturating_sub(points)/(cursors.len().max(1) as u32);for cursor in &mut cursors {cursor.ceiling=cursor.loaded+extra;}
    let extra=crate::sheet_stream::CEILING.saturating_sub(segments)/(sheets.len().max(1) as u32);for cursor in &mut sheets {cursor.ceiling=cursor.from.saturating_add(extra);}
    Ok((cursors,sheets))
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-4.rs"
```

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-5.rs"
```

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-6.rs"
```

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-7.rs"
```

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-8.rs"
```

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-9.rs"
```

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-10.rs"
```

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-11.rs"
```

</details>

### 3. `src/browser_runtime.rs`

Export the current replacement entry point and start reading after the runtime borrow ends.

<details>
<summary>Locate the existing block</summary>

```rust
    ACTIVE.with(|slot| slot.borrow_mut().take()).is_some()
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn runtime_running() -> bool { ACTIVE.with(|slot| slot.borrow().is_some()) }
```

</details>

After that block, type:

```rust

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn reload_scene(url:Option<String>)->Result<(),wasm_bindgen::JsValue>{
    let control=ACTIVE.with(|slot| {
        let mut active=slot.borrow_mut();let runtime=active.as_mut().ok_or("Viewer runtime is not running")?;
        let publication=match url {Some(path)=>crate::browser_publication::Publication::at(runtime.document.clone(),runtime.fault.clone(),&path)?,None=>crate::browser_publication::Publication::new(runtime.document.clone(),runtime.fault.clone())?};
        if !publication.has_route(){return Ok(None);}
        runtime.request.borrow_mut().cancel();runtime.reload.borrow_mut().cancel();
        runtime._publication.replace(publication);let control=runtime._publication.control();
        Ok::<_,wasm_bindgen::JsValue>(Some(control))
    })?;
    if let Some(control)=control{control.begin();}Ok(())
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-12.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/reload_complete_tests.rs`

Supply actual history, original-owner and manifest-cache release assertions.

<details>
<summary>Locate the existing block</summary>

```text
ly.complete(&mut e).unwrap_err(), "Network failed");
    assert_eq!(e.scene.objects()[0].model.m, model); assert!(e.scene.objects()[0].geometry().is_none());
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-13.rs" skip=160
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/publication_route.rs`

Supply current absolute, nested and bare named-scene routing checks.

<details>
<summary>Locate the existing block</summary>

```text
t_eq!(route.poll_ms,Some(1000));assert_eq!(route.notify.as_deref(),Some("https://example.test/events"));assert_eq!(route.base,"https://example.test/");
    }
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-14.rs" skip=160
```

</details>

</details>

<a id="checkpoint-96zzx-update-only-changed-slot-runs"></a>

### Step 4 of 24 · Update only changed slot runs (`96zzx-update-only-changed-slot-runs`)

- **Goal:** Patch changed instance slots without rewriting their unchanged neighbours.
- **Data Flow Trace:** `changed rows → contiguous runs → queue.write_buffer → unchanged draw slots`
- **Files You Touch:** `src/text_gpu_support.rs` (*Write aligned existing rows without changing capacity or the active count.*) · `src/draw_slots.rs` (*Report the bytes actually uploaded by this slot update.*) · `src/instance_gpu.rs` (*Count partial slot uploads rather than every slot in the batch.*) · `src/draw_slots_tests.rs` (*Supply real GPU readback checks for partial writes, unchanged rows, growth, shrink and bounds.*)

A selection change can update one slot while its shared geometry remains untouched. Add bounded writes to the existing growing buffer. Compare slot rows, group adjacent changes and upload those runs. Keep the sentinel and reuse capacity when the active count shrinks.

> **Self-Check Question:** *Why compare rows before writing the buffer?*  
> **Answer:** An unchanged row needs no upload. Grouping adjacent changes also avoids one queue write per changed row.

- **Hands-On Lab Experiment:** Select and deselect one placed object. Its drawing changes without uploading its source geometry. Repeating the same selection should add no slot writes.

<details>
<summary>Implementation 96zzx-update-only-changed-slot-runs: Update only changed slot runs</summary>

Patch changed instance slots without rewriting their unchanged neighbours.

Data flow: changed rows → contiguous runs → queue.write_buffer → unchanged draw slots.

### 1. `src/text_gpu_support.rs`

Write aligned existing rows without changing capacity or the active count.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn append<const N:usize>(&mut self,ctx:&GpuCtx,vertices:&[[f32;N]]) {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn write_bytes_at(&self,ctx:&GpuCtx,at:u32,bytes:&[u8]) {
        assert_eq!(bytes.len()as u64%self.stride,0);
        let rows=bytes.len()as u64/self.stride;
        assert!(u64::from(at)+rows<=u64::from(self.len));
        if !bytes.is_empty(){ctx.queue.write_buffer(&self.buf,u64::from(at)*self.stride,bytes);}
    }

    pub fn append<const N:usize>(&mut self,ctx:&GpuCtx,vertices:&[[f32;N]]) {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzx-update-only-changed-slot-runs-1.rs"
```

</details>

### 2. `src/draw_slots.rs`

Report the bytes actually uploaded by this slot update. Initialize upload accounting. Patch changed contiguous runs; append new rows and retain capacity on shrink.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::text_gpu_support::{GpuCtx,GrowBuf};
pub const OWN_ROW:u32=u32::MAX;
pub fn sentinel_shader()->String{format!("const OWN_ROW:u32={OWN_ROW}u;")}
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

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::text_gpu_support::{GpuCtx,GrowBuf};
pub const OWN_ROW:u32=u32::MAX;
pub fn sentinel_shader()->String{format!("const OWN_ROW:u32={OWN_ROW}u;")}
pub type Slot=[u32;2];
const ATTRIBUTES:[wgpu::VertexAttribute;2]=[
 wgpu::VertexAttribute {offset:0,shader_location:0,format:wgpu::VertexFormat::Uint32},
 wgpu::VertexAttribute {offset:4,shader_location:1,format:wgpu::VertexFormat::Uint32},
];
pub fn layout()->wgpu::VertexBufferLayout<'static>{wgpu::VertexBufferLayout {array_stride:8,step_mode:wgpu::VertexStepMode::Instance,attributes:&ATTRIBUTES}}
pub struct Slots {pub(crate) buffer:GrowBuf,previous:Vec<Slot>,pub written_bytes:u64}
impl Slots {
 pub fn new(ctx:&GpuCtx)->Self {
  let mut slots=Self {buffer:GrowBuf::new(ctx,"instance draw slots",8,wgpu::BufferUsages::VERTEX),previous:Vec::new(),written_bytes:0};
  slots.set(ctx,&[]);slots
 }
 pub fn set(&mut self,ctx:&GpuCtx,rows:&[Slot])->bool{
  self.written_bytes=0;
  let bytes=|rows:&[Slot]|rows.iter().flatten().flat_map(|v|v.to_ne_bytes()).collect::<Vec<_>>();
  if self.buffer.len()==0 {
   self.buffer.append_bytes(ctx,&bytes(&[[OWN_ROW,0]]));self.written_bytes=8;
  }
  let inside=self.previous.len().min(rows.len());let mut at=0;
  while at<inside {
   if self.previous[at]==rows[at]{at+=1;continue;}
   let first=at;while at<inside&&self.previous[at]!=rows[at]{at+=1;}
   self.buffer.write_bytes_at(ctx,(first+1)as u32,&bytes(&rows[first..at]));
   self.written_bytes+=(at-first)as u64*8;
  }
  if rows.len()>self.previous.len(){
   let tail=bytes(&rows[inside..]);self.buffer.append_bytes(ctx,&tail);self.written_bytes+=tail.len()as u64;
  }else if rows.len()<self.previous.len(){
   self.buffer.reset();self.buffer.append_bytes(ctx,&bytes(&[[OWN_ROW,0]]));self.buffer.append_bytes(ctx,&bytes(rows));
   self.written_bytes=(rows.len()as u64+1)*8;
  }
  self.previous.clear();self.previous.extend_from_slice(rows);self.written_bytes>0
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
--8<-- "journey/code/96zzx-update-only-changed-slot-runs-2.rs"
```

```text
--8<-- "journey/code/96zzx-update-only-changed-slot-runs-3.rs"
```

```text
--8<-- "journey/code/96zzx-update-only-changed-slot-runs-4.rs"
```

</details>

### 3. `src/instance_gpu.rs`

Count partial slot uploads rather than every slot in the batch.

<details>
<summary>Locate the existing block</summary>

```rust
                    if batch.slots.set(&ctx,&slots){self.writes+=1;self.written_bytes+=(slots.len()as u64+1)*8;}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
                    if batch.slots.set(&ctx,&slots){self.writes+=1;self.written_bytes+=batch.slots.written_bytes;}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzx-update-only-changed-slot-runs-5.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/draw_slots_tests.rs`

Supply real GPU readback checks for partial writes, unchanged rows, growth, shrink and bounds.

<details>
<summary>Locate the existing block</summary>

```text
(slots.instances(),0);assert_eq!(slots.allocated_bytes(),8);
 assert_eq!(crate::growing_buffer_tests::read(&ctx,&slots.buffer.buf,8),bytes(&[[u32::MAX,0]]));
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzx-update-only-changed-slot-runs-6.rs" skip=160
```

</details>

</details>

<a id="checkpoint-96zzy-share-one-slot-buffer-across-surface-draws"></a>

### Step 5 of 24 · Share one slot buffer across surface draws (`96zzy-share-one-slot-buffer-across-surface-draws`)

- **Goal:** Bind ordinary surfaces and every instance batch from one slot buffer.
- **Data Flow Trace:** `one slot owner → sentinel or batch range → shared surface vertex entry`
- **Files You Touch:** `src/instance_gpu.rs` (*Store a range start instead of another slot owner.*) · `src/renderer.rs` (*Remove the duplicate ordinary slot owner.*) · `src/global_outline_tests.rs` (*Retain the mapped sentinel and released-scene checks against the shared owner.*)

Each batch currently owns another slot buffer. Move those slots into one lane owner. Keep the sentinel at row zero and record each batch’s starting row. Bind that range for placed draws and row zero for ordinary surfaces and temporary highlights.

> **Self-Check Question:** *Why keep each batch’s object rows local?*  
> **Answer:** Its storage binding still contains that batch’s records. A global slot offset chooses the draw range; the row stored in that slot chooses the local record.

- **Hands-On Lab Experiment:** Draw two different shared definitions, select one placed original and switch opacity. The ordinary and instanced paths must preserve original ownership and restore the same pixels.

<details>
<summary>Implementation 96zzy-share-one-slot-buffer-across-surface-draws: Share one slot buffer across surface draws</summary>

Bind ordinary surfaces and every instance batch from one slot buffer.

Data flow: one slot owner → sentinel or batch range → shared surface vertex entry.

### 1. `src/instance_gpu.rs`

Store a range start instead of another slot owner. Own the single surface slot buffer in the lane. Initialize the single sentinel owner. Use the current device queue for the initial sentinel. Keep the concatenated slot order identical to the ordered draw batches. Remove the unused group owner. Gather each batch into the same slot table. Use one queue context after gathering the batches. Remember the first shared slot; row zero remains reserved. Retain the batch’s range alongside its current records. Remove per-batch slot allocation and duplicate sentinels. Upload the shared rows once and release scene capacity when no batches remain. Bind each batch’s shared slot range. Let ordinary draws read the same sentinel owner. Count the shared scene allocation once; the empty sentinel remains a static pipeline resource.

<details>
<summary>Locate the existing block</summary>

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
  .replace("return object_settings;","return objects[index];");
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
                let slots:Vec<crate::draw_slots::Slot>=records.iter().enumerate().map(|(row,record)|[row as u32,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]).collect();
                let key=(source,part);let bytes=records.iter().flatten().copied().collect::<Vec<_>>();
                let batch=if let Some(mut batch)=old.remove(&key).filter(|b|Rc::ptr_eq(&b.geometry,&geometry)&&b.records.size()>=bytes.len() as u64) {
                    if batch.slots.set(&ctx,&slots){self.writes+=1;self.written_bytes+=batch.slots.written_bytes;}
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

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh};
use std::{collections::BTreeMap,rc::Rc};
use wgpu::util::DeviceExt;
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,records:wgpu::Buffer,group:wgpu::BindGroup,bytes:Vec<u8>,count:u32,first_slot:u32}
pub struct Lane {pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<(usize,usize),Batch>,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn source()->String {
 let source=include_str!("triangle.wgsl")
  .replace("var<uniform> object_settings: ObjectSettings;","var<storage, read> objects: array<ObjectSettings>;")
  .replace("return object_settings;","return objects[index];");
 crate::clipping_gpu::source(&source,2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("shared instance records"),entries:&[wgpu::BindGroupLayoutEntry {binding:0,visibility:wgpu::ShaderStages::VERTEX,ty:wgpu::BindingType::Buffer {ty:wgpu::BufferBindingType::Storage {read_only:true},has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(BYTES as u64)},count:None}]});
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(format.into()),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        Self {pipeline,layout,batches:BTreeMap::new(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=(u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size)/BYTES as u64) as usize;
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:BTreeMap<usize,(Rc<GpuGeometry>,Vec<Record>)>=BTreeMap::new();
        for row in rows {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push(row.packed());}
        let mut old=std::mem::take(&mut self.batches);let mut batches=BTreeMap::new();let mut all_slots=Vec::new();
        for (source,(geometry,records)) in groups {
            for (part,records) in records.chunks(limit).enumerate() {
                let slots:Vec<crate::draw_slots::Slot>=records.iter().enumerate().map(|(row,record)|[row as u32,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]).collect();
                let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;all_slots.extend_from_slice(&slots);
                let key=(source,part);let bytes=records.iter().flatten().copied().collect::<Vec<_>>();
                let batch=if let Some(mut batch)=old.remove(&key).filter(|b|Rc::ptr_eq(&b.geometry,&geometry)&&b.records.size()>=bytes.len() as u64) {
                    batch.first_slot=first_slot;
                    if batch.bytes!=bytes {queue.write_buffer(&batch.records,0,&bytes);self.writes+=1;self.written_bytes+=bytes.len() as u64;batch.bytes=bytes;}batch.count=records.len() as u32;batch
                } else {
                    let buffer=device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:Some("independent source placements"),contents:&bytes,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST});let group=crate::gpu_layout::group(device,&self.layout,&buffer);self.writes+=1;self.written_bytes+=bytes.len() as u64;Batch {geometry:geometry.clone(),records:buffer,group,bytes,count:records.len() as u32,first_slot}
                };batches.insert(key,batch);
            }
        }
        let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
        if batches.is_empty()&&!old.is_empty(){self.slots.clear(&ctx,true);}else{self.slots.set(&ctx,&all_slots);}
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;Ok(())
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        for batch in self.batches.values() {pass.set_bind_group(1,&batch.group,&[]);self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.batches.values().map(|b|b.records.size()).sum::<u64>()+if self.batches.is_empty(){0}else{self.slots.allocated_bytes()},self.writes,self.written_bytes]}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzy-share-one-slot-buffer-across-surface-draws-1.rs"
```

```text
--8<-- "journey/code/96zzy-share-one-slot-buffer-across-surface-draws-2.rs"
```

```text
--8<-- "journey/code/96zzy-share-one-slot-buffer-across-surface-draws-3.rs"
```

```text
--8<-- "journey/code/96zzy-share-one-slot-buffer-across-surface-draws-4.rs"
```

```text
--8<-- "journey/code/96zzy-share-one-slot-buffer-across-surface-draws-5.rs"
```

```text
--8<-- "journey/code/96zzy-share-one-slot-buffer-across-surface-draws-6.rs"
```

```text
--8<-- "journey/code/96zzy-share-one-slot-buffer-across-surface-draws-7.rs"
```

```text
--8<-- "journey/code/96zzy-share-one-slot-buffer-across-surface-draws-9.rs"
```

```text
--8<-- "journey/code/96zzy-share-one-slot-buffer-across-surface-draws-10.rs"
```

```text
--8<-- "journey/code/96zzy-share-one-slot-buffer-across-surface-draws-11.rs"
```

```text
--8<-- "journey/code/96zzy-share-one-slot-buffer-across-surface-draws-12.rs"
```

```text
--8<-- "journey/code/96zzy-share-one-slot-buffer-across-surface-draws-13.rs"
```

```text
--8<-- "journey/code/96zzy-share-one-slot-buffer-across-surface-draws-14.rs"
```

```text
--8<-- "journey/code/96zzy-share-one-slot-buffer-across-surface-draws-15.rs"
```

</details>

### 2. `src/renderer.rs`

Remove the duplicate ordinary slot owner. Pass the current queue to the lane. Keep only the lane’s sentinel. Construct one surface slot owner. Rebind the shared sentinel before ordinary surfaces or temporary highlights.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            pass.set_bind_group(2,&clip_group,&[]);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.own_slot.bind(&mut pass,0,0);
            for mesh in self.meshes.iter().filter(|row|self.opacity.get()<1.||!self.instance_rows.contains(&row.id)) {mesh.draw_pulled(&mut pass);}
            if self.opacity.get()>=1.{self.instances.draw(&mut pass);}
            pass.set_pipeline(&self.highlight_pipeline); pass.set_bind_group(0, &self.view_group, &[]);
            self.own_slot.bind(&mut pass,0,0);
            self.highlight.mesh(&mut pass);
        }
        self.caps.borrow_mut().draw(&mut encoder,view,&self.primitive.view,&self.depth,&self.view_group,&clip_group,&self.meshes,if self.clipping.borrow().enabled{self.clipping.borrow().rows.len()as u32}else{0});
```

</details>

Replace it with:

```rust
            pass.set_bind_group(2,&clip_group,&[]);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            for mesh in self.meshes.iter().filter(|row|self.opacity.get()<1.||!self.instance_rows.contains(&row.id)) {mesh.draw_pulled(&mut pass);}
            if self.opacity.get()>=1.{self.instances.draw(&mut pass);}
            pass.set_pipeline(&self.highlight_pipeline); pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            self.highlight.mesh(&mut pass);
        }
        self.caps.borrow_mut().draw(&mut encoder,view,&self.primitive.view,&self.depth,&self.view_group,&clip_group,&self.meshes,if self.clipping.borrow().enabled{self.clipping.borrow().rows.len()as u32}else{0});
```

<details>
<summary>Locate this block</summary>

```rust
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets=crate::sheet_gpu::Lane::new(&device,format);
        let measurement=crate::measure_gpu::Measurement::new(&device,format);let guides=crate::guide_gpu::Guide::new(&device,format);
        let mut renderer = Self {opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,own_slot,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, primitive, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy, size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode: Default::default() };
        for lane in [&mut renderer.mesh_ink,&mut renderer.strokes,&mut renderer.paths,&mut renderer.markers,&mut renderer.grid]{lane.enable_finite(&renderer.device,format);}
```

</details>

Replace it with:

```rust
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets=crate::sheet_gpu::Lane::new(&device,format);
        let measurement=crate::measure_gpu::Measurement::new(&device,format);let guides=crate::guide_gpu::Guide::new(&device,format);
        let mut renderer = Self {opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, primitive, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy, size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode: Default::default() };
        for lane in [&mut renderer.mesh_ink,&mut renderer.strokes,&mut renderer.paths,&mut renderer.markers,&mut renderer.grid]{lane.enable_finite(&renderer.device,format);}
```

<details>
<summary>Locate this block</summary>

```rust
        });
        let pipeline = mesh_pipeline(crate::depth::surface(),true);
        let tiles=std::cell::RefCell::new(crate::tile_gpu::Tiles::new(&device,&view_layout));
        let instances=crate::instance_gpu::Lane::new(&device,format,&view_layout);
        let own_slot=crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(&device,&queue));
        let highlight_pipeline = mesh_pipeline(crate::depth::ink(),false);
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
```

</details>

Replace it with:

```rust
        });
        let pipeline = mesh_pipeline(crate::depth::surface(),true);
        let tiles=std::cell::RefCell::new(crate::tile_gpu::Tiles::new(&device,&view_layout));
        let instances=crate::instance_gpu::Lane::new(&device,&queue,format,&view_layout);
        let highlight_pipeline = mesh_pipeline(crate::depth::ink(),false);
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
```

<details>
<summary>Locate this block</summary>

```rust
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    instances:crate::instance_gpu::Lane,
    own_slot:crate::draw_slots::Slots,
    instance_rows:std::collections::HashSet<ObjectId>,
    pipeline: wgpu::RenderPipeline,
    highlight_pipeline: wgpu::RenderPipeline,
```

</details>

Replace it with:

```rust
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    instances:crate::instance_gpu::Lane,
    instance_rows:std::collections::HashSet<ObjectId>,
    pipeline: wgpu::RenderPipeline,
    highlight_pipeline: wgpu::RenderPipeline,
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
            base=base.checked_add(mesh.geometry.index_count/3).filter(|n|*n<crate::primitive_image::CAP_START).expect("Bounded projected triangle range");
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
            self.instances.bind_own(&mut pass);
            for mesh in self.meshes.iter().filter(|row|self.opacity.get()<1.||!self.instance_rows.contains(&row.id)) {mesh.draw_pulled(&mut pass);}
            if self.opacity.get()>=1.{self.instances.draw(&mut pass);}
            pass.set_pipeline(&self.highlight_pipeline); pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
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
--8<-- "journey/code/96zzy-share-one-slot-buffer-across-surface-draws-17.rs"
```

```text
--8<-- "journey/code/96zzy-share-one-slot-buffer-across-surface-draws-19.rs"
```

```text
--8<-- "journey/code/96zzy-share-one-slot-buffer-across-surface-draws-20.rs"
```

```text
--8<-- "journey/code/96zzy-share-one-slot-buffer-across-surface-draws-21.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/global_outline_tests.rs`

Retain the mapped sentinel and released-scene checks against the shared owner.

<details>
<summary>Locate the existing block</summary>

```text
 let mut scene=Scene::empty();crate::scene_document::import(&mut scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=[crate::draw_slots::OWN_ROW,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();assert_eq!(gpu.own_slot.allocated_bytes(),8);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.own_slot.buffer.buf,8),bytes);let originals=scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>();let owner=scene.objects()[0].id;
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzy-share-one-slot-buffer-across-surface-draws-22.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/global_outline_tests.rs`

Retain the mapped sentinel and released-scene checks against the shared owner.

<details>
<summary>Locate the existing block</summary>

```text
 assert_eq!(scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>(),originals);gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.own_slot.allocated_bytes(),8);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.own_slot.buffer.buf,8),bytes);
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzy-share-one-slot-buffer-across-surface-draws-23.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/global_outline_tests.rs`

Supply actual two-definition GPU drawing, mapped range, selection and release checks.

<details>
<summary>Locate the existing block</summary>

```text
ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzy-share-one-slot-buffer-across-surface-draws-24.rs" skip=160
```

</details>

</details>

<a id="checkpoint-96zzz-prepare-the-integer-primitive-slot-texture"></a>

### Step 6 of 24 · Prepare the integer primitive-slot texture (`96zzz-prepare-the-integer-primitive-slot-texture`)

- **Goal:** Store slot lookup records in an integer texture with bounded partial uploads.
- **Data Flow Trace:** `slot index → texture row and column → four exact integer words`
- **Files You Touch:** `src/primitive_slot_table.rs` (*Prepare one RGBA32Uint lookup owner, full replacement and bounded row writes.*) · `src/lib.rs` (*Register the lookup resource without changing the existing draw path.*) · `src/primitive_slot_table_tests.rs` (*Supply actual integer texture readback, row-crossing, bounds and release checks.*)

The vertex buffer selects placements. A lookup texture will let fragment passes find their original row and triangle interval. Prepare four integer words per texel, a fixed row width and partial writes that split at texture-row boundaries. The drawing remains unchanged until this table is connected.

> **Self-Check Question:** *Why split a partial upload at each texture-row boundary?*  
> **Answer:** A rectangular texture write cannot wrap from the end of one row to the beginning of the next. Each run must fit its destination row.

- **Hands-On Lab Experiment:** Predict the columns and rows for slots 1023, 1024 and 1025 using index % WIDTH and index / WIDTH. Trace a three-texel upload starting at 1023. Keep the existing scene running: this preparatory table does not change its drawing.

<details>
<summary>Implementation 96zzz-prepare-the-integer-primitive-slot-texture: Prepare the integer primitive-slot texture</summary>

Store slot lookup records in an integer texture with bounded partial uploads.

Data flow: slot index → texture row and column → four exact integer words.

### 1. `src/primitive_slot_table.rs`

Prepare one RGBA32Uint lookup owner, full replacement and bounded row writes.

Create the file and type:

```rust
--8<-- "journey/code/96zzz-prepare-the-integer-primitive-slot-texture-1.rs"
```

### 2. `src/lib.rs`

Register the lookup resource without changing the existing draw path.

<details>
<summary>Locate the existing block</summary>

```rust
mod draw_slots;
```

</details>

After that block, type:

```rust
mod primitive_slot_table;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzz-prepare-the-integer-primitive-slot-texture-2.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/primitive_slot_table_tests.rs`

Supply actual integer texture readback, row-crossing, bounds and release checks.

Copy this supplied file:

```text
--8<-- "journey/code/96zzz-prepare-the-integer-primitive-slot-texture-3.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/lib.rs`

Register the optional GPU validation fixture.

<details>
<summary>Locate the existing block</summary>

```text
#[cfg(test)] mod draw_slots_tests;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzz-prepare-the-integer-primitive-slot-texture-4.rs" skip=34
```

</details>

</details>

<a id="checkpoint-96zzza-describe-slot-and-triangle-runs"></a>

### Step 7 of 24 · Describe slot and triangle runs (`96zzza-describe-slot-and-triangle-runs`)

- **Goal:** Represent reserved slot capacity separately from occupied members and triangle IDs.
- **Data Flow Trace:** `definition → reserved slots and IDs → used members`
- **Files You Touch:** `src/slot_space.rs` (*Describe slot and ID capacity, definitions and change reports.*) · `src/lib.rs` (*Register the preparatory allocator types.*)

A definition needs slots for its placed members and IDs for their triangles. Describe both ranges in one run. Keep used length separate from capacity, and represent free room without a definition key. Reuse the existing sentinel and slot type.

> **Self-Check Question:** *Why separate used length from capacity?*  
> **Answer:** A small definition can gain members without moving its existing slots when reserved capacity remains.

- **Hands-On Lab Experiment:** Sketch a run with capacity four, two occupied members and three triangles per member. Mark the occupied slots separately from their reserved triangle-ID interval. The browser draw path is unchanged.

<details>
<summary>Implementation 96zzza-describe-slot-and-triangle-runs: Describe slot and triangle runs</summary>

Represent reserved slot capacity separately from occupied members and triangle IDs.

Data flow: definition → reserved slots and IDs → used members.

### 1. `src/slot_space.rs`

Describe slot and ID capacity, definitions and change reports.

Create the file and type:

```rust
--8<-- "journey/code/96zzza-describe-slot-and-triangle-runs-1.rs"
```

### 2. `src/lib.rs`

Register the preparatory allocator types.

<details>
<summary>Locate the existing block</summary>

```rust
mod primitive_slot_table;
```

</details>

After that block, type:

```rust
mod slot_space;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzza-describe-slot-and-triangle-runs-2.rs"
```

</details>

</details>

<a id="checkpoint-96zzzb-read-slot-runs-and-lookup-texels"></a>

### Step 8 of 24 · Read slot runs and lookup texels (`96zzzb-read-slot-runs-and-lookup-texels`)

- **Goal:** Derive GPU slot values and lookup texels from the same CPU run.
- **Data Flow Trace:** `CPU run → vertex slot and fragment lookup texel`
- **Files You Touch:** `src/slot_space.rs` (*Build this day’s whole allocator methods on the preceding types.*)

Read the row and triangle offset for a slot. Derive the four-word lookup texel from that same run, with zero triangles for unused room. Report draws in definition order and merge overlapping changed ranges.

> **Self-Check Question:** *Why does an unused slot have zero triangles?*  
> **Answer:** Reserved capacity is not a live member. Fragment lookup must not treat its spare triangle IDs as drawing.

- **Hands-On Lab Experiment:** Reuse the [run sketch](../../96zzza-describe-slot-and-triangle-runs.md). Read one occupied slot and one spare slot, then derive their lookup texels. The spare slot must describe zero live triangles.

<details>
<summary>Implementation 96zzzb-read-slot-runs-and-lookup-texels: Read slot runs and lookup texels</summary>

Derive GPU slot values and lookup texels from the same CPU run.

Data flow: CPU run → vertex slot and fragment lookup texel.

### 1. `src/slot_space.rs`

Build this day’s whole allocator methods on the preceding types.

<details>
<summary>Locate the existing block</summary>

```rust
    (!self.all).then(|| self.slots.iter().map(|range| range.len() as u32).sum())
  }
}
#[derive(Default)]
pub struct SlotSpace {
  rows: Vec<u32>,
  runs: Vec<Run>,
  base: u32,
  arena: u32,
  pub written: u32,
}
```

</details>

After that block, type:

```rust

impl SlotSpace {
  pub fn end(&self) -> u32 {
    self.runs.last().map_or(1, |run| run.start + run.cap)
  }
  pub fn triangle_count(&self) -> Option<u32> {
    let last = self.runs.last()?;
    let any = self.runs.iter().any(|run| run.key.is_some() && run.len > 0);
    any.then(|| self.base + last.ids + last.span)
  }
  pub fn draws(&self) -> Vec<Draw> {
    let mut held: Vec<&Run> = Vec::new();
    for run in self
      .runs
      .iter()
      .filter(|run| run.key.is_some() && run.len > 0)
    {
      held.insert(held.partition_point(|other| other.key < run.key), run);
    }
    held.iter()
      .map(|run| Draw {
        slots: run.start..run.start + run.len,
        ..run.draw.clone()
      })
      .collect()
  }
  fn run_at(&self, slot: u32) -> usize {
    self.runs.partition_point(|run| run.start + run.cap <= slot)
  }
  pub fn slot(&self, slot: u32) -> Slot {
    let run = &self.runs[self.run_at(slot)];
    let start = self.base + run.ids + (slot - run.start) * run.triangles;
    [self.rows[slot as usize], start.wrapping_sub(run.first)]
  }
  pub fn texel(&self, slot: u32) -> [u32; 4] {
    if slot == 0 {
      return [self.end() - 1, self.arena, 0, 0];
    }
    let run = &self.runs[self.run_at(slot)];
    let j = slot - run.start;
    let start = self.base + run.ids + j * run.triangles;
    let count = if run.key.is_some() && j < run.len {
      run.triangles
    } else {
      0
    };
    [self.rows[slot as usize], start, run.first, count]
  }
}
fn shared_draw(definition: &Definition) -> Draw {
  Draw {
    faces: definition.faces.clone(),
    pipes: definition.pipes.clone(),
    ribbons: definition.ribbons.clone(),
    slots: 0..0,
  }
}
fn coalesce(ranges: Vec<Range<u32>>, end: u32) -> Vec<Range<u32>> {
  let mut sorted: Vec<Range<u32>> = Vec::with_capacity(ranges.len());
  for range in ranges {
    sorted.insert(sorted.partition_point(|r| r.start <= range.start), range);
  }
  let mut out: Vec<Range<u32>> = Vec::with_capacity(sorted.len());
  for range in sorted {
    let range = range.start.min(end)..range.end.min(end);
    if range.is_empty() {
      continue;
    }
    match out.last_mut() {
      Some(last) if range.start <= last.end => last.end = last.end.max(range.end),
      _ => out.push(range),
    }
  }
  out
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzb-read-slot-runs-and-lookup-texels-1.rs"
```

</details>

</details>

<a id="checkpoint-96zzzc-reuse-and-merge-free-slot-ranges"></a>

### Step 9 of 24 · Reuse and merge free slot ranges (`96zzzc-reuse-and-merge-free-slot-ranges`)

- **Goal:** Allocate fitting holes, retain unused room and merge released neighbours.
- **Data Flow Trace:** `free range → fitting allocation → retained remainder`
- **Files You Touch:** `src/slot_space.rs` (*Build this day’s whole allocator methods on the preceding types.*)

Find the first free run with enough slots and triangle IDs. Split its unused tail instead of discarding it. Fill the occupied run from its definition, then join neighbouring free runs and remove unused room at the end.

> **Self-Check Question:** *Why check both slot capacity and triangle span?*  
> **Answer:** Equal member counts can refer to definitions with different triangle counts. A hole must fit both resources.

- **Hands-On Lab Experiment:** Release two adjacent runs, merge their free intervals and allocate a smaller definition into the hole. Check both member capacity and triangle span; retain the unused remainder.

<details>
<summary>Implementation 96zzzc-reuse-and-merge-free-slot-ranges: Reuse and merge free slot ranges</summary>

Allocate fitting holes, retain unused room and merge released neighbours.

Data flow: free range → fitting allocation → retained remainder.

### 1. `src/slot_space.rs`

Build this day’s whole allocator methods on the preceding types.

<details>
<summary>Locate the existing block</summary>

```rust
    }
    match out.last_mut() {
      Some(last) if range.start <= last.end => last.end = last.end.max(range.end),
      _ => out.push(range),
    }
  }
  out
}
```

</details>

After that block, type:

```rust

impl SlotSpace {
  fn hold(&mut self, at: usize, definition: &Definition) -> Range<u32> {
    let run = &mut self.runs[at];
    let len = definition.rows.len();
    run.key = Some(definition.key);
    run.len = len as u32;
    run.first = definition.faces.start / 3;
    run.triangles = definition.faces.len() as u32 / 3;
    run.draw = shared_draw(definition);
    let (start, end) = (run.start as usize, (run.start + run.cap) as usize);
    if self.rows.len() < end {
      self.rows.resize(end, 0);
    }
    self.rows[start..start + len].copy_from_slice(definition.rows);
    start as u32..end as u32
  }
  fn allocate(&mut self, cap: u32, triangles: u32) -> usize {
    let span = cap * triangles;
    let fits = |run: &Run| run.key.is_none() && run.cap >= cap && run.span >= span;
    let Some(at) = self.runs.iter().position(fits) else {
      let (start, ids) = self
        .runs
        .last()
        .map_or((1, 0), |run| (run.start + run.cap, run.ids + run.span));
      self.runs.push(Run::free(start, cap, ids, span));
      return self.runs.len() - 1;
    };
    let hole = self.runs[at].clone();
    if hole.cap > cap {
      self.runs[at] = Run::free(hole.start, cap, hole.ids, span);
      let rest = Run::free(
        hole.start + cap,
        hole.cap - cap,
        hole.ids + span,
        hole.span - span,
      );
      self.runs.insert(at + 1, rest);
    }
    at
  }
  fn merge_free(&mut self) {
    let mut merged: Vec<Run> = Vec::with_capacity(self.runs.len());
    for run in self.runs.drain(..) {
      match merged.last_mut() {
        Some(last) if last.key.is_none() && run.key.is_none() => {
          last.cap += run.cap;
          last.span += run.span;
        }
        _ => merged.push(run),
      }
    }
    if merged.last().is_some_and(|run| run.key.is_none()) {
      merged.pop();
    }
    self.runs = merged;
    self.rows.truncate(self.end() as usize);
  }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzc-reuse-and-merge-free-slot-ranges-1.rs"
```

</details>

</details>

<a id="checkpoint-96zzzd-follow-arena-growth-with-slot-headroom"></a>

### Step 10 of 24 · Follow arena growth with slot headroom (`96zzzd-follow-arena-growth-with-slot-headroom`)

- **Goal:** Preserve instance IDs while arena growth remains inside reserved headroom.
- **Data Flow Trace:** `arena count → reserved headroom → header update or renumber`
- **Files You Touch:** `src/slot_space.rs` (*Build this day’s whole allocator methods on the preceding types.*)

A load or compaction places every definition again. Later arena growth should change only the lookup header while it fits before existing instance IDs. Renumber when the arena crosses that space, and retire runs whose definition or shared faces changed.

> **Self-Check Question:** *Why avoid renumbering on every arena append?*  
> **Answer:** Changing all instance IDs would also invalidate their lookup records. Headroom keeps small growth local.

- **Hands-On Lab Experiment:** Append geometry inside reserved headroom and compare every existing slot ID. Then cross the headroom boundary and trace the required growth. This preparation still leaves browser drawing unchanged.

<details>
<summary>Implementation 96zzzd-follow-arena-growth-with-slot-headroom: Follow arena growth with slot headroom</summary>

Preserve instance IDs while arena growth remains inside reserved headroom.

Data flow: arena count → reserved headroom → header update or renumber.

### 1. `src/slot_space.rs`

Build this day’s whole allocator methods on the preceding types.

<details>
<summary>Locate the existing block</summary>

```rust
    if merged.last().is_some_and(|run| run.key.is_none()) {
      merged.pop();
    }
    self.runs = merged;
    self.rows.truncate(self.end() as usize);
  }
}
```

</details>

After that block, type:

```rust

impl SlotSpace {
  fn place_all(&mut self, definitions: &[Definition], base: u32) -> Change {
    self.runs.clear();
    self.rows.clear();
    self.rows.push(OWN_ROW);
    self.base = base;
    for definition in definitions {
      let len = definition.rows.len() as u32;
      if len > 0 {
        let at = self.allocate(len, definition.faces.len() as u32 / 3);
        self.hold(at, definition);
      }
    }
    Change::all()
  }
  pub fn reset(&mut self, definitions: &[Definition], arena: u32) -> Change {
    self.arena = arena;
    self.place_all(definitions, arena)
  }
  pub fn follow(&mut self, arena: u32) -> Change {
    if arena == self.arena {
      return Change::default();
    }
    self.arena = arena;
    if self.runs.is_empty() {
      self.base = arena;
      return Change::default();
    }
    if arena <= self.base && self.base - arena <= 2 * headroom(arena) {
      return Change {
        all: false,
        slots: vec![0..1],
      };
    }
    let held: Vec<(u32, Vec<u32>, Draw)> = self
      .runs
      .iter()
      .filter_map(|run| {
        let rows = self.rows[run.start as usize..(run.start + run.len) as usize].to_vec();
        Some((run.key?, rows, run.draw.clone()))
      })
      .collect();
    let definitions: Vec<Definition> = held
      .iter()
      .map(|(key, rows, draw)| Definition {
        key: *key,
        rows,
        faces: draw.faces.clone(),
        pipes: draw.pipes.clone(),
        ribbons: draw.ribbons.clone(),
      })
      .collect();
    self.place_all(&definitions, arena + headroom(arena))
  }
  fn retire(&mut self,definitions:&[Definition],live:&HashMap<u32,usize>)->Vec<Range<u32>> {
    let mut dirty=vec![0..1];
    for run in &mut self.runs {
      let Some(key) = run.key else {
        continue;
      };
      let same = live.get(&key).is_some_and(|&i| {
        let faces = &definitions[i].faces;
        faces.start / 3 == run.first && faces.len() as u32 / 3 == run.triangles
      });
      if !same {
        dirty.push(run.start..run.start + run.cap);
        *run = run.freed();
      }
    }
    dirty
  }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzd-follow-arena-growth-with-slot-headroom-1.rs"
```

</details>

</details>

<a id="checkpoint-96zzze-report-only-changed-instance-slots"></a>

### Step 11 of 24 · Report only changed instance slots (`96zzze-report-only-changed-instance-slots`)

- **Goal:** Update touched members and growing definitions while reporting bounded dirty ranges.
- **Data Flow Trace:** `touched members → reused runs → coalesced dirty ranges`
- **Files You Touch:** `src/slot_space.rs` (*Build this day’s whole allocator methods on the preceding types.*)

Connect the allocator helpers. Retire obsolete runs, patch touched members and reuse capacity for definitions that still fit. Move full runs into new room and compact excessive spare space. Return only the slot ranges that need GPU updates.

> **Self-Check Question:** *Why report changed ranges instead of every slot?*  
> **Answer:** The GPU owner can update existing rows and texels in place. Unchanged definitions retain their allocations and uploads.

- **Hands-On Lab Experiment:** Trace a member joining and leaving a definition. Identify the changed slots and any retained spare range. The existing drawing remains unchanged: the allocator is prepared here and connected to GPU ownership later.

<details>
<summary>Implementation 96zzze-report-only-changed-instance-slots: Report only changed instance slots</summary>

Update touched members and growing definitions while reporting bounded dirty ranges.

Data flow: touched members → reused runs → coalesced dirty ranges.

### 1. `src/slot_space.rs`

Build this day’s whole allocator methods on the preceding types.

<details>
<summary>Locate the existing block</summary>

```rust
        faces.start / 3 == run.first && faces.len() as u32 / 3 == run.triangles
      });
      if !same {
        dirty.push(run.start..run.start + run.cap);
        *run = run.freed();
      }
    }
    dirty
  }
}
```

</details>

After that block, type:

```rust

impl SlotSpace {
  pub fn update(
    &mut self,
    definitions: &[Definition],
    touched: &[(u32, u32)],
    arena: u32,
  ) -> Change {
    if self.follow(arena).all {
      return self.place_all(definitions, self.base);
    }
    let mut members: Vec<u32> = Vec::new();
    let live: HashMap<u32, usize> = (0..definitions.len())
      .map(|i| (definitions[i].key, i))
      .collect();
    let mut dirty=self.retire(definitions,&live);
    let held: HashMap<u32, usize> = self
      .runs
      .iter()
      .filter_map(|run| Some((run.key?, run.start as usize)))
      .collect();
    for &(key, position) in touched {
      let (Some(&start), Some(&i)) = (held.get(&key), live.get(&key)) else {
        continue;
      };
      let run = &self.runs[self.run_at(start as u32)];
      let rows = definitions[i].rows;
      if position < run.cap && (position as usize) < rows.len() {
        let slot = run.start + position;
        self.rows[slot as usize] = rows[position as usize];
        members.push(slot);
      }
    }
    for definition in definitions {
      let len = definition.rows.len() as u32;
      let grow = held
        .get(&definition.key)
        .map(|&start| self.run_at(start as u32));
      if let Some(at) = grow {
        let run = &mut self.runs[at];
        if len <= run.cap {
          if len < run.len {
            dirty.push(run.start + len..run.start + run.len);
          }
          run.len = len;
          run.draw = shared_draw(definition);
          continue;
        }
        dirty.push(run.start..run.start + run.cap);
        *run = run.freed();
      }
      if len == 0 {
        continue;
      }
      let triangles = definition.faces.len() as u32 / 3;
      let cap = if grow.is_some() {
        grown_cap(len, triangles)
      } else {
        len
      };
      let at = self.allocate(cap, triangles);
      let run = self.hold(at, definition);
      dirty.push(run);
    }
    self.merge_free();
    let (mut free, mut used, mut free_ids, mut used_ids) = (0, 0, 0u64, 0u64);
    for run in &self.runs {
      let ids = u64::from(run.len) * u64::from(run.triangles);
      free += run.cap - run.len;
      used += run.len;
      free_ids += u64::from(run.span) - ids;
      used_ids += ids;
    }
    let spare_ids = u64::from(SPARE_IDS) * self.runs.len() as u64;
    if free > used + 256 || free_ids > used_ids / 2 + spare_ids {
      return self.place_all(definitions, self.base);
    }
    members.sort_unstable();
    members.dedup();
    dirty.extend(
      members
        .chunk_by(|a, b| a + 1 == *b)
        .map(|run| run[0]..run[0] + run.len() as u32),
    );
    Change {
      all: false,
      slots: coalesce(dirty, self.end()),
    }
  }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzze-report-only-changed-instance-slots-1.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/slot_space.rs`

Supply the five original allocator fixtures, retaining their allocation and changed-range assertions.

<details>
<summary>Locate the existing block</summary>

```text
a + 1 == *b)
        .map(|run| run[0]..run[0] + run.len() as u32),
    );
    Change {
      all: false,
      slots: coalesce(dirty, self.end()),
    }
  }
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzze-report-only-changed-instance-slots-2.rs" skip=160
```

</details>

</details>

<a id="checkpoint-96zzzf-project-triangles-at-their-physical-addresses"></a>

### Step 12 of 24 · Project triangles at their physical addresses (`96zzzf-project-triangles-at-their-physical-addresses`)

- **Goal:** Keep finite triangle records at the same addresses written into the primitive image.
- **Data Flow Trace:** `object primitive base → projected record interval → finite tile references`
- **Files You Touch:** `src/gpu_mesh.rs` (*Expose the existing address owner rather than deriving another base.*) · `src/tile_gpu.rs` (*Invalidate projection when physical addresses change.*) · `src/global_outline_tests.rs` (*Supply actual mapped gap, relocated record, cache, pixel restoration and device-budget checks.*)

The color pass writes each object’s primitive base, while projection still assumes dense rows. Read that same base when placing projected triangles. Bound the largest interval by the device budget, include addresses in the cache key and clear unused records before projection.

> **Self-Check Question:** *Why clear records when an interval moves?*  
> **Answer:** The buffer retains capacity. An old record in a newly unused address could otherwise keep contributing to tile lists.

- **Hands-On Lab Experiment:** Keep the existing original scene running and compare its outlines before and after rebuilding. Its dense addresses still produce the same drawing; the supplied GPU fixture also checks gaps, address changes and bounded fallback.

<details>
<summary>Implementation 96zzzf-project-triangles-at-their-physical-addresses: Project triangles at their physical addresses</summary>

Keep finite triangle records at the same addresses written into the primitive image.

Data flow: object primitive base → projected record interval → finite tile references.

### 1. `src/gpu_mesh.rs`

Expose the existing address owner rather than deriving another base.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn set_primitive_base(&mut self,queue:&wgpu::Queue,base:u32)->Result<(),&'static str>{
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn primitive_base(&self)->u32{self.primitive_base}
    pub fn set_primitive_base(&mut self,queue:&wgpu::Queue,base:u32)->Result<(),&'static str>{
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzf-project-triangles-at-their-physical-addresses-1.rs"
```

</details>

### 2. `src/tile_gpu.rs`

Invalidate projection when physical addresses change. Budget the largest physical interval, including gaps. Retain the base in the same geometry/model witness. Remove the second address accumulator. Write the object’s physical start into its projection metadata. Retain one address owner. Zero inactive records before projecting the active intervals.

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

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{gpu_mesh::GpuMesh,tile_math::{TileLayout,PROJECTED_BYTES,projected_records,next_pool_words,project_groups},tile_pipelines::{Pipelines,group},tile_report::PoolReport};
use wgpu::util::DeviceExt;
type Witness=(wgpu::Buffer,wgpu::Buffer,wgpu::Buffer,[u8;64],u32);
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
  let count=meshes.iter().filter(|row|row.geometry.index_count>0).try_fold(0u32,|end,row|row.primitive_base().checked_add(row.geometry.index_count/3).map(|next|end.max(next)));
  self.fallback=count.is_none_or(|count|u64::from(count)*PROJECTED_BYTES>limit);
  if self.fallback{self.rows.clear();self.witness.clear();self.triangles=0;self.key=None;return}
  let witness=meshes.iter().filter(|row|row.geometry.index_count>0).map(|row|{let(vertices,indices)=row.geometry.buffers();(vertices,indices,row.model_buffer().clone(),row.packed()[..64].try_into().unwrap(),row.primitive_base())}).collect::<Vec<_>>();
  if witness==self.witness{return}
  self.rows.clear();
  for row in meshes.iter().filter(|row|row.geometry.index_count>0){let triangles=row.geometry.index_count/3;
   let bytes=[triangles,row.primitive_base(),0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();
   let count=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("finite original triangle interval"),contents:&bytes,usage:wgpu::BufferUsages::UNIFORM});
   self.rows.push(Row{vertices:row.geometry.buffers().0,indices:row.geometry.buffers().1,model:row.model_buffer().clone(),count,triangles,group:None});
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
  encoder.clear_buffer(&self.projected,0,None);
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
--8<-- "journey/code/96zzzf-project-triangles-at-their-physical-addresses-2.rs"
```

```text
--8<-- "journey/code/96zzzf-project-triangles-at-their-physical-addresses-3.rs"
```

```text
--8<-- "journey/code/96zzzf-project-triangles-at-their-physical-addresses-4.rs"
```

```text
--8<-- "journey/code/96zzzf-project-triangles-at-their-physical-addresses-5.rs"
```

```text
--8<-- "journey/code/96zzzf-project-triangles-at-their-physical-addresses-6.rs"
```

```text
--8<-- "journey/code/96zzzf-project-triangles-at-their-physical-addresses-7.rs"
```

```text
--8<-- "journey/code/96zzzf-project-triangles-at-their-physical-addresses-8.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/global_outline_tests.rs`

Supply actual mapped gap, relocated record, cache, pixel restoration and device-budget checks.

<details>
<summary>Locate the existing block</summary>

```text
eline);gpu.set_scene(&crate::scene::Scene::empty(),None);assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzf-project-triangles-at-their-physical-addresses-9.rs" skip=160
```

</details>

</details>

<a id="checkpoint-96zzzg-clear-the-current-arctic-background"></a>

### Step 13 of 24 · Clear the current Arctic background (`96zzzg-clear-the-current-arctic-background`)

- **Goal:** Use the current Arctic background without another draw or geometry upload.
- **Data Flow Trace:** `Arctic setting → existing colour attachment clear → sRGB target → stored background pixel`
- **Files You Touch:** `src/renderer.rs` (*Use the existing colour clear; no extra fullscreen pipeline is needed.*) · `src/ambient_gpu_tests.rs` (*Keep the native UNORM contact-shadow assertion below the linear grey background, so the background itself cannot satisfy it.*)

Change the existing render-pass clear to linear grey 0.94 when Arctic is enabled. Keep the ordinary background when it is disabled. The sRGB target converts that linear value when storing pixels; do not put its already encoded byte value into the clear colour.

> **Self-Check Question:** *Why does a clear colour of 0.94 not store byte 240 in an sRGB target?*  
> **Answer:** Clear values are linear. The sRGB attachment encodes them before storage, giving about byte 248. Encoding twice would give the wrong shade.

- **What You Should See:** Arctic uses the current slightly grey background; ordinary display returns to its previous background.
- **Hands-On Lab Experiment:** Toggle Arctic at both display densities. Check a background pixel, then confirm that object identities, placements and geometry uploads remain unchanged.

<details>
<summary>Implementation 96zzzg-clear-the-current-arctic-background: Clear the current Arctic background</summary>

Use the current Arctic background without another draw or geometry upload.

Data flow: Arctic setting → existing colour attachment clear → sRGB target → stored background pixel.

### 1. `src/renderer.rs`

Use the existing colour clear; no extra fullscreen pipeline is needed.

<details>
<summary>Locate the existing block</summary>

```rust
        let [r, g, b] = background.rgb();
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        let [r, g, b] = if self.ambient_enabled.get() { [0.94; 3] } else { background.rgb() };
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzg-clear-the-current-arctic-background-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/ambient_gpu_tests.rs`

Keep the native UNORM contact-shadow assertion below the linear grey background, so the background itself cannot satisfy it.

<details>
<summary>Locate the existing block</summary>

```text
a[..3]==[255,255,255]&&b[..3].iter().any(|v|*v<250)
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzg-clear-the-current-arctic-background-02.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/ambient_gpu_tests.rs`

Measure surface darkening only on geometry pixels; the changed background is not ambient shading.

<details>
<summary>Locate the existing block</summary>

```text
filter(|(a,b)|a[..3].iter().zip(&b[..3]).any
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzg-clear-the-current-arctic-background-03.rs"
```

</details>

</details>

<a id="checkpoint-96zzzh-share-changed-record-runs"></a>

### Step 14 of 24 · Share changed record runs (`96zzzh-share-changed-record-runs`)

- **Goal:** Reuse the existing contiguous-change scan for slot and object records.
- **Data Flow Trace:** `previous records → changed contiguous runs → existing bounded buffer writes`
- **Files You Touch:** `src/draw_slots.rs` (*Extract the existing comparison, retaining its order.*)

Move the slot comparison into one generic helper. Keep the existing sentinel offset, tail append, shrink and release behavior. The next object table will use this same scan instead of teaching a second upload algorithm.

> **Self-Check Question:** *Why stop the comparison at the shorter slice?*  
> **Answer:** Only records present in both slices can be updated in place. Existing append and shrink paths handle the remaining records.

- **What You Should See:** The browser scene is unchanged. Slot writes reuse one changed-run scan.
- **Hands-On Lab Experiment:** Repeat the existing instance selection and Undo exercise. Selection changes only the changed slot bytes; drawing and original IDs stay the same.

<details>
<summary>Implementation 96zzzh-share-changed-record-runs: Share changed record runs</summary>

Reuse the existing contiguous-change scan for slot and object records.

Data flow: previous records → changed contiguous runs → existing bounded buffer writes.

### 1. `src/draw_slots.rs`

Extract the existing comparison, retaining its order. Use one changed-run owner; keep slot zero outside the scan.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::text_gpu_support::{GpuCtx,GrowBuf};
pub const OWN_ROW:u32=u32::MAX;
pub fn sentinel_shader()->String{format!("const OWN_ROW:u32={OWN_ROW}u;")}
pub type Slot=[u32;2];
const ATTRIBUTES:[wgpu::VertexAttribute;2]=[
 wgpu::VertexAttribute {offset:0,shader_location:0,format:wgpu::VertexFormat::Uint32},
 wgpu::VertexAttribute {offset:4,shader_location:1,format:wgpu::VertexFormat::Uint32},
];
pub fn layout()->wgpu::VertexBufferLayout<'static>{wgpu::VertexBufferLayout {array_stride:8,step_mode:wgpu::VertexStepMode::Instance,attributes:&ATTRIBUTES}}
pub struct Slots {pub(crate) buffer:GrowBuf,previous:Vec<Slot>,pub written_bytes:u64}
impl Slots {
 pub fn new(ctx:&GpuCtx)->Self {
  let mut slots=Self {buffer:GrowBuf::new(ctx,"instance draw slots",8,wgpu::BufferUsages::VERTEX),previous:Vec::new(),written_bytes:0};
  slots.set(ctx,&[]);slots
 }
 pub fn set(&mut self,ctx:&GpuCtx,rows:&[Slot])->bool{
  self.written_bytes=0;
  let bytes=|rows:&[Slot]|rows.iter().flatten().flat_map(|v|v.to_ne_bytes()).collect::<Vec<_>>();
  if self.buffer.len()==0 {
   self.buffer.append_bytes(ctx,&bytes(&[[OWN_ROW,0]]));self.written_bytes=8;
  }
  let inside=self.previous.len().min(rows.len());let mut at=0;
  while at<inside {
   if self.previous[at]==rows[at]{at+=1;continue;}
   let first=at;while at<inside&&self.previous[at]!=rows[at]{at+=1;}
   self.buffer.write_bytes_at(ctx,(first+1)as u32,&bytes(&rows[first..at]));
   self.written_bytes+=(at-first)as u64*8;
  }
  if rows.len()>self.previous.len(){
   let tail=bytes(&rows[inside..]);self.buffer.append_bytes(ctx,&tail);self.written_bytes+=tail.len()as u64;
  }else if rows.len()<self.previous.len(){
   self.buffer.reset();self.buffer.append_bytes(ctx,&bytes(&[[OWN_ROW,0]]));self.buffer.append_bytes(ctx,&bytes(rows));
   self.written_bytes=(rows.len()as u64+1)*8;
  }
  self.previous.clear();self.previous.extend_from_slice(rows);self.written_bytes>0
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
pub fn sentinel_shader()->String{format!("const OWN_ROW:u32={OWN_ROW}u;")}
pub type Slot=[u32;2];
const ATTRIBUTES:[wgpu::VertexAttribute;2]=[
 wgpu::VertexAttribute {offset:0,shader_location:0,format:wgpu::VertexFormat::Uint32},
 wgpu::VertexAttribute {offset:4,shader_location:1,format:wgpu::VertexFormat::Uint32},
];
pub fn layout()->wgpu::VertexBufferLayout<'static>{wgpu::VertexBufferLayout {array_stride:8,step_mode:wgpu::VertexStepMode::Instance,attributes:&ATTRIBUTES}}
pub(crate) fn changed_runs<T:PartialEq>(previous:&[T],next:&[T])->Vec<std::ops::Range<usize>> {
 let end=previous.len().min(next.len());let mut at=0;let mut runs=Vec::new();
 while at<end {
  if previous[at]==next[at]{at+=1;continue;}
  let first=at;while at<end&&previous[at]!=next[at]{at+=1;}
  runs.push(first..at);
 }
 runs
}
pub struct Slots {pub(crate) buffer:GrowBuf,previous:Vec<Slot>,pub written_bytes:u64}
impl Slots {
 pub fn new(ctx:&GpuCtx)->Self {
  let mut slots=Self {buffer:GrowBuf::new(ctx,"instance draw slots",8,wgpu::BufferUsages::VERTEX),previous:Vec::new(),written_bytes:0};
  slots.set(ctx,&[]);slots
 }
 pub fn set(&mut self,ctx:&GpuCtx,rows:&[Slot])->bool{
  self.written_bytes=0;
  let bytes=|rows:&[Slot]|rows.iter().flatten().flat_map(|v|v.to_ne_bytes()).collect::<Vec<_>>();
  if self.buffer.len()==0 {
   self.buffer.append_bytes(ctx,&bytes(&[[OWN_ROW,0]]));self.written_bytes=8;
  }
  let inside=self.previous.len().min(rows.len());
  for run in changed_runs(&self.previous,rows) {
   self.buffer.write_bytes_at(ctx,(run.start+1)as u32,&bytes(&rows[run.clone()]));
   self.written_bytes+=run.len()as u64*8;
  }
  if rows.len()>self.previous.len(){
   let tail=bytes(&rows[inside..]);self.buffer.append_bytes(ctx,&tail);self.written_bytes+=tail.len()as u64;
  }else if rows.len()<self.previous.len(){
   self.buffer.reset();self.buffer.append_bytes(ctx,&bytes(&[[OWN_ROW,0]]));self.buffer.append_bytes(ctx,&bytes(rows));
   self.written_bytes=(rows.len()as u64+1)*8;
  }
  self.previous.clear();self.previous.extend_from_slice(rows);self.written_bytes>0
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
--8<-- "journey/code/96zzzh-share-changed-record-runs-01.rs"
```

```text
--8<-- "journey/code/96zzzh-share-changed-record-runs-02.rs"
```

</details>

</details>

<a id="checkpoint-96zzzi-prepare-one-shared-object-record-table"></a>

### Step 15 of 24 · Prepare one shared object record table (`96zzzi-prepare-one-shared-object-record-table`)

- **Goal:** Prepare one growable storage table without duplicating slot upload logic.
- **Data Flow Trace:** `packed object records → changed runs → growing storage buffer → current immutable binding`
- **Files You Touch:** `src/object_rows.rs` (*Reuse the aligned record, growing buffer and shared changed-run helper.*) · `src/lib.rs` (*Expose the preparatory owner.*)

Use the existing aligned object record and growing GPU buffer. Update changed records through the shared scan, append new records, rebuild the immutable binding after buffer growth and release scene capacity when empty. Validate device limits before mutating either owner. This table is preparatory; the next lesson connects instance drawing.

> **Self-Check Question:** *Why replace a bind group after buffer growth?*  
> **Answer:** A bind group retains the buffer it was created with. Growing the owner does not change that retained resource.

- **What You Should See:** The browser scene is unchanged. The growing object-record table is ready for its first drawing consumer.
- **Hands-On Lab Experiment:** Keep the original browser scene running. This preparatory table adds no drawing or allocation until it is connected; inspect the supplied GPU readback showing retained neighbors, high-bit ID bytes and changed binding after growth.

<details>
<summary>Implementation 96zzzi-prepare-one-shared-object-record-table: Prepare one shared object record table</summary>

Prepare one growable storage table without duplicating slot upload logic.

Data flow: packed object records → changed runs → growing storage buffer → current immutable binding.

### 1. `src/object_rows.rs`

Reuse the aligned record, growing buffer and shared changed-run helper.

Create the file and type:

```rust
--8<-- "journey/code/96zzzi-prepare-one-shared-object-record-table-01.rs"
```

### 2. `src/lib.rs`

Expose the preparatory owner.

<details>
<summary>Locate the existing block</summary>

```rust
mod draw_slots;
```

</details>

After that block, type:

```rust
mod object_rows;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzi-prepare-one-shared-object-record-table-02.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/object_rows.rs`

Supply mapped GPU checks for unchanged neighbors, high-bit fields, growth, binding replacement and release.

<details>
<summary>Locate the existing block</summary>

```text
use crate::{object_record::{BYTES,Record},text_gpu_support::{GpuCtx,GrowBuf}};
pub struct Rows {
 pub(crate) buffer:GrowBuf,
 group:wgpu::BindGroup,
 previous:Vec<Record>,
 pub written_bytes:u64,
}
impl Rows {
 pub fn new(ctx:&GpuCtx,layout:&wgpu::BindGroupLayout)->Self {
  let buffer=GrowBuf::new(ctx,"shared object records",BYTES as u64,wgpu::BufferUsages::STORAGE);
  let group=crate::gpu_layout::group(&ctx.device,layout,&buffer.buf);
  Self {buffer,group,previous:Vec::new(),written_bytes:0}
 }
 pub fn set(&mut self,ctx:&GpuCtx,layout:&wgpu::BindGroupLayout,rows:&[Record])->Result<bool,&'static str> {
  let limits=ctx.device.limits();let limit=limits.max_buffer_size.min(u64::from(limits.max_storage_buffer_binding_size));
  if rows.len()as u64>limit/BYTES as u64||u32::try_from(rows.len()).is_err(){return Err("Object records exceed device limits");}
  self.written_bytes=0;
  if rows.is_empty() {
   if !self.previous.is_empty(){self.buffer.release(ctx);self.group=crate::gpu_layout::group(&ctx.device,layout,&self.buffer.buf);self.previous=Vec::new();}
   return Ok(false);
  }
  let bytes=|rows:&[Record]|rows.iter().flatten().copied().collect::<Vec<_>>();
  let mut grew=false;
  if rows.len()<self.previous.len() {
   self.buffer.reset();let data=bytes(rows);grew=self.buffer.append_bytes(ctx,&data);self.written_bytes=data.len()as u64;
  }else {
   for run in crate::draw_slots::changed_runs(&self.previous,rows) {
    let data=bytes(&rows[run.clone()]);self.buffer.write_bytes_at(ctx,run.start as u32,&data);self.written_bytes+=data.len()as u64;
   }
   let tail=bytes(&rows[self.previous.len()..]);
   if !tail.is_empty(){grew=self.buffer.append_bytes(ctx,&tail);self.written_bytes+=tail.len()as u64;}
  }
  if grew{self.group=crate::gpu_layout::group(&ctx.device,layout,&self.buffer.buf);}
  self.previous.clear();self.previous.extend_from_slice(rows);Ok(self.written_bytes>0)
 }
 pub fn bind(&self,pass:&mut wgpu::RenderPass<'_>){pass.set_bind_group(1,&self.group,&[]);}
 pub fn len(&self)->u32{self.previous.len()as u32}
 pub fn allocated_bytes(&self)->u64{if self.previous.is_empty(){0}else{self.buffer.buf.size()}}
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzi-prepare-one-shared-object-record-table-03.rs" skip=2140
```

</details>

</details>

<a id="checkpoint-96zzzj-share-instance-object-records-across-definitions"></a>

### Step 16 of 24 · Share instance object records across definitions (`96zzzj-share-instance-object-records-across-definitions`)

- **Goal:** Draw distinct definitions from one shared placement-record buffer.
- **Data Flow Trace:** `original placements → shared object records → global slot row → unchanged pulled surface shader`
- **Files You Touch:** `src/instance_gpu.rs` (*Remove batch-specific buffer creation.*) · `src/global_outline_tests.rs` (*The second definition now addresses global row one, while the ordinary sentinel stays unchanged.*)

Replace each definition batch’s storage buffer with the shared object table. Slot row numbers now address that table across all definitions. Keep geometry ownership, the ordinary sentinel and primitive bases unchanged. Check the complete table against the device storage limit before adopting a new drawing.

> **Self-Check Question:** *Why must the second definition stop numbering rows from zero?*  
> **Answer:** All definitions now bind the same array. A local zero would read the first definition’s placement, original ID and selection state.

- **What You Should See:** Original placed copies retain their colours, selection and picking. Their drawing batches now share one object-record buffer.
- **Hands-On Lab Experiment:** Open the original instance specimen, then a second original definition. Select and move one copy, Undo and change opacity. Confirm that each definition retains its own placement and original picking; empty scene adoption releases both record and slot capacity.

<details>
<summary>Implementation 96zzzj-share-instance-object-records-across-definitions: Share instance object records across definitions</summary>

Draw distinct definitions from one shared placement-record buffer.

Data flow: original placements → shared object records → global slot row → unchanged pulled surface shader.

### 1. `src/instance_gpu.rs`

Remove batch-specific buffer creation. A batch retains shared geometry and its draw interval only. The lane owns one object-record table. Bind the existing instance layout to the shared table. Number records across definitions before publishing any GPU change. Reuse changed-record writes, current bindings and empty release. Every definition reads the same current object table. Count the shared allocation once. One shared table removes the old per-definition chunk key. Retain each original geometry definition once. Expose the current shared buffer for the authoring GPU readback.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh};
use std::{collections::BTreeMap,rc::Rc};
use wgpu::util::DeviceExt;
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,records:wgpu::Buffer,group:wgpu::BindGroup,bytes:Vec<u8>,count:u32,first_slot:u32}
pub struct Lane {pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<(usize,usize),Batch>,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn source()->String {
 let source=include_str!("triangle.wgsl")
  .replace("var<uniform> object_settings: ObjectSettings;","var<storage, read> objects: array<ObjectSettings>;")
  .replace("return object_settings;","return objects[index];");
 crate::clipping_gpu::source(&source,2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("shared instance records"),entries:&[wgpu::BindGroupLayoutEntry {binding:0,visibility:wgpu::ShaderStages::VERTEX,ty:wgpu::BindingType::Buffer {ty:wgpu::BufferBindingType::Storage {read_only:true},has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(BYTES as u64)},count:None}]});
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(format.into()),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        Self {pipeline,layout,batches:BTreeMap::new(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=(u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size)/BYTES as u64) as usize;
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:BTreeMap<usize,(Rc<GpuGeometry>,Vec<Record>)>=BTreeMap::new();
        for row in rows {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push(row.packed());}
        let mut old=std::mem::take(&mut self.batches);let mut batches=BTreeMap::new();let mut all_slots=Vec::new();
        for (source,(geometry,records)) in groups {
            for (part,records) in records.chunks(limit).enumerate() {
                let slots:Vec<crate::draw_slots::Slot>=records.iter().enumerate().map(|(row,record)|[row as u32,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]).collect();
                let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;all_slots.extend_from_slice(&slots);
                let key=(source,part);let bytes=records.iter().flatten().copied().collect::<Vec<_>>();
                let batch=if let Some(mut batch)=old.remove(&key).filter(|b|Rc::ptr_eq(&b.geometry,&geometry)&&b.records.size()>=bytes.len() as u64) {
                    batch.first_slot=first_slot;
                    if batch.bytes!=bytes {queue.write_buffer(&batch.records,0,&bytes);self.writes+=1;self.written_bytes+=bytes.len() as u64;batch.bytes=bytes;}batch.count=records.len() as u32;batch
                } else {
                    let buffer=device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:Some("independent source placements"),contents:&bytes,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST});let group=crate::gpu_layout::group(device,&self.layout,&buffer);self.writes+=1;self.written_bytes+=bytes.len() as u64;Batch {geometry:geometry.clone(),records:buffer,group,bytes,count:records.len() as u32,first_slot}
                };batches.insert(key,batch);
            }
        }
        let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
        if batches.is_empty()&&!old.is_empty(){self.slots.clear(&ctx,true);}else{self.slots.set(&ctx,&all_slots);}
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;Ok(())
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        for batch in self.batches.values() {pass.set_bind_group(1,&batch.group,&[]);self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.batches.values().map(|b|b.records.size()).sum::<u64>()+if self.batches.is_empty(){0}else{self.slots.allocated_bytes()},self.writes,self.written_bytes]}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh};
use std::{collections::BTreeMap,rc::Rc};
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn source()->String {
 let source=include_str!("triangle.wgsl")
  .replace("var<uniform> object_settings: ObjectSettings;","var<storage, read> objects: array<ObjectSettings>;")
  .replace("return object_settings;","return objects[index];");
 crate::clipping_gpu::source(&source,2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("shared instance records"),entries:&[wgpu::BindGroupLayoutEntry {binding:0,visibility:wgpu::ShaderStages::VERTEX,ty:wgpu::BindingType::Buffer {ty:wgpu::BufferBindingType::Storage {read_only:true},has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(BYTES as u64)},count:None}]});
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(format.into()),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {pipeline,layout,batches:BTreeMap::new(),records,slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=(u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size)/BYTES as u64) as usize;
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:BTreeMap<usize,(Rc<GpuGeometry>,Vec<Record>)>=BTreeMap::new();
        for row in rows {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push(row.packed());}
        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let mut batches=BTreeMap::new();let mut all_slots=Vec::new();let mut all_records=Vec::with_capacity(count);
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
            for record in &records {
                let row=u32::try_from(all_records.len()).map_err(|_|"Instance record row")?;
                all_slots.push([row,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]);
                all_records.push(*record);
            }
            batches.insert(source,Batch {geometry,count:records.len()as u32,first_slot});
        }
        let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
        self.records.set(&ctx,&self.layout,&all_records)?;
        if self.records.written_bytes>0{self.writes+=1;self.written_bytes+=self.records.written_bytes;}
        if batches.is_empty()&&!self.batches.is_empty(){self.slots.clear(&ctx,true);}else{self.slots.set(&ctx,&all_slots);}
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;Ok(())
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        self.records.bind(pass);
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub(crate) fn record_buffer(&self)->&wgpu::Buffer{&self.records.buffer.buf}
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.records.allocated_bytes()+if self.batches.is_empty(){0}else{self.slots.allocated_bytes()},self.writes,self.written_bytes]}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzj-share-instance-object-records-across-definitions-02.rs"
```

```text
--8<-- "journey/code/96zzzj-share-instance-object-records-across-definitions-03.rs"
```

```text
--8<-- "journey/code/96zzzj-share-instance-object-records-across-definitions-04.rs"
```

```text
--8<-- "journey/code/96zzzj-share-instance-object-records-across-definitions-05.rs"
```

```text
--8<-- "journey/code/96zzzj-share-instance-object-records-across-definitions-06.rs"
```

```text
--8<-- "journey/code/96zzzj-share-instance-object-records-across-definitions-07.rs"
```

```text
--8<-- "journey/code/96zzzj-share-instance-object-records-across-definitions-08.rs"
```

```text
--8<-- "journey/code/96zzzj-share-instance-object-records-across-definitions-09.rs"
```

```text
--8<-- "journey/code/96zzzj-share-instance-object-records-across-definitions-10.rs"
```

```text
--8<-- "journey/code/96zzzj-share-instance-object-records-across-definitions-11.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/global_outline_tests.rs`

The second definition now addresses global row one, while the ordinary sentinel stays unchanged.

<details>
<summary>Locate the existing block</summary>

```text
assert_eq!(u32::from_ne_bytes(slots[16..20].try_into().unwrap()),0);
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzj-share-instance-object-records-across-definitions-12.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/global_outline_tests.rs`

Supply exact mapped shared records and actual original-colored pixels in both placed halves; retain selection, opacity restoration and release assertions.

<details>
<summary>Locate the existing block</summary>

```text
 let baseline=crate::cap_gpu_tests::pixels(&gpu);let uploads=gpu.stats()[0];let owner=scene.objects()[0].id;
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzj-share-instance-object-records-across-definitions-13.rs"
```

</details>

</details>

<a id="checkpoint-96zzzk-reuse-checked-curve-control-coordinates"></a>

### Step 17 of 24 · Reuse checked curve control coordinates (`96zzzk-reuse-checked-curve-control-coordinates`)

- **Goal:** Use one checked rational-control decoder for sampling and control markers.
- **Data Flow Trace:** `original curve and index → shared checked control coordinate → sampling or marker display`
- **Files You Touch:** `src/controls.rs` (*Move the checked method already written into this module position; the next edit changes its signature. The body is reused, not a supplied new algorithm.*) · `src/curve.rs` (*Remove the second rational-coordinate decoder.*)

The original curve sampler and control owner both decode the same rational coordinates. Extract the existing checked calculation into a borrowed helper. The control owner keeps its Rc; the sampler calls the helper without allocating another owner. Keep curve-domain, positive-weight and sampling guards in their existing owner.

> **Self-Check Question:** *Why keep curve validation separate?*  
> **Answer:** A valid control coordinate does not establish a valid curve domain, knot vector or display sampling policy. Each caller retains those different obligations.

- **What You Should See:** Original curve samples and control markers retain their checked positions through one coordinate decoder.
- **Hands-On Lab Experiment:** Draw a rational arc and enable its controls. Move one original control, Undo and confirm the restored arc. Inspect the retained supplied malformed-control and invalid-curve fixtures; no duplicate coordinate decoder remains.

<details>
<summary>Implementation 96zzzk-reuse-checked-curve-control-coordinates: Reuse checked curve control coordinates</summary>

Use one checked rational-control decoder for sampling and control markers.

Data flow: original curve and index → shared checked control coordinate → sampling or marker display.

### 1. `src/controls.rs`

Move the checked method already written into this module position; the next edit changes its signature. The body is reused, not a supplied new algorithm.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

<details>
<summary>Previously typed code reference</summary>

Use this reference when moving the existing code; it is excluded from the typing estimate.

<details>
<summary>Locate the existing block</summary>

```text
#[derive(Clone)]
```

</details>

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzk-reuse-checked-curve-control-coordinates-01.rs"
```

</details>

### 2. `src/controls.rs`

Give the moved body a borrowed curve and index; remove the former self binding. Keep the source owner and existing public coordinate API.

<details>
<summary>Locate the existing block</summary>

```rust
use std::rc::Rc;
use session_rust::NurbsCurve;
use crate::marker::Marker;

    pub fn position(&self) -> Result<[f64; 3], &'static str> {
        let c = &self.source;
        if !(2..=3).contains(&c.m_dim) || !(1..=10_000).contains(&c.m_cv_count) || self.index >= c.m_cv_count {
            return Err("Control dimension, count or index is invalid");
        }
        let size = c.m_dim + usize::from(c.m_is_rat);
        if c.m_cv_stride < size { return Err("Control stride is too short"); }
        let start = self.index.checked_mul(c.m_cv_stride).ok_or("Control offset overflows")?;
        let end = start.checked_add(size).ok_or("Control range overflows")?;
        let values = c.m_cv.get(start..end).ok_or("Control values are incomplete")?;
        let weight = if c.m_is_rat { values[c.m_dim] } else { 1.0 };
        if !weight.is_finite() || weight == 0.0 { return Err("Control weight must be finite and nonzero"); }
        let mut point = [0.0; 3];
        for axis in 0..c.m_dim { point[axis] = values[axis] / weight; }
        if point.iter().any(|v| !v.is_finite()) { return Err("Control position must be finite"); }
        Ok(point)
    }
#[derive(Clone)]
pub struct Control { pub source: Rc<NurbsCurve>, pub index: usize }

impl Control {
    pub fn new(source: Rc<NurbsCurve>, index: usize) -> Result<Self, &'static str> {
        let control = Self { source, index }; control.position()?; Ok(control)
    }

    pub fn position(&self) -> Result<[f64; 3], &'static str> {
        let c = &self.source;
        if !(2..=3).contains(&c.m_dim) || !(1..=10_000).contains(&c.m_cv_count) || self.index >= c.m_cv_count {
            return Err("Control dimension, count or index is invalid");
        }
        let size = c.m_dim + usize::from(c.m_is_rat);
        if c.m_cv_stride < size { return Err("Control stride is too short"); }
        let start = self.index.checked_mul(c.m_cv_stride).ok_or("Control offset overflows")?;
        let end = start.checked_add(size).ok_or("Control range overflows")?;
        let values = c.m_cv.get(start..end).ok_or("Control values are incomplete")?;
        let weight = if c.m_is_rat { values[c.m_dim] } else { 1.0 };
        if !weight.is_finite() || weight == 0.0 { return Err("Control weight must be finite and nonzero"); }
        let mut point = [0.0; 3];
        for axis in 0..c.m_dim { point[axis] = values[axis] / weight; }
        if point.iter().any(|v| !v.is_finite()) { return Err("Control position must be finite"); }
        Ok(point)
    }

    pub fn marker(&self) -> Result<Marker, &'static str> {
        let marker = Marker { center: self.position()?.map(|v| v as f32), diameter: 6.0, colour: [0.0, 0.0, 0.0, 1.0] };
        if marker.values().iter().any(|v| !v.is_finite()) { return Err("Control marker exceeds the display range"); }
        Ok(marker)
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use std::rc::Rc;
use session_rust::NurbsCurve;
use crate::marker::Marker;

pub(crate) fn position(c:&NurbsCurve,index:usize)->Result<[f64;3],&'static str> {
        if !(2..=3).contains(&c.m_dim) || !(1..=10_000).contains(&c.m_cv_count) || index >= c.m_cv_count {
            return Err("Control dimension, count or index is invalid");
        }
        let size = c.m_dim + usize::from(c.m_is_rat);
        if c.m_cv_stride < size { return Err("Control stride is too short"); }
        let start = index.checked_mul(c.m_cv_stride).ok_or("Control offset overflows")?;
        let end = start.checked_add(size).ok_or("Control range overflows")?;
        let values = c.m_cv.get(start..end).ok_or("Control values are incomplete")?;
        let weight = if c.m_is_rat { values[c.m_dim] } else { 1.0 };
        if !weight.is_finite() || weight == 0.0 { return Err("Control weight must be finite and nonzero"); }
        let mut point = [0.0; 3];
        for axis in 0..c.m_dim { point[axis] = values[axis] / weight; }
        if point.iter().any(|v| !v.is_finite()) { return Err("Control position must be finite"); }
        Ok(point)
}

#[derive(Clone)]
pub struct Control { pub source: Rc<NurbsCurve>, pub index: usize }

impl Control {
    pub fn new(source: Rc<NurbsCurve>, index: usize) -> Result<Self, &'static str> {
        let control = Self { source, index }; control.position()?; Ok(control)
    }

    pub fn position(&self) -> Result<[f64; 3], &'static str> {
        position(&self.source,self.index)
    }

    pub fn marker(&self) -> Result<Marker, &'static str> {
        let marker = Marker { center: self.position()?.map(|v| v as f32), diameter: 6.0, colour: [0.0, 0.0, 0.0, 1.0] };
        if marker.values().iter().any(|v| !v.is_finite()) { return Err("Control marker exceeds the display range"); }
        Ok(marker)
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzk-reuse-checked-curve-control-coordinates-borrowed-signature.rs"
```

```text
--8<-- "journey/code/96zzzk-reuse-checked-curve-control-coordinates-02.rs"
```

</details>

### 3. `src/curve.rs`

Remove the second rational-coordinate decoder. Reuse checked coordinates without an Rc clone, retaining the sampler’s existing extent error.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
pub fn sample(source: &NurbsCurve) -> Result<Vec<[f64; 3]>, &'static str> {
    let (start, end) = validate(source)?;
    let mut turning = 0.0; let mut previous: Option<[f64; 3]> = None;
    let control = |index: usize| {
        let offset = index * source.m_cv_stride;
        let weight = if source.m_is_rat { source.m_cv[offset + source.m_dim] } else { 1.0 };
        std::array::from_fn::<_, 3, _>(|axis| if axis < source.m_dim { source.m_cv[offset + axis] / weight } else { 0.0 })
    };
    for index in 1..source.m_cv_count {
        let a = control(index - 1); let b = control(index);
        let direction: [f64; 3] = std::array::from_fn(|axis| b[axis] - a[axis]);
        let length = direction.iter().map(|v| v * v).sum::<f64>().sqrt();
        if length <= 1e-12 { continue; }
```

</details>

Replace it with:

```rust
pub fn sample(source: &NurbsCurve) -> Result<Vec<[f64; 3]>, &'static str> {
    let (start, end) = validate(source)?;
    let mut turning = 0.0; let mut previous: Option<[f64; 3]> = None;
    for index in 1..source.m_cv_count {
        let a = crate::controls::position(source,index-1).map_err(|_|"Curve control extent exceeds the display range")?; let b = crate::controls::position(source,index).map_err(|_|"Curve control extent exceeds the display range")?;
        let direction: [f64; 3] = std::array::from_fn(|axis| b[axis] - a[axis]);
        let length = direction.iter().map(|v| v * v).sum::<f64>().sqrt();
        if length <= 1e-12 { continue; }
```

<details>
<summary>Complete updated block reference</summary>

```rust
use session_rust::NurbsCurve;

pub fn validate(source: &NurbsCurve) -> Result<(f64, f64), &'static str> {
    if !(2..=3).contains(&source.m_dim) || !(2..=10_000).contains(&source.m_cv_count)
        || !(2..=32).contains(&source.m_order) || source.m_order > source.m_cv_count {
        return Err("Curve dimension, count or order is invalid");
    }
    let size = source.m_dim + usize::from(source.m_is_rat);
    if source.m_cv_stride < size { return Err("Curve control stride is too short"); }
    let end = (source.m_cv_count - 1).checked_mul(source.m_cv_stride)
        .and_then(|offset| offset.checked_add(size)).ok_or("Curve control range overflows")?;
    if source.m_cv.len() < end { return Err("Curve control values are incomplete"); }
    for index in 0..source.m_cv_count {
        let start = index * source.m_cv_stride;
        let values = &source.m_cv[start..start + size];
        if values.iter().any(|v| !v.is_finite()) { return Err("Curve controls must be finite"); }
        if source.m_is_rat && values[source.m_dim] <= 0.0 { return Err("Curve display requires positive rational weights"); }
    }
    let count = source.m_cv_count + source.m_order - 2;
    if source.m_nurbsknot.len() != count || source.m_nurbsknot.iter().any(|v| !v.is_finite())
        || source.m_nurbsknot.windows(2).any(|pair| pair[0] > pair[1]) {
        return Err("Curve knot vector is incomplete or unordered");
    }
    let domain = source.domain();
    if domain.0 >= domain.1 || !(domain.1 - domain.0).is_finite() { return Err("Curve domain must have positive length"); }
    Ok(domain)
}

pub fn sample(source: &NurbsCurve) -> Result<Vec<[f64; 3]>, &'static str> {
    let (start, end) = validate(source)?;
    let mut turning = 0.0; let mut previous: Option<[f64; 3]> = None;
    for index in 1..source.m_cv_count {
        let a = crate::controls::position(source,index-1).map_err(|_|"Curve control extent exceeds the display range")?; let b = crate::controls::position(source,index).map_err(|_|"Curve control extent exceeds the display range")?;
        let direction: [f64; 3] = std::array::from_fn(|axis| b[axis] - a[axis]);
        let length = direction.iter().map(|v| v * v).sum::<f64>().sqrt();
        if length <= 1e-12 { continue; }
        if !length.is_finite() { return Err("Curve control extent exceeds the display range"); }
        let unit = direction.map(|v| v / length);
        if let Some(before) = previous {
            let dot = (0..3).map(|axis| before[axis] * unit[axis]).sum::<f64>().clamp(-1.0, 1.0);
            turning += dot.acos().to_degrees();
        }
        previous = Some(unit);
    }
    let count = ((turning / 3.0).ceil() as usize).clamp(1, 512);
    let mut parameters: Vec<_> = (0..=count).map(|i| start + (end - start) * i as f64 / count as f64).collect();
    parameters.extend(source.m_nurbsknot.iter().copied().filter(|&t| t > start && t < end));
    parameters.sort_by(f64::total_cmp); parameters.dedup();
    let mut points = Vec::with_capacity(parameters.len());
    for parameter in parameters {
        let point = source.point_at(parameter); let point: [f64; 3] = std::array::from_fn(|axis| point[axis]);
        if point.iter().any(|&v| !(v as f32).is_finite()) { return Err("Sampled curve exceeds the display range"); }
        points.push(point);
    }
    Ok(points)
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzk-reuse-checked-curve-control-coordinates-04.rs"
```

</details>

</details>

<a id="checkpoint-96zzzl-publish-allocator-texels-through-the-integer-table"></a>

### Step 18 of 24 · Publish allocator texels through the integer table (`96zzzl-publish-allocator-texels-through-the-integer-table`)

- **Goal:** Connect allocator changes to the existing exact integer texture writer.
- **Data Flow Trace:** `SlotSpace change → exact allocator texels → full or bounded texture write → current texture view`
- **Files You Touch:** `src/primitive_slot_table.rs` (*Share the existing texture-shape rule instead of repeating it.*) · `src/primitive_slot_table_tests.rs` (*Supply actual mapped allocator words, high-bit rows, retired slots, unchanged neighbors and zero release.*)

The CPU slot allocator and GPU lookup texture are already written. Publish full texels when the texture shape changes; otherwise write only the allocator’s changed ranges. Reuse the existing texture writer and preserve its exact integer words. This connects publication helpers; fragment lookup remains a later step.

> **Self-Check Question:** *Why upload every texel after replacing the texture?*  
> **Answer:** A new texture contains no previous records. Writing only this edit’s dirty range would lose untouched live slots.

- **What You Should See:** The browser scene is unchanged. Allocator changes can now publish exact words through the existing GPU texture writer.
- **Hands-On Lab Experiment:** Keep the original browser scene running. The helper adds no drawing until connected to a consumer. Inspect the supplied GPU readback for high-bit rows, retired slots, untouched neighbors and release to one zero texel.

<details>
<summary>Implementation 96zzzl-publish-allocator-texels-through-the-integer-table: Publish allocator texels through the integer table</summary>

Connect allocator changes to the existing exact integer texture writer.

Data flow: SlotSpace change → exact allocator texels → full or bounded texture write → current texture view.

### 1. `src/primitive_slot_table.rs`

Share the existing texture-shape rule instead of repeating it. Keep full writes and allocator publication on the same dimensions. Publish existing dirty ranges and report whether immutable consumer bindings must be rebuilt.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::text_gpu_support::GpuCtx;
pub const WIDTH:u32=1024;
pub struct Table {pub(crate) texture:wgpu::Texture,pub view:wgpu::TextureView}
fn texture(ctx:&GpuCtx,width:u32,height:u32)->wgpu::Texture{
 ctx.device.create_texture(&wgpu::TextureDescriptor {label:Some("primitive slot lookup"),size:wgpu::Extent3d {width,height,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba32Uint,usage:wgpu::TextureUsages::TEXTURE_BINDING|wgpu::TextureUsages::COPY_DST|wgpu::TextureUsages::COPY_SRC,view_formats:&[]})
}
impl Table {
 pub fn new(ctx:&GpuCtx)->Self{let texture=texture(ctx,1,1);let view=texture.create_view(&Default::default());Self {texture,view}}
 pub fn write(&mut self,ctx:&GpuCtx,texels:&[[u32;4]]){
  let count=u32::try_from(texels.len()).expect("Bounded slot table");let(width,height)=if count<=1{(1,1)}else{(WIDTH,count.div_ceil(WIDTH))};
  assert!(height<=ctx.device.limits().max_texture_dimension_2d);
  if self.texture.width()!=width||self.texture.height()!=height{self.texture=texture(ctx,width,height);self.view=self.texture.create_view(&Default::default());}
  let mut data=texels.to_vec();data.resize((width*height)as usize,[0;4]);self.write_at(ctx,0,&data);
 }
 pub fn write_at(&self,ctx:&GpuCtx,first:u32,texels:&[[u32;4]])->bool{
  let width=self.texture.width();if u64::from(first)+texels.len()as u64>u64::from(width)*u64::from(self.texture.height()){return false;}
  let mut at=first;let mut rest=texels;
  while !rest.is_empty(){
   let(x,y)=(at%width,at/width);let(row,tail)=rest.split_at(((width-x)as usize).min(rest.len()));let bytes:Vec<u8>=row.iter().flatten().flat_map(|v|v.to_le_bytes()).collect();
   ctx.queue.write_texture(wgpu::TexelCopyTextureInfo {origin:wgpu::Origin3d {x,y,z:0},..self.texture.as_image_copy()},&bytes,wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(row.len()as u32*16),rows_per_image:Some(1)},wgpu::Extent3d {width:row.len()as u32,height:1,depth_or_array_layers:1});
   at+=row.len()as u32;rest=tail;
  }true
 }
 pub fn allocated_bytes(&self)->u64{u64::from(self.texture.width())*u64::from(self.texture.height())*16}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::text_gpu_support::GpuCtx;
pub const WIDTH:u32=1024;
pub struct Table {pub(crate) texture:wgpu::Texture,pub view:wgpu::TextureView}
fn dimensions(count:u32)->(u32,u32){if count<=1{(1,1)}else{(WIDTH,count.div_ceil(WIDTH))}}
fn texture(ctx:&GpuCtx,width:u32,height:u32)->wgpu::Texture{
 ctx.device.create_texture(&wgpu::TextureDescriptor {label:Some("primitive slot lookup"),size:wgpu::Extent3d {width,height,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba32Uint,usage:wgpu::TextureUsages::TEXTURE_BINDING|wgpu::TextureUsages::COPY_DST|wgpu::TextureUsages::COPY_SRC,view_formats:&[]})
}
impl Table {
 pub fn new(ctx:&GpuCtx)->Self{let texture=texture(ctx,1,1);let view=texture.create_view(&Default::default());Self {texture,view}}
 pub fn write(&mut self,ctx:&GpuCtx,texels:&[[u32;4]]){
  let count=u32::try_from(texels.len()).expect("Bounded slot table");let(width,height)=dimensions(count);
  assert!(height<=ctx.device.limits().max_texture_dimension_2d);
  if self.texture.width()!=width||self.texture.height()!=height{self.texture=texture(ctx,width,height);self.view=self.texture.create_view(&Default::default());}
  let mut data=texels.to_vec();data.resize((width*height)as usize,[0;4]);self.write_at(ctx,0,&data);
 }
 pub fn write_at(&self,ctx:&GpuCtx,first:u32,texels:&[[u32;4]])->bool{
  let width=self.texture.width();if u64::from(first)+texels.len()as u64>u64::from(width)*u64::from(self.texture.height()){return false;}
  let mut at=first;let mut rest=texels;
  while !rest.is_empty(){
   let(x,y)=(at%width,at/width);let(row,tail)=rest.split_at(((width-x)as usize).min(rest.len()));let bytes:Vec<u8>=row.iter().flatten().flat_map(|v|v.to_le_bytes()).collect();
   ctx.queue.write_texture(wgpu::TexelCopyTextureInfo {origin:wgpu::Origin3d {x,y,z:0},..self.texture.as_image_copy()},&bytes,wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(row.len()as u32*16),rows_per_image:Some(1)},wgpu::Extent3d {width:row.len()as u32,height:1,depth_or_array_layers:1});
   at+=row.len()as u32;rest=tail;
  }true
 }
 pub fn publish(&mut self,ctx:&GpuCtx,space:&crate::slot_space::SlotSpace,change:&crate::slot_space::Change)->bool {
  let previous=self.texture.clone();let (width,height)=dimensions(space.end());
  if change.all||self.texture.width()!=width||self.texture.height()!=height {
   let texels=(0..space.end()).map(|slot|space.texel(slot)).collect::<Vec<_>>();self.write(ctx,&texels);
  }else {
   for range in &change.slots {
    assert!(range.start<=range.end&&range.end<=space.end());
    let texels=range.clone().map(|slot|space.texel(slot)).collect::<Vec<_>>();assert!(self.write_at(ctx,range.start,&texels));
   }
  }
  self.texture!=previous
 }
 pub fn allocated_bytes(&self)->u64{u64::from(self.texture.width())*u64::from(self.texture.height())*16}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzl-publish-allocator-texels-through-the-integer-table-01.rs"
```

```text
--8<-- "journey/code/96zzzl-publish-allocator-texels-through-the-integer-table-02.rs"
```

```text
--8<-- "journey/code/96zzzl-publish-allocator-texels-through-the-integer-table-03.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/primitive_slot_table_tests.rs`

Supply actual mapped allocator words, high-bit rows, retired slots, unchanged neighbors and zero release.

<details>
<summary>Locate the existing block</summary>

```text
use crate::{primitive_slot_table::Table,text_gpu_support::GpuCtx};
fn read(ctx:&GpuCtx,table:&Table)->Vec<[u32;4]>{
 let size=table.texture.size();let pitch=(size.width*16+255)/256*256;let bytes=u64::from(pitch)*u64::from(size.height);
 let output=ctx.device.create_buffer(&wgpu::BufferDescriptor {label:Some("slot texture readback"),size:bytes,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::MAP_READ,mapped_at_creation:false});let mut encoder=ctx.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(table.texture.as_image_copy(),wgpu::TexelCopyBufferInfo {buffer:&output,layout:wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(size.height)}},size);ctx.queue.submit([encoder.finish()]);
 let(tx,rx)=std::sync::mpsc::channel();output.slice(..).map_async(wgpu::MapMode::Read,move|result|tx.send(result).unwrap());ctx.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();rx.recv().unwrap().unwrap();let mapped=output.slice(..).get_mapped_range();let result=mapped.chunks_exact(pitch as usize).flat_map(|row|row[..size.width as usize*16].chunks_exact(16).map(|texel|std::array::from_fn(|i|u32::from_le_bytes(texel[i*4..i*4+4].try_into().unwrap())))).collect();drop(mapped);output.unmap();result
}
#[test]
fn actual_integer_slot_texture_preserves_ids_across_partial_row_writes_and_release(){
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut table=Table::new(&ctx);assert_eq!(table.allocated_bytes(),16);assert_eq!(read(&ctx,&table),vec![[0;4]]);
 let mut wanted=(0..1050).map(|i|[i,0x80000000+i,16_777_217+i,3]).collect::<Vec<_>>();table.write(&ctx,&wanted);assert_eq!([table.texture.width(),table.texture.height()],[1024,2]);assert_eq!(table.texture.format(),wgpu::TextureFormat::Rgba32Uint);let texture=table.texture.clone();wanted.resize(2048,[0;4]);assert_eq!(read(&ctx,&table),wanted);
 let changed=[[7,u32::MAX-1,31,9],[11,0x80000000,37,12],[13,16_777_217,41,15]];assert!(table.write_at(&ctx,1023,&changed));wanted[1023..1026].copy_from_slice(&changed);assert_eq!(table.texture,texture);assert_eq!(read(&ctx,&table),wanted);
 assert!(!table.write_at(&ctx,u32::MAX,&changed));assert!(!table.write_at(&ctx,2047,&changed));assert!(table.write_at(&ctx,2048,&[]));assert_eq!(read(&ctx,&table),wanted);
 table.write(&ctx,&[]);assert_eq!(table.allocated_bytes(),16);assert_eq!(read(&ctx,&table),vec![[0;4]]);assert!(!table.write_at(&ctx,0,&changed));assert_eq!(read(&ctx,&table),vec![[0;4]]);
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzl-publish-allocator-texels-through-the-integer-table-04.rs" skip=2598
```

</details>

</details>

<a id="checkpoint-96zzzm-resolve-physical-triangles-with-integer-slots"></a>

### Step 19 of 24 · Resolve physical triangles with integer slots (`96zzzm-resolve-physical-triangles-with-integer-slots`)

- **Goal:** Resolve a physical triangle ID back to its definition triangle and object row.
- **Data Flow Trace:** `One-based physical ID → unsigned table search → definition triangle and object row`
- **Files You Touch:** `src/primitive_slot_lookup.wgsl` (*Resolve one-based physical IDs with unsigned integer texels and reject unused ranges.*) · `src/primitive_slot_table_tests.rs` (*Supply an actual GPU dispatch through the taught shader; check original IDs, high-bit rows, retired holes and an empty table.*)

An instance has its own physical triangle IDs while sharing geometry. Read the integer table header, keep ordinary triangles on their own rows, then find the last slot starting before the requested ID. Reject retired slots and spare ranges. Write this shader once so later GPU consumers share the same rule.

> **Self-Check Question:** *Why check the slot triangle count after the binary search?*  
> **Answer:** The search finds the preceding range, which can be a retired slot or reserved spare space. Its count determines whether this ID names a live triangle.

- **What You Should See:** The browser scene is unchanged. The shared lookup shader is prepared for later projection and ambient consumers.
- **Hands-On Lab Experiment:** Keep the original browser scene running. Inspect the supplied dispatch results for ordinary triangles, two definitions, high-bit rows, retired holes and a texture-row boundary. A table with no instances still resolves ordinary arena IDs. The visible renderer is connected in a later step.

<details>
<summary>Implementation 96zzzm-resolve-physical-triangles-with-integer-slots: Resolve physical triangles with integer slots</summary>

Resolve a physical triangle ID back to its definition triangle and object row.

Data flow: One-based physical ID → unsigned table search → definition triangle and object row.

### 1. `src/primitive_slot_lookup.wgsl`

Resolve one-based physical IDs with unsigned integer texels and reject unused ranges.

Create the file and type:

```wgsl
--8<-- "journey/code/96zzzm-resolve-physical-triangles-with-integer-slots-01.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/primitive_slot_table_tests.rs`

Supply an actual GPU dispatch through the taught shader; check original IDs, high-bit rows, retired holes and an empty table.

<details>
<summary>Locate the existing block</summary>

```text
use crate::{primitive_slot_table::Table,text_gpu_support::GpuCtx};
fn read(ctx:&GpuCtx,table:&Table)->Vec<[u32;4]>{
 let size=table.texture.size();let pitch=(size.width*16+255)/256*256;let bytes=u64::from(pitch)*u64::from(size.height);
 let output=ctx.device.create_buffer(&wgpu::BufferDescriptor {label:Some("slot texture readback"),size:bytes,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::MAP_READ,mapped_at_creation:false});let mut encoder=ctx.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(table.texture.as_image_copy(),wgpu::TexelCopyBufferInfo {buffer:&output,layout:wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(size.height)}},size);ctx.queue.submit([encoder.finish()]);
 let(tx,rx)=std::sync::mpsc::channel();output.slice(..).map_async(wgpu::MapMode::Read,move|result|tx.send(result).unwrap());ctx.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();rx.recv().unwrap().unwrap();let mapped=output.slice(..).get_mapped_range();let result=mapped.chunks_exact(pitch as usize).flat_map(|row|row[..size.width as usize*16].chunks_exact(16).map(|texel|std::array::from_fn(|i|u32::from_le_bytes(texel[i*4..i*4+4].try_into().unwrap())))).collect();drop(mapped);output.unmap();result
}
#[test]
fn actual_integer_slot_texture_preserves_ids_across_partial_row_writes_and_release(){
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut table=Table::new(&ctx);assert_eq!(table.allocated_bytes(),16);assert_eq!(read(&ctx,&table),vec![[0;4]]);
 let mut wanted=(0..1050).map(|i|[i,0x80000000+i,16_777_217+i,3]).collect::<Vec<_>>();table.write(&ctx,&wanted);assert_eq!([table.texture.width(),table.texture.height()],[1024,2]);assert_eq!(table.texture.format(),wgpu::TextureFormat::Rgba32Uint);let texture=table.texture.clone();wanted.resize(2048,[0;4]);assert_eq!(read(&ctx,&table),wanted);
 let changed=[[7,u32::MAX-1,31,9],[11,0x80000000,37,12],[13,16_777_217,41,15]];assert!(table.write_at(&ctx,1023,&changed));wanted[1023..1026].copy_from_slice(&changed);assert_eq!(table.texture,texture);assert_eq!(read(&ctx,&table),wanted);
 assert!(!table.write_at(&ctx,u32::MAX,&changed));assert!(!table.write_at(&ctx,2047,&changed));assert!(table.write_at(&ctx,2048,&[]));assert_eq!(read(&ctx,&table),wanted);
 table.write(&ctx,&[]);assert_eq!(table.allocated_bytes(),16);assert_eq!(read(&ctx,&table),vec![[0;4]]);assert!(!table.write_at(&ctx,0,&changed));assert_eq!(read(&ctx,&table),vec![[0;4]]);
}

#[test]
fn actual_allocator_publication_keeps_high_rows_free_slots_neighbours_and_release() {
 use crate::slot_space::{Definition,SlotSpace};
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut table=Table::new(&ctx);let mut space=SlotSpace::default();
 let mut a=[7,0x80000001];let b=[u32::MAX-1];
 fn definitions<'a>(a:&'a[u32],b:&'a[u32])->Vec<Definition<'a>>{vec![Definition {key:1,rows:a,faces:30..36,pipes:0..0,ribbons:0..0},Definition {key:2,rows:b,faces:60..63,pipes:0..0,ribbons:0..0}]}
 let change=space.reset(&definitions(&a,&b),100);assert!(table.publish(&ctx,&space,&change));let texture=table.texture.clone();let bytes=read(&ctx,&table);assert_eq!(&bytes[..4],&[[3,100,0,0],[7,100,10,2],[0x80000001,102,10,2],[u32::MAX-1,104,20,1]]);
 a[0]=0x80000007;let change=space.update(&definitions(&a,&b),&[(1,0)],100);assert!(!change.all);assert!(!table.publish(&ctx,&space,&change));assert_eq!(table.texture,texture);let next=read(&ctx,&table);assert_eq!(next[1],[0x80000007,100,10,2]);assert_eq!(&next[2..],&bytes[2..]);
 let remaining=[Definition {key:2,rows:&b,faces:60..63,pipes:0..0,ribbons:0..0}];let change=space.update(&remaining,&[],100);table.publish(&ctx,&space,&change);let next=read(&ctx,&table);assert_eq!(next[1][3],0);assert_eq!(next[2][3],0);assert_eq!(next[3],[u32::MAX-1,104,20,1]);
 let change=space.reset(&[],0);assert!(table.publish(&ctx,&space,&change));assert_eq!(table.allocated_bytes(),16);assert_eq!(read(&ctx,&table),vec![[0;4]]);
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzm-resolve-physical-triangles-with-integer-slots-02.rs" skip=4137
```

</details>

</details>

<a id="checkpoint-96zzzn-batch-shared-geometry-with-original-ownership"></a>

### Step 20 of 24 · Batch shared geometry with original ownership (`96zzzn-batch-shared-geometry-with-original-ownership`)

- **Goal:** Reuse instance drawing for copied originals sharing GPU geometry.
- **Data Flow Trace:** `Visible original rows → shared GPU geometry → object-record batch → original IDs`
- **Files You Touch:** `src/renderer.rs` (*Batch reused GPU geometry without turning copied originals into kernel InstanceRef owners.*) · `src/pick_gpu_tests.rs` (*Supply actual copied-original pixels, high-bit original picks, selection restoration, hide/reopen and CPU/GPU release.*) · `src/instance_gpu.rs` (*Share the existing record capacity and retain ordinary drawing when optional copied-original batches exceed it.*) · `src/object_rows.rs` (*Own the shared record-capacity calculation beside the record table.*)

Copying an original can reuse its GPU geometry without becoming a saved InstanceRef. Choose the GPU drawing path from shared geometry and declared instances, while leaving source ownership, editing, archives and original pick IDs in the scene. When only one ordinary original remains visible, use the existing ordinary draw path again. Keep copied originals on the ordinary path when their optional shared batch would exceed the existing record-table capacity.

> **Self-Check Question:** *Why keep GPU batching separate from source ownership?*  
> **Answer:** Sharing a GPU allocation is a drawing decision. It must not rewrite an original CAD object as a saved kernel instance.

- **What You Should See:** Copied originals share the object-record drawing buffer while keeping their original identities. A single visible ordinary original returns to ordinary drawing.
- **Hands-On Lab Experiment:** Copy an original object, move the copy, select it and hide it. Show it again and check that both originals retain their positions and independent picks. Inspect the supplied pixel and release fixture without running native tests.

<details>
<summary>Implementation 96zzzn-batch-shared-geometry-with-original-ownership: Batch shared geometry with original ownership</summary>

Reuse instance drawing for copied originals sharing GPU geometry.

Data flow: Visible original rows → shared GPU geometry → object-record batch → original IDs.

### 1. `src/renderer.rs`

Batch reused GPU geometry without turning copied originals into kernel InstanceRef owners.

<details>
<summary>Locate the existing block</summary>

```rust
        self.instance_rows=scene.instances.keys().copied().collect();
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzn-batch-shared-geometry-with-original-ownership-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/pick_gpu_tests.rs`

Supply actual copied-original pixels, high-bit original picks, selection restoration, hide/reopen and CPU/GPU release.

<details>
<summary>Locate the existing block</summary>

```text
use super::*;
use crate::{renderer::Renderer, viewport::Viewport};

fn renderer(scene: &Scene) -> Renderer {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: wgpu::Backends::VULKAN, flags: Default::default(), memory_budget_thresholds: Default::default(), backend_options: Default::default(), display: None });
    let adapter = pollster::block_on(instance.request_adapter(&Default::default())).expect("Actual GPU adapter");
    let (device, queue) = pollster::block_on(adapter.request_device(&Default::default())).expect("Actual GPU device");
    let mut renderer = Renderer::new(device, queue, wgpu::TextureFormat::Rgba8Unorm, scene);
    renderer.resize(Viewport { width: 320, height: 240 }); renderer
}

fn finish(renderer: &mut Renderer) -> Option<[u32; 2]> {
    renderer.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: Some(std::time::Duration::from_secs(5)) }).unwrap();
    renderer.take_pick(identity()).expect("Map callback completed").unwrap()
}

fn identity() -> [f32; 16] { session_rust::Xform::identity().m.map(|n| n as f32) }
fn triangle(z: f64) -> PreparedMesh {
    PreparedMesh::triangle([[-0.8, -0.8, z], [0.8, -0.8, z], [0.0, 0.8, z]], [0.8, 0.2, 0.1]).unwrap()
}


#[test]
fn actual_projected_feature_pixel_selects_original_element_not_display_path() {
    let mut scene = Scene::empty();
    let mut element = session_rust::Element::new("feature pick");
    element.set_geometry(session_rust::Mesh::create_box(0.2,0.2,0.2));
    let outline = session_rust::Polyline::new(vec![session_rust::Point::new(-0.8,0.4,0.2),session_rust::Point::new(0.8,0.4,0.2)]);
    element.add_feature(session_rust::element::ElementFeature::new("cut", 0, vec![outline], "pick line"));
    let original = Rc::new(element);
    let owner = crate::element_original::insert(&mut scene, original.clone()).unwrap();
    crate::feature_display::append(&mut scene,owner,&session_rust::Session::new("test"),&original).unwrap();
    assert_eq!(scene.paths().len(),1); let path=scene.paths()[0].id;
    let mut renderer=renderer(&scene); renderer.request_pick([0.,0.4],identity()).unwrap();
    let pixel=finish(&mut renderer).unwrap();assert_eq!(pixel,[owner.value(),0]);assert_ne!(pixel[0],path.value());
    assert_eq!(renderer.resolve_pick(&scene,pixel),Some(crate::pick_sources::Target {owner,part:crate::pick_sources::Part::Object}));
    Rc::make_mut(&mut scene.view).lock(&[owner],true);renderer.set_scene(&scene,None);
    renderer.request_pick([0.,0.4],identity()).unwrap();assert_eq!(finish(&mut renderer),None);
}

#[test]
fn actual_feature_point_disc_uses_original_owner_and_css_diameter() {
    let mut scene = Scene::empty();
    let mut element = session_rust::Element::new("feature dot");
    element.set_geometry(session_rust::Mesh::create_box(0.2, 0.2, 0.2));
    let mut dot = session_rust::Polyline::new(vec![session_rust::Point::new(0.5, 0.4, 0.2), session_rust::Point::new(0.5, 0.4, 0.2)]);
    dot.linecolor = session_rust::Color::new(0.9, 0.1, 0.1, 1.);
    element.add_feature(session_rust::element::ElementFeature::new("drill", -1, vec![dot], "dot"));
    let original = Rc::new(element);
    let owner = crate::element_original::insert(&mut scene, original.clone()).unwrap();
    crate::feature_display::append(&mut scene, owner, &session_rust::Session::new("dot"), &original).unwrap();
    assert_eq!(scene.points().len(), 1);
    let mut renderer = renderer(&scene);
    for density in [1., 2.] {
        renderer.resize(Viewport {width: 320, height: 240});
        renderer.set_density(density);
        let image = colour_pixels(&mut renderer);
        let pixels: Vec<_> = image.chunks_exact(4).enumerate().filter_map(|(index, pixel)| {
            let (x, y) = (index % 320, index / 320);
            (x > 210 && (40..105).contains(&y) && u16::from(pixel[0]) > 2 * u16::from(pixel[1])
                && u16::from(pixel[0]) > 2 * u16::from(pixel[2])).then_some((x, y))
        }).collect();
        assert!(!pixels.is_empty());
        for axis in [0, 1] {
            let values: Vec<_> = pixels.iter().map(|&(x, y)| if axis == 0 { x } else { y }).collect();
            let span = values.iter().max().unwrap() - values.iter().min().unwrap() + 1;
            assert!((span as f32 - 12. * density).abs() <= 1., "DPR {density}, axis {axis}, physical span {span}");
        }
        // Picking searches the already-taught six-CSS-pixel neighbourhood around the disc.
        for (offset, expected) in [(0., Some([owner.value(), 0])), (5., Some([owner.value(), 0])), (13., None)] {
            renderer.request_pick([0.5 + 2. * offset * density / 320., 0.4], identity()).unwrap();
            assert_eq!(finish(&mut renderer), expected, "DPR {density}, CSS offset {offset}");
        }
    }
}

#[test]
fn actual_integer_pick_preserves_high_ids_depth_and_locked_occlusion() {
    let mut scene = Scene::demo(); scene.clear(); scene.next_id = 16_777_217;
    let back = scene.insert(triangle(0.6)).unwrap(); let mut renderer = renderer(&scene);
    renderer.request_pick([0.0, 0.0], identity()).unwrap(); assert_eq!(finish(&mut renderer), Some([back.value(), 0]));
    let front = scene.insert(triangle(0.2)).unwrap(); renderer.set_scene(&scene, None);
    renderer.request_pick([0.0, 0.0], identity()).unwrap(); assert_eq!(finish(&mut renderer), Some([front.value(), 0]));
    Rc::make_mut(&mut scene.view).lock(&[front],true);renderer.set_scene(&scene,None);
    renderer.request_pick([0.0, 0.0], identity()).unwrap(); assert_eq!(finish(&mut renderer), None);
    assert_eq!(scene.pick_id(front.value()), None);
    Rc::make_mut(&mut scene.view).hide(&[front],true);renderer.set_scene(&scene,None);
    renderer.request_pick([0.0, 0.0], identity()).unwrap(); assert_eq!(finish(&mut renderer), Some([back.value(), 0]));
    renderer.request_pick([0.95, 0.95], identity()).unwrap(); assert_eq!(finish(&mut renderer), None);
}

#[test]
fn actual_ink_pick_uses_coverage_depth_density_and_bounded_coalescing() {
    let mut scene = Scene::demo(); scene.clear();
    let mesh = scene.insert(triangle(0.5)).unwrap();
    let mut point = session_rust::Point::new(0.0, 0.0, 0.2); point.width = 12.0;
    let point = scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(point)).unwrap()).unwrap();
    let mut line = session_rust::Line::new(-0.4, 0.4, 0.2, 0.4, 0.4, 0.2); line.width = 5.0;
    let line = scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(line)).unwrap()).unwrap();
    let mut path = session_rust::Polyline::new(vec![session_rust::Point::new(-0.4, -0.4, 0.2), session_rust::Point::new(0.4, -0.4, 0.2)]);
    path.width = 5.0;
    let path = scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(path)).unwrap()).unwrap();
    let mut renderer = renderer(&scene); let geometry = renderer.stats();
    for density in [1.0, 2.0] {
        renderer.set_density(density); renderer.resize(Viewport { width: (320.0 * density) as u32, height: (240.0 * density) as u32 });
        for (cursor, expected) in [([0.0, 0.0], point), ([0.0, 0.4], line), ([0.0, -0.4], path), ([0.0, 0.65], mesh)] {
            renderer.request_pick(cursor, identity()).unwrap(); assert_eq!(finish(&mut renderer).unwrap()[0], expected.value());
        }
    }
    assert_eq!(renderer.stats(), geometry, "Picking and resize reuse visible geometry");
    renderer.request_pick([0.0, 0.0], identity()).unwrap();
    for _ in 0..20 { renderer.request_pick([0.95, 0.95], identity()).unwrap(); }
    assert_eq!(&renderer.pick_usage()[..2], &[1, 1]); assert_eq!(renderer.pick_usage()[4], 19);
    renderer.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: Some(std::time::Duration::from_secs(5)) }).unwrap();
    assert!(renderer.take_pick(identity()).is_none(), "A newer cursor supersedes the first result");
    assert_eq!(finish(&mut renderer), None);
    assert_eq!(&renderer.pick_usage()[..2], &[0, 0]);
    assert!(renderer.pick_usage()[2] <= 25 * 256 + 31 * 31 * 16 + 192 + 3 * 52, "Readback does not grow with the full drawing");
}

#[test]
fn cancelled_readback_cannot_select_a_new_document_or_keep_its_source_alive() {
    let mut scene = Scene::demo(); scene.clear(); let old = scene.insert(triangle(0.2)).unwrap();
    let weak = Rc::downgrade(&scene.objects[0].mesh); let mut renderer = renderer(&scene);
    renderer.request_pick([0.0, 0.0], identity()).unwrap(); scene.clear(); renderer.set_scene(&scene, None);
    assert!(weak.upgrade().is_none()); assert_eq!(&renderer.pick_usage()[..3], &[0, 0, 160]);
    let new = scene.insert(triangle(0.3)).unwrap(); assert_ne!(old, new); renderer.set_scene(&scene, None);
    renderer.request_pick([0.0, 0.0], identity()).unwrap(); assert_eq!(finish(&mut renderer), Some([new.value(), 0]));
    assert!(renderer.take_pick(identity()).is_none(), "The cancelled map has no result for the replacement");
    let uploads = renderer.stats(); renderer.request_pick([0.0, 0.0], identity()).unwrap(); renderer.cancel_pick();
    renderer.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: Some(std::time::Duration::from_secs(5)) }).unwrap();
    assert!(renderer.take_pick(identity()).is_none()); assert_eq!(renderer.stats(), uploads);
}

#[test]
fn a_real_readback_from_an_old_camera_cannot_select_in_the_current_view() {
    let mut scene = Scene::demo(); scene.clear(); scene.insert(triangle(0.2)).unwrap(); let mut renderer = renderer(&scene);
    renderer.request_pick([0.0, 0.0], identity()).unwrap();
    renderer.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: Some(std::time::Duration::from_secs(5)) }).unwrap();
    let mut moved = identity(); moved[12] = 0.2;
    assert!(renderer.take_pick(moved).is_none(), "Same scene, different camera: no accepted hit");
    assert_eq!(&renderer.pick_usage()[..2], &[0, 0]);
    let submitted = renderer.pick_usage()[3]; let mut invalid = moved; invalid[0] = f32::NAN;
    assert!(renderer.request_pick([0.0, 0.0], invalid).is_err()); assert_eq!(renderer.pick_usage()[3], submitted);
    renderer.request_pick([0.0, 0.0], identity()).unwrap(); assert!(finish(&mut renderer).is_some());
    renderer.request_pick([0.0, 0.0], identity()).unwrap(); renderer.set_density(2.0);
    renderer.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: Some(std::time::Duration::from_secs(5)) }).unwrap();
    assert!(renderer.take_pick(identity()).is_none(), "Density changed while the map was pending");
}

#[test]
fn actual_face_and_edge_ids_preserve_sparse_source_keys_and_exclude_a_rendering_diagonal() {
    let mut source = session_rust::Mesh::new();
    for (key, p) in [(10, [-0.5, -0.5, 0.5]), (20, [0.5, -0.5, 0.5]), (30, [0.5, 0.5, 0.5]), (40, [-0.5, 0.5, 0.5])] {
        source.add_vertex(session_rust::Point::new(p[0], p[1], p[2]), Some(key));
    }
    source.add_face(vec![10, 20, 30, 40], Some(37)); source.set_face_triangulation(37, vec![[10, 20, 40], [20, 30, 40]]);
    source.set_facecolors(vec![session_rust::Color::new(0.8, 0.2, 0.1, 1.)]);
    let mut scene = Scene::demo(); scene.clear(); let id = scene.insert(PreparedMesh::new(source).unwrap()).unwrap(); let mut renderer = renderer(&scene);
    let before = renderer.stats(); renderer.set_pick_mode(crate::pick_sources::Mode::Face);
    renderer.request_pick([0., 0.], identity()).unwrap(); let hit = finish(&mut renderer).unwrap();
    assert_eq!(renderer.resolve_pick(&scene, hit), Some(crate::pick_sources::Target { owner: id, part: crate::pick_sources::Part::Face(37) }));
    renderer.set_pick_mode(crate::pick_sources::Mode::Edge); renderer.request_pick([0., -0.5], identity()).unwrap(); let hit = finish(&mut renderer).unwrap();
    assert_eq!(renderer.resolve_pick(&scene, hit), Some(crate::pick_sources::Target { owner: id, part: crate::pick_sources::Part::MeshEdge(10, 20) }));
    renderer.request_pick([0., 0.], identity()).unwrap(); assert!(finish(&mut renderer).is_none(), "The triangulation diagonal is not an editable edge");
    renderer.set_pick_mode(crate::pick_sources::Mode::Object); renderer.request_pick([0., 0.], identity()).unwrap();
    assert_eq!(finish(&mut renderer).unwrap()[0], id.value()); assert_eq!(renderer.stats(), before);
}

#[test]
fn actual_control_hits_resolve_the_original_curve_control_instead_of_a_sample() {
    let mut editor = crate::editor::Editor::default(); editor.apply(crate::editor::Action::Close).unwrap();
    editor.apply(crate::editor::Action::AddCurve).unwrap(); editor.apply(crate::editor::Action::Controls(true)).unwrap();
    let id = editor.scene.paths()[0].id; let mut renderer = renderer(&editor.scene);
    renderer.set_pick_mode(crate::pick_sources::Mode::Control); renderer.request_pick([0., 0.6], identity()).unwrap();
    let hit = finish(&mut renderer).unwrap(); assert_eq!(renderer.resolve_pick(&editor.scene, hit), Some(crate::pick_sources::Target { owner: id, part: crate::pick_sources::Part::Control(1) }));
    renderer.set_pick_mode(crate::pick_sources::Mode::Edge); renderer.request_pick([0., 0.1], identity()).unwrap();
    let hit = finish(&mut renderer).unwrap(); assert_eq!(renderer.resolve_pick(&editor.scene, hit), Some(crate::pick_sources::Target { owner: id, part: crate::pick_sources::Part::Curve }));
    editor.scene.show_controls(false); renderer.set_scene(&editor.scene, None);
    renderer.set_pick_mode(crate::pick_sources::Mode::Control); renderer.request_pick([0., 0.6], identity()).unwrap(); assert!(finish(&mut renderer).is_none());
}

fn colour_pixels(renderer: &mut Renderer) -> Vec<u8> {colour_pixels_at(renderer,identity())}
fn colour_pixels_at(renderer:&mut Renderer,transform:[f32;16])->Vec<u8> {colour_pixels_display(renderer,transform,false)}
fn colour_pixels_display(renderer:&mut Renderer,transform:[f32;16],normals:bool)->Vec<u8> {
    let texture = renderer.device.create_texture(&wgpu::TextureDescriptor { label: Some("selection pixels"),
        size: wgpu::Extent3d { width: 320, height: 240, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1,
        dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC, view_formats: &[] });
    renderer.draw_mode(&texture.create_view(&Default::default()), &Default::default(), &transform, normals);
    let bytes = renderer.device.create_buffer(&wgpu::BufferDescriptor { label: Some("selection readback"), size: 320 * 240 * 4,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
    let mut encoder = renderer.device.create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(texture.as_image_copy(), wgpu::TexelCopyBufferInfo { buffer: &bytes,
        layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(1280), rows_per_image: Some(240) } }, texture.size());
    renderer.queue.submit([encoder.finish()]); let (send, receive) = std::sync::mpsc::channel();
    bytes.slice(..).map_async(wgpu::MapMode::Read, move |result| { let _ = send.send(result); });
    renderer.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: Some(std::time::Duration::from_secs(5)) }).unwrap();
    receive.recv_timeout(std::time::Duration::from_secs(5)).unwrap().unwrap();
    let result = bytes.slice(..).get_mapped_range().to_vec(); bytes.unmap(); result
}

#[test]
fn actual_part_highlights_are_local_leave_black_edges_and_release_derived_data() {
    use crate::pick_sources::{Part, Target};
    let mut source = session_rust::Mesh::new();
    for (key, p) in [(10, [-0.8, -0.6, 0.5]), (20, [0., -0.6, 0.5]), (30, [0., 0.6, 0.5]), (40, [-0.8, 0.6, 0.5]),
                     (50, [0.8, -0.6, 0.5]), (60, [0.8, 0.6, 0.5])] {
        source.add_vertex(session_rust::Point::new(p[0], p[1], p[2]), Some(key));
    }
    source.add_face(vec![10, 20, 30, 40], Some(37)); source.add_face(vec![20, 50, 60, 30], Some(91));
    let mut scene = Scene::demo(); scene.clear(); let owner = scene.insert(PreparedMesh::new(source).unwrap()).unwrap();
    let weak = Rc::downgrade(&scene.objects()[0].mesh); let mut renderer = renderer(&scene); let stats = renderer.stats();
    let before = colour_pixels(&mut renderer);
    renderer.set_highlight(&scene, Some(Target { owner, part: Part::Face(37) })).unwrap();
    let face = colour_pixels(&mut renderer); assert!(renderer.highlight_bytes() > 0); let mut changed = 0;
    for (index, (a, b)) in before.chunks_exact(4).zip(face.chunks_exact(4)).enumerate() { if a != b {
        let x = index % 320; let y = index / 320; assert!((31..=160).contains(&x) && (47..=192).contains(&y)); changed += 1;
    } }
    assert!(changed > 10000, "The original face, rather than the whole object, is visibly selected");
    renderer.set_highlight(&scene, Some(Target { owner, part: Part::MeshEdge(10, 20) })).unwrap();
    let edge = colour_pixels(&mut renderer); let mut changed = 0; let mut black = 0;
    for (index, (a, b)) in before.chunks_exact(4).zip(edge.chunks_exact(4)).enumerate() {
        let x = index % 320; let y = index / 320;
        if a != b { assert!((30..=161).contains(&x) && (190..=193).contains(&y)); changed += 1; }
        if (35..150).contains(&x) && (191..=192).contains(&y) && b[0] < 160 && b[1] < 140 && b[2] < 80 { black += 1; }
    }
    assert!(changed > 80 && black > 80, "A yellow fringe surrounds the original black edge: changed={changed}, dark={black}");
    assert_eq!(renderer.stats(), stats, "Highlights reuse primary geometry");
    renderer.set_highlight(&scene, None).unwrap(); assert_eq!(renderer.highlight_bytes(), 0); assert_eq!(colour_pixels(&mut renderer), before);
    scene.clear(); renderer.set_scene(&scene, None); assert!(weak.upgrade().is_none()); assert_eq!(renderer.highlight_bytes(), 0);
}

#[test]
fn actual_control_highlight_surrounds_only_the_original_marker_and_keeps_source_precision() {
    let mut editor = crate::editor::Editor::default(); editor.apply(crate::editor::Action::Close).unwrap();
    editor.apply(crate::editor::Action::AddCurve).unwrap(); editor.apply(crate::editor::Action::Controls(true)).unwrap();
    let owner = editor.scene.paths()[0].id; let original = editor.scene.paths()[0].points(); let mut renderer = renderer(&editor.scene);
    let before = colour_pixels(&mut renderer); renderer.set_highlight(&editor.scene, Some(crate::pick_sources::Target {
        owner, part: crate::pick_sources::Part::Control(1) })).unwrap(); let after = colour_pixels(&mut renderer); let mut changed = 0;
    for (index, (a, b)) in before.chunks_exact(4).zip(after.chunks_exact(4)).enumerate() { if a != b {
        let x = index % 320; let y = index / 320; assert!((151..=168).contains(&x) && (39..=56).contains(&y)); changed += 1;
    } }
    assert!(changed > 20, "The halo belongs to the original control rather than a curve sample");
    assert_eq!(editor.scene.paths()[0].points(), original);
    editor.apply(crate::editor::Action::Close).unwrap(); renderer.set_scene(&editor.scene, None); assert_eq!(renderer.highlight_bytes(), 0);
}

#[test]
fn actual_multiple_mesh_selection_reuses_geometry_and_pick_ownership() {
    let mut scene = Scene::demo(); scene.clear();
    let panel = |x: f64| PreparedMesh::triangle([[x - 0.25, -0.4, 0.5], [x + 0.25, -0.4, 0.5], [x, 0.4, 0.5]], [0.5, 0.5, 0.5]).unwrap();
    let a = scene.insert(panel(-0.4)).unwrap(); let b = scene.insert(panel(0.4)).unwrap(); let mut renderer = renderer(&scene);
    let baseline = colour_pixels(&mut renderer); let geometry = renderer.stats();
    renderer.select_objects(&scene, &[a, b]).unwrap(); let selected = colour_pixels(&mut renderer);
    let pixel = |bytes: &[u8], x: usize| bytes[(120 * 320 + x) * 4..(120 * 320 + x) * 4 + 3].to_vec();
    for x in [96, 224] { let p = pixel(&selected, x); assert!(p[0] > 0 && p[0] == p[1] && p[2] == 0); }
    renderer.select_objects(&scene, &[b]).unwrap(); let one = colour_pixels(&mut renderer);
    assert_eq!(pixel(&one, 96), pixel(&baseline, 96)); assert_eq!(pixel(&one, 224), pixel(&selected, 224));
    assert_eq!(&renderer.stats()[..2], &geometry[..2], "Only selected settings change, not primary geometry");
    for (x, id) in [(-0.4, a), (0.4, b)] { renderer.request_pick([x, 0.], identity()).unwrap(); assert_eq!(finish(&mut renderer).unwrap()[0], id.value()); }
}

#[test]
fn actual_tool_preview_is_temporary_bounded_and_absent_from_object_picking() {
    let mut scene = Scene::demo(); scene.clear(); let mut renderer = renderer(&scene);
    let before = colour_pixels(&mut renderer); let stats = renderer.stats();
    let mut draft = crate::tool::Draft::new(crate::tool::Shape::Line);
    draft.submit([-0.5, 0., 0.2]).unwrap(); draft.hover(Some([0.5, 0., 0.2])).unwrap();
    renderer.set_preview(Some(&draft)).unwrap(); assert_eq!(renderer.preview_bytes(), 76);
    let after = colour_pixels(&mut renderer);
    let changed = before.chunks_exact(4).zip(after.chunks_exact(4)).filter(|(a,b)| a!=b).count();
    assert!(changed > 100, "The actual unfinished rubber band and cursor render");
    let p = &after[(120*320+160)*4..(120*320+160)*4+3]; assert_eq!(p, &[255,255,0], "The preview uses current selection yellow: {p:?}");
    renderer.request_pick([0., 0.], identity()).unwrap(); assert!(finish(&mut renderer).is_none(), "Drafts have no saved-object pick owner");
    assert_eq!(renderer.stats(), stats); assert!(scene.lines().is_empty());
    renderer.set_preview(None).unwrap(); assert_eq!(renderer.preview_bytes(), 0); assert_eq!(colour_pixels(&mut renderer), before);
    renderer.set_preview(Some(&draft)).unwrap(); scene.clear(); renderer.set_scene(&scene, None); assert_eq!(renderer.preview_bytes(), 0);
}

#[test]
fn actual_layer_colours_change_display_settings_without_editing_or_reuploading_original_geometry() {
    let mut scene=Scene::demo();scene.clear();let id=scene.insert(triangle(0.5)).unwrap();let source=Rc::clone(scene.objects()[0].geometry().unwrap());let original=source.to_proto();
    let mut renderer=renderer(&scene);let before=colour_pixels(&mut renderer);let allocations=renderer.stats();
    Rc::make_mut(&mut scene.view).faces.insert(id,[0,0,255]);renderer.set_scene(&scene,None);let blue=colour_pixels(&mut renderer);
    assert_ne!(blue,before);let centre=(120*320+160)*4;assert!(blue[centre+2]>blue[centre].saturating_add(100)&&blue[centre+2]>blue[centre+1].saturating_add(100));
    assert_eq!(&renderer.stats()[..2],&allocations[..2],"Layer colour reuses primary geometry and its settings allocation");assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
    Rc::make_mut(&mut scene.view).edges.insert(id,[0,255,0]);renderer.set_scene(&scene,None);let green_edges=colour_pixels(&mut renderer);let count=green_edges.chunks_exact(4).filter(|p|p[1]>p[0].saturating_add(100)&&p[1]>p[2].saturating_add(100)).count();assert!(count>100,"Actual source boundary colour is independent: {count}");
    Rc::make_mut(&mut scene.view).faces.remove(&id);renderer.set_scene(&scene,None);let restored_faces=colour_pixels(&mut renderer);assert_eq!(&restored_faces[centre..centre+4],&before[centre..centre+4]);assert_ne!(restored_faces,before,"Restoring Faces leaves the independent edge override");
    Rc::make_mut(&mut scene.view).edges.remove(&id);renderer.set_scene(&scene,None);assert_eq!(colour_pixels(&mut renderer),before);assert!(Rc::ptr_eq(&source,scene.objects()[0].geometry().unwrap()));
}

#[test]
fn actual_delayed_gpu_answer_finishes_a_released_drag_and_reuses_its_geometry() {
    use crate::editor::{Editor,Action,Change};
    let mut e=Editor::default();e.scene.clear();let id=e.scene.insert(triangle(0.2)).unwrap();let original=e.scene.placement(id).unwrap().m;let source=e.scene.objects()[0].geometry().unwrap().clone();
    let mut renderer=renderer(&e.scene);let transform=e.camera.uniform();let gpu=renderer.stats();let size=[320.,240.];
    e.apply(Action::CanvasStart {from:[0.,0.],to:[0.2,0.],size,modified:false,pointer:1}).unwrap();renderer.request_pick([0.,0.],transform).unwrap();
    e.apply(Action::CanvasRelease {screen:[0.4,0.],size,add:false,remove:false}).unwrap();assert_eq!(e.scene.placement(id).unwrap().m,original);
    renderer.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();let hit=renderer.take_pick(transform).unwrap().unwrap().and_then(|pixel|renderer.resolve_pick(&e.scene,pixel));assert_eq!(hit.unwrap().owner,id);
    assert_eq!(e.apply(Action::CanvasHit(hit.map(|target|target.owner))).unwrap(),Change::Scene);assert_ne!(e.scene.placement(id).unwrap().m,original);assert!(Rc::ptr_eq(&source,e.scene.objects()[0].geometry().unwrap()));
    renderer.set_scene(&e.scene,e.selected);assert_eq!(&renderer.stats()[..2],&gpu[..2]);e.apply(Action::Undo).unwrap();assert_eq!(e.scene.placement(id).unwrap().m,original);
}

#[test]
fn actual_surface_pick_keeps_one_cad_face_and_four_domain_edge_ids() {
    use crate::pick_sources::{Mode,Part};
    let mut scene=Scene::demo();scene.clear();let source=Rc::new(crate::surface::example());let _=source.guid();let before=source.to_proto();let id=scene.insert_surface(source.clone()).unwrap();let mut renderer=renderer(&scene);let camera=crate::camera::Camera::default();let transform=camera.uniform();let gpu=renderer.stats();
    let finish_at=|renderer:&mut Renderer|{renderer.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();renderer.take_pick(transform).unwrap().unwrap().and_then(|hit|renderer.resolve_pick(&scene,hit))};
    renderer.set_pick_mode(Mode::Face);renderer.request_pick([0.,0.],transform).unwrap();assert_eq!(finish_at(&mut renderer).unwrap().part,Part::Face(0));
    renderer.set_pick_mode(Mode::Edge);for (point,edge) in [([-1.,0.,0.],0),([1.,0.,0.],1),([0.,-1.,0.],2),([0.,1.,0.],3)] {
        let point=camera.view_projection().transform_point(&session_rust::Point::new(point[0],point[1],point[2]));renderer.request_pick([point[0] as f32,point[1] as f32],transform).unwrap();let target=finish_at(&mut renderer).unwrap();assert_eq!(target.owner,id);assert_eq!(target.part,Part::SurfaceEdge(edge));
    }
    assert_eq!(renderer.stats(),gpu);assert_eq!(source.to_proto(),before);
}

#[test]
fn actual_cad_hole_has_no_face_pick_and_integer_faces_retain_original_identity() {
    use crate::pick_sources::{Mode,Part};
    let mut scene=Scene::demo();scene.clear();let mut original=session_rust::BRep::create_block_with_hole(1.6,1.2,0.24,0.26);original.surfacecolor=session_rust::Color::new(0.35,0.65,0.85,1.);let source=Rc::new(original);let _=source.guid();let before=source.to_proto();let id=scene.insert_cad(source.clone()).unwrap();let mut renderer=renderer(&scene);let camera=crate::camera::Camera::default();let transform=camera.uniform();let gpu=renderer.stats();
    let finish_at=|renderer:&mut Renderer|{renderer.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();renderer.take_pick(transform).unwrap().unwrap().and_then(|hit|renderer.resolve_pick(&scene,hit))};
    renderer.set_pick_mode(Mode::Face);renderer.request_pick([0.,0.],transform).unwrap();assert!(finish_at(&mut renderer).is_none(),"The actual shaded hole stays empty");
    let p=camera.view_projection().transform_point(&session_rust::Point::new(0.5,0.,0.12));renderer.request_pick([p[0] as f32,p[1] as f32],transform).unwrap();let target=finish_at(&mut renderer).unwrap();assert_eq!(target.owner,id);assert_eq!(target.part,Part::Face(6),"The visible top cap retains its original CAD identity");renderer.set_pick_mode(Mode::Edge);let mut seen=0;
    for border in &scene.objects()[0].cad.as_ref().unwrap().edges {
        let points=&border.points;if !points.iter().all(|p|(p[2]-0.12).abs()<1e-10) {continue;}
        let middle=points.windows(2).nth((points.len()-1)/2).unwrap();let p=std::array::from_fn::<_,3,_>(|i|(middle[0][i]+middle[1][i])*0.5);let p=camera.view_projection().transform_point(&session_rust::Point::new(p[0],p[1],p[2]));renderer.request_pick([p[0] as f32,p[1] as f32],transform).unwrap();assert_eq!(finish_at(&mut renderer).unwrap_or_else(||panic!("Missing original CAD edge {} at {:?}",border.edge,p)).part,Part::BRepEdge(border.edge));seen+=1;
    }
    assert_eq!(seen,5,"All four outer edges and the original hole rim resolve");assert_eq!(renderer.stats(),gpu);assert_eq!(source.to_proto(),before);
}

#[test]
fn actual_screen_text_overlay_does_not_create_scene_pick_owners_or_reupload_geometry() {
    let mut scene=Scene::demo();scene.clear();let id=scene.insert(triangle(0.2)).unwrap();let original=scene.objects()[0].geometry().unwrap().to_proto();let mut renderer=renderer(&scene);let before=colour_pixels(&mut renderer);let gpu=renderer.stats();renderer.text_demo().unwrap();let with_text=colour_pixels(&mut renderer);assert_ne!(with_text,before);assert_eq!(renderer.text_error(),None);assert_eq!(renderer.stats(),gpu);assert_eq!(scene.objects().len(),1);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
    renderer.request_pick([0.,0.],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],id.value());renderer.request_pick([-0.85,0.8],identity()).unwrap();assert!(finish(&mut renderer).is_none());assert_eq!(renderer.text_usage()[0],1);renderer.set_text(Vec::new()).unwrap();assert_eq!(renderer.text_usage(),[0;6]);assert_eq!(colour_pixels(&mut renderer),before);
}

#[test]
fn actual_owned_text_plane_respects_depth_integer_picks_selection_cache_and_close() {
    use crate::{scene_label::{Prepared,example},text::TextPlacement};
    let mut scene=Scene::demo();scene.clear();let mesh=scene.insert(triangle(0.2)).unwrap();let mut renderer=renderer(&scene);let before=colour_pixels(&mut renderer);let gpu=renderer.stats();
    let mut source=example(TextPlacement::WorldPlane {world:[-0.1,0.05,0.6],right:[1.,0.,0.],up:[0.,1.,0.],world_height:0.08});source.label.text="Hi".into();let source=Rc::new(source);let original=serde_json::to_string(&*source).unwrap();let weak=Rc::downgrade(&source);let id=scene.insert_text(Prepared::new(source.clone()).unwrap()).unwrap();drop(source);renderer.set_scene(&scene,None);
    assert_eq!(colour_pixels(&mut renderer),before,"Actual fixed text behind the opaque solid is hidden");renderer.request_pick([-0.06,0.02],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],mesh.value());
    scene.place_text(id,session_rust::Xform::translation(0.,0.,-0.45)).unwrap();renderer.set_scene(&scene,None);let front=colour_pixels(&mut renderer);assert_ne!(front,before);assert_eq!(renderer.text_error(),None);assert!(front.chunks_exact(4).filter(|p|p[0]<10&&p[1]<10&&p[2]<10).count()>20);let raster=renderer.text_scene_usage()[2];assert_eq!(raster,1,"Placement reuses already rasterized original words");assert_eq!(renderer.text_usage()[0],1);
    renderer.request_pick([-0.06,0.02],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap(),[id.value(),0]);renderer.select_objects(&scene,&[id]).unwrap();let selected=colour_pixels(&mut renderer);assert!(selected.chunks_exact(4).filter(|p|p[0]>200&&p[1]>200&&p[2]<40).count()>20);assert_eq!(renderer.text_scene_usage()[2],raster);assert_eq!(renderer.text_usage()[0],1);assert_eq!(&renderer.stats()[..2],&gpu[..2]);assert_eq!(serde_json::to_string(&*scene.texts()[0].prepared.source).unwrap(),original);
    Rc::make_mut(&mut scene.view).lock(&[id],true);renderer.set_scene(&scene,None);renderer.request_pick([-0.06,-0.008],identity()).unwrap();assert!(finish(&mut renderer).is_none(),"The locked plate remains an occluder without a selectable id");Rc::make_mut(&mut scene.view).hide(&[id],true);renderer.set_scene(&scene,None);renderer.request_pick([-0.06,0.02],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],mesh.value());
    scene.clear();renderer.set_scene(&scene,None);assert!(weak.upgrade().is_none());assert_eq!(renderer.text_scene_usage(),[0;4]);assert_eq!(renderer.text_usage(),[0;6]);
}

#[test]
fn actual_anchor_nameplate_and_billboard_follow_the_current_depth_and_plate_rules() {
    use crate::{scene_label::{Prepared,example},text::TextPlacement};
    for (placement,overlay) in [(TextPlacement::Anchor {world:[0.,0.,0.6],offset:[0.;2]},false),(TextPlacement::Nameplate {world:[0.,0.,0.6],padding:[17.,4.],rounded:true},true),(TextPlacement::WorldBillboard {world:[0.,0.,0.6],world_height:0.08},false)] {
        let mut scene=Scene::demo();scene.clear();let mesh=scene.insert(triangle(0.2)).unwrap();let mut renderer=renderer(&scene);let before=colour_pixels(&mut renderer);let mut source=example(placement);source.label.text="Hi".into();let id=scene.insert_text(Prepared::new(Rc::new(source)).unwrap()).unwrap();renderer.set_scene(&scene,None);let after=colour_pixels(&mut renderer);assert_eq!(after!=before,overlay,"Only the nameplate overlays its own solid");renderer.request_pick([0.,0.],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],if overlay {id.value()} else {mesh.value()});assert_eq!(renderer.text_error(),None);
        let fonts=vec![crate::text::FONT_BYTES.to_vec(),crate::text::FALLBACK_BYTES.to_vec(),crate::text::SYMBOL_BYTES.to_vec()];assert!(renderer.replace_text_fonts(vec![vec![0;8]]).is_err());assert_eq!(colour_pixels(&mut renderer),after);renderer.replace_text_fonts(fonts).unwrap();assert_eq!(colour_pixels(&mut renderer),after);assert_eq!(renderer.text_usage()[5],1);
    }
}

#[test]
fn actual_world_text_clip_uses_canvas_coordinates_in_the_small_integer_pick_window() {
    use crate::{scene_label::{Prepared,example},text::TextPlacement};
    let mut scene=Scene::demo();scene.clear();let mut source=example(TextPlacement::WorldPlane {world:[-0.2,0.1,0.3],right:[1.,0.,0.],up:[0.,1.,0.],world_height:0.2});source.label.text="Hi".into();source.label.clip=Some([140.,100.,160.,140.]);let id=scene.insert_text(Prepared::new(Rc::new(source)).unwrap()).unwrap();let mut renderer=renderer(&scene);let before=colour_pixels(&mut renderer);assert_eq!(renderer.text_error(),None);
    let mut clean=Scene::demo();clean.clear();let mut no_text=super::pick_gpu_tests::renderer(&clean);let baseline=colour_pixels(&mut no_text);let mut changed=0;
    for (i,(p,b)) in before.chunks_exact(4).zip(baseline.chunks_exact(4)).enumerate() {let (x,y)=(i%320,i/320);if x<140||x>=160||y<100||y>=140 {assert_eq!(p,b,"Clipped text does not affect pixels outside the CSS clip");} else {changed+=usize::from(p!=b);}}
    assert!(changed>50);renderer.request_pick([-0.0625,0.],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],id.value());renderer.request_pick([0.125,0.],identity()).unwrap();assert!(finish(&mut renderer).is_none());
}

#[test]
fn actual_selected_silhouette_has_one_union_no_internal_diagonal_and_keeps_source_and_picks() {
    let mut scene=Scene::demo();scene.clear();let mesh=|x:f64|{let mut source=session_rust::Mesh::create_box(0.6,0.8,0.1);source.set_objectcolor(session_rust::Color::new(0.5,0.5,0.5,1.));let mut prepared=PreparedMesh::new(source).unwrap();prepared.model=session_rust::Xform::translation(x,0.,0.5);prepared};let a=scene.insert(mesh(-0.3)).unwrap();let b=scene.insert(mesh(0.3)).unwrap();let sources=scene.objects().iter().map(|row|row.geometry().unwrap().to_proto()).collect::<Vec<_>>();let mut renderer=renderer(&scene);let geometry=renderer.stats();renderer.select_objects(&scene,&[a,b]).unwrap();let first=colour_pixels(&mut renderer);let alpha=renderer.outline_alpha();assert_eq!(alpha.len(),320*240);assert!(alpha.iter().filter(|v|**v>0).count()>300);for y in 90..150 {for x in 90..230 {assert_eq!(alpha[y*320+x],0,"The combined coverage has no outline along the shared object join or tessellation diagonals");}}
    let stats=renderer.outline_usage();assert_eq!(colour_pixels(&mut renderer),first);assert_eq!(renderer.outline_alpha(),alpha);assert_eq!(renderer.outline_usage()[1],stats[1]);assert!(renderer.outline_usage()[2]>stats[2]);assert_eq!(&renderer.stats()[..2],&geometry[..2]);assert_eq!(scene.objects().iter().map(|row|row.geometry().unwrap().to_proto()).collect::<Vec<_>>(),sources);renderer.request_pick([-0.3,0.],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],a.value());renderer.request_pick([0.3,0.],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],b.value());
    renderer.select_objects(&scene,&[]).unwrap();colour_pixels(&mut renderer);assert!(renderer.outline_alpha().is_empty());assert_eq!(renderer.outline_usage()[0],0);
}

#[test]
fn actual_surface_outline_culls_occluded_selection_and_selected_edge_ink_cannot_narrow_its_alpha() {
    use crate::pick_sources::{Target,Part};
    let mut scene=Scene::demo();scene.clear();let back=scene.insert(triangle(0.6)).unwrap();let front=scene.insert(triangle(0.2)).unwrap();let mut renderer=renderer(&scene);renderer.select_objects(&scene,&[back]).unwrap();colour_pixels(&mut renderer);assert!(renderer.outline_alpha().iter().all(|v|*v==0),"A selected surface wholly behind the current opaque depth adds no silhouette");renderer.select_objects(&scene,&[front]).unwrap();let plain=colour_pixels(&mut renderer);let alpha=renderer.outline_alpha();assert!(alpha.iter().any(|v|*v>128));let edge=scene.objects().iter().find(|row|row.id==front).unwrap().mesh.parts.edges[0];renderer.set_highlight(&scene,Some(Target {owner:front,part:Part::MeshEdge(edge.0,edge.1)})).unwrap();let inked=colour_pixels(&mut renderer);assert_eq!(renderer.outline_alpha(),alpha,"Source edge highlighting cannot change the actual outline width");let mut opaque=0;for ((a,b),coverage) in plain.chunks_exact(4).zip(inked.chunks_exact(4)).zip(&alpha) {if *coverage>=250&&a[..3].iter().all(|c|*c<8) {opaque+=1;assert!(b[..3].iter().all(|c|*c<12),"Yellow source edge ink cannot paint over black outline pixels");}}assert!(opaque>100);scene.clear();renderer.set_scene(&scene,None);colour_pixels(&mut renderer);assert_eq!(renderer.outline_usage()[0],0);
}

#[test]
fn actual_curved_and_flat_silhouettes_keep_the_same_css_width_at_two_densities() {
    let mut widths=Vec::new();
    for curved in [false,true] {
        let mut scene=Scene::demo();scene.clear();let id=if curved {scene.insert_cad(Rc::new(session_rust::BRep::create_cylinder(0.4,0.8))).unwrap()} else {let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.8,0.8,0.2)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,0.5);scene.insert(prepared).unwrap()};let mut renderer=renderer(&scene);renderer.select_objects(&scene,&[id]).unwrap();let geometry=renderer.stats();let mut css=Vec::new();
        for density in [1.,2.] {renderer.set_density(density);colour_pixels(&mut renderer);let alpha=renderer.outline_alpha();assert!(alpha.iter().any(|v|*v==255));let row=&alpha[120*320..121*320];let width=row[160..].iter().map(|v|*v as f64/255.).sum::<f64>()/density as f64;assert!((0.7..1.7).contains(&width),"Actual outer silhouette CSS coverage remains bounded: {width}");css.push(width);assert_eq!(&renderer.stats()[..2],&geometry[..2]);}
        assert!((css[0]-css[1]).abs()<0.5,"DPI keeps the outline in CSS units: {css:?}");widths.push(css);
    }
    for i in 0..2 {assert!((widths[0][i]-widths[1][i]).abs()<0.25,"A smooth cylinder and flat box use one coverage width: {widths:?}");}
}

#[test]
fn actual_selected_bore_keeps_its_open_center_and_black_inner_contour() {
    let source=Rc::new(session_rust::BRep::create_block_with_hole(1.2,1.2,0.4,0.25));
    let _=source.guid();let original=source.to_proto();let mut scene=Scene::demo();scene.clear();
    let id=scene.insert_cad(source.clone()).unwrap();let mut renderer=renderer(&scene);
    renderer.select_objects(&scene,&[id]).unwrap();let pixels=colour_pixels(&mut renderer);
    let alpha=renderer.outline_alpha();
    for y in 112..128 {for x in 152..168 {assert_eq!(alpha[y*320+x],0,"The bore center stays open");}}
    let mut inner=0;
    for y in 80..160 {for x in 110..210 {
        let i=y*320+x;
        if alpha[i]>=250 {inner+=1;assert!(pixels[i*4..i*4+3].iter().all(|c|*c<12),"The actual inner silhouette is black");}
    }}
    assert!(inner>100,"The real bore boundary has an inner outline: {inner}");
    renderer.request_pick([0.,0.],identity()).unwrap();assert!(finish(&mut renderer).is_none());
    renderer.request_pick([0.4,0.],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],id.value());
    assert_eq!(source.to_proto(),original,"Coverage cannot change original topology");
}

#[test]
fn actual_cloud_pixels_and_whole_ids_keep_cached_source_uploads_density_and_locked_occlusion() {
    let mut source=session_rust::PointCloud::from_coords(vec![0.,0.,0.2],vec![255,0,0,255],vec![]);source.point_size=16.;let source=Rc::new(source);let _=source.guid();let before=source.to_proto();
    let mut scene=Scene::demo();scene.clear();let mesh=scene.insert(triangle(0.6)).unwrap();let cloud=scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap()).unwrap();let mut renderer=renderer(&scene);
    for density in [1.,2.] {renderer.set_density(density);let first=colour_pixels(&mut renderer);let center=&first[(120*320+160)*4..(120*320+160)*4+4];assert!(center[0]>240&&center[1]<10&&center[2]<10,"Original cloud color covers the solid behind it");let usage=renderer.cloud_usage();assert_eq!(usage[0],1);assert_eq!(usage[2],1);assert!(usage[1]>320*240*16);assert_eq!(colour_pixels(&mut renderer),first);assert_eq!(renderer.cloud_usage()[3],usage[3]);assert!(renderer.cloud_usage()[4]>usage[4]);renderer.request_pick([0.,0.],identity()).unwrap();assert_eq!(finish(&mut renderer),Some([cloud.value(),0]));}
    renderer.set_density(1.);Rc::make_mut(&mut scene.view).lock(&[cloud],true);renderer.set_scene(&scene,None);renderer.request_pick([0.,0.],identity()).unwrap();assert_eq!(finish(&mut renderer),None,"The locked cloud writes zero while occluding the mesh");
    Rc::make_mut(&mut scene.view).hide(&[cloud],true);renderer.set_scene(&scene,None);renderer.request_pick([0.,0.],identity()).unwrap();assert_eq!(finish(&mut renderer),Some([mesh.value(),0]));assert_eq!(source.to_proto(),before);
    scene.clear();renderer.set_scene(&scene,None);assert_eq!(renderer.cloud_usage(),[0;5]);
}
#[test]
fn actual_cloud_copy_and_normal_shading_preserve_one_source_and_release_all_derived_owners() {
    let source=Rc::new(session_rust::PointCloud::from_coords(vec![-0.3,0.,0.2,0.3,0.,0.2],vec![0,255,0,255,0,255,0,255],vec![0.,0.,1.,0.,0.,1.]));let weak=Rc::downgrade(&source);let mut scene=Scene::demo();scene.clear();let cloud=scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap()).unwrap();let mut renderer=renderer(&scene);let pixels=colour_pixels(&mut renderer);assert!(pixels.chunks_exact(4).any(|p|p[1]>160&&p[0]<10&&p[2]<10));assert_eq!(renderer.cloud_usage()[2],1);
    let copy=scene.copy_row(cloud,false).unwrap();scene.translate(copy,[0.,0.3,0.]).unwrap();renderer.set_scene(&scene,None);colour_pixels(&mut renderer);assert_eq!(renderer.cloud_usage()[2],1,"Copies share one original upload");assert_eq!(renderer.cloud_usage()[0],4);renderer.request_pick([-0.3,0.3],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],copy.value());drop(source);scene.clear();renderer.set_scene(&scene,None);assert_eq!(renderer.cloud_usage(),[0;5]);assert!(weak.upgrade().is_none());
}

fn camera_pixels(renderer: &mut Renderer,camera:&crate::camera::Camera) -> Vec<u8> {
    let texture = renderer.device.create_texture(&wgpu::TextureDescriptor { label: Some("selection pixels"),
        size: wgpu::Extent3d { width: 320, height: 240, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1,
        dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC, view_formats: &[] });
    renderer.cloud_camera(camera);renderer.draw_mode(&texture.create_view(&Default::default()), &Default::default(), &camera.uniform(), false);
    let bytes = renderer.device.create_buffer(&wgpu::BufferDescriptor { label: Some("selection readback"), size: 320 * 240 * 4,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
    let mut encoder = renderer.device.create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(texture.as_image_copy(), wgpu::TexelCopyBufferInfo { buffer: &bytes,
        layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(1280), rows_per_image: Some(240) } }, texture.size());
    renderer.queue.submit([encoder.finish()]); let (send, receive) = std::sync::mpsc::channel();
    bytes.slice(..).map_async(wgpu::MapMode::Read, move |result| { let _ = send.send(result); });
    renderer.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: Some(std::time::Duration::from_secs(5)) }).unwrap();
    receive.recv_timeout(std::time::Duration::from_secs(5)).unwrap().unwrap();
    let result = bytes.slice(..).get_mapped_range().to_vec(); bytes.unmap(); result
}

#[test]
fn actual_large_cloud_changes_detail_with_the_view_and_reuses_its_original_upload() {
    let source=Rc::new(crate::cloud::large_example());assert_eq!(source.len(),2_000_064);let mut scene=Scene::demo();scene.clear();scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap()).unwrap();let mut renderer=renderer(&scene);
    let mut camera=crate::camera::Camera::default();camera.aspect=320./240.;camera.projection=crate::camera::Projection::Orthographic;camera.distance=100./std::f64::consts::FRAC_PI_6.tan();
    let coarse=camera_pixels(&mut renderer,&camera);assert_eq!(renderer.cloud_usage()[0],64);let uploads=renderer.cloud_usage()[2];let prepared=renderer.cloud_usage()[3];assert_eq!(camera_pixels(&mut renderer,&camera),coarse);assert_eq!(renderer.cloud_usage()[3],prepared);
    camera.distance=0.7/std::f64::consts::FRAC_PI_6.tan();let fine=camera_pixels(&mut renderer,&camera);assert_eq!(renderer.cloud_usage()[0],2_000_064);assert_ne!(fine,coarse);assert_eq!(renderer.cloud_usage()[2],uploads);assert_eq!(source.point_id(64),64);assert_eq!(source.point_id(source.len()-1),2_000_063);
    camera.distance=100./std::f64::consts::FRAC_PI_6.tan();assert_eq!(camera_pixels(&mut renderer,&camera),coarse);assert_eq!(renderer.cloud_usage()[0],64);assert_eq!(renderer.cloud_usage()[2],uploads);
    scene.clear();renderer.set_scene(&scene,None);assert_eq!(renderer.cloud_usage(),[0;5]);
}

#[test]
fn actual_cloud_control_pick_preserves_full_u32_source_ids_and_local_highlight() {
    use crate::pick_sources::{Mode,Part};
    let mut proto=session_rust::PointCloud::from_coords(vec![-0.4,0.,0.2,0.4,0.,0.2],vec![],vec![]).to_proto();
    proto.point_size=16.;proto.point_ids=vec![u32::MAX,16_777_217];let source=Rc::new(session_rust::PointCloud::from_proto(proto));let _=source.guid();let original=source.to_proto();
    let mut scene=Scene::demo();scene.clear();let owner=scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap()).unwrap();scene.cloud_controls(Some(owner));let mut renderer=renderer(&scene);renderer.set_pick_mode(Mode::Control);
    let uploaded=renderer.cloud_usage()[2];let geometry=renderer.stats();
    for density in [1.,2.] {
        renderer.set_density(density);renderer.resize(Viewport {width:(320.*density) as u32,height:(240.*density) as u32});
        for (x,id) in [(-0.4,u32::MAX),(0.4,16_777_217)] {
            renderer.request_pick([x,0.],identity()).unwrap();let pixel=finish(&mut renderer).unwrap();let target=renderer.resolve_pick(&scene,pixel).unwrap();assert_eq!(target.part,Part::CloudPoint(id));assert_eq!(target.owner,owner);
            renderer.set_highlight(&scene,Some(target)).unwrap();assert!(renderer.highlight_bytes()>0);assert_eq!(renderer.stats(),geometry);
        }
    }
    assert_eq!(renderer.cloud_usage()[2],uploaded);assert_eq!(source.to_proto(),original);
    Rc::make_mut(&mut scene.view).lock(&[owner],true);renderer.set_density(1.);renderer.resize(Viewport {width:320,height:240});renderer.set_scene(&scene,None);
    renderer.request_pick([-0.4,0.],identity()).unwrap();assert!(finish(&mut renderer).is_none());
    scene.clear();renderer.set_scene(&scene,None);renderer.set_highlight(&scene,None).unwrap();assert_eq!(renderer.cloud_usage(),[0;5]);assert_eq!(renderer.highlight_bytes(),0);
}

#[test]
fn actual_resident_cloud_controls_query_original_points_outside_the_coarse_display_subset() {
    use crate::pick_sources::{Mode,Part};
    let mut proto=crate::cloud::large_example().to_proto();for xyz in proto.coords[..64*3].chunks_exact_mut(3) {xyz[2]=0.;}
    let source=Rc::new(session_rust::PointCloud::from_proto(proto));let mut scene=Scene::demo();scene.clear();let owner=scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap()).unwrap();let mut renderer=renderer(&scene);
    let mut camera=crate::camera::Camera::default();camera.aspect=320./240.;camera.projection=crate::camera::Projection::Orthographic;camera.distance=100./std::f64::consts::FRAC_PI_6.tan();
    camera_pixels(&mut renderer,&camera);assert_eq!(renderer.cloud_usage()[0],64);let uploads=renderer.cloud_usage()[2];
    scene.cloud_controls(Some(owner));renderer.set_scene(&scene,None);renderer.set_pick_mode(Mode::Control);camera_pixels(&mut renderer,&camera);assert_eq!(renderer.cloud_usage()[0],2_000_064);
    let transform=camera.view_projection().to_f32();let p=camera.view_projection().transform_point(&session_rust::Point::new(0.25,0.25,0.2));renderer.request_pick([p[0] as f32,p[1] as f32],transform).unwrap();renderer.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();
    let pixel=renderer.take_pick(transform).unwrap().unwrap().unwrap();let target=renderer.resolve_pick(&scene,pixel).unwrap();let Part::CloudPoint(id)=target.part else {panic!("Original cloud point identity");};assert!(id>=64);assert!(target.valid(&scene));assert_eq!(renderer.cloud_usage()[2],uploads);
    renderer.request_pick([0.,0.],transform).unwrap();renderer.cancel_pick();assert!(renderer.take_pick(transform).is_none());
    scene.cloud_controls(None);renderer.set_scene(&scene,None);renderer.set_pick_mode(Mode::Object);camera_pixels(&mut renderer,&camera);assert_eq!(renderer.cloud_usage()[0],64);assert_eq!(renderer.cloud_usage()[2],uploads);
    scene.clear();renderer.set_scene(&scene,None);assert_eq!(renderer.cloud_usage(),[0;5]);
}

#[test]
fn actual_streamed_pages_pick_original_ids_share_uploads_and_release_after_eviction() {
    use crate::pick_sources::{Mode,Part};
    let prepared=crate::range_example::prepared();let weak=Rc::downgrade(&prepared.source);
    let mut scene=Scene::empty();let owner=scene.insert_stream(prepared.clone(),session_rust::Xform::identity(),12.).unwrap();scene.cloud_controls(Some(owner));
    let mut renderer=renderer(&scene);renderer.set_pick_mode(Mode::Control);
    for (cursor,id) in [([-0.5,0.5],u32::MAX),([0.5,0.5],16_777_217),([-0.5,-0.5],10)] {
        renderer.request_pick(cursor,identity()).unwrap();let hit=finish(&mut renderer).expect("Actual streamed point coverage");let target=renderer.resolve_pick(&scene,hit).unwrap();assert_eq!(target.owner,owner);assert_eq!(target.part,Part::CloudPoint(id));
    }
    let pixels=colour_pixels(&mut renderer);assert!(pixels.chunks_exact(4).any(|p|p[2]>160&&p[0]<10));assert!(pixels.chunks_exact(4).any(|p|p[0]>160&&p[2]<10));
    let uploads=renderer.cloud_usage()[2];let copy=scene.copy_row(owner,false).unwrap();scene.translate(copy,[0.2,0.,0.]).unwrap();renderer.set_scene(&scene,None);colour_pixels(&mut renderer);assert_eq!(renderer.cloud_usage()[2],uploads,"Placed owners share original page uploads");
    Rc::make_mut(&mut scene.view).lock(&[owner,copy],true);renderer.set_scene(&scene,None);renderer.request_pick([-0.5,0.5],identity()).unwrap();assert!(finish(&mut renderer).is_none());
    Rc::make_mut(&mut scene.view).lock(&[owner,copy],false);let held=Rc::downgrade(&prepared.page(2).unwrap());prepared.pages.borrow_mut().clear();assert!(held.upgrade().is_some(),"GPU page owner retains precise pick identities");renderer.set_scene(&scene,None);assert!(held.upgrade().is_none());renderer.request_pick([-0.5,0.5],identity()).unwrap();assert!(finish(&mut renderer).is_none(),"Evicted detailed points are absent");
    renderer.request_pick([-0.5,-0.5],identity()).unwrap();let hit=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&scene,hit).unwrap().part,Part::CloudPoint(10));
    scene.clear();renderer.set_scene(&scene,None);drop(prepared);assert_eq!(renderer.cloud_usage(),[0;5]);assert!(weak.upgrade().is_none());
}

#[test]
fn actual_definition_instances_batch_one_geometry_keep_integer_picks_and_release_records() {
    let mut original=crate::instance_specimen::message();original.objects.as_mut().unwrap().instances[0].xform=None;original.objects.as_mut().unwrap().instances[1].flags=0;
    original.xforms=vec![session_rust::proto::XformEntry {guid:"first".into(),xform:Some(session_rust::Xform::translation(-0.5,0.,0.3).to_proto())},session_rust::proto::XformEntry {guid:"second".into(),xform:Some(session_rust::Xform::translation(0.5,0.,0.3).to_proto())}];
    let mut scene=Scene::empty();scene.next_id=16_777_217;crate::scene_document::import(&mut scene,&prost::Message::encode_to_vec(&original),"",&session_rust::Xform::identity(),0.).unwrap();let a=scene.objects()[0].id;let b=scene.objects()[1].id;let weak=Rc::downgrade(scene.objects()[0].geometry().unwrap());let mut renderer=renderer(&scene);assert_eq!(&renderer.instance_usage()[..3],&[1,2,376]);let source_uploads=renderer.stats()[0];let pixels=colour_pixels(&mut renderer);assert!(pixels.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(20)&&p[2]>p[1].saturating_add(10)).count()>1000);
    for (cursor,id) in [([-0.5,-0.1],a),([0.5,-0.1],b)] {renderer.request_pick(cursor,identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],id.value());}
    let mut coloured=original.clone();let first=&mut coloured.objects.as_mut().unwrap().instances[0];first.flags|=session_rust::InstanceRef::FLAG_COLOR;first.color=Some(session_rust::proto::Color {r:0.9,g:0.08,b:0.05,a:0.5,..Default::default()});let mut colour_scene=Scene::empty();crate::scene_document::import(&mut colour_scene,&prost::Message::encode_to_vec(&coloured),"",&session_rust::Xform::identity(),0.).unwrap();let mut colour_renderer=super::pick_gpu_tests::renderer(&colour_scene);let colours=colour_pixels(&mut colour_renderer);assert!(colours.chunks_exact(4).filter(|p|p[0]>p[1].saturating_add(50)&&p[0]>p[2].saturating_add(50)).count()>500);assert!(colours.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(20)).count()>500);
    let writes=renderer.instance_usage()[3];colour_pixels(&mut renderer);assert_eq!(renderer.instance_usage()[3],writes);scene.translate(a,[0.,0.2,0.]).unwrap();renderer.set_scene(&scene,None);assert_eq!(renderer.stats()[0],source_uploads);assert!(renderer.instance_usage()[3]>writes);renderer.select_objects(&scene,&[a]).unwrap();let selected=colour_pixels(&mut renderer);assert!(selected.chunks_exact(4).any(|p|p[0]>140&&p[1]>90&&p[2]<40));
    Rc::make_mut(&mut scene.view).hide(&[b],true);renderer.set_scene(&scene,None);assert_eq!(renderer.instance_usage()[1],1);scene.clear();renderer.set_scene(&scene,None);assert_eq!(&renderer.instance_usage()[..3],&[0,0,0]);assert!(weak.upgrade().is_none());
}

#[test]
fn actual_congruent_cad_replay_matches_independent_faces_ink_and_owner_picks() {
    let first=Rc::new(session_rust::BRep::create_block_with_hole(0.7,0.6,0.2,0.12).transformed(&session_rust::Xform::translation(-0.35,-0.3,0.3)));let mut cache=crate::cad_shape_cache::Cache::default();cache.prepare(first.clone()).unwrap();let motion=&session_rust::Xform::translation(0.1,-0.1,0.)*&session_rust::Xform::rotation_z(90.,true);let copy=Rc::new(first.transformed(&motion));let mut shared=Scene::empty();let id=shared.insert_prepared_cad(cache.prepare(copy.clone()).unwrap()).unwrap();assert_eq!(cache.replays,1);let mut alone=Scene::empty();alone.insert_cad(copy.clone()).unwrap();let mut shared_renderer=renderer(&shared);let mut independent_renderer=renderer(&alone);let a=colour_pixels(&mut shared_renderer);let b=colour_pixels(&mut independent_renderer);let changed=a.chunks_exact(4).zip(b.chunks_exact(4)).filter(|(a,b)|a[..3].iter().zip(&b[..3]).any(|(a,b)|a.abs_diff(*b)>3)).count();assert!(changed<100,"Congruent faces and source ink differed at {changed} pixels");assert!(a.chunks_exact(4).filter(|p|p[0]<180&&p[1]<180&&p[2]<180).count()>500);
    let mut hits=0;for point in [[-0.1,-0.05],[0.25,-0.05],[0.1,0.15]] {shared_renderer.request_pick(point,identity()).unwrap();independent_renderer.request_pick(point,identity()).unwrap();let actual=finish(&mut shared_renderer);let independent=finish(&mut independent_renderer);assert_eq!(actual,independent);if let Some(actual)=actual {assert_eq!(actual[0],id.value());hits+=1;}}assert!(hits>0,"Actual original CAD ownership must be picked");
    let weak=Rc::downgrade(&copy);drop(copy);shared.clear();alone.clear();shared_renderer.set_scene(&shared,None);independent_renderer.set_scene(&alone,None);drop(cache);assert!(weak.upgrade().is_none());
}

#[test]
fn actual_nested_rotated_and_mirrored_definition_placements_keep_independent_integer_owners() {
    let mut scene=Scene::empty();scene.next_id=16_777_217;crate::scene_document::import(&mut scene,&crate::nested_example::bytes(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=renderer(&scene);assert_eq!(&gpu.instance_usage()[..3],&[1,2,376]);for row in scene.objects() {let point=row.model.transform_point(&session_rust::Point::new((0.123456789012345+0.8)/3.,0.8/3.,0.));gpu.request_pick([point[0] as f32,point[1] as f32],identity()).unwrap();assert_eq!(finish(&mut gpu).unwrap()[0],row.id.value());}
    let id=scene.objects()[1].id;gpu.select_objects(&scene,&[id]).unwrap();let pixels=colour_pixels(&mut gpu);assert!(pixels.chunks_exact(4).filter(|p|p[0]>140&&p[1]>90&&p[2]<40).count()>500);let bytes=crate::scene_archive::save(&scene).unwrap();let mut restored=Scene::empty();crate::scene_document::open(&mut restored,&bytes).unwrap();for a in scene.objects() {let guid=scene.instances[&a.id].instance.guid();let b=restored.objects().iter().find(|row|restored.instances[&row.id].instance.guid()==guid).unwrap();for (a,b) in a.model.m.iter().zip(&b.model.m) {assert!((a-b).abs()<1e-12,"Canonical placement for {guid}: {a} {b}");}}
}

#[test]
fn actual_sheet_batches_share_source_uploads_keep_world_pens_full_entity_ids_and_locked_occlusion() {
    use crate::pick_sources::{Part,Mode,Target};
    let mut source=crate::sheet_example::source();source.entity_count=u32::MAX;source.source_ids=vec![u32::MAX-1;5];let mut scene=Scene::empty();scene.next_id=16_777_217;let id=scene.insert_sheet(Rc::new(crate::sheet_source::Prepared::new(Rc::new(source)).unwrap())).unwrap();let copy=scene.copy_row(id,false).unwrap();scene.place_sheet(copy,session_rust::Xform::translation(0.,0.1,0.)).unwrap();let weak=Rc::downgrade(&scene.sheets()[0].prepared);let mut renderer=renderer(&scene);assert_eq!(renderer.sheet_usage()[..2],[2,1]);let uploads=renderer.sheet_usage()[3];renderer.set_scene(&scene,None);assert_eq!(renderer.sheet_usage()[3],uploads);
    renderer.set_pick_mode(Mode::Edge);renderer.request_pick([0.,-0.5],identity()).unwrap();let hit=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&scene,hit),Some(Target {owner:id,part:Part::SheetEntity(u32::MAX-1)}));renderer.request_pick([0.,-0.4],identity()).unwrap();let hit=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&scene,hit),Some(Target {owner:copy,part:Part::SheetEntity(u32::MAX-1)}));assert!(hit[1]<u32::MAX-1);
    let pixels=colour_pixels(&mut renderer);let black=(0..240).filter(|y| {let p=&pixels[(*y*320+160)*4..][..4];p[0]<30&&p[1]<30&&p[2]<30}).count();assert!(black>=8&&black<20,"Physical .025mm pens draw separate three-pixel boundaries: {black}");scene.place_sheet(copy,session_rust::Xform::translation(0.,0.2,0.)).unwrap();renderer.set_scene(&scene,None);assert_eq!(renderer.sheet_usage()[3],uploads);
    let front=scene.insert(triangle(0.1)).unwrap();renderer.set_scene(&scene,None);renderer.set_pick_mode(Mode::Object);renderer.request_pick([0.,-0.5],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],front.value());Rc::make_mut(&mut scene.view).lock(&[front],true);renderer.set_scene(&scene,None);renderer.request_pick([0.,-0.5],identity()).unwrap();assert!(finish(&mut renderer).is_none());Rc::make_mut(&mut scene.view).hide(&[front],true);Rc::make_mut(&mut scene.view).lock(&[id],true);renderer.set_scene(&scene,None);renderer.request_pick([0.,-0.5],identity()).unwrap();assert!(finish(&mut renderer).is_none());scene.clear();renderer.set_scene(&scene,None);assert_eq!(renderer.sheet_usage()[..2],[0,0]);assert!(weak.upgrade().is_none());
}

#[test]
fn actual_evicted_sheet_pages_remain_charged_until_renderer_refresh_cancels_old_picks() {
    let budget=crate::page_cache::Budget::new(256);let source=Rc::new(crate::sheet_range::Source {guid:crate::sheet_example::source().guid,name:"paged drawing".into(),url:"plan.pb".into(),fields:crate::stream::SheetFields {count:2,entities:u32::MAX,..Default::default()}});let stream=crate::sheet_stream::Stream::new(source,budget.clone());let raw=[-0.5f64,0.,0.3,0.5,0.,0.3].into_iter().flat_map(f64::to_le_bytes).collect::<Vec<_>>();let page=crate::sheet_page::Page::decode(0..1,2,u32::MAX,&raw,None,None,Some(&(u32::MAX-1).to_le_bytes())).unwrap();let held=stream.adopt(page,budget.reserve(192).unwrap()).unwrap();let weak=Rc::downgrade(&held);let mut scene=Scene::empty();let id=scene.insert_sheet(Rc::new(crate::sheet_source::Prepared::streamed(stream.clone()))).unwrap();let mut renderer=renderer(&scene);drop(held);renderer.set_pick_mode(crate::pick_sources::Mode::Edge);renderer.request_pick([0.,0.],identity()).unwrap();let hit=finish(&mut renderer).unwrap();assert_eq!(hit[0],id.value());assert!(renderer.resolve_pick(&scene,hit).is_some());assert_eq!(budget.used(),84);stream.pages.borrow_mut().clear();assert_eq!(budget.used(),84);assert!(budget.reserve(192).is_err());renderer.request_pick([0.,0.],identity()).unwrap();renderer.set_scene(&scene,None);assert!(renderer.take_pick(identity()).is_none());assert_eq!(budget.used(),0);assert!(weak.upgrade().is_none());assert!(renderer.resolve_pick(&scene,hit).is_none());renderer.request_pick([0.,0.],identity()).unwrap();assert!(finish(&mut renderer).is_none());
}

#[test]
fn actual_sheet_pen_projection_uses_camera_lens_independently_of_panel_frame_shift() {
    let source=session_rust::proto::Sheet {guid:crate::sheet_example::source().guid,coords:vec![-0.6,0.,0.3,0.6,0.,0.3],widths:vec![0.35],segment_count:1,..Default::default()};let mut scene=Scene::empty();scene.insert_sheet(Rc::new(crate::sheet_source::Prepared::new(Rc::new(source)).unwrap())).unwrap();let mut renderer=renderer(&scene);let mut camera=crate::camera::Camera::default();let pen_rows=|pixels:Vec<u8>|pixels.chunks_exact(320*4).filter(|row|row[160*4..][..3].iter().all(|v|*v<30)).count();renderer.sheet_camera(&camera);let first=pen_rows(colour_pixels_at(&mut renderer,camera.uniform()));assert!(first>20);camera.frame=[0.,0.,1.,0.2];renderer.sheet_camera(&camera);let shifted=pen_rows(colour_pixels_at(&mut renderer,camera.uniform()));assert_eq!(shifted,first,"A panel shifts the frame without changing the original world pen");
}

#[test]
fn actual_mesh_control_pixel_keeps_sparse_vertex_key_at_both_densities() {
    let mut mesh=session_rust::Mesh::new();
    for(key,p)in[(10,[-0.5,-0.5,0.25]),(20,[0.5,-0.5,0.25]),(30,[0.,0.5,0.25])]{mesh.add_vertex(session_rust::Point::new(p[0],p[1],p[2]),Some(key));}
    mesh.add_face(vec![10,20,30],Some(37));let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(mesh).unwrap()).unwrap();scene.show_controls(true);
    let source=crate::source_edit::original(&scene,owner).unwrap();let model=session_rust::Xform::translation(0.125,0.25,0.);scene.place(owner,model).unwrap();
    for density in [1.,2.] {
        let mut renderer=renderer(&scene);renderer.set_density(density);renderer.resize(Viewport{width:(320.*density)as u32,height:(240.*density)as u32});
        renderer.set_pick_mode(crate::pick_sources::Mode::Control);renderer.request_pick([-0.375,-0.25],identity()).unwrap();
        let pixel=finish(&mut renderer).expect("Actual placed control pixel");
        assert_eq!(renderer.resolve_pick(&scene,pixel),Some(crate::pick_sources::Target{owner,part:crate::pick_sources::Part::SourceControl(crate::source_controls::ControlId::Vertex(10))}));
        scene.show_controls(false);renderer.set_scene(&scene,None);renderer.request_pick([-0.375,-0.25],identity()).unwrap();assert!(finish(&mut renderer).is_none());scene.show_controls(true);
    }
    assert!(crate::source_edit::same(&source,&crate::source_edit::original(&scene,owner).unwrap()));
}

#[test]
fn actual_controls_are_blue_seven_css_pixels_and_selected_yellow_at_both_densities() {
    let mut mesh=session_rust::Mesh::new();
    for(key,p)in[(10,[-0.5,-0.5,0.25]),(20,[0.1,0.1,0.25]),(30,[0.8,0.1,0.25]),(40,[0.1,0.8,0.25])]{mesh.add_vertex(session_rust::Point::new(p[0],p[1],p[2]),Some(key));}
    mesh.add_face(vec![20,30,40],Some(37));let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(mesh).unwrap()).unwrap();scene.show_controls(true);
    assert!(scene.control_markers().iter().all(|m|m.display.diameter==7.&&m.display.colour==[0.15,0.35,0.9,1.]));
    let source=crate::source_edit::original(&scene,owner).unwrap();let target=crate::pick_sources::Target{owner,part:crate::pick_sources::Part::SourceControl(crate::source_controls::ControlId::Vertex(10))};
    for density in [1.,2.] {
        let mut renderer=renderer(&scene);renderer.set_density(density);
        let pixels=colour_pixels(&mut renderer);let center=(180*320+80)*4;
        for (actual,expected) in pixels[center..center+4].iter().zip([38_i16,89,230,255]){assert!((*actual as i16-expected).abs()<=2,"Actual blue control pixel {:?}",&pixels[center..center+4]);}
        let coverage:f32=(60..100).map(|x|(255.-pixels[(180*320+x)*4]as f32)/217.).sum();
        assert!((coverage/density-7.).abs()<0.6,"Seven CSS pixel row coverage at density {density}: {coverage}");
        renderer.set_highlight(&scene,Some(target)).unwrap();let selected=colour_pixels(&mut renderer);
        assert_eq!(&selected[center..center+4],&[255,255,0,255]);
        renderer.set_pick_mode(crate::pick_sources::Mode::Control);renderer.request_pick([-0.5,-0.5],identity()).unwrap();let pixel=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&scene,pixel),Some(target));
    }
    assert!(crate::source_edit::same(&source,&crate::source_edit::original(&scene,owner).unwrap()));
}

#[test]
fn actual_control_net_pixel_addresses_parent_without_a_fake_edge_or_control() {
    let mut editor=crate::editor::Editor::default();editor.apply(crate::editor::Action::Close).unwrap();editor.apply(crate::editor::Action::AddCurve).unwrap();editor.apply(crate::editor::Action::Controls(true)).unwrap();
    let owner=editor.scene.paths()[0].id;let nets=editor.scene.control_nets();assert_eq!(nets.len(),2);let markers=editor.scene.control_markers();
    for(parent,line)in &nets{assert_eq!(*parent,owner);assert!(markers.iter().any(|m|m.display.center==line.start));assert!(markers.iter().any(|m|m.display.center==line.end));assert_eq!(line.width,1.);}
    let source=crate::source_edit::original(&editor.scene,owner).unwrap();
    for density in [1.,2.] {
        let mut renderer=renderer(&editor.scene);renderer.set_density(density);renderer.resize(Viewport{width:(320.*density)as u32,height:(240.*density)as u32});renderer.set_pick_mode(crate::pick_sources::Mode::Object);renderer.request_pick([-0.3,0.1],identity()).unwrap();assert_eq!(finish(&mut renderer),Some([owner.value(),0]));
        for mode in [crate::pick_sources::Mode::Edge,crate::pick_sources::Mode::Control]{renderer.set_pick_mode(mode);renderer.request_pick([-0.3,0.1],identity()).unwrap();assert!(finish(&mut renderer).is_none());}
    }
    assert!(crate::source_edit::same(&source,&crate::source_edit::original(&editor.scene,owner).unwrap()));editor.scene.show_controls(false);assert!(editor.scene.control_nets().is_empty());
}

#[test]
fn actual_edited_element_controls_move_their_integer_pixels_and_undo() {
    use crate::pick_sources::{Mode,Part,Target};
    let mut mesh=session_rust::Mesh::new();for(key,p)in[(10,[-0.5,-0.5,0.25]),(20,[0.5,-0.5,0.25]),(30,[0.,0.5,0.25])]{mesh.add_vertex(session_rust::Point::new(p[0],p[1],p[2]),Some(key));}mesh.add_face(vec![10,20,30],Some(37));
    let mut element=session_rust::Element::new("edited control pixel");element.set_geometry(mesh);let mut base=Scene::empty();let owner=crate::element_original::insert(&mut base,std::rc::Rc::new(element)).unwrap();base.owner_controls(owner).unwrap();let original=crate::source_edit::original(&base,owner).unwrap();let target=Target{owner,part:Part::SourceControl(crate::source_controls::ControlId::Vertex(10))};
    for density in [1.,2.] {
        let mut scene=base.clone();let mut renderer=renderer(&scene);renderer.set_density(density);renderer.resize(Viewport{width:(320.*density)as u32,height:(240.*density)as u32});renderer.set_pick_mode(Mode::Control);renderer.request_pick([-0.5,-0.5],identity()).unwrap();let pixel=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&scene,pixel),Some(target));
        let mut preview=crate::part_preview::Preview::new(&scene,target).unwrap();preview.set(session_rust::Xform::translation(0.25,0.,0.)).unwrap();renderer.set_scene(&preview.scene,None);renderer.request_pick([-0.25,-0.5],identity()).unwrap();let pixel=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&preview.scene,pixel),Some(target));renderer.request_pick([-0.5,-0.5],identity()).unwrap();assert!(finish(&mut renderer).is_none());
        let mut history=crate::history::History::default();assert!(preview.commit(&mut scene,&mut history).unwrap());renderer.set_scene(&scene,None);renderer.request_pick([-0.25,-0.5],identity()).unwrap();let pixel=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&scene,pixel),Some(target));
        assert!(history.undo(&mut scene));renderer.set_scene(&scene,None);renderer.request_pick([-0.5,-0.5],identity()).unwrap();let pixel=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&scene,pixel),Some(target));assert!(crate::source_edit::same(&original,&crate::source_edit::original(&scene,owner).unwrap()));
    }
}

#[test]
fn actual_pulled_source_normals_keep_shared_reflected_placements_and_original_picks() {
    let mut source=session_rust::Mesh::new();
    for(key,p)in[(10,[-0.2,-0.4,0.3]),(20,[0.2,-0.4,0.3]),(30,[0.,0.4,0.3])]{source.add_vertex(session_rust::Point::new(p[0],p[1],p[2]),Some(key));for(name,value)in[("nx",4.),("ny",0.),("nz",3.)]{source.set_vertex_attribute(key,name,value);}}
    source.add_face(vec![30,10,20],Some(37));let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::new(source).unwrap()).unwrap();let original=scene.objects()[0].geometry().unwrap().to_proto();let b=scene.copy_row(a,false).unwrap();
    scene.place(a,session_rust::Xform::translation(-0.5,0.,0.)).unwrap();let mut reflected=session_rust::Xform::identity();reflected.m[0]=-1.;reflected.m[12]=0.5;scene.place(b,reflected).unwrap();assert!(Rc::ptr_eq(scene.objects()[0].geometry().unwrap(),scene.objects()[1].geometry().unwrap()));
    let mut gpu=renderer(&scene);let usage=gpu.stats();
    for density in [1.,2.] {gpu.set_density(density);let pixels=colour_pixels_display(&mut gpu,identity(),true);
        for(x,expected,owner,position)in[(80,[230u8,128,204],a,-0.5),(240,[230u8,128,51],b,0.5)]{let pixel=&pixels[(132*320+x)*4..][..3];for(actual,expected)in pixel.iter().zip(expected){assert!(actual.abs_diff(expected)<=2,"Actual placed source normal: {pixel:?}");}gpu.request_pick([position,-0.1],identity()).unwrap();assert_eq!(finish(&mut gpu).unwrap()[0],owner.value());}
        assert_eq!(&gpu.stats()[..2],&usage[..2]);
    }
    assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);assert_eq!(scene.objects()[1].geometry().unwrap().to_proto(),original);
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzn-batch-shared-geometry-with-original-ownership-02.rs" skip=72099
```

</details>

### 3. `src/instance_gpu.rs`

Share the existing record capacity and retain ordinary drawing when optional copied-original batches exceed it. Keep optional batching and the record writer on one device-limit calculation.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh};
use std::{collections::BTreeMap,rc::Rc};
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn source()->String {
 let source=include_str!("triangle.wgsl")
  .replace("var<uniform> object_settings: ObjectSettings;","var<storage, read> objects: array<ObjectSettings>;")
  .replace("return object_settings;","return objects[index];");
 crate::clipping_gpu::source(&source,2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("shared instance records"),entries:&[wgpu::BindGroupLayoutEntry {binding:0,visibility:wgpu::ShaderStages::VERTEX,ty:wgpu::BindingType::Buffer {ty:wgpu::BufferBindingType::Storage {read_only:true},has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(BYTES as u64)},count:None}]});
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(format.into()),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {pipeline,layout,batches:BTreeMap::new(),records,slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=(u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size)/BYTES as u64) as usize;
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:BTreeMap<usize,(Rc<GpuGeometry>,Vec<Record>)>=BTreeMap::new();
        for row in rows {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push(row.packed());}
        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let mut batches=BTreeMap::new();let mut all_slots=Vec::new();let mut all_records=Vec::with_capacity(count);
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
            for record in &records {
                let row=u32::try_from(all_records.len()).map_err(|_|"Instance record row")?;
                all_slots.push([row,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]);
                all_records.push(*record);
            }
            batches.insert(source,Batch {geometry,count:records.len()as u32,first_slot});
        }
        let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
        self.records.set(&ctx,&self.layout,&all_records)?;
        if self.records.written_bytes>0{self.writes+=1;self.written_bytes+=self.records.written_bytes;}
        if batches.is_empty()&&!self.batches.is_empty(){self.slots.clear(&ctx,true);}else{self.slots.set(&ctx,&all_slots);}
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;Ok(())
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        self.records.bind(pass);
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub(crate) fn record_buffer(&self)->&wgpu::Buffer{&self.records.buffer.buf}
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.records.allocated_bytes()+if self.batches.is_empty(){0}else{self.slots.allocated_bytes()},self.writes,self.written_bytes]}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh};
use std::{collections::BTreeMap,rc::Rc};
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn batch_rows(rows:impl IntoIterator<Item=(crate::scene::ObjectId,usize,bool)>,limit:usize)->std::collections::HashSet<crate::scene::ObjectId> {
 let rows=rows.into_iter().collect::<Vec<_>>();let mut uses=std::collections::HashMap::new();
 for &(_,key,_)in &rows {*uses.entry(key).or_insert(0usize)+=1;}
 let candidates=rows.iter().filter(|(_,key,declared)|*declared||uses[key]>1).map(|(id,_,_)|*id).collect::<std::collections::HashSet<_>>();
 if candidates.len()<=limit {candidates}else{rows.into_iter().filter(|(_,_,declared)|*declared).map(|(id,_,_)|id).collect()}
}
pub(crate) fn source()->String {
 let source=include_str!("triangle.wgsl")
  .replace("var<uniform> object_settings: ObjectSettings;","var<storage, read> objects: array<ObjectSettings>;")
  .replace("return object_settings;","return objects[index];");
 crate::clipping_gpu::source(&source,2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("shared instance records"),entries:&[wgpu::BindGroupLayoutEntry {binding:0,visibility:wgpu::ShaderStages::VERTEX,ty:wgpu::BindingType::Buffer {ty:wgpu::BufferBindingType::Storage {read_only:true},has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(BYTES as u64)},count:None}]});
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(format.into()),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {pipeline,layout,batches:BTreeMap::new(),records,slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=crate::object_rows::capacity(device);
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:BTreeMap<usize,(Rc<GpuGeometry>,Vec<Record>)>=BTreeMap::new();
        for row in rows {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push(row.packed());}
        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let mut batches=BTreeMap::new();let mut all_slots=Vec::new();let mut all_records=Vec::with_capacity(count);
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
            for record in &records {
                let row=u32::try_from(all_records.len()).map_err(|_|"Instance record row")?;
                all_slots.push([row,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]);
                all_records.push(*record);
            }
            batches.insert(source,Batch {geometry,count:records.len()as u32,first_slot});
        }
        let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
        self.records.set(&ctx,&self.layout,&all_records)?;
        if self.records.written_bytes>0{self.writes+=1;self.written_bytes+=self.records.written_bytes;}
        if batches.is_empty()&&!self.batches.is_empty(){self.slots.clear(&ctx,true);}else{self.slots.set(&ctx,&all_slots);}
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;Ok(())
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        self.records.bind(pass);
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub(crate) fn record_buffer(&self)->&wgpu::Buffer{&self.records.buffer.buf}
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.records.allocated_bytes()+if self.batches.is_empty(){0}else{self.slots.allocated_bytes()},self.writes,self.written_bytes]}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzn-batch-shared-geometry-with-original-ownership-03.rs"
```

```text
--8<-- "journey/code/96zzzn-batch-shared-geometry-with-original-ownership-04.rs"
```

</details>

### 4. `src/object_rows.rs`

Own the shared record-capacity calculation beside the record table. Use the same limit for optional batching, instance writes and object-record writes.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{object_record::{BYTES,Record},text_gpu_support::{GpuCtx,GrowBuf}};
pub struct Rows {
 pub(crate) buffer:GrowBuf,
 group:wgpu::BindGroup,
 previous:Vec<Record>,
 pub written_bytes:u64,
}
impl Rows {
 pub fn new(ctx:&GpuCtx,layout:&wgpu::BindGroupLayout)->Self {
  let buffer=GrowBuf::new(ctx,"shared object records",BYTES as u64,wgpu::BufferUsages::STORAGE);
  let group=crate::gpu_layout::group(&ctx.device,layout,&buffer.buf);
  Self {buffer,group,previous:Vec::new(),written_bytes:0}
 }
 pub fn set(&mut self,ctx:&GpuCtx,layout:&wgpu::BindGroupLayout,rows:&[Record])->Result<bool,&'static str> {
  let limits=ctx.device.limits();let limit=limits.max_buffer_size.min(u64::from(limits.max_storage_buffer_binding_size));
  if rows.len()as u64>limit/BYTES as u64||u32::try_from(rows.len()).is_err(){return Err("Object records exceed device limits");}
  self.written_bytes=0;
  if rows.is_empty() {
   if !self.previous.is_empty(){self.buffer.release(ctx);self.group=crate::gpu_layout::group(&ctx.device,layout,&self.buffer.buf);self.previous=Vec::new();}
   return Ok(false);
  }
  let bytes=|rows:&[Record]|rows.iter().flatten().copied().collect::<Vec<_>>();
  let mut grew=false;
  if rows.len()<self.previous.len() {
   self.buffer.reset();let data=bytes(rows);grew=self.buffer.append_bytes(ctx,&data);self.written_bytes=data.len()as u64;
  }else {
   for run in crate::draw_slots::changed_runs(&self.previous,rows) {
    let data=bytes(&rows[run.clone()]);self.buffer.write_bytes_at(ctx,run.start as u32,&data);self.written_bytes+=data.len()as u64;
   }
   let tail=bytes(&rows[self.previous.len()..]);
   if !tail.is_empty(){grew=self.buffer.append_bytes(ctx,&tail);self.written_bytes+=tail.len()as u64;}
  }
  if grew{self.group=crate::gpu_layout::group(&ctx.device,layout,&self.buffer.buf);}
  self.previous.clear();self.previous.extend_from_slice(rows);Ok(self.written_bytes>0)
 }
 pub fn bind(&self,pass:&mut wgpu::RenderPass<'_>){pass.set_bind_group(1,&self.group,&[]);}
 pub fn len(&self)->u32{self.previous.len()as u32}
 pub fn allocated_bytes(&self)->u64{if self.previous.is_empty(){0}else{self.buffer.buf.size()}}
}

#[cfg(test)]
mod tests {
 use super::*;
#[test]
fn shared_object_rows_keep_neighbours_growth_and_release() {
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 let layout=gpu.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:None,entries:&[wgpu::BindGroupLayoutEntry {binding:0,visibility:wgpu::ShaderStages::VERTEX,ty:wgpu::BindingType::Buffer {ty:wgpu::BufferBindingType::Storage {read_only:true},has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(crate::object_record::BYTES as u64)},count:None}]});
 let mut table=Rows::new(&ctx,&layout);let mut rows=vec![[7;BYTES],[9;BYTES],[11;BYTES]];
 let check=|table:&Rows,rows:&[Record]|{let bytes=rows.iter().flatten().copied().collect::<Vec<_>>();assert_eq!(crate::growing_buffer_tests::read(&ctx,&table.buffer.buf,bytes.len()as u64),bytes);};
 table.set(&ctx,&layout,&rows).unwrap();check(&table,&rows);let buffer=table.buffer.buf.clone();let group=table.group.clone();
 rows[1][68..72].copy_from_slice(&0x80000001u32.to_ne_bytes());table.set(&ctx,&layout,&rows).unwrap();assert_eq!(table.written_bytes,BYTES as u64);assert_eq!(table.buffer.buf,buffer);assert_eq!(table.group,group);check(&table,&rows);
 assert!(!table.set(&ctx,&layout,&rows).unwrap());assert_eq!(table.written_bytes,0);
 rows.extend([[13;BYTES],[17;BYTES],[19;BYTES],[23;BYTES]]);table.set(&ctx,&layout,&rows).unwrap();assert_ne!(table.buffer.buf,buffer);assert_ne!(table.group,group);check(&table,&rows);let capacity=table.allocated_bytes();
 rows.truncate(2);table.set(&ctx,&layout,&rows).unwrap();assert_eq!(table.allocated_bytes(),capacity);check(&table,&rows);
 table.set(&ctx,&layout,&[]).unwrap();assert_eq!(table.len(),0);assert_eq!(table.allocated_bytes(),0);assert_eq!(table.previous.capacity(),0);assert_eq!(table.buffer.buf.size(),BYTES as u64);
 table.set(&ctx,&layout,&rows).unwrap();check(&table,&rows);
}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{object_record::{BYTES,Record},text_gpu_support::{GpuCtx,GrowBuf}};
pub(crate) fn capacity(device:&wgpu::Device)->usize {(u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size)/BYTES as u64) as usize}
pub struct Rows {
 pub(crate) buffer:GrowBuf,
 group:wgpu::BindGroup,
 previous:Vec<Record>,
 pub written_bytes:u64,
}
impl Rows {
 pub fn new(ctx:&GpuCtx,layout:&wgpu::BindGroupLayout)->Self {
  let buffer=GrowBuf::new(ctx,"shared object records",BYTES as u64,wgpu::BufferUsages::STORAGE);
  let group=crate::gpu_layout::group(&ctx.device,layout,&buffer.buf);
  Self {buffer,group,previous:Vec::new(),written_bytes:0}
 }
 pub fn set(&mut self,ctx:&GpuCtx,layout:&wgpu::BindGroupLayout,rows:&[Record])->Result<bool,&'static str> {
  if rows.len()>capacity(&ctx.device)||u32::try_from(rows.len()).is_err(){return Err("Object records exceed device limits");}
  self.written_bytes=0;
  if rows.is_empty() {
   if !self.previous.is_empty(){self.buffer.release(ctx);self.group=crate::gpu_layout::group(&ctx.device,layout,&self.buffer.buf);self.previous=Vec::new();}
   return Ok(false);
  }
  let bytes=|rows:&[Record]|rows.iter().flatten().copied().collect::<Vec<_>>();
  let mut grew=false;
  if rows.len()<self.previous.len() {
   self.buffer.reset();let data=bytes(rows);grew=self.buffer.append_bytes(ctx,&data);self.written_bytes=data.len()as u64;
  }else {
   for run in crate::draw_slots::changed_runs(&self.previous,rows) {
    let data=bytes(&rows[run.clone()]);self.buffer.write_bytes_at(ctx,run.start as u32,&data);self.written_bytes+=data.len()as u64;
   }
   let tail=bytes(&rows[self.previous.len()..]);
   if !tail.is_empty(){grew=self.buffer.append_bytes(ctx,&tail);self.written_bytes+=tail.len()as u64;}
  }
  if grew{self.group=crate::gpu_layout::group(&ctx.device,layout,&self.buffer.buf);}
  self.previous.clear();self.previous.extend_from_slice(rows);Ok(self.written_bytes>0)
 }
 pub fn bind(&self,pass:&mut wgpu::RenderPass<'_>){pass.set_bind_group(1,&self.group,&[]);}
 pub fn len(&self)->u32{self.previous.len()as u32}
 pub fn allocated_bytes(&self)->u64{if self.previous.is_empty(){0}else{self.buffer.buf.size()}}
}

#[cfg(test)]
mod tests {
 use super::*;
#[test]
fn shared_object_rows_keep_neighbours_growth_and_release() {
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 let layout=gpu.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:None,entries:&[wgpu::BindGroupLayoutEntry {binding:0,visibility:wgpu::ShaderStages::VERTEX,ty:wgpu::BindingType::Buffer {ty:wgpu::BufferBindingType::Storage {read_only:true},has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(crate::object_record::BYTES as u64)},count:None}]});
 let mut table=Rows::new(&ctx,&layout);let mut rows=vec![[7;BYTES],[9;BYTES],[11;BYTES]];
 let check=|table:&Rows,rows:&[Record]|{let bytes=rows.iter().flatten().copied().collect::<Vec<_>>();assert_eq!(crate::growing_buffer_tests::read(&ctx,&table.buffer.buf,bytes.len()as u64),bytes);};
 table.set(&ctx,&layout,&rows).unwrap();check(&table,&rows);let buffer=table.buffer.buf.clone();let group=table.group.clone();
 rows[1][68..72].copy_from_slice(&0x80000001u32.to_ne_bytes());table.set(&ctx,&layout,&rows).unwrap();assert_eq!(table.written_bytes,BYTES as u64);assert_eq!(table.buffer.buf,buffer);assert_eq!(table.group,group);check(&table,&rows);
 assert!(!table.set(&ctx,&layout,&rows).unwrap());assert_eq!(table.written_bytes,0);
 rows.extend([[13;BYTES],[17;BYTES],[19;BYTES],[23;BYTES]]);table.set(&ctx,&layout,&rows).unwrap();assert_ne!(table.buffer.buf,buffer);assert_ne!(table.group,group);check(&table,&rows);let capacity=table.allocated_bytes();
 rows.truncate(2);table.set(&ctx,&layout,&rows).unwrap();assert_eq!(table.allocated_bytes(),capacity);check(&table,&rows);
 table.set(&ctx,&layout,&[]).unwrap();assert_eq!(table.len(),0);assert_eq!(table.allocated_bytes(),0);assert_eq!(table.previous.capacity(),0);assert_eq!(table.buffer.buf.size(),BYTES as u64);
 table.set(&ctx,&layout,&rows).unwrap();check(&table,&rows);
}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzn-batch-shared-geometry-with-original-ownership-05.rs"
```

```text
--8<-- "journey/code/96zzzn-batch-shared-geometry-with-original-ownership-06.rs"
```

</details>

</details>

<a id="checkpoint-96zzzo-shrink-record-ranges-without-rewriting-live-prefixes"></a>

### Step 21 of 24 · Shrink record ranges without rewriting live prefixes (`96zzzo-shrink-record-ranges-without-rewriting-live-prefixes`)

- **Goal:** Reuse one logical buffer truncate operation for object records and draw slots.
- **Data Flow Trace:** `Shorter live range → same allocation → changed live runs only → current draw count`
- **Files You Touch:** `src/text_gpu_support.rs` (*Shorten a logical record range without replacing its allocation or rewriting the live prefix.*) · `src/object_rows.rs` (*Reuse the existing changed-run scan on shrinking tables; write only changed live records.*) · `src/draw_slots.rs` (*Keep slot zero and unchanged instance slots while shortening their logical range.*) · `src/draw_slots_tests.rs` (*Supply exact retained slot bytes and one changed high-bit slot through a second shrink.*)

Removing a last record changes the live range without changing its prefix. Keep the allocation and immutable binding, shorten the logical count, and reuse the changed-run scan for records that really changed. An empty object table still follows the existing full-release path; ordinary slot zero remains available.

> **Self-Check Question:** *Does logical truncation erase the reserved tail?*  
> **Answer:** No. It changes the live count. Drawing must use that count; complete scene release still drops the old allocation through the existing release path.

- **What You Should See:** The same original scene remains visible. Removing records can now preserve unchanged live bytes and current GPU bindings.
- **Hands-On Lab Experiment:** Hide and show a copied original, restore selection and compare the scene. Inspect the supplied mapped-byte checks for zero writes on an unchanged shortened prefix and one changed high-bit record.

<details>
<summary>Implementation 96zzzo-shrink-record-ranges-without-rewriting-live-prefixes: Shrink record ranges without rewriting live prefixes</summary>

Reuse one logical buffer truncate operation for object records and draw slots.

Data flow: Shorter live range → same allocation → changed live runs only → current draw count.

### 1. `src/text_gpu_support.rs`

Shorten a logical record range without replacing its allocation or rewriting the live prefix.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn write_bytes_at(&self,ctx:&GpuCtx,at:u32,bytes:&[u8]) {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn truncate(&mut self,len:u32){assert!(len<=self.len);self.len=len;}

    pub fn write_bytes_at(&self,ctx:&GpuCtx,at:u32,bytes:&[u8]) {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzo-shrink-record-ranges-without-rewriting-live-prefixes-01.rs"
```

</details>

### 2. `src/object_rows.rs`

Reuse the existing changed-run scan on shrinking tables; write only changed live records.

<details>
<summary>Locate the existing block</summary>

```rust
  if rows.len()<self.previous.len() {
   self.buffer.reset();let data=bytes(rows);grew=self.buffer.append_bytes(ctx,&data);self.written_bytes=data.len()as u64;
  }else {
   for run in crate::draw_slots::changed_runs(&self.previous,rows) {
    let data=bytes(&rows[run.clone()]);self.buffer.write_bytes_at(ctx,run.start as u32,&data);self.written_bytes+=data.len()as u64;
   }
   let tail=bytes(&rows[self.previous.len()..]);
   if !tail.is_empty(){grew=self.buffer.append_bytes(ctx,&tail);self.written_bytes+=tail.len()as u64;}
  }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
  if rows.len()<self.previous.len(){self.buffer.truncate(rows.len()as u32);}
  for run in crate::draw_slots::changed_runs(&self.previous,rows) {
   let data=bytes(&rows[run.clone()]);self.buffer.write_bytes_at(ctx,run.start as u32,&data);self.written_bytes+=data.len()as u64;
  }
  let tail=bytes(&rows[self.previous.len().min(rows.len())..]);
  if !tail.is_empty(){grew=self.buffer.append_bytes(ctx,&tail);self.written_bytes+=tail.len()as u64;}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzo-shrink-record-ranges-without-rewriting-live-prefixes-02.rs"
```

</details>

### 3. `src/draw_slots.rs`

Keep slot zero and unchanged instance slots while shortening their logical range.

<details>
<summary>Locate the existing block</summary>

```rust
   self.buffer.reset();self.buffer.append_bytes(ctx,&bytes(&[[OWN_ROW,0]]));self.buffer.append_bytes(ctx,&bytes(rows));
   self.written_bytes=(rows.len()as u64+1)*8;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
   self.buffer.truncate(u32::try_from(rows.len()+1).expect("Bounded draw slots"));
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzo-shrink-record-ranges-without-rewriting-live-prefixes-03.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/object_rows.rs`

Supply mapped live prefixes, exact partial-write counts and current immutable bindings while shrinking.

<details>
<summary>Locate the existing block</summary>

```text
 rows.truncate(2);table.set(&ctx,&layout,&rows).unwrap();assert_eq!(table.allocated_bytes(),capacity);check(&table,&rows);
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzo-shrink-record-ranges-without-rewriting-live-prefixes-04.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/draw_slots_tests.rs`

Supply exact retained slot bytes and one changed high-bit slot through a second shrink.

<details>
<summary>Locate the existing block</summary>

```text
 rows.truncate(2);let capacity=slots.allocated_bytes();slots.set(&ctx,&rows);assert_eq!(slots.written_bytes,24);assert_eq!(slots.allocated_bytes(),capacity);check(&slots,&rows);
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzo-shrink-record-ranges-without-rewriting-live-prefixes-05.rs"
```

</details>

</details>

<a id="checkpoint-96zzzp-share-opaque-ordinary-surface-records"></a>

### Step 22 of 24 · Share opaque ordinary surface records (`96zzzp-share-opaque-ordinary-surface-records`)

- **Goal:** Draw ordinary opaque surfaces from the same object-record table as instances.
- **Data Flow Trace:** `Ordinary visible mesh → shared packed record and draw slot → existing storage shader → original pixels and primitive IDs`
- **Files You Touch:** `src/instance_gpu.rs` (*Keep each ordinary original’s draw slot alongside the shared record table.*) · `src/renderer.rs` (*Refresh both ordinary and instance records after scene or selection changes.*) · `src/pick_gpu_tests.rs` (*The surviving ordinary original now retains its shared record allocation; its original pick and full final release remain checked.*) · `src/global_outline_tests.rs` (*Supply mapped ordinary records and exact storage-versus-legacy pixels and primitive addresses through growth, selection, opacity restoration and release.*) · `src/object_rows.rs` (*Reuse the record table’s existing CPU snapshot; use the current ordinary uniform path after a mesh range or record changes.*)

The shared record table already draws instances. Append ordinary visible mesh records when it has room and bind their draw slots in the existing scene order. Reuse the same storage shader. Keep the ordinary uniform path for transparency, temporary overlays and a table that cannot hold the optional ordinary records. A draw uses the shared record only while its packed snapshot matches the current mesh; otherwise restore slot zero and use the current uniform path.

> **Self-Check Question:** *Why preserve the ordinary scene order instead of grouping every draw?*  
> **Answer:** Changing the order can change which coincident surface wins the depth test. Sharing records does not require changing that order.

- **What You Should See:** With Opacity 1, ordinary surfaces and instances read one shared object-record table. Original pixels, picks and selection remain intact.
- **Hands-On Lab Experiment:** Use Opacity 1 on the original scene. Select an ordinary mesh, restore its appearance, then use Opacity 0.5 and return to 1. Inspect the supplied exact comparison with the earlier uniform path.

<details>
<summary>Implementation 96zzzp-share-opaque-ordinary-surface-records: Share opaque ordinary surface records</summary>

Draw ordinary opaque surfaces from the same object-record table as instances.

Data flow: Ordinary visible mesh → shared packed record and draw slot → existing storage shader → original pixels and primitive IDs.

### 1. `src/instance_gpu.rs`

Keep each ordinary original’s draw slot alongside the shared record table. Start with no ordinary record leases. Receive ordinary visible meshes without changing the existing instance grouping. Append ordinary records when the table has room; keep their legacy path when it does not. Release all scene slots after both ordinary and instance leases end. Adopt current record leases together with the instance batches. Draw ordinary opaque meshes in their existing scene order through the same storage-record shader. Account for every live slot allocation once, including ordinary record consumers.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh};
use std::{collections::BTreeMap,rc::Rc};
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn batch_rows(rows:impl IntoIterator<Item=(crate::scene::ObjectId,usize,bool)>,limit:usize)->std::collections::HashSet<crate::scene::ObjectId> {
 let rows=rows.into_iter().collect::<Vec<_>>();let mut uses=std::collections::HashMap::new();
 for &(_,key,_)in &rows {*uses.entry(key).or_insert(0usize)+=1;}
 let candidates=rows.iter().filter(|(_,key,declared)|*declared||uses[key]>1).map(|(id,_,_)|*id).collect::<std::collections::HashSet<_>>();
 if candidates.len()<=limit {candidates}else{rows.into_iter().filter(|(_,_,declared)|*declared).map(|(id,_,_)|id).collect()}
}
pub(crate) fn source()->String {
 let source=include_str!("triangle.wgsl")
  .replace("var<uniform> object_settings: ObjectSettings;","var<storage, read> objects: array<ObjectSettings>;")
  .replace("return object_settings;","return objects[index];");
 crate::clipping_gpu::source(&source,2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("shared instance records"),entries:&[wgpu::BindGroupLayoutEntry {binding:0,visibility:wgpu::ShaderStages::VERTEX,ty:wgpu::BindingType::Buffer {ty:wgpu::BufferBindingType::Storage {read_only:true},has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(BYTES as u64)},count:None}]});
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(format.into()),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {pipeline,layout,batches:BTreeMap::new(),records,slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=crate::object_rows::capacity(device);
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:BTreeMap<usize,(Rc<GpuGeometry>,Vec<Record>)>=BTreeMap::new();
        for row in rows {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push(row.packed());}
        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let mut batches=BTreeMap::new();let mut all_slots=Vec::new();let mut all_records=Vec::with_capacity(count);
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
            for record in &records {
                let row=u32::try_from(all_records.len()).map_err(|_|"Instance record row")?;
                all_slots.push([row,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]);
                all_records.push(*record);
            }
            batches.insert(source,Batch {geometry,count:records.len()as u32,first_slot});
        }
        let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
        self.records.set(&ctx,&self.layout,&all_records)?;
        if self.records.written_bytes>0{self.writes+=1;self.written_bytes+=self.records.written_bytes;}
        if batches.is_empty()&&!self.batches.is_empty(){self.slots.clear(&ctx,true);}else{self.slots.set(&ctx,&all_slots);}
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;Ok(())
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        self.records.bind(pass);
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub(crate) fn record_buffer(&self)->&wgpu::Buffer{&self.records.buffer.buf}
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.records.allocated_bytes()+if self.batches.is_empty(){0}else{self.slots.allocated_bytes()},self.writes,self.written_bytes]}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh};
use std::{collections::BTreeMap,rc::Rc};
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) ordinary:std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn batch_rows(rows:impl IntoIterator<Item=(crate::scene::ObjectId,usize,bool)>,limit:usize)->std::collections::HashSet<crate::scene::ObjectId> {
 let rows=rows.into_iter().collect::<Vec<_>>();let mut uses=std::collections::HashMap::new();
 for &(_,key,_)in &rows {*uses.entry(key).or_insert(0usize)+=1;}
 let candidates=rows.iter().filter(|(_,key,declared)|*declared||uses[key]>1).map(|(id,_,_)|*id).collect::<std::collections::HashSet<_>>();
 if candidates.len()<=limit {candidates}else{rows.into_iter().filter(|(_,_,declared)|*declared).map(|(id,_,_)|id).collect()}
}
pub(crate) fn source()->String {
 let source=include_str!("triangle.wgsl")
  .replace("var<uniform> object_settings: ObjectSettings;","var<storage, read> objects: array<ObjectSettings>;")
  .replace("return object_settings;","return objects[index];");
 crate::clipping_gpu::source(&source,2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("shared instance records"),entries:&[wgpu::BindGroupLayoutEntry {binding:0,visibility:wgpu::ShaderStages::VERTEX,ty:wgpu::BindingType::Buffer {ty:wgpu::BufferBindingType::Storage {read_only:true},has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(BYTES as u64)},count:None}]});
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(format.into()),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {pipeline,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>,ordinary:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=crate::object_rows::capacity(device);
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:BTreeMap<usize,(Rc<GpuGeometry>,Vec<Record>)>=BTreeMap::new();
        for row in rows {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push(row.packed());}
        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let mut batches=BTreeMap::new();let mut all_slots=Vec::new();let mut all_records=Vec::with_capacity(count);
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
            for record in &records {
                let row=u32::try_from(all_records.len()).map_err(|_|"Instance record row")?;
                all_slots.push([row,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]);
                all_records.push(*record);
            }
            batches.insert(source,Batch {geometry,count:records.len()as u32,first_slot});
        }
        let own=ordinary.into_iter().collect::<Vec<_>>();let mut ordinary=std::collections::HashMap::new();
        if own.len()<=limit-count {
            for mesh in own {
                let first=u32::try_from(all_slots.len()+1).map_err(|_|"Ordinary slot range")?;
                let row=u32::try_from(all_records.len()).map_err(|_|"Ordinary record row")?;
                all_slots.push([row,mesh.primitive_base()]);all_records.push(mesh.packed());ordinary.insert(mesh.id,(first,row));
            }
        }
        let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
        self.records.set(&ctx,&self.layout,&all_records)?;
        if self.records.written_bytes>0{self.writes+=1;self.written_bytes+=self.records.written_bytes;}
        if all_slots.is_empty()&&self.slots.instances()>0{self.slots.clear(&ctx,true);}else{self.slots.set(&ctx,&all_slots);}
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;self.ordinary=ordinary;Ok(())
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {
        let Some(&(first,row))=self.ordinary.get(&mesh.id)else{return false};
        if !self.records.matches(row,&mesh.packed()){return false;}
        pass.set_pipeline(&self.pipeline);self.records.bind(pass);self.slots.bind(pass,0,first);mesh.geometry.draw_pulled(pass,1);true
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        self.records.bind(pass);
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub(crate) fn record_buffer(&self)->&wgpu::Buffer{&self.records.buffer.buf}
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.records.allocated_bytes()+if self.slots.instances()==0{0}else{self.slots.allocated_bytes()},self.writes,self.written_bytes]}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzp-share-opaque-ordinary-surface-records-01.rs"
```

```text
--8<-- "journey/code/96zzzp-share-opaque-ordinary-surface-records-02.rs"
```

```text
--8<-- "journey/code/96zzzp-share-opaque-ordinary-surface-records-03.rs"
```

```text
--8<-- "journey/code/96zzzp-share-opaque-ordinary-surface-records-04.rs"
```

```text
--8<-- "journey/code/96zzzp-share-opaque-ordinary-surface-records-05.rs"
```

```text
--8<-- "journey/code/96zzzp-share-opaque-ordinary-surface-records-06.rs"
```

```text
--8<-- "journey/code/96zzzp-share-opaque-ordinary-surface-records-07.rs"
```

```text
--8<-- "journey/code/96zzzp-share-opaque-ordinary-surface-records-08.rs"
```

</details>

### 2. `src/renderer.rs`

Refresh both ordinary and instance records after scene or selection changes. Use storage records for opaque ordinary surfaces; retain the earlier transparent and capacity-fallback paths.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            for mesh in self.meshes.iter().filter(|row|self.opacity.get()<1.||!self.instance_rows.contains(&row.id)) {mesh.draw_pulled(&mut pass);}
            if self.opacity.get()>=1.{self.instances.draw(&mut pass);}
            pass.set_pipeline(&self.highlight_pipeline); pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
```

</details>

Replace it with:

```rust
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            for mesh in self.meshes.iter().filter(|row|self.opacity.get()<1.||!self.instance_rows.contains(&row.id)) {if !(self.opacity.get()>=1.&&self.instances.draw_ordinary(&mut pass,mesh)){pass.set_pipeline(&self.pipeline);self.instances.bind_own(&mut pass);mesh.draw_pulled(&mut pass);}}
            if self.opacity.get()>=1.{self.instances.draw(&mut pass);}
            pass.set_pipeline(&self.highlight_pipeline); pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
```

<details>
<summary>Locate this block</summary>

```rust
            let [calls, bytes] = mesh.update(&self.queue, row,style(scene,row.id,ids.contains(&row.id)));
            self.settings_writes += calls; self.settings_bytes += bytes;
        } }
        self.instances.set(&self.device,&self.queue,self.meshes.iter().filter(|row|self.instance_rows.contains(&row.id)))?;
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        self.set_clouds(scene,ids)?;self.sheets.set(&self.device,&self.queue,scene,ids,&self.sources.sheet_bases)?;
        if !ids.is_empty() { self.highlight.objects(&self.device, scene, ids)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
```

</details>

Replace it with:

```rust
            let [calls, bytes] = mesh.update(&self.queue, row,style(scene,row.id,ids.contains(&row.id)));
            self.settings_writes += calls; self.settings_bytes += bytes;
        } }
        self.instances.set(&self.device,&self.queue,self.meshes.iter().filter(|row|self.instance_rows.contains(&row.id)),self.meshes.iter().filter(|row|!self.instance_rows.contains(&row.id)))?;
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        self.set_clouds(scene,ids)?;self.sheets.set(&self.device,&self.queue,scene,ids,&self.sources.sheet_bases)?;
        if !ids.is_empty() { self.highlight.objects(&self.device, scene, ids)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
```

<details>
<summary>Locate this block</summary>

```rust
        }
        self.tiles.borrow_mut().set(&self.device,&self.meshes);
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
        self.instances.set(&self.device,&self.queue,self.meshes.iter().filter(|row|self.instance_rows.contains(&row.id))).expect("Validated instance records fit device limits");
        self.sheets.set(&self.device,&self.queue,scene,selected.as_slice(),&self.sources.sheet_bases).expect("Prepared original sheet batches fit device limits");
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
```

</details>

Replace it with:

```rust
        }
        self.tiles.borrow_mut().set(&self.device,&self.meshes);
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
        self.instances.set(&self.device,&self.queue,self.meshes.iter().filter(|row|self.instance_rows.contains(&row.id)),self.meshes.iter().filter(|row|!self.instance_rows.contains(&row.id))).expect("Validated instance records fit device limits");
        self.sheets.set(&self.device,&self.queue,scene,selected.as_slice(),&self.sources.sheet_bases).expect("Prepared original sheet batches fit device limits");
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
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
            base=base.checked_add(mesh.geometry.index_count/3).filter(|n|*n<crate::primitive_image::CAP_START).expect("Bounded projected triangle range");
        }
        self.tiles.borrow_mut().set(&self.device,&self.meshes);
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
        self.instances.set(&self.device,&self.queue,self.meshes.iter().filter(|row|self.instance_rows.contains(&row.id)),self.meshes.iter().filter(|row|!self.instance_rows.contains(&row.id))).expect("Validated instance records fit device limits");
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
        self.instances.set(&self.device,&self.queue,self.meshes.iter().filter(|row|self.instance_rows.contains(&row.id)),self.meshes.iter().filter(|row|!self.instance_rows.contains(&row.id)))?;
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
        let [r, g, b] = if self.ambient_enabled.get() { [0.94; 3] } else { background.rgb() };
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
            self.instances.bind_own(&mut pass);
            for mesh in self.meshes.iter().filter(|row|self.opacity.get()<1.||!self.instance_rows.contains(&row.id)) {if !(self.opacity.get()>=1.&&self.instances.draw_ordinary(&mut pass,mesh)){pass.set_pipeline(&self.pipeline);self.instances.bind_own(&mut pass);mesh.draw_pulled(&mut pass);}}
            if self.opacity.get()>=1.{self.instances.draw(&mut pass);}
            pass.set_pipeline(&self.highlight_pipeline); pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
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
--8<-- "journey/code/96zzzp-share-opaque-ordinary-surface-records-09.rs"
```

```text
--8<-- "journey/code/96zzzp-share-opaque-ordinary-surface-records-10.rs"
```

```text
--8<-- "journey/code/96zzzp-share-opaque-ordinary-surface-records-11.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/pick_gpu_tests.rs`

The surviving ordinary original now retains its shared record allocation; its original pick and full final release remain checked.

<details>
<summary>Locate the existing block</summary>

```text
Rc::make_mut(&mut scene.view).hide(&[b],true);gpu.set_scene(&scene,None);assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);gpu.request_pick([-0.45,0.],identity())
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzp-share-opaque-ordinary-surface-records-12.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/global_outline_tests.rs`

Supply mapped ordinary records and exact storage-versus-legacy pixels and primitive addresses through growth, selection, opacity restoration and release.

<details>
<summary>Locate the existing block</summary>

```text
use crate::{scene::Scene,prepared::PreparedMesh,cap_gpu_tests::{renderer,pixels,matrix,wait}};
use std::rc::Rc;
fn width_pixels(r:&super::Renderer){let texture=r.device.create_texture(&wgpu::TextureDescriptor{label:Some("global CSS width"),size:wgpu::Extent3d{width:320,height:240,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});r.draw(&texture.create_view(&Default::default()),&Default::default(),&session_rust::Xform::identity().m.map(|v|v as f32));wait(r);}
#[test]
fn actual_global_curved_and_flat_silhouettes_keep_the_same_css_width_at_two_densities() {
    let mut widths=Vec::new();
    for curved in [false,true] {
        let mut scene=Scene::demo();scene.clear();let id=if curved {scene.insert_cad(Rc::new(session_rust::BRep::create_cylinder(0.4,0.8))).unwrap()} else {let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.8,0.8,0.2)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,0.5);scene.insert(prepared).unwrap()};let mut renderer=renderer(&scene);renderer.set_outlines(true);assert!(!renderer.meshes.iter().any(crate::gpu_mesh::GpuMesh::selected));let _=id;let geometry=renderer.stats();let mut css=Vec::new();
        for density in [1.,2.] {renderer.set_density(density);width_pixels(&renderer);let alpha=renderer.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));let row=&alpha[120*320..121*320];let width=row[160..].iter().map(|v|*v as f64/255.).sum::<f64>()/density as f64;assert!((0.7..1.7).contains(&width),"Actual outer silhouette CSS coverage remains bounded: {width}");css.push(width);assert_eq!(&renderer.stats()[..2],&geometry[..2]);}
        assert!((css[0]-css[1]).abs()<0.5,"DPI keeps the outline in CSS units: {css:?}");widths.push(css);
    }
    for i in 0..2 {assert!((widths[0][i]-widths[1][i]).abs()<0.25,"A smooth cylinder and flat box use one coverage width: {widths:?}");}
}

#[test]
fn actual_global_and_selected_outlines_share_alpha_and_one_black_blend(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();
 let original=scene.objects()[0].geometry().unwrap().to_proto();let mut r=renderer(&scene);let plain=pixels(&r);let uploads=r.stats()[0];r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().filter(|v|**v>0).count()>300);let usage=r.global_outline_usage();assert_eq!(pixels(&r),global);assert_eq!(r.global_outline_usage()[1],usage[1]);assert!(r.global_outline_usage()[2]>usage[2]);
 r.select_objects(&scene,&[owner]).unwrap();let both=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"same source and CSS width yield identical masks");
 let mut boundary=0;for(i,(&coverage,(a,b)))in alpha.iter().zip(global.chunks_exact(4).zip(both.chunks_exact(4))).enumerate(){if coverage>0&&plain[i*4..i*4+3]==[255,255,255]{boundary+=1;assert_eq!(a,b,"selected plus global uses max alpha in one blend");}}
 assert!(boundary>200);assert_eq!(r.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
 r.request_pick([0.,0.],matrix()).unwrap();wait(&r);let hit=r.take_pick(matrix()).expect("mapped original pick").unwrap();assert_eq!(hit.unwrap()[0],owner.value());
 r.select_objects(&scene,&[]).unwrap();r.set_outlines(false);assert_eq!(r.global_outline_usage(),[0;4]);assert_eq!(pixels(&r),plain);
 scene.clear();r.set_scene(&scene,None);r.set_outlines(true);pixels(&r);assert_eq!(r.global_outline_usage(),[0;4]);
}
#[test]
fn actual_visible_cap_coverage_keeps_selected_and_global_masks_equal(){
 let(mut scene,owner)=crate::cap_gpu_tests::scene(false);let mut r=renderer(&scene);r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));
 r.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"selected visible cap marker joins original coverage");
 for(i,v)in alpha.iter().enumerate(){if *v==255{assert!(global[i*4..i*4+3].iter().all(|v|*v<12));assert!(selected[i*4..i*4+3].iter().all(|v|*v<12));}}
 r.set_clipping(false,true);pixels(&r);assert_eq!(r.global_outline_alpha(),alpha,"horizontal cut and uncut original share the same outer XY silhouette");assert_eq!(r.cap_usage()[0],16);
 let side=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Yz,&[[0.,0.,0.]],0.9).unwrap();crate::plane_original::insert(&mut scene,Rc::new(side)).unwrap();r.set_scene(&scene,Some(owner));r.set_clipping(true,true);pixels(&r);assert_ne!(r.global_outline_alpha(),alpha,"changed section inputs cannot reuse stale coverage");
}
#[test]
fn actual_global_outline_includes_locked_and_releases_hidden_originals(){
 let mut scene=Scene::empty();let mut source=crate::cap_gpu_tests::source(false);source.is_locked=true;let owner=scene.insert(PreparedMesh::new(source).unwrap()).unwrap();let mut r=renderer(&scene);r.set_outlines(true);pixels(&r);assert!(r.global_outline_alpha().iter().any(|v|*v==255));assert_eq!(scene.pick_id(owner.value()),None);
 Rc::make_mut(&mut scene.view).hidden.insert(owner);r.set_scene(&scene,None);pixels(&r);assert!(r.global_outline_alpha().is_empty());assert_eq!(r.global_outline_usage(),[0;4]);
}

#[test]
fn actual_triangle_ranges_keep_selection_updates_visibility_and_original_owners(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    let bases=|gpu:&super::Renderer|gpu.meshes.iter().map(|row|u32::from_ne_bytes(row.packed()[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())).collect::<Vec<_>>();assert_eq!(bases(&gpu),vec![0,1]);assert!(gpu.meshes.iter().all(|row|!row.selected()));let baseline=pixels(&gpu);
    gpu.select_objects(&scene,&[b]).unwrap();assert_eq!(bases(&gpu),vec![0,1]);assert_eq!(gpu.meshes.iter().map(|row|row.selected()).collect::<Vec<_>>(),vec![false,true]);let selected=pixels(&gpu);assert_ne!(selected,baseline);
    scene.translate(b,[0.,0.1,0.]).unwrap();gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![0,1]);assert!(gpu.meshes[1].selected());for(position,owner)in[([-0.5,-0.1],a),([0.5,0.],b)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
    let before=gpu.meshes[1].packed();assert!(gpu.meshes[1].set_primitive_base(&gpu.queue,crate::primitive_image::CAP_START).is_err());assert_eq!(gpu.meshes[1].packed(),before);
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![0]);assert_eq!(gpu.meshes[0].id,b);assert!(gpu.meshes[0].selected());pixels(&gpu);
}

fn primitive_pixels(gpu:&super::Renderer,texture:&wgpu::Texture)->Vec<u32>{
        let width=texture.width();let height=texture.height();let pitch=(width*4+255)/256*256;
        let read=gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("actual primitive addresses"),size:u64::from(pitch)*u64::from(height),usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let mut encoder=gpu.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&read,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},texture.size());gpu.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|result|tx.send(result).unwrap());wait(gpu);rx.recv().unwrap().unwrap();let bytes=read.slice(..).get_mapped_range();let values=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize*4].chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap()))).collect();drop(bytes);read.unmap();values
    }
#[test]
fn actual_primitive_mrt_keeps_placed_intervals_and_overlay_addresses(){

    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    for density in [1.,2.]{gpu.set_density(density);pixels(&gpu);let before=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(before[0],0);assert_eq!(before[120*320+80],1);assert_eq!(before[120*320+240],2);assert_eq!(gpu.primitive.texture.format(),crate::primitive_image::FORMAT);
        let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner:a,part:crate::pick_sources::Part::Face(key)})).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),before,"A derived selected-face mesh must retain original triangle addresses");
        gpu.set_highlight(&scene,None).unwrap();
    }
    let mut instances=Scene::empty();crate::scene_document::import(&mut instances,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();gpu.set_scene(&instances,None);pixels(&gpu);let placed=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(placed[120*320+64],1);assert_eq!(placed[120*320+256],2);
    gpu.resize(crate::viewport::Viewport{width:640,height:480});assert_eq!([gpu.primitive.texture.width(),gpu.primitive.texture.height()],[640,480]);gpu.resize(crate::viewport::Viewport{width:320,height:240});gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|id|*id==0));
}

#[test]
fn actual_first_pick_primitives_match_current_cropped_owners_before_color(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
    for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
        for(position,owner,primitive)in[([-0.5,0.],a,1u32),([0.5,0.],b,2)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();assert_eq!([texture.width(),texture.height()],window.draw[2..]);assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],primitive);}
    }
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,None);gpu.request_pick([0.5,0.],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],b.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],1,"Visible intervals must rebuild without a color frame");
}

fn cap_primitive_frame(gpu:&super::Renderer){
 let texture=gpu.device.create_texture(&wgpu::TextureDescriptor{label:Some("cap primitive fixture"),size:wgpu::Extent3d{width:gpu.size[0],height:gpu.size[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});
 gpu.draw(&texture.create_view(&Default::default()),&Default::default(),&matrix());wait(gpu);
}
#[test]
fn actual_cap_color_primitives_keep_plane_selection_and_empty_pixels(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});cap_primitive_frame(&gpu);
  let words=primitive_pixels(&gpu,&gpu.primitive.texture);let at=(96*density*320*density+208*density)as usize;
  assert_eq!(words[at],0xfffffff0);assert_eq!(words[0],0);
  gpu.select_objects(&scene,&[owner]).unwrap();cap_primitive_frame(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture)[at],0xfffffff1);
  gpu.select_objects(&scene,&[]).unwrap();
 }
 gpu.set_scene(&Scene::empty(),None);cap_primitive_frame(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
}

#[test]
fn actual_first_cap_pick_has_reserved_primitive_and_exact_original_owner(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
  assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
  gpu.request_pick([0.3,0.2],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
  let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let words=primitive_pixels(&gpu,&texture);
  let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];
  assert_eq!(words[(y*texture.width()+x)as usize],0xfffffff0);
  assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);
 }
}

#[test]
fn actual_compact_surface_upload_uses_twenty_bytes_per_vertex_and_releases(){
 let scene=Scene::demo();let mut gpu=renderer(&scene);let source=scene.objects()[0].mesh.clone();let original=source.vertices().to_vec();
 let buffers=gpu.meshes[0].geometry.buffers();
 assert_eq!(gpu.meshes[0].geometry.allocated_bytes(),0);
 assert_eq!(gpu.usage()[1],buffers.0.size()+buffers.1.size(),"One arena owner accounts for the actual shared buffers");
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let mut cache=crate::geometry_cache::GeometryCache::default();let uploaded=cache.get(&ctx,&source);
 assert_eq!(uploaded.buffers().0.size(),source.vertices().len()as u64*20);
 assert_eq!(cache.uploaded_bytes,cache.allocated_bytes(),"The first nonempty compact upload matches actual GPU allocation");
 let written=cache.uploaded_bytes;cache.get(&ctx,&source);assert_eq!(cache.uploads,1);assert_eq!(cache.uploaded_bytes,written);
 let empty=crate::gpu_geometry::GpuGeometry::upload(&gpu.device,std::rc::Rc::new(crate::mesh::Mesh::new(vec![],vec![]).unwrap()));assert_eq!(empty.allocated_bytes(),24);
 pixels(&gpu);assert_eq!(source.vertices(),original);let uploads=gpu.stats();pixels(&gpu);assert_eq!(gpu.stats(),uploads);
 gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.arena_cpu_bytes(),0);
}
#[test]
fn actual_nonzero_shared_ranges_keep_surface_face_picks_projection_outlines_and_caps() {
    use std::cell::RefCell;
    use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh,text_gpu_support::GpuCtx};
    for clipped in [false,true] {
        let scene=if clipped {crate::cap_gpu_tests::scene(false).0} else {
            let mut scene=Scene::empty();scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();scene
        };
        let mut r=renderer(&scene);r.set_outlines(true);
        let expected=pixels(&r);
        let mut hits=Vec::new();
        for mode in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face] {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            hits.push(r.take_pick(matrix()).unwrap().unwrap());
        }
        assert!(hits.iter().all(Option::is_some));
        if !clipped {assert_ne!(hits[1].unwrap()[1],0,"The uncut source returns an original face address");}
        let ctx=GpuCtx::new(&r.device,&r.queue);
        let arena=Rc::new(RefCell::new(crate::mesh_arena::Arena::new(&ctx)));
        let decoy=Rc::new(crate::mesh::Mesh::new(vec![[0.7,0.7,-0.6,0.,1.,0.],[0.9,0.7,-0.6,0.,1.,0.],[0.8,0.9,-0.6,0.,1.,0.]],vec![0,1,2]).unwrap());
        arena.borrow_mut().upload(&ctx,&decoy).unwrap();
        let layout=r.pipeline.get_bind_group_layout(1);
        for mesh in &mut r.meshes {
            let object=scene.objects().iter().find(|object|object.id==mesh.id).unwrap();
            let geometry=Rc::new(GpuGeometry::shared(&ctx,object.mesh.clone(),arena.clone()).unwrap());
            assert!(geometry.index_start>=3);
            *mesh=GpuMesh::with_geometry(&r.device,&layout,object,super::style(&scene,object.id,false),geometry);
        }
        r.caps.borrow_mut().set_scene(&r.queue,&scene,&mut r.meshes,&r.clipping.borrow().rows);
        let mut base=0;for mesh in &mut r.meshes {mesh.set_primitive_base(&r.queue,base).unwrap();base+=mesh.geometry.index_count/3;}
        r.tiles.borrow_mut().set(&r.device,&r.meshes);r.faces.clear();
        if let Some(outline)=r.global_outline.borrow_mut().as_mut(){outline.invalidate();}
        assert_eq!(pixels(&r),expected,"Nonzero shared ranges preserve actual color, clipping and outline pixels");
        for (mode,expected) in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face].into_iter().zip(hits) {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            assert_eq!(r.take_pick(matrix()).unwrap().unwrap(),expected,"Shared source addressing preserves original picks");
        }
        assert!(r.tiles.borrow().usage()[3]>0);
        if clipped {assert_eq!(r.cap_usage()[1],1);}
    }
}

#[test]
fn actual_contact_marker_is_opaque_unlit_selected_and_keeps_original_ownership(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();
 let vertices=prepared.display.vertices().len();prepared.display=prepared.display.with_alphas(vec![1./255.;vertices]).unwrap();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);
 let source=scene.objects()[0].mesh.clone();let color=source.vertices()[0][3..6].to_vec();let expected=color.iter().map(|v|(v.clamp(0.,1.)*255.).round()as u8).collect::<Vec<_>>();
 let uploads=gpu.stats()[0];for opacity in [1.,0.5,0.] {gpu.set_opacity(opacity);let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],expected.as_slice());gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
 gpu.select_objects(&scene,&[owner]).unwrap();let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],&[255,255,0]);
 assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);assert_eq!(gpu.stats()[0],uploads);assert_eq!(source.alphas(),vec![1./255.;vertices]);
 scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn current_palette_is_shared_by_surface_shader_and_preserves_original_picks_and_color(){
 use wgpu::naga;
 let rgba=crate::colour::selection();assert_eq!(rgba,session_rust::Color::yellow().to_unified_array());assert_eq!(rgba,[1.,1.,0.,1.]);
 let shader=crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0);assert!(shader.contains(&crate::colour::selection_shader()));naga::valid::Validator::new(naga::valid::ValidationFlags::all(),naga::valid::Capabilities::all()).validate(&naga::front::wgsl::parse_str(&shader).unwrap()).unwrap();
 let mut scene=Scene::empty();let prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let plain=pixels(&gpu);let uploads=gpu.stats()[0];
 gpu.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&gpu);assert!(selected.chunks_exact(4).any(|p|p[0]>10&&p[0]==p[1]&&p[2]==0));assert_ne!(plain,selected);gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
 gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),plain);assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn full_primitive_base_survives_surface_selection_and_reserved_range_checks(){
 assert!(include_str!("clip.wgsl").contains(&format!("const CAP_PRIMITIVE: u32 = 0x{:08x}u;",crate::primitive_image::CAP_START)));
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.4,-0.4,-0.4],[0.4,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let mut gpu=renderer(&scene);let original=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 for base in [0x8000_0000,crate::primitive_image::CAP_START-2]{
  gpu.meshes[0].set_primitive_base(&gpu.queue,base).unwrap();pixels(&gpu);let plain=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(plain[120*320+160],base+1);
  gpu.select_objects(&scene,&[owner]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);assert_eq!(u32::from_ne_bytes(gpu.meshes[0].packed()[64..68].try_into().unwrap()),1);
  gpu.select_objects(&scene,&[]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);let before=gpu.meshes[0].packed();for invalid in [crate::primitive_image::CAP_START-1,crate::primitive_image::CAP_START,u32::MAX]{assert!(gpu.meshes[0].set_primitive_base(&gpu.queue,invalid).is_err());assert_eq!(gpu.meshes[0].packed(),before);}
 }
 assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
}

#[test]
fn actual_ordinary_surfaces_keep_one_own_row_slot_through_instances_highlights_and_release(){
 let mut scene=Scene::empty();crate::scene_document::import(&mut scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=[crate::draw_slots::OWN_ROW,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();assert!(gpu.instances.slots.allocated_bytes()>=8);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);let originals=scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>();let owner=scene.objects()[0].id;
 for opacity in [1.,0.5]{gpu.set_opacity(opacity);let plain=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner,part:crate::pick_sources::Part::Face(key)})).unwrap();assert_ne!(pixels(&gpu),plain);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_highlight(&scene,None).unwrap();assert_eq!(pixels(&gpu),plain);}
 assert_eq!(scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>(),originals);gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);
}

#[test]
fn actual_surface_batches_share_one_slot_owner_and_restore_ordinary_drawing(){
 use prost::Message;let mut source=crate::instance_specimen::message();let mut second=source.definitions.as_ref().unwrap().meshes[0].clone();second.guid="other-definition".into();source.definitions.as_mut().unwrap().meshes.push(second);source.objects.as_mut().unwrap().instances[1].definition_guid="other-definition".into();source.objects.as_mut().unwrap().instances[0].xform=None;source.objects.as_mut().unwrap().instances[1].flags=0;source.xforms=vec![session_rust::proto::XformEntry {guid:"first".into(),xform:Some(session_rust::Xform::translation(-0.6,0.,0.).to_proto())},session_rust::proto::XformEntry {guid:"second".into(),xform:Some(session_rust::Xform::translation(0.6,0.,0.).to_proto())}];
 let mut scene=crate::scene::Scene::empty();crate::scene_document::import(&mut scene,&source.encode_to_vec(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=crate::cap_gpu_tests::renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(&gpu.instance_usage()[..3],&[2,2,376]);assert_eq!(gpu.instances.slots.instances(),2);let slots=crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,24);assert_eq!(&slots[..8],&[u32::MAX,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(u32::from_ne_bytes(slots[8..12].try_into().unwrap()),0);assert_eq!(u32::from_ne_bytes(slots[16..20].try_into().unwrap()),1);
 let records=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),2*crate::object_record::BYTES as u64);
 let mut actual=records.chunks_exact(crate::object_record::BYTES).map(|r|r.to_vec()).collect::<Vec<_>>();actual.sort_unstable();let mut expected=gpu.meshes.iter().map(|m|m.packed().to_vec()).collect::<Vec<_>>();expected.sort_unstable();assert_eq!(actual,expected,"global records retain both original placements, IDs and ranges");
 let baseline=crate::cap_gpu_tests::pixels(&gpu);
 for half in 0..2 {let blue=baseline.chunks_exact(4).enumerate().filter(|(i,p)|(*i%320)/160==half&&p[2]>p[0].saturating_add(50)&&p[2]>p[1].saturating_add(30)).count();assert!(blue>100,"placed original blue triangles in half {half}: {blue}");}
 let uploads=gpu.stats()[0];let owner=scene.objects()[0].id;gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 gpu.set_opacity(0.5);crate::cap_gpu_tests::pixels(&gpu);gpu.set_opacity(1.);assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.set_scene(&crate::scene::Scene::empty(),None);assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_projected_records_follow_physical_gaps_clear_old_ranges_and_bound_fallback(){
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[-0.4,-0.4,0.],[0.4,-0.4,0.],[0.,0.4,0.]],[0.2,0.4,0.8]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let baseline=pixels(&gpu);let triangles=gpu.meshes[0].geometry.index_count/3;assert_eq!(gpu.meshes.len(),1);let initial=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],u64::from(triangles)*96);let uploads=gpu.stats()[0];
 gpu.meshes[0].set_primitive_base(&gpu.queue,7).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+7));let projected=gpu.tile_buffers()[0].clone();let data=crate::growing_buffer_tests::read(&ctx,&projected,u64::from(triangles+7)*96);assert!(data[..7*96].iter().all(|v|*v==0));assert_eq!(&data[7*96..],initial);let projections=gpu.tile_usage()[5];pixels(&gpu);assert_eq!(gpu.tile_usage()[5],projections);
 gpu.meshes[0].set_primitive_base(&gpu.queue,3).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+3));assert_eq!(gpu.tile_buffers()[0],projected);let data=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],projected.size());assert!(data[..3*96].iter().all(|v|*v==0));assert_eq!(&data[3*96..(triangles as usize+3)*96],initial);assert!(data[(triangles as usize+3)*96..].iter().all(|v|*v==0));
 gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles));let current=gpu.tile_buffers()[0].clone();assert_eq!(current.size(),initial.len()as u64);assert_eq!(crate::growing_buffer_tests::read(&ctx,&current,current.size()),initial);assert_eq!(gpu.stats()[0],uploads);
 gpu.meshes[0].set_primitive_base(&gpu.queue,0x80000000).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);pixels(&gpu);assert!(gpu.tiles.borrow().fallback);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
 gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzp-share-opaque-ordinary-surface-records-13.rs" skip=28867
```

</details>

### 5. `src/object_rows.rs`

Reuse the record table’s existing CPU snapshot; use the current ordinary uniform path after a mesh range or record changes.

<details>
<summary>Locate the existing block</summary>

```rust
 pub fn bind(&self,pass:&mut wgpu::RenderPass<'_>){pass.set_bind_group(1,&self.group,&[]);}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
 pub(crate) fn matches(&self,row:u32,record:&Record)->bool {self.previous.get(row as usize)==Some(record)}
 pub fn bind(&self,pass:&mut wgpu::RenderPass<'_>){pass.set_bind_group(1,&self.group,&[]);}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzp-share-opaque-ordinary-surface-records-14.rs"
```

</details>

</details>

<a id="checkpoint-96zzzq-blend-ordinary-surface-records"></a>

### Step 23 of 24 · Blend ordinary surface records (`96zzzq-blend-ordinary-surface-records`)

- **Goal:** Use the shared ordinary record table at the current translucent default.
- **Data Flow Trace:** `Shared ordinary record → alpha-blended color plus unchanged integer primitive → earlier scene order`
- **Files You Touch:** `src/instance_gpu.rs` (*Keep a blended ordinary pipeline beside the existing instance pipeline.*) · `src/renderer.rs` (*Ordinary storage records now work at the current translucent default as well as Opacity 1.*) · `src/global_outline_tests.rs` (*Supply exact storage-versus-uniform RGBA and primitive images at x-ray, half opacity, the current default and full opacity.*)

An ordinary surface uses alpha blending in the earlier renderer. Keep that exact policy while reading its shared storage record. Build the ordinary and instance pipeline states from one descriptor and shader, then preserve the earlier ordinary draw order. Temporary overlays and capacity or changed-record fallbacks still use their current uniform path.

> **Self-Check Question:** *Why share a shader but keep a distinct ordinary blend state?*  
> **Answer:** The ordinary renderer already blends color while writing the same primitive IDs. Changing the record resource must preserve that pipeline state; the existing instance state remains separate.

- **What You Should See:** Ordinary surfaces read the shared table at Opacity 0.95, 0.5 and 1, with exact earlier colors and original picks.
- **Hands-On Lab Experiment:** Use Opacity 0.95, 0.5, 0 and 1. Select an ordinary original and restore it. Inspect the supplied exact color and primitive comparison with the earlier uniform renderer.

<details>
<summary>Implementation 96zzzq-blend-ordinary-surface-records: Blend ordinary surface records</summary>

Use the shared ordinary record table at the current translucent default.

Data flow: Shared ordinary record → alpha-blended color plus unchanged integer primitive → earlier scene order.

### 1. `src/instance_gpu.rs`

Keep a blended ordinary pipeline beside the existing instance pipeline. Share the full pipeline descriptor instead of repeating it for blending. Use the same ordinary alpha-blend policy already written in the uniform renderer. Create both states from one shader, layout and descriptor. Own the ordinary blend state with the existing shared record consumers. Blend ordinary records while keeping their exact primitive IDs, draw count and scene order.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {
        let Some(&(first,row))=self.ordinary.get(&mesh.id)else{return false};
        if !self.records.matches(row,&mesh.packed()){return false;}
        pass.set_pipeline(&self.pipeline);self.records.bind(pass);self.slots.bind(pass,0,first);mesh.geometry.draw_pulled(pass,1);true
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
```

</details>

Replace it with:

```rust
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {
        let Some(&(first,row))=self.ordinary.get(&mesh.id)else{return false};
        if !self.records.matches(row,&mesh.packed()){return false;}
        pass.set_pipeline(&self.ordinary_pipeline);self.records.bind(pass);self.slots.bind(pass,0,first);mesh.geometry.draw_pulled(pass,1);true
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
```

<details>
<summary>Locate this block</summary>

```rust
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(format.into()),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {pipeline,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>,ordinary:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=crate::object_rows::capacity(device);
```

</details>

Replace it with:

```rust
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let create=|blend|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        let pipeline=create(None);let ordinary_pipeline=create(Some(wgpu::BlendState::ALPHA_BLENDING));
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {pipeline,ordinary_pipeline,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>,ordinary:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=crate::object_rows::capacity(device);
```

<details>
<summary>Locate this block</summary>

```rust
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) ordinary:std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn batch_rows(rows:impl IntoIterator<Item=(crate::scene::ObjectId,usize,bool)>,limit:usize)->std::collections::HashSet<crate::scene::ObjectId> {
 let rows=rows.into_iter().collect::<Vec<_>>();let mut uses=std::collections::HashMap::new();
 for &(_,key,_)in &rows {*uses.entry(key).or_insert(0usize)+=1;}
```

</details>

Replace it with:

```rust
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pipeline:wgpu::RenderPipeline,ordinary_pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) ordinary:std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn batch_rows(rows:impl IntoIterator<Item=(crate::scene::ObjectId,usize,bool)>,limit:usize)->std::collections::HashSet<crate::scene::ObjectId> {
 let rows=rows.into_iter().collect::<Vec<_>>();let mut uses=std::collections::HashMap::new();
 for &(_,key,_)in &rows {*uses.entry(key).or_insert(0usize)+=1;}
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh};
use std::{collections::BTreeMap,rc::Rc};
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pipeline:wgpu::RenderPipeline,ordinary_pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) ordinary:std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn batch_rows(rows:impl IntoIterator<Item=(crate::scene::ObjectId,usize,bool)>,limit:usize)->std::collections::HashSet<crate::scene::ObjectId> {
 let rows=rows.into_iter().collect::<Vec<_>>();let mut uses=std::collections::HashMap::new();
 for &(_,key,_)in &rows {*uses.entry(key).or_insert(0usize)+=1;}
 let candidates=rows.iter().filter(|(_,key,declared)|*declared||uses[key]>1).map(|(id,_,_)|*id).collect::<std::collections::HashSet<_>>();
 if candidates.len()<=limit {candidates}else{rows.into_iter().filter(|(_,_,declared)|*declared).map(|(id,_,_)|id).collect()}
}
pub(crate) fn source()->String {
 let source=include_str!("triangle.wgsl")
  .replace("var<uniform> object_settings: ObjectSettings;","var<storage, read> objects: array<ObjectSettings>;")
  .replace("return object_settings;","return objects[index];");
 crate::clipping_gpu::source(&source,2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("shared instance records"),entries:&[wgpu::BindGroupLayoutEntry {binding:0,visibility:wgpu::ShaderStages::VERTEX,ty:wgpu::BindingType::Buffer {ty:wgpu::BufferBindingType::Storage {read_only:true},has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(BYTES as u64)},count:None}]});
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let create=|blend|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        let pipeline=create(None);let ordinary_pipeline=create(Some(wgpu::BlendState::ALPHA_BLENDING));
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {pipeline,ordinary_pipeline,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>,ordinary:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=crate::object_rows::capacity(device);
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:BTreeMap<usize,(Rc<GpuGeometry>,Vec<Record>)>=BTreeMap::new();
        for row in rows {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push(row.packed());}
        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let mut batches=BTreeMap::new();let mut all_slots=Vec::new();let mut all_records=Vec::with_capacity(count);
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
            for record in &records {
                let row=u32::try_from(all_records.len()).map_err(|_|"Instance record row")?;
                all_slots.push([row,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]);
                all_records.push(*record);
            }
            batches.insert(source,Batch {geometry,count:records.len()as u32,first_slot});
        }
        let own=ordinary.into_iter().collect::<Vec<_>>();let mut ordinary=std::collections::HashMap::new();
        if own.len()<=limit-count {
            for mesh in own {
                let first=u32::try_from(all_slots.len()+1).map_err(|_|"Ordinary slot range")?;
                let row=u32::try_from(all_records.len()).map_err(|_|"Ordinary record row")?;
                all_slots.push([row,mesh.primitive_base()]);all_records.push(mesh.packed());ordinary.insert(mesh.id,(first,row));
            }
        }
        let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
        self.records.set(&ctx,&self.layout,&all_records)?;
        if self.records.written_bytes>0{self.writes+=1;self.written_bytes+=self.records.written_bytes;}
        if all_slots.is_empty()&&self.slots.instances()>0{self.slots.clear(&ctx,true);}else{self.slots.set(&ctx,&all_slots);}
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;self.ordinary=ordinary;Ok(())
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {
        let Some(&(first,row))=self.ordinary.get(&mesh.id)else{return false};
        if !self.records.matches(row,&mesh.packed()){return false;}
        pass.set_pipeline(&self.ordinary_pipeline);self.records.bind(pass);self.slots.bind(pass,0,first);mesh.geometry.draw_pulled(pass,1);true
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        self.records.bind(pass);
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub(crate) fn record_buffer(&self)->&wgpu::Buffer{&self.records.buffer.buf}
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.records.allocated_bytes()+if self.slots.instances()==0{0}else{self.slots.allocated_bytes()},self.writes,self.written_bytes]}
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzq-blend-ordinary-surface-records-01.rs"
```

```text
--8<-- "journey/code/96zzzq-blend-ordinary-surface-records-02.rs"
```

```text
--8<-- "journey/code/96zzzq-blend-ordinary-surface-records-03.rs"
```

```text
--8<-- "journey/code/96zzzq-blend-ordinary-surface-records-04.rs"
```

```text
--8<-- "journey/code/96zzzq-blend-ordinary-surface-records-05.rs"
```

```text
--8<-- "journey/code/96zzzq-blend-ordinary-surface-records-06.rs"
```

</details>

### 2. `src/renderer.rs`

Ordinary storage records now work at the current translucent default as well as Opacity 1.

<details>
<summary>Locate the existing block</summary>

```rust
            for mesh in self.meshes.iter().filter(|row|self.opacity.get()<1.||!self.instance_rows.contains(&row.id)) {if !(self.opacity.get()>=1.&&self.instances.draw_ordinary(&mut pass,mesh)){pass.set_pipeline(&self.pipeline);self.instances.bind_own(&mut pass);mesh.draw_pulled(&mut pass);}}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
            for mesh in self.meshes.iter().filter(|row|self.opacity.get()<1.||!self.instance_rows.contains(&row.id)) {if !self.instances.draw_ordinary(&mut pass,mesh){pass.set_pipeline(&self.pipeline);self.instances.bind_own(&mut pass);mesh.draw_pulled(&mut pass);}}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzq-blend-ordinary-surface-records-07.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/global_outline_tests.rs`

Supply exact storage-versus-uniform RGBA and primitive images at x-ray, half opacity, the current default and full opacity.

<details>
<summary>Locate the existing block</summary>

```text
 scene.clear();gpu.set_scene(&scene,None);assert!(gpu.instances.ordinary.is_empty());assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzq-blend-ordinary-surface-records-08.rs"
```

</details>

</details>

<a id="checkpoint-96zzzr-share-transparent-instance-records"></a>

### Step 24 of 24 · Share transparent instance records (`96zzzr-share-transparent-instance-records`)

- **Goal:** Draw transparent placements from the same shared records in scene order.
- **Data Flow Trace:** `Original object ID → existing shared record and slot → one blended draw in scene order`
- **Files You Touch:** `src/instance_gpu.rs` (*Retain each original owner beside its packed instance record.*) · `src/global_outline_tests.rs` (*Supply exact transparent-copy color and primitive comparisons, independent selection, unchanged originals, uploads and full release.*)

An opaque placement can join a geometry batch. A translucent placement must retain the earlier individual scene order. Save the original object ID beside each packed record, then reuse the existing one-object draw lookup for both ordinary surfaces and placements. This changes the resource lookup without changing CAD ownership or introducing another shader.

> **Self-Check Question:** *Why retain a one-object lookup for records already used by a batch?*  
> **Answer:** Transparency uses individual scene order. The lookup selects the same record and physical primitive base without regrouping those draws or copying another record table.

- **What You Should See:** Transparent placements consume the shared object table while retaining original pixels, picks and scene order.
- **Hands-On Lab Experiment:** Set Opacity to 0.5 and 0.95 in Example Instances. Select each placement, clear selection and restore Opacity 1. Inspect the supplied exact comparison against the earlier uniform draw path.

<details>
<summary>Implementation 96zzzr-share-transparent-instance-records: Share transparent instance records</summary>

Draw transparent placements from the same shared records in scene order.

Data flow: Original object ID → existing shared record and slot → one blended draw in scene order.

### 1. `src/instance_gpu.rs`

Retain each original owner beside its packed instance record. Keep source ownership while preparing shared geometry batches. Build one lookup for individual draws alongside the existing batches. Read the original identity without encoding it as a floating-point value. Index the existing instance slot and record for a single blended draw. Append optional ordinary rows to the same lookup. Distinguish the ordinary input iterator from the individual-draw lookup.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {pipeline,ordinary_pipeline,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>,ordinary:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=crate::object_rows::capacity(device);
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:BTreeMap<usize,(Rc<GpuGeometry>,Vec<Record>)>=BTreeMap::new();
        for row in rows {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push(row.packed());}
        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let mut batches=BTreeMap::new();let mut all_slots=Vec::new();let mut all_records=Vec::with_capacity(count);
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
            for record in &records {
                let row=u32::try_from(all_records.len()).map_err(|_|"Instance record row")?;
                all_slots.push([row,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]);
                all_records.push(*record);
            }
            batches.insert(source,Batch {geometry,count:records.len()as u32,first_slot});
        }
        let own=ordinary.into_iter().collect::<Vec<_>>();let mut ordinary=std::collections::HashMap::new();
        if own.len()<=limit-count {
            for mesh in own {
                let first=u32::try_from(all_slots.len()+1).map_err(|_|"Ordinary slot range")?;
```

</details>

Replace it with:

```rust
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {pipeline,ordinary_pipeline,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>,ordinary_rows:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=crate::object_rows::capacity(device);
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:BTreeMap<usize,(Rc<GpuGeometry>,Vec<(crate::scene::ObjectId,Record)>)>=BTreeMap::new();
        for row in rows {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push((row.id,row.packed()));}
        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let mut batches=BTreeMap::new();let mut ordinary=std::collections::HashMap::new();let mut all_slots=Vec::new();let mut all_records=Vec::with_capacity(count);
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
            for (id,record) in &records {
                let row=u32::try_from(all_records.len()).map_err(|_|"Instance record row")?;
                all_slots.push([row,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]);
                ordinary.insert(*id,(u32::try_from(all_slots.len()).map_err(|_|"Instance slot range")?,row));
                all_records.push(*record);
            }
            batches.insert(source,Batch {geometry,count:records.len()as u32,first_slot});
        }
        let own=ordinary_rows.into_iter().collect::<Vec<_>>();
        if own.len()<=limit-count {
            for mesh in own {
                let first=u32::try_from(all_slots.len()+1).map_err(|_|"Ordinary slot range")?;
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh};
use std::{collections::BTreeMap,rc::Rc};
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pipeline:wgpu::RenderPipeline,ordinary_pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) ordinary:std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn batch_rows(rows:impl IntoIterator<Item=(crate::scene::ObjectId,usize,bool)>,limit:usize)->std::collections::HashSet<crate::scene::ObjectId> {
 let rows=rows.into_iter().collect::<Vec<_>>();let mut uses=std::collections::HashMap::new();
 for &(_,key,_)in &rows {*uses.entry(key).or_insert(0usize)+=1;}
 let candidates=rows.iter().filter(|(_,key,declared)|*declared||uses[key]>1).map(|(id,_,_)|*id).collect::<std::collections::HashSet<_>>();
 if candidates.len()<=limit {candidates}else{rows.into_iter().filter(|(_,_,declared)|*declared).map(|(id,_,_)|id).collect()}
}
pub(crate) fn source()->String {
 let source=include_str!("triangle.wgsl")
  .replace("var<uniform> object_settings: ObjectSettings;","var<storage, read> objects: array<ObjectSettings>;")
  .replace("return object_settings;","return objects[index];");
 crate::clipping_gpu::source(&source,2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("shared instance records"),entries:&[wgpu::BindGroupLayoutEntry {binding:0,visibility:wgpu::ShaderStages::VERTEX,ty:wgpu::BindingType::Buffer {ty:wgpu::BufferBindingType::Storage {read_only:true},has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(BYTES as u64)},count:None}]});
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let create=|blend|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        let pipeline=create(None);let ordinary_pipeline=create(Some(wgpu::BlendState::ALPHA_BLENDING));
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {pipeline,ordinary_pipeline,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>,ordinary_rows:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=crate::object_rows::capacity(device);
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:BTreeMap<usize,(Rc<GpuGeometry>,Vec<(crate::scene::ObjectId,Record)>)>=BTreeMap::new();
        for row in rows {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push((row.id,row.packed()));}
        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let mut batches=BTreeMap::new();let mut ordinary=std::collections::HashMap::new();let mut all_slots=Vec::new();let mut all_records=Vec::with_capacity(count);
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
            for (id,record) in &records {
                let row=u32::try_from(all_records.len()).map_err(|_|"Instance record row")?;
                all_slots.push([row,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]);
                ordinary.insert(*id,(u32::try_from(all_slots.len()).map_err(|_|"Instance slot range")?,row));
                all_records.push(*record);
            }
            batches.insert(source,Batch {geometry,count:records.len()as u32,first_slot});
        }
        let own=ordinary_rows.into_iter().collect::<Vec<_>>();
        if own.len()<=limit-count {
            for mesh in own {
                let first=u32::try_from(all_slots.len()+1).map_err(|_|"Ordinary slot range")?;
                let row=u32::try_from(all_records.len()).map_err(|_|"Ordinary record row")?;
                all_slots.push([row,mesh.primitive_base()]);all_records.push(mesh.packed());ordinary.insert(mesh.id,(first,row));
            }
        }
        let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
        self.records.set(&ctx,&self.layout,&all_records)?;
        if self.records.written_bytes>0{self.writes+=1;self.written_bytes+=self.records.written_bytes;}
        if all_slots.is_empty()&&self.slots.instances()>0{self.slots.clear(&ctx,true);}else{self.slots.set(&ctx,&all_slots);}
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;self.ordinary=ordinary;Ok(())
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {
        let Some(&(first,row))=self.ordinary.get(&mesh.id)else{return false};
        if !self.records.matches(row,&mesh.packed()){return false;}
        pass.set_pipeline(&self.ordinary_pipeline);self.records.bind(pass);self.slots.bind(pass,0,first);mesh.geometry.draw_pulled(pass,1);true
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        self.records.bind(pass);
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub(crate) fn record_buffer(&self)->&wgpu::Buffer{&self.records.buffer.buf}
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.records.allocated_bytes()+if self.slots.instances()==0{0}else{self.slots.allocated_bytes()},self.writes,self.written_bytes]}
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzr-share-transparent-instance-records-01.rs"
```

```text
--8<-- "journey/code/96zzzr-share-transparent-instance-records-02.rs"
```

```text
--8<-- "journey/code/96zzzr-share-transparent-instance-records-03.rs"
```

```text
--8<-- "journey/code/96zzzr-share-transparent-instance-records-04.rs"
```

```text
--8<-- "journey/code/96zzzr-share-transparent-instance-records-05.rs"
```

```text
--8<-- "journey/code/96zzzr-share-transparent-instance-records-06.rs"
```

```text
--8<-- "journey/code/96zzzr-share-transparent-instance-records-07.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/global_outline_tests.rs`

Supply exact transparent-copy color and primitive comparisons, independent selection, unchanged originals, uploads and full release.

<details>
<summary>Locate the existing block</summary>

```text
 scene.clear();gpu.set_scene(&scene,None);assert!(gpu.instances.ordinary.is_empty());
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzr-share-transparent-instance-records-08.rs"
```

</details>

</details>

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 96zzzr-share-transparent-instance-records
npm --prefix ../session_tests run course -- check 96zzzr-share-transparent-instance-records
npm --prefix ../session_tests run course -- save 96zzzr-share-transparent-instance-records
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

Transparent placements consume the shared object table while retaining original pixels, picks and scene order.

- **Final Lesson Experiment:** Set Opacity to 0.5 and 0.95 in Example Instances. Select each placement, clear selection and restore Opacity 1. Inspect the supplied exact comparison against the earlier uniform draw path.

Continue with [Original picks, ambient metadata and cap consumers](../../consolidation/96zzzs-allocate-stable-object-record-rows.md).
