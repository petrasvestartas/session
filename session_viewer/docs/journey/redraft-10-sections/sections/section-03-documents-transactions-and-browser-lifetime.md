# Section 3: Scene Documents, Transactions, and Browser GPU Lifetime

**Absorbs:** Ch 3 (*Documents, editing and ownership*), Ch 4 (*Browser lifetime and recovery*) · **Lessons:** 12 · **Checkpoints:** 136 (`17-solid` → `34geb-reload`)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Kernel Box + Face Normals N</text>
  <polygon points="186,52 256,82 186,112 116,82" fill="#f8fafc" stroke="#111111" stroke-width="1.75"/>
  <polygon points="116,82 186,112 186,160 116,130" fill="#e2e8f0" stroke="#111111" stroke-width="1.75"/>
  <polygon points="186,112 256,82 256,130 186,160" fill="#cbd5e1" stroke="#111111" stroke-width="1.75"/>
  <line x1="186" y1="82" x2="186" y2="38" stroke="#2563eb" stroke-width="2" marker-end="url(#arrB)"/>
  <line x1="222" y1="120" x2="268" y2="140" stroke="#dc2626" stroke-width="2" marker-end="url(#arrR)"/>
  <line x1="150" y1="120" x2="104" y2="140" stroke="#16a34a" stroke-width="2" marker-end="url(#arrG)"/>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">@fragment: max(dot(N, L), 0.0)</text>
  <line x1="470" y1="134" x2="650" y2="134" stroke="#111111" stroke-width="2"/>
  <line x1="560" y1="134" x2="560" y2="54" stroke="#2563eb" stroke-width="2.25" marker-end="url(#arrB)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="546" y="60" font-family="monospace" font-size="10" font-weight="700" fill="#2563eb">N</text>
  <line x1="560" y1="134" x2="628" y2="66" stroke="#f59e0b" stroke-width="2.25" marker-end="url(#arr)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="640" y="68" font-family="monospace" font-size="10" font-weight="700" fill="#d97706">L</text>
  <path d="M 560 98 A 36 36 0 0 1 585 109" fill="none" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="582" y="94" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">θ</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="156" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">lit = base * (amb + diff * cos θ)</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Shaded 3D Solid in Viewer</text>
  <rect x="814" y="44" width="240" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="934,54 1004,82 934,110 864,82" fill="#93c5fd" stroke="#111111" stroke-width="1.75"/>
  <polygon points="864,82 934,110 934,154 864,126" fill="#60a5fa" stroke="#111111" stroke-width="1.75"/>
  <polygon points="934,110 1004,82 1004,126 934,154" fill="#2563eb" stroke="#111111" stroke-width="1.75"/>
</svg>
</div>

> **Learning Outcome:** Integrate the `session_rust` geometry kernel, manage a single `Editor` owner with high-DPI viewport resizing and pointer capture, decode/save `.pb` Protobuf scene files asynchronously with transactional rollback and source-ownership guards, and harden the browser runtime with `Rc`/`Drop` listener cleanup, structured GPU fault diagnostics, `pagehide`/heartbeat suspension, and conservative WebGPU device recovery.

## 1. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

| Layer | Construct | Role |
| :--- | :--- | :--- |
| **Rust (Smart Ptrs)** | `Rc<RefCell<Runtime>>` + `Closure::wrap` | Shares mutable application state safely with JS event callbacks in single-threaded WASM and drops closures cleanly on shutdown. |
| **Protobuf / Rust** | `Message::decode(bytes)?` | Deserializes binary `session_proto` records into validated `session_rust` geometry structs. |
| **Rust (Generations)** | `Ticket { id: u64, revision: u64 }` | Ensures an older slow async file/network fetch can never overwrite a newer scene state. |
| **WebGPU (Faults)** | `device.on_uncaptured_error(...)` | Captures validation/out-of-memory faults into a bounded schema-validated diagnostic report and triggers safe reload. |

## 2. Lessons in This Section

| Redraft | Existing | Lesson Title | Checkpoints | Range | Open Lesson |
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

## 3. Core Features & Exercises

| Checkpoint | Exercise | Expected Result |
| :--- | :--- | :--- |
| `22-shortcuts / 26a-keys` | Import `sample-23.pb`, switch between `Top`, `Front`, and `Perspective` views, select one object, and run `Fit Selected`. | The camera frames only the selected object’s world bounding box while keeping the object above the Command Dock. |
| `30e-recovery / 32gja-failures` | Release cold CPU source geometry, trigger a move edit that requires the original source, and test importing a truncated `.pb` file. | The truncated `.pb` is rejected atomically without clearing the current scene; the edit fetches the exact version-matched source and replays the move. |
| `33b-cache / 34geb-reload` | Dispatch `pagehide` (`persisted: true`) and `pageshow` events in the browser console, then trigger a simulated GPU fault. | The runtime suspends timers on `pagehide`, resumes on `pageshow`, records a bounded diagnostic JSON report on GPU fault, and recovers the scene cleanly. |
