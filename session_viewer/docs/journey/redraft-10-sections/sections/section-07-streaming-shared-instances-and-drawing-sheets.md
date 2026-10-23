# Section 7: Out-of-Core HTTP Range Streaming, Shared Instances, and Drawing Sheets

> **Gemini Notebook Student Guide** · **Absorbs:** Chapter 12 (*Files and streaming*), Chapter 13 (*Shared definitions and drawing sheets*) · **Lessons:** 8 (`12.1`–`13.4`) · **Checkpoints:** 132 (`66-read-scene-manifests` → `70za-read-original-sheet-metadata-records`)

## 1. Learning Outcome

Stream gigabyte-scale scene manifests and octree point clouds over HTTP `206 Partial Content` range requests without downloading full file bodies, enforce charged LRU memory residency budgets with coarse root pinning and generation-safe cancellation, render thousands of independently placed/colored CAD instances from a single shared definition (`69`), and stream/query 2D/3D architectural drawing sheets (`70za`).

## 2. WebGPU & Architecture Schematic (First-Year Student Guide)

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
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Section 7 Visual Explanation: HTTP 206 Range Streaming, GPU Hardware Instancing & 2D Drawing Sheets</text>
  <text x="20" y="45" font-size="11" fill="#111111">Left: Fetching frustum-prioritized byte ranges. Center: 1 mesh buffer × N instance transforms. Right: Paper-space sheets.</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    <rect x="18" y="64" width="245" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="140" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">A. HTTP 206 Byte-Range Stream</text>
    <rect x="34" y="98" width="212" height="30" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
    <rect x="74" y="98" width="60" height="30" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
    <rect x="164" y="98" width="48" height="30" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
    <text x="140" y="144" text-anchor="middle" font-size="9.5" font-weight="700" fill="#111111">Range: bytes=start-end (Visible First)</text>
    <text x="140" y="164" text-anchor="middle" font-size="9.5" fill="#111111">• Bounded Permit Pool &amp; Page Cache</text>
    <text x="140" y="182" text-anchor="middle" font-size="9.5" fill="#111111">• Evicts cold off-screen chunks safely</text>

    <line x1="263" y1="154" x2="285" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="287" y="64" width="275" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="424" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">B. GPU Hardware Instancing (1 → N)</text>
    <rect x="305" y="114" width="68" height="48" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
    <text x="339" y="142" text-anchor="middle" font-size="9.5" font-weight="700" fill="#111111">1 Source</text>
    <line x1="373" y1="126" x2="425" y2="106" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>
    <line x1="373" y1="138" x2="425" y2="138" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>
    <line x1="373" y1="150" x2="425" y2="170" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>
    <rect x="428" y="92" width="118" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
    <text x="487" y="108" text-anchor="middle" font-size="9" fill="#111111">Instance #0 (M0, ID0)</text>
    <rect x="428" y="126" width="118" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
    <text x="487" y="142" text-anchor="middle" font-size="9" fill="#111111">Instance #1 (M1, ID1)</text>
    <rect x="428" y="160" width="118" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
    <text x="487" y="176" text-anchor="middle" font-size="9" fill="#111111">Instance #N (MN, IDN)</text>
    <text x="424" y="212" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">@builtin(instance_index) draw_indexed</text>

    <line x1="562" y1="154" x2="584" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="586" y="64" width="236" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="704" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">C. 2D Drawing Sheet &amp; Title Block</text>
    <rect x="612" y="96" width="184" height="104" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
    <rect x="716" y="168" width="80" height="32" fill="url(#hatch)" stroke="#111111" stroke-width="1.2"/>
    <text x="756" y="188" text-anchor="middle" font-size="8.5" font-weight="700" fill="#111111">Title Block</text>
    <text x="704" y="220" text-anchor="middle" font-size="9.5" fill="#111111">Paper-space mm/inch pen widths</text>
    
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
