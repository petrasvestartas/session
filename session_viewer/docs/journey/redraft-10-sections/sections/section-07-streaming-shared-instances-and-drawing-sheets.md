# Section 7: Out-of-Core HTTP Range Streaming, Shared Instances, and Drawing Sheets

> **Gemini Notebook Student Guide** · **Absorbs:** Chapter 12 (*Files and streaming*), Chapter 13 (*Shared definitions and drawing sheets*) · **Lessons:** 8 (`12.1`–`13.4`) · **Checkpoints:** 132 (`66-read-scene-manifests` → `70za-read-original-sheet-metadata-records`)

## 1. Learning Outcome

Stream gigabyte-scale scene manifests and octree point clouds over HTTP `206 Partial Content` range requests without downloading full file bodies, enforce charged LRU memory residency budgets with coarse root pinning and generation-safe cancellation, render thousands of independently placed/colored CAD instances from a single shared definition (`69`), and stream/query 2D/3D architectural drawing sheets (`70za`).

## 2. WebGPU & Architecture Schematic (First-Year Student Guide)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 240" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="232" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Section 7 WebGPU Streaming & Instancing: HTTP Range Requests, Shared Source Meshes & 2D Drawing Sheets</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">How massive multi-gigabyte scenes stream out-of-core via HTTP 206 Partial Content and draw 1,000+ placements from 1 GPU buffer.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    <rect x="18" y="68" width="245" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="140" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">1. HTTP 206 Range Streaming (Ch 12)</text>
    <text x="28" y="108" font-size="10.5" fill="#111111">• Fetch compact Manifest + World Bounds</text>
    <text x="28" y="126" font-size="10.5" font-weight="600" fill="#111111">• Prioritize visible frustum chunks</text>
    <text x="28" y="144" font-size="10.5" fill="#111111">• Range: bytes=start-end (bounded memory)</text>
    <text x="28" y="162" font-size="10.5" fill="#111111">• Evict cold off-screen chunks under</text>
    <text x="28" y="178" font-size="10.5" fill="#111111">  memory budget without losing metadata</text>

    <line x1="263" y1="138" x2="293" y2="138" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="295" y="68" width="265" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="427" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">2. GPU Hardware Instancing (Ch 13)</text>
    <text x="307" y="108" font-size="10.5" fill="#111111">• 1 Shared Definition Buffer (e.g. Bolt)</text>
    <text x="307" y="126" font-size="10.5" font-weight="600" fill="#111111">• N Instance Rows (@builtin(instance_index)):</text>
    <text x="307" y="142" font-size="10" fill="#111111">  Per-instance mat4x4 transform, tint,</text>
    <text x="307" y="157" font-size="10" fill="#111111">  and distinct instance_owner_id</text>
    <text x="307" y="176" font-size="10.5" fill="#111111">• draw_indexed(0..idx_cnt, 0, 0..N_inst)</text>

    <line x1="560" y1="138" x2="590" y2="138" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="592" y="68" width="228" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="706" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">3. 2D Drawing Sheets &amp; Blocks</text>
    <text x="604" y="108" font-size="10.5" fill="#111111">• Paper-space Sheet frames &amp; title</text>
    <text x="604" y="124" font-size="10.5" fill="#111111">  blocks (SheetInit / SheetRows)</text>
    <text x="604" y="144" font-size="10.5" font-weight="600" fill="#111111">• Calibrated mm/inch pen widths</text>
    <text x="604" y="162" font-size="10.5" fill="#111111">• Switch smoothly between 3D Model</text>
    <text x="604" y="178" font-size="10.5" fill="#111111">  view and 2D Sheet layouts</text>
    
</svg>
</div>

```
+---------------------------------------------------------------------------------------------------+
|            HTTP 206 RANGE STREAMING, LRU RESIDENCY & SHARED INSTANCING (CHAPTERS 12 & 13)         |
+---------------------------------------------------------------------------------------------------+
  Remote .pb Source (HTTP Server)
    |
    +--> 1. Header Probe (Range: bytes=0-65535) ---> Locate Array Byte Offsets (Zero full download!)
    |
    +--> 2. Permit Queue (Max N in-flight HTTP 206 Range Reads, Revision-checked, Cancellable)
    |
    v
  +-----------------------------------------------------------------------------------------------+
  | Charged Page Residency Cache (Reserve bytes BEFORE read; Pin Coarse Octree Root; Evict LRU)   |
  +-----------------------------------------------------------------------------------------------+
    |
    v
  +-----------------------------------------------------------------------------------------------+
  | Shared Definition Table: 1 Geometry Upload  ===>  Instance[0] (Transform T0, Color C0, ID 101)|
  | (Mesh / B-Rep / Cloud / Drawing Sheet Page) ===>  Instance[1] (Transform T1, Color C1, ID 102)|
  +-----------------------------------------------------------------------------------------------+
```

### Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `Browser`)

| Layer | Construct | What It Does in Plain English |
| :--- | :--- | :--- |
| **HTTP / Fetch** | `Range: bytes=start-end` -> `206 Partial Content` | Fetches only the exact byte slice of an octree node or sheet column needed for the current camera view. |
| **Protobuf Wire** | Varint tag + length prefix scanner | Skips over multi-megabyte coordinate arrays by reading their byte range offsets without allocating their contents. |
| **Rust (Residency)** | `budget.reserve(bytes)?` -> LRU eviction | Guarantees resident + in-flight pages never exceed the memory cap; coarse root pages stay pinned so the scene is never blank. |
| **GPU Instancing** | Shared definition buffer + per-instance record | Draws N placed copies of a complex CAD component using 1 shared vertex/index allocation and N compact instance records. |

## 3. Existing Chapters & Lessons Absorbed

| Redraft Lesson | Existing Lesson | Title | Checkpoints | Range (`first` → `last`) | Consolidated Lesson Page |
| :--- | :--- | :--- | ---: | :--- | :--- |
| `7.1` | `12.1` | **Complete viewer files and published scene sources** | 44 | `66-read-scene-manifests` → `66af-connect-browser-save-open-url-adoption` | [66-read-scene-manifests.md](../../consolidation/66-read-scene-manifests.md) · [Redraft](../lessons/66-read-scene-manifests.md) |
| `7.2` | `12.2` | **Locate original arrays without downloading their bodies** | 10 | `67-supply-checked-array-layout-readers` → `67si-validate-complete-octrees-before-range-streaming` | [67-supply-checked-array-layout-readers.md](../../consolidation/67-supply-checked-array-layout-readers.md) · [Redraft](../lessons/67-supply-checked-array-layout-readers.md) |
| `7.3` | `12.3` | **Bounded scheduling, exact replies and cancellation** | 17 | `66za-own-original-range-read-permits` → `67m-cancel-range-work-when-its-runtime-closes` | [66za-own-original-range-read-permits.md](../../consolidation/66za-own-original-range-read-permits.md) · [Redraft](../lessons/66za-own-original-range-read-permits.md) |
| `7.4` | `12.4` | **Charged residency and current streamed pages** | 31 | `68-reserve-page-bytes-before-reading` → `68ac-connect-current-pages-to-the-browser-renderer` | [68-reserve-page-bytes-before-reading.md](../../consolidation/68-reserve-page-bytes-before-reading.md) · [Redraft](../lessons/68-reserve-page-bytes-before-reading.md) |
| `7.5` | `13.1` | **Shared original definitions and independently placed instances** | 9 | `69-validate-original-definition-references` → `69i-check-sharing-precision-history-and-release` | [69-validate-original-definition-references.md](../../consolidation/69-validate-original-definition-references.md) · [Redraft](../lessons/69-validate-original-definition-references.md) |
| `7.6` | `13.2` | **Independent presentation and reusable CAD preparation** | 15 | `69j-preserve-per-instance-original-colours` → `69af-exercise-independent-original-cad-reuse` | [69j-preserve-per-instance-original-colours.md](../../consolidation/69j-preserve-per-instance-original-colours.md) · [Redraft](../lessons/69j-preserve-per-instance-original-colours.md) |
| `7.7` | `13.3` | **Feature ancestry and canonical local-to-world placement** | 5 | `70-evaluate-source-ancestry-once-per-document` → `70d-exercise-rotated-and-mirrored-original-references` | [70-evaluate-source-ancestry-once-per-document.md](../../consolidation/70-evaluate-source-ancestry-once-per-document.md) · [Redraft](../lessons/70-evaluate-source-ancestry-once-per-document.md) |
| `7.8` | `13.4` | **Original sheet entity metadata** | 1 | `70za-read-original-sheet-metadata-records` → `70za-read-original-sheet-metadata-records` | [70za-read-original-sheet-metadata-records.md](../../consolidation/70za-read-original-sheet-metadata-records.md) · [Redraft](../lessons/70za-read-original-sheet-metadata-records.md) |

## 4. Rendering, Geometry, Interaction, and Runtime Features Covered

### Rendering & Shaders
- Out-of-core paged GPU buffers: pinning coarse octree root pages while streaming visible child pages into GPU storage
- Shared CAD definition instancing: uploading each definition mesh/B-Rep once and drawing multiple world-placed instances with per-instance colors and integer pick IDs
- Rigid-frame CAD preparation caching (replaying sampled tessellation into new rigid frames without re-evaluating identical NURBS geometry)

### Geometry & Data Structures
- Zero-body Protobuf wire-format scanning: locating array byte offsets and lengths (`67-supply-checked-array-layout-readers`) before fetching payload bytes
- Hierarchical local-to-world matrix ancestry (`Parent * Local`) preserved without flattening on save/reload
- Architectural drawing sheet records (`SheetInit`, `SheetFields`, `SheetRows`) and entity metadata tables

### User Interaction & Selection
- Live URL scene watching, range-streamed point picking (mapping compact resident page offsets back to original manifest IDs), and independent instance selection/part-edit protection

### Browser Runtime & Production Authority (`session_viewer/src`)
- Bounded HTTP `Range` request queue, permit semaphore, chunked body limits, stall timeouts, and LRU page eviction (`src/app/range_gate.rs`, `src/app/live.rs`)
- Production authority: `src/app/manifest.rs`, `src/app/range_gate.rs`, `src/app/live.rs`, `src/app/live_revision.rs`, `src/app/scene_instances.rs`, `src/app/sheet_query.rs`

## 5. Practical Exercises & Expected Visible Results

| Checkpoint(s) | Practical Student Exercise | Expected Visible Result |
| :--- | :--- | :--- |
| `67si / 68ac` | Open a ranged octree scene under a tight page budget, zoom into a detailed corner, then pan to another region and close the document mid-stream. | The coarse root stays visible immediately; fine pages stream in for the zoomed corner; panning evicts cold unpinned pages in LRU order; closing cancels in-flight range reads. |
| `69i / 69af / 70d` | Place three instances of a shared CAD definition with different colors and a mirrored/rotated parent frame, then attempt a part edit. | All three instances share one geometry preparation while picking and coloring independently; part edits protect shared definitions and save/reload preserves nested frames without flattening. |

## 6. Verified Work vs. Remaining Implementation or Validation

### Verified in Repository Evidence
- [x] All 8 consolidated lessons (`12.1`–`12.4`, `13.1`–`13.4`) and 132 checkpoints (`66-read-scene-manifests`..`70za`) are integrated and pass `checkpoints.test.ts` and the strict Vite docs build.
- [x] Regenerated `course-instance-features.pb` and `course-metadata.pb` deterministic fixtures compiled and verified across two processes; native metadata-filter checks pass 7/7 (`current-state.json` lines 25, 28).

### Remaining Implementation or Validation
- [ ] Browser acceptance of regenerated `course-instance-features.pb` and `course-metadata.pb` fixtures remains open.
- [ ] Chapter-specific accepted visual review images for Chapters 12 and 13 remain open.
