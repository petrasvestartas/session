# 12.4 · Charged residency and current streamed pages

**Goal:** Connect current pages to the browser renderer. · **Steps:** 31 · **Source:** [app/manifest.rs](../../../../src/app/manifest.rs) · [app/session_io.rs](../../../../src/app/session_io.rs) · [app/stream.rs](../../../../src/app/stream.rs) · [app/range_gate.rs](../../../../src/app/range_gate.rs) · [app/fetch.rs](../../../../src/app/fetch.rs) · [app/loader.rs](../../../../src/app/loader.rs) · [app/live.rs](../../../../src/app/live.rs) (`session_viewer/src/lib.rs`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 12.4 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/page_cache.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/stream_generation.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">31 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/engine/gpu/device.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Reserve page bytes before…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Vertex RGB &amp; Uniforms</text>
  <polygon points="610,56 690,56 610,126" fill="#bfdbfe" stroke="#111111" stroke-width="1.75"/>
  <polygon points="690,56 690,126 610,126" fill="#fde68a" stroke="#111111" stroke-width="1.75"/>
  <circle cx="610" cy="56" r="4" fill="#ef4444"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="590" y="58" font-family="monospace" font-size="9" font-weight="700" fill="#111111">v0</text>
  <circle cx="690" cy="56" r="4" fill="#22c55e"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="696" y="58" font-family="monospace" font-size="9" font-weight="700" fill="#111111">v1</text>
  <circle cx="610" cy="126" r="4" fill="#3b82f6"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="590" y="130" font-family="monospace" font-size="9" font-weight="700" fill="#111111">v2</text>
  <circle cx="690" cy="126" r="4" fill="#f59e0b"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="696" y="130" font-family="monospace" font-size="9" font-weight="700" fill="#111111">v3</text>
  <rect x="720" y="64" width="90" height="54" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="765" y="88" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">view_proj</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="765" y="104" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">mat4x4&lt;f32&gt;</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Charged residency and curre…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Depth32Float &amp; Ray Pick</text>
  <rect x="936" y="54" width="96" height="62" rx="4" fill="#93c5fd" stroke="#111111" stroke-width="1.5" stroke-dasharray="3 3"/>
  <rect x="892" y="74" width="96" height="62" rx="4" fill="#fdba74" stroke="#111111" stroke-width="2"/>
  <line x1="864" y1="138" x2="932" y2="102" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <circle cx="936" cy="100" r="5" fill="#facc15" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="896" y="68" font-family="monospace" font-size="9" font-weight="700" fill="#111111">z=0.2 &lt; z=0.7</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Connect current pages to…</text>
</svg>
</div>

## <a id="checkpoint-68-reserve-page-bytes-before-reading"></a>1. Reserve page bytes before reading

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/page_cache.rs, src/lib.…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">byte budget</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">allocation</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">automatic release</text>
</svg>
</div>

**What this code does:** Reserve page bytes before reading.

### 1. `src/page_cache.rs`

Reserve bytes before allocation and release them with the owning charge.

Create the file and type:

```rust
--8<-- "journey/code/68-reserve-page-bytes-before-reading-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust

pub mod range_source;
```

After that block, type:

```rust
--8<-- "journey/code/68-reserve-page-bytes-before-reading-02.rs" skip=23
```

## <a id="checkpoint-68a-keep-shared-pages-alive-during-eviction"></a>2. Keep shared pages alive during eviction

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/page_cache.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared page</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removed entry</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained external owner</text>
</svg>
</div>

**What this code does:** Keep shared pages alive during eviction.

### 1. `src/page_cache.rs`

Keep page values and their accounting alive across cache removal.

**Locate this existing block:**

```rust
impl Drop for Charge {
    fn drop(&mut self) {self.budget.used.set(self.budget.used.get()-self.bytes);}
}
```

After that block, type:

```rust
--8<-- "journey/code/68a-keep-shared-pages-alive-during-eviction-01.rs" skip=3
```

## <a id="checkpoint-68b-evict-unpinned-pages-in-access-order"></a>3. Evict unpinned pages in access order

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/page_cache.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">required bytes</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">sufficient room</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reserved read</text>
</svg>
</div>

**What this code does:** Evict unpinned pages in access order.

### 1. `src/page_cache.rs`

Reclaim sufficient unpinned payload before reserving a new page.

**Locate this existing block:**

```rust
        self.clock=self.clock.checked_add(1)?;let entry=self.entries.get_mut(key)?;entry.touch=self.clock;Some(entry.value.clone())
    }
    pub fn oldest(&self)->Option<K> {self.entries.iter().min_by_key(|(_,entry)|entry.touch).map(|(key,_)|key.clone())}
    pub fn remove(&mut self,key:&K)->bool {self.entries.remove(key).is_some()}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/68b-evict-unpinned-pages-in-access-order-01.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/page_cache.rs`

Copy cache ownership, reservation and eviction regression fixtures.

**Locate this existing block:**

```text
    pub fn used(&self)->usize {self.budget.used()}
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/68b-evict-unpinned-pages-in-access-order-02.rs" skip=53
```

## <a id="checkpoint-68c-reject-late-source-generations"></a>4. Reject late source generations

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/stream_generation.rs, s…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source generation</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">current completion</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one adoption</text>
</svg>
</div>

**What this code does:** Reject late source generations.

### 1. `src/stream_generation.rs`

Check source generation and exactly one completion per request.

Create the file and type:

```rust
--8<-- "journey/code/68c-reject-late-source-generations-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust

pub mod page_cache;
```

After that block, type:

```rust
--8<-- "journey/code/68c-reject-late-source-generations-02.rs" skip=21
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/stream_generation.rs`

Copy concurrent completion, cancellation and overflow tests.

**Locate this existing block:**

```text
    pub fn close(&mut self) {self.active=false;self.pending.clear();}
    pub fn pending(&self)->usize {self.pending.len()}
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/68c-reject-late-source-generations-03.rs" skip=125
```

## <a id="checkpoint-68d-bound-source-prefixes-and-decode-work"></a>5. Bound source prefixes and decode work

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/stream_budget.rs, src/l…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source count</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">checked temporary bytes</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">bounded page</text>
</svg>
</div>

**What this code does:** Bound source prefixes and decode work.

### 1. `src/stream_budget.rs`

Limit point ranges and reserve checked decode work.

Create the file and type:

```rust
--8<-- "journey/code/68d-bound-source-prefixes-and-decode-work-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust

pub mod stream_generation;
```

After that block, type:

```rust
--8<-- "journey/code/68d-bound-source-prefixes-and-decode-work-02.rs" skip=28
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/stream_budget.rs`

Copy range ceiling and allocation overflow fixtures.

**Locate this existing block:**

```text
}
pub fn temporary_page_bytes(count:u32)->Option<usize> {usize::try_from(count).ok()?.checked_mul(160)}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/68d-bound-source-prefixes-and-decode-work-03.rs" skip=104
```

## <a id="checkpoint-68e-validate-original-streamed-presentation"></a>6. Validate streamed presentation

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 6 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/stream_properties.rs, s…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original style fields</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">checked values</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source properties</text>
</svg>
</div>

**What this code does:** Validate original streamed presentation.

### 1. `src/stream_properties.rs`

Retain and validate original presentation fields.

Create the file and type:

```rust
--8<-- "journey/code/68e-validate-original-streamed-presentation-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust

pub mod stream_budget;
```

After that block, type:

```rust
--8<-- "journey/code/68e-validate-original-streamed-presentation-02.rs" skip=24
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/stream_properties.rs`

Copy original presentation value/error fixtures.

**Locate this existing block:**

```text
    }
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/68e-validate-original-streamed-presentation-03.rs" skip=8
```

## <a id="checkpoint-68f-reuse-a-checked-source-probe"></a>7. Reuse a checked source probe

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 7 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/stream.rs, src/range_so…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source probe</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original properties</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source descriptor</text>
</svg>
</div>

**What this code does:** Reuse a checked source probe.

### 1. `src/stream.rs`

Read checked original style fields through bounded metadata windows.

**Locate this existing block:**

```rust
    }

    /// Points `[from, to)` of the cloud.
    pub async fn fetch_positions(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/68f-reuse-a-checked-source-probe-01.rs"
```

### 2. `src/range_source.rs`

Reuse the probe and retain original source properties.

Replace the entire file with:

```rust
pub struct Source {pub properties:crate::stream_properties::Properties,pub guid:String,pub name:String,pub url:String,pub fields:crate::stream::CloudFields,pub lod:crate::stream::CloudLod}

#[cfg(target_arch="wasm32")]
impl Source {
    pub async fn open(url:String)->Result<Self,String> {
        let probe=crate::stream::probe(&url).await.ok_or("Source probe failed")?;
        Self::from_probe(url,&probe).await
    }
    pub async fn from_probe(url:String,probe:&crate::range_fetch::Reply)->Result<Self,String> {
        if probe.etag.is_none()&&!crate::manifest::immutable_key(&url) {return Err("Streaming requires an ETag or immutable source URL".into());}
        let (guid,name)=crate::stream::cloud_identity(&probe.bytes).ok_or("Missing original cloud identity")?;
        let mut fields=crate::stream::cloud_fields(&url,probe).await.ok_or("Not a single range-readable cloud")?;
        let lod=crate::stream::cloud_lod(&url,&mut fields).await.ok_or("Invalid original cloud array or octree layout")?;
        let properties=crate::stream::cloud_properties(&url,&fields).await.ok_or("Invalid original cloud presentation fields")?;
        Ok(Self {properties,guid,name,url,fields,lod})
    }
    pub async fn page(&self,rows:std::ops::Range<u32>)->Result<crate::range_page::Page,String> {
        if rows.start>rows.end||rows.end>self.fields.count {return Err("Point rows outside source".into());}
        let count=u64::from(rows.end-rows.start);
        let at=self.fields.coords_at.checked_add(u64::from(rows.start)*24).ok_or("Coordinate offset overflow")?;
        let raw=crate::stream::source_range(&self.url,at,count*24,&self.fields.revision).await.ok_or("Coordinate range failed")?;
        let ids=if self.fields.ids_len==0 {None} else {
            let at=self.fields.ids_at.checked_add(u64::from(rows.start)*4).ok_or("ID offset overflow")?;
            Some(crate::stream::source_range(&self.url,at,count*4,&self.fields.revision).await.ok_or("Original ID range failed")?)
        };
        crate::range_page::Page::decode(rows,self.fields.count,&raw,ids.as_deref()).map_err(str::to_owned)
    }

}

#[cfg(test)]
mod tests {
    use prost::Message;
    #[test]
    fn range_layout_and_original_pages_match_the_kernel_protobuf() {
        let mut cloud=session_rust::PointCloud::from_coords(vec![1_000_000_000.125,2.,3.,1_000_000_000.375,4.,5.],vec![],vec![]);
        cloud.name="Precise ranged source".into();let _=cloud.guid();let mut proto=cloud.to_proto();proto.point_ids=vec![u32::MAX,7];
        let mut objects=session_rust::proto::Objects::default();objects.pointclouds.push(proto.clone());let document=session_rust::proto::Session {objects:Some(objects),..Default::default()};let raw=document.encode_to_vec();
        assert_eq!(crate::stream::cloud_identity(&raw).unwrap(),(proto.guid.clone(),proto.name.clone()));
        let (at,length)=crate::stream::walk_to_coords(&raw).unwrap();assert_eq!(length,48);
        let page=crate::range_page::Page::decode(0..2,2,&raw[at as usize..(at+length) as usize],Some(&proto.point_ids.iter().flat_map(|id|id.to_le_bytes()).collect::<Vec<_>>())).unwrap();
        assert_eq!(page.coordinates,proto.coords);assert_eq!(page.ids,proto.point_ids);
        assert!(crate::stream::walk_to_coords(&raw[..at as usize-1]).is_none());
    }
}
```

## <a id="checkpoint-68fa-validate-complete-source-metadata"></a>8. Validate complete source metadata

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 8 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/range_shape.rs, src/lib…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source header</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">bounded root metadata</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">safe range display</text>
</svg>
</div>

**What this code does:** Refuse mixed or placed originals before choosing sampled range display.

### 1. `src/range_shape.rs`

Validate single-cloud layout and complete original metadata before sampling.

Create the file and type:

```rust
--8<-- "journey/code/68fa-validate-complete-source-metadata-01.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/range_shape.rs`

Copy original placement, source extension and metadata refusal fixtures.

**Locate this existing block:**

```text
    Ok(())
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/68fa-validate-complete-source-metadata-02.rs" skip=13
```

### 3. `src/lib.rs`

Register complete original metadata validation.

**Locate this existing block:**

```rust

pub mod stream_properties;
```

After that block, type:

```rust
--8<-- "journey/code/68fa-validate-complete-source-metadata-03.rs" skip=28
```

### 4. `src/range_source.rs`

Refuse unapplied original metadata and mixed source geometry before range adoption.

Replace the entire file with:

```rust
pub struct Source {pub properties:crate::stream_properties::Properties,pub guid:String,pub name:String,pub url:String,pub fields:crate::stream::CloudFields,pub lod:crate::stream::CloudLod}


#[cfg(target_arch="wasm32")]
impl Source {
    pub async fn open(url:String)->Result<Self,String> {
        let probe=crate::stream::probe(&url).await.ok_or("Source probe failed")?;
        Self::from_probe(url,&probe).await
    }
    pub async fn from_probe(url:String,probe:&crate::range_fetch::Reply)->Result<Self,String> {
        if !crate::range_shape::single_cloud(&probe.bytes) {return Err("Ranged source must contain one original cloud".into());}
        if probe.etag.is_none()&&!crate::manifest::immutable_key(&url) {return Err("Streaming requires an ETag or immutable source URL".into());}
        let (guid,name)=crate::stream::cloud_identity(&probe.bytes).ok_or("Missing original cloud identity")?;
        let mut fields=crate::stream::cloud_fields(&url,probe).await.ok_or("Not a single range-readable cloud")?;
        let lod=crate::stream::cloud_lod(&url,&mut fields).await.ok_or("Invalid original cloud array or octree layout")?;
        let properties=crate::stream::cloud_properties(&url,&fields).await.ok_or("Invalid original cloud presentation fields")?;
        let total=probe.total.ok_or("Missing original file size")?;
        let length=total.checked_sub(fields.end).ok_or("Cloud exceeds its original file")?;
        if length>1024*1024 {return Err("Original source metadata exceeds its range limit".into());}
        if length>0 {
            let raw=crate::stream::source_range(&url,fields.end,length,&fields.revision).await.ok_or("Original source metadata range failed")?;
            crate::range_shape::metadata(&raw,&guid)?;
        }
        Ok(Self {properties,guid,name,url,fields,lod})
    }
    pub async fn page(&self,rows:std::ops::Range<u32>)->Result<crate::range_page::Page,String> {
        if rows.start>rows.end||rows.end>self.fields.count {return Err("Point rows outside source".into());}
        let count=u64::from(rows.end-rows.start);
        let at=self.fields.coords_at.checked_add(u64::from(rows.start)*24).ok_or("Coordinate offset overflow")?;
        let raw=crate::stream::source_range(&self.url,at,count*24,&self.fields.revision).await.ok_or("Coordinate range failed")?;
        let ids=if self.fields.ids_len==0 {None} else {
            let at=self.fields.ids_at.checked_add(u64::from(rows.start)*4).ok_or("ID offset overflow")?;
            Some(crate::stream::source_range(&self.url,at,count*4,&self.fields.revision).await.ok_or("Original ID range failed")?)
        };
        crate::range_page::Page::decode(rows,self.fields.count,&raw,ids.as_deref()).map_err(str::to_owned)
    }

}

#[cfg(test)]
mod tests {
    use prost::Message;
    #[test]
    fn range_layout_and_original_pages_match_the_kernel_protobuf() {
        let mut cloud=session_rust::PointCloud::from_coords(vec![1_000_000_000.125,2.,3.,1_000_000_000.375,4.,5.],vec![],vec![]);
        cloud.name="Precise ranged source".into();let _=cloud.guid();let mut proto=cloud.to_proto();proto.point_ids=vec![u32::MAX,7];
        let mut objects=session_rust::proto::Objects::default();objects.pointclouds.push(proto.clone());let document=session_rust::proto::Session {objects:Some(objects),..Default::default()};let raw=document.encode_to_vec();
        assert!(crate::range_shape::single_cloud(&raw));assert_eq!(crate::stream::cloud_identity(&raw).unwrap(),(proto.guid.clone(),proto.name.clone()));
        let (at,length)=crate::stream::walk_to_coords(&raw).unwrap();assert_eq!(length,48);
        let page=crate::range_page::Page::decode(0..2,2,&raw[at as usize..(at+length) as usize],Some(&proto.point_ids.iter().flat_map(|id|id.to_le_bytes()).collect::<Vec<_>>())).unwrap();
        assert_eq!(page.coordinates,proto.coords);assert_eq!(page.ids,proto.point_ids);
        assert!(crate::stream::walk_to_coords(&raw[..at as usize-1]).is_none());let mut mixed=document.clone();mixed.objects.as_mut().unwrap().points.push(session_rust::Point::new(0.,0.,0.).to_proto());assert!(!crate::range_shape::single_cloud(&mixed.encode_to_vec()));
    }
}
```

## <a id="checkpoint-68g-slice-original-pages-without-losing-attributes"></a>9. Slice pages without losing attributes

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 9 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/range_page.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original page</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original identities</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained byte charge</text>
</svg>
</div>

**What this code does:** Slice original pages without losing attributes.

### 1. `src/range_page.rs`

Keep original attributes and slices with checked page lengths.

Replace the entire file with:

```rust
use std::ops::Range;

pub struct Page {
    pub rows:Range<u32>,
    pub coordinates:Vec<f64>,
    pub display:Vec<f32>,
    pub ids:Vec<u32>,
    pub colors:Vec<u32>,
    pub normals:Vec<u32>,
}
impl Page {
    pub fn decode(rows:Range<u32>,total:u32,raw:&[u8],ids:Option<&[u8]>)->Result<Self,&'static str> {
        if rows.start>rows.end||rows.end>total {return Err("Point rows outside source");}
        let count=(rows.end-rows.start) as usize;
        if raw.len()!=count.checked_mul(24).ok_or("Page size overflow")? {return Err("Incomplete original coordinate page");}
        let coordinates=raw.chunks_exact(8).map(|v|f64::from_le_bytes(v.try_into().unwrap())).collect::<Vec<_>>();
        if coordinates.iter().any(|v|!v.is_finite()||!(*v as f32).is_finite()) {return Err("Invalid source coordinates");}
        let ids=if let Some(ids)=ids {
            if ids.len()!=count.checked_mul(4).ok_or("ID page size overflow")? {return Err("Incomplete original point ID page");}
            ids.chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap())).collect::<Vec<_>>()
        } else {rows.clone().collect()};
        if ids.iter().copied().collect::<std::collections::HashSet<_>>().len()!=count {return Err("Repeated original point IDs");}
        let display=coordinates.iter().map(|v|*v as f32).collect();Ok(Self {rows,coordinates,display,ids,colors:Vec::new(),normals:Vec::new()})
    }
    pub fn slice(&self,rows:Range<u32>)->Result<Self,&'static str> {
        if rows.start<self.rows.start||rows.end>self.rows.end||rows.start>=rows.end {return Err("Slice is outside its original page");}
        let a=(rows.start-self.rows.start) as usize;let b=(rows.end-self.rows.start) as usize;
        Ok(Self {rows,coordinates:self.coordinates[a*3..b*3].to_vec(),display:self.display[a*3..b*3].to_vec(),ids:self.ids[a..b].to_vec(),colors:if self.colors.is_empty() {Vec::new()} else {self.colors[a..b].to_vec()},normals:if self.normals.is_empty() {Vec::new()} else {self.normals[a..b].to_vec()}})
    }
    pub fn attributes(mut self,colors:Vec<u32>,normals:Vec<u32>)->Result<Self,&'static str> {
        let count=(self.rows.end-self.rows.start) as usize;
        if !colors.is_empty()&&colors.len()!=count||!normals.is_empty()&&normals.len()!=count {return Err("Source page attributes do not match its point rows");}
        self.colors=colors;self.normals=normals;Ok(self)
    }
    pub fn bytes(&self)->usize {self.coordinates.capacity()*8+self.display.capacity()*4+self.ids.capacity()*4+self.colors.capacity()*4+self.normals.capacity()*4}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pages_keep_original_f64_coordinates_high_ids_and_fallback_rows() {
        let raw=[1_000_000_000.125f64,2.,3.,1_000_000_000.375,4.,5.].into_iter().flat_map(f64::to_le_bytes).collect::<Vec<_>>();
        let ids=[u32::MAX,7].into_iter().flat_map(u32::to_le_bytes).collect::<Vec<_>>();let page=Page::decode(8..10,20,&raw,Some(&ids)).unwrap();
        assert_eq!(page.coordinates[0],1_000_000_000.125);assert_eq!(page.coordinates[3],1_000_000_000.375);assert_eq!(page.display[0],page.display[3]);assert_eq!(page.ids,[u32::MAX,7]);assert_eq!(Page::decode(8..10,20,&raw,None).unwrap().ids,[8,9]);
        assert!(Page::decode(8..10,20,&raw[..47],Some(&ids)).is_err());assert!(Page::decode(8..10,20,&raw,Some(&ids[..7])).is_err());assert!(Page::decode(8..10,9,&raw,None).is_err());assert!(Page::decode(10..8,20,&raw,None).is_err());
        let duplicates=[7u32,7].into_iter().flat_map(u32::to_le_bytes).collect::<Vec<_>>();assert!(Page::decode(8..10,20,&raw,Some(&duplicates)).is_err());
    }
    #[test]
    fn invalid_original_doubles_never_become_a_display_page() {
        for value in [f64::NAN,f64::INFINITY,f64::MAX] {let raw=[value,0.,0.].into_iter().flat_map(f64::to_le_bytes).collect::<Vec<_>>();assert!(Page::decode(0..1,1,&raw,None).is_err());}
    }
}

impl crate::page_cache::ByteSize for Page {fn bytes(&self)->usize {Page::bytes(self)}}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/range_page.rs`

Copy exact attribute and slice regression fixtures.

**Locate this existing block:**

```text

impl crate::page_cache::ByteSize for Page {fn bytes(&self)->usize {Page::bytes(self)}}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/68g-slice-original-pages-without-losing-attributes-04.rs" skip=88
```

## <a id="checkpoint-68h-prepare-shared-remote-source-owners"></a>10. Prepare shared remote source owners

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 10 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/range_cloud.rs, src/lib…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original descriptor</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">precise bounds</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared owner</text>
</svg>
</div>

**What this code does:** Prepare shared remote source owners.

### 1. `src/range_cloud.rs`

Own the complete remote descriptor and precise original bounds.

Create the file and type:

```rust
--8<-- "journey/code/68h-prepare-shared-remote-source-owners-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust

pub mod stream_properties;
```

After that block, type:

```rust
--8<-- "journey/code/68h-prepare-shared-remote-source-owners-02.rs" skip=28
```

## <a id="checkpoint-68i-keep-independent-streamed-placements"></a>11. Keep independent streamed placements

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 11 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/range_cloud.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared original</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">f64 model</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">placed bounds</text>
</svg>
</div>

**What this code does:** Keep independent streamed placements.

### 1. `src/range_cloud.rs`

Keep independent owner placements and precise world bounds.

**Locate this existing block:**

```rust
    }
}
```

After that block, type:

```rust
--8<-- "journey/code/68i-keep-independent-streamed-placements-01.rs" skip=8
```

## <a id="checkpoint-68j-pin-coarse-pages-and-adopt-checked-ranges"></a>12. Pin coarse pages and adopt checked ranges

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 12 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/range_cloud.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">coarse pin</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared cached value</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">independent eviction</text>
</svg>
</div>

**What this code does:** Pin coarse pages and adopt checked ranges.

### 1. `src/range_cloud.rs`

Adopt checked original pages and retain a coarse fallback.

**Locate this existing block:**

```rust
        Ok(Rc::new(Self {source,nodes:Rc::new(nodes),bounds:Bounds {min,max},pages:RefCell::new(Cache::new(budget)),root:RefCell::new(None),generation:RefCell::new(Default::default()),loaded:std::cell::Cell::new(0)}))
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/68j-pin-coarse-pages-and-adopt-checked-ranges-01.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/range_cloud.rs`

Copy source descriptor, page identity and lifetime fixtures.

**Locate this existing block:**

```text
        for x in [bounds.min[0],bounds.max[0]] {for y in [bounds.min[1],bounds.max[1]] {for z in [bounds.min[2],bounds.max[2]] {let p=self.model.transform_point(&session_rust::Point::new(x,y,z));result.include([p[0],p[1],p[2]]);}}}result
    }
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/68j-pin-coarse-pages-and-adopt-checked-ranges-02.rs" skip=246
```

## <a id="checkpoint-68k-supply-coarse-and-detailed-page-specimens"></a>13. Supply coarse and detailed page specimens

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 13 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/range_example.rs, src/l…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original source specimen</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">pinned coarse page</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained detailed page</text>
</svg>
</div>

**What this code does:** Supply coarse and detailed page specimens.

**Supplied example data:**

Copy this supplied data once; it is excluded from the typing estimate.

### 1. `src/range_example.rs`

Copy source-page rendering and ownership specimens.

Copy this supplied file:

```text
--8<-- "journey/code/68k-supply-coarse-and-detailed-page-specimens-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust

pub mod range_cloud;
```

After that block, type:

```rust
--8<-- "journey/code/68k-supply-coarse-and-detailed-page-specimens-02.rs" skip=22
```

## <a id="checkpoint-68l-insert-streamed-rows-on-the-current-layer"></a>14. Insert streamed rows on the current layer

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 14 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/scene.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">validated source placement</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original flags</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">current layer</text>
</svg>
</div>

**What this code does:** Insert streamed rows on the current layer.

### 1. `src/scene.rs`

Insert validated streamed rows and release them with the scene.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    pub fn remove(&mut self, id: ObjectId) -> bool {
        let existed = Rc::make_mut(&mut self.ids).remove(&id);
        self.objects.retain(|row| row.id != id); self.lines.retain(|row| row.id != id);
        self.paths.retain(|row| row.id != id); self.points.retain(|row| row.id != id);self.texts.retain(|row|row.id!=id);self.clouds.retain(|row|row.id!=id);
        if self.extra == Some(id) { self.extra = None; }
        existed
    }
```

Replace it with:

```rust
    pub fn remove(&mut self, id: ObjectId) -> bool {
        let existed = Rc::make_mut(&mut self.ids).remove(&id);
        self.objects.retain(|row| row.id != id); self.lines.retain(|row| row.id != id);
        self.paths.retain(|row| row.id != id); self.points.retain(|row| row.id != id);self.texts.retain(|row|row.id!=id);self.clouds.retain(|row|row.id!=id);self.streamed.retain(|row|row.id!=id);
        if self.extra == Some(id) { self.extra = None; }
        existed
    }
```

**Locate this block:**

```rust
        self.ids=Default::default();
        self.view=Default::default();
        self.forest=Default::default();
        self.clouds=Vec::new();
        self.texts=Vec::new();
        self.points = Vec::new();
        self.paths = Vec::new();
```

Replace it with:

```rust
        self.ids=Default::default();
        self.view=Default::default();
        self.forest=Default::default();
        self.clouds=Vec::new();self.streamed=Vec::new();
        self.texts=Vec::new();
        self.points = Vec::new();
        self.paths = Vec::new();
```

**Locate this block:**

```rust
        let parent=parent.filter(|id|self.pick_id(id.value()).is_some());
        for row in &mut self.clouds {row.controls_visible=Some(row.id)==parent;}
    }
    pub fn clouds(&self)->&[crate::cloud::Row] {&self.clouds}
    pub fn insert_cloud(&mut self,prepared:crate::cloud::Prepared)->Result<ObjectId,&'static str> {
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let id=ObjectId(self.next_id);let mut guid=prepared.source.guid().to_string();while self.clouds.iter().any(|r|r.guid==guid) {guid=uuid::Uuid::new_v4().to_string();}
```

Replace it with:

```rust
        let parent=parent.filter(|id|self.pick_id(id.value()).is_some());
        for row in &mut self.clouds {row.controls_visible=Some(row.id)==parent;}
    }
    pub fn streams(&self)->&[crate::range_cloud::Row] {&self.streamed}
    pub fn insert_stream(&mut self,prepared:Rc<crate::range_cloud::Prepared>,model:session_rust::Xform,point_px:f32)->Result<ObjectId,&'static str> {
        if !crate::placement::valid(&model.m)||!point_px.is_finite()||point_px<0. {return Err("Invalid streamed source placement or point size");}
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let id=ObjectId(self.next_id);let mut guid=prepared.source.guid.clone();while self.streamed.iter().any(|row|row.guid==guid) {guid=uuid::Uuid::new_v4().to_string();}
        let row=crate::range_cloud::Row {id,guid:guid.clone(),prepared,model,point_px,controls_visible:false};
        if row.bounds().min.into_iter().chain(row.bounds().max).any(|v|!v.is_finite()||!(v as f32).is_finite()) {return Err("Streamed source placement exceeds display range");}
        let name=row.prepared.source.name.clone();let properties=row.prepared.source.properties;
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(id);Rc::make_mut(&mut self.view).initial(id,properties.visible,properties.locked);self.streamed.push(row);crate::layer_edit::attach_generated(self,id,guid,name)?;Ok(id)
    }
    pub fn place_stream(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(),&'static str> {
        if !crate::placement::valid(&model.m) {return Err("Invalid streamed source placement");}
        let row=self.streamed.iter_mut().find(|row|row.id==id).ok_or("Streamed source is gone")?;let mut candidate=row.clone();candidate.model=model;
        if candidate.bounds().min.into_iter().chain(candidate.bounds().max).any(|v|!v.is_finite()||!(v as f32).is_finite()) {return Err("Streamed source placement exceeds display range");}*row=candidate;Ok(())
    }
    pub fn clouds(&self)->&[crate::cloud::Row] {&self.clouds}
    pub fn insert_cloud(&mut self,prepared:crate::cloud::Prepared)->Result<ObjectId,&'static str> {
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let id=ObjectId(self.next_id);let mut guid=prepared.source.guid().to_string();while self.clouds.iter().any(|r|r.guid==guid) {guid=uuid::Uuid::new_v4().to_string();}
```

**Locate this block:**

```rust
    }

    pub fn empty()->Self {
        Self {created:None,ids:Default::default(),view:Default::default(),forest:Default::default(),clouds:Vec::new(),texts:Vec::new(),objects:Vec::new(),points:Vec::new(),paths:Vec::new(),lines:Vec::new(),next_id:1,extra:None}
    }

    pub fn demo() -> Self {
```

Replace it with:

```rust
    }

    pub fn empty()->Self {
        Self {created:None,ids:Default::default(),view:Default::default(),forest:Default::default(),streamed:Vec::new(),clouds:Vec::new(),texts:Vec::new(),objects:Vec::new(),points:Vec::new(),paths:Vec::new(),lines:Vec::new(),next_id:1,extra:None}
    }

    pub fn demo() -> Self {
```

**Locate this block:**

```rust
    ids:Rc<std::collections::HashSet<ObjectId>>,
    pub view:Rc<crate::view_flags::Flags>,
    pub forest:crate::hierarchy::Forest,
    clouds:Vec<crate::cloud::Row>,
    texts:Vec<crate::scene_label::Row>,
    points: Vec<PointObject>,
```

Replace it with:

```rust
    ids:Rc<std::collections::HashSet<ObjectId>>,
    pub view:Rc<crate::view_flags::Flags>,
    pub forest:crate::hierarchy::Forest,
    streamed:Vec<crate::range_cloud::Row>,
    clouds:Vec<crate::cloud::Row>,
    texts:Vec<crate::scene_label::Row>,
    points: Vec<PointObject>,
```

## <a id="checkpoint-68m-carry-source-placement-through-scene-operations"></a>15. Carry source placement through scene operations

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 15 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/scene.rs, src/hierarchy…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source row</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared source</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">independent owner identity</text>
</svg>
</div>

**What this code does:** Carry source placement through scene operations.

### 1. `src/scene.rs`

Carry streamed owners through scene identity and placement operations.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    pub fn contains(&self,id:ObjectId)->bool {self.ids.contains(&id)}

    pub fn next(&self,current:Option<ObjectId>)->Option<ObjectId> {
        let mut ids=self.objects.iter().map(|r|r.id).chain(self.lines.iter().map(|r|r.id)).chain(self.paths.iter().map(|r|r.id)).chain(self.points.iter().map(|r|r.id)).chain(self.texts.iter().map(|r|r.id)).chain(self.clouds.iter().map(|r|r.id)).filter(|id|self.pick_id(id.value()).is_some()).collect::<Vec<_>>();
        ids.sort_unstable_by_key(|id|id.value());if ids.is_empty() {return None;}
        let start=ids.iter().position(|id|Some(*id)==current).map_or(0,|i|i+1);Some(ids[start%ids.len()])
    }
```

Replace it with:

```rust
    pub fn contains(&self,id:ObjectId)->bool {self.ids.contains(&id)}

    pub fn next(&self,current:Option<ObjectId>)->Option<ObjectId> {
        let mut ids=self.objects.iter().map(|r|r.id).chain(self.lines.iter().map(|r|r.id)).chain(self.paths.iter().map(|r|r.id)).chain(self.points.iter().map(|r|r.id)).chain(self.texts.iter().map(|r|r.id)).chain(self.clouds.iter().map(|r|r.id)).chain(self.streamed.iter().map(|r|r.id)).filter(|id|self.pick_id(id.value()).is_some()).collect::<Vec<_>>();
        ids.sort_unstable_by_key(|id|id.value());if ids.is_empty() {return None;}
        let start=ids.iter().position(|id|Some(*id)==current).map_or(0,|i|i+1);Some(ids[start%ids.len()])
    }
```

**Locate this block:**

```rust
            let mut guid=source.row_guid(*old).ok_or("Original source owner is missing")?.to_owned();while !used.insert(guid.clone()) {guid=uuid::Uuid::new_v4().to_string();}names.insert(*old,guid);
        }
        macro_rules! rows {($field:ident)=>{for row in &source.$field {let mut row=row.clone();let old=row.id;row.id=remap[&old];row.guid=names[&old].clone();candidate.$field.push(row);}};}
        rows!(objects);rows!(lines);rows!(paths);rows!(points);rows!(clouds);rows!(texts);
        for old in &ids {
            let id=remap[old];candidate.place_following(id,place*&source.placement(*old).ok_or("Original placement is missing")?)?;
            let view=Rc::make_mut(&mut candidate.view);view.initial(id,source.visible(*old),source.view.locked.contains(old));
            if let Some(rgb)=source.view.faces.get(old) {view.faces.insert(id,*rgb);}if let Some(rgb)=source.view.edges.get(old) {view.edges.insert(id,*rgb);}
            if source.clouds.iter().any(|row|row.id==*old)&&point_px>0. {candidate.cloud_point_size(id,point_px)?;}
        }
        for doc in &source.forest.documents {
            let mut doc=(**doc).clone();let old_key=doc.key;doc.key=candidate.reserve_document_key()?;
```

Replace it with:

```rust
            let mut guid=source.row_guid(*old).ok_or("Original source owner is missing")?.to_owned();while !used.insert(guid.clone()) {guid=uuid::Uuid::new_v4().to_string();}names.insert(*old,guid);
        }
        macro_rules! rows {($field:ident)=>{for row in &source.$field {let mut row=row.clone();let old=row.id;row.id=remap[&old];row.guid=names[&old].clone();candidate.$field.push(row);}};}
        rows!(objects);rows!(lines);rows!(paths);rows!(points);rows!(clouds);rows!(streamed);rows!(texts);
        for old in &ids {
            let id=remap[old];candidate.place_following(id,place*&source.placement(*old).ok_or("Original placement is missing")?)?;
            let view=Rc::make_mut(&mut candidate.view);view.initial(id,source.visible(*old),source.view.locked.contains(old));
            if let Some(rgb)=source.view.faces.get(old) {view.faces.insert(id,*rgb);}if let Some(rgb)=source.view.edges.get(old) {view.edges.insert(id,*rgb);}
            if source.clouds.iter().any(|row|row.id==*old)&&point_px>0. {candidate.cloud_point_size(id,point_px)?;}
            if source.streamed.iter().any(|row|row.id==*old)&&point_px>0. {candidate.streamed.iter_mut().find(|row|row.id==id).unwrap().point_px=point_px;}
        }
        for doc in &source.forest.documents {
            let mut doc=(**doc).clone();let old_key=doc.key;doc.key=candidate.reserve_document_key()?;
```

**Locate this block:**

```rust
            .or_else(||self.points.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.lines.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.clouds.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.texts.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
    }
```

Replace it with:

```rust
            .or_else(||self.points.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.lines.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.clouds.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.streamed.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.texts.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
    }
```

**Locate this block:**

```rust
        else if let Some(row)=self.lines.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.lines.push(row);}
        else if let Some(row)=self.paths.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.paths.push(row);}
        else if let Some(row)=self.points.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.points.push(row);}
        else if let Some(row)=self.clouds.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;row.controls_visible=false;self.clouds.push(row);}
        else if let Some(row)=self.texts.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.texts.push(row);}
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(copy);
```

Replace it with:

```rust
        else if let Some(row)=self.lines.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.lines.push(row);}
        else if let Some(row)=self.paths.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.paths.push(row);}
        else if let Some(row)=self.points.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.points.push(row);}
        else if let Some(row)=self.streamed.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;row.controls_visible=false;self.streamed.push(row);}
        else if let Some(row)=self.clouds.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;row.controls_visible=false;self.clouds.push(row);}
        else if let Some(row)=self.texts.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.texts.push(row);}
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(copy);
```

**Locate this block:**

```rust
    pub fn cloud_controls(&mut self,parent:Option<ObjectId>) {
        let parent=parent.filter(|id|self.pick_id(id.value()).is_some());
        for row in &mut self.clouds {row.controls_visible=Some(row.id)==parent;}
    }
    pub fn streams(&self)->&[crate::range_cloud::Row] {&self.streamed}
    pub fn insert_stream(&mut self,prepared:Rc<crate::range_cloud::Prepared>,model:session_rust::Xform,point_px:f32)->Result<ObjectId,&'static str> {
```

Replace it with:

```rust
    pub fn cloud_controls(&mut self,parent:Option<ObjectId>) {
        let parent=parent.filter(|id|self.pick_id(id.value()).is_some());
        for row in &mut self.clouds {row.controls_visible=Some(row.id)==parent;}
        for row in &mut self.streamed {row.controls_visible=Some(row.id)==parent;}
    }
    pub fn streams(&self)->&[crate::range_cloud::Row] {&self.streamed}
    pub fn insert_stream(&mut self,prepared:Rc<crate::range_cloud::Prepared>,model:session_rust::Xform,point_px:f32)->Result<ObjectId,&'static str> {
```

**Locate this block:**

```rust
    pub fn selected_bounds(&self, id: ObjectId) -> Option<crate::bounds::Bounds> {
        if !self.visible(id) { return None; }
        if let Some(row)=self.clouds.iter().find(|row|row.id==id) {return Some(row.bounds());}
        if let Some(row)=self.texts.iter().find(|row|row.id==id) {return row.bounds();}
        let object = self.objects.iter().find(|object| object.id == id)?;
        let mut vertices = object.mesh.vertices().iter();
```

Replace it with:

```rust
    pub fn selected_bounds(&self, id: ObjectId) -> Option<crate::bounds::Bounds> {
        if !self.visible(id) { return None; }
        if let Some(row)=self.clouds.iter().find(|row|row.id==id) {return Some(row.bounds());}
        if let Some(row)=self.streamed.iter().find(|row|row.id==id) {return Some(row.bounds());}
        if let Some(row)=self.texts.iter().find(|row|row.id==id) {return row.bounds();}
        let object = self.objects.iter().find(|object| object.id == id)?;
        let mut vertices = object.mesh.vertices().iter();
```

**Locate this block:**

```rust
        if let Some(row) = self.lines().iter().find(|r| r.id == id) { let model = &shift * &row.model; return self.place_line(id, model); }
        if let Some(row) = self.paths().iter().find(|r| r.id == id) { let model = &shift * &row.model; return self.place_path(id, model); }
        if let Some(row) = self.points().iter().find(|r| r.id == id) { let model = &shift * &row.model; return self.place_point(id, model); }
        if let Some(row)=self.clouds.iter().find(|r|r.id==id) {let model=&shift*&row.model;return self.place_cloud(id,model);}
        if let Some(row)=self.texts.iter().find(|r|r.id==id) {let model=&shift*&row.model;return self.place_text(id,model);}
        Err("Object not found")
```

Replace it with:

```rust
        if let Some(row) = self.lines().iter().find(|r| r.id == id) { let model = &shift * &row.model; return self.place_line(id, model); }
        if let Some(row) = self.paths().iter().find(|r| r.id == id) { let model = &shift * &row.model; return self.place_path(id, model); }
        if let Some(row) = self.points().iter().find(|r| r.id == id) { let model = &shift * &row.model; return self.place_point(id, model); }
        if let Some(row)=self.streamed.iter().find(|r|r.id==id) {let model=&shift*&row.model;return self.place_stream(id,model);}
        if let Some(row)=self.clouds.iter().find(|r|r.id==id) {let model=&shift*&row.model;return self.place_cloud(id,model);}
        if let Some(row)=self.texts.iter().find(|r|r.id==id) {let model=&shift*&row.model;return self.place_text(id,model);}
        Err("Object not found")
```

**Locate this block:**

```rust
        else if self.lines.iter().any(|r| r.id==id) { self.place_line(id,model) }
        else if self.paths.iter().any(|r| r.id==id) { self.place_path(id,model) }
        else if self.clouds.iter().any(|r|r.id==id) {self.place_cloud(id,model)}
        else if self.texts.iter().any(|r|r.id==id) {self.place_text(id,model)}
        else { self.place_point(id,model) }
    }
```

Replace it with:

```rust
        else if self.lines.iter().any(|r| r.id==id) { self.place_line(id,model) }
        else if self.paths.iter().any(|r| r.id==id) { self.place_path(id,model) }
        else if self.clouds.iter().any(|r|r.id==id) {self.place_cloud(id,model)}
        else if self.streamed.iter().any(|r|r.id==id) {self.place_stream(id,model)}
        else if self.texts.iter().any(|r|r.id==id) {self.place_text(id,model)}
        else { self.place_point(id,model) }
    }
```

**Locate this block:**

```rust
            .or_else(|| self.paths.iter().find(|r| r.id==id).map(|r| r.model.clone()))
            .or_else(|| self.points.iter().find(|r| r.id==id).map(|r| r.model.clone()))
            .or_else(|| self.clouds.iter().find(|r|r.id==id).map(|r|r.model.clone()))
            .or_else(|| self.texts.iter().find(|r| r.id==id).map(|r| r.model.clone()))
    }
```

Replace it with:

```rust
            .or_else(|| self.paths.iter().find(|r| r.id==id).map(|r| r.model.clone()))
            .or_else(|| self.points.iter().find(|r| r.id==id).map(|r| r.model.clone()))
            .or_else(|| self.clouds.iter().find(|r|r.id==id).map(|r|r.model.clone()))
            .or_else(|| self.streamed.iter().find(|r|r.id==id).map(|r|r.model.clone()))
            .or_else(|| self.texts.iter().find(|r| r.id==id).map(|r| r.model.clone()))
    }
```

**Locate this block:**

```rust
            match &mut bounds { Some(bounds) => bounds.include(p), None => bounds = Some(crate::bounds::Bounds::point(p)) }
        }
        for row in self.texts.iter().filter(|row|self.visible(row.id)) {if let Some(next)=row.bounds() {if let Some(bounds)=&mut bounds {bounds.include(next.min);bounds.include(next.max);} else {bounds=Some(next);}}}
        for row in self.clouds.iter().filter(|r|self.visible(r.id)) {let next=row.bounds();if let Some(bounds)=&mut bounds {bounds.include(next.min);bounds.include(next.max);} else {bounds=Some(next);}}
        bounds
    }
```

Replace it with:

```rust
            match &mut bounds { Some(bounds) => bounds.include(p), None => bounds = Some(crate::bounds::Bounds::point(p)) }
        }
        for row in self.texts.iter().filter(|row|self.visible(row.id)) {if let Some(next)=row.bounds() {if let Some(bounds)=&mut bounds {bounds.include(next.min);bounds.include(next.max);} else {bounds=Some(next);}}}
        for row in self.streamed.iter().filter(|r|self.visible(r.id)) {let next=row.bounds();if let Some(bounds)=&mut bounds {bounds.include(next.min);bounds.include(next.max);} else {bounds=Some(next);}}
        for row in self.clouds.iter().filter(|r|self.visible(r.id)) {let next=row.bounds();if let Some(bounds)=&mut bounds {bounds.include(next.min);bounds.include(next.max);} else {bounds=Some(next);}}
        bounds
    }
```

### 2. `src/hierarchy.rs`

Name streamed source rows in the original hierarchy.

**Locate this existing block:**

```rust
    }
    for row in scene.objects().iter().filter(|r|!placed.contains(&r.id)) {let index=rows.len();rows.push(Row {key:format!("created/{}",row.id.value()),label:row.metadata.name.clone(),depth:0,end:index+1,ids:vec![row.id],readonly:false});}
    let other=scene.lines().iter().map(|r|(r.id,r.prepared.source.name.as_str())).chain(scene.paths().iter().map(|r|(r.id,r.prepared.source.name()))).chain(scene.points().iter().map(|r|(r.id,r.prepared.source.name.as_str()))).chain(scene.texts().iter().map(|r|(r.id,r.prepared.source.name.as_str()))).chain(scene.clouds().iter().map(|r|(r.id,r.prepared.source.name.as_str())));
    for (id,label) in other.filter(|(id,_)|!placed.contains(id)) {let index=rows.len();rows.push(Row {key:format!("created/{}",id.value()),label:label.to_owned(),depth:0,end:index+1,ids:vec![id],readonly:false});}
    rows
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/68m-carry-source-placement-through-scene-operations-12.rs"
```

## <a id="checkpoint-68n-resolve-compact-picks-to-original-point-ids"></a>16. Resolve compact picks to point IDs

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 16 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/cloud_pick.rs, src/pick…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">GPU page address</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original ID</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">f64 world position</text>
</svg>
</div>

**What this code does:** Resolve compact picks to original point IDs.

### 1. `src/cloud_pick.rs`

Connect streamed selection and original identities while refusing incomplete editable Save.

Replace the entire file with:

```rust
use crate::{pick_sources::{Part,Target},scene::{ObjectId,Scene}};

// Storage indices address current GPU columns; original IDs survive octree reordering and may use every bit of a u32.
pub fn resolve(scene:&Scene,owner:ObjectId,index:usize)->Option<Target> {
    if let Some(row)=scene.streams().iter().find(|r|r.id==owner&&r.controls_visible) {
        scene.pick_id(owner.value())?;let mut address=0usize;
        for page in row.prepared.pages() {let count=page.value.ids.len();if index>=address&&index-address<count {return Some(Target {owner,part:Part::CloudPoint(page.value.ids[index-address])});}address=address.checked_add(count)?;}return None;
    }
    let row=scene.clouds().iter().find(|r|r.id==owner && r.controls_visible)?;
    if scene.pick_id(owner.value()).is_none() || index>=row.prepared.source.point_count() {return None;}
    let target=Target {owner,part:Part::CloudPoint(row.prepared.source.point_id(index))};
    Some(target)
}

pub fn position(scene:&Scene,owner:ObjectId,id:u32)->Option<session_rust::Point> {
    if let Some(row)=scene.streams().iter().find(|r|r.id==owner&&r.controls_visible) {
        scene.pick_id(owner.value())?;let p=row.prepared.position(id)?;return Some(row.model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])));
    }
    let row=scene.clouds().iter().find(|r|r.id==owner && r.controls_visible)?;
    scene.pick_id(owner.value())?;
    let index=row.prepared.source.index_of_id(id)?;
    Some(row.model.transform_point(&row.prepared.source.get_point(index)))
}
```

### 2. `src/pick_sources.rs`

Connect streamed selection and original identities while refusing incomplete editable Save.

**Locate this existing block:**

```rust
        if mode == Mode::Object { return Some(Target { owner, part: Part::Object }); }
        let (kind, address) = sub_id::decode(pixel[1])?;
        if mode==Mode::Control && kind==Kind::Control && scene.clouds().iter().any(|r|r.id==owner) {
            return crate::cloud_pick::resolve(scene,owner,address);
        }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/68n-resolve-compact-picks-to-original-point-ids-03.rs"
```

### 3. `src/editor.rs`

Connect streamed selection and original identities while refusing incomplete editable Save.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                self.history.try_edit(&mut self.scene, |scene| scene.insert_path(prepared).map(|_| ()))?;
            }
            Action::Controls(visible) => {
                let parent=if visible {self.selected.filter(|id|self.scene.clouds().iter().any(|r|r.id==*id))} else {None};
                if parent.is_some() && self.pick_mode==crate::pick_sources::Mode::Control && self.scene.clouds().iter().any(|r|Some(r.id)==parent && r.controls_visible) {return Ok(Change::View);}
                self.scene.cloud_controls(parent);
                self.scene.show_controls(visible);
                if parent.is_some() {self.pick_mode=crate::pick_sources::Mode::Control;self.objects.clear();self.selected_part=None;}
```

Replace it with:

```rust
                self.history.try_edit(&mut self.scene, |scene| scene.insert_path(prepared).map(|_| ()))?;
            }
            Action::Controls(visible) => {
                let parent=if visible {self.selected.filter(|id|self.scene.clouds().iter().any(|r|r.id==*id)||self.scene.streams().iter().any(|r|r.id==*id))} else {None};
                if parent.is_some() && self.pick_mode==crate::pick_sources::Mode::Control && (self.scene.clouds().iter().any(|r|Some(r.id)==parent && r.controls_visible)||self.scene.streams().iter().any(|r|Some(r.id)==parent&&r.controls_visible)) {return Ok(Change::View);}
                self.scene.cloud_controls(parent);
                self.scene.show_controls(visible);
                if parent.is_some() {self.pick_mode=crate::pick_sources::Mode::Control;self.objects.clear();self.selected_part=None;}
```

**Locate this block:**

```rust
            }
            Action::SelectAll => {
                let ids: Vec<_> = self.scene.objects().iter().map(|r| r.id).chain(self.scene.lines().iter().map(|r| r.id))
                    .chain(self.scene.paths().iter().map(|r| r.id)).chain(self.scene.points().iter().map(|r| r.id)).chain(self.scene.texts().iter().map(|r|r.id)).chain(self.scene.clouds().iter().map(|r|r.id)).collect();
                self.select_objects(ids, false);return Ok(Change::View);
            }
            Action::PickedTarget(target) => {
```

Replace it with:

```rust
            }
            Action::SelectAll => {
                let ids: Vec<_> = self.scene.objects().iter().map(|r| r.id).chain(self.scene.lines().iter().map(|r| r.id))
                    .chain(self.scene.paths().iter().map(|r| r.id)).chain(self.scene.points().iter().map(|r| r.id)).chain(self.scene.texts().iter().map(|r|r.id)).chain(self.scene.clouds().iter().map(|r|r.id)).chain(self.scene.streams().iter().map(|r|r.id)).collect();
                self.select_objects(ids, false);return Ok(Change::View);
            }
            Action::PickedTarget(target) => {
```

**Locate this block:**

```rust

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::Touch(actions)=> {let mut change=Change::View;for action in actions {let next=self.apply(action)?;if next!=Change::View {change=next;}}return Ok(change);},
            Action::CanvasStart {from,to,size,modified,pointer}=>{self.canvas_drag=crate::canvas_drag::Drag::new(from,to,size,modified);if let Some(drag)=self.canvas_drag.as_mut() {drag.pointer=pointer;}return Ok(Change::View);},
            Action::CanvasMove {screen,size}=>{if let Some(drag)=self.canvas_drag.as_mut() {if drag.released.is_none()&&drag.size==size&&screen.iter().all(|v|v.is_finite()) {drag.to=screen;drag.update(&self.camera)?;}if drag.preview.is_some() {return Ok(Change::Preview);}}return Ok(Change::View);},
```

Replace it with:

```rust

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::StreamRefresh=>{self.snap_index=None;return Ok(Change::Scene);},
            Action::Touch(actions)=> {let mut change=Change::View;for action in actions {let next=self.apply(action)?;if next!=Change::View {change=next;}}return Ok(change);},
            Action::CanvasStart {from,to,size,modified,pointer}=>{self.canvas_drag=crate::canvas_drag::Drag::new(from,to,size,modified);if let Some(drag)=self.canvas_drag.as_mut() {drag.pointer=pointer;}return Ok(Change::View);},
            Action::CanvasMove {screen,size}=>{if let Some(drag)=self.canvas_drag.as_mut() {if drag.released.is_none()&&drag.size==size&&screen.iter().all(|v|v.is_finite()) {drag.to=screen;drag.update(&self.camera)?;}if drag.preview.is_some() {return Ok(Change::Preview);}}return Ok(Change::View);},
```

**Locate this block:**

```rust

    pub fn gizmo_origin(&self)->Option<[f64;3]> {
        if self.draft.is_some()||self.canvas_drag.is_some() {return None;}
        if self.scene.clouds().iter().any(|r|r.controls_visible) {return None;}
        if let Some(edit)=self.gizmo_edit.as_ref() {return Some(edit.origin());}
        if let Some(target)=self.selection().filter(|_|self.selected_part.is_some()) {return crate::part_preview::origin(&self.scene,target);}
        self.scene.bounds_of(&self.selected_objects()).map(|b|b.centre())
```

Replace it with:

```rust

    pub fn gizmo_origin(&self)->Option<[f64;3]> {
        if self.draft.is_some()||self.canvas_drag.is_some() {return None;}
        if self.scene.clouds().iter().any(|r|r.controls_visible)||self.scene.streams().iter().any(|r|r.controls_visible) {return None;}
        if let Some(edit)=self.gizmo_edit.as_ref() {return Some(edit.origin());}
        if let Some(target)=self.selection().filter(|_|self.selected_part.is_some()) {return crate::part_preview::origin(&self.scene,target);}
        self.scene.bounds_of(&self.selected_objects()).map(|b|b.centre())
```

**Locate this block:**

```rust
use crate::{background::Background, camera::Camera, history::History, scene::{ObjectId, Scene}};

pub enum Action {
    CanvasStart {from:[f32;2],to:[f32;2],size:[f64;2],modified:bool,pointer:i32},
    CanvasMove {screen:[f32;2],size:[f64;2]},
    CanvasRelease {screen:[f32;2],size:[f64;2],add:bool,remove:bool},
```

Replace it with:

```rust
use crate::{background::Background, camera::Camera, history::History, scene::{ObjectId, Scene}};

pub enum Action {
    StreamRefresh,
    CanvasStart {from:[f32;2],to:[f32;2],size:[f64;2],modified:bool,pointer:i32},
    CanvasMove {screen:[f32;2],size:[f64;2]},
    CanvasRelease {screen:[f32;2],size:[f64;2],add:bool,remove:bool},
```

### 4. `src/scene_archive.rs`

Connect streamed selection and original identities while refusing incomplete editable Save.

**Locate this existing block:**

```rust

pub fn save(scene:&Scene)->Result<Vec<u8>,String> {
    let mut namespaces=HashSet::new();let mut documents=Vec::new();let mut bytes_used=0usize;
    let mut metadata=Metadata {created_doc:None,current_layer:None,documents:vec![],hidden:vec![],locked:vec![],colors:vec![],edge_colors:Some(vec![]),texts:vec![],groups:vec![]};
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/68n-resolve-compact-picks-to-original-point-ids-09.rs"
```

## <a id="checkpoint-68o-account-for-original-and-retained-source-data"></a>17. Account for and retained source data

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 17 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/memory.rs, src/stream_r…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">scene snapshots</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained pages</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">independent counts</text>
</svg>
</div>

**What this code does:** Account for original and retained source data.

### 1. `src/memory.rs`

Count shared streamed sources and retained payload separately.

**Locate this existing block:**

```rust
    usage[1]=seen.len();usage
}
```

After that block, type:

```rust
--8<-- "journey/code/68o-account-for-original-and-retained-source-data-01.rs" skip=32
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/stream_row_tests.rs`

Copy streamed identity, placement, accounting and lifetime regressions.

Copy this supplied file:

```text
--8<-- "journey/code/68o-account-for-original-and-retained-source-data-02.rs"
```

### 3. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust

pub mod range_example;
```

After that block, type:

```rust
--8<-- "journey/code/68o-account-for-original-and-retained-source-data-03.rs" skip=24
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/lib.rs`

Keep regression fixtures outside browser builds.

**Locate this existing block:**

```text
pub mod range_example;

pub mod stream_row_tests;
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/68o-account-for-original-and-retained-source-data-04.rs"
```

## <a id="checkpoint-68p-keep-original-attributes-in-ranged-pages"></a>18. Keep attributes in ranged pages

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 18 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/stream.rs, src/range_so…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">coordinate and ID range</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">matching attributes</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">checked original page</text>
</svg>
</div>

**What this code does:** Keep original attributes in ranged pages.

### 1. `src/stream.rs`

Validate present colour and normal columns before decoding.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            body_end(fields.normals_at, fields.normals_len, fields.end)?,
        )?;
        let raw = source_range(url, at, length, &fields.revision).await?;
        (raw.len() as u64 == length).then(|| normals_from(&raw))
    }
```

Replace it with:

```rust
            body_end(fields.normals_at, fields.normals_len, fields.end)?,
        )?;
        let raw = source_range(url, at, length, &fields.revision).await?;
        if raw.chunks_exact(8).any(|bytes|!f64::from_le_bytes(bytes.try_into().unwrap()).is_finite()) {return None;}
        (raw.len() as u64 == length).then(|| normals_from(&raw))
    }
```

**Locate this block:**

```rust
                seen[field - 8] = true;
            }

            if field == 5 && length == fields.coords_len {
                normals = (body, length);
            }
```

Replace it with:

```rust
                seen[field - 8] = true;
            }

            if field == 5 {
                if length!=fields.coords_len||normals.1!=0 {return None;}
                normals = (body, length);
            }
```

**Locate this block:**

```rust
        for k in &mut rgba {
            let (v, n) = varint(raw, i)?;
            i += n;
            *k = (v & 255) as u8;
        }

        out.push(u32::from_le_bytes(rgba));
```

Replace it with:

```rust
        for k in &mut rgba {
            let (v, n) = varint(raw, i)?;
            i += n;
            *k = u8::try_from(v).ok()?;
        }

        out.push(u32::from_le_bytes(rgba));
```

### 2. `src/range_source.rs`

Read matching original attributes for each source page.

**Locate this existing block:**

```rust
        crate::range_page::Page::decode(rows,self.fields.count,&raw,ids.as_deref()).map_err(str::to_owned)
    }

}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/68p-keep-original-attributes-in-ranged-pages-04.rs"
```

## <a id="checkpoint-68q-retain-page-owners-beside-gpu-buffers"></a>19. Retain page owners beside GPU buffers

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 19 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/cloud_gpu.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">charged original page</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">derived GPU columns</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained pick identity</text>
</svg>
</div>

**What this code does:** Retain page owners beside GPU buffers.

### 1. `src/cloud_gpu.rs`

Retain charged original page owners with their derived GPU columns.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
      .flat_map(u32::to_ne_bytes)
      .collect::<Vec<_>>();
    Ok(Self {
      positions: buffer(&ctx.device, "original cloud positions", &positions),
      colors: buffer(&ctx.device, "original cloud colors", &colors),
      normals: buffer(&ctx.device, "original cloud normals", &normals),
      has_normals,
    })
  }
  fn bytes(&self) -> u64 {
    self.positions.size() + self.colors.size() + self.normals.size()
  }
```

Replace it with:

```rust
      .flat_map(u32::to_ne_bytes)
      .collect::<Vec<_>>();
    Ok(Self {
      _page:None,
      positions: buffer(&ctx.device, "original cloud positions", &positions),
      colors: buffer(&ctx.device, "original cloud colors", &colors),
      normals: buffer(&ctx.device, "original cloud normals", &normals),
      has_normals,
    })
  }
  fn page(ctx:&GpuCtx,held:Rc<crate::page_cache::Held<crate::range_page::Page>>)->Result<Self,&'static str> {
    let page=&held.value;let count=page.rows.end-page.rows.start;
    if u64::from(count)*12>u64::from(ctx.device.limits().max_storage_buffer_binding_size) {return Err("Cloud page exceeds the device storage binding limit");}
    let positions=page.display.iter().flat_map(|value|value.to_ne_bytes()).collect::<Vec<_>>();let colors=(0..count as usize).flat_map(|i|page.colors.get(i).copied().unwrap_or(0xff00_0000).to_ne_bytes()).collect::<Vec<_>>();let normals=page.normals.iter().flat_map(|v|v.to_ne_bytes()).collect::<Vec<_>>();
    Ok(Self {positions:buffer(&ctx.device,"streamed source positions",&positions),colors:buffer(&ctx.device,"streamed source colours",&colors),normals:buffer(&ctx.device,"streamed source normals",&normals),has_normals:!page.normals.is_empty(),_page:Some(held)})
  }
  fn bytes(&self) -> u64 {
    self.positions.size() + self.colors.size() + self.normals.size()
  }
```

**Locate this block:**

```rust
  }
}
struct Columns {
  positions: wgpu::Buffer,
  colors: wgpu::Buffer,
  normals: wgpu::Buffer,
```

Replace it with:

```rust
  }
}
struct Columns {
  _page:Option<Rc<crate::page_cache::Held<crate::range_page::Page>>>,
  positions: wgpu::Buffer,
  colors: wgpu::Buffer,
  normals: wgpu::Buffer,
```

## <a id="checkpoint-68r-describe-resident-and-streamed-draw-rows"></a>20. Describe resident and streamed draw rows

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 20 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/cloud_gpu.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source owner</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">placed draw row</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">compact page address</text>
</svg>
</div>

**What this code does:** Describe resident and streamed draw rows.

### 1. `src/cloud_gpu.rs`

Describe source ownership and compact retained-page addresses independently.

**Locate this existing block:**

```rust
    usage: wgpu::BufferUsages::STORAGE,
  })
}
struct Draw {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/68r-describe-resident-and-streamed-draw-rows-01.rs"
```

## <a id="checkpoint-68s-draw-shared-source-pages-at-independent-placements"></a>21. Draw shared source pages at independent placements

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 21 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/cloud_gpu.rs, src/cloud…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared page columns</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">clipped LOD range</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">visible pixels</text>
</svg>
</div>

**What this code does:** Draw shared source pages at independent placements.

### 1. `src/cloud_gpu.rs`

Connect resident and streamed original columns to compact bounded draw records.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
      .map(f32::to_bits)
      .into_iter()
      .chain([rgb[0].to_bits(), rgb[1].to_bits(), rgb[2].to_bits(), (px * density * 0.5).max(0.5).to_bits()])
      .chain([range.first, range.count, cumulative, radius_factor(range, px, scale, view.ortho_h).to_bits()])
      .chain(rot.map(f32::to_bits))
      .chain([if draw.columns.has_normals { range.first } else { u32::MAX }, instance, flags, 0]);
    bytes.extend(words.flat_map(u32::to_ne_bytes));
  }
  bytes[..4].copy_from_slice(&(ranges.len() as u32).to_ne_bytes());
```

Replace it with:

```rust
      .map(f32::to_bits)
      .into_iter()
      .chain([rgb[0].to_bits(), rgb[1].to_bits(), rgb[2].to_bits(), (px * density * 0.5).max(0.5).to_bits()])
      .chain([range.first-row.rows.start, range.count, cumulative, radius_factor(range, px, scale, view.ortho_h).to_bits()])
      .chain(rot.map(f32::to_bits))
      .chain([if draw.columns.has_normals { range.first-row.rows.start } else { u32::MAX }, instance, flags, row.address]);
    bytes.extend(words.flat_map(u32::to_ne_bytes));
  }
  bytes[..4].copy_from_slice(&(ranges.len() as u32).to_ne_bytes());
```

**Locate this block:**

```rust
    model[14],
  ];
  let rgb = draw.color.map_or([1., 1., 1.], |c| c.map(|v| v as f32 / 255.));
  let px = if row.point_px > 0. { row.point_px } else { source.point_size as f32 };
  let flags = u32::from(draw.selected) | if draw.color.is_some() { 256 } else { 0 };
  let instance = if draw.pickable { row.id.value() - 1 } else { u32::MAX };
  let mut bytes = vec![0; 16];
```

Replace it with:

```rust
    model[14],
  ];
  let rgb = draw.color.map_or([1., 1., 1.], |c| c.map(|v| v as f32 / 255.));
  let px = row.point_px;
  let flags = u32::from(draw.selected) | if draw.color.is_some() { 256 } else { 0 };
  let instance = if draw.pickable { row.id.value() - 1 } else { u32::MAX };
  let mut bytes = vec![0; 16];
```

**Locate this block:**

```rust
  ranges: &[crate::cloud_lod::Range],
) -> Result<(Vec<u8>, u32), &'static str> {
  let row = &draw.row;
  let source = &row.prepared.source;
  let model = row.model.to_f32();
  let scale = row.model.uniform_scale();
  let mut total = 0u32;
```

Replace it with:

```rust
  ranges: &[crate::cloud_lod::Range],
) -> Result<(Vec<u8>, u32), &'static str> {
  let row = &draw.row;
  let model = row.model.to_f32();
  let scale = row.model.uniform_scale();
  let mut total = 0u32;
```

**Locate this block:**

```rust
        ortho_h: view.ortho_h,
        height_px: size[1],
        lod_px: if row.controls_visible {0.} else {view.lod_px},
        nodes: &row.prepared.nodes,
      };
      walk.select(&projection, &row.prepared.descriptor(), &model);
      records_used += walk.ranges.len();
      if records_used > MAX_RECORDS {
        return Err("Cloud view exceeds the 4096-record budget");
```

Replace it with:

```rust
        ortho_h: view.ortho_h,
        height_px: size[1],
        lod_px: if row.controls_visible {0.} else {view.lod_px},
        nodes: &row.nodes,
      };
      walk.select(&projection, &row.descriptor, &model);
      walk.ranges.retain_mut(|range| {let start=range.first.max(row.rows.start);let end=(range.first+range.count).min(row.rows.end);range.first=start;range.count=end.saturating_sub(start);range.count>0});
      records_used += walk.ranges.len();
      if records_used > MAX_RECORDS {
        return Err("Cloud view exceeds the 4096-record budget");
```

**Locate this block:**

```rust
    scene: &crate::scene::Scene,
    selected: &[crate::scene::ObjectId],
  ) -> Result<(), &'static str> {
    let mut cache: HashMap<usize, Rc<Columns>> =
      self.draws.iter().map(|d| (Rc::as_ptr(&d.row.prepared.source) as usize, d.columns.clone())).collect();
    let mut old = std::mem::take(&mut self.draws);
    let mut draws = Vec::with_capacity(scene.clouds().len());
    for row in scene.clouds() {
      let address = Rc::as_ptr(&row.prepared.source) as usize;
      let columns = if let Some(columns) = cache.get(&address) {
        columns.clone()
      } else {
        let columns = Rc::new(Columns::new(ctx, row)?);
        self.uploads += 1;
        cache.insert(address, columns.clone());
        columns
      };
      let (records, group) =
        if let Some(i) = old.iter().position(|d| d.row.id == row.id && Rc::ptr_eq(&d.columns, &columns)) {
          let d = old.swap_remove(i);
          (d.records, d.group)
        } else {
          let records = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("bounded cloud draw records"),
            size: 16,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
          });
          let group = resource_group(
            ctx,
            &self.columns_layout,
            "cloud source and records",
            [
              (0, records.as_entire_binding()),
              (1, columns.positions.as_entire_binding()),
              (2, columns.colors.as_entire_binding()),
              (3, columns.normals.as_entire_binding()),
            ],
          );
          (records, group)
        };
      draws.push(Draw {
        row: row.clone(),
        columns,
        records,
        group,
        visible: scene.visible(row.id),
        pickable: scene.pick_id(row.id.value()).is_some(),
        selected: selected.contains(&row.id),
        color: scene.view.faces.get(&row.id).copied(),
        total: 0,
      });
    }
    self.draws = draws;
    self.revision = self.revision.wrapping_add(1);
```

Replace it with:

```rust
    scene: &crate::scene::Scene,
    selected: &[crate::scene::ObjectId],
  ) -> Result<(), &'static str> {
    let mut cache:HashMap<(usize,u32,u32),Rc<Columns>>=self.draws.iter().map(|draw|(draw.row.key(),draw.columns.clone())).collect();
    let mut candidates=Vec::new();
    for row in scene.clouds() {let view=RowView::resident(row);let key=view.key();let columns=if let Some(columns)=cache.get(&key) {columns.clone()} else {let columns=Rc::new(Columns::new(ctx,row)?);self.uploads+=1;cache.insert(key,columns.clone());columns};candidates.push((view,columns));}
    for row in scene.streams() {let mut address=0u32;for page in row.prepared.pages() {
      let view=RowView::streamed(row,page.value.rows.clone(),address);let key=view.key();let columns=if let Some(columns)=cache.get(&key) {columns.clone()} else {let columns=Rc::new(Columns::page(ctx,page.clone())?);self.uploads+=1;cache.insert(key,columns.clone());columns};address=address.checked_add(page.value.rows.end-page.value.rows.start).filter(|value|*value<=crate::sub_id::LIMIT).ok_or("Streamed pick addresses exceed their source table")?;candidates.push((view,columns));
    }}
    let mut old=std::mem::take(&mut self.draws);let mut draws=Vec::with_capacity(candidates.len());
    for (row,columns) in candidates {
      let (records,group)=if let Some(i)=old.iter().position(|draw|draw.row.id==row.id&&draw.row.rows==row.rows&&Rc::ptr_eq(&draw.columns,&columns)) {let draw=old.swap_remove(i);(draw.records,draw.group)} else {
        let records=ctx.device.create_buffer(&wgpu::BufferDescriptor {label:Some("bounded cloud draw records"),size:16,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});
        let group=resource_group(ctx,&self.columns_layout,"cloud source and records",[(0,records.as_entire_binding()),(1,columns.positions.as_entire_binding()),(2,columns.colors.as_entire_binding()),(3,columns.normals.as_entire_binding())]);(records,group)
      };
      let id=row.id;draws.push(Draw {row,columns,records,group,visible:scene.visible(id),pickable:scene.pick_id(id.value()).is_some(),selected:selected.contains(&id),color:scene.view.faces.get(&id).copied(),total:0});
    }
    self.draws = draws;
    self.revision = self.revision.wrapping_add(1);
```

**Locate this block:**

```rust
  fn key(&self)->(usize,u32,u32) {let (source,first)=self.keep.key(self.rows.start);(source,first,self.rows.end)}
}
struct Draw {
  row: Row,
  columns: Rc<Columns>,
  records: wgpu::Buffer,
  group: wgpu::BindGroup,
  visible: bool,
  pickable: bool,
  selected: bool,
  color: Option<[u8; 3]>,
  total: u32,
}
struct Image {
  texture: wgpu::Texture,
```

Replace it with:

```rust
  fn key(&self)->(usize,u32,u32) {let (source,first)=self.keep.key(self.rows.start);(source,first,self.rows.end)}
}
struct Draw {
  row:RowView,
  columns:Rc<Columns>,
  records:wgpu::Buffer,
  group:wgpu::BindGroup,
  visible:bool,
  pickable:bool,
  selected:bool,
  color:Option<[u8;3]>,
  total:u32,
}
struct Image {
  texture: wgpu::Texture,
```

### 2. `src/cloud_points.wgsl`

Connect resident and streamed original columns to compact bounded draw records.

**Locate this existing block:**

```wgsl
        vec4<f32>(rec_f(base, 12u), rec_f(base, 13u), rec_f(base, 14u), rec_f(base, 15u)),
    );
    s.row = i;
    s.instance = table[base + 37u];
    let point = vec3<f32>(positions[i * 3u], positions[i * 3u + 1u], positions[i * 3u + 2u]);
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
--8<-- "journey/code/68s-draw-shared-source-pages-at-independent-placements-07.rs"
```

### 3. `src/renderer.rs`

Connect resident and streamed original columns to compact bounded draw records.

**Locate this existing block:**

```rust
    pub fn cloud_error(&self)->Option<String> {self.cloud_error.borrow().clone()}
    fn set_clouds(&self,scene:&Scene,selected:&[ObjectId])->Result<(),&'static str> {
        if scene.clouds().is_empty() {self.clouds.borrow_mut().take();self.cloud_error.borrow_mut().take();return Ok(());}
        let mut owner=self.clouds.borrow_mut();let lane=owner.get_or_insert_with(||crate::cloud_gpu::Lane::new(&self.text_context(),self.format));lane.set_scene(&self.text_context(),scene,selected)
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/68s-draw-shared-source-pages-at-independent-placements-08.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/pick_gpu_tests.rs`

Copy actual GPU source ID, page ownership, colour, reuse, eviction and cleanup fixtures.

**Locate this existing block:**

```text
    scene.cloud_controls(None);renderer.set_scene(&scene,None);renderer.set_pick_mode(Mode::Object);camera_pixels(&mut renderer,&camera);assert_eq!(renderer.cloud_usage()[0],64);assert_eq!(renderer.cloud_usage()[2],uploads);
    scene.clear();renderer.set_scene(&scene,None);assert_eq!(renderer.cloud_usage(),[0;5]);
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/68s-draw-shared-source-pages-at-independent-placements-09.rs" skip=319
```

## <a id="checkpoint-68t-validate-encoded-source-metadata"></a>22. Validate encoded source metadata

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 22 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/source_encoding.rs, src…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">manifest encoding</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">supported codec</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">declared decoded limit</text>
</svg>
</div>

**What this code does:** Validate encoded source metadata.

### 1. `src/source_encoding.rs`

Validate supported source encodings and exact decoded size.

Create the file and type:

```rust
--8<-- "journey/code/68t-validate-encoded-source-metadata-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
#[cfg(test)]
mod stream_row_tests;
```

After that block, type:

```rust
--8<-- "journey/code/68t-validate-encoded-source-metadata-02.rs" skip=35
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/source_encoding.rs`

Copy encoded size and codec refusal fixtures.

**Locate this existing block:**

```text
        if bytes>crate::archive::LIMIT||self.decoded.is_some_and(|size|size!=bytes) {return Err("Decoded source size does not match its manifest".into());}Ok(())
    }
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/68t-validate-encoded-source-metadata-03.rs" skip=169
```

## <a id="checkpoint-68u-decode-bounded-cancellable-source-streams"></a>23. Decode bounded cancellable source streams

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 23 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Cargo.toml, src/source_unpa…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">packed source</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">bounded chunks</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">exact original bytes</text>
</svg>
</div>

**What this code does:** Decode bounded cancellable source streams.

### 1. `Cargo.toml`

Copy the pinned browser stream dependency configuration.

Replace the entire file with:

```text
--8<-- "journey/code/68u-decode-bounded-cancellable-source-streams-cargo.toml"
```

### 2. `src/source_unpack.rs`

Decode bounded cancellable original source streams.

Create the file and type:

```rust
--8<-- "journey/code/68u-decode-bounded-cancellable-source-streams-01.rs"
```

### 3. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust

pub mod source_encoding;
```

After that block, type:

```rust
--8<-- "journey/code/68u-decode-bounded-cancellable-source-streams-02.rs" skip=26
```

## <a id="checkpoint-68v-refuse-conflicting-source-encoding-revisions"></a>24. Refuse conflicting source encoding revisions

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 24 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/publication_index.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">new manifest</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original inventory</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">atomic replacement</text>
</svg>
</div>

**What this code does:** Refuse conflicting source encoding revisions.

### 1. `src/publication_index.rs`

Validate manifest source formats before changing the original inventory.

Replace the entire file with:

```rust
use crate::{manifest::Manifest,scene::Scene};
use std::{collections::{HashMap,HashSet},rc::Rc};

pub struct Index {
    base:String,
    revisions:crate::live_revision::Revisions,
    manifest:Option<Manifest>,
    sources:HashMap<String,Scene>,
    sizes:HashMap<String,usize>,
    pending:bool,
}
impl Index {
    pub fn new(base:String)->Self {Self {base,revisions:Default::default(),manifest:None,sources:HashMap::new(),sizes:HashMap::new(),pending:false}}
    pub fn etag(&self,url:&str)->Option<String> {self.revisions.etag(url)}
    pub fn has_manifest(&self)->bool {self.manifest.is_some()}
    pub fn cached(&self,url:&str)->bool {self.sources.contains_key(url)}
    pub fn forget(&mut self,url:&str) {self.revisions.forget(url);}
    pub fn urls(&self)->Vec<String> {self.manifest.as_ref().map_or_else(Vec::new,|manifest|manifest.items.iter().map(|item|crate::publication_route::join(&self.base,&item.file)).collect())}
    pub fn encoding(&self,url:&str)->Result<crate::source_encoding::Encoding,String> {
        let manifest=self.manifest.as_ref().ok_or("Source manifest is missing")?;let mut encoding=None;
        for item in &manifest.items {if crate::publication_route::join(&self.base,&item.file)==url {
            let value=crate::source_encoding::Encoding::new(&item.encoding,item.size)?;
            if encoding.as_ref().is_some_and(|previous|previous!=&value) {return Err("Repeated source has conflicting encoding metadata".into());}encoding=Some(value);
        }}encoding.ok_or_else(||"Source URL is absent from its manifest".into())
    }
    pub fn manifest(&mut self,url:&str,etag:Option<String>,bytes:&[u8])->Result<bool,String> {
        if !self.revisions.changed(url,etag,bytes) {return Ok(false);}
        let manifest=match Manifest::parse(bytes) {Ok(value)=>value,Err(error)=>{self.revisions.forget(url);return Err(error);}};
        let mut formats=HashMap::new();
        for item in &manifest.items {
            let format=match crate::source_encoding::Encoding::new(&item.encoding,item.size) {Ok(format)=>format,Err(error)=>{self.revisions.forget(url);return Err(error);}};
            let key=crate::publication_route::join(&self.base,&item.file);
            if formats.insert(key,format.clone()).is_some_and(|previous|previous!=format) {self.revisions.forget(url);return Err("Repeated source has conflicting encoding metadata".into());}
        }
        self.manifest=Some(manifest);self.pending=true;let used=self.urls().into_iter().collect::<HashSet<_>>();self.sources.retain(|url,_|used.contains(url));self.sizes.retain(|url,_|used.contains(url));Ok(true)
    }
    pub fn source(&mut self,url:&str,etag:Option<String>,bytes:&[u8])->Result<bool,String> {
        if !self.revisions.changed(url,etag,bytes) {return Ok(false);}
        let size=self.sizes.iter().filter(|(key,_)|key.as_str()!=url).try_fold(bytes.len(),|sum,(_,size)|sum.checked_add(*size)).ok_or("Source inventory size overflow")?;
        if size>crate::archive::LIMIT {self.revisions.forget(url);return Err("Source inventory exceeds the 512 MiB file limit".into());}
        let source=match crate::publication::prepare(&Scene::empty(),bytes,true,Some(Rc::new(crate::reload_url::ReloadUrl::new(url.into())))) {Ok(source)=>source,Err(error)=>{self.revisions.forget(url);return Err(error);}};
        self.sources.insert(url.into(),source);self.sizes.insert(url.into(),bytes.len());self.pending=true;Ok(true)
    }
    pub fn replacement(&mut self,previous:&Scene)->Result<Option<Scene>,String> {
        if !self.pending {return Ok(None);}
        let manifest=self.manifest.as_ref().ok_or("Live manifest is missing")?;
        for (item,url) in manifest.items.iter().zip(self.urls()) {if let Some(expected)=item.encoded_size() {if self.sizes.get(&url).copied().map(|size|size as u64)!=Some(expected) {self.revisions.forget(&url);return Err("Encoded source decoded size does not match the manifest".into());}}}
        let scene=crate::scene_manifest::prepare(previous,manifest,&self.urls(),&self.sources)?;self.pending=false;Ok(Some(scene))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validators_and_unchanged_bytes_keep_original_sources_and_viewer_ids() {
        let mut index=Index::new("files/".into());let manifest=b"items: [{file: mesh.pb}]";let raw=crate::specimen::bytes();let mut scene=Scene::empty();assert!(index.manifest("scene",Some("a".into()),manifest).unwrap());assert!(index.source("files/mesh.pb",Some("b".into()),&raw).unwrap());scene=index.replacement(&scene).unwrap().unwrap();let id=scene.objects()[0].id;let geometry=Rc::clone(scene.objects()[0].geometry().unwrap());
        assert!(!index.manifest("scene",Some("W/c".into()),manifest).unwrap());assert!(!index.source("files/mesh.pb",None,&raw).unwrap());assert_eq!(index.etag("scene"),Some("W/c".into()));assert_eq!(index.etag("files/mesh.pb"),None);assert!(index.replacement(&scene).unwrap().is_none());assert_eq!(scene.objects()[0].id,id);assert!(Rc::ptr_eq(scene.objects()[0].geometry().unwrap(),&geometry));
        assert!(index.source("files/mesh.pb",None,&crate::specimen::precise_bytes()).unwrap());let changed=index.replacement(&scene).unwrap().unwrap();assert_ne!(changed.objects()[0].id,id);assert!(!Rc::ptr_eq(changed.objects()[0].geometry().unwrap(),&geometry));assert!(Rc::ptr_eq(scene.objects()[0].geometry().unwrap(),&geometry));
    }
    #[test]
    fn invalid_or_incomplete_live_updates_keep_the_last_display_and_retry_failed_bytes() {
        let mut index=Index::new(String::new());let manifest=b"items: [{file: mesh.pb}]";let raw=crate::specimen::bytes();index.manifest("scene",None,manifest).unwrap();assert!(index.replacement(&Scene::empty()).is_err());index.source("mesh.pb",Some("good".into()),&raw).unwrap();let scene=index.replacement(&Scene::empty()).unwrap().unwrap();let id=scene.objects()[0].id;
        assert!(index.source("mesh.pb",Some("bad".into()),b"broken").is_err());assert_eq!(index.etag("mesh.pb"),None);assert!(index.replacement(&scene).unwrap().is_none());assert_eq!(scene.objects()[0].id,id);assert!(index.manifest("scene",None,b"invalid: manifest").is_err());assert_eq!(index.etag("scene"),None);assert!(index.manifest("scene",None,manifest).unwrap());assert!(index.replacement(&scene).unwrap().is_some());
        let mesh=Rc::downgrade(scene.objects()[0].geometry().unwrap());drop(scene);index.manifest("scene",None,b"items: []").unwrap();assert!(mesh.upgrade().is_none());assert!(!index.cached("mesh.pb"));
    }
    #[test]
    fn refused_encoding_changes_keep_the_existing_manifest_and_original_sources() {
        let mut index=Index::new("files/".into());let raw=crate::specimen::bytes();index.manifest("scene",None,b"items: [{file: mesh.pb}]").unwrap();index.source("files/mesh.pb",None,&raw).unwrap();let scene=index.replacement(&Scene::empty()).unwrap().unwrap();let original=Rc::clone(scene.objects()[0].geometry().unwrap());
        for manifest in [b"items: [{file: mesh.pb, encoding: gzip, size: 2}, {file: mesh.pb, encoding: gzip, size: 3}]".as_slice(),b"items: [{file: mesh.pb, encoding: brotli, size: 24}]".as_slice()] {
            assert!(index.manifest("scene",Some("bad".into()),manifest).is_err());assert_eq!(index.etag("scene"),None);assert_eq!(index.urls(),["files/mesh.pb"]);assert!(index.replacement(&scene).unwrap().is_none());assert!(Rc::ptr_eq(scene.objects()[0].geometry().unwrap(),&original));
        }
        assert!(index.manifest("scene",None,b"items: [{file: mesh.pb}]").unwrap());assert!(index.replacement(&scene).unwrap().is_some());
    }

}
```

## <a id="checkpoint-68w-adopt-remote-original-revisions-independently"></a>25. Adopt remote revisions independently

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 25 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/publication_index.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">remote validator</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">manifest placements</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">independent viewer rows</text>
</svg>
</div>

**What this code does:** Adopt remote original revisions independently.

### 1. `src/publication_index.rs`

Adopt versioned remote originals while retaining complete-source byte identity for whole documents.

**Locate this existing block:**

```rust
        self.sources.insert(url.into(),source);self.sizes.insert(url.into(),bytes.len());self.pending=true;Ok(true)
    }
    pub fn replacement(&mut self,previous:&Scene)->Result<Option<Scene>,String> {
        if !self.pending {return Ok(None);}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/68w-adopt-remote-original-revisions-independently-01.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/publication_index.rs`

Copy remote revision, placement and previous-scene retention fixtures.

**Locate this existing block:**

```text
    }

}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/68w-adopt-remote-original-revisions-independently-02.rs" skip=9
```

## <a id="checkpoint-68x-prepare-a-coarse-pin-and-bounded-prefix"></a>26. Prepare a coarse pin and bounded prefix

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 26 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/stream_load.rs, src/lib…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source metadata</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">bounded prefix</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">initial source owner</text>
</svg>
</div>

**What this code does:** Prepare a coarse pin and bounded prefix.

### 1. `src/stream_load.rs`

Prepare a pinned original coarse page and a budgeted initial prefix.

Create the file and type:

```rust
--8<-- "journey/code/68x-prepare-a-coarse-pin-and-bounded-prefix-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
#[cfg(target_arch = "wasm32")]
pub mod source_unpack;
```

After that block, type:

```rust
--8<-- "journey/code/68x-prepare-a-coarse-pin-and-bounded-prefix-02.rs" skip=54
```

## <a id="checkpoint-68y-redraw-before-reusing-evicted-page-addresses"></a>27. Redraw before reusing evicted page addresses

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 27 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/stream_load.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">explicit eviction</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">renderer release</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reserved page</text>
</svg>
</div>

**What this code does:** Redraw before reusing evicted page addresses.

### 1. `src/stream_load.rs`

Synchronize visible eviction before reusing released page memory.

**Locate this existing block:**

```rust
    }
    Err("Empty streamed source page".into())
}
```

After that block, type:

```rust
--8<-- "journey/code/68y-redraw-before-reusing-evicted-page-addresses-01.rs" skip=54
```

## <a id="checkpoint-68z-continue-pages-only-for-current-source-owners"></a>28. Continue pages only for current source owners

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 28 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/stream_load.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">current owner</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">revision-bound read</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">redraw</text>
</svg>
</div>

**What this code does:** Continue pages only for current source owners.

### 1. `src/stream_load.rs`

Continue checked source pages only while their scene/runtime owner remains current.

**Locate this existing block:**

```rust
    Ok(Cursor {prepared,from:count,color_at,loaded:count,ceiling:count})
}
```

After that block, type:

```rust
--8<-- "journey/code/68z-continue-pages-only-for-current-source-owners-01.rs" skip=76
```

## <a id="checkpoint-68aa-own-cancellation-for-each-published-source"></a>29. Own cancellation for each published source

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 29 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/range_fetch.rs, src/bro…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">publication owner</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">closed generation</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">aborted ranges</text>
</svg>
</div>

**What this code does:** Own cancellation for each published source.

### 1. `src/range_fetch.rs`

Cancel active range reads belonging to replaced source URLs.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    let (start,length)=opts.range.ok_or("This reader requires a byte range")?;
    if length==0 {return Ok(Reply {status:206,etag:None,total:None,bytes:vec![]});}
    let range=crate::range_rules::header(start,length)?;
    let mut read=Read::new()?;let init=web_sys::RequestInit::new();init.set_signal(Some(&read.controller.signal()));
    init.set_mode(web_sys::RequestMode::Cors);
    if opts.revalidate {init.set_cache(web_sys::RequestCache::NoCache);}
    let headers=web_sys::Headers::new().map_err(|error|describe(error.into()))?;headers.set("Range",&range).map_err(|error|describe(error.into()))?;init.set_headers(&headers);
```

Replace it with:

```rust
    let (start,length)=opts.range.ok_or("This reader requires a byte range")?;
    if length==0 {return Ok(Reply {status:206,etag:None,total:None,bytes:vec![]});}
    let range=crate::range_rules::header(start,length)?;
    let mut read=Read::new(url)?;let init=web_sys::RequestInit::new();init.set_signal(Some(&read.controller.signal()));
    init.set_mode(web_sys::RequestMode::Cors);
    if opts.revalidate {init.set_cache(web_sys::RequestCache::NoCache);}
    let headers=web_sys::Headers::new().map_err(|error|describe(error.into()))?;headers.set("Range",&range).map_err(|error|describe(error.into()))?;init.set_headers(&headers);
```

**Locate this block:**

```rust
    callback:Closure<dyn FnMut()>,
}
impl Read {
    fn new()->Result<Self,String> {
        let controller=web_sys::AbortController::new().map_err(|error|describe(error.into()))?;
        let key=NEXT.with(|next| {let key=next.get().checked_add(1).ok_or("Range request IDs exhausted")?;next.set(key);Ok::<_,String>(key)})?;
        let stalled=Rc::new(Cell::new(false));let owned=controller.clone();let flag=stalled.clone();
        let callback=Closure::<dyn FnMut()>::new(move || {flag.set(true);owned.abort();});
        let mut read=Self {key,controller,timer:0,stalled,callback};read.touch()?;
        REQUESTS.with(|requests|requests.borrow_mut().insert(key,read.controller.clone()));Ok(read)
    }
    fn touch(&mut self)->Result<(),String> {
        let window=web_sys::window().ok_or("No browser window")?;window.clear_timeout_with_handle(self.timer);
```

Replace it with:

```rust
    callback:Closure<dyn FnMut()>,
}
impl Read {
    fn new(url:&str)->Result<Self,String> {
        let controller=web_sys::AbortController::new().map_err(|error|describe(error.into()))?;
        let key=NEXT.with(|next| {let key=next.get().checked_add(1).ok_or("Range request IDs exhausted")?;next.set(key);Ok::<_,String>(key)})?;
        let stalled=Rc::new(Cell::new(false));let owned=controller.clone();let flag=stalled.clone();
        let callback=Closure::<dyn FnMut()>::new(move || {flag.set(true);owned.abort();});
        let mut read=Self {key,controller,timer:0,stalled,callback};read.touch()?;
        REQUESTS.with(|requests|requests.borrow_mut().insert(key,(url.to_owned(),read.controller.clone())));Ok(read)
    }
    fn touch(&mut self)->Result<(),String> {
        let window=web_sys::window().ok_or("No browser window")?;window.clear_timeout_with_handle(self.timer);
```

**Locate this block:**

```rust
pub use crate::range_rules::PROBE_BYTES;

thread_local! {
    static REQUESTS:RefCell<BTreeMap<u64,web_sys::AbortController>>=const {RefCell::new(BTreeMap::new())};
    static NEXT:Cell<u64>=const {Cell::new(0)};
}

pub fn cancel() {
    for controller in REQUESTS.with(|requests|std::mem::take(&mut *requests.borrow_mut())).into_values() {controller.abort();}
}

struct Read {
    key:u64,
    controller:web_sys::AbortController,
```

Replace it with:

```rust
pub use crate::range_rules::PROBE_BYTES;

thread_local! {
    static REQUESTS:RefCell<BTreeMap<u64,(String,web_sys::AbortController)>>=const {RefCell::new(BTreeMap::new())};
    static NEXT:Cell<u64>=const {Cell::new(0)};
}

pub fn cancel() {
    for (_,controller) in REQUESTS.with(|requests|std::mem::take(&mut *requests.borrow_mut())).into_values() {controller.abort();}
}

pub fn cancel_url(url:&str) {REQUESTS.with(|requests|requests.borrow_mut().retain(|_,(source,controller)| {if source==url {controller.abort();false} else {true}}));}

struct Read {
    key:u64,
    controller:web_sys::AbortController,
```

### 2. `src/browser_publication.rs`

Own pending source cursors and close every source reader with its publication.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        poll(state);
    }
    pub fn cancel(&self) {
        let Some(state)=self.state() else {return;};let mut state=state.borrow_mut();state.active=false;state.gate.cancel();state.delivery=None;state.index=None;
        if let Some(abort)=state.abort.take() {abort.abort();}if let Some(id)=state.timer.take() {if let Some(window)=web_sys::window() {window.clear_interval_with_handle(id);}}
        if let Some(source)=state.notify.take() {source.set_onmessage(None);source.close();}state.message=None;state.tick=None;
    }
```

Replace it with:

```rust
        poll(state);
    }
    pub fn cancel(&self) {
        let Some(state)=self.state() else {return;};let mut state=state.borrow_mut();state.active=false;state.cancelled.set(true);for source in state.streams.drain(..).filter_map(|s|s.upgrade()) {source.generation.borrow_mut().close();}state.pending_streams.clear();crate::stream::reset_range_gate();state.gate.cancel();state.delivery=None;state.index=None;
        if let Some(abort)=state.abort.take() {abort.abort();}if let Some(id)=state.timer.take() {if let Some(window)=web_sys::window() {window.clear_interval_with_handle(id);}}
        if let Some(source)=state.notify.take() {source.set_onmessage(None);source.close();}state.message=None;state.tick=None;
    }
```

**Locate this block:**

```rust
        let window=web_sys::window().ok_or("No browser window")?;let url=web_sys::Url::new(&window.location().href()?)?;let q=url.search_params();
        let scene=q.get("scene");let data=q.get("data");let live=q.get("live");let poll=q.get("poll");let notify=q.get("notify");
        let Some(route)=crate::publication_route::select(scene.as_deref(),&window.location().pathname()?,data.as_deref(),live.as_deref(),poll.as_deref(),notify.as_deref()) else {return Ok(Self(None));};
        let index=Index::new(route.base.clone());Ok(Self(Some(Rc::new(RefCell::new(State {active:true,inflight:false,again:false,gate:Default::default(),abort:None,route,index:Some(index),document,fault,delivery:None,last_error:None,timer:None,tick:None,notify:None,message:None})))))
    }
    pub fn control(&self)->Control {Control(self.0.as_ref().map(Rc::downgrade))}
}
```

Replace it with:

```rust
        let window=web_sys::window().ok_or("No browser window")?;let url=web_sys::Url::new(&window.location().href()?)?;let q=url.search_params();
        let scene=q.get("scene");let data=q.get("data");let live=q.get("live");let poll=q.get("poll");let notify=q.get("notify");
        let Some(route)=crate::publication_route::select(scene.as_deref(),&window.location().pathname()?,data.as_deref(),live.as_deref(),poll.as_deref(),notify.as_deref()) else {return Ok(Self(None));};
        let index=Index::new(route.base.clone());Ok(Self(Some(Rc::new(RefCell::new(State {active:true,inflight:false,again:false,pending_streams:Vec::new(),streams:Vec::new(),cancelled:Rc::new(Cell::new(false)),gate:Default::default(),abort:None,route,index:Some(index),document,fault,delivery:None,last_error:None,timer:None,tick:None,notify:None,message:None})))))
    }
    pub fn control(&self)->Control {Control(self.0.as_ref().map(Rc::downgrade))}
}
```

**Locate this block:**

```rust
use crate::{publication_index::Index,publication_route::Route};
use std::{cell::RefCell,collections::HashSet,rc::{Rc,Weak}};
use wasm_bindgen::{JsCast,JsValue,closure::Closure};

struct State {
    active:bool,inflight:bool,again:bool,
    gate:crate::read_gate::ReadGate,
    abort:Option<web_sys::AbortController>,
    route:Route,index:Option<Index>,
```

Replace it with:

```rust
use crate::{publication_index::Index,publication_route::Route};
use std::{cell::{Cell,RefCell},collections::HashSet,rc::{Rc,Weak}};
use wasm_bindgen::{JsCast,JsValue,closure::Closure};

struct State {
    active:bool,inflight:bool,again:bool,
    pending_streams:Vec<crate::stream_load::Cursor>,streams:Vec<std::rc::Weak<crate::range_cloud::Prepared>>,cancelled:Rc<Cell<bool>>,
    gate:crate::read_gate::ReadGate,
    abort:Option<web_sys::AbortController>,
    route:Route,index:Option<Index>,
```

## <a id="checkpoint-68ab-choose-bounded-range-loading-for-large-originals"></a>30. Choose bounded range loading for large originals

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 30 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/browser_publication.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">manifest source</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original descriptor</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">staged replacement</text>
</svg>
</div>

**What this code does:** Choose bounded range loading for large originals.

### 1. `src/browser_publication.rs`

Stage original ranged prefixes and keep failed revisions off the current scene.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    });
}

async fn read(route:&Route,index:&mut Index,signal:&web_sys::AbortSignal)->Result<(),String> {
    let started=crate::browser_phase::now();let mut response=crate::publication_fetch::get(&route.manifest,index.etag(&route.manifest),4*1024*1024,signal).await?;
    if response.status==404&&route.manifest.ends_with(".toml") {let yaml=format!("{}.yaml",route.manifest.trim_end_matches(".toml"));response=crate::publication_fetch::get(&yaml,index.etag(&route.manifest),4*1024*1024,signal).await?;}
    match response.status {304 if index.has_manifest()=>{},200..=299=>{index.manifest(&route.manifest,response.etag,&response.bytes)?;crate::browser_phase::finish("manifest",started,response.bytes.len() as u64,&route.manifest);},status=>return Err(format!("Manifest HTTP {status}"))}
    let mut seen=HashSet::new();
    for url in index.urls() {
        if !seen.insert(url.clone())||crate::manifest::immutable_key(&url)&&index.cached(&url) {continue;}
        let response=crate::publication_fetch::get(&url,index.etag(&url),crate::archive::LIMIT,signal).await?;
        match response.status {
            304 if index.cached(&url)=>{},
            200..=299=>{let started=crate::browser_phase::now();index.source(&url,response.etag,&response.bytes)?;crate::browser_phase::finish("decode/validation/display",started,response.bytes.len() as u64,&url);},
            status=>{index.forget(&url);return Err(format!("Source HTTP {status}: {url}"));}
        }
    }
    Ok(())
}
```

Replace it with:

```rust
    });
}

async fn read(route:&Route,index:&mut Index,signal:&web_sys::AbortSignal)->Result<Vec<crate::stream_load::Cursor>,String> {
    let mut cursors=Vec::new();let mut points=index.streamed_points();let budget=crate::page_cache::Budget::new(crate::stream_budget::PAGE_BYTES);
    let started=crate::browser_phase::now();let mut response=crate::publication_fetch::get(&route.manifest,index.etag(&route.manifest),4*1024*1024,signal).await?;
    if response.status==404&&route.manifest.ends_with(".toml") {let yaml=format!("{}.yaml",route.manifest.trim_end_matches(".toml"));response=crate::publication_fetch::get(&yaml,index.etag(&route.manifest),4*1024*1024,signal).await?;}
    match response.status {304 if index.has_manifest()=>{},200..=299=>{index.manifest(&route.manifest,response.etag,&response.bytes)?;crate::browser_phase::finish("manifest",started,response.bytes.len() as u64,&route.manifest);},status=>return Err(format!("Manifest HTTP {status}"))}
    let mut seen=HashSet::new();
    for url in index.urls() {
        if !seen.insert(url.clone())||crate::manifest::immutable_key(&url)&&index.cached(&url) {continue;}
        let encoding=index.encoding(&url)?;
        if encoding.name.is_empty()&&url.split('?').next().is_some_and(|path|path.ends_with(".pb")) {
            if let Some(probe)=crate::stream::probe(&url).await {
                if crate::range_shape::single_cloud(&probe.bytes)&&crate::stream::cloud_identity(&probe.bytes).is_some() {
                    let source=crate::range_source::Source::from_probe(url.clone(),&probe).await?;
                    if source.fields.count>crate::stream_budget::PREFIX_POINTS||source.fields.coords_len>=crate::stream_budget::MIN_BYTES {
                        if index.cached(&url)&&source.fields.revision.is_some()&&index.etag(&url)==source.fields.revision {continue;}
                        if signal.aborted() {return Err("Source read cancelled".into());}
                        points=points.saturating_sub(index.source_points(&url));
                        let remaining=crate::stream_budget::MAX_POINTS.saturating_sub(points);
                        if remaining==0 {return Err("Manifest sources exceed the point ceiling".into());}
                        let cursor=crate::stream_load::start(Rc::new(source),budget.clone(),remaining).await?;
                        if signal.aborted() {cursor.prepared.generation.borrow_mut().close();return Err("Source read cancelled".into());}
                        points=points.saturating_add(cursor.loaded);index.stream(&url,cursor.prepared.clone())?;cursors.push(cursor);continue;
                    }
                }
            }
        }
        let response=crate::publication_fetch::get(&url,index.etag(&url),crate::archive::LIMIT,signal).await?;
        match response.status {
            304 if index.cached(&url)=>{},
            200..=299=>{let started=crate::browser_phase::now();let bytes=crate::source_unpack::decode(response.bytes,&encoding,signal).await?;index.source(&url,response.etag,&bytes)?;crate::browser_phase::finish("decode/validation/display",started,bytes.len() as u64,&url);},
            status=>{index.forget(&url);return Err(format!("Source HTTP {status}: {url}"));}
        }
    }
    let extra=crate::stream_budget::MAX_POINTS.saturating_sub(points)/(cursors.len().max(1) as u32);for cursor in &mut cursors {cursor.ceiling=cursor.loaded+extra;}
    Ok(cursors)
}
```

**Locate this block:**

```rust
        let result=read(&route,&mut index,&abort.signal()).await;
        let mut owner=state.borrow_mut();let current=owner.active&&owner.gate.finish(ticket)&&crate::browser_runtime::owns(&owner.fault)&&owner.fault.message().is_none();
        owner.inflight=false;owner.abort=None;if !current {return;}
        let outcome=result.and_then(|()|index.replacement(&owner.document.borrow().scene));owner.index=Some(index);
        let changed=match outcome {Ok(Some(scene))=>{owner.last_error=None;owner.delivery=Some(Ok(Box::new(scene)));true},Ok(None)=>false,Err(error)=>{if owner.last_error.as_ref()==Some(&error) {false} else {let _=crate::browser_report::observe("publication",&error);owner.last_error=Some(error.clone());owner.delivery=Some(Err(error));true}}};
        let again=std::mem::take(&mut owner.again);drop(owner);
        if changed {if let Some(window)=web_sys::window() {if let Ok(event)=web_sys::Event::new("viewer-publication") {let _=window.dispatch_event(&event);}}}
```

Replace it with:

```rust
        let result=read(&route,&mut index,&abort.signal()).await;
        let mut owner=state.borrow_mut();let current=owner.active&&owner.gate.finish(ticket)&&crate::browser_runtime::owns(&owner.fault)&&owner.fault.message().is_none();
        owner.inflight=false;owner.abort=None;if !current {return;}
        let outcome=result.and_then(|cursors| {for cursor in &cursors {owner.streams.push(Rc::downgrade(&cursor.prepared));}owner.pending_streams.extend(cursors);index.replacement(&owner.document.borrow().scene)});owner.index=Some(index);
        let changed=match outcome {Ok(Some(scene))=>{owner.last_error=None;owner.delivery=Some(Ok(Box::new(scene)));true},Ok(None)=>false,Err(error)=>{if owner.last_error.as_ref()==Some(&error) {false} else {let _=crate::browser_report::observe("publication",&error);owner.last_error=Some(error.clone());owner.delivery=Some(Err(error));true}}};
        let again=std::mem::take(&mut owner.again);drop(owner);
        if changed {if let Some(window)=web_sys::window() {if let Ok(event)=web_sys::Event::new("viewer-publication") {let _=window.dispatch_event(&event);}}}
```

## <a id="checkpoint-68ac-connect-current-pages-to-the-browser-renderer"></a>31. Connect current pages to the browser renderer

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 31 of 31</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/browser_publication.rs,…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">adopted original</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">cancelled old pick</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source diagnostics</text>
</svg>
</div>

**What this code does:** Connect current pages to the browser renderer.

### 1. `src/browser_publication.rs`

Connect current streamed page delivery, original source diagnostics and bounded runtime ownership.

**Locate this existing block:**

```rust
        if let Some(source)=state.notify.take() {source.set_onmessage(None);source.close();}state.message=None;state.tick=None;
    }
    pub fn take(&self)->Result<Option<crate::editor::Action>,String> {let Some(state)=self.state() else {return Ok(None);};let mut state=state.borrow_mut();if !state.active {state.delivery=None;return Ok(None);}state.delivery.take().transpose().map(|scene|scene.map(crate::editor::Action::Live))}
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/68ac-connect-current-pages-to-the-browser-renderer-01.rs"
```

### 2. `src/browser.rs`

Connect current streamed page delivery, original source diagnostics and bounded runtime ownership.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    canvas.set_attribute("data-preview-bytes", &renderer.preview_bytes().to_string())?;
    canvas.set_attribute("data-highlight-bytes", &renderer.highlight_bytes().to_string())?;
    canvas.set_attribute("data-selection-part", &editor.selection().map_or_else(String::new, |target| target.describe()))?;
    canvas.set_attribute("data-pick-mode", &format!("{:?}", editor.pick_mode))?;
    canvas.set_attribute("data-part-usage", &serde_json::to_string(&renderer.part_usage()).unwrap())?;
    canvas.set_attribute("data-selected-ids", &serde_json::to_string(&editor.selected_objects().iter().map(|id| id.value()).collect::<Vec<_>>()).unwrap())?;
```

Replace it with:

```rust
    canvas.set_attribute("data-preview-bytes", &renderer.preview_bytes().to_string())?;
    canvas.set_attribute("data-highlight-bytes", &renderer.highlight_bytes().to_string())?;
    canvas.set_attribute("data-selection-part", &editor.selection().map_or_else(String::new, |target| target.describe()))?;
    let stream_usage=crate::memory::streams(editor.scenes());
    canvas.set_attribute("data-range-sources",&serde_json::to_string(&stream_usage).unwrap())?;
    let streamed=editor.scene.streams().iter().map(|row|serde_json::json!({"id":row.id.value(),"guid":row.guid,"source_guid":row.prepared.source.guid,"name":row.prepared.source.name,"url":row.prepared.source.url,"revision":row.prepared.source.fields.revision,"total":row.prepared.source.fields.count,"model":row.model.m,"point_px":row.point_px,"controls":row.controls_visible})).collect::<Vec<_>>();
    canvas.set_attribute("data-range-documents",&serde_json::to_string(&streamed).unwrap())?;

    canvas.set_attribute("data-pick-mode", &format!("{:?}", editor.pick_mode))?;
    canvas.set_attribute("data-part-usage", &serde_json::to_string(&renderer.part_usage()).unwrap())?;
    canvas.set_attribute("data-selected-ids", &serde_json::to_string(&editor.selected_objects().iter().map(|id| id.value()).collect::<Vec<_>>()).unwrap())?;
```

**Locate this block:**

```rust
    listeners.listen(document.as_ref(), "cancel")?;
    listeners.listen(window.as_ref(), "viewer-pick")?;
    listeners.listen(window.as_ref(), "viewer-publication")?;
    listeners.listen(window.as_ref(), "viewer-file")?;
    listeners.listen(window.as_ref(), "viewer-file-error")?;
    listeners.listen(window.as_ref(), "viewer-reload")?;
```

Replace it with:

```rust
    listeners.listen(document.as_ref(), "cancel")?;
    listeners.listen(window.as_ref(), "viewer-pick")?;
    listeners.listen(window.as_ref(), "viewer-publication")?;
    listeners.listen(window.as_ref(), "viewer-stream-page")?;
    listeners.listen(window.as_ref(), "viewer-file")?;
    listeners.listen(window.as_ref(), "viewer-file-error")?;
    listeners.listen(window.as_ref(), "viewer-reload")?;
```

**Locate this block:**

```rust
                }
            }
            None
        } else if event.type_() == "viewer-publication" {
            match publication_control.take() {Ok(action)=>action,Err(error)=>{let message=format!("Cannot load scene: {error}");report(&message);panel.result(&message);None}}
        } else if event.type_() == "viewer-file" {
```

Replace it with:

```rust
                }
            }
            None
        } else if event.type_() == "viewer-stream-page" {
            renderer.cancel_pick();Some(Action::StreamRefresh)
        } else if event.type_() == "viewer-publication" {
            match publication_control.take() {Ok(action)=>action,Err(error)=>{let message=format!("Cannot load scene: {error}");report(&message);panel.result(&message);None}}
        } else if event.type_() == "viewer-file" {
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 12.4
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 12.4
npm --prefix ../session_tests run course -- save 12.4
```

Continue with [13.1 · Shared original definitions and independently placed instances](../../consolidation/69-validate-original-definition-references.md).
