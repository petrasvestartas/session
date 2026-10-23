# 4.4 · Heartbeat suspension and revocable startup

**Goal:** Stop delayed adapter or device delivery from installing a renderer after final exit. · **Steps:** 8 · **Source:** `src/lib.rs,src/selftest/lifecycle.rs,src/engine/gpu/device.rs`

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 4.4 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/diagnostic.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/heartbeat_tests.rs</text>
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Refresh heartbeat without…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Tab Visibility &amp; Drop</text>
  <rect x="598" y="66" width="84" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="640" y="96" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Active</text>
  <path d="M 682 82 L 718 82" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <path d="M 722 102 L 686 102" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="722" y="66" width="84" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="764" y="96" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Paused</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Heartbeat suspension and re…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Fault Ring &amp; 1x Recovery</text>
  <circle cx="934" cy="96" r="36" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <circle cx="934" cy="96" r="22" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Ring</text>
  <rect x="986" y="72" width="86" height="48" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1029" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1x Safe</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1029" y="108" text-anchor="middle" font-family="sans-serif" font-size="9" fill="#111111">Rebuild</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Refuse GPU results after…</text>
</svg>
</div>

## <a id="checkpoint-34g-heartbeat"></a>1. Refresh heartbeat without rewriting failure evidence

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/diagnostic.rs, src/hear…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">real UTC/context</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">release report borrow</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">bounded metadata persiste…</text>
</svg>
</div>

**What this code does:** Refresh actual last-seen metadata independently of first failure, event history and GPU ownership.

### 1. `src/diagnostic.rs`

Advance heartbeat independently of observations, outcome and the first fatal timestamp; invalid timestamp text leaves the report unchanged.

**Locate this existing block:**

```rust
    pub fn failure(&self) -> Option<&Event> { self.failure.as_ref() }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn heartbeat(&mut self, time: String) -> Result<(), &'static str> {
        if time.is_empty() || time.chars().count() > 64 { return Err("Invalid heartbeat timestamp"); }
        self.last_seen = time; Ok(())
    }

    pub fn failure(&self) -> Option<&Event> { self.failure.as_ref() }
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/heartbeat_tests.rs`

Optional authoring fixture. Verify all run outcomes, first-failure/event retention and atomic timestamp-text bounds without a browser clock dependency.

Copy this supplied file:

```text
--8<-- "journey/code/34g-heartbeat-02.rs"
```

### 3. `src/lib.rs`

Include native heartbeat state proofs.

**Locate this existing block:**

```rust
pub mod diagnostic;
```

After that block, type:

```rust
#[cfg(test)]
mod heartbeat_tests;
```

### 4. `src/browser_report.rs`

Refresh actual browser context and UTC heartbeat, release its borrow, then persist metadata without invoking renderer or recording synthetic observations.

**Locate this existing block:**

```rust
pub fn download() -> Result<(), JsValue> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn heartbeat() -> Result<(), JsValue> {
    let context = context()?; let time = js_sys::Date::new_0().to_iso_string().into();
    REPORT.with(|slot| {
        let mut slot = slot.borrow_mut(); let report = slot.as_mut().ok_or("No report")?;
        report.heartbeat(time).map_err(JsValue::from_str)?; report.context = context; Ok::<_, JsValue>(())
    })?;
    let _ = persist(); Ok(())
}

pub fn download() -> Result<(), JsValue> {
```

## <a id="checkpoint-34ga-timer"></a>2. Own the periodic diagnostic heartbeat

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/timer.rs, src/lib.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">metadata startup</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">heartbeat persists metada…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">clear interval before cal…</text>
</svg>
</div>

**What this code does:** Schedule a real 15-second metadata heartbeat independently of GPU lifetime and cancel its callback through an owner.

### 1. `src/timer.rs`

Own the actual interval and callback together; clear its native timer before Rust releases the closure.

Create the file and type:

```rust
--8<-- "journey/code/34ga-timer-01.rs"
```

### 2. `src/lib.rs`

Keep browser interval ownership out of the native editor model.

**Locate this existing block:**

```rust
pub mod report_storage;
```

After that block, type:

```rust
#[cfg(target_arch = "wasm32")]
mod timer;
```

### 3. `src/browser_report.rs`

Retain metadata scheduling independently of the GPU runtime lifetime. Install periodic metadata refresh at run startup; report scheduler unavailability without rejecting drawing or current downloads. Replace one owned interval at a time, cancel outside the slot borrow, and keep the callback metadata-only.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    crate::file_output::download_named(json.as_bytes(), "viewer-diagnostic.json")
}

fn persist() -> bool {
    let report = REPORT.with(|slot| slot.borrow().clone());
    let Some(report) = report else { return false; };
```

Replace it with:

```rust
    crate::file_output::download_named(json.as_bytes(), "viewer-diagnostic.json")
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn start_periodic() -> Result<(), JsValue> {
    stop_periodic();
    let timer = crate::timer::Timer::every(15000, || { let _ = heartbeat(); })?;
    BEAT.with(|slot| slot.replace(Some(timer))); Ok(())
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn stop_periodic() -> bool {
    let timer = BEAT.with(|slot| slot.borrow_mut().take());
    let active = timer.is_some(); drop(timer); active
}

fn persist() -> bool {
    let report = REPORT.with(|slot| slot.borrow().clone());
    let Some(report) = report else { return false; };
```

**Locate this block:**

```rust
    let report = Report::new(store.tab.clone(), js_sys::Date::new_0().to_iso_string().into(), context()?);
    PREVIOUS.with(|slot| slot.replace(previous)); STORE.with(|slot| slot.replace(Some(store)));
    REPORT.with(|slot| slot.replace(Some(report)));
    let _ = persist(); Ok(())
}

pub fn observe(kind: &str, message: &str) -> Result<(), JsValue> {
```

Replace it with:

```rust
    let report = Report::new(store.tab.clone(), js_sys::Date::new_0().to_iso_string().into(), context()?);
    PREVIOUS.with(|slot| slot.replace(previous)); STORE.with(|slot| slot.replace(Some(store)));
    REPORT.with(|slot| slot.replace(Some(report)));
    let _ = persist();
    if let Err(error) = start_periodic() { let _ = observe("diagnostic", &format!("Heartbeat unavailable: {error:?}")); }
    Ok(())
}

pub fn observe(kind: &str, message: &str) -> Result<(), JsValue> {
```

**Locate this block:**

```rust
    static REPORT: RefCell<Option<Report>> = const { RefCell::new(None) };
    static STORE: RefCell<Option<crate::report_storage::Store>> = const { RefCell::new(None) };
    static PREVIOUS: RefCell<Option<Report>> = const { RefCell::new(None) };
}

fn context() -> Result<Context, JsValue> {
```

Replace it with:

```rust
    static REPORT: RefCell<Option<Report>> = const { RefCell::new(None) };
    static STORE: RefCell<Option<crate::report_storage::Store>> = const { RefCell::new(None) };
    static PREVIOUS: RefCell<Option<Report>> = const { RefCell::new(None) };
    static BEAT: RefCell<Option<crate::timer::Timer>> = const { RefCell::new(None) };
}

fn context() -> Result<Context, JsValue> {
```

## <a id="checkpoint-34gb-close"></a>3. Mark a final healthy run closed

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/diagnostic.rs, src/clos…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">real UTC/context</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">release borrow</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">metadata-only persistence</text>
</svg>
</div>

**What this code does:** Persist a healthy final exit as closed while preserving first-failure evidence and every observation.

### 1. `src/diagnostic.rs`

Mark a healthy final exit closed without erasing a failure or appending synthetic observations.

**Locate this existing block:**

```rust
    pub fn failure(&self) -> Option<&Event> { self.failure.as_ref() }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn close(&mut self, time: String) -> Result<(), &'static str> {
        self.heartbeat(time)?;
        if self.failure.is_none() { self.outcome = Outcome::Closed; }
        Ok(())
    }

    pub fn failure(&self) -> Option<&Event> { self.failure.as_ref() }
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/close_report_tests.rs`

Optional authoring fixture. Prove healthy/failed final outcomes, repeatability and atomic refusal separately from browser suspension.

Copy this supplied file:

```text
--8<-- "journey/code/34gb-close-02.rs"
```

### 3. `src/lib.rs`

Include native final-close policy proofs.

**Locate this existing block:**

```rust
pub mod diagnostic;
```

After that block, type:

```rust
#[cfg(test)]
mod close_report_tests;
```

### 4. `src/browser_report.rs`

Refresh real context/time, release the report borrow and persist the final outcome without accessing GPU state.

**Locate this existing block:**

```rust
pub fn download() -> Result<(), JsValue> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn close_report() -> Result<(), JsValue> {
    let context = context()?; let time = js_sys::Date::new_0().to_iso_string().into();
    REPORT.with(|slot| {
        let mut slot = slot.borrow_mut(); let report = slot.as_mut().ok_or("No report")?;
        report.close(time).map_err(JsValue::from_str)?; report.context = context; Ok::<_, JsValue>(())
    })?;
    let _ = persist(); Ok(())
}

pub fn download() -> Result<(), JsValue> {
```

### 5. `src/diagnostic.rs`

A delayed ready milestone must not reopen a final closed run; cache suspension retains Ready without closing.

**Locate this existing block:**

```rust
        } else if kind == "milestone" && message == "geometry on screen" && self.failure.is_none() {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        } else if kind == "milestone" && message == "geometry on screen" && self.failure.is_none() && self.outcome != Outcome::Closed {
```

### 6. `src/browser_report.rs`

Let Chrome submit a real late milestone through the same metadata path; release builds gain no user control.

**Locate this existing block:**

```rust
pub fn observe(kind: &str, message: &str) -> Result<(), JsValue> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn observe(kind: &str, message: &str) -> Result<(), JsValue> {
```

## <a id="checkpoint-34gba-lifecycle"></a>4. Own diagnostic page transitions

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/report_lifecycle.rs, sr…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">owned metadata listeners</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">deferred matching-owner d…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">cached pageshow resumes o…</text>
</svg>
</div>

**What this code does:** Connect real pagehide/pageshow to independent diagnostics, cached suspension and safe final callback cleanup.

### 1. `src/report_lifecycle.rs`

Own two metadata callbacks independently of GPU; retain cached outcomes, defer final callback destruction, and guard cleanup by allocation identity.

Create the file and type:

```rust
--8<-- "journey/code/34gba-lifecycle-01.rs"
```

### 2. `src/lib.rs`

Keep page transition ownership in the browser layer.

**Locate this existing block:**

```rust
mod timer;
```

After that block, type:

```rust
#[cfg(target_arch = "wasm32")]
mod report_lifecycle;
```

### 3. `src/browser_report.rs`

Install metadata lifecycle ownership before scheduling heartbeat; denied listener registration leaves drawing and current downloads available.

**Locate this existing block:**

```rust
    if let Err(error) = start_periodic() { let _ = observe("diagnostic", &format!("Heartbeat unavailable: {error:?}")); }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    if let Err(error) = crate::report_lifecycle::install() { let _ = observe("diagnostic", &format!("Lifecycle unavailable: {error:?}")); }
    if let Err(error) = start_periodic() { let _ = observe("diagnostic", &format!("Heartbeat unavailable: {error:?}")); }
```

## <a id="checkpoint-34gbb-suspension"></a>5. Keep each heartbeat pause reason separate

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/suspension.rs, src/susp…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">hidden + frozen + cached…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one scheduling decision</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one scheduling decision</text>
</svg>
</div>

**What this code does:** Check that clearing one pause reason cannot resume a still-suspended heartbeat.

### 1. `src/suspension.rs`

Keep hidden, frozen and cached reasons independent; final closure permanently refuses scheduling.

Create the file and type:

```rust
--8<-- "journey/code/34gbb-suspension-01.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/suspension_tests.rs`

Optional authoring fixture. Check overlapping reasons, initial hidden state, duplicate delivery and final closure that cannot resume.

Copy this supplied file:

```text
--8<-- "journey/code/34gbb-suspension-02.rs"
```

### 3. `src/lib.rs`

Register the pure policy and its native checks.

**Locate this existing block:**

```rust
pub mod diagnostic;
```

After that block, type:

```rust
pub mod suspension;
#[cfg(test)]
mod suspension_tests;
```

## <a id="checkpoint-34gbba-browser"></a>6. Connect visibility and freezing to heartbeat scheduling

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/report_lifecycle.rs, sr…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Document and Window events</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">independent pause reasons</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">stop or retain one heartb…</text>
</svg>
</div>

**What this code does:** Pause the actual diagnostic timer while any browser suspension reason remains.

### 1. `src/report_lifecycle.rs`

Use the scheduling policy in the existing metadata owner. Store pause reasons beside the owned listeners and allocation token. Keep replacement identity independent of scheduling state. Read or update scheduling state without retaining a borrow while the timer changes. Read the initial hidden state and retain Document for later visibility observations. Pause cached scheduling; close the policy permanently before final cleanup is deferred. On page return, clear only the cached reason and refresh actual visibility. Observe Document visibility and freezing, then synchronize one timer after updating all affected reasons. Bind all three Document events; partial registration still cleans up through the listener owner.

Replace the entire file with:

```rust
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use crate::{browser_report as report, listeners::Listeners, suspension::{Reason, Suspension}};

thread_local! { static ACTIVE: RefCell<Option<(Rc<()>, Listeners, Suspension)>> = const { RefCell::new(None) }; }

fn current(token: &Rc<()>) -> bool {
    ACTIVE.with(|slot| slot.borrow().as_ref().is_some_and(|(id, _, _)| Rc::ptr_eq(id, token)))
}

pub fn permits() -> bool {
    ACTIVE.with(|slot| slot.borrow().as_ref().is_some_and(|(_, _, state)| state.permits()))
}

fn change(reason: Reason, paused: bool) {
    ACTIVE.with(|slot| { if let Some((_, _, state)) = slot.borrow_mut().as_mut() { state.change(reason, paused); } });
}

#[wasm_bindgen::prelude::wasm_bindgen(js_name = stop_report_lifecycle)]

pub fn stop() -> bool {
    let owner = ACTIVE.with(|slot| slot.borrow_mut().take());
    let active = owner.is_some(); report::stop_periodic(); drop(owner); active
}

#[wasm_bindgen::prelude::wasm_bindgen(js_name = install_report_lifecycle)]
pub fn install() -> Result<(), JsValue> {
    stop(); let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let hidden = document.hidden(); let observed = document.clone();
    let token = Rc::new(()); let guarded = Rc::clone(&token);
    let callback = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        if !current(&guarded) { return; }
        match event.type_().as_str() {
            "pagehide" => {
                let cached = event.dyn_ref::<web_sys::PageTransitionEvent>().is_some_and(|event| event.persisted());
                let _ = report::observe("lifecycle", if cached { "pagehide cached" } else { "pagehide final" });
                change(Reason::Cached, true);
                report::stop_periodic();
                if !cached {
                    ACTIVE.with(|slot| { if let Some((_, _, state)) = slot.borrow_mut().as_mut() { state.close(); } });
                    let _ = report::close_report(); let token = Rc::clone(&guarded);
                    wasm_bindgen_futures::spawn_local(async move { if current(&token) { stop(); } });
                }
            }
            "pageshow" => {
                let _ = report::observe("lifecycle", "pageshow");
                change(Reason::Cached, false); change(Reason::Hidden, observed.hidden());
            }
            "visibilitychange" => {
                change(Reason::Hidden, observed.hidden());
                let _ = report::observe("lifecycle", if observed.hidden() { "hidden" } else { "visible" });
            }
            "freeze" | "resume" => {
                change(Reason::Frozen, event.type_() == "freeze");
                change(Reason::Hidden, observed.hidden());
                let _ = report::observe("lifecycle", &event.type_());
            }
            _ => return,
        }
        if let Err(error) = report::start_periodic() {
            let _ = report::observe("diagnostic", &format!("Heartbeat unavailable: {error:?}"));
        }
    });
    let mut listeners = Listeners::new(callback);
    listeners.listen(window.as_ref(), "pagehide")?; listeners.listen(window.as_ref(), "pageshow")?;
    for name in ["visibilitychange", "freeze", "resume"] { listeners.listen(document.as_ref(), name)?; }
    ACTIVE.with(|slot| slot.replace(Some((token, listeners, Suspension::new(hidden))))); Ok(())
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn report_lifecycle_running() -> bool { ACTIVE.with(|slot| slot.borrow().is_some()) }
```

### 2. `src/browser_report.rs`

Refuse suspended scheduling and retain an existing timer, so repeated resume events cannot restart or duplicate it.

**Locate this existing block:**

```rust
pub fn start_periodic() -> Result<(), JsValue> {
    stop_periodic();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn start_periodic() -> Result<(), JsValue> {
    if !crate::report_lifecycle::permits() { stop_periodic(); return Ok(()); }
    if BEAT.with(|slot| slot.borrow().is_some()) { return Ok(()); }
```

## <a id="checkpoint-34gbc-authority"></a>7. Give pending startup a revocable ticket

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/startup.rs, src/startup…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">task ticket + close ticket</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared flag</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">delayed result refused af…</text>
</svg>
</div>

**What this code does:** Prevent a waiting task from regaining startup permission after closure.

### 1. `src/startup.rs`

Share startup permission between the waiting task and its close callback.

Create the file and type:

```rust
--8<-- "journey/code/34gbc-authority-01.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/startup_tests.rs`

Optional authoring fixture. Check delayed tickets, repeated closure and an independent replacement owner.

Copy this supplied file:

```text
--8<-- "journey/code/34gbc-authority-02.rs"
```

### 3. `src/lib.rs`

Register startup authority and its native checks.

**Locate this existing block:**

```rust
pub mod suspension;
```

After that block, type:

```rust
pub mod startup;
#[cfg(test)]
mod startup_tests;
```

## <a id="checkpoint-34gbca-startup"></a>8. Refuse GPU results after final page exit

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/browser_startup.rs, src…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">final exit</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">adapter/device resolves</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">refuse renderer installat…</text>
</svg>
</div>

**What this code does:** Stop delayed adapter or device delivery from installing a renderer after final exit.

### 1. `src/browser_startup.rs`

Own one temporary page-exit binding across GPU awaits, independent of optional diagnostic listeners.

Create the file and type:

```rust
--8<-- "journey/code/34gbca-startup-01.rs"
```

### 2. `src/lib.rs`

Register the browser startup guard.

**Locate this existing block:**

```rust
mod report_lifecycle;
```

After that block, type:

```rust
#[cfg(target_arch = "wasm32")]
mod browser_startup;
```

### 3. `src/browser.rs`

Keep the guard alive from startup until the runtime owns the drawing callbacks. After the adapter await, refuse a revoked result before requesting a device or interpreting an error. After the device await, destroy a late device and return quietly before configuring the surface or creating resources.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
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
```

Replace it with:

```rust
            compatible_surface: Some(&surface),
            ..Default::default()
        })
        .await;
    if !startup.permits() { return Ok(()); }
    let adapter = adapter.map_err(|error| JsValue::from_str(&error.to_string()))?;
    let device = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await;
    if !startup.permits() {
        if let Ok((device, _)) = device { device.destroy(); }
        return Ok(());
    }
    let (device, queue) = device.map_err(|error| JsValue::from_str(&error.to_string()))?;
    let fault = crate::gpu_fault::Fault::default();
    let errors = fault.clone();
    device.on_uncaptured_error(std::sync::Arc::new(move |error| failed(errors.clone(), format!("WebGPU error: {error}"))));
```

**Locate this block:**

```rust
        .ok_or("Missing canvas")?
        .dyn_into()?;
    crate::browser_report::start()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        flags: Default::default(),
```

Replace it with:

```rust
        .ok_or("Missing canvas")?
        .dyn_into()?;
    crate::browser_report::start()?;
    let startup = crate::browser_startup::Guard::new()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        flags: Default::default(),
```

## Build & Run

![Recorded visual output for Lesson 4.4 (34gbca-startup)](../../screenshots/journey/34gbca-startup-browser.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 4.4
npm --prefix ../session_tests run course -- save 4.4
```

Continue with [4.5 · Measured phases and conservative GPU recovery](34gc-messages.md).
