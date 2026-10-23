# 4.1 · Browser callbacks with a real lifetime

**Goal:** Distinguish a reusable cached page from a final exit so Back navigation cannot restore an already-disposed viewer. · **Steps:** 3 · **Source:** `src/lib.rs,src/selftest/lifecycle.rs,src/engine/gpu/device.rs`

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

## <a id="checkpoint-33-owner"></a>1. Own browser listeners instead of forgetting callbacks

**What this code does:** Build a browser listener owner that detaches every registered event before freeing the Rust callback and its captured data.

### 1. `src/listeners.rs`

Own one Rust callback and every successful browser registration; detach handlers before its captured values drop. The debug probe exercises real EventTargets.

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

Register browser-only lifetime ownership without changing native rendering.

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

**What this code does:** Bind real viewer input to owned callbacks and cancel pending file/source authority when the page hides.

### 1. `src/browser_runtime.rs`

Keep the live owner in one runtime slot; Drop cancels read/fetch authority before listeners and captured editor/GPU owners are released.

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

Register browser runtime ownership beside the listener owner.

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

Retain cancellation handles outside the moving callback. Defer page-exit disposal until the active callback has returned. Transfer the event callback into the owner before attaching its targets. Register all eighteen live bindings through the owner; retain it in the runtime instead of forgetting the Rust callback.

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

**What this code does:** Distinguish a reusable cached page from a final exit so Back navigation cannot restore an already-disposed viewer.

### 1. `Cargo.toml`

Enable the browser event type that tells us whether the page may return from its cache.

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

Cancel an unfinished drag, retain the cached page’s live owner, and dispose only a final page exit.

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

![Recorded visual output for Lesson 4.1 (33b-cache)](../../../screenshots/journey/33b-cache-browser.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 4.1
npm --prefix ../session_tests run course -- save 4.1
```

Continue with [4.2 · First failure and independent diagnostic reports](../../consolidation/34-fault.md).
