# 4.5 · Measured phases and conservative GPU recovery

**Goal:** Save device-loss diagnostics, reload once and draw conservatively until the next ordinary reload. · **Steps:** 15 · **Source:** `src/lib.rs,src/selftest/lifecycle.rs,src/engine/gpu/device.rs`

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

## <a id="checkpoint-34gc-messages"></a>1. Bound browser failure messages

**What this code does:** Keep browser failure text readable and small enough for the existing report.

### 1. `src/error_message.rs`

Bound message and source lengths while preserving Unicode and excluding source query strings.

Create the file and type:

```rust
fn detail(message: &str) -> String {
    let text = message.trim();
    // Iterating characters bounds diagnostic text without cutting a UTF-8 character; empty browser messages still receive a useful fallback.
    if text.is_empty() {
        "Unspecified error".into()
    } else {
        text.chars().take(2048).collect()
    }
}

pub fn error(message: &str, filename: &str, line: u32, column: u32) -> String {
    let source: String = filename
        .split(['?', '#'])
        .next()
        .unwrap_or("")
        .chars()
        .take(1024)
        .collect();
    let message = format!("Browser error: {}", detail(message));
    if source.is_empty() {
        message
    } else {
        format!("{message}\nAt {source}:{line}:{column}")
    }
}

pub fn rejection(reason: &str) -> String {
    format!("Unhandled rejection: {}", detail(reason))
}
```

### 2. `src/error_message_tests.rs`

Optional authoring fixture. Check Unicode bounds, source positions, omitted query strings and missing-detail fallbacks.

Create the file and type:

```rust
use crate::error_message::{error, rejection};

#[test]
fn long_unicode_errors_fit_the_report_budget() {
    let message = error(
        &"é😀".repeat(10000),
        &"source".repeat(1000),
        u32::MAX,
        u32::MAX,
    );
    assert!(message.chars().count() < 4096);
    assert!(message.starts_with("Browser error: é😀"));
    assert!(message.ends_with(":4294967295:4294967295"));
    assert!(
        rejection(&"😀".repeat(10000))
            .chars()
            .count()
            < 4096
    );
}

#[test]
fn source_locations_keep_positions_without_queries_or_fragments() {
    assert_eq!(
        error(
            "bad input",
            "https://example.test/app.js?private=value#part",
            42,
            7
        ),
        "Browser error: bad input\nAt https://example.test/app.js:42:7"
    );
    assert_eq!(
        error(
            "bad input",
            "app.js#part?private=value",
            8,
            2
        ),
        "Browser error: bad input\nAt app.js:8:2"
    );
}

#[test]
fn missing_details_have_readable_labels() {
    assert_eq!(
        error("  ", "", 0, 0),
        "Browser error: Unspecified error"
    );
    assert_eq!(
        rejection(""),
        "Unhandled rejection: Unspecified error"
    );
    assert_eq!(
        rejection(" timeout "),
        "Unhandled rejection: timeout"
    );
}
```

### 3. `src/lib.rs`

Register diagnostic message formatting and its native checks.

**Locate this existing block:**

```rust
pub mod editor;
pub mod gesture;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod editor;
pub mod error_message;
#[cfg(test)]
mod error_message_tests;
pub mod gesture;
```

## <a id="checkpoint-34gca-errors"></a>2. Observe uncaught browser failures

**What this code does:** Retain the first uncaught browser failure while later events remain bounded and drawing stays independently owned.

### 1. `src/browser_errors.rs`

Read native error events and safe promise-reason values; avoid serializing objects and catch unavailable Error messages.

Create the file and type:

```rust
use wasm_bindgen::{JsCast, JsValue};

fn reason(value: &JsValue) -> String {
    if let Some(text) = value.as_string() {
        return text;
    }
    if value.is_instance_of::<js_sys::Error>() {
        return js_sys::Reflect::get(value, &"message".into())
            .ok()
            .and_then(|message| message.as_string())
            .unwrap_or_else(|| "Error message unavailable".into());
    }
    if let Some(number) = value.as_f64() {
        return number.to_string();
    }
    if let Some(flag) = value.as_bool() {
        return flag.to_string();
    }
    if value.is_null() {
        return "null".into();
    }
    if value.is_undefined() {
        return "undefined".into();
    }
    "Non-text rejection reason".into()
}

pub fn message(event: &web_sys::Event) -> Option<String> {
    match event.type_().as_str() {
        "error" => {
            let error = event.dyn_ref::<web_sys::ErrorEvent>()?;
            Some(crate::error_message::error(
                &error.message(),
                &error.filename(),
                error.lineno(),
                error.colno(),
            ))
        }
        "unhandledrejection" => {
            let rejected = event.dyn_ref::<web_sys::PromiseRejectionEvent>()?;
            Some(crate::error_message::rejection(&reason(
                &rejected.reason(),
            )))
        }
        _ => None,
    }
}
```

### 2. `Cargo.toml`

Enable the two browser event interfaces without changing dependency versions.

**Locate this existing block:**

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "Location", "Navigator", "Storage", "Performance", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
console_error_panic_hook = "=0.1.7"
```

Edit the existing block to match this code; keep unchanged lines:

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "ErrorEvent", "PromiseRejectionEvent", "Location", "Navigator", "Storage", "Performance", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
console_error_panic_hook = "=0.1.7"
```

### 3. `src/lib.rs`

Register browser failure observation.

**Locate this existing block:**

```rust
#[cfg(target_arch = "wasm32")]
mod browser_report;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(target_arch = "wasm32")]
mod browser_errors;
#[cfg(target_arch = "wasm32")]
mod browser_report;
```

### 4. `src/report_lifecycle.rs`

Record failures through the existing first-failure path; keep event defaults and heartbeat scheduling unchanged. Own both Window failure bindings beside the five existing metadata bindings, including partial-registration cleanup.

**Locate this existing block:**

```rust
                }
                _ => return,
            }
            if let Err(error) = report::start_periodic() {
                let _ = report::observe(
                    "diagnostic",
                    &format!("Heartbeat unavailable: {error:?}"),
                );
            }
        });
    let mut listeners = Listeners::new(callback);
    listeners.listen(window.as_ref(), "pagehide")?;
    listeners.listen(window.as_ref(), "pageshow")?;
    for name in ["visibilitychange", "freeze", "resume"] {
        listeners.listen(document.as_ref(), name)?;
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                }
                "error" | "unhandledrejection" => {
                    if let Some(message) = crate::browser_errors::message(&event) {
                        report::fatal(&message);
                    }
                    return;
                }
                _ => return,
            }
            if let Err(error) = report::start_periodic() {
                let _ = report::observe(
                    "diagnostic",
                    &format!("Heartbeat unavailable: {error:?}"),
                );
            }
        });
    let mut listeners = Listeners::new(callback);
    listeners.listen(window.as_ref(), "pagehide")?;
    listeners.listen(window.as_ref(), "pageshow")?;
    for name in ["visibilitychange", "freeze", "resume"] {
        listeners.listen(document.as_ref(), name)?;
    }
    for name in ["error", "unhandledrejection"] {
        listeners.listen(window.as_ref(), name)?;
    }
```

## <a id="checkpoint-34gd-adapter"></a>3. Retain adapter identity in the report

**What this code does:** Preserve bounded adapter identity even when recent events rotate out of the report.

### 1. `src/adapter_info.rs`

Keep four bounded adapter strings without retaining a GPU object.

Create the file and type:

```rust
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AdapterInfo {
    pub vendor: String,
    pub architecture: String,
    pub device: String,
    pub description: String,
}

impl AdapterInfo {
    pub fn valid(&self) -> bool {
        [
            &self.vendor,
            &self.architecture,
            &self.device,
            &self.description,
        ]
        .into_iter()
        .all(|text| text.chars().count() <= 256)
    }
}
```

### 2. `src/diagnostic.rs`

Retain optional adapter identity separately from the recent-event queue. Start without adapter identity; the browser will supply it after its request. Reject oversized adapter fields when validating a saved report. Validate identity before changing the report.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    pub fn failure(&self) -> Option<&Event> {
        self.failure.as_ref()
    }
```

Replace it with:

```rust
    pub fn failure(&self) -> Option<&Event> {
        self.failure.as_ref()
    }

    pub fn set_adapter(
        &mut self,
        info: crate::adapter_info::AdapterInfo,
    ) -> Result<(), &'static str> {
        if !info.valid() {
            return Err("Invalid adapter identity");
        }
        self.adapter = Some(info);
        Ok(())
    }
```

**Locate this block:**

```rust
            && text(&self.context.browser, 4096)
            && self.context.device_pixel_ratio.is_finite()
            && self.context.device_pixel_ratio > 0.0
            && self.events.len() <= 24
            && self.events.iter().all(event)
            && self
```

Replace it with:

```rust
            && text(&self.context.browser, 4096)
            && self.context.device_pixel_ratio.is_finite()
            && self.context.device_pixel_ratio > 0.0
            && self
                .adapter
                .as_ref()
                .is_none_or(|info| info.valid())
            && self.events.len() <= 24
            && self.events.iter().all(event)
            && self
```

**Locate this block:**

```rust
            context,
            events: Default::default(),
            failure: None,
        }
    }
```

Replace it with:

```rust
            context,
            events: Default::default(),
            failure: None,
            adapter: None,
        }
    }
```

**Locate this block:**

```rust
    failure: Option<Event>,
}

impl Report {
```

Replace it with:

```rust
    failure: Option<Event>,
    // An optional field lets old reports decode unchanged; keeping identity outside the event queue preserves it when later events rotate.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none"
    )]
    adapter: Option<crate::adapter_info::AdapterInfo>,
}

impl Report {
```

### 3. `src/report_store.rs`

Admit the optional adapter field through the existing strict decoder.

**Locate this existing block:**

```rust
        "failure",
    ];
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        "failure",
        "adapter",
    ];
```

### 4. `src/adapter_info_tests.rs`

Copy the checks for retained identity, legacy decoding, strict fields, Unicode bounds and atomic refusal.

Create the file and type:

```rust
use crate::{adapter_info::AdapterInfo, diagnostic_shape_tests::report};

fn identity() -> AdapterInfo {
    AdapterInfo {
        vendor: "vendor".into(),
        architecture: "family".into(),
        device: "model".into(),
        description: "GPU".into(),
    }
}

#[test]
fn identity_survives_event_rotation_and_failure() {
    let mut report = report();
    let info = identity();
    report.set_adapter(info.clone()).unwrap();
    report
        .record(
            "first".into(),
            1.0,
            "fatal",
            "original failure",
        )
        .unwrap();
    let first = report.failure().unwrap().clone();
    for i in 0..30 {
        report
            .record(
                "later".into(),
                i as f64 + 2.0,
                "phase",
                "later activity",
            )
            .unwrap();
    }
    let value = serde_json::to_value(&report).unwrap();
    assert_eq!(value["events"].as_array().unwrap().len(), 24);
    assert_eq!(
        value["adapter"],
        serde_json::to_value(info).unwrap()
    );
    assert_eq!(report.failure(), Some(&first));
}

#[test]
fn old_reports_and_new_identity_round_trip() {
    let mut report = report();
    let old = serde_json::to_string(&report).unwrap();
    assert!(
        serde_json::to_value(&report)
            .unwrap()
            .get("adapter")
            .is_none()
    );
    assert_eq!(
        crate::report_store::decode(&old),
        Some(report.clone())
    );
    report.set_adapter(identity()).unwrap();
    assert_eq!(
        crate::report_store::decode(&serde_json::to_string(&report).unwrap()),
        Some(report.clone())
    );
    let mut value = serde_json::to_value(report).unwrap();
    value["adapter"]["unknown"] = 1.into();
    assert!(crate::report_store::decode(&value.to_string()).is_none());
}

#[test]
fn invalid_identity_is_atomic_and_empty_fields_are_valid() {
    let mut report = report();
    let before = report.clone();
    for field in [
        "vendor",
        "architecture",
        "device",
        "description",
    ] {
        let mut value = serde_json::to_value(identity()).unwrap();
        value[field] = "😀".repeat(257).into();
        let invalid: AdapterInfo = serde_json::from_value(value).unwrap();
        assert!(report.set_adapter(invalid).is_err());
        assert_eq!(report, before);
    }
    let mut info = identity();
    info.vendor = "😀".repeat(256);
    info.architecture.clear();
    info.device.clear();
    info.description.clear();
    assert!(info.valid());
    report.set_adapter(info).unwrap();
    assert!(report.valid());
}
```

### 5. `src/lib.rs`

Register adapter identity and its native checks.

**Locate this existing block:**

```rust
pub mod background;
pub mod bounds;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod adapter_info;
#[cfg(test)]
mod adapter_info_tests;
pub mod background;
pub mod bounds;
```

## <a id="checkpoint-34gda-browser"></a>4. Read the viewer’s adapter identity

**What this code does:** Retain vendor and architecture from the same adapter request used for drawing.

### 1. `src/adapter_info.js`

Observe the same native adapter request; retain only strings and restore the original property when released.

Create the file and type:

```javascript
function text(value) {
    if (typeof value !== 'string') return '';
    let result = '', count = 0;
    for (const letter of value) { if (count++ === 256) break; result += letter; }
    return result;
}

export function begin() {
    const gpu = navigator.gpu, original = gpu.requestAdapter;
    const previous = Object.getOwnPropertyDescriptor(gpu, 'requestAdapter');
    let active = true, value = null;
    async function wrapped(...args) {
        const adapter = await Reflect.apply(original, this, args);
        if (active && adapter) {
            try {
                const info = adapter.info;
                value = JSON.stringify({vendor: text(info.vendor), architecture: text(info.architecture),
                    device: text(info.device), description: text(info.description)});
            } catch { value = null; }
        }
        return adapter;
    }
    Object.defineProperty(gpu, 'requestAdapter', {value: wrapped, writable: true, configurable: true});
    return {read: () => value, stop() {
        active = false; value = null;
        if (Object.getOwnPropertyDescriptor(gpu, 'requestAdapter')?.value !== wrapped) return;
        if (previous) Object.defineProperty(gpu, 'requestAdapter', previous);
        else delete gpu.requestAdapter;
    }};
}
```

### 2. `src/browser_adapter.rs`

Own the JavaScript observation across one await and release it through Drop.

Create the file and type:

```rust
use wasm_bindgen::prelude::*;

// An extern block names JavaScript functions; wasm-bindgen imports the local module and turns JavaScript exceptions into Result through catch.
#[wasm_bindgen(module = "/src/adapter_info.js")]
extern "C" {
    type Capture;
    #[wasm_bindgen(catch)]
    fn begin() -> Result<Capture, JsValue>;
    #[wasm_bindgen(catch, method)]
    fn read(this: &Capture) -> Result<Option<String>, JsValue>;
    #[wasm_bindgen(catch, method)]
    fn stop(this: &Capture) -> Result<(), JsValue>;
}

pub struct Probe(Capture);
impl Probe {
    pub fn new() -> Result<Self, JsValue> {
        begin().map(Self)
    }
    pub fn info(&self) -> Option<crate::adapter_info::AdapterInfo> {
        serde_json::from_str(&self.0.read().ok().flatten()?).ok()
    }
}
impl Drop for Probe {
    fn drop(&mut self) {
        let _ = self.0.stop();
    }
}
```

### 3. `src/browser_report.rs`

Persist validated adapter identity without changing the run outcome or event history.

**Locate this existing block:**

```rust
}

pub fn fatal(message: &str) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub fn adapter(info: crate::adapter_info::AdapterInfo) -> Result<(), JsValue> {
    REPORT.with(|slot| {
        slot.borrow_mut()
            .as_mut()
            .ok_or("No report")?
            .set_adapter(info)
            .map_err(JsValue::from_str)
    })?;
    let _ = persist();
    Ok(())
}

pub fn fatal(message: &str) {
```

### 4. `src/browser.rs`

Install observation immediately before the viewer requests its adapter. Read identity only while startup remains authorized; detach observation before requesting a device.

**Locate this existing block:**

```rust
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        })
        .await;
    if !startup.permits() {
        return Ok(());
    }
    let adapter = adapter.map_err(|error| JsValue::from_str(&error.to_string()))?;
    let device = adapter
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let probe = crate::browser_adapter::Probe::new().ok();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        })
        .await;
    if !startup.permits() {
        return Ok(());
    }
    let adapter = adapter.map_err(|error| JsValue::from_str(&error.to_string()))?;
    if let Some(info) = probe.as_ref().and_then(|probe| probe.info()) {
        let _ = crate::browser_report::adapter(info);
    } else {
        let _ = crate::browser_report::observe(
            "diagnostic",
            "Adapter identity unavailable",
        );
    }
    drop(probe);
    let device = adapter
```

### 5. `src/lib.rs`

Register the browser adapter bridge.

**Locate this existing block:**

```rust
#[cfg(target_arch = "wasm32")]
mod browser_errors;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(target_arch = "wasm32")]
mod browser_adapter;
#[cfg(target_arch = "wasm32")]
mod browser_errors;
```

## <a id="checkpoint-34gdb-phases"></a>5. Retain measured loading phases

**What this code does:** Keep measured load operations after their recent event messages rotate away.

### 1. `src/load_phase.rs`

Describe one completed load operation with duration, bytes, sanitized source and completion time.

Create the file and type:

```rust
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Phase {
    pub name: String,
    pub duration_ms: f64,
    pub bytes: u64,
    pub source: String,
    pub elapsed_ms: f64,
}

impl Phase {
    pub fn valid(&self) -> bool {
        !self.name.is_empty()
            && self.name.chars().count() <= 64
            && self.source.chars().count() <= 1024
            && !self.source.contains(['?', '#'])
            && self.duration_ms.is_finite()
            && self.duration_ms >= 0.0
            && self.elapsed_ms.is_finite()
            && self.elapsed_ms >= self.duration_ms
    }
}

pub fn zero(value: &u64) -> bool {
    *value == 0
}
```

### 2. `src/diagnostic.rs`

Retain phases outside the recent events; old reports omit both optional fields. Start with no measured phases or discarded entries. Validate retained phases and the existing one MiB report limit. Record validated phases and count entries discarded by either retention limit. Keep phase retention within budget when later errors grow the event window. Account for adapter identity when retaining phase entries.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            self.events.pop_front();
        }
        self.events.push_back(event);
        Ok(())
```

Replace it with:

```rust
            self.events.pop_front();
        }
        self.events.push_back(event);
        self.trim_phases();
        Ok(())
```

**Locate this block:**

```rust
            return Err("Invalid adapter identity");
        }
        self.adapter = Some(info);
        Ok(())
    }

    pub fn record(
        &mut self,
        time: String,
```

Replace it with:

```rust
            return Err("Invalid adapter identity");
        }
        self.adapter = Some(info);
        self.trim_phases();
        Ok(())
    }

    fn fits(&self) -> bool {
        serde_json::to_vec_pretty(self)
            .is_ok_and(|bytes| bytes.len() <= crate::report_store::MAX_BYTES)
    }

    fn trim_phases(&mut self) {
        // JSON escapes can expand one character into six bytes; trim the oldest measurements against the encoded budget, not character counts.
        while self.phases.len() > 256 || (!self.phases.is_empty() && !self.fits()) {
            self.phases.pop_front();
            self.phases_dropped = self.phases_dropped.saturating_add(1);
        }
    }

    pub fn phase(
        &mut self,
        time: String,
        phase: crate::load_phase::Phase,
    ) -> Result<(), &'static str> {
        if time.is_empty() || time.chars().count() > 64 || !phase.valid() {
            return Err("Invalid load phase");
        }
        let message =
            serde_json::to_string(&phase).map_err(|_| "Cannot encode phase")?;
        self.record(time, phase.elapsed_ms, "phase", &message)?;
        self.phases.push_back(phase);
        self.trim_phases();
        Ok(())
    }

    pub fn record(
        &mut self,
        time: String,
```

**Locate this block:**

```rust
                .adapter
                .as_ref()
                .is_none_or(|info| info.valid())
            && self.events.len() <= 24
            && self.events.iter().all(event)
            && self
```

Replace it with:

```rust
                .adapter
                .as_ref()
                .is_none_or(|info| info.valid())
            && self.phases.len() <= 256
            && self.phases.iter().all(|phase| phase.valid())
            && self.fits()
            && self.events.len() <= 24
            && self.events.iter().all(event)
            && self
```

**Locate this block:**

```rust
            events: Default::default(),
            failure: None,
            adapter: None,
        }
    }
```

Replace it with:

```rust
            events: Default::default(),
            failure: None,
            adapter: None,
            phases: Default::default(),
            phases_dropped: 0,
        }
    }
```

**Locate this block:**

```rust
    adapter: Option<crate::adapter_info::AdapterInfo>,
}

impl Report {
```

Replace it with:

```rust
    adapter: Option<crate::adapter_info::AdapterInfo>,
    #[serde(
        default,
        skip_serializing_if = "std::collections::VecDeque::is_empty"
    )]
    phases: std::collections::VecDeque<crate::load_phase::Phase>,
    #[serde(
        default,
        skip_serializing_if = "crate::load_phase::zero"
    )]
    phases_dropped: u64,
}

impl Report {
```

### 3. `src/report_store.rs`

Admit only the two declared phase fields through saved-report decoding.

**Locate this existing block:**

```rust
        "adapter",
    ];
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        "adapter",
        "phases",
        "phasesDropped",
    ];
```

### 4. `src/load_phase_tests.rs`

Copy checks for strict decoding, Unicode limits, event rotation and escaped JSON byte retention.

Create the file and type:

```rust
use crate::{diagnostic_shape_tests::report, load_phase::Phase};

fn phase(i: usize) -> Phase {
    Phase {
        name: format!("decode {i}"),
        duration_ms: 2.0,
        bytes: 100,
        source: "scene.pb".into(),
        elapsed_ms: i as f64 + 2.0,
    }
}

#[test]
fn retention_survives_event_rotation_and_counts_evictions() {
    let mut report = report();
    for i in 0..270 {
        report
            .phase("later".into(), phase(i))
            .unwrap();
    }
    let value = serde_json::to_value(&report).unwrap();
    assert_eq!(
        value["phases"].as_array().unwrap().len(),
        256
    );
    assert_eq!(value["phases"][0]["name"], "decode 14");
    assert_eq!(value["phasesDropped"], 14);
    for i in 0..30 {
        report
            .record(
                "later".into(),
                i as f64,
                "activity",
                "camera",
            )
            .unwrap();
    }
    assert_eq!(
        serde_json::to_value(&report).unwrap()["phases"],
        value["phases"]
    );
    assert!(report.valid());
}

#[test]
fn bad_phases_are_refused_without_mutation() {
    let mut report = report();
    let before = report.clone();
    for change in 0..9 {
        let mut invalid = phase(0);
        match change {
            0 => invalid.name.clear(),
            1 => invalid.name = "x".repeat(65),
            2 => invalid.source = "😀".repeat(1025),
            3 => invalid.source = "file?secret".into(),
            4 => invalid.source = "file#secret".into(),
            5 => invalid.duration_ms = -1.0,
            6 => invalid.duration_ms = f64::NAN,
            7 => invalid.elapsed_ms = f64::INFINITY,
            _ => invalid.elapsed_ms = 1.0,
        }
        assert!(
            report
                .phase("later".into(), invalid)
                .is_err()
        );
        assert_eq!(report, before);
    }
    for time in [String::new(), "x".repeat(65)] {
        assert!(report.phase(time, phase(0)).is_err());
        assert_eq!(report, before);
    }
    let mut unicode = phase(0);
    unicode.source = "😀".repeat(1024);
    report
        .phase("later".into(), unicode)
        .unwrap();
}

#[test]
fn old_reports_decode_and_nested_shape_is_strict() {
    let mut report = report();
    let old = serde_json::to_string(&report).unwrap();
    assert!(
        serde_json::to_value(&report)
            .unwrap()
            .get("phases")
            .is_none()
    );
    assert_eq!(
        crate::report_store::decode(&old),
        Some(report.clone())
    );
    report
        .phase("later".into(), phase(0))
        .unwrap();
    let text = serde_json::to_string_pretty(&report).unwrap();
    assert_eq!(
        crate::report_store::decode(&text),
        Some(report.clone())
    );
    let mut value = serde_json::to_value(report).unwrap();
    value["phases"][0]["unknown"] = true.into();
    assert!(crate::report_store::decode(&value.to_string()).is_none());
    value["phases"][0]
        .as_object_mut()
        .unwrap()
        .remove("unknown");
    value["phases"] = serde_json::json!(vec![phase(0); 257]);
    assert!(crate::report_store::decode(&value.to_string()).is_none());
}

#[test]
fn encoded_budget_preserves_first_failure_and_recent_events() {
    let mut report = report();
    for i in 0..256 {
        let mut measured = phase(i);
        measured.name = "\u{0001}".repeat(64);
        measured.source = "\u{0001}".repeat(1024);
        report
            .phase("later".into(), measured)
            .unwrap();
    }
    let before = serde_json::to_value(&report).unwrap()["phasesDropped"]
        .as_u64()
        .unwrap();
    assert!(before > 0);
    for i in 0..30 {
        report
            .record(
                "later".into(),
                i as f64,
                if i == 0 {
                    "fatal"
                } else {
                    "activity"
                },
                &"\u{0001}".repeat(4096),
            )
            .unwrap();
    }
    let failure = report.failure().unwrap().clone();
    let mut context = report.context.clone();
    context.page = "\u{0001}".repeat(4096);
    context.browser = "\u{0001}".repeat(4096);
    report.refresh_context(context);
    report.heartbeat("x".repeat(64)).unwrap();
    let text = serde_json::to_string_pretty(&report).unwrap();
    assert!(text.len() <= crate::report_store::MAX_BYTES);
    assert!(report.valid());
    assert_eq!(
        crate::report_store::decode(&text),
        Some(report.clone())
    );
    let value = serde_json::to_value(&report).unwrap();
    assert_eq!(value["events"].as_array().unwrap().len(), 24);
    assert!(value["phasesDropped"].as_u64().unwrap() > before);
    assert_eq!(report.failure(), Some(&failure));
}
```

### 5. `src/lib.rs`

Register load phase data and its native checks.

**Locate this existing block:**

```rust
pub mod history;
pub mod memory;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod history;
pub mod load_phase;
#[cfg(test)]
mod load_phase_tests;
pub mod memory;
```

### 6. `src/diagnostic.rs`

Recheck phase retention when current browser context changes. Retain the encoded budget after heartbeat timestamp changes.

**Locate this existing block:**

```rust
        self.last_seen = time;
        Ok(())
    }

    pub fn close(&mut self, time: String) -> Result<(), &'static str> {
        self.heartbeat(time)?;
        if self.failure.is_none() {
            self.outcome = Outcome::Closed;
        }
        Ok(())
    }

    pub fn failure(&self) -> Option<&Event> {
        self.failure.as_ref()
    }

    pub fn set_adapter(
        &mut self,
        info: crate::adapter_info::AdapterInfo,
    ) -> Result<(), &'static str> {
        if !info.valid() {
            return Err("Invalid adapter identity");
        }
        self.adapter = Some(info);
        self.trim_phases();
        Ok(())
    }

    fn fits(&self) -> bool {
        serde_json::to_vec_pretty(self)
            .is_ok_and(|bytes| bytes.len() <= crate::report_store::MAX_BYTES)
    }

    fn trim_phases(&mut self) {
        // JSON escapes can expand one character into six bytes; trim the oldest measurements against the encoded budget, not character counts.
        while self.phases.len() > 256 || (!self.phases.is_empty() && !self.fits()) {
            self.phases.pop_front();
            self.phases_dropped = self.phases_dropped.saturating_add(1);
        }
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        self.last_seen = time;
        self.trim_phases();
        Ok(())
    }

    pub fn close(&mut self, time: String) -> Result<(), &'static str> {
        self.heartbeat(time)?;
        if self.failure.is_none() {
            self.outcome = Outcome::Closed;
        }
        Ok(())
    }

    pub fn failure(&self) -> Option<&Event> {
        self.failure.as_ref()
    }

    pub fn set_adapter(
        &mut self,
        info: crate::adapter_info::AdapterInfo,
    ) -> Result<(), &'static str> {
        if !info.valid() {
            return Err("Invalid adapter identity");
        }
        self.adapter = Some(info);
        self.trim_phases();
        Ok(())
    }

    fn fits(&self) -> bool {
        serde_json::to_vec_pretty(self)
            .is_ok_and(|bytes| bytes.len() <= crate::report_store::MAX_BYTES)
    }

    fn trim_phases(&mut self) {
        // JSON escapes can expand one character into six bytes; trim the oldest measurements against the encoded budget, not character counts.
        while self.phases.len() > 256 || (!self.phases.is_empty() && !self.fits()) {
            self.phases.pop_front();
            self.phases_dropped = self.phases_dropped.saturating_add(1);
        }
    }

    pub fn refresh_context(&mut self, context: Context) {
        self.context = context;
        self.trim_phases();
    }
```

## <a id="checkpoint-34gdc-startup"></a>6. Measure GPU startup and the first frame

**What this code does:** Download actual adapter, device, renderer and first completed frame timings.

### 1. `src/browser_phase.rs`

Measure completed operations with the browser clock and strip private URL suffixes.

Create the file and type:

```rust
pub fn now() -> f64 {
    web_sys::window()
        .and_then(|window| window.performance())
        .map_or(0.0, |clock| clock.now())
}

pub fn finish(name: &str, started: f64, bytes: u64, source: &str) {
    let elapsed_ms = now();
    let phase = crate::load_phase::Phase {
        name: name.chars().take(64).collect(),
        duration_ms: elapsed_ms - started,
        bytes,
        source: source
            .split(['?', '#'])
            .next()
            .unwrap_or("")
            .chars()
            .take(1024)
            .collect(),
        elapsed_ms,
    };
    let _ = crate::browser_report::phase(phase);
}
```

### 2. `src/browser_report.rs`

Persist measured phases without changing the run outcome.

**Locate this existing block:**

```rust
}

pub fn adapter(info: crate::adapter_info::AdapterInfo) -> Result<(), JsValue> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub fn phase(phase: crate::load_phase::Phase) -> Result<(), JsValue> {
    REPORT.with(|slot| {
        slot.borrow_mut()
            .as_mut()
            .ok_or("No report")?
            .phase(
                js_sys::Date::new_0().to_iso_string().into(),
                phase,
            )
            .map_err(JsValue::from_str)
    })?;
    let _ = persist();
    Ok(())
}

pub fn adapter(info: crate::adapter_info::AdapterInfo) -> Result<(), JsValue> {
```

### 3. `src/browser_runtime.rs`

Let deferred frame completion verify ownership of the current GPU runtime.

**Locate this existing block:**

```rust
}

pub fn stop_if(fault: &crate::gpu_fault::Fault) -> bool {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub fn owns(fault: &crate::gpu_fault::Fault) -> bool {
    ACTIVE.with(|slot| {
        slot.borrow()
            .as_ref()
            .is_some_and(|runtime| runtime.fault.same_device(fault))
    })
}

pub fn stop_if(fault: &crate::gpu_fault::Fault) -> bool {
```

### 4. `src/browser.rs`

Start adapter timing before the actual request. Retain adapter timing only after the existing startup guard permits adoption. Time device creation separately from adapter selection. Retain active device timing, including a rejected request before its fatal report. Start timing construction of the initial renderer and its pipeline. Record CPU setup duration; GPU compilation latency remains part of first-frame completion. Start first-frame timing before encoding and submitting the real drawing. Mark readiness after submitted work completes, only while this GPU runtime still owns the page.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            &editor.camera.uniform(),
            &mut panel,
        )?;
        crate::browser_report::observe("milestone", "geometry on screen")?;
    }
```

Replace it with:

```rust
            &editor.camera.uniform(),
            &mut panel,
        )?;
        let completing = fault.clone();
        // Queue completion runs after submitted GPU work; checking the current device prevents a late result from reviving an exited runtime.
        renderer
            .queue
            .on_submitted_work_done(move || {
                if crate::browser_runtime::owns(&completing)
                    && completing.message().is_none()
                {
                    crate::browser_phase::finish(
                        "first frame complete",
                        frame_started,
                        0,
                        "GPU",
                    );
                    let _ = crate::browser_report::observe(
                        "milestone",
                        "geometry on screen",
                    );
                }
            });
    }
```

**Locate this block:**

```rust
        &mut editor,
    ) {
        panel.update(None, &canvas)?;
        present(
            &surface,
            &renderer,
```

Replace it with:

```rust
        &mut editor,
    ) {
        panel.update(None, &canvas)?;
        let frame_started = crate::browser_phase::now();
        present(
            &surface,
            &renderer,
```

**Locate this block:**

```rust
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
```

Replace it with:

```rust
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let mut editor = Editor::default();
    let pipeline_started = crate::browser_phase::now();
    let mut renderer = Renderer::new(
        device,
        queue,
        config.format.add_srgb_suffix(),
        &editor.scene,
    );
    crate::browser_phase::finish("renderer setup", pipeline_started, 0, "GPU");
    renderer.resize(Viewport { width, height });
    inspect(&editor, &renderer)?;
    let mut panel = crate::panel::Panel::new(
```

**Locate this block:**

```rust
        }
        return Ok(());
    }
    let (device, queue) =
        device.map_err(|error| JsValue::from_str(&error.to_string()))?;
    let fault = crate::gpu_fault::Fault::default();
```

Replace it with:

```rust
        }
        return Ok(());
    }
    crate::browser_phase::finish("device request", device_started, 0, "GPU");
    let (device, queue) =
        device.map_err(|error| JsValue::from_str(&error.to_string()))?;
    let fault = crate::gpu_fault::Fault::default();
```

**Locate this block:**

```rust
        );
    }
    drop(probe);
    let device = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await;
```

Replace it with:

```rust
        );
    }
    drop(probe);
    let device_started = crate::browser_phase::now();
    let device = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await;
```

**Locate this block:**

```rust
    if !startup.permits() {
        return Ok(());
    }
    let adapter = adapter.map_err(|error| JsValue::from_str(&error.to_string()))?;
    if let Some(info) = probe.as_ref().and_then(|probe| probe.info()) {
        let _ = crate::browser_report::adapter(info);
```

Replace it with:

```rust
    if !startup.permits() {
        return Ok(());
    }
    crate::browser_phase::finish("adapter request", adapter_started, 0, "GPU");
    let adapter = adapter.map_err(|error| JsValue::from_str(&error.to_string()))?;
    if let Some(info) = probe.as_ref().and_then(|probe| probe.info()) {
        let _ = crate::browser_report::adapter(info);
```

**Locate this block:**

```rust
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let probe = crate::browser_adapter::Probe::new().ok();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
```

Replace it with:

```rust
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let adapter_started = crate::browser_phase::now();
    let probe = crate::browser_adapter::Probe::new().ok();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
```

### 5. `src/lib.rs`

Register browser phase timing.

**Locate this existing block:**

```rust
#[cfg(target_arch = "wasm32")]
mod browser_report;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(target_arch = "wasm32")]
mod browser_phase;
#[cfg(target_arch = "wasm32")]
mod browser_report;
```

### 6. `src/browser_report.rs`

Keep observed context and phase retention within the saved report budget. Recheck the byte budget when a heartbeat updates browser dimensions. Keep final context and retained measurements within the same limit. Apply the report budget before downloading fresh context.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    REPORT.with(|slot| {
        let mut slot = slot.borrow_mut();
        let report = slot.as_mut().ok_or("No report")?;
        report.context = context;
        serde_json::to_string_pretty(report)
```

Replace it with:

```rust
    REPORT.with(|slot| {
        let mut slot = slot.borrow_mut();
        let report = slot.as_mut().ok_or("No report")?;
        report.refresh_context(context);
        serde_json::to_string_pretty(report)
```

**Locate this block:**

```rust
        report
            .close(time)
            .map_err(JsValue::from_str)?;
        report.context = context;
        Ok::<_, JsValue>(())
    })?;
    let _ = persist();
```

Replace it with:

```rust
        report
            .close(time)
            .map_err(JsValue::from_str)?;
        report.refresh_context(context);
        Ok::<_, JsValue>(())
    })?;
    let _ = persist();
```

**Locate this block:**

```rust
        report
            .heartbeat(time)
            .map_err(JsValue::from_str)?;
        report.context = context;
        Ok::<_, JsValue>(())
    })?;
    let _ = persist();
```

Replace it with:

```rust
        report
            .heartbeat(time)
            .map_err(JsValue::from_str)?;
        report.refresh_context(context);
        Ok::<_, JsValue>(())
    })?;
    let _ = persist();
```

**Locate this block:**

```rust
        let report = slot.as_mut().ok_or("No report")?;
        report.context = context;
        report
            .record(
                js_sys::Date::new_0().to_iso_string().into(),
```

Replace it with:

```rust
        let report = slot.as_mut().ok_or("No report")?;
        report.refresh_context(context);
        report
            .record(
                js_sys::Date::new_0().to_iso_string().into(),
```

## <a id="checkpoint-34gdd-read"></a>7. Measure the selected file read

**What this code does:** Retain the duration and byte count of the selected file’s actual read.

### 1. `src/file_input.rs`

Measure an actual completed read only after its ticket is accepted.

**Locate this existing block:**

```rust
        }
        let result = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await;
        let accepted = request.borrow_mut().finish(id);
        if !accepted {
            return;
        }
        let result = result.and_then(|buffer| deliver(buffer, mode, file, &delivery));
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        }
        let started = crate::browser_phase::now();
        let result = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await;
        let accepted = request.borrow_mut().finish(id);
        if !accepted {
            return;
        }
        let bytes = result.as_ref().map_or(0, |buffer| {
            js_sys::Uint8Array::new(buffer).length() as u64
        });
        crate::browser_phase::finish(
            if result.is_ok() {
                "file read"
            } else {
                "file read failed"
            },
            started,
            bytes,
            &file.name(),
        );

        let result = result.and_then(|buffer| deliver(buffer, mode, file, &delivery));
```

## <a id="checkpoint-34gde-measure"></a>8. Measure a completed CPU operation

**What this code does:** Measure CPU work without changing its success or failure.

### 1. `src/load_measure.rs`

Keep timing metadata separate from the operation’s owned data and result.

Create the file and type:

```rust
pub struct Completed<'a> {
    pub name: &'a str,
    pub duration_ms: f64,
    pub elapsed_ms: f64,
    pub bytes: u64,
    pub succeeded: bool,
}

pub fn run<T>(
    name: &str,
    bytes: u64,
    clock: &mut impl FnMut() -> f64,
    observe: &mut impl FnMut(Completed<'_>),
    operation: impl FnOnce() -> Result<T, &'static str>,
) -> Result<T, &'static str> {
    let started = clock();
    let result = operation();
    let elapsed_ms = clock();
    let duration_ms = elapsed_ms - started;
    if started.is_finite()
        && started >= 0.0
        && elapsed_ms.is_finite()
        && duration_ms >= 0.0
    {
        observe(Completed {
            name,
            duration_ms,
            elapsed_ms,
            bytes,
            succeeded: result.is_ok(),
        });
    }
    result
}

pub fn now() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        crate::browser_phase::now()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        static ORIGIN: std::sync::OnceLock<std::time::Instant> =
            std::sync::OnceLock::new();
        ORIGIN
            .get_or_init(std::time::Instant::now)
            .elapsed()
            .as_secs_f64()
            * 1000.0
    }
}
```

### 2. `src/lib.rs`

Register the portable clock boundary and its tests.

**Locate this existing block:**

```rust
pub mod history;
pub mod load_phase;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod history;
pub mod load_measure;
#[cfg(test)]
mod load_measure_tests;
pub mod load_phase;
```

### 3. `src/load_measure_tests.rs`

Copy deterministic checks for the clock boundary.

Create the file and type:

```rust
use crate::load_measure::{now, run};

#[test]
fn completed_work_keeps_its_result_and_exact_clock_boundaries() {
    for result in [Ok(42), Err("rejected")] {
        let mut samples = [12.5, 17.0].into_iter();
        let mut observed = Vec::new();
        let actual = run(
            "decode",
            512,
            &mut || samples.next().unwrap(),
            &mut |phase| {
                observed.push((
                    phase.name.to_owned(),
                    phase.duration_ms,
                    phase.elapsed_ms,
                    phase.bytes,
                    phase.succeeded,
                ));
            },
            || result,
        );
        assert_eq!(actual, result);
        assert_eq!(
            observed,
            vec![(
                "decode".into(),
                4.5,
                17.0,
                512,
                result.is_ok()
            )]
        );
        assert!(samples.next().is_none());
    }
}

#[test]
fn invalid_clocks_cannot_change_or_retain_an_operation() {
    for samples in [
        [2.0, 1.0],
        [f64::NAN, 3.0],
        [0.0, f64::INFINITY],
        [-1.0, 2.0],
    ] {
        let mut samples = samples.into_iter();
        let mut calls = 0;
        let result = run(
            "validation",
            8,
            &mut || samples.next().unwrap(),
            &mut |_| calls += 1,
            || Err::<(), _>("invalid geometry"),
        );
        assert_eq!(result, Err("invalid geometry"));
        assert_eq!(calls, 0);
    }
}

#[test]
fn native_clock_is_finite_and_monotonic() {
    let first = now();
    let second = now();
    assert!(first.is_finite() && first >= 0.0 && second >= first);
}
```

## <a id="checkpoint-34gdea-import"></a>9. Measure each document preparation phase

**What this code does:** Record decode, validation, kernel construction and display preparation at their actual boundaries.

### 1. `src/document.rs`

Measure real preparation before committing any document state.

**Locate this existing block:**

```rust
) -> Result<Loaded, &'static str> {
    let message = decode(bytes)?;
    let mut origin = crate::origin::Origin::new(&message, bytes);
    origin.location = location;
    let origin = Rc::new(origin);
    let document =
        Rc::new(Session::from_proto(message).map_err(|_| "Cannot construct session")?);
    let mut meshes = Vec::new();
    for mesh in document.objects.meshes.iter() {
        let mut prepared = PreparedMesh::from_shared(Rc::clone(mesh))?;
        prepared.model = document
            .xforms
            .get(mesh.guid())
            .cloned()
            .unwrap_or_else(session_rust::Xform::identity);
        prepared.source = Some(Source {
            document: Rc::clone(&document),
            guid: mesh.guid().to_owned(),
            origin: Rc::clone(&origin),
        });
        meshes.push(prepared);
    }
    Ok(Loaded { document, meshes })
}

fn decode(bytes: &[u8]) -> Result<proto::Session, &'static str> {
    if bytes.len() > MAX_BYTES {
        return Err("This checkpoint accepts files up to 4 MiB");
    }
    let message =
        proto::Session::decode(bytes).map_err(|_| "Invalid session protobuf")?;
    validate(&message)?;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
) -> Result<Loaded, &'static str> {
    load_observed(
        bytes,
        location,
        &mut crate::load_measure::now,
        &mut |phase| {
            #[cfg(target_arch = "wasm32")]
            crate::browser_phase::completed(phase, "document");
            #[cfg(not(target_arch = "wasm32"))]
            let _ = phase;
        },
    )
}

pub fn load_observed(
    bytes: &[u8],
    location: Option<Rc<crate::reload_url::ReloadUrl>>,
    clock: &mut impl FnMut() -> f64,
    observe: &mut impl FnMut(crate::load_measure::Completed<'_>),
) -> Result<Loaded, &'static str> {
    use crate::load_measure::run;
    let size = bytes.len() as u64;
    let message = run("decode", size, clock, observe, || {
        decode_raw(bytes)
    })?;
    run("validation", size, clock, observe, || {
        validate(&message)
    })?;
    let mut origin = crate::origin::Origin::new(&message, bytes);
    origin.location = location;
    let origin = Rc::new(origin);
    let document = run("kernel", size, clock, observe, || {
        Session::from_proto(message)
            .map(Rc::new)
            .map_err(|_| "Cannot construct session")
    })?;
    let meshes = run("display walk", size, clock, observe, || {
        let mut meshes = Vec::new();
        for mesh in document.objects.meshes.iter() {
            let mut prepared = PreparedMesh::from_shared(Rc::clone(mesh))?;
            prepared.model = document
                .xforms
                .get(mesh.guid())
                .cloned()
                .unwrap_or_else(session_rust::Xform::identity);
            prepared.source = Some(Source {
                document: Rc::clone(&document),
                guid: mesh.guid().to_owned(),
                origin: Rc::clone(&origin),
            });
            meshes.push(prepared);
        }
        Ok(meshes)
    })?;
    Ok(Loaded { document, meshes })
}

fn decode_raw(bytes: &[u8]) -> Result<proto::Session, &'static str> {
    if bytes.len() > MAX_BYTES {
        return Err("This checkpoint accepts files up to 4 MiB");
    }
    proto::Session::decode(bytes).map_err(|_| "Invalid session protobuf")
}

fn decode(bytes: &[u8]) -> Result<proto::Session, &'static str> {
    let message = decode_raw(bytes)?;
    validate(&message)?;
```

### 2. `src/browser_phase.rs`

Store completed CPU samples without taking another end timestamp.

**Locate this existing block:**

```rust
}

pub fn finish(name: &str, started: f64, bytes: u64, source: &str) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub fn completed(phase: crate::load_measure::Completed<'_>, source: &str) {
    let name = if phase.succeeded {
        phase.name.to_owned()
    } else {
        format!("{} failed", phase.name)
    };
    let _ = crate::browser_report::phase(crate::load_phase::Phase {
        name,
        duration_ms: phase.duration_ms,
        elapsed_ms: phase.elapsed_ms,
        bytes: phase.bytes,
        source: source.to_owned(),
    });
}

pub fn finish(name: &str, started: f64, bytes: u64, source: &str) {
```

### 3. `src/lib.rs`

Register document phase checks.

**Locate this existing block:**

```rust
#[cfg(test)]
mod document_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod document_phase_tests;
#[cfg(test)]
mod document_tests;
```

### 4. `src/document_phase_tests.rs`

Copy checks for phase ordering, early failure and unchanged source precision.

Create the file and type:

```rust
use crate::{document, specimen};
use prost::Message;

fn observed(
    bytes: &[u8],
) -> (
    Result<document::Loaded, &'static str>,
    Vec<(String, bool)>,
) {
    let mut tick = 0.0;
    let mut phases = Vec::new();
    let result = document::load_observed(
        bytes,
        None,
        &mut || {
            tick += 1.0;
            tick
        },
        &mut |phase| {
            assert_eq!(phase.duration_ms, 1.0);
            assert_eq!(phase.bytes, bytes.len() as u64);
            assert_eq!(
                phase.elapsed_ms,
                (phases.len() + 1) as f64 * 2.0
            );
            phases.push((phase.name.to_owned(), phase.succeeded));
        },
    );
    (result, phases)
}

#[test]
fn successful_preparation_records_actual_stages_and_preserves_source_precision() {
    let bytes = specimen::precise_bytes();
    let (result, phases) = observed(&bytes);
    assert_eq!(
        phases,
        [
            "decode",
            "validation",
            "kernel",
            "display walk"
        ]
        .map(|name| (name.into(), true))
    );
    let loaded = result.unwrap();
    assert_eq!(loaded.meshes.len(), 3);
    assert_eq!(
        loaded.document.objects.meshes[0]
            .to_proto()
            .vertices[&0]
            .x,
        -0.775000000123456
    );
}

#[test]
fn failed_stages_stop_before_later_work() {
    let (result, phases) = observed(&[0xff]);
    assert!(result.is_err());
    assert_eq!(phases, vec![("decode".into(), false)]);
    let mut message =
        session_rust::proto::Session::decode(specimen::bytes().as_slice()).unwrap();
    message.objects.as_mut().unwrap().meshes[0].color_mode = 1;
    let (result, phases) = observed(&message.encode_to_vec());
    assert!(result.is_err());
    assert_eq!(
        phases,
        vec![
            ("decode".into(), true),
            ("validation".into(), false)
        ]
    );
    message.objects.as_mut().unwrap().meshes[0].color_mode = 0;
    for face in message.objects.as_mut().unwrap().meshes[0]
        .faces
        .values_mut()
    {
        face.vertices.truncate(3);
    }
    message.objects.as_mut().unwrap().meshes[0]
        .triangulation
        .clear();
    message.objects.as_mut().unwrap().meshes[0]
        .vertices
        .get_mut(&0)
        .unwrap()
        .x = 1e100;
    let (result, phases) = observed(&message.encode_to_vec());
    assert!(result.is_err());
    assert_eq!(
        phases,
        vec![
            ("decode".into(), true),
            ("validation".into(), true),
            ("kernel".into(), true),
            ("display walk".into(), false)
        ]
    );
}
```

## <a id="checkpoint-34gdf-upload"></a>10. Measure upload preparation and GPU completion

**What this code does:** Distinguish CPU buffer preparation from completion on the drawing queue.

### 1. `src/geometry_cache.rs`

Count actual geometry payload, independently of resident buffer sizes. Advance only when a new geometry buffer is initialized.

**Locate this existing block:**

```rust
    pub uploads: usize,
}

impl GeometryCache {
    pub fn get(&mut self, device: &wgpu::Device, source: &Rc<Mesh>) -> Rc<GpuGeometry> {
        let key = Rc::as_ptr(source);
        if let Some(geometry) = self
            .entries
            .get(&key)
            .and_then(Weak::upgrade)
        {
            return geometry;
        }
        let geometry = Rc::new(GpuGeometry::upload(
            device,
            Rc::clone(source),
        ));
        self.entries
            .insert(key, Rc::downgrade(&geometry));
        self.uploads += 1;
        geometry
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub uploads: usize,
    pub uploaded_bytes: u64,
}

impl GeometryCache {
    pub fn get(&mut self, device: &wgpu::Device, source: &Rc<Mesh>) -> Rc<GpuGeometry> {
        let key = Rc::as_ptr(source);
        if let Some(geometry) = self
            .entries
            .get(&key)
            .and_then(Weak::upgrade)
        {
            return geometry;
        }
        let geometry = Rc::new(GpuGeometry::upload(
            device,
            Rc::clone(source),
        ));
        self.entries
            .insert(key, Rc::downgrade(&geometry));
        self.uploads += 1;
        self.uploaded_bytes +=
            (source.vertices().len() * 24 + source.indices().len() * 2) as u64;
        geometry
```

### 2. `src/renderer.rs`

Include initialized object settings and subsequent writes, excluding camera uniforms.

**Locate this existing block:**

```rust
    }

    pub fn stats(&self) -> [usize; 4] {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn uploaded_bytes(&self) -> u64 {
        self.geometry.uploaded_bytes
            + self.settings_allocations as u64 * 80
            + self.settings_bytes as u64
    }

    pub fn stats(&self) -> [usize; 4] {
```

### 3. `src/browser_upload.rs`

Fence the drawing queue and reject completion from a retired or failed device.

Create the file and type:

```rust
pub fn prepare(
    renderer: &mut crate::renderer::Renderer,
    scene: &crate::scene::Scene,
    selected: Option<crate::scene::ObjectId>,
    fault: crate::gpu_fault::Fault,
) {
    let started = crate::browser_phase::now();
    let before = renderer.uploaded_bytes();
    renderer.set_scene(scene, selected);
    let bytes = renderer.uploaded_bytes() - before;
    crate::browser_phase::finish("upload preparation", started, bytes, "GPU");
    if bytes == 0 {
        return;
    }
    let submitted = crate::browser_phase::now();
    renderer.queue.submit([]);
    renderer
        .queue
        .on_submitted_work_done(move || {
            if crate::browser_runtime::owns(&fault) && fault.message().is_none() {
                crate::browser_phase::finish(
                    "GPU upload ready",
                    submitted,
                    bytes,
                    "GPU",
                );
            }
        });
}
```

### 4. `src/lib.rs`

Register the browser-only queue boundary.

**Locate this existing block:**

```rust
#[cfg(target_arch = "wasm32")]
mod listeners;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(target_arch = "wasm32")]
mod browser_upload;
#[cfg(target_arch = "wasm32")]
mod listeners;
```

### 5. `src/browser.rs`

Measure successful scene synchronization after the editor transaction commits.

**Locate this existing block:**

```rust
                    Ok(Some(Change::Scene)) => {
                        renderer.set_scene(&editor.scene, editor.selected);
                        if unloading {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                    Ok(Some(Change::Scene)) => {
                        crate::browser_upload::prepare(
                            &mut renderer,
                            &editor.scene,
                            editor.selected,
                            active_fault.clone(),
                        );
                        if unloading {
```

## <a id="checkpoint-34gdg-network"></a>11. Measure accepted source fetches

**What this code does:** Retain real fetch and Resource Timing measurements only for the accepted reload ticket.

### 1. `Cargo.toml`

Enable the browser’s native network timing entry.

**Locate this existing block:**

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "ErrorEvent", "PromiseRejectionEvent", "Location", "Navigator", "Storage", "Performance", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
console_error_panic_hook = "=0.1.7"
```

Edit the existing block to match this code; keep unchanged lines:

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "ErrorEvent", "PromiseRejectionEvent", "Location", "Navigator", "Storage", "Performance", "PerformanceResourceTiming", "PerformanceEntry", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
console_error_panic_hook = "=0.1.7"
```

### 2. `src/browser_network.rs`

Collect actual body and browser-network measurements without retaining source owners.

Create the file and type:

```rust
use wasm_bindgen::JsCast;

pub fn source(url: &str) -> String {
    let Ok(value) = web_sys::Url::new(url) else {
        return "unavailable URL".into();
    };
    let _ = value.set_username("");
    let _ = value.set_password("");
    value.set_search("");
    value.set_hash("");
    value.href().chars().take(1024).collect()
}

pub fn measured(
    url: &str,
    started: f64,
    succeeded: bool,
    bytes: u64,
) -> Vec<crate::load_phase::Phase> {
    let ended = crate::browser_phase::now();
    let source = source(url);
    let mut phases = vec![crate::load_phase::Phase {
        name: if succeeded {
            "source fetch"
        } else {
            "source fetch failed"
        }
        .into(),
        duration_ms: ended - started,
        elapsed_ms: ended,
        bytes,
        source: source.clone(),
    }];
    if let Some(clock) = web_sys::window().and_then(|window| window.performance()) {
        for entry in clock.get_entries_by_name(url).iter().rev() {
            let Ok(entry) = entry.dyn_into::<web_sys::PerformanceResourceTiming>()
            else {
                continue;
            };
            if entry.start_time() < started || entry.response_end() > ended {
                continue;
            }
            phases.push(crate::load_phase::Phase {
                name: "network response".into(),
                duration_ms: entry.response_end() - entry.fetch_start(),
                elapsed_ms: entry.response_end(),
                bytes: entry.transfer_size() as u64,
                source,
            });
            break;
        }
    }
    phases
}
```

### 3. `src/lib.rs`

Register network measurement and safe URL handling.

**Locate this existing block:**

```rust
#[cfg(target_arch = "wasm32")]
mod browser_reload;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(target_arch = "wasm32")]
mod browser_network;
#[cfg(target_arch = "wasm32")]
mod browser_reload;
```

### 4. `src/browser_reload.rs`

Retain measured fetch phases only after the existing ticket accepts the task.

**Locate this existing block:**

```rust
    wasm_bindgen_futures::spawn_local(async move {
        let result: Result<Vec<Vec<u8>>, String> = async {
            let mut values = Vec::new();
            for url in request.urls {
                values.push(crate::source_fetch::fetch(&url, &signal).await?);
            }
            Ok(values)
        }
        .await;
        let Some((keys, intent)) = shared
            .borrow_mut()
            .finish(request.ticket, result.is_err())
        else {
            return;
        };
        let reply = Reply::new(keys, intent, result);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    wasm_bindgen_futures::spawn_local(async move {
        let mut phases = Vec::new();
        let result: Result<Vec<Vec<u8>>, String> = async {
            let mut values = Vec::new();
            for url in request.urls {
                let started = crate::browser_phase::now();
                let value = crate::source_fetch::fetch(&url, &signal).await;
                phases.extend(crate::browser_network::measured(
                    &url,
                    started,
                    value.is_ok(),
                    value
                        .as_ref()
                        .map_or(0, |bytes| bytes.len() as u64),
                ));
                values.push(value?);
            }
            Ok(values)
        }
        .await;
        let Some((keys, intent)) = shared
            .borrow_mut()
            .finish(request.ticket, result.is_err())
        else {
            return;
        };
        for phase in phases {
            let _ = crate::browser_report::phase(phase);
        }
        let reply = Reply::new(keys, intent, result);
```

## <a id="checkpoint-34gdga-revision"></a>12. Record accepted file revisions

**What this code does:** Identify accepted source versions without recording rejected or stale documents.

### 1. `src/revision.rs`

Collect only bounded hash strings from the accepted scene.

Create the file and type:

```rust
pub fn versions(scene: &crate::scene::Scene) -> Vec<String> {
    let mut versions: Vec<_> = scene
        .objects()
        .iter()
        .filter_map(|object| {
            object
                .origin()
                .map(|origin| origin.version.hex())
        })
        .collect();
    versions.sort();
    versions.dedup();
    versions
}

#[cfg(test)]
mod tests {
    use crate::{
        editor::{Action, Editor},
        origin::FileVersion,
    };
    #[test]
    fn revisions_describe_distinct_active_sources_without_owning_them() {
        let mut editor = Editor::default();
        let bytes = crate::specimen::bytes();
        editor
            .apply(Action::Import(bytes.clone()))
            .unwrap();
        editor
            .apply(Action::Import(bytes.clone()))
            .unwrap();
        let weak = std::rc::Rc::downgrade(
            &editor
                .scene
                .objects()
                .last()
                .unwrap()
                .source()
                .unwrap()
                .origin,
        );
        let versions = super::versions(&editor.scene);
        assert_eq!(versions, vec![FileVersion::of(&bytes).hex()]);
        let replacement = crate::specimen::precise_bytes();
        editor
            .apply(Action::Replace(replacement.clone()))
            .unwrap();
        assert_eq!(
            super::versions(&editor.scene),
            vec![FileVersion::of(&replacement).hex()]
        );
        editor.apply(Action::Close).unwrap();
        assert!(weak.upgrade().is_none());
        assert!(super::versions(&editor.scene).is_empty());
        assert_eq!(versions.len(), 1);
    }
}
```

### 2. `src/lib.rs`

Register accepted content identity.

**Locate this existing block:**

```rust
mod replace_tests;
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod replace_tests;
pub mod revision;
#[cfg(test)]
```

### 3. `src/browser_phase.rs`

Record each accepted version as a bounded event.

**Locate this existing block:**

```rust
}

pub fn completed(phase: crate::load_measure::Completed<'_>, source: &str) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub fn revision(scene: &crate::scene::Scene) {
    for version in crate::revision::versions(scene) {
        let _ = crate::browser_report::observe("revision", &version);
    }
}

pub fn completed(phase: crate::load_measure::Completed<'_>, source: &str) {
```

### 4. `src/browser.rs`

Observe versions only after successful file adoption. Measure restored display synchronization and identify accepted source versions.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                            report("Document closed.");
                            panel.result("Document closed.");
                        } else if event.type_() == "viewer-file" {
                            let message = if replacement {
```

Replace it with:

```rust
                            report("Document closed.");
                            panel.result("Document closed.");
                        } else if event.type_() == "viewer-file" {
                            crate::browser_phase::revision(&editor.scene);
                            let message = if replacement {
```

**Locate this block:**

```rust
                        Ok(Some(crate::edit_replay::Reply::Changed(_))) => {
                            renderer.set_scene(&editor.scene, editor.selected);
                            let message = if command == "Reload Sources" {
                                "Editable sources restored; display retained."
                                    .to_owned()
```

Replace it with:

```rust
                        Ok(Some(crate::edit_replay::Reply::Changed(_))) => {
                            crate::browser_upload::prepare(
                                &mut renderer,
                                &editor.scene,
                                editor.selected,
                                active_fault.clone(),
                            );
                            crate::browser_phase::revision(&editor.scene);
                            let message = if command == "Reload Sources" {
                                "Editable sources restored; display retained."
                                    .to_owned()
```

## <a id="checkpoint-34ge-owner"></a>13. Keep the document outside the GPU runtime

**What this code does:** Give GPU runtimes shared access to a separately owned CPU document.

### 1. `src/drawing_document.rs`

Give CPU document state an owner independent of a device or renderer.

Create the file and type:

```rust
pub type Shared = std::rc::Rc<std::cell::RefCell<crate::editor::Editor>>;

pub fn new() -> Shared {
    std::rc::Rc::new(std::cell::RefCell::new(Default::default()))
}
```

### 2. `src/lib.rs`

Register the independent document owner.

**Locate this existing block:**

```rust
mod diagnostic_tests;
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod diagnostic_tests;
pub mod drawing_document;
#[cfg(test)]
```

### 3. `src/browser.rs`

Allow a drawing runtime to use an existing CPU document. Borrow the retained editor during startup. End the startup borrow before the event closure borrows the same editor. Retain document authority beside the runtime’s cancellation owners.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    listeners.listen(window.as_ref(), "viewer-file-error")?;
    listeners.listen(window.as_ref(), "viewer-reload")?;
    listeners.listen(window.as_ref(), "pagehide")?;
    crate::browser_runtime::install(listeners, owned_request, owned_reload, fault);
    report(
```

Replace it with:

```rust
    listeners.listen(window.as_ref(), "viewer-file-error")?;
    listeners.listen(window.as_ref(), "viewer-reload")?;
    listeners.listen(window.as_ref(), "pagehide")?;
    crate::browser_runtime::install(
        listeners,
        owned_request,
        owned_reload,
        fault,
        owned_editor,
    );
    report(
```

**Locate this block:**

```rust
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

Replace it with:

```rust
    let owned_request = std::rc::Rc::clone(&request);
    let owned_reload = std::rc::Rc::clone(&reload);
    let active_fault = fault.clone();
    drop(editor);
    let owned_editor = std::rc::Rc::clone(&shared);
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(
        move |event: web_sys::Event| {
            if active_fault.message().is_some() {
                return;
            }
            let mut editor = shared.borrow_mut();
            if event.type_() == "pagehide" {
                gesture.cancel();
                if event
```

**Locate this block:**

```rust
        .ok_or("No compatible surface format")?;
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let mut editor = Editor::default();
    let pipeline_started = crate::browser_phase::now();
    let mut renderer = Renderer::new(
        device,
```

Replace it with:

```rust
        .ok_or("No compatible surface format")?;
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let mut editor = shared.borrow_mut();
    let pipeline_started = crate::browser_phase::now();
    let mut renderer = Renderer::new(
        device,
```

**Locate this block:**

```rust
pub async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document
```

Replace it with:

```rust
pub async fn run() -> Result<(), JsValue> {
    run_with(crate::drawing_document::new()).await
}

pub async fn run_with(shared: crate::drawing_document::Shared) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document
```

### 4. `src/browser_runtime.rs`

Keep a CPU owner available to an authorized recovery path. Only the current drawing device can obtain the retained CPU document.

**Locate this existing block:**

```rust
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
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    fault: crate::gpu_fault::Fault,
    document: crate::drawing_document::Shared,
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
    document: crate::drawing_document::Shared,
) {
    ACTIVE.with(|slot| {
        slot.replace(Some(Runtime {
            _listeners: listeners,
            request,
            reload,
            fault,
            document,
        }))
    });
}

pub fn document(
    fault: &crate::gpu_fault::Fault,
) -> Option<crate::drawing_document::Shared> {
    ACTIVE.with(|slot| {
        slot.borrow()
            .as_ref()
            .filter(|runtime| runtime.fault.same_device(fault))
            .map(|runtime| Rc::clone(&runtime.document))
    })
}
```

## <a id="checkpoint-34gea-policy"></a>14. Permit one conservative recovery reload

**What this code does:** Allow one device-loss recovery reload and retain reduced quality until an ordinary reload.

### 1. `src/recovery.rs`

Permit one recovery only for a normal run that lost its device.

Create the file and type:

```rust
#[derive(Default)]
pub struct Recovery {
    reduced: bool,
    requested: bool,
}

impl Recovery {
    pub fn adopted() -> Self {
        Self {
            reduced: true,
            requested: false,
        }
    }
    pub fn reduced(&self) -> bool {
        self.reduced
    }

    pub fn request(&mut self, device_lost: bool) -> bool {
        if !device_lost || self.reduced || self.requested {
            return false;
        }
        self.requested = true;
        true
    }
}
```

### 2. `src/lib.rs`

Register the one-reload policy and its tests.

**Locate this existing block:**

```rust
pub mod prepared;
pub mod rehydrate;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod prepared;
pub mod recovery;
#[cfg(test)]
mod recovery_tests;
pub mod rehydrate;
```

### 3. `src/recovery_tests.rs`

Copy normal, duplicate, recovered and fresh-run checks.

Create the file and type:

```rust
use crate::recovery::Recovery;

#[test]
fn only_device_loss_can_request_one_recovery_reload() {
    let mut policy = Recovery::default();
    assert!(!policy.reduced());
    assert!(!policy.request(false));
    assert!(policy.request(true));
    assert!(!policy.request(true));
    assert!(!policy.request(false));
}

#[test]
fn recovered_runs_refuse_repeated_reload_until_an_ordinary_reload() {
    let mut recovered = Recovery::adopted();
    assert!(recovered.reduced());
    assert!(!recovered.request(true));
    let mut fresh = Recovery::default();
    assert!(!fresh.reduced());
    assert!(fresh.request(true));
}
```

## <a id="checkpoint-34geb-reload"></a>15. Recover once at reduced canvas density

**What this code does:** Save device-loss diagnostics, reload once and draw conservatively until the next ordinary reload.

### 1. `Cargo.toml`

Enable recovery URL cleanup through the browser history API.

**Locate this existing block:**

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "ErrorEvent", "PromiseRejectionEvent", "Location", "Navigator", "Storage", "Performance", "PerformanceResourceTiming", "PerformanceEntry", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
console_error_panic_hook = "=0.1.7"
```

Edit the existing block to match this code; keep unchanged lines:

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "ErrorEvent", "PromiseRejectionEvent", "Location", "History", "UrlSearchParams", "Navigator", "Storage", "Performance", "PerformanceResourceTiming", "PerformanceEntry", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
console_error_panic_hook = "=0.1.7"
```

### 2. `src/browser_recovery.rs`

Adopt one conservative reload and retain all other URL parameters.

Create the file and type:

```rust
use std::cell::RefCell;
use wasm_bindgen::JsValue;

thread_local! { static POLICY: RefCell<crate::recovery::Recovery> = RefCell::new(Default::default()); }

pub fn density(value: f64) -> f64 {
    POLICY.with(|policy| {
        if policy.borrow().reduced() {
            value.min(1.0)
        } else {
            value
        }
    })
}

pub fn adopt() -> Result<Option<String>, JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let url = web_sys::Url::new(&window.location().href()?)?;
    let params = url.search_params();
    let Some(reason) = params.get("recovered") else {
        return Ok(None);
    };
    POLICY.with(|policy| policy.replace(crate::recovery::Recovery::adopted()));
    params.delete("recovered");
    let _ =
        window
            .history()?
            .replace_state_with_url(&JsValue::NULL, "", Some(&url.href()));
    Ok(Some(format!(
        "{}; drawing at device scale 1 until the next reload",
        reason.chars().take(200).collect::<String>()
    )))
}

pub fn request(message: &str) -> bool {
    if !POLICY.with(|policy| {
        policy
            .borrow_mut()
            .request(message.contains("device lost"))
    }) {
        return false;
    }
    let navigate = || -> Result<(), JsValue> {
        let window = web_sys::window().ok_or("No browser window")?;
        let url = web_sys::Url::new(&window.location().href()?)?;
        url.search_params().set(
            "recovered",
            &message.chars().take(200).collect::<String>(),
        );
        window.location().replace(&url.href())
    };
    navigate().is_ok()
}
```

### 3. `src/lib.rs`

Register browser recovery policy and navigation.

**Locate this existing block:**

```rust
#[cfg(target_arch = "wasm32")]
mod browser_reload;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(target_arch = "wasm32")]
mod browser_recovery;
#[cfg(target_arch = "wasm32")]
mod browser_reload;
```

### 4. `src/browser.rs`

Persist and download the failed run before requesting its recovery reload. Adopt reduced quality before beginning the new drawing run. Keep the recovery reason visible in the real command dock. Report conservative recovery without changing the normal startup instruction. Cap recovered device density while preserving normal high-DPI drawing.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    let Some(size) = Viewport::from_css(
        rect.width(),
        rect.height(),
        window.device_pixel_ratio(),
        limit,
```

Replace it with:

```rust
    let Some(size) = Viewport::from_css(
        rect.width(),
        rect.height(),
        crate::browser_recovery::density(window.device_pixel_ratio()),
        limit,
```

**Locate this block:**

```rust
        fault,
        owned_editor,
    );
    report(
        "Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.",
    );
    Ok(())
}
```

Replace it with:

```rust
        fault,
        owned_editor,
    );
    report(&recovery.unwrap_or_else(|| {
        "Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag."
            .into()
    }));
    Ok(())
}
```

**Locate this block:**

```rust
    if let Some(message) = crate::browser_report::previous_notice() {
        panel.result(message);
    }
    panel.update(None, &canvas)?;
    if resize(
        &window,
```

Replace it with:

```rust
    if let Some(message) = crate::browser_report::previous_notice() {
        panel.result(message);
    }
    if let Some(message) = &recovery {
        panel.result(message);
    }
    panel.update(None, &canvas)?;
    if resize(
        &window,
```

**Locate this block:**

```rust
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?
        .dyn_into()?;
    crate::browser_report::start()?;
    let startup = crate::browser_startup::Guard::new()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
```

Replace it with:

```rust
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?
        .dyn_into()?;
    let recovery = crate::browser_recovery::adopt()?;
    crate::browser_report::start()?;
    let startup = crate::browser_startup::Guard::new()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
```

**Locate this block:**

```rust
            crate::browser_report::fatal(&message);
            report(&format!("Cannot draw: {message}"));
        }
    });
```

Replace it with:

```rust
            crate::browser_report::fatal(&message);
            if crate::browser_recovery::request(&message) {
                return;
            }
            report(&format!("Cannot draw: {message}"));
        }
    });
```

## Build & Run

![Recorded visual output for Lesson 4.5 (34geb-reload)](../../screenshots/journey/34geb-reload-browser.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 4.5
npm --prefix ../session_tests run course -- save 4.5
```

Continue with [5.1 · Original lines and a screen-space stroke lane](35-stroke.md).
