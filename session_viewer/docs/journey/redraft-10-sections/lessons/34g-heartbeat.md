# 4.4 · Heartbeat suspension and revocable startup

**Goal:** Stop delayed adapter or device delivery from installing a renderer after final exit. · **Steps:** 8 · **Source:** `src/lib.rs,src/selftest/lifecycle.rs,src/engine/gpu/device.rs`

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

## <a id="checkpoint-34g-heartbeat"></a>1. Refresh heartbeat without rewriting failure evidence

**What this code does:** Refresh actual last-seen metadata independently of first failure, event history and GPU ownership.

### 1. `src/diagnostic.rs`

Advance heartbeat independently of observations, outcome and the first fatal timestamp; invalid timestamp text leaves the report unchanged.

**Locate this existing block:**

```rust
    }

    pub fn failure(&self) -> Option<&Event> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn heartbeat(&mut self, time: String) -> Result<(), &'static str> {
        if time.is_empty() || time.chars().count() > 64 {
            return Err("Invalid heartbeat timestamp");
        }
        self.last_seen = time;
        Ok(())
    }

    pub fn failure(&self) -> Option<&Event> {
```

### 2. `src/heartbeat_tests.rs`

Optional authoring fixture. Verify all run outcomes, first-failure/event retention and atomic timestamp-text bounds without a browser clock dependency.

Create the file and type:

```rust
use crate::{diagnostic::Outcome, diagnostic_shape_tests::report};

#[test]
fn heartbeat_preserves_everything_except_last_seen() {
    for outcome in [
        Outcome::Running,
        Outcome::Ready,
        Outcome::Closed,
        Outcome::Failed,
    ] {
        let mut value = report();
        if outcome == Outcome::Failed {
            value
                .record(
                    "first".into(),
                    1.0,
                    "fatal",
                    "original failure",
                )
                .unwrap();
        }
        value.outcome = outcome;
        let mut expected = value.clone();
        expected.last_seen = "heartbeat".into();
        value.heartbeat("heartbeat".into()).unwrap();
        assert_eq!(value, expected);
        assert!(value.valid());
    }
}

#[test]
fn heartbeat_rejects_invalid_text_without_mutation() {
    let mut value = report();
    let before = value.clone();
    for bad in [String::new(), "x".repeat(65)] {
        assert!(value.heartbeat(bad).is_err());
        assert_eq!(value, before);
    }
    value.heartbeat("x".repeat(64)).unwrap();
    assert!(value.valid());
}
```

### 3. `src/lib.rs`

Include native heartbeat state proofs.

**Locate this existing block:**

```rust
mod gpu_fault_tests;
pub mod history;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod gpu_fault_tests;
#[cfg(test)]
mod heartbeat_tests;
pub mod history;
```

### 4. `src/browser_report.rs`

Refresh actual browser context and UTC heartbeat, release its borrow, then persist metadata without invoking renderer or recording synthetic observations.

**Locate this existing block:**

```rust
}

pub fn download() -> Result<(), JsValue> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn heartbeat() -> Result<(), JsValue> {
    let context = context()?;
    let time = js_sys::Date::new_0().to_iso_string().into();
    REPORT.with(|slot| {
        let mut slot = slot.borrow_mut();
        let report = slot.as_mut().ok_or("No report")?;
        report
            .heartbeat(time)
            .map_err(JsValue::from_str)?;
        report.context = context;
        Ok::<_, JsValue>(())
    })?;
    let _ = persist();
    Ok(())
}

pub fn download() -> Result<(), JsValue> {
```

## <a id="checkpoint-34ga-timer"></a>2. Own the periodic diagnostic heartbeat

**What this code does:** Schedule a real 15-second metadata heartbeat independently of GPU lifetime and cancel its callback through an owner.

### 1. `src/timer.rs`

Own the actual interval and callback together; clear its native timer before Rust releases the closure.

Create the file and type:

```rust
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

pub struct Timer {
    window: web_sys::Window,
    id: i32,
    _callback: Closure<dyn FnMut()>,
}

impl Timer {
    pub fn every(
        milliseconds: i32,
        tick: impl FnMut() + 'static,
    ) -> Result<Self, JsValue> {
        let window = web_sys::window().ok_or("No browser window")?;
        let callback = Closure::wrap(Box::new(tick) as Box<dyn FnMut()>);
        let id = window.set_interval_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            milliseconds,
        )?;
        Ok(Self {
            window,
            id,
            _callback: callback,
        })
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        self.window
            .clear_interval_with_handle(self.id);
    }
}
```

### 2. `src/lib.rs`

Keep browser interval ownership out of the native editor model.

**Locate this existing block:**

```rust
#[cfg(target_arch = "wasm32")]
pub mod report_storage;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(target_arch = "wasm32")]
pub mod report_storage;
#[cfg(target_arch = "wasm32")]
mod timer;
```

### 3. `src/browser_report.rs`

Retain metadata scheduling independently of the GPU runtime lifetime. Install periodic metadata refresh at run startup; report scheduler unavailability without rejecting drawing or current downloads. Replace one owned interval at a time, cancel outside the slot borrow, and keep the callback metadata-only.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    let _ = persist();
    crate::file_output::download_named(json.as_bytes(), "viewer-diagnostic.json")
}
```

Replace it with:

```rust
    let _ = persist();
    crate::file_output::download_named(json.as_bytes(), "viewer-diagnostic.json")
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn start_periodic() -> Result<(), JsValue> {
    stop_periodic();
    let timer = crate::timer::Timer::every(15000, || {
        let _ = heartbeat();
    })?;
    BEAT.with(|slot| slot.replace(Some(timer)));
    Ok(())
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn stop_periodic() -> bool {
    let timer = BEAT.with(|slot| slot.borrow_mut().take());
    let active = timer.is_some();
    drop(timer);
    active
}
```

**Locate this block:**

```rust
    STORE.with(|slot| slot.replace(Some(store)));
    REPORT.with(|slot| slot.replace(Some(report)));
    let _ = persist();
    Ok(())
}
```

Replace it with:

```rust
    STORE.with(|slot| slot.replace(Some(store)));
    REPORT.with(|slot| slot.replace(Some(report)));
    let _ = persist();
    if let Err(error) = start_periodic() {
        let _ = observe(
            "diagnostic",
            &format!("Heartbeat unavailable: {error:?}"),
        );
    }
    Ok(())
}
```

**Locate this block:**

```rust
    static PREVIOUS: RefCell<Option<Report>> = const { RefCell::new(None) };
}

fn context() -> Result<Context, JsValue> {
```

Replace it with:

```rust
    static PREVIOUS: RefCell<Option<Report>> = const { RefCell::new(None) };
    static BEAT: RefCell<Option<crate::timer::Timer>> = const { RefCell::new(None) };
}

fn context() -> Result<Context, JsValue> {
```

## <a id="checkpoint-34gb-close"></a>3. Mark a final healthy run closed

**What this code does:** Persist a healthy final exit as closed while preserving first-failure evidence and every observation.

### 1. `src/diagnostic.rs`

Mark a healthy final exit closed without erasing a failure or appending synthetic observations.

**Locate this existing block:**

```rust
    }

    pub fn failure(&self) -> Option<&Event> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn close(&mut self, time: String) -> Result<(), &'static str> {
        self.heartbeat(time)?;
        if self.failure.is_none() {
            self.outcome = Outcome::Closed;
        }
        Ok(())
    }

    pub fn failure(&self) -> Option<&Event> {
```

### 2. `src/close_report_tests.rs`

Optional authoring fixture. Prove healthy/failed final outcomes, repeatability and atomic refusal separately from browser suspension.

Create the file and type:

```rust
use crate::{diagnostic::Outcome, diagnostic_shape_tests::report};

#[test]
fn final_close_keeps_failure_and_observations() {
    for outcome in [
        Outcome::Running,
        Outcome::Ready,
        Outcome::Closed,
        Outcome::Failed,
    ] {
        let mut value = report();
        if outcome == Outcome::Failed {
            value
                .record(
                    "first".into(),
                    1.0,
                    "fatal",
                    "original failure",
                )
                .unwrap();
        }
        value.outcome = outcome;
        let mut expected = value.clone();
        expected.last_seen = "closed".into();
        if expected.failure().is_none() {
            expected.outcome = Outcome::Closed;
        }
        value.close("closed".into()).unwrap();
        assert_eq!(value, expected);
        assert!(value.valid());
        value.close("closed".into()).unwrap();
        assert_eq!(value, expected);
    }
}

#[test]
fn close_rejects_invalid_time_atomically() {
    let mut value = report();
    let before = value.clone();
    for bad in [String::new(), "x".repeat(65)] {
        assert!(value.close(bad).is_err());
        assert_eq!(value, before);
    }
}

#[test]
fn late_ready_milestone_cannot_reopen_final_run() {
    let mut value = report();
    value.close("closed".into()).unwrap();
    value
        .record(
            "later".into(),
            1.0,
            "milestone",
            "geometry on screen",
        )
        .unwrap();
    assert_eq!(value.outcome, Outcome::Closed);
    assert!(value.valid());
}
```

### 3. `src/lib.rs`

Include native final-close policy proofs.

**Locate this existing block:**

```rust
pub mod camera;
pub mod diagnostic;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod camera;
#[cfg(test)]
mod close_report_tests;
pub mod diagnostic;
```

### 4. `src/browser_report.rs`

Refresh real context/time, release the report borrow and persist the final outcome without accessing GPU state.

**Locate this existing block:**

```rust
}

pub fn download() -> Result<(), JsValue> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn close_report() -> Result<(), JsValue> {
    let context = context()?;
    let time = js_sys::Date::new_0().to_iso_string().into();
    REPORT.with(|slot| {
        let mut slot = slot.borrow_mut();
        let report = slot.as_mut().ok_or("No report")?;
        report
            .close(time)
            .map_err(JsValue::from_str)?;
        report.context = context;
        Ok::<_, JsValue>(())
    })?;
    let _ = persist();
    Ok(())
}

pub fn download() -> Result<(), JsValue> {
```

### 5. `src/diagnostic.rs`

A delayed ready milestone must not reopen a final closed run; cache suspension retains Ready without closing.

**Locate this existing block:**

```rust
            && self.failure.is_none()
        {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            && self.failure.is_none()
            && self.outcome != Outcome::Closed
        {
```

### 6. `src/browser_report.rs`

Let Chrome submit a real late milestone through the same metadata path; release builds gain no user control.

**Locate this existing block:**

```rust
}

pub fn observe(kind: &str, message: &str) -> Result<(), JsValue> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn observe(kind: &str, message: &str) -> Result<(), JsValue> {
```

## <a id="checkpoint-34gba-lifecycle"></a>4. Own diagnostic page transitions

**What this code does:** Connect real pagehide/pageshow to independent diagnostics, cached suspension and safe final callback cleanup.

### 1. `src/report_lifecycle.rs`

Own two metadata callbacks independently of GPU; retain cached outcomes, defer final callback destruction, and guard cleanup by allocation identity.

Create the file and type:

```rust
use crate::{browser_report as report, listeners::Listeners};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

thread_local! { static ACTIVE: RefCell<Option<(Rc<()>, Listeners)>> = const { RefCell::new(None) }; }

fn current(token: &Rc<()>) -> bool {
    ACTIVE.with(|slot| {
        slot.borrow()
            .as_ref()
            .is_some_and(|(id, _)| Rc::ptr_eq(id, token))
    })
}

#[wasm_bindgen::prelude::wasm_bindgen(js_name = stop_report_lifecycle)]
pub fn stop() -> bool {
    let owner = ACTIVE.with(|slot| slot.borrow_mut().take());
    let active = owner.is_some();
    report::stop_periodic();
    drop(owner);
    active
}

#[wasm_bindgen::prelude::wasm_bindgen(js_name = install_report_lifecycle)]
pub fn install() -> Result<(), JsValue> {
    stop();
    let window = web_sys::window().ok_or("No browser window")?;
    let token = Rc::new(());
    let guarded = Rc::clone(&token);
    let callback =
        Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
            if !current(&guarded) {
                return;
            }
            match event.type_().as_str() {
                "pagehide" => {
                    let cached = event
                        .dyn_ref::<web_sys::PageTransitionEvent>()
                        .is_some_and(|event| event.persisted());
                    let _ = report::observe(
                        "lifecycle",
                        if cached {
                            "pagehide cached"
                        } else {
                            "pagehide final"
                        },
                    );
                    report::stop_periodic();
                    if !cached {
                        let _ = report::close_report();
                        let token = Rc::clone(&guarded);
                        wasm_bindgen_futures::spawn_local(async move {
                            if current(&token) {
                                stop();
                            }
                        });
                    }
                }
                "pageshow" => {
                    let _ = report::observe("lifecycle", "pageshow");
                    if let Err(error) = report::start_periodic() {
                        let _ = report::observe(
                            "diagnostic",
                            &format!("Heartbeat unavailable: {error:?}"),
                        );
                    }
                }
                _ => {}
            }
        });
    let mut listeners = Listeners::new(callback);
    listeners.listen(window.as_ref(), "pagehide")?;
    listeners.listen(window.as_ref(), "pageshow")?;
    ACTIVE.with(|slot| slot.replace(Some((token, listeners))));
    Ok(())
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn report_lifecycle_running() -> bool {
    ACTIVE.with(|slot| slot.borrow().is_some())
}
```

### 2. `src/lib.rs`

Keep page transition ownership in the browser layer.

**Locate this existing block:**

```rust
#[cfg(target_arch = "wasm32")]
pub mod report_storage;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(target_arch = "wasm32")]
mod report_lifecycle;
#[cfg(target_arch = "wasm32")]
pub mod report_storage;
```

### 3. `src/browser_report.rs`

Install metadata lifecycle ownership before scheduling heartbeat; denied listener registration leaves drawing and current downloads available.

**Locate this existing block:**

```rust
    let _ = persist();
    if let Err(error) = start_periodic() {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let _ = persist();
    if let Err(error) = crate::report_lifecycle::install() {
        let _ = observe(
            "diagnostic",
            &format!("Lifecycle unavailable: {error:?}"),
        );
    }
    if let Err(error) = start_periodic() {
```

## <a id="checkpoint-34gbb-suspension"></a>5. Keep each heartbeat pause reason separate

**What this code does:** Check that clearing one pause reason cannot resume a still-suspended heartbeat.

### 1. `src/suspension.rs`

Keep hidden, frozen and cached reasons independent; final closure permanently refuses scheduling.

Create the file and type:

```rust
#[derive(Clone, Copy)]
pub enum Reason {
    Hidden,
    Frozen,
    Cached,
}

#[derive(Default)]
pub struct Suspension {
    hidden: bool,
    frozen: bool,
    cached: bool,
    closed: bool,
}

impl Suspension {
    pub fn new(hidden: bool) -> Self {
        Self {
            hidden,
            ..Self::default()
        }
    }

    // Each browser event clears only its own reason; scheduling resumes after every remaining reason has cleared.
    pub fn change(&mut self, reason: Reason, paused: bool) -> bool {
        match reason {
            Reason::Hidden => self.hidden = paused,
            Reason::Frozen => self.frozen = paused,
            Reason::Cached => self.cached = paused,
        }
        self.permits()
    }

    pub fn permits(&self) -> bool {
        !self.hidden && !self.frozen && !self.cached && !self.closed
    }

    pub fn close(&mut self) {
        self.closed = true;
    }
}
```

### 2. `src/suspension_tests.rs`

Optional authoring fixture. Check overlapping reasons, initial hidden state, duplicate delivery and final closure that cannot resume.

Create the file and type:

```rust
use crate::suspension::{
    Reason::{Cached, Frozen, Hidden},
    Suspension,
};

#[test]
fn every_remaining_reason_prevents_resumption() {
    for order in [
        [Hidden, Frozen, Cached],
        [Cached, Hidden, Frozen],
        [Frozen, Cached, Hidden],
    ] {
        let mut state = Suspension::new(false);
        assert!(state.permits());
        for reason in order {
            assert!(!state.change(reason, true));
        }
        assert!(!state.change(order[0], false));
        assert!(!state.change(order[1], false));
        assert!(state.change(order[2], false));
    }
}

#[test]
fn initial_hidden_and_duplicate_events_remain_paused() {
    let mut state = Suspension::new(true);
    assert!(!state.permits());
    assert!(!state.change(Frozen, true));
    assert!(!state.change(Frozen, true));
    assert!(!state.change(Frozen, false));
    assert!(!state.change(Frozen, false));
    assert!(state.change(Hidden, false));
}

#[test]
fn final_closure_cannot_be_resumed() {
    let mut state = Suspension::new(true);
    state.change(Frozen, true);
    state.change(Cached, true);
    state.close();
    state.close();
    for reason in [Hidden, Frozen, Cached] {
        assert!(!state.change(reason, false));
    }
    assert!(!state.permits());
}
```

### 3. `src/lib.rs`

Register the pure policy and its native checks.

**Locate this existing block:**

```rust
mod shortcuts_tests;
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod shortcuts_tests;
pub mod suspension;
#[cfg(test)]
mod suspension_tests;
#[cfg(test)]
```

## <a id="checkpoint-34gbba-browser"></a>6. Connect visibility and freezing to heartbeat scheduling

**What this code does:** Pause the actual diagnostic timer while any browser suspension reason remains.

### 1. `src/report_lifecycle.rs`

Use the scheduling policy in the existing metadata owner. Store pause reasons beside the owned listeners and allocation token. Keep replacement identity independent of scheduling state. Read or update scheduling state without retaining a borrow while the timer changes. Read the initial hidden state and retain Document for later visibility observations. Pause cached scheduling; close the policy permanently before final cleanup is deferred. On page return, clear only the cached reason and refresh actual visibility. Observe Document visibility and freezing, then synchronize one timer after updating all affected reasons. Bind all three Document events; partial registration still cleans up through the listener owner.

**Locate this existing block:**

```rust
use crate::{browser_report as report, listeners::Listeners};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

thread_local! { static ACTIVE: RefCell<Option<(Rc<()>, Listeners)>> = const { RefCell::new(None) }; }

fn current(token: &Rc<()>) -> bool {
    ACTIVE.with(|slot| {
        slot.borrow()
            .as_ref()
            .is_some_and(|(id, _)| Rc::ptr_eq(id, token))
    })
}

#[wasm_bindgen::prelude::wasm_bindgen(js_name = stop_report_lifecycle)]
pub fn stop() -> bool {
    let owner = ACTIVE.with(|slot| slot.borrow_mut().take());
    let active = owner.is_some();
    report::stop_periodic();
    drop(owner);
    active
}

#[wasm_bindgen::prelude::wasm_bindgen(js_name = install_report_lifecycle)]
pub fn install() -> Result<(), JsValue> {
    stop();
    let window = web_sys::window().ok_or("No browser window")?;
    let token = Rc::new(());
    let guarded = Rc::clone(&token);
    let callback =
        Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
            if !current(&guarded) {
                return;
            }
            match event.type_().as_str() {
                "pagehide" => {
                    let cached = event
                        .dyn_ref::<web_sys::PageTransitionEvent>()
                        .is_some_and(|event| event.persisted());
                    let _ = report::observe(
                        "lifecycle",
                        if cached {
                            "pagehide cached"
                        } else {
                            "pagehide final"
                        },
                    );
                    report::stop_periodic();
                    if !cached {
                        let _ = report::close_report();
                        let token = Rc::clone(&guarded);
                        wasm_bindgen_futures::spawn_local(async move {
                            if current(&token) {
                                stop();
                            }
                        });
                    }
                }
                "pageshow" => {
                    let _ = report::observe("lifecycle", "pageshow");
                    if let Err(error) = report::start_periodic() {
                        let _ = report::observe(
                            "diagnostic",
                            &format!("Heartbeat unavailable: {error:?}"),
                        );
                    }
                }
                _ => {}
            }
        });
    let mut listeners = Listeners::new(callback);
    listeners.listen(window.as_ref(), "pagehide")?;
    listeners.listen(window.as_ref(), "pageshow")?;
    ACTIVE.with(|slot| slot.replace(Some((token, listeners))));
    Ok(())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{
    browser_report as report,
    listeners::Listeners,
    suspension::{Reason, Suspension},
};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

thread_local! { static ACTIVE: RefCell<Option<(Rc<()>, Listeners, Suspension)>> = const { RefCell::new(None) }; }

fn current(token: &Rc<()>) -> bool {
    ACTIVE.with(|slot| {
        slot.borrow()
            .as_ref()
            .is_some_and(|(id, _, _)| Rc::ptr_eq(id, token))
    })
}

pub fn permits() -> bool {
    ACTIVE.with(|slot| {
        slot.borrow()
            .as_ref()
            .is_some_and(|(_, _, state)| state.permits())
    })
}

fn change(reason: Reason, paused: bool) {
    ACTIVE.with(|slot| {
        if let Some((_, _, state)) = slot.borrow_mut().as_mut() {
            state.change(reason, paused);
        }
    });
}

#[wasm_bindgen::prelude::wasm_bindgen(js_name = stop_report_lifecycle)]

pub fn stop() -> bool {
    let owner = ACTIVE.with(|slot| slot.borrow_mut().take());
    let active = owner.is_some();
    report::stop_periodic();
    drop(owner);
    active
}

#[wasm_bindgen::prelude::wasm_bindgen(js_name = install_report_lifecycle)]
pub fn install() -> Result<(), JsValue> {
    stop();
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let hidden = document.hidden();
    let observed = document.clone();
    let token = Rc::new(());
    let guarded = Rc::clone(&token);
    let callback =
        Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
            if !current(&guarded) {
                return;
            }
            match event.type_().as_str() {
                "pagehide" => {
                    let cached = event
                        .dyn_ref::<web_sys::PageTransitionEvent>()
                        .is_some_and(|event| event.persisted());
                    let _ = report::observe(
                        "lifecycle",
                        if cached {
                            "pagehide cached"
                        } else {
                            "pagehide final"
                        },
                    );
                    change(Reason::Cached, true);
                    report::stop_periodic();
                    if !cached {
                        ACTIVE.with(|slot| {
                            if let Some((_, _, state)) = slot.borrow_mut().as_mut() {
                                state.close();
                            }
                        });
                        let _ = report::close_report();
                        let token = Rc::clone(&guarded);
                        wasm_bindgen_futures::spawn_local(async move {
                            if current(&token) {
                                stop();
                            }
                        });
                    }
                }
                "pageshow" => {
                    let _ = report::observe("lifecycle", "pageshow");
                    change(Reason::Cached, false);
                    change(Reason::Hidden, observed.hidden());
                }
                "visibilitychange" => {
                    change(Reason::Hidden, observed.hidden());
                    let _ = report::observe(
                        "lifecycle",
                        if observed.hidden() {
                            "hidden"
                        } else {
                            "visible"
                        },
                    );
                }
                "freeze" | "resume" => {
                    change(Reason::Frozen, event.type_() == "freeze");
                    change(Reason::Hidden, observed.hidden());
                    let _ = report::observe("lifecycle", &event.type_());
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
    ACTIVE.with(|slot| {
        slot.replace(Some((
            token,
            listeners,
            Suspension::new(hidden),
        )))
    });
    Ok(())
```

### 2. `src/browser_report.rs`

Refuse suspended scheduling and retain an existing timer, so repeated resume events cannot restart or duplicate it.

**Locate this existing block:**

```rust
pub fn start_periodic() -> Result<(), JsValue> {
    stop_periodic();
    let timer = crate::timer::Timer::every(15000, || {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn start_periodic() -> Result<(), JsValue> {
    if !crate::report_lifecycle::permits() {
        stop_periodic();
        return Ok(());
    }
    if BEAT.with(|slot| slot.borrow().is_some()) {
        return Ok(());
    }
    let timer = crate::timer::Timer::every(15000, || {
```

## <a id="checkpoint-34gbc-authority"></a>7. Give pending startup a revocable ticket

**What this code does:** Prevent a waiting task from regaining startup permission after closure.

### 1. `src/startup.rs`

Share startup permission between the waiting task and its close callback.

Create the file and type:

```rust
use std::{cell::Cell, rc::Rc};

#[derive(Clone)]
pub struct Startup(Rc<Cell<bool>>);

impl Default for Startup {
    fn default() -> Self {
        Self(Rc::new(Cell::new(true)))
    }
}

impl Startup {
    // Cloned tickets share one revocation flag, so a delayed result cannot regain permission after the page closes.
    pub fn permits(&self) -> bool {
        self.0.get()
    }

    pub fn revoke(&self) {
        self.0.set(false);
    }
}
```

### 2. `src/startup_tests.rs`

Optional authoring fixture. Check delayed tickets, repeated closure and an independent replacement owner.

Create the file and type:

```rust
use crate::startup::Startup;

#[test]
fn close_revokes_every_waiting_ticket() {
    let owner = Startup::default();
    let adapter = owner.clone();
    let device = owner.clone();
    assert!(adapter.permits());
    assert!(device.permits());
    owner.revoke();
    assert!(!adapter.permits());
    assert!(!device.permits());
    assert!(!owner.clone().permits());
}

#[test]
fn repeated_revocation_never_restores_permission() {
    let owner = Startup::default();
    let waiting = owner.clone();
    waiting.revoke();
    owner.revoke();
    assert!(!owner.permits());
    assert!(!waiting.permits());
}

#[test]
fn a_replacement_has_its_own_permission() {
    let previous = Startup::default();
    let replacement = Startup::default();
    previous.revoke();
    assert!(!previous.permits());
    assert!(replacement.permits());
    replacement.revoke();
    assert!(!replacement.permits());
}
```

### 3. `src/lib.rs`

Register startup authority and its native checks.

**Locate this existing block:**

```rust
mod shortcuts_tests;
pub mod suspension;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod shortcuts_tests;
pub mod startup;
#[cfg(test)]
mod startup_tests;
pub mod suspension;
```

## <a id="checkpoint-34gbca-startup"></a>8. Refuse GPU results after final page exit

**What this code does:** Stop delayed adapter or device delivery from installing a renderer after final exit.

### 1. `src/browser_startup.rs`

Own one temporary page-exit binding across GPU awaits, independent of optional diagnostic listeners.

Create the file and type:

```rust
use crate::{listeners::Listeners, startup::Startup};
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

pub struct Guard {
    authority: Startup,
    _listeners: Listeners,
}

impl Guard {
    pub fn new() -> Result<Self, JsValue> {
        let window = web_sys::window().ok_or("No browser window")?;
        let authority = Startup::default();
        let closing = authority.clone();
        let callback =
            Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
                let cached = event
                    .dyn_ref::<web_sys::PageTransitionEvent>()
                    .is_some_and(|event| event.persisted());
                if !cached {
                    closing.revoke();
                    let _ = crate::browser_report::close_report();
                }
            });
        let mut listeners = Listeners::new(callback);
        listeners.listen(window.as_ref(), "pagehide")?;
        Ok(Self {
            authority,
            _listeners: listeners,
        })
    }

    pub fn permits(&self) -> bool {
        self.authority.permits()
    }
}
```

### 2. `src/lib.rs`

Register the browser startup guard.

**Locate this existing block:**

```rust
#[cfg(target_arch = "wasm32")]
mod listeners;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(target_arch = "wasm32")]
mod browser_startup;
#[cfg(target_arch = "wasm32")]
mod listeners;
```

### 3. `src/browser.rs`

Keep the guard alive from startup until the runtime owns the drawing callbacks. After the adapter await, refuse a revoked result before requesting a device or interpreting an error. After the device await, destroy a late device and return quietly before configuring the surface or creating resources.

**Locate this existing block:**

```rust
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
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    crate::browser_report::start()?;
    let startup = crate::browser_startup::Guard::new()?;
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
        .await;
    if !startup.permits() {
        return Ok(());
    }
    let adapter = adapter.map_err(|error| JsValue::from_str(&error.to_string()))?;
    let device = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await;
    if !startup.permits() {
        if let Ok((device, _)) = device {
            device.destroy();
        }
        return Ok(());
    }
    let (device, queue) =
        device.map_err(|error| JsValue::from_str(&error.to_string()))?;
    let fault = crate::gpu_fault::Fault::default();
```

## Build & Run

![Recorded visual output for Lesson 4.4 (34gbca-startup)](../../../screenshots/journey/34gbca-startup-browser.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 4.4
npm --prefix ../session_tests run course -- save 4.4
```

Continue with [4.5 · Measured phases and conservative GPU recovery](../../consolidation/34gc-messages.md).
