# Section 7: Out-of-Core HTTP Range Streaming, Shared Instances, and Drawing Sheets

**Absorbs:** Ch 12 (*Files and streaming*), Ch 13 (*Shared definitions and drawing sheets*) · **Lessons:** 8 · **Checkpoints:** 132 (`66-read-scene-manifests` → `70za-read-original-sheet-metadata-records`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="200" viewBox="0 0 1120 200" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 7.1 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/cplane.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">ch7-start -&gt; ch7-start</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/cplane.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/gizmo.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/>
  <text x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Rust CPU Input</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. C-Plane Snap &amp; Preview</text>
  <polygon points="602,128 652,72 798,72 748,128" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <line x1="628" y1="50" x2="696" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <circle cx="700" cy="102" r="5" fill="#f59e0b" stroke="#111111" stroke-width="1.5"/>
  <circle cx="700" cy="102" r="10" fill="none" stroke="#d97706" stroke-width="2"/>
  <text x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Chapter 7 Architecture</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. RGB 3D Gumball Gizmo</text>
  <circle cx="956" cy="118" r="5" fill="#facc15" stroke="#111111" stroke-width="1.5"/>
  <line x1="956" y1="118" x2="956" y2="52" stroke="#2563eb" stroke-width="2.5" marker-end="url(#arrB)"/>
  <text x="940" y="60" font-family="monospace" font-size="10.5" font-weight="700" fill="#2563eb">Z</text>
  <line x1="956" y1="118" x2="1036" y2="118" stroke="#dc2626" stroke-width="2.5" marker-end="url(#arrR)"/>
  <text x="1044" y="122" font-family="monospace" font-size="10.5" font-weight="700" fill="#dc2626">X</text>
  <line x1="956" y1="118" x2="904" y2="144" stroke="#16a34a" stroke-width="2.5" marker-end="url(#arrG)"/>
  <text x="888" y="146" font-family="monospace" font-size="10.5" font-weight="700" fill="#16a34a">Y</text>
  <path d="M 956 80 A 38 38 0 0 1 994 118" fill="none" stroke="#dc2626" stroke-width="2"/>
  <text x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Screen Output</text>
</svg>
</div>

> **Learning Outcome:** Stream gigabyte-scale scene manifests and octree point clouds over HTTP `206 Partial Content` range requests without downloading full file bodies, enforce charged LRU memory residency budgets with coarse root pinning and generation-safe cancellation, render thousands of independently placed/colored CAD instances from a single shared definition (`69`), and stream/query 2D/3D architectural drawing sheets (`70za`).

## 1. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

| Layer | Construct | Role |
| :--- | :--- | :--- |
| **HTTP / Fetch** | `Range: bytes=start-end` -> `206 Partial Content` | Fetches only the exact byte slice of an octree node or sheet column needed for the current camera view. |
| **Protobuf Wire** | Varint tag + length prefix scanner | Skips over multi-megabyte coordinate arrays by reading their byte range offsets without allocating their contents. |
| **Rust (Residency)** | `budget.reserve(bytes)?` -> LRU eviction | Guarantees resident + in-flight pages never exceed the memory cap; coarse root pages stay pinned so the scene is never blank. |
| **GPU Instancing** | Shared definition buffer + per-instance record | Draws N placed copies of a complex CAD component using 1 shared vertex/index allocation and N compact instance records. |

## 2. Lessons in This Section

| Redraft | Existing | Lesson Title | Checkpoints | Range | Open Lesson |
| :--- | :--- | :--- | ---: | :--- | :--- |
| `7.1` | `12.1` | **Complete viewer files and published scene sources** | 44 | `66-read-scene-manifests` → `66af-connect-browser-save-open-url-adoption` | [66-read-scene-manifests.md](../../consolidation/66-read-scene-manifests.md) · [Redraft](../lessons/66-read-scene-manifests.md) |
| `7.2` | `12.2` | **Locate original arrays without downloading their bodies** | 10 | `67-supply-checked-array-layout-readers` → `67si-validate-complete-octrees-before-range-streaming` | [67-supply-checked-array-layout-readers.md](../../consolidation/67-supply-checked-array-layout-readers.md) · [Redraft](../lessons/67-supply-checked-array-layout-readers.md) |
| `7.3` | `12.3` | **Bounded scheduling, exact replies and cancellation** | 17 | `66za-own-original-range-read-permits` → `67m-cancel-range-work-when-its-runtime-closes` | [66za-own-original-range-read-permits.md](../../consolidation/66za-own-original-range-read-permits.md) · [Redraft](../lessons/66za-own-original-range-read-permits.md) |
| `7.4` | `12.4` | **Charged residency and current streamed pages** | 31 | `68-reserve-page-bytes-before-reading` → `68ac-connect-current-pages-to-the-browser-renderer` | [68-reserve-page-bytes-before-reading.md](../../consolidation/68-reserve-page-bytes-before-reading.md) · [Redraft](../lessons/68-reserve-page-bytes-before-reading.md) |
| `7.5` | `13.1` | **Shared original definitions and independently placed instances** | 9 | `69-validate-original-definition-references` → `69i-check-sharing-precision-history-and-release` | [69-validate-original-definition-references.md](../../consolidation/69-validate-original-definition-references.md) · [Redraft](../lessons/69-validate-original-definition-references.md) |
| `7.6` | `13.2` | **Independent presentation and reusable CAD preparation** | 15 | `69j-preserve-per-instance-original-colours` → `69af-exercise-independent-original-cad-reuse` | [69j-preserve-per-instance-original-colours.md](../../consolidation/69j-preserve-per-instance-original-colours.md) · [Redraft](../lessons/69j-preserve-per-instance-original-colours.md) |
| `7.7` | `13.3` | **Feature ancestry and canonical local-to-world placement** | 5 | `70-evaluate-source-ancestry-once-per-document` → `70d-exercise-rotated-and-mirrored-original-references` | [70-evaluate-source-ancestry-once-per-document.md](../../consolidation/70-evaluate-source-ancestry-once-per-document.md) · [Redraft](../lessons/70-evaluate-source-ancestry-once-per-document.md) |
| `7.8` | `13.4` | **Original sheet entity metadata** | 1 | `70za-read-original-sheet-metadata-records` → `70za-read-original-sheet-metadata-records` | [70za-read-original-sheet-metadata-records.md](../../consolidation/70za-read-original-sheet-metadata-records.md) · [Redraft](../lessons/70za-read-original-sheet-metadata-records.md) |

## 3. Core Features & Exercises

| Checkpoint | Exercise | Expected Result |
| :--- | :--- | :--- |
| `67si / 68ac` | Open a ranged octree scene under a tight page budget, zoom into a detailed corner, then pan to another region and close the document mid-stream. | The coarse root stays visible immediately; fine pages stream in for the zoomed corner; panning evicts cold unpinned pages in LRU order; closing cancels in-flight range reads. |
| `69i / 69af / 70d` | Place three instances of a shared CAD definition with different colors and a mirrored/rotated parent frame, then attempt a part edit. | All three instances share one geometry preparation while picking and coloring independently; part edits protect shared definitions and save/reload preserves nested frames without flattening. |
