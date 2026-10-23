# 20.5 · Original picks, ambient metadata and cap consumers

**Section 10: Shared GPU Arenas, Floating-Origin Precision, Multisampling, and Production Parity** · **Lesson 20.5** (Redraft `10.9`) · **16 checkpoints (`96zzzs-allocate-stable-object-record-rows` → `96zzzzh-pick-caps-through-shared-records`)** · **Study time: 12.800000000000002–16 h**

> **What You Will Learn & Build:** Prepare bounded row reuse without moving surviving originals. By the final checkpoint (`96zzzzh-pick-caps-through-shared-records`), you will connect both cap crossing passes to current records and retain original owners.

## 1. Visual Pipeline & WebGPU Architecture (Black & White Schematics)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 220" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="212" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Lesson 20.5 Execution Flow: Original picks, ambient metadata and cap consumers</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">Concrete data pipeline you build in this lesson (16 Checkpoints: 96zzzs-allocate-stable-object-record-rows → 96zzzzh-pick-caps-through-shared-records)</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="89" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="92" x2="151" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="89" y="108" text-anchor="middle" font-size="11" fill="#111111">Original IDs plus</text>
    <text x="89" y="124" text-anchor="middle" font-size="11" fill="#111111">previous rows</text>
    <line x1="161" y1="108" x2="181" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="183" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="254" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="193" y1="92" x2="316" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="254" y="108" text-anchor="middle" font-size="11" fill="#111111">retained rows and</text>
    <text x="254" y="124" text-anchor="middle" font-size="11" fill="#111111">reused holes</text>
    <line x1="326" y1="108" x2="346" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="348" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="358" y1="92" x2="481" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">Current record and</text>
    <text x="419" y="124" text-anchor="middle" font-size="11" fill="#111111">explicit plane</text>
    <line x1="491" y1="108" x2="511" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="513" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="584" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="523" y1="92" x2="646" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="584" y="108" text-anchor="middle" font-size="11" fill="#111111">atomic crossings</text>
    <line x1="656" y1="108" x2="676" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="678" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="749" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 5</text>
    <line x1="688" y1="92" x2="811" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="749" y="108" text-anchor="middle" font-size="11" fill="#111111">original integer</text>
    <text x="749" y="124" text-anchor="middle" font-size="11" fill="#111111">cap owner</text>
    
    <rect x="18" y="162" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="181" font-size="11" font-weight="700" fill="#111111">Files You Edit in Lesson 20.5:</text>
    <text x="215" y="181" font-size="11" fill="#111111">src/instance_gpu.rs  ·  src/global_outline_tests.rs  ·  src/object_rows.rs  ·  src/triangle_pick.wgsl  ·  src/pick_pipeline.rs  ·  src/pick_gpu.rs</text>
    <text x="32" y="198" font-size="10.5" fill="#111111">16 Checkpoints: 96zzzs-allocate-stable-object-record-rows → 96zzzzh-pick-caps-through-shared-records — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
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
| **1** | [`96zzzs-allocate-stable-object-record-rows`](#checkpoint-96zzzs-allocate-stable-object-record-rows) | Prepare bounded row reuse without moving surviving originals. | `src/instance_gpu.rs` | `Original IDs plus previous rows → retained rows and reused holes` |
| **2** | [`96zzzt-retain-live-surface-record-rows`](#checkpoint-96zzzt-retain-live-surface-record-rows) | Connect stable row reuse to the existing surface record table. | `src/instance_gpu.rs`, `src/global_outline_tests.rs` | `Live original IDs → stable record indices → existing draw slots` |
| **3** | [`96zzzu-prepare-shared-record-pick-consumers`](#checkpoint-96zzzu-prepare-shared-record-pick-consumers) | Share the storage layout and shader adaptation before connecting first picks. | `src/object_rows.rs`, `src/instance_gpu.rs`, `src/triangle_pick.wgsl` | `Object record → flat original owner plus physical primitive → integer pick fragment` |
| **4** | [`96zzzv-pick-originals-through-shared-records`](#checkpoint-96zzzv-pick-originals-through-shared-records) | Connect first surface picks to current shared object records. | `src/instance_gpu.rs`, `src/pick_pipeline.rs`, `src/pick_gpu.rs`, `src/pick_encode.rs` | `Current shared record and slot → original owner and physical primitive → existing asynchronous readback` |
| **5** | [`96zzzw-read-source-faces-through-shared-records`](#checkpoint-96zzzw-read-source-faces-through-shared-records) | Connect original source-face picking to the same current object records. | `src/instance_gpu.rs`, `src/face_pick.wgsl`, `src/face_pick.rs`, `src/pick_encode.rs` | `Current object record → original source face plus original owner → existing face readback` |
| **6** | [`96zzzx-pack-existing-ambient-radius-in-object-records`](#checkpoint-96zzzx-pack-existing-ambient-radius-in-object-records) | Carry the existing source radius in a spare aligned record word. | `src/gpu_mesh.rs`, `src/renderer.rs`, `src/ambient_sources.wgsl`, `src/ambient_source_pipeline.rs` | `Existing placed-source radius → exact float bits in aligned object record → prepared property vertex entry` |
| **7** | [`96zzzy-read-ambient-properties-through-shared-records`](#checkpoint-96zzzy-read-ambient-properties-through-shared-records) | Connect source normal and radius drawing to the existing object table. | `src/ambient_source_pipeline.rs`, `src/ambient_source_gpu.rs`, `src/ambient_gpu.rs`, `src/renderer.rs` | `Current object row and arena geometry → original source normals and radius → existing ambient recipe` |
| **8** | [`96zzzz-share-the-object-record-shader-schema`](#checkpoint-96zzzz-share-the-object-record-shader-schema) | Keep one aligned object record declaration across drawing, caps and projection. | `src/object_settings.wgsl`, `src/triangle.wgsl`, `src/cap_count.wgsl`, `src/finite_project.wgsl` | `One aligned object declaration → raster and compute shader assembly → unchanged GPU bytes` |
| **9** | [`96zzzza-prepare-shared-record-triangle-projection`](#checkpoint-96zzzza-prepare-shared-record-triangle-projection) | Prepare the compute shader to select the same current object row. | `src/instance_gpu.rs`, `src/finite_project.wgsl`, `src/finite_project_source.rs` | `Current packed-record match → object row → prepared projection shader lookup` |
| **10** | [`96zzzzb-project-triangles-through-shared-records`](#checkpoint-96zzzzb-project-triangles-through-shared-records) | Connect finite triangle projection to the current shared object table. | `src/tile_pipelines.rs`, `src/tile_gpu.rs`, `src/renderer.rs`, `src/global_outline_tests.rs` | `Current object row and arena interval → existing triangle projection → unchanged finite visibility` |
| **11** | [`96zzzzc-prepare-shared-record-cap-counting`](#checkpoint-96zzzzc-prepare-shared-record-cap-counting) | Separate cap crossing geometry from the object resource and vertex format. | `src/cap_count.wgsl`, `src/cap_pipeline.rs` | `Selected object and plane → existing crossing function → unchanged cap color and owner` |
| **12** | [`96zzzzd-prepare-cap-plane-and-arena-bindings`](#checkpoint-96zzzzd-prepare-cap-plane-and-arena-bindings) | Prepare explicit plane parameters with current immutable arena bindings. | `src/cap_geometry.rs`, `src/lib.rs` | `Explicit plane plus current arena buffers → reusable cap geometry group` |
| **13** | [`96zzzze-count-cap-color-through-shared-records`](#checkpoint-96zzzze-count-cap-color-through-shared-records) | Connect cap crossing color to current object records and explicit planes. | `src/cap_count.wgsl`, `src/cap_pipeline.rs`, `src/caps_gpu.rs`, `src/renderer.rs` | `Current object row plus explicit plane → original crossing count → unchanged cap color and depth` |
| **14** | [`96zzzzf-prepare-shared-record-cap-picks`](#checkpoint-96zzzzf-prepare-shared-record-cap-picks) | Prepare both cap pick crossing states without changing original ownership. | `src/cap_count.wgsl`, `src/cap_pick_pipeline.rs`, `src/current_inventory.json` | `Existing pick resources plus explicit arena range → shared vertex entry → unchanged crossing fragments` |
| **15** | [`96zzzzg-own-shared-record-cap-pick-resources`](#checkpoint-96zzzzg-own-shared-record-cap-pick-resources) | Reuse explicit plane and arena bindings under the existing pick lifetime. | `src/cap_geometry.rs`, `src/cap_pick_gpu.rs`, `src/current_inventory.json` | `Pick preparation → bounded plane values and current bindings → existing pick lifetime` |
| **16** | [`96zzzzh-pick-caps-through-shared-records`](#checkpoint-96zzzzh-pick-caps-through-shared-records) | Connect both cap crossing passes to current records and retain original owners. | `src/cap_pick_gpu.rs`, `src/caps_gpu.rs`, `src/pick_encode.rs`, `src/cap_pick_tests.rs` | `Current record and explicit plane → atomic crossings → original integer cap owner` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Read the **Concept Walkthrough** and **Self-Check Q&A** first, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-96zzzs-allocate-stable-object-record-rows"></a>

### Step 1 of 16 · Allocate stable object record rows (`96zzzs-allocate-stable-object-record-rows`)

- **Goal:** Prepare bounded row reuse without moving surviving originals.
- **Data Flow Trace:** `Original IDs plus previous rows → retained rows and reused holes`
- **Files You Touch:** `src/instance_gpu.rs` (*Retain surviving original IDs, reuse the lowest free row and enforce the shared device bound.*)

A packed table can retain a live object at the same row across edits. Preserve rows belonging to surviving IDs, then assign new objects to the lowest free rows. Retired holes are reusable. Bound every row by the same device capacity already used by the record writer. This lesson prepares the allocator; the next lesson connects it to drawing.

> **Self-Check Question:** *Why retain surviving rows before assigning new IDs?*  
> **Answer:** Assigning a new ID first could steal a row still owned by a live object. Reserve survivors, then reuse only free rows.

- **What You Should See:** The bounded stable-row allocator is written; drawing still uses the earlier sequential rows.
- **Hands-On Lab Experiment:** Inspect the supplied reorder, deletion, insertion and capacity fixture. In the browser, use the existing copied-object selection and opacity exercise; allocation is not connected yet.

<details>
<summary>Implementation 96zzzs-allocate-stable-object-record-rows: Allocate stable object record rows</summary>

Prepare bounded row reuse without moving surviving originals.

Data flow: Original IDs plus previous rows → retained rows and reused holes.

### 1. `src/instance_gpu.rs`

Retain surviving original IDs, reuse the lowest free row and enforce the shared device bound.

<details>
<summary>Locate the existing block</summary>

```rust
pub(crate) fn source()->String {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub(crate) fn stable_rows(previous:&std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,ids:&[crate::scene::ObjectId],limit:usize)->Result<std::collections::HashMap<crate::scene::ObjectId,u32>,&'static str> {
 let mut rows=std::collections::HashMap::new();let mut used=std::collections::HashSet::new();
 for id in ids {if let Some(&(_,row))=previous.get(id) {if row as usize>=limit||!used.insert(row){return Err("Invalid retained object row");}rows.insert(*id,row);}}
 let mut next=0usize;
 for id in ids {if rows.contains_key(id){continue;}while next<limit&&used.contains(&(next as u32)){next+=1;}if next>=limit||u32::try_from(next).is_err(){return Err("Object records exceed device limits");}let row=next as u32;rows.insert(*id,row);used.insert(row);}
 Ok(rows)
}
pub(crate) fn source()->String {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzs-allocate-stable-object-record-rows-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/instance_gpu.rs`

Supply original-ID reorder, retired-hole reuse, bounded growth, overflow and empty-release checks.

<details>
<summary>Locate the existing block</summary>

```text
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.records.allocated_bytes()+if self.slots.instances()==0{0}else{self.slots.allocated_bytes()},self.writes,self.written_bytes]}
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzs-allocate-stable-object-record-rows-02.rs" skip=248
```

</details>

</details>

<a id="checkpoint-96zzzt-retain-live-surface-record-rows"></a>

### Step 2 of 16 · Retain live surface record rows (`96zzzt-retain-live-surface-record-rows`)

- **Goal:** Connect stable row reuse to the existing surface record table.
- **Data Flow Trace:** `Live original IDs → stable record indices → existing draw slots`
- **Files You Touch:** `src/instance_gpu.rs` (*Reserve surviving rows before regrouping instance geometry and reuse retired holes.*) · `src/global_outline_tests.rs` (*Supply retained original-row checks through removal and insertion, exact mapped records and full release.*)

Rebuilding geometry batches must not renumber surviving object records. Use the allocator just written, store each packed record at its retained row, and let draw slots select that row. Retired holes contain zero records until a new original reuses them. Keep the existing scene order and geometry batches; release the whole table when the last surface leaves.

> **Self-Check Question:** *Why can draw slots remain dense while object records contain holes?*  
> **Answer:** A draw slot explicitly names its object row. Slot order serves geometry batches; record indices retain live ownership independently.

- **What You Should See:** Surviving visible surfaces retain their record rows through removal, insertion and batching changes.
- **Hands-On Lab Experiment:** Copy a surface, remove one original, add another and select the survivors. Compare transparency and picks, then reload an empty scene to release the table.

<details>
<summary>Implementation 96zzzt-retain-live-surface-record-rows: Retain live surface record rows</summary>

Connect stable row reuse to the existing surface record table.

Data flow: Live original IDs → stable record indices → existing draw slots.

### 1. `src/instance_gpu.rs`

Reserve surviving rows before regrouping instance geometry and reuse retired holes. Point each instance slot at its stable original record row. Write the packed instance record in place, leaving retired holes zero. Use the same retained-row lookup for ordinary and individually blended surfaces.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
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
```

</details>

Replace it with:

```rust
        for row in rows {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push((row.id,row.packed()));}
        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let own=ordinary_rows.into_iter().collect::<Vec<_>>();let own=if own.len()<=limit-count {own}else{Vec::new()};
        let ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
        let indices=stable_rows(&self.ordinary,&ids,limit)?;
        let mut batches=BTreeMap::new();let mut ordinary=std::collections::HashMap::new();let mut all_slots=Vec::new();let mut all_records=vec![[0;BYTES];indices.values().max().map_or(0,|row|*row as usize+1)];
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
            for (id,record) in &records {
                let row=indices[id];
                all_slots.push([row,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]);
                ordinary.insert(*id,(u32::try_from(all_slots.len()).map_err(|_|"Instance slot range")?,row));
                all_records[row as usize]=*record;
            }
            batches.insert(source,Batch {geometry,count:records.len()as u32,first_slot});
        }
        for mesh in own {
            let first=u32::try_from(all_slots.len()+1).map_err(|_|"Ordinary slot range")?;let row=indices[&mesh.id];
            all_slots.push([row,mesh.primitive_base()]);all_records[row as usize]=mesh.packed();ordinary.insert(mesh.id,(first,row));
        }
        let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
        self.records.set(&ctx,&self.layout,&all_records)?;
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
pub(crate) fn stable_rows(previous:&std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,ids:&[crate::scene::ObjectId],limit:usize)->Result<std::collections::HashMap<crate::scene::ObjectId,u32>,&'static str> {
 let mut rows=std::collections::HashMap::new();let mut used=std::collections::HashSet::new();
 for id in ids {if let Some(&(_,row))=previous.get(id) {if row as usize>=limit||!used.insert(row){return Err("Invalid retained object row");}rows.insert(*id,row);}}
 let mut next=0usize;
 for id in ids {if rows.contains_key(id){continue;}while next<limit&&used.contains(&(next as u32)){next+=1;}if next>=limit||u32::try_from(next).is_err(){return Err("Object records exceed device limits");}let row=next as u32;rows.insert(*id,row);used.insert(row);}
 Ok(rows)
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
        let own=ordinary_rows.into_iter().collect::<Vec<_>>();let own=if own.len()<=limit-count {own}else{Vec::new()};
        let ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
        let indices=stable_rows(&self.ordinary,&ids,limit)?;
        let mut batches=BTreeMap::new();let mut ordinary=std::collections::HashMap::new();let mut all_slots=Vec::new();let mut all_records=vec![[0;BYTES];indices.values().max().map_or(0,|row|*row as usize+1)];
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
            for (id,record) in &records {
                let row=indices[id];
                all_slots.push([row,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]);
                ordinary.insert(*id,(u32::try_from(all_slots.len()).map_err(|_|"Instance slot range")?,row));
                all_records[row as usize]=*record;
            }
            batches.insert(source,Batch {geometry,count:records.len()as u32,first_slot});
        }
        for mesh in own {
            let first=u32::try_from(all_slots.len()+1).map_err(|_|"Ordinary slot range")?;let row=indices[&mesh.id];
            all_slots.push([row,mesh.primitive_base()]);all_records[row as usize]=mesh.packed();ordinary.insert(mesh.id,(first,row));
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

#[cfg(test)]
mod stable_row_tests {
 use super::*;
 #[test]
 fn retained_ids_reuse_holes_and_reject_capacity_overflow() {
  let mut scene=crate::scene::Scene::empty();let mesh=||crate::prepared::PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap();let a=scene.insert(mesh()).unwrap();let b=scene.insert(mesh()).unwrap();let c=scene.insert(mesh()).unwrap();
  let old=std::collections::HashMap::from([(a,(1,0)),(b,(2,1)),(c,(3,2))]);let rows=stable_rows(&old,&[c,a,b],3).unwrap();assert_eq!((rows[&a],rows[&b],rows[&c]),(0,1,2));
  let d=scene.insert(mesh()).unwrap();let rows=stable_rows(&old,&[c,d,a],3).unwrap();assert_eq!((rows[&a],rows[&d],rows[&c]),(0,1,2));assert!(!rows.contains_key(&b));assert!(stable_rows(&old,&[a,b,c,d],3).is_err());assert!(stable_rows(&old,&[c],2).is_err());assert!(stable_rows(&old,&[],0).unwrap().is_empty());
 }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzt-retain-live-surface-record-rows-01.rs"
```

```text
--8<-- "journey/code/96zzzt-retain-live-surface-record-rows-02.rs"
```

```text
--8<-- "journey/code/96zzzt-retain-live-surface-record-rows-03.rs"
```

```text
--8<-- "journey/code/96zzzt-retain-live-surface-record-rows-04.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/global_outline_tests.rs`

Supply retained original-row checks through removal and insertion, exact mapped records and full release.

<details>
<summary>Locate the existing block</summary>

```text
 scene.clear();gpu.set_scene(&scene,None);assert!(gpu.instances.ordinary.is_empty());
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzt-retain-live-surface-record-rows-05.rs"
```

</details>

</details>

<a id="checkpoint-96zzzu-prepare-shared-record-pick-consumers"></a>

### Step 3 of 16 · Prepare shared record pick consumers (`96zzzu-prepare-shared-record-pick-consumers`)

- **Goal:** Share the storage layout and shader adaptation before connecting first picks.
- **Data Flow Trace:** `Object record → flat original owner plus physical primitive → integer pick fragment`
- **Files You Touch:** `src/object_rows.rs` (*Own the record layout and shader adaptation once for every shared consumer.*) · `src/instance_gpu.rs` (*Reuse the exact existing storage binding contract.*) · `src/triangle_pick.wgsl` (*Carry the original owner as a flat unsigned vertex result.*)

The pick fragment needs the original owner, not an object buffer of its own. Carry that integer from the selected object record through a flat vertex output. Reuse one record layout and one shader adaptation for shared consumers. Add a slot-aware vertex entry beside the existing ordinary entry; both call the same geometry helper. The next lesson selects the storage entry when a current record exists.

> **Self-Check Question:** *Why carry an unsigned owner through the vertex output?*  
> **Answer:** The fragment must identify the original object selected by the draw slot. A flat unsigned varying preserves every ID and removes its direct uniform dependency.

- **What You Should See:** Ordinary picks preserve their earlier owner and primitive output; shared record picking is prepared but not connected.
- **Hands-On Lab Experiment:** Pick both copied originals and an original source face. Inspect the slot-aware entry and compare the existing high-bit original-ID GPU fixtures.

<details>
<summary>Implementation 96zzzu-prepare-shared-record-pick-consumers: Prepare shared record pick consumers</summary>

Share the storage layout and shader adaptation before connecting first picks.

Data flow: Object record → flat original owner plus physical primitive → integer pick fragment.

### 1. `src/object_rows.rs`

Own the record layout and shader adaptation once for every shared consumer.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Rows {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn layout(device:&wgpu::Device)->wgpu::BindGroupLayout {
 device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("shared object records"),entries:&[wgpu::BindGroupLayoutEntry {binding:0,visibility:wgpu::ShaderStages::VERTEX,ty:wgpu::BindingType::Buffer {ty:wgpu::BufferBindingType::Storage {read_only:true},has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(BYTES as u64)},count:None}]})
}
pub fn source(shader:&str)->String {
 shader.replace("var<uniform> object_settings: ObjectSettings;","var<storage, read> objects: array<ObjectSettings>;").replace("return object_settings;","return objects[index];")
}
pub struct Rows {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzu-prepare-shared-record-pick-consumers-01.rs"
```

</details>

### 2. `src/instance_gpu.rs`

Reuse the exact existing storage binding contract. Adapt the same surface shader through the shared owner.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
 Ok(rows)
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
```

</details>

Replace it with:

```rust
 Ok(rows)
}
pub(crate) fn source()->String {
 crate::clipping_gpu::source(&crate::object_rows::source(include_str!("triangle.wgsl")),2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=crate::object_rows::layout(device);
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
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
pub(crate) fn stable_rows(previous:&std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,ids:&[crate::scene::ObjectId],limit:usize)->Result<std::collections::HashMap<crate::scene::ObjectId,u32>,&'static str> {
 let mut rows=std::collections::HashMap::new();let mut used=std::collections::HashSet::new();
 for id in ids {if let Some(&(_,row))=previous.get(id) {if row as usize>=limit||!used.insert(row){return Err("Invalid retained object row");}rows.insert(*id,row);}}
 let mut next=0usize;
 for id in ids {if rows.contains_key(id){continue;}while next<limit&&used.contains(&(next as u32)){next+=1;}if next>=limit||u32::try_from(next).is_err(){return Err("Object records exceed device limits");}let row=next as u32;rows.insert(*id,row);used.insert(row);}
 Ok(rows)
}
pub(crate) fn source()->String {
 crate::clipping_gpu::source(&crate::object_rows::source(include_str!("triangle.wgsl")),2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=crate::object_rows::layout(device);
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
        let own=ordinary_rows.into_iter().collect::<Vec<_>>();let own=if own.len()<=limit-count {own}else{Vec::new()};
        let ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
        let indices=stable_rows(&self.ordinary,&ids,limit)?;
        let mut batches=BTreeMap::new();let mut ordinary=std::collections::HashMap::new();let mut all_slots=Vec::new();let mut all_records=vec![[0;BYTES];indices.values().max().map_or(0,|row|*row as usize+1)];
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
            for (id,record) in &records {
                let row=indices[id];
                all_slots.push([row,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]);
                ordinary.insert(*id,(u32::try_from(all_slots.len()).map_err(|_|"Instance slot range")?,row));
                all_records[row as usize]=*record;
            }
            batches.insert(source,Batch {geometry,count:records.len()as u32,first_slot});
        }
        for mesh in own {
            let first=u32::try_from(all_slots.len()+1).map_err(|_|"Ordinary slot range")?;let row=indices[&mesh.id];
            all_slots.push([row,mesh.primitive_base()]);all_records[row as usize]=mesh.packed();ordinary.insert(mesh.id,(first,row));
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

#[cfg(test)]
mod stable_row_tests {
 use super::*;
 #[test]
 fn retained_ids_reuse_holes_and_reject_capacity_overflow() {
  let mut scene=crate::scene::Scene::empty();let mesh=||crate::prepared::PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap();let a=scene.insert(mesh()).unwrap();let b=scene.insert(mesh()).unwrap();let c=scene.insert(mesh()).unwrap();
  let old=std::collections::HashMap::from([(a,(1,0)),(b,(2,1)),(c,(3,2))]);let rows=stable_rows(&old,&[c,a,b],3).unwrap();assert_eq!((rows[&a],rows[&b],rows[&c]),(0,1,2));
  let d=scene.insert(mesh()).unwrap();let rows=stable_rows(&old,&[c,d,a],3).unwrap();assert_eq!((rows[&a],rows[&d],rows[&c]),(0,1,2));assert!(!rows.contains_key(&b));assert!(stable_rows(&old,&[a,b,c,d],3).is_err());assert!(stable_rows(&old,&[c],2).is_err());assert!(stable_rows(&old,&[],0).unwrap().is_empty());
 }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzu-prepare-shared-record-pick-consumers-02.rs"
```

```text
--8<-- "journey/code/96zzzu-prepare-shared-record-pick-consumers-03.rs"
```

</details>

### 3. `src/triangle_pick.wgsl`

Carry the original owner as a flat unsigned vertex result. Share one pick geometry function between ordinary and slot-selected object records. Use the selected original owner without reading a per-object uniform in the fragment.

<details>
<summary>Locate the existing block</summary>

```wgsl
@group(2) @binding(0) var<uniform> crop:Crop;
struct MeshPickOutput{@builtin(position) position:vec4<f32>,@location(0) world:vec3<f32>,@location(1) @interpolate(flat) flags:u32,@location(2) @interpolate(flat) primitive:u32}
@vertex fn pick_vertex(@builtin(vertex_index) occurrence:u32)->MeshPickOutput{
    let input=mesh_vertex(occurrence,object_settings.range.x);let placed=geometry(input.position,input.colour,input.normal,object_settings);
    return MeshPickOutput(crop.transform*placed.position,placed.world,placed.clip_flags,object_settings.range.y+occurrence/3u+1u);
}
struct PickFragment{@location(0) owner:vec2<u32>,@location(1) primitive:vec2<u32>}
@fragment fn pick_fragment(input:MeshPickOutput)->PickFragment{
    if((input.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(input.flags,input.world)){discard;}
    return PickFragment(vec2<u32>(select(0u,object_settings.selection.y,crop.mode==0u),0u),physical_primitive(input.primitive));
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(2) @binding(0) var<uniform> crop:Crop;
struct MeshPickOutput{@builtin(position) position:vec4<f32>,@location(0) world:vec3<f32>,@location(1) @interpolate(flat) flags:u32,@location(2) @interpolate(flat) primitive:u32,@location(3) @interpolate(flat) owner:u32}
fn pick_geometry(occurrence:u32,object:ObjectSettings,base:u32)->MeshPickOutput {
 let input=mesh_vertex(occurrence,object.range.x);let placed=geometry(input.position,input.colour,input.normal,object);
 return MeshPickOutput(crop.transform*placed.position,placed.world,placed.clip_flags,base+occurrence/3u+1u,object.selection.y);
}
@vertex fn pick_vertex(@builtin(vertex_index) occurrence:u32)->MeshPickOutput {let object=object_at(0u);return pick_geometry(occurrence,object,object.range.y);}
@vertex fn pick_record_vertex(@builtin(vertex_index) occurrence:u32,@location(0) row:u32,@location(1) base:u32)->MeshPickOutput {return pick_geometry(occurrence,object_at(row),base);}
struct PickFragment{@location(0) owner:vec2<u32>,@location(1) primitive:vec2<u32>}
@fragment fn pick_fragment(input:MeshPickOutput)->PickFragment{
    if((input.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(input.flags,input.world)){discard;}
    return PickFragment(vec2<u32>(select(0u,input.owner,crop.mode==0u),0u),physical_primitive(input.primitive));
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzu-prepare-shared-record-pick-consumers-04.rs"
```

```text
--8<-- "journey/code/96zzzu-prepare-shared-record-pick-consumers-05.rs"
```

```text
--8<-- "journey/code/96zzzu-prepare-shared-record-pick-consumers-06.rs"
```

</details>

</details>

<a id="checkpoint-96zzzv-pick-originals-through-shared-records"></a>

### Step 4 of 16 · Pick originals through shared records (`96zzzv-pick-originals-through-shared-records`)

- **Goal:** Connect first surface picks to current shared object records.
- **Data Flow Trace:** `Current shared record and slot → original owner and physical primitive → existing asynchronous readback`
- **Files You Touch:** `src/instance_gpu.rs` (*Share current-record validation and the existing record/slot draw across color and first picks.*) · `src/pick_pipeline.rs` (*Reuse one pick pipeline descriptor for the uniform and shared-record entries.*) · `src/pick_gpu.rs` (*Own the shared pick state beside the existing fallback state.*) · `src/pick_encode.rs` (*Accept shared surface records without altering the other pick passes.*) · `src/renderer.rs` (*Use the renderer’s existing record owner for original first picks.*)

Select the slot-aware pick vertex entry when the object has a current shared record. Bind the same record and slot already used for color drawing. Preserve the earlier uniform pick path for capacity or changed-record fallbacks. Both states use one pipeline descriptor, shader and pick fragment; readback and stale-request cancellation remain unchanged.

> **Self-Check Question:** *Why retain the packed-record match guard for picking?*  
> **Answer:** A mesh may have a newer placement or primitive base than the table snapshot. The guard uses its current uniform until the shared table refreshes, preserving the existing exact picks.

- **What You Should See:** First surface picks consume current shared records while retaining original ownership and stale-request protection.
- **Hands-On Lab Experiment:** Pick both placements at Opacity 0.95 and 1, select and restore them, then reload a different scene while a read is pending. Inspect the unchanged high-bit and relocated-range pick fixtures.

<details>
<summary>Implementation 96zzzv-pick-originals-through-shared-records: Pick originals through shared records</summary>

Connect first surface picks to current shared object records.

Data flow: Current shared record and slot → original owner and physical primitive → existing asynchronous readback.

### 1. `src/instance_gpu.rs`

Share current-record validation and the existing record/slot draw across color and first picks. Use the requested pipeline with the same object row, slot and geometry.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
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
```

</details>

Replace it with:

```rust
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;self.ordinary=ordinary;Ok(())
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {self.draw_with(pass,mesh,&self.ordinary_pipeline)}
    pub fn draw_with(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh,pipeline:&wgpu::RenderPipeline)->bool {
        let Some(&(first,row))=self.ordinary.get(&mesh.id)else{return false};
        if !self.records.matches(row,&mesh.packed()){return false;}
        pass.set_pipeline(pipeline);self.records.bind(pass);self.slots.bind(pass,0,first);mesh.geometry.draw_pulled(pass,1);true
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
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
pub(crate) fn stable_rows(previous:&std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,ids:&[crate::scene::ObjectId],limit:usize)->Result<std::collections::HashMap<crate::scene::ObjectId,u32>,&'static str> {
 let mut rows=std::collections::HashMap::new();let mut used=std::collections::HashSet::new();
 for id in ids {if let Some(&(_,row))=previous.get(id) {if row as usize>=limit||!used.insert(row){return Err("Invalid retained object row");}rows.insert(*id,row);}}
 let mut next=0usize;
 for id in ids {if rows.contains_key(id){continue;}while next<limit&&used.contains(&(next as u32)){next+=1;}if next>=limit||u32::try_from(next).is_err(){return Err("Object records exceed device limits");}let row=next as u32;rows.insert(*id,row);used.insert(row);}
 Ok(rows)
}
pub(crate) fn source()->String {
 crate::clipping_gpu::source(&crate::object_rows::source(include_str!("triangle.wgsl")),2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=crate::object_rows::layout(device);
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
        let own=ordinary_rows.into_iter().collect::<Vec<_>>();let own=if own.len()<=limit-count {own}else{Vec::new()};
        let ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
        let indices=stable_rows(&self.ordinary,&ids,limit)?;
        let mut batches=BTreeMap::new();let mut ordinary=std::collections::HashMap::new();let mut all_slots=Vec::new();let mut all_records=vec![[0;BYTES];indices.values().max().map_or(0,|row|*row as usize+1)];
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
            for (id,record) in &records {
                let row=indices[id];
                all_slots.push([row,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]);
                ordinary.insert(*id,(u32::try_from(all_slots.len()).map_err(|_|"Instance slot range")?,row));
                all_records[row as usize]=*record;
            }
            batches.insert(source,Batch {geometry,count:records.len()as u32,first_slot});
        }
        for mesh in own {
            let first=u32::try_from(all_slots.len()+1).map_err(|_|"Ordinary slot range")?;let row=indices[&mesh.id];
            all_slots.push([row,mesh.primitive_base()]);all_records[row as usize]=mesh.packed();ordinary.insert(mesh.id,(first,row));
        }
        let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
        self.records.set(&ctx,&self.layout,&all_records)?;
        if self.records.written_bytes>0{self.writes+=1;self.written_bytes+=self.records.written_bytes;}
        if all_slots.is_empty()&&self.slots.instances()>0{self.slots.clear(&ctx,true);}else{self.slots.set(&ctx,&all_slots);}
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;self.ordinary=ordinary;Ok(())
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {self.draw_with(pass,mesh,&self.ordinary_pipeline)}
    pub fn draw_with(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh,pipeline:&wgpu::RenderPipeline)->bool {
        let Some(&(first,row))=self.ordinary.get(&mesh.id)else{return false};
        if !self.records.matches(row,&mesh.packed()){return false;}
        pass.set_pipeline(pipeline);self.records.bind(pass);self.slots.bind(pass,0,first);mesh.geometry.draw_pulled(pass,1);true
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

#[cfg(test)]
mod stable_row_tests {
 use super::*;
 #[test]
 fn retained_ids_reuse_holes_and_reject_capacity_overflow() {
  let mut scene=crate::scene::Scene::empty();let mesh=||crate::prepared::PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap();let a=scene.insert(mesh()).unwrap();let b=scene.insert(mesh()).unwrap();let c=scene.insert(mesh()).unwrap();
  let old=std::collections::HashMap::from([(a,(1,0)),(b,(2,1)),(c,(3,2))]);let rows=stable_rows(&old,&[c,a,b],3).unwrap();assert_eq!((rows[&a],rows[&b],rows[&c]),(0,1,2));
  let d=scene.insert(mesh()).unwrap();let rows=stable_rows(&old,&[c,d,a],3).unwrap();assert_eq!((rows[&a],rows[&d],rows[&c]),(0,1,2));assert!(!rows.contains_key(&b));assert!(stable_rows(&old,&[a,b,c,d],3).is_err());assert!(stable_rows(&old,&[c],2).is_err());assert!(stable_rows(&old,&[],0).unwrap().is_empty());
 }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzv-pick-originals-through-shared-records-01.rs"
```

```text
--8<-- "journey/code/96zzzv-pick-originals-through-shared-records-02.rs"
```

</details>

### 2. `src/pick_pipeline.rs`

Reuse one pick pipeline descriptor for the uniform and shared-record entries. Adapt the existing pick shader through the shared record owner. Choose the slot-aware entry for shared records. Read the same eight-byte object-row and primitive-base slot.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn crop_clipping_layout(device:&wgpu::Device)->wgpu::BindGroupLayout{
    let entries=[80u64,448];
    let entries=entries.into_iter().enumerate().map(|(binding,size)|wgpu::BindGroupLayoutEntry{
        binding:binding as u32,visibility:wgpu::ShaderStages::VERTEX_FRAGMENT,
        ty:wgpu::BindingType::Buffer{ty:wgpu::BufferBindingType::Uniform,has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(size)},count:None,
    }).collect::<Vec<_>>();
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("pick crop and clipping"),entries:&entries})
}
pub fn crop_clipping_group(device:&wgpu::Device,crop:&wgpu::Buffer,clipping:&wgpu::Buffer)->wgpu::BindGroup{
    device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("pick crop and clipping"),layout:&crop_clipping_layout(device),entries:&[
        wgpu::BindGroupEntry{binding:0,resource:crop.as_entire_binding()},
        wgpu::BindGroupEntry{binding:1,resource:clipping.as_entire_binding()},
    ]})
}

pub fn mesh(device: &wgpu::Device, view_layout: &wgpu::BindGroupLayout,
    model_layout: &wgpu::BindGroupLayout, crop_layout: &wgpu::BindGroupLayout) -> wgpu::RenderPipeline {
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("mesh IDs"),
            bind_group_layouts: &[Some(view_layout), Some(model_layout), Some(&crop_clipping_layout(device)),Some(&crate::gpu_geometry::GpuGeometry::storage_layout(device))], immediate_size: 0 });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("mesh IDs"),
            source: wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(concat!(include_str!("triangle.wgsl"), "\n", include_str!("pick_crop.wgsl"), "\n", include_str!("triangle_pick.wgsl")),2,1).into()) });
        // An integer attachment stores exact owner IDs; opaque depth rejects hidden faces before any pixel is read back.
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("mesh IDs"), layout: Some(&layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: Some("pick_vertex"), compilation_options: Default::default(),
                buffers: &[]},
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some("pick_fragment"), compilation_options: Default::default(),
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into()),Some(crate::primitive_image::target(true))] }), primitive: Default::default(),
            depth_stencil: Some(crate::depth::surface()), multisample: Default::default(), multiview_mask: None, cache: None,
        })
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn crop_clipping_layout(device:&wgpu::Device)->wgpu::BindGroupLayout{
    let entries=[80u64,448];
    let entries=entries.into_iter().enumerate().map(|(binding,size)|wgpu::BindGroupLayoutEntry{
        binding:binding as u32,visibility:wgpu::ShaderStages::VERTEX_FRAGMENT,
        ty:wgpu::BindingType::Buffer{ty:wgpu::BufferBindingType::Uniform,has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(size)},count:None,
    }).collect::<Vec<_>>();
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("pick crop and clipping"),entries:&entries})
}
pub fn crop_clipping_group(device:&wgpu::Device,crop:&wgpu::Buffer,clipping:&wgpu::Buffer)->wgpu::BindGroup{
    device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("pick crop and clipping"),layout:&crop_clipping_layout(device),entries:&[
        wgpu::BindGroupEntry{binding:0,resource:crop.as_entire_binding()},
        wgpu::BindGroupEntry{binding:1,resource:clipping.as_entire_binding()},
    ]})
}

pub fn mesh(device: &wgpu::Device, view_layout: &wgpu::BindGroupLayout,
    model_layout: &wgpu::BindGroupLayout, crop_layout: &wgpu::BindGroupLayout) -> wgpu::RenderPipeline {
    mesh_with(device,view_layout,model_layout,crop_layout,false)
}
pub fn mesh_with(device:&wgpu::Device,view_layout:&wgpu::BindGroupLayout,model_layout:&wgpu::BindGroupLayout,crop_layout:&wgpu::BindGroupLayout,shared:bool)->wgpu::RenderPipeline {
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("mesh IDs"),
            bind_group_layouts: &[Some(view_layout), Some(model_layout), Some(&crop_clipping_layout(device)),Some(&crate::gpu_geometry::GpuGeometry::storage_layout(device))], immediate_size: 0 });
        let source=concat!(include_str!("triangle.wgsl"), "
", include_str!("pick_crop.wgsl"), "
", include_str!("triangle_pick.wgsl"));let source=if shared {crate::object_rows::source(source)}else{source.into()};
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("mesh IDs"),source:wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(&source,2,1).into())});let slots=[crate::draw_slots::layout()];
        // An integer attachment stores exact owner IDs; opaque depth rejects hidden faces before any pixel is read back.
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("mesh IDs"), layout: Some(&layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: Some(if shared {"pick_record_vertex"}else{"pick_vertex"}), compilation_options: Default::default(),
                buffers: if shared {&slots}else{&[]}},
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some("pick_fragment"), compilation_options: Default::default(),
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into()),Some(crate::primitive_image::target(true))] }), primitive: Default::default(),
            depth_stencil: Some(crate::depth::surface()), multisample: Default::default(), multiview_mask: None, cache: None,
        })
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzv-pick-originals-through-shared-records-03.rs"
```

```text
--8<-- "journey/code/96zzzv-pick-originals-through-shared-records-04.rs"
```

```text
--8<-- "journey/code/96zzzv-pick-originals-through-shared-records-05.rs"
```

```text
--8<-- "journey/code/96zzzv-pick-originals-through-shared-records-06.rs"
```

</details>

### 3. `src/pick_gpu.rs`

Own the shared pick state beside the existing fallback state. Create the shared pick state from the same descriptor and record layout. Retain the shared pick state for the browser lifetime. Keep isolated picker fixtures on the existing uniform path. Pass the existing shared record owner into first picking. Pair the current table with its shared pick state without changing readback ownership.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
        for lane in ink { lane.view(queue, &request.transform, window.full, request.density); }
        if let Some(sheets)=sheets {sheets.view(queue,&request.transform,window.full,request.density);}
        queue.submit([crate::pick_encode::draw(device, image, window, &self.pipeline,
            &self.view_group, &self.crop_group, meshes, ink, faces, request.mode,text,clouds,sheets,&crate::text_gpu_support::GpuCtx {device:device.clone(),queue:queue.clone()},tables,clipping,caps,&self.crop,clipping_buffer)]);
        let ready = Arc::new(AtomicU8::new(0)); self.active = Some(Flight { ready: Arc::clone(&ready), ticket: queued.ticket, view: queued.view }); let alive = Arc::clone(&self.alive);
        // Mapping finishes asynchronously; the callback publishes readiness, and the owner reads the bytes without blocking browser input.
        image.readback.slice(..).map_async(wgpu::MapMode::Read, move |result| {
```

</details>

Replace it with:

```rust
        for lane in ink { lane.view(queue, &request.transform, window.full, request.density); }
        if let Some(sheets)=sheets {sheets.view(queue,&request.transform,window.full,request.density);}
        queue.submit([crate::pick_encode::draw(device, image, window, &self.pipeline,
            &self.view_group, &self.crop_group, meshes, ink, faces, request.mode,text,clouds,sheets,&crate::text_gpu_support::GpuCtx {device:device.clone(),queue:queue.clone()},tables,clipping,caps,&self.crop,clipping_buffer,records.map(|rows|(rows,&self.record_pipeline)))]);
        let ready = Arc::new(AtomicU8::new(0)); self.active = Some(Flight { ready: Arc::clone(&ready), ticket: queued.ticket, view: queued.view }); let alive = Arc::clone(&self.alive);
        // Mapping finishes asynchronously; the callback publishes readiness, and the owner reads the bytes without blocking browser input.
        image.readback.slice(..).map_async(wgpu::MapMode::Read, move |result| {
```

<details>
<summary>Locate this block</summary>

```rust
    }


    pub(crate) fn submit(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, queued: Queued, meshes: &[GpuMesh], ink: &[&Lane], faces: &crate::face_pick::Faces,text:Option<&crate::scene_text_gpu::Lane>,clouds:Option<&crate::cloud_gpu::Lane>,sheets:Option<&crate::sheet_gpu::Lane>) {let clipping=self.legacy_clipping.get_or_insert_with(||crate::clipping_gpu::Clipping::new(device));let group=clipping.group.clone();let buffer=clipping.buffer.clone();self.submit_finite(device,queue,queued,meshes,ink,faces,text,clouds,sheets,None,&group,&buffer,None)}
    pub(crate) fn submit_finite(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, queued: Queued, meshes: &[GpuMesh], ink: &[&Lane], faces: &crate::face_pick::Faces,text:Option<&crate::scene_text_gpu::Lane>,clouds:Option<&crate::cloud_gpu::Lane>,sheets:Option<&crate::sheet_gpu::Lane>,tables:Option<&crate::finite_ink::Tables>,clipping:&wgpu::BindGroup,clipping_buffer:&wgpu::Buffer,caps:Option<&mut crate::caps_gpu::Caps>) {
        assert!(self.active.is_none(), "Only one pick may be in flight");
        let request = queued.request; let window = request.window;
        if self.image.as_ref().is_none_or(|image| image.window.draw[2..] != window.draw[2..] || image.readback.size() != window.bytes()) {
```

</details>

Replace it with:

```rust
    }


    pub(crate) fn submit(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, queued: Queued, meshes: &[GpuMesh], ink: &[&Lane], faces: &crate::face_pick::Faces,text:Option<&crate::scene_text_gpu::Lane>,clouds:Option<&crate::cloud_gpu::Lane>,sheets:Option<&crate::sheet_gpu::Lane>) {let clipping=self.legacy_clipping.get_or_insert_with(||crate::clipping_gpu::Clipping::new(device));let group=clipping.group.clone();let buffer=clipping.buffer.clone();self.submit_finite(device,queue,queued,meshes,ink,faces,text,clouds,sheets,None,&group,&buffer,None,None)}
    pub(crate) fn submit_finite(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, queued: Queued, meshes: &[GpuMesh], ink: &[&Lane], faces: &crate::face_pick::Faces,text:Option<&crate::scene_text_gpu::Lane>,clouds:Option<&crate::cloud_gpu::Lane>,sheets:Option<&crate::sheet_gpu::Lane>,tables:Option<&crate::finite_ink::Tables>,clipping:&wgpu::BindGroup,clipping_buffer:&wgpu::Buffer,caps:Option<&mut crate::caps_gpu::Caps>,records:Option<&crate::instance_gpu::Lane>) {
        assert!(self.active.is_none(), "Only one pick may be in flight");
        let request = queued.request; let window = request.window;
        if self.image.as_ref().is_none_or(|image| image.window.draw[2..] != window.draw[2..] || image.readback.size() != window.bytes()) {
```

<details>
<summary>Locate this block</summary>

```rust
    pub fn new(device: &wgpu::Device, view_layout: &wgpu::BindGroupLayout, model_layout: &wgpu::BindGroupLayout) -> Self {
        let crop_layout = crate::gpu_layout::uniform(device, 80);
        let pipeline = crate::pick_pipeline::mesh(device, view_layout, model_layout, &crop_layout);
        let buffer = |size| device.create_buffer(&wgpu::BufferDescriptor { label: Some("pick view"), size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        let view = buffer(80); let crop = buffer(80);
        let view_group = crate::gpu_layout::group(device, view_layout, &view);
        let crop_group = crate::gpu_layout::group(device, &crop_layout, &crop);
        Self { legacy_clipping:None,pipeline, view, view_group, crop, crop_group, image: None, active: None, pending: None,
            alive: Arc::new(AtomicBool::new(true)), gate: Default::default(), submitted: 0, coalesced: 0 }
    }
```

</details>

Replace it with:

```rust
    pub fn new(device: &wgpu::Device, view_layout: &wgpu::BindGroupLayout, model_layout: &wgpu::BindGroupLayout) -> Self {
        let crop_layout = crate::gpu_layout::uniform(device, 80);
        let pipeline = crate::pick_pipeline::mesh(device, view_layout, model_layout, &crop_layout);
        let record_pipeline=crate::pick_pipeline::mesh_with(device,view_layout,&crate::object_rows::layout(device),&crop_layout,true);
        let buffer = |size| device.create_buffer(&wgpu::BufferDescriptor { label: Some("pick view"), size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        let view = buffer(80); let crop = buffer(80);
        let view_group = crate::gpu_layout::group(device, view_layout, &view);
        let crop_group = crate::gpu_layout::group(device, &crop_layout, &crop);
        Self { legacy_clipping:None,pipeline,record_pipeline, view, view_group, crop, crop_group, image: None, active: None, pending: None,
            alive: Arc::new(AtomicBool::new(true)), gate: Default::default(), submitted: 0, coalesced: 0 }
    }
```

<details>
<summary>Locate this block</summary>

```rust
pub struct Picker {
    legacy_clipping:Option<crate::clipping_gpu::Clipping>,
    pipeline: wgpu::RenderPipeline,
    view: wgpu::Buffer,
    view_group: wgpu::BindGroup,
    crop: wgpu::Buffer,
```

</details>

Replace it with:

```rust
pub struct Picker {
    legacy_clipping:Option<crate::clipping_gpu::Clipping>,
    pipeline: wgpu::RenderPipeline,
    record_pipeline:wgpu::RenderPipeline,
    view: wgpu::Buffer,
    view_group: wgpu::BindGroup,
    crop: wgpu::Buffer,
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{gpu_mesh::GpuMesh, pick_window::Window, stroke_gpu::Lane};
use std::sync::{Arc, atomic::{AtomicBool, AtomicU8, Ordering}};

#[derive(Clone, Copy)]
pub struct Request { pub window: Window, pub transform: [f32; 16], pub density: f32, pub mode: crate::pick_sources::Mode }

pub(crate) struct Queued { request: Request, ticket: crate::pick_gate::Ticket, view: crate::pick_view::View }
struct Flight { ready: Arc<AtomicU8>, ticket: crate::pick_gate::Ticket, view: crate::pick_view::View }
pub enum Reply { Hit(Option<[u32; 2]>), Stale }


pub struct Picker {
    legacy_clipping:Option<crate::clipping_gpu::Clipping>,
    pipeline: wgpu::RenderPipeline,
    record_pipeline:wgpu::RenderPipeline,
    view: wgpu::Buffer,
    view_group: wgpu::BindGroup,
    crop: wgpu::Buffer,
    crop_group: wgpu::BindGroup,
    image: Option<crate::pick_image::Image>,
    active: Option<Flight>,
    pending: Option<Queued>,
    alive: Arc<AtomicBool>,
    gate: crate::pick_gate::Gate,
    pub submitted: u64,
    pub coalesced: u64,
}

impl Drop for Picker { fn drop(&mut self) { self.alive.store(false, Ordering::Release); self.gate.close(); } }

impl Picker {
    pub fn new(device: &wgpu::Device, view_layout: &wgpu::BindGroupLayout, model_layout: &wgpu::BindGroupLayout) -> Self {
        let crop_layout = crate::gpu_layout::uniform(device, 80);
        let pipeline = crate::pick_pipeline::mesh(device, view_layout, model_layout, &crop_layout);
        let record_pipeline=crate::pick_pipeline::mesh_with(device,view_layout,&crate::object_rows::layout(device),&crop_layout,true);
        let buffer = |size| device.create_buffer(&wgpu::BufferDescriptor { label: Some("pick view"), size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        let view = buffer(80); let crop = buffer(80);
        let view_group = crate::gpu_layout::group(device, view_layout, &view);
        let crop_group = crate::gpu_layout::group(device, &crop_layout, &crop);
        Self { legacy_clipping:None,pipeline,record_pipeline, view, view_group, crop, crop_group, image: None, active: None, pending: None,
            alive: Arc::new(AtomicBool::new(true)), gate: Default::default(), submitted: 0, coalesced: 0 }
    }

    pub fn queue(&mut self, request: Request) -> Result<(), &'static str> {
        let view = crate::pick_view::View::new(request.transform, request.window.full, request.density)?;
        let ticket = self.gate.issue()?;
        if self.pending.replace(Queued { request, ticket, view }).is_some() { self.coalesced += 1; }
        Ok(())
    }

    pub fn queued(&self) -> bool { self.pending.is_some() }

    pub fn cancel(&mut self) {
        self.gate.invalidate(); self.pending = None;
        let mapped = self.active.take().is_some_and(|flight| flight.ready.load(Ordering::Acquire) <= 1);
        if let Some(image) = self.image.take() { if mapped { image.readback.unmap(); } }
    }

    pub(crate) fn next(&mut self, view: crate::pick_view::View) -> Option<Queued> {
        if self.active.is_some() { return None; }
        let queued = self.pending.take()?;
        if queued.view == view { Some(queued) } else { self.gate.invalidate(); None }
    }


    pub(crate) fn submit(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, queued: Queued, meshes: &[GpuMesh], ink: &[&Lane], faces: &crate::face_pick::Faces,text:Option<&crate::scene_text_gpu::Lane>,clouds:Option<&crate::cloud_gpu::Lane>,sheets:Option<&crate::sheet_gpu::Lane>) {let clipping=self.legacy_clipping.get_or_insert_with(||crate::clipping_gpu::Clipping::new(device));let group=clipping.group.clone();let buffer=clipping.buffer.clone();self.submit_finite(device,queue,queued,meshes,ink,faces,text,clouds,sheets,None,&group,&buffer,None,None)}
    pub(crate) fn submit_finite(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, queued: Queued, meshes: &[GpuMesh], ink: &[&Lane], faces: &crate::face_pick::Faces,text:Option<&crate::scene_text_gpu::Lane>,clouds:Option<&crate::cloud_gpu::Lane>,sheets:Option<&crate::sheet_gpu::Lane>,tables:Option<&crate::finite_ink::Tables>,clipping:&wgpu::BindGroup,clipping_buffer:&wgpu::Buffer,caps:Option<&mut crate::caps_gpu::Caps>,records:Option<&crate::instance_gpu::Lane>) {
        assert!(self.active.is_none(), "Only one pick may be in flight");
        let request = queued.request; let window = request.window;
        if self.image.as_ref().is_none_or(|image| image.window.draw[2..] != window.draw[2..] || image.readback.size() != window.bytes()) {
            self.image = Some(crate::pick_image::Image::new(device, window));
        }
        let image = self.image.as_mut().unwrap(); image.window = window;
        let mut bytes: Vec<u8> = request.transform.iter().flat_map(|n| n.to_ne_bytes()).collect(); bytes.resize(80, 0);
        queue.write_buffer(&self.view, 0, &bytes);
        let mut bytes: Vec<u8> = window.crop().iter().flat_map(|n| n.to_ne_bytes()).collect();
        bytes.extend((request.mode as u32).to_ne_bytes());bytes.extend(window.draw[0].to_ne_bytes());bytes.extend(window.draw[1].to_ne_bytes());bytes.resize(80, 0); queue.write_buffer(&self.crop, 0, &bytes);
        for lane in ink { lane.view(queue, &request.transform, window.full, request.density); }
        if let Some(sheets)=sheets {sheets.view(queue,&request.transform,window.full,request.density);}
        queue.submit([crate::pick_encode::draw(device, image, window, &self.pipeline,
            &self.view_group, &self.crop_group, meshes, ink, faces, request.mode,text,clouds,sheets,&crate::text_gpu_support::GpuCtx {device:device.clone(),queue:queue.clone()},tables,clipping,caps,&self.crop,clipping_buffer,records.map(|rows|(rows,&self.record_pipeline)))]);
        let ready = Arc::new(AtomicU8::new(0)); self.active = Some(Flight { ready: Arc::clone(&ready), ticket: queued.ticket, view: queued.view }); let alive = Arc::clone(&self.alive);
        // Mapping finishes asynchronously; the callback publishes readiness, and the owner reads the bytes without blocking browser input.
        image.readback.slice(..).map_async(wgpu::MapMode::Read, move |result| {
            ready.store(if result.is_ok() { 1 } else { 2 }, Ordering::Release);
            if alive.load(Ordering::Acquire) {
                #[cfg(target_arch = "wasm32")] crate::browser_pick::wake();
            }
        });
        self.submitted += 1;
    }

    pub fn take(&mut self, view: crate::pick_view::View) -> Option<Result<Reply, &'static str>> {
        let status = self.active.as_ref()?.ready.load(Ordering::Acquire);
        if status == 0 { return None; }
        let flight = self.active.take().unwrap();
        let current = self.gate.accept(flight.ticket) && flight.view == view;
        let image = self.image.as_ref().unwrap();
        if status != 1 { return Some(if current { Err("Cannot read GPU pick image") } else { Ok(Reply::Stale) }); }
        let bytes = image.readback.slice(..).get_mapped_range();
        let result = if current { image.window.closest(&bytes).map(Reply::Hit) } else { Ok(Reply::Stale) };
        drop(bytes); image.readback.unmap(); Some(result)
    }

    #[cfg(all(test,not(target_arch="wasm32")))]
    pub(crate) fn primitive_for_tests(&self)->Option<(wgpu::Texture,Window)>{
        self.image.as_ref().map(|image|(image.primitive.texture.clone(),image.window))
    }

    pub fn usage(&self) -> [u64; 3] {
        [u64::from(self.active.is_some()), u64::from(self.pending.is_some()), self.image.as_ref().map_or(0, |image|
            image.readback.size() + u64::from(image.window.draw[2]) * u64::from(image.window.draw[3]) * 16) + 160+u64::from(self.legacy_clipping.is_some())*448]
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzv-pick-originals-through-shared-records-07.rs"
```

```text
--8<-- "journey/code/96zzzv-pick-originals-through-shared-records-08.rs"
```

```text
--8<-- "journey/code/96zzzv-pick-originals-through-shared-records-09.rs"
```

```text
--8<-- "journey/code/96zzzv-pick-originals-through-shared-records-10.rs"
```

```text
--8<-- "journey/code/96zzzv-pick-originals-through-shared-records-11.rs"
```

```text
--8<-- "journey/code/96zzzv-pick-originals-through-shared-records-12.rs"
```

</details>

### 4. `src/pick_encode.rs`

Accept shared surface records without altering the other pick passes. Draw through current records or the earlier exact uniform fallback.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{gpu_mesh::GpuMesh, pick_image::Image, pick_window::Window, stroke_gpu::Lane};

pub(crate) fn draw(device: &wgpu::Device, image: &Image, window: Window, pipeline: &wgpu::RenderPipeline,
    view_group: &wgpu::BindGroup, crop_group: &wgpu::BindGroup, meshes: &[GpuMesh], ink: &[&Lane], faces: &crate::face_pick::Faces, mode: crate::pick_sources::Mode,text:Option<&crate::scene_text_gpu::Lane>,clouds:Option<&crate::cloud_gpu::Lane>,sheets:Option<&crate::sheet_gpu::Lane>,ctx:&crate::text_gpu_support::GpuCtx,tables:Option<&crate::finite_ink::Tables>,clipping:&wgpu::BindGroup,caps:Option<&mut crate::caps_gpu::Caps>,crop:&wgpu::Buffer,clipping_buffer:&wgpu::Buffer) -> wgpu::CommandBuffer {
        let mut encoder = device.create_command_encoder(&Default::default());
        let mesh_crop=crate::pick_pipeline::crop_clipping_group(device,crop,clipping_buffer);
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { label: Some("IDs"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: &image.colour, depth_slice: None, resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store } }),Some(wgpu::RenderPassColorAttachment{view:&image.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &image.depth,
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }), stencil_ops: None }), ..Default::default() });
            pass.set_pipeline(pipeline); pass.set_bind_group(0, view_group, &[]); pass.set_bind_group(2, &mesh_crop, &[]);
            for mesh in meshes { mesh.draw_pulled(&mut pass); }

        }
        if mode==crate::pick_sources::Mode::Face{
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original source-face IDs"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:None,stencil_ops:None}),..Default::default()});
            faces.draw(&mut pass,meshes,view_group,crop_group);
        }
        if let Some(caps)=caps{caps.encode_pick(device,&ctx.queue,&mut encoder,image,window,view_group,crop,clipping,meshes,mode);}
        let tables=tables.map(|tables|crate::finite_ink::Tables{projected:tables.projected.clone(),tiles:tables.tiles.clone(),primitive:image.primitive.view.clone()});
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite integer ink"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:None,stencil_ops:None}),..Default::default()});
            pass.set_bind_group(3,clipping,&[]);
            for lane in ink {lane.pick_finite(device,&mut pass,crop_group,tables.as_ref(),&image.depth);}
        }
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("other original integer owners"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});
            if let Some(clouds)=clouds {clouds.pick(ctx,&mut pass,window,mode);}
            if let Some(sheets)=sheets {sheets.pick(&mut pass,crop_group,clipping);}
            if let Some(text)=text {text.draw_ids(&mut pass,crop_group);}
        }
        // The surrounding halo supports rasterization; only the smaller cursor window is copied back for hit testing.
        encoder.copy_texture_to_buffer(wgpu::TexelCopyTextureInfo { texture: &image.texture, mip_level: 0,
            origin: wgpu::Origin3d { x: window.copy[0] - window.draw[0], y: window.copy[1] - window.draw[1], z: 0 }, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: &image.readback, layout: wgpu::TexelCopyBufferLayout { offset: 0,
                bytes_per_row: Some(window.row_bytes()), rows_per_image: Some(window.copy[3]) } },
            wgpu::Extent3d { width: window.copy[2], height: window.copy[3], depth_or_array_layers: 1 });
        encoder.finish()
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{gpu_mesh::GpuMesh, pick_image::Image, pick_window::Window, stroke_gpu::Lane};

pub(crate) fn draw(device: &wgpu::Device, image: &Image, window: Window, pipeline: &wgpu::RenderPipeline,
    view_group: &wgpu::BindGroup, crop_group: &wgpu::BindGroup, meshes: &[GpuMesh], ink: &[&Lane], faces: &crate::face_pick::Faces, mode: crate::pick_sources::Mode,text:Option<&crate::scene_text_gpu::Lane>,clouds:Option<&crate::cloud_gpu::Lane>,sheets:Option<&crate::sheet_gpu::Lane>,ctx:&crate::text_gpu_support::GpuCtx,tables:Option<&crate::finite_ink::Tables>,clipping:&wgpu::BindGroup,caps:Option<&mut crate::caps_gpu::Caps>,crop:&wgpu::Buffer,clipping_buffer:&wgpu::Buffer,records:Option<(&crate::instance_gpu::Lane,&wgpu::RenderPipeline)>) -> wgpu::CommandBuffer {
        let mut encoder = device.create_command_encoder(&Default::default());
        let mesh_crop=crate::pick_pipeline::crop_clipping_group(device,crop,clipping_buffer);
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { label: Some("IDs"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: &image.colour, depth_slice: None, resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store } }),Some(wgpu::RenderPassColorAttachment{view:&image.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &image.depth,
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }), stencil_ops: None }), ..Default::default() });
            pass.set_pipeline(pipeline); pass.set_bind_group(0, view_group, &[]); pass.set_bind_group(2, &mesh_crop, &[]);
            for mesh in meshes {if !records.is_some_and(|(rows,shared)|rows.draw_with(&mut pass,mesh,shared)){pass.set_pipeline(pipeline);mesh.draw_pulled(&mut pass);}}

        }
        if mode==crate::pick_sources::Mode::Face{
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original source-face IDs"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:None,stencil_ops:None}),..Default::default()});
            faces.draw(&mut pass,meshes,view_group,crop_group);
        }
        if let Some(caps)=caps{caps.encode_pick(device,&ctx.queue,&mut encoder,image,window,view_group,crop,clipping,meshes,mode);}
        let tables=tables.map(|tables|crate::finite_ink::Tables{projected:tables.projected.clone(),tiles:tables.tiles.clone(),primitive:image.primitive.view.clone()});
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite integer ink"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:None,stencil_ops:None}),..Default::default()});
            pass.set_bind_group(3,clipping,&[]);
            for lane in ink {lane.pick_finite(device,&mut pass,crop_group,tables.as_ref(),&image.depth);}
        }
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("other original integer owners"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});
            if let Some(clouds)=clouds {clouds.pick(ctx,&mut pass,window,mode);}
            if let Some(sheets)=sheets {sheets.pick(&mut pass,crop_group,clipping);}
            if let Some(text)=text {text.draw_ids(&mut pass,crop_group);}
        }
        // The surrounding halo supports rasterization; only the smaller cursor window is copied back for hit testing.
        encoder.copy_texture_to_buffer(wgpu::TexelCopyTextureInfo { texture: &image.texture, mip_level: 0,
            origin: wgpu::Origin3d { x: window.copy[0] - window.draw[0], y: window.copy[1] - window.draw[1], z: 0 }, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: &image.readback, layout: wgpu::TexelCopyBufferLayout { offset: 0,
                bytes_per_row: Some(window.row_bytes()), rows_per_image: Some(window.copy[3]) } },
            wgpu::Extent3d { width: window.copy[2], height: window.copy[3], depth_or_array_layers: 1 });
        encoder.finish()
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzv-pick-originals-through-shared-records-13.rs"
```

```text
--8<-- "journey/code/96zzzv-pick-originals-through-shared-records-14.rs"
```

</details>

### 5. `src/renderer.rs`

Use the renderer’s existing record owner for original first picks.

<details>
<summary>Locate the existing block</summary>

```rust
            &self.meshes, &[&self.mesh_ink, &self.strokes, &self.paths, &self.markers], &self.faces,self.text.borrow().as_ref(),self.clouds.borrow().as_ref(),Some(&self.sheets),Some(&tables),&clipping.group,&clipping.buffer,Some(&mut self.caps.borrow_mut())); }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
            &self.meshes, &[&self.mesh_ink, &self.strokes, &self.paths, &self.markers], &self.faces,self.text.borrow().as_ref(),self.clouds.borrow().as_ref(),Some(&self.sheets),Some(&tables),&clipping.group,&clipping.buffer,Some(&mut self.caps.borrow_mut()),Some(&self.instances)); }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzv-pick-originals-through-shared-records-15.rs"
```

</details>

</details>

<a id="checkpoint-96zzzw-read-source-faces-through-shared-records"></a>

### Step 5 of 16 · Read source faces through shared records (`96zzzw-read-source-faces-through-shared-records`)

- **Goal:** Connect original source-face picking to the same current object records.
- **Data Flow Trace:** `Current object record → original source face plus original owner → existing face readback`
- **Files You Touch:** `src/instance_gpu.rs` (*Reuse current-record validation and slot binding for consumers with different geometry layouts.*) · `src/face_pick.wgsl` (*Carry original ownership into the source-face fragment as an unsigned value.*) · `src/face_pick.rs` (*Keep a shared-record source-face state beside the existing fallback.*) · `src/pick_encode.rs` (*Share the original object table with both first and source-face pick passes.*)

A source face needs its original face metadata as well as placement and owner. Bind the existing shared object row and slot, then retain the source-face buffers and crop group. Separate record binding from geometry drawing so each consumer can keep its own geometry layout. Both source-face pipeline states share one descriptor and shader; their owner travels through the same unsigned vertex result.

> **Self-Check Question:** *Why separate binding an object record from drawing its geometry?*  
> **Answer:** The source-face pass has its own face metadata and crop groups. It can reuse the current object row and slot while preserving those consumer-specific bindings.

- **What You Should See:** Original source-face reads consume current shared object records with exact owner and face identity.
- **Hands-On Lab Experiment:** Select Face on ordinary and copied surfaces. Move and restore a copied original, then repeat the face pick. Inspect the unchanged relocated-range and original-hole face fixtures.

<details>
<summary>Implementation 96zzzw-read-source-faces-through-shared-records: Read source faces through shared records</summary>

Connect original source-face picking to the same current object records.

Data flow: Current object record → original source face plus original owner → existing face readback.

### 1. `src/instance_gpu.rs`

Reuse current-record validation and slot binding for consumers with different geometry layouts. Bind only the shared object record and slot; let each consumer draw its own geometry.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {self.draw_with(pass,mesh,&self.ordinary_pipeline)}
    pub fn draw_with(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh,pipeline:&wgpu::RenderPipeline)->bool {
        let Some(&(first,row))=self.ordinary.get(&mesh.id)else{return false};
        if !self.records.matches(row,&mesh.packed()){return false;}
        pass.set_pipeline(pipeline);self.records.bind(pass);self.slots.bind(pass,0,first);mesh.geometry.draw_pulled(pass,1);true
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
```

</details>

Replace it with:

```rust
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {self.draw_with(pass,mesh,&self.ordinary_pipeline)}
    pub fn draw_with(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh,pipeline:&wgpu::RenderPipeline)->bool {
        if !self.bind_record(pass,mesh){return false;}pass.set_pipeline(pipeline);mesh.geometry.draw_pulled(pass,1);true
    }
    pub fn bind_record(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {
        let Some(&(first,row))=self.ordinary.get(&mesh.id)else{return false};
        if !self.records.matches(row,&mesh.packed()){return false;}
        self.records.bind(pass);self.slots.bind(pass,0,first);true
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
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
pub(crate) fn stable_rows(previous:&std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,ids:&[crate::scene::ObjectId],limit:usize)->Result<std::collections::HashMap<crate::scene::ObjectId,u32>,&'static str> {
 let mut rows=std::collections::HashMap::new();let mut used=std::collections::HashSet::new();
 for id in ids {if let Some(&(_,row))=previous.get(id) {if row as usize>=limit||!used.insert(row){return Err("Invalid retained object row");}rows.insert(*id,row);}}
 let mut next=0usize;
 for id in ids {if rows.contains_key(id){continue;}while next<limit&&used.contains(&(next as u32)){next+=1;}if next>=limit||u32::try_from(next).is_err(){return Err("Object records exceed device limits");}let row=next as u32;rows.insert(*id,row);used.insert(row);}
 Ok(rows)
}
pub(crate) fn source()->String {
 crate::clipping_gpu::source(&crate::object_rows::source(include_str!("triangle.wgsl")),2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=crate::object_rows::layout(device);
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
        let own=ordinary_rows.into_iter().collect::<Vec<_>>();let own=if own.len()<=limit-count {own}else{Vec::new()};
        let ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
        let indices=stable_rows(&self.ordinary,&ids,limit)?;
        let mut batches=BTreeMap::new();let mut ordinary=std::collections::HashMap::new();let mut all_slots=Vec::new();let mut all_records=vec![[0;BYTES];indices.values().max().map_or(0,|row|*row as usize+1)];
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
            for (id,record) in &records {
                let row=indices[id];
                all_slots.push([row,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]);
                ordinary.insert(*id,(u32::try_from(all_slots.len()).map_err(|_|"Instance slot range")?,row));
                all_records[row as usize]=*record;
            }
            batches.insert(source,Batch {geometry,count:records.len()as u32,first_slot});
        }
        for mesh in own {
            let first=u32::try_from(all_slots.len()+1).map_err(|_|"Ordinary slot range")?;let row=indices[&mesh.id];
            all_slots.push([row,mesh.primitive_base()]);all_records[row as usize]=mesh.packed();ordinary.insert(mesh.id,(first,row));
        }
        let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
        self.records.set(&ctx,&self.layout,&all_records)?;
        if self.records.written_bytes>0{self.writes+=1;self.written_bytes+=self.records.written_bytes;}
        if all_slots.is_empty()&&self.slots.instances()>0{self.slots.clear(&ctx,true);}else{self.slots.set(&ctx,&all_slots);}
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;self.ordinary=ordinary;Ok(())
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {self.draw_with(pass,mesh,&self.ordinary_pipeline)}
    pub fn draw_with(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh,pipeline:&wgpu::RenderPipeline)->bool {
        if !self.bind_record(pass,mesh){return false;}pass.set_pipeline(pipeline);mesh.geometry.draw_pulled(pass,1);true
    }
    pub fn bind_record(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {
        let Some(&(first,row))=self.ordinary.get(&mesh.id)else{return false};
        if !self.records.matches(row,&mesh.packed()){return false;}
        self.records.bind(pass);self.slots.bind(pass,0,first);true
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

#[cfg(test)]
mod stable_row_tests {
 use super::*;
 #[test]
 fn retained_ids_reuse_holes_and_reject_capacity_overflow() {
  let mut scene=crate::scene::Scene::empty();let mesh=||crate::prepared::PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap();let a=scene.insert(mesh()).unwrap();let b=scene.insert(mesh()).unwrap();let c=scene.insert(mesh()).unwrap();
  let old=std::collections::HashMap::from([(a,(1,0)),(b,(2,1)),(c,(3,2))]);let rows=stable_rows(&old,&[c,a,b],3).unwrap();assert_eq!((rows[&a],rows[&b],rows[&c]),(0,1,2));
  let d=scene.insert(mesh()).unwrap();let rows=stable_rows(&old,&[c,d,a],3).unwrap();assert_eq!((rows[&a],rows[&d],rows[&c]),(0,1,2));assert!(!rows.contains_key(&b));assert!(stable_rows(&old,&[a,b,c,d],3).is_err());assert!(stable_rows(&old,&[c],2).is_err());assert!(stable_rows(&old,&[],0).unwrap().is_empty());
 }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-01.rs"
```

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-02.rs"
```

</details>

### 2. `src/face_pick.wgsl`

Carry original ownership into the source-face fragment as an unsigned value. Share one source-face geometry function between ordinary and slot-selected records. Read the original owner selected by the source-face vertex.

<details>
<summary>Locate the existing block</summary>

```wgsl
struct FaceSettings { base: u32, _a: u32, _b: u32, _c: u32 }
@group(2) @binding(0) var<storage, read> positions: array<PackedVertex>;
@group(2) @binding(1) var<storage, read> indices: array<u32>;
@group(2) @binding(2) var<storage, read> faces: array<u32>;
@group(2) @binding(3) var<uniform> source: FaceSettings;
@group(3) @binding(0) var<uniform> crop: Crop;
struct FaceOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) @interpolate(flat) face: u32,
    @location(1) world:vec3<f32>,@location(2) @interpolate(flat) flags:u32,
}
@vertex
fn face_vertex(@builtin(vertex_index) index: u32) -> FaceOutput {
    let vertex = positions[indices[index+object_settings.range.x]];
    let placed = geometry(packed_position(vertex), unpack4x8unorm(vertex.color), vec3<f32>(0.0),object_settings);
    return FaceOutput(crop.transform * placed.position, faces[index / 3u] + source.base,placed.world,placed.clip_flags);
}
@fragment
fn face_fragment(input: FaceOutput) -> @location(0) vec2<u32> {
    if((input.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(input.flags,input.world)){discard;}
    return vec2<u32>(object_settings.selection.y, (0x20000000u | input.face) + 1u);
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct FaceSettings { base: u32, _a: u32, _b: u32, _c: u32 }
@group(2) @binding(0) var<storage, read> positions: array<PackedVertex>;
@group(2) @binding(1) var<storage, read> indices: array<u32>;
@group(2) @binding(2) var<storage, read> faces: array<u32>;
@group(2) @binding(3) var<uniform> source: FaceSettings;
@group(3) @binding(0) var<uniform> crop: Crop;
struct FaceOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) @interpolate(flat) face: u32,
    @location(1) world:vec3<f32>,@location(2) @interpolate(flat) flags:u32,@location(3) @interpolate(flat) owner:u32,
}
fn face_geometry(index:u32,object:ObjectSettings)->FaceOutput {
 let vertex=positions[indices[index+object.range.x]];
 let placed=geometry(packed_position(vertex),unpack4x8unorm(vertex.color),vec3<f32>(0.),object);
 return FaceOutput(crop.transform*placed.position,faces[index/3u]+source.base,placed.world,placed.clip_flags,object.selection.y);
}
@vertex fn face_vertex(@builtin(vertex_index) index:u32)->FaceOutput {return face_geometry(index,object_at(0u));}
@vertex fn face_record_vertex(@builtin(vertex_index) index:u32,@location(0) row:u32)->FaceOutput {return face_geometry(index,object_at(row));}
@fragment
fn face_fragment(input: FaceOutput) -> @location(0) vec2<u32> {
    if((input.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(input.flags,input.world)){discard;}
    return vec2<u32>(input.owner, (0x20000000u | input.face) + 1u);
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-03.rs"
```

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-04.rs"
```

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-05.rs"
```

</details>

### 3. `src/face_pick.rs`

Keep a shared-record source-face state beside the existing fallback. Share the source-face pipeline descriptor across both record resources. Use the shared storage contract or the current uniform contract. Adapt the same source-face shader through the existing record owner. Preserve source-face clipping after shader adaptation. Select the source-face record entry and existing draw-slot layout. Build both source-face states from one descriptor and shader. Accept the existing renderer record owner for original source-face reads. Use current records or the exact uniform fallback, retaining original face metadata and crop bindings.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::gpu_mesh::GpuMesh;
use wgpu::util::DeviceExt;

struct Row { group: wgpu::BindGroup, bytes: u64 }

pub struct Faces { pipeline: wgpu::RenderPipeline, rows: Vec<Option<Row>> }

fn layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let mut entries = Vec::new();
    for binding in 0..4 { entries.push(wgpu::BindGroupLayoutEntry { binding, visibility: wgpu::ShaderStages::VERTEX,
        ty: wgpu::BindingType::Buffer { ty: if binding == 3 { wgpu::BufferBindingType::Uniform }
            else { wgpu::BufferBindingType::Storage { read_only: true } }, has_dynamic_offset: false, min_binding_size: None }, count: None }); }
    entries.push(crate::tile_pipelines::entry(4,wgpu::ShaderStages::FRAGMENT,wgpu::BufferBindingType::Uniform));
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("source face metadata"), entries: &entries })
}

impl Faces {
    pub fn new(device: &wgpu::Device) -> Self {
        let storage = layout(device); let view = crate::gpu_layout::uniform(device, 80);
        let model = crate::gpu_layout::uniform(device, crate::object_record::BYTES as u64); let crop = crate::gpu_layout::uniform(device, 80);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("source face IDs"),
            bind_group_layouts: &[Some(&view), Some(&model), Some(&storage), Some(&crop)], immediate_size: 0 });
        let source = concat!(include_str!("triangle.wgsl"), "\n", include_str!("pick_crop.wgsl"), "\n", include_str!("face_pick.wgsl"));
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("source faces"), source: wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(source,2,4).into()) });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor { label: Some("source face IDs"), layout: Some(&layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: Some("face_vertex"), buffers: &[], compilation_options: Default::default() },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some("face_fragment"), compilation_options: Default::default(),
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into())] }), primitive: Default::default(), depth_stencil: Some(crate::depth::ink()),
            multisample: Default::default(), multiview_mask: None, cache: None });
        Self { pipeline, rows: Vec::new() }
    }

    pub fn set(&mut self, device: &wgpu::Device, meshes: &[GpuMesh], bases: &[(crate::scene::ObjectId, u32)],clipping:&wgpu::Buffer) {
        self.rows.clear();
        for (mesh, &(owner, base)) in meshes.iter().zip(bases) {
            assert_eq!(mesh.id, owner); let geometry = &mesh.geometry;
            if geometry.source.parts.triangles.is_empty() { self.rows.push(None); continue; }
            // Vertex pulling reads the existing index and position buffers; one small per-triangle stream adds original face identity.
            let bytes: Vec<u8> = geometry.source.parts.triangles.iter().flat_map(|n| n.to_ne_bytes()).collect();
            let faces = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("source triangle faces"), contents: &bytes, usage: wgpu::BufferUsages::STORAGE });
            let data: Vec<u8> = [base, 0, 0, 0].into_iter().flat_map(u32::to_ne_bytes).collect();
            let settings = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("source face base"), contents: &data, usage: wgpu::BufferUsages::UNIFORM });
            let(vertices,indices)=geometry.buffers();
            let buffers = [&vertices, &indices, &faces, &settings,clipping];
            let entries: Vec<_> = buffers.iter().enumerate().map(|(binding, buffer)| wgpu::BindGroupEntry { binding: binding as u32, resource: buffer.as_entire_binding() }).collect();
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("source face metadata"), layout: &self.pipeline.get_bind_group_layout(2), entries: &entries });
            self.rows.push(Some(Row { group, bytes: faces.size() + settings.size() }));
        }
        assert_eq!(meshes.len(), bases.len());
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>, meshes: &[GpuMesh], view: &wgpu::BindGroup, crop: &wgpu::BindGroup) {
        pass.set_pipeline(&self.pipeline); pass.set_bind_group(0, view, &[]); pass.set_bind_group(3, crop, &[]);
        for (mesh, row) in meshes.iter().zip(&self.rows) { if let Some(row) = row {
            mesh.bind_model(pass); pass.set_bind_group(2, &row.group, &[]); pass.draw(0..mesh.geometry.index_count, 0..1);
        } }
    }

    pub fn clear(&mut self) { self.rows.clear(); }
    pub fn ready(&self, count: usize) -> bool { self.rows.len() == count }

    pub fn bytes(&self) -> u64 { self.rows.iter().flatten().map(|row| row.bytes).sum() }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::gpu_mesh::GpuMesh;
use wgpu::util::DeviceExt;

struct Row { group: wgpu::BindGroup, bytes: u64 }

pub struct Faces { pipeline: wgpu::RenderPipeline,record_pipeline:wgpu::RenderPipeline, rows: Vec<Option<Row>> }

fn layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let mut entries = Vec::new();
    for binding in 0..4 { entries.push(wgpu::BindGroupLayoutEntry { binding, visibility: wgpu::ShaderStages::VERTEX,
        ty: wgpu::BindingType::Buffer { ty: if binding == 3 { wgpu::BufferBindingType::Uniform }
            else { wgpu::BufferBindingType::Storage { read_only: true } }, has_dynamic_offset: false, min_binding_size: None }, count: None }); }
    entries.push(crate::tile_pipelines::entry(4,wgpu::ShaderStages::FRAGMENT,wgpu::BufferBindingType::Uniform));
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("source face metadata"), entries: &entries })
}

impl Faces {
    pub fn new(device: &wgpu::Device) -> Self {
        let create=|shared| {let storage = layout(device); let view = crate::gpu_layout::uniform(device, 80);
        let model=if shared {crate::object_rows::layout(device)}else{crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64)}; let crop = crate::gpu_layout::uniform(device, 80);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("source face IDs"),
            bind_group_layouts: &[Some(&view), Some(&model), Some(&storage), Some(&crop)], immediate_size: 0 });
        let source = concat!(include_str!("triangle.wgsl"), "\n", include_str!("pick_crop.wgsl"), "\n", include_str!("face_pick.wgsl"));
        let source=if shared {crate::object_rows::source(source)}else{source.into()};let slots=[crate::draw_slots::layout()];
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("source faces"), source: wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(&source,2,4).into()) });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor { label: Some("source face IDs"), layout: Some(&layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: Some(if shared {"face_record_vertex"}else{"face_vertex"}), buffers: if shared {&slots}else{&[]}, compilation_options: Default::default() },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some("face_fragment"), compilation_options: Default::default(),
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into())] }), primitive: Default::default(), depth_stencil: Some(crate::depth::ink()),
            multisample: Default::default(), multiview_mask: None, cache: None });
        pipeline};let pipeline=create(false);let record_pipeline=create(true);Self {pipeline,record_pipeline,rows:Vec::new()}
    }

    pub fn set(&mut self, device: &wgpu::Device, meshes: &[GpuMesh], bases: &[(crate::scene::ObjectId, u32)],clipping:&wgpu::Buffer) {
        self.rows.clear();
        for (mesh, &(owner, base)) in meshes.iter().zip(bases) {
            assert_eq!(mesh.id, owner); let geometry = &mesh.geometry;
            if geometry.source.parts.triangles.is_empty() { self.rows.push(None); continue; }
            // Vertex pulling reads the existing index and position buffers; one small per-triangle stream adds original face identity.
            let bytes: Vec<u8> = geometry.source.parts.triangles.iter().flat_map(|n| n.to_ne_bytes()).collect();
            let faces = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("source triangle faces"), contents: &bytes, usage: wgpu::BufferUsages::STORAGE });
            let data: Vec<u8> = [base, 0, 0, 0].into_iter().flat_map(u32::to_ne_bytes).collect();
            let settings = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("source face base"), contents: &data, usage: wgpu::BufferUsages::UNIFORM });
            let(vertices,indices)=geometry.buffers();
            let buffers = [&vertices, &indices, &faces, &settings,clipping];
            let entries: Vec<_> = buffers.iter().enumerate().map(|(binding, buffer)| wgpu::BindGroupEntry { binding: binding as u32, resource: buffer.as_entire_binding() }).collect();
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("source face metadata"), layout: &self.pipeline.get_bind_group_layout(2), entries: &entries });
            self.rows.push(Some(Row { group, bytes: faces.size() + settings.size() }));
        }
        assert_eq!(meshes.len(), bases.len());
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>, meshes: &[GpuMesh], view: &wgpu::BindGroup, crop: &wgpu::BindGroup,records:Option<&crate::instance_gpu::Lane>) {
        pass.set_pipeline(&self.pipeline); pass.set_bind_group(0, view, &[]); pass.set_bind_group(3, crop, &[]);
        for (mesh, row) in meshes.iter().zip(&self.rows) { if let Some(row) = row {
            if records.is_some_and(|records|records.bind_record(pass,mesh)){pass.set_pipeline(&self.record_pipeline);}else{pass.set_pipeline(&self.pipeline);mesh.bind_model(pass);}pass.set_bind_group(2,&row.group,&[]); pass.draw(0..mesh.geometry.index_count, 0..1);
        } }
    }

    pub fn clear(&mut self) { self.rows.clear(); }
    pub fn ready(&self, count: usize) -> bool { self.rows.len() == count }

    pub fn bytes(&self) -> u64 { self.rows.iter().flatten().map(|row| row.bytes).sum() }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-06.rs"
```

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-07.rs"
```

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-08.rs"
```

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-09.rs"
```

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-10.rs"
```

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-11.rs"
```

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-12.rs"
```

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-13.rs"
```

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-14.rs"
```

</details>

### 4. `src/pick_encode.rs`

Share the original object table with both first and source-face pick passes.

<details>
<summary>Locate the existing block</summary>

```rust
            faces.draw(&mut pass,meshes,view_group,crop_group);
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
            faces.draw(&mut pass,meshes,view_group,crop_group,records.map(|(rows,_)|rows));
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-15.rs"
```

</details>

</details>

<a id="checkpoint-96zzzx-pack-existing-ambient-radius-in-object-records"></a>

### Step 6 of 16 · Pack existing ambient radius in object records (`96zzzx-pack-existing-ambient-radius-in-object-records`)

- **Goal:** Carry the existing source radius in a spare aligned record word.
- **Data Flow Trace:** `Existing placed-source radius → exact float bits in aligned object record → prepared property vertex entry`
- **Files You Touch:** `src/gpu_mesh.rs` (*Store the existing source radius in the spare aligned word and write only a changed value.*) · `src/renderer.rs` (*Reuse the existing source-bound calculations before refreshing shared records.*) · `src/ambient_sources.wgsl` (*Reuse the surface view and object declarations instead of repeating their storage contract.*) · `src/ambient_source_pipeline.rs` (*Assemble property shading with the surface record and geometry helpers already written.*)

The ambient radius is already computed from the placed source bounds. Preserve that one calculation and store its float bits in the spare range word. Keep it when selection updates rebuild the packed record. Refactor the property vertex stage into one helper and prepare a slot-aware entry. Property drawing still uses its earlier uniform pipeline until the next lesson connects the shared entry.

> **Self-Check Question:** *Why bitcast the radius instead of converting its numeric value to an integer?*  
> **Answer:** The record range is an unsigned vector. A bitcast preserves the original float bits, including fractional radii; numeric conversion would truncate the radius.

- **What You Should See:** Object records carry the existing source radius and ordinary property drawing retains its earlier output.
- **Hands-On Lab Experiment:** Toggle Arctic on ordinary and copied surfaces, then select and restore an original. Inspect the packed radius word and the prepared slot-aware property entry; it is not connected yet.

<details>
<summary>Implementation 96zzzx-pack-existing-ambient-radius-in-object-records: Pack existing ambient radius in object records</summary>

Carry the existing source radius in a spare aligned record word.

Data flow: Existing placed-source radius → exact float bits in aligned object record → prepared property vertex entry.

### 1. `src/gpu_mesh.rs`

Store the existing source radius in the spare aligned word and write only a changed value. Retain derived source metadata when rebuilding the current record. Selection and style updates keep the current radius bits.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    pub fn settings_bytes(&self) -> u64 { self.model.size() }

    pub fn update(&mut self, queue: &wgpu::Queue, object: &Object, style:Style) -> [usize; 2] {
        let next = settings(object, style,self.cap_flags|if self.xray&&self.cap_flags&8192!=0{32768}else{0},self.cap_token,self.primitive_base,self.geometry.index_start);
        let mut writes = [0, 0];
        for (start, end) in [(0, 64), (64, 80), (80, 144), (144,RANGE),(RANGE,BYTES)] {
            if next[start..end] != self.previous[start..end] {
```

</details>

Replace it with:

```rust
    pub fn settings_bytes(&self) -> u64 { self.model.size() }

    pub fn update(&mut self, queue: &wgpu::Queue, object: &Object, style:Style) -> [usize; 2] {
        let mut next = settings(object, style,self.cap_flags|if self.xray&&self.cap_flags&8192!=0{32768}else{0},self.cap_token,self.primitive_base,self.geometry.index_start);
        next[RANGE+8..RANGE+12].copy_from_slice(&self.previous[RANGE+8..RANGE+12]);
        let mut writes = [0, 0];
        for (start, end) in [(0, 64), (64, 80), (80, 144), (144,RANGE),(RANGE,BYTES)] {
            if next[start..end] != self.previous[start..end] {
```

<details>
<summary>Locate this block</summary>

```rust
    pub(crate) fn packed(&self)->Record {self.previous}
    pub fn selected(&self)->bool {u32::from_ne_bytes(self.previous[64..68].try_into().unwrap())&1!=0}

    pub fn primitive_base(&self)->u32{self.primitive_base}
    pub fn set_primitive_base(&mut self,queue:&wgpu::Queue,base:u32)->Result<(),&'static str>{
        if base.checked_add(self.geometry.index_count/3).is_none_or(|end|end>=crate::primitive_image::CAP_START){return Err("Primitive range overlaps reserved caps");}
```

</details>

Replace it with:

```rust
    pub(crate) fn packed(&self)->Record {self.previous}
    pub fn selected(&self)->bool {u32::from_ne_bytes(self.previous[64..68].try_into().unwrap())&1!=0}

    pub fn set_ambient_radius(&mut self,queue:&wgpu::Queue,radius:f32){let bytes=radius.to_ne_bytes();if self.previous[RANGE+8..RANGE+12]!=bytes {self.previous[RANGE+8..RANGE+12].copy_from_slice(&bytes);queue.write_buffer(&self.model,(RANGE+8)as u64,&bytes);}}
    pub fn primitive_base(&self)->u32{self.primitive_base}
    pub fn set_primitive_base(&mut self,queue:&wgpu::Queue,base:u32)->Result<(),&'static str>{
        if base.checked_add(self.geometry.index_count/3).is_none_or(|end|end>=crate::primitive_image::CAP_START){return Err("Primitive range overlaps reserved caps");}
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

    pub fn set_ambient_radius(&mut self,queue:&wgpu::Queue,radius:f32){let bytes=radius.to_ne_bytes();if self.previous[RANGE+8..RANGE+12]!=bytes {self.previous[RANGE+8..RANGE+12].copy_from_slice(&bytes);queue.write_buffer(&self.model,(RANGE+8)as u64,&bytes);}}
    pub fn primitive_base(&self)->u32{self.primitive_base}
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
        let mut next = settings(object, style,self.cap_flags|if self.xray&&self.cap_flags&8192!=0{32768}else{0},self.cap_token,self.primitive_base,self.geometry.index_start);
        next[RANGE+8..RANGE+12].copy_from_slice(&self.previous[RANGE+8..RANGE+12]);
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
--8<-- "journey/code/96zzzx-pack-existing-ambient-radius-in-object-records-01.rs"
```

```text
--8<-- "journey/code/96zzzx-pack-existing-ambient-radius-in-object-records-02.rs"
```

```text
--8<-- "journey/code/96zzzx-pack-existing-ambient-radius-in-object-records-03.rs"
```

</details>

### 2. `src/renderer.rs`

Reuse the existing source-bound calculations before refreshing shared records.

<details>
<summary>Locate the existing block</summary>

```rust
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        let radii=self.ambient_rows.specs.iter().copied().collect::<std::collections::HashMap<_,_>>();for mesh in &mut self.meshes {mesh.set_ambient_radius(&self.queue,radii.get(&mesh.id.value()).copied().unwrap_or(0.));}
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzx-pack-existing-ambient-radius-in-object-records-04.rs"
```

</details>

### 3. `src/ambient_sources.wgsl`

Reuse the surface view and object declarations instead of repeating their storage contract. Carry the exact per-object radius into the property fragment. Share one property vertex function and prepare a slot-selected record entry without changing the earlier normal policy. Use the radius carried by the selected vertex entry.

<details>
<summary>Locate the existing block</summary>

```wgsl
struct View{transform:mat4x4<f32>,display:vec4<f32>}
struct Object{model:mat4x4<f32>,selection:vec4<u32>,normal_matrix:mat4x4<f32>,face_colour:vec4<f32>,range:vec4<u32>}
@group(0) @binding(0) var<uniform> view:View;
@group(1) @binding(0) var<uniform> object:Object;
@group(3) @binding(0) var<uniform> ambient_radius:vec4<f32>;
struct Vertex{@builtin(position) position:vec4<f32>,@location(0) world:vec3<f32>,@location(1) normal:vec3<f32>,@location(2) @interpolate(flat) flags:u32}
@vertex fn vs_main(@location(0) position:vec3<f32>,@location(2) normal:u32)->Vertex{let world=object.model*vec4<f32>(position,1.);return Vertex(view.transform*world,world.xyz,(object.normal_matrix*vec4<f32>(oct32_decode(normal),0.)).xyz,object.selection.z);}
fn encode_normal(n:vec3<f32>)->vec2<f32>{let p=n.xy/(abs(n.x)+abs(n.y)+abs(n.z));let folded=(1.-abs(p.yx))*select(vec2<f32>(-1.),vec2<f32>(1.),p>=vec2<f32>(0.));return select(folded,p,n.z>=0.);}
struct Source{@location(0) radius:f32,@location(1) normal:vec2<f32>}
@fragment fn fs_main(in:Vertex,@builtin(front_facing) front:bool)->Source{
 let face=cross(dpdx(in.world),dpdy(in.world));if((in.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(in.flags,in.world)){discard;}
 let normal=normalize(select(select(face,-face,front),in.normal,dot(in.normal,in.normal)>0.));
 return Source(ambient_radius.x,encode_normal(normal)*0.5+0.5);
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(3) @binding(0) var<uniform> ambient_radius:vec4<f32>;
struct Vertex{@builtin(position) position:vec4<f32>,@location(0) world:vec3<f32>,@location(1) normal:vec3<f32>,@location(2) @interpolate(flat) flags:u32,@location(3) @interpolate(flat) radius:f32}
fn ambient_vertex(position:vec3<f32>,normal:vec3<f32>,object:ObjectSettings,radius:f32)->Vertex {
 let world=object.model*vec4<f32>(position,1.);return Vertex(view.transform*world,world.xyz,(object.normal_matrix*vec4<f32>(normal,0.)).xyz,object.selection.z,radius);
}
@vertex fn vs_main(@location(0) position:vec3<f32>,@location(2) normal:u32)->Vertex {return ambient_vertex(position,oct32_decode(normal),object_at(0u),ambient_radius.x);}
@vertex fn vs_record(@builtin(vertex_index) occurrence:u32,@location(0) row:u32)->Vertex {let object=object_at(row);let input=mesh_vertex(occurrence,object.range.x);return ambient_vertex(input.position,input.normal,object,bitcast<f32>(object.range.z));}
fn encode_normal(n:vec3<f32>)->vec2<f32>{let p=n.xy/(abs(n.x)+abs(n.y)+abs(n.z));let folded=(1.-abs(p.yx))*select(vec2<f32>(-1.),vec2<f32>(1.),p>=vec2<f32>(0.));return select(folded,p,n.z>=0.);}
struct Source{@location(0) radius:f32,@location(1) normal:vec2<f32>}
@fragment fn fs_main(in:Vertex,@builtin(front_facing) front:bool)->Source{
 let face=cross(dpdx(in.world),dpdy(in.world));if((in.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(in.flags,in.world)){discard;}
 let normal=normalize(select(select(face,-face,front),in.normal,dot(in.normal,in.normal)>0.));
 return Source(in.radius,encode_normal(normal)*0.5+0.5);
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzx-pack-existing-ambient-radius-in-object-records-06.rs"
```

```text
--8<-- "journey/code/96zzzx-pack-existing-ambient-radius-in-object-records-07.rs"
```

```text
--8<-- "journey/code/96zzzx-pack-existing-ambient-radius-in-object-records-08.rs"
```

</details>

### 4. `src/ambient_source_pipeline.rs`

Assemble property shading with the surface record and geometry helpers already written.

<details>
<summary>Locate the existing block</summary>

```rust
  let view=crate::gpu_layout::uniform(device,80);let model=crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64);let clip=crate::clipping_gpu::layout(device);let radius=crate::gpu_layout::uniform(device,16);let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("ambient original source metadata"),bind_group_layouts:&[Some(&view),Some(&model),Some(&clip),Some(&radius)],immediate_size:0});let module=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("ambient original normals and radii"),source:wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(include_str!("ambient_sources.wgsl"),2,0).into())});
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
  let view=crate::gpu_layout::uniform(device,80);let model=crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64);let clip=crate::clipping_gpu::layout(device);let radius=crate::gpu_layout::uniform(device,16);let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("ambient original source metadata"),bind_group_layouts:&[Some(&view),Some(&model),Some(&clip),Some(&radius)],immediate_size:0});let module=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("ambient original normals and radii"),source:wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(concat!(include_str!("triangle.wgsl"),"
",include_str!("ambient_sources.wgsl")),2,0).into())});
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzx-pack-existing-ambient-radius-in-object-records-09.rs"
```

</details>

</details>

<a id="checkpoint-96zzzy-read-ambient-properties-through-shared-records"></a>

### Step 7 of 16 · Read ambient properties through shared records (`96zzzy-read-ambient-properties-through-shared-records`)

- **Goal:** Connect source normal and radius drawing to the existing object table.
- **Data Flow Trace:** `Current object row and arena geometry → original source normals and radius → existing ambient recipe`
- **Files You Touch:** `src/ambient_source_pipeline.rs` (*Own the shared property state beside its existing fallback.*) · `src/ambient_source_gpu.rs` (*Accept the existing renderer object table.*) · `src/ambient_gpu.rs` (*Keep the object table available while preparing original source properties.*) · `src/renderer.rs` (*Use the same current object records for rendering, picking and ambient properties.*) · `src/ambient_gpu_tests.rs` (*Supply an exact shared-versus-uniform ambient comparison while retaining every original cache, source, shading and release assertion.*)

Read the normal transform, flags and radius from the current shared object row. Pull positions and packed normals from the same arena already used for color and picks. Keep one property shader and descriptor for shared and uniform fallback states. The ambient algorithm and its cache remain unchanged; only its source resource path changes.

> **Self-Check Question:** *Why keep the ambient recipe unchanged during this connection?*  
> **Answer:** Changing its source resource should preserve the earlier property values and pixels. The exact comparison isolates that resource change from later contact and pipeline-order work.

- **What You Should See:** Ambient source properties consume current shared records and retain the earlier ambient pixels.
- **Hands-On Lab Experiment:** Toggle Arctic on an ordinary original and a copied placement. Select and restore them, then inspect the supplied exact shared-versus-uniform ambient comparison and full release checks.

<details>
<summary>Implementation 96zzzy-read-ambient-properties-through-shared-records: Read ambient properties through shared records</summary>

Connect source normal and radius drawing to the existing object table.

Data flow: Current object row and arena geometry → original source normals and radius → existing ambient recipe.

### 1. `src/ambient_source_pipeline.rs`

Own the shared property state beside its existing fallback. Move the fallback radius layout to the shared descriptor owner. Build both property states from one descriptor. Select the current shared record layout or the earlier uniform layout. Use the existing arena geometry binding for pulled properties. Shared properties read radius from their record; fallback properties retain their uniform. Adapt the same property shader through the existing record owner. Share the shader source across both property states. Select the slot-aware property entry for shared records. Keep the indexed vertex attributes alive while creating either property state. Read draw slots for shared properties and retain the earlier indexed vertex format for fallback. Create both states with the same property outputs and depth policy.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Pipeline{pub draw:wgpu::RenderPipeline,pub radius:wgpu::BindGroupLayout}
impl Pipeline{
 pub fn new(device:&wgpu::Device)->Self{
  let view=crate::gpu_layout::uniform(device,80);let model=crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64);let clip=crate::clipping_gpu::layout(device);let radius=crate::gpu_layout::uniform(device,16);let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("ambient original source metadata"),bind_group_layouts:&[Some(&view),Some(&model),Some(&clip),Some(&radius)],immediate_size:0});let module=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("ambient original normals and radii"),source:wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(concat!(include_str!("triangle.wgsl"),"
",include_str!("ambient_sources.wgsl")),2,0).into())});
  let draw=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("ambient original normals and radii"),layout:Some(&layout),vertex:wgpu::VertexState{module:&module,entry_point:Some("vs_main"),compilation_options:Default::default(),buffers:&[wgpu::VertexBufferLayout{array_stride:20,step_mode:wgpu::VertexStepMode::Vertex,attributes:&wgpu::vertex_attr_array![0=>Float32x3,2=>Uint32]}]},fragment:Some(wgpu::FragmentState{module:&module,entry_point:Some("fs_main"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::R32Float,blend:None,write_mask:wgpu::ColorWrites::RED}),Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::Rg8Unorm,blend:None,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(crate::depth::state_for_ambient()),multisample:Default::default(),multiview_mask:None,cache:None});Self{draw,radius}
 }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub struct Pipeline{pub draw:wgpu::RenderPipeline,pub record_draw:wgpu::RenderPipeline,pub radius:wgpu::BindGroupLayout}
impl Pipeline{
 pub fn new(device:&wgpu::Device)->Self{
  let radius=crate::gpu_layout::uniform(device,16);let create=|shared| {
  let view=crate::gpu_layout::uniform(device,80);let model=if shared {crate::object_rows::layout(device)}else{crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64)};let clip=crate::clipping_gpu::layout(device);let geometry=crate::gpu_geometry::GpuGeometry::storage_layout(device);let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("ambient original source metadata"),bind_group_layouts:&[Some(&view),Some(&model),Some(&clip),Some(if shared {&geometry}else{&radius})],immediate_size:0});let source=concat!(include_str!("triangle.wgsl"),"
",include_str!("ambient_sources.wgsl"));let source=if shared {crate::object_rows::source(source)}else{source.into()};let module=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("ambient original normals and radii"),source:wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(&source,2,0).into())});
  let attributes=wgpu::vertex_attr_array![0=>Float32x3,2=>Uint32];let draw=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("ambient original normals and radii"),layout:Some(&layout),vertex:wgpu::VertexState{module:&module,entry_point:Some(if shared {"vs_record"}else{"vs_main"}),compilation_options:Default::default(),buffers:&[if shared {crate::draw_slots::layout()}else{wgpu::VertexBufferLayout{array_stride:20,step_mode:wgpu::VertexStepMode::Vertex,attributes:&attributes}}]},fragment:Some(wgpu::FragmentState{module:&module,entry_point:Some("fs_main"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::R32Float,blend:None,write_mask:wgpu::ColorWrites::RED}),Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::Rg8Unorm,blend:None,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(crate::depth::state_for_ambient()),multisample:Default::default(),multiview_mask:None,cache:None});draw};let draw=create(false);let record_draw=create(true);Self{draw,record_draw,radius}
 }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-01.rs"
```

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-03.rs"
```

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-04.rs"
```

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-05.rs"
```

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-06.rs"
```

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-07.rs"
```

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-08.rs"
```

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-09.rs"
```

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-attributes.rs"
```

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-10.rs"
```

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-11.rs"
```

</details>

### 2. `src/ambient_source_gpu.rs`

Accept the existing renderer object table. Read current shared properties or the earlier exact uniform fallback.

<details>
<summary>Locate the existing block</summary>

```rust
use std::collections::HashMap;
pub struct Sources{pipes:crate::ambient_source_pipeline::Pipeline,rows:HashMap<u32,(wgpu::Buffer,wgpu::BindGroup,f32)>,pub writes:u64,pub draws:u64}
impl Sources{
 pub fn new(device:&wgpu::Device)->Self{Self{pipes:crate::ambient_source_pipeline::Pipeline::new(device),rows:Default::default(),writes:0,draws:0}}
 pub fn set(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,specs:&[(u32,f32)]){
  let live=specs.iter().map(|(id,_)|*id).collect::<std::collections::HashSet<_>>();self.rows.retain(|id,_|live.contains(id));for &(id,radius)in specs{let row=self.rows.entry(id).or_insert_with(||{let buffer=device.create_buffer(&wgpu::BufferDescriptor{label:Some("original ambient contact radius"),size:16,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let group=crate::gpu_layout::group(device,&self.pipes.radius,&buffer);(buffer,group,f32::NAN)});if row.2!=radius{queue.write_buffer(&row.0,0,&radius.to_ne_bytes());row.2=radius;self.writes+=1;}}if self.rows.is_empty(){self.rows.shrink_to_fit();}
 }
 pub fn encode(&mut self,encoder:&mut wgpu::CommandEncoder,radius:&wgpu::TextureView,normals:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clip:&wgpu::BindGroup,meshes:&[crate::gpu_mesh::GpuMesh]){
  let attachments=[radius,normals].map(|view|Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}}));let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original ambient source properties"),color_attachments:&attachments,depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&self.pipes.draw);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clip,&[]);for mesh in meshes{if let Some((_,group,_))=self.rows.get(&mesh.id.value()){mesh.bind_model(&mut pass);pass.set_bind_group(3,group,&[]);mesh.geometry.draw(&mut pass);self.draws+=1;}}
 }
 pub fn bytes(&self)->u64{self.rows.values().map(|r|r.0.size()).sum()}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use std::collections::HashMap;
pub struct Sources{pipes:crate::ambient_source_pipeline::Pipeline,rows:HashMap<u32,(wgpu::Buffer,wgpu::BindGroup,f32)>,pub writes:u64,pub draws:u64}
impl Sources{
 pub fn new(device:&wgpu::Device)->Self{Self{pipes:crate::ambient_source_pipeline::Pipeline::new(device),rows:Default::default(),writes:0,draws:0}}
 pub fn set(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,specs:&[(u32,f32)]){
  let live=specs.iter().map(|(id,_)|*id).collect::<std::collections::HashSet<_>>();self.rows.retain(|id,_|live.contains(id));for &(id,radius)in specs{let row=self.rows.entry(id).or_insert_with(||{let buffer=device.create_buffer(&wgpu::BufferDescriptor{label:Some("original ambient contact radius"),size:16,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let group=crate::gpu_layout::group(device,&self.pipes.radius,&buffer);(buffer,group,f32::NAN)});if row.2!=radius{queue.write_buffer(&row.0,0,&radius.to_ne_bytes());row.2=radius;self.writes+=1;}}if self.rows.is_empty(){self.rows.shrink_to_fit();}
 }
 pub fn encode(&mut self,encoder:&mut wgpu::CommandEncoder,radius:&wgpu::TextureView,normals:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clip:&wgpu::BindGroup,meshes:&[crate::gpu_mesh::GpuMesh],records:Option<&crate::instance_gpu::Lane>){
  let attachments=[radius,normals].map(|view|Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}}));let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original ambient source properties"),color_attachments:&attachments,depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&self.pipes.draw);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clip,&[]);for mesh in meshes{if let Some((_,group,_))=self.rows.get(&mesh.id.value()){if records.is_some_and(|rows|rows.bind_record(&mut pass,mesh)){pass.set_pipeline(&self.pipes.record_draw);mesh.geometry.draw_pulled(&mut pass,1);}else{pass.set_pipeline(&self.pipes.draw);mesh.bind_model(&mut pass);pass.set_bind_group(3,group,&[]);mesh.geometry.draw(&mut pass);}self.draws+=1;}}
 }
 pub fn bytes(&self)->u64{self.rows.values().map(|r|r.0.size()).sum()}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-12.rs"
```

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-13.rs"
```

</details>

### 3. `src/ambient_gpu.rs`

Keep the object table available while preparing original source properties. Pass the existing record owner through the unchanged ambient recipe.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{ambient_images::Images,ambient_groups::Groups,ambient_pass::draw};
struct State{images:Images,groups:Groups,input:(wgpu::TextureView,wgpu::TextureView,wgpu::BindGroup)}
pub struct Ambient{pipes:crate::ambient_pipeline::Pipelines,pub sources:crate::ambient_source_gpu::Sources,frame:wgpu::Buffer,state:Option<State>,cached:Option<([f32;80],[u64;2])>,pub preparations:u64,pub writes:u64,pub passes:u64,pub error:Option<&'static str>}
impl Ambient{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{Self{pipes:crate::ambient_pipeline::Pipelines::new(device,format),sources:crate::ambient_source_gpu::Sources::new(device),frame:device.create_buffer(&wgpu::BufferDescriptor{label:Some("current ambient camera"),size:320,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false}),state:None,cached:None,preparations:0,writes:0,passes:0,error:None}}
 pub fn clear(&mut self,device:&wgpu::Device,queue:&wgpu::Queue){self.state=None;self.cached=None;self.sources.set(device,queue,&[]);self.error=None;}
 fn prepare(&mut self,device:&wgpu::Device,full:[u32;2],density:f32,depth:&wgpu::TextureView,sections:&wgpu::TextureView,clip:&wgpu::Buffer){
  let(w,h)=crate::pyramid_size::resolution((full[0],full[1]),f64::from(density));let size=[w.max(32),h.max(32)];
  if self.state.as_ref().is_none_or(|s|s.images.full!=full||s.images.size!=size){let images=Images::new(device,full,density);let groups=Groups::new(device,&self.pipes,&images);let group=Groups::source(device,&self.pipes,&images,depth,sections,&self.frame,clip);self.state=Some(State{images,groups,input:(depth.clone(),sections.clone(),group)});self.cached=None;self.preparations+=1;}
  let state=self.state.as_mut().unwrap();if state.input.0!=*depth||state.input.1!=*sections{state.input=(depth.clone(),sections.clone(),Groups::source(device,&self.pipes,&state.images,depth,sections,&self.frame,clip));self.cached=None;}
 }
 pub fn draw(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,encoder:&mut wgpu::CommandEncoder,color:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,clip:&wgpu::Buffer,sections:&wgpu::TextureView,meshes:&[crate::gpu_mesh::GpuMesh],matrix:[f32;16],full:[u32;2],density:f32,ground:[f32;2],revision:[u64;2]){
  if ground[1]<=0.0{self.clear(device,queue);return;}if matrix.iter().any(|v|!v.is_finite())||full.contains(&0)||!density.is_finite()||density<=0.{self.error=Some("Invalid ambient view");return;}
  self.prepare(device,full,density,depth,sections,clip);let state=self.state.as_ref().unwrap();let Some(uniform)=crate::ambient_camera::uniform(matrix,full,state.images.size,ground)else{self.error=Some("Invalid ambient camera inverse");return;};
  if self.cached!=Some((uniform,revision)){
   queue.write_buffer(&self.frame,0,&uniform.into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());self.writes+=1;
   let images=&state.images;let groups=&state.groups;let source=&state.input.2;self.sources.encode(encoder,&images.source_radius.view,&images.source_normals.view,depth,view,clipping,meshes);self.passes+=1;
   draw(encoder,&self.pipes.prepare,&[&images.linear.levels[0],&images.radii.levels[0],&images.normals.view],&[(0,source)],true);
   for level in 1..6{draw(encoder,&self.pipes.reduce,&[&images.linear.levels[level],&images.radii.levels[level]],&[(0,&groups.levels[level-1])],true);}self.passes+=6;
   draw(encoder,&self.pipes.occupancy,&[&images.occupancy.view],&[(0,&groups.levels[0])],true);
   draw(encoder,&self.pipes.ground,&[&images.ground.view],&[(0,source),(1,&groups.levels[0]),(2,&groups.occupancy),(3,&groups.history)],true);
   draw(encoder,&self.pipes.raw,&[&images.ao[0].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ground),(3,&groups.history)],true);
   draw(encoder,&self.pipes.filter[0],&[&images.ao[1].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[0]),(3,&groups.history)],true);
   draw(encoder,&self.pipes.filter[1],&[&images.ao[0].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[1]),(3,&groups.history)],true);
   draw(encoder,&self.pipes.upsample,&[&images.ao[2].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[0]),(3,&groups.history)],true);self.passes+=6;
   self.cached=Some((uniform,revision));
  }
  draw(encoder,&self.pipes.composite,&[color],&[(1,&state.groups.ao[2])],false);self.passes+=1;self.error=None;
 }
 pub fn usage(&self)->[u64;5]{[self.frame.size()+self.sources.bytes()+self.state.as_ref().map_or(0,|s|s.images.bytes()+s.groups.buffers.iter().map(wgpu::Buffer::size).sum::<u64>()),1,self.preparations,self.writes,self.passes]}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{ambient_images::Images,ambient_groups::Groups,ambient_pass::draw};
struct State{images:Images,groups:Groups,input:(wgpu::TextureView,wgpu::TextureView,wgpu::BindGroup)}
pub struct Ambient{pipes:crate::ambient_pipeline::Pipelines,pub sources:crate::ambient_source_gpu::Sources,frame:wgpu::Buffer,state:Option<State>,cached:Option<([f32;80],[u64;2])>,pub preparations:u64,pub writes:u64,pub passes:u64,pub error:Option<&'static str>}
impl Ambient{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{Self{pipes:crate::ambient_pipeline::Pipelines::new(device,format),sources:crate::ambient_source_gpu::Sources::new(device),frame:device.create_buffer(&wgpu::BufferDescriptor{label:Some("current ambient camera"),size:320,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false}),state:None,cached:None,preparations:0,writes:0,passes:0,error:None}}
 pub fn clear(&mut self,device:&wgpu::Device,queue:&wgpu::Queue){self.state=None;self.cached=None;self.sources.set(device,queue,&[]);self.error=None;}
 fn prepare(&mut self,device:&wgpu::Device,full:[u32;2],density:f32,depth:&wgpu::TextureView,sections:&wgpu::TextureView,clip:&wgpu::Buffer){
  let(w,h)=crate::pyramid_size::resolution((full[0],full[1]),f64::from(density));let size=[w.max(32),h.max(32)];
  if self.state.as_ref().is_none_or(|s|s.images.full!=full||s.images.size!=size){let images=Images::new(device,full,density);let groups=Groups::new(device,&self.pipes,&images);let group=Groups::source(device,&self.pipes,&images,depth,sections,&self.frame,clip);self.state=Some(State{images,groups,input:(depth.clone(),sections.clone(),group)});self.cached=None;self.preparations+=1;}
  let state=self.state.as_mut().unwrap();if state.input.0!=*depth||state.input.1!=*sections{state.input=(depth.clone(),sections.clone(),Groups::source(device,&self.pipes,&state.images,depth,sections,&self.frame,clip));self.cached=None;}
 }
 pub fn draw(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,encoder:&mut wgpu::CommandEncoder,color:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,clip:&wgpu::Buffer,sections:&wgpu::TextureView,meshes:&[crate::gpu_mesh::GpuMesh],matrix:[f32;16],full:[u32;2],density:f32,ground:[f32;2],revision:[u64;2],records:Option<&crate::instance_gpu::Lane>){
  if ground[1]<=0.0{self.clear(device,queue);return;}if matrix.iter().any(|v|!v.is_finite())||full.contains(&0)||!density.is_finite()||density<=0.{self.error=Some("Invalid ambient view");return;}
  self.prepare(device,full,density,depth,sections,clip);let state=self.state.as_ref().unwrap();let Some(uniform)=crate::ambient_camera::uniform(matrix,full,state.images.size,ground)else{self.error=Some("Invalid ambient camera inverse");return;};
  if self.cached!=Some((uniform,revision)){
   queue.write_buffer(&self.frame,0,&uniform.into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());self.writes+=1;
   let images=&state.images;let groups=&state.groups;let source=&state.input.2;self.sources.encode(encoder,&images.source_radius.view,&images.source_normals.view,depth,view,clipping,meshes,records);self.passes+=1;
   draw(encoder,&self.pipes.prepare,&[&images.linear.levels[0],&images.radii.levels[0],&images.normals.view],&[(0,source)],true);
   for level in 1..6{draw(encoder,&self.pipes.reduce,&[&images.linear.levels[level],&images.radii.levels[level]],&[(0,&groups.levels[level-1])],true);}self.passes+=6;
   draw(encoder,&self.pipes.occupancy,&[&images.occupancy.view],&[(0,&groups.levels[0])],true);
   draw(encoder,&self.pipes.ground,&[&images.ground.view],&[(0,source),(1,&groups.levels[0]),(2,&groups.occupancy),(3,&groups.history)],true);
   draw(encoder,&self.pipes.raw,&[&images.ao[0].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ground),(3,&groups.history)],true);
   draw(encoder,&self.pipes.filter[0],&[&images.ao[1].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[0]),(3,&groups.history)],true);
   draw(encoder,&self.pipes.filter[1],&[&images.ao[0].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[1]),(3,&groups.history)],true);
   draw(encoder,&self.pipes.upsample,&[&images.ao[2].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[0]),(3,&groups.history)],true);self.passes+=6;
   self.cached=Some((uniform,revision));
  }
  draw(encoder,&self.pipes.composite,&[color],&[(1,&state.groups.ao[2])],false);self.passes+=1;self.error=None;
 }
 pub fn usage(&self)->[u64;5]{[self.frame.size()+self.sources.bytes()+self.state.as_ref().map_or(0,|s|s.images.bytes()+s.groups.buffers.iter().map(wgpu::Buffer::size).sum::<u64>()),1,self.preparations,self.writes,self.passes]}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-14.rs"
```

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-15.rs"
```

</details>

### 4. `src/renderer.rs`

Use the same current object records for rendering, picking and ambient properties.

<details>
<summary>Locate the existing block</summary>

```rust
        if self.ambient_enabled.get(){let clip=self.clipping.borrow();if let Some(ambient)=self.ambient.borrow_mut().as_mut(){ambient.draw(&self.device,&self.queue,&mut encoder,view,&self.depth,&self.view_group,&clip_group,&clip.buffer,&self.primitive.view,&self.meshes,*transform,self.size,self.density,self.ambient_rows.ground,[self.pyramid_revision,clip_revision]);}}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        if self.ambient_enabled.get(){let clip=self.clipping.borrow();if let Some(ambient)=self.ambient.borrow_mut().as_mut(){ambient.draw(&self.device,&self.queue,&mut encoder,view,&self.depth,&self.view_group,&clip_group,&clip.buffer,&self.primitive.view,&self.meshes,*transform,self.size,self.density,self.ambient_rows.ground,[self.pyramid_revision,clip_revision],Some(&self.instances));}}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-16.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/ambient_gpu_tests.rs`

Supply an exact shared-versus-uniform ambient comparison while retaining every original cache, source, shading and release assertion.

<details>
<summary>Locate the existing block</summary>

```text
fn actual_ambient_darkens_original_contacts_in_one_frame_caches_and_releases_large_images(){
 let mut scene=Scene::empty();let owner=scene.insert(crate::prepared::PreparedMesh::new(source(false)).unwrap()).unwrap();let original=scene.objects()[0].geometry().unwrap().to_proto();let mut camera=crate::camera::Camera::default();camera.isometric();camera.aspect=4./3.;camera.target=[0.,0.,-0.5];camera.distance=3.;let matrix=camera.uniform();let r=renderer(&scene);let plain=pixels(&r,&matrix);let uploads=r.stats()[0];r.set_ambient(true);let shaded=pixels(&r,&matrix);assert_eq!(r.ambient_error(),None);let usage=r.ambient_usage();assert!(usage[0]>100_000);assert_eq!(usage[1],1);assert_eq!(usage[3],1);let second=pixels(&r,&matrix);assert_eq!(second,shaded,"one frame completes ambient shading");assert_eq!(&r.ambient_usage()[1..4],&usage[1..4]);assert_eq!(r.stats()[0],uploads);let darker=plain.chunks_exact(4).zip(shaded.chunks_exact(4)).filter(|(a,b)|a[..3]!=[255,255,255]&&a[..3].iter().zip(&b[..3]).any(|(a,b)|a.saturating_sub(*b)>3)).count();assert!(darker>100,"actual ambient shading: {darker} pixels");let shadow=plain.chunks_exact(4).zip(shaded.chunks_exact(4)).filter(|(a,b)|a[..3]==[255,255,255]&&b[..3].iter().any(|v|*v<235)).count();assert!(shadow>20,"actual virtual-ground contacts: {shadow}");assert_eq!(scene.objects()[0].id,owner);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);r.set_ambient(false);assert_eq!(r.ambient_usage()[0],320);assert_eq!(pixels(&r,&matrix),plain);r.set_ambient(true);assert_eq!(pixels(&r,&matrix),shaded);assert_eq!(r.ambient_usage()[1],1,"style toggles retain compiled pipelines");assert_eq!(r.stats()[0],uploads);
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-17.rs"
```

</details>

</details>

<a id="checkpoint-96zzzz-share-the-object-record-shader-schema"></a>

### Step 8 of 16 · Share the object record shader schema (`96zzzz-share-the-object-record-shader-schema`)

- **Goal:** Keep one aligned object record declaration across drawing, caps and projection.
- **Data Flow Trace:** `One aligned object declaration → raster and compute shader assembly → unchanged GPU bytes`
- **Files You Touch:** `src/object_settings.wgsl` (*Write the existing aligned record schema once.*) · `src/triangle.wgsl` (*Use the record declaration supplied by shared shader assembly.*) · `src/cap_count.wgsl` (*Cap crossings use the identical record declaration.*) · `src/finite_project.wgsl` (*Projection uses the same byte-compatible model and range fields.*) · `src/object_record.rs` (*Expose the shared declaration through the existing record owner.*) · `src/clipping_gpu.rs` (*Assemble every clipped raster consumer with the shared record declaration.*) · `src/finite_project_source.rs` (*Compute projection includes the same declaration before its packed geometry helper.*) · `src/object_record_tests.rs` (*Verify that projection uses the same aligned members as both raster states after consolidating the flag vector.*)

Surface and cap shaders already read the same record bytes, while projection spells the equivalent fields separately. Extract the aligned declaration once and assemble each shader with that declaration. Keep every offset and resource binding unchanged. This avoids repeating the schema before the projection and cap consumers connect to shared records.

> **Self-Check Question:** *Why centralize the declaration before connecting more consumers?*  
> **Answer:** A duplicated layout can drift even when the buffers are shared. One declaration keeps model, flags, normal transform, presentation and range offsets consistent.

- **What You Should See:** Surface, cap and projection shaders use one object schema with unchanged rendering and picking.
- **Hands-On Lab Experiment:** Inspect the single declaration, then repeat ordinary and copied-object picks, clipping and Arctic restoration. Compare the existing exact byte and pixel fixtures.

<details>
<summary>Implementation 96zzzz-share-the-object-record-shader-schema: Share the object record shader schema</summary>

Keep one aligned object record declaration across drawing, caps and projection.

Data flow: One aligned object declaration → raster and compute shader assembly → unchanged GPU bytes.

### 1. `src/object_settings.wgsl`

Write the existing aligned record schema once.

Create the file and type:

```wgsl
--8<-- "journey/code/96zzzz-share-the-object-record-shader-schema-01.rs"
```

### 2. `src/triangle.wgsl`

Use the record declaration supplied by shared shader assembly.

<details>
<summary>Locate the existing block</summary>

```wgsl
struct ObjectSettings {
    model: mat4x4<f32>,
    selection: vec4<u32>,
    normal_matrix: mat4x4<f32>,
    face_colour:vec4<f32>,
    range:vec4<u32>,
}
@group(1) @binding(0) var<uniform> object_settings: ObjectSettings;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(1) @binding(0) var<uniform> object_settings: ObjectSettings;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

</details>

### 3. `src/cap_count.wgsl`

Cap crossings use the identical record declaration.

<details>
<summary>Locate the existing block</summary>

```wgsl
struct ObjectSettings {
    model: mat4x4<f32>,
    selection: vec4<u32>,
    normal_matrix: mat4x4<f32>,
    face_colour:vec4<f32>,
    range:vec4<u32>,
}
@group(1) @binding(0) var<uniform> object: ObjectSettings;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(1) @binding(0) var<uniform> object: ObjectSettings;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

</details>

### 4. `src/finite_project.wgsl`

Projection uses the same byte-compatible model and range fields.

<details>
<summary>Locate the existing block</summary>

```wgsl
struct ObjectSettings {model:mat4x4<f32>,selected:u32,id:u32,pad:vec2<u32>,normals:mat4x4<f32>,colour:vec4<f32>,range:vec4<u32>}
@group(0) @binding(0) var<uniform> view:View;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(0) @binding(0) var<uniform> view:View;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

</details>

### 5. `src/object_record.rs`

Expose the shared declaration through the existing record owner.

<details>
<summary>Locate the existing block</summary>

```rust
pub type Record = [u8; BYTES];
```

</details>

After that block, type:

```rust
pub fn shader()->&'static str {include_str!("object_settings.wgsl")}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzz-share-the-object-record-shader-schema-05.rs"
```

</details>

### 6. `src/clipping_gpu.rs`

Assemble every clipped raster consumer with the shared record declaration. Keep the shared declaration separate from shader resource bindings.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{clipping_rows::Row,clipping_uniform::{ClipUniform,ClipView,clip_uniform}};
use wgpu::util::DeviceExt;
pub fn layout(device:&wgpu::Device)->wgpu::BindGroupLayout{crate::gpu_layout::uniform(device,448)}
pub fn source(base:&str,group:u32,binding:u32)->String{format!("{}\n{}\n@group({group}) @binding({binding}) var<uniform> clipping:ClipUniform;\n{}\n{base}",crate::colour::selection_shader(),crate::draw_slots::sentinel_shader(),concat!(include_str!("packed_vertex.wgsl"),"\n",include_str!("physical.wgsl"),"\n",include_str!("clip.wgsl")))}
pub fn bytes(uniform:&ClipUniform)->Vec<u8>{
    let mut bytes=uniform.planes.iter().chain(&uniform.screen).chain(&uniform.hatch).chain(&uniform.hatch_w).chain(&uniform.sides).flatten().copied().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();
    bytes.extend([uniform.count,uniform.samples,uniform.fill].into_iter().flat_map(u32::to_ne_bytes));bytes.extend([uniform.spacing,uniform.width,uniform.outline,uniform.pad[0],uniform.pad[1]].into_iter().flat_map(f32::to_ne_bytes));assert_eq!(bytes.len(),448);bytes
}
pub struct Clipping{pub buffer:wgpu::Buffer,pub group:wgpu::BindGroup,pub rows:Vec<Row>,pub error:Option<&'static str>,pub enabled:bool,pub solid:bool,key:Vec<u8>,pub writes:u64}
impl Clipping{
    pub fn new(device:&wgpu::Device)->Self{let key=bytes(&ClipUniform::default());let buffer=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("current original clipping planes"),contents:&key,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST});let group=crate::gpu_layout::group(device,&layout(device),&buffer);Self{buffer,group,rows:Vec::new(),error:None,enabled:true,solid:true,key,writes:0}}
    pub fn set(&mut self,scene:&crate::scene::Scene){match crate::clipping_rows::current(scene){Ok(rows)=>{self.rows=rows;self.error=None},Err(error)=>{self.rows.clear();self.error=Some(error)}}}
    pub fn frame(&mut self,queue:&wgpu::Queue,matrix:[f32;16],height:u32,density:f32){let matrix=session_rust::Xform::from_matrix(matrix.map(f64::from));let planes=if self.enabled{self.rows.iter().map(|r|r.plane).collect::<Vec<_>>()}else{Vec::new()};let uniform=clip_uniform(&planes,u32::from(self.solid),&ClipView{view_proj:&matrix,anchor:[0.;3],height,pixel_scale:f64::from(density),samples:1,forward_depth:true});let bytes=bytes(&uniform);if bytes!=self.key{queue.write_buffer(&self.buffer,0,&bytes);self.key=bytes;self.writes+=1;}}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{clipping_rows::Row,clipping_uniform::{ClipUniform,ClipView,clip_uniform}};
use wgpu::util::DeviceExt;
pub fn layout(device:&wgpu::Device)->wgpu::BindGroupLayout{crate::gpu_layout::uniform(device,448)}
pub fn source(base:&str,group:u32,binding:u32)->String{format!("{}\n{}\n{}\n@group({group}) @binding({binding}) var<uniform> clipping:ClipUniform;\n{}\n{base}",crate::colour::selection_shader(),crate::draw_slots::sentinel_shader(),crate::object_record::shader(),concat!(include_str!("packed_vertex.wgsl"),"\n",include_str!("physical.wgsl"),"\n",include_str!("clip.wgsl")))}
pub fn bytes(uniform:&ClipUniform)->Vec<u8>{
    let mut bytes=uniform.planes.iter().chain(&uniform.screen).chain(&uniform.hatch).chain(&uniform.hatch_w).chain(&uniform.sides).flatten().copied().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();
    bytes.extend([uniform.count,uniform.samples,uniform.fill].into_iter().flat_map(u32::to_ne_bytes));bytes.extend([uniform.spacing,uniform.width,uniform.outline,uniform.pad[0],uniform.pad[1]].into_iter().flat_map(f32::to_ne_bytes));assert_eq!(bytes.len(),448);bytes
}
pub struct Clipping{pub buffer:wgpu::Buffer,pub group:wgpu::BindGroup,pub rows:Vec<Row>,pub error:Option<&'static str>,pub enabled:bool,pub solid:bool,key:Vec<u8>,pub writes:u64}
impl Clipping{
    pub fn new(device:&wgpu::Device)->Self{let key=bytes(&ClipUniform::default());let buffer=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("current original clipping planes"),contents:&key,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST});let group=crate::gpu_layout::group(device,&layout(device),&buffer);Self{buffer,group,rows:Vec::new(),error:None,enabled:true,solid:true,key,writes:0}}
    pub fn set(&mut self,scene:&crate::scene::Scene){match crate::clipping_rows::current(scene){Ok(rows)=>{self.rows=rows;self.error=None},Err(error)=>{self.rows.clear();self.error=Some(error)}}}
    pub fn frame(&mut self,queue:&wgpu::Queue,matrix:[f32;16],height:u32,density:f32){let matrix=session_rust::Xform::from_matrix(matrix.map(f64::from));let planes=if self.enabled{self.rows.iter().map(|r|r.plane).collect::<Vec<_>>()}else{Vec::new()};let uniform=clip_uniform(&planes,u32::from(self.solid),&ClipView{view_proj:&matrix,anchor:[0.;3],height,pixel_scale:f64::from(density),samples:1,forward_depth:true});let bytes=bytes(&uniform);if bytes!=self.key{queue.write_buffer(&self.buffer,0,&bytes);self.key=bytes;self.writes+=1;}}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzz-share-the-object-record-shader-schema-06.rs"
```

```text
--8<-- "journey/code/96zzzz-share-the-object-record-shader-schema-07.rs"
```

</details>

### 7. `src/finite_project_source.rs`

Compute projection includes the same declaration before its packed geometry helper.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn source() -> String { format!("{}\n{}",include_str!("packed_vertex.wgsl"),expand(include_str!("finite_project.wgsl"))) }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn source() -> String { format!("{}\n{}\n{}",crate::object_record::shader(),include_str!("packed_vertex.wgsl"),expand(include_str!("finite_project.wgsl"))) }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzz-share-the-object-record-shader-schema-08.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 8. `src/object_record_tests.rs`

Verify that projection uses the same aligned members as both raster states after consolidating the flag vector.

<details>
<summary>Locate the existing block</summary>

```text
(crate::finite_project_source::source(), vec![0, 64, 68, 72, 80, 144, 160])
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzz-share-the-object-record-shader-schema-layout-fixture.rs"
```

</details>

</details>

<a id="checkpoint-96zzzza-prepare-shared-record-triangle-projection"></a>

### Step 9 of 16 · Prepare shared record triangle projection (`96zzzza-prepare-shared-record-triangle-projection`)

- **Goal:** Prepare the compute shader to select the same current object row.
- **Data Flow Trace:** `Current packed-record match → object row → prepared projection shader lookup`
- **Files You Touch:** `src/instance_gpu.rs` (*Use one packed-record match guard for raster and compute consumers.*) · `src/finite_project.wgsl` (*Use the shared object shader adaptation without duplicating the projection algorithm.*) · `src/finite_project_source.rs` (*Prepare the storage compute source through the existing record owner.*)

Projection needs the same model and arena range as color and picks. Expose the existing current-record lookup once, then let the projection shader select an object through the same shader adapter. Its interval already contains two spare unsigned words; reserve the third for the object row. This lesson prepares the storage compute source while ordinary projection keeps its existing uniform binding.

> **Self-Check Question:** *Why share the current-record lookup with projection?*  
> **Answer:** A changed mesh packet must use current placement and range information. Reusing the same guard prevents the compute pass from projecting a stale record.

- **What You Should See:** Projection has validated uniform and shared-record shader sources; the storage compute path is not connected yet.
- **Hands-On Lab Experiment:** Inspect both supplied shader-interface checks, then repeat placed-object clipping and original face picks. Projection still uses its earlier uniform resource at this checkpoint.

<details>
<summary>Implementation 96zzzza-prepare-shared-record-triangle-projection: Prepare shared record triangle projection</summary>

Prepare the compute shader to select the same current object row.

Data flow: Current packed-record match → object row → prepared projection shader lookup.

### 1. `src/instance_gpu.rs`

Use one packed-record match guard for raster and compute consumers.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn bind_record(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {
        let Some(&(first,row))=self.ordinary.get(&mesh.id)else{return false};
        if !self.records.matches(row,&mesh.packed()){return false;}
        self.records.bind(pass);self.slots.bind(pass,0,first);true
    }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96zzzza-prepare-shared-record-triangle-projection-01.rs"
```

### 2. `src/finite_project.wgsl`

Use the shared object shader adaptation without duplicating the projection algorithm. Read the selected row from the existing unsigned projection interval.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```wgsl
    depth: vec3<f32>, // screen gradient xy and depth at the viewport centre
};
fn physical_clip_corner(index:u32)->vec4<f32>{
 let point=packed_position(physical_vertices[physical_indices[index+object.range.x]]);
 let clip=view.transform*object.model*vec4<f32>(point,1.0);
 return vec4<f32>(clip.xy,clip.w-clip.z,clip.w);
```

</details>

Replace it with:

```wgsl
    depth: vec3<f32>, // screen gradient xy and depth at the viewport centre
};
fn physical_clip_corner(index:u32)->vec4<f32>{
 let object=object_at(live_count.z);
 let point=packed_position(physical_vertices[physical_indices[index+object.range.x]]);
 let clip=view.transform*object.model*vec4<f32>(point,1.0);
 return vec4<f32>(clip.xy,clip.w-clip.z,clip.w);
```

<details>
<summary>Locate this block</summary>

```wgsl
@group(1) @binding(0) var<uniform> line:TileLine;
@group(2) @binding(0) var<storage,read> physical_vertices:array<PackedVertex>;
@group(2) @binding(1) var<storage,read> physical_indices:array<u32>;
@group(2) @binding(2) var<uniform> object:ObjectSettings;
@group(2) @binding(3) var<storage,read_write> projected:array<ProjectedTriangle>;
@group(2) @binding(4) var<uniform> live_count:vec4<u32>;
#include "projected_triangle.wgsl"
```

</details>

Replace it with:

```wgsl
@group(1) @binding(0) var<uniform> line:TileLine;
@group(2) @binding(0) var<storage,read> physical_vertices:array<PackedVertex>;
@group(2) @binding(1) var<storage,read> physical_indices:array<u32>;
@group(2) @binding(2) var<uniform> object_settings: ObjectSettings;
fn object_at(index:u32)->ObjectSettings {return object_settings;}
@group(2) @binding(3) var<storage,read_write> projected:array<ProjectedTriangle>;
@group(2) @binding(4) var<uniform> live_count:vec4<u32>;
#include "projected_triangle.wgsl"
```

<details>
<summary>Complete updated block reference</summary>

```wgsl
struct View { transform:mat4x4<f32>, size:vec2<f32>, density:f32 }
struct TileLine {thickness:f32,proj_y:f32,ortho_h:f32,vp_h:f32,vp_w:f32}
@group(0) @binding(0) var<uniform> view:View;
@group(1) @binding(0) var<uniform> line:TileLine;
@group(2) @binding(0) var<storage,read> physical_vertices:array<PackedVertex>;
@group(2) @binding(1) var<storage,read> physical_indices:array<u32>;
@group(2) @binding(2) var<uniform> object_settings: ObjectSettings;
fn object_at(index:u32)->ObjectSettings {return object_settings;}
@group(2) @binding(3) var<storage,read_write> projected:array<ProjectedTriangle>;
@group(2) @binding(4) var<uniform> live_count:vec4<u32>;
#include "projected_triangle.wgsl"
struct ProjectedPolygon {
    points: array<vec3<f32>,
    4>,
    count: u32, // corners; 0 = not drawn
    depth: vec3<f32>, // screen gradient xy and depth at the viewport centre
};
fn physical_clip_corner(index:u32)->vec4<f32>{
 let object=object_at(live_count.z);
 let point=packed_position(physical_vertices[physical_indices[index+object.range.x]]);
 let clip=view.transform*object.model*vec4<f32>(point,1.0);
 return vec4<f32>(clip.xy,clip.w-clip.z,clip.w);
}

fn project_physical_triangle(primitive: u32, row: u32) -> ProjectedPolygon {
    var polygon: ProjectedPolygon;

    if (primitive == 0u || primitive > arrayLength(&physical_indices)/3u) {
        return polygon;
    }

    let base = (primitive-1u)*3u;

    let input = array<vec4<f32>, 3>(physical_clip_corner(base), physical_clip_corner(base+1u), physical_clip_corner(base+2u));
    // Homogeneous barycentric coordinates give z_ndc as an affine plane without
    // dividing corners by w. Its constant is the depth at ndc (0, 0).
    let edge0 = cross(input[1].xyw, input[2].xyw);
    let determinant = dot(edge0, input[0].xyw);

    if (determinant == 0.0) {
        return polygon;
    }

    let plane = (input[0].z * edge0 + input[1].z * cross(input[2].xyw, input[0].xyw)
        + input[2].z * cross(input[0].xyw, input[1].xyw)) / determinant;
    polygon.depth = vec3<f32>(2.0 * plane.x / line.vp_w, -2.0 * plane.y / line.vp_h, plane.z);
    var clipped: array<vec4<f32>, 4>;
    var previous = input[2];
    var previous_distance = previous.w - previous.z;

    // clip each edge against the near plane
    for (var i = 0u; i<3u; i++) {
        let current = input[i];
        let distance = current.w - current.z;

        if ((distance > 0.0 && previous_distance < 0.0) || (distance < 0.0 && previous_distance > 0.0)) {
            let t = previous_distance / (previous_distance-distance);
            clipped[polygon.count] = mix(previous, current, t);
            polygon.count++;
        }

        if (distance >= 0.0) {
            clipped[polygon.count] = current;
            polygon.count++;
        }

        previous = current;
        previous_distance = distance;
    }

    // clip space to screen pixels
    for (var i = 0u; i<polygon.count; i++) {
        let clip = clipped[i];

        if (clip.w <= 0.0) {
            polygon.count = 0u;
            return polygon;
        }

        let ndc = clip.xyz/clip.w;
        polygon.points[i] = vec3<f32>((ndc.x*0.5+0.5)*line.vp_w, (0.5-ndc.y*0.5)*line.vp_h, ndc.z);
    }

    return polygon;
}
fn physical_cross(a: vec2<f32>, b: vec2<f32>) -> f32 {
    return a.x*b.y-a.y*b.x;
}
fn physical_polygon_area(polygon: ProjectedPolygon) -> f32 {
    // zero only for a degenerate polygon
    return physical_cross(polygon.points[1].xy-polygon.points[0].xy, polygon.points[2].xy-polygon.points[0].xy);
}

@compute @workgroup_size(64)
fn cs_main(@builtin(global_invocation_id) id: vec3<u32>) {
    // rows of 32768 workgroups (PROJECT_ROW_GROUPS): a dispatch dimension holds at most 65535
    let local = id.x + id.y * 2097152u;
    let index = local + live_count.y;

    if (local>=live_count.x) {
        return;
    }

    let polygon = project_physical_triangle(local+1u, 0u);
    var out: ProjectedTriangle;

    // otherwise the record stays all zero
    if (polygon.count>=3u) {
        // screen box
        var lo = polygon.points[0].xy;
        var hi = lo;

        for (var i = 1u;i<polygon.count;i++) {
            lo = min(lo, polygon.points[i].xy);
            hi = max(hi, polygon.points[i].xy);
        }

        let area = physical_polygon_area(polygon);

        // degenerate: empty record
        if (abs(area)<1e-12) {
            projected[index] = out;
            return;
        }

        // one inward line equation per edge
        var equations: array<vec4<f32>, 4>;

        for (var i = 0u;i<polygon.count;i++) {
            let a = polygon.points[i].xy;
            let edge = polygon.points[(i+1u)%polygon.count].xy-a;
            let normal = sign(area)*vec2<f32>(-edge.y, edge.x)/length(edge);
            equations[i] = vec4<f32>(normal, -dot(normal, a), 0.0);
        }

        // Small far triangles need a nearby reference; extrapolating their homogeneous
        // plane to the viewport centre magnifies cancellation. Huge near-clipped faces
        // still use that centre, keeping their million-pixel corners out of the depth fit.
        var reference = vec3<f32>(line.vp_w * 0.5, line.vp_h * 0.5, polygon.depth.z);
        var gradient = polygon.depth.xy;
        let viewport = vec2<f32>(line.vp_w, line.vp_h);
        if (polygon.count == 3u && all(lo >= -viewport) && all(hi <= 2.0 * viewport)) {
            let ab = polygon.points[1] - polygon.points[0];
            let ac = polygon.points[2] - polygon.points[0];
            gradient = vec2<f32>(ab.z * ac.y - ac.z * ab.y, ab.x * ac.z - ac.x * ab.z) / area;
            reference = polygon.points[0];
        }
        out.edge0 = vec4<f32>(equations[0].xyz, reference.x);
        out.edge1 = vec4<f32>(equations[1].xyz, reference.y);
        out.edge2 = vec4<f32>(equations[2].xyz, reference.z);
        out.edge3 = vec4<f32>(equations[3].xyz, f32(polygon.count));
        // nearest corner depth
        var nearest = polygon.points[0].z;

        for (var i = 1u;i<polygon.count;i++) {
            nearest = max(nearest, polygon.points[i].z);
        }

        out.gradient = vec4<f32>(gradient, nearest, 0.0);
        out.bounds = vec4<f32>(lo, hi);
    }

    projected[index] = out;
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzza-prepare-shared-record-triangle-projection-02.rs"
```

```text
--8<-- "journey/code/96zzzza-prepare-shared-record-triangle-projection-03.rs"
```

</details>

### 3. `src/finite_project_source.rs`

Prepare the storage compute source through the existing record owner.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn records()->String {crate::object_rows::source(&source())}

#[cfg(test)]
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzza-prepare-shared-record-triangle-projection-04.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/finite_project_source.rs`

Supply validation of both uniform and storage compute interfaces.

<details>
<summary>Locate the existing block</summary>

```text
        let source = super::source();
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzza-prepare-shared-record-triangle-projection-05.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/finite_project_source.rs`

Both assembled shader sources must retain the existing compute entry.

<details>
<summary>Locate the existing block</summary>

```text
            && entry.stage == naga::ShaderStage::Compute));
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzza-prepare-shared-record-triangle-projection-06.rs" skip=59
```

</details>

</details>

<a id="checkpoint-96zzzzb-project-triangles-through-shared-records"></a>

### Step 10 of 16 · Project triangles through shared records (`96zzzzb-project-triangles-through-shared-records`)

- **Goal:** Connect finite triangle projection to the current shared object table.
- **Data Flow Trace:** `Current object row and arena interval → existing triangle projection → unchanged finite visibility`
- **Files You Touch:** `src/tile_pipelines.rs` (*Own a shared projection binding state beside the earlier fallback.*) · `src/tile_gpu.rs` (*Observe source arena offsets and stable row selectors as well as model and buffer identity.*) · `src/renderer.rs` (*Refresh projection after the object table, so its row selectors and replacement buffers are current.*) · `src/global_outline_tests.rs` (*Supply exact storage-versus-uniform projected bytes and color comparisons while retaining the original range, owner and release assertions.*)

The projection interval now names an object row in the same table used by color, picks and ambient properties. Select a storage compute state for current records and retain the earlier uniform state otherwise. Build both states from one layout recipe. Track arena offsets and row selectors in the existing projection witness, then refresh bindings after the object table updates.

> **Self-Check Question:** *Why must projection observe arena offsets and record row selectors?*  
> **Answer:** The same buffers can contain a newly placed range or a reused object row. Buffer identity alone cannot identify the correct source interval or record.

- **What You Should See:** Finite triangle projection consumes current shared records and retains exact earlier projected bytes and pixels.
- **Hands-On Lab Experiment:** Move and copy originals, select and restore them, then clip the scene. Inspect the exact uniform/storage projected-buffer comparison and original sparse-range checks.

<details>
<summary>Implementation 96zzzzb-project-triangles-through-shared-records: Project triangles through shared records</summary>

Connect finite triangle projection to the current shared object table.

Data flow: Current object row and arena interval → existing triangle projection → unchanged finite visibility.

### 1. `src/tile_pipelines.rs`

Own a shared projection binding state beside the earlier fallback. Keep both compute states with the existing projection owner. Build uniform and record projection states from one binding recipe and algorithm. Remove the replaced single-state constructor. Retain the shared projection state with the existing scans and bins.

<details>
<summary>Locate the existing block</summary>

```rust
fn compute(device:&wgpu::Device,source:&str,entry_point:&str,groups:&[Option<&wgpu::BindGroupLayout>])->wgpu::ComputePipeline{let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some(entry_point),source:wgpu::ShaderSource::Wgsl(source.into())});let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some(entry_point),bind_group_layouts:groups,immediate_size:0});device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor{label:Some(entry_point),layout:Some(&layout),module:&shader,entry_point:Some(entry_point),compilation_options:Default::default(),cache:None})}
fn raster(device:&wgpu::Device,shader:&wgpu::ShaderModule,layout:&wgpu::PipelineLayout,entry:&str)->wgpu::RenderPipeline{device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some(entry),layout:Some(&layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some("vs_main"),compilation_options:Default::default(),buffers:&[]},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some(entry),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::R8Unorm,blend:None,write_mask:wgpu::ColorWrites::empty()})]}),primitive:wgpu::PrimitiveState{topology:wgpu::PrimitiveTopology::TriangleStrip,..Default::default()},depth_stencil:None,multisample:Default::default(),multiview_mask:None,cache:None})}
use wgpu::{BufferBindingType as Type,ShaderStages as Stage};
pub fn entry(binding:u32,visibility:Stage,ty:Type)->wgpu::BindGroupLayoutEntry{wgpu::BindGroupLayoutEntry{binding,visibility,ty:wgpu::BindingType::Buffer{ty,has_dynamic_offset:false,min_binding_size:None},count:None}}
pub fn group(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,buffers:&[&wgpu::Buffer])->wgpu::BindGroup{device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("finite triangle buffers"),layout,entries:&buffers.iter().enumerate().map(|(binding,buffer)|wgpu::BindGroupEntry{binding:binding as u32,resource:buffer.as_entire_binding()}).collect::<Vec<_>>()})}
pub struct Pipelines{pub line:wgpu::BindGroupLayout,pub project_layout:wgpu::BindGroupLayout,pub raster_layout:wgpu::BindGroupLayout,pub scan_layout:wgpu::BindGroupLayout,pub project:wgpu::ComputePipeline,pub count:wgpu::RenderPipeline,pub fill:wgpu::RenderPipeline,pub scans:[wgpu::ComputePipeline;3]}
impl Pipelines{
 pub fn new(device:&wgpu::Device,view:&wgpu::BindGroupLayout)->Self{
  let layout=|label,entries:&[wgpu::BindGroupLayoutEntry]|device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some(label),entries});
  let line=layout("finite view dimensions",&[entry(0,Stage::VERTEX_FRAGMENT|Stage::COMPUTE,Type::Uniform)]);
  let project_layout=layout("finite original projection",&[entry(0,Stage::COMPUTE,Type::Storage{read_only:true}),entry(1,Stage::COMPUTE,Type::Storage{read_only:true}),entry(2,Stage::COMPUTE,Type::Uniform),entry(3,Stage::COMPUTE,Type::Storage{read_only:false}),entry(4,Stage::COMPUTE,Type::Uniform)]);
  let raster_layout=layout("finite triangle tiles",&[entry(0,Stage::VERTEX,Type::Storage{read_only:true}),entry(1,Stage::FRAGMENT,Type::Storage{read_only:false})]);
  let scan_layout=layout("finite prefix scan",&[entry(0,Stage::COMPUTE,Type::Storage{read_only:false})]);

  let project=compute(device,&crate::finite_project_source::source(),"cs_main",&[Some(view),Some(&line),Some(&project_layout)]);
  let scans=["scan_tiles","scan_blocks","finish_offsets"].map(|entry|compute(device,&crate::finite_project_source::scan(),entry,&[Some(&line),Some(&scan_layout)]));
  let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("finite bins"),source:wgpu::ShaderSource::Wgsl(crate::finite_project_source::tiles().into())});
  let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("finite bins"),bind_group_layouts:&[Some(&line),Some(&raster_layout)],immediate_size:0});

  let count=raster(device,&shader,&layout,"fs_count");let fill=raster(device,&shader,&layout,"fs_fill");Self{line,project_layout,raster_layout,scan_layout,project,count,fill,scans}
 }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
fn compute(device:&wgpu::Device,source:&str,entry_point:&str,groups:&[Option<&wgpu::BindGroupLayout>])->wgpu::ComputePipeline{let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some(entry_point),source:wgpu::ShaderSource::Wgsl(source.into())});let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some(entry_point),bind_group_layouts:groups,immediate_size:0});device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor{label:Some(entry_point),layout:Some(&layout),module:&shader,entry_point:Some(entry_point),compilation_options:Default::default(),cache:None})}
fn raster(device:&wgpu::Device,shader:&wgpu::ShaderModule,layout:&wgpu::PipelineLayout,entry:&str)->wgpu::RenderPipeline{device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some(entry),layout:Some(&layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some("vs_main"),compilation_options:Default::default(),buffers:&[]},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some(entry),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::R8Unorm,blend:None,write_mask:wgpu::ColorWrites::empty()})]}),primitive:wgpu::PrimitiveState{topology:wgpu::PrimitiveTopology::TriangleStrip,..Default::default()},depth_stencil:None,multisample:Default::default(),multiview_mask:None,cache:None})}
use wgpu::{BufferBindingType as Type,ShaderStages as Stage};
pub fn entry(binding:u32,visibility:Stage,ty:Type)->wgpu::BindGroupLayoutEntry{wgpu::BindGroupLayoutEntry{binding,visibility,ty:wgpu::BindingType::Buffer{ty,has_dynamic_offset:false,min_binding_size:None},count:None}}
pub fn group(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,buffers:&[&wgpu::Buffer])->wgpu::BindGroup{device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("finite triangle buffers"),layout,entries:&buffers.iter().enumerate().map(|(binding,buffer)|wgpu::BindGroupEntry{binding:binding as u32,resource:buffer.as_entire_binding()}).collect::<Vec<_>>()})}
pub struct Pipelines{pub line:wgpu::BindGroupLayout,pub project_layout:wgpu::BindGroupLayout,pub record_project_layout:wgpu::BindGroupLayout,pub raster_layout:wgpu::BindGroupLayout,pub scan_layout:wgpu::BindGroupLayout,pub project:wgpu::ComputePipeline,pub record_project:wgpu::ComputePipeline,pub count:wgpu::RenderPipeline,pub fill:wgpu::RenderPipeline,pub scans:[wgpu::ComputePipeline;3]}
impl Pipelines{
 pub fn new(device:&wgpu::Device,view:&wgpu::BindGroupLayout)->Self{
  let layout=|label,entries:&[wgpu::BindGroupLayoutEntry]|device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some(label),entries});
  let line=layout("finite view dimensions",&[entry(0,Stage::VERTEX_FRAGMENT|Stage::COMPUTE,Type::Uniform)]);
  let create=|shared| {let project_layout=layout("finite original projection",&[entry(0,Stage::COMPUTE,Type::Storage{read_only:true}),entry(1,Stage::COMPUTE,Type::Storage{read_only:true}),entry(2,Stage::COMPUTE,if shared {Type::Storage{read_only:true}}else{Type::Uniform}),entry(3,Stage::COMPUTE,Type::Storage{read_only:false}),entry(4,Stage::COMPUTE,Type::Uniform)]);
   let source=if shared {crate::finite_project_source::records()}else{crate::finite_project_source::source()};let project=compute(device,&source,"cs_main",&[Some(view),Some(&line),Some(&project_layout)]);(project_layout,project)};
  let(project_layout,project)=create(false);let(record_project_layout,record_project)=create(true);
  let raster_layout=layout("finite triangle tiles",&[entry(0,Stage::VERTEX,Type::Storage{read_only:true}),entry(1,Stage::FRAGMENT,Type::Storage{read_only:false})]);
  let scan_layout=layout("finite prefix scan",&[entry(0,Stage::COMPUTE,Type::Storage{read_only:false})]);

  let scans=["scan_tiles","scan_blocks","finish_offsets"].map(|entry|compute(device,&crate::finite_project_source::scan(),entry,&[Some(&line),Some(&scan_layout)]));
  let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("finite bins"),source:wgpu::ShaderSource::Wgsl(crate::finite_project_source::tiles().into())});
  let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("finite bins"),bind_group_layouts:&[Some(&line),Some(&raster_layout)],immediate_size:0});

  let count=raster(device,&shader,&layout,"fs_count");let fill=raster(device,&shader,&layout,"fs_fill");Self{line,project_layout,record_project_layout,raster_layout,scan_layout,project,record_project,count,fill,scans}
 }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzb-project-triangles-through-shared-records-01.rs"
```

```text
--8<-- "journey/code/96zzzzb-project-triangles-through-shared-records-02.rs"
```

```text
--8<-- "journey/code/96zzzzb-project-triangles-through-shared-records-03.rs"
```

```text
--8<-- "journey/code/96zzzzb-project-triangles-through-shared-records-05.rs"
```

</details>

### 2. `src/tile_gpu.rs`

Observe source arena offsets and stable row selectors as well as model and buffer identity. Each projection row selects its current resource contract. Preserve isolated original fixtures while connecting the renderer’s record owner. Use current table rows or the exact earlier model buffer. Invalidate moved arena intervals and changed record selectors without reprojecting selection-only changes. Store the current object row in the existing sixteen-byte interval. Bind the selected model resource without a second table. Use the matching uniform or storage layout for the cached projection group. Dispatch the same projection algorithm through the selected resource state.

<details>
<summary>Locate the existing block</summary>

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

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{gpu_mesh::GpuMesh,tile_math::{TileLayout,PROJECTED_BYTES,projected_records,next_pool_words,project_groups},tile_pipelines::{Pipelines,group},tile_report::PoolReport};
use wgpu::util::DeviceExt;
type Witness=(wgpu::Buffer,wgpu::Buffer,wgpu::Buffer,[u8;64],u32,u32,u32,Option<u32>);
struct Row{vertices:wgpu::Buffer,indices:wgpu::Buffer,model:wgpu::Buffer,count:wgpu::Buffer,triangles:u32,shared:bool,group:Option<wgpu::BindGroup>}
pub struct Tiles{pub projected:wgpu::Buffer,pub buffer:wgpu::Buffer,pipes:Pipelines,line:wgpu::Buffer,line_group:wgpu::BindGroup,raster:wgpu::BindGroup,scan:wgpu::BindGroup,rows:Vec<Row>,witness:Vec<Witness>,triangles:u32,viewport:[u32;2],layout:Option<TileLayout>,target:Option<wgpu::TextureView>,pool:u64,report:PoolReport,key:Option<[f32;16]>,pub projections:u64,pub bins:u64,reported:Option<u64>,pub fallback:bool}
fn buffer(device:&wgpu::Device,label:&str,size:u64)->wgpu::Buffer{device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::COPY_SRC,mapped_at_creation:false})}
impl Tiles{
 pub fn new(device:&wgpu::Device,view:&wgpu::BindGroupLayout)->Self{
  let pipes=Pipelines::new(device,view);let projected=buffer(device,"finite projected placeholder",PROJECTED_BYTES);let buffer=buffer(device,"finite tile placeholder",16);
  let line=device.create_buffer(&wgpu::BufferDescriptor{label:Some("finite view dimensions"),size:32,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});
  let line_group=group(device,&pipes.line,&[&line]);let raster=group(device,&pipes.raster_layout,&[&projected,&buffer]);let scan=group(device,&pipes.scan_layout,&[&buffer]);
  Self{projected,buffer,pipes,line,line_group,raster,scan,rows:Vec::new(),witness:Vec::new(),triangles:0,viewport:[0;2],layout:None,target:None,pool:0,report:PoolReport::new(device),key:None,projections:0,bins:0,reported:None,fallback:false}
 }
 pub fn set(&mut self,device:&wgpu::Device,meshes:&[GpuMesh]){self.set_records(device,meshes,None)}
 pub fn set_records(&mut self,device:&wgpu::Device,meshes:&[GpuMesh],records:Option<&crate::instance_gpu::Lane>){
  let limit=u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size);
  let count=meshes.iter().filter(|row|row.geometry.index_count>0).try_fold(0u32,|end,row|row.primitive_base().checked_add(row.geometry.index_count/3).map(|next|end.max(next)));
  self.fallback=count.is_none_or(|count|u64::from(count)*PROJECTED_BYTES>limit);
  if self.fallback{self.rows.clear();self.witness.clear();self.triangles=0;self.key=None;return}
  let current=|mesh:&GpuMesh|records.and_then(|rows|rows.current_row(mesh)).map(|(_,row)|row);
  let model=|mesh:&GpuMesh|if current(mesh).is_some(){records.unwrap().record_buffer().clone()}else{mesh.model_buffer().clone()};
  let witness=meshes.iter().filter(|row|row.geometry.index_count>0).map(|row|{let(vertices,indices)=row.geometry.buffers();(vertices,indices,model(row),row.packed()[..64].try_into().unwrap(),row.primitive_base(),row.geometry.index_start,row.geometry.index_count,current(row))}).collect::<Vec<_>>();
  if witness==self.witness{return}
  self.rows.clear();
  for row in meshes.iter().filter(|row|row.geometry.index_count>0){let triangles=row.geometry.index_count/3;
   let bytes=[triangles,row.primitive_base(),current(row).unwrap_or(0),0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();
   let count=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("finite original triangle interval"),contents:&bytes,usage:wgpu::BufferUsages::UNIFORM});
   self.rows.push(Row{vertices:row.geometry.buffers().0,indices:row.geometry.buffers().1,model:model(row),count,triangles,shared:current(row).is_some(),group:None});
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
  for row in &mut self.rows{let bindings=row.group.get_or_insert_with(||group(device,if row.shared {&self.pipes.record_project_layout}else{&self.pipes.project_layout},&[&row.vertices,&row.indices,&row.model,&self.projected,&row.count]));let mut pass=encoder.begin_compute_pass(&wgpu::ComputePassDescriptor{label:Some("finite original projection"),timestamp_writes:None});pass.set_pipeline(if row.shared {&self.pipes.record_project}else{&self.pipes.project});pass.set_bind_group(0,view,&[]);pass.set_bind_group(1,&self.line_group,&[]);pass.set_bind_group(2,&*bindings,&[]);let(x,y)=project_groups(row.triangles);pass.dispatch_workgroups(x,y,1);}
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
--8<-- "journey/code/96zzzzb-project-triangles-through-shared-records-06.rs"
```

```text
--8<-- "journey/code/96zzzzb-project-triangles-through-shared-records-07.rs"
```

```text
--8<-- "journey/code/96zzzzb-project-triangles-through-shared-records-08.rs"
```

```text
--8<-- "journey/code/96zzzzb-project-triangles-through-shared-records-09.rs"
```

```text
--8<-- "journey/code/96zzzzb-project-triangles-through-shared-records-10.rs"
```

```text
--8<-- "journey/code/96zzzzb-project-triangles-through-shared-records-11.rs"
```

```text
--8<-- "journey/code/96zzzzb-project-triangles-through-shared-records-12.rs"
```

```text
--8<-- "journey/code/96zzzzb-project-triangles-through-shared-records-13.rs"
```

```text
--8<-- "journey/code/96zzzzb-project-triangles-through-shared-records-14.rs"
```

</details>

### 3. `src/renderer.rs`

Refresh projection after the object table, so its row selectors and replacement buffers are current. Use the renderer’s current table for finite projection.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            mesh.set_primitive_base(&self.queue,base).expect("Bounded projected triangle base");
            base=base.checked_add(mesh.geometry.index_count/3).filter(|n|*n<crate::primitive_image::CAP_START).expect("Bounded projected triangle range");
        }
        self.tiles.borrow_mut().set(&self.device,&self.meshes);
        let radii=self.ambient_rows.specs.iter().copied().collect::<std::collections::HashMap<_,_>>();for mesh in &mut self.meshes {mesh.set_ambient_radius(&self.queue,radii.get(&mesh.id.value()).copied().unwrap_or(0.));}
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
        self.instances.set(&self.device,&self.queue,self.meshes.iter().filter(|row|self.instance_rows.contains(&row.id)),self.meshes.iter().filter(|row|!self.instance_rows.contains(&row.id))).expect("Validated instance records fit device limits");
        self.sheets.set(&self.device,&self.queue,scene,selected.as_slice(),&self.sources.sheet_bases).expect("Prepared original sheet batches fit device limits");
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
```

</details>

Replace it with:

```rust
            mesh.set_primitive_base(&self.queue,base).expect("Bounded projected triangle base");
            base=base.checked_add(mesh.geometry.index_count/3).filter(|n|*n<crate::primitive_image::CAP_START).expect("Bounded projected triangle range");
        }
        let radii=self.ambient_rows.specs.iter().copied().collect::<std::collections::HashMap<_,_>>();for mesh in &mut self.meshes {mesh.set_ambient_radius(&self.queue,radii.get(&mesh.id.value()).copied().unwrap_or(0.));}
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
        self.instances.set(&self.device,&self.queue,self.meshes.iter().filter(|row|self.instance_rows.contains(&row.id)),self.meshes.iter().filter(|row|!self.instance_rows.contains(&row.id))).expect("Validated instance records fit device limits");
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
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
        let radii=self.ambient_rows.specs.iter().copied().collect::<std::collections::HashMap<_,_>>();for mesh in &mut self.meshes {mesh.set_ambient_radius(&self.queue,radii.get(&mesh.id.value()).copied().unwrap_or(0.));}
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
        self.instances.set(&self.device,&self.queue,self.meshes.iter().filter(|row|self.instance_rows.contains(&row.id)),self.meshes.iter().filter(|row|!self.instance_rows.contains(&row.id))).expect("Validated instance records fit device limits");
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
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
            &self.meshes, &[&self.mesh_ink, &self.strokes, &self.paths, &self.markers], &self.faces,self.text.borrow().as_ref(),self.clouds.borrow().as_ref(),Some(&self.sheets),Some(&tables),&clipping.group,&clipping.buffer,Some(&mut self.caps.borrow_mut()),Some(&self.instances)); }
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
            for mesh in self.meshes.iter().filter(|row|self.opacity.get()<1.||!self.instance_rows.contains(&row.id)) {if !self.instances.draw_ordinary(&mut pass,mesh){pass.set_pipeline(&self.pipeline);self.instances.bind_own(&mut pass);mesh.draw_pulled(&mut pass);}}
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
        if self.ambient_enabled.get(){let clip=self.clipping.borrow();if let Some(ambient)=self.ambient.borrow_mut().as_mut(){ambient.draw(&self.device,&self.queue,&mut encoder,view,&self.depth,&self.view_group,&clip_group,&clip.buffer,&self.primitive.view,&self.meshes,*transform,self.size,self.density,self.ambient_rows.ground,[self.pyramid_revision,clip_revision],Some(&self.instances));}}
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
--8<-- "journey/code/96zzzzb-project-triangles-through-shared-records-16.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/global_outline_tests.rs`

Supply exact storage-versus-uniform projected bytes and color comparisons while retaining the original range, owner and release assertions.

<details>
<summary>Locate the existing block</summary>

```text
 scene.clear();gpu.set_scene(&scene,None);assert!(gpu.instances.ordinary.is_empty());
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzb-project-triangles-through-shared-records-17.rs"
```

</details>

</details>

<a id="checkpoint-96zzzzc-prepare-shared-record-cap-counting"></a>

### Step 11 of 16 · Prepare shared record cap counting (`96zzzzc-prepare-shared-record-cap-counting`)

- **Goal:** Separate cap crossing geometry from the object resource and vertex format.
- **Data Flow Trace:** `Selected object and plane → existing crossing function → unchanged cap color and owner`
- **Files You Touch:** `src/cap_count.wgsl` (*Use the existing object shader adapter for cap crossing entries.*) · `src/cap_pipeline.rs` (*Reuse the existing pipeline descriptor with an explicit vertex-buffer format.*)

Cap color and cap picking use the same closed-solid crossing function. Pass the selected object into that function and reuse the existing shader adapter. Keep one render-pipeline descriptor while allowing each consumer to specify its vertex format. Ordinary crossings retain their earlier object and plane behavior; the shared entry connects in the following lessons.

> **Self-Check Question:** *Why separate the vertex format from the cap count depth policy?*  
> **Answer:** A pulled shared-record entry uses different vertex inputs, but its crossing target still follows the same count pass. Keeping those choices separate avoids another pipeline descriptor.

- **What You Should See:** Ordinary cap counting retains its earlier behavior with shared object selection and pipeline inputs prepared.
- **Hands-On Lab Experiment:** Clip an ordinary and copied solid, pick its cap, then select and restore it. Inspect the original multi-plane, mirrored and cropped-owner GPU fixtures.

<details>
<summary>Implementation 96zzzzc-prepare-shared-record-cap-counting: Prepare shared record cap counting</summary>

Separate cap crossing geometry from the object resource and vertex format.

Data flow: Selected object and plane → existing crossing function → unchanged cap color and owner.

### 1. `src/cap_count.wgsl`

Use the existing object shader adapter for cap crossing entries. Share the closed-solid crossing algorithm across object resource paths. Keep vs_count on its current object and plane path. Keep vs_pick_count on its current object and plane path.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```wgsl
}

@group(3) @binding(1) var<uniform> cap_crop:Crop;
@vertex fn vs_pick_count(@location(0) position:vec3<f32>,@builtin(instance_index) plane:u32)->CountOut{var output=count_corner(position,plane);output.pos=cap_crop.transform*output.pos;return output;}
```

</details>

Replace it with:

```wgsl
}

@group(3) @binding(1) var<uniform> cap_crop:Crop;
@vertex fn vs_pick_count(@location(0) position:vec3<f32>,@builtin(instance_index) plane:u32)->CountOut{var output=count_corner(position,plane,object_at(0u));output.pos=cap_crop.transform*output.pos;return output;}
```

<details>
<summary>Locate this block</summary>

```wgsl
    @location(4) @interpolate(flat) selected: u32, // nonzero when the solid is selected
}

fn count_corner(position:vec3<f32>,plane:u32)->CountOut{
 var out:CountOut;out.pos=vec4<f32>(3.0,3.0,0.5,1.0);
 if((object.selection.z&(FLAG_CLOSED|FLAG_CLIPPING_PLANE))!=FLAG_CLOSED){return out;}
 let world=object.model*vec4<f32>(position,1.0);let mirrored=dot(object.model[0].xyz,cross(object.model[1].xyz,object.model[2].xyz))<0.0;
 out.pos=view.transform*world;out.world_pos=world.xyz;out.inst_id=object.selection.w;out.plane=plane;out.flip=select(0u,1u,mirrored!=((object.selection.z&FLAG_INWARD)!=0u));out.selected=object.selection.x&1u;return out;
}
@vertex fn vs_count(@location(0) position:vec3<f32>,@builtin(instance_index) plane:u32)->CountOut{return count_corner(position,plane);}
fn behind(plane: u32, p: vec3<f32>) -> f32 {
    return -clip_eye_side(plane) * clip_distance(plane, p);
}
```

</details>

Replace it with:

```wgsl
    @location(4) @interpolate(flat) selected: u32, // nonzero when the solid is selected
}

fn count_corner(position:vec3<f32>,plane:u32,object:ObjectSettings)->CountOut{
 var out:CountOut;out.pos=vec4<f32>(3.0,3.0,0.5,1.0);
 if((object.selection.z&(FLAG_CLOSED|FLAG_CLIPPING_PLANE))!=FLAG_CLOSED){return out;}
 let world=object.model*vec4<f32>(position,1.0);let mirrored=dot(object.model[0].xyz,cross(object.model[1].xyz,object.model[2].xyz))<0.0;
 out.pos=view.transform*world;out.world_pos=world.xyz;out.inst_id=object.selection.w;out.plane=plane;out.flip=select(0u,1u,mirrored!=((object.selection.z&FLAG_INWARD)!=0u));out.selected=object.selection.x&1u;return out;
}
@vertex fn vs_count(@location(0) position:vec3<f32>,@builtin(instance_index) plane:u32)->CountOut{return count_corner(position,plane,object_at(0u));}
fn behind(plane: u32, p: vec3<f32>) -> f32 {
    return -clip_eye_side(plane) * clip_distance(plane, p);
}
```

<details>
<summary>Locate this block</summary>

```wgsl
struct ViewSettings { transform: mat4x4<f32>, display: vec4<f32>, }
@group(0) @binding(0) var<uniform> view: ViewSettings;
@group(1) @binding(0) var<uniform> object: ObjectSettings;

struct CountOut {
    @builtin(position) pos: vec4<f32>, // clip position
```

</details>

Replace it with:

```wgsl
struct ViewSettings { transform: mat4x4<f32>, display: vec4<f32>, }
@group(0) @binding(0) var<uniform> view: ViewSettings;
@group(1) @binding(0) var<uniform> object_settings: ObjectSettings;
fn object_at(index:u32)->ObjectSettings {return object_settings;}

struct CountOut {
    @builtin(position) pos: vec4<f32>, // clip position
```

<details>
<summary>Complete updated block reference</summary>

```wgsl
struct ViewSettings { transform: mat4x4<f32>, display: vec4<f32>, }
@group(0) @binding(0) var<uniform> view: ViewSettings;
@group(1) @binding(0) var<uniform> object_settings: ObjectSettings;
fn object_at(index:u32)->ObjectSettings {return object_settings;}

struct CountOut {
    @builtin(position) pos: vec4<f32>, // clip position
    @location(0) world_pos: vec3<f32>, // scene-space position
    @location(1) @interpolate(flat) inst_id: u32, // object row
    @location(2) @interpolate(flat) plane: u32, // clipping plane
    @location(3) @interpolate(flat) flip: u32, // 1 when the solid is mirrored or wound inward
    @location(4) @interpolate(flat) selected: u32, // nonzero when the solid is selected
}

fn count_corner(position:vec3<f32>,plane:u32,object:ObjectSettings)->CountOut{
 var out:CountOut;out.pos=vec4<f32>(3.0,3.0,0.5,1.0);
 if((object.selection.z&(FLAG_CLOSED|FLAG_CLIPPING_PLANE))!=FLAG_CLOSED){return out;}
 let world=object.model*vec4<f32>(position,1.0);let mirrored=dot(object.model[0].xyz,cross(object.model[1].xyz,object.model[2].xyz))<0.0;
 out.pos=view.transform*world;out.world_pos=world.xyz;out.inst_id=object.selection.w;out.plane=plane;out.flip=select(0u,1u,mirrored!=((object.selection.z&FLAG_INWARD)!=0u));out.selected=object.selection.x&1u;return out;
}
@vertex fn vs_count(@location(0) position:vec3<f32>,@builtin(instance_index) plane:u32)->CountOut{return count_corner(position,plane,object_at(0u));}
fn behind(plane: u32, p: vec3<f32>) -> f32 {
    return -clip_eye_side(plane) * clip_distance(plane, p);
}
fn leaving(in: CountOut, front: bool) -> bool {
    return front == (in.flip != 0u);
}
@fragment fn fs_count(in:CountOut,@builtin(front_facing) front:bool)->@location(0) vec4<f32>{if(clip_eye_side(in.plane)==0.0||behind(in.plane,in.world_pos)<0.0){discard;}let sign=select(-1.0,1.0,leaving(in,front));let weight=select(1.0,SELECTED_CROSSING,in.selected!=0u);return vec4<f32>(sign*weight,0.0,0.0,0.0);}
@group(3) @binding(0) var<storage,read_write> pick_caps:array<atomic<u32>>;
struct PickFrame{full:vec2<u32>,size:vec2<u32>,origin:vec2<u32>,mode:u32,pad:u32}
@group(3) @binding(2) var<uniform> pick_frame:PickFrame;
fn pick_cap_word(plane: u32, px: vec2<f32>) -> u32 {
    return clip_pick_word(plane, px, pick_frame.size);
}
@fragment fn fs_pick_count(in: CountOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    if (clip_eye_side(in.plane) == 0.0 || behind(in.plane, in.world_pos) < 0.0) {
        discard;
    }

    let leaves = leaving(in, front);
    let sign = select(-1, 1, leaves);
    let word = pick_cap_word(in.plane, in.pos.xy);
    atomicAdd(&pick_caps[word], bitcast<u32>(sign));
    atomicAdd(&pick_caps[word + 1u], bitcast<u32>(sign * i32(in.inst_id)));

    if (leaves) {
        atomicMax(&pick_caps[word + 2u], bitcast<u32>(1.0-in.pos.z));
    }

    return vec4<f32>(0.0);
}
@fragment
fn fs_pick_owner(in: CountOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    if (clip_eye_side(in.plane) == 0.0 || behind(in.plane, in.world_pos) < 0.0 || !leaving(in, front)) {
        discard;
    }

    let word = pick_cap_word(in.plane, in.pos.xy);

    if (bitcast<u32>(1.0-in.pos.z) == atomicLoad(&pick_caps[word + 2u])) {
        atomicStore(&pick_caps[word + 3u], in.inst_id);
    }

    return vec4<f32>(0.0);
}

struct Crop { transform: mat4x4<f32>, mode: u32, _a: u32, _b: u32, _c: u32 }
fn wanted(pixel: u32, mode: u32) -> bool {
    if (mode == 0u) { return true; }
    if (pixel == 0u) { return false; }
    let kind = (pixel - 1u) & 0xe0000000u;
    return (mode == 2u && kind == 0x80000000u) || (mode == 3u && kind == 0x40000000u);
}

@group(3) @binding(1) var<uniform> cap_crop:Crop;
@vertex fn vs_pick_count(@location(0) position:vec3<f32>,@builtin(instance_index) plane:u32)->CountOut{var output=count_corner(position,plane,object_at(0u));output.pos=cap_crop.transform*output.pos;return output;}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzc-prepare-shared-record-cap-counting-01.rs"
```

```text
--8<-- "journey/code/96zzzzc-prepare-shared-record-cap-counting-02.rs"
```

```text
--8<-- "journey/code/96zzzzc-prepare-shared-record-cap-counting-03.rs"
```

```text
--8<-- "journey/code/96zzzzc-prepare-shared-record-cap-counting-04.rs"
```

</details>

### 2. `src/cap_pipeline.rs`

Reuse the existing pipeline descriptor with an explicit vertex-buffer format. Remove the repeated format owner from the shared descriptor. Use the selected format without changing targets, blending or depth policy.

<details>
<summary>Locate the existing block</summary>

```rust
pub(crate) fn pipeline(device:&wgpu::Device,source:String,groups:&[&wgpu::BindGroupLayout],vertex:&str,fragment:&str,targets:&[wgpu::ColorTargetState],count:bool)->wgpu::RenderPipeline{
 let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("original closed-solid sections"),source:wgpu::ShaderSource::Wgsl(source.into())});let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("original closed-solid sections"),bind_group_layouts:&groups.iter().map(|g|Some(*g)).collect::<Vec<_>>(),immediate_size:0});let attributes=wgpu::vertex_attr_array![0=>Float32x3];let buffers=if count{vec![wgpu::VertexBufferLayout{array_stride:20,step_mode:wgpu::VertexStepMode::Vertex,attributes:&attributes}]}else{Vec::new()};
 device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("original closed-solid sections"),layout:Some(&layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some(vertex),compilation_options:Default::default(),buffers:&buffers},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some(fragment),compilation_options:Default::default(),targets:&targets.iter().cloned().map(Some).collect::<Vec<_>>()}),primitive:Default::default(),depth_stencil:if count{None}else{Some(crate::depth::surface())},multisample:Default::default(),multiview_mask:None,cache:None})
}
pub struct Pipelines{pub count:wgpu::RenderPipeline,pub color:wgpu::RenderPipeline,pub counts:wgpu::BindGroupLayout,pub frame:wgpu::BindGroupLayout}
impl Pipelines{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{
  let view=crate::gpu_layout::uniform(device,80);let model=crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64);let clip=crate::clipping_gpu::layout(device);let frame=crate::gpu_layout::uniform(device,16);let counts=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("section crossing texture"),entries:&[crate::text_gpu_support::texture_entry(0,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Float{filterable:false},false)]});
  let add=wgpu::BlendComponent{src_factor:wgpu::BlendFactor::One,dst_factor:wgpu::BlendFactor::One,operation:wgpu::BlendOperation::Add};let count=pipeline(device,crate::clipping_gpu::source(include_str!("cap_count.wgsl"),2,0),&[&view,&model,&clip],"vs_count","fs_count",&[wgpu::ColorTargetState{format:wgpu::TextureFormat::R16Float,blend:Some(wgpu::BlendState{color:add,alpha:add}),write_mask:wgpu::ColorWrites::RED}],true);
  let color=pipeline(device,crate::clipping_gpu::source(include_str!("cap_color.wgsl"),1,0),&[&frame,&clip,&counts],"vs_cap","fs_cap",&[wgpu::ColorTargetState{format,blend:None,write_mask:wgpu::ColorWrites::ALL},crate::primitive_image::target(true)],false);Self{count,color,counts,frame}
 }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub(crate) fn pipeline(device:&wgpu::Device,source:String,groups:&[&wgpu::BindGroupLayout],vertex:&str,fragment:&str,targets:&[wgpu::ColorTargetState],count:bool)->wgpu::RenderPipeline{
 let attributes=wgpu::vertex_attr_array![0=>Float32x3];let buffers=if count{vec![wgpu::VertexBufferLayout{array_stride:20,step_mode:wgpu::VertexStepMode::Vertex,attributes:&attributes}]}else{Vec::new()};
 pipeline_buffers(device,source,groups,vertex,fragment,targets,&buffers,count)
}
pub(crate) fn pipeline_buffers(device:&wgpu::Device,source:String,groups:&[&wgpu::BindGroupLayout],vertex:&str,fragment:&str,targets:&[wgpu::ColorTargetState],buffers:&[wgpu::VertexBufferLayout<'_>],count:bool)->wgpu::RenderPipeline{
 let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("original closed-solid sections"),source:wgpu::ShaderSource::Wgsl(source.into())});let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("original closed-solid sections"),bind_group_layouts:&groups.iter().map(|g|Some(*g)).collect::<Vec<_>>(),immediate_size:0});
 device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("original closed-solid sections"),layout:Some(&layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some(vertex),compilation_options:Default::default(),buffers},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some(fragment),compilation_options:Default::default(),targets:&targets.iter().cloned().map(Some).collect::<Vec<_>>()}),primitive:Default::default(),depth_stencil:if count{None}else{Some(crate::depth::surface())},multisample:Default::default(),multiview_mask:None,cache:None})
}
pub struct Pipelines{pub count:wgpu::RenderPipeline,pub color:wgpu::RenderPipeline,pub counts:wgpu::BindGroupLayout,pub frame:wgpu::BindGroupLayout}
impl Pipelines{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{
  let view=crate::gpu_layout::uniform(device,80);let model=crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64);let clip=crate::clipping_gpu::layout(device);let frame=crate::gpu_layout::uniform(device,16);let counts=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("section crossing texture"),entries:&[crate::text_gpu_support::texture_entry(0,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Float{filterable:false},false)]});
  let add=wgpu::BlendComponent{src_factor:wgpu::BlendFactor::One,dst_factor:wgpu::BlendFactor::One,operation:wgpu::BlendOperation::Add};let count=pipeline(device,crate::clipping_gpu::source(include_str!("cap_count.wgsl"),2,0),&[&view,&model,&clip],"vs_count","fs_count",&[wgpu::ColorTargetState{format:wgpu::TextureFormat::R16Float,blend:Some(wgpu::BlendState{color:add,alpha:add}),write_mask:wgpu::ColorWrites::RED}],true);
  let color=pipeline(device,crate::clipping_gpu::source(include_str!("cap_color.wgsl"),1,0),&[&frame,&clip,&counts],"vs_cap","fs_cap",&[wgpu::ColorTargetState{format,blend:None,write_mask:wgpu::ColorWrites::ALL},crate::primitive_image::target(true)],false);Self{count,color,counts,frame}
 }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzc-prepare-shared-record-cap-counting-05.rs"
```

```text
--8<-- "journey/code/96zzzzc-prepare-shared-record-cap-counting-06.rs"
```

```text
--8<-- "journey/code/96zzzzc-prepare-shared-record-cap-counting-07.rs"
```

</details>

</details>

<a id="checkpoint-96zzzzd-prepare-cap-plane-and-arena-bindings"></a>

### Step 12 of 16 · Prepare cap plane and arena bindings (`96zzzzd-prepare-cap-plane-and-arena-bindings`)

- **Goal:** Prepare explicit plane parameters with current immutable arena bindings.
- **Data Flow Trace:** `Explicit plane plus current arena buffers → reusable cap geometry group`
- **Files You Touch:** `src/cap_geometry.rs` (*Keep explicit plane parameters and arena binding replacement in one resource owner.*) · `src/lib.rs` (*Declare the resource without connecting it to drawing yet.*)

A shared object slot selects a surface; the cap count also needs a plane. Keep those two inputs independent. Prepare a sixteen-byte plane parameter and a geometry group that reads the current arena buffers. Cache groups while those buffers remain identical, replace them after arena growth, and release the cache when inactive. This resource connects to cap color in the next lesson.

> **Self-Check Question:** *Why keep the plane independent of the slot instance index?*  
> **Answer:** The slot attribute selects the object record. Reusing its instance index for a plane would fetch a different slot. An explicit plane parameter preserves both identities.

- **What You Should See:** Plane parameters and arena groups are prepared; cap drawing still uses its earlier path.
- **Hands-On Lab Experiment:** Inspect the mapped plane, retained binding, grown arena and release fixture. Repeat existing cap clipping and picking in the browser; the new group is not connected yet.

<details>
<summary>Implementation 96zzzzd-prepare-cap-plane-and-arena-bindings: Prepare cap plane and arena bindings</summary>

Prepare explicit plane parameters with current immutable arena bindings.

Data flow: Explicit plane plus current arena buffers → reusable cap geometry group.

### 1. `src/cap_geometry.rs`

Keep explicit plane parameters and arena binding replacement in one resource owner.

Create the file and type:

```rust
--8<-- "journey/code/96zzzzd-prepare-cap-plane-and-arena-bindings-01.rs"
```

### 2. `src/lib.rs`

Declare the resource without connecting it to drawing yet.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod cap_pipeline;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod cap_geometry;
mod cap_pipeline;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzd-prepare-cap-plane-and-arena-bindings-02.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/cap_geometry.rs`

Supply actual mapped plane values, exact binding reuse/replacement through arena growth, bounded refusal and full release.

<details>
<summary>Locate the existing block</summary>

```text
use std::{collections::HashMap,rc::Rc};
use wgpu::util::DeviceExt;
#[derive(Default)]
pub struct Geometry {planes:Vec<wgpu::Buffer>,groups:HashMap<(usize,u32),(wgpu::Buffer,wgpu::Buffer,wgpu::BindGroup)>}
pub fn layout(device:&wgpu::Device)->wgpu::BindGroupLayout {
 use wgpu::{ShaderStages as S,BufferBindingType as B};device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("cap plane and arena"),entries:&[crate::tile_pipelines::entry(0,S::VERTEX,B::Storage{read_only:true}),crate::tile_pipelines::entry(1,S::VERTEX,B::Uniform),crate::tile_pipelines::entry(2,S::VERTEX,B::Storage{read_only:true})]})
}
impl Geometry {
 pub fn set_planes(&mut self,device:&wgpu::Device,count:u32)->Result<(),&'static str> {
  if count as usize>crate::clipping_uniform::MAX_PLANES{return Err("Too many cap planes");}if self.planes.len()==count as usize{return Ok(());}
  self.clear();self.planes=(0..count).map(|plane|{let bytes=[plane,0,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:Some("cap plane selector"),contents:&bytes,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_SRC})}).collect();Ok(())
 }
 pub fn group(&mut self,device:&wgpu::Device,layout:&wgpu::BindGroupLayout,geometry:&Rc<crate::gpu_geometry::GpuGeometry>,plane:u32)->Result<wgpu::BindGroup,&'static str> {
  let parameter=self.planes.get(plane as usize).ok_or("Missing cap plane")?;let key=(Rc::as_ptr(geometry)as usize,plane);let(vertices,indices)=geometry.buffers();
  if let Some((old_vertices,old_indices,group))=self.groups.get(&key){if *old_vertices==vertices&&*old_indices==indices{return Ok(group.clone());}}
  let group=crate::tile_pipelines::group(device,layout,&[&vertices,parameter,&indices]);self.groups.insert(key,(vertices,indices,group.clone()));Ok(group)
 }
 pub fn invalidate(&mut self){self.groups.clear();}
 pub fn clear(&mut self){*self=Self::default();}
 pub fn bytes(&self)->u64 {self.planes.iter().map(wgpu::Buffer::size).sum()}
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzd-prepare-cap-plane-and-arena-bindings-03.rs" skip=2026
```

</details>

</details>

<a id="checkpoint-96zzzze-count-cap-color-through-shared-records"></a>

### Step 13 of 16 · Count cap color through shared records (`96zzzze-count-cap-color-through-shared-records`)

- **Goal:** Connect cap crossing color to current object records and explicit planes.
- **Data Flow Trace:** `Current object row plus explicit plane → original crossing count → unchanged cap color and depth`
- **Files You Touch:** `src/cap_count.wgsl` (*Pull the current arena range and keep explicit plane selection independent of the object slot.*) · `src/cap_pipeline.rs` (*Own the shared crossing state with the existing cap color pipeline.*) · `src/caps_gpu.rs` (*Let the existing crossing target own its plane parameters and cached geometry groups.*) · `src/renderer.rs` (*Pass the existing device to cap count resource preparation.*) · `src/cap_gpu_tests.rs` (*Include the one actual sixteen-byte plane parameter in exact cap ownership accounting.*)

Bind the current object record and slot, pull its arena positions, and read the plane from the parameter just prepared. Draw one instance so the object slot is independent of plane numbering. Keep the same crossing function, additive target and final cap color/depth pass. Changed records retain the earlier exact uniform path.

> **Self-Check Question:** *Why does a shared cap crossing draw use one instance?*  
> **Answer:** Its slot fetch selects one object. The explicit parameter selects the plane, so advancing the instance index must not advance to another object slot.

- **What You Should See:** Cap color crossings consume shared records while retaining original clipping, selection and fill pixels.
- **Hands-On Lab Experiment:** Clip a copied closed solid with two planes. Select and restore it, then compare cap pixels and original picks. Inspect the retained mirrored, nested and release fixtures.

<details>
<summary>Implementation 96zzzze-count-cap-color-through-shared-records: Count cap color through shared records</summary>

Connect cap crossing color to current object records and explicit planes.

Data flow: Current object row plus explicit plane → original crossing count → unchanged cap color and depth.

### 1. `src/cap_count.wgsl`

Pull the current arena range and keep explicit plane selection independent of the object slot.

<details>
<summary>Locate the existing block</summary>

```wgsl
fn behind(plane: u32, p: vec3<f32>) -> f32 {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(3) @binding(0) var<storage,read> cap_vertices:array<PackedVertex>;
@group(3) @binding(1) var<uniform> cap_plane:vec4<u32>;
@group(3) @binding(2) var<storage,read> cap_indices:array<u32>;
@vertex fn vs_record_count(@builtin(vertex_index) occurrence:u32,@location(0) row:u32)->CountOut {let object=object_at(row);let vertex=cap_vertices[cap_indices[occurrence+object.range.x]];return count_corner(packed_position(vertex),cap_plane.x,object);}
fn behind(plane: u32, p: vec3<f32>) -> f32 {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzze-count-cap-color-through-shared-records-01.rs"
```

</details>

### 2. `src/cap_pipeline.rs`

Own the shared crossing state with the existing cap color pipeline. Reuse the crossing descriptor, shader, additive blend and depth policy with current records. Retain the shared count state beside the earlier uniform fallback.

<details>
<summary>Locate the existing block</summary>

```rust
pub(crate) fn pipeline(device:&wgpu::Device,source:String,groups:&[&wgpu::BindGroupLayout],vertex:&str,fragment:&str,targets:&[wgpu::ColorTargetState],count:bool)->wgpu::RenderPipeline{
 let attributes=wgpu::vertex_attr_array![0=>Float32x3];let buffers=if count{vec![wgpu::VertexBufferLayout{array_stride:20,step_mode:wgpu::VertexStepMode::Vertex,attributes:&attributes}]}else{Vec::new()};
 pipeline_buffers(device,source,groups,vertex,fragment,targets,&buffers,count)
}
pub(crate) fn pipeline_buffers(device:&wgpu::Device,source:String,groups:&[&wgpu::BindGroupLayout],vertex:&str,fragment:&str,targets:&[wgpu::ColorTargetState],buffers:&[wgpu::VertexBufferLayout<'_>],count:bool)->wgpu::RenderPipeline{
 let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("original closed-solid sections"),source:wgpu::ShaderSource::Wgsl(source.into())});let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("original closed-solid sections"),bind_group_layouts:&groups.iter().map(|g|Some(*g)).collect::<Vec<_>>(),immediate_size:0});
 device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("original closed-solid sections"),layout:Some(&layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some(vertex),compilation_options:Default::default(),buffers},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some(fragment),compilation_options:Default::default(),targets:&targets.iter().cloned().map(Some).collect::<Vec<_>>()}),primitive:Default::default(),depth_stencil:if count{None}else{Some(crate::depth::surface())},multisample:Default::default(),multiview_mask:None,cache:None})
}
pub struct Pipelines{pub count:wgpu::RenderPipeline,pub color:wgpu::RenderPipeline,pub counts:wgpu::BindGroupLayout,pub frame:wgpu::BindGroupLayout}
impl Pipelines{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{
  let view=crate::gpu_layout::uniform(device,80);let model=crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64);let clip=crate::clipping_gpu::layout(device);let frame=crate::gpu_layout::uniform(device,16);let counts=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("section crossing texture"),entries:&[crate::text_gpu_support::texture_entry(0,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Float{filterable:false},false)]});
  let add=wgpu::BlendComponent{src_factor:wgpu::BlendFactor::One,dst_factor:wgpu::BlendFactor::One,operation:wgpu::BlendOperation::Add};let count=pipeline(device,crate::clipping_gpu::source(include_str!("cap_count.wgsl"),2,0),&[&view,&model,&clip],"vs_count","fs_count",&[wgpu::ColorTargetState{format:wgpu::TextureFormat::R16Float,blend:Some(wgpu::BlendState{color:add,alpha:add}),write_mask:wgpu::ColorWrites::RED}],true);
  let color=pipeline(device,crate::clipping_gpu::source(include_str!("cap_color.wgsl"),1,0),&[&frame,&clip,&counts],"vs_cap","fs_cap",&[wgpu::ColorTargetState{format,blend:None,write_mask:wgpu::ColorWrites::ALL},crate::primitive_image::target(true)],false);Self{count,color,counts,frame}
 }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub(crate) fn pipeline(device:&wgpu::Device,source:String,groups:&[&wgpu::BindGroupLayout],vertex:&str,fragment:&str,targets:&[wgpu::ColorTargetState],count:bool)->wgpu::RenderPipeline{
 let attributes=wgpu::vertex_attr_array![0=>Float32x3];let buffers=if count{vec![wgpu::VertexBufferLayout{array_stride:20,step_mode:wgpu::VertexStepMode::Vertex,attributes:&attributes}]}else{Vec::new()};
 pipeline_buffers(device,source,groups,vertex,fragment,targets,&buffers,count)
}
pub(crate) fn pipeline_buffers(device:&wgpu::Device,source:String,groups:&[&wgpu::BindGroupLayout],vertex:&str,fragment:&str,targets:&[wgpu::ColorTargetState],buffers:&[wgpu::VertexBufferLayout<'_>],count:bool)->wgpu::RenderPipeline{
 let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("original closed-solid sections"),source:wgpu::ShaderSource::Wgsl(source.into())});let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("original closed-solid sections"),bind_group_layouts:&groups.iter().map(|g|Some(*g)).collect::<Vec<_>>(),immediate_size:0});
 device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("original closed-solid sections"),layout:Some(&layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some(vertex),compilation_options:Default::default(),buffers},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some(fragment),compilation_options:Default::default(),targets:&targets.iter().cloned().map(Some).collect::<Vec<_>>()}),primitive:Default::default(),depth_stencil:if count{None}else{Some(crate::depth::surface())},multisample:Default::default(),multiview_mask:None,cache:None})
}
pub struct Pipelines{pub count:wgpu::RenderPipeline,pub record_count:wgpu::RenderPipeline,pub geometry:wgpu::BindGroupLayout,pub color:wgpu::RenderPipeline,pub counts:wgpu::BindGroupLayout,pub frame:wgpu::BindGroupLayout}
impl Pipelines{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{
  let view=crate::gpu_layout::uniform(device,80);let model=crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64);let clip=crate::clipping_gpu::layout(device);let frame=crate::gpu_layout::uniform(device,16);let counts=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("section crossing texture"),entries:&[crate::text_gpu_support::texture_entry(0,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Float{filterable:false},false)]});
  let add=wgpu::BlendComponent{src_factor:wgpu::BlendFactor::One,dst_factor:wgpu::BlendFactor::One,operation:wgpu::BlendOperation::Add};let target=wgpu::ColorTargetState{format:wgpu::TextureFormat::R16Float,blend:Some(wgpu::BlendState{color:add,alpha:add}),write_mask:wgpu::ColorWrites::RED};let count=pipeline(device,crate::clipping_gpu::source(include_str!("cap_count.wgsl"),2,0),&[&view,&model,&clip],"vs_count","fs_count",&[target.clone()],true);
  let geometry=crate::cap_geometry::layout(device);let source=crate::object_rows::source(include_str!("cap_count.wgsl"));let record_count=pipeline_buffers(device,crate::clipping_gpu::source(&source,2,0),&[&view,&crate::object_rows::layout(device),&clip,&geometry],"vs_record_count","fs_count",&[target],&[crate::draw_slots::layout()],true);
  let color=pipeline(device,crate::clipping_gpu::source(include_str!("cap_color.wgsl"),1,0),&[&frame,&clip,&counts],"vs_cap","fs_cap",&[wgpu::ColorTargetState{format,blend:None,write_mask:wgpu::ColorWrites::ALL},crate::primitive_image::target(true)],false);Self{count,record_count,geometry,color,counts,frame}
 }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzze-count-cap-color-through-shared-records-02.rs"
```

```text
--8<-- "journey/code/96zzzze-count-cap-color-through-shared-records-03.rs"
```

```text
--8<-- "journey/code/96zzzze-count-cap-color-through-shared-records-04.rs"
```

</details>

### 3. `src/caps_gpu.rs`

Let the existing crossing target own its plane parameters and cached geometry groups. Drop stale geometry references when scene ownership changes. Keep the new resource under the existing count-image release lifetime. Prepare bounded plane values before encoding crossings. Create current immutable arena bindings on the existing device. Use the renderer’s current record owner for cap color crossings. Refresh the crossing target’s cached geometry groups as needed. Reuse groups for current records and retain exact target-plane filtering. Draw one object slot with an explicit plane, or use the unchanged uniform crossing path. Account plane parameters once and release them with the crossing image.

<details>
<summary>Locate the existing block</summary>

```rust
use wgpu::util::DeviceExt;
use crate::{gpu_mesh::GpuMesh,solid_orientation::CLOSED};
use crate::cap_image::Image;
struct Counts{image:Image,group:wgpu::BindGroup}
pub struct Caps{pub enabled:bool,planes:u32,pick:Option<crate::cap_pick_gpu::Pick>,pub pick_error:Option<&'static str>,bounds:crate::cap_source_bounds::Bounds,targets:std::collections::HashMap<u32,u32>,pub solids:crate::solid_cache::Solids,pipes:Option<crate::cap_pipeline::Pipelines>,counts:Option<Counts>,frame:wgpu::Buffer,frame_group:Option<wgpu::BindGroup>,pub rows:Vec<[u32;2]>,pub draws:u64,pub passes:u64}
impl Caps{
 pub fn new(device:&wgpu::Device)->Self{let frame=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("section canvas frame"),contents:&[0;16],usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST});Self{enabled:true,planes:0,pick:None,pick_error:None,bounds:Default::default(),targets:Default::default(),solids:Default::default(),pipes:None,counts:None,frame,frame_group:None,rows:Vec::new(),draws:0,passes:0}}
 pub fn set_scene(&mut self,queue:&wgpu::Queue,scene:&crate::scene::Scene,meshes:&mut[GpuMesh],planes:&[crate::clipping_rows::Row]){
  self.planes=planes.len()as u32;self.rows.clear();self.targets.clear();for(mesh,object)in meshes.iter_mut().zip(scene.objects().iter().filter(|r|scene.visible(r.id))){assert_eq!(mesh.id,object.id);let flags=self.solids.flags(object);let token=if !planes.is_empty()&&flags&CLOSED!=0{self.rows.push([mesh.id.value(),u32::from(mesh.pickable())]);self.rows.len()as u32}else{0};mesh.set_caps(queue,flags,token);if token>0{if let Some(bounds)=self.bounds.placed(&object.mesh,&object.model){let mask=planes.iter().enumerate().fold(0,|mask,(i,row)|mask|if row.plane.crosses(&bounds,[0.;3]){1<<i}else{0});self.targets.insert(mesh.id.value(),mask);}}}
  self.solids.prune();self.bounds.prune();if self.rows.is_empty(){self.rows.shrink_to_fit();self.targets.shrink_to_fit();}
 }
 pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,size:[u32;2],active:bool){
  let size=if active&&!self.rows.is_empty(){size}else{[1,1]};
  if size==[1,1]{self.counts=None;return;}
  if self.pipes.is_none(){let pipes=crate::cap_pipeline::Pipelines::new(device,format);self.frame_group=Some(crate::gpu_layout::group(device,&pipes.frame,&self.frame));self.pipes=Some(pipes);}
  let pipes=self.pipes.as_ref().unwrap();if self.counts.as_ref().is_none_or(|c|c.image.size!=size){let image=Image::new(device,size,wgpu::TextureFormat::R16Float,2);let group=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("original section crossings"),layout:&pipes.counts,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(&image.view)}]});self.counts=Some(Counts{image,group});}
  let bytes=[size[0]as f32,size[1]as f32,0.,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();queue.write_buffer(&self.frame,0,&bytes);
 }
 pub fn draw(&mut self,encoder:&mut wgpu::CommandEncoder,color:&wgpu::TextureView,primitive:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,meshes:&[GpuMesh],planes:u32){
  let(Some(pipes),Some(counts))=(&self.pipes,&self.counts)else{return};
  for plane in 0..planes{
   {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original closed-solid crossings"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&counts.image.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&pipes.count);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clipping,&[]);for mesh in meshes.iter().filter(|m|m.closed()&&self.targets.get(&m.id.value()).is_some_and(|mask|mask&(1<<plane)!=0)){mesh.bind_model(&mut pass);mesh.geometry.draw_range(&mut pass,plane..plane+1);self.draws+=1;}}
   {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section color and depth"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:color,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:primitive,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&pipes.color);pass.set_bind_group(0,self.frame_group.as_ref().unwrap(),&[]);pass.set_bind_group(1,clipping,&[]);pass.set_bind_group(2,&counts.group,&[]);pass.draw(0..3,plane..plane+1);self.draws+=1;}
   self.passes+=2;
  }
 }
 pub fn encode_pick(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,encoder:&mut wgpu::CommandEncoder,image:&crate::pick_image::Image,window:crate::pick_window::Window,view:&wgpu::BindGroup,crop:&wgpu::Buffer,clipping:&wgpu::BindGroup,meshes:&[GpuMesh],mode:crate::pick_sources::Mode){
  if !self.enabled||self.planes==0||self.rows.is_empty(){self.pick=None;self.pick_error=None;return;}
  let pick=self.pick.get_or_insert_with(||crate::cap_pick_gpu::Pick::new(device));if let Err(error)=pick.prepare(device,queue,window,crop,&self.rows,self.planes,mode){self.pick_error=Some(error);self.pick=None;return;}self.pick_error=None;pick.encode(encoder,image,window,view,clipping,meshes,&self.targets,self.planes);
 }
 pub fn pick_bytes(&self)->u64{self.pick.as_ref().map_or(0,crate::cap_pick_gpu::Pick::bytes)}
 pub fn usage(&self)->[u64;5]{[self.frame.size()+self.pick_bytes()+self.counts.as_ref().map_or(0,|c|c.image.bytes),self.rows.len()as u64,self.solids.checks,self.draws,self.passes]}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use wgpu::util::DeviceExt;
use crate::{gpu_mesh::GpuMesh,solid_orientation::CLOSED};
use crate::cap_image::Image;
struct Counts{image:Image,group:wgpu::BindGroup,geometry:crate::cap_geometry::Geometry}
pub struct Caps{pub enabled:bool,planes:u32,pick:Option<crate::cap_pick_gpu::Pick>,pub pick_error:Option<&'static str>,bounds:crate::cap_source_bounds::Bounds,targets:std::collections::HashMap<u32,u32>,pub solids:crate::solid_cache::Solids,pipes:Option<crate::cap_pipeline::Pipelines>,counts:Option<Counts>,frame:wgpu::Buffer,frame_group:Option<wgpu::BindGroup>,pub rows:Vec<[u32;2]>,pub draws:u64,pub passes:u64}
impl Caps{
 pub fn new(device:&wgpu::Device)->Self{let frame=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("section canvas frame"),contents:&[0;16],usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST});Self{enabled:true,planes:0,pick:None,pick_error:None,bounds:Default::default(),targets:Default::default(),solids:Default::default(),pipes:None,counts:None,frame,frame_group:None,rows:Vec::new(),draws:0,passes:0}}
 pub fn set_scene(&mut self,queue:&wgpu::Queue,scene:&crate::scene::Scene,meshes:&mut[GpuMesh],planes:&[crate::clipping_rows::Row]){
  if let Some(counts)=self.counts.as_mut(){counts.geometry.invalidate();}
  self.planes=planes.len()as u32;self.rows.clear();self.targets.clear();for(mesh,object)in meshes.iter_mut().zip(scene.objects().iter().filter(|r|scene.visible(r.id))){assert_eq!(mesh.id,object.id);let flags=self.solids.flags(object);let token=if !planes.is_empty()&&flags&CLOSED!=0{self.rows.push([mesh.id.value(),u32::from(mesh.pickable())]);self.rows.len()as u32}else{0};mesh.set_caps(queue,flags,token);if token>0{if let Some(bounds)=self.bounds.placed(&object.mesh,&object.model){let mask=planes.iter().enumerate().fold(0,|mask,(i,row)|mask|if row.plane.crosses(&bounds,[0.;3]){1<<i}else{0});self.targets.insert(mesh.id.value(),mask);}}}
  self.solids.prune();self.bounds.prune();if self.rows.is_empty(){self.rows.shrink_to_fit();self.targets.shrink_to_fit();}
 }
 pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,size:[u32;2],active:bool){
  let size=if active&&!self.rows.is_empty(){size}else{[1,1]};
  if size==[1,1]{self.counts=None;return;}
  if self.pipes.is_none(){let pipes=crate::cap_pipeline::Pipelines::new(device,format);self.frame_group=Some(crate::gpu_layout::group(device,&pipes.frame,&self.frame));self.pipes=Some(pipes);}
  let pipes=self.pipes.as_ref().unwrap();if self.counts.as_ref().is_none_or(|c|c.image.size!=size){let image=Image::new(device,size,wgpu::TextureFormat::R16Float,2);let group=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("original section crossings"),layout:&pipes.counts,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(&image.view)}]});self.counts=Some(Counts{image,group,geometry:Default::default()});}
  self.counts.as_mut().unwrap().geometry.set_planes(device,self.planes).expect("Validated clipping plane count");
  let bytes=[size[0]as f32,size[1]as f32,0.,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();queue.write_buffer(&self.frame,0,&bytes);
 }
 pub fn draw(&mut self,device:&wgpu::Device,encoder:&mut wgpu::CommandEncoder,color:&wgpu::TextureView,primitive:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,meshes:&[GpuMesh],planes:u32,records:Option<&crate::instance_gpu::Lane>){
  let(Some(pipes),Some(counts))=(&self.pipes,&mut self.counts)else{return};
  for plane in 0..planes{
   let rows=meshes.iter().filter(|m|m.closed()&&self.targets.get(&m.id.value()).is_some_and(|mask|mask&(1<<plane)!=0)).map(|mesh|{let group=records.and_then(|records|records.current_row(mesh)).map(|_|counts.geometry.group(device,&pipes.geometry,&mesh.geometry,plane).expect("Prepared cap plane"));(mesh,group)}).collect::<Vec<_>>();
   {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original closed-solid crossings"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&counts.image.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&pipes.count);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clipping,&[]);for(mesh,group)in &rows{if let Some(group)=group.as_ref().filter(|_|records.is_some_and(|records|records.bind_record(&mut pass,mesh))){pass.set_pipeline(&pipes.record_count);pass.set_bind_group(3,group,&[]);pass.draw(0..mesh.geometry.index_count,0..1);}else{pass.set_pipeline(&pipes.count);mesh.bind_model(&mut pass);mesh.geometry.draw_range(&mut pass,plane..plane+1);}self.draws+=1;}}
   {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section color and depth"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:color,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:primitive,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&pipes.color);pass.set_bind_group(0,self.frame_group.as_ref().unwrap(),&[]);pass.set_bind_group(1,clipping,&[]);pass.set_bind_group(2,&counts.group,&[]);pass.draw(0..3,plane..plane+1);self.draws+=1;}
   self.passes+=2;
  }
 }
 pub fn encode_pick(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,encoder:&mut wgpu::CommandEncoder,image:&crate::pick_image::Image,window:crate::pick_window::Window,view:&wgpu::BindGroup,crop:&wgpu::Buffer,clipping:&wgpu::BindGroup,meshes:&[GpuMesh],mode:crate::pick_sources::Mode){
  if !self.enabled||self.planes==0||self.rows.is_empty(){self.pick=None;self.pick_error=None;return;}
  let pick=self.pick.get_or_insert_with(||crate::cap_pick_gpu::Pick::new(device));if let Err(error)=pick.prepare(device,queue,window,crop,&self.rows,self.planes,mode){self.pick_error=Some(error);self.pick=None;return;}self.pick_error=None;pick.encode(encoder,image,window,view,clipping,meshes,&self.targets,self.planes);
 }
 pub fn pick_bytes(&self)->u64{self.pick.as_ref().map_or(0,crate::cap_pick_gpu::Pick::bytes)}
 pub fn usage(&self)->[u64;5]{[self.frame.size()+self.pick_bytes()+self.counts.as_ref().map_or(0,|c|c.image.bytes+c.geometry.bytes()),self.rows.len()as u64,self.solids.checks,self.draws,self.passes]}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzze-count-cap-color-through-shared-records-05.rs"
```

```text
--8<-- "journey/code/96zzzze-count-cap-color-through-shared-records-06.rs"
```

```text
--8<-- "journey/code/96zzzze-count-cap-color-through-shared-records-07.rs"
```

```text
--8<-- "journey/code/96zzzze-count-cap-color-through-shared-records-08.rs"
```

```text
--8<-- "journey/code/96zzzze-count-cap-color-through-shared-records-09.rs"
```

```text
--8<-- "journey/code/96zzzze-count-cap-color-through-shared-records-10.rs"
```

```text
--8<-- "journey/code/96zzzze-count-cap-color-through-shared-records-11.rs"
```

```text
--8<-- "journey/code/96zzzze-count-cap-color-through-shared-records-12.rs"
```

```text
--8<-- "journey/code/96zzzze-count-cap-color-through-shared-records-13.rs"
```

```text
--8<-- "journey/code/96zzzze-count-cap-color-through-shared-records-14.rs"
```

</details>

### 4. `src/renderer.rs`

Pass the existing device to cap count resource preparation. Use the same current object records for cap color, ordinary surfaces and property consumers.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            self.instances.bind_own(&mut pass);
            self.highlight.mesh(&mut pass);
        }
        self.caps.borrow_mut().draw(&mut encoder,view,&self.primitive.view,&self.depth,&self.view_group,&clip_group,&self.meshes,if self.clipping.borrow().enabled{self.clipping.borrow().rows.len()as u32}else{0});
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("cloud depth composite"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});if let Some(clouds)=self.clouds.borrow().as_ref(){clouds.draw(&mut pass);}}
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite scene ink"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:None,stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});
```

</details>

Replace it with:

```rust
            self.instances.bind_own(&mut pass);
            self.highlight.mesh(&mut pass);
        }
        self.caps.borrow_mut().draw(&self.device,&mut encoder,view,&self.primitive.view,&self.depth,&self.view_group,&clip_group,&self.meshes,if self.clipping.borrow().enabled{self.clipping.borrow().rows.len()as u32}else{0},Some(&self.instances));
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("cloud depth composite"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});if let Some(clouds)=self.clouds.borrow().as_ref(){clouds.draw(&mut pass);}}
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite scene ink"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:None,stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});
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
        let radii=self.ambient_rows.specs.iter().copied().collect::<std::collections::HashMap<_,_>>();for mesh in &mut self.meshes {mesh.set_ambient_radius(&self.queue,radii.get(&mesh.id.value()).copied().unwrap_or(0.));}
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
        self.instances.set(&self.device,&self.queue,self.meshes.iter().filter(|row|self.instance_rows.contains(&row.id)),self.meshes.iter().filter(|row|!self.instance_rows.contains(&row.id))).expect("Validated instance records fit device limits");
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
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
            &self.meshes, &[&self.mesh_ink, &self.strokes, &self.paths, &self.markers], &self.faces,self.text.borrow().as_ref(),self.clouds.borrow().as_ref(),Some(&self.sheets),Some(&tables),&clipping.group,&clipping.buffer,Some(&mut self.caps.borrow_mut()),Some(&self.instances)); }
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
        if self.ambient_enabled.get(){let clip=self.clipping.borrow();if let Some(ambient)=self.ambient.borrow_mut().as_mut(){ambient.draw(&self.device,&self.queue,&mut encoder,view,&self.depth,&self.view_group,&clip_group,&clip.buffer,&self.primitive.view,&self.meshes,*transform,self.size,self.density,self.ambient_rows.ground,[self.pyramid_revision,clip_revision],Some(&self.instances));}}
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
--8<-- "journey/code/96zzzze-count-cap-color-through-shared-records-15.rs"
```

```text
--8<-- "journey/code/96zzzze-count-cap-color-through-shared-records-16.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/cap_gpu_tests.rs`

Include the one actual sixteen-byte plane parameter in exact cap ownership accounting.

<details>
<summary>Locate the existing block</summary>

```text
assert_eq!(r.cap_usage()[0],16+320*240*2);
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzze-count-cap-color-through-shared-records-17.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/cap_gpu_tests.rs`

Supply an exact shared/uniform cap comparison and restoration while retaining selected pixels, original closedness cache and full release.

<details>
<summary>Locate the existing block</summary>

```text
scene.clear();r.set_scene(&scene,None);pixels(&r);assert_eq!(r.cap_usage()[0],16);
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzze-count-cap-color-through-shared-records-18.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/renderer.rs`

Supply test-only access to the uniform fallback without exposing renderer internals in the browser application.

<details>
<summary>Locate the existing block</summary>

```text
    pub fn set_scene(
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzze-count-cap-color-through-shared-records-test-access.rs"
```

</details>

</details>

<a id="checkpoint-96zzzzf-prepare-shared-record-cap-picks"></a>

### Step 14 of 16 · Prepare shared record cap picks (`96zzzzf-prepare-shared-record-cap-picks`)

- **Goal:** Prepare both cap pick crossing states without changing original ownership.
- **Data Flow Trace:** `Existing pick resources plus explicit arena range → shared vertex entry → unchanged crossing fragments`
- **Files You Touch:** `src/cap_count.wgsl` (*Retain the original atomic, crop and frame bindings while adding pulled geometry to the shared pick vertex entry.*) · `src/cap_pick_pipeline.rs` (*Own both shared crossing passes beside the original fallback.*) · `src/current_inventory.json` (*Copy the dated current runtime-input inventory after kernel/wood changes. This supplied data refresh preserves the historical inventory and every viewer source hash; it does not replace implementation work.*)

Cap color already pulls current records. Picking also needs atomic crossings, crop and frame resources in its fourth bind group. Extend that group with arena positions, indices and an explicit plane while retaining the existing entries. Prepare both crossing states using the same fragment algorithms. The next lesson connects the resource owner and draws.

> **Self-Check Question:** *Why retain the original first three pick bindings?*  
> **Answer:** They hold atomic crossing storage, the crop and frame. Replacing them with the color group would lose picking semantics.

- **What You Should See:** Both shared cap pick pipeline states compile; drawing still follows the existing uniform path.
- **Hands-On Lab Experiment:** Inspect the six group entries, then clip and pick the same original solid. Behavior remains unchanged at this preparatory checkpoint.

<details>
<summary>Implementation 96zzzzf-prepare-shared-record-cap-picks: Prepare shared record cap picks</summary>

Prepare both cap pick crossing states without changing original ownership.

Data flow: Existing pick resources plus explicit arena range → shared vertex entry → unchanged crossing fragments.

### 1. `src/cap_count.wgsl`

Retain the original atomic, crop and frame bindings while adding pulled geometry to the shared pick vertex entry.

<details>
<summary>Locate the existing block</summary>

```wgsl
fn wanted(pixel: u32, mode: u32) -> bool {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(3) @binding(3) var<storage,read> pick_cap_vertices:array<PackedVertex>;
@group(3) @binding(4) var<uniform> pick_cap_plane:vec4<u32>;
@group(3) @binding(5) var<storage,read> pick_cap_indices:array<u32>;
@vertex fn vs_record_pick_count(@builtin(vertex_index) occurrence:u32,@location(0) row:u32)->CountOut {let object=object_at(row);let vertex=pick_cap_vertices[pick_cap_indices[occurrence+object.range.x]];var output=count_corner(packed_position(vertex),pick_cap_plane.x,object);output.pos=cap_crop.transform*output.pos;return output;}
fn wanted(pixel: u32, mode: u32) -> bool {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzf-prepare-shared-record-cap-picks-01.rs"
```

</details>

### 2. `src/cap_pick_pipeline.rs`

Own both shared crossing passes beside the original fallback. Extend the pick group without replacing its atomic crossing, crop or frame resources. Reuse the original empty color target for both shared passes. Reuse both fragment algorithms, the crop transform and crossing descriptor with the current object record. Keep the preparatory shared states under the original pick pipeline owner.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Pipelines{pub count:wgpu::RenderPipeline,pub nearest:wgpu::RenderPipeline,pub ids:wgpu::RenderPipeline,pub records:wgpu::BindGroupLayout,pub owners:wgpu::BindGroupLayout,pub frame:wgpu::BindGroupLayout}
impl Pipelines{
 pub fn new(device:&wgpu::Device)->Self{
  use wgpu::{ShaderStages as S,BufferBindingType as B};
  let frame=crate::gpu_layout::uniform(device,32);let clip=crate::clipping_gpu::layout(device);let view=crate::gpu_layout::uniform(device,80);let model=crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64);
  let records=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("bounded cap crossings and original crop"),entries:&[crate::tile_pipelines::entry(0,S::FRAGMENT,B::Storage{read_only:false}),crate::tile_pipelines::entry(1,S::VERTEX,B::Uniform),crate::tile_pipelines::entry(2,S::FRAGMENT,B::Uniform)]});
  let owners=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("cap crossings and original owner table"),entries:&[crate::tile_pipelines::entry(0,S::FRAGMENT,B::Storage{read_only:false}),crate::tile_pipelines::entry(1,S::FRAGMENT,B::Storage{read_only:true})]});
  let source=crate::clipping_gpu::source(include_str!("cap_count.wgsl"),2,0);let target=wgpu::ColorTargetState{format:wgpu::TextureFormat::R8Unorm,blend:None,write_mask:wgpu::ColorWrites::empty()};let count=crate::cap_pipeline::pipeline(device,source.clone(),&[&view,&model,&clip,&records],"vs_pick_count","fs_pick_count",&[target.clone()],true);let nearest=crate::cap_pipeline::pipeline(device,source,&[&view,&model,&clip,&records],"vs_pick_count","fs_pick_owner",&[target],true);
  let ids=crate::cap_pipeline::pipeline(device,crate::clipping_gpu::source(include_str!("cap_pick.wgsl"),1,0),&[&frame,&clip,&owners],"vs_cap","fs_cap_id",&[wgpu::ColorTargetState{format:wgpu::TextureFormat::Rg32Uint,blend:None,write_mask:wgpu::ColorWrites::ALL},crate::primitive_image::target(true)],false);
  Self{count,nearest,ids,records,owners,frame}
 }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub struct Pipelines{pub record_count:wgpu::RenderPipeline,pub record_nearest:wgpu::RenderPipeline,pub record_geometry:wgpu::BindGroupLayout,pub count:wgpu::RenderPipeline,pub nearest:wgpu::RenderPipeline,pub ids:wgpu::RenderPipeline,pub records:wgpu::BindGroupLayout,pub owners:wgpu::BindGroupLayout,pub frame:wgpu::BindGroupLayout}
impl Pipelines{
 pub fn new(device:&wgpu::Device)->Self{
  use wgpu::{ShaderStages as S,BufferBindingType as B};
  let frame=crate::gpu_layout::uniform(device,32);let clip=crate::clipping_gpu::layout(device);let view=crate::gpu_layout::uniform(device,80);let model=crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64);
  let records=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("bounded cap crossings and original crop"),entries:&[crate::tile_pipelines::entry(0,S::FRAGMENT,B::Storage{read_only:false}),crate::tile_pipelines::entry(1,S::VERTEX,B::Uniform),crate::tile_pipelines::entry(2,S::FRAGMENT,B::Uniform)]});
  let record_geometry=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("shared cap pick crossings"),entries:&[crate::tile_pipelines::entry(0,S::FRAGMENT,B::Storage{read_only:false}),crate::tile_pipelines::entry(1,S::VERTEX,B::Uniform),crate::tile_pipelines::entry(2,S::FRAGMENT,B::Uniform),crate::tile_pipelines::entry(3,S::VERTEX,B::Storage{read_only:true}),crate::tile_pipelines::entry(4,S::VERTEX,B::Uniform),crate::tile_pipelines::entry(5,S::VERTEX,B::Storage{read_only:true})]});
  let owners=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("cap crossings and original owner table"),entries:&[crate::tile_pipelines::entry(0,S::FRAGMENT,B::Storage{read_only:false}),crate::tile_pipelines::entry(1,S::FRAGMENT,B::Storage{read_only:true})]});
  let source=crate::clipping_gpu::source(include_str!("cap_count.wgsl"),2,0);let target=wgpu::ColorTargetState{format:wgpu::TextureFormat::R8Unorm,blend:None,write_mask:wgpu::ColorWrites::empty()};let count=crate::cap_pipeline::pipeline(device,source.clone(),&[&view,&model,&clip,&records],"vs_pick_count","fs_pick_count",&[target.clone()],true);let nearest=crate::cap_pipeline::pipeline(device,source,&[&view,&model,&clip,&records],"vs_pick_count","fs_pick_owner",&[target.clone()],true);
  let shared=crate::object_rows::source(include_str!("cap_count.wgsl"));let source=crate::clipping_gpu::source(&shared,2,0);let layouts=[&view,&crate::object_rows::layout(device),&clip,&record_geometry];let record_count=crate::cap_pipeline::pipeline_buffers(device,source.clone(),&layouts,"vs_record_pick_count","fs_pick_count",&[target.clone()],&[crate::draw_slots::layout()],true);let record_nearest=crate::cap_pipeline::pipeline_buffers(device,source,&layouts,"vs_record_pick_count","fs_pick_owner",&[target],&[crate::draw_slots::layout()],true);
  let ids=crate::cap_pipeline::pipeline(device,crate::clipping_gpu::source(include_str!("cap_pick.wgsl"),1,0),&[&frame,&clip,&owners],"vs_cap","fs_cap_id",&[wgpu::ColorTargetState{format:wgpu::TextureFormat::Rg32Uint,blend:None,write_mask:wgpu::ColorWrites::ALL},crate::primitive_image::target(true)],false);
  Self{record_count,record_nearest,record_geometry,count,nearest,ids,records,owners,frame}
 }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzf-prepare-shared-record-cap-picks-02.rs"
```

```text
--8<-- "journey/code/96zzzzf-prepare-shared-record-cap-picks-03.rs"
```

```text
--8<-- "journey/code/96zzzzf-prepare-shared-record-cap-picks-04.rs"
```

```text
--8<-- "journey/code/96zzzzf-prepare-shared-record-cap-picks-05.rs"
```

```text
--8<-- "journey/code/96zzzzf-prepare-shared-record-cap-picks-06.rs"
```

</details>

<details>
<summary>Supplied data</summary>

Copy this supplied data once; it is excluded from the typing estimate.

### 3. `src/current_inventory.json`

Copy the dated current runtime-input inventory after kernel/wood changes. This supplied data refresh preserves the historical inventory and every viewer source hash; it does not replace implementation work.

<details>
<summary>Locate the existing block</summary>

```text
{
  "session_head": "2f96b35d225015cfcb9bfe5e8ec66565ef99aa28",
  "wood_head": "4916884a45e2558d57dd1a0175bca1bd8a627410",
  "kernel_head": "1c415309c46dd10f2a95a44eb980b067a5ef1651",
  "scope": "All files under the three Rust src trees and existing Cargo/build/web entries. Includes the preserved local rational-control change. Assets are separately hashed by the dependency evidence closure; C++ and generated models require their own provenance. Includes preserved October 10 hierarchy-path and parent Element-lamp changes; the original inventory remains historical.",
  "files": [
    {
      "root": "viewer",
      "path": "Cargo.lock",
      "sha256": "9b313409fa85751cba3224e7902022ee6a9420f7fdeaf69acf13c0ec62a6d9de",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
    },
    {
      "root": "viewer",
      "path": "Cargo.toml",
      "sha256": "f3c4a4e33279d03a069b041b21d5b25be8c5f2c1846adbf406c693fc40234002",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
    },
    {
      "root": "viewer",
      "path": "Trunk.toml",
      "sha256": "e36cf10ae1b9029bdcaa6b55468bef4dab8a3211db52763dcfc1e7521bebb9f8",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
    },
    {
      "root": "viewer",
      "path": "build.rs",
      "sha256": "868449c0976bf081f2a3bdf270c1ca0842f923b33a9b9e34dfcb0804c7f5363f",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
    },
    {
      "root": "viewer",
      "path": "index.html",
      "sha256": "fd308bc334190d76723135c1c82b4cd0d0343f3dff0dda573bf338aa5d3961b0",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
    },
    {
      "root": "viewer",
      "path": "src/app/agent.rs",
      "sha256": "555a699f8286aaefa426d3a0f9704e1444f9a4af2ccb7105c41c40dadcb5455b",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/clipping.rs",
      "sha256": "33047462339a5d4fe32742d38e1ca8e364aea126186fd107f1007fbf4158a8e1",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/cloud_query.rs",
      "sha256": "96e5ea670b083218e2105e0f2ffa0578cc5c25050b8ce3ee790d00dc3c401f0c",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/mod.rs",
      "sha256": "4e7ae69c626379e2e2ca8383a0cdec283ff59c4376bf4b09d3e5b27bf7546f52",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tests.rs",
      "sha256": "7b075fc2845ccdb68ac48a148d3c98ac9f54403896e21bb0b4f616430ffa1bcc",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tool/choices.rs",
      "sha256": "2b128f729985b18c0901ebe6dd8780d54abf701394b020da3c3408f3b2b3841f",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tool/cut.rs",
      "sha256": "d0d00438f5ec9230dd73cb19b14c3ad5cd65c043c294bc341c477e81f4ed5a5f",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tool/elements.rs",
      "sha256": "ced86184ffcb868f8677c92445378647ffbccb1ee1c6efc0f626873f904b3eff",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tool/gather.rs",
      "sha256": "acb5cc3b8b424c5c0889fed4ccb994f3a13da0d4de10dcc0944f05fd932d913a",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tool/options.rs",
      "sha256": "b86ccb79060e5159fbba2cd57d7b0fd22c14ca8472ab499f03bdbd5e1971e938",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tool/shape.rs",
      "sha256": "54c275cb3c8e5fbd4c0e3ea4a47910ca1ffcd5e92405d8055ff5088e05f049e7",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tool/surfacing.rs",
      "sha256": "ef9ef71fcb065485333be2ad358e832de162c3acb30a754a73009b22fc8df7ce",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tool.rs",
      "sha256": "c08608743e48879cab82f76157d5f028215af1848cd5b9eeefc343f039001b1b",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/add_edge.rs",
      "sha256": "90e9e807c4a56b5b10bf5f1c3ad35326a5ffcda043043b62680068dc6b5585ed",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/add_group.rs",
      "sha256": "193f4e783390255a028dc24581a66659b2c82602e20a67f06d42d70540b8c10d",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/arctic.rs",
      "sha256": "04637eef3834bfb9c9d76d415d30f7803d857a267ad4472c91117dabbb374f39",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/area.rs",
      "sha256": "9951cf47f2ba308fca8b14e79d0b21dd9de84ec71328f50c808bac970571b73e",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/arrow.rs",
      "sha256": "cd80826f3f4170566ac9d72ee72fc24399c137a761282b1211392e1d2e187997",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/arrowhead.rs",
      "sha256": "33d52b895d61bd9754cfdbc590e867bb60ce7efa3dc8f8a208e99ff1b6114c87",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/attributes.rs",
      "sha256": "fb0300c25c2c5decebac7bfea8d7740e031fe58c16444c2bb36074f353bb7e19",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/block_with_hole.rs",
      "sha256": "ef3b9abac04888315e6cb616f95cad7a296ffef58c5f3d417ed812379b2816e2",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/bounding_box.rs",
      "sha256": "5baa3e329f91c9c6a2be572e4f8752e8f49effbbf564e3230bd00d8fbfb21479",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/box.rs",
      "sha256": "0f6b98541d82ea317acd703a136d8a5962bee64fe5f07150db36945c4443e986",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/capsule.rs",
      "sha256": "193edcbb4eff8c02c47ee461fdc0cee1ea7ad98777e882472dba3b5c1918e3f5",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/clipping_plane.rs",
      "sha256": "400dc33be2b869ae93c0a981c17116fba56672dc6326ac50efb23ff71e7bc294",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/close.rs",
      "sha256": "6ecb9cf60556c359550c1c85dab54700a98b31f9883dd9317e6d1129241a24c5",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/cone.rs",
      "sha256": "c3021c10760195345f67555ed9db7920c58a0a7c19ad925f1a9bcb5d912e0992",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/construction_plane.rs",
      "sha256": "078a4a7462a809f4b71978ac5579ecb8b9844eb41ccc9ad121a7efeab67aaa23",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/controls.rs",
      "sha256": "12bf2eb1ef670991804bbefcab39aeb3f450037d96981f973494f403b88fd909",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/copy.rs",
      "sha256": "eecc6b3b7ab0b379e8bfaa61e9be5210a13ab6e76917909ed6a0de6c34bcd257",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/curve.rs",
      "sha256": "00f85352d925c0196603c314a63090d359081f5b5aaafe635356d4ec1dc4ded1",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/cylinder.rs",
      "sha256": "8c69c4c8a45352d92b7963fa59a247594e67b9d5fee09b985f288fbdeb53744f",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/delete.rs",
      "sha256": "c573604cbdc1487b2b94c8430ae04b4a2dae3421b11a6fdf02176de202d4ec19",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/dodecahedron.rs",
      "sha256": "ce468793396a169f84b3c5454f931cfbeecfa6d07993604c1f4b7c192d0a431a",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/edge.rs",
      "sha256": "fe7666c920dddffd7b9e195121966ea519aba55b7716f86b26ae3788b05e90c7",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/element_beam.rs",
      "sha256": "b99b68f91c511f4fd1d8443b72de9c81670ba48ff7bebb9fdf10366845057e00",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/element_beam_variable.rs",
      "sha256": "b0898d92d19ec629a2c673cb7a64a452e1bc182e09a98cf66ac7d9d299ce6bba",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/element_block.rs",
      "sha256": "751684bcc9a7581e90fe2f2dc6f74b5f33fc0bda14a95ae9a4a10ee98d8ad601",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/element_column.rs",
      "sha256": "e3a32d979a0de748ea4440f488b0e9493cfb4238d6492eb25043fec32d2fefbb",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/element_plate.rs",
      "sha256": "0d984d66a5a49c347e6801ca7d0d8dace30ddec622ca9d4a02bd8046ad12ab07",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/element_support.rs",
      "sha256": "4781d8a89d967bcc99dd142eee41066f85b6cf99fd46347f36ae340716379ea6",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/escape.rs",
      "sha256": "0e7a4072d6a64c4e1b5c5b76cae3d3da6a563461379be9222ea035ad132965d1",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/explode.rs",
      "sha256": "353e44e0cc9953f0e06aead9eeb42cfc9605bbcb8caa72941b945d16f732c7ac",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/extend/reach.rs",
      "sha256": "37f9ed15f0b4148bebc6788829a6fc8bca4a558201348c145cdbd670d5e18a34",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/extend.rs",
      "sha256": "c86295b7fd4314a6ba86581c4ea12d142d0f867ff7698c4b542ce3b0ffe33de3",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/extrude.rs",
      "sha256": "ebc559cd931a5803c1236d521509e929fa11f0e3b987e5eaa0167f2cd0ff2428",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/face.rs",
      "sha256": "c5dc6a0907e7ce3239eb3396036c96c1370f2ff4e6a8df355d453660f8e05bc7",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/fit.rs",
      "sha256": "b1181698ef63b6facb5c50203928959d9b5f54087dea0e0e76cf065567ec719d",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/geometry.rs",
      "sha256": "d1612ae09503003fca3792ad9e4fcac3666069f7f3781ca8b59de7d9653ae586",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/grid_snap.rs",
      "sha256": "ab63e5d6f45c37d9b8b15ea8535d8386da1cdcce725b2a3398ccdc41e01b06c7",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/hide.rs",
      "sha256": "c4a8c1c4fb3d20b2768e3903841ccfae51c8fd8fb7d6b30b1ec3ab61cbdca800",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/icosahedron.rs",
      "sha256": "02407c66c611d0b1961e5726196cfe944082ab19fd4e49a11d22c7e40c458765",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/interactions.rs",
      "topic": "Element attributes and interactions",
      "lessons": "52–54,92,96,97",
      "sha256": "1a4836207a54e368ad3d91d6e53df5c089e4584b8ce53dc5bd7b0bcfe3750d0e"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/layers.rs",
      "sha256": "a0168bfe8c941fee7fba32c9a259c9f847640f24baa958d38383948843f6db6f",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/length.rs",
      "sha256": "64a425e2e952b2daa0ec729235f2702934bb81f472b17f8c140b26415a618204",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/length_bounding_box.rs",
      "sha256": "654498f77d15404be2ef63a4a6f96138186e2eb4d4799bae01310e8b89439fb3",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/line.rs",
      "sha256": "cce63f69a0a8c3dd422e67bf1fec62075bdd6aef55b0a9a79a6f53191d2a278e",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/loft.rs",
      "sha256": "0bd5c5e9169fc10b067629df1567ba2abfbae68aba97148b226e2ad16e96f818",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/measure.rs",
      "sha256": "004c72cbad5f8a24e7ae6141528997ce1e8deecb2e2caa22af68b34d5133c28d",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/measure_distance.rs",
      "sha256": "2b3813e899954595d609de0357984c5b5f01a8f7f031e6320ccffde088001421",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/mod.rs",
      "sha256": "7feb8f196ccecf0bf11c3222eb200dd3d00b8a58d9058fd57d4aa48ad973ff5c",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/move.rs",
      "sha256": "884f591bcde1cb18537f34b02a6a271e921fcd44aec30eb0f3da69017fd57b3a",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_curve_arc.rs",
      "sha256": "c6b50ac35e93520029884440498e0d17f9c2daf09e82c2b77ed3db66f372d7a1",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_curve_circle.rs",
      "sha256": "207decf5247f2fec9b84cf70ff0319e2f646b3af082459825628a098729718d1",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_curve_ellipse.rs",
      "sha256": "5140c1af03b1e011512258bc60901b04cde23222a780829b0328d0e7518dcce0",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_curve_parabola.rs",
      "sha256": "47f3b6a61efdf230fb1d03658b44aacd5268e69d1a359d1cb40d8654d21542b7",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_surface_4_points.rs",
      "sha256": "20464c6908c865053e38d7a9eb953e05f00cb02112262296a1fb36817bdf8981",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_surface_loft.rs",
      "sha256": "108be02d75b82e2f5ac6f34c13dd46b638ccee81314bca8ad6051970d16a588f",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_surface_network.rs",
      "sha256": "7f94d0f5074736b627f761b7b812ff4abc05592fe9ec4bf142410f2cc9643854",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_surface_revolve.rs",
      "sha256": "b447738bee31531180d016dabf2e77d93e30583c85812485114f7bf34850276c",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_surface_sweep1.rs",
      "sha256": "aa18f91aa0a35fb3b552f3bd41bad8e9a2ed565751254adc58b511904b137b84",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_surface_sweep2.rs",
      "sha256": "750b15d8349ce66719eb982dfd85abd590a9f122cbbbf19b676d2ae2b5e04992",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/object.rs",
      "sha256": "fcce94137a4f68c8e9c54617caaca0f7ede2828780203cf8413ef5fff76813b8",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/octahedron.rs",
      "sha256": "203d8f4f5afb243be5ffcce6abdf3bf379d5abc8519b1c5b12aafaf10cca7f86",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/opacity.rs",
      "sha256": "0db647dcb96fa436c851edb2ef728df150fc8ef0681aac483481f0ff1c286ab8",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/open.rs",
      "sha256": "7b71238ac03238c84d483701c161649ecbabe1ed6681025c81408f8862f76b43",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/orient_3_points.rs",
      "sha256": "943bea38cb8607eef2153cbaa1e20260a2a09531bd5d469a2bc525ed35cc9f9d",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/outline.rs",
      "sha256": "537e232c2f91846dec99eb3e6e30e016f482577b1ef82a62c4f4f8a115566053",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/point.rs",
      "sha256": "d77b9be0208a33c75ed0134e99ca50815f2613fbbeb728adb33c30ad14306b0a",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/polyline.rs",
      "sha256": "15a7f496a783fe8864fbc1a090a1c3ea884a2cedff7320ecf0fb85136c59e93d",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/project_to_plane.rs",
      "sha256": "11dc267a1a5d36b974f4723fca487c38c958cc3ac2a0f926e81f99507e6904bb",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/pyramid.rs",
      "sha256": "a1de9a501bdaf8b12212a51da89160d5abb6d49f437529ed8582678ba31919cd",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/quad_sphere.rs",
      "sha256": "d801aa84e8d8c513d89483113c3076389b14c67eb613aae40f7b970af13d96ba",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/redo.rs",
      "sha256": "cc5ded7fc9d8c23b2e2638d5c48910a2ac6ad83a2815aa2976907d53511eded0",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/report.rs",
      "sha256": "8b8e73ef4da70c6221cbec41e75f54db70a5d39df05ee29c19b35efb8afe9a63",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/rotate.rs",
      "sha256": "814e405ba0eac925a1322fc88fa7918e0d436cc291f4ab322cb5ae6dca4911c8",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/save.rs",
      "sha256": "d2ed39ab8cc2f6755f5155a75269a1a097988799f161fe174587927e90084ad5",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/scale.rs",
      "sha256": "bc3f8280720976d74fd46bd4447da41ed63943c014dc62d2d75fbeada88ac5c2",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/select_all.rs",
      "sha256": "e263949a3a7f78ae516f1e3e4e0f53815a96547f0ffc39c0248cc258fd72ab8a",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/select_by_name.rs",
      "sha256": "90d070cb338b3a721a4501e777bf9ff048cfc368fe5d98fd1facbab18531001c",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/select_lasso.rs",
      "sha256": "dcef1e302ab9bea2353e2f85fa391b35117f6f7f97c153beb7901744a0571a7b",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/select_small.rs",
      "sha256": "d4591b944daf9036df7f6854083adf9fb1f593e410d61ee141f1cdd98ee4ab5e",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/selecting.rs",
      "sha256": "49b3172f3487531566fae3e4c955d9a438a0e153cf37d7f29a622bc070a48d27",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/show.rs",
      "sha256": "b55d0061a288fb54d8de3faf65e56df41a9ea98b3f2b353acb1802b9d52c9803",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/snap.rs",
      "sha256": "e85c98cc314cab74c8b9bcb74cf1d32429e49b49c76cf2c83f8a2e929b35603d",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/sphere.rs",
      "sha256": "75e2438143ed9c3f6875596c517211e14000f8525376bf166e52372e82f4ef91",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/split.rs",
      "sha256": "8c029c76b77546dd5fb8831693397b48ef16852ed1379253c2a028349f99b54a",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/tetrahedron.rs",
      "sha256": "f09e06bacd0c09b5b2c7b8b569c93267a4df5cf0fefab653868d154a0b02bd15",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/text.rs",
      "sha256": "7e0f56386df9eff853a325856005ba2670af975edbd7203e60a3dbb70bd030a7",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/torus.rs",
      "sha256": "c64075764f74caf608a647bcfe27aa1537c4b85cc091071cf3728c457f288c08",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/trim/parts.rs",
      "sha256": "d78d99b59d5da7cf85de9cff7d9b161ac01c3b107d6b50fd0a67966f4f4ba71b",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/trim.rs",
      "sha256": "60c27bce49c24b3867c658171267e1c159c24e0c1293de34067711f2c7c4b72d",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/undo.rs",
      "sha256": "6e82fd3e78bf7d00c006740fd49264bb5d83c18957758bec545fab9ca09210aa",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/view.rs",
      "sha256": "a025bf0d864aeb1bfc1c993df44e112118c86e290797a687f22935a442f30e39",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/volume.rs",
      "sha256": "310b7d44e53155c80a823aa411e758d3171a1c8b3a3bdbf273113c36430c0ad9",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/coords.rs",
      "sha256": "3b19ba73fee97ed22f48b7d5fe04bfc1090f1c31ca21c44265e936893a0e447c",
      "topic": "Camera, snapping and construction planes",
      "lessons": "26–27,72–76,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/cplane.rs",
      "sha256": "4cc6d3d6ddeed8baef51fd8e49ea8809156b9bdb8105b114ad1b906273e3ea26",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/decode.rs",
      "sha256": "b9156c5c746b33a6f4fd8a86a9897ef7c0f2ddadd252a3d2004eb21c404bf8fd",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/deform.rs",
      "sha256": "ed009f15ba5ace800914b14e78252b5990d76f001049fa667397ba361633959a",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/edit.rs",
      "sha256": "896328d97d69df9750d6f17c738deb0624ca3021e60fb8d468dc4e0fdb94e49e",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/feedback.rs",
      "sha256": "951457335d199ede0babce3425fb62466a0e717d12cee30816fd13f749058e53",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/fetch.rs",
      "sha256": "57d886ea78208f2bb19ffa13a1a252af18ccfddc7a17cd7c9432a981a1560fed",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/fonts.rs",
      "sha256": "d51582cbb4566c5efb172401510ed2d33675a20a0b63f0a54d9218fa7ee14e6a",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/gesture/box_select.rs",
      "sha256": "f52620890c23afa10a99d046a9985fccaa8b15639c972a46a0fae94a2bf88044",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/gesture/control.rs",
      "sha256": "e58ecb9d4da6bda5216ac2693a3d319be7f9fc386f80afb2d6e5f8df012b2f22",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/gesture/gizmo.rs",
      "sha256": "6ff5f1d9cbd6eb5830a0e80c46813699ec9a195bfc7f1476c342d064b6bd850c",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/gesture/mod.rs",
      "sha256": "a592ddc3964fa3b7732c08d9ac9339491980235e3f8273f9bcf75d0c043218e3",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/gesture/object.rs",
      "sha256": "009f27a8d77f8a767e90187456824324f27fb70d0256d30eb2af9503ad8806c5",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/gizmo.rs",
      "sha256": "397a00a8a1f4cda1f34ba244a40f5a842f27b1c83d7affc298d0f4276d396340",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/hierarchy.rs",
      "sha256": "2f3698997bebcf6ee66cb4e5a4290b796d309de877674b059b48ef92e2af7a03",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/input.rs",
      "sha256": "8dbcf36cc41b9ccb88a925c86c423396eaa572bd730e23d7b3f8df0e956392c1",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/inspection/source_memory.rs",
      "sha256": "0828d7ad8d7b3c0ec05422be79edec175739a05e7f6fe94341d77aa10e12541d",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/inspection.rs",
      "sha256": "157ba8e01eb1b68e7be7182fa92be0188ba17cae357ae3a922e43b8860aa0891",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/interactions.rs",
      "topic": "Element attributes and interactions",
      "lessons": "52–54,92,96,97",
      "sha256": "d0ff1f7f682dc03297e79054202c860598862d6e83db1e7033c858bfbcae5b9c"
    },
    {
      "root": "viewer",
      "path": "src/app/keys.rs",
      "sha256": "28abcbf79f108564e48b4c61ee96b8521b69062d467d2c5c2ad787e6c230289a",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/knobs.rs",
      "sha256": "b9dba07b897e6498d0a4210dca1b909365a82f46df32eb1a2e8dfd712889d99a",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/layers.rs",
      "sha256": "196e1a664f8d6db2611c1c0d581972741b864f8da80037b84a86c8f299cd4d3d",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/live.rs",
      "sha256": "c219787e600906040aedf875182a61635b03e64839c3797b6c9a4eddded7c9c4",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/live_revision.rs",
      "sha256": "91680efc998ea6d91d59bf46380fa938b3f1b9b8a3e9d4be7835096a6bb13c60",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/loader.rs",
      "sha256": "61e21d567eba5f336a45f8cae5b6e3d7c51438315cfb447ec73c9ff5604f50f7",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/manifest.rs",
      "sha256": "ab020e177195951fa7d07d8f132f93de2b41d23c51d6625d19486658baef8228",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/mesh_preview.rs",
      "sha256": "ec41fae6255735322acdb9f9e9cdd3891ba36ed1251b3f438678cc64891c8c4e",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/mod.rs",
      "sha256": "b74ca6bc7b9a0fc8c0889435b79e9e77de6bab4b953934267587f1c2a4a227fc",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/modeling.rs",
      "sha256": "7aa4202bc1756cee495ce0f2daefe9c2c320e651294322898c03624b761f9e04",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/range_gate.rs",
      "sha256": "7191b2de72b0b5e97074c977abbb9ee6b4d62c383f225c55148913528cb9ce1a",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/route.rs",
      "sha256": "009e6ae7e824edebe981be1d31c30dabafb89ae227aec188113f680bfa6dc2b9",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene.rs",
      "sha256": "22e53f5e48fdbfdffb296a266b2c29ae4dd25f309bb2d566a5041cd12bdeb52d",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_instances.rs",
      "sha256": "fa216bc9ebadbc25596c0e76711d49b72a9544317b265146f63614362ca39039",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_release.rs",
      "sha256": "b886d822f51516d904726d7ab2380447da7b8dd99f112c87d406ef54732fd95e",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_rows.rs",
      "sha256": "d21f7804c6a58979381774fff193155e16b9b96c43c5f2cd0022eb3e4f1c7fb6",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/allocation.rs",
      "sha256": "ecb2566170adffa7450aced660683e5bf6bb5e910f95a3d8161ee4359b29ad13",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/commands_tests.rs",
      "sha256": "ce45af59fc2375833c7292459056175568d73d1f678106f650fccf214ccaa65b",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/compaction.rs",
      "sha256": "b7f65a6f4d78543ee866bed81f96ad807df7b154411cef59234c39367bd7d52e",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/editing_tests.rs",
      "sha256": "0e8e6b90788ae1d45ddc2f03cbfaf5e807b1e7c9b47e66e97df70bbad8af8826",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/nodes.rs",
      "sha256": "9c70230b1bd744d5c476938d594ae2be256c3b39b815b147707cb05759ddfbd4",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/notes.rs",
      "sha256": "fa02d60b45d4f74823d75f6786780f7b24448cdf17664b9fdfa4e7f66f21a3ba",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/panel_tests.rs",
      "sha256": "4beba4f873594d0aa0dc1836720ad18739c0124a13ad0c0884fff5e4095f17c4",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/preview.rs",
      "sha256": "9df6f4543433ac9da0cdd853ae93ead478a40404a4b824096d22235c23341e8e",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/split_tests.rs",
      "sha256": "ef2a718cb61c31593c0d5d255604635ce33ad7a1a639f9e0a408ec21dd9c6b70",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/testing.rs",
      "sha256": "9716dce2aeebcced6e6aa72463950f0f4e1e17471942e23752b00ab52af943fa",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/tests.rs",
      "sha256": "30b78b84c2cd80a13cdb33f4cc68bac04645a05596b74216999df4e98a9d22bf",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/tombs.rs",
      "sha256": "a5b5d0082c95e04309c1c61f262a315c2eb610999bf471af8c80246a2a76c7d3",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync.rs",
      "sha256": "7160ce61ef8ba840ef536bff9532ed315b7c2b613301dc7993b61105411d0126",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_text.rs",
      "sha256": "b519d58bf6b0829e157f338300cf704f7986377cb1091294beb7f4c5e8863c8c",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/selection.rs",
      "sha256": "3d635d3895cb8b9446d1b0b6771e6377c625985a599574fef0eee8f56cbe83cb",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/session_io.rs",
      "sha256": "ca46739b5e8d4ff3fe5b5bd22b2d35ab76d0a55c0af3ec2a4ce38ec97c3b87be",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/sheet_query.rs",
      "sha256": "9ff2b40923abe62148dbf2ac355ea7f6db00a3a0402a6f2cf1dbaacaa6916ab7",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/snap.rs",
      "sha256": "174fe242e49e33d825acecf477d771a2ed4a7a0bdd7120c25938ee90241ec7cd",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/splitting.rs",
      "sha256": "50840e7f5750aa42a0e9058427dbea29beac8247b700b69e1fae7885f540b513",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/stream.rs",
      "sha256": "17b125e85e513f0d16ee7bba163a178a41901a9e4205886b9227729ee5203299",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/surface_preview.rs",
      "sha256": "7971672cbd085559288dac7495c9d6b57fe0518fe052c8d95bfa8949d8abe8f5",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/touch.rs",
      "sha256": "6e45e9c59952bcfb102bc98ac9cd0c243d042c52a623d330605ef6b1b3ac6460",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/command_line/tests.rs",
      "sha256": "f42ad65f70e9889f3d09e62bab1a04043f7f1d611f49ba7adb835394a3ed52e4",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/command_line.rs",
      "sha256": "e1465627dfb1b6afa326bbed8ce025de3ec0daa4ec4f0d34504099475e3d1bc0",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/graph.rs",
      "sha256": "6ed37b504001f80ab29a864567ff78fa8125a997cf4b179d538e8b580c7f11b9",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/layers.rs",
      "sha256": "3ed1b04e06c3b56d6ff39c0880be39c0944dcb4fc5847bce2adb127050da96ba",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/mod.rs",
      "sha256": "e281d789a462716213d8ec44b3faec0f9cca7abd65cdb6c3b1306895f9d97c59",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/number_box.rs",
      "sha256": "d38b57f74e9bb3ea749111bff86d8ef67fab2bcdd1293664c487bab08619b7e7",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/overlay.rs",
      "sha256": "99e66230fe357a9922134146603afb7043628bff4a0a4e69b16fe52863812739",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/phone.rs",
      "sha256": "40633a8dc3394a6898d30d324b19912f7002505720777a94a5d3fd64a3d32182",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/pointer.rs",
      "sha256": "7172927a19316efaa813fff3b3963d68fb5228e968f2c6e9ee55bb26cd60323e",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/theme.rs",
      "sha256": "a45673d3ff48b2114846750a22e2ced36f22dbfd7b1e11752914ecbd54b71f3d",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/validate.rs",
      "sha256": "c7e6e4fbe33ea7af6898a18b92758df01f46cb43ea2aacd09d5da346a57623be",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/bounds.rs",
      "sha256": "257a8f622f4ba277e235ae73c69e59b14c8a6cc2ffc52b9ed07076e25868991b",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/brep.rs",
      "sha256": "e49dcc0911686b0f2f4fbd7c9aacc411bb989e45b5c0c1444544f087d886c285",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/brep_edges.rs",
      "sha256": "2a19dd35e93bf62103e9549fc16f9c9d0fe6859a68be424b970d60dea17b5d2d",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/brep_orient.rs",
      "sha256": "0778910bea71d83e47737d5d35100b38f18df8b9dca3d886f642b36125260638",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/brep_shapes.rs",
      "sha256": "c1621011e4006473015572ec811300241204838db2b96d4372b84a740e6fae2c",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/cloud.rs",
      "sha256": "813e12a5731d575101f15a1f18c5d5db2759ab400ba0d8e1701b7047f37207ce",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/curves.rs",
      "sha256": "4da38304e6eb4dbed57d0406fe36a9dc4da823eb4f044d4434160983792756d7",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/encode.rs",
      "sha256": "a925576401ada960e52a28d669f7bee634c5c4213cc74207846c3a5a22af3a83",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/frames.rs",
      "sha256": "1bf83056cc5e36a94514c86ab43ef13d7a10ad9b9de62cfc8df847e3dbd33d8f",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/mesh.rs",
      "sha256": "a070c4a31a303339687adfffd579446e31f33d44c4a3a5c4ca49cd216265e060",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/mesh_ink.rs",
      "sha256": "349f85de9cb23dcb928a80b520468c58e84c4a03a6fd6d60280d2ec10a85cef4",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/mesh_topology.rs",
      "sha256": "9a207b8d0152661e846568bbf5971c3c24fdf5b33e3d5bf4acf6b5e925846ddc",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/mod.rs",
      "sha256": "048bf6d1a2e1b67948b011d5e3ede466e32722991e8c765097308de070fea88b",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/plane.rs",
      "sha256": "9be8bd22c56cb84393584e3fa969723563805be0350b49db1a9aea04ec58c30d",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/points.rs",
      "sha256": "af7baa2ff846c8ede26274635f105bd34377cf2aa63576424ff53da843d38330",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/sheet.rs",
      "sha256": "c5148556dd5a0ff2688c63316b89c53ac79fbb747bd8d70de706ed2fbbff115f",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/camera.rs",
      "sha256": "2eb0d51723129bdea1cddc3f35760217923959afe2a50dfe1fc6e129c8a3d58f",
      "topic": "Camera, snapping and construction planes",
      "lessons": "26–27,72–76,85,96"
    },
    {
      "root": "viewer",
      "path": "src/command_dock/mod.rs",
      "sha256": "c41184b767cb4947bf33bfe8ded268a6d3c7185870a6fa71054a588c002a90d4",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/command_dock/theme.rs",
      "sha256": "2064473cf413d0be10ca1f71440db34e11912c30893a360f11e446a5ffa2099d",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/command_dock/view.rs",
      "sha256": "2b6c7b55d1257a7be5736edd22e69194655b6e14d4dfbffe5c1c01e86456f204",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/ambient_warm.rs",
      "sha256": "94bb4b260a7875b4c82be81acd8131e3cee556020f3cc5145608ec33d6f52585",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/arena.rs",
      "sha256": "7f0d4e53f55d3a1bf384da7ac7781766b8faf4e9f7038ab0dc2c29bb7f6e6441",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/backdrop.rs",
      "sha256": "afb51eaef8ce003e22ab9b4ce0e0af2c1ee786c98b98ceb22992f2a3edf4d6bc",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/buffers.rs",
      "sha256": "f226f75be1899a1540a4f80d4491019aa873aeed811dc73d0573405927a13f2c",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/clip/editing_tests.rs",
      "sha256": "ad0a23dda8ba3b14a7f55db51001d3bd61d24ec51cbea0552babcce6e206044d",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/clip/pipelines.rs",
      "sha256": "483249eef637d0deff02d1e80d96ad5ab56247e18e76faeff2fdc26cd4fdab73",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/clip/tests.rs",
      "sha256": "9f7a926a7e0e57d623a226d9baf781287d0ad6da3c4b0c9108d2612fcdd8d691",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/clip.rs",
      "sha256": "e0b31a38015a8c5bc31c404261ae46757e0655774413de0fd5ab92bb2282d99a",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/cloud.rs",
      "sha256": "ebf520f190ed3924efa465cdeac6d743ee9c2f1db337067955ba08d96959e3a5",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/device.rs",
      "sha256": "6d8d81150175551c886ba7dc412b384c2ef9be30fbc9ffc3fff53cbff6f67a1c",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/faces.rs",
      "sha256": "b48b373d830c28cb9e9456e421390d7f326b4910c9608f487dcf1834be78ff71",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/frame.rs",
      "sha256": "2a492fe36f15db4201a687a1d06e120945d38410a075375fece6ec1d3d4323fb",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/glyphs.rs",
      "sha256": "f992838e16e6792174214bd4ab67040e33026b9d471fad5b988ebd0597716970",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/hull.rs",
      "sha256": "42571ee70a810306267f2f1574e593819231f5633e245dc64f5460d5ff8679ad",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/instance.rs",
      "sha256": "89198da7abf06d41040aae716580d539eab2eed5b4e68763f348c5b7de31bd70",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/instanced.rs",
      "sha256": "f6fe9bb471dcf30fcc81534c6b20ebfbc1755c9cb0cea0edadf6272ab9e673db",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/lane.rs",
      "sha256": "2e16f454ed7cbc29a0422d180ec3eb8aa9adbe7503f7425eadf24765dcfea838",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/lod.rs",
      "sha256": "75dc349e43f44ca3bbbcc26a88f7bc3d56189a4268b0a2e3523e7903e9148a08",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/mod.rs",
      "sha256": "994339d5f4eb05eaee30be2c0352c7033d4b686afd6cad0c1e2ea84204d06035",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/objects.rs",
      "sha256": "58abc890347f29a8d3deb9b7651e3a43b4444a1e1429bf3de830ceb54516e7d5",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/pacing.rs",
      "sha256": "9e5916950f6658ad02b28aae1e009cfdc4546c4a4c4ba24c59c9194a7675b427",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/pass.rs",
      "sha256": "cce6cc842567b136d01ab84d8ef1e39a5b149063128a5df932babf6950c80dc6",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/patch.rs",
      "sha256": "00d8fae175ca925c7e48b8304c702550b1633f56747b4130ef22d6218cd97100",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/pick.rs",
      "sha256": "8dae0bfee0fd18e17ab90b1c0a61cb028af4d87bac29badb72a08c0325ea65f0",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/planes.rs",
      "sha256": "f0e509a16d84aa675ffd8c6b335a2a4f6074f120690881e4b1eae202a22e1f61",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/present.rs",
      "sha256": "ff2820ea362b9fad33463c03007deb90a72f22bf84f68f9c53356f31b7e61f79",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/render.rs",
      "sha256": "48150fc2c3afa9a453af5fa0d0aa35768ed42ee57454a3c46ef0016a518dfabc",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/segments.rs",
      "sha256": "e0d9532d31ce5ffd592eff3c0ca47300e81aca229408fd39a7d248176118ad2a",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/slots.rs",
      "sha256": "ef303f3f3e534502a9d90561955d840c091dcf9b2f1f69fea4f5540fbbfc18fd",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/splat.rs",
      "sha256": "13c99ea37920f0dd4c39750deba7e57d274268b366c3cd56d2a938dec113fa3e",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/ssao/pipelines.rs",
      "sha256": "e1c9ee95daa6fa83ae924519bd7f07a759e3f8d00c939f3dd1bce749ded218f3",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/ssao/tests.rs",
      "sha256": "474fdf5ec95cab2ef4f6a6d2d19528c344e1f3abb4f94cec8ff7ce442fb2e5ae",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/ssao.rs",
      "sha256": "932d34fde6d1d92a37c449e9f3b7c921e3aed63308c7427ac72e080c6c909830",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/surface_outline/tests.rs",
      "sha256": "d669e0da39d775ce45b86a1b01b54f8969b288b92a7cdb76b2941575f8f8940d",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/surface_outline.rs",
      "sha256": "08e790732dcd0ff16abe734be9d13b63dc26e6825f9d1d7578645ee529e4db03",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/targets.rs",
      "sha256": "f16908f3fd299e876687d0894374a440820ed713d3adc57689765f9991d2f140",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/text.rs",
      "sha256": "ac22022ad4c3820c2a834ce060286094f24d7a0a7a30633eac4a26231cfa054a",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/text_outline.rs",
      "sha256": "8444903f4b6c8132375822bd8d95eb5e06b51e023cb3487a730e90c5b3b842a1",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/text_plane.rs",
      "sha256": "641063fe094b466de058a601013f47e3227955eb63ccc0d42564f1b0b8a6063c",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/text_plate.rs",
      "sha256": "b7ec607e1759845577f9c10596ebfea2377bef4f3777ced791f0592f6a71a9c1",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/timing.rs",
      "sha256": "9165356ffbe6773316ddcfa7bd4e68c7e7ecd36892eba36a7dcb9d8760ef8ccc",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/triangle_tiles.rs",
      "sha256": "9e337004028bc94be8f79c5007ad7d04711967c1ea5d093fa6b3cb882123e2c9",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/ui.rs",
      "sha256": "a74d2783ebbb7404e496ceeb01d62671f48018e3173837c66e0f83720cadb263",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/upload.rs",
      "sha256": "d8a88e7f7fcd03c316981bdaddee78ec6f6986be87e69bcf0f298c9b172dab5f",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/upload_padding.rs",
      "sha256": "f713558ae631388ee310c7d6e3e9178eba12f383111cf2fba5f9ccd184fa1f52",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/vectors.rs",
      "sha256": "539d407e8e529d9cd9d4c507f675221805506221a49177773571e51e02d7fa3d",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/view.rs",
      "sha256": "dec8a12c4a27401682d4a506590d107ffe762f1253af19774409e8d32e3879f1",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/widget.rs",
      "sha256": "7227605387d824dd13210d70101a818ed95f9525a000bca1dc796dd7d21d0104",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/widget_mesh.rs",
      "sha256": "89d73dd598c5a0f7639409e724f3df910c09f3ffcf21de16f0cbbd8e669c7306",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/mod.rs",
      "sha256": "f9d05dbaaa0d9b2f6289a368e9c06657159eb8d9a0880936acaa682f736d97d9",
      "topic": "Derived display, text and rendering infrastructure",
      "lessons": "core GPU chapters,60–65,94"
    },
    {
      "root": "viewer",
      "path": "src/engine/performance.rs",
      "sha256": "de679efdb2e8adf6e257324ce37d6026649654d7d4c4a7ab38dcf1b46d5bd5d8",
      "topic": "Frame pacing and performance",
      "lessons": "95"
    },
    {
      "root": "viewer",
      "path": "src/engine/pipelines/bindings.rs",
      "sha256": "7799e0467c095eea09730a0fc7c34847dfd8409dc1280ddda0bf171d35a77509",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/pipelines/layouts.rs",
      "sha256": "2152f1a6ab2fe4fd17e7d3a3eea01606dcab2a6b80ba69bf0760a216d42d3f45",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/pipelines/mod.rs",
      "sha256": "50590524bc796a7839dbf7fc075ad278909b53050fdb47c6e721d5d9f6c1bee5",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/text.rs",
      "sha256": "918ec5b3bcc2a775c81e348d25b943299bc19d2d6ffd9b7ca647c10c9299591c",
      "topic": "Derived display, text and rendering infrastructure",
      "lessons": "core GPU chapters,60–65,94"
    },
    {
      "root": "viewer",
      "path": "src/lib.rs",
      "sha256": "12a3ae5e9c08583ed1088714889ba96bebd7fde315d0e4c0370692a2b12457ab",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
    },
    {
      "root": "viewer",
      "path": "src/selftest/lifecycle.rs",
      "sha256": "01744a1500c73dbeb25e4caa46aec8c82ee240ba706b76895363169dd864991a",
      "topic": "Maintained native rendering and correctness reference",
      "lessons": "94–96"
    },
    {
      "root": "viewer",
      "path": "src/selftest/oracle.rs",
      "sha256": "65e4c530ed8d341c7f1a4bbc5f2691861945e4828bb7056dc74e7202177455f3",
      "topic": "Maintained native rendering and correctness reference",
      "lessons": "94–96"
    },
    {
      "root": "viewer",
      "path": "src/selftest/profile.rs",
      "sha256": "6c8cfa7be644e9e527c1b4a289ca8d2afb10075843207e7ef15415a56b1d6bb1",
      "topic": "Maintained native rendering and correctness reference",
      "lessons": "94–96"
    },
    {
      "root": "viewer",
      "path": "src/selftest.rs",
      "sha256": "dec37fbc978002c1184181627769ac0c6daeec91da64b13373ef1d33bdaa08fc",
      "topic": "Maintained native rendering and correctness reference",
      "lessons": "94–96"
    },
    {
      "root": "viewer",
      "path": "src/shaders/ambient_composite.wgsl",
      "sha256": "15eab6b52a9dff22c38cb505ff7462b5b4cd6bdbe3e5223716902a231b9dd6d8",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/ambient_depth.wgsl",
      "sha256": "80258c80084418bc9211c6c31438cc52d0944509a23418f9e9af4a7765d84cac",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/ambient_geometry.wgsl",
      "sha256": "3168319a5398a270b4c89d13342e16bb2f4353f97fce0341a2d500728f9868ee",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/arrow.wgsl",
      "sha256": "73cd0ec57df2490b9b3099c56fbb9930c4667f8eb55080a400ca98b4e8c0b2a1",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/background.wgsl",
      "sha256": "ea62408626759db7cf4a3756d83dbbd5aead2621aeee028f4c54de4310ef2aa6",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/cap.wgsl",
      "sha256": "1892c0314d7ed98949f6f56711e2cb002ec29b2c1c1c46ff9c4b31357ef7bde9",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/clip.wgsl",
      "sha256": "56aae3cacb18a7c036e05833264577d8ee181f29c25af56d2f96ac2582df01d1",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/face_coverage.wgsl",
      "sha256": "e4b37b7fa30f5cf6237a0aa7e20287ad36b3cdd07893e43899e2bc33fb1ddd16",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/glyph.wgsl",
      "sha256": "7a1ed8abcaa5ab684ebe31cea2fbece63df6248057aed2e84d0ce8e9cc5e4471",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/grid.wgsl",
      "sha256": "67daedfc89ed5b5553d2061429a4d725d1bc12b7f8e503d04bf1d709befc21fb",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/ink_visibility.wgsl",
      "sha256": "3423dfccfef9d924b4329d0b973554dc47bb77b7f7725c9a045c5181398a055e",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/normals.wgsl",
      "sha256": "1776ddecbef8a7ca5454216697fb4d4aff07e631ea3452a885b6a47df8056733",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/physical.wgsl",
      "sha256": "197265d033815c44654368907825505fd55b03ee29cbb3ffa7bf04d3a4dd6c35",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/plane.wgsl",
      "sha256": "e9934e566c2cdb31ce648cd0f221f1ba7728df130f15fe8e69b8eba44d452312",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/project_triangles.wgsl",
      "sha256": "60f8ca735f51e5b977b0d977f55cf7f71fe5b841409ea4765de8dc6687867224",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/projected_triangle.wgsl",
      "sha256": "3e569eabf25978668964bec693e0e250d8e634900fdd4d14808db6338a815b73",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/ribbon.wgsl",
      "sha256": "f19427f01f8eeda0bd8200457f326717d972eaa4c8bc349c339c45243300f617",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/scan_triangle_tiles.wgsl",
      "sha256": "d04d5a98009c0d253b4e5ef5c5751b8ee48239878e800753eb99e5f856e02ed0",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/scene.wgsl",
      "sha256": "e17b1d992b7c986e48abececae70cb1be1edc356125a0ae1f2805cd6a6a73281",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/slot_table.wgsl",
      "sha256": "0837fcc793e41b6706c3e00b7dde58972fe4fa9364d3227c8f1f5a193a17fdcc",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/sphere.wgsl",
      "sha256": "4a4bbdb2853a6525beab037611cb9b2135ce42ad881154b8ab1d46c8e2c24a4a",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/splat.wgsl",
      "sha256": "38fa50b2d6aedcf8dd923f534a21b6f05fc4a94c32b6855e2d2ecd6b3559eda4",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/splat_resolve.wgsl",
      "sha256": "529a0f871e59615087bce401df552a09243406da12240c3c84c64663a9decc0a",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/ssao.wgsl",
      "sha256": "abc35b0b2547741b4d092f9778b74932a2e035b23125d7feddffe19a71bc765f",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/surface_outline.wgsl",
      "sha256": "446689ea45e5cf0fd03011250271a296eddf4942ad9e0dfd4432c8b57b089885",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/text_outline.wgsl",
      "sha256": "c06b165b85ee2a0bd20d0348f1aea3d1ce811a7fd76d66911380ef62fa8be03e",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/text_plane.wgsl",
      "sha256": "56c0dcf2431ff25d546d3b46dd6038823035744ac939c3895d7fcbe372073248",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/text_plate.wgsl",
      "sha256": "5721973099036446bcd3310cb4ea28eb522b109009236602f4c00d757900b4cd",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/triangle.wgsl",
      "sha256": "ea26805ce07269fb242562ea71dec4fee114794d5cf24d385c2f1c2e9516960b",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/triangle_tiles.wgsl",
      "sha256": "21bf7777d18bf7e506f97c0a31d4ab5a64bf6e062ffe0698dd67402806876c0f",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/vector.wgsl",
      "sha256": "b67a636f91ab079c58e80361c5ed8782764acd47e133045a94c50f412b72030a",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/widget.wgsl",
      "sha256": "4337efb0b24e1fa1116db7dc9fb6690e3745433168a704ff7edd71597416f8d0",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/state/box_select.rs",
      "sha256": "3b0fda742185f5e1fac8836aa094707ccf7a66cba115300b52e1b40520c5c07d",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/clipping.rs",
      "sha256": "0b49d749ae19ee39f8adf3a26607b3743b6e0ce7958eef7b86f56afba36d2148",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/cloud_query.rs",
      "sha256": "5aa517e86ec917b672f38e4feb4e698c19addf6dbcd498987e777eb9c8ada999",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/drag.rs",
      "sha256": "e94d90344c4cc68710cf5abcb2cdffc31aca7d7a53d0f3f49e093b0380a2e061",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/drawing.rs",
      "sha256": "002139c15068a0b99bf58f4e98c06ab76d639a9632e387e221b26ce6b403cebf",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/edit.rs",
      "sha256": "ae99d6455cb03aacc2402cb5d8a6c292894add723f88453d145cd4a27bf866bd",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/features.rs",
      "sha256": "346212c4f10751e092ab484d2e91988454c832fb39d1751d1cc365aa6ff86a53",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/hydrate.rs",
      "sha256": "5425a603e63bc9090e2f9e7cbc8e16afff75aa6a9382295fe1911b95a90e6b5f",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/number_box.rs",
      "sha256": "c32afcd959720c2010f60127f2be1315f468f502146935d39468e9fa5f88b9c1",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/panel.rs",
      "sha256": "3523ff5721284e190ebbe6b35c68851151a017e5a7af90ae76ac0ef467cf0bee",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/sheet_query.rs",
      "sha256": "fc2d5fff6c5e5d794ee31d4f438e71b87e1043f45b0c36439f837d7f0691d038",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/splitting.rs",
      "sha256": "a8d8586f1b203bc82040a37b1d52b314fd3f03a9f1a5abb766999ea76d6f4924",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/text.rs",
      "sha256": "48ea023bdaee730f0d55bff43ef28937d6370a973bbd0819b5e484c66f7ced68",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/tool.rs",
      "sha256": "044fd2c8f01f1ce7995d89f04022de261609f9e65e552b4d3485c74f190c2889",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state.rs",
      "sha256": "48b6e5f4c403d3b89a3f6ad691b28e815b8fce8c905f0e363aa1e3e346555cbb",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
    },
    {
      "root": "viewer",
      "path": "src/text_quality.rs",
      "sha256": "7fef20819989ace4919f20705cdda904656fe960ea557028e9a5dc7b34b68edc",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
    },
    {
      "root": "kernel",
      "path": "Cargo.lock",
      "sha256": "19a5db84f182cf5eb18e4ebc991abee9cda016caa7cecee98e1d30b680d961c5",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "Cargo.toml",
      "sha256": "07eee3412699f3b83805b845b7e8f1bf09550db774a2de28df399176f2c9a46b",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "build.rs",
      "sha256": "cd8d703eca80af4ef136c63956bd97202178a8a86910e5abdfbb0d5fe1f428ce",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/aabb.rs",
      "sha256": "c23fa5f21df2587c9bec613810599bd4f6dd5992e206e79531f1ede5d08710bb",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/aabb_test.rs",
      "sha256": "29307cbe6191102d62ec23f82c5b85e3c518b4c6ce98fa8cb1971d04cda473a5",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/bin/minitest.rs",
      "sha256": "c262cab354a0c2150ebd78880a48f0fbf854556a04ffa8143046835ea303be80",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/bin/pdf_import.rs",
      "sha256": "834253bd8462d1fed0d9f16a2ce1391f9c8422d5a469ef0583f7107e9e85fb9f",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/boolean_polyline.rs",
      "sha256": "bad8ebaefeecd46ebac35b33d54ecc9320191d5e6b03c4516ee316c61dca4f87",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/boolean_polyline_test.rs",
      "sha256": "df6abe260a9b1422b32e6c84e33c8f87276c6733b0932d1ece4f7b90fbfced8c",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/brep.rs",
      "sha256": "2eb96d33e16a294c197a63e8ee5d65759d8320394232b5fcabd49d0b02c5b84a",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/brep_test.rs",
      "sha256": "f074d549288699afb223a31ce63a4eee05f9270829e40cd860458ff2fd18f3a5",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/closest.rs",
      "sha256": "95f4e14a0421603076ce8f6bc4ba448a255a63ca46ca461ea6f1e69c52feba4d",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/closest_test.rs",
      "sha256": "cfa7376f74f4024f298e1c179bda6d5f8bde1caaa43a6722c31ec6809d58256b",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/collection.rs",
      "sha256": "b01397286ac7a827347580bba1ff884fd844d1c4cf502cc6c97e005ee8d7e617",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/collection_test.rs",
      "sha256": "88cb7eca30b51939868e842fe8ae000a547716fc2b503eb379d9f6d8006717f4",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/color.rs",
      "sha256": "cd560ba1d5b30fe4d2a5a89884117251e482fe08d61c7a63b6f135f270f8c3e8",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/color_test.rs",
      "sha256": "31ea2af6afd5d9590359c63a7498a624b6818b8d38c06506a74b41cce29bdd7c",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/convex_hull.rs",
      "sha256": "489528349623b9d49242e632e537e6956341f91a065a6f7b3ccb66d9fa3de541",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/convex_hull_test.rs",
      "sha256": "f97a384db885e6e12421fb349c6ee15f78a3b1eac13be6189d370327ccab0d01",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/element.rs",
      "sha256": "09ebffbf70831adf2bab9e1a25c61856ae12c14029cae22c71845db7f6b197f0",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/element_test.rs",
      "sha256": "94eb3a53a8ee5fd5324026a3b0111871633235d053303313f2322bc6e650d0b9",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/file_encoders.rs",
      "sha256": "d1ca6e177ced274a6ed795e61f8fdcab52c78b5143d7a4b3e88daa096cedcb7c",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/file_encoders_test.rs",
      "sha256": "b6127b52a4dc1ef11b7b6b6d047e3a1d5ae63fc2b934577457deec450d697d2c",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/file_obj.rs",
      "sha256": "ad31bfe1e318f33d1351a75862898b113f50609eac72801f5b8b6b2effe7f2b3",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/file_obj_test.rs",
      "sha256": "7484d9dcebc1a708be7895b49bb5c8d0712ece848835e80b54665376cdac221d",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/file_step.rs",
      "sha256": "20148416d9815d0ce5170f842c6de4f2a81f9ea10002eed207a418cd39a7f5ed",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/file_step_test.rs",
      "sha256": "72638e3a27de36f4b8d95eb44be8a5faa146575dd4c08ef513441fecfe3c4000",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/file_xyz.rs",
      "sha256": "227068f507310e6ed8abe2c722073ce51155387f48c2c804c90f9bcacf65bcd4",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/file_xyz_test.rs",
      "sha256": "b1c09fde3b71627a80929ffdbd8c7ba55440c90be67c8a4d66d0ccc44d39c140",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/graph.rs",
      "sha256": "d3be20d9978ccc02b7789f94d33cc73eaf612c5b7a2897eb1f22be1259e315e2",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/graph_test.rs",
      "sha256": "04c8ed991cce02fc7962828b8b0d2f8f85ba0433da32fbbc9591e7240e871f83",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/guid_serde.rs",
      "sha256": "0dda61bc6a049dbbcd077e22e1b28ac0bea66ff9fa8f0f3359f64d055c071310",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/history.rs",
      "sha256": "f2df00f9e8f1ae0c2e35fba779849f5d4cd2efdd9faae8f52a444943d6012ba3",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/history_test.rs",
      "sha256": "ebd6ba188af75ef60dfabc4b3da65a98a2cc69925736fca03c1c35e2d805dd5c",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/instance_ref.rs",
      "sha256": "1a7b50e2125cdeaa5fe2f93784b2b5e041a46601124f61ebe36c61e494d35d25",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/instance_ref_test.rs",
      "sha256": "02c668632f02f2a711296ab67d67fd137100b5fa28996328fa93b642b014c831",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/interaction.rs",
      "sha256": "e1747cd3b610bdeb8f3b4e258e31942b518ee1f78daa75d1c0c4b213b63fc048",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/interaction_test.rs",
      "sha256": "af845f98e0d67ebb03239f9c8aa3812c1106905dc1cd3e177782ba7ed466351e",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/intersection.rs",
      "sha256": "4abfa2bc740b4dc2763fc3e40189a60c909bf16100df5d027d39464fd7beeb66",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/intersection_test.rs",
      "sha256": "3bd6fb198b14c92b63dc611694f4e61d26791f7a266c1ad09dd703ff8b2a4ada",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/lib.rs",
      "sha256": "51f6a8c03945f43f639b4827a1cde307b929e2cc1f9dabc63fee45a9e327b1ec",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/line.rs",
      "sha256": "1b052733b7544605fc92a4a4a3d12b539fed176b069c003b7c29137214d3119b",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/line_test.rs",
      "sha256": "610aed482852c277f906b85d4bf5e2969b1f8f281e8beb2da863a73eedec1185",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/main.rs",
      "sha256": "c97d546a386a35c2bc0858eb24661bf8897035894a1858fb187dc8fb71165e87",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/matrix.rs",
      "sha256": "7f4c6bf3570a3c88da6f0ac3a59325a0902e4d6d07dbd0742faf9739f396cf73",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/matrix_test.rs",
      "sha256": "c74b06f07ac71dc0e9e5080eb6994dc2e6fc4bb91e4e93952cba304dd494bc5e",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/mesh.rs",
      "sha256": "9e58e7e17cb2429a4a4cdff237558356103c85954baf4764691c6096e91833bb",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/mesh_test.rs",
      "sha256": "24750286db18ea92e5d0bb8cac3138c9ebb3c4655c2fd32c9cef77901fdaae50",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/mini_test.rs",
      "sha256": "eea605bbd1ca0c869944aa86e9eaf1b8e8c9bb5a9bf58d8a8a58283242119e93",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/nurbscurve.rs",
      "sha256": "ae7e12cf67da47f8f674558ce2870868123d979e58b3fe099a8e2fc581b0df14",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/nurbscurve_test.rs",
      "sha256": "94c5ae70bbbe5922746b0ec4de1da3b9403a254638e9ddd0f3705a04e695d317",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/nurbsknot.rs",
      "sha256": "c24be2250ab44b64b4b9f37afb0f4650f2e911dc8b084b0dc67a720bbc45243d",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/nurbsknot_test.rs",
      "sha256": "3d51a23e3e4144dd81e59228d30781c3d6be879627b4160e587f7d5e657fa150",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/nurbssurface.rs",
      "sha256": "24da855f532214fe06c7c8f48e411b00806a6d7c9e9392b256563cbdf8d0c19a",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/nurbssurface_test.rs",
      "sha256": "ca096628ab4ea15c65d82de436596220b24d4b8cdd4c92c74df1792ad3a7a622",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/nurbssurface_trimmed.rs",
      "sha256": "a2a9ae122c047df3b2f926c1de9794b0e3a93bf83d316673ff8877d6e0e73661",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/nurbssurface_trimmed_test.rs",
      "sha256": "6fbc0f6e39df5f283e21205bc6836e2981119fca00965503855d86c150ab2ad0",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/obb.rs",
      "sha256": "dc230125ed4dfd0c273092d5ed701fbffd52e1290ac93e3c27539a5949ad23ca",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/obb_test.rs",
      "sha256": "5e568654f05fe24f8b789a69d57c283e6b0eeeecbe5c543479fc4fda61989a63",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/objects.rs",
      "sha256": "5d2bdaa6924e69c2723bfb811273e00002ae78c04fecd03d33abe1f66412918e",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/objects_test.rs",
      "sha256": "43acf66ccad9c77eef360b04f499dd44f37d6f5fad06ab473a8ca2b98562de48",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/pdf.rs",
      "sha256": "498e731dd7d1663688025254207c1d80df4b61694b9c276853816e2f79b26ec5",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/pdf_test.rs",
      "sha256": "e1a204d8687d1972c206037205350056ff2ad1f6ad0d101df79144adc218e716",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/picking_test.rs",
      "sha256": "5ce0445f4f40fb24779838a61f806ed2631f134a6c63beb0551b5981d27512b9",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/plane.rs",
      "sha256": "851c2a4041284507ea87e72005f6e1279070f2aada4870b09c1c770b957dcbd1",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/plane_test.rs",
      "sha256": "6d7ba45bd35be4b12f6ef2e838a685b37df302eb9816cc8e2b6cf9e3a5af10fb",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/point.rs",
      "sha256": "b34781c548eddbe08aaaf07c3bb90e5206edd8424cfed988ad7df02ec42bc04b",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/point_test.rs",
      "sha256": "8e6257e9d626fd71cebd96d29f89aff7864b828ca0defd050e70b37f086e3d60",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/pointcloud.rs",
      "sha256": "0cb8b3ab79e7e9947a583ae47cd498a00a63d01ba605df5c8b1fc9532cc68b24",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/pointcloud_test.rs",
      "sha256": "4bedbc144dc3a891e64f8a63b57fc658a497894ad66850781806cac839421689",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/polyline.rs",
      "sha256": "7d60104324a69a636d6b06e5478fd24c92be8704f02792672a2cef795dafb245",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/polyline_test.rs",
      "sha256": "262d7dfac7bc9ad57aef36572864fee97481bcbbbdde7a433ea799a001c713ce",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/primitives.rs",
      "sha256": "cf102029fc6b3d6f5d05de9f495334e1ccd862b592bb2e70c7c1617103d8d871",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/primitives_test.rs",
      "sha256": "fd13401c77292c0d9cc62b736e1f43dbf948f1e6d87b8c00d2f11c3f8402e8d5",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/proto/.gitattributes",
      "sha256": "a79691a93b46e49ce460c26ef22afcc03d6eca1e63bf2edbc20e96159510f6c9",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/proto/session_proto.rs",
      "sha256": "8d56e772d9d742137014857cfc8da51c47f63c86c1423b31f9022fd0e1a557ff",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/quaternion.rs",
      "sha256": "a27375664e0e418324308ccbaca754256d1f11346786b00fcfc74265e1b1c25d",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/quaternion_test.rs",
      "sha256": "66a067fd4acf7179f10f9a3d9cacf8c7ff8765384537a46bac43ff2bf8c6d998",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/remesh_cdt.rs",
      "sha256": "e9e366c91c1ed06d0382bd73b63f357df7e3d17f3e558c3167771f4358c59c47",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/remesh_cdt_test.rs",
      "sha256": "dcaeae17bfea503bb88a203aadec5003350d18f7334ca77b35060aa75c880e27",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/remesh_nurbssurface_adaptive.rs",
      "sha256": "7bff923021c72c65d4baa153952683bf8dca5a5f3d783838d0576d9dc1fe1084",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/remesh_nurbssurface_adaptive_test.rs",
      "sha256": "721823cdb23029a96f3ca1a363f2d9677d2434d0b3878ccf684c2d7bc50a38d8",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/remesh_nurbssurface_grid.rs",
      "sha256": "5d059020f5b0c604c71c019bd07f09ecfe970d51d69054fb5ffb939b98fad453",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/remesh_nurbssurface_grid_test.rs",
      "sha256": "0fb60e49e2fafbaebc6a1d6784958a260aceb3496617f762793709052209ef6f",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/render_mesh.rs",
      "sha256": "0ff23f70c2d793f13a33bad1e87a090269db57c88d5bd17be35380bc642c9dab",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/session.rs",
      "sha256": "0e79f5a0cd79f5091c046c65bab1716a6b14aa17a6cbfcb5481798af00a2a6ff",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/session_config.rs",
      "sha256": "68f3f4d11d562a81a66a38ba27e7cf620d1ae8793e8cd5d1cb91e2478fe0fd5f",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/session_config_test.rs",
      "sha256": "fca94b4145b8260f51250d78116eb8e3b91188b094c0553551858cd1fc78c004",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/session_test.rs",
      "sha256": "b9dd833467772fcdfdd352d4e60a880cd47727eee232cdd4b4d14d749ca05928",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_aabbtree.rs",
      "sha256": "1abe0e715da8389a3463a8335f3b5a29b62b1105b3e42c3fd03e6017d24a8872",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_aabbtree_test.rs",
      "sha256": "f70954ccac3ea600aead350f0733283421d21595a87bcbde19f96155a9a66506",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_bvh.rs",
      "sha256": "d73869d86c4c339fdd36f58d4f92517609adfc0052102f53b09f49735d4d16d6",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_bvh_test.rs",
      "sha256": "a30af59a56e8adc06bff7fade212dd8e1c7bae6be7f52ddefe731c6dc301649a",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_kdtree.rs",
      "sha256": "20184b911763cf1c531e7f4240f5dcaedcf2dc1c75efc183316c8385149f3b59",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_kdtree_test.rs",
      "sha256": "bfb31c7bab8a7d50ee05831078e76b9627fbde740b581d38ee68cbf91cc47924",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_octree.rs",
      "sha256": "99ad88d205dc2ba61d2e9ce71002d4ffc7ea4cff9d20b833137e94f35e678b00",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_octree_test.rs",
      "sha256": "68b23ec3f391ab3a30f44ec2eb98453ffba12dc72a2f066469bfa111fd6abf8e",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_rtree.rs",
      "sha256": "c5c3fff7576eb0df92d264f0f5258fe0781e031377649ed35f6f52fc5e852384",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_rtree_test.rs",
      "sha256": "ed1cbf26a626b0780c3d65ed8d50197f170ad7b94dbdd7fde1f46895c0e7b762",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/split.rs",
      "sha256": "9ab9e25fce719171842595c16ec27e992acc082da5a07103f3bf660fec93b94a",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/split_test.rs",
      "sha256": "22c07ef52e5323abe8bd51c80909d6f0c434ea3e3d97d25dd0498ff115fdd474",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/tolerance.rs",
      "sha256": "d5dab04656fbddb2a5dd47b670d172f53a316f9de3799cd214e7a2205bb407ab",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/tolerance_test.rs",
      "sha256": "e8c87898cc2413066003ce96b12eb9fc4beb979e34b31bdc2a285b66a39330a6",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/tree.rs",
      "sha256": "f353b0f1ede8a33828dc36c57441f0dc4f49d105c85312f509026c5ab2b28756",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/tree_test.rs",
      "sha256": "6896501557b233f6393a67a9f0a2b99b200ab9da7e51c63845270d4ff155731e",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/vector.rs",
      "sha256": "972b1be360e8073f1da6b974b7e35d45a6a8d76afd579407d39682d6920ca43a",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/vector_test.rs",
      "sha256": "57b7621e012b92917102553dd9db0571eeb1269c63379e70879001e968b33e16",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/xform.rs",
      "sha256": "fa68b59f28584e5266b61e0d547773487bfca509de67d3a3d1c76eb18d362b5b",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/xform_test.rs",
      "sha256": "192c281556e2c44bbd1ec567e6c6477635ad6b22954952cf637405171d5389fa",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "wood",
      "path": "Cargo.lock",
      "sha256": "61e22d4c3026e1d3a7a65fb5a5ddacdaeaca36b76a6cdba5455c6018b5b75689",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "Cargo.toml",
      "sha256": "6319241ba295b453fa0812f3a4b123b62523a46f1dfcb9adcb0a3c408626320d",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "build.rs",
      "sha256": "9e5a81d2d6f6df9b812b7372c6b8d2696b21c30de8439ea950e19b8a2d60def4",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/beam.rs",
      "sha256": "ba841fe3c55eae7cebf5a10dd50e0f55c77a9bc43947196422a6585e72afc296",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/beam_variable.rs",
      "sha256": "dc203e3737fea4969482bc183f71be77d7f924323b47c8e0c4e5087046c0831d",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/block.rs",
      "sha256": "fbd810e4fe5a98d9fe706f22c27c8af2ee21072e1c965d97c8c92f1a09479e4d",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/column.rs",
      "sha256": "e44ea0467821c7036eadd34c59d0f9a8a8a02b34d74d9f15cc996412a926927d",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/element.rs",
      "sha256": "0fae5a4eddf930a86f1ea7ae21d96d04488b38bce0399ed3b9f41aca19a11a8c",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/geometry.rs",
      "sha256": "19dd88c0840ec311b8f4d7561697ecb5143d8992b6dad74766b8e38d6a6f8490",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/lib.rs",
      "sha256": "e0f0fd7d599eeb27168e25083946496a38e5057e96f3797b6e088dbfb0694cdd",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/plate.rs",
      "sha256": "59058e25eca287d11e42dc065a4f1830d32958c4dbd11de29c23fd4b28a3c03e",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/proto/wood_proto.rs",
      "sha256": "916d5994429e36d418e863b721f9c1bfb8929316ab17e3af945a504e40785661",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/support.rs",
      "sha256": "9af8c322a9c8b396105b2b43104c9634950279a96ad7dcb76b04a7b86650fb60",
      "topic": "Element builders and features",
      "lessons": "92,96"
    }
  ],
  "captured": "2026-10-11"
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzf-prepare-shared-record-cap-picks-current-inputs.json"
```

</details>

</details>

<a id="checkpoint-96zzzzg-own-shared-record-cap-pick-resources"></a>

### Step 15 of 16 · Own shared record cap pick resources (`96zzzzg-own-shared-record-cap-pick-resources`)

- **Goal:** Reuse explicit plane and arena bindings under the existing pick lifetime.
- **Data Flow Trace:** `Pick preparation → bounded plane values and current bindings → existing pick lifetime`
- **Files You Touch:** `src/cap_geometry.rs` (*Reuse the same plane and arena resource owner for groups with retained pick bindings.*) · `src/cap_pick_gpu.rs` (*Keep cap-pick plane parameters and geometry groups under the existing pick lifetime.*) · `src/current_inventory.json` (*Copy the supplied checkpoint inventory after regeneration of Session protocol documentation comments. This records current inputs without changing the strict inventory check or historical evidence.*)

Let the cap geometry owner append its bindings after the existing pick resources. Each pick preparation invalidates immutable groups because crossing buffers or the crop can change. Retain bounded plane parameters under the existing pick owner and account their bytes once. The next lesson connects both crossing draws.

> **Self-Check Question:** *Why invalidate cached groups at pick preparation?*  
> **Answer:** A resized crossing buffer or changed crop requires a fresh immutable group, even when the arena buffers remain unchanged.

- **What You Should See:** Cap picking owns bounded shared geometry resources; the original uniform draws remain active.
- **Hands-On Lab Experiment:** Repeat a small pick, resize the viewport and pick again. Clear the scene and inspect released cap resources.

<details>
<summary>Implementation 96zzzzg-own-shared-record-cap-pick-resources: Own shared record cap pick resources</summary>

Reuse explicit plane and arena bindings under the existing pick lifetime.

Data flow: Pick preparation → bounded plane values and current bindings → existing pick lifetime.

### 1. `src/cap_geometry.rs`

Reuse the same plane and arena resource owner for groups with retained pick bindings. Append geometry bindings after existing pick resources; color keeps the original three entries.

<details>
<summary>Locate the existing block</summary>

```rust
use std::{collections::HashMap,rc::Rc};
use wgpu::util::DeviceExt;
#[derive(Default)]
pub struct Geometry {planes:Vec<wgpu::Buffer>,groups:HashMap<(usize,u32),(wgpu::Buffer,wgpu::Buffer,wgpu::BindGroup)>}
pub fn layout(device:&wgpu::Device)->wgpu::BindGroupLayout {
 use wgpu::{ShaderStages as S,BufferBindingType as B};device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("cap plane and arena"),entries:&[crate::tile_pipelines::entry(0,S::VERTEX,B::Storage{read_only:true}),crate::tile_pipelines::entry(1,S::VERTEX,B::Uniform),crate::tile_pipelines::entry(2,S::VERTEX,B::Storage{read_only:true})]})
}
impl Geometry {
 pub fn set_planes(&mut self,device:&wgpu::Device,count:u32)->Result<(),&'static str> {
  if count as usize>crate::clipping_uniform::MAX_PLANES{return Err("Too many cap planes");}if self.planes.len()==count as usize{return Ok(());}
  self.clear();self.planes=(0..count).map(|plane|{let bytes=[plane,0,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:Some("cap plane selector"),contents:&bytes,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_SRC})}).collect();Ok(())
 }
 pub fn group(&mut self,device:&wgpu::Device,layout:&wgpu::BindGroupLayout,geometry:&Rc<crate::gpu_geometry::GpuGeometry>,plane:u32)->Result<wgpu::BindGroup,&'static str> {
  let parameter=self.planes.get(plane as usize).ok_or("Missing cap plane")?;let key=(Rc::as_ptr(geometry)as usize,plane);let(vertices,indices)=geometry.buffers();
  if let Some((old_vertices,old_indices,group))=self.groups.get(&key){if *old_vertices==vertices&&*old_indices==indices{return Ok(group.clone());}}
  let group=crate::tile_pipelines::group(device,layout,&[&vertices,parameter,&indices]);self.groups.insert(key,(vertices,indices,group.clone()));Ok(group)
 }
 pub fn invalidate(&mut self){self.groups.clear();}
 pub fn clear(&mut self){*self=Self::default();}
 pub fn bytes(&self)->u64 {self.planes.iter().map(wgpu::Buffer::size).sum()}
}

#[cfg(test)]
mod tests {
 use super::*;
 #[test]
 fn planes_keep_current_arena_bindings_and_release() {
  let renderer=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=crate::text_gpu_support::GpuCtx::new(&renderer.device,&renderer.queue);let layout=layout(&ctx.device);let mut groups=Geometry::default();groups.set_planes(&ctx.device,2).unwrap();assert_eq!(groups.bytes(),32);
  for(plane,buffer)in groups.planes.iter().enumerate(){assert_eq!(crate::growing_buffer_tests::read(&ctx,buffer,16),[plane as u32,0,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());}
  let mut scene=crate::scene::Scene::empty();scene.insert(crate::prepared::PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap()).unwrap();let arena=Rc::new(std::cell::RefCell::new(crate::mesh_arena::Arena::new(&ctx)));let geometry=Rc::new(crate::gpu_geometry::GpuGeometry::shared(&ctx,scene.objects()[0].mesh.clone(),arena.clone()).unwrap());let old=groups.group(&ctx.device,&layout,&geometry,1).unwrap();assert_eq!(groups.group(&ctx.device,&layout,&geometry,1).unwrap(),old);assert!(groups.group(&ctx.device,&layout,&geometry,2).is_err());
  scene.insert(crate::prepared::PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();arena.borrow_mut().upload(&ctx,&scene.objects()[1].mesh).unwrap();let fresh=groups.group(&ctx.device,&layout,&geometry,1).unwrap();assert_ne!(fresh,old);assert_eq!(groups.group(&ctx.device,&layout,&geometry,1).unwrap(),fresh);
  assert!(groups.set_planes(&ctx.device,crate::clipping_uniform::MAX_PLANES as u32+1).is_err());assert_eq!(groups.bytes(),32);groups.invalidate();assert!(groups.groups.is_empty());groups.clear();assert_eq!(groups.bytes(),0);assert_eq!(groups.planes.capacity(),0);assert_eq!(groups.groups.capacity(),0);
 }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use std::{collections::HashMap,rc::Rc};
use wgpu::util::DeviceExt;
#[derive(Default)]
pub struct Geometry {planes:Vec<wgpu::Buffer>,groups:HashMap<(usize,u32),(wgpu::Buffer,wgpu::Buffer,wgpu::BindGroup)>}
pub fn layout(device:&wgpu::Device)->wgpu::BindGroupLayout {
 use wgpu::{ShaderStages as S,BufferBindingType as B};device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("cap plane and arena"),entries:&[crate::tile_pipelines::entry(0,S::VERTEX,B::Storage{read_only:true}),crate::tile_pipelines::entry(1,S::VERTEX,B::Uniform),crate::tile_pipelines::entry(2,S::VERTEX,B::Storage{read_only:true})]})
}
impl Geometry {
 pub fn set_planes(&mut self,device:&wgpu::Device,count:u32)->Result<(),&'static str> {
  if count as usize>crate::clipping_uniform::MAX_PLANES{return Err("Too many cap planes");}if self.planes.len()==count as usize{return Ok(());}
  self.clear();self.planes=(0..count).map(|plane|{let bytes=[plane,0,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:Some("cap plane selector"),contents:&bytes,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_SRC})}).collect();Ok(())
 }
 pub fn group(&mut self,device:&wgpu::Device,layout:&wgpu::BindGroupLayout,geometry:&Rc<crate::gpu_geometry::GpuGeometry>,plane:u32)->Result<wgpu::BindGroup,&'static str> {self.group_with(device,layout,geometry,plane,0,&[]) }
 pub fn group_with(&mut self,device:&wgpu::Device,layout:&wgpu::BindGroupLayout,geometry:&Rc<crate::gpu_geometry::GpuGeometry>,plane:u32,first:u32,extra:&[wgpu::BindGroupEntry])->Result<wgpu::BindGroup,&'static str> {
  let parameter=self.planes.get(plane as usize).ok_or("Missing cap plane")?;let key=(Rc::as_ptr(geometry)as usize,plane);let(vertices,indices)=geometry.buffers();
  if let Some((old_vertices,old_indices,group))=self.groups.get(&key){if *old_vertices==vertices&&*old_indices==indices{return Ok(group.clone());}}
  let buffers=[&vertices,parameter,&indices];let mut entries=extra.to_vec();entries.extend(buffers.into_iter().enumerate().map(|(i,buffer)|wgpu::BindGroupEntry{binding:first+i as u32,resource:buffer.as_entire_binding()}));let group=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("cap plane and current arena"),layout,entries:&entries});self.groups.insert(key,(vertices,indices,group.clone()));Ok(group)
 }
 pub fn invalidate(&mut self){self.groups.clear();}
 pub fn clear(&mut self){*self=Self::default();}
 pub fn bytes(&self)->u64 {self.planes.iter().map(wgpu::Buffer::size).sum()}
}

#[cfg(test)]
mod tests {
 use super::*;
 #[test]
 fn planes_keep_current_arena_bindings_and_release() {
  let renderer=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=crate::text_gpu_support::GpuCtx::new(&renderer.device,&renderer.queue);let layout=layout(&ctx.device);let mut groups=Geometry::default();groups.set_planes(&ctx.device,2).unwrap();assert_eq!(groups.bytes(),32);
  for(plane,buffer)in groups.planes.iter().enumerate(){assert_eq!(crate::growing_buffer_tests::read(&ctx,buffer,16),[plane as u32,0,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());}
  let mut scene=crate::scene::Scene::empty();scene.insert(crate::prepared::PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap()).unwrap();let arena=Rc::new(std::cell::RefCell::new(crate::mesh_arena::Arena::new(&ctx)));let geometry=Rc::new(crate::gpu_geometry::GpuGeometry::shared(&ctx,scene.objects()[0].mesh.clone(),arena.clone()).unwrap());let old=groups.group(&ctx.device,&layout,&geometry,1).unwrap();assert_eq!(groups.group(&ctx.device,&layout,&geometry,1).unwrap(),old);assert!(groups.group(&ctx.device,&layout,&geometry,2).is_err());
  scene.insert(crate::prepared::PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();arena.borrow_mut().upload(&ctx,&scene.objects()[1].mesh).unwrap();let fresh=groups.group(&ctx.device,&layout,&geometry,1).unwrap();assert_ne!(fresh,old);assert_eq!(groups.group(&ctx.device,&layout,&geometry,1).unwrap(),fresh);
  assert!(groups.set_planes(&ctx.device,crate::clipping_uniform::MAX_PLANES as u32+1).is_err());assert_eq!(groups.bytes(),32);groups.invalidate();assert!(groups.groups.is_empty());groups.clear();assert_eq!(groups.bytes(),0);assert_eq!(groups.planes.capacity(),0);assert_eq!(groups.groups.capacity(),0);
 }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzg-own-shared-record-cap-pick-resources-01.rs"
```

```text
--8<-- "journey/code/96zzzzg-own-shared-record-cap-pick-resources-02.rs"
```

</details>

### 2. `src/cap_pick_gpu.rs`

Keep cap-pick plane parameters and geometry groups under the existing pick lifetime. Initialize the resource owner without allocating a new arena. Invalidate immutable pick groups when crossing buffers or crop bindings can change, then prepare bounded plane values. Count plane parameters once; arena and borrowed crop allocations retain their original owners.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{cap_image::Image,pick_window::Window};
pub struct Pick{pipes:crate::cap_pick_pipeline::Pipelines,records:wgpu::Buffer,owners:wgpu::Buffer,frame:wgpu::Buffer,frame_group:wgpu::BindGroup,record_group:Option<wgpu::BindGroup>,owner_group:Option<wgpu::BindGroup>,target:Option<Image>,pub preparations:u64}
impl Pick{
 pub fn new(device:&wgpu::Device)->Self{let pipes=crate::cap_pick_pipeline::Pipelines::new(device);let buffer=|label,size,usage|device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size,usage:usage|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let records=buffer("bounded original cap crossings",16,wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_SRC);let owners=buffer("exact original cap owners",8,wgpu::BufferUsages::STORAGE);let frame=buffer("cap physical crop frame",32,wgpu::BufferUsages::UNIFORM);let frame_group=crate::gpu_layout::group(device,&pipes.frame,&frame);Self{pipes,records,owners,frame,frame_group,record_group:None,owner_group:None,target:None,preparations:0}}
 fn capacity(device:&wgpu::Device,needed:u64,buffer:&mut wgpu::Buffer,label:&str)->Result<(),&'static str>{if needed>u64::from(device.limits().max_storage_buffer_binding_size){return Err("Cap picking exceeds the device storage binding limit")}if buffer.size()<needed{*buffer=device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size:needed,usage:buffer.usage(),mapped_at_creation:false});}Ok(())}
 pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,window:Window,crop:&wgpu::Buffer,rows:&[[u32;2]],planes:u32,mode:crate::pick_sources::Mode)->Result<(),&'static str>{
  let size=[window.draw[2],window.draw[3]];let bytes=u64::from(size[0]).checked_mul(u64::from(size[1])).and_then(|n|n.checked_mul(u64::from(planes))).and_then(|n|n.checked_mul(16)).ok_or("Cap pick storage overflow")?;Self::capacity(device,bytes.max(16),&mut self.records,"bounded cap crossings")?;let owners=rows.iter().flatten().copied().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();Self::capacity(device,(owners.len()as u64).max(8),&mut self.owners,"original cap owner map")?;if !owners.is_empty(){queue.write_buffer(&self.owners,0,&owners);}
  let words=[window.full[0],window.full[1],size[0],size[1],window.draw[0],window.draw[1],mode as u32,0];queue.write_buffer(&self.frame,0,&words.into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());
  self.record_group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("cap record frame"),layout:&self.pipes.records,entries:&[wgpu::BindGroupEntry{binding:0,resource:self.records.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:crop.as_entire_binding()},wgpu::BindGroupEntry{binding:2,resource:self.frame.as_entire_binding()}]}));
  self.owner_group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("original cap owners"),layout:&self.pipes.owners,entries:&[wgpu::BindGroupEntry{binding:0,resource:self.records.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:self.owners.as_entire_binding()}]}));if self.target.as_ref().is_none_or(|t|t.size!=size){self.target=Some(Image::new(device,size,wgpu::TextureFormat::R8Unorm,1));}self.preparations+=1;Ok(())
 }
 pub fn encode(&self,encoder:&mut wgpu::CommandEncoder,image:&crate::pick_image::Image,window:Window,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,meshes:&[crate::gpu_mesh::GpuMesh],targets:&std::collections::HashMap<u32,u32>,planes:u32){
  let Some(target)=&self.target else{return};encoder.clear_buffer(&self.records,0,None);
  for pipeline in [&self.pipes.count,&self.pipes.nearest]{let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section count and nearest owner"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&target.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(pipeline);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clipping,&[]);pass.set_bind_group(3,self.record_group.as_ref().unwrap(),&[]);for plane in 0..planes{for mesh in meshes.iter().filter(|m|m.closed()&&targets.get(&m.id.value()).is_some_and(|mask|mask&(1<<plane)!=0)){mesh.bind_model(&mut pass);mesh.geometry.draw_range(&mut pass,plane..plane+1);}}}
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section integer owners"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:&image.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&self.pipes.ids);pass.set_bind_group(0,&self.frame_group,&[]);pass.set_bind_group(1,clipping,&[]);pass.set_bind_group(2,self.owner_group.as_ref().unwrap(),&[]);pass.draw(0..3,0..planes);let _=window;
 }
 pub fn bytes(&self)->u64{self.records.size()+self.owners.size()+self.frame.size()+self.target.as_ref().map_or(0,|t|t.bytes)}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{cap_image::Image,pick_window::Window};
pub struct Pick{geometry:crate::cap_geometry::Geometry,crop:Option<wgpu::Buffer>,pipes:crate::cap_pick_pipeline::Pipelines,records:wgpu::Buffer,owners:wgpu::Buffer,frame:wgpu::Buffer,frame_group:wgpu::BindGroup,record_group:Option<wgpu::BindGroup>,owner_group:Option<wgpu::BindGroup>,target:Option<Image>,pub preparations:u64}
impl Pick{
 pub fn new(device:&wgpu::Device)->Self{let pipes=crate::cap_pick_pipeline::Pipelines::new(device);let buffer=|label,size,usage|device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size,usage:usage|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let records=buffer("bounded original cap crossings",16,wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_SRC);let owners=buffer("exact original cap owners",8,wgpu::BufferUsages::STORAGE);let frame=buffer("cap physical crop frame",32,wgpu::BufferUsages::UNIFORM);let frame_group=crate::gpu_layout::group(device,&pipes.frame,&frame);Self{geometry:Default::default(),crop:None,pipes,records,owners,frame,frame_group,record_group:None,owner_group:None,target:None,preparations:0}}
 fn capacity(device:&wgpu::Device,needed:u64,buffer:&mut wgpu::Buffer,label:&str)->Result<(),&'static str>{if needed>u64::from(device.limits().max_storage_buffer_binding_size){return Err("Cap picking exceeds the device storage binding limit")}if buffer.size()<needed{*buffer=device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size:needed,usage:buffer.usage(),mapped_at_creation:false});}Ok(())}
 pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,window:Window,crop:&wgpu::Buffer,rows:&[[u32;2]],planes:u32,mode:crate::pick_sources::Mode)->Result<(),&'static str>{
  self.geometry.invalidate();self.geometry.set_planes(device,planes)?;self.crop=Some(crop.clone());
  let size=[window.draw[2],window.draw[3]];let bytes=u64::from(size[0]).checked_mul(u64::from(size[1])).and_then(|n|n.checked_mul(u64::from(planes))).and_then(|n|n.checked_mul(16)).ok_or("Cap pick storage overflow")?;Self::capacity(device,bytes.max(16),&mut self.records,"bounded cap crossings")?;let owners=rows.iter().flatten().copied().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();Self::capacity(device,(owners.len()as u64).max(8),&mut self.owners,"original cap owner map")?;if !owners.is_empty(){queue.write_buffer(&self.owners,0,&owners);}
  let words=[window.full[0],window.full[1],size[0],size[1],window.draw[0],window.draw[1],mode as u32,0];queue.write_buffer(&self.frame,0,&words.into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());
  self.record_group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("cap record frame"),layout:&self.pipes.records,entries:&[wgpu::BindGroupEntry{binding:0,resource:self.records.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:crop.as_entire_binding()},wgpu::BindGroupEntry{binding:2,resource:self.frame.as_entire_binding()}]}));
  self.owner_group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("original cap owners"),layout:&self.pipes.owners,entries:&[wgpu::BindGroupEntry{binding:0,resource:self.records.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:self.owners.as_entire_binding()}]}));if self.target.as_ref().is_none_or(|t|t.size!=size){self.target=Some(Image::new(device,size,wgpu::TextureFormat::R8Unorm,1));}self.preparations+=1;Ok(())
 }
 pub fn encode(&self,encoder:&mut wgpu::CommandEncoder,image:&crate::pick_image::Image,window:Window,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,meshes:&[crate::gpu_mesh::GpuMesh],targets:&std::collections::HashMap<u32,u32>,planes:u32){
  let Some(target)=&self.target else{return};encoder.clear_buffer(&self.records,0,None);
  for pipeline in [&self.pipes.count,&self.pipes.nearest]{let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section count and nearest owner"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&target.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(pipeline);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clipping,&[]);pass.set_bind_group(3,self.record_group.as_ref().unwrap(),&[]);for plane in 0..planes{for mesh in meshes.iter().filter(|m|m.closed()&&targets.get(&m.id.value()).is_some_and(|mask|mask&(1<<plane)!=0)){mesh.bind_model(&mut pass);mesh.geometry.draw_range(&mut pass,plane..plane+1);}}}
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section integer owners"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:&image.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&self.pipes.ids);pass.set_bind_group(0,&self.frame_group,&[]);pass.set_bind_group(1,clipping,&[]);pass.set_bind_group(2,self.owner_group.as_ref().unwrap(),&[]);pass.draw(0..3,0..planes);let _=window;
 }
 pub fn bytes(&self)->u64{self.geometry.bytes()+self.records.size()+self.owners.size()+self.frame.size()+self.target.as_ref().map_or(0,|t|t.bytes)}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzg-own-shared-record-cap-pick-resources-03.rs"
```

```text
--8<-- "journey/code/96zzzzg-own-shared-record-cap-pick-resources-04.rs"
```

```text
--8<-- "journey/code/96zzzzg-own-shared-record-cap-pick-resources-05.rs"
```

```text
--8<-- "journey/code/96zzzzg-own-shared-record-cap-pick-resources-06.rs"
```

</details>

<details>
<summary>Supplied data</summary>

Copy this supplied data once; it is excluded from the typing estimate.

### 3. `src/current_inventory.json`

Copy the supplied checkpoint inventory after regeneration of Session protocol documentation comments. This records current inputs without changing the strict inventory check or historical evidence.

<details>
<summary>Locate the existing block</summary>

```text
{
  "session_head": "31a84775e6bf72448f4ce5df34ae23528e1cf376",
  "wood_head": "07c57552c0d18c403bab0c16ba858d80561ffc47",
  "kernel_head": "b95a2c43ec7adf54c17c56d63522379e7122369e",
  "scope": "All files under the three Rust src trees and existing Cargo/build/web entries. Includes the preserved local rational-control change. Assets are separately hashed by the dependency evidence closure; C++ and generated models require their own provenance. Includes preserved October 10 hierarchy-path and parent Element-lamp changes; the original inventory remains historical. October 14 refresh records seven changed kernel/wood inputs and current repository heads; all viewer file hashes and the historical inventories remain unchanged.",
  "files": [
    {
      "root": "viewer",
      "path": "Cargo.lock",
      "sha256": "9b313409fa85751cba3224e7902022ee6a9420f7fdeaf69acf13c0ec62a6d9de",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
    },
    {
      "root": "viewer",
      "path": "Cargo.toml",
      "sha256": "f3c4a4e33279d03a069b041b21d5b25be8c5f2c1846adbf406c693fc40234002",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
    },
    {
      "root": "viewer",
      "path": "Trunk.toml",
      "sha256": "e36cf10ae1b9029bdcaa6b55468bef4dab8a3211db52763dcfc1e7521bebb9f8",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
    },
    {
      "root": "viewer",
      "path": "build.rs",
      "sha256": "868449c0976bf081f2a3bdf270c1ca0842f923b33a9b9e34dfcb0804c7f5363f",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
    },
    {
      "root": "viewer",
      "path": "index.html",
      "sha256": "fd308bc334190d76723135c1c82b4cd0d0343f3dff0dda573bf338aa5d3961b0",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
    },
    {
      "root": "viewer",
      "path": "src/app/agent.rs",
      "sha256": "555a699f8286aaefa426d3a0f9704e1444f9a4af2ccb7105c41c40dadcb5455b",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/clipping.rs",
      "sha256": "33047462339a5d4fe32742d38e1ca8e364aea126186fd107f1007fbf4158a8e1",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/cloud_query.rs",
      "sha256": "96e5ea670b083218e2105e0f2ffa0578cc5c25050b8ce3ee790d00dc3c401f0c",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/mod.rs",
      "sha256": "4e7ae69c626379e2e2ca8383a0cdec283ff59c4376bf4b09d3e5b27bf7546f52",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tests.rs",
      "sha256": "7b075fc2845ccdb68ac48a148d3c98ac9f54403896e21bb0b4f616430ffa1bcc",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tool/choices.rs",
      "sha256": "2b128f729985b18c0901ebe6dd8780d54abf701394b020da3c3408f3b2b3841f",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tool/cut.rs",
      "sha256": "d0d00438f5ec9230dd73cb19b14c3ad5cd65c043c294bc341c477e81f4ed5a5f",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tool/elements.rs",
      "sha256": "ced86184ffcb868f8677c92445378647ffbccb1ee1c6efc0f626873f904b3eff",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tool/gather.rs",
      "sha256": "acb5cc3b8b424c5c0889fed4ccb994f3a13da0d4de10dcc0944f05fd932d913a",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tool/options.rs",
      "sha256": "b86ccb79060e5159fbba2cd57d7b0fd22c14ca8472ab499f03bdbd5e1971e938",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tool/shape.rs",
      "sha256": "54c275cb3c8e5fbd4c0e3ea4a47910ca1ffcd5e92405d8055ff5088e05f049e7",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tool/surfacing.rs",
      "sha256": "ef9ef71fcb065485333be2ad358e832de162c3acb30a754a73009b22fc8df7ce",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/tool.rs",
      "sha256": "c08608743e48879cab82f76157d5f028215af1848cd5b9eeefc343f039001b1b",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/add_edge.rs",
      "sha256": "90e9e807c4a56b5b10bf5f1c3ad35326a5ffcda043043b62680068dc6b5585ed",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/add_group.rs",
      "sha256": "193f4e783390255a028dc24581a66659b2c82602e20a67f06d42d70540b8c10d",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/arctic.rs",
      "sha256": "04637eef3834bfb9c9d76d415d30f7803d857a267ad4472c91117dabbb374f39",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/area.rs",
      "sha256": "9951cf47f2ba308fca8b14e79d0b21dd9de84ec71328f50c808bac970571b73e",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/arrow.rs",
      "sha256": "cd80826f3f4170566ac9d72ee72fc24399c137a761282b1211392e1d2e187997",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/arrowhead.rs",
      "sha256": "33d52b895d61bd9754cfdbc590e867bb60ce7efa3dc8f8a208e99ff1b6114c87",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/attributes.rs",
      "sha256": "fb0300c25c2c5decebac7bfea8d7740e031fe58c16444c2bb36074f353bb7e19",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/block_with_hole.rs",
      "sha256": "ef3b9abac04888315e6cb616f95cad7a296ffef58c5f3d417ed812379b2816e2",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/bounding_box.rs",
      "sha256": "5baa3e329f91c9c6a2be572e4f8752e8f49effbbf564e3230bd00d8fbfb21479",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/box.rs",
      "sha256": "0f6b98541d82ea317acd703a136d8a5962bee64fe5f07150db36945c4443e986",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/capsule.rs",
      "sha256": "193edcbb4eff8c02c47ee461fdc0cee1ea7ad98777e882472dba3b5c1918e3f5",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/clipping_plane.rs",
      "sha256": "400dc33be2b869ae93c0a981c17116fba56672dc6326ac50efb23ff71e7bc294",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/close.rs",
      "sha256": "6ecb9cf60556c359550c1c85dab54700a98b31f9883dd9317e6d1129241a24c5",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/cone.rs",
      "sha256": "c3021c10760195345f67555ed9db7920c58a0a7c19ad925f1a9bcb5d912e0992",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/construction_plane.rs",
      "sha256": "078a4a7462a809f4b71978ac5579ecb8b9844eb41ccc9ad121a7efeab67aaa23",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/controls.rs",
      "sha256": "12bf2eb1ef670991804bbefcab39aeb3f450037d96981f973494f403b88fd909",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/copy.rs",
      "sha256": "eecc6b3b7ab0b379e8bfaa61e9be5210a13ab6e76917909ed6a0de6c34bcd257",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/curve.rs",
      "sha256": "00f85352d925c0196603c314a63090d359081f5b5aaafe635356d4ec1dc4ded1",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/cylinder.rs",
      "sha256": "8c69c4c8a45352d92b7963fa59a247594e67b9d5fee09b985f288fbdeb53744f",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/delete.rs",
      "sha256": "c573604cbdc1487b2b94c8430ae04b4a2dae3421b11a6fdf02176de202d4ec19",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/dodecahedron.rs",
      "sha256": "ce468793396a169f84b3c5454f931cfbeecfa6d07993604c1f4b7c192d0a431a",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/edge.rs",
      "sha256": "fe7666c920dddffd7b9e195121966ea519aba55b7716f86b26ae3788b05e90c7",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/element_beam.rs",
      "sha256": "b99b68f91c511f4fd1d8443b72de9c81670ba48ff7bebb9fdf10366845057e00",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/element_beam_variable.rs",
      "sha256": "b0898d92d19ec629a2c673cb7a64a452e1bc182e09a98cf66ac7d9d299ce6bba",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/element_block.rs",
      "sha256": "751684bcc9a7581e90fe2f2dc6f74b5f33fc0bda14a95ae9a4a10ee98d8ad601",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/element_column.rs",
      "sha256": "e3a32d979a0de748ea4440f488b0e9493cfb4238d6492eb25043fec32d2fefbb",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/element_plate.rs",
      "sha256": "0d984d66a5a49c347e6801ca7d0d8dace30ddec622ca9d4a02bd8046ad12ab07",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/element_support.rs",
      "sha256": "4781d8a89d967bcc99dd142eee41066f85b6cf99fd46347f36ae340716379ea6",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/escape.rs",
      "sha256": "0e7a4072d6a64c4e1b5c5b76cae3d3da6a563461379be9222ea035ad132965d1",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/explode.rs",
      "sha256": "353e44e0cc9953f0e06aead9eeb42cfc9605bbcb8caa72941b945d16f732c7ac",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/extend/reach.rs",
      "sha256": "37f9ed15f0b4148bebc6788829a6fc8bca4a558201348c145cdbd670d5e18a34",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/extend.rs",
      "sha256": "c86295b7fd4314a6ba86581c4ea12d142d0f867ff7698c4b542ce3b0ffe33de3",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/extrude.rs",
      "sha256": "ebc559cd931a5803c1236d521509e929fa11f0e3b987e5eaa0167f2cd0ff2428",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/face.rs",
      "sha256": "c5dc6a0907e7ce3239eb3396036c96c1370f2ff4e6a8df355d453660f8e05bc7",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/fit.rs",
      "sha256": "b1181698ef63b6facb5c50203928959d9b5f54087dea0e0e76cf065567ec719d",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/geometry.rs",
      "sha256": "d1612ae09503003fca3792ad9e4fcac3666069f7f3781ca8b59de7d9653ae586",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/grid_snap.rs",
      "sha256": "ab63e5d6f45c37d9b8b15ea8535d8386da1cdcce725b2a3398ccdc41e01b06c7",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/hide.rs",
      "sha256": "c4a8c1c4fb3d20b2768e3903841ccfae51c8fd8fb7d6b30b1ec3ab61cbdca800",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/icosahedron.rs",
      "sha256": "02407c66c611d0b1961e5726196cfe944082ab19fd4e49a11d22c7e40c458765",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/interactions.rs",
      "topic": "Element attributes and interactions",
      "lessons": "52–54,92,96,97",
      "sha256": "1a4836207a54e368ad3d91d6e53df5c089e4584b8ce53dc5bd7b0bcfe3750d0e"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/layers.rs",
      "sha256": "a0168bfe8c941fee7fba32c9a259c9f847640f24baa958d38383948843f6db6f",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/length.rs",
      "sha256": "64a425e2e952b2daa0ec729235f2702934bb81f472b17f8c140b26415a618204",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/length_bounding_box.rs",
      "sha256": "654498f77d15404be2ef63a4a6f96138186e2eb4d4799bae01310e8b89439fb3",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/line.rs",
      "sha256": "cce63f69a0a8c3dd422e67bf1fec62075bdd6aef55b0a9a79a6f53191d2a278e",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/loft.rs",
      "sha256": "0bd5c5e9169fc10b067629df1567ba2abfbae68aba97148b226e2ad16e96f818",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/measure.rs",
      "sha256": "004c72cbad5f8a24e7ae6141528997ce1e8deecb2e2caa22af68b34d5133c28d",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/measure_distance.rs",
      "sha256": "2b3813e899954595d609de0357984c5b5f01a8f7f031e6320ccffde088001421",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/mod.rs",
      "sha256": "7feb8f196ccecf0bf11c3222eb200dd3d00b8a58d9058fd57d4aa48ad973ff5c",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/move.rs",
      "sha256": "884f591bcde1cb18537f34b02a6a271e921fcd44aec30eb0f3da69017fd57b3a",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_curve_arc.rs",
      "sha256": "c6b50ac35e93520029884440498e0d17f9c2daf09e82c2b77ed3db66f372d7a1",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_curve_circle.rs",
      "sha256": "207decf5247f2fec9b84cf70ff0319e2f646b3af082459825628a098729718d1",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_curve_ellipse.rs",
      "sha256": "5140c1af03b1e011512258bc60901b04cde23222a780829b0328d0e7518dcce0",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_curve_parabola.rs",
      "sha256": "47f3b6a61efdf230fb1d03658b44aacd5268e69d1a359d1cb40d8654d21542b7",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_surface_4_points.rs",
      "sha256": "20464c6908c865053e38d7a9eb953e05f00cb02112262296a1fb36817bdf8981",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_surface_loft.rs",
      "sha256": "108be02d75b82e2f5ac6f34c13dd46b638ccee81314bca8ad6051970d16a588f",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_surface_network.rs",
      "sha256": "7f94d0f5074736b627f761b7b812ff4abc05592fe9ec4bf142410f2cc9643854",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_surface_revolve.rs",
      "sha256": "b447738bee31531180d016dabf2e77d93e30583c85812485114f7bf34850276c",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_surface_sweep1.rs",
      "sha256": "aa18f91aa0a35fb3b552f3bd41bad8e9a2ed565751254adc58b511904b137b84",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/nurbs_surface_sweep2.rs",
      "sha256": "750b15d8349ce66719eb982dfd85abd590a9f122cbbbf19b676d2ae2b5e04992",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/object.rs",
      "sha256": "fcce94137a4f68c8e9c54617caaca0f7ede2828780203cf8413ef5fff76813b8",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/octahedron.rs",
      "sha256": "203d8f4f5afb243be5ffcce6abdf3bf379d5abc8519b1c5b12aafaf10cca7f86",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/opacity.rs",
      "sha256": "0db647dcb96fa436c851edb2ef728df150fc8ef0681aac483481f0ff1c286ab8",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/open.rs",
      "sha256": "7b71238ac03238c84d483701c161649ecbabe1ed6681025c81408f8862f76b43",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/orient_3_points.rs",
      "sha256": "943bea38cb8607eef2153cbaa1e20260a2a09531bd5d469a2bc525ed35cc9f9d",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/outline.rs",
      "sha256": "537e232c2f91846dec99eb3e6e30e016f482577b1ef82a62c4f4f8a115566053",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/point.rs",
      "sha256": "d77b9be0208a33c75ed0134e99ca50815f2613fbbeb728adb33c30ad14306b0a",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/polyline.rs",
      "sha256": "15a7f496a783fe8864fbc1a090a1c3ea884a2cedff7320ecf0fb85136c59e93d",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/project_to_plane.rs",
      "sha256": "11dc267a1a5d36b974f4723fca487c38c958cc3ac2a0f926e81f99507e6904bb",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/pyramid.rs",
      "sha256": "a1de9a501bdaf8b12212a51da89160d5abb6d49f437529ed8582678ba31919cd",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/quad_sphere.rs",
      "sha256": "d801aa84e8d8c513d89483113c3076389b14c67eb613aae40f7b970af13d96ba",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/redo.rs",
      "sha256": "cc5ded7fc9d8c23b2e2638d5c48910a2ac6ad83a2815aa2976907d53511eded0",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/report.rs",
      "sha256": "8b8e73ef4da70c6221cbec41e75f54db70a5d39df05ee29c19b35efb8afe9a63",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/rotate.rs",
      "sha256": "814e405ba0eac925a1322fc88fa7918e0d436cc291f4ab322cb5ae6dca4911c8",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/save.rs",
      "sha256": "d2ed39ab8cc2f6755f5155a75269a1a097988799f161fe174587927e90084ad5",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/scale.rs",
      "sha256": "bc3f8280720976d74fd46bd4447da41ed63943c014dc62d2d75fbeada88ac5c2",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/select_all.rs",
      "sha256": "e263949a3a7f78ae516f1e3e4e0f53815a96547f0ffc39c0248cc258fd72ab8a",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/select_by_name.rs",
      "sha256": "90d070cb338b3a721a4501e777bf9ff048cfc368fe5d98fd1facbab18531001c",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/select_lasso.rs",
      "sha256": "dcef1e302ab9bea2353e2f85fa391b35117f6f7f97c153beb7901744a0571a7b",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/select_small.rs",
      "sha256": "d4591b944daf9036df7f6854083adf9fb1f593e410d61ee141f1cdd98ee4ab5e",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/selecting.rs",
      "sha256": "49b3172f3487531566fae3e4c955d9a438a0e153cf37d7f29a622bc070a48d27",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/show.rs",
      "sha256": "b55d0061a288fb54d8de3faf65e56df41a9ea98b3f2b353acb1802b9d52c9803",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/snap.rs",
      "sha256": "e85c98cc314cab74c8b9bcb74cf1d32429e49b49c76cf2c83f8a2e929b35603d",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/sphere.rs",
      "sha256": "75e2438143ed9c3f6875596c517211e14000f8525376bf166e52372e82f4ef91",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/split.rs",
      "sha256": "8c029c76b77546dd5fb8831693397b48ef16852ed1379253c2a028349f99b54a",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/tetrahedron.rs",
      "sha256": "f09e06bacd0c09b5b2c7b8b569c93267a4df5cf0fefab653868d154a0b02bd15",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/text.rs",
      "sha256": "7e0f56386df9eff853a325856005ba2670af975edbd7203e60a3dbb70bd030a7",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/torus.rs",
      "sha256": "c64075764f74caf608a647bcfe27aa1537c4b85cc091071cf3728c457f288c08",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/trim/parts.rs",
      "sha256": "d78d99b59d5da7cf85de9cff7d9b161ac01c3b107d6b50fd0a67966f4f4ba71b",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/trim.rs",
      "sha256": "60c27bce49c24b3867c658171267e1c159c24e0c1293de34067711f2c7c4b72d",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/undo.rs",
      "sha256": "6e82fd3e78bf7d00c006740fd49264bb5d83c18957758bec545fab9ca09210aa",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/view.rs",
      "sha256": "a025bf0d864aeb1bfc1c993df44e112118c86e290797a687f22935a442f30e39",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/command/verbs/volume.rs",
      "sha256": "310b7d44e53155c80a823aa411e758d3171a1c8b3a3bdbf273113c36430c0ad9",
      "topic": "Command grammar, defaults and tools",
      "lessons": "drawing/editing chapters,85,92,93,96"
    },
    {
      "root": "viewer",
      "path": "src/app/coords.rs",
      "sha256": "3b19ba73fee97ed22f48b7d5fe04bfc1090f1c31ca21c44265e936893a0e447c",
      "topic": "Camera, snapping and construction planes",
      "lessons": "26–27,72–76,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/cplane.rs",
      "sha256": "4cc6d3d6ddeed8baef51fd8e49ea8809156b9bdb8105b114ad1b906273e3ea26",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/decode.rs",
      "sha256": "b9156c5c746b33a6f4fd8a86a9897ef7c0f2ddadd252a3d2004eb21c404bf8fd",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/deform.rs",
      "sha256": "ed009f15ba5ace800914b14e78252b5990d76f001049fa667397ba361633959a",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/edit.rs",
      "sha256": "896328d97d69df9750d6f17c738deb0624ca3021e60fb8d468dc4e0fdb94e49e",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/feedback.rs",
      "sha256": "951457335d199ede0babce3425fb62466a0e717d12cee30816fd13f749058e53",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/fetch.rs",
      "sha256": "57d886ea78208f2bb19ffa13a1a252af18ccfddc7a17cd7c9432a981a1560fed",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/fonts.rs",
      "sha256": "d51582cbb4566c5efb172401510ed2d33675a20a0b63f0a54d9218fa7ee14e6a",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/gesture/box_select.rs",
      "sha256": "f52620890c23afa10a99d046a9985fccaa8b15639c972a46a0fae94a2bf88044",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/gesture/control.rs",
      "sha256": "e58ecb9d4da6bda5216ac2693a3d319be7f9fc386f80afb2d6e5f8df012b2f22",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/gesture/gizmo.rs",
      "sha256": "6ff5f1d9cbd6eb5830a0e80c46813699ec9a195bfc7f1476c342d064b6bd850c",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/gesture/mod.rs",
      "sha256": "a592ddc3964fa3b7732c08d9ac9339491980235e3f8273f9bcf75d0c043218e3",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/gesture/object.rs",
      "sha256": "009f27a8d77f8a767e90187456824324f27fb70d0256d30eb2af9503ad8806c5",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/gizmo.rs",
      "sha256": "397a00a8a1f4cda1f34ba244a40f5a842f27b1c83d7affc298d0f4276d396340",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/hierarchy.rs",
      "sha256": "2f3698997bebcf6ee66cb4e5a4290b796d309de877674b059b48ef92e2af7a03",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/input.rs",
      "sha256": "8dbcf36cc41b9ccb88a925c86c423396eaa572bd730e23d7b3f8df0e956392c1",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/inspection/source_memory.rs",
      "sha256": "0828d7ad8d7b3c0ec05422be79edec175739a05e7f6fe94341d77aa10e12541d",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/inspection.rs",
      "sha256": "157ba8e01eb1b68e7be7182fa92be0188ba17cae357ae3a922e43b8860aa0891",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/interactions.rs",
      "topic": "Element attributes and interactions",
      "lessons": "52–54,92,96,97",
      "sha256": "d0ff1f7f682dc03297e79054202c860598862d6e83db1e7033c858bfbcae5b9c"
    },
    {
      "root": "viewer",
      "path": "src/app/keys.rs",
      "sha256": "28abcbf79f108564e48b4c61ee96b8521b69062d467d2c5c2ad787e6c230289a",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/knobs.rs",
      "sha256": "b9dba07b897e6498d0a4210dca1b909365a82f46df32eb1a2e8dfd712889d99a",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/layers.rs",
      "sha256": "196e1a664f8d6db2611c1c0d581972741b864f8da80037b84a86c8f299cd4d3d",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/live.rs",
      "sha256": "c219787e600906040aedf875182a61635b03e64839c3797b6c9a4eddded7c9c4",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/live_revision.rs",
      "sha256": "91680efc998ea6d91d59bf46380fa938b3f1b9b8a3e9d4be7835096a6bb13c60",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/loader.rs",
      "sha256": "61e21d567eba5f336a45f8cae5b6e3d7c51438315cfb447ec73c9ff5604f50f7",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/manifest.rs",
      "sha256": "ab020e177195951fa7d07d8f132f93de2b41d23c51d6625d19486658baef8228",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/mesh_preview.rs",
      "sha256": "ec41fae6255735322acdb9f9e9cdd3891ba36ed1251b3f438678cc64891c8c4e",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/mod.rs",
      "sha256": "b74ca6bc7b9a0fc8c0889435b79e9e77de6bab4b953934267587f1c2a4a227fc",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/modeling.rs",
      "sha256": "7aa4202bc1756cee495ce0f2daefe9c2c320e651294322898c03624b761f9e04",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/range_gate.rs",
      "sha256": "7191b2de72b0b5e97074c977abbb9ee6b4d62c383f225c55148913528cb9ce1a",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/route.rs",
      "sha256": "009e6ae7e824edebe981be1d31c30dabafb89ae227aec188113f680bfa6dc2b9",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene.rs",
      "sha256": "22e53f5e48fdbfdffb296a266b2c29ae4dd25f309bb2d566a5041cd12bdeb52d",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_instances.rs",
      "sha256": "fa216bc9ebadbc25596c0e76711d49b72a9544317b265146f63614362ca39039",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_release.rs",
      "sha256": "b886d822f51516d904726d7ab2380447da7b8dd99f112c87d406ef54732fd95e",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_rows.rs",
      "sha256": "d21f7804c6a58979381774fff193155e16b9b96c43c5f2cd0022eb3e4f1c7fb6",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/allocation.rs",
      "sha256": "ecb2566170adffa7450aced660683e5bf6bb5e910f95a3d8161ee4359b29ad13",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/commands_tests.rs",
      "sha256": "ce45af59fc2375833c7292459056175568d73d1f678106f650fccf214ccaa65b",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/compaction.rs",
      "sha256": "b7f65a6f4d78543ee866bed81f96ad807df7b154411cef59234c39367bd7d52e",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/editing_tests.rs",
      "sha256": "0e8e6b90788ae1d45ddc2f03cbfaf5e807b1e7c9b47e66e97df70bbad8af8826",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/nodes.rs",
      "sha256": "9c70230b1bd744d5c476938d594ae2be256c3b39b815b147707cb05759ddfbd4",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/notes.rs",
      "sha256": "fa02d60b45d4f74823d75f6786780f7b24448cdf17664b9fdfa4e7f66f21a3ba",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/panel_tests.rs",
      "sha256": "4beba4f873594d0aa0dc1836720ad18739c0124a13ad0c0884fff5e4095f17c4",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/preview.rs",
      "sha256": "9df6f4543433ac9da0cdd853ae93ead478a40404a4b824096d22235c23341e8e",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/split_tests.rs",
      "sha256": "ef2a718cb61c31593c0d5d255604635ce33ad7a1a639f9e0a408ec21dd9c6b70",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/testing.rs",
      "sha256": "9716dce2aeebcced6e6aa72463950f0f4e1e17471942e23752b00ab52af943fa",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/tests.rs",
      "sha256": "30b78b84c2cd80a13cdb33f4cc68bac04645a05596b74216999df4e98a9d22bf",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync/tombs.rs",
      "sha256": "a5b5d0082c95e04309c1c61f262a315c2eb610999bf471af8c80246a2a76c7d3",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_sync.rs",
      "sha256": "7160ce61ef8ba840ef536bff9532ed315b7c2b613301dc7993b61105411d0126",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/scene_text.rs",
      "sha256": "b519d58bf6b0829e157f338300cf704f7986377cb1091294beb7f4c5e8863c8c",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/selection.rs",
      "sha256": "3d635d3895cb8b9446d1b0b6771e6377c625985a599574fef0eee8f56cbe83cb",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/session_io.rs",
      "sha256": "ca46739b5e8d4ff3fe5b5bd22b2d35ab76d0a55c0af3ec2a4ce38ec97c3b87be",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/sheet_query.rs",
      "sha256": "9ff2b40923abe62148dbf2ac355ea7f6db00a3a0402a6f2cf1dbaacaa6916ab7",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/snap.rs",
      "sha256": "174fe242e49e33d825acecf477d771a2ed4a7a0bdd7120c25938ee90241ec7cd",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/splitting.rs",
      "sha256": "50840e7f5750aa42a0e9058427dbea29beac8247b700b69e1fae7885f540b513",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/stream.rs",
      "sha256": "17b125e85e513f0d16ee7bba163a178a41901a9e4205886b9227729ee5203299",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/surface_preview.rs",
      "sha256": "7971672cbd085559288dac7495c9d6b57fe0518fe052c8d95bfa8949d8abe8f5",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/touch.rs",
      "sha256": "6e45e9c59952bcfb102bc98ac9cd0c243d042c52a623d330605ef6b1b3ac6460",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/command_line/tests.rs",
      "sha256": "f42ad65f70e9889f3d09e62bab1a04043f7f1d611f49ba7adb835394a3ed52e4",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/command_line.rs",
      "sha256": "e1465627dfb1b6afa326bbed8ce025de3ec0daa4ec4f0d34504099475e3d1bc0",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/graph.rs",
      "sha256": "6ed37b504001f80ab29a864567ff78fa8125a997cf4b179d538e8b580c7f11b9",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/layers.rs",
      "sha256": "3ed1b04e06c3b56d6ff39c0880be39c0944dcb4fc5847bce2adb127050da96ba",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/mod.rs",
      "sha256": "e281d789a462716213d8ec44b3faec0f9cca7abd65cdb6c3b1306895f9d97c59",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/number_box.rs",
      "sha256": "d38b57f74e9bb3ea749111bff86d8ef67fab2bcdd1293664c487bab08619b7e7",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/overlay.rs",
      "sha256": "99e66230fe357a9922134146603afb7043628bff4a0a4e69b16fe52863812739",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/phone.rs",
      "sha256": "40633a8dc3394a6898d30d324b19912f7002505720777a94a5d3fd64a3d32182",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/pointer.rs",
      "sha256": "7172927a19316efaa813fff3b3963d68fb5228e968f2c6e9ee55bb26cd60323e",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/ui/theme.rs",
      "sha256": "a45673d3ff48b2114846750a22e2ced36f22dbfd7b1e11752914ecbd54b71f3d",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/validate.rs",
      "sha256": "c7e6e4fbe33ea7af6898a18b92758df01f46cb43ea2aacd09d5da346a57623be",
      "topic": "Loading, source ownership, archives and routing",
      "lessons": "43–54,66–71,85,96"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/bounds.rs",
      "sha256": "257a8f622f4ba277e235ae73c69e59b14c8a6cc2ffc52b9ed07076e25868991b",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/brep.rs",
      "sha256": "e49dcc0911686b0f2f4fbd7c9aacc411bb989e45b5c0c1444544f087d886c285",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/brep_edges.rs",
      "sha256": "2a19dd35e93bf62103e9549fc16f9c9d0fe6859a68be424b970d60dea17b5d2d",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/brep_orient.rs",
      "sha256": "0778910bea71d83e47737d5d35100b38f18df8b9dca3d886f642b36125260638",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/brep_shapes.rs",
      "sha256": "c1621011e4006473015572ec811300241204838db2b96d4372b84a740e6fae2c",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/cloud.rs",
      "sha256": "813e12a5731d575101f15a1f18c5d5db2759ab400ba0d8e1701b7047f37207ce",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/curves.rs",
      "sha256": "4da38304e6eb4dbed57d0406fe36a9dc4da823eb4f044d4434160983792756d7",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/encode.rs",
      "sha256": "a925576401ada960e52a28d669f7bee634c5c4213cc74207846c3a5a22af3a83",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/frames.rs",
      "sha256": "1bf83056cc5e36a94514c86ab43ef13d7a10ad9b9de62cfc8df847e3dbd33d8f",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/mesh.rs",
      "sha256": "a070c4a31a303339687adfffd579446e31f33d44c4a3a5c4ca49cd216265e060",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/mesh_ink.rs",
      "sha256": "349f85de9cb23dcb928a80b520468c58e84c4a03a6fd6d60280d2ec10a85cef4",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/mesh_topology.rs",
      "sha256": "9a207b8d0152661e846568bbf5971c3c24fdf5b33e3d5bf4acf6b5e925846ddc",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/mod.rs",
      "sha256": "048bf6d1a2e1b67948b011d5e3ede466e32722991e8c765097308de070fea88b",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/plane.rs",
      "sha256": "9be8bd22c56cb84393584e3fa969723563805be0350b49db1a9aea04ec58c30d",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/points.rs",
      "sha256": "af7baa2ff846c8ede26274635f105bd34377cf2aa63576424ff53da843d38330",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/app/walk/sheet.rs",
      "sha256": "c5148556dd5a0ff2688c63316b89c53ac79fbb747bd8d70de706ed2fbbff115f",
      "topic": "Original source walking and shared display",
      "lessons": "55–71,92,94,95"
    },
    {
      "root": "viewer",
      "path": "src/camera.rs",
      "sha256": "2eb0d51723129bdea1cddc3f35760217923959afe2a50dfe1fc6e129c8a3d58f",
      "topic": "Camera, snapping and construction planes",
      "lessons": "26–27,72–76,85,96"
    },
    {
      "root": "viewer",
      "path": "src/command_dock/mod.rs",
      "sha256": "c41184b767cb4947bf33bfe8ded268a6d3c7185870a6fa71054a588c002a90d4",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/command_dock/theme.rs",
      "sha256": "2064473cf413d0be10ca1f71440db34e11912c30893a360f11e446a5ffa2099d",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/command_dock/view.rs",
      "sha256": "2b6c7b55d1257a7be5736edd22e69194655b6e14d4dfbffe5c1c01e86456f204",
      "topic": "Panels, command input and keyboard focus",
      "lessons": "03,35,85,96"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/ambient_warm.rs",
      "sha256": "94bb4b260a7875b4c82be81acd8131e3cee556020f3cc5145608ec33d6f52585",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/arena.rs",
      "sha256": "7f0d4e53f55d3a1bf384da7ac7781766b8faf4e9f7038ab0dc2c29bb7f6e6441",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/backdrop.rs",
      "sha256": "afb51eaef8ce003e22ab9b4ce0e0af2c1ee786c98b98ceb22992f2a3edf4d6bc",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/buffers.rs",
      "sha256": "f226f75be1899a1540a4f80d4491019aa873aeed811dc73d0573405927a13f2c",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/clip/editing_tests.rs",
      "sha256": "ad0a23dda8ba3b14a7f55db51001d3bd61d24ec51cbea0552babcce6e206044d",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/clip/pipelines.rs",
      "sha256": "483249eef637d0deff02d1e80d96ad5ab56247e18e76faeff2fdc26cd4fdab73",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/clip/tests.rs",
      "sha256": "9f7a926a7e0e57d623a226d9baf781287d0ad6da3c4b0c9108d2612fcdd8d691",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/clip.rs",
      "sha256": "e0b31a38015a8c5bc31c404261ae46757e0655774413de0fd5ab92bb2282d99a",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/cloud.rs",
      "sha256": "ebf520f190ed3924efa465cdeac6d743ee9c2f1db337067955ba08d96959e3a5",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/device.rs",
      "sha256": "6d8d81150175551c886ba7dc412b384c2ef9be30fbc9ffc3fff53cbff6f67a1c",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/faces.rs",
      "sha256": "b48b373d830c28cb9e9456e421390d7f326b4910c9608f487dcf1834be78ff71",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/frame.rs",
      "sha256": "2a492fe36f15db4201a687a1d06e120945d38410a075375fece6ec1d3d4323fb",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/glyphs.rs",
      "sha256": "f992838e16e6792174214bd4ab67040e33026b9d471fad5b988ebd0597716970",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/hull.rs",
      "sha256": "42571ee70a810306267f2f1574e593819231f5633e245dc64f5460d5ff8679ad",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/instance.rs",
      "sha256": "89198da7abf06d41040aae716580d539eab2eed5b4e68763f348c5b7de31bd70",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/instanced.rs",
      "sha256": "f6fe9bb471dcf30fcc81534c6b20ebfbc1755c9cb0cea0edadf6272ab9e673db",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/lane.rs",
      "sha256": "2e16f454ed7cbc29a0422d180ec3eb8aa9adbe7503f7425eadf24765dcfea838",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/lod.rs",
      "sha256": "75dc349e43f44ca3bbbcc26a88f7bc3d56189a4268b0a2e3523e7903e9148a08",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/mod.rs",
      "sha256": "994339d5f4eb05eaee30be2c0352c7033d4b686afd6cad0c1e2ea84204d06035",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/objects.rs",
      "sha256": "58abc890347f29a8d3deb9b7651e3a43b4444a1e1429bf3de830ceb54516e7d5",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/pacing.rs",
      "sha256": "9e5916950f6658ad02b28aae1e009cfdc4546c4a4c4ba24c59c9194a7675b427",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/pass.rs",
      "sha256": "cce6cc842567b136d01ab84d8ef1e39a5b149063128a5df932babf6950c80dc6",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/patch.rs",
      "sha256": "00d8fae175ca925c7e48b8304c702550b1633f56747b4130ef22d6218cd97100",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/pick.rs",
      "sha256": "8dae0bfee0fd18e17ab90b1c0a61cb028af4d87bac29badb72a08c0325ea65f0",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/planes.rs",
      "sha256": "f0e509a16d84aa675ffd8c6b335a2a4f6074f120690881e4b1eae202a22e1f61",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/present.rs",
      "sha256": "ff2820ea362b9fad33463c03007deb90a72f22bf84f68f9c53356f31b7e61f79",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/render.rs",
      "sha256": "48150fc2c3afa9a453af5fa0d0aa35768ed42ee57454a3c46ef0016a518dfabc",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/segments.rs",
      "sha256": "e0d9532d31ce5ffd592eff3c0ca47300e81aca229408fd39a7d248176118ad2a",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/slots.rs",
      "sha256": "ef303f3f3e534502a9d90561955d840c091dcf9b2f1f69fea4f5540fbbfc18fd",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/splat.rs",
      "sha256": "13c99ea37920f0dd4c39750deba7e57d274268b366c3cd56d2a938dec113fa3e",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/ssao/pipelines.rs",
      "sha256": "e1c9ee95daa6fa83ae924519bd7f07a759e3f8d00c939f3dd1bce749ded218f3",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/ssao/tests.rs",
      "sha256": "474fdf5ec95cab2ef4f6a6d2d19528c344e1f3abb4f94cec8ff7ce442fb2e5ae",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/ssao.rs",
      "sha256": "932d34fde6d1d92a37c449e9f3b7c921e3aed63308c7427ac72e080c6c909830",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/surface_outline/tests.rs",
      "sha256": "d669e0da39d775ce45b86a1b01b54f8969b288b92a7cdb76b2941575f8f8940d",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/surface_outline.rs",
      "sha256": "08e790732dcd0ff16abe734be9d13b63dc26e6825f9d1d7578645ee529e4db03",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/targets.rs",
      "sha256": "f16908f3fd299e876687d0894374a440820ed713d3adc57689765f9991d2f140",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/text.rs",
      "sha256": "ac22022ad4c3820c2a834ce060286094f24d7a0a7a30633eac4a26231cfa054a",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/text_outline.rs",
      "sha256": "8444903f4b6c8132375822bd8d95eb5e06b51e023cb3487a730e90c5b3b842a1",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/text_plane.rs",
      "sha256": "641063fe094b466de058a601013f47e3227955eb63ccc0d42564f1b0b8a6063c",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/text_plate.rs",
      "sha256": "b7ec607e1759845577f9c10596ebfea2377bef4f3777ced791f0592f6a71a9c1",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/timing.rs",
      "sha256": "9165356ffbe6773316ddcfa7bd4e68c7e7ecd36892eba36a7dcb9d8760ef8ccc",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/triangle_tiles.rs",
      "sha256": "9e337004028bc94be8f79c5007ad7d04711967c1ea5d093fa6b3cb882123e2c9",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/ui.rs",
      "sha256": "a74d2783ebbb7404e496ceeb01d62671f48018e3173837c66e0f83720cadb263",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/upload.rs",
      "sha256": "d8a88e7f7fcd03c316981bdaddee78ec6f6986be87e69bcf0f298c9b172dab5f",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/upload_padding.rs",
      "sha256": "f713558ae631388ee310c7d6e3e9178eba12f383111cf2fba5f9ccd184fa1f52",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/vectors.rs",
      "sha256": "539d407e8e529d9cd9d4c507f675221805506221a49177773571e51e02d7fa3d",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/view.rs",
      "sha256": "dec8a12c4a27401682d4a506590d107ffe762f1253af19774409e8d32e3879f1",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/widget.rs",
      "sha256": "7227605387d824dd13210d70101a818ed95f9525a000bca1dc796dd7d21d0104",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/gpu/widget_mesh.rs",
      "sha256": "89d73dd598c5a0f7639409e724f3df910c09f3ffcf21de16f0cbbd8e669c7306",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/mod.rs",
      "sha256": "f9d05dbaaa0d9b2f6289a368e9c06657159eb8d9a0880936acaa682f736d97d9",
      "topic": "Derived display, text and rendering infrastructure",
      "lessons": "core GPU chapters,60–65,94"
    },
    {
      "root": "viewer",
      "path": "src/engine/performance.rs",
      "sha256": "de679efdb2e8adf6e257324ce37d6026649654d7d4c4a7ab38dcf1b46d5bd5d8",
      "topic": "Frame pacing and performance",
      "lessons": "95"
    },
    {
      "root": "viewer",
      "path": "src/engine/pipelines/bindings.rs",
      "sha256": "7799e0467c095eea09730a0fc7c34847dfd8409dc1280ddda0bf171d35a77509",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/pipelines/layouts.rs",
      "sha256": "2152f1a6ab2fe4fd17e7d3a3eea01606dcab2a6b80ba69bf0760a216d42d3f45",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/pipelines/mod.rs",
      "sha256": "50590524bc796a7839dbf7fc075ad278909b53050fdb47c6e721d5d9f6c1bee5",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/engine/text.rs",
      "sha256": "918ec5b3bcc2a775c81e348d25b943299bc19d2d6ffd9b7ca647c10c9299591c",
      "topic": "Derived display, text and rendering infrastructure",
      "lessons": "core GPU chapters,60–65,94"
    },
    {
      "root": "viewer",
      "path": "src/lib.rs",
      "sha256": "12a3ae5e9c08583ed1088714889ba96bebd7fde315d0e4c0370692a2b12457ab",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
    },
    {
      "root": "viewer",
      "path": "src/selftest/lifecycle.rs",
      "sha256": "01744a1500c73dbeb25e4caa46aec8c82ee240ba706b76895363169dd864991a",
      "topic": "Maintained native rendering and correctness reference",
      "lessons": "94–96"
    },
    {
      "root": "viewer",
      "path": "src/selftest/oracle.rs",
      "sha256": "65e4c530ed8d341c7f1a4bbc5f2691861945e4828bb7056dc74e7202177455f3",
      "topic": "Maintained native rendering and correctness reference",
      "lessons": "94–96"
    },
    {
      "root": "viewer",
      "path": "src/selftest/profile.rs",
      "sha256": "6c8cfa7be644e9e527c1b4a289ca8d2afb10075843207e7ef15415a56b1d6bb1",
      "topic": "Maintained native rendering and correctness reference",
      "lessons": "94–96"
    },
    {
      "root": "viewer",
      "path": "src/selftest.rs",
      "sha256": "dec37fbc978002c1184181627769ac0c6daeec91da64b13373ef1d33bdaa08fc",
      "topic": "Maintained native rendering and correctness reference",
      "lessons": "94–96"
    },
    {
      "root": "viewer",
      "path": "src/shaders/ambient_composite.wgsl",
      "sha256": "15eab6b52a9dff22c38cb505ff7462b5b4cd6bdbe3e5223716902a231b9dd6d8",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/ambient_depth.wgsl",
      "sha256": "80258c80084418bc9211c6c31438cc52d0944509a23418f9e9af4a7765d84cac",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/ambient_geometry.wgsl",
      "sha256": "3168319a5398a270b4c89d13342e16bb2f4353f97fce0341a2d500728f9868ee",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/arrow.wgsl",
      "sha256": "73cd0ec57df2490b9b3099c56fbb9930c4667f8eb55080a400ca98b4e8c0b2a1",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/background.wgsl",
      "sha256": "ea62408626759db7cf4a3756d83dbbd5aead2621aeee028f4c54de4310ef2aa6",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/cap.wgsl",
      "sha256": "1892c0314d7ed98949f6f56711e2cb002ec29b2c1c1c46ff9c4b31357ef7bde9",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/clip.wgsl",
      "sha256": "56aae3cacb18a7c036e05833264577d8ee181f29c25af56d2f96ac2582df01d1",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/face_coverage.wgsl",
      "sha256": "e4b37b7fa30f5cf6237a0aa7e20287ad36b3cdd07893e43899e2bc33fb1ddd16",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/glyph.wgsl",
      "sha256": "7a1ed8abcaa5ab684ebe31cea2fbece63df6248057aed2e84d0ce8e9cc5e4471",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/grid.wgsl",
      "sha256": "67daedfc89ed5b5553d2061429a4d725d1bc12b7f8e503d04bf1d709befc21fb",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/ink_visibility.wgsl",
      "sha256": "3423dfccfef9d924b4329d0b973554dc47bb77b7f7725c9a045c5181398a055e",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/normals.wgsl",
      "sha256": "1776ddecbef8a7ca5454216697fb4d4aff07e631ea3452a885b6a47df8056733",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/physical.wgsl",
      "sha256": "197265d033815c44654368907825505fd55b03ee29cbb3ffa7bf04d3a4dd6c35",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/plane.wgsl",
      "sha256": "e9934e566c2cdb31ce648cd0f221f1ba7728df130f15fe8e69b8eba44d452312",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/project_triangles.wgsl",
      "sha256": "60f8ca735f51e5b977b0d977f55cf7f71fe5b841409ea4765de8dc6687867224",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/projected_triangle.wgsl",
      "sha256": "3e569eabf25978668964bec693e0e250d8e634900fdd4d14808db6338a815b73",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/ribbon.wgsl",
      "sha256": "f19427f01f8eeda0bd8200457f326717d972eaa4c8bc349c339c45243300f617",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/scan_triangle_tiles.wgsl",
      "sha256": "d04d5a98009c0d253b4e5ef5c5751b8ee48239878e800753eb99e5f856e02ed0",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/scene.wgsl",
      "sha256": "e17b1d992b7c986e48abececae70cb1be1edc356125a0ae1f2805cd6a6a73281",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/slot_table.wgsl",
      "sha256": "0837fcc793e41b6706c3e00b7dde58972fe4fa9364d3227c8f1f5a193a17fdcc",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/sphere.wgsl",
      "sha256": "4a4bbdb2853a6525beab037611cb9b2135ce42ad881154b8ab1d46c8e2c24a4a",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/splat.wgsl",
      "sha256": "38fa50b2d6aedcf8dd923f534a21b6f05fc4a94c32b6855e2d2ecd6b3559eda4",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/splat_resolve.wgsl",
      "sha256": "529a0f871e59615087bce401df552a09243406da12240c3c84c64663a9decc0a",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/ssao.wgsl",
      "sha256": "abc35b0b2547741b4d092f9778b74932a2e035b23125d7feddffe19a71bc765f",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/surface_outline.wgsl",
      "sha256": "446689ea45e5cf0fd03011250271a296eddf4942ad9e0dfd4432c8b57b089885",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/text_outline.wgsl",
      "sha256": "c06b165b85ee2a0bd20d0348f1aea3d1ce811a7fd76d66911380ef62fa8be03e",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/text_plane.wgsl",
      "sha256": "56c0dcf2431ff25d546d3b46dd6038823035744ac939c3895d7fcbe372073248",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/text_plate.wgsl",
      "sha256": "5721973099036446bcd3310cb4ea28eb522b109009236602f4c00d757900b4cd",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/triangle.wgsl",
      "sha256": "ea26805ce07269fb242562ea71dec4fee114794d5cf24d385c2f1c2e9516960b",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/triangle_tiles.wgsl",
      "sha256": "21bf7777d18bf7e506f97c0a31d4ab5a64bf6e062ffe0698dd67402806876c0f",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/vector.wgsl",
      "sha256": "b67a636f91ab079c58e80361c5ed8782764acd47e133045a94c50f412b72030a",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/shaders/widget.wgsl",
      "sha256": "4337efb0b24e1fa1116db7dc9fb6690e3745433168a704ff7edd71597416f8d0",
      "topic": "GPU pipelines and current rendering",
      "lessons": "86–94"
    },
    {
      "root": "viewer",
      "path": "src/state/box_select.rs",
      "sha256": "3b0fda742185f5e1fac8836aa094707ccf7a66cba115300b52e1b40520c5c07d",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/clipping.rs",
      "sha256": "0b49d749ae19ee39f8adf3a26607b3743b6e0ce7958eef7b86f56afba36d2148",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/cloud_query.rs",
      "sha256": "5aa517e86ec917b672f38e4feb4e698c19addf6dbcd498987e777eb9c8ada999",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/drag.rs",
      "sha256": "e94d90344c4cc68710cf5abcb2cdffc31aca7d7a53d0f3f49e093b0380a2e061",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/drawing.rs",
      "sha256": "002139c15068a0b99bf58f4e98c06ab76d639a9632e387e221b26ce6b403cebf",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/edit.rs",
      "sha256": "ae99d6455cb03aacc2402cb5d8a6c292894add723f88453d145cd4a27bf866bd",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/features.rs",
      "sha256": "346212c4f10751e092ab484d2e91988454c832fb39d1751d1cc365aa6ff86a53",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/hydrate.rs",
      "sha256": "5425a603e63bc9090e2f9e7cbc8e16afff75aa6a9382295fe1911b95a90e6b5f",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/number_box.rs",
      "sha256": "c32afcd959720c2010f60127f2be1315f468f502146935d39468e9fa5f88b9c1",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/panel.rs",
      "sha256": "3523ff5721284e190ebbe6b35c68851151a017e5a7af90ae76ac0ef467cf0bee",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/sheet_query.rs",
      "sha256": "fc2d5fff6c5e5d794ee31d4f438e71b87e1043f45b0c36439f837d7f0691d038",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/splitting.rs",
      "sha256": "a8d8586f1b203bc82040a37b1d52b314fd3f03a9f1a5abb766999ea76d6f4924",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/text.rs",
      "sha256": "48ea023bdaee730f0d55bff43ef28937d6370a973bbd0819b5e484c66f7ced68",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state/tool.rs",
      "sha256": "044fd2c8f01f1ce7995d89f04022de261609f9e65e552b4d3485c74f190c2889",
      "topic": "Interaction, selection and editing state",
      "lessons": "26–33,72–85,92,96"
    },
    {
      "root": "viewer",
      "path": "src/state.rs",
      "sha256": "48b6e5f4c403d3b89a3f6ad691b28e815b8fce8c905f0e363aa1e3e346555cbb",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
    },
    {
      "root": "viewer",
      "path": "src/text_quality.rs",
      "sha256": "7fef20819989ace4919f20705cdda904656fe960ea557028e9a5dc7b34b68edc",
      "topic": "Build, startup and application ownership",
      "lessons": "00–02,34–35,94,96"
    },
    {
      "root": "kernel",
      "path": "Cargo.lock",
      "sha256": "19a5db84f182cf5eb18e4ebc991abee9cda016caa7cecee98e1d30b680d961c5",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "Cargo.toml",
      "sha256": "86d0fd7c6296c4c98341014446c9298112a6f77a93a5033d46336ab711579db3",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "build.rs",
      "sha256": "cd8d703eca80af4ef136c63956bd97202178a8a86910e5abdfbb0d5fe1f428ce",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/aabb.rs",
      "sha256": "c23fa5f21df2587c9bec613810599bd4f6dd5992e206e79531f1ede5d08710bb",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/aabb_test.rs",
      "sha256": "29307cbe6191102d62ec23f82c5b85e3c518b4c6ce98fa8cb1971d04cda473a5",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/bin/minitest.rs",
      "sha256": "c262cab354a0c2150ebd78880a48f0fbf854556a04ffa8143046835ea303be80",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/bin/pdf_import.rs",
      "sha256": "834253bd8462d1fed0d9f16a2ce1391f9c8422d5a469ef0583f7107e9e85fb9f",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/boolean_polyline.rs",
      "sha256": "bad8ebaefeecd46ebac35b33d54ecc9320191d5e6b03c4516ee316c61dca4f87",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/boolean_polyline_test.rs",
      "sha256": "df6abe260a9b1422b32e6c84e33c8f87276c6733b0932d1ece4f7b90fbfced8c",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/brep.rs",
      "sha256": "2eb96d33e16a294c197a63e8ee5d65759d8320394232b5fcabd49d0b02c5b84a",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/brep_test.rs",
      "sha256": "f074d549288699afb223a31ce63a4eee05f9270829e40cd860458ff2fd18f3a5",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/closest.rs",
      "sha256": "95f4e14a0421603076ce8f6bc4ba448a255a63ca46ca461ea6f1e69c52feba4d",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/closest_test.rs",
      "sha256": "cfa7376f74f4024f298e1c179bda6d5f8bde1caaa43a6722c31ec6809d58256b",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/collection.rs",
      "sha256": "b01397286ac7a827347580bba1ff884fd844d1c4cf502cc6c97e005ee8d7e617",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/collection_test.rs",
      "sha256": "88cb7eca30b51939868e842fe8ae000a547716fc2b503eb379d9f6d8006717f4",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/color.rs",
      "sha256": "cd560ba1d5b30fe4d2a5a89884117251e482fe08d61c7a63b6f135f270f8c3e8",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/color_test.rs",
      "sha256": "31ea2af6afd5d9590359c63a7498a624b6818b8d38c06506a74b41cce29bdd7c",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/convex_hull.rs",
      "sha256": "489528349623b9d49242e632e537e6956341f91a065a6f7b3ccb66d9fa3de541",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/convex_hull_test.rs",
      "sha256": "f97a384db885e6e12421fb349c6ee15f78a3b1eac13be6189d370327ccab0d01",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/element.rs",
      "sha256": "09ebffbf70831adf2bab9e1a25c61856ae12c14029cae22c71845db7f6b197f0",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/element_test.rs",
      "sha256": "94eb3a53a8ee5fd5324026a3b0111871633235d053303313f2322bc6e650d0b9",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/file_encoders.rs",
      "sha256": "d1ca6e177ced274a6ed795e61f8fdcab52c78b5143d7a4b3e88daa096cedcb7c",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/file_encoders_test.rs",
      "sha256": "b6127b52a4dc1ef11b7b6b6d047e3a1d5ae63fc2b934577457deec450d697d2c",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/file_obj.rs",
      "sha256": "ad31bfe1e318f33d1351a75862898b113f50609eac72801f5b8b6b2effe7f2b3",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/file_obj_test.rs",
      "sha256": "7484d9dcebc1a708be7895b49bb5c8d0712ece848835e80b54665376cdac221d",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/file_step.rs",
      "sha256": "20148416d9815d0ce5170f842c6de4f2a81f9ea10002eed207a418cd39a7f5ed",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/file_step_test.rs",
      "sha256": "72638e3a27de36f4b8d95eb44be8a5faa146575dd4c08ef513441fecfe3c4000",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/file_xyz.rs",
      "sha256": "227068f507310e6ed8abe2c722073ce51155387f48c2c804c90f9bcacf65bcd4",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/file_xyz_test.rs",
      "sha256": "b1c09fde3b71627a80929ffdbd8c7ba55440c90be67c8a4d66d0ccc44d39c140",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/graph.rs",
      "sha256": "d3be20d9978ccc02b7789f94d33cc73eaf612c5b7a2897eb1f22be1259e315e2",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/graph_test.rs",
      "sha256": "04c8ed991cce02fc7962828b8b0d2f8f85ba0433da32fbbc9591e7240e871f83",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/guid_serde.rs",
      "sha256": "0dda61bc6a049dbbcd077e22e1b28ac0bea66ff9fa8f0f3359f64d055c071310",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/history.rs",
      "sha256": "f2df00f9e8f1ae0c2e35fba779849f5d4cd2efdd9faae8f52a444943d6012ba3",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/history_test.rs",
      "sha256": "ebd6ba188af75ef60dfabc4b3da65a98a2cc69925736fca03c1c35e2d805dd5c",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/instance_ref.rs",
      "sha256": "5f110dad59f0251883aca3b34f4f602ce88ac619a3afd95af75a68eb8a4c38fc",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/instance_ref_test.rs",
      "sha256": "02c668632f02f2a711296ab67d67fd137100b5fa28996328fa93b642b014c831",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/interaction.rs",
      "sha256": "e1747cd3b610bdeb8f3b4e258e31942b518ee1f78daa75d1c0c4b213b63fc048",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/interaction_test.rs",
      "sha256": "af845f98e0d67ebb03239f9c8aa3812c1106905dc1cd3e177782ba7ed466351e",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/intersection.rs",
      "sha256": "4abfa2bc740b4dc2763fc3e40189a60c909bf16100df5d027d39464fd7beeb66",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/intersection_test.rs",
      "sha256": "3bd6fb198b14c92b63dc611694f4e61d26791f7a266c1ad09dd703ff8b2a4ada",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/lib.rs",
      "sha256": "51f6a8c03945f43f639b4827a1cde307b929e2cc1f9dabc63fee45a9e327b1ec",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/line.rs",
      "sha256": "1b052733b7544605fc92a4a4a3d12b539fed176b069c003b7c29137214d3119b",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/line_test.rs",
      "sha256": "610aed482852c277f906b85d4bf5e2969b1f8f281e8beb2da863a73eedec1185",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/main.rs",
      "sha256": "c97d546a386a35c2bc0858eb24661bf8897035894a1858fb187dc8fb71165e87",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/matrix.rs",
      "sha256": "7f4c6bf3570a3c88da6f0ac3a59325a0902e4d6d07dbd0742faf9739f396cf73",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/matrix_test.rs",
      "sha256": "c74b06f07ac71dc0e9e5080eb6994dc2e6fc4bb91e4e93952cba304dd494bc5e",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/mesh.rs",
      "sha256": "9e58e7e17cb2429a4a4cdff237558356103c85954baf4764691c6096e91833bb",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/mesh_test.rs",
      "sha256": "24750286db18ea92e5d0bb8cac3138c9ebb3c4655c2fd32c9cef77901fdaae50",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/mini_test.rs",
      "sha256": "eea605bbd1ca0c869944aa86e9eaf1b8e8c9bb5a9bf58d8a8a58283242119e93",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/nurbscurve.rs",
      "sha256": "ae7e12cf67da47f8f674558ce2870868123d979e58b3fe099a8e2fc581b0df14",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/nurbscurve_test.rs",
      "sha256": "94c5ae70bbbe5922746b0ec4de1da3b9403a254638e9ddd0f3705a04e695d317",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/nurbsknot.rs",
      "sha256": "c24be2250ab44b64b4b9f37afb0f4650f2e911dc8b084b0dc67a720bbc45243d",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/nurbsknot_test.rs",
      "sha256": "3d51a23e3e4144dd81e59228d30781c3d6be879627b4160e587f7d5e657fa150",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/nurbssurface.rs",
      "sha256": "24da855f532214fe06c7c8f48e411b00806a6d7c9e9392b256563cbdf8d0c19a",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/nurbssurface_test.rs",
      "sha256": "ca096628ab4ea15c65d82de436596220b24d4b8cdd4c92c74df1792ad3a7a622",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/nurbssurface_trimmed.rs",
      "sha256": "a2a9ae122c047df3b2f926c1de9794b0e3a93bf83d316673ff8877d6e0e73661",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/nurbssurface_trimmed_test.rs",
      "sha256": "6fbc0f6e39df5f283e21205bc6836e2981119fca00965503855d86c150ab2ad0",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/obb.rs",
      "sha256": "dc230125ed4dfd0c273092d5ed701fbffd52e1290ac93e3c27539a5949ad23ca",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/obb_test.rs",
      "sha256": "5e568654f05fe24f8b789a69d57c283e6b0eeeecbe5c543479fc4fda61989a63",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/objects.rs",
      "sha256": "5d2bdaa6924e69c2723bfb811273e00002ae78c04fecd03d33abe1f66412918e",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/objects_test.rs",
      "sha256": "43acf66ccad9c77eef360b04f499dd44f37d6f5fad06ab473a8ca2b98562de48",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/pdf.rs",
      "sha256": "498e731dd7d1663688025254207c1d80df4b61694b9c276853816e2f79b26ec5",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/pdf_test.rs",
      "sha256": "e1a204d8687d1972c206037205350056ff2ad1f6ad0d101df79144adc218e716",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/picking_test.rs",
      "sha256": "5ce0445f4f40fb24779838a61f806ed2631f134a6c63beb0551b5981d27512b9",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/plane.rs",
      "sha256": "851c2a4041284507ea87e72005f6e1279070f2aada4870b09c1c770b957dcbd1",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/plane_test.rs",
      "sha256": "6d7ba45bd35be4b12f6ef2e838a685b37df302eb9816cc8e2b6cf9e3a5af10fb",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/point.rs",
      "sha256": "b34781c548eddbe08aaaf07c3bb90e5206edd8424cfed988ad7df02ec42bc04b",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/point_test.rs",
      "sha256": "8e6257e9d626fd71cebd96d29f89aff7864b828ca0defd050e70b37f086e3d60",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/pointcloud.rs",
      "sha256": "0cb8b3ab79e7e9947a583ae47cd498a00a63d01ba605df5c8b1fc9532cc68b24",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/pointcloud_test.rs",
      "sha256": "4bedbc144dc3a891e64f8a63b57fc658a497894ad66850781806cac839421689",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/polyline.rs",
      "sha256": "c466fd62352c9f4408fc722c060c993c79b5338ac1a4d6adcf4ce21954c64cb6",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/polyline_test.rs",
      "sha256": "9c77584b465a8edf79d6a40a1bf08fa9eb9843ff0c5e3b50cca98fa349a18050",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/primitives.rs",
      "sha256": "cf102029fc6b3d6f5d05de9f495334e1ccd862b592bb2e70c7c1617103d8d871",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/primitives_test.rs",
      "sha256": "fd13401c77292c0d9cc62b736e1f43dbf948f1e6d87b8c00d2f11c3f8402e8d5",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/proto/.gitattributes",
      "sha256": "a79691a93b46e49ce460c26ef22afcc03d6eca1e63bf2edbc20e96159510f6c9",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/proto/session_proto.rs",
      "sha256": "8d56e772d9d742137014857cfc8da51c47f63c86c1423b31f9022fd0e1a557ff",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/quaternion.rs",
      "sha256": "a27375664e0e418324308ccbaca754256d1f11346786b00fcfc74265e1b1c25d",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/quaternion_test.rs",
      "sha256": "66a067fd4acf7179f10f9a3d9cacf8c7ff8765384537a46bac43ff2bf8c6d998",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/remesh_cdt.rs",
      "sha256": "e9e366c91c1ed06d0382bd73b63f357df7e3d17f3e558c3167771f4358c59c47",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/remesh_cdt_test.rs",
      "sha256": "dcaeae17bfea503bb88a203aadec5003350d18f7334ca77b35060aa75c880e27",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/remesh_nurbssurface_adaptive.rs",
      "sha256": "7bff923021c72c65d4baa153952683bf8dca5a5f3d783838d0576d9dc1fe1084",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/remesh_nurbssurface_adaptive_test.rs",
      "sha256": "721823cdb23029a96f3ca1a363f2d9677d2434d0b3878ccf684c2d7bc50a38d8",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/remesh_nurbssurface_grid.rs",
      "sha256": "5d059020f5b0c604c71c019bd07f09ecfe970d51d69054fb5ffb939b98fad453",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/remesh_nurbssurface_grid_test.rs",
      "sha256": "0fb60e49e2fafbaebc6a1d6784958a260aceb3496617f762793709052209ef6f",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/render_mesh.rs",
      "sha256": "0ff23f70c2d793f13a33bad1e87a090269db57c88d5bd17be35380bc642c9dab",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/session.rs",
      "sha256": "30848b72bf983420353ec16e75d03ded78ae2876cf1433a915454898e3a2f65d",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/session_config.rs",
      "sha256": "68f3f4d11d562a81a66a38ba27e7cf620d1ae8793e8cd5d1cb91e2478fe0fd5f",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/session_config_test.rs",
      "sha256": "fca94b4145b8260f51250d78116eb8e3b91188b094c0553551858cd1fc78c004",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/session_test.rs",
      "sha256": "b9dd833467772fcdfdd352d4e60a880cd47727eee232cdd4b4d14d749ca05928",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_aabbtree.rs",
      "sha256": "1abe0e715da8389a3463a8335f3b5a29b62b1105b3e42c3fd03e6017d24a8872",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_aabbtree_test.rs",
      "sha256": "f70954ccac3ea600aead350f0733283421d21595a87bcbde19f96155a9a66506",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_bvh.rs",
      "sha256": "d73869d86c4c339fdd36f58d4f92517609adfc0052102f53b09f49735d4d16d6",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_bvh_test.rs",
      "sha256": "a30af59a56e8adc06bff7fade212dd8e1c7bae6be7f52ddefe731c6dc301649a",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_kdtree.rs",
      "sha256": "20184b911763cf1c531e7f4240f5dcaedcf2dc1c75efc183316c8385149f3b59",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_kdtree_test.rs",
      "sha256": "bfb31c7bab8a7d50ee05831078e76b9627fbde740b581d38ee68cbf91cc47924",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_octree.rs",
      "sha256": "99ad88d205dc2ba61d2e9ce71002d4ffc7ea4cff9d20b833137e94f35e678b00",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_octree_test.rs",
      "sha256": "68b23ec3f391ab3a30f44ec2eb98453ffba12dc72a2f066469bfa111fd6abf8e",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_rtree.rs",
      "sha256": "c5c3fff7576eb0df92d264f0f5258fe0781e031377649ed35f6f52fc5e852384",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/spatial_rtree_test.rs",
      "sha256": "ed1cbf26a626b0780c3d65ed8d50197f170ad7b94dbdd7fde1f46895c0e7b762",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/split.rs",
      "sha256": "9ab9e25fce719171842595c16ec27e992acc082da5a07103f3bf660fec93b94a",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/split_test.rs",
      "sha256": "22c07ef52e5323abe8bd51c80909d6f0c434ea3e3d97d25dd0498ff115fdd474",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/tolerance.rs",
      "sha256": "d5dab04656fbddb2a5dd47b670d172f53a316f9de3799cd214e7a2205bb407ab",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/tolerance_test.rs",
      "sha256": "e8c87898cc2413066003ce96b12eb9fc4beb979e34b31bdc2a285b66a39330a6",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/tree.rs",
      "sha256": "f353b0f1ede8a33828dc36c57441f0dc4f49d105c85312f509026c5ab2b28756",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/tree_test.rs",
      "sha256": "6896501557b233f6393a67a9f0a2b99b200ab9da7e51c63845270d4ff155731e",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/vector.rs",
      "sha256": "972b1be360e8073f1da6b974b7e35d45a6a8d76afd579407d39682d6920ca43a",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/vector_test.rs",
      "sha256": "57b7621e012b92917102553dd9db0571eeb1269c63379e70879001e968b33e16",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/xform.rs",
      "sha256": "fa68b59f28584e5266b61e0d547773487bfca509de67d3a3d1c76eb18d362b5b",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "kernel",
      "path": "src/xform_test.rs",
      "sha256": "192c281556e2c44bbd1ec567e6c6477635ad6b22954952cf637405171d5389fa",
      "topic": "Original geometry and serialization",
      "lessons": "source chapters,96"
    },
    {
      "root": "wood",
      "path": "Cargo.lock",
      "sha256": "61e22d4c3026e1d3a7a65fb5a5ddacdaeaca36b76a6cdba5455c6018b5b75689",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "Cargo.toml",
      "sha256": "6319241ba295b453fa0812f3a4b123b62523a46f1dfcb9adcb0a3c408626320d",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "build.rs",
      "sha256": "9e5a81d2d6f6df9b812b7372c6b8d2696b21c30de8439ea950e19b8a2d60def4",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/beam.rs",
      "sha256": "4e6ffd383e74febf21ee80a337db1c18651307c23f0a4ea2471a790c28769f07",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/beam_variable.rs",
      "sha256": "dc203e3737fea4969482bc183f71be77d7f924323b47c8e0c4e5087046c0831d",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/block.rs",
      "sha256": "fbd810e4fe5a98d9fe706f22c27c8af2ee21072e1c965d97c8c92f1a09479e4d",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/column.rs",
      "sha256": "e44ea0467821c7036eadd34c59d0f9a8a8a02b34d74d9f15cc996412a926927d",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/element.rs",
      "sha256": "0fae5a4eddf930a86f1ea7ae21d96d04488b38bce0399ed3b9f41aca19a11a8c",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/geometry.rs",
      "sha256": "19dd88c0840ec311b8f4d7561697ecb5143d8992b6dad74766b8e38d6a6f8490",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/lib.rs",
      "sha256": "e0f0fd7d599eeb27168e25083946496a38e5057e96f3797b6e088dbfb0694cdd",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/plate.rs",
      "sha256": "59058e25eca287d11e42dc065a4f1830d32958c4dbd11de29c23fd4b28a3c03e",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/proto/wood_proto.rs",
      "sha256": "ae7367198c7f8fe9cfc05e08a90818e18b6a2e340ad3daf90e279b0ca417d870",
      "topic": "Element builders and features",
      "lessons": "92,96"
    },
    {
      "root": "wood",
      "path": "src/support.rs",
      "sha256": "9af8c322a9c8b396105b2b43104c9634950279a96ad7dcb76b04a7b86650fb60",
      "topic": "Element builders and features",
      "lessons": "92,96"
    }
  ],
  "captured": "2026-10-14"
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzg-own-shared-record-cap-pick-resources-current-inputs.json"
```

</details>

</details>

<a id="checkpoint-96zzzzh-pick-caps-through-shared-records"></a>

### Step 16 of 16 · Pick caps through shared records (`96zzzzh-pick-caps-through-shared-records`)

- **Goal:** Connect both cap crossing passes to current records and retain original owners.
- **Data Flow Trace:** `Current record and explicit plane → atomic crossings → original integer cap owner`
- **Files You Touch:** `src/cap_pick_gpu.rs` (*Prepare immutable current arena groups before encoding either crossing pass.*) · `src/caps_gpu.rs` (*Thread the same optional object owner through cap picking.*) · `src/pick_encode.rs` (*Share object records with first picks, source faces and cap picks in the existing submission.*) · `src/cap_pick_tests.rs` (*Supply full-u32 shared/fallback owner comparison and restoration before a color frame, retaining all original geometry assertions.*)

Prepare each eligible object and plane group once. Both counting and nearest-owner passes pull current arena positions through the object slot, with an explicit plane independent of instance numbering. Keep the atomic crossing algorithms and final integer-owner pass. Changed packed records use the existing uniform fallback.

> **Self-Check Question:** *Why share geometry groups between both crossing passes?*  
> **Answer:** They read the same object and plane during one pick; only the fragment operation changes.

- **What You Should See:** Cap picking consumes shared object records and retains original full integer identities, clipping and crop behavior.
- **Hands-On Lab Experiment:** Pick a cap before drawing a color frame, then pick the corner shared by two clipping planes. Repeat after selection and scene replacement.

<details>
<summary>Implementation 96zzzzh-pick-caps-through-shared-records: Pick caps through shared records</summary>

Connect both cap crossing passes to current records and retain original owners.

Data flow: Current record and explicit plane → atomic crossings → original integer cap owner.

### 1. `src/cap_pick_gpu.rs`

Prepare immutable current arena groups before encoding either crossing pass. Use the existing renderer record owner without introducing another table. Prepare exact target-plane groups once and reuse them across count and nearest-owner passes. Count and resolve owners from current records, retaining the original uniform path for changed packed records.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{cap_image::Image,pick_window::Window};
pub struct Pick{geometry:crate::cap_geometry::Geometry,crop:Option<wgpu::Buffer>,pipes:crate::cap_pick_pipeline::Pipelines,records:wgpu::Buffer,owners:wgpu::Buffer,frame:wgpu::Buffer,frame_group:wgpu::BindGroup,record_group:Option<wgpu::BindGroup>,owner_group:Option<wgpu::BindGroup>,target:Option<Image>,pub preparations:u64}
impl Pick{
 pub fn new(device:&wgpu::Device)->Self{let pipes=crate::cap_pick_pipeline::Pipelines::new(device);let buffer=|label,size,usage|device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size,usage:usage|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let records=buffer("bounded original cap crossings",16,wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_SRC);let owners=buffer("exact original cap owners",8,wgpu::BufferUsages::STORAGE);let frame=buffer("cap physical crop frame",32,wgpu::BufferUsages::UNIFORM);let frame_group=crate::gpu_layout::group(device,&pipes.frame,&frame);Self{geometry:Default::default(),crop:None,pipes,records,owners,frame,frame_group,record_group:None,owner_group:None,target:None,preparations:0}}
 fn capacity(device:&wgpu::Device,needed:u64,buffer:&mut wgpu::Buffer,label:&str)->Result<(),&'static str>{if needed>u64::from(device.limits().max_storage_buffer_binding_size){return Err("Cap picking exceeds the device storage binding limit")}if buffer.size()<needed{*buffer=device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size:needed,usage:buffer.usage(),mapped_at_creation:false});}Ok(())}
 pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,window:Window,crop:&wgpu::Buffer,rows:&[[u32;2]],planes:u32,mode:crate::pick_sources::Mode)->Result<(),&'static str>{
  self.geometry.invalidate();self.geometry.set_planes(device,planes)?;self.crop=Some(crop.clone());
  let size=[window.draw[2],window.draw[3]];let bytes=u64::from(size[0]).checked_mul(u64::from(size[1])).and_then(|n|n.checked_mul(u64::from(planes))).and_then(|n|n.checked_mul(16)).ok_or("Cap pick storage overflow")?;Self::capacity(device,bytes.max(16),&mut self.records,"bounded cap crossings")?;let owners=rows.iter().flatten().copied().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();Self::capacity(device,(owners.len()as u64).max(8),&mut self.owners,"original cap owner map")?;if !owners.is_empty(){queue.write_buffer(&self.owners,0,&owners);}
  let words=[window.full[0],window.full[1],size[0],size[1],window.draw[0],window.draw[1],mode as u32,0];queue.write_buffer(&self.frame,0,&words.into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());
  self.record_group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("cap record frame"),layout:&self.pipes.records,entries:&[wgpu::BindGroupEntry{binding:0,resource:self.records.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:crop.as_entire_binding()},wgpu::BindGroupEntry{binding:2,resource:self.frame.as_entire_binding()}]}));
  self.owner_group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("original cap owners"),layout:&self.pipes.owners,entries:&[wgpu::BindGroupEntry{binding:0,resource:self.records.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:self.owners.as_entire_binding()}]}));if self.target.as_ref().is_none_or(|t|t.size!=size){self.target=Some(Image::new(device,size,wgpu::TextureFormat::R8Unorm,1));}self.preparations+=1;Ok(())
 }
 pub fn encode(&self,encoder:&mut wgpu::CommandEncoder,image:&crate::pick_image::Image,window:Window,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,meshes:&[crate::gpu_mesh::GpuMesh],targets:&std::collections::HashMap<u32,u32>,planes:u32){
  let Some(target)=&self.target else{return};encoder.clear_buffer(&self.records,0,None);
  for pipeline in [&self.pipes.count,&self.pipes.nearest]{let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section count and nearest owner"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&target.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(pipeline);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clipping,&[]);pass.set_bind_group(3,self.record_group.as_ref().unwrap(),&[]);for plane in 0..planes{for mesh in meshes.iter().filter(|m|m.closed()&&targets.get(&m.id.value()).is_some_and(|mask|mask&(1<<plane)!=0)){mesh.bind_model(&mut pass);mesh.geometry.draw_range(&mut pass,plane..plane+1);}}}
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section integer owners"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:&image.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&self.pipes.ids);pass.set_bind_group(0,&self.frame_group,&[]);pass.set_bind_group(1,clipping,&[]);pass.set_bind_group(2,self.owner_group.as_ref().unwrap(),&[]);pass.draw(0..3,0..planes);let _=window;
 }
 pub fn bytes(&self)->u64{self.geometry.bytes()+self.records.size()+self.owners.size()+self.frame.size()+self.target.as_ref().map_or(0,|t|t.bytes)}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{cap_image::Image,pick_window::Window};
pub struct Pick{geometry:crate::cap_geometry::Geometry,crop:Option<wgpu::Buffer>,pipes:crate::cap_pick_pipeline::Pipelines,records:wgpu::Buffer,owners:wgpu::Buffer,frame:wgpu::Buffer,frame_group:wgpu::BindGroup,record_group:Option<wgpu::BindGroup>,owner_group:Option<wgpu::BindGroup>,target:Option<Image>,pub preparations:u64}
impl Pick{
 pub fn new(device:&wgpu::Device)->Self{let pipes=crate::cap_pick_pipeline::Pipelines::new(device);let buffer=|label,size,usage|device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size,usage:usage|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let records=buffer("bounded original cap crossings",16,wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_SRC);let owners=buffer("exact original cap owners",8,wgpu::BufferUsages::STORAGE);let frame=buffer("cap physical crop frame",32,wgpu::BufferUsages::UNIFORM);let frame_group=crate::gpu_layout::group(device,&pipes.frame,&frame);Self{geometry:Default::default(),crop:None,pipes,records,owners,frame,frame_group,record_group:None,owner_group:None,target:None,preparations:0}}
 fn capacity(device:&wgpu::Device,needed:u64,buffer:&mut wgpu::Buffer,label:&str)->Result<(),&'static str>{if needed>u64::from(device.limits().max_storage_buffer_binding_size){return Err("Cap picking exceeds the device storage binding limit")}if buffer.size()<needed{*buffer=device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size:needed,usage:buffer.usage(),mapped_at_creation:false});}Ok(())}
 pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,window:Window,crop:&wgpu::Buffer,rows:&[[u32;2]],planes:u32,mode:crate::pick_sources::Mode)->Result<(),&'static str>{
  self.geometry.invalidate();self.geometry.set_planes(device,planes)?;self.crop=Some(crop.clone());
  let size=[window.draw[2],window.draw[3]];let bytes=u64::from(size[0]).checked_mul(u64::from(size[1])).and_then(|n|n.checked_mul(u64::from(planes))).and_then(|n|n.checked_mul(16)).ok_or("Cap pick storage overflow")?;Self::capacity(device,bytes.max(16),&mut self.records,"bounded cap crossings")?;let owners=rows.iter().flatten().copied().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();Self::capacity(device,(owners.len()as u64).max(8),&mut self.owners,"original cap owner map")?;if !owners.is_empty(){queue.write_buffer(&self.owners,0,&owners);}
  let words=[window.full[0],window.full[1],size[0],size[1],window.draw[0],window.draw[1],mode as u32,0];queue.write_buffer(&self.frame,0,&words.into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());
  self.record_group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("cap record frame"),layout:&self.pipes.records,entries:&[wgpu::BindGroupEntry{binding:0,resource:self.records.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:crop.as_entire_binding()},wgpu::BindGroupEntry{binding:2,resource:self.frame.as_entire_binding()}]}));
  self.owner_group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("original cap owners"),layout:&self.pipes.owners,entries:&[wgpu::BindGroupEntry{binding:0,resource:self.records.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:self.owners.as_entire_binding()}]}));if self.target.as_ref().is_none_or(|t|t.size!=size){self.target=Some(Image::new(device,size,wgpu::TextureFormat::R8Unorm,1));}self.preparations+=1;Ok(())
 }
 pub fn encode(&mut self,device:&wgpu::Device,encoder:&mut wgpu::CommandEncoder,image:&crate::pick_image::Image,window:Window,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,meshes:&[crate::gpu_mesh::GpuMesh],targets:&std::collections::HashMap<u32,u32>,planes:u32,records:Option<&crate::instance_gpu::Lane>){
  let Some(target)=&self.target else{return};encoder.clear_buffer(&self.records,0,None);
  let extra=[wgpu::BindGroupEntry{binding:0,resource:self.records.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:self.crop.as_ref().unwrap().as_entire_binding()},wgpu::BindGroupEntry{binding:2,resource:self.frame.as_entire_binding()}];let rows=(0..planes).map(|plane|meshes.iter().filter(|m|m.closed()&&targets.get(&m.id.value()).is_some_and(|mask|mask&(1<<plane)!=0)).map(|mesh|{let group=records.and_then(|records|records.current_row(mesh)).map(|_|self.geometry.group_with(device,&self.pipes.record_geometry,&mesh.geometry,plane,3,&extra).expect("Prepared cap pick plane"));(mesh,group)}).collect::<Vec<_>>()).collect::<Vec<_>>();
  for(pipeline,shared)in [(&self.pipes.count,&self.pipes.record_count),(&self.pipes.nearest,&self.pipes.record_nearest)]{let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section count and nearest owner"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&target.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(pipeline);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clipping,&[]);pass.set_bind_group(3,self.record_group.as_ref().unwrap(),&[]);for(plane,rows)in rows.iter().enumerate(){for(mesh,group)in rows{if let Some(group)=group.as_ref().filter(|_|records.is_some_and(|records|records.bind_record(&mut pass,mesh))){pass.set_pipeline(shared);pass.set_bind_group(3,group,&[]);pass.draw(0..mesh.geometry.index_count,0..1);}else{pass.set_pipeline(pipeline);pass.set_bind_group(3,self.record_group.as_ref().unwrap(),&[]);mesh.bind_model(&mut pass);mesh.geometry.draw_range(&mut pass,plane as u32..plane as u32+1);}}}}
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section integer owners"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:&image.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&self.pipes.ids);pass.set_bind_group(0,&self.frame_group,&[]);pass.set_bind_group(1,clipping,&[]);pass.set_bind_group(2,self.owner_group.as_ref().unwrap(),&[]);pass.draw(0..3,0..planes);let _=window;
 }
 pub fn bytes(&self)->u64{self.geometry.bytes()+self.records.size()+self.owners.size()+self.frame.size()+self.target.as_ref().map_or(0,|t|t.bytes)}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzh-pick-caps-through-shared-records-01.rs"
```

```text
--8<-- "journey/code/96zzzzh-pick-caps-through-shared-records-02.rs"
```

```text
--8<-- "journey/code/96zzzzh-pick-caps-through-shared-records-03.rs"
```

```text
--8<-- "journey/code/96zzzzh-pick-caps-through-shared-records-04.rs"
```

</details>

### 2. `src/caps_gpu.rs`

Thread the same optional object owner through cap picking. Keep preparation and encoding under the existing bounded cap pick owner.

<details>
<summary>Locate the existing block</summary>

```rust
use wgpu::util::DeviceExt;
use crate::{gpu_mesh::GpuMesh,solid_orientation::CLOSED};
use crate::cap_image::Image;
struct Counts{image:Image,group:wgpu::BindGroup,geometry:crate::cap_geometry::Geometry}
pub struct Caps{pub enabled:bool,planes:u32,pick:Option<crate::cap_pick_gpu::Pick>,pub pick_error:Option<&'static str>,bounds:crate::cap_source_bounds::Bounds,targets:std::collections::HashMap<u32,u32>,pub solids:crate::solid_cache::Solids,pipes:Option<crate::cap_pipeline::Pipelines>,counts:Option<Counts>,frame:wgpu::Buffer,frame_group:Option<wgpu::BindGroup>,pub rows:Vec<[u32;2]>,pub draws:u64,pub passes:u64}
impl Caps{
 pub fn new(device:&wgpu::Device)->Self{let frame=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("section canvas frame"),contents:&[0;16],usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST});Self{enabled:true,planes:0,pick:None,pick_error:None,bounds:Default::default(),targets:Default::default(),solids:Default::default(),pipes:None,counts:None,frame,frame_group:None,rows:Vec::new(),draws:0,passes:0}}
 pub fn set_scene(&mut self,queue:&wgpu::Queue,scene:&crate::scene::Scene,meshes:&mut[GpuMesh],planes:&[crate::clipping_rows::Row]){
  if let Some(counts)=self.counts.as_mut(){counts.geometry.invalidate();}
  self.planes=planes.len()as u32;self.rows.clear();self.targets.clear();for(mesh,object)in meshes.iter_mut().zip(scene.objects().iter().filter(|r|scene.visible(r.id))){assert_eq!(mesh.id,object.id);let flags=self.solids.flags(object);let token=if !planes.is_empty()&&flags&CLOSED!=0{self.rows.push([mesh.id.value(),u32::from(mesh.pickable())]);self.rows.len()as u32}else{0};mesh.set_caps(queue,flags,token);if token>0{if let Some(bounds)=self.bounds.placed(&object.mesh,&object.model){let mask=planes.iter().enumerate().fold(0,|mask,(i,row)|mask|if row.plane.crosses(&bounds,[0.;3]){1<<i}else{0});self.targets.insert(mesh.id.value(),mask);}}}
  self.solids.prune();self.bounds.prune();if self.rows.is_empty(){self.rows.shrink_to_fit();self.targets.shrink_to_fit();}
 }
 pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,size:[u32;2],active:bool){
  let size=if active&&!self.rows.is_empty(){size}else{[1,1]};
  if size==[1,1]{self.counts=None;return;}
  if self.pipes.is_none(){let pipes=crate::cap_pipeline::Pipelines::new(device,format);self.frame_group=Some(crate::gpu_layout::group(device,&pipes.frame,&self.frame));self.pipes=Some(pipes);}
  let pipes=self.pipes.as_ref().unwrap();if self.counts.as_ref().is_none_or(|c|c.image.size!=size){let image=Image::new(device,size,wgpu::TextureFormat::R16Float,2);let group=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("original section crossings"),layout:&pipes.counts,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(&image.view)}]});self.counts=Some(Counts{image,group,geometry:Default::default()});}
  self.counts.as_mut().unwrap().geometry.set_planes(device,self.planes).expect("Validated clipping plane count");
  let bytes=[size[0]as f32,size[1]as f32,0.,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();queue.write_buffer(&self.frame,0,&bytes);
 }
 pub fn draw(&mut self,device:&wgpu::Device,encoder:&mut wgpu::CommandEncoder,color:&wgpu::TextureView,primitive:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,meshes:&[GpuMesh],planes:u32,records:Option<&crate::instance_gpu::Lane>){
  let(Some(pipes),Some(counts))=(&self.pipes,&mut self.counts)else{return};
  for plane in 0..planes{
   let rows=meshes.iter().filter(|m|m.closed()&&self.targets.get(&m.id.value()).is_some_and(|mask|mask&(1<<plane)!=0)).map(|mesh|{let group=records.and_then(|records|records.current_row(mesh)).map(|_|counts.geometry.group(device,&pipes.geometry,&mesh.geometry,plane).expect("Prepared cap plane"));(mesh,group)}).collect::<Vec<_>>();
   {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original closed-solid crossings"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&counts.image.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&pipes.count);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clipping,&[]);for(mesh,group)in &rows{if let Some(group)=group.as_ref().filter(|_|records.is_some_and(|records|records.bind_record(&mut pass,mesh))){pass.set_pipeline(&pipes.record_count);pass.set_bind_group(3,group,&[]);pass.draw(0..mesh.geometry.index_count,0..1);}else{pass.set_pipeline(&pipes.count);mesh.bind_model(&mut pass);mesh.geometry.draw_range(&mut pass,plane..plane+1);}self.draws+=1;}}
   {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section color and depth"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:color,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:primitive,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&pipes.color);pass.set_bind_group(0,self.frame_group.as_ref().unwrap(),&[]);pass.set_bind_group(1,clipping,&[]);pass.set_bind_group(2,&counts.group,&[]);pass.draw(0..3,plane..plane+1);self.draws+=1;}
   self.passes+=2;
  }
 }
 pub fn encode_pick(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,encoder:&mut wgpu::CommandEncoder,image:&crate::pick_image::Image,window:crate::pick_window::Window,view:&wgpu::BindGroup,crop:&wgpu::Buffer,clipping:&wgpu::BindGroup,meshes:&[GpuMesh],mode:crate::pick_sources::Mode){
  if !self.enabled||self.planes==0||self.rows.is_empty(){self.pick=None;self.pick_error=None;return;}
  let pick=self.pick.get_or_insert_with(||crate::cap_pick_gpu::Pick::new(device));if let Err(error)=pick.prepare(device,queue,window,crop,&self.rows,self.planes,mode){self.pick_error=Some(error);self.pick=None;return;}self.pick_error=None;pick.encode(encoder,image,window,view,clipping,meshes,&self.targets,self.planes);
 }
 pub fn pick_bytes(&self)->u64{self.pick.as_ref().map_or(0,crate::cap_pick_gpu::Pick::bytes)}
 pub fn usage(&self)->[u64;5]{[self.frame.size()+self.pick_bytes()+self.counts.as_ref().map_or(0,|c|c.image.bytes+c.geometry.bytes()),self.rows.len()as u64,self.solids.checks,self.draws,self.passes]}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use wgpu::util::DeviceExt;
use crate::{gpu_mesh::GpuMesh,solid_orientation::CLOSED};
use crate::cap_image::Image;
struct Counts{image:Image,group:wgpu::BindGroup,geometry:crate::cap_geometry::Geometry}
pub struct Caps{pub enabled:bool,planes:u32,pick:Option<crate::cap_pick_gpu::Pick>,pub pick_error:Option<&'static str>,bounds:crate::cap_source_bounds::Bounds,targets:std::collections::HashMap<u32,u32>,pub solids:crate::solid_cache::Solids,pipes:Option<crate::cap_pipeline::Pipelines>,counts:Option<Counts>,frame:wgpu::Buffer,frame_group:Option<wgpu::BindGroup>,pub rows:Vec<[u32;2]>,pub draws:u64,pub passes:u64}
impl Caps{
 pub fn new(device:&wgpu::Device)->Self{let frame=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("section canvas frame"),contents:&[0;16],usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST});Self{enabled:true,planes:0,pick:None,pick_error:None,bounds:Default::default(),targets:Default::default(),solids:Default::default(),pipes:None,counts:None,frame,frame_group:None,rows:Vec::new(),draws:0,passes:0}}
 pub fn set_scene(&mut self,queue:&wgpu::Queue,scene:&crate::scene::Scene,meshes:&mut[GpuMesh],planes:&[crate::clipping_rows::Row]){
  if let Some(counts)=self.counts.as_mut(){counts.geometry.invalidate();}
  self.planes=planes.len()as u32;self.rows.clear();self.targets.clear();for(mesh,object)in meshes.iter_mut().zip(scene.objects().iter().filter(|r|scene.visible(r.id))){assert_eq!(mesh.id,object.id);let flags=self.solids.flags(object);let token=if !planes.is_empty()&&flags&CLOSED!=0{self.rows.push([mesh.id.value(),u32::from(mesh.pickable())]);self.rows.len()as u32}else{0};mesh.set_caps(queue,flags,token);if token>0{if let Some(bounds)=self.bounds.placed(&object.mesh,&object.model){let mask=planes.iter().enumerate().fold(0,|mask,(i,row)|mask|if row.plane.crosses(&bounds,[0.;3]){1<<i}else{0});self.targets.insert(mesh.id.value(),mask);}}}
  self.solids.prune();self.bounds.prune();if self.rows.is_empty(){self.rows.shrink_to_fit();self.targets.shrink_to_fit();}
 }
 pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,size:[u32;2],active:bool){
  let size=if active&&!self.rows.is_empty(){size}else{[1,1]};
  if size==[1,1]{self.counts=None;return;}
  if self.pipes.is_none(){let pipes=crate::cap_pipeline::Pipelines::new(device,format);self.frame_group=Some(crate::gpu_layout::group(device,&pipes.frame,&self.frame));self.pipes=Some(pipes);}
  let pipes=self.pipes.as_ref().unwrap();if self.counts.as_ref().is_none_or(|c|c.image.size!=size){let image=Image::new(device,size,wgpu::TextureFormat::R16Float,2);let group=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("original section crossings"),layout:&pipes.counts,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(&image.view)}]});self.counts=Some(Counts{image,group,geometry:Default::default()});}
  self.counts.as_mut().unwrap().geometry.set_planes(device,self.planes).expect("Validated clipping plane count");
  let bytes=[size[0]as f32,size[1]as f32,0.,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();queue.write_buffer(&self.frame,0,&bytes);
 }
 pub fn draw(&mut self,device:&wgpu::Device,encoder:&mut wgpu::CommandEncoder,color:&wgpu::TextureView,primitive:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,meshes:&[GpuMesh],planes:u32,records:Option<&crate::instance_gpu::Lane>){
  let(Some(pipes),Some(counts))=(&self.pipes,&mut self.counts)else{return};
  for plane in 0..planes{
   let rows=meshes.iter().filter(|m|m.closed()&&self.targets.get(&m.id.value()).is_some_and(|mask|mask&(1<<plane)!=0)).map(|mesh|{let group=records.and_then(|records|records.current_row(mesh)).map(|_|counts.geometry.group(device,&pipes.geometry,&mesh.geometry,plane).expect("Prepared cap plane"));(mesh,group)}).collect::<Vec<_>>();
   {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original closed-solid crossings"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&counts.image.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&pipes.count);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clipping,&[]);for(mesh,group)in &rows{if let Some(group)=group.as_ref().filter(|_|records.is_some_and(|records|records.bind_record(&mut pass,mesh))){pass.set_pipeline(&pipes.record_count);pass.set_bind_group(3,group,&[]);pass.draw(0..mesh.geometry.index_count,0..1);}else{pass.set_pipeline(&pipes.count);mesh.bind_model(&mut pass);mesh.geometry.draw_range(&mut pass,plane..plane+1);}self.draws+=1;}}
   {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section color and depth"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:color,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:primitive,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&pipes.color);pass.set_bind_group(0,self.frame_group.as_ref().unwrap(),&[]);pass.set_bind_group(1,clipping,&[]);pass.set_bind_group(2,&counts.group,&[]);pass.draw(0..3,plane..plane+1);self.draws+=1;}
   self.passes+=2;
  }
 }
 pub fn encode_pick(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,encoder:&mut wgpu::CommandEncoder,image:&crate::pick_image::Image,window:crate::pick_window::Window,view:&wgpu::BindGroup,crop:&wgpu::Buffer,clipping:&wgpu::BindGroup,meshes:&[GpuMesh],mode:crate::pick_sources::Mode,records:Option<&crate::instance_gpu::Lane>){
  if !self.enabled||self.planes==0||self.rows.is_empty(){self.pick=None;self.pick_error=None;return;}
  let pick=self.pick.get_or_insert_with(||crate::cap_pick_gpu::Pick::new(device));if let Err(error)=pick.prepare(device,queue,window,crop,&self.rows,self.planes,mode){self.pick_error=Some(error);self.pick=None;return;}self.pick_error=None;pick.encode(device,encoder,image,window,view,clipping,meshes,&self.targets,self.planes,records);
 }
 pub fn pick_bytes(&self)->u64{self.pick.as_ref().map_or(0,crate::cap_pick_gpu::Pick::bytes)}
 pub fn usage(&self)->[u64;5]{[self.frame.size()+self.pick_bytes()+self.counts.as_ref().map_or(0,|c|c.image.bytes+c.geometry.bytes()),self.rows.len()as u64,self.solids.checks,self.draws,self.passes]}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzh-pick-caps-through-shared-records-05.rs"
```

```text
--8<-- "journey/code/96zzzzh-pick-caps-through-shared-records-06.rs"
```

</details>

### 3. `src/pick_encode.rs`

Share object records with first picks, source faces and cap picks in the existing submission.

<details>
<summary>Locate the existing block</summary>

```rust
        if let Some(caps)=caps{caps.encode_pick(device,&ctx.queue,&mut encoder,image,window,view_group,crop,clipping,meshes,mode);}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        if let Some(caps)=caps{caps.encode_pick(device,&ctx.queue,&mut encoder,image,window,view_group,crop,clipping,meshes,mode,records.map(|(rows,_)|rows));}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzh-pick-caps-through-shared-records-07.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/cap_pick_tests.rs`

Supply full-u32 shared/fallback owner comparison and restoration before a color frame, retaining all original geometry assertions.

<details>
<summary>Locate the existing block</summary>

```text
assert!(r.cap_usage()[0]>24);pixels(&r);
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzh-pick-caps-through-shared-records-08.rs"
```

</details>

</details>

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 96zzzzh-pick-caps-through-shared-records
npm --prefix ../session_tests run course -- check 96zzzzh-pick-caps-through-shared-records
npm --prefix ../session_tests run course -- save 96zzzzh-pick-caps-through-shared-records
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

Cap picking consumes shared object records and retains original full integer identities, clipping and crop behavior.

- **Final Lesson Experiment:** Pick a cap before drawing a color frame, then pick the corner shared by two clipping planes. Repeat after selection and scene replacement.

Continue with [Retained allocation, live lookup and safe reclamation](../../consolidation/96zzzzi-adapt-incremental-slots-to-pulled-draws.md).
