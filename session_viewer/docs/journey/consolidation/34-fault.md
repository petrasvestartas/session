# 4.2 · First failure and independent diagnostic reports

**Section 3: Scene Documents, Transactions, and Browser GPU Lifetime** · **Lesson 4.2** (Redraft `3.9`) · **6 checkpoints (`34-fault` → `34d-download`)** · **Study time: 6–12 h**

> **What You Will Learn & Build:** Give asynchronous GPU callbacks one shared first-failure value without confusing an older device with its replacement. By the final checkpoint (`34d-download`), you will download a current ready-run report with Report and attempt one independent first-failure download after GPU disposal.

## 1. Visual WebGPU Explanation & Architecture (Black & White Schematics)

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
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Chapter 4 Visual Explanation: Browser Lifetime State Machine, Circular Fault Ring-Buffer & GPU Recovery</text>
  <text x="20" y="45" font-size="11" fill="#111111">Left: Active ↔ Hidden (bfcache) ↔ Device Lost state transitions. Center: Bounded circular fault ring. Right: Rc/Drop listener guard.</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    <!-- Panel A: State Machine Diagram -->
    <rect x="18" y="64" width="280" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="158" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">A. Browser &amp; GPU State Machine</text>
    <rect x="34" y="98" width="110" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="89" y="118" text-anchor="middle" font-size="10" font-weight="700" fill="#111111">Active Frame</text>
    <text x="89" y="132" text-anchor="middle" font-size="9" fill="#111111">Heartbeat ON</text>
    <rect x="174" y="98" width="110" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="229" y="118" text-anchor="middle" font-size="10" font-weight="700" fill="#111111">Tab Hidden</text>
    <text x="229" y="132" text-anchor="middle" font-size="9" fill="#111111">Timers Paused</text>
    <line x1="144" y1="114" x2="172" y2="114" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>
    <line x1="174" y1="128" x2="146" y2="128" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>
    <!-- Bottom recovery state -->
    <rect x="54" y="172" width="210" height="56" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="159" y="191" text-anchor="middle" font-size="10" font-weight="700" fill="#111111">Device Lost → Conservative Rebuild</text>
    <text x="159" y="206" text-anchor="middle" font-size="9.5" fill="#111111">Re-create Adapter/Device at safe 1x MSAA</text>
    <text x="159" y="220" text-anchor="middle" font-size="9" fill="#111111">&amp; restore retained Editor scene</text>
    <line x1="89" y1="142" x2="115" y2="170" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>
    <line x1="195" y1="172" x2="125" y2="144" stroke="#111111" stroke-width="1.5" stroke-dasharray="3,3" marker-end="url(#arrow)"/>

    <line x1="298" y1="154" x2="320" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <!-- Panel B: Circular Fault Ring-Buffer -->
    <rect x="322" y="64" width="240" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="442" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">B. Bounded Fault Ring-Buffer</text>
    <circle cx="442" cy="144" r="46" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
    <circle cx="442" cy="144" r="22" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
    <line x1="442" y1="98" x2="442" y2="122" stroke="#111111" stroke-width="1.2"/>
    <line x1="442" y1="166" x2="442" y2="190" stroke="#111111" stroke-width="1.2"/>
    <line x1="396" y1="144" x2="420" y2="144" stroke="#111111" stroke-width="1.2"/>
    <line x1="464" y1="144" x2="488" y2="144" stroke="#111111" stroke-width="1.2"/>
    <line x1="410" y1="112" x2="426" y2="128" stroke="#111111" stroke-width="1.2"/>
    <line x1="458" y1="160" x2="474" y2="176" stroke="#111111" stroke-width="1.2"/>
    <line x1="474" y1="112" x2="458" y2="128" stroke="#111111" stroke-width="1.2"/>
    <line x1="426" y1="160" x2="410" y2="176" stroke="#111111" stroke-width="1.2"/>
    <text x="442" y="148" text-anchor="middle" font-size="9" font-weight="700" fill="#111111">v1 JSON</text>
    <text x="442" y="210" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">Fixed capacity: overwrites oldest</text>
    <text x="442" y="225" text-anchor="middle" font-size="9" fill="#111111">Uncaptured GPU errors never OOM</text>

    <line x1="562" y1="154" x2="584" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <!-- Panel C: Rc/Drop Event Listener Guard -->
    <rect x="586" y="64" width="236" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="704" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">C. Rc / Drop Listener Table</text>
    <rect x="602" y="98" width="204" height="54" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
    <text x="704" y="116" text-anchor="middle" font-size="10" font-weight="700" fill="#111111">Runtime::new() Registration</text>
    <text x="614" y="133" font-size="9.5" fill="#111111">Stores (EventTarget, type, Closure)</text>
    <text x="614" y="146" font-size="9.5" fill="#111111">Uses Weak::upgrade() in callbacks</text>
    <line x1="704" y1="152" x2="704" y2="170" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>
    <rect x="602" y="172" width="204" height="56" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="704" y="190" text-anchor="middle" font-size="10" font-weight="700" fill="#111111">impl Drop for Runtime</text>
    <text x="614" y="207" font-size="9.5" fill="#111111">Calls remove_event_listener()</text>
    <text x="614" y="220" font-size="9.5" font-weight="600" fill="#111111">Zero leaked WASM closures!</text>
    
</svg>
</div>

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg width="840" height="214" viewBox="0 0 840 214" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
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
  <rect x="4" y="4" width="832" height="206" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Lesson 4.2 Pipeline Schematic: First failure and independent diagnostic reports</text>
  <text x="20" y="45" font-size="11" fill="#111111">Step-by-step data flow you implement in this lesson (6 Checkpoints: 34-fault → 34d-download)</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="89" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="90" x2="151" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="89" y="108" text-anchor="middle" font-size="11" fill="#111111">callback clone</text>
    <line x1="161" y1="108" x2="181" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="183" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="254" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="193" y1="90" x2="316" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="254" y="108" text-anchor="middle" font-size="11" fill="#111111">lock shared failure</text>
    <line x1="326" y1="108" x2="346" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="348" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="358" y1="90" x2="481" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">first reason wins</text>
    <line x1="491" y1="108" x2="511" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="513" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="584" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="523" y1="90" x2="646" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="584" y="108" text-anchor="middle" font-size="11" fill="#111111">later errors cannot</text>
    <text x="584" y="124" text-anchor="middle" font-size="11" fill="#111111">overwrite it</text>
    <line x1="656" y1="108" x2="676" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="678" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="749" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 5</text>
    <line x1="688" y1="90" x2="811" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="749" y="108" text-anchor="middle" font-size="11" fill="#111111">first scene</text>
    <text x="749" y="124" text-anchor="middle" font-size="11" fill="#111111">presented</text>
    
    <rect x="18" y="158" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="177" font-size="11" font-weight="700" fill="#111111">Files Modified in Lesson 4.2:</text>
    <text x="235" y="177" font-size="11" fill="#111111">src/gpu_fault.rs  ·  src/gpu_fault_tests.rs  ·  src/lib.rs  ·  src/browser_runtime.rs  ·  src/browser.rs  ·  src/diagnostic.rs</text>
    <text x="32" y="194" font-size="10.5" fill="#111111">6 Checkpoints: 34-fault → 34d-download — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
</svg>
</div>

## 2. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

Before writing code, review how the three layers in this chapter talk to each other:

| Layer | Construct | Plain-English Explanation for Students |
| :--- | :--- | :--- |
| **Rust / WASM (`wasm_bindgen`)** | `Rc<RefCell<Runtime>>` + `Drop` Listener Table | Explicitly unregisters DOM `EventListener` closures on teardown so re-mounting or disposing the viewer never leaks memory or panics on late events. |
| **Browser DOM** | `visibilitychange` / `pagehide` / `pageshow` | Pauses frame timers and GPU work when the tab hides or enters `bfcache`, and resumes cleanly on return. |
| **WebGPU (`wgpu`)** | Uncaptured Fault Ring-Buffer & Device Recovery | Captures validation/out-of-memory faults into a versioned JSON ring-buffer and rebuilds GPU pipelines at conservative `1x` settings after device loss. |

**Authoritative production source (`session_viewer/src`):** `session_viewer/src`

## 3. Checkpoint Roadmap (Jump to Any Notebook Step)

| Step | Checkpoint | What You Build | Files Modified | Pipeline Trace |
| ---: | :--- | :--- | :--- | :--- |
| **1** | [`34-fault`](#checkpoint-34-fault) | Give asynchronous GPU callbacks one shared first-failure value without confusing an older device with its replacement. | `src/gpu_fault.rs`, `src/gpu_fault_tests.rs`, `src/lib.rs` | `callback clone → lock shared failure → first reason wins → later errors cannot overwrite it` |
| **2** | [`34a-stop`](#checkpoint-34a-stop) | Connect actual GPU failure callbacks, stop new UI and GPU work, and dispose only the runtime that belongs to the failed device. | `src/browser_runtime.rs`, `src/browser.rs` | `GPU callback → first fault recorded → input guard → deferred matching-owner disposal → visible failure` |
| **3** | [`34b-report`](#checkpoint-34b-report) | Define owned diagnostic context and a serializable run outcome, without retaining scene or GPU owners. | `src/diagnostic.rs`, `src/diagnostic_tests.rs`, `src/lib.rs` | `browser metadata → owned Context → Report header → explicit JSON → independent snapshot` |
| **4** | [`34ba-events`](#checkpoint-34ba-events) | Bound diagnostic observations while keeping the original failure independently of recent event rotation. | `src/diagnostic.rs`, `src/diagnostic_tests.rs` | `finite relative time → bounded event → first failure retained → last 24 observations → stable failed outcome` |
| **5** | [`34c-browser`](#checkpoint-34c-browser) | Start a bounded page report and read real browser context independently of the renderer’s lifetime. | `src/browser_report.rs`, `Cargo.toml`, `src/lib.rs`, `src/browser.rs` | `page begins → sanitized context → independent report slot → current context → JSON snapshot` |
| **6** | [`34d-download`](#checkpoint-34d-download) | Download a current ready-run report with Report and attempt one independent first-failure download after GPU disposal. | `src/file_output.rs`, `src/browser_report.rs`, `src/browser.rs`, `src/lib.rs` | `first scene presented → ready report → typed Report → JSON Blob → actual download → delayed URL cleanup` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Study the **Visual Step Trace**, read the **Concept Walkthrough** and **Self-Check Q&A**, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-34-fault"></a>

### Step 1 of 6 · Keep the first GPU failure with its device (`34-fault`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 1/6 Visual Trace (34-fault): Keep the first GPU failure with its device</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/gpu_fault.rs, src/gpu_fault_tests.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">callback clone</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">lock shared failure</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">first reason wins</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">later errors cannot</text>
    <text x="726" y="81" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">overwrite…</text>
    
</svg>
</div>

- **Goal:** Give asynchronous GPU callbacks one shared first-failure value without confusing an older device with its replacement.
- **Data Flow Trace:** `callback clone → lock shared failure → first reason wins → later errors cannot overwrite it`
- **Files You Touch:** `src/gpu_fault.rs` (*Create the shared first-failure value; callback clones identify the same device lifetime.*) · `src/gpu_fault_tests.rs` (*Optional authoring fixture. Verify first-reason retention, replacement-device isolation and concurrent callback delivery.*) · `src/lib.rs` (*Expose the signal and include its native tests.*)

Keep the first GPU error: later errors caused by the same loss must not overwrite its reason. This lesson adds the shared value; the next connects it to GPU callbacks.

`Fault` owns an `Arc<Mutex<Option<String>>>`. `Arc::clone` shares one allocation. Locking the mutex makes checking and storing the first message one operation. `remember` returns true only to the first writer; `message` returns a copy so no lock stays held while reporting the error.

Give a replacement device a new `Fault::default()`. `Arc::ptr_eq` compares allocations, so a late callback from the old device cannot identify the replacement as its owner. No mutex guard crosses an `.await`.

> **Self-Check Question:** *Why must a late error from an old GPU device remain separate from a new device’s failure state?*  
> **Answer:** Each new device gets a fresh Arc allocation. Clones share that allocation; same_device compares allocation identity so an old callback cannot be mistaken for the replacement.

- **What You Should See:** Run the first-failure checks below. Two callback writers must share one original reason; a replacement device gets a separate Fault. Browser GPU callbacks are connected next.
- **Hands-On Lab Experiment:** Change the first.is_some guard so every callback overwrites the stored reason. Inspect the tests and explain which useful evidence is lost. Then make a replacement by cloning the old Fault and explain why its identity test fails.

<details>
<summary>Implementation 34-fault: Keep the first GPU failure with its device</summary>

Give asynchronous GPU callbacks one shared first-failure value without confusing an older device with its replacement.

Data flow: callback clone → lock shared failure → first reason wins → later errors cannot overwrite it.

### 1. `src/gpu_fault.rs`

Create the shared first-failure value; callback clones identify the same device lifetime.

Create the file and type:

```rust
--8<-- "journey/code/34-fault-01.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/gpu_fault_tests.rs`

Optional authoring fixture. Verify first-reason retention, replacement-device isolation and concurrent callback delivery.

Copy this supplied file:

```text
--8<-- "journey/code/34-fault-02.rs"
```

</details>

### 3. `src/lib.rs`

Expose the signal and include its native tests.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod background;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod gpu_fault;
#[cfg(test)]
mod gpu_fault_tests;
pub mod background;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/34-fault-03.rs"
```

</details>

</details>

<a id="checkpoint-34a-stop"></a>

### Step 2 of 6 · Stop the viewer when its GPU device fails (`34a-stop`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 2/6 Visual Trace (34a-stop): Stop the viewer when its GPU device fails</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/browser_runtime.rs, src/browser.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="91" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">GPU callback</text>
    <line x1="164" y1="68" x2="180" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="182" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="255" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">first fault recorded</text>
    <line x1="328" y1="68" x2="344" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="346" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">input guard</text>
    <line x1="492" y1="68" x2="508" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="510" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="583" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">deferred</text>
    <text x="583" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">matching-owner…</text>
    <line x1="656" y1="68" x2="672" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="674" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="747" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">visible failure</text>
    
</svg>
</div>

- **Goal:** Connect actual GPU failure callbacks, stop new UI and GPU work, and dispose only the runtime that belongs to the failed device.
- **Data Flow Trace:** `GPU callback → first fault recorded → input guard → deferred matching-owner disposal → visible failure`
- **Files You Touch:** `src/browser_runtime.rs` (*Retain the device failure identity beside the callbacks that use that device.*) · `src/browser.rs` (*Mark failure immediately, then defer identity-checked disposal and visible feedback until callbacks return.*)

Connect both GPU error callbacks to the same `Fault`. When the first error arrives, mark the device failed immediately so the input handler cannot submit another frame.

Schedule disposal on the next microtask: the active callback must return before its Rust closure is released. Compare the captured Fault allocation with the installed runtime before taking that runtime out of its slot. An old device’s callback must leave a replacement alone.

Drop then cancels pending requests and removes input listeners. The page shows “Cannot draw”. Reloading starts a new viewer; automatic recovery comes later.

> **Self-Check Question:** *Why record a failure before deferring cleanup, and why check the device identity again during cleanup?*  
> **Answer:** The immediate signal blocks any intervening input or GPU updates. Deferred cleanup waits for the active callback to return; its identity check prevents an old device’s callback from stopping a replacement.

- **What You Should See:** Inspect the supplied state fixtures. The first GPU failure must remain unchanged when another callback reports a later failure.
- **Hands-On Lab Experiment:** Move the failure guard below panel.update and explain which GPU writes could happen before it. Then remove the identity check in stop_if and explain how an older callback could stop a new runtime.

<details>
<summary>Implementation 34a-stop: Stop the viewer when its GPU device fails</summary>

Connect actual GPU failure callbacks, stop new UI and GPU work, and dispose only the runtime that belongs to the failed device.

Data flow: GPU callback → first fault recorded → input guard → deferred matching-owner disposal → visible failure.

### 1. `src/browser_runtime.rs`

Retain the device failure identity beside the callbacks that use that device. Only the matching device may dispose the active runtime; drop its owner after the slot borrow ends.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{browser_reload::Shared, listeners::Listeners, read_gate::ReadGate};
use std::{cell::RefCell, rc::Rc};

struct Runtime {
    _listeners: Listeners,
    request: Rc<RefCell<ReadGate>>,
    reload: Shared,
}

impl Drop for Runtime {
    fn drop(&mut self) {
        self.request.borrow_mut().cancel();
        self.reload.borrow_mut().cancel();
    }
}

thread_local! { static ACTIVE: RefCell<Option<Runtime>> = const { RefCell::new(None) }; }

pub fn install(listeners: Listeners, request: Rc<RefCell<ReadGate>>, reload: Shared) {
    ACTIVE.with(|slot| slot.replace(Some(Runtime { _listeners: listeners, request, reload })));
}

pub fn stop() -> bool {
    ACTIVE.with(|slot| slot.borrow_mut().take()).is_some()
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn runtime_running() -> bool { ACTIVE.with(|slot| slot.borrow().is_some()) }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{browser_reload::Shared, listeners::Listeners, read_gate::ReadGate};
use std::{cell::RefCell, rc::Rc};

struct Runtime {
    _listeners: Listeners,
    request: Rc<RefCell<ReadGate>>,
    reload: Shared,
    fault: crate::gpu_fault::Fault,
}

impl Drop for Runtime {
    fn drop(&mut self) {
        self.request.borrow_mut().cancel();
        self.reload.borrow_mut().cancel();
    }
}

thread_local! { static ACTIVE: RefCell<Option<Runtime>> = const { RefCell::new(None) }; }

pub fn install(listeners: Listeners, request: Rc<RefCell<ReadGate>>, reload: Shared, fault: crate::gpu_fault::Fault) {
    ACTIVE.with(|slot| slot.replace(Some(Runtime { _listeners: listeners, request, reload, fault })));
}

pub fn stop_if(fault: &crate::gpu_fault::Fault) -> bool {
    let owner = ACTIVE.with(|slot| {
        let mut current = slot.borrow_mut();
        if current.as_ref().is_some_and(|runtime| runtime.fault.same_device(fault)) { current.take() }
        else { None }
    });
    owner.is_some()
}

pub fn stop() -> bool {
    ACTIVE.with(|slot| slot.borrow_mut().take()).is_some()
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn runtime_running() -> bool { ACTIVE.with(|slot| slot.borrow().is_some()) }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/34a-stop-01.rs"
```

```text
--8<-- "journey/code/34a-stop-02.rs"
```

</details>

### 2. `src/browser.rs`

Mark failure immediately, then defer identity-checked disposal and visible feedback until callbacks return. Connect the real error and device-loss callbacks to this device’s shared signal. Refuse new input, UI updates and GPU work as soon as the callback marks a failure. Install the runtime with the same device identity used by its failure callbacks.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    listeners.listen(window.as_ref(), "viewer-file-error")?;
    listeners.listen(window.as_ref(), "viewer-reload")?;
    listeners.listen(window.as_ref(), "pagehide")?;
    crate::browser_runtime::install(listeners, owned_request, owned_reload);
    report("Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.");
    Ok(())
}
```

</details>

Replace it with:

```rust
    listeners.listen(window.as_ref(), "viewer-file-error")?;
    listeners.listen(window.as_ref(), "viewer-reload")?;
    listeners.listen(window.as_ref(), "pagehide")?;
    crate::browser_runtime::install(listeners, owned_request, owned_reload, fault);
    report("Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.");
    Ok(())
}
```

<details>
<summary>Locate this block</summary>

```rust
    let reload_delivery: crate::browser_reload::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let owned_request = std::rc::Rc::clone(&request);
    let owned_reload = std::rc::Rc::clone(&reload);
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        if event.type_() == "pagehide" {
            gesture.cancel();
            if event.dyn_ref::<web_sys::PageTransitionEvent>().is_some_and(|event| event.persisted()) { return; }
```

</details>

Replace it with:

```rust
    let reload_delivery: crate::browser_reload::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let owned_request = std::rc::Rc::clone(&request);
    let owned_reload = std::rc::Rc::clone(&reload);
    let active_fault = fault.clone();
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        if active_fault.message().is_some() { return; }
        if event.type_() == "pagehide" {
            gesture.cancel();
            if event.dyn_ref::<web_sys::PageTransitionEvent>().is_some_and(|event| event.persisted()) { return; }
```

<details>
<summary>Locate this block</summary>

```rust
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    device.on_uncaptured_error(std::sync::Arc::new(|error| report(&error.to_string())));
    let limit = device.limits().max_texture_dimension_2d;
    let width = (window.inner_width()?.as_f64().ok_or("No window width")? as u32).clamp(1, limit);
    let height = (window.inner_height()?.as_f64().ok_or("No window height")? as u32).clamp(1, limit);
```

</details>

Replace it with:

```rust
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let fault = crate::gpu_fault::Fault::default();
    let errors = fault.clone();
    device.on_uncaptured_error(std::sync::Arc::new(move |error| failed(errors.clone(), format!("WebGPU error: {error}"))));
    let lost = fault.clone();
    device.set_device_lost_callback(move |reason, message| failed(lost.clone(), format!("WebGPU device lost ({reason:?}): {message}")));
    let limit = device.limits().max_texture_dimension_2d;
    let width = (window.inner_width()?.as_f64().ok_or("No window width")? as u32).clamp(1, limit);
    let height = (window.inner_height()?.as_f64().ok_or("No window height")? as u32).clamp(1, limit);
```

<details>
<summary>Locate this block</summary>

```rust
    }
}

fn save_result(bytes: &[u8]) -> Result<String, String> {
    crate::file_output::download(bytes)
        .map(|()| format!("Saved editable document ({} bytes)", bytes.len()))
```

</details>

Replace it with:

```rust
    }
}

fn failed(fault: crate::gpu_fault::Fault, message: String) {
    if !fault.remember(message.clone()) { return; }
    wasm_bindgen_futures::spawn_local(async move {
        if crate::browser_runtime::stop_if(&fault) { report(&format!("Cannot draw: {message}")); }
    });
}

fn save_result(bytes: &[u8]) -> Result<String, String> {
    crate::file_output::download(bytes)
        .map(|()| format!("Saved editable document ({} bytes)", bytes.len()))
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::background::Background;
use crate::editor::{Action, Change, Editor};
use crate::gesture::Gesture;
use crate::renderer::Renderer;
use crate::viewport::Viewport;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

pub fn report(message: &str) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        if let Some(status) = document.get_element_by_id("status") {
            status.set_text_content(Some(message));
            if message.starts_with("Cannot draw:") { let _ = status.remove_attribute("hidden"); }
        }
    }
}

fn failed(fault: crate::gpu_fault::Fault, message: String) {
    if !fault.remember(message.clone()) { return; }
    wasm_bindgen_futures::spawn_local(async move {
        if crate::browser_runtime::stop_if(&fault) { report(&format!("Cannot draw: {message}")); }
    });
}

fn save_result(bytes: &[u8]) -> Result<String, String> {
    crate::file_output::download(bytes)
        .map(|()| format!("Saved editable document ({} bytes)", bytes.len()))
        .map_err(|error| format!("Save failed: {error:?}"))
}

pub async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?
        .dyn_into()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let surface = instance
        .create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        })
        .await
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let fault = crate::gpu_fault::Fault::default();
    let errors = fault.clone();
    device.on_uncaptured_error(std::sync::Arc::new(move |error| failed(errors.clone(), format!("WebGPU error: {error}"))));
    let lost = fault.clone();
    device.set_device_lost_callback(move |reason, message| failed(lost.clone(), format!("WebGPU device lost ({reason:?}): {message}")));
    let limit = device.limits().max_texture_dimension_2d;
    let width = (window.inner_width()?.as_f64().ok_or("No window width")? as u32).clamp(1, limit);
    let height = (window.inner_height()?.as_f64().ok_or("No window height")? as u32).clamp(1, limit);
    canvas.set_width(width);
    canvas.set_height(height);
    let mut config = surface
        .get_default_config(&adapter, width, height)
        .ok_or("No compatible surface format")?;
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let mut editor = Editor::default();
    let mut renderer = Renderer::new(
        device,
        queue,
        config.format.add_srgb_suffix(),
        &editor.scene,
    );
    renderer.resize(Viewport { width, height });
    inspect(&editor, &renderer)?;
    let mut panel = crate::panel::Panel::new(
        &renderer,
        config.format.add_srgb_suffix(),
        &[
            "Help",
            "Open",
            "Open Replace",
            "Cancel Open",
            "Close",
            "Unload Sources",
            "Reload Sources",
            "Cancel Reload",
            "Save",
            "Example Box",
            "Example Triangle",
            "Select Next",
            "Delete",
            "Undo",
            "Redo",
            "Background",
            "Zoom In",
            "Zoom Out",
            "Pan Left",
            "Pan Right",
            "Orbit Right",
            "Orbit Up",
            "View Isometric",
            "View Perspective",
            "View Orthographic",
            "Fit",
            "Fit Selected",
            "Move",
            "View Reset",
        ],
    );
    panel.update(None, &canvas)?;
    if resize(
        &window,
        &canvas,
        &surface,
        &mut config,
        &mut renderer,
        &mut editor,
    ) {
        panel.update(None, &canvas)?;
        present(
            &surface,
            &renderer,
            &editor.background,
            &editor.camera.uniform(),
            &mut panel,
        )?;
    }
    let input_canvas = canvas.clone();
    let browser_window = window.clone();
    let pointer_canvas = canvas.clone();
    let mut gesture = Gesture::default();
    let request = std::rc::Rc::new(std::cell::RefCell::new(crate::read_gate::ReadGate::default()));
    let read_mode = std::cell::Cell::new(crate::file_input::Mode::Append);
    let delivery: crate::file_input::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let reload: crate::browser_reload::Shared = std::rc::Rc::new(std::cell::RefCell::new(Default::default()));
    let reload_delivery: crate::browser_reload::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let owned_request = std::rc::Rc::clone(&request);
    let owned_reload = std::rc::Rc::clone(&reload);
    let active_fault = fault.clone();
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        if active_fault.message().is_some() { return; }
        if event.type_() == "pagehide" {
            gesture.cancel();
            if event.dyn_ref::<web_sys::PageTransitionEvent>().is_some_and(|event| event.persisted()) { return; }
            wasm_bindgen_futures::spawn_local(async { crate::browser_runtime::stop(); });
            return;
        }
        let shortcut = event.dyn_ref::<web_sys::KeyboardEvent>().and_then(|key| panel.shortcut(key));
        let line = if let Some(shortcut) = shortcut {
            event.prevent_default();
            panel.answer(shortcut.command(), "");
            Some(shortcut.command().to_ascii_lowercase())
        } else { match panel.update(Some(&event), &input_canvas) {
            Ok(line) => line,
            Err(error) => {
                report(&format!("Cannot read command: {error:?}"));
                return;
            }
        } };

        let action = if event.type_() == "cancel" && event.target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
            .is_some_and(|input| input.id() == "open")
        {
            request.borrow_mut().cancel();
            report("Open cancelled.");
            panel.answer("Open", "Open cancelled.");
            None
        } else if event.type_() == "change" {
            crate::file_input::choose(&event, std::rc::Rc::clone(&request), read_mode.get(), std::rc::Rc::clone(&delivery));
            None
        } else if event.type_() == "viewer-file-error" {
            if let Some(message) = event.dyn_ref::<web_sys::CustomEvent>().and_then(|event| event.detail().as_string()) {
                report(&message);
                panel.answer("Open", &message);
            }
            None
        } else if event.type_() == "viewer-reload" {
            if let Some(reply) = reload_delivery.borrow_mut().take() {
                let command = reply.intent.map_or("Reload Sources", |intent| intent.command());
                match reply.complete(&mut editor) {
                    Ok(Some(crate::edit_replay::Reply::Changed(_))) => {
                        renderer.set_scene(&editor.scene, editor.selected);
                        let message = if command == "Reload Sources" {
                            "Editable sources restored; display retained.".to_owned()
                        } else { format!("{command} completed after source reload.") };
                        report(&message); panel.answer(command, &message);
                    }
                    Ok(Some(crate::edit_replay::Reply::Saved(bytes))) => {
                        let message = save_result(&bytes).unwrap_or_else(|error| error);
                        report(&message); panel.answer(command, &message);
                    }
                    Ok(None) => {}
                    Err(error) => { report(&error); panel.answer(command, &error); }
                }
            }
            None
        } else if event.type_() == "viewer-file" {
            match crate::file_input::action(&event, &delivery) {
                Ok(action) => action,
                Err(error) => { let message = format!("Cannot retain reload file: {error:?}");
                    report(&message); panel.answer("Open", &message); None }
            }
        } else if let Some(line) = line {
            if line == "reload sources" {
                let result = crate::browser_reload::start(std::rc::Rc::clone(&reload), editor.reload_keys(), std::rc::Rc::clone(&reload_delivery));
                match result {
                    Ok(()) => panel.result("Reloading editable sources…"),
                    Err(error) => { report(&error); panel.result(&error); }
                }
                None
            } else if line == "cancel reload" {
                reload.borrow_mut().cancel();
                report("Reload cancelled."); panel.result("Reload cancelled.");
                None
            } else if line == "unload sources" {
                Some(Action::UnloadSources)
            } else if line == "close" {
                request.borrow_mut().cancel();
                Some(Action::Close)
            } else if line == "cancel open" {
                request.borrow_mut().cancel();
                report("Open cancelled.");
                panel.result("Open cancelled.");
                None
            } else if line == "open" || line == "open replace" {
                let ticket = request.borrow_mut().begin();
                if let Err(error) = ticket { report(error); panel.result(error); return; }
                if line == "open replace" { reload.borrow_mut().cancel(); }
                read_mode.set(if line == "open replace" { crate::file_input::Mode::Replace }
                    else { crate::file_input::Mode::Append });
                if let Some(input) = web_sys::window()
                    .and_then(|window| window.document())
                    .and_then(|document| document.get_element_by_id("open"))
                    .and_then(|element| element.dyn_into::<web_sys::HtmlInputElement>().ok())
                {
                    input.click();
                }
                None
            } else if line == "save" {
                let result = crate::browser_reload::restore_for(crate::edit_intent::Intent::Save,
                    &editor, std::rc::Rc::clone(&reload), std::rc::Rc::clone(&reload_delivery))
                    .and_then(|waiting| {
                        if waiting { Ok("Reloading editable sources…".to_owned()) }
                        else { crate::document::snapshot(&editor.scene).map_err(str::to_owned)
                            .and_then(|bytes| save_result(&bytes)) }
                    });
                panel.result(&result.unwrap_or_else(|error| error));
                None
            } else if line == "move" || line.starts_with("move ") {
                match crate::offset::parse(&line) {
                    Ok(offset) => Some(Action::Translate(offset)),
                    Err(error) => { panel.result(error); None }
                }
            } else {
                let action = match line.as_str() {
                    "example box" => Action::AddBox,
                    "example triangle" => Action::ToggleExtra,
                    "select next" => Action::SelectNext,
                    "delete" => Action::Delete,
                    "undo" => Action::Undo,
                    "redo" => Action::Redo,
                    "background" => Action::Background,
                    "zoom in" => Action::Zoom(2.0),
                    "zoom out" => Action::Zoom(0.5),
                    "pan left" => Action::Pan(-0.25, 0.0),
                    "pan right" => Action::Pan(0.25, 0.0),
                    "orbit right" => Action::Orbit(std::f64::consts::FRAC_PI_4, 0.0),
                    "orbit up" => Action::Orbit(0.0, std::f64::consts::FRAC_PI_6),
                    "view isometric" => Action::Isometric,
                    "view perspective" => {
                        Action::Projection(crate::camera::Projection::Perspective)
                    }
                    "view orthographic" => {
                        Action::Projection(crate::camera::Projection::Orthographic)
                    }
                    "fit" if shortcut.is_some() && editor.selected.is_some() => Action::FitSelected,
                    "fit" => Action::Fit,

                    "fit selected" => Action::FitSelected,
                    "view reset" => Action::ResetView,
                    _ => return,
                };
                Some(action)
            }
        } else if !panel.consumed {
            navigation_action(&event, &pointer_canvas, &mut gesture)
        } else {
            None
        };
        if let Some(action) = action {
            if matches!(&action, Action::Close | Action::UnloadSources | Action::Undo | Action::Redo
                | Action::Replace(_) | Action::ReplaceAt(_, _)) { reload.borrow_mut().cancel(); }
            let replacement = matches!(&action, Action::Replace(_) | Action::ReplaceAt(_, _));
            let closing = matches!(&action, Action::Close);
            let unloading = matches!(&action, Action::UnloadSources);
            let waiting = crate::edit_intent::Intent::capture(&action, editor.selected)
                .map_err(str::to_owned).and_then(|intent| match intent {
                    Some(intent) => crate::browser_reload::restore_for(intent, &editor,
                        std::rc::Rc::clone(&reload), std::rc::Rc::clone(&reload_delivery)),
                    None => Ok(false),
                });
            let change = waiting.and_then(|waiting| {
                if waiting { panel.result("Reloading editable sources…"); Ok(None) }
                else { editor.apply(action).map(Some).map_err(str::to_owned) }
            });
            match change {
                Ok(Some(Change::Scene)) => {
                    renderer.set_scene(&editor.scene, editor.selected);
                    if unloading {
                        report("Editable sources unloaded; display retained.");
                        panel.result("Editable sources unloaded; display retained.");
                    } else if closing {
                        report("Document closed.");
                        panel.result("Document closed.");
                    } else if event.type_() == "viewer-file" {
                        let message = if replacement { "Document replaced. Undo restores the previous document." }
                            else { "File imported. Undo removes the entire import." };
                        report(message);
                        panel.answer(if replacement { "Open Replace" } else { "Open" }, message);
                    }
                }
                Ok(Some(Change::View)) | Ok(None) => {}
                Err(error) => {
                    report(&error);
                    if event.type_() == "viewer-file" { panel.answer("Open", &error); }
                    else { panel.result(&error); }
                }
            }
        }
        if let Err(error) = inspect(&editor, &renderer) {
            report(&format!("Cannot report projection: {error:?}"));
            return;
        }
        if resize(
            &browser_window,
            &pointer_canvas,
            &surface,
            &mut config,
            &mut renderer,
            &mut editor,
        ) {
            if let Err(error) = panel.update(None, &input_canvas) {
                report(&format!("Cannot lay out commands: {error:?}"));
                return;
            }
            if let Err(error) = present(
                &surface,
                &renderer,
                &editor.background,
                &editor.camera.uniform(),
                &mut panel,
            ) {
                report(&format!("Cannot redraw: {error:?}"));
            }
        }
    });
    let mut listeners = crate::listeners::Listeners::new(update);
    for name in [
        "pointerdown",
        "pointermove",
        "pointerup",
        "pointercancel",
        "lostpointercapture",
        "contextmenu",
        "keydown",
        "keyup",
        "blur",
    ] {
        listeners.listen(canvas.as_ref(), name)?;
    }
    listeners.listen(canvas.as_ref(), "wheel")?;
    listeners.listen(window.as_ref(), "resize")?;
    listeners.listen(window.as_ref(), "blur")?;
    listeners.listen(document.as_ref(), "change")?;
    listeners.listen(document.as_ref(), "cancel")?;
    listeners.listen(window.as_ref(), "viewer-file")?;
    listeners.listen(window.as_ref(), "viewer-file-error")?;
    listeners.listen(window.as_ref(), "viewer-reload")?;
    listeners.listen(window.as_ref(), "pagehide")?;
    crate::browser_runtime::install(listeners, owned_request, owned_reload, fault);
    report("Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.");
    Ok(())
}

fn inspect(editor: &Editor, renderer: &Renderer) -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or("No document")?;
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?;
    let placements: Vec<_> = editor.scene.objects().iter().map(|row| (format!("{:?}", row.id), row.model.m)).collect();
    canvas.set_attribute("data-row-placements", &serde_json::to_string(&placements).map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-selected-id", &editor.selected.map(|id| format!("{id:?}")).unwrap_or_default())?;
    canvas.set_attribute("data-camera-matrix", &serde_json::to_string(&editor.camera.view_projection().m).map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-object-count", &editor.scene.objects().len().to_string())?;
    let metadata: Vec<_> = editor.scene.objects().iter().map(|row| {
        let m = &row.metadata;
        (&row.guid, &m.source_guid, &m.name, m.is_visible, m.is_locked)
    }).collect();
    let origins: Vec<_> = editor.scene.objects().iter().filter_map(|row| row.origin().map(|origin|
        (&row.guid, origin.id.to_string(), origin.version.hex(),
            origin.location.as_ref().map(|location| location.value())))).collect();
    let residency: Vec<_> = editor.scene.objects().iter().map(|row|
        (&row.guid, row.geometry().is_some(), row.release_epoch())).collect();
    canvas.set_attribute("data-source-residency", &serde_json::to_string(&residency)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-source-origins", &serde_json::to_string(&origins)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-row-metadata", &serde_json::to_string(&metadata)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-gpu-stats", &format!("{:?}", renderer.stats()))?;
    let cpu = crate::memory::cpu(editor.scenes(), renderer.geometry_owners().map(|owner| &owner.source));
    canvas.set_attribute("data-cpu-usage", &format!("{cpu:?}"))?;
    canvas.set_attribute("data-gpu-usage", &format!("{:?}", renderer.usage()))?;
    canvas.set_attribute("data-projection", &format!("{:?}", editor.camera.projection))
}

fn navigation_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
    gesture: &mut Gesture,
) -> Option<Action> {
    let action = match event.type_().as_str() {
        "wheel" => {
            let wheel = event.dyn_ref::<web_sys::WheelEvent>()?;
            let mode = wheel.delta_mode();
            crate::navigation::wheel(
                wheel.delta_y(),
                mode,
                canvas.get_bounding_client_rect().height(),
            )
        }
        "keydown" => {
            let key = event.dyn_ref::<web_sys::KeyboardEvent>()?;
            if key.is_composing() || key.alt_key() {
                return None;
            }
            if key.key() == "Escape" {
                gesture.cancel();
                event.prevent_default();
                return None;
            }
            return None;
        }
        _ => return pointer_action(event, canvas, gesture),
    }?;
    event.prevent_default();
    Some(action)
}

fn pointer_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
    gesture: &mut Gesture,
) -> Option<Action> {
    match event.type_().as_str() {
        "contextmenu" => {
            event.prevent_default();
            return None;
        }
        "blur" | "resize" => {
            gesture.cancel();
            return None;
        }
        _ => {}
    }
    let pointer = event.dyn_ref::<web_sys::PointerEvent>()?;
    let id = pointer.pointer_id();
    let position = [pointer.client_x() as f64, pointer.client_y() as f64];
    let motion = match event.type_().as_str() {
        "pointerdown" => {
            if pointer.is_primary() && gesture.press(id, pointer.button(), position) {
                event.prevent_default();
                let options = web_sys::FocusOptions::new();
                options.set_prevent_scroll(true);
                if canvas.focus_with_options(&options).is_err()
                    || canvas.set_pointer_capture(id).is_err()
                {
                    gesture.cancel();
                }
            }
            None
        }
        "pointermove" => gesture.move_to(id, position),
        "pointerup" => {
            let motion = gesture.release(id, position);
            let _ = canvas.release_pointer_capture(id);
            motion
        }
        "pointercancel" | "lostpointercapture" => {
            gesture.cancel_pointer(id);
            None
        }
        _ => None,
    }?;
    let rect = canvas.get_bounding_client_rect();
    motion.action([rect.left(), rect.top(), rect.width(), rect.height()])
}

fn resize(
    window: &web_sys::Window,
    canvas: &web_sys::HtmlCanvasElement,
    surface: &wgpu::Surface<'_>,
    config: &mut wgpu::SurfaceConfiguration,
    renderer: &mut Renderer,
    editor: &mut Editor,
) -> bool {
    let rect = canvas.get_bounding_client_rect();
    let limit = renderer.device.limits().max_texture_dimension_2d;
    let Some(size) = Viewport::from_css(
        rect.width(),
        rect.height(),
        window.device_pixel_ratio(),
        limit,
    ) else {
        return false;
    };
    if size.width != config.width || size.height != config.height {
        canvas.set_width(size.width);
        canvas.set_height(size.height);
        config.width = size.width;
        config.height = size.height;
        surface.configure(&renderer.device, config);
        renderer.resize(size);
    }
    editor.camera.aspect = size.aspect();
    true
}

fn present(
    surface: &wgpu::Surface<'_>,
    renderer: &Renderer,
    background: &Background,
    transform: &[f32; 16],
    panel: &mut crate::panel::Panel,
) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame)
        | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
        other => {
            return Err(JsValue::from_str(&format!(
                "Surface unavailable: {other:?}"
            )));
        }
    };
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
        format: Some(frame.texture.format().add_srgb_suffix()),
        ..Default::default()
    });
    renderer.draw(&view, background, transform);
    panel.draw(renderer, &view);
    frame.present();
    Ok(())
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/34a-stop-03.rs"
```

```text
--8<-- "journey/code/34a-stop-04.rs"
```

```text
--8<-- "journey/code/34a-stop-05.rs"
```

```text
--8<-- "journey/code/34a-stop-06.rs"
```

</details>

</details>

<a id="checkpoint-34b-report"></a>

### Step 3 of 6 · Describe a viewer run without keeping its document (`34b-report`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 3/6 Visual Trace (34b-report): Describe a viewer run without keeping its document</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/diagnostic.rs, src/diagnostic_tests.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="91" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">browser metadata</text>
    <line x1="164" y1="68" x2="180" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="182" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="255" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">owned Context</text>
    <line x1="328" y1="68" x2="344" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="346" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">Report header</text>
    <line x1="492" y1="68" x2="508" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="510" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="583" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">explicit JSON</text>
    <line x1="656" y1="68" x2="672" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="674" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="747" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">independent snapshot</text>
    
</svg>
</div>

- **Goal:** Define owned diagnostic context and a serializable run outcome, without retaining scene or GPU owners.
- **Data Flow Trace:** `browser metadata → owned Context → Report header → explicit JSON → independent snapshot`
- **Files You Touch:** `src/diagnostic.rs` (*Create a serializable report header with owned browser context and explicit run outcome.*) · `src/diagnostic_tests.rs` (*Optional authoring fixture. Check the actual JSON field names, round trip and independence of a retained snapshot.*) · `src/lib.rs` (*Expose the report model and include its native tests.*)

Create a report that survives losing the GPU. `Context` stores page/browser details and dimensions. `Report` stores that context, run identity, timestamps and outcome; it owns no document or device.

Owned strings and copied dimensions make a clone an independent snapshot. `Outcome` distinguishes Running, Ready, Closed and Failed. A Running value alone is not proof of a crash.

Serde’s derives serialize and parse the value. `rename_all` writes camelCase JSON names; `flatten` puts Context fields in the report’s top level. `version` identifies the format. `started` stays fixed while `lastSeen` can advance.

This lesson builds the Rust value. Reading live browser metadata and downloading it are separate steps below.

> **Self-Check Question:** *Why does a diagnostic snapshot own small strings and dimensions instead of keeping the editor or renderer?*  
> **Answer:** The report must survive the viewer’s disposal without extending scene or GPU lifetimes. Owned metadata records the relevant context independently; document values and rendering handles are absent.

- **What You Should See:** Run the report checks below. Serialize a report and inspect its flat JSON header; changing a cloned snapshot must not change the live value. Browser metadata is connected later.
- **Hands-On Lab Experiment:** Remove serde(flatten), run the JSON assertion, and explain how the public format changed. Then compare the live and cloned report after changing its drawing-buffer size.

<details>
<summary>Implementation 34b-report: Describe a viewer run without keeping its document</summary>

Define owned diagnostic context and a serializable run outcome, without retaining scene or GPU owners.

Data flow: browser metadata → owned Context → Report header → explicit JSON → independent snapshot.

### 1. `src/diagnostic.rs`

Create a serializable report header with owned browser context and explicit run outcome.

Create the file and type:

```rust
--8<-- "journey/code/34b-report-01.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/diagnostic_tests.rs`

Optional authoring fixture. Check the actual JSON field names, round trip and independence of a retained snapshot.

Copy this supplied file:

```text
--8<-- "journey/code/34b-report-02.rs"
```

</details>

### 3. `src/lib.rs`

Expose the report model and include its native tests.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod gpu_fault;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod diagnostic;
#[cfg(test)]
mod diagnostic_tests;
pub mod gpu_fault;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/34b-report-03.rs"
```

</details>

</details>

<a id="checkpoint-34ba-events"></a>

### Step 4 of 6 · Keep recent events without losing the first failure (`34ba-events`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 4/6 Visual Trace (34ba-events): Keep recent events without losing the first failure</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/diagnostic.rs, src/diagnostic_tests.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="91" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">finite relative time</text>
    <line x1="164" y1="68" x2="180" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="182" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="255" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">bounded event</text>
    <line x1="328" y1="68" x2="344" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="346" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">first failure</text>
    <text x="419" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">retained</text>
    <line x1="492" y1="68" x2="508" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="510" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="583" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">last 24 observations</text>
    <line x1="656" y1="68" x2="672" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="674" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="747" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">stable failed</text>
    <text x="747" y="81" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">outcome</text>
    
</svg>
</div>

- **Goal:** Bound diagnostic observations while keeping the original failure independently of recent event rotation.
- **Data Flow Trace:** `finite relative time → bounded event → first failure retained → last 24 observations → stable failed outcome`
- **Files You Touch:** `src/diagnostic.rs` (*Describe an observation separately from its bounded owner and preserve its relative timing.*) · `src/diagnostic_tests.rs` (*Optional authoring fixture. Prove bounded history/text, retained first failure, ready-state semantics and atomic rejection of invalid timing.*)

Keep recent observations and the first fatal reason separately. `VecDeque` is a queue: pop its oldest entry before appending when it reaches 24 events. The first-failure field survives that rotation.

`record` rejects negative or non-finite elapsed time before mutation, caps kind/message text at 64/4096 Unicode scalar values, updates lastSeen and appends one event. A fatal event sets Failed only once; a later ready milestone cannot clear the failure.

These limits govern recording. Deserialized JSON still needs an admission check before the viewer trusts it.

> **Self-Check Question:** *Why keep the first failure separately from the recent event queue?*  
> **Answer:** Recent observations rotate to bound memory. The original failure must remain available even after follow-on errors evict that event from the recent window.

- **What You Should See:** Run the event checks below. After 30 observations, exactly 24 remain, while the first fatal reason is preserved separately. Recording a success must not clear failure.
- **Hands-On Lab Experiment:** Store failure only in the recent queue and run the rotation test. Then remove the failed-outcome guard and explain why a later successful milestone would misdescribe the run.

<details>
<summary>Implementation 34ba-events: Keep recent events without losing the first failure</summary>

Bound diagnostic observations while keeping the original failure independently of recent event rotation.

Data flow: finite relative time → bounded event → first failure retained → last 24 observations → stable failed outcome.

### 1. `src/diagnostic.rs`

Describe an observation separately from its bounded owner and preserve its relative timing. Retain the recent event window separately from the first failure, so rotation cannot erase it. Validate relative time, bound observation text/history, and make failure survive later milestones and event rotation.

<details>
<summary>Locate the existing block</summary>

```rust
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub page: String,
    pub browser: String,
    pub secure_context: bool,
    pub webgpu: bool,
    pub viewport: [u32; 2],
    pub canvas: [u32; 2],
    pub device_pixel_ratio: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Outcome { Running, Ready, Closed, Failed }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub version: u32,
    pub tab: String,
    pub started: String,
    pub last_seen: String,
    pub outcome: Outcome,
    #[serde(flatten)]
    pub context: Context,
}

impl Report {
    pub fn new(tab: String, started: String, context: Context) -> Self {
        Self { version: 1, tab, last_seen: started.clone(), started, outcome: Outcome::Running, context }
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub page: String,
    pub browser: String,
    pub secure_context: bool,
    pub webgpu: bool,
    pub viewport: [u32; 2],
    pub canvas: [u32; 2],
    pub device_pixel_ratio: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Outcome { Running, Ready, Closed, Failed }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub time: String,
    pub elapsed_ms: f64,
    pub kind: String,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub version: u32,
    pub tab: String,
    pub started: String,
    pub last_seen: String,
    pub outcome: Outcome,
    #[serde(flatten)]
    pub context: Context,
    events: std::collections::VecDeque<Event>,
    failure: Option<Event>,
}

impl Report {
    pub fn new(tab: String, started: String, context: Context) -> Self {
        Self { version: 1, tab, last_seen: started.clone(), started, outcome: Outcome::Running,
            context, events: Default::default(), failure: None }
    }

    pub fn failure(&self) -> Option<&Event> { self.failure.as_ref() }

    pub fn record(&mut self, time: String, elapsed_ms: f64, kind: &str, message: &str) -> Result<(), &'static str> {
        if !elapsed_ms.is_finite() || elapsed_ms < 0.0 { return Err("Invalid elapsed time"); }
        let event = Event { time, elapsed_ms, kind: kind.chars().take(64).collect(), message: message.chars().take(4096).collect() };
        self.last_seen.clone_from(&event.time);
        if kind == "fatal" {
            if self.failure.is_none() { self.failure = Some(event.clone()); }
            self.outcome = Outcome::Failed;
        } else if kind == "milestone" && message == "geometry on screen" && self.failure.is_none() {
            self.outcome = Outcome::Ready;
        }
        if self.events.len() == 24 { self.events.pop_front(); }
        self.events.push_back(event);
        Ok(())
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/34ba-events-01.rs"
```

```text
--8<-- "journey/code/34ba-events-02.rs"
```

```text
--8<-- "journey/code/34ba-events-03.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/diagnostic_tests.rs`

Optional authoring fixture. Prove bounded history/text, retained first failure, ready-state semantics and atomic rejection of invalid timing.

<details>
<summary>Locate the existing block</summary>

```text
use crate::diagnostic::{Context, Outcome, Report};

fn context() -> Context {
    Context { page: "https://example.test/viewer/".into(), browser: "Test browser".into(),
        secure_context: true, webgpu: true, viewport: [900, 760], canvas: [900, 760], device_pixel_ratio: 1.0 }
}

#[test]
fn report_starts_running_with_flat_browser_context() {
    let report = Report::new("tab-a".into(), "2026-10-03T18:00:00.000Z".into(), context());
    let value = serde_json::to_value(&report).unwrap();
    assert_eq!(value["version"], 1); assert_eq!(value["outcome"], "running");
    assert_eq!(value["started"], value["lastSeen"]);
    assert_eq!(value["secureContext"], true); assert_eq!(value["devicePixelRatio"], 1.0);
    assert_eq!(value["viewport"], serde_json::json!([900, 760]));
    assert!(value.get("context").is_none());
    assert_eq!(serde_json::from_value::<Report>(value).unwrap(), report);
}

#[test]
fn report_snapshot_keeps_its_own_context() {
    let mut live = Report::new("tab-a".into(), "start".into(), context()); let snapshot = live.clone();
    live.context.canvas = [618, 1373]; live.context.device_pixel_ratio = 2.625;
    live.outcome = Outcome::Ready;
    assert_eq!(snapshot.context.canvas, [900, 760]); assert_eq!(snapshot.outcome, Outcome::Running);
    assert_eq!(serde_json::from_str::<Report>(&serde_json::to_string(&live).unwrap()).unwrap(), live);
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/34ba-events-04.rs" skip=1382
```

</details>

</details>

<a id="checkpoint-34c-browser"></a>

### Step 5 of 6 · Read live diagnostic context outside the GPU runtime (`34c-browser`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 5/6 Visual Trace (34c-browser): Read live diagnostic context outside the GPU runtime</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/browser_report.rs, Cargo.toml, src/lib.rs, src/browser.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="91" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">page begins</text>
    <line x1="164" y1="68" x2="180" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="182" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="255" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">sanitized context</text>
    <line x1="328" y1="68" x2="344" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="346" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">independent report</text>
    <text x="419" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">slot</text>
    <line x1="492" y1="68" x2="508" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="510" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="583" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">current context</text>
    <line x1="656" y1="68" x2="672" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="674" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="747" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">JSON snapshot</text>
    
</svg>
</div>

- **Goal:** Start a bounded page report and read real browser context independently of the renderer’s lifetime.
- **Data Flow Trace:** `page begins → sanitized context → independent report slot → current context → JSON snapshot`
- **Files You Touch:** `src/browser_report.rs` (*Own one bounded report for the page and read actual browser metadata without keeping any viewer or GPU owner.*) · `Cargo.toml` (*Enable typed browser address, identity and monotonic-clock access.*) · `src/lib.rs` (*Compile the browser report bridge only for WebAssembly.*) · `src/browser.rs` (*Start diagnostic context before awaiting the GPU connection; failure observation and downloads are connected next.*)

Populate the report from the page before requesting a GPU. One thread-local `RefCell<Option<Report>>` owns metadata independently of the drawing runtime. `RefCell` checks short shared/mutable borrows at runtime.

`context` reads the real viewport, canvas, density and browser details. Build the page value from origin and path, excluding query and fragment. `Reflect` reads WebGPU availability without generated WebGPU bindings.

`start` creates a fresh run ID and UTC start time. `observe` combines `performance.now()` timing with a UTC timestamp. `diagnostic_snapshot` refreshes context and serializes the current report. Its debug export lets you inspect the JSON in the console; it does not add a feature button.

Readiness and fatal observations are wired in the next lesson.

> **Self-Check Question:** *Why does the report live outside the Runtime that GPU failure disposes?*  
> **Answer:** A failed Runtime releases input, source and GPU owners. Its report must remain readable afterward, so a separate bounded metadata slot owns it without retaining those resources.

- **What You Should See:** In the debug console, inspect `window.wasmBindings.diagnostic_snapshot()`. Its viewport and canvas dimensions must match this browser. Resize and read again: context should refresh without changing the scene.
- **Hands-On Lab Experiment:** Put the report inside Runtime instead, trigger the actual device-loss check, and explain why it would disappear with the evidence it was meant to retain. Compare a URL containing a query parameter with the report’s page field.

<details>
<summary>Implementation 34c-browser: Read live diagnostic context outside the GPU runtime</summary>

Start a bounded page report and read real browser context independently of the renderer’s lifetime.

Data flow: page begins → sanitized context → independent report slot → current context → JSON snapshot.

### 1. `src/browser_report.rs`

Own one bounded report for the page and read actual browser metadata without keeping any viewer or GPU owner.

Create the file and type:

```rust
--8<-- "journey/code/34c-browser-01.rs"
```

### 2. `Cargo.toml`

Enable typed browser address, identity and monotonic-clock access.

<details>
<summary>Locate the existing block</summary>

```toml
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```toml
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "Location", "Navigator", "Performance", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/34c-browser-02.rs"
```

</details>

### 3. `src/lib.rs`

Compile the browser report bridge only for WebAssembly.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(target_arch = "wasm32")]
mod browser_runtime;
```

</details>

After that block, type:

```rust
#[cfg(target_arch = "wasm32")]
mod browser_report;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/34c-browser-03.rs"
```

</details>

### 4. `src/browser.rs`

Start diagnostic context before awaiting the GPU connection; failure observation and downloads are connected next.

<details>
<summary>Locate the existing block</summary>

```rust
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    crate::browser_report::start()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/34c-browser-04.rs"
```

</details>

</details>

<a id="checkpoint-34d-download"></a>

### Step 6 of 6 · Download diagnostics through the real command line (`34d-download`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 6/6 Visual Trace (34d-download): Download diagnostics through the real command line</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/file_output.rs, src/browser_report.rs, src/browser.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="91" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">first scene</text>
    <text x="91" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">presented</text>
    <line x1="164" y1="68" x2="180" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="182" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="255" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">ready report</text>
    <line x1="328" y1="68" x2="344" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="346" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">JSON Blob</text>
    <line x1="492" y1="68" x2="508" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="510" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="583" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">actual download</text>
    <line x1="656" y1="68" x2="672" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="674" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="747" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">delayed URL cleanup</text>
    
</svg>
</div>

- **Goal:** Download a current ready-run report with Report and attempt one independent first-failure download after GPU disposal.
- **Data Flow Trace:** `first scene presented → ready report → typed Report → JSON Blob → actual download → delayed URL cleanup`
- **Files You Touch:** `src/file_output.rs` (*Reuse the existing owned Blob/anchor cleanup for a named report while keeping Save’s filename unchanged.*) · `src/browser_report.rs` (*Download a current metadata snapshot and attempt one automatic download for the first fatal observation.*) · `src/browser.rs` (*Keep the matching device’s failure evidence after its GPU runtime is disposed.*) · `src/lib.rs` (*Capture startup failure after diagnostic initialization, even when no GPU runtime could be installed.*)

Add `Report` to the command dock. It downloads the current metadata as `viewer-diagnostic.json`. The first presented scene records a geometry-on-screen milestone and marks a healthy run Ready.

Finish serializing before clicking the temporary download anchor. This releases the report’s `RefCell` borrow before browser behavior can call back into Rust. The existing download owner revokes its Blob URL after ten seconds.

A GPU failure first stops its matching runtime, then records the fatal reason and attempts one report download. Startup failures are recorded too when metadata initialization succeeded. Browsers may block automatic downloads; storing a report for later retrieval comes next. `Save` still writes the editable document and neither download consumes Undo.

> **Self-Check Question:** *Why can the failed viewer download a report after GPU input and drawing have stopped?*  
> **Answer:** The independent report owns metadata only. JSON serialization and the existing Blob/anchor download need no GPU resources, so disposal can stop the renderer without destroying its evidence.

- **What You Should See:** Type `Report` and open `viewer-diagnostic.json`. A drawn scene should have outcome `ready` and a geometry-on-screen event. The document and Undo history stay unchanged.
- **Hands-On Lab Experiment:** Keep the report inside the GPU Runtime and explain why the fatal download would lose it. Then compare the actual document Save and diagnostic downloads and explain why they cannot be used interchangeably.

<details>
<summary>Implementation 34d-download: Download diagnostics through the real command line</summary>

Download a current ready-run report with Report and attempt one independent first-failure download after GPU disposal.

Data flow: first scene presented → ready report → typed Report → JSON Blob → actual download → delayed URL cleanup.

### 1. `src/file_output.rs`

Reuse the existing owned Blob/anchor cleanup for a named report while keeping Save’s filename unchanged. Choose the diagnostic JSON filename without changing the document download contract.

<details>
<summary>Locate the existing block</summary>

```rust
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

pub fn download(bytes: &[u8]) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let body = document.body().ok_or("No document body")?;
    let anchor: web_sys::HtmlAnchorElement = document.create_element("a")?.dyn_into()?;
    anchor.set_hidden(true);
    anchor.set_download("viewer.session");
    body.append_child(&anchor)?;
    let parts = js_sys::Array::new();
    parts.push(&js_sys::Uint8Array::from(bytes));
    let blob = match web_sys::Blob::new_with_u8_array_sequence(&parts) {
        Ok(blob) => blob,
        Err(error) => { anchor.remove(); return Err(error); }
    };
    let url = match web_sys::Url::create_object_url_with_blob(&blob) {
        Ok(url) => url,
        Err(error) => { anchor.remove(); return Err(error); }
    };
    anchor.set_href(&url);
    anchor.click();
    anchor.remove();
    let cleanup: js_sys::Function = Closure::once_into_js(move || {
        let _ = web_sys::Url::revoke_object_url(&url);
    }).unchecked_into();
    if let Err(error) = window.set_timeout_with_callback_and_timeout_and_arguments_0(&cleanup, 10_000) {
        let _ = cleanup.call0(&JsValue::NULL);
        return Err(error);
    }
    Ok(())
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

pub fn download(bytes: &[u8]) -> Result<(), JsValue> { download_named(bytes, "viewer.session") }

pub fn download_named(bytes: &[u8], name: &str) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let body = document.body().ok_or("No document body")?;
    let anchor: web_sys::HtmlAnchorElement = document.create_element("a")?.dyn_into()?;
    anchor.set_hidden(true);
    anchor.set_download(name);
    body.append_child(&anchor)?;
    let parts = js_sys::Array::new();
    parts.push(&js_sys::Uint8Array::from(bytes));
    let blob = match web_sys::Blob::new_with_u8_array_sequence(&parts) {
        Ok(blob) => blob,
        Err(error) => { anchor.remove(); return Err(error); }
    };
    let url = match web_sys::Url::create_object_url_with_blob(&blob) {
        Ok(url) => url,
        Err(error) => { anchor.remove(); return Err(error); }
    };
    anchor.set_href(&url);
    anchor.click();
    anchor.remove();
    let cleanup: js_sys::Function = Closure::once_into_js(move || {
        let _ = web_sys::Url::revoke_object_url(&url);
    }).unchecked_into();
    if let Err(error) = window.set_timeout_with_callback_and_timeout_and_arguments_0(&cleanup, 10_000) {
        let _ = cleanup.call0(&JsValue::NULL);
        return Err(error);
    }
    Ok(())
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/34d-download-01.rs"
```

```text
--8<-- "journey/code/34d-download-02.rs"
```

</details>

### 2. `src/browser_report.rs`

Download a current metadata snapshot and attempt one automatic download for the first fatal observation.

<details>
<summary>Locate the existing block</summary>

```rust
#[wasm_bindgen::prelude::wasm_bindgen]
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn download() -> Result<(), JsValue> {
    let json = diagnostic_snapshot()?;
    crate::file_output::download_named(json.as_bytes(), "viewer-diagnostic.json")
}

pub fn fatal(message: &str) {
    let first = REPORT.with(|slot| slot.borrow().as_ref().is_some_and(|report| report.failure().is_none()));
    if observe("fatal", message).is_ok() && first { let _ = download(); }
}

#[wasm_bindgen::prelude::wasm_bindgen]
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/34d-download-03.rs"
```

</details>

### 3. `src/browser.rs`

Keep the matching device’s failure evidence after its GPU runtime is disposed. Expose diagnostics through the actual command dock without an HTML feature button. Mark the report ready only after the initial scene frame is presented. Handle the typed report command separately from document Save and edit history.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
                    input.click();
                }
                None
            } else if line == "save" {
                let result = crate::browser_reload::restore_for(crate::edit_intent::Intent::Save,
                    &editor, std::rc::Rc::clone(&reload), std::rc::Rc::clone(&reload_delivery))
```

</details>

Replace it with:

```rust
                    input.click();
                }
                None
            } else if line == "report" {
                let result = crate::browser_report::download().map(|()| "Diagnostic report downloaded.".to_owned())
                    .unwrap_or_else(|error| format!("Diagnostic report failed: {error:?}"));
                panel.result(&result);
                None
            } else if line == "save" {
                let result = crate::browser_reload::restore_for(crate::edit_intent::Intent::Save,
                    &editor, std::rc::Rc::clone(&reload), std::rc::Rc::clone(&reload_delivery))
```

<details>
<summary>Locate this block</summary>

```rust
            &editor.camera.uniform(),
            &mut panel,
        )?;
    }
    let input_canvas = canvas.clone();
    let browser_window = window.clone();
```

</details>

Replace it with:

```rust
            &editor.camera.uniform(),
            &mut panel,
        )?;
        crate::browser_report::observe("milestone", "geometry on screen")?;
    }
    let input_canvas = canvas.clone();
    let browser_window = window.clone();
```

<details>
<summary>Locate this block</summary>

```rust
            "Reload Sources",
            "Cancel Reload",
            "Save",
            "Example Box",
            "Example Triangle",
            "Select Next",
```

</details>

Replace it with:

```rust
            "Reload Sources",
            "Cancel Reload",
            "Save",
            "Report",
            "Example Box",
            "Example Triangle",
            "Select Next",
```

<details>
<summary>Locate this block</summary>

```rust
fn failed(fault: crate::gpu_fault::Fault, message: String) {
    if !fault.remember(message.clone()) { return; }
    wasm_bindgen_futures::spawn_local(async move {
        if crate::browser_runtime::stop_if(&fault) { report(&format!("Cannot draw: {message}")); }
    });
}
```

</details>

Replace it with:

```rust
fn failed(fault: crate::gpu_fault::Fault, message: String) {
    if !fault.remember(message.clone()) { return; }
    wasm_bindgen_futures::spawn_local(async move {
        if crate::browser_runtime::stop_if(&fault) {
            crate::browser_report::fatal(&message);
            report(&format!("Cannot draw: {message}"));
        }
    });
}
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::background::Background;
use crate::editor::{Action, Change, Editor};
use crate::gesture::Gesture;
use crate::renderer::Renderer;
use crate::viewport::Viewport;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

pub fn report(message: &str) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        if let Some(status) = document.get_element_by_id("status") {
            status.set_text_content(Some(message));
            if message.starts_with("Cannot draw:") { let _ = status.remove_attribute("hidden"); }
        }
    }
}

fn failed(fault: crate::gpu_fault::Fault, message: String) {
    if !fault.remember(message.clone()) { return; }
    wasm_bindgen_futures::spawn_local(async move {
        if crate::browser_runtime::stop_if(&fault) {
            crate::browser_report::fatal(&message);
            report(&format!("Cannot draw: {message}"));
        }
    });
}

fn save_result(bytes: &[u8]) -> Result<String, String> {
    crate::file_output::download(bytes)
        .map(|()| format!("Saved editable document ({} bytes)", bytes.len()))
        .map_err(|error| format!("Save failed: {error:?}"))
}

pub async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?
        .dyn_into()?;
    crate::browser_report::start()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let surface = instance
        .create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        })
        .await
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let fault = crate::gpu_fault::Fault::default();
    let errors = fault.clone();
    device.on_uncaptured_error(std::sync::Arc::new(move |error| failed(errors.clone(), format!("WebGPU error: {error}"))));
    let lost = fault.clone();
    device.set_device_lost_callback(move |reason, message| failed(lost.clone(), format!("WebGPU device lost ({reason:?}): {message}")));
    let limit = device.limits().max_texture_dimension_2d;
    let width = (window.inner_width()?.as_f64().ok_or("No window width")? as u32).clamp(1, limit);
    let height = (window.inner_height()?.as_f64().ok_or("No window height")? as u32).clamp(1, limit);
    canvas.set_width(width);
    canvas.set_height(height);
    let mut config = surface
        .get_default_config(&adapter, width, height)
        .ok_or("No compatible surface format")?;
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let mut editor = Editor::default();
    let mut renderer = Renderer::new(
        device,
        queue,
        config.format.add_srgb_suffix(),
        &editor.scene,
    );
    renderer.resize(Viewport { width, height });
    inspect(&editor, &renderer)?;
    let mut panel = crate::panel::Panel::new(
        &renderer,
        config.format.add_srgb_suffix(),
        &[
            "Help",
            "Open",
            "Open Replace",
            "Cancel Open",
            "Close",
            "Unload Sources",
            "Reload Sources",
            "Cancel Reload",
            "Save",
            "Report",
            "Example Box",
            "Example Triangle",
            "Select Next",
            "Delete",
            "Undo",
            "Redo",
            "Background",
            "Zoom In",
            "Zoom Out",
            "Pan Left",
            "Pan Right",
            "Orbit Right",
            "Orbit Up",
            "View Isometric",
            "View Perspective",
            "View Orthographic",
            "Fit",
            "Fit Selected",
            "Move",
            "View Reset",
        ],
    );
    panel.update(None, &canvas)?;
    if resize(
        &window,
        &canvas,
        &surface,
        &mut config,
        &mut renderer,
        &mut editor,
    ) {
        panel.update(None, &canvas)?;
        present(
            &surface,
            &renderer,
            &editor.background,
            &editor.camera.uniform(),
            &mut panel,
        )?;
        crate::browser_report::observe("milestone", "geometry on screen")?;
    }
    let input_canvas = canvas.clone();
    let browser_window = window.clone();
    let pointer_canvas = canvas.clone();
    let mut gesture = Gesture::default();
    let request = std::rc::Rc::new(std::cell::RefCell::new(crate::read_gate::ReadGate::default()));
    let read_mode = std::cell::Cell::new(crate::file_input::Mode::Append);
    let delivery: crate::file_input::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let reload: crate::browser_reload::Shared = std::rc::Rc::new(std::cell::RefCell::new(Default::default()));
    let reload_delivery: crate::browser_reload::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let owned_request = std::rc::Rc::clone(&request);
    let owned_reload = std::rc::Rc::clone(&reload);
    let active_fault = fault.clone();
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        if active_fault.message().is_some() { return; }
        if event.type_() == "pagehide" {
            gesture.cancel();
            if event.dyn_ref::<web_sys::PageTransitionEvent>().is_some_and(|event| event.persisted()) { return; }
            wasm_bindgen_futures::spawn_local(async { crate::browser_runtime::stop(); });
            return;
        }
        let shortcut = event.dyn_ref::<web_sys::KeyboardEvent>().and_then(|key| panel.shortcut(key));
        let line = if let Some(shortcut) = shortcut {
            event.prevent_default();
            panel.answer(shortcut.command(), "");
            Some(shortcut.command().to_ascii_lowercase())
        } else { match panel.update(Some(&event), &input_canvas) {
            Ok(line) => line,
            Err(error) => {
                report(&format!("Cannot read command: {error:?}"));
                return;
            }
        } };

        let action = if event.type_() == "cancel" && event.target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
            .is_some_and(|input| input.id() == "open")
        {
            request.borrow_mut().cancel();
            report("Open cancelled.");
            panel.answer("Open", "Open cancelled.");
            None
        } else if event.type_() == "change" {
            crate::file_input::choose(&event, std::rc::Rc::clone(&request), read_mode.get(), std::rc::Rc::clone(&delivery));
            None
        } else if event.type_() == "viewer-file-error" {
            if let Some(message) = event.dyn_ref::<web_sys::CustomEvent>().and_then(|event| event.detail().as_string()) {
                report(&message);
                panel.answer("Open", &message);
            }
            None
        } else if event.type_() == "viewer-reload" {
            if let Some(reply) = reload_delivery.borrow_mut().take() {
                let command = reply.intent.map_or("Reload Sources", |intent| intent.command());
                match reply.complete(&mut editor) {
                    Ok(Some(crate::edit_replay::Reply::Changed(_))) => {
                        renderer.set_scene(&editor.scene, editor.selected);
                        let message = if command == "Reload Sources" {
                            "Editable sources restored; display retained.".to_owned()
                        } else { format!("{command} completed after source reload.") };
                        report(&message); panel.answer(command, &message);
                    }
                    Ok(Some(crate::edit_replay::Reply::Saved(bytes))) => {
                        let message = save_result(&bytes).unwrap_or_else(|error| error);
                        report(&message); panel.answer(command, &message);
                    }
                    Ok(None) => {}
                    Err(error) => { report(&error); panel.answer(command, &error); }
                }
            }
            None
        } else if event.type_() == "viewer-file" {
            match crate::file_input::action(&event, &delivery) {
                Ok(action) => action,
                Err(error) => { let message = format!("Cannot retain reload file: {error:?}");
                    report(&message); panel.answer("Open", &message); None }
            }
        } else if let Some(line) = line {
            if line == "reload sources" {
                let result = crate::browser_reload::start(std::rc::Rc::clone(&reload), editor.reload_keys(), std::rc::Rc::clone(&reload_delivery));
                match result {
                    Ok(()) => panel.result("Reloading editable sources…"),
                    Err(error) => { report(&error); panel.result(&error); }
                }
                None
            } else if line == "cancel reload" {
                reload.borrow_mut().cancel();
                report("Reload cancelled."); panel.result("Reload cancelled.");
                None
            } else if line == "unload sources" {
                Some(Action::UnloadSources)
            } else if line == "close" {
                request.borrow_mut().cancel();
                Some(Action::Close)
            } else if line == "cancel open" {
                request.borrow_mut().cancel();
                report("Open cancelled.");
                panel.result("Open cancelled.");
                None
            } else if line == "open" || line == "open replace" {
                let ticket = request.borrow_mut().begin();
                if let Err(error) = ticket { report(error); panel.result(error); return; }
                if line == "open replace" { reload.borrow_mut().cancel(); }
                read_mode.set(if line == "open replace" { crate::file_input::Mode::Replace }
                    else { crate::file_input::Mode::Append });
                if let Some(input) = web_sys::window()
                    .and_then(|window| window.document())
                    .and_then(|document| document.get_element_by_id("open"))
                    .and_then(|element| element.dyn_into::<web_sys::HtmlInputElement>().ok())
                {
                    input.click();
                }
                None
            } else if line == "report" {
                let result = crate::browser_report::download().map(|()| "Diagnostic report downloaded.".to_owned())
                    .unwrap_or_else(|error| format!("Diagnostic report failed: {error:?}"));
                panel.result(&result);
                None
            } else if line == "save" {
                let result = crate::browser_reload::restore_for(crate::edit_intent::Intent::Save,
                    &editor, std::rc::Rc::clone(&reload), std::rc::Rc::clone(&reload_delivery))
                    .and_then(|waiting| {
                        if waiting { Ok("Reloading editable sources…".to_owned()) }
                        else { crate::document::snapshot(&editor.scene).map_err(str::to_owned)
                            .and_then(|bytes| save_result(&bytes)) }
                    });
                panel.result(&result.unwrap_or_else(|error| error));
                None
            } else if line == "move" || line.starts_with("move ") {
                match crate::offset::parse(&line) {
                    Ok(offset) => Some(Action::Translate(offset)),
                    Err(error) => { panel.result(error); None }
                }
            } else {
                let action = match line.as_str() {
                    "example box" => Action::AddBox,
                    "example triangle" => Action::ToggleExtra,
                    "select next" => Action::SelectNext,
                    "delete" => Action::Delete,
                    "undo" => Action::Undo,
                    "redo" => Action::Redo,
                    "background" => Action::Background,
                    "zoom in" => Action::Zoom(2.0),
                    "zoom out" => Action::Zoom(0.5),
                    "pan left" => Action::Pan(-0.25, 0.0),
                    "pan right" => Action::Pan(0.25, 0.0),
                    "orbit right" => Action::Orbit(std::f64::consts::FRAC_PI_4, 0.0),
                    "orbit up" => Action::Orbit(0.0, std::f64::consts::FRAC_PI_6),
                    "view isometric" => Action::Isometric,
                    "view perspective" => {
                        Action::Projection(crate::camera::Projection::Perspective)
                    }
                    "view orthographic" => {
                        Action::Projection(crate::camera::Projection::Orthographic)
                    }
                    "fit" if shortcut.is_some() && editor.selected.is_some() => Action::FitSelected,
                    "fit" => Action::Fit,

                    "fit selected" => Action::FitSelected,
                    "view reset" => Action::ResetView,
                    _ => return,
                };
                Some(action)
            }
        } else if !panel.consumed {
            navigation_action(&event, &pointer_canvas, &mut gesture)
        } else {
            None
        };
        if let Some(action) = action {
            if matches!(&action, Action::Close | Action::UnloadSources | Action::Undo | Action::Redo
                | Action::Replace(_) | Action::ReplaceAt(_, _)) { reload.borrow_mut().cancel(); }
            let replacement = matches!(&action, Action::Replace(_) | Action::ReplaceAt(_, _));
            let closing = matches!(&action, Action::Close);
            let unloading = matches!(&action, Action::UnloadSources);
            let waiting = crate::edit_intent::Intent::capture(&action, editor.selected)
                .map_err(str::to_owned).and_then(|intent| match intent {
                    Some(intent) => crate::browser_reload::restore_for(intent, &editor,
                        std::rc::Rc::clone(&reload), std::rc::Rc::clone(&reload_delivery)),
                    None => Ok(false),
                });
            let change = waiting.and_then(|waiting| {
                if waiting { panel.result("Reloading editable sources…"); Ok(None) }
                else { editor.apply(action).map(Some).map_err(str::to_owned) }
            });
            match change {
                Ok(Some(Change::Scene)) => {
                    renderer.set_scene(&editor.scene, editor.selected);
                    if unloading {
                        report("Editable sources unloaded; display retained.");
                        panel.result("Editable sources unloaded; display retained.");
                    } else if closing {
                        report("Document closed.");
                        panel.result("Document closed.");
                    } else if event.type_() == "viewer-file" {
                        let message = if replacement { "Document replaced. Undo restores the previous document." }
                            else { "File imported. Undo removes the entire import." };
                        report(message);
                        panel.answer(if replacement { "Open Replace" } else { "Open" }, message);
                    }
                }
                Ok(Some(Change::View)) | Ok(None) => {}
                Err(error) => {
                    report(&error);
                    if event.type_() == "viewer-file" { panel.answer("Open", &error); }
                    else { panel.result(&error); }
                }
            }
        }
        if let Err(error) = inspect(&editor, &renderer) {
            report(&format!("Cannot report projection: {error:?}"));
            return;
        }
        if resize(
            &browser_window,
            &pointer_canvas,
            &surface,
            &mut config,
            &mut renderer,
            &mut editor,
        ) {
            if let Err(error) = panel.update(None, &input_canvas) {
                report(&format!("Cannot lay out commands: {error:?}"));
                return;
            }
            if let Err(error) = present(
                &surface,
                &renderer,
                &editor.background,
                &editor.camera.uniform(),
                &mut panel,
            ) {
                report(&format!("Cannot redraw: {error:?}"));
            }
        }
    });
    let mut listeners = crate::listeners::Listeners::new(update);
    for name in [
        "pointerdown",
        "pointermove",
        "pointerup",
        "pointercancel",
        "lostpointercapture",
        "contextmenu",
        "keydown",
        "keyup",
        "blur",
    ] {
        listeners.listen(canvas.as_ref(), name)?;
    }
    listeners.listen(canvas.as_ref(), "wheel")?;
    listeners.listen(window.as_ref(), "resize")?;
    listeners.listen(window.as_ref(), "blur")?;
    listeners.listen(document.as_ref(), "change")?;
    listeners.listen(document.as_ref(), "cancel")?;
    listeners.listen(window.as_ref(), "viewer-file")?;
    listeners.listen(window.as_ref(), "viewer-file-error")?;
    listeners.listen(window.as_ref(), "viewer-reload")?;
    listeners.listen(window.as_ref(), "pagehide")?;
    crate::browser_runtime::install(listeners, owned_request, owned_reload, fault);
    report("Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.");
    Ok(())
}

fn inspect(editor: &Editor, renderer: &Renderer) -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or("No document")?;
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?;
    let placements: Vec<_> = editor.scene.objects().iter().map(|row| (format!("{:?}", row.id), row.model.m)).collect();
    canvas.set_attribute("data-row-placements", &serde_json::to_string(&placements).map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-selected-id", &editor.selected.map(|id| format!("{id:?}")).unwrap_or_default())?;
    canvas.set_attribute("data-camera-matrix", &serde_json::to_string(&editor.camera.view_projection().m).map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-object-count", &editor.scene.objects().len().to_string())?;
    let metadata: Vec<_> = editor.scene.objects().iter().map(|row| {
        let m = &row.metadata;
        (&row.guid, &m.source_guid, &m.name, m.is_visible, m.is_locked)
    }).collect();
    let origins: Vec<_> = editor.scene.objects().iter().filter_map(|row| row.origin().map(|origin|
        (&row.guid, origin.id.to_string(), origin.version.hex(),
            origin.location.as_ref().map(|location| location.value())))).collect();
    let residency: Vec<_> = editor.scene.objects().iter().map(|row|
        (&row.guid, row.geometry().is_some(), row.release_epoch())).collect();
    canvas.set_attribute("data-source-residency", &serde_json::to_string(&residency)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-source-origins", &serde_json::to_string(&origins)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-row-metadata", &serde_json::to_string(&metadata)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-gpu-stats", &format!("{:?}", renderer.stats()))?;
    let cpu = crate::memory::cpu(editor.scenes(), renderer.geometry_owners().map(|owner| &owner.source));
    canvas.set_attribute("data-cpu-usage", &format!("{cpu:?}"))?;
    canvas.set_attribute("data-gpu-usage", &format!("{:?}", renderer.usage()))?;
    canvas.set_attribute("data-projection", &format!("{:?}", editor.camera.projection))
}

fn navigation_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
    gesture: &mut Gesture,
) -> Option<Action> {
    let action = match event.type_().as_str() {
        "wheel" => {
            let wheel = event.dyn_ref::<web_sys::WheelEvent>()?;
            let mode = wheel.delta_mode();
            crate::navigation::wheel(
                wheel.delta_y(),
                mode,
                canvas.get_bounding_client_rect().height(),
            )
        }
        "keydown" => {
            let key = event.dyn_ref::<web_sys::KeyboardEvent>()?;
            if key.is_composing() || key.alt_key() {
                return None;
            }
            if key.key() == "Escape" {
                gesture.cancel();
                event.prevent_default();
                return None;
            }
            return None;
        }
        _ => return pointer_action(event, canvas, gesture),
    }?;
    event.prevent_default();
    Some(action)
}

fn pointer_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
    gesture: &mut Gesture,
) -> Option<Action> {
    match event.type_().as_str() {
        "contextmenu" => {
            event.prevent_default();
            return None;
        }
        "blur" | "resize" => {
            gesture.cancel();
            return None;
        }
        _ => {}
    }
    let pointer = event.dyn_ref::<web_sys::PointerEvent>()?;
    let id = pointer.pointer_id();
    let position = [pointer.client_x() as f64, pointer.client_y() as f64];
    let motion = match event.type_().as_str() {
        "pointerdown" => {
            if pointer.is_primary() && gesture.press(id, pointer.button(), position) {
                event.prevent_default();
                let options = web_sys::FocusOptions::new();
                options.set_prevent_scroll(true);
                if canvas.focus_with_options(&options).is_err()
                    || canvas.set_pointer_capture(id).is_err()
                {
                    gesture.cancel();
                }
            }
            None
        }
        "pointermove" => gesture.move_to(id, position),
        "pointerup" => {
            let motion = gesture.release(id, position);
            let _ = canvas.release_pointer_capture(id);
            motion
        }
        "pointercancel" | "lostpointercapture" => {
            gesture.cancel_pointer(id);
            None
        }
        _ => None,
    }?;
    let rect = canvas.get_bounding_client_rect();
    motion.action([rect.left(), rect.top(), rect.width(), rect.height()])
}

fn resize(
    window: &web_sys::Window,
    canvas: &web_sys::HtmlCanvasElement,
    surface: &wgpu::Surface<'_>,
    config: &mut wgpu::SurfaceConfiguration,
    renderer: &mut Renderer,
    editor: &mut Editor,
) -> bool {
    let rect = canvas.get_bounding_client_rect();
    let limit = renderer.device.limits().max_texture_dimension_2d;
    let Some(size) = Viewport::from_css(
        rect.width(),
        rect.height(),
        window.device_pixel_ratio(),
        limit,
    ) else {
        return false;
    };
    if size.width != config.width || size.height != config.height {
        canvas.set_width(size.width);
        canvas.set_height(size.height);
        config.width = size.width;
        config.height = size.height;
        surface.configure(&renderer.device, config);
        renderer.resize(size);
    }
    editor.camera.aspect = size.aspect();
    true
}

fn present(
    surface: &wgpu::Surface<'_>,
    renderer: &Renderer,
    background: &Background,
    transform: &[f32; 16],
    panel: &mut crate::panel::Panel,
) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame)
        | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
        other => {
            return Err(JsValue::from_str(&format!(
                "Surface unavailable: {other:?}"
            )));
        }
    };
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
        format: Some(frame.texture.format().add_srgb_suffix()),
        ..Default::default()
    });
    renderer.draw(&view, background, transform);
    panel.draw(renderer, &view);
    frame.present();
    Ok(())
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/34d-download-04.rs"
```

```text
--8<-- "journey/code/34d-download-05.rs"
```

```text
--8<-- "journey/code/34d-download-06.rs"
```

```text
--8<-- "journey/code/34d-download-07.rs"
```

</details>

### 4. `src/lib.rs`

Capture startup failure after diagnostic initialization, even when no GPU runtime could be installed.

<details>
<summary>Locate the existing block</summary>

```rust
            browser::report(&format!("Cannot draw: {error:?}"));
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
            browser_report::fatal(&format!("Cannot draw: {error:?}"));
            browser::report(&format!("Cannot draw: {error:?}"));
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/34d-download-08.rs"
```

</details>

</details>

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 34d-download
npm --prefix ../session_tests run course -- check 34d-download
npm --prefix ../session_tests run course -- save 34d-download
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

![Recorded visual output for Lesson 4.2 (34d-download)](../../screenshots/journey/34d-download-browser.png)

Type `Report` and open `viewer-diagnostic.json`. A drawn scene should have outcome `ready` and a geometry-on-screen event. The document and Undo history stay unchanged.

- **Final Lesson Experiment:** Keep the report inside the GPU Runtime and explain why the fatal download would lose it. Then compare the actual document Save and diagnostic downloads and explain why they cannot be used interchangeably.

Continue with [Admit useful evidence and bound stored runs](34e-schema.md).
