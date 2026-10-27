# 4.3 · Admit useful evidence and bound stored runs

**Steps:** 8 · **Source:** `src/lib.rs` · `src/selftest/lifecycle.rs` · `src/engine/gpu/device.rs`

## <a id="checkpoint-34e-schema"></a>1. Check the bounded shape of a report

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="214" viewBox="0 0 1120 214" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr-14bc6aa1" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arr-r-14bc6aa1" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arr-g-14bc6aa1" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arr-b-14bc6aa1" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="grid-14bc6aa1" width="20" height="20" patternUnits="userSpaceOnUse">
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
  <rect x="14" y="38" width="344" height="164" rx="6" fill="url(#grid-14bc6aa1)" stroke="#111111" stroke-width="1.5"/>
  <line x1="14" y1="60" x2="358" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1 · First GPU Fault Latched (Zero Cascade)</text>
  <rect x="44" y="76" width="284" height="104" rx="4" fill="#fee2e2" stroke="#dc2626" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">onuncapturederror / device.lost</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="122" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">1. Latch root-cause FaultRecord once</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="142" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">2. Halt requestAnimationFrame loop</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="162" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">3. Surface copyable JSON report in UI</text>
  <line x1="358" y1="120" x2="384" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-14bc6aa1)"/>
  <rect x="388" y="38" width="344" height="164" rx="6" fill="url(#grid-14bc6aa1)" stroke="#111111" stroke-width="1.5"/>
  <line x1="388" y1="60" x2="732" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2 · Bounded 3-Run localStorage Ring</text>
  <rect x="414" y="84" width="90" height="76" rx="4" fill="#f1f5f9" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="459" y="116" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">Run n-2</text>
  <rect x="515" y="84" width="90" height="76" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="116" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">Run n-1</text>
  <rect x="616" y="84" width="90" height="76" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="661" y="116" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#16a34a">Run n</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="180" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#111111">Evicts oldest run &amp; corrupt schema entries</text>
  <line x1="732" y1="120" x2="758" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-14bc6aa1)"/>
  <rect x="762" y="38" width="344" height="164" rx="6" fill="url(#grid-14bc6aa1)" stroke="#111111" stroke-width="1.75"/>
  <line x1="762" y1="60" x2="1106" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3 · Diagnostic Report Download</text>
  <rect x="806" y="68" width="256" height="122" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="826" y="86" width="216" height="40" rx="4" fill="#fef9c3" stroke="#d97706" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="110" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Download fault-report.json</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="156" text-anchor="middle" font-family="monospace" font-size="9" fill="#16a34a">Survives browser tab crash ✓</text>
</svg>
</div>

### 1. `src/diagnostic.rs`

**Locate this existing block:**

```rust
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
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn valid(&self) -> bool {
        let text = |s: &str, max| s.chars().count() <= max;
        let event = |e: &Event| {
            !e.time.is_empty()
                && text(&e.time, 64)
                && !e.kind.is_empty()
                && text(&e.kind, 64)
                && text(&e.message, 4096)
                && e.elapsed_ms.is_finite()
                && e.elapsed_ms >= 0.0
        };
        self.version == 1
            && !self.tab.is_empty()
            && text(&self.tab, 128)
            && !self.started.is_empty()
            && text(&self.started, 64)
            && !self.last_seen.is_empty()
            && text(&self.last_seen, 64)
            && !self.context.page.is_empty()
            && text(&self.context.page, 4096)
            && !self.context.page.contains(['?', '#'])
            && text(&self.context.browser, 4096)
            && self.context.device_pixel_ratio.is_finite()
            && self.context.device_pixel_ratio > 0.0
            && self.events.len() <= 24
            && self.events.iter().all(event)
            && self
                .events
                .iter()
                .all(|e| e.kind != "fatal" || self.failure.is_some())
            && match &self.failure {
                Some(first) => {
                    event(first)
                        && first.kind == "fatal"
                        && self.outcome == Outcome::Failed
                }
                None => self.outcome != Outcome::Failed,
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
        while self.events.len() >= 24 {
            self.events.pop_front();
```

### 2. `src/diagnostic_shape_tests.rs`

Create the file and type:

```rust
use crate::diagnostic::{Context, Report};

pub(crate) fn report() -> Report {
    Report::new(
        "tab".into(),
        "2026-10-03T18:00:00.000Z".into(),
        Context {
            page: "https://example.test/".into(),
            browser: "Test".into(),
            secure_context: true,
            webgpu: true,
            viewport: [900, 760],
            canvas: [900, 760],
            device_pixel_ratio: 1.0,
        },
    )
}

#[test]
fn shape_rejects_bad_metadata() {
    let good = serde_json::to_value(report()).unwrap();
    for (key, bad) in [
        ("version", serde_json::json!(2)),
        ("tab", serde_json::json!("")),
        (
            "page",
            serde_json::json!("https://example.test/?private=1"),
        ),
        ("devicePixelRatio", serde_json::json!(0)),
        ("outcome", serde_json::json!("failed")),
    ] {
        let mut value = good.clone();
        value[key] = bad;
        assert!(
            !serde_json::from_value::<Report>(value)
                .unwrap()
                .valid(),
            "{key}"
        );
    }
    assert!(report().valid());
}

#[test]
fn shape_bounds_events_and_failure() {
    let mut failure = report();
    failure
        .record("later".into(), 1.0, "fatal", "first")
        .unwrap();
    assert!(failure.valid());
    let mut value = serde_json::to_value(failure).unwrap();
    let event = serde_json::json!({"time":"now","kind":"phase","message":"test","elapsedMs":1.0});
    value["events"] = serde_json::json!(vec![event; 25]);
    let oversized: Report = serde_json::from_value(value.clone()).unwrap();
    assert!(!oversized.valid());
    value["events"] = serde_json::json!([]);
    value["failure"]["message"] = serde_json::json!("😀".repeat(4097));
    assert!(
        !serde_json::from_value::<Report>(value.clone())
            .unwrap()
            .valid()
    );
    value["failure"]["message"] = serde_json::json!("first");
    value["failure"]["kind"] = serde_json::json!("phase");
    assert!(
        !serde_json::from_value::<Report>(value)
            .unwrap()
            .valid()
    );
}
```

### 3. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(test)]
mod diagnostic_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod diagnostic_shape_tests;
#[cfg(test)]
mod diagnostic_tests;
```

## <a id="checkpoint-34ea-decode"></a>2. Admit only supported bounded saved JSON

### 1. `src/report_store.rs`

Create the file and type:

```rust
use crate::diagnostic::Report;

pub const MAX_BYTES: usize = 1024 * 1024;

pub fn decode(text: &str) -> Option<Report> {
    if text.len() > MAX_BYTES {
        return None;
    }
    let value: serde_json::Value = serde_json::from_str(text).ok()?;
    let fields = [
        "version",
        "tab",
        "started",
        "lastSeen",
        "outcome",
        "page",
        "browser",
        "secureContext",
        "webgpu",
        "viewport",
        "canvas",
        "devicePixelRatio",
        "events",
        "failure",
    ];
    if value
        .as_object()?
        .keys()
        .any(|key| !fields.contains(&key.as_str()))
    {
        return None;
    }
    let report: Report = serde_json::from_value(value).ok()?;
    report.valid().then_some(report)
}
```

### 2. `src/report_store_tests.rs`

Create the file and type:

```rust
use crate::{
    diagnostic::Report,
    diagnostic_shape_tests::report,
    report_store::{MAX_BYTES, decode},
};

#[test]
fn stored_json_rejects_invalid_shape() {
    let good = serde_json::to_value(report()).unwrap();
    for (key, bad) in [
        ("version", serde_json::json!(2)),
        ("tab", serde_json::json!("")),
        (
            "page",
            serde_json::json!("https://example.test/?private=1"),
        ),
        ("devicePixelRatio", serde_json::json!(0)),
        (
            "events",
            serde_json::json!([{"time":"now","kind":"fatal","message":"bad","elapsedMs":null}]),
        ),
        ("outcome", serde_json::json!("failed")),
        ("unknown", serde_json::json!(true)),
    ] {
        let mut value = good.clone();
        value[key] = bad;
        assert!(decode(&value.to_string()).is_none(), "{key}");
    }
    assert!(decode("{").is_none());
}

#[test]
fn stored_json_bounds_bytes_events_and_text() {
    let json = serde_json::to_string(&report()).unwrap();
    let padding = MAX_BYTES - json.len();
    let padded = json + &" ".repeat(padding);
    assert!(decode(&padded).is_some());
    assert!(decode(&(padded + " ")).is_none());
    let mut value = serde_json::to_value(report()).unwrap();
    let event = serde_json::json!({"time":"now","kind":"phase","message":"test","elapsedMs":1.0});
    value["events"] = serde_json::json!(vec![event; 25]);
    assert!(decode(&value.to_string()).is_none());
    let mut bypass: Report = serde_json::from_value(value).unwrap();
    bypass
        .record("later".into(), 2.0, "phase", "bounded")
        .unwrap();
    assert_eq!(
        serde_json::to_value(bypass).unwrap()["events"]
            .as_array()
            .unwrap()
            .len(),
        24
    );
    let mut failure = report();
    failure
        .record("later".into(), 1.0, "fatal", "first reason")
        .unwrap();
    let json = serde_json::to_string(&failure).unwrap();
    assert_eq!(decode(&json), Some(failure));
    let mut value: serde_json::Value = serde_json::from_str(&json).unwrap();
    value["failure"]["message"] = serde_json::json!("😀".repeat(4097));
    assert!(decode(&value.to_string()).is_none());
}
```

### 3. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod reload_url;
pub mod row_metadata;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod reload_url;
pub mod report_store;
#[cfg(test)]
mod report_store_tests;
pub mod row_metadata;
```

## <a id="checkpoint-34eb-recency"></a>3. Choose a recent failure without blaming active tabs

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

### 1. `src/report_recency.rs`

Create the file and type:

```rust
use crate::diagnostic::{Outcome, Report};

pub fn eligible(
    report: &Report,
    now: f64,
    tab: &str,
    parse: impl Fn(&str) -> f64,
) -> Option<f64> {
    if !report.valid() || !now.is_finite() || now < 0.0 {
        return None;
    }
    let started = parse(&report.started);
    let seen = parse(&report.last_seen);
    if !started.is_finite() || !seen.is_finite() || started > seen || seen > now {
        return None;
    }
    match report.outcome {
        Outcome::Failed => {
            let failed = parse(&report.failure()?.time);
            (failed.is_finite()
                && failed >= started
                && failed <= seen
                && now - failed < 7_200_000.0)
                .then_some(failed)
        }
        Outcome::Running
            if now - seen < 7_200_000.0
                && (report.tab == tab || now - seen > 120_000.0) =>
        {
            Some(seen)
        }
        _ => None,
    }
}
```

### 2. `src/report_recency_tests.rs`

Create the file and type:

```rust
use crate::{
    diagnostic::Outcome, diagnostic_shape_tests::report, report_recency::eligible,
};

#[test]
fn previous_run_selection_respects_actual_failure_time() {
    let now = 1_000_000_000.0;
    for (name, failed, same, seen_age, failure_age, expected) in [
        (
            "old failure, fresh heartbeat",
            true,
            false,
            5000.0,
            259_200_000.0,
            false,
        ),
        (
            "recent failure",
            true,
            false,
            5000.0,
            10000.0,
            true,
        ),
        (
            "failure at cutoff",
            true,
            false,
            5000.0,
            7_200_000.0,
            false,
        ),
        (
            "failure after heartbeat",
            true,
            false,
            5000.0,
            1000.0,
            false,
        ),
        (
            "future heartbeat",
            true,
            false,
            -1.0,
            1000.0,
            false,
        ),
        (
            "future failure",
            true,
            false,
            0.0,
            -1.0,
            false,
        ),
        (
            "active other tab",
            false,
            false,
            120_000.0,
            0.0,
            false,
        ),
        (
            "interrupted other tab",
            false,
            false,
            120_001.0,
            0.0,
            true,
        ),
        (
            "stale other tab",
            false,
            false,
            7_200_000.0,
            0.0,
            false,
        ),
        (
            "interrupted same tab",
            false,
            true,
            0.0,
            0.0,
            true,
        ),
    ] {
        let mut value = report();
        value.started = "start".into();
        if failed {
            value
                .record("failed".into(), 1.0, "fatal", "original")
                .unwrap();
        }
        value.last_seen = "seen".into();
        value.tab = if same {
            "current"
        } else {
            "other"
        }
        .into();
        let parser = |time: &str| match time {
            "start" => now - 259_200_000.0,
            "seen" => now - seen_age,
            "failed" => now - failure_age,
            _ => f64::NAN,
        };
        let selected = eligible(&value, now, "current", parser);
        assert_eq!(selected.is_some(), expected, "{name}");
        if expected {
            assert_eq!(
                selected,
                Some(
                    now - if failed {
                        failure_age
                    } else {
                        seen_age
                    }
                )
            );
        }
    }
}
```

### 3. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod reload_url;
pub mod report_store;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod reload_url;
pub mod report_recency;
#[cfg(test)]
mod report_recency_tests;
pub mod report_store;
```

## <a id="checkpoint-34ec-proof"></a>4. Prove saved-run exclusions before adopting storage

### 1. `src/report_recency_tests.rs`

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

#[test]
fn invalid_clocks_and_healthy_runs_are_not_interruption_evidence() {
    let mut value = report();
    value.started = "start".into();
    value.last_seen = "seen".into();
    let clock = |time: &str| {
        if time == "start" {
            0.0
        } else {
            1000.0
        }
    };
    for now in [f64::NAN, f64::INFINITY, -1.0] {
        assert!(eligible(&value, now, "tab", clock).is_none());
    }
    assert!(eligible(&value, 2000.0, "tab", |_| f64::NAN).is_none());
    assert!(
        eligible(&value, 2000.0, "tab", |time| {
            if time == "start" {
                3000.0
            } else {
                1000.0
            }
        })
        .is_none()
    );
    for state in [Outcome::Ready, Outcome::Closed] {
        value.outcome = state;
        assert!(eligible(&value, 2000.0, "tab", clock).is_none());
    }
}

#[test]
fn failure_before_start_is_rejected_without_mutating_report() {
    let mut value = report();
    value
        .record("failed".into(), 1.0, "fatal", "original")
        .unwrap();
    value.started = "start".into();
    value.last_seen = "seen".into();
    let before = value.clone();
    let clock = |time: &str| match time {
        "start" => 1500.0,
        "seen" => 1900.0,
        "failed" => 1000.0,
        _ => f64::NAN,
    };
    assert!(eligible(&value, 2000.0, "tab", clock).is_none());
    assert_eq!(value, before);
    assert!(
        eligible(&value, 2000.0, "tab", |time| {
            if time == "failed" {
                f64::NAN
            } else {
                clock(time)
            }
        })
        .is_none()
    );
}
```

## <a id="checkpoint-34f-storage"></a>5. Read previous reports from real browser storage

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">DOM Resize / Visibility Events</text>
  <rect x="64" y="54" width="108" height="72" rx="4" fill="#f8fafc" stroke="#111111" stroke-width="1.5" stroke-dasharray="3 3"/>
  <rect x="64" y="54" width="224" height="98" rx="4" fill="none" stroke="#2563eb" stroke-width="2"/>
  <line x1="172" y1="90" x2="276" y2="90" stroke="#2563eb" stroke-width="2" marker-end="url(#arrB)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="224" y="140" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Resize (W, H, DPR)</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Matched Color + Depth32Float Attachments</text>
  <rect x="444" y="56" width="104" height="84" rx="4" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="496" y="102" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Surface</text>
  <rect x="572" y="56" width="104" height="84" rx="4" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="624" y="102" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Depth32</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="156" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">aspect = W / H</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Unstretched Aspect-Correct View</text>
  <rect x="802" y="44" width="264" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="934,58 994,84 934,110 874,84" fill="#93c5fd" stroke="#111111" stroke-width="1.75"/>
  <polygon points="874,84 934,110 934,150 874,124" fill="#60a5fa" stroke="#111111" stroke-width="1.75"/>
  <polygon points="934,110 994,84 994,124 934,150" fill="#2563eb" stroke="#111111" stroke-width="1.75"/>
</svg>
</div>

### 1. `src/report_storage.rs`

Create the file and type:

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
        let session = window
            .as_ref()
            .and_then(|w| w.session_storage().ok().flatten());
        let tab = session
            .as_ref()
            .and_then(|s| s.get_item(PREFIX).ok().flatten())
            .filter(|tab| !tab.is_empty() && tab.chars().count() <= 128)
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if let Some(session) = session {
            let _ = session.set_item(PREFIX, &tab);
        }
        Self {
            tab,
            key: format!("{PREFIX}{}", uuid::Uuid::new_v4()),
            storage: window.and_then(|w| w.local_storage().ok().flatten()),
        }
    }

    pub fn reports(&self) -> Vec<(String, Report)> {
        let Some(storage) = &self.storage else {
            return Vec::new();
        };
        (0..storage.length().unwrap_or(0).min(256))
            .filter_map(|i| storage.key(i).ok().flatten())
            .filter(|key| key.starts_with(PREFIX))
            .take(32)
            .filter_map(|key| {
                let text = storage.get_item(&key).ok().flatten()?;
                Some((key, crate::report_store::decode(&text)?))
            })
            .collect()
    }

    pub fn previous(&self) -> Option<Report> {
        let now = js_sys::Date::now();
        self.reports()
            .into_iter()
            .filter_map(|(_, report)| {
                let score = crate::report_recency::eligible(
                    &report,
                    now,
                    &self.tab,
                    js_sys::Date::parse,
                )?;
                Some((score, report))
            })
            .max_by(|a, b| a.0.total_cmp(&b.0))
            .map(|(_, report)| report)
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn previous_storage_probe() -> String {
    let store = Store::open();
    serde_json::json!({"tab": store.tab, "key": store.key, "previous": store.previous()}).to_string()
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(target_arch = "wasm32")]
mod panel;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(target_arch = "wasm32")]
mod panel;
#[cfg(target_arch = "wasm32")]
pub mod report_storage;
```

### 3. `Cargo.toml`

**Locate this existing block:**

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "Location", "Navigator", "Performance", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
console_error_panic_hook = "=0.1.7"
```

Edit the existing block to match this code; keep unchanged lines:

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "Location", "Navigator", "Storage", "Performance", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
console_error_panic_hook = "=0.1.7"
```

## <a id="checkpoint-34fa-retain"></a>6. Retain only three diagnostic runs

### 1. `src/report_storage.rs`

**Locate this existing block:**

```rust
    }

    pub fn previous(&self) -> Option<Report> {
        let now = js_sys::Date::now();
        self.reports()
            .into_iter()
            .filter_map(|(_, report)| {
                let score = crate::report_recency::eligible(
                    &report,
                    now,
                    &self.tab,
                    js_sys::Date::parse,
                )?;
                Some((score, report))
            })
            .max_by(|a, b| a.0.total_cmp(&b.0))
            .map(|(_, report)| report)
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn previous_storage_probe() -> String {
    let store = Store::open();
    serde_json::json!({"tab": store.tab, "key": store.key, "previous": store.previous()}).to_string()
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn write(&self, report: &Report) -> bool {
        if !report.valid() || report.tab != self.tab {
            return false;
        }
        let Some(storage) = &self.storage else {
            return false;
        };
        let Ok(length) = storage.length() else {
            return false;
        };
        if length > 256 {
            return false;
        }
        let keys: Result<Vec<_>, _> = (0..length).map(|i| storage.key(i)).collect();
        let Ok(keys) = keys else {
            return false;
        };
        let keys: Vec<_> = keys
            .into_iter()
            .flatten()
            .filter(|key| key.starts_with(PREFIX))
            .collect();
        if keys.len() > 32 {
            return false;
        }
        let mut old = self.reports();
        old.retain(|(key, _)| key != &self.key);
        old.retain(|(_, report)| js_sys::Date::parse(&report.last_seen).is_finite());
        old.sort_by(|a, b| {
            js_sys::Date::parse(&b.1.last_seen)
                .total_cmp(&js_sys::Date::parse(&a.1.last_seen))
        });
        let keep: Vec<_> = old
            .iter()
            .take(2)
            .map(|(key, _)| key)
            .collect();
        let Ok(text) = serde_json::to_string(report) else {
            return false;
        };
        if text.len() > crate::report_store::MAX_BYTES
            || storage.set_item(&self.key, &text).is_err()
        {
            return false;
        }
        let mut success = true;
        for key in keys {
            if key != self.key && !keep.contains(&&key) {
                success &= storage.remove_item(&key).is_ok();
            }
        }
        success
    }

    pub fn previous(&self) -> Option<Report> {
        let now = js_sys::Date::now();
        self.reports()
            .into_iter()
            .filter_map(|(_, report)| {
                let score = crate::report_recency::eligible(
                    &report,
                    now,
                    &self.tab,
                    js_sys::Date::parse,
                )?;
                Some((score, report))
            })
            .max_by(|a, b| a.0.total_cmp(&b.0))
            .map(|(_, report)| report)
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
    let store = Store::open();
    let first = store.write(&report);
    let second = store.write(&report);
    Ok(
        serde_json::json!({"key": store.key, "first": first, "second": second})
            .to_string(),
    )
}
```

## <a id="checkpoint-34fb-store"></a>7. Retrieve saved failure evidence through the command line

### 1. `src/browser_report.rs`

**Locate this existing block:**

```rust
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

pub fn download() -> Result<(), JsValue> {
    let json = diagnostic_snapshot()?;
    crate::file_output::download_named(json.as_bytes(), "viewer-diagnostic.json")
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use wasm_bindgen::{JsCast, JsValue};

thread_local! {
    static REPORT: RefCell<Option<Report>> = const { RefCell::new(None) };
    static STORE: RefCell<Option<crate::report_storage::Store>> = const { RefCell::new(None) };
    static PREVIOUS: RefCell<Option<Report>> = const { RefCell::new(None) };
}

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
    let store = crate::report_storage::Store::open();
    let previous = store.previous();
    let report = Report::new(
        store.tab.clone(),
        js_sys::Date::new_0().to_iso_string().into(),
        context()?,
    );
    PREVIOUS.with(|slot| slot.replace(previous));
    STORE.with(|slot| slot.replace(Some(store)));
    REPORT.with(|slot| slot.replace(Some(report)));
    let _ = persist();
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
    })?;
    let _ = persist();
    Ok(())
}

pub fn download() -> Result<(), JsValue> {
    let json = diagnostic_snapshot()?;
    let _ = persist();
    crate::file_output::download_named(json.as_bytes(), "viewer-diagnostic.json")
}

fn persist() -> bool {
    let report = REPORT.with(|slot| slot.borrow().clone());
    let Some(report) = report else {
        return false;
    };
    STORE.with(|slot| {
        slot.borrow()
            .as_ref()
            .is_some_and(|store| store.write(&report))
    })
}

pub fn previous_notice() -> Option<&'static str> {
    PREVIOUS.with(|slot| {
        slot.borrow().as_ref().map(|report| {
            if report.outcome == crate::diagnostic::Outcome::Failed {
                "Previous viewer run failed. Type Report Previous."
            } else {
                "Previous viewer run was interrupted. Type Report Previous."
            }
        })
    })
}

pub fn download_previous() -> Result<(), JsValue> {
    let json = previous_diagnostic_snapshot()?;
    crate::file_output::download_named(
        json.as_bytes(),
        "viewer-diagnostic-previous.json",
    )
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn previous_diagnostic_snapshot() -> Result<String, JsValue> {
    PREVIOUS.with(|slot| {
        let slot = slot.borrow();
        let report = slot.as_ref().ok_or("No previous report")?;
        serde_json::to_string_pretty(report)
            .map_err(|error| JsValue::from_str(&error.to_string()))
    })
}
```

### 2. `src/browser.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                        input.click();
                    }
                    None
                } else if line == "report" {
                    let result = crate::browser_report::download()
                        .map(|()| "Diagnostic report downloaded.".to_owned())
```

Replace it with:

```rust
                        input.click();
                    }
                    None
                } else if line == "report" || line == "report previous" {
                    let output = if line.ends_with(" previous") {
                        crate::browser_report::download_previous()
                    } else {
                        crate::browser_report::download()
                    };
                    let result = output
                        .map(|()| "Diagnostic report downloaded.".to_owned())
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
    if let Some(message) = crate::browser_report::previous_notice() {
        panel.result(message);
    }
    panel.update(None, &canvas)?;
    if resize(
        &window,
```

**Locate this block:**

```rust
            "Report",
            "Example Box",
            "Example Triangle",
            "Select Next",
```

Replace it with:

```rust
            "Report",
            "Report Previous",
            "Example Box",
            "Example Triangle",
            "Select Next",
```

### 3. `src/panel.rs`

**Locate this existing block:**

```rust
    pub fn inspect(&self) -> String {
        serde_json::json!({"controls": self.controls, "command": self.model.command, "history": self.model.history,
            "pointer_owned": self.pointer_owned,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn inspect(&self) -> String {
        serde_json::json!({"controls": self.controls, "command": self.model.command, "history": self.model.history, "status": self.model.status,
            "pointer_owned": self.pointer_owned,
```

## <a id="checkpoint-34fc-retention"></a>8. Preserve unsupported telemetry while pruning

### 1. `src/report_store.rs`

**Locate this existing block:**

```rust
pub const MAX_BYTES: usize = 1024 * 1024;

pub fn decode(text: &str) -> Option<Report> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub const MAX_BYTES: usize = 1024 * 1024;

pub fn retention_time(
    text: &str,
    now: f64,
    parse: impl Fn(&str) -> f64,
) -> Option<f64> {
    if text.len() > MAX_BYTES || !now.is_finite() || now < 0.0 {
        return None;
    }
    let value: serde_json::Value = serde_json::from_str(text).ok()?;
    if value.get("version")?.as_u64()? == 0 {
        return None;
    }
    let seen = value.get("lastSeen")?.as_str()?;
    if seen.is_empty() || seen.chars().count() > 64 {
        return None;
    }
    let time = parse(seen);
    (time.is_finite() && time <= now).then_some(time)
}

pub fn decode(text: &str) -> Option<Report> {
```

### 2. `src/report_storage.rs`

**Locate this existing block:**

```rust
        }
        let mut old = self.reports();
        old.retain(|(key, _)| key != &self.key);
        old.retain(|(_, report)| js_sys::Date::parse(&report.last_seen).is_finite());
        old.sort_by(|a, b| {
            js_sys::Date::parse(&b.1.last_seen)
                .total_cmp(&js_sys::Date::parse(&a.1.last_seen))
        });
        let keep: Vec<_> = old
            .iter()
            .take(2)
            .map(|(key, _)| key)
            .collect();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        }
        let now = js_sys::Date::now();
        let mut old = Vec::new();
        for key in keys.iter().filter(|key| *key != &self.key) {
            let Ok(Some(text)) = storage.get_item(key) else {
                return false;
            };
            if let Some(time) =
                crate::report_store::retention_time(&text, now, js_sys::Date::parse)
            {
                old.push((time, key.clone()));
            }
        }
        old.sort_by(|a, b| b.0.total_cmp(&a.0));
        let keep: Vec<_> = old
            .iter()
            .take(2)
            .map(|(_, key)| key)
            .collect();
```

### 3. `src/report_store_tests.rs`

**Locate this existing block:**

```rust
    assert!(decode(&value.to_string()).is_none());
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert!(decode(&value.to_string()).is_none());
}

#[test]
fn unsupported_telemetry_can_be_retained_without_adoption() {
    let mut value = serde_json::to_value(report()).unwrap();
    value["lastSeen"] = serde_json::json!("seen");
    value["futureTelemetry"] = serde_json::json!({"resources":["keep original"]});
    for version in [1, 2] {
        value["version"] = serde_json::json!(version);
        let text = value.to_string();
        let before = text.clone();
        assert!(decode(&text).is_none());
        assert_eq!(
            crate::report_store::retention_time(&text, 2000.0, |_| 1000.0),
            Some(1000.0)
        );
        assert_eq!(text, before);
    }
}

#[test]
fn retention_headers_reject_invalid_bounds_and_clocks() {
    let text = serde_json::to_string(&report()).unwrap();
    let retain = |text: &str, now: f64, parsed: f64| {
        crate::report_store::retention_time(text, now, |_| parsed)
    };
    for now in [f64::NAN, f64::INFINITY, -1.0] {
        assert!(retain(&text, now, 0.0).is_none());
    }
    for seen in [f64::NAN, f64::INFINITY, 2001.0] {
        assert!(retain(&text, 2000.0, seen).is_none());
    }
    for bad in [
        "{",
        "{}",
        r#"{"version":0,"lastSeen":"seen"}"#,
    ] {
        assert!(retain(bad, 2000.0, 0.0).is_none());
    }
    let mut value = serde_json::to_value(report()).unwrap();
    value["lastSeen"] = serde_json::json!("x".repeat(65));
    assert!(retain(&value.to_string(), 2000.0, 0.0).is_none());
    let padded = text.clone() + &" ".repeat(MAX_BYTES - text.len());
    assert!(retain(&padded, 2000.0, 0.0).is_some());
    assert!(retain(&(padded + " "), 2000.0, 0.0).is_none());
}
```

## Build & Run

![Recorded visual output for Lesson 4.3 (34fc-retention)](../../screenshots/journey/34fc-retention-browser.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 4.3
npm --prefix ../session_tests run course -- save 4.3
```

Continue with [4.4 · Heartbeat suspension and revocable startup](34g-heartbeat.md).
