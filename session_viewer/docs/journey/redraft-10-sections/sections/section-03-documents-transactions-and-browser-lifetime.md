# Section 3: Scene Documents, Transactions, and Browser GPU Lifetime

**Absorbs:** Ch 3 (*Documents, editing and ownership*), Ch 4 (*Browser lifetime and recovery*) · **Lessons:** 12 · **Checkpoints:** 136 (`17-solid` → `34geb-reload`)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
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
  </defs>
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 3.1 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/decode.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">1 Coding Step</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/decode.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/loader.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Rust CPU Input</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Placed Kernel Mesh (.pb)</text>
  <rect x="598" y="62" width="66" height="56" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="631" y="88" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">.pb</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="631" y="104" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">Decode</text>
  <line x1="664" y1="90" x2="704" y2="90" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <polygon points="716,128 760,54 804,128" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <line x1="716" y1="128" x2="760" y2="100" stroke="#111111" stroke-width="1.25" stroke-dasharray="3 3"/>
  <line x1="804" y1="128" x2="760" y2="100" stroke="#111111" stroke-width="1.25" stroke-dasharray="3 3"/>
  <line x1="760" y1="54" x2="760" y2="100" stroke="#111111" stroke-width="1.25" stroke-dasharray="3 3"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Chapter 3 Architecture</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Incremental GPU Sync</text>
  <rect x="876" y="52" width="196" height="40" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="974" y="70" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">sync_scene = false</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="974" y="84" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Orbit: 64B Camera Only</text>
  <rect x="876" y="102" width="196" height="40" rx="5" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="974" y="120" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">sync_scene = true</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="974" y="134" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Edit: Upload Dirty Buffers</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Screen Output</text>
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
