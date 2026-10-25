# 4.1 · Browser callbacks with a real lifetime

**Steps:** 3 · **Source:** `src/lib.rs` · `src/selftest/lifecycle.rs` · `src/engine/gpu/device.rs`

## <a id="checkpoint-33-owner"></a>1. Own browser listeners instead of forgetting callbacks

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="214" viewBox="0 0 1120 214" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr-4d0e3729" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arr-r-4d0e3729" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arr-g-4d0e3729" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arr-b-4d0e3729" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="grid-4d0e3729" width="20" height="20" patternUnits="userSpaceOnUse">
      <path d="M 20 0 L 0 0 0 20" fill="none" stroke="#f1f5f9" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="210" rx="8" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="2" y="2" width="1116" height="28" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="2" y1="30" x2="1118" y2="30" stroke="#111111" stroke-width="1.5"/>
  <rect x="12" y="7" width="176" height="18" rx="3" fill="#111111"/>
  <text x="100" y="19.5" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#ffffff" paint-order="normal" stroke="none">33..34 · GPU LIFECYCLE</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="518" y="20" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">RAII Listener Drop, Latching GPU Fault Trap, Heartbeat &amp; DPR=1.0 Recovery Reload</text>
  <rect x="838" y="7" width="268" height="18" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="972" y="19.5" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">OwnedListener::drop -&gt; removeEventListener</text>
  <rect x="14" y="38" width="344" height="164" rx="6" fill="url(#grid-4d0e3729)" stroke="#111111" stroke-width="1.5"/>
  <line x1="14" y1="60" x2="358" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1 · RAII OwnedListener &amp; Startup Ticket</text>
  <rect x="44" y="76" width="284" height="48" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="96" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">OwnedListener { target, name, closure }</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="114" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#2563eb">Drop detaches DOM listener automatically</text>
  <rect x="44" y="134" width="284" height="48" rx="4" fill="#fee2e2" stroke="#dc2626" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="154" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">First GPU Fault Latched (device.lost)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="172" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#111111">Stops render loop; preserves root cause</text>
  <line x1="358" y1="120" x2="384" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-4d0e3729)"/>
  <rect x="388" y="38" width="344" height="164" rx="6" fill="url(#grid-4d0e3729)" stroke="#111111" stroke-width="1.5"/>
  <line x1="388" y1="60" x2="732" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2 · 3-Run Storage Ring &amp; Heartbeat Pause Bits</text>
  <rect x="414" y="76" width="292" height="50" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="96" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">localStorage: [Run n-2, Run n-1, Run n]</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="114" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#16a34a">Hidden / Frozen tabs pause heartbeat timer</text>
  <rect x="414" y="134" width="292" height="48" rx="4" fill="#fef9c3" stroke="#d97706" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="154" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Phased Timing: Startup / Decode / Upload</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="172" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#2563eb">1-Shot Conservative DPR=1.0 Recovery</text>
  <line x1="732" y1="120" x2="758" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-4d0e3729)"/>
  <rect x="762" y="38" width="344" height="164" rx="6" fill="url(#grid-4d0e3729)" stroke="#111111" stroke-width="1.75"/>
  <line x1="762" y1="60" x2="1106" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3 · Retained Document Survives GPU Recovery</text>
  <rect x="806" y="68" width="256" height="122" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="826" y="82" width="216" height="44" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="108" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">Document Outside GPU Runtime</text>
  <rect x="826" y="136" width="216" height="40" rx="4" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="160" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Re-Uploaded at DPR = 1.0 ✓</text>
</svg>
</div>

### 1. `src/listeners.rs`

Create the file and type:

```rust
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

pub struct Listeners {
    callback: Closure<dyn FnMut(web_sys::Event)>,
    bindings: Vec<(web_sys::EventTarget, &'static str)>,
}

impl Listeners {
    pub fn new(callback: Closure<dyn FnMut(web_sys::Event)>) -> Self {
        Self {
            callback,
            bindings: Vec::new(),
        }
    }

    pub fn listen(
        &mut self,
        target: &web_sys::EventTarget,
        name: &'static str,
    ) -> Result<(), JsValue> {
        let options = web_sys::AddEventListenerOptions::new();
        options.set_passive(false);
        target.add_event_listener_with_callback_and_add_event_listener_options(
            name,
            self.callback.as_ref().unchecked_ref(),
            &options,
        )?;
        self.bindings.push((target.clone(), name));
        Ok(())
    }
}

impl Drop for Listeners {
    fn drop(&mut self) {
        for (target, name) in &self.bindings {
            let _ = target.remove_event_listener_with_callback(
                name,
                self.callback.as_ref().unchecked_ref(),
            );
        }
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn listener_probe() -> Result<Vec<u32>, JsValue> {
    use std::{cell::Cell, rc::Rc};
    let first = web_sys::EventTarget::new()?;
    let second = web_sys::EventTarget::new()?;
    let hits = Rc::new(Cell::new(0));
    let counted = Rc::clone(&hits);
    let captured = Rc::new(());
    let observer = Rc::downgrade(&captured);
    let callback = Closure::new(move |_: web_sys::Event| {
        assert_eq!(Rc::strong_count(&captured), 1);
        counted.set(counted.get() + 1);
    });
    let mut listeners = Listeners::new(callback);
    listeners.listen(&first, "proof")?;
    listeners.listen(&second, "proof")?;
    let event = web_sys::Event::new("proof")?;
    first.dispatch_event(&event)?;
    second.dispatch_event(&event)?;
    let before = hits.get();
    drop(listeners);
    first.dispatch_event(&event)?;
    second.dispatch_event(&event)?;
    Ok(vec![
        before,
        hits.get(),
        observer.strong_count() as u32,
    ])
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
mod listeners;
#[cfg(target_arch = "wasm32")]
mod panel;
```

## <a id="checkpoint-33a-runtime"></a>2. Dispose the viewer without leaving pending work alive

### 1. `src/browser_runtime.rs`

Create the file and type:

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
    ACTIVE.with(|slot| {
        slot.replace(Some(Runtime {
            _listeners: listeners,
            request,
            reload,
        }))
    });
}

pub fn stop() -> bool {
    ACTIVE
        .with(|slot| slot.borrow_mut().take())
        .is_some()
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn runtime_running() -> bool {
    ACTIVE.with(|slot| slot.borrow().is_some())
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(target_arch = "wasm32")]
mod listeners;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(target_arch = "wasm32")]
mod browser_runtime;
#[cfg(target_arch = "wasm32")]
mod listeners;
```

### 3. `src/browser.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        "keyup",
        "blur",
    ] {
        canvas
            .add_event_listener_with_callback(name, update.as_ref().unchecked_ref())?;
    }
    let options = web_sys::AddEventListenerOptions::new();
    options.set_passive(false);
    canvas.add_event_listener_with_callback_and_add_event_listener_options(
        "wheel",
        update.as_ref().unchecked_ref(),
        &options,
    )?;
    window
        .add_event_listener_with_callback("resize", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("blur", update.as_ref().unchecked_ref())?;
    document
        .add_event_listener_with_callback("change", update.as_ref().unchecked_ref())?;
    document
        .add_event_listener_with_callback("cancel", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback(
        "viewer-file",
        update.as_ref().unchecked_ref(),
    )?;
    window.add_event_listener_with_callback(
        "viewer-file-error",
        update.as_ref().unchecked_ref(),
    )?;
    window.add_event_listener_with_callback(
        "viewer-reload",
        update.as_ref().unchecked_ref(),
    )?;
    // Both event sources retain this one callback for the lifetime of the page.
    update.forget();
    report(
```

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
    report(
```

**Locate this block:**

```rust
            }
        },
    );
    for name in [
        "pointerdown",
        "pointermove",
```

Replace it with:

```rust
            }
        },
    );
    let mut listeners = crate::listeners::Listeners::new(update);
    for name in [
        "pointerdown",
        "pointermove",
```

**Locate this block:**

```rust
        std::rc::Rc::new(std::cell::RefCell::new(None));
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(
        move |event: web_sys::Event| {
            let shortcut = event
                .dyn_ref::<web_sys::KeyboardEvent>()
                .and_then(|key| panel.shortcut(key));
```

Replace it with:

```rust
        std::rc::Rc::new(std::cell::RefCell::new(None));
    let owned_request = std::rc::Rc::clone(&request);
    let owned_reload = std::rc::Rc::clone(&reload);
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(
        move |event: web_sys::Event| {
            if event.type_() == "pagehide" {
                wasm_bindgen_futures::spawn_local(async {
                    crate::browser_runtime::stop();
                });
                return;
            }
            let shortcut = event
                .dyn_ref::<web_sys::KeyboardEvent>()
                .and_then(|key| panel.shortcut(key));
```

## <a id="checkpoint-33b-cache"></a>3. Keep a cached viewer ready for Back navigation

### 1. `Cargo.toml`

**Locate this existing block:**

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
console_error_panic_hook = "=0.1.7"
```

Edit the existing block to match this code; keep unchanged lines:

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
console_error_panic_hook = "=0.1.7"
```

### 2. `src/browser.rs`

**Locate this existing block:**

```rust
            if event.type_() == "pagehide" {
                wasm_bindgen_futures::spawn_local(async {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            if event.type_() == "pagehide" {
                gesture.cancel();
                if event
                    .dyn_ref::<web_sys::PageTransitionEvent>()
                    .is_some_and(|event| event.persisted())
                {
                    return;
                }
                wasm_bindgen_futures::spawn_local(async {
```

## Build & Run

![Recorded visual output for Lesson 4.1 (33b-cache)](../../screenshots/journey/33b-cache-browser.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 4.1
npm --prefix ../session_tests run course -- save 4.1
```

Continue with [4.2 · First failure and independent diagnostic reports](34-fault.md).
