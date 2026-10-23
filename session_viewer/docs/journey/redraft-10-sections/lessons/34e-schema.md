# 4.3 · Admit useful evidence and bound stored runs

**Goal:** Separate trusted-report admission from raw storage retention so an older viewer cannot strip or prematurely delete newer telemetry. · **Steps:** 8 · **Source:** `src/lib.rs,src/selftest/lifecycle.rs,src/engine/gpu/device.rs`

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 4.3 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/diagnostic.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/diagnostic_shape_test…</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">8 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/selftest/lifecycle.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Check the bounded shape o…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Tab Visibility &amp; Drop</text>
  <rect x="598" y="66" width="84" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="640" y="96" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Active</text>
  <path d="M 682 82 L 718 82" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <path d="M 722 102 L 686 102" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="722" y="66" width="84" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="764" y="96" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Paused</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Admit useful evidence and b…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Fault Ring &amp; 1x Recovery</text>
  <circle cx="934" cy="96" r="36" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <circle cx="934" cy="96" r="22" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Ring</text>
  <rect x="986" y="72" width="86" height="48" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1029" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1x Safe</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1029" y="108" text-anchor="middle" font-family="sans-serif" font-size="9" fill="#111111">Rebuild</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Preserve unsupported tele…</text>
</svg>
</div>

## <a id="checkpoint-34e-schema"></a>1. Check the bounded shape of a report

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 8</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/diagnostic.rs, src/diag…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">typed report</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">consistent failure/outcome</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">valid shape</text>
</svg>
</div>

**What this code does:** Validate a typed report’s limits and failure/outcome invariants before the later storage decoder adopts it.

### 1. `src/diagnostic.rs`

Validate shape, limits and failure/outcome consistency before trusting a decoded stored value. Keep recording bounded even if a caller bypasses admission and directly deserializes an oversized queue.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        } else if kind == "milestone" && message == "geometry on screen" && self.failure.is_none() {
            self.outcome = Outcome::Ready;
        }
        if self.events.len() == 24 { self.events.pop_front(); }
        self.events.push_back(event);
        Ok(())
    }
```

Replace it with:

```rust
        } else if kind == "milestone" && message == "geometry on screen" && self.failure.is_none() {
            self.outcome = Outcome::Ready;
        }
        while self.events.len() >= 24 { self.events.pop_front(); }
        self.events.push_back(event);
        Ok(())
    }
```

**Locate this block:**

```rust
            context, events: Default::default(), failure: None }
    }

    pub fn failure(&self) -> Option<&Event> { self.failure.as_ref() }

    pub fn record(&mut self, time: String, elapsed_ms: f64, kind: &str, message: &str) -> Result<(), &'static str> {
```

Replace it with:

```rust
            context, events: Default::default(), failure: None }
    }

    pub fn valid(&self) -> bool {
        let text = |s: &str, max| s.chars().count() <= max;
        let event = |e: &Event| !e.time.is_empty() && text(&e.time, 64) && !e.kind.is_empty()
            && text(&e.kind, 64) && text(&e.message, 4096) && e.elapsed_ms.is_finite() && e.elapsed_ms >= 0.0;
        self.version == 1 && !self.tab.is_empty() && text(&self.tab, 128)
            && !self.started.is_empty() && text(&self.started, 64) && !self.last_seen.is_empty() && text(&self.last_seen, 64)
            && !self.context.page.is_empty() && text(&self.context.page, 4096) && !self.context.page.contains(['?', '#'])
            && text(&self.context.browser, 4096) && self.context.device_pixel_ratio.is_finite() && self.context.device_pixel_ratio > 0.0
            && self.events.len() <= 24 && self.events.iter().all(event)
            && self.events.iter().all(|e| e.kind != "fatal" || self.failure.is_some())
            && match &self.failure {
                Some(first) => event(first) && first.kind == "fatal" && self.outcome == Outcome::Failed,
                None => self.outcome != Outcome::Failed,
            }
    }

    pub fn failure(&self) -> Option<&Event> { self.failure.as_ref() }

    pub fn record(&mut self, time: String, elapsed_ms: f64, kind: &str, message: &str) -> Result<(), &'static str> {
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/diagnostic_shape_tests.rs`

Optional authoring fixture. Verify typed metadata, bounded queues and consistent first-failure state.

Copy this supplied file:

```text
--8<-- "journey/code/34e-schema-03.rs"
```

### 3. `src/lib.rs`

Include the native shape-invariant checks.

**Locate this existing block:**

```rust
pub mod diagnostic;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod diagnostic_shape_tests;
pub mod diagnostic;
```

## <a id="checkpoint-34ea-decode"></a>2. Admit only supported bounded saved JSON

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2 of 8</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/report_store.rs, src/re…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">raw text byte limit</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">typed shape validator</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">candidate for recency</text>
</svg>
</div>

**What this code does:** Reject oversized, malformed or unsupported saved JSON before it can become a diagnostic candidate.

### 1. `src/report_store.rs`

Bound raw input and reject unsupported JSON fields before adopting a valid typed report.

Create the file and type:

```rust
--8<-- "journey/code/34ea-decode-01.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/report_store_tests.rs`

Optional authoring fixture. Prove actual stored JSON admission and byte/text/schema boundaries.

Copy this supplied file:

```text
--8<-- "journey/code/34ea-decode-02.rs"
```

### 3. `src/lib.rs`

Expose the decoder and its native acceptance checks.

**Locate this existing block:**

```rust
pub mod diagnostic;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod report_store;
#[cfg(test)]
mod report_store_tests;
pub mod diagnostic;
```

## <a id="checkpoint-34eb-recency"></a>3. Choose a recent failure without blaming active tabs

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3 of 8</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/report_recency.rs, src/…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">validated report</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">ordered finite times</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">ranking timestamp</text>
</svg>
</div>

**What this code does:** Select recent failed or interrupted runs using actual failure time, valid chronology and tab identity.

### 1. `src/report_recency.rs`

Interpret a validated report’s chronology and return an eligible timestamp for ranking, using an injected clock parser.

Create the file and type:

```rust
--8<-- "journey/code/34eb-recency-01.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/report_recency_tests.rs`

Optional authoring fixture. Cover the original stale-heartbeat regression, active tabs, temporal cutoffs and failure-time ranking.

Copy this supplied file:

```text
--8<-- "journey/code/34eb-recency-02.rs"
```

### 3. `src/lib.rs`

Expose native selection policy before binding it to browser storage.

**Locate this existing block:**

```rust
pub mod report_store;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod report_recency;
#[cfg(test)]
mod report_recency_tests;
pub mod report_store;
```

## <a id="checkpoint-34ec-proof"></a>4. Prove saved-run exclusions before adopting storage

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4 of 8</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/report_recency_tests.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">candidate</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">invalid clock/healthy sta…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">excluded without mutation</text>
</svg>
</div>

**What this code does:** Complete timestamp-policy acceptance before a stored candidate can create a notice or previous-report download.

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/report_recency_tests.rs`

Optional authoring fixture. Prove invalid clocks, healthy/closed runs, invalid failure ordering and unchanged metadata cannot become interruption evidence.

**Locate this existing block:**

```text
use crate::{diagnostic::Outcome, diagnostic_shape_tests::report, report_recency::eligible};

#[test]
fn previous_run_selection_respects_actual_failure_time() {
    let now = 1_000_000_000.0;
    for (name, failed, same, seen_age, failure_age, expected) in [
        ("old failure, fresh heartbeat", true, false, 5000.0, 259_200_000.0, false),
        ("recent failure", true, false, 5000.0, 10000.0, true),
        ("failure at cutoff", true, false, 5000.0, 7_200_000.0, false),
        ("failure after heartbeat", true, false, 5000.0, 1000.0, false),
        ("future heartbeat", true, false, -1.0, 1000.0, false),
        ("future failure", true, false, 0.0, -1.0, false),
        ("active other tab", false, false, 120_000.0, 0.0, false),
        ("interrupted other tab", false, false, 120_001.0, 0.0, true),
        ("stale other tab", false, false, 7_200_000.0, 0.0, false),
        ("interrupted same tab", false, true, 0.0, 0.0, true),
    ] {
        let mut value = report(); value.started = "start".into();
        if failed { value.record("failed".into(), 1.0, "fatal", "original").unwrap(); }
        value.last_seen = "seen".into(); value.tab = if same { "current" } else { "other" }.into();
        let parser = |time: &str| match time { "start" => now - 259_200_000.0, "seen" => now - seen_age,
            "failed" => now - failure_age, _ => f64::NAN };
        let selected = eligible(&value, now, "current", parser);
        assert_eq!(selected.is_some(), expected, "{name}");
        if expected { assert_eq!(selected, Some(now - if failed { failure_age } else { seen_age })); }
    }
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/34ec-proof-01.rs" skip=1607
```

## <a id="checkpoint-34f-storage"></a>5. Read previous reports from real browser storage

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5 of 8</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/report_storage.rs, src/…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">sessionStorage tab</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">byte/schema/shape admissi…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">latest eligible evidence</text>
</svg>
</div>

**What this code does:** Read admitted saved reports using stable tab identity, the real browser timestamp parser and bounded scanning.

### 1. `src/report_storage.rs`

Read bounded validated candidates from actual browser storage and rank with the real Date parser; retain only metadata and storage handles.

Create the file and type:

```rust
--8<-- "journey/code/34f-storage-01.rs"
```

### 2. `src/lib.rs`

Build the storage owner only for the browser target and expose its debug acceptance probe.

**Locate this existing block:**

```rust
mod browser_report;
```

After that block, type:

```rust
#[cfg(target_arch = "wasm32")]
pub mod report_storage;
```

### 3. `Cargo.toml`

Enable the pinned browser Storage bindings without changing package versions.

**Locate this existing block:**

```toml
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "Location", "Navigator", "Performance", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
```

Edit the existing block to match this code; keep unchanged lines:

```toml
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "Location", "Navigator", "Storage", "Performance", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
```

## <a id="checkpoint-34fa-retain"></a>6. Retain only three diagnostic runs

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 6 of 8</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/report_storage.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">valid matching-tab report</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">current run write</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">release other owned keys</text>
</svg>
</div>

**What this code does:** Write admitted report metadata to an independent run key and retain a bounded set without disturbing viewer state.

### 1. `src/report_storage.rs`

Admit metadata, save to this run’s key and retain at most two older valid reports; refuse unbounded namespace examination and tolerate unavailable storage. Verify repeated writes use the same key and reach actual Web Storage; this debug export is not a runtime feature control.

Replace the entire file with:

```rust
use crate::diagnostic::Report;
use web_sys::Storage;

pub const PREFIX: &str = "viewer-journey-report:";

pub struct Store {
    pub tab: String,
    key: String,
    storage: Option<Storage>,
}

impl Store {
    pub fn open() -> Self {
        let window = web_sys::window();
        let session = window.as_ref().and_then(|w| w.session_storage().ok().flatten());
        let tab = session.as_ref().and_then(|s| s.get_item(PREFIX).ok().flatten())
            .filter(|tab| !tab.is_empty() && tab.chars().count() <= 128)
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if let Some(session) = session { let _ = session.set_item(PREFIX, &tab); }
        Self { tab, key: format!("{PREFIX}{}", uuid::Uuid::new_v4()),
            storage: window.and_then(|w| w.local_storage().ok().flatten()) }
    }

    pub fn reports(&self) -> Vec<(String, Report)> {
        let Some(storage) = &self.storage else { return Vec::new(); };
        (0..storage.length().unwrap_or(0).min(256))
            .filter_map(|i| storage.key(i).ok().flatten()).filter(|key| key.starts_with(PREFIX)).take(32)
            .filter_map(|key| {
                let text = storage.get_item(&key).ok().flatten()?;
                Some((key, crate::report_store::decode(&text)?))
            }).collect()
    }

    pub fn write(&self, report: &Report) -> bool {
        if !report.valid() || report.tab != self.tab { return false; }
        let Some(storage) = &self.storage else { return false; };
        let Ok(length) = storage.length() else { return false; };
        if length > 256 { return false; }
        let keys: Result<Vec<_>, _> = (0..length).map(|i| storage.key(i)).collect();
        let Ok(keys) = keys else { return false; };
        let keys: Vec<_> = keys.into_iter().flatten().filter(|key| key.starts_with(PREFIX)).collect();
        if keys.len() > 32 { return false; }
        let mut old = self.reports(); old.retain(|(key, _)| key != &self.key);
        old.retain(|(_, report)| js_sys::Date::parse(&report.last_seen).is_finite());
        old.sort_by(|a, b| js_sys::Date::parse(&b.1.last_seen).total_cmp(&js_sys::Date::parse(&a.1.last_seen)));
        let keep: Vec<_> = old.iter().take(2).map(|(key, _)| key).collect();
        let Ok(text) = serde_json::to_string(report) else { return false; };
        if text.len() > crate::report_store::MAX_BYTES || storage.set_item(&self.key, &text).is_err() { return false; }
        let mut success = true;
        for key in keys {
            if key != self.key && !keep.contains(&&key) { success &= storage.remove_item(&key).is_ok(); }
        }
        success
    }

    pub fn previous(&self) -> Option<Report> {
        let now = js_sys::Date::now();
        self.reports().into_iter().filter_map(|(_, report)| {
            let score = crate::report_recency::eligible(&report, now, &self.tab, js_sys::Date::parse)?;
            Some((score, report))
        }).max_by(|a, b| a.0.total_cmp(&b.0)).map(|(_, report)| report)
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn previous_storage_probe() -> String {
    let store = Store::open();
    serde_json::json!({"tab": store.tab, "key": store.key, "previous": store.previous()}).to_string()
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn stored_report_probe(text: &str) -> Result<String, wasm_bindgen::JsValue> {
    let report = crate::report_store::decode(text).ok_or("Unsupported report")?;
    let store = Store::open(); let first = store.write(&report); let second = store.write(&report);
    Ok(serde_json::json!({"key": store.key, "first": first, "second": second}).to_string())
}
```

## <a id="checkpoint-34fb-store"></a>7. Retrieve saved failure evidence through the command line

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 7 of 8</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/browser_report.rs, src/…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">startup selects previous</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">healthy reload</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Report Previous</text>
</svg>
</div>

**What this code does:** Persist current diagnostics independently of GPU lifetime and retrieve eligible previous evidence with a typed command.

### 1. `src/browser_report.rs`

Keep current and previous diagnostic metadata independent of the GPU runtime and source documents. Select earlier evidence before writing this new run, retain stable tab identity and let denied storage leave startup usable. Save successful observations after releasing the current-report borrow; storage failure cannot replace a diagnostic error or interrupt drawing. Persist refreshed current context while retaining manual downloads when storage is unavailable. Serialize independent previous metadata before the download callback; report interruption separately from proof of a crash.

Replace the entire file with:

```rust
use crate::diagnostic::{Context, Report};
use std::cell::RefCell;
use wasm_bindgen::{JsCast, JsValue};

thread_local! {
    static REPORT: RefCell<Option<Report>> = const { RefCell::new(None) };
    static STORE: RefCell<Option<crate::report_storage::Store>> = const { RefCell::new(None) };
    static PREVIOUS: RefCell<Option<Report>> = const { RefCell::new(None) };
}

fn context() -> Result<Context, JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document.get_element_by_id("canvas").ok_or("No canvas")?.dyn_into()?;
    let location = window.location(); let navigator = window.navigator();
    Ok(Context {
        page: format!("{}{}", location.origin()?, location.pathname()?),
        browser: navigator.user_agent()?, secure_context: window.is_secure_context(),
        webgpu: js_sys::Reflect::get(navigator.as_ref(), &"gpu".into())?.is_object(),
        viewport: [window.inner_width()?.as_f64().ok_or("No width")? as u32,
            window.inner_height()?.as_f64().ok_or("No height")? as u32],
        canvas: [canvas.width(), canvas.height()], device_pixel_ratio: window.device_pixel_ratio(),
    })
}

pub fn start() -> Result<(), JsValue> {
    let store = crate::report_storage::Store::open(); let previous = store.previous();
    let report = Report::new(store.tab.clone(), js_sys::Date::new_0().to_iso_string().into(), context()?);
    PREVIOUS.with(|slot| slot.replace(previous)); STORE.with(|slot| slot.replace(Some(store)));
    REPORT.with(|slot| slot.replace(Some(report)));
    let _ = persist(); Ok(())
}

pub fn observe(kind: &str, message: &str) -> Result<(), JsValue> {
    let context = context()?;
    let elapsed = web_sys::window().and_then(|window| window.performance()).ok_or("No clock")?.now();
    REPORT.with(|slot| {
        let mut slot = slot.borrow_mut(); let report = slot.as_mut().ok_or("No report")?;
        report.context = context;
        report.record(js_sys::Date::new_0().to_iso_string().into(), elapsed, kind, message).map_err(JsValue::from_str)
    })?;
    let _ = persist(); Ok(())
}

pub fn download() -> Result<(), JsValue> {
    let json = diagnostic_snapshot()?; let _ = persist();
    crate::file_output::download_named(json.as_bytes(), "viewer-diagnostic.json")
}

fn persist() -> bool {
    let report = REPORT.with(|slot| slot.borrow().clone());
    let Some(report) = report else { return false; };
    STORE.with(|slot| slot.borrow().as_ref().is_some_and(|store| store.write(&report)))
}

pub fn previous_notice() -> Option<&'static str> {
    PREVIOUS.with(|slot| slot.borrow().as_ref().map(|report| if report.outcome == crate::diagnostic::Outcome::Failed {
        "Previous viewer run failed. Type Report Previous."
    } else { "Previous viewer run was interrupted. Type Report Previous." }))
}

pub fn download_previous() -> Result<(), JsValue> {
    let json = previous_diagnostic_snapshot()?;
    crate::file_output::download_named(json.as_bytes(), "viewer-diagnostic-previous.json")
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn previous_diagnostic_snapshot() -> Result<String, JsValue> {
    PREVIOUS.with(|slot| {
        let slot = slot.borrow(); let report = slot.as_ref().ok_or("No previous report")?;
        serde_json::to_string_pretty(report).map_err(|error| JsValue::from_str(&error.to_string()))
    })
}

pub fn fatal(message: &str) {
    let first = REPORT.with(|slot| slot.borrow().as_ref().is_some_and(|report| report.failure().is_none()));
    if observe("fatal", message).is_ok() && first { let _ = download(); }
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn diagnostic_snapshot() -> Result<String, JsValue> {
    let context = context()?;
    REPORT.with(|slot| {
        let mut slot = slot.borrow_mut(); let report = slot.as_mut().ok_or("No report")?;
        report.context = context;
        serde_json::to_string_pretty(report).map_err(|error| JsValue::from_str(&error.to_string()))
    })
}
```

### 2. `src/browser.rs`

Expose previous-report retrieval as a named command in the actual dock. Show an eligible previous-run notice in the existing command dock, keeping HTML free of feature buttons. Dispatch current and previous JSON downloads without changing document history or the stopped-runtime failure guard.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    input.click();
                }
                None
            } else if line == "report" {
                let result = crate::browser_report::download().map(|()| "Diagnostic report downloaded.".to_owned())
                    .unwrap_or_else(|error| format!("Diagnostic report failed: {error:?}"));
                panel.result(&result);
                None
```

Replace it with:

```rust
                    input.click();
                }
                None
            } else if line == "report" || line == "report previous" {
                let output = if line.ends_with(" previous") { crate::browser_report::download_previous() }
                    else { crate::browser_report::download() };
                let result = output.map(|()| "Diagnostic report downloaded.".to_owned())
                    .unwrap_or_else(|error| format!("Diagnostic report failed: {error:?}"));
                panel.result(&result);
                None
```

**Locate this block:**

```rust
            "View Reset",
        ],
    );
    panel.update(None, &canvas)?;
    if resize(
        &window,
```

Replace it with:

```rust
            "View Reset",
        ],
    );
    if let Some(message) = crate::browser_report::previous_notice() { panel.result(message); }
    panel.update(None, &canvas)?;
    if resize(
        &window,
```

**Locate this block:**

```rust
            "Cancel Reload",
            "Save",
            "Report",
            "Example Box",
            "Example Triangle",
            "Select Next",
```

Replace it with:

```rust
            "Cancel Reload",
            "Save",
            "Report",
            "Report Previous",
            "Example Box",
            "Example Triangle",
            "Select Next",
```

### 3. `src/panel.rs`

Expose the existing drawn status in the same diagnostic inspector so Chrome can verify the initial notice and command errors.

**Locate this existing block:**

```rust
        serde_json::json!({"controls": self.controls, "command": self.model.command, "history": self.model.history,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        serde_json::json!({"controls": self.controls, "command": self.model.command, "history": self.model.history, "status": self.model.status,
```

## <a id="checkpoint-34fc-retention"></a>8. Preserve unsupported telemetry while pruning

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 8 of 8</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/report_store.rs, src/re…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">bounded raw JSON header</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">keep original stored byte…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">candidate notice/download</text>
</svg>
</div>

**What this code does:** Separate trusted-report admission from raw storage retention so an older viewer cannot strip or prematurely delete newer telemetry.

### 1. `src/report_store.rs`

Read a bounded date/version header for retention without adopting, converting or rewriting unsupported telemetry.

**Locate this existing block:**

```rust
pub const MAX_BYTES: usize = 1024 * 1024;
```

After that block, type:

```rust

pub fn retention_time(text: &str, now: f64, parse: impl Fn(&str) -> f64) -> Option<f64> {
    if text.len() > MAX_BYTES || !now.is_finite() || now < 0.0 { return None; }
    let value: serde_json::Value = serde_json::from_str(text).ok()?;
    if value.get("version")?.as_u64()? == 0 { return None; }
    let seen = value.get("lastSeen")?.as_str()?;
    if seen.is_empty() || seen.chars().count() > 64 { return None; }
    let time = parse(seen);
    (time.is_finite() && time <= now).then_some(time)
}
```

### 2. `src/report_storage.rs`

Retain raw supported or unsupported bounded JSON by header time; a failed read aborts before writing or pruning.

**Locate this existing block:**

```rust
        let mut old = self.reports(); old.retain(|(key, _)| key != &self.key);
        old.retain(|(_, report)| js_sys::Date::parse(&report.last_seen).is_finite());
        old.sort_by(|a, b| js_sys::Date::parse(&b.1.last_seen).total_cmp(&js_sys::Date::parse(&a.1.last_seen)));
        let keep: Vec<_> = old.iter().take(2).map(|(key, _)| key).collect();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let now = js_sys::Date::now(); let mut old = Vec::new();
        for key in keys.iter().filter(|key| *key != &self.key) {
            let Ok(Some(text)) = storage.get_item(key) else { return false; };
            if let Some(time) = crate::report_store::retention_time(&text, now, js_sys::Date::parse) {
                old.push((time, key.clone()));
            }
        }
        old.sort_by(|a, b| b.0.total_cmp(&a.0));
        let keep: Vec<_> = old.iter().take(2).map(|(_, key)| key).collect();
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/report_store_tests.rs`

Optional authoring fixture. Prove unsupported fields/versions remain dateable without admission, with unchanged input and bounded header parsing.

**Locate this existing block:**

```text
use crate::{diagnostic::Report, diagnostic_shape_tests::report, report_store::{decode, MAX_BYTES}};

#[test]
fn stored_json_rejects_invalid_shape() {
    let good = serde_json::to_value(report()).unwrap();
    for (key, bad) in [("version", serde_json::json!(2)), ("tab", serde_json::json!("")),
        ("page", serde_json::json!("https://example.test/?private=1")), ("devicePixelRatio", serde_json::json!(0)),
        ("events", serde_json::json!([{"time":"now","kind":"fatal","message":"bad","elapsedMs":null}])),
        ("outcome", serde_json::json!("failed")), ("unknown", serde_json::json!(true))] {
        let mut value = good.clone(); value[key] = bad;
        assert!(decode(&value.to_string()).is_none(), "{key}");
    }
    assert!(decode("{").is_none());
}

#[test]
fn stored_json_bounds_bytes_events_and_text() {
    let json = serde_json::to_string(&report()).unwrap();
    let padding = MAX_BYTES - json.len(); let padded = json + &" ".repeat(padding);
    assert!(decode(&padded).is_some()); assert!(decode(&(padded + " ")).is_none());
    let mut value = serde_json::to_value(report()).unwrap();
    let event = serde_json::json!({"time":"now","kind":"phase","message":"test","elapsedMs":1.0});
    value["events"] = serde_json::json!(vec![event; 25]); assert!(decode(&value.to_string()).is_none());
    let mut bypass: Report = serde_json::from_value(value).unwrap();
    bypass.record("later".into(), 2.0, "phase", "bounded").unwrap();
    assert_eq!(serde_json::to_value(bypass).unwrap()["events"].as_array().unwrap().len(), 24);
    let mut failure = report(); failure.record("later".into(), 1.0, "fatal", "first reason").unwrap();
    let json = serde_json::to_string(&failure).unwrap(); assert_eq!(decode(&json), Some(failure));
    let mut value: serde_json::Value = serde_json::from_str(&json).unwrap();
    value["failure"]["message"] = serde_json::json!("😀".repeat(4097)); assert!(decode(&value.to_string()).is_none());
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/34fc-retention-03.rs" skip=1952
```

## Build & Run

![Recorded visual output for Lesson 4.3 (34fc-retention)](../../../screenshots/journey/34fc-retention-browser.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 4.3
npm --prefix ../session_tests run course -- save 4.3
```

Continue with [4.4 · Heartbeat suspension and revocable startup](../../consolidation/34g-heartbeat.md).
