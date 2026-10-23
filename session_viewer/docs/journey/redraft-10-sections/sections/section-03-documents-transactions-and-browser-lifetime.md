# Section 3: Scene Documents, Transactions, and Browser GPU Lifetime

> **Gemini Notebook Student Guide** · **Absorbs:** Chapter 3 (*Documents, editing and ownership*), Chapter 4 (*Browser lifetime and recovery*) · **Lessons:** 12 (`3.1`–`4.5`) · **Checkpoints:** 136 (`17-solid` → `34geb-reload`)

## 1. Learning Outcome

Integrate the `session_rust` geometry kernel, manage a single `Editor` owner with high-DPI viewport resizing and pointer capture, decode/save `.pb` Protobuf scene files asynchronously with transactional rollback and source-ownership guards, and harden the browser runtime with `Rc`/`Drop` listener cleanup, structured GPU fault diagnostics, `pagehide`/heartbeat suspension, and conservative WebGPU device recovery.

## 2. WebGPU & Architecture Schematic (First-Year Student Guide)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 240" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="232" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Chapter 3 Architecture: Transactional Protobuf (.pb) Decode, World Placements & Incremental GPU Sync</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">Separating exact session_rust kernel geometry from GPU display buffers so view changes never re-upload unchanged meshes.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    <rect x="18" y="68" width="185" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="110" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">1. Async .pb File / URL I/O</text>
    <text x="28" y="108" font-size="10.5" fill="#111111">• Ticket #N (cancels #N-1)</text>
    <text x="28" y="126" font-size="10.5" fill="#111111">• Bounded Protobuf decode</text>
    <text x="28" y="144" font-size="10.5" font-weight="600" fill="#111111">• Topological validation</text>
    <text x="28" y="162" font-size="10.5" fill="#111111">• Corrupt file? Reject with</text>
    <text x="28" y="177" font-size="10.5" fill="#111111">  ZERO partial scene mutation</text>

    <line x1="203" y1="138" x2="233" y2="138" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="235" y="68" width="275" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="372" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">2. Editor Owner (Browser-Independent)</text>
    <text x="247" y="108" font-size="10.5" fill="#111111">• Kernel Mesh (local-space f64 coordinates)</text>
    <text x="247" y="126" font-size="10.5" fill="#111111">• Placement 4x4 Matrix (world transform)</text>
    <text x="247" y="144" font-size="10.5" font-weight="600" fill="#111111">• Releasable Source Ownership:</text>
    <text x="247" y="160" font-size="10" fill="#111111">  Drop cold CPU source after GPU upload;</text>
    <text x="247" y="175" font-size="10" fill="#111111">  re-fetch exact SHA/revision on edit!</text>
    <text x="247" y="195" font-size="10.5" fill="#111111">• Emits Change { sync_scene, redraw }</text>

    <line x1="510" y1="115" x2="542" y2="115" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>
    <line x1="510" y1="175" x2="542" y2="175" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="544" y="68" width="276" height="68" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="556" y="86" font-size="11" font-weight="700" fill="#111111">3A. Geometry Changed (sync_scene = true)</text>
    <text x="556" y="104" font-size="10.5" fill="#111111">• Upload changed mesh vertex/index buffers</text>
    <text x="556" y="120" font-size="10.5" fill="#111111">• Retain unchanged object GPU allocations</text>

    <rect x="544" y="145" width="276" height="68" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="556" y="163" font-size="11" font-weight="700" fill="#111111">3B. View / Camera Only (sync_scene = false)</text>
    <text x="556" y="181" font-size="10.5" fill="#111111">• queue.write_buffer(camera_uniform) ONLY</text>
    <text x="556" y="197" font-size="10.5" fill="#111111">• Zero geometry re-uploads during Orbit/Fit!</text>
    
</svg>
</div>

```
+---------------------------------------------------------------------------------------------------+
|                   ASYNC TRANSACTION & GPU LIFETIME ARCHITECTURE (CHAPTERS 3 & 4)                  |
+---------------------------------------------------------------------------------------------------+
  [File / URL .pb Bytes] 
         |  (Ticket #N created; cancels Ticket #N-1)
         v
  +------------------------------+      Rejects invalid Protobuf     +------------------------------+
  | Bounded Decode & Validation  | --------------------------------> | Keep Existing Document Intact|
  +------------------------------+      (Zero partial mutation)      +------------------------------+
         | Validated Scene
         v
  +------------------------------+      Release cold CPU bytes       +------------------------------+
  | Editor Owner (Document +     | <===============================> | Shared GPU Geometry Cache    |
  | Undo History + Placements)   |      Re-fetch exact revision      | (Retained Vertex/Index Buf)  |
  +------------------------------+      by SHA/Version on edit       +------------------------------+
         |
         v
  +-------------------------------------------------------------------------------------------------+
  | Browser Runtime Guard: Active Listeners (Drop-safe) | Heartbeat Suspend | GPU Fault -> Recover  |
  +-------------------------------------------------------------------------------------------------+
```

### Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `Browser`)

| Layer | Construct | What It Does in Plain English |
| :--- | :--- | :--- |
| **Rust (Smart Ptrs)** | `Rc<RefCell<Runtime>>` + `Closure::wrap` | Shares mutable application state safely with JS event callbacks in single-threaded WASM and drops closures cleanly on shutdown. |
| **Protobuf / Rust** | `Message::decode(bytes)?` | Deserializes binary `session_proto` records into validated `session_rust` geometry structs. |
| **Rust (Generations)** | `Ticket { id: u64, revision: u64 }` | Ensures an older slow async file/network fetch can never overwrite a newer scene state. |
| **WebGPU (Faults)** | `device.on_uncaptured_error(...)` | Captures validation/out-of-memory faults into a bounded schema-validated diagnostic report and triggers safe reload. |

## 3. Existing Chapters & Lessons Absorbed

| Redraft Lesson | Existing Lesson | Title | Checkpoints | Range (`first` → `last`) | Consolidated Lesson Page |
| :--- | :--- | :--- | ---: | :--- | :--- |
| `3.1` | `3.1` | **Kernel geometry and a single editor owner** | 5 | `17-solid` → `19-actions` | [17-solid.md](../../consolidation/17-solid.md) · [Redraft](../lessons/17-solid.md) |
| `3.2` | `3.2` | **One drawing size and one captured gesture** | 8 | `19a-viewport` → `22-shortcuts` | [19a-viewport.md](../../consolidation/19a-viewport.md) · [Redraft](../lessons/19a-viewport.md) |
| `3.3` | `3.3` | **Validated files, bounds and useful projections** | 14 | `22a-records` → `26a-keys` | [22a-records.md](../../consolidation/22a-records.md) · [Redraft](../lessons/22a-records.md) |
| `3.4` | `3.4` | **Local sources, world placement and display caches** | 9 | `27-placement` → `28c-ownership` | [27-placement.md](../../consolidation/27-placement.md) · [Redraft](../lessons/27-placement.md) |
| `3.5` | `3.5` | **Save, replace and asynchronous transaction results** | 11 | `29-identity` → `30e-recovery` | [29-identity.md](../../consolidation/29-identity.md) · [Redraft](../lessons/29-identity.md) |
| `3.6` | `3.6` | **Shared GPU geometry and releasable source ownership** | 23 | `31-settings` → `32fl-guards` | [31-settings.md](../../consolidation/31-settings.md) · [Redraft](../lessons/31-settings.md) |
| `3.7` | `3.7` | **Restore exact sources and replay the captured edit** | 26 | `32g-keys` → `32gja-failures` | [32g-keys.md](../../consolidation/32g-keys.md) · [Redraft](../lessons/32g-keys.md) |
| `3.8` | `4.1` | **Browser callbacks with a real lifetime** | 3 | `33-owner` → `33b-cache` | [33-owner.md](../../consolidation/33-owner.md) · [Redraft](../lessons/33-owner.md) |
| `3.9` | `4.2` | **First failure and independent diagnostic reports** | 6 | `34-fault` → `34d-download` | [34-fault.md](../../consolidation/34-fault.md) · [Redraft](../lessons/34-fault.md) |
| `3.10` | `4.3` | **Admit useful evidence and bound stored runs** | 8 | `34e-schema` → `34fc-retention` | [34e-schema.md](../../consolidation/34e-schema.md) · [Redraft](../lessons/34e-schema.md) |
| `3.11` | `4.4` | **Heartbeat suspension and revocable startup** | 8 | `34g-heartbeat` → `34gbca-startup` | [34g-heartbeat.md](../../consolidation/34g-heartbeat.md) · [Redraft](../lessons/34g-heartbeat.md) |
| `3.12` | `4.5` | **Measured phases and conservative GPU recovery** | 15 | `34gc-messages` → `34geb-reload` | [34gc-messages.md](../../consolidation/34gc-messages.md) · [Redraft](../lessons/34gc-messages.md) |

## 4. Rendering, Geometry, Interaction, and Runtime Features Covered

### Rendering & Shaders
- Surface directional/headlight Lambertian lighting (`dot(normal, light_dir)`)
- Device-pixel-ratio (`DPR`) physical surface reconfiguration on window resize
- Incremental GPU setting updates vs geometry cache reuse (updating view uniforms without re-uploading unchanging mesh buffers)

### Geometry & Data Structures
- Kernel `session_rust` solids (`Mesh`), axis-aligned scene bounding boxes (`Bounds`), and camera `Fit` / named projections (`Top`, `Front`, `Right`, `Perspective`, `Orthographic`)
- Local-space source geometry vs 4x4 world placement matrices (`Model` transform)
- Protobuf binary scene encoding/decoding (`session_proto`) with schema validation before allocation

### User Interaction & Selection
- Pointer capture (`setPointerCapture`) so drags stay owned across window boundaries; normalized wheel delta units (line vs pixel scroll)
- Keyboard shortcuts, asynchronous file Open/Save dialogs, and transactional tickets that cancel stale in-flight imports

### Browser Runtime & Production Authority (`session_viewer/src`)
- Releasable CPU/GPU source ownership, fetch-on-demand source restoration, and replay of captured edits after source hydration
- Browser callback lifetime ownership, bounded diagnostic event ring-buffer & JSON report download, `visibilitychange`/`pagehide` heartbeat suspension, and GPU device-lost recovery
- Production authority: `src/state.rs`, `src/app/session_io.rs`, `src/app/decode.rs`, `src/app/fetch.rs`, `src/app/scene_release.rs`, `src/app/loader.rs`, `src/engine/gpu/device.rs`

## 5. Practical Exercises & Expected Visible Results

| Checkpoint(s) | Practical Student Exercise | Expected Visible Result |
| :--- | :--- | :--- |
| `22-shortcuts / 26a-keys` | Import `sample-23.pb`, switch between `Top`, `Front`, and `Perspective` views, select one object, and run `Fit Selected`. | The camera frames only the selected object’s world bounding box while keeping the object above the Command Dock. |
| `30e-recovery / 32gja-failures` | Release cold CPU source geometry, trigger a move edit that requires the original source, and test importing a truncated `.pb` file. | The truncated `.pb` is rejected atomically without clearing the current scene; the edit fetches the exact version-matched source and replays the move. |
| `33b-cache / 34geb-reload` | Dispatch `pagehide` (`persisted: true`) and `pageshow` events in the browser console, then trigger a simulated GPU fault. | The runtime suspends timers on `pagehide`, resumes on `pageshow`, records a bounded diagnostic JSON report on GPU fault, and recovers the scene cleanly. |

## 6. Verified Work vs. Remaining Implementation or Validation

### Verified in Repository Evidence
- [x] All 12 consolidated lessons (`3.1`–`3.7`, `4.1`–`4.5`) and all 136 checkpoints (`17-solid`..`34geb-reload`) are integrated and pass `checkpoints.test.ts` and the strict Vite docs build.
- [x] Current endpoint passes 5 file-read cases, document phases, atomic failed import, and one-reload recovery in browser checks (`current-state.json` line 35).

### Remaining Implementation or Validation
- [ ] Browser lifecycle acceptance and the 5 measured performance categories (cold load, input-to-frame, steady frame time, upload/allocation, idle) remain open in Stage B2.
- [ ] Chapter-specific accepted visual review images for Chapters 3 and 4 remain open.
