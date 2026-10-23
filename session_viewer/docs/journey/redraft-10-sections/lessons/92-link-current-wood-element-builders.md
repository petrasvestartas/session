# 18.4 · Original Wood Elements and portable source records

**Goal:** Prove Element recipes source history and archives. · **Steps:** 17 · **Source:** [engine/gpu/ssao.rs](../../../../src/engine/gpu/ssao.rs) · [engine/gpu/ssao/pipelines.rs](../../../../src/engine/gpu/ssao/pipelines.rs) · [shaders/ssao.wgsl](../../../../src/shaders/ssao.wgsl) · [shaders/triangle.wgsl](../../../../src/shaders/triangle.wgsl) · [shaders/ink_visibility.wgsl](../../../../src/shaders/ink_visibility.wgsl) · [engine/gpu/surface_outline.rs](../../../../src/engine/gpu/surface_outline.rs) · [engine/gpu/render.rs](../../../../src/engine/gpu/render.rs) · [engine/performance.rs](../../../../src/engine/performance.rs) · [engine/gpu/timing.rs](../../../../src/engine/gpu/timing.rs) · [app/command/tool/elements.rs](../../../../src/app/command/tool/elements.rs) (`session_viewer/src/lib.rs`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 18.4 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Cargo.toml</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/element_values.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">17 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/engine/gpu/device.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Link current Wood element…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Scene Tree Hierarchy</text>
  <rect x="650" y="50" width="100" height="28" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="68" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Group (Mp)</text>
  <line x1="680" y1="78" x2="640" y2="108" stroke="#111111" stroke-width="1.5" marker-end="url(#arr)"/>
  <line x1="720" y1="78" x2="760" y2="108" stroke="#111111" stroke-width="1.5" marker-end="url(#arr)"/>
  <rect x="600" y="112" width="80" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="640" y="129" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Part A</text>
  <rect x="720" y="112" width="80" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="760" y="129" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Part B</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Original Wood Elements and…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Layer Colours &amp; Lamps</text>
  <rect x="880" y="52" width="188" height="36" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <circle cx="898" cy="70" r="6" fill="#facc15" stroke="#111111" stroke-width="1.25"/>
  <rect x="912" y="63" width="14" height="14" rx="3" fill="#ef4444" stroke="#111111" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="994" y="74" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Layer_01 (Lamp ON)</text>
  <rect x="880" y="98" width="188" height="36" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <circle cx="898" cy="116" r="6" fill="#e5e7eb" stroke="#111111" stroke-width="1.25"/>
  <rect x="912" y="109" width="14" height="14" rx="3" fill="#3b82f6" stroke="#111111" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="994" y="120" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">[Rename Layer_02|]</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Prove Element recipes sou…</text>
</svg>
</div>

## <a id="checkpoint-92-link-current-wood-element-builders"></a>1. Link current Wood element builders

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 17</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Cargo.toml</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original ordered picks</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">typed Element and attribu…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one history edit</text>
</svg>
</div>

**What this code does:** Link current Wood element builders.

### 1. `Cargo.toml`

Link current Wood element builders beside the original kernel.

**Locate this existing block:**

```toml

[dependencies]
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["console", "Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "ErrorEvent", "PromiseRejectionEvent", "Location", "History", "UrlSearchParams", "Navigator", "Storage", "Performance", "PerformanceResourceTiming", "PerformanceEntry", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "ReadableWritablePair", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "RequestMode", "RequestCache", "EventSource", "MessageEvent", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
```

Edit the existing block to match this code; keep unchanged lines:

```toml
--8<-- "journey/code/92-link-current-wood-element-builders-01-console.rs"
```

## <a id="checkpoint-91zza-write-original-beam-column-and-plate-builders"></a>2. Write beam column and plate builders

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2 of 17</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/element_values.rs, src/…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original picks</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing Wood kernel</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">typed element</text>
</svg>
</div>

**What this code does:** Parse positive finite lengths while retaining omitted defaults.

### 1. `src/element_values.rs`

Adapt the maintained positive-length recipe to the earlier number parser.

Create the file and type:

```rust
--8<-- "journey/code/91zza-write-original-beam-column-and-plate-builders-1.rs"
```

### 2. `src/lib.rs`

Register the completed Element component.

**Locate this existing block:**

```rust
#[cfg(test)]
mod global_outline_command_tests;
```

After that block, type:

```rust
--8<-- "journey/code/91zza-write-original-beam-column-and-plate-builders-2.rs" skip=47
```

### 3. `src/element_beam.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/91zza-write-original-beam-column-and-plate-builders-3.rs"
```

### 4. `src/lib.rs`

Register this original element builder.

**Locate this existing block:**

```rust

pub mod element_values;
```

After that block, type:

```rust
--8<-- "journey/code/91zza-write-original-beam-column-and-plate-builders-4.rs" skip=25
```

### 5. `src/element_column.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/91zza-write-original-beam-column-and-plate-builders-5.rs"
```

### 6. `src/lib.rs`

Register this original element builder.

**Locate this existing block:**

```rust

pub mod element_beam;
```

After that block, type:

```rust
--8<-- "journey/code/91zza-write-original-beam-column-and-plate-builders-6.rs" skip=23
```

### 7. `src/element_plate.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/91zza-write-original-beam-column-and-plate-builders-7.rs"
```

### 8. `src/lib.rs`

Register this original element builder.

**Locate this existing block:**

```rust

pub mod element_column;
```

After that block, type:

```rust
--8<-- "journey/code/91zza-write-original-beam-column-and-plate-builders-8.rs" skip=25
```

## <a id="checkpoint-92a-retain-current-positive-lengths-and-six-builders"></a>3. Write support variable beam and block builders

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3 of 17</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/element_support.rs, src…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original picks</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing Wood kernel</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">typed element</text>
</svg>
</div>

**What this code does:** Add the remaining original Support, Beam Variable and Block wrappers.

### 1. `src/element_support.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/92a-retain-current-positive-lengths-and-six-builders-1.rs"
```

### 2. `src/lib.rs`

Register this original element builder.

**Locate this existing block:**

```rust

pub mod element_plate;
```

After that block, type:

```rust
--8<-- "journey/code/92a-retain-current-positive-lengths-and-six-builders-2.rs" skip=24
```

### 3. `src/element_beam_variable.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/92a-retain-current-positive-lengths-and-six-builders-3.rs"
```

### 4. `src/lib.rs`

Register this original element builder.

**Locate this existing block:**

```rust

pub mod element_support;
```

After that block, type:

```rust
--8<-- "journey/code/92a-retain-current-positive-lengths-and-six-builders-4.rs" skip=26
```

### 5. `src/element_block.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/92a-retain-current-positive-lengths-and-six-builders-5.rs"
```

### 6. `src/lib.rs`

Register this original element builder.

**Locate this existing block:**

```rust

pub mod element_beam_variable;
```

After that block, type:

```rust
--8<-- "journey/code/92a-retain-current-positive-lengths-and-six-builders-6.rs" skip=32
```

## <a id="checkpoint-92b-own-original-element-geometry"></a>4. Own Element geometry

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4 of 17</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/element_original.rs, sr…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original ordered picks</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">typed Element and attribu…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one history edit</text>
</svg>
</div>

**What this code does:** Own original Element geometry.

### 1. `src/element_original.rs`

Retain the original Element beside its derived display.

Create the file and type:

```rust
--8<-- "journey/code/92b-own-original-element-geometry-01.rs"
```

### 2. `src/lib.rs`

Register the completed Element component.

**Locate this existing block:**

```rust

pub mod element_block;
```

After that block, type:

```rust
--8<-- "journey/code/92b-own-original-element-geometry-02.rs" skip=24
```

## <a id="checkpoint-92c-read-current-element-command-defaults"></a>5. Read current Element command defaults

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5 of 17</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/element_input.rs, src/l…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original ordered picks</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">typed Element and attribu…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one history edit</text>
</svg>
</div>

**What this code does:** Read current Element command defaults.

### 1. `src/element_input.rs`

Own the six current command kinds and exact defaults.

Create the file and type:

```rust
--8<-- "journey/code/92c-read-current-element-command-defaults-01.rs"
```

### 2. `src/lib.rs`

Register the completed Element component.

**Locate this existing block:**

```rust

pub mod element_original;
```

After that block, type:

```rust
--8<-- "journey/code/92c-read-current-element-command-defaults-02.rs" skip=27
```

## <a id="checkpoint-92d-build-typed-elements-from-ordered-world-picks"></a>6. Build typed Elements from ordered world picks

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 6 of 17</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/element_input.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original ordered picks</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">typed Element and attribu…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one history edit</text>
</svg>
</div>

**What this code does:** Build typed Elements from ordered world picks.

### 1. `src/element_input.rs`

Route ordered original sources into current Wood builders and base planes.

**Locate this existing block:**

```rust
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Param{pub name:&'static str,pub value:f64}
```

After that block, type:

```rust
--8<-- "journey/code/92d-build-typed-elements-from-ordered-world-picks-01.rs" skip=92
```

## <a id="checkpoint-92e-capture-original-picks-while-the-tool-runs"></a>7. Capture picks while the tool runs

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 7 of 17</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/element_draft.rs, src/l…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original ordered picks</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">typed Element and attribu…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one history edit</text>
</svg>
</div>

**What this code does:** Capture original picks while the tool runs.

### 1. `src/element_draft.rs`

Capture fitting original sources and ordered tool state.

Create the file and type:

```rust
--8<-- "journey/code/92e-capture-original-picks-while-the-tool-runs-01.rs"
```

### 2. `src/lib.rs`

Register the completed Element component.

**Locate this existing block:**

```rust

pub mod element_input;
```

After that block, type:

```rust
--8<-- "journey/code/92e-capture-original-picks-while-the-tool-runs-02.rs" skip=24
```

## <a id="checkpoint-92f-teach-named-and-bare-element-values"></a>8. Teach named and bare Element values

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 8 of 17</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/element_draft.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original ordered picks</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">typed Element and attribu…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one history edit</text>
</svg>
</div>

**What this code does:** Teach named and bare Element values.

### 1. `src/element_draft.rs`

Adapt the maintained Element parameter word recipe without changing its semantics.

**Locate this existing block:**

```rust
 pub fn prompt(&self)->String{if let Some(i)=self.asking{format!("{} <{}>: type a number · Enter keeps it",self.params[i].name,self.params[i].value)}else{format!("{} · pick original sources ({} picked) · Enter creates · Esc cancels",self.kind.name(),self.picked.len())}}
 pub fn buttons(&self)->Vec<(String,String)>{self.params.iter().map(|p|(format!("{}={}",p.name,p.value),p.name.into())).chain([("Create".into(),"".into()),("Cancel".into(),"Escape".into())]).collect()}

}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/92f-teach-named-and-bare-element-values-01.rs"
```

## <a id="checkpoint-92g-make-element-input-batches-atomic"></a>9. Make Element input batches atomic

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 9 of 17</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/element_draft.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original ordered picks</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">typed Element and attribu…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one history edit</text>
</svg>
</div>

**What this code does:** Make Element input batches atomic.

### 1. `src/element_draft.rs`

Adopt only validated current parameter batches.

**Locate this existing block:**

```rust
 pub fn valid(&self,scene:&Scene)->Result<(),String>{for o in &self.picked{o.valid(scene).map_err(str::to_owned)?;}Ok(())}
 pub fn build(&self,scene:&Scene)->Result<Vec<Made>,String>{self.valid(scene)?;crate::element_input::build(self.kind,&self.params,&self.picked.iter().map(|o|(o.source.clone(),o.model.clone())).collect::<Vec<_>>())}

 pub fn values(&self)->String{self.params.iter().map(|p|format!("{} {}",p.name,p.value)).collect::<Vec<_>>().join(" · ")}
 pub fn prompt(&self)->String{if let Some(i)=self.asking{format!("{} <{}>: type a number · Enter keeps it",self.params[i].name,self.params[i].value)}else{format!("{} · pick original sources ({} picked) · Enter creates · Esc cancels",self.kind.name(),self.picked.len())}}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/92g-make-element-input-batches-atomic-01.rs"
```

## <a id="checkpoint-92h-attach-readonly-element-attributes"></a>10. Attach readonly Element attributes

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 10 of 17</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/layer_edit.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original ordered picks</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">typed Element and attribu…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one history edit</text>
</svg>
</div>

**What this code does:** Attach readonly Element attributes.

### 1. `src/layer_edit.rs`

Attach generated original attributes under their Element owner.

**Locate this existing block:**

```rust
    replace(scene,owner,tree);Rc::make_mut(&mut scene.groups).insert((doc.key,group_guid));Ok(name)
}
```

After that block, type:

```rust
--8<-- "journey/code/92h-attach-readonly-element-attributes-01.rs" skip=102
```

## <a id="checkpoint-92i-adopt-elements-and-consume-sources-once"></a>11. Adopt Elements and consume sources once

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 11 of 17</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/element_adopt.rs, src/l…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original ordered picks</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">typed Element and attribu…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one history edit</text>
</svg>
</div>

**What this code does:** Adopt Elements and consume sources once.

### 1. `src/element_adopt.rs`

Adopt original Element payloads and readonly attributes as one edit.

Create the file and type:

```rust
--8<-- "journey/code/92i-adopt-elements-and-consume-sources-once-01.rs"
```

### 2. `src/lib.rs`

Register the completed Element component.

**Locate this existing block:**

```rust

pub mod element_draft;
```

After that block, type:

```rust
--8<-- "journey/code/92i-adopt-elements-and-consume-sources-once-02.rs" skip=24
```

### 3. `src/source_edit.rs`

Validate captured original Element identity by shared source ownership.

**Locate this existing block:**

```rust
    match (a, b) {
        (Geometry::PointCloud(a), Geometry::PointCloud(b)) => Rc::ptr_eq(a,b),
        (Geometry::Plane(a), Geometry::Plane(b)) => Rc::ptr_eq(a,b),
        (Geometry::OBB(a), Geometry::OBB(b)) => Rc::ptr_eq(a, b),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/92i-adopt-elements-and-consume-sources-once-03.rs"
```

## <a id="checkpoint-92j-save-and-open-original-element-records"></a>12. Save and open Element records

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 12 of 17</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/geometry_snapshot.rs, s…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original ordered picks</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">typed Element and attribu…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one history edit</text>
</svg>
</div>

**What this code does:** Save and open original Element records.

### 1. `src/geometry_snapshot.rs`

Route original Element records through mixed-document archives.

**Locate this existing block:**

```rust
    }
    for row in scene.objects().iter().filter(|row|ids.contains(&row.id)&&!scene.instances.contains_key(&row.id)) {
        if let Some(session_rust::Geometry::Plane(source))=&row.original {push!(planes,row.id,source.to_proto());}
        else if let Some(session_rust::Geometry::OBB(source))=&row.original {push!(bboxes,row.id,source.to_proto());}
        else if let Some(cad)=row.cad.as_ref() {push!(breps,row.id,cad.source.to_proto());}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/92j-save-and-open-original-element-records-01.rs"
```

### 2. `src/archive_identity.rs`

Route original Element records through mixed-document archives.

**Locate this existing block:**

```rust
pub(crate) fn objects(objects:&mut proto::Objects,names:&HashMap<String,String>) {
    macro_rules! remap {($field:ident)=>{for object in &mut objects.$field {if let Some(guid)=names.get(&object.guid) {object.guid=guid.clone();}}};}
    remap!(planes);remap!(bboxes);remap!(meshes);remap!(lines);remap!(points);remap!(polylines);remap!(nurbscurves);remap!(nurbssurfaces);remap!(breps);remap!(pointclouds);remap!(instances);remap!(sheets);
}
pub(crate) fn tree(node:&mut proto::TreeNode,names:&HashMap<String,String>) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/92j-save-and-open-original-element-records-02.rs"
```

### 3. `src/document_source.rs`

Route original Element records through mixed-document archives.

Replace the entire file with:

```rust
use session_rust::{proto,Session,Xform};
use prost::Message;
use std::{collections::{HashMap,HashSet},rc::Rc};

pub(crate) struct Source {pub document:Rc<Session>,pub origin:Rc<crate::origin::Origin>,pub tree:Option<proto::TreeNode>,pub world:HashMap<String,Xform>,pub sheets:Vec<Rc<crate::sheet_source::Prepared>>}
pub(crate) fn prepare(bytes:&[u8],place:&Xform,point_px:f32)->Result<Source,String> {
    if bytes.len()>crate::archive::LIMIT {return Err("Session exceeds the 512 MiB file limit".into());}
    let message=proto::Session::decode(bytes).map_err(|e|e.to_string())?;
    crate::source_validate::session(&message)?;
    supported(&message)?;
    if !crate::placement::valid(&place.m)||!point_px.is_finite() {return Err("Invalid document placement or point size".into());}
    let tree=message.tree.as_ref().and_then(|tree|tree.root.clone());
    let mut guids=HashSet::new();let objects=message.objects.as_ref().ok_or("Session has no objects")?;
    macro_rules! identity {($field:ident)=>{for object in &objects.$field {if object.guid.is_empty()||!guids.insert(object.guid.as_str()) {return Err("Source GUIDs must be distinct within a document".into());}}};}
    identity!(elements);identity!(planes);identity!(bboxes);identity!(meshes);identity!(points);identity!(lines);identity!(polylines);identity!(nurbscurves);identity!(nurbssurfaces);identity!(breps);identity!(pointclouds);identity!(instances);identity!(sheets);
    if let Some(root)=tree.as_ref() {crate::hierarchy::validate(root,&guids).map_err(str::to_owned)?;}
    let mut placed=HashSet::new();for entry in &message.xforms {if !placed.insert(entry.guid.as_str())||!crate::placement::valid(&entry.xform.as_ref().ok_or("Missing placement matrix")?.matrix) {return Err("Invalid source placement".into());}}
    let sheets=objects.sheets.iter().cloned().map(|sheet|crate::sheet_source::Prepared::new(Rc::new(sheet)).map(Rc::new).map_err(str::to_owned)).collect::<Result<Vec<_>,_>>()?;
    let origin=Rc::new(crate::origin::Origin::new(&message,bytes));
    let document=Rc::new(Session::from_proto(message).map_err(|e|e.to_string())?);
    let world=crate::nested_placement::prepare(&document,place).map_err(str::to_owned)?;
    Ok(Source {document,origin,tree,world,sheets})
}

fn supported(source:&proto::Session)->Result<(),String> {
    crate::instance_plan::validate(source)?;
    if !source.interactions.is_empty() {return Err("Interactions arrive with element features".into());}
    let objects=source.objects.as_ref().ok_or("Session has no objects")?;
    if !objects.components.is_empty() {return Err("This subject cannot open assemblies yet".into());}
    Ok(())
}
```

### 4. `src/document_geometry.rs`

Route original Element records through mixed-document archives.

**Locate this existing block:**

```rust
    let mut owners=Vec::new();let mut shapes=crate::cad_shape_cache::Cache::default();
    macro_rules! add {($source:expr,$insert:expr)=>{{let source=$source;let guid=source.guid().to_owned();let label=source.name.clone();let model=crate::nested_placement::model(&loaded.world,place,&guid);let id=$insert.map_err(str::to_owned)?;scene.place_following(id,model).map_err(str::to_owned)?;owners.push((guid,id,label));id}};}
    for source in &loaded.document.objects.meshes {
        let mut prepared=crate::prepared::PreparedMesh::from_shared(source.clone()).map_err(str::to_owned)?;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/92j-save-and-open-original-element-records-05.rs"
```

## <a id="checkpoint-92k-own-independent-element-tool-state"></a>13. Own independent Element tool state

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 13 of 17</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/editor.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original ordered picks</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">typed Element and attribu…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one history edit</text>
</svg>
</div>

**What this code does:** Own independent Element tool state.

### 1. `src/editor.rs`

Own Element draft state and cleanup before routing its commands.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            }
            Action::ToolPoint(point) => return self.place_tool_point(point),
            Action::ToolHover(point) => { if let Some(draft) = self.draft.as_mut() { draft.snapped=None; draft.hover(point)?; } return self.update_edit_preview(); }
            Action::CancelTool => {if let Some(before)=self.draft.as_ref().and_then(|d|d.project.as_ref()).map(|p|p.before.clone()).or_else(||self.draft.as_ref().and_then(|d|d.extend.as_ref()).map(|e|e.before.clone())){self.select_objects(before,false);}if let Some(before)=self.draft.as_ref().and_then(|d|d.gather.as_ref()).map(|g|g.before.clone()){self.select_objects(before,false);}let changed=self.edit_preview.take().is_some()|self.transform_preview.take().is_some(); self.draft=None;self.plane_menu=false;self.plane_only=false; self.tool_status="Tool cancelled.".into(); return Ok(if changed { Change::Preview } else { Change::View }); }
            Action::Picked(id) => self.select_objects(id, false),
            Action::PickedMany { target, additive } => {
                if self.pick_mode != crate::pick_sources::Mode::Object { return self.apply(Action::PickedTarget(target)); }
```

Replace it with:

```rust
            }
            Action::ToolPoint(point) => return self.place_tool_point(point),
            Action::ToolHover(point) => { if let Some(draft) = self.draft.as_mut() { draft.snapped=None; draft.hover(point)?; } return self.update_edit_preview(); }
            Action::CancelTool => {self.element=None;if let Some(before)=self.draft.as_ref().and_then(|d|d.project.as_ref()).map(|p|p.before.clone()).or_else(||self.draft.as_ref().and_then(|d|d.extend.as_ref()).map(|e|e.before.clone())){self.select_objects(before,false);}if let Some(before)=self.draft.as_ref().and_then(|d|d.gather.as_ref()).map(|g|g.before.clone()){self.select_objects(before,false);}let changed=self.edit_preview.take().is_some()|self.transform_preview.take().is_some(); self.draft=None;self.plane_menu=false;self.plane_only=false; self.tool_status="Tool cancelled.".into(); return Ok(if changed { Change::Preview } else { Change::View }); }
            Action::Picked(id) => self.select_objects(id, false),
            Action::PickedMany { target, additive } => {
                if self.pick_mode != crate::pick_sources::Mode::Object { return self.apply(Action::PickedTarget(target)); }
```

**Locate this block:**

```rust
            Action::CancelTrim=>{if let Some(trim)=self.trim.take(){self.select_objects(trim.before,false)}self.tool_status="Trim cancelled · nothing changed".into();return Ok(Change::Preview)},
            Action::Query(query)=>{let ids=query.found(&self.scene).map_err(|message|self.primitive_failure(message))?;self.select_objects(ids,false);return Ok(Change::View)},
            Action::Arrowhead(head)=>return self.curve_heads(head),
            Action::Escape=>{if self.draft.is_some()||self.split.is_some()||self.gizmo_edit.is_some(){self.apply_action(Action::GizmoCancel)?;return self.apply_action(Action::CancelTool)}self.select_objects([],false);return Ok(Change::View)},
            Action::Measure(kind)=>return self.measure_selection(kind),
            Action::MeasureDistance(text)=>return self.begin_measure_distance(&text),
            Action::BoundingBox=>return self.bounding_box(),
```

Replace it with:

```rust
            Action::CancelTrim=>{if let Some(trim)=self.trim.take(){self.select_objects(trim.before,false)}self.tool_status="Trim cancelled · nothing changed".into();return Ok(Change::Preview)},
            Action::Query(query)=>{let ids=query.found(&self.scene).map_err(|message|self.primitive_failure(message))?;self.select_objects(ids,false);return Ok(Change::View)},
            Action::Arrowhead(head)=>return self.curve_heads(head),
            Action::Escape=>{if self.element.is_some()||self.draft.is_some()||self.split.is_some()||self.gizmo_edit.is_some(){self.apply_action(Action::GizmoCancel)?;return self.apply_action(Action::CancelTool)}self.select_objects([],false);return Ok(Change::View)},
            Action::Measure(kind)=>return self.measure_selection(kind),
            Action::MeasureDistance(text)=>return self.begin_measure_distance(&text),
            Action::BoundingBox=>return self.bounding_box(),
```

**Locate this block:**

```rust
        let draft=self.draft.as_ref().ok_or("No transform tool is running")?;let mut next=draft.transform.clone().ok_or("No transform tool is running")?;
        let outcome=next.text(text,draft.cursor,self.construction_plane).map_err(|message|self.primitive_failure(message))?;self.transform_outcome(next,outcome)
    }
    pub fn highlighted_objects(&self)->Vec<ObjectId>{if let Some(trim)=&self.trim{return if trim.phase==crate::trim_draft::Phase::Parts{Vec::new()}else if trim.phase==crate::trim_draft::Phase::Targets{trim.targets.iter().map(|o|o.id).collect()}else{trim.cutters.iter().map(|o|o.id).collect()}}if let Some(split)=&self.split{return std::iter::once(split.target.id).chain(split.cutters.iter().map(|o|o.id)).collect()}if let Some(extend)=self.draft.as_ref().and_then(|d|d.extend.as_ref()){return extend.boundaries.iter().map(|o|o.id).collect()}self.draft.as_ref().and_then(|d|d.gather.as_ref()).map_or_else(||self.selected_objects(),|g|{g.progress.picked().iter().filter_map(|row|g.curves.iter().find(|(id,_)|id.value()==*row).map(|(id,_)|*id)).collect()})}
    pub fn gathering_curves(&self)->bool{self.draft.as_ref().is_some_and(|d|d.plane_points.is_none()&&d.gather.as_ref().is_some_and(|g|g.picks()))}
    fn gather_adopt(&mut self,next:crate::gather_draft::Draft,made:Option<crate::gather_input::Made>)->Result<Change,&'static str>{
        next.valid(&self.scene).map_err(|message|self.primitive_failure(message))?;
```

Replace it with:

```rust
        let draft=self.draft.as_ref().ok_or("No transform tool is running")?;let mut next=draft.transform.clone().ok_or("No transform tool is running")?;
        let outcome=next.text(text,draft.cursor,self.construction_plane).map_err(|message|self.primitive_failure(message))?;self.transform_outcome(next,outcome)
    }
    pub fn highlighted_objects(&self)->Vec<ObjectId>{if let Some(e)=&self.element{return e.ids()}if let Some(trim)=&self.trim{return if trim.phase==crate::trim_draft::Phase::Parts{Vec::new()}else if trim.phase==crate::trim_draft::Phase::Targets{trim.targets.iter().map(|o|o.id).collect()}else{trim.cutters.iter().map(|o|o.id).collect()}}if let Some(split)=&self.split{return std::iter::once(split.target.id).chain(split.cutters.iter().map(|o|o.id)).collect()}if let Some(extend)=self.draft.as_ref().and_then(|d|d.extend.as_ref()){return extend.boundaries.iter().map(|o|o.id).collect()}self.draft.as_ref().and_then(|d|d.gather.as_ref()).map_or_else(||self.selected_objects(),|g|{g.progress.picked().iter().filter_map(|row|g.curves.iter().find(|(id,_)|id.value()==*row).map(|(id,_)|*id)).collect()})}
    pub fn gathering_curves(&self)->bool{self.draft.as_ref().is_some_and(|d|d.plane_points.is_none()&&d.gather.as_ref().is_some_and(|g|g.picks()))}
    fn gather_adopt(&mut self,next:crate::gather_draft::Draft,made:Option<crate::gather_input::Made>)->Result<Change,&'static str>{
        next.valid(&self.scene).map_err(|message|self.primitive_failure(message))?;
```

**Locate this block:**

```rust
impl Default for Editor {
    fn default() -> Self {
        Self {
            arctic:false,outlines:false,clipping_enabled:true,clipping_solid:true,interaction_epoch:0,lasso:None,trim:None,last_text_height:0.,mark:None,unit:crate::measure_mark::Unit::Millimeters,split:None,canvas_drag:None, gizmo_edit:None, gizmo_hover:None, edit_preview: None, snapping: Default::default(), snap_index: None,
            transform_preview:None,draft: None, tool_status: String::new(), construction_plane:None, plane_menu:false, plane_only:false,
            scene: Scene::demo(),
            selected: None, selected_part: None, pick_mode: Default::default(), objects: Default::default(),
```

Replace it with:

```rust
impl Default for Editor {
    fn default() -> Self {
        Self {
            element:None,element_attributes:false,element_interactions:true,arctic:false,outlines:false,clipping_enabled:true,clipping_solid:true,interaction_epoch:0,lasso:None,trim:None,last_text_height:0.,mark:None,unit:crate::measure_mark::Unit::Millimeters,split:None,canvas_drag:None, gizmo_edit:None, gizmo_hover:None, edit_preview: None, snapping: Default::default(), snap_index: None,
            transform_preview:None,draft: None, tool_status: String::new(), construction_plane:None, plane_menu:false, plane_only:false,
            scene: Scene::demo(),
            selected: None, selected_part: None, pick_mode: Default::default(), objects: Default::default(),
```

**Locate this block:**

```rust
}

pub struct Editor {
    pub arctic:bool,
    pub outlines:bool,
    pub clipping_enabled:bool,pub clipping_solid:bool,
```

Replace it with:

```rust
}

pub struct Editor {
    pub element:Option<crate::element_draft::Draft>,
    pub element_attributes:bool,
    pub element_interactions:bool,
    pub arctic:bool,
    pub outlines:bool,
    pub clipping_enabled:bool,pub clipping_solid:bool,
```

## <a id="checkpoint-92l-commit-one-element-creation-and-retry-errors"></a>14. Commit one Element creation and retry errors

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 14 of 17</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/editor.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original ordered picks</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">typed Element and attribu…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one history edit</text>
</svg>
</div>

**What this code does:** Commit one Element creation and retry errors.

### 1. `src/editor.rs`

Connect staged Element creation, named input and ordered picks.

**Locate this existing block:**

```rust
    pub fn snap_usage(&self) -> [u64; 2] { self.snap_index.as_ref().map_or([0,0], |index| [index.builds,index.visits as u64]) }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        let action=match action{Action::Touch(actions)=>{let mut change=Change::View;for action in actions{let next=self.apply(action)?;if next!=Change::View{change=next;}}return Ok(change)},action=>action};
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/92l-commit-one-element-creation-and-retry-errors-01.rs"
```

## <a id="checkpoint-92m-route-current-element-commands-and-feature-visibility"></a>15. Route Element commands and independent visibility

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 15 of 17</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/editor.rs, src/commands…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original ordered picks</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">typed Element and attribu…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one history edit</text>
</svg>
</div>

**What this code does:** Toggle Element Attributes and Element Interactions independently.

### 1. `src/editor.rs`

Route Element actions and current feature presentation.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        if matches!(&action, Action::Close | Action::Undo | Action::Redo | Action::Adopt {..} | Action::Live(_) | Action::Import(_) | Action::Replace(_) | Action::ImportAt(_, _) | Action::ReplaceAt(_, _)) { self.draft = None;self.plane_menu=false;self.plane_only=false; }
        if self.split.is_some()&&!matches!(&action,Action::Split|Action::SplitPick(_)|Action::CancelSplit|Action::TreePick{..}|Action::Zoom(_)|Action::ZoomAt{..}|Action::PanScreen{..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::NamedView(_)|Action::Isometric|Action::ResetView|Action::Background|Action::NormalView(_)|Action::Grid(_)|Action::ConstructionPlane(_)|Action::PlaneMenu|Action::Plane3){self.split=None;}
        match action {
            Action::ViewOutline(_)=>unreachable!("Outline presentation returned before editor mutation"),
            Action::Arctic(_)=>unreachable!("Arctic presentation returned before editor mutation"),
            Action::Clipping(request)=>{return self.clipping_request(request)},
```

Replace it with:

```rust
        if matches!(&action, Action::Close | Action::Undo | Action::Redo | Action::Adopt {..} | Action::Live(_) | Action::Import(_) | Action::Replace(_) | Action::ImportAt(_, _) | Action::ReplaceAt(_, _)) { self.draft = None;self.plane_menu=false;self.plane_only=false; }
        if self.split.is_some()&&!matches!(&action,Action::Split|Action::SplitPick(_)|Action::CancelSplit|Action::TreePick{..}|Action::Zoom(_)|Action::ZoomAt{..}|Action::PanScreen{..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::NamedView(_)|Action::Isometric|Action::ResetView|Action::Background|Action::NormalView(_)|Action::Grid(_)|Action::ConstructionPlane(_)|Action::PlaneMenu|Action::Plane3){self.split=None;}
        match action {
            Action::ElementBegin{kind,text}=>return self.element_begin(kind,&text),
            Action::ElementText(text)=>return self.element_text(&text),
            Action::ElementPick(id)=>return self.element_pick(id),
            Action::ElementAttributes(on)|Action::ElementInteractions(on)=>{
                let (shown,ids,label)=if matches!(action,Action::ElementInteractions(_)) {
                    (&mut self.element_interactions,crate::element_adopt::interaction_ids(&self.scene),"Interactions")
                }else{(&mut self.element_attributes,crate::element_adopt::attribute_ids(&self.scene),"Attributes")};
                *shown=on.unwrap_or(!*shown);
                std::rc::Rc::make_mut(&mut self.scene.view).hide(&ids,!*shown);
                self.tool_status=format!("Element {label} {}",if *shown{"On"}else{"Off"});
                return Ok(Change::Scene)
            },
            Action::ViewOutline(_)=>unreachable!("Outline presentation returned before editor mutation"),
            Action::Arctic(_)=>unreachable!("Arctic presentation returned before editor mutation"),
            Action::Clipping(request)=>{return self.clipping_request(request)},
```

**Locate this block:**

```rust

    pub fn drawing_plane(&self)->crate::cplane::CPlane{self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())))}
    pub fn tool_line(&self, text: &str) -> Option<Result<Action, &'static str>> {
        if self.draft.as_ref().is_some_and(|d|d.tool.name()=="Clipping Plane"){if let Some(mode)=crate::clipping_geometry::Mode::parse(text.trim()){return Some(Ok(Action::Clipping(crate::clipping_input::Request::Begin(mode))))}}
        if self.trim.is_some(){return match text.trim(){""=>Some(Ok(Action::Trim)),v if v.eq_ignore_ascii_case("Trim")=>Some(Ok(Action::Trim)),v if v.eq_ignore_ascii_case("Escape")||v.eq_ignore_ascii_case("Esc")=>Some(Ok(Action::CancelTrim)),_=>None}}
        if self.mark.is_some()&&text.trim().eq_ignore_ascii_case("Escape"){return Some(Ok(Action::CancelTool))}
```

Replace it with:

```rust

    pub fn drawing_plane(&self)->crate::cplane::CPlane{self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())))}
    pub fn tool_line(&self, text: &str) -> Option<Result<Action, &'static str>> {
        if self.element.is_some(){return Some(Ok(if text.trim().eq_ignore_ascii_case("Escape"){Action::CancelTool}else{Action::ElementText(text.into())}))}
        if self.draft.as_ref().is_some_and(|d|d.tool.name()=="Clipping Plane"){if let Some(mode)=crate::clipping_geometry::Mode::parse(text.trim()){return Some(Ok(Action::Clipping(crate::clipping_input::Request::Begin(mode))))}}
        if self.trim.is_some(){return match text.trim(){""=>Some(Ok(Action::Trim)),v if v.eq_ignore_ascii_case("Trim")=>Some(Ok(Action::Trim)),v if v.eq_ignore_ascii_case("Escape")||v.eq_ignore_ascii_case("Esc")=>Some(Ok(Action::CancelTrim)),_=>None}}
        if self.mark.is_some()&&text.trim().eq_ignore_ascii_case("Escape"){return Some(Ok(Action::CancelTool))}
```

**Locate this block:**

```rust
use crate::{background::Background, camera::Camera, history::History, scene::{ObjectId, Scene}};

pub enum Action {
    Arctic(Option<bool>),
    ViewOutline(Option<bool>),
    Clipping(crate::clipping_input::Request),
```

Replace it with:

```rust
use crate::{background::Background, camera::Camera, history::History, scene::{ObjectId, Scene}};

pub enum Action {
    ElementBegin{kind:crate::element_input::Kind,text:String},ElementText(String),ElementPick(Option<ObjectId>),ElementAttributes(Option<bool>),ElementInteractions(Option<bool>),
    Arctic(Option<bool>),
    ViewOutline(Option<bool>),
    Clipping(crate::clipping_input::Request),
```

### 2. `src/commands.rs`

Register current Element commands and strict argument parsing.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    if let Some(action)=crate::transform_commands::action(name,&words[count..]){return action.map(Command::Editor);}
    if let Some(action)=crate::gather_registry::command(name,&words[count..].join(" ")){return action.map(Command::Editor);}
    if let Some(action)=crate::primitive_registry::command(name,&words[count..].join(" ")) {return Ok(Command::Editor(action));}
    if name=="View Outline"{let on=match &words[count..]{[]=>None,[word]if word.eq_ignore_ascii_case("On")=>Some(true),[word]if word.eq_ignore_ascii_case("Off")=>Some(false),_=>return Err("View Outline (On Off)".into())};return Ok(Command::Editor(Action::ViewOutline(on)));}
    if name=="Arctic"{let on=match &words[count..]{[]=>None,[word]if word.eq_ignore_ascii_case("On")=>Some(true),[word]if word.eq_ignore_ascii_case("Off")=>Some(false),_=>return Err("Arctic (On Off)".into())};return Ok(Command::Editor(Action::Arctic(on)));}
    if name=="Clipping Plane"{return crate::clipping_input::parse(&words[count..]).map(|r|Command::Editor(Action::Clipping(r)));}
```

Replace it with:

```rust
    if let Some(action)=crate::transform_commands::action(name,&words[count..]){return action.map(Command::Editor);}
    if let Some(action)=crate::gather_registry::command(name,&words[count..].join(" ")){return action.map(Command::Editor);}
    if let Some(action)=crate::primitive_registry::command(name,&words[count..].join(" ")) {return Ok(Command::Editor(action));}
    if let Some(kind)=[("Element Beam",crate::element_input::Kind::Beam),("Element Column",crate::element_input::Kind::Column),("Element Plate",crate::element_input::Kind::Plate),("Element Support",crate::element_input::Kind::Support),("Element Beam Variable",crate::element_input::Kind::Variable),("Element Block",crate::element_input::Kind::Block)].into_iter().find_map(|(n,k)|(n==name).then_some(k)){let text=words[count..].join(" ");kind.read(&text)?;return Ok(Command::Editor(Action::ElementBegin{kind,text}));}
    if matches!(name,"Element Attributes"|"Element Interactions") {
        let on=crate::element_input::visibility(&words[count..])?;
        return Ok(Command::Editor(if name=="Element Attributes"{Action::ElementAttributes(on)}else{Action::ElementInteractions(on)}));
    }
    if name=="View Outline"{let on=match &words[count..]{[]=>None,[word]if word.eq_ignore_ascii_case("On")=>Some(true),[word]if word.eq_ignore_ascii_case("Off")=>Some(false),_=>return Err("View Outline (On Off)".into())};return Ok(Command::Editor(Action::ViewOutline(on)));}
    if name=="Arctic"{let on=match &words[count..]{[]=>None,[word]if word.eq_ignore_ascii_case("On")=>Some(true),[word]if word.eq_ignore_ascii_case("Off")=>Some(false),_=>return Err("Arctic (On Off)".into())};return Ok(Command::Editor(Action::Arctic(on)));}
    if name=="Clipping Plane"{return crate::clipping_input::parse(&words[count..]).map(|r|Command::Editor(Action::Clipping(r)));}
```

**Locate this block:**

```rust

pub type Factory = fn() -> Action;
pub const ACTIONS: &[(&str, Factory)] = &[
    ("View Outline",||Action::ViewOutline(None)),
    ("Arctic",||Action::Arctic(None)),
    ("Select By Name",||Action::Query(crate::selection_query::Query::Name(String::new()))),
```

Replace it with:

```rust

pub type Factory = fn() -> Action;
pub const ACTIONS: &[(&str, Factory)] = &[
    ("Element Beam Variable",||Action::ElementBegin{kind:crate::element_input::Kind::Variable,text:String::new()}),
    ("Element Beam",||Action::ElementBegin{kind:crate::element_input::Kind::Beam,text:String::new()}),
    ("Element Column",||Action::ElementBegin{kind:crate::element_input::Kind::Column,text:String::new()}),
    ("Element Plate",||Action::ElementBegin{kind:crate::element_input::Kind::Plate,text:String::new()}),
    ("Element Support",||Action::ElementBegin{kind:crate::element_input::Kind::Support,text:String::new()}),
    ("Element Block",||Action::ElementBegin{kind:crate::element_input::Kind::Block,text:String::new()}),
    ("Element Attributes",||Action::ElementAttributes(None)),
    ("Element Interactions",||Action::ElementInteractions(None)),
    ("View Outline",||Action::ViewOutline(None)),
    ("Arctic",||Action::Arctic(None)),
    ("Select By Name",||Action::Query(crate::selection_query::Query::Name(String::new()))),
```

## <a id="checkpoint-92n-pick-elements-through-the-real-command-dock"></a>16. Pick Elements through the real command dock

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 16 of 17</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/browser.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original ordered picks</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">typed Element and attribu…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one history edit</text>
</svg>
</div>

**What this code does:** Pick Elements through the real command dock.

### 1. `src/browser.rs`

Route actual browser original picks, command results and Element parameter controls.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        panel.lasso=editor.lasso.as_ref().map_or_else(Vec::new,|l|l.points.iter().map(|p|[(p.0/l.density) as f32,(p.1/l.density) as f32]).collect());
        panel.tree.refresh(&editor.scene,editor.selected_objects());
        if let Err(error) = renderer.set_preview(editor.draft.as_ref()) { panel.result(error); }
        renderer.set_pick_mode(if editor.trimming_picks()||editor.split.is_some()||editor.editing_boundaries()||editor.gathering_curves(){crate::pick_sources::Mode::Object}else{editor.pick_mode});
        if highlight_changed {
            if let Err(error) = renderer.set_highlight(editor.display_scene(), editor.selection())
                .and_then(|()| renderer.select_objects(editor.display_scene(), &editor.highlighted_objects())) { report(error); return; }
```

Replace it with:

```rust
        panel.lasso=editor.lasso.as_ref().map_or_else(Vec::new,|l|l.points.iter().map(|p|[(p.0/l.density) as f32,(p.1/l.density) as f32]).collect());
        panel.tree.refresh(&editor.scene,editor.selected_objects());
        if let Err(error) = renderer.set_preview(editor.draft.as_ref()) { panel.result(error); }
        renderer.set_pick_mode(if editor.element.is_some()||editor.trimming_picks()||editor.split.is_some()||editor.editing_boundaries()||editor.gathering_curves(){crate::pick_sources::Mode::Object}else{editor.pick_mode});
        if highlight_changed {
            if let Err(error) = renderer.set_highlight(editor.display_scene(), editor.selection())
                .and_then(|()| renderer.select_objects(editor.display_scene(), &editor.highlighted_objects())) { report(error); return; }
```

**Locate this block:**

```rust
                }
            }
        }
        if editor.plane_menu {panel.tool(editor.tool_status.clone(),crate::drawing_input::buttons());}
        else if let Some(trim)=&editor.trim{panel.tool(trim.prompt(),vec![("Enter".into(),"Trim".into()),("Cancel".into(),"Escape".into())]);}
        else if let Some(split)=&editor.split{panel.tool(split.prompt(),vec![("Split".into(),"Split".into()),("Cancel".into(),"Escape".into())]);}
        else if let Some(draft) = editor.draft.as_ref() {let mut buttons=draft.tool.buttons();if draft.text.is_some(){buttons.push(("Height".into(),"Height".into()));}buttons.extend(crate::drawing_input::buttons());panel.tool(draft.prompt(),buttons);}
```

Replace it with:

```rust
                }
            }
        }
        if let Some(element)=&editor.element{panel.tool(element.prompt(),element.buttons());}
        else if editor.plane_menu {panel.tool(editor.tool_status.clone(),crate::drawing_input::buttons());}
        else if let Some(trim)=&editor.trim{panel.tool(trim.prompt(),vec![("Enter".into(),"Trim".into()),("Cancel".into(),"Escape".into())]);}
        else if let Some(split)=&editor.split{panel.tool(split.prompt(),vec![("Split".into(),"Split".into()),("Cancel".into(),"Escape".into())]);}
        else if let Some(draft) = editor.draft.as_ref() {let mut buttons=draft.tool.buttons();if draft.text.is_some(){buttons.push(("Height".into(),"Height".into()));}buttons.extend(crate::drawing_input::buttons());panel.tool(draft.prompt(),buttons);}
```

**Locate this block:**

```rust
            highlight_changed |= matches!(&action, Action::SelectAll);
            let group_action=matches!(&action,Action::AddGroup(_));let graph_action=matches!(&action,Action::AddEdge);
            let rename_action=matches!(&action,Action::LayerEdit {edit:crate::layer_edit::Edit::Rename(_),..});
            let tool_action = matches!(&action, Action::AddGroup(_) | Action::AddEdge | Action::SelectLasso | Action::LassoRelease{..} | Action::LassoCancel | Action::TextBegin(_) | Action::Trim | Action::TrimPick(_) | Action::TrimScreen{commit:true,..} | Action::CancelTrim | Action::Arrowhead(_) | Action::Explode | Action::Query(_) | Action::Measure(_) | Action::MeasureDistance(_) | Action::BoundingBox | Action::Split | Action::SplitPick(_) | Action::CancelSplit | Action::BeginTool(_) | Action::GatherBegin{..} | Action::GatherPick(_) | Action::Project(_) | Action::ExtendBegin{..} | Action::ExtendPick(_) | Action::Interval(_) | Action::ToolPoint(_) | Action::CancelTool | Action::Snap(_) | Action::GizmoRelease {..} | Action::GizmoCancel | Action::GizmoNumber(_) | Action::ToolScreen { commit:true,.. });
            if matches!(&action, Action::CancelTool | Action::CancelSplit | Action::CancelTrim) { panel.cancel_input(); }
            if matches!(&action, Action::Close | Action::UnloadSources | Action::Live(_) | Action::Undo | Action::Redo
                | Action::Adopt {..} | Action::Replace(_) | Action::ReplaceAt(_, _)) { reload.borrow_mut().cancel(); }
```

Replace it with:

```rust
            highlight_changed |= matches!(&action, Action::SelectAll);
            let group_action=matches!(&action,Action::AddGroup(_));let graph_action=matches!(&action,Action::AddEdge);
            let rename_action=matches!(&action,Action::LayerEdit {edit:crate::layer_edit::Edit::Rename(_),..});
            let tool_action = matches!(&action, Action::ElementBegin{..}|Action::ElementText(_)|Action::ElementPick(_)|Action::ElementAttributes(_)| Action::AddGroup(_) | Action::AddEdge | Action::SelectLasso | Action::LassoRelease{..} | Action::LassoCancel | Action::TextBegin(_) | Action::Trim | Action::TrimPick(_) | Action::TrimScreen{commit:true,..} | Action::CancelTrim | Action::Arrowhead(_) | Action::Explode | Action::Query(_) | Action::Measure(_) | Action::MeasureDistance(_) | Action::BoundingBox | Action::Split | Action::SplitPick(_) | Action::CancelSplit | Action::BeginTool(_) | Action::GatherBegin{..} | Action::GatherPick(_) | Action::Project(_) | Action::ExtendBegin{..} | Action::ExtendPick(_) | Action::Interval(_) | Action::ToolPoint(_) | Action::CancelTool | Action::Snap(_) | Action::GizmoRelease {..} | Action::GizmoCancel | Action::GizmoNumber(_) | Action::ToolScreen { commit:true,.. });
            if matches!(&action, Action::CancelTool | Action::CancelSplit | Action::CancelTrim) { panel.cancel_input(); }
            if matches!(&action, Action::Close | Action::UnloadSources | Action::Live(_) | Action::Undo | Action::Redo
                | Action::Adopt {..} | Action::Replace(_) | Action::ReplaceAt(_, _)) { reload.borrow_mut().cancel(); }
```

**Locate this block:**

```rust
            Some(Action::Touch(actions))=> {
                let mut applied=Vec::new();
                for action in actions {match action {
                    Action::Pick(screen)=>{if editor.trimming_parts(){let r=pointer_canvas.get_bounding_client_rect();applied.push(Action::TrimScreen{screen,size:[r.width(),r.height()],commit:true});}else if editor.trimming_picks()||editor.split.is_some()||editor.editing_boundaries()||editor.gathering_curves(){pick_additive.set(false);pick_remove.set(false);if let Err(error)=renderer.request_pick(screen,editor.camera.uniform()){report(error)}}else if editor.draft.is_some() {let r=pointer_canvas.get_bounding_client_rect();applied.push(Action::ToolScreen {screen,size:[r.width(),r.height()],commit:true});} else {pick_additive.set(false);pick_remove.set(false);if let Err(error)=renderer.request_pick(screen,editor.camera.uniform()) {report(error);}}},
                    action=>{renderer.cancel_pick();applied.push(action);}
                }}Some(Action::Touch(applied))
            },
```

Replace it with:

```rust
            Some(Action::Touch(actions))=> {
                let mut applied=Vec::new();
                for action in actions {match action {
                    Action::Pick(screen)=>{if editor.trimming_parts(){let r=pointer_canvas.get_bounding_client_rect();applied.push(Action::TrimScreen{screen,size:[r.width(),r.height()],commit:true});}else if editor.element.is_some()||editor.trimming_picks()||editor.split.is_some()||editor.editing_boundaries()||editor.gathering_curves(){pick_additive.set(false);pick_remove.set(false);if let Err(error)=renderer.request_pick(screen,editor.camera.uniform()){report(error)}}else if editor.draft.is_some() {let r=pointer_canvas.get_bounding_client_rect();applied.push(Action::ToolScreen {screen,size:[r.width(),r.height()],commit:true});} else {pick_additive.set(false);pick_remove.set(false);if let Err(error)=renderer.request_pick(screen,editor.camera.uniform()) {report(error);}}},
                    action=>{renderer.cancel_pick();applied.push(action);}
                }}Some(Action::Touch(applied))
            },
```

**Locate this block:**

```rust
            }
        } else { match action {
            Some(Action::Pick(screen)) if editor.trimming_parts()=>{let r=pointer_canvas.get_bounding_client_rect();Some(Action::TrimScreen{screen,size:[r.width(),r.height()],commit:true})},
            Some(Action::Pick(screen)) if editor.trimming_picks()||editor.split.is_some()||editor.editing_boundaries()||editor.gathering_curves()=>{pick_additive.set(false);pick_remove.set(false);if let Err(error)=renderer.request_pick(screen,editor.camera.uniform()){report(error)}None},
            Some(Action::Pick(screen)) if editor.draft.is_some() => {
                Some(Action::ToolScreen { screen,size:[pointer_canvas.get_bounding_client_rect().width(),pointer_canvas.get_bounding_client_rect().height()],commit:true })
            }
```

Replace it with:

```rust
            }
        } else { match action {
            Some(Action::Pick(screen)) if editor.trimming_parts()=>{let r=pointer_canvas.get_bounding_client_rect();Some(Action::TrimScreen{screen,size:[r.width(),r.height()],commit:true})},
            Some(Action::Pick(screen)) if editor.element.is_some()||editor.trimming_picks()||editor.split.is_some()||editor.editing_boundaries()||editor.gathering_curves()=>{pick_additive.set(false);pick_remove.set(false);if let Err(error)=renderer.request_pick(screen,editor.camera.uniform()){report(error)}None},
            Some(Action::Pick(screen)) if editor.draft.is_some() => {
                Some(Action::ToolScreen { screen,size:[pointer_canvas.get_bounding_client_rect().width(),pointer_canvas.get_bounding_client_rect().height()],commit:true })
            }
```

**Locate this block:**

```rust
            match renderer.take_pick(editor.camera.uniform()) {
                Some(Ok(hit)) => {
                    let target=hit.and_then(|hit|renderer.resolve_pick(&editor.scene,hit));
                    if editor.trimming_picks(){Some(Action::TrimPick(target.map(|t|t.owner)))}else if editor.split.is_some(){Some(Action::SplitPick(target.map(|t|t.owner)))}else if editor.editing_boundaries(){Some(Action::ExtendPick(target.map(|t|t.owner)))}else if editor.gathering_curves(){Some(Action::GatherPick(target.map(|t|t.owner)))}
                    else if editor.canvas_drag.is_some() {Some(Action::CanvasHit(target.map(|t|t.owner)))}
                    else if pick_remove.get() {Some(Action::SelectRows {ids:target.filter(|t|t.part==crate::pick_sources::Part::Object).map(|t|t.owner).into_iter().collect(),add:false,remove:true})}
                    else {Some(Action::PickedMany {target,additive:pick_additive.get()})}
```

Replace it with:

```rust
            match renderer.take_pick(editor.camera.uniform()) {
                Some(Ok(hit)) => {
                    let target=hit.and_then(|hit|renderer.resolve_pick(&editor.scene,hit));
                    if editor.element.is_some(){Some(Action::ElementPick(target.map(|t|t.owner)))}else if editor.trimming_picks(){Some(Action::TrimPick(target.map(|t|t.owner)))}else if editor.split.is_some(){Some(Action::SplitPick(target.map(|t|t.owner)))}else if editor.editing_boundaries(){Some(Action::ExtendPick(target.map(|t|t.owner)))}else if editor.gathering_curves(){Some(Action::GatherPick(target.map(|t|t.owner)))}
                    else if editor.canvas_drag.is_some() {Some(Action::CanvasHit(target.map(|t|t.owner)))}
                    else if pick_remove.get() {Some(Action::SelectRows {ids:target.filter(|t|t.part==crate::pick_sources::Part::Object).map(|t|t.owner).into_iter().collect(),add:false,remove:true})}
                    else {Some(Action::PickedMany {target,additive:pick_additive.get()})}
```

## <a id="checkpoint-92o-prove-element-recipes-source-history-and-archives"></a>17. Prove Element recipes source history and archives

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 17 of 17</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/element_tests.rs, src/l…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original ordered picks</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">typed Element and attribu…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one history edit</text>
</svg>
</div>

**What this code does:** Prove Element recipes source history and archives.

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/element_tests.rs`

Copy supplied original Element adapter and archive fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/92o-prove-element-recipes-source-history-and-archives-01.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/lib.rs`

Register the completed Element component.

**Locate this existing block:**

```text

pub mod element_adopt;
```

After that block, append this supplied code:

```text
--8<-- "journey/code/92o-prove-element-recipes-source-history-and-archives-02.rs" skip=24
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/element_beam.rs`

Copy maintained beam solid and command fixtures.

**Locate this existing block:**

```text
        false => Ok(beams),
    }
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/92o-prove-element-recipes-source-history-and-archives-03.rs" skip=3
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/element_column.rs`

Copy maintained column solid and command fixtures.

**Locate this existing block:**

```text
        false => Ok(columns),
    }
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/92o-prove-element-recipes-source-history-and-archives-04.rs" skip=3
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/element_plate.rs`

Copy maintained plate solid and command fixtures.

**Locate this existing block:**

```text
            .collect()),
    }
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/92o-prove-element-recipes-source-history-and-archives-05.rs" skip=3
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/element_support.rs`

Copy maintained support solid and command fixtures.

**Locate this existing block:**

```text
            .collect()),
    }
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/92o-prove-element-recipes-source-history-and-archives-06.rs" skip=3
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/element_beam_variable.rs`

Copy maintained beam variable solid and command fixtures.

**Locate this existing block:**

```text
    Ok(BeamVariable::through(sections, "beam_variable"))
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/92o-prove-element-recipes-source-history-and-archives-07.rs" skip=60
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 8. `src/element_block.rs`

Copy maintained block solid and command fixtures.

**Locate this existing block:**

```text
    Ok(Block::new(loops, "block"))
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/92o-prove-element-recipes-source-history-and-archives-08.rs" skip=38
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 18.4
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 18.4
npm --prefix ../session_tests run course -- save 18.4
```

Continue with [18.5 · Glass, X-ray and shared visibility rules](../../consolidation/93-classify-original-solids-independently-of-clipping.md).
