# Section 3: Scene Documents, Transactions, and Browser GPU Lifetime

**Absorbs:** Ch 3 (*Documents, editing and ownership*), Ch 4 (*Browser lifetime and recovery*) · **Lessons:** 12 · **Checkpoints:** 136 (`17-solid` → `34geb-reload`)

<div class="webgpu-schematic" style="margin:14px 0;">
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="220" viewBox="0 0 840 220" style="width:100%;max-width:840px;height:auto;display:block;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="216" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="20" y="20" width="230" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="135" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">1. Near Clip (w &gt; 0)</text>
  <line x1="115" y1="58" x2="115" y2="162" stroke="#111111" stroke-width="2"/>
  <text x="115" y="176" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">w = near</text>
  <line x1="55" y1="142" x2="115" y2="110" stroke="#111111" stroke-width="1.75" stroke-dasharray="4 4"/>
  <line x1="115" y1="110" x2="210" y2="68" stroke="#111111" stroke-width="3"/>
  <circle cx="55" cy="142" r="3.5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <circle cx="115" cy="110" r="4.5" fill="#111111"/>
  <circle cx="210" cy="68" r="4" fill="#111111"/>
  <text x="68" y="156" font-family="monospace" font-size="9" fill="#111111">Clipped</text>
  <text x="180" y="95" font-family="monospace" font-size="9" font-weight="700" fill="#111111">Kept</text>
  <line x1="250" y1="110" x2="298" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="304" y="20" width="230" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="419" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">2. Ribbon Quad</text>
  <polygon points="338,120 480,68 498,108 356,160" fill="url(#hatch)" stroke="#111111" stroke-width="1.75"/>
  <line x1="347" y1="140" x2="489" y2="88" stroke="#111111" stroke-width="2" stroke-dasharray="4 3"/>
  <circle cx="347" cy="140" r="4" fill="#111111"/>
  <circle cx="489" cy="88" r="4" fill="#111111"/>
  <line x1="418" y1="114" x2="407" y2="88" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <text x="385" y="84" font-family="monospace" font-size="10" font-weight="700" fill="#111111">+n</text>
  <text x="419" y="186" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">6-Vert Extrusion</text>
  <line x1="534" y1="110" x2="582" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="588" y="20" width="232" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="704" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">3. SDF Anti-Alias</text>
  <line x1="620" y1="148" x2="788" y2="148" stroke="#111111" stroke-width="1.5"/>
  <path d="M 628 148 L 656 76 L 752 76 L 780 148" fill="url(#hatch)" stroke="#111111" stroke-width="2"/>
  <text x="704" y="68" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">alpha = 1.0</text>
  <text x="704" y="186" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">smoothstep() Edge</text>
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
