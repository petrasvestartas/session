# 4.1 · Browser callbacks with a real lifetime

**Section 3: Scene Documents, Transactions, and Browser GPU Lifetime** · **Lesson 4.1** (Redraft `3.8`) · **3 checkpoints (`33-owner` → `33b-cache`)** · **Study time: 3–6 h**

> **What You Will Learn & Build:** Build a browser listener owner that detaches every registered event before freeing the Rust callback and its captured data. By the final checkpoint (`33b-cache`), you will distinguish a reusable cached page from a final exit so Back navigation cannot restore an already-disposed viewer.

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
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Lesson 4.1 Pipeline Schematic: Browser callbacks with a real lifetime</text>
  <text x="20" y="45" font-size="11" fill="#111111">Step-by-step data flow you implement in this lesson (3 Checkpoints: 33-owner → 33b-cache)</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="89" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="90" x2="151" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="89" y="108" text-anchor="middle" font-size="11" fill="#111111">Rust Closure</text>
    <line x1="161" y1="108" x2="181" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="183" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="254" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="193" y1="90" x2="316" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="254" y="108" text-anchor="middle" font-size="11" fill="#111111">successful</text>
    <text x="254" y="124" text-anchor="middle" font-size="11" fill="#111111">EventTarget</text>
    <text x="254" y="140" text-anchor="middle" font-size="11" fill="#111111">bindings</text>
    <line x1="326" y1="108" x2="346" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="348" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="358" y1="90" x2="481" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">owner Drop</text>
    <line x1="491" y1="108" x2="511" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="513" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="584" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="523" y1="90" x2="646" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="584" y="108" text-anchor="middle" font-size="11" fill="#111111">remove handlers</text>
    <line x1="656" y1="108" x2="676" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="678" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="749" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 5</text>
    <line x1="688" y1="90" x2="811" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="749" y="108" text-anchor="middle" font-size="11" fill="#111111">free captured</text>
    <text x="749" y="124" text-anchor="middle" font-size="11" fill="#111111">values</text>
    
    <rect x="18" y="158" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="177" font-size="11" font-weight="700" fill="#111111">Files Modified in Lesson 4.1:</text>
    <text x="235" y="177" font-size="11" fill="#111111">src/listeners.rs  ·  src/lib.rs  ·  src/browser_runtime.rs  ·  src/browser.rs  ·  Cargo.toml</text>
    <text x="32" y="194" font-size="10.5" fill="#111111">3 Checkpoints: 33-owner → 33b-cache — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
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
| **1** | [`33-owner`](#checkpoint-33-owner) | Build a browser listener owner that detaches every registered event before freeing the Rust callback and its captured data. | `src/listeners.rs`, `src/lib.rs` | `Rust Closure → successful EventTarget bindings → owner Drop → remove handlers → free captured values` |
| **2** | [`33a-runtime`](#checkpoint-33a-runtime) | Bind real viewer input to owned callbacks and cancel pending file/source authority when the page hides. | `src/browser_runtime.rs`, `src/lib.rs`, `src/browser.rs` | `Live runtime → pagehide → next microtask → cancel read/fetch → detach eighteen listeners → release captures` |
| **3** | [`33b-cache`](#checkpoint-33b-cache) | Distinguish a reusable cached page from a final exit so Back navigation cannot restore an already-disposed viewer. | `Cargo.toml`, `src/browser.rs` | `pagehide → cancel drag → persisted? → keep owner for cached return; otherwise deferred disposal` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Study the **Visual Step Trace**, read the **Concept Walkthrough** and **Self-Check Q&A**, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-33-owner"></a>

### Step 1 of 3 · Own browser listeners instead of forgetting callbacks (`33-owner`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 1/3 Visual Trace (33-owner): Own browser listeners instead of forgetting callbacks</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/listeners.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="91" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">Rust Closure</text>
    <line x1="164" y1="68" x2="180" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="182" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="255" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">successful</text>
    <text x="255" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">EventTarget…</text>
    <line x1="328" y1="68" x2="344" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="346" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">owner Drop</text>
    <line x1="492" y1="68" x2="508" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="510" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="583" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">remove handlers</text>
    <line x1="656" y1="68" x2="672" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="674" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="747" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">free captured values</text>
    
</svg>
</div>

- **Goal:** Build a browser listener owner that detaches every registered event before freeing the Rust callback and its captured data.
- **Data Flow Trace:** `Rust Closure → successful EventTarget bindings → owner Drop → remove handlers → free captured values`
- **Files You Touch:** `src/listeners.rs` (*Own one Rust callback and every successful browser registration; detach handlers before its captured values drop. The debug probe exercises real EventTargets.*) · `src/lib.rs` (*Register browser-only lifetime ownership without changing native rendering.*)

Keep a JavaScript callback alive with its registered listeners. Dropping this owner removes every listener, including after partial setup failure.

A Closure owns the Rust environment used by a JavaScript callback. Listeners keeps that closure beside each successfully registered target and event name. One callback can serve the canvas, window and document. listen records a binding only after registration succeeds; if later setup fails, dropping the owner removes earlier registrations.

Drop runs before Rust drops the fields. Its loop removes each handler while callback still exists, then normal field cleanup releases the closure and captured values. Registration uses passive false so the existing wheel and pointer prevention behavior can be retained during integration. The next checkpoint replaces the main viewer’s forgotten callback with this owner and adds explicit runtime disposal. This checkpoint prepares ownership; it does not yet stop the existing viewer runtime.

> **Self-Check Question:** *Why must event listeners be removed before the Rust closure is dropped?*  
> **Answer:** The browser keeps JavaScript references to the callback. Dropping its Rust environment first would leave a registered handler that can invoke an invalid closure. Detach every handler while the closure is still valid.

- **What You Should See:** In the debug browser console, run `Array.from(window.wasmBindings.listener_probe())`. Expect `[2, 2, 0]`: two live calls, no calls after Drop, and no retained captured owner.
- **Hands-On Lab Experiment:** Remove the Drop loop and dispatch the probe events after dropping the owner. Predict the invalid-closure error before running it; refresh to restore the verified checkpoint.

<details>
<summary>Implementation 33-owner: Own browser listeners instead of forgetting callbacks</summary>

Build a browser listener owner that detaches every registered event before freeing the Rust callback and its captured data.

Data flow: Rust Closure → successful EventTarget bindings → owner Drop → remove handlers → free captured values.

### 1. `src/listeners.rs`

Own one Rust callback and every successful browser registration; detach handlers before its captured values drop. The debug probe exercises real EventTargets.

Create the file and type:

```rust
--8<-- "journey/code/33-owner-01.rs"
```

### 2. `src/lib.rs`

Register browser-only lifetime ownership without changing native rendering.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(target_arch = "wasm32")]
mod panel;
```

</details>

After that block, type:

```rust
#[cfg(target_arch = "wasm32")]
mod listeners;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/33-owner-02.rs"
```

</details>

</details>

<a id="checkpoint-33a-runtime"></a>

### Step 2 of 3 · Dispose the viewer without leaving pending work alive (`33a-runtime`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 2/3 Visual Trace (33a-runtime): Dispose the viewer without leaving pending work alive</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/browser_runtime.rs, src/lib.rs, src/browser.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="91" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">Live runtime</text>
    <line x1="164" y1="68" x2="180" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="182" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="255" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">pagehide</text>
    <line x1="328" y1="68" x2="344" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="346" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">cancel read/fetch</text>
    <line x1="492" y1="68" x2="508" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="510" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="583" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">detach eighteen</text>
    <text x="583" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">listeners</text>
    <line x1="656" y1="68" x2="672" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="674" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="747" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">release captures</text>
    
</svg>
</div>

- **Goal:** Bind real viewer input to owned callbacks and cancel pending file/source authority when the page hides.
- **Data Flow Trace:** `Live runtime → pagehide → next microtask → cancel read/fetch → detach eighteen listeners → release captures`
- **Files You Touch:** `src/browser_runtime.rs` (*Keep the live owner in one runtime slot; Drop cancels read/fetch authority before listeners and captured editor/GPU owners are released.*) · `src/lib.rs` (*Register browser runtime ownership beside the listener owner.*) · `src/browser.rs` (*Retain cancellation handles outside the moving callback. Defer page-exit disposal until the active callback has returned.*)

Keep the viewer’s callbacks alive after startup returns. Final page exit releases them; replacing the runtime releases the previous owner.

The main viewer now transfers update into Listeners and records all canvas, document and window bindings. A thread-local Option<Runtime> keeps that owner alive after run returns. install replaces any earlier owner; stop takes the current owner out of the slot and consumes it. The slot’s mutable borrow has ended before Runtime is dropped.

Runtime’s Drop first cancels the read gate and abortable source flight. It then allows the listener owner to detach handlers and release the callback, editor, renderer and other captured values. Cancelling releases pending source keys even when an asynchronous task remains alive. This is ownership cleanup; it does not claim browser or driver memory is reclaimed at the same instant.

pagehide schedules stop through spawn_local and returns immediately. Its task runs on the next microtask so the event callback is no longer active when it is freed. Document Close retains its existing meaning and can still be followed by Open. Page exit disposes the viewer itself.

> **Self-Check Question:** *Why is disposal deferred until the next microtask instead of dropping the runtime inside its own event callback?*  
> **Answer:** The pagehide callback is still executing from the owned closure. Disposal waits for that invocation to return, then cancels work and drops the closure safely. spawn_local schedules the task for the next microtask even when its future is immediately ready.

- **What You Should See:** In the debug console, dispatch `window.dispatchEvent(new Event("pagehide"))`. On the next task, `window.wasmBindings.runtime_running()` must be false. Reload the same page to restart.
- **Hands-On Lab Experiment:** Skip read/fetch cancellation in Runtime::drop, hold a source response and dispose the viewer. Explain which source URL or late-read acceptance checks reveal retained or revived authority.

<details>
<summary>Implementation 33a-runtime: Dispose the viewer without leaving pending work alive</summary>

Bind real viewer input to owned callbacks and cancel pending file/source authority when the page hides.

Data flow: Live runtime → pagehide → next microtask → cancel read/fetch → detach eighteen listeners → release captures.

### 1. `src/browser_runtime.rs`

Keep the live owner in one runtime slot; Drop cancels read/fetch authority before listeners and captured editor/GPU owners are released.

Create the file and type:

```rust
--8<-- "journey/code/33a-runtime-01.rs"
```

### 2. `src/lib.rs`

Register browser runtime ownership beside the listener owner.

<details>
<summary>Locate the existing block</summary>

```rust
mod listeners;
```

</details>

After that block, type:

```rust
#[cfg(target_arch = "wasm32")]
mod browser_runtime;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/33a-runtime-02.rs"
```

</details>

### 3. `src/browser.rs`

Retain cancellation handles outside the moving callback. Defer page-exit disposal until the active callback has returned. Transfer the event callback into the owner before attaching its targets. Register all eighteen live bindings through the owner; retain it in the runtime instead of forgetting the Rust callback.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
        "keyup",
        "blur",
    ] {
        canvas.add_event_listener_with_callback(name, update.as_ref().unchecked_ref())?;
    }
    let options = web_sys::AddEventListenerOptions::new();
    options.set_passive(false);
    canvas.add_event_listener_with_callback_and_add_event_listener_options(
        "wheel",
        update.as_ref().unchecked_ref(),
        &options,
    )?;
    window.add_event_listener_with_callback("resize", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("blur", update.as_ref().unchecked_ref())?;
    document.add_event_listener_with_callback("change", update.as_ref().unchecked_ref())?;
    document.add_event_listener_with_callback("cancel", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-file", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-file-error", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-reload", update.as_ref().unchecked_ref())?;
    // Both event sources retain this one callback for the lifetime of the page.
    update.forget();
    report("Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.");
    Ok(())
}
```

</details>

Replace it with:

```rust
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
    crate::browser_runtime::install(listeners, owned_request, owned_reload);
    report("Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.");
    Ok(())
}
```

<details>
<summary>Locate this block</summary>

```rust
            }
        }
    });
    for name in [
        "pointerdown",
        "pointermove",
```

</details>

Replace it with:

```rust
            }
        }
    });
    let mut listeners = crate::listeners::Listeners::new(update);
    for name in [
        "pointerdown",
        "pointermove",
```

<details>
<summary>Locate this block</summary>

```rust
    let delivery: crate::file_input::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let reload: crate::browser_reload::Shared = std::rc::Rc::new(std::cell::RefCell::new(Default::default()));
    let reload_delivery: crate::browser_reload::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let shortcut = event.dyn_ref::<web_sys::KeyboardEvent>().and_then(|key| panel.shortcut(key));
        let line = if let Some(shortcut) = shortcut {
            event.prevent_default();
```

</details>

Replace it with:

```rust
    let delivery: crate::file_input::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let reload: crate::browser_reload::Shared = std::rc::Rc::new(std::cell::RefCell::new(Default::default()));
    let reload_delivery: crate::browser_reload::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let owned_request = std::rc::Rc::clone(&request);
    let owned_reload = std::rc::Rc::clone(&reload);
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        if event.type_() == "pagehide" {
            wasm_bindgen_futures::spawn_local(async { crate::browser_runtime::stop(); });
            return;
        }
        let shortcut = event.dyn_ref::<web_sys::KeyboardEvent>().and_then(|key| panel.shortcut(key));
        let line = if let Some(shortcut) = shortcut {
            event.prevent_default();
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
    device.on_uncaptured_error(std::sync::Arc::new(|error| report(&error.to_string())));
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
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        if event.type_() == "pagehide" {
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
    crate::browser_runtime::install(listeners, owned_request, owned_reload);
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
--8<-- "journey/code/33a-runtime-03.rs"
```

```text
--8<-- "journey/code/33a-runtime-04.rs"
```

```text
--8<-- "journey/code/33a-runtime-05.rs"
```

</details>

</details>

<a id="checkpoint-33b-cache"></a>

### Step 3 of 3 · Keep a cached viewer ready for Back navigation (`33b-cache`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 3/3 Visual Trace (33b-cache): Keep a cached viewer ready for Back navigation</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: Cargo.toml, src/browser.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">pagehide</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">cancel drag</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">persisted?</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">keep owner for cached</text>
    <text x="726" y="81" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">return;…</text>
    
</svg>
</div>

- **Goal:** Distinguish a reusable cached page from a final exit so Back navigation cannot restore an already-disposed viewer.
- **Data Flow Trace:** `pagehide → cancel drag → persisted? → keep owner for cached return; otherwise deferred disposal`
- **Files You Touch:** `Cargo.toml` (*Enable the browser event type that tells us whether the page may return from its cache.*) · `src/browser.rs` (*Cancel an unfinished drag, retain the cached page’s live owner, and dispose only a final page exit.*)

Keep the viewer alive when the browser caches its page. Returning to that page must preserve working commands and camera control.

The [HTML lifecycle contract](https://html.spec.whatwg.org/dev/nav-history-apis.html#the-pagetransitionevent-interface) distinguishes final departure from a page that may return from the back/forward cache. A cached page reuses its existing document and scripts. Disposing its callbacks without restarting them would leave that restored viewer unable to accept commands.

The handler cancels an unfinished camera gesture, then checks the actual PageTransitionEvent. A persisted transition keeps the runtime. A final exit still schedules the disposal proved in the previous checkpoint. A plain synthetic Event has no persisted flag and retains the previous final-exit behavior. No new listener or feature control is needed.

> **Self-Check Question:** *Does every pagehide mean the page and its WebAssembly instance are going away forever?*  
> **Answer:** No. PageTransitionEvent.persisted marks a page that may be reused on Back navigation. Retain its runtime in that case; a final exit still disposes the viewer.

- **What You Should See:** In the debug console, run `window.dispatchEvent(new PageTransitionEvent("pagehide", {persisted: true}))`, then the same for `"pageshow"`. `window.wasmBindings.runtime_running()` stays true and typed commands still work.
- **Hands-On Lab Experiment:** Remove the persisted check and run the retained-runtime assertions. Explain why a fresh normal load could conceal the broken cached-return path.

<details>
<summary>Implementation 33b-cache: Keep a cached viewer ready for Back navigation</summary>

Distinguish a reusable cached page from a final exit so Back navigation cannot restore an already-disposed viewer.

Data flow: pagehide → cancel drag → persisted? → keep owner for cached return; otherwise deferred disposal.

### 1. `Cargo.toml`

Enable the browser event type that tells us whether the page may return from its cache.

<details>
<summary>Locate the existing block</summary>

```toml
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```toml
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/33b-cache-01.rs"
```

</details>

### 2. `src/browser.rs`

Cancel an unfinished drag, retain the cached page’s live owner, and dispose only a final page exit.

<details>
<summary>Locate the existing block</summary>

```rust
        if event.type_() == "pagehide" {
            wasm_bindgen_futures::spawn_local(async { crate::browser_runtime::stop(); });
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        if event.type_() == "pagehide" {
            gesture.cancel();
            if event.dyn_ref::<web_sys::PageTransitionEvent>().is_some_and(|event| event.persisted()) { return; }
            wasm_bindgen_futures::spawn_local(async { crate::browser_runtime::stop(); });
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/33b-cache-02.rs"
```

</details>

</details>

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 33b-cache
npm --prefix ../session_tests run course -- check 33b-cache
npm --prefix ../session_tests run course -- save 33b-cache
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

![Recorded visual output for Lesson 4.1 (33b-cache)](../../../screenshots/journey/33b-cache-browser.png)

In the debug console, run `window.dispatchEvent(new PageTransitionEvent("pagehide", {persisted: true}))`, then the same for `"pageshow"`. `window.wasmBindings.runtime_running()` stays true and typed commands still work.

- **Final Lesson Experiment:** Remove the persisted check and run the retained-runtime assertions. Explain why a fresh normal load could conceal the broken cached-return path.

Continue with [First failure and independent diagnostic reports](../../consolidation/34-fault.md).
