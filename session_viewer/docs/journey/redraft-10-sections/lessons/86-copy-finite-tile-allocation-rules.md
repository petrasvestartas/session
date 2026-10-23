# 17.1 · Bounded projected triangles and tile lists

**Section 9: Compute Tile Visibility, Clipping Half-Spaces, Section Caps, and Ambient Occlusion** · **Lesson 17.1** (Redraft `9.1`) · **27 checkpoints (`86-copy-finite-tile-allocation-rules` → `86r-cache-projection-and-finish-current-reports`)** · **Study time: 13.5–27 h**

> **What You Will Learn & Build:** Bound the physical tile grid and reference storage. By the final checkpoint (`86r-cache-projection-and-finish-current-reports`), you will cache projection and finish current reports.

## 1. Visual Pipeline & WebGPU Architecture (Black & White Schematics)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 220" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="212" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Lesson 17.1 Execution Flow: Bounded projected triangles and tile lists</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">Concrete data pipeline you build in this lesson (27 Checkpoints: 86-copy-finite-tile-allocation-rules → 86r-cache-projection-and-finish-current-reports)</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="89" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="92" x2="151" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="89" y="108" text-anchor="middle" font-size="11" fill="#111111">physical canvas</text>
    <line x1="161" y1="108" x2="181" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="183" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="254" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="193" y1="92" x2="316" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="254" y="108" text-anchor="middle" font-size="11" fill="#111111">bounded tile layout</text>
    <line x1="326" y1="108" x2="346" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="348" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="358" y1="92" x2="481" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">shared word pool</text>
    <line x1="491" y1="108" x2="511" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="513" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="584" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="523" y1="92" x2="646" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="584" y="108" text-anchor="middle" font-size="11" fill="#111111">current originals</text>
    <text x="584" y="124" text-anchor="middle" font-size="11" fill="#111111">and view</text>
    <line x1="656" y1="108" x2="676" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="678" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="749" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 5</text>
    <line x1="688" y1="92" x2="811" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="749" y="108" text-anchor="middle" font-size="11" fill="#111111">bounded buffers</text>
    
    <rect x="18" y="162" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="181" font-size="11" font-weight="700" fill="#111111">Files You Edit in Lesson 17.1:</text>
    <text x="215" y="181" font-size="11" fill="#111111">src/tile_math.rs  ·  src/lib.rs  ·  src/projected_triangle.wgsl  ·  src/finite_project.wgsl  ·  src/finite_project_source.rs  ·  src/renderer.rs</text>
    <text x="32" y="198" font-size="10.5" fill="#111111">27 Checkpoints: 86-copy-finite-tile-allocation-rules → 86r-cache-projection-and-finish-current-reports — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
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
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Section 9 Compute Tile Visibility, Clipping Half-Spaces, Winding Section Caps & Horizon GTAO (Ch 17–18)</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">How @compute shaders bin triangles into screen tiles for hidden-line removal, clip solids with winding caps, and shade crevices.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    <rect x="18" y="68" width="250" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="143" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">1. @compute Tile Visibility (Ch 17)</text>
    <text x="28" y="108" font-size="10.5" fill="#111111">• projected_triangle.wgsl projects tris</text>
    <text x="28" y="126" font-size="10.5" font-weight="600" fill="#111111">• atomicAdd tile counts + prefix-sum CSR</text>
    <text x="28" y="144" font-size="10.5" fill="#111111">• ink_visibility.wgsl tests center-line</text>
    <text x="28" y="160" font-size="10.5" fill="#111111">  against covering triangles in tile</text>
    <text x="28" y="178" font-size="10.5" fill="#111111">• Hidden lines dashed/faded cleanly!</text>

    <line x1="268" y1="138" x2="298" y2="138" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="300" y="68" width="265" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="432" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">2. Clipping &amp; Winding Caps (Ch 17)</text>
    <text x="312" y="108" font-size="10.5" fill="#111111">• Half-space: dot(pos_world, plane) &lt; 0</text>
    <text x="312" y="126" font-size="10.5" font-weight="600" fill="#111111">• Signed front/back crossing count</text>
    <text x="312" y="142" font-size="10" fill="#111111">  at clip plane depth (cap.wgsl):</text>
    <text x="312" y="158" font-size="10" fill="#111111">  Fills closed solid interiors while</text>
    <text x="312" y="173" font-size="10" fill="#111111">  leaving internal bores hollow!</text>
    <text x="312" y="191" font-size="10.5" fill="#111111">• Cap picks resolve to solid owner_id</text>

    <line x1="565" y1="138" x2="595" y2="138" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="597" y="68" width="223" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="708" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">3. Horizon GTAO &amp; X-Ray (Ch 18)</text>
    <text x="609" y="108" font-size="10.5" fill="#111111">• ambient_depth.wgsl -&gt; 6-mip pyramid</text>
    <text x="609" y="126" font-size="10.5" font-weight="600" fill="#111111">• ssao.wgsl + bilateral blur +</text>
    <text x="609" y="142" font-size="10.5" font-weight="600" fill="#111111">  ambient_composite.wgsl (Arctic)</text>
    <text x="609" y="160" font-size="10" fill="#111111">• Glass / X-Ray translucent shells</text>
    <text x="609" y="175" font-size="10" fill="#111111">  allow picking interior Wood joints</text>
    
</svg>
</div>

## 2. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

Before writing code, review how the three layers in this chapter talk to each other:

| Layer | Construct | Plain-English Explanation for Students |
| :--- | :--- | :--- |
| **WGSL (`@compute`)** | `@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3<u32>)` | Runs general-purpose parallel GPU threads (outside vertex/fragment rasterization) to project triangles and prefix-sum tile counts. |
| **WGSL (Atomics)** | `atomicAdd(&tile_counts[tile_id], 1u)` | Safely increments shared GPU buffer counters when multiple triangles overlap the same screen tile concurrently. |
| **Math (Winding Cap)** | `signed_crossings != 0` at clip plane depth | Fills only the solid interior of a clipped B-Rep/mesh while leaving interior bores and open shells hollow. |
| **WGSL (`ssao.wgsl`)** | Multi-level depth pyramid horizon integration | Samples 6 mip levels of linear depth and per-object contact radius to shade crevices and ground contact. |

**Authoritative production source (`session_viewer/src`):** [engine/gpu/triangle_tiles.rs](../../../../src/engine/gpu/triangle_tiles.rs), [shaders/project_triangles.wgsl](../../../../src/shaders/project_triangles.wgsl), [shaders/ink_visibility.wgsl](../../../../src/shaders/ink_visibility.wgsl), [shaders/ribbon.wgsl](../../../../src/shaders/ribbon.wgsl), [shaders/vector.wgsl](../../../../src/shaders/vector.wgsl), [engine/gpu/clip.rs](../../../../src/engine/gpu/clip.rs), [shaders/cap.wgsl](../../../../src/shaders/cap.wgsl), [engine/gpu/render.rs](../../../../src/engine/gpu/render.rs), [engine/gpu/pass.rs](../../../../src/engine/gpu/pass.rs), [build.rs](../../../../build.rs).

## 3. Checkpoint Roadmap (Jump to Any Notebook Step)

| Step | Checkpoint | What You Build | Files Modified | Pipeline Trace |
| ---: | :--- | :--- | :--- | :--- |
| **1** | [`86-copy-finite-tile-allocation-rules`](#checkpoint-86-copy-finite-tile-allocation-rules) | Bound the physical tile grid and reference storage. | `src/tile_math.rs`, `src/lib.rs` | `physical canvas → bounded tile layout → shared word pool` |
| **2** | [`86ma-grow-projection-and-reference-pools-from-demand`](#checkpoint-86ma-grow-projection-and-reference-pools-from-demand) | Grow projection and reference pools from demand. | `src/tile_math.rs` | `physical canvas → bounded tile layout → shared word pool` |
| **3** | [`86a-copy-maintained-projection-and-scan-recipes`](#checkpoint-86a-copy-maintained-projection-and-scan-recipes) | Store stable screen-space triangle records and reuse one coverage and tile-span owner. | `src/projected_triangle.wgsl` | `finite polygons → tile counts → prefix offsets → references` |
| **4** | [`86b-copy-the-project-recipe`](#checkpoint-86b-copy-the-project-recipe) | Bind existing original mesh buffers and transform each indexed corner into the projection convention. | `src/finite_project.wgsl` | `declare finite projection buffers and original vertex access` |
| **5** | [`86ba-clip-original-triangles-before-dividing-by-depth`](#checkpoint-86ba-clip-original-triangles-before-dividing-by-depth) | Clip against the homogeneous near plane, retain up to four corners and measure signed screen area. | `src/finite_project.wgsl` | `clip original triangles before dividing by depth` |
| **6** | [`86bb-write-bounded-projected-triangle-records-on-the-gpu`](#checkpoint-86bb-write-bounded-projected-triangle-records-on-the-gpu) | Dispatch one invocation per original triangle and write screen bounds, inward edge equations and a conditioned depth plane. | `src/finite_project.wgsl` | `write bounded projected triangle records on the gpu` |
| **7** | [`86c-project-existing-original-mesh-buffers`](#checkpoint-86c-project-existing-original-mesh-buffers) | Reuse shared WGSL declarations and validate the completed compute entry point. | `src/finite_project_source.rs`, `src/lib.rs`, `src/renderer.rs` | `original buffers → finite projected records → tile references` |
| **8** | [`86d-copy-the-triangle-tiles-recipe`](#checkpoint-86d-copy-the-triangle-tiles-recipe) | Bind projected triangle records and a shared bounded reference pool with atomic count and cursor words. | `src/finite_triangle_tiles.wgsl`, `src/finite_project_source.rs` | `declare atomic tile records and flat triangle inputs` |
| **9** | [`86da-count-conservative-triangle-coverage-per-tile`](#checkpoint-86da-count-conservative-triangle-coverage-per-tile) | Rasterize one tile-grid quad per triangle and reject tiles outside any inward edge before counting. | `src/finite_triangle_tiles.wgsl` | `count conservative triangle coverage per tile` |
| **10** | [`86db-fill-bounded-original-triangle-reference-lists`](#checkpoint-86db-fill-bounded-original-triangle-reference-lists) | Atomically reserve a tile slot, store the original projected triangle address and depth, and flag overflow instead of writing past the pool. | `src/finite_triangle_tiles.wgsl` | `fill bounded original triangle reference lists` |
| **11** | [`86e-count-and-fill-original-triangle-references`](#checkpoint-86e-count-and-fill-original-triangle-references) | Reuse shared WGSL source and validate the complete tiles pass interfaces. | `src/renderer.rs`, `src/finite_project_source.rs`, `src/finite_tiles_contract.rs`, `src/lib.rs` | `original buffers → finite projected records → tile references` |
| **12** | [`86f-copy-the-scan-triangle-tiles-recipe`](#checkpoint-86f-copy-the-scan-triangle-tiles-recipe) | Build a saturating exclusive prefix sum with uniform barriers and shared scratch storage. | `src/finite_scan_triangle_tiles.wgsl` | `scan bounded tile counts in workgroup memory` |
| **13** | [`86fa-scan-tile-blocks-and-their-block-totals`](#checkpoint-86fa-scan-tile-blocks-and-their-block-totals) | Run the same prefix owner first over tile counts and then over their block totals. | `src/finite_scan_triangle_tiles.wgsl` | `scan tile blocks and their block totals` |
| **14** | [`86fb-finalize-bounded-tile-list-offsets-and-demand`](#checkpoint-86fb-finalize-bounded-tile-list-offsets-and-demand) | Combine block and local offsets, reset fill cursors, and record actual pool demand in the shared header. | `src/finite_scan_triangle_tiles.wgsl` | `finalize bounded tile list offsets and demand` |
| **15** | [`86fc-assemble-and-validate-the-bounded-scan-shader`](#checkpoint-86fc-assemble-and-validate-the-bounded-scan-shader) | Reuse shared WGSL source and validate the complete scan pass interfaces. | `src/renderer.rs`, `src/finite_project_source.rs`, `src/finite_scan_contract.rs`, `src/lib.rs` | `original buffers → finite projected records → tile references` |
| **16** | [`86g-wake-only-after-a-tile-report-arrives`](#checkpoint-86g-wake-only-after-a-tile-report-arrives) | Wake only after a tile report arrives. | `src/browser_tiles.rs`, `src/lib.rs` | `completed map → browser event → one redraw` |
| **17** | [`86h-read-actual-pool-demand-asynchronously`](#checkpoint-86h-read-actual-pool-demand-asynchronously) | Read GPU tile-pool demand asynchronously, with one map in flight and one reusable readback buffer. | `src/tile_report.rs`, `src/lib.rs` | `scan header → sixteen-byte copy → map result` |
| **18** | [`86i-describe-explicit-finite-gpu-layouts`](#checkpoint-86i-describe-explicit-finite-gpu-layouts) | Compile the finite pipeline resource types before their helper functions. | `src/tile_pipelines.rs`, `src/lib.rs` | `uniform and storage visibility → explicit binding layouts` |
| **19** | [`86j-build-compute-and-count-pipeline-helpers`](#checkpoint-86j-build-compute-and-count-pipeline-helpers) | Build compute and count pipeline helpers. | `src/tile_pipelines.rs` | `validated shader → explicit pipeline layout → GPU pipeline` |
| **20** | [`86k-connect-projection-count-scan-and-fill-pipelines`](#checkpoint-86k-connect-projection-count-scan-and-fill-pipelines) | Connect projection count scan and fill pipelines. | `src/tile_pipelines.rs` | `project → count → scan → fill` |
| **21** | [`86l-own-bounded-finite-tile-resources`](#checkpoint-86l-own-bounded-finite-tile-resources) | Compile bounded tile resource types before allocation and population. | `src/tile_gpu.rs`, `src/gpu_mesh.rs`, `src/lib.rs` | `original buffers → derived GPU owner → explicit cleanup` |
| **22** | [`86m-allocate-empty-tile-placeholders`](#checkpoint-86m-allocate-empty-tile-placeholders) | Allocate empty tile placeholders. | `src/tile_gpu.rs` | `current originals and view → bounded buffers → cached finite lists` |
| **23** | [`86n-witness-original-buffers-and-world-placement`](#checkpoint-86n-witness-original-buffers-and-world-placement) | Witness original buffers and world placement. | `src/tile_gpu.rs` | `current originals and view → bounded buffers → cached finite lists` |
| **24** | [`86o-release-obsolete-tile-lists-and-bindings`](#checkpoint-86o-release-obsolete-tile-lists-and-bindings) | Release obsolete tile lists and bindings. | `src/tile_gpu.rs` | `current originals and view → bounded buffers → cached finite lists` |
| **25** | [`86p-grow-lists-only-from-actual-demand`](#checkpoint-86p-grow-lists-only-from-actual-demand) | Grow lists only from actual demand. | `src/tile_gpu.rs` | `current originals and view → bounded buffers → cached finite lists` |
| **26** | [`86q-rasterize-bounded-tile-counts-and-references`](#checkpoint-86q-rasterize-bounded-tile-counts-and-references) | Rasterize bounded tile counts and references. | `src/tile_gpu.rs` | `current originals and view → bounded buffers → cached finite lists` |
| **27** | [`86r-cache-projection-and-finish-current-reports`](#checkpoint-86r-cache-projection-and-finish-current-reports) | Cache projection and finish current reports. | `src/tile_gpu.rs` | `current originals and view → bounded buffers → cached finite lists` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Read the **Concept Walkthrough** and **Self-Check Q&A** first, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-86-copy-finite-tile-allocation-rules"></a>

### Step 1 of 27 · Bound the physical tile grid and reference storage (`86-copy-finite-tile-allocation-rules`)

- **Goal:** Bound the physical tile grid and reference storage.
- **Data Flow Trace:** `physical canvas → bounded tile layout → shared word pool`
- **Files You Touch:** `src/tile_math.rs` (*Bound the physical tile grid and reference storage.*) · `src/lib.rs` (*Register the completed source owner.*)

Tile coverage uses physical framebuffer pixels, not CSS pixels.

Begin with four-pixel tiles and double their span until the tile count fits the fixed budget. Reserve sixteen bytes per tile record, plus hierarchical scan totals; triangle references then share one word pool rather than a fixed list for every tile. A reference stores two words: a triangle index and depth. Compute the initial pool from the viewport and triangle count, then cap it by the maximum total reference budget. These CPU sizes must agree with the WGSL offsets in the following scan and fill lessons.

> **Self-Check Question:** *Why grow tile span instead of allocating an unrestricted grid?*  
> **Answer:** Doubling the span reduces the number of tiles by roughly four. This bounds both the header and maximum reference pool when the physical canvas grows.

- **What You Should See:** The preceding drawing remains visible. This CPU allocation owner compiles; GPU projection and reference-list dispatch connect in the following finite-ink lessons.
- **Hands-On Lab Experiment:** Calculate the grid at 1800 × 1400 and 2800 × 1800 pixels. Explain why the second canvas needs eight-pixel tiles and why doubling device pixel ratio may change the layout.

<details>
<summary>Implementation 86-copy-finite-tile-allocation-rules: Bound the physical tile grid and reference storage</summary>

Bound the physical tile grid and reference storage.

Data flow: physical canvas → bounded tile layout → shared word pool.

### 1. `src/tile_math.rs`

Bound the physical tile grid and reference storage.

Create the file and type:

```rust
--8<-- "journey/code/86-copy-finite-tile-allocation-rules-typed-1.rs"
```

### 2. `src/lib.rs`

Register the completed source owner.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod lasso_geometry_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/86-copy-finite-tile-allocation-rules-02.rs" skip=39
```

</details>

<a id="checkpoint-86ma-grow-projection-and-reference-pools-from-demand"></a>

### Step 2 of 27 · Grow projection and reference pools from demand (`86ma-grow-projection-and-reference-pools-from-demand`)

- **Goal:** Grow projection and reference pools from demand.
- **Data Flow Trace:** `physical canvas → bounded tile layout → shared word pool`
- **Files You Touch:** `src/tile_math.rs` (*Grow projection and reference pools from demand.*)

Map triangle projection onto two-dimensional dispatch rows of 64 invocations per workgroup, keeping each dimension under the device dispatch limit.

Reuse the projected table while it is between one quarter full and capacity; allocate an exact table for a large load jump, or an eighth of spare capacity for an edit. Grow the shared reference pool only from its initial floor or an asynchronous report that reached capacity. Saturating doubling and the explicit ceiling keep that growth bounded. The later GPU owner will enforce device buffer limits and consume this calculation; no synchronous GPU readback is introduced here.

> **Self-Check Question:** *Why treat a report exactly equal to capacity as saturated?*  
> **Answer:** A clamped report cannot distinguish exactly full from overflow. Conservatively growing prevents a full pool from remaining permanently too small.

- **Hands-On Lab Experiment:** Starting at 1000 words, trace four saturated reports with a 10000-word ceiling. Then compare no report with a report below capacity.

<details>
<summary>Implementation 86ma-grow-projection-and-reference-pools-from-demand: Grow projection and reference pools from demand</summary>

Grow projection and reference pools from demand.

Data flow: physical canvas → bounded tile layout → shared word pool.

### 1. `src/tile_math.rs`

Grow projection and reference pools from demand.

<details>
<summary>Locate the existing block</summary>

```rust
        self.header_records() * 16 + pool_words * 4
    }

    /// Largest pool, in words.
    pub fn max_pool_words(self) -> u64 {
        self.count() as u64 * REFERENCES_PER_TILE * REFERENCE_WORDS
    }

    /// First pool size for `triangles`, in words.
    pub fn initial_pool_words(self, triangles: u32) -> u64 {
        let references = self.count() as u64 * 2 + u64::from(triangles) * 8;
        (references * REFERENCE_WORDS)
            .max(MIN_POOL_WORDS)
            .min(self.max_pool_words())
    }
}
```

</details>

After that block, type:

```rust
/// Projection workgroups across and down for `triangles`, 64 each, rows of PROJECT_ROW_GROUPS.
pub fn project_groups(triangles: u32) -> (u32, u32) {
    let groups = triangles.div_ceil(64);
    (
        groups.min(PROJECT_ROW_GROUPS),
        groups.div_ceil(PROJECT_ROW_GROUPS),
    )
}

/// Next pool size: at least `floor`; doubled when the report overflowed.
/// Records for a table of `need` triangles, None while `capacity` fits: exact after a load jump, an eighth spare after an edit.
pub fn projected_records(need: u64, capacity: u64, most: u64) -> Option<u64> {
    if need <= capacity && need * 4 >= capacity {
        return None;
    }

    let spare = if need > capacity * 3 / 2 { 0 } else { need / 8 };
    Some((need + spare).min(most).max(need))
}

pub fn next_pool_words(
    current: u64,
    floor: u64,
    capacity: u64,
    report: Option<u64>,
    ceiling: u64,
) -> u64 {
    let mut want = current.max(floor);

    // a report at or past capacity means the lists did not fit
    if let Some(needed) = report
        && needed >= capacity
    {
        want = want.max(current.saturating_mul(2));
    }

    want.min(ceiling)
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86ma-grow-projection-and-reference-pools-from-demand-typed-1.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/tile_math.rs`

Grow projection and reference pools from demand.

<details>
<summary>Locate the existing block</summary>

```text
et spare = if need > capacity * 3 / 2 { 0 } else { need / 8 };
    Some((need + spare).min(most).max(need))
}

pub fn next_pool_words(
    current: u64,
    floor: u64,
    capacity: u64,
    report: Option<u64>,
    ceiling: u64,
) -> u64 {
    let mut want = current.max(floor);

    // a report at or past capacity means the lists did not fit
    if let Some(needed) = report
        && needed >= capacity
    {
        want = want.max(current.saturating_mul(2));
    }

    want.min(ceiling)
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/86ma-grow-projection-and-reference-pools-from-demand-typed-2.rs" skip=500
```

</details>

</details>

<a id="checkpoint-86a-copy-maintained-projection-and-scan-recipes"></a>

### Step 3 of 27 · Define one shared projected-triangle coverage contract (`86a-copy-maintained-projection-and-scan-recipes`)

- **Goal:** Store stable screen-space triangle records and reuse one coverage and tile-span owner.
- **Data Flow Trace:** `finite polygons → tile counts → prefix offsets → references`
- **Files You Touch:** `src/projected_triangle.wgsl` (*Copy the maintained shader recipe.*)

Write the96-byte projected-triangle record used by the finite passes. Four inward edge equations describe an original triangle or its near-clipped polygon. The stored reference and gradient evaluate depth near the original face instead of extrapolating every face from the origin.

The shared coverage helper first rejects an empty record and its screen bounds, then checks the inward edge distances with1/256pixel raster-snap slack. Return both depth and a coverage flag: depth zero is a valid value, so it cannot stand for absence. The shared tile-span helper doubles the span until the grid fits the existing bounded tile count.

Projection, tile binning and finite ink will reuse this source through explicit assembly markers. There is one implementation of these shared formulas. The original unused copies of the maintained count and scan shaders are removed from the learner files; the following86d–86f lessons develop the connected passes.

> **Self-Check Question:** *Why does the triangle coverage helper return a separate coverage flag with depth?*  
> **Answer:** Depth zero can be a valid covered surface. A separate flag distinguishes that surface from an empty record or a point outside its bounds and edges.

- **What You Should See:** One shared projected-triangle record, coverage test and bounded tile-span helper is written for the following GPU passes.
- **Hands-On Lab Experiment:** Trace a point just outside one edge and a point inside with depth zero. Predict the two returned vectors. Compare the sixvec4 fields with the existing Rust storage record before the later source assembly.

<details>
<summary>Implementation 86a-copy-maintained-projection-and-scan-recipes: Define one shared projected-triangle coverage contract</summary>

Store stable screen-space triangle records and reuse one coverage and tile-span owner.

Data flow: finite polygons → tile counts → prefix offsets → references.

### 1. `src/projected_triangle.wgsl`

Copy the maintained shader recipe.

Create the file and type:

```wgsl
--8<-- "journey/code/86a-copy-maintained-projection-and-scan-recipes-01.rs"
```

</details>

<a id="checkpoint-86b-copy-the-project-recipe"></a>

### Step 4 of 27 · Declare finite projection buffers and original vertex access (`86b-copy-the-project-recipe`)

- **Goal:** Bind existing original mesh buffers and transform each indexed corner into the projection convention.
- **Data Flow Trace:** `declare finite projection buffers and original vertex access`
- **Files You Touch:** `src/finite_project.wgsl` (*Bind existing original mesh buffers and transform each indexed corner into the projection convention.*)

Bind existing original mesh buffers and transform each indexed corner into the projection convention.

Use the existing original mesh and object layouts from this application. Reuse the projected-triangle declaration and coverage helper from86a through a shared source marker; do not type that algorithm again. The shader is assembled and validated at86c. The existing browser canvas still runs while this compute pass is being developed.

> **Self-Check Question:** *Why does a source vertex index multiply by six before reading its position?*  
> **Answer:** The existing packed mesh vertex has six scalar words: position then normal. Its index addresses a vertex record, not an individual scalar.

- **What You Should See:** This projection section is written. The current browser app runs; the new compute pass is not connected yet.
- **Hands-On Lab Experiment:** Trace vertex index2 through its six-word record and the existing model/view transforms. Explain which buffer is read and where the projected record will be written.

<details>
<summary>Implementation 86b-copy-the-project-recipe: Declare finite projection buffers and original vertex access</summary>

Bind existing original mesh buffers and transform each indexed corner into the projection convention.

Data flow: declare finite projection buffers and original vertex access.

### 1. `src/finite_project.wgsl`

Create the file and type:

```wgsl
--8<-- "journey/code/86b-copy-the-project-recipe-01.wgsl"
```

</details>

<a id="checkpoint-86ba-clip-original-triangles-before-dividing-by-depth"></a>

### Step 5 of 27 · Clip original triangles before dividing by depth (`86ba-clip-original-triangles-before-dividing-by-depth`)

- **Goal:** Clip against the homogeneous near plane, retain up to four corners and measure signed screen area.
- **Data Flow Trace:** `clip original triangles before dividing by depth`
- **Files You Touch:** `src/finite_project.wgsl` (*Clip against the homogeneous near plane, retain up to four corners and measure signed screen area.*)

Clip against the homogeneous near plane, retain up to four corners and measure signed screen area.

Work in homogeneous coordinates until every retained corner can be divided safely. The clip convention converts the existing forward-depth camera into the near-plane convention used by this bounded visibility pass. Preserve the original mesh; the output is only a projected display record.

> **Self-Check Question:** *Why must near-plane intersections be found before the perspective divide?*  
> **Answer:** A corner behind the eye cannot safely be divided into screen coordinates. Interpolate in homogeneous coordinates first, then divide the retained polygon.

- **Hands-On Lab Experiment:** Sketch a triangle with one corner behind the near plane. Predict the retained polygon and winding before running the later connected pass.

<details>
<summary>Implementation 86ba-clip-original-triangles-before-dividing-by-depth: Clip original triangles before dividing by depth</summary>

Clip against the homogeneous near plane, retain up to four corners and measure signed screen area.

Data flow: clip original triangles before dividing by depth.

### 1. `src/finite_project.wgsl`

<details>
<summary>Locate the existing block</summary>

```wgsl
 let base=physical_indices[index]*6u;
 let point=vec3<f32>(physical_vertices[base],physical_vertices[base+1u],physical_vertices[base+2u]);
 let clip=view.transform*object.model*vec4<f32>(point,1.0);
 return vec4<f32>(clip.xy,clip.w-clip.z,clip.w);
}
```

</details>

After that block, type:

```wgsl

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
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86ba-clip-original-triangles-before-dividing-by-depth-01.wgsl"
```

</details>

</details>

<a id="checkpoint-86bb-write-bounded-projected-triangle-records-on-the-gpu"></a>

### Step 6 of 27 · Write bounded projected triangle records on the GPU (`86bb-write-bounded-projected-triangle-records-on-the-gpu`)

- **Goal:** Dispatch one invocation per original triangle and write screen bounds, inward edge equations and a conditioned depth plane.
- **Data Flow Trace:** `write bounded projected triangle records on the gpu`
- **Files You Touch:** `src/finite_project.wgsl` (*Dispatch one invocation per original triangle and write screen bounds, inward edge equations and a conditioned depth plane.*)

Dispatch one invocation per original triangle and write screen bounds, inward edge equations and a conditioned depth plane.

Guard the live triangle range before writing. Two dispatch dimensions cover large triangle counts without exceeding the per-dimension workgroup limit. Store one empty record for degenerate polygons; otherwise record inward edge equations, conservative bounds and the depth reference used by finite ink.

> **Self-Check Question:** *Why use a nearby depth reference for small distant triangles?*  
> **Answer:** Extrapolating a small distant face to the viewport centre magnifies cancellation. A nearby reference keeps its plane evaluation stable; large clipped faces use the centre to avoid huge corner coordinates.

- **What You Should See:** The finite projection compute recipe is assembled for source validation; its runtime pipeline is connected by the existing following lessons.
- **Hands-On Lab Experiment:** Predict the output for a degenerate triangle and for a small distant face. After the later pipeline connection, inspect finite ink around a near-clipped face without changing original geometry.

<details>
<summary>Implementation 86bb-write-bounded-projected-triangle-records-on-the-gpu: Write bounded projected triangle records on the GPU</summary>

Dispatch one invocation per original triangle and write screen bounds, inward edge equations and a conditioned depth plane.

Data flow: write bounded projected triangle records on the gpu.

### 1. `src/finite_project.wgsl`

<details>
<summary>Locate the existing block</summary>

```wgsl
    return a.x*b.y-a.y*b.x;
}
fn physical_polygon_area(polygon: ProjectedPolygon) -> f32 {
    // zero only for a degenerate polygon
    return physical_cross(polygon.points[1].xy-polygon.points[0].xy, polygon.points[2].xy-polygon.points[0].xy);
}
```

</details>

After that block, type:

```wgsl

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

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86bb-write-bounded-projected-triangle-records-on-the-gpu-01.wgsl"
```

</details>

</details>

<a id="checkpoint-86c-project-existing-original-mesh-buffers"></a>

### Step 7 of 27 · Assemble and validate the shared finite projection shader (`86c-project-existing-original-mesh-buffers`)

- **Goal:** Reuse shared WGSL declarations and validate the completed compute entry point.
- **Data Flow Trace:** `original buffers → finite projected records → tile references`
- **Files You Touch:** `src/finite_project_source.rs` (*Assemble one shared shader interface and validate its compute entry point.*) · `src/lib.rs` (*Expose the existing-source assembly boundary for the later pipeline.*) · `src/renderer.rs` (*Compile the completed recipe on the actual browser device and report asynchronous compilation info.*)

WGSL has no source-file import in this pipeline.

Expand the explicit shared-source marker before passing shader text to wgpu. Reuse the projected-triangle declaration and coverage helpers already supplied at86a instead of maintaining another copy. The new compute functions read the original mesh layout from the outset; no second geometry adapter replaces them.

Parse and validate the assembled source using the Naga version already reexported by wgpu. This is a shader interface check; the later real pipeline and pixel tests remain necessary.

The learner checks this source on the actual browser GPU. Reuse the shader-check helper already written at64hd during renderer initialization. Open the developer console for the Finite projection report; no native test command is required. This validates the completed shader before the later pipeline connection.

> **Self-Check Question:** *Which source must be validated when a shader contains a shared-source marker?*  
> **Answer:** Validate the assembled WGSL passed to wgpu, after expanding the marker. Validating the unexpanded file would not test the actual shader interface.

- **What You Should See:** The actual browser GPU compiles the assembled projection source and reports its shader errors; the compute pipeline is connected in the following lessons.
- **Hands-On Lab Experiment:** Run the browser and read the Finite projection compilation report. Temporarily change a WGSL return type so it disagrees with its return value: inspect the real GPU compiler message and location, then restore the type and reload. Finally compare the shared record layout with the existing Rust buffer.

<details>
<summary>Implementation 86c-project-existing-original-mesh-buffers: Assemble and validate the shared finite projection shader</summary>

Reuse shared WGSL declarations and validate the completed compute entry point.

Data flow: original buffers → finite projected records → tile references.

### 1. `src/finite_project_source.rs`

Assemble one shared shader interface and validate its compute entry point.

Create the file and type:

```rust
--8<-- "journey/code/86c-project-existing-original-mesh-buffers-source-browser-info-reuse-check.rs"
```

### 2. `src/lib.rs`

Expose the existing-source assembly boundary for the later pipeline.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod tile_math;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/86c-project-existing-original-mesh-buffers-register.rs" skip=19
```

### 3. `src/renderer.rs`

Compile the completed recipe on the actual browser device and report asynchronous compilation info.

<details>
<summary>Locate the existing block</summary>

```rust
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        crate::shader_check::check(&device, crate::finite_project_source::source(), "Finite projection");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86c-project-existing-original-mesh-buffers-04-source-reuse-check.rs"
```

</details>

</details>

<a id="checkpoint-86d-copy-the-triangle-tiles-recipe"></a>

### Step 8 of 27 · Declare atomic tile records and flat triangle inputs (`86d-copy-the-triangle-tiles-recipe`)

- **Goal:** Bind projected triangle records and a shared bounded reference pool with atomic count and cursor words.
- **Data Flow Trace:** `declare atomic tile records and flat triangle inputs`
- **Files You Touch:** `src/finite_triangle_tiles.wgsl` (*Bind projected triangle records and a shared bounded reference pool with atomic count and cursor words.*) · `src/finite_project_source.rs` (*Expand shared source sections once for projection, tile passes and finite ink.*)

Describe the projected-triangle and reference-pool bindings before introducing either raster pass. Counts and cursors use separate atomic words; both passes will reuse the existing conservative tile-span helper. These shader sections remain preparatory until their complete source is assembled and validated.

> **Self-Check Question:** *Why are count and cursor words atomic in the raster passes?*  
> **Answer:** Many triangle fragments can reach one tile concurrently. Atomic updates give each contribution a distinct count or list slot.

- **What You Should See:** This tiles section is written; its GPU passes are not connected yet.
- **Hands-On Lab Experiment:** Trace a triangle crossing a tile corner. Predict its count and reserved list slots, then compare the later connected demand readback. Why are count and cursor words atomic in the raster passes?

<details>
<summary>Implementation 86d-copy-the-triangle-tiles-recipe: Declare atomic tile records and flat triangle inputs</summary>

Bind projected triangle records and a shared bounded reference pool with atomic count and cursor words.

Data flow: declare atomic tile records and flat triangle inputs.

### 1. `src/finite_triangle_tiles.wgsl`

Bind projected triangle records and a shared bounded reference pool with atomic count and cursor words.

Create the file and type:

```wgsl
--8<-- "journey/code/86d-copy-the-triangle-tiles-recipe-01.wgsl"
```

### 2. `src/finite_project_source.rs`

Expand shared source sections once for projection, tile passes and finite ink.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn source() -> String {
    let shared = include_str!("projected_triangle.wgsl")
        .split("// Pixels per tile side for this canvas.").next().unwrap();
    include_str!("finite_project.wgsl").replace("#include \"projected_triangle.wgsl\"", shared)
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/86d-copy-the-triangle-tiles-recipe-02-source.rs"
```

</details>

<a id="checkpoint-86da-count-conservative-triangle-coverage-per-tile"></a>

### Step 9 of 27 · Count conservative triangle coverage per tile (`86da-count-conservative-triangle-coverage-per-tile`)

- **Goal:** Rasterize one tile-grid quad per triangle and reject tiles outside any inward edge before counting.
- **Data Flow Trace:** `count conservative triangle coverage per tile`
- **Files You Touch:** `src/finite_triangle_tiles.wgsl` (*Rasterize one tile-grid quad per triangle and reject tiles outside any inward edge before counting.*)

Add the count pass to the [tile bindings](../../86d-copy-the-triangle-tiles-recipe.md). Its vertex stage covers each triangle’s tile-grid rectangle; its fragment stage applies the shared inward-edge test before incrementing a tile count. It reserves no reference-pool entries.

> **Self-Check Question:** *Why reject a whole tile with an edge equation instead of testing only its centre?*  
> **Answer:** A centre can be outside while a corner overlaps the triangle. Include the tile half-span in the edge bound to keep the test conservative.

- **Hands-On Lab Experiment:** Trace a triangle crossing a tile corner. Predict its count and reserved list slots, then compare the later connected demand readback. Why reject a whole tile with an edge equation instead of testing only its centre?

<details>
<summary>Implementation 86da-count-conservative-triangle-coverage-per-tile: Count conservative triangle coverage per tile</summary>

Rasterize one tile-grid quad per triangle and reject tiles outside any inward edge before counting.

Data flow: count conservative triangle coverage per tile.

### 1. `src/finite_triangle_tiles.wgsl`

Rasterize one tile-grid quad per triangle and reject tiles outside any inward edge before counting.

<details>
<summary>Locate the existing block</summary>

```wgsl
    @location(6) @interpolate(flat) gradient: vec3<f32>, // depth slope and nearest depth
    @location(7) @interpolate(flat) reference: vec3<f32>, // reference point and its depth
};
#include "projected_triangle.wgsl"
#include "projected_triangle.wgsl:viewport-span"
```

</details>

After that block, type:

```wgsl

@vertex
fn vs_main(@builtin(vertex_index) vertex: u32, @builtin(instance_index) instance: u32) -> TileVertex {
    let triangle = projected[instance];
    var out: TileVertex;
    out.clip = vec4<f32>(2.0, 2.0, 0.0, 1.0);

    // empty triangle: off screen
    if (triangle.edge3.w<3.0) {
        return out;
    }

    // tile grid size
    let size = ceil(vec2<f32>(line.vp_w, line.vp_h)/f32(visibility_tile_span()));
    let lo = clamp(floor((triangle.bounds.xy-0.00390625)/f32(visibility_tile_span())), vec2<f32>(0.0), size);
    let hi = clamp(floor((triangle.bounds.zw+0.00390625)/f32(visibility_tile_span()))+1.0, vec2<f32>(0.0), size);
    // triangle strip: two triangles from four corners
    let corners = array<vec2<f32>, 4>(vec2<f32>(0.0, 0.0), vec2<f32>(1.0, 0.0), vec2<f32>(0.0, 1.0), vec2<f32>(1.0, 1.0));
    let position = mix(lo, hi, corners[vertex])/size;
    out.clip = vec4<f32>(position.x*2.0-1.0, 1.0-position.y*2.0, 0.0, 1.0);
    out.edge0 = triangle.edge0.xyz;
    out.edge1 = triangle.edge1.xyz;
    out.edge2 = triangle.edge2.xyz;
    out.edge3 = triangle.edge3.xyz;
    out.bounds = triangle.bounds;
    out.primitive = instance+1u;
    out.gradient = triangle.gradient.xyz;
    out.reference = vec3<f32>(triangle.edge0.w, triangle.edge1.w, triangle.edge2.w);
    return out;
}
fn tile_outside(edge: vec3<f32>, centre: vec2<f32>) -> bool {
    return dot(edge.xy, centre)+edge.z+0.5*f32(visibility_tile_span())*(abs(edge.x)+abs(edge.y)) < -0.00390625;
}
fn covered_tile(v: TileVertex) -> u32 {
    let centre = (floor(v.clip.xy)+0.5)*f32(visibility_tile_span());

    if (tile_outside(v.edge0, centre) || tile_outside(v.edge1, centre) || tile_outside(v.edge2, centre) || tile_outside(v.edge3, centre)) {
        discard;
    }

    let size = vec2<u32>(ceil(vec2<f32>(line.vp_w, line.vp_h)/f32(visibility_tile_span())));
    let tile = 1u+u32(v.clip.y)*size.x+u32(v.clip.x);
    return tile;
}
@fragment
fn fs_count(v: TileVertex) -> @location(0) f32 {
    let tile = covered_tile(v);
    atomicAdd(&tile_records[tile].values[0], 1u);
    return 0.0;
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86da-count-conservative-triangle-coverage-per-tile-01.wgsl"
```

</details>

</details>

<a id="checkpoint-86db-fill-bounded-original-triangle-reference-lists"></a>

### Step 10 of 27 · Fill bounded original triangle reference lists (`86db-fill-bounded-original-triangle-reference-lists`)

- **Goal:** Atomically reserve a tile slot, store the original projected triangle address and depth, and flag overflow instead of writing past the pool.
- **Data Flow Trace:** `fill bounded original triangle reference lists`
- **Files You Touch:** `src/finite_triangle_tiles.wgsl` (*Atomically reserve a tile slot, store the original projected triangle address and depth, and flag overflow instead of writing past the pool.*)

Use the [count pass’s coverage](../../86da-count-conservative-triangle-coverage-per-tile.md) for filling. Each covered tile reserves one cursor position, checks the pool limit and writes the original projected-triangle address and depth. Overflow sets the shared flag instead of writing outside the buffer.

> **Self-Check Question:** *What happens when a reserved reference is outside this tile’s list capacity?*  
> **Answer:** Set its overflow word and skip the store. The later demand readback grows the bounded pool; an allocation guess must not become an out-of-bounds GPU write.

- **What You Should See:** The complete tiles recipe is ready for source assembly and validation.
- **Hands-On Lab Experiment:** Trace a triangle crossing a tile corner. Predict its count and reserved list slots, then compare the later connected demand readback. What happens when a reserved reference is outside this tile’s list capacity?

<details>
<summary>Implementation 86db-fill-bounded-original-triangle-reference-lists: Fill bounded original triangle reference lists</summary>

Atomically reserve a tile slot, store the original projected triangle address and depth, and flag overflow instead of writing past the pool.

Data flow: fill bounded original triangle reference lists.

### 1. `src/finite_triangle_tiles.wgsl`

Atomically reserve a tile slot, store the original projected triangle address and depth, and flag overflow instead of writing past the pool.

<details>
<summary>Locate the existing block</summary>

```wgsl
    let tile = 1u+u32(v.clip.y)*size.x+u32(v.clip.x);
    return tile;
}
@fragment
fn fs_count(v: TileVertex) -> @location(0) f32 {
    let tile = covered_tile(v);
    atomicAdd(&tile_records[tile].values[0], 1u);
    return 0.0;
}
```

</details>

After that block, type:

```wgsl

@fragment
fn fs_fill(v: TileVertex) -> @location(0) f32 {
    let tile = covered_tile(v);

    // list overflowed: stop
    if (atomicLoad(&tile_records[tile].values[3])!=0u) {
        return 0.0;
    }

    let cursor = atomicAdd(&tile_records[tile].values[2], 1u);

    // more writes than counted: mark overflow
    if (cursor>=atomicLoad(&tile_records[tile].values[0])) {
        atomicStore(&tile_records[tile].values[3], 1u);
        return 0.0;
    }

    let offset = atomicLoad(&tile_records[tile].values[1])+cursor*2u;
    atomicStore(&tile_records[offset/4u].values[offset%4u], v.primitive);
    let centre = (floor(v.clip.xy)+0.5)*f32(visibility_tile_span());
    let slope = abs(v.gradient.x)+abs(v.gradient.y);
    // nearest depth the triangle reaches in this tile
    let nearest = min(v.gradient.z, v.reference.z+dot(v.gradient.xy, centre-v.reference.xy)+0.5*f32(visibility_tile_span())*slope);
    // plus float and vertex-snap tolerance
    let bound = nearest+abs(nearest)*1.9073486e-6+slope*0.00390625;
    atomicStore(&tile_records[(offset+1u)/4u].values[(offset+1u)%4u], bitcast<u32>(bound));
    return 0.0;
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86db-fill-bounded-original-triangle-reference-lists-01.wgsl"
```

</details>

</details>

<a id="checkpoint-86e-count-and-fill-original-triangle-references"></a>

### Step 11 of 27 · Assemble and validate the bounded tiles shader (`86e-count-and-fill-original-triangle-references`)

- **Goal:** Reuse shared WGSL source and validate the complete tiles pass interfaces.
- **Data Flow Trace:** `original buffers → finite projected records → tile references`
- **Files You Touch:** `src/renderer.rs` (*Check the complete recipe on the actual browser GPU before the later pass connection.*) · `src/finite_project_source.rs` (*Reuse one source expansion owner for the completed recipe.*) · `src/finite_tiles_contract.rs` (*Supply independent assembled WGSL entry-point witnesses.*) · `src/lib.rs` (*Register the supplied shader interface fixture.*)

Reuse the shader source owner introduced at86c.

The completed tiles recipe already targets the application’s original buffer layout; do not copy a second recipe and replace its geometry adapter. Validate the actual assembled text, then use this same source in the later pipeline. The browser device now compiles the complete shader and reports compilation info in the developer console. The later connection checks execution and pixels.

> **Self-Check Question:** *Why must the tiles shader be validated after shared-source expansion?*  
> **Answer:** The expanded text is the shader passed to wgpu. Its shared record layouts, entry points and resource types must agree before pipeline creation.

- **What You Should See:** The actual browser device compiles the assembled tiles shader and reports its compilation info before pass connection.
- **Hands-On Lab Experiment:** Read the Finite tiles compilation report in the browser developer console. Temporarily introduce and restore a WGSL type error. Then list the storage bindings and predict the ordering with projection/count/scan/fill before the later connected fixture.

<details>
<summary>Implementation 86e-count-and-fill-original-triangle-references: Assemble and validate the bounded tiles shader</summary>

Reuse shared WGSL source and validate the complete tiles pass interfaces.

Data flow: original buffers → finite projected records → tile references.

### 1. `src/renderer.rs`

Check the complete recipe on the actual browser GPU before the later pass connection.

<details>
<summary>Locate the existing block</summary>

```rust
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        crate::shader_check::check(&device, crate::finite_project_source::tiles(), "Finite tiles");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86e-count-and-fill-original-triangle-references-01-source-reuse-check.rs"
```

</details>

### 2. `src/finite_project_source.rs`

Reuse one source expansion owner for the completed recipe.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn source() -> String { expand(include_str!("finite_project.wgsl")) }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn tiles() -> String { expand(include_str!("finite_triangle_tiles.wgsl")) }

pub fn source() -> String { expand(include_str!("finite_project.wgsl")) }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86e-count-and-fill-original-triangle-references-02-source.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/finite_tiles_contract.rs`

Supply independent assembled WGSL entry-point witnesses.

Copy this supplied file:

```text
--8<-- "journey/code/86e-count-and-fill-original-triangle-references-03-source.rs"
```

</details>

### 4. `src/lib.rs`

Register the supplied shader interface fixture.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod finite_project_source;
```

</details>

After that block, type:

```rust
#[cfg(test)]
mod finite_tiles_contract;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86e-count-and-fill-original-triangle-references-04-source.rs"
```

</details>

</details>

<a id="checkpoint-86f-copy-the-scan-triangle-tiles-recipe"></a>

### Step 12 of 27 · Scan bounded tile counts in workgroup memory (`86f-copy-the-scan-triangle-tiles-recipe`)

- **Goal:** Build a saturating exclusive prefix sum with uniform barriers and shared scratch storage.
- **Data Flow Trace:** `scan bounded tile counts in workgroup memory`
- **Files You Touch:** `src/finite_scan_triangle_tiles.wgsl` (*Build a saturating exclusive prefix sum with uniform barriers and shared scratch storage.*)

Define one saturating exclusive-prefix helper for both levels of the scan. Every lane reaches its workgroup barriers, including lanes outside the live input range. This helper remains preparatory until the [complete scan source is validated](../../86fc-assemble-and-validate-the-bounded-scan-shader.md).

> **Self-Check Question:** *Why must inactive lanes still enter every workgroup barrier?*  
> **Answer:** A barrier synchronizes the whole workgroup. Inactive lanes contribute zero and continue, so active lanes never wait on a lane that returned early.

- **What You Should See:** This scan section is written; its GPU passes are not connected yet.
- **Hands-On Lab Experiment:** Trace257 live tiles through two workgroups. Include the inactive lanes, the block totals and an exhausted pool. Why must inactive lanes still enter every workgroup barrier?

<details>
<summary>Implementation 86f-copy-the-scan-triangle-tiles-recipe: Scan bounded tile counts in workgroup memory</summary>

Build a saturating exclusive prefix sum with uniform barriers and shared scratch storage.

Data flow: scan bounded tile counts in workgroup memory.

### 1. `src/finite_scan_triangle_tiles.wgsl`

Build a saturating exclusive prefix sum with uniform barriers and shared scratch storage.

Create the file and type:

```wgsl
--8<-- "journey/code/86f-copy-the-scan-triangle-tiles-recipe-01.wgsl"
```

</details>

<a id="checkpoint-86fa-scan-tile-blocks-and-their-block-totals"></a>

### Step 13 of 27 · Scan tile blocks and their block totals (`86fa-scan-tile-blocks-and-their-block-totals`)

- **Goal:** Run the same prefix owner first over tile counts and then over their block totals.
- **Data Flow Trace:** `scan tile blocks and their block totals`
- **Files You Touch:** `src/finite_scan_triangle_tiles.wgsl` (*Run the same prefix owner first over tile counts and then over their block totals.*)

Reuse the [prefix helper](../../86f-copy-the-scan-triangle-tiles-recipe.md) in two dispatch entry points. The first scans tile counts within each block and publishes its total; the second scans those totals. Dispatch order supplies the boundary between the two levels.

> **Self-Check Question:** *Why use a second scan over block totals?*  
> **Answer:** The workgroup prefix sees only its own256 lanes. Scanning block totals provides the base offset for each block without a global barrier inside one dispatch.

- **Hands-On Lab Experiment:** Trace257 live tiles through two workgroups. Include the inactive lanes, the block totals and an exhausted pool. Why use a second scan over block totals?

<details>
<summary>Implementation 86fa-scan-tile-blocks-and-their-block-totals: Scan tile blocks and their block totals</summary>

Run the same prefix owner first over tile counts and then over their block totals.

Data flow: scan tile blocks and their block totals.

### 1. `src/finite_scan_triangle_tiles.wgsl`

Run the same prefix owner first over tile counts and then over their block totals.

<details>
<summary>Locate the existing block</summary>

```wgsl
            previous = scan[lane-stride];
        }

        workgroupBarrier();
        scan[lane] = min(scan[lane]+previous, capacity);
        workgroupBarrier();
    }

    return select(scan[lane]-bounded, capacity, scan[lane]==capacity);
}
```

</details>

After that block, type:

```wgsl

@compute @workgroup_size(256)
fn scan_tiles(@builtin(global_invocation_id) id: vec3<u32>, @builtin(local_invocation_index) lane: u32, @builtin(workgroup_id) group: vec3<u32>) {
    let count = tile_count();
    var value = 0u;

    if (id.x<count) {
        // two words per reference
        value = records[1u+id.x].values[0]*2u;
    }

    let offset = prefix(lane, value);

    if (id.x<count) {
        records[1u+id.x].values[1] = offset;
    }

    // block total, for pass 2
    if (lane==255u) {
        records[1u+count+group.x].values[0] = scan[255];
    }
}
@compute @workgroup_size(256)
fn scan_blocks(@builtin(global_invocation_id) id: vec3<u32>, @builtin(local_invocation_index) lane: u32, @builtin(workgroup_id) group: vec3<u32>) {
    let count = tile_count();
    let block_count = blocks(count);
    var value = 0u;

    if (id.x<block_count) {
        value = records[1u+count+id.x].values[0];
    }

    let offset = prefix(lane, value);

    if (id.x<block_count) {
        records[1u+count+id.x].values[1] = offset;
    }

    if (lane==255u) {
        records[1u+count+block_count+group.x].values[0] = scan[255];
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86fa-scan-tile-blocks-and-their-block-totals-01.wgsl"
```

</details>

</details>

<a id="checkpoint-86fb-finalize-bounded-tile-list-offsets-and-demand"></a>

### Step 14 of 27 · Finalize bounded tile list offsets and demand (`86fb-finalize-bounded-tile-list-offsets-and-demand`)

- **Goal:** Combine block and local offsets, reset fill cursors, and record actual pool demand in the shared header.
- **Data Flow Trace:** `finalize bounded tile list offsets and demand`
- **Files You Touch:** `src/finite_scan_triangle_tiles.wgsl` (*Combine block and local offsets, reset fill cursors, and record actual pool demand in the shared header.*)

Combine each tile’s block base and local prefix into its final list offset. Reset the fill cursor, retain bounded overflow reporting and publish the total reference demand. The [next checkpoint](../../86fc-assemble-and-validate-the-bounded-scan-shader.md) assembles and validates the complete shader.

> **Self-Check Question:** *Why reset the fill cursor after calculating the list start?*  
> **Answer:** The count pass measured demand. The fill pass needs a new cursor starting at zero so each reference is written once inside its assigned list.

- **What You Should See:** The complete scan recipe is ready for source assembly and validation.
- **Hands-On Lab Experiment:** Trace257 live tiles through two workgroups. Include the inactive lanes, the block totals and an exhausted pool. Why reset the fill cursor after calculating the list start?

<details>
<summary>Implementation 86fb-finalize-bounded-tile-list-offsets-and-demand: Finalize bounded tile list offsets and demand</summary>

Combine block and local offsets, reset fill cursors, and record actual pool demand in the shared header.

Data flow: finalize bounded tile list offsets and demand.

### 1. `src/finite_scan_triangle_tiles.wgsl`

Combine block and local offsets, reset fill cursors, and record actual pool demand in the shared header.

<details>
<summary>Locate the existing block</summary>

```wgsl
    let offset = prefix(lane, value);

    if (id.x<block_count) {
        records[1u+count+id.x].values[1] = offset;
    }

    if (lane==255u) {
        records[1u+count+block_count+group.x].values[0] = scan[255];
    }
}
```

</details>

After that block, type:

```wgsl

@compute @workgroup_size(256)
fn finish_offsets(@builtin(global_invocation_id) id: vec3<u32>) {
    let count = tile_count();

    if (id.x==0u) {
        // record 0: valid flag
        records[0].values[0] = 1u;
    }

    if (id.x>=count) {
        return;
    }

    let block_count = blocks(count);
    let pool = 4u*(1u+count+block_count+blocks(block_count));
    let tile = 1u+id.x;
    let block = id.x/256u;
    var offset = records[tile].values[1]+records[1u+count+block].values[1];

    for (var i = 0u;i<block/256u;i++) {
        offset+=records[1u+count+block_count+i].values[0];
    }

    let start = pool+offset;
    records[tile].values[1] = start;
    // overflow flag when the list runs past the buffer
    records[tile].values[3] = select(0u, 1u, start+records[tile].values[0]*2u>arrayLength(&records)*4u);

    // record 0 word 1: words needed; the CPU reads it back
    if (id.x==count-1u) {
        records[0].values[1] = start+records[tile].values[0]*2u;
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86fb-finalize-bounded-tile-list-offsets-and-demand-01.wgsl"
```

</details>

</details>

<a id="checkpoint-86fc-assemble-and-validate-the-bounded-scan-shader"></a>

### Step 15 of 27 · Assemble and validate the bounded scan shader (`86fc-assemble-and-validate-the-bounded-scan-shader`)

- **Goal:** Reuse shared WGSL source and validate the complete scan pass interfaces.
- **Data Flow Trace:** `original buffers → finite projected records → tile references`
- **Files You Touch:** `src/renderer.rs` (*Check the complete recipe on the actual browser GPU before the later pass connection.*) · `src/finite_project_source.rs` (*Reuse one source expansion owner for the completed recipe.*) · `src/finite_scan_contract.rs` (*Supply independent assembled WGSL entry-point witnesses.*) · `src/lib.rs` (*Register the supplied shader interface fixture.*)

Reuse the shader source owner introduced at86c.

The completed scan recipe already targets the application’s original buffer layout; do not copy a second recipe and replace its geometry adapter. Validate the actual assembled text, then use this same source in the later pipeline. The browser device now compiles the complete shader and reports compilation info in the developer console. The later connection checks execution and pixels.

> **Self-Check Question:** *What must the complete scan shader validate before dispatch?*  
> **Answer:** Both scan entry points must agree on the shared prefix helper, bindings and workgroup types. Validate the actual expanded module, as with the [count/fill shader](../../86e-count-and-fill-original-triangle-references.md).

- **What You Should See:** The actual browser device compiles the assembled scan shader and reports its compilation info before pass connection.
- **Hands-On Lab Experiment:** Read the Finite scan compilation report in the browser developer console. Temporarily introduce and restore a WGSL type error. Then list the storage bindings and predict the ordering with projection/count/scan/fill before the later connected fixture.

<details>
<summary>Implementation 86fc-assemble-and-validate-the-bounded-scan-shader: Assemble and validate the bounded scan shader</summary>

Reuse shared WGSL source and validate the complete scan pass interfaces.

Data flow: original buffers → finite projected records → tile references.

### 1. `src/renderer.rs`

Check the complete recipe on the actual browser GPU before the later pass connection.

<details>
<summary>Locate the existing block</summary>

```rust
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        crate::shader_check::check(&device, crate::finite_project_source::scan(), "Finite scan");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86fc-assemble-and-validate-the-bounded-scan-shader-01-source-reuse-check.rs"
```

</details>

### 2. `src/finite_project_source.rs`

Reuse one source expansion owner for the completed recipe.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn source() -> String { expand(include_str!("finite_project.wgsl")) }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn scan() -> String { expand(include_str!("finite_scan_triangle_tiles.wgsl")) }

pub fn source() -> String { expand(include_str!("finite_project.wgsl")) }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86fc-assemble-and-validate-the-bounded-scan-shader-02-source.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/finite_scan_contract.rs`

Supply independent assembled WGSL entry-point witnesses.

Copy this supplied file:

```text
--8<-- "journey/code/86fc-assemble-and-validate-the-bounded-scan-shader-03-source.rs"
```

</details>

### 4. `src/lib.rs`

Register the supplied shader interface fixture.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod finite_project_source;
```

</details>

After that block, type:

```rust
#[cfg(test)]
mod finite_scan_contract;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86fc-assemble-and-validate-the-bounded-scan-shader-04-source.rs"
```

</details>

</details>

<a id="checkpoint-86g-wake-only-after-a-tile-report-arrives"></a>

### Step 16 of 27 · Wake only after a tile report arrives (`86g-wake-only-after-a-tile-report-arrives`)

- **Goal:** Wake only after a tile report arrives.
- **Data Flow Trace:** `completed map → browser event → one redraw`
- **Files You Touch:** `src/browser_tiles.rs` (*Wake only after a tile report arrives.*) · `src/lib.rs` (*Register the completed source owner.*)

Wake only after a tile report arrives.

> **Self-Check Question:** *Does an unchanged view need a timer loop?*  
> **Answer:** No. A completed report schedules the next required frame; a settled view stays idle.

- **Hands-On Lab Experiment:** Inspect the event sent only by the asynchronous callback.

<details>
<summary>Implementation 86g-wake-only-after-a-tile-report-arrives: Wake only after a tile report arrives</summary>

Wake only after a tile report arrives.

Data flow: completed map → browser event → one redraw.

### 1. `src/browser_tiles.rs`

Create the file and type:

```rust
--8<-- "journey/code/86g-wake-only-after-a-tile-report-arrives-01.rs"
```

### 2. `src/lib.rs`

Register the completed source owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod tile_math;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/86g-wake-only-after-a-tile-report-arrives-02.rs" skip=20
```

</details>

<a id="checkpoint-86h-read-actual-pool-demand-asynchronously"></a>

### Step 17 of 27 · Read actual pool demand asynchronously (`86h-read-actual-pool-demand-asynchronously`)

- **Goal:** Read GPU tile-pool demand asynchronously, with one map in flight and one reusable readback buffer.
- **Data Flow Trace:** `scan header → sixteen-byte copy → map result`
- **Files You Touch:** `src/tile_report.rs` (*Type the complete existing WebGPU helper and reuse its current consumer.*) · `src/lib.rs` (*Register the completed source owner.*)

Write the existing sixteen-byte GPU readback owner. Encode a copy only when no map is in flight, start asynchronous mapping after submission, and poll the completion flag without blocking the browser. Drop the mapped view before unmapping and reuse the same buffer for the next report.

> **Self-Check Question:** *Can a mapping in flight accept another copy?*  
> **Answer:** No. Return whether the copy was encoded; the bin generation retries only its unreported current result.

- **Hands-On Lab Experiment:** Trace callback state, map failure and buffer unmapping.

<details>
<summary>Implementation 86h-read-actual-pool-demand-asynchronously: Read actual pool demand asynchronously</summary>

Read GPU tile-pool demand asynchronously, with one map in flight and one reusable readback buffer.

Data flow: scan header → sixteen-byte copy → map result.

### 1. `src/tile_report.rs`

Type the complete existing WebGPU helper and reuse its current consumer. Return actual copy admission so a newer bin generation can retry after an old report.

Create the file and type:

```rust
use std::sync::{Arc,atomic::{AtomicU8,Ordering}};
/// Reads back how many words the last scan needed.
pub struct PoolReport {
    buffer: wgpu::Buffer, // 16-byte CPU-readable copy
    ready: Arc<AtomicU8>, // 0 waiting, 1 mapped, 2 failed
    copied: bool,         // a copy was encoded this frame
    inflight: bool,       // the copy is being mapped
}

impl PoolReport {
    /// Create the readback buffer.
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            buffer: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("triangle.tiles.report"),
                size: 16,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            }),
            ready: Arc::new(AtomicU8::new(0)),
            copied: false,
            inflight: false,
        }
    }

    /// Copy the first tile record into the readback buffer.
    pub fn copy(&mut self, encoder: &mut wgpu::CommandEncoder, tiles: &wgpu::Buffer) ->bool {
        if self.inflight {
            return false;
        }

        encoder.copy_buffer_to_buffer(tiles, 0, &self.buffer, 0, 16);
        self.copied = true;
        true
    }

    /// Start mapping the copy; call once after submit.
    pub fn map(&mut self) {
        if !self.copied || self.inflight {
            return;
        }

        self.copied = false;
        self.inflight = true;
        let flag = self.ready.clone();
        self.buffer
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                flag.store(if result.is_ok() { 1 } else { 2 }, Ordering::Release);
                #[cfg(target_arch="wasm32")] crate::browser_tiles::wake();
            });
    }

    /// Words the last scan needed, once the copy is readable.
    pub fn poll(&mut self) -> Option<u64> {
        if !self.inflight {
            return None;
        }

        let status = self.ready.load(Ordering::Acquire);

        if status == 0 {
            return None;
        }

        self.ready.store(0, Ordering::Release);
        self.inflight = false;

        if status != 1 {
            return None;
        }

        let words = {
            let bytes = self.buffer.slice(..).get_mapped_range();
            u32::from_le_bytes(bytes[4..8].try_into().expect("report words"))
        };
        self.buffer.unmap();
        Some(u64::from(words))
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86h-read-actual-pool-demand-asynchronously-typed-tile_report.rs"
```

```text
--8<-- "journey/code/86h-read-actual-pool-demand-asynchronously-02.rs"
```

</details>

### 2. `src/lib.rs`

Register the completed source owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod browser_tiles;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/86h-read-actual-pool-demand-asynchronously-03.rs" skip=24
```

</details>

<a id="checkpoint-86i-describe-explicit-finite-gpu-layouts"></a>

### Step 18 of 27 · Describe explicit finite GPU layouts (`86i-describe-explicit-finite-gpu-layouts`)

- **Goal:** Compile the finite pipeline resource types before their helper functions.
- **Data Flow Trace:** `uniform and storage visibility → explicit binding layouts`
- **Files You Touch:** `src/tile_pipelines.rs` (*Describe explicit finite GPU layouts.*) · `src/lib.rs` (*Register the resource types immediately so every following method is checked by the browser compiler.*)

Declare the pipeline and layout owner, and register its module immediately. These fields establish the resource boundary; the following lessons add the builder helpers and pipeline creation. The existing drawing stays unchanged.

> **Self-Check Question:** *Does an inferred shader layout stay compatible with another pipeline?*  
> **Answer:** It need not. These pipelines share explicit layouts for projection, raster counts and scans.

- **What You Should See:** The Pipelines type now belongs to the browser module graph. Declaring it creates no GPU pipelines yet.
- **Hands-On Lab Experiment:** Build the browser and inspect the existing drawing. Read the layout and pipeline fields, then locate the module declaration that makes Rust check the following helper methods.

<details>
<summary>Implementation 86i-describe-explicit-finite-gpu-layouts: Describe explicit finite GPU layouts</summary>

Compile the finite pipeline resource types before their helper functions.

Data flow: uniform and storage visibility → explicit binding layouts.

### 1. `src/tile_pipelines.rs`

Describe explicit finite GPU layouts.

Create the file and type:

```rust
--8<-- "journey/code/86i-describe-explicit-finite-gpu-layouts-01.rs"
```

### 2. `src/lib.rs`

Register the resource types immediately so every following method is checked by the browser compiler.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod tile_report;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/86k-connect-projection-count-scan-and-fill-pipelines-02.rs" skip=22
```

</details>

<a id="checkpoint-86j-build-compute-and-count-pipeline-helpers"></a>

### Step 19 of 27 · Build compute and count pipeline helpers (`86j-build-compute-and-count-pipeline-helpers`)

- **Goal:** Build compute and count pipeline helpers.
- **Data Flow Trace:** `validated shader → explicit pipeline layout → GPU pipeline`
- **Files You Touch:** `src/tile_pipelines.rs` (*Build compute and count pipeline helpers.*)

Build compute and count pipeline helpers.

> **Self-Check Question:** *Does the raster target carry a picture?*  
> **Answer:** No. Its R8 target drives covered fragments with color writes disabled.

- **Hands-On Lab Experiment:** Compare compute entry points with triangle-strip count/fill rasterization.

<details>
<summary>Implementation 86j-build-compute-and-count-pipeline-helpers: Build compute and count pipeline helpers</summary>

Build compute and count pipeline helpers.

Data flow: validated shader → explicit pipeline layout → GPU pipeline.

### 1. `src/tile_pipelines.rs`

<details>
<summary>Locate the existing block</summary>

```rust


use wgpu::{BufferBindingType as Type,ShaderStages as Stage};


pub struct Pipelines{pub line:wgpu::BindGroupLayout,pub project_layout:wgpu::BindGroupLayout,pub raster_layout:wgpu::BindGroupLayout,pub scan_layout:wgpu::BindGroupLayout,pub project:wgpu::ComputePipeline,pub count:wgpu::RenderPipeline,pub fill:wgpu::RenderPipeline,pub scans:[wgpu::ComputePipeline;3]}
impl Pipelines{
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/86j-build-compute-and-count-pipeline-helpers-01.rs"
```

</details>

<a id="checkpoint-86k-connect-projection-count-scan-and-fill-pipelines"></a>

### Step 20 of 27 · Connect projection count scan and fill pipelines (`86k-connect-projection-count-scan-and-fill-pipelines`)

- **Goal:** Connect projection count scan and fill pipelines.
- **Data Flow Trace:** `project → count → scan → fill`
- **Files You Touch:** `src/tile_pipelines.rs` (*Connect projection count scan and fill pipelines.*)

Connect projection count scan and fill pipelines.

> **Self-Check Question:** *Can fill precede the prefix scan?*  
> **Answer:** No. Each tile needs its disjoint output interval before writing references.

- **Hands-On Lab Experiment:** Inspect the ordered entry-point names and their exact binding groups.

<details>
<summary>Implementation 86k-connect-projection-count-scan-and-fill-pipelines: Connect projection count scan and fill pipelines</summary>

Connect projection count scan and fill pipelines.

Data flow: project → count → scan → fill.

### 1. `src/tile_pipelines.rs`

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Pipelines{pub line:wgpu::BindGroupLayout,pub project_layout:wgpu::BindGroupLayout,pub raster_layout:wgpu::BindGroupLayout,pub scan_layout:wgpu::BindGroupLayout,pub project:wgpu::ComputePipeline,pub count:wgpu::RenderPipeline,pub fill:wgpu::RenderPipeline,pub scans:[wgpu::ComputePipeline;3]}
impl Pipelines{

}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/86k-connect-projection-count-scan-and-fill-pipelines-01-shared-source-shared-bins.rs"
```

</details>

<a id="checkpoint-86l-own-bounded-finite-tile-resources"></a>

### Step 21 of 27 · Own bounded finite tile resources (`86l-own-bounded-finite-tile-resources`)

- **Goal:** Compile bounded tile resource types before allocation and population.
- **Data Flow Trace:** `original buffers → derived GPU owner → explicit cleanup`
- **Files You Touch:** `src/tile_gpu.rs` (*Own bounded finite tile resources.*) · `src/gpu_mesh.rs` (*Expose existing buffers without changing original data.*) · `src/lib.rs` (*Register the resource types immediately so every following method is checked by the browser compiler.*)

Declare the tile buffers, cache witnesses and report owner after their pipeline and layout prerequisites. Register the module now so each following method is compiled when introduced. The renderer will supply meshes and dimensions at its later connection.

> **Self-Check Question:** *Do cloned buffer handles allocate geometry again?*  
> **Answer:** No. They reference existing original uploads; the tile owner allocates only its derived records and metadata.

- **What You Should See:** The tile resource types and buffer factory compile. Their constructor and renderer connection follow in the next lessons.
- **Hands-On Lab Experiment:** Build the unchanged browser viewer, then trace the tile owner fields to Pipelines, TileLayout and PoolReport. Distinguish declaring a buffer handle from calling the allocation helper.

<details>
<summary>Implementation 86l-own-bounded-finite-tile-resources: Own bounded finite tile resources</summary>

Compile bounded tile resource types before allocation and population.

Data flow: original buffers → derived GPU owner → explicit cleanup.

### 1. `src/tile_gpu.rs`

Own bounded finite tile resources.

Create the file and type:

```rust
--8<-- "journey/code/86l-own-bounded-finite-tile-resources-01.rs"
```

### 2. `src/gpu_mesh.rs`

Expose existing buffers without changing original data.

<details>
<summary>Locate the existing block</summary>

```rust
    }

    pub(crate) fn packed(&self)->[u8;160] {self.previous}
    pub fn selected(&self)->bool {u32::from_ne_bytes(self.previous[64..68].try_into().unwrap())!=0}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/86ak-expose-original-gpu-models-to-projection-02.rs"
```

### 3. `src/lib.rs`

Register the resource types immediately so every following method is checked by the browser compiler.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod tile_pipelines;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/86r-cache-projection-and-finish-current-reports-02.rs" skip=25
```

</details>

<a id="checkpoint-86m-allocate-empty-tile-placeholders"></a>

### Step 22 of 27 · Allocate empty tile placeholders (`86m-allocate-empty-tile-placeholders`)

- **Goal:** Allocate empty tile placeholders.
- **Data Flow Trace:** `current originals and view → bounded buffers → cached finite lists`
- **Files You Touch:** `src/tile_gpu.rs` (*Allocate empty tile placeholders.*)

Allocate empty tile placeholders.

> **Self-Check Question:** *Why retain a single projected record?*  
> **Answer:** Empty storage bindings remain valid. Empty scenes release large arrays and keep only small placeholders.

- **Hands-On Lab Experiment:** Inspect96 projected bytes and16 header bytes.

<details>
<summary>Implementation 86m-allocate-empty-tile-placeholders: Allocate empty tile placeholders</summary>

Allocate empty tile placeholders.

Data flow: current originals and view → bounded buffers → cached finite lists.

### 1. `src/tile_gpu.rs`

<details>
<summary>Locate the existing block</summary>

```rust
fn buffer(device:&wgpu::Device,label:&str,size:u64)->wgpu::Buffer{device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::COPY_SRC,mapped_at_creation:false})}
impl Tiles{
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/86m-allocate-empty-tile-placeholders-01.rs"
```

</details>

<a id="checkpoint-86n-witness-original-buffers-and-world-placement"></a>

### Step 23 of 27 · Witness original buffers and world placement (`86n-witness-original-buffers-and-world-placement`)

- **Goal:** Witness original buffers and world placement.
- **Data Flow Trace:** `current originals and view → bounded buffers → cached finite lists`
- **Files You Touch:** `src/tile_gpu.rs` (*Witness original buffers and world placement.*)

Witness original buffers and world placement.

> **Self-Check Question:** *Can a moved row reuse stale projected triangles?*  
> **Answer:** No. Buffer identities and the complete64-byte GPU model invalidate its witness.

- **Hands-On Lab Experiment:** Compare original u32 triangle intervals and device binding limits.

<details>
<summary>Implementation 86n-witness-original-buffers-and-world-placement: Witness original buffers and world placement</summary>

Witness original buffers and world placement.

Data flow: current originals and view → bounded buffers → cached finite lists.

### 1. `src/tile_gpu.rs`

<details>
<summary>Locate the existing block</summary>

```rust
  Self{projected,buffer,pipes,line,line_group,raster,scan,rows:Vec::new(),witness:Vec::new(),triangles:0,viewport:[0;2],layout:None,target:None,pool:0,report:PoolReport::new(device),key:None,projections:0,bins:0,reported:None,fallback:false}
 }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/86n-witness-original-buffers-and-world-placement-01.rs"
```

</details>

<a id="checkpoint-86o-release-obsolete-tile-lists-and-bindings"></a>

### Step 24 of 27 · Release obsolete tile lists and bindings (`86o-release-obsolete-tile-lists-and-bindings`)

- **Goal:** Release obsolete tile lists and bindings.
- **Data Flow Trace:** `current originals and view → bounded buffers → cached finite lists`
- **Files You Touch:** `src/tile_gpu.rs` (*Release obsolete tile lists and bindings.*)

Release obsolete tile lists and bindings.

> **Self-Check Question:** *Does releasing tiles delete originals?*  
> **Answer:** No. Derived arrays and bindings are dropped; original scene ownership stays separate.

- **Hands-On Lab Experiment:** Inspect placeholder rebinding and released row groups.

<details>
<summary>Implementation 86o-release-obsolete-tile-lists-and-bindings: Release obsolete tile lists and bindings</summary>

Release obsolete tile lists and bindings.

Data flow: current originals and view → bounded buffers → cached finite lists.

### 1. `src/tile_gpu.rs`

<details>
<summary>Locate the existing block</summary>

```rust
  self.witness=witness;self.triangles=count.unwrap();self.key=None;
 }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/86o-release-obsolete-tile-lists-and-bindings-01.rs"
```

</details>

<a id="checkpoint-86p-grow-lists-only-from-actual-demand"></a>

### Step 25 of 27 · Grow lists only from actual demand (`86p-grow-lists-only-from-actual-demand`)

- **Goal:** Grow lists only from actual demand.
- **Data Flow Trace:** `current originals and view → bounded buffers → cached finite lists`
- **Files You Touch:** `src/tile_gpu.rs` (*Grow lists only from actual demand.*)

Grow lists only from actual demand.

> **Self-Check Question:** *Does every orbit allocate a larger pool?*  
> **Answer:** No. Consume actual scan reports and grow only when demand exceeds capacity, within the device ceiling.

- **Hands-On Lab Experiment:** Compare resize, unchanged tile dimensions, spare projected capacity and physical-depth fallback.

<details>
<summary>Implementation 86p-grow-lists-only-from-actual-demand: Grow lists only from actual demand</summary>

Grow lists only from actual demand.

Data flow: current originals and view → bounded buffers → cached finite lists.

### 1. `src/tile_gpu.rs`

<details>
<summary>Locate the existing block</summary>

```rust
 fn rebind(&mut self,device:&wgpu::Device){self.raster=group(device,&self.pipes.raster_layout,&[&self.projected,&self.buffer]);self.scan=group(device,&self.pipes.scan_layout,&[&self.buffer]);for row in &mut self.rows{row.group=None;}}
 fn release(&mut self,device:&wgpu::Device){if self.layout.take().is_none()&&self.projected.size()==PROJECTED_BYTES&&self.buffer.size()==16{return}self.projected=buffer(device,"finite projected placeholder",PROJECTED_BYTES);self.buffer=buffer(device,"finite tile placeholder",16);self.pool=0;self.target=None;self.key=None;self.rebind(device);}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/86p-grow-lists-only-from-actual-demand-01.rs"
```

</details>

<a id="checkpoint-86q-rasterize-bounded-tile-counts-and-references"></a>

### Step 26 of 27 · Rasterize bounded tile counts and references (`86q-rasterize-bounded-tile-counts-and-references`)

- **Goal:** Rasterize bounded tile counts and references.
- **Data Flow Trace:** `current originals and view → bounded buffers → cached finite lists`
- **Files You Touch:** `src/tile_gpu.rs` (*Rasterize bounded tile counts and references.*)

Rasterize bounded tile counts and references.

> **Self-Check Question:** *Why draw four vertices per triangle?*  
> **Answer:** A strip visits its screen bounding box; the fragment tests finite triangle coverage and updates the matching tiles.

- **Hands-On Lab Experiment:** Trace original triangle count and the temporary tile target.

<details>
<summary>Implementation 86q-rasterize-bounded-tile-counts-and-references: Rasterize bounded tile counts and references</summary>

Rasterize bounded tile counts and references.

Data flow: current originals and view → bounded buffers → cached finite lists.

### 1. `src/tile_gpu.rs`

<details>
<summary>Locate the existing block</summary>

```rust
 }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/86q-rasterize-bounded-tile-counts-and-references-01.rs"
```

</details>

<a id="checkpoint-86r-cache-projection-and-finish-current-reports"></a>

### Step 27 of 27 · Cache projection and finish current reports (`86r-cache-projection-and-finish-current-reports`)

- **Goal:** Cache projection and finish current reports.
- **Data Flow Trace:** `current originals and view → bounded buffers → cached finite lists`
- **Files You Touch:** `src/tile_gpu.rs` (*Cache projection and finish current reports.*)

Cache projection and finish current reports.

> **Self-Check Question:** *Can a late old report strand newer demand?*  
> **Answer:** No. Track the reported bin generation and retry the current unreported header after the previous map completes.

- **Hands-On Lab Experiment:** Compare resting frames, rotation, dense overlap and released geometry.

<details>
<summary>Implementation 86r-cache-projection-and-finish-current-reports: Cache projection and finish current reports</summary>

Cache projection and finish current reports.

Data flow: current originals and view → bounded buffers → cached finite lists.

### 1. `src/tile_gpu.rs`

<details>
<summary>Locate the existing block</summary>

```rust
  self.layout=Some(layout);if moved{self.key=None;self.rebind(device);}
 }

 fn bin(&self,encoder:&mut wgpu::CommandEncoder,pipeline:&wgpu::RenderPipeline){let Some(target)=&self.target else{return};let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite tile lists"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:target,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Discard}})],depth_stencil_attachment:None,timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});pass.set_pipeline(pipeline);pass.set_bind_group(0,&self.line_group,&[]);pass.set_bind_group(1,&self.raster,&[]);pass.draw(0..4,0..self.triangles);}


}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/86r-cache-projection-and-finish-current-reports-01.rs"
```

</details>

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 86r-cache-projection-and-finish-current-reports
npm --prefix ../session_tests run course -- check 86r-cache-projection-and-finish-current-reports
npm --prefix ../session_tests run course -- save 86r-cache-projection-and-finish-current-reports
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

- **Final Lesson Experiment:** Compare resting frames, rotation, dense overlap and released geometry.

Continue with [Finite ink visibility and original colour/pick ownership](../../consolidation/86s-test-visibility-at-the-original-center-line.md).
