# 4.2 · First failure and independent diagnostic reports

**Steps:** 6 · **Source:** `src/lib.rs` · `src/selftest/lifecycle.rs` · `src/engine/gpu/device.rs`

## <a id="checkpoint-34-fault"></a>1. Keep the first GPU failure with its device

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="214" viewBox="0 0 1120 214" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr-430deea4" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arr-r-430deea4" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arr-g-430deea4" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arr-b-430deea4" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="grid-430deea4" width="20" height="20" patternUnits="userSpaceOnUse">
      <path d="M 20 0 L 0 0 0 20" fill="none" stroke="#f1f5f9" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="210" rx="8" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="2" y="2" width="1116" height="28" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="2" y1="30" x2="1118" y2="30" stroke="#111111" stroke-width="1.5"/>
  <rect x="12" y="7" width="176" height="18" rx="3" fill="#111111"/>
  <text x="100" y="19.5" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#ffffff" paint-order="normal" stroke="none">34..34F · FAULT &amp; RING</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="518" y="20" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">First-Fault Latch (Device Lost / Validation), Render Stop &amp; 3-Run localStorage Ring</text>
  <rect x="838" y="7" width="268" height="18" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="972" y="19.5" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">latched_fault.get_or_init(err) · ring[0..3]</text>
  <rect x="14" y="38" width="344" height="164" rx="6" fill="url(#grid-430deea4)" stroke="#111111" stroke-width="1.5"/>
  <line x1="14" y1="60" x2="358" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1 · First GPU Fault Latched (Zero Cascade)</text>
  <rect x="44" y="76" width="284" height="104" rx="4" fill="#fee2e2" stroke="#dc2626" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">onuncapturederror / device.lost</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="122" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">1. Latch root-cause FaultRecord once</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="142" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">2. Halt requestAnimationFrame loop</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="162" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">3. Surface copyable JSON report in UI</text>
  <line x1="358" y1="120" x2="384" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-430deea4)"/>
  <rect x="388" y="38" width="344" height="164" rx="6" fill="url(#grid-430deea4)" stroke="#111111" stroke-width="1.5"/>
  <line x1="388" y1="60" x2="732" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2 · Bounded 3-Run localStorage Ring</text>
  <rect x="414" y="84" width="90" height="76" rx="4" fill="#f1f5f9" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="459" y="116" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">Run n-2</text>
  <rect x="515" y="84" width="90" height="76" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="116" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">Run n-1</text>
  <rect x="616" y="84" width="90" height="76" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="661" y="116" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#16a34a">Run n</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="180" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#111111">Evicts oldest run &amp; corrupt schema entries</text>
  <line x1="732" y1="120" x2="758" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-430deea4)"/>
  <rect x="762" y="38" width="344" height="164" rx="6" fill="url(#grid-430deea4)" stroke="#111111" stroke-width="1.75"/>
  <line x1="762" y1="60" x2="1106" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3 · Diagnostic Report Download</text>
  <rect x="806" y="68" width="256" height="122" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="826" y="86" width="216" height="40" rx="4" fill="#fef9c3" stroke="#d97706" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="110" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Download fault-report.json</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="156" text-anchor="middle" font-family="monospace" font-size="9" fill="#16a34a">Survives browser tab crash ✓</text>
</svg>
</div>

### 1. `src/gpu_fault.rs`

Create the file and type:

```rust
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct Fault(Arc<Mutex<Option<String>>>);

impl Fault {
    pub fn remember(&self, message: String) -> bool {
        let mut first = self.0.lock().unwrap();
        if first.is_some() {
            return false;
        }
        *first = Some(message);
        true
    }

    pub fn message(&self) -> Option<String> {
        self.0.lock().unwrap().clone()
    }

    pub fn same_device(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}
```

### 2. `src/gpu_fault_tests.rs`

Create the file and type:

```rust
use crate::gpu_fault::Fault;

#[test]
fn cloned_callback_keeps_the_first_reason() {
    let fault = Fault::default();
    let callback = fault.clone();
    assert!(fault.message().is_none());
    assert!(callback.same_device(&fault));
    assert!(callback.remember("device lost: original reason".into()));
    assert!(!fault.remember("validation error after loss".into()));
    assert_eq!(
        fault.message().as_deref(),
        Some("device lost: original reason")
    );
}

#[test]
fn replacement_device_has_an_independent_fault() {
    let old = Fault::default();
    let replacement = Fault::default();
    assert!(!old.same_device(&replacement));
    assert!(old.remember("late failure from old device".into()));
    assert!(replacement.message().is_none());
    assert!(replacement.remember("new device failed".into()));
    assert_eq!(
        old.message().as_deref(),
        Some("late failure from old device")
    );
}

#[test]
fn concurrent_callbacks_accept_exactly_one_reason() {
    let fault = Fault::default();
    let a = fault.clone();
    let b = fault.clone();
    let a = std::thread::spawn(move || a.remember("first contender".into()));
    let b = std::thread::spawn(move || b.remember("second contender".into()));
    assert_ne!(a.join().unwrap(), b.join().unwrap());
    assert!(matches!(
        fault.message().as_deref(),
        Some("first contender" | "second contender")
    ));
}
```

### 3. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod gesture;
pub mod history;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod gesture;
pub mod gpu_fault;
#[cfg(test)]
mod gpu_fault_tests;
pub mod history;
```

## <a id="checkpoint-34a-stop"></a>2. Stop the viewer when its GPU device fails

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU (Rust / WASM): Adapter -&gt; (Device, Queue)</text>
  <rect x="48" y="50" width="118" height="44" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="107" y="76" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">wgpu::Instance</text>
  <line x1="166" y1="72" x2="196" y2="72" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="200" y="46" width="124" height="52" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="262" y="68" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">wgpu::Device</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="262" y="86" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">+ wgpu::Queue</text>
  <rect x="74" y="112" width="224" height="38" rx="4" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="135" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Rc&lt;RefCell&lt;BrowserState&gt;&gt;</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU -&gt; GPU Bus: surface.configure(&amp;device, &amp;config)</text>
  <rect x="424" y="50" width="272" height="98" rx="6" fill="#fef9c3" stroke="#d97706" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="74" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">SurfaceConfiguration</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="96" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">format: Bgra8UnormSrgb</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="116" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#2563eb">width × height: clamp(1..max_2d)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="136" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#16a34a">present_mode: Fifo (VSync)</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">GPU Swapchain Texture -&gt; Browser &lt;canvas&gt;</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="826" y="58" width="216" height="58" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="84" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">get_current_texture()</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="102" text-anchor="middle" font-family="monospace" font-size="9" fill="#2563eb">-&gt; wgpu::TextureView</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="142" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#16a34a">frame.present() ✓</text>
</svg>
</div>

### 1. `src/browser_runtime.rs`

**Locate this existing block:**

```rust
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
    ACTIVE.with(|slot| {
        slot.replace(Some(Runtime {
            _listeners: listeners,
            request,
            reload,
        }))
    });
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
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

pub fn install(
    listeners: Listeners,
    request: Rc<RefCell<ReadGate>>,
    reload: Shared,
    fault: crate::gpu_fault::Fault,
) {
    ACTIVE.with(|slot| {
        slot.replace(Some(Runtime {
            _listeners: listeners,
            request,
            reload,
            fault,
        }))
    });
}

pub fn stop_if(fault: &crate::gpu_fault::Fault) -> bool {
    let owner = ACTIVE.with(|slot| {
        let mut current = slot.borrow_mut();
        if current
            .as_ref()
            .is_some_and(|runtime| runtime.fault.same_device(fault))
        {
            current.take()
        } else {
            None
        }
    });
    owner.is_some()
}
```

### 2. `src/browser.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    listeners.listen(window.as_ref(), "viewer-file-error")?;
    listeners.listen(window.as_ref(), "viewer-reload")?;
    listeners.listen(window.as_ref(), "pagehide")?;
    crate::browser_runtime::install(listeners, owned_request, owned_reload);
    report(
```

Replace it with:

```rust
    listeners.listen(window.as_ref(), "viewer-file-error")?;
    listeners.listen(window.as_ref(), "viewer-reload")?;
    listeners.listen(window.as_ref(), "pagehide")?;
    crate::browser_runtime::install(listeners, owned_request, owned_reload, fault);
    report(
```

**Locate this block:**

```rust
        std::rc::Rc::new(std::cell::RefCell::new(None));
    let owned_request = std::rc::Rc::clone(&request);
    let owned_reload = std::rc::Rc::clone(&reload);
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(
        move |event: web_sys::Event| {
            if event.type_() == "pagehide" {
                gesture.cancel();
                if event
```

Replace it with:

```rust
        std::rc::Rc::new(std::cell::RefCell::new(None));
    let owned_request = std::rc::Rc::clone(&request);
    let owned_reload = std::rc::Rc::clone(&reload);
    let active_fault = fault.clone();
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(
        move |event: web_sys::Event| {
            if active_fault.message().is_some() {
                return;
            }
            if event.type_() == "pagehide" {
                gesture.cancel();
                if event
```

**Locate this block:**

```rust
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    device.on_uncaptured_error(std::sync::Arc::new(|error| {
        report(&error.to_string())
    }));
    let limit = device.limits().max_texture_dimension_2d;
    let density = window.device_pixel_ratio();
    let density = if density.is_finite() && density > 0.0 {
```

Replace it with:

```rust
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let fault = crate::gpu_fault::Fault::default();
    let errors = fault.clone();
    device.on_uncaptured_error(std::sync::Arc::new(move |error| {
        failed(
            errors.clone(),
            format!("WebGPU error: {error}"),
        )
    }));
    let lost = fault.clone();
    device.set_device_lost_callback(move |reason, message| {
        failed(
            lost.clone(),
            format!("WebGPU device lost ({reason:?}): {message}"),
        )
    });
    let limit = device.limits().max_texture_dimension_2d;
    let density = window.device_pixel_ratio();
    let density = if density.is_finite() && density > 0.0 {
```

**Locate this block:**

```rust
}

fn save_result(bytes: &[u8]) -> Result<String, String> {
    crate::file_output::download(bytes)
        .map(|()| {
```

Replace it with:

```rust
}

fn failed(fault: crate::gpu_fault::Fault, message: String) {
    if !fault.remember(message.clone()) {
        return;
    }
    wasm_bindgen_futures::spawn_local(async move {
        if crate::browser_runtime::stop_if(&fault) {
            report(&format!("Cannot draw: {message}"));
        }
    });
}

fn save_result(bytes: &[u8]) -> Result<String, String> {
    crate::file_output::download(bytes)
        .map(|()| {
```

## <a id="checkpoint-34b-report"></a>3. Describe a viewer run without keeping its document

### 1. `src/diagnostic.rs`

Create the file and type:

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
pub enum Outcome {
    Running,
    Ready,
    Closed,
    Failed,
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
}

impl Report {
    pub fn new(tab: String, started: String, context: Context) -> Self {
        Self {
            version: 1,
            tab,
            last_seen: started.clone(),
            started,
            outcome: Outcome::Running,
            context,
        }
    }
}
```

### 2. `src/diagnostic_tests.rs`

Create the file and type:

```rust
use crate::diagnostic::{Context, Outcome, Report};

fn context() -> Context {
    Context {
        page: "https://example.test/viewer/".into(),
        browser: "Test browser".into(),
        secure_context: true,
        webgpu: true,
        viewport: [900, 760],
        canvas: [900, 760],
        device_pixel_ratio: 1.0,
    }
}

#[test]
fn report_starts_running_with_flat_browser_context() {
    let report = Report::new(
        "tab-a".into(),
        "2026-10-03T18:00:00.000Z".into(),
        context(),
    );
    let value = serde_json::to_value(&report).unwrap();
    assert_eq!(value["version"], 1);
    assert_eq!(value["outcome"], "running");
    assert_eq!(value["started"], value["lastSeen"]);
    assert_eq!(value["secureContext"], true);
    assert_eq!(value["devicePixelRatio"], 1.0);
    assert_eq!(
        value["viewport"],
        serde_json::json!([900, 760])
    );
    assert!(value.get("context").is_none());
    assert_eq!(
        serde_json::from_value::<Report>(value).unwrap(),
        report
    );
}

#[test]
fn report_snapshot_keeps_its_own_context() {
    let mut live = Report::new("tab-a".into(), "start".into(), context());
    let snapshot = live.clone();
    live.context.canvas = [618, 1373];
    live.context.device_pixel_ratio = 2.625;
    live.outcome = Outcome::Ready;
    assert_eq!(snapshot.context.canvas, [900, 760]);
    assert_eq!(snapshot.outcome, Outcome::Running);
    assert_eq!(
        serde_json::from_str::<Report>(&serde_json::to_string(&live).unwrap()).unwrap(),
        live
    );
}
```

### 3. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod camera;
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod camera;
pub mod diagnostic;
#[cfg(test)]
mod diagnostic_tests;
#[cfg(test)]
```

## <a id="checkpoint-34ba-events"></a>4. Keep recent events without losing the first failure

### 1. `src/diagnostic.rs`

**Locate this existing block:**

```rust
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
        Self {
            version: 1,
            tab,
            last_seen: started.clone(),
            started,
            outcome: Outcome::Running,
            context,
        }
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
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
        Self {
            version: 1,
            tab,
            last_seen: started.clone(),
            started,
            outcome: Outcome::Running,
            context,
            events: Default::default(),
            failure: None,
        }
    }

    pub fn failure(&self) -> Option<&Event> {
        self.failure.as_ref()
    }

    pub fn record(
        &mut self,
        time: String,
        elapsed_ms: f64,
        kind: &str,
        message: &str,
    ) -> Result<(), &'static str> {
        if !elapsed_ms.is_finite() || elapsed_ms < 0.0 {
            return Err("Invalid elapsed time");
        }
        let event = Event {
            time,
            elapsed_ms,
            kind: kind.chars().take(64).collect(),
            message: message.chars().take(4096).collect(),
        };
        self.last_seen.clone_from(&event.time);
        if kind == "fatal" {
            if self.failure.is_none() {
                self.failure = Some(event.clone());
            }
            self.outcome = Outcome::Failed;
        } else if kind == "milestone"
            && message == "geometry on screen"
            && self.failure.is_none()
        {
            self.outcome = Outcome::Ready;
        }
        if self.events.len() == 24 {
            self.events.pop_front();
        }
        self.events.push_back(event);
        Ok(())
    }
```

### 2. `src/diagnostic_tests.rs`

**Locate this existing block:**

```rust
        live
    );
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        live
    );
}

#[test]
fn first_failure_survives_later_events() {
    let mut report = Report::new("tab".into(), "start".into(), context());
    report
        .record("first".into(), 1.0, "fatal", "device lost")
        .unwrap();
    let first = report.failure().unwrap().clone();
    for i in 0..30 {
        report
            .record(
                format!("late-{i}"),
                i as f64 + 2.0,
                "fatal",
                &format!("later-{i}"),
            )
            .unwrap();
    }
    assert_eq!(report.failure(), Some(&first));
    let value = serde_json::to_value(&report).unwrap();
    assert_eq!(value["events"].as_array().unwrap().len(), 24);
    assert_eq!(value["events"][0]["message"], "later-6");
    report
        .record(
            "drawn".into(),
            40.0,
            "milestone",
            "geometry on screen",
        )
        .unwrap();
    assert_eq!(report.outcome, Outcome::Failed);
    assert_eq!(report.failure(), Some(&first));
    assert_eq!(
        serde_json::from_str::<Report>(&serde_json::to_string(&report).unwrap())
            .unwrap(),
        report
    );
}

#[test]
fn invalid_time_is_atomic_and_text_is_bounded() {
    let mut report = Report::new("tab".into(), "start".into(), context());
    report
        .record(
            "drawn".into(),
            1.0,
            "milestone",
            "geometry on screen",
        )
        .unwrap();
    assert_eq!(report.outcome, Outcome::Ready);
    report
        .record(
            "later".into(),
            2.0,
            &"x".repeat(65),
            &"😀".repeat(4097),
        )
        .unwrap();
    let value = serde_json::to_value(&report).unwrap();
    assert_eq!(
        value["events"][1]["kind"]
            .as_str()
            .unwrap()
            .chars()
            .count(),
        64
    );
    assert_eq!(
        value["events"][1]["message"]
            .as_str()
            .unwrap()
            .chars()
            .count(),
        4096
    );
    let before = report.clone();
    for invalid in [f64::NAN, f64::INFINITY, -1.0] {
        assert!(
            report
                .record("bad".into(), invalid, "fatal", "bad time")
                .is_err()
        );
    }
    assert_eq!(report, before);
}
```

## <a id="checkpoint-34c-browser"></a>5. Read live diagnostic context outside the GPU runtime

### 1. `src/browser_report.rs`

Create the file and type:

```rust
use crate::diagnostic::{Context, Report};
use std::cell::RefCell;
use wasm_bindgen::{JsCast, JsValue};

thread_local! { static REPORT: RefCell<Option<Report>> = const { RefCell::new(None) }; }

fn context() -> Result<Context, JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .ok_or("No canvas")?
        .dyn_into()?;
    let location = window.location();
    let navigator = window.navigator();
    Ok(Context {
        page: format!(
            "{}{}",
            location.origin()?,
            location.pathname()?
        ),
        browser: navigator.user_agent()?,
        secure_context: window.is_secure_context(),
        webgpu: js_sys::Reflect::get(navigator.as_ref(), &"gpu".into())?.is_object(),
        viewport: [
            window
                .inner_width()?
                .as_f64()
                .ok_or("No width")? as u32,
            window
                .inner_height()?
                .as_f64()
                .ok_or("No height")? as u32,
        ],
        canvas: [canvas.width(), canvas.height()],
        device_pixel_ratio: window.device_pixel_ratio(),
    })
}

pub fn start() -> Result<(), JsValue> {
    let report = Report::new(
        uuid::Uuid::new_v4().to_string(),
        js_sys::Date::new_0().to_iso_string().into(),
        context()?,
    );
    REPORT.with(|slot| slot.replace(Some(report)));
    Ok(())
}

pub fn observe(kind: &str, message: &str) -> Result<(), JsValue> {
    let context = context()?;
    let elapsed = web_sys::window()
        .and_then(|window| window.performance())
        .ok_or("No clock")?
        .now();
    REPORT.with(|slot| {
        let mut slot = slot.borrow_mut();
        let report = slot.as_mut().ok_or("No report")?;
        report.context = context;
        report
            .record(
                js_sys::Date::new_0().to_iso_string().into(),
                elapsed,
                kind,
                message,
            )
            .map_err(JsValue::from_str)
    })
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn diagnostic_snapshot() -> Result<String, JsValue> {
    let context = context()?;
    REPORT.with(|slot| {
        let mut slot = slot.borrow_mut();
        let report = slot.as_mut().ok_or("No report")?;
        report.context = context;
        serde_json::to_string_pretty(report)
            .map_err(|error| JsValue::from_str(&error.to_string()))
    })
}
```

### 2. `Cargo.toml`

**Locate this existing block:**

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
console_error_panic_hook = "=0.1.7"
```

Edit the existing block to match this code; keep unchanged lines:

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "Location", "Navigator", "Performance", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
console_error_panic_hook = "=0.1.7"
```

### 3. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(target_arch = "wasm32")]
mod browser_runtime;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(target_arch = "wasm32")]
mod browser_report;
#[cfg(target_arch = "wasm32")]
mod browser_runtime;
```

### 4. `src/browser.rs`

**Locate this existing block:**

```rust
        .dyn_into()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .dyn_into()?;
    crate::browser_report::start()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
```

## <a id="checkpoint-34d-download"></a>6. Download diagnostics through the real command line

### 1. `src/file_output.rs`

**Locate this existing block:**

```rust
pub fn download(bytes: &[u8]) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let body = document.body().ok_or("No document body")?;
    let anchor: web_sys::HtmlAnchorElement =
        document.create_element("a")?.dyn_into()?;
    anchor.set_hidden(true);
    anchor.set_download("viewer.session");
    body.append_child(&anchor)?;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn download(bytes: &[u8]) -> Result<(), JsValue> {
    download_named(bytes, "viewer.session")
}

pub fn download_named(bytes: &[u8], name: &str) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let body = document.body().ok_or("No document body")?;
    let anchor: web_sys::HtmlAnchorElement =
        document.create_element("a")?.dyn_into()?;
    anchor.set_hidden(true);
    anchor.set_download(name);
    body.append_child(&anchor)?;
```

### 2. `src/browser_report.rs`

**Locate this existing block:**

```rust
}

#[wasm_bindgen::prelude::wasm_bindgen]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub fn download() -> Result<(), JsValue> {
    let json = diagnostic_snapshot()?;
    crate::file_output::download_named(json.as_bytes(), "viewer-diagnostic.json")
}

pub fn fatal(message: &str) {
    let first = REPORT.with(|slot| {
        slot.borrow()
            .as_ref()
            .is_some_and(|report| report.failure().is_none())
    });
    if observe("fatal", message).is_ok() && first {
        let _ = download();
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
```

### 3. `src/browser.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                        input.click();
                    }
                    None
```

Replace it with:

```rust
                        input.click();
                    }
                    None
                } else if line == "report" {
                    let result = crate::browser_report::download()
                        .map(|()| "Diagnostic report downloaded.".to_owned())
                        .unwrap_or_else(|error| {
                            format!("Diagnostic report failed: {error:?}")
                        });
                    panel.result(&result);
                    None
```

**Locate this block:**

```rust
            &editor.camera.uniform(),
            &mut panel,
        )?;
    }
    let input_canvas = canvas.clone();
    let browser_window = window.clone();
```

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

**Locate this block:**

```rust
            "Reload Sources",
            "Cancel Reload",
            "Save",
            "Example Box",
            "Example Triangle",
            "Select Next",
```

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

**Locate this block:**

```rust
        if crate::browser_runtime::stop_if(&fault) {
            report(&format!("Cannot draw: {message}"));
        }
    });
```

Replace it with:

```rust
        if crate::browser_runtime::stop_if(&fault) {
            crate::browser_report::fatal(&message);
            report(&format!("Cannot draw: {message}"));
        }
    });
```

### 4. `src/lib.rs`

**Locate this existing block:**

```rust
        if let Err(error) = browser::run().await {
            browser::report(&format!("Cannot draw: {error:?}"));
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        if let Err(error) = browser::run().await {
            browser_report::fatal(&format!("Cannot draw: {error:?}"));
            browser::report(&format!("Cannot draw: {error:?}"));
```

## Build & Run

![Recorded visual output for Lesson 4.2 (34d-download)](../../screenshots/journey/34d-download-browser.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 4.2
npm --prefix ../session_tests run course -- save 4.2
```

Continue with [4.3 · Admit useful evidence and bound stored runs](34e-schema.md).
