# 12.3 · Bounded scheduling, exact replies and cancellation

Run only a few source reads at once and accept each exact revision-matched interval once.

## Ownership and data flow

A FIFO gate grants a permit whose lifetime releases one slot; waiting and active work have explicit cancellation ownership. Validate HTTP status, Content-Range, byte counts and revisions before decoding a page. Bound chunk accumulation and stalled-read duration rather than trusting a declared length. A current request owns the precise point/ID query and completion; runtime closure revokes queued and in-flight authority so a late body cannot revive a closed source.

## The maintained implementation

app/range_gate.rs RangeGate limits maintained range reads to three and closes waiting work; app/fetch.rs owns request/reply and body bounds. app/loader.rs owns load/generation lifetime, and app/cloud_query.rs consumes exact original point intervals. The course permit/request types stage this boundary; production signatures remain authoritative.

All production paths above are relative to `session_viewer/src`. The implementation below preserves the runnable teaching sequence. Early `src/renderer.rs`, `src/browser.rs` and local WGSL are staged prerequisites; production drawing remains in `engine/gpu`, `engine/pipelines` and `src/shaders`. Later rendering lessons must converge to those maintained paths.

Production source reference: [app/manifest.rs](../../../src/app/manifest.rs), [app/session_io.rs](../../../src/app/session_io.rs), [app/stream.rs](../../../src/app/stream.rs), [app/range_gate.rs](../../../src/app/range_gate.rs), [app/fetch.rs](../../../src/app/fetch.rs), [app/loader.rs](../../../src/app/loader.rs), [app/live.rs](../../../src/app/live.rs).

## Build the capability

Apply these implementation checkpoints in order in the same `workspace/journey` project. Expand a block while typing; each retains its original complete Type section, including its snippet includes and locate/replace context. Intermediate IDs remain recoverable checkpoints within this lesson.

<a id="checkpoint-66za-own-original-range-read-permits"></a>

<details>
<summary>Implementation 66za-own-original-range-read-permits: Own original range-read permits</summary>

Write the per-scene gate, queued turn, waiting-read and permit types.

Data flow: queued range read → permit → browser request → release.

### 1. `src/range_gate.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/66za-own-original-range-read-permits-1.rs"
```

### 2. `src/lib.rs`

Register the original browser range-read gate.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod stream;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/66za-own-original-range-read-permits-2.rs" skip=17
```

</details>

<a id="checkpoint-67a-queue-range-reads-in-arrival-order"></a>

<details>
<summary>Implementation 67a-queue-range-reads-in-arrival-order: Poll and cancel original queued range reads</summary>

Complete the waiting-read Future and its Drop behavior.

Data flow: queued range read → permit → browser request → release.

### 1. `src/range_gate.rs`

Extend this original owner with today’s complete helper.

<details>
<summary>Locate the existing block</summary>

```rust
        }
    }
}

/// A read waiting at the gate.
pub struct Enter {
    gate: Rc<RangeGate>,    // the gate
    turn: Option<Rc<Turn>>, // its place in the queue, once queued
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/67a-queue-range-reads-in-arrival-order-1.rs" skip=180
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/range_gate.rs`

Supply only the unchanged original validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, Waker};

/// Range reads a scene keeps in flight; the rest wait their turn.
pub const RANGE_READS: usize = 3;

/// A waiting read's place in the queue.
#[derive(Default)]
struct Turn {
    given: Cell<bool>,             // a slot was handed to it
    waker: RefCell<Option<Waker>>, // wakes its read
}

impl Turn {
    /// Wake the read waiting on this turn.
    fn wake(&self) {
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }
}

/// Lets a few reads run at once, first come first served; a closed gate turns waiting reads away.
pub struct RangeGate {
    limit: usize,                       // slots
    running: Cell<usize>,               // slots taken
    queue: RefCell<VecDeque<Rc<Turn>>>, // waiting reads, oldest first
    closed: Cell<bool>,                 // its scene is gone
}

impl RangeGate {
    /// A gate with `limit` slots.
    pub fn new(limit: usize) -> Rc<Self> {
        Rc::new(Self {
            limit: limit.max(1),
            running: Cell::new(0),
            queue: RefCell::new(VecDeque::new()),
            closed: Cell::new(false),
        })
    }

    /// Wait for a slot; None once the gate closed.
    pub fn enter(self: &Rc<Self>) -> Enter {
        Enter {
            gate: self.clone(),
            turn: None,
        }
    }

    /// Turn every waiting read away.
    pub fn close(&self) {
        self.closed.set(true);

        for turn in self.queue.take() {
            turn.wake();
        }
    }

    /// The gate's scene is gone.
    pub fn is_closed(&self) -> bool {
        self.closed.get()
    }

    /// Reads running and waiting.
    pub fn load(&self) -> (usize, usize) {
        (self.running.get(), self.queue.borrow().len())
    }

    /// A slot came free: hand it to the oldest waiting read.
    fn release(&self) {
        let next = self.queue.borrow_mut().pop_front();

        match next {
            Some(turn) => {
                turn.given.set(true);
                turn.wake();
            }
            None => self.running.set(self.running.get().saturating_sub(1)),
        }
    }
}

/// A read waiting at the gate.
pub struct Enter {
    gate: Rc<RangeGate>,    // the gate
    turn: Option<Rc<Turn>>, // its place in the queue, once queued
}

impl Future for Enter {
    type Output = Option<Permit>;

    /// A slot when one is free and no older read waits.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let gate = this.gate.clone();

        if gate.closed.get() {
            this.turn = None;
            return Poll::Ready(None);
        }

        match &this.turn {
            None if gate.running.get() < gate.limit && gate.queue.borrow().is_empty() => {
                gate.running.set(gate.running.get() + 1);
                Poll::Ready(Some(Permit { gate }))
            }
            None => {
                let turn = Rc::new(Turn::default());
                *turn.waker.borrow_mut() = Some(cx.waker().clone());
                gate.queue.borrow_mut().push_back(turn.clone());
                this.turn = Some(turn);
                Poll::Pending
            }
            Some(turn) if turn.given.get() => {
                this.turn = None;
                Poll::Ready(Some(Permit { gate }))
            }
            Some(turn) => {
                *turn.waker.borrow_mut() = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

impl Drop for Enter {
    /// A read dropped while waiting leaves the queue, or passes on a slot it was given.
    fn drop(&mut self) {
        let Some(turn) = self.turn.take() else {
            return;
        };

        if turn.given.get() {
            self.gate.release();
        } else {
            self.gate
                .queue
                .borrow_mut()
                .retain(|queued| !Rc::ptr_eq(queued, &turn));
        }
    }
}

/// A slot at the gate, freed when dropped.
pub struct Permit {
    gate: Rc<RangeGate>, // the gate it came from
}

impl Drop for Permit {
    /// Free the slot.
    fn drop(&mut self) {
        self.gate.release();
    }
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/67a-queue-range-reads-in-arrival-order-2.rs" skip=4322
```

</details>

</details>

<a id="checkpoint-67b-validate-exact-http-range-replies"></a>

<details>
<summary>Implementation 67b-validate-exact-http-range-replies: Validate exact HTTP range replies</summary>

Validate exact HTTP range replies.

Data flow: requested interval → HTTP206 → matching Content-Range → accepted bytes.

### 1. `src/range_rules.rs`

Validate the source interval before issuing or accepting a read.

Create the file and type:

```rust
--8<-- "journey/code/67b-validate-exact-http-range-replies-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod range_gate;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/67b-validate-exact-http-range-replies-02.rs" skip=21
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/range_rules.rs`

Copy exact-response, EOF-probe and overflow regression fixtures.

<details>
<summary>Locate the existing block</summary>

```text
    Ok(total)
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/67b-validate-exact-http-range-replies-03.rs" skip=16
```

</details>

</details>

<a id="checkpoint-67c-retain-original-point-page-values"></a>

<details>
<summary>Implementation 67c-retain-original-point-page-values: Retain original point page values</summary>

Retain original point page values.

Data flow: original bytes → precise coordinates and IDs → separate display floats.

### 1. `src/range_page.rs`

Retain original f64 pages and stable IDs separately from derived display values.

Create the file and type:

```rust
--8<-- "journey/code/67c-retain-original-point-page-values-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod range_rules;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/67c-retain-original-point-page-values-02.rs" skip=22
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/range_page.rs`

Copy source precision, high-ID, incomplete page and invalid-double fixtures.

<details>
<summary>Locate the existing block</summary>

```text
    pub fn bytes(&self)->usize {self.coordinates.capacity()*8+self.display.capacity()*4+self.ids.capacity()*4}
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/67c-retain-original-point-page-values-03.rs" skip=113
```

</details>

</details>

<a id="checkpoint-67d-own-cancellable-range-requests"></a>

<details>
<summary>Implementation 67d-own-cancellable-range-requests: Own cancellable range requests</summary>

Own cancellable range requests.

Data flow: request owner → retained abort handle → cancellation → request release.

### 1. `Cargo.toml`

Enable browser CORS request configuration without changing package versions.

<details>
<summary>Locate the existing block</summary>

```toml
[package]
name = "viewer_journey"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib", "rlib"]

[workspace]

[dependencies]
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["console", "Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "ErrorEvent", "PromiseRejectionEvent", "Location", "History", "UrlSearchParams", "Navigator", "Storage", "Performance", "PerformanceResourceTiming", "PerformanceEntry", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "RequestCache", "EventSource", "MessageEvent", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
console_error_panic_hook = "=0.1.7"
prost = "=0.14.4"
sha2 = "=0.10.8"
uuid = { version = "=1.26.1", features = ["v4", "js"] }
js-sys = "=0.3.105"
wasm-bindgen-futures = "=0.4.78"
wgpu = "=29.0.4"
session_rust = { path = "../../../session_rust", default-features = false }
getrandom = { version = "=0.2.17", features = ["js"] }

egui = { version = "=0.34.3", default-features = false }
egui-wgpu = { version = "=0.34.3", default-features = false }

serde = { version = "1.0", features = ["derive"] }
serde_json = { version = "1.0", features = ["float_roundtrip"] }

glyphon = "=0.11.0"
anyhow = "=1.0.104"
ttf-parser = {version="=0.25.1",default-features=false}

serde_yaml_ng = "=0.10.0"
toml = { version = "=0.8.23", optional = true }

[features]
json-sessions = []
toml-manifests = ["dep:toml"]

[dev-dependencies]
pollster = "=0.4.0"
log = "=0.4.34"
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```toml
--8<-- "journey/code/67d-own-cancellable-range-requests-cargo.toml"
```

### 2. `src/range_fetch.rs`

Own active browser request handles and explicit cancellation.

Create the file and type:

```rust
--8<-- "journey/code/67d-own-cancellable-range-requests-01.rs"
```

### 3. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod range_page;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/67d-own-cancellable-range-requests-02.rs" skip=21
```

</details>

<a id="checkpoint-67e-time-out-stalled-source-reads"></a>

<details>
<summary>Implementation 67e-time-out-stalled-source-reads: Time out stalled source reads</summary>

Time out stalled source reads.

Data flow: received chunk → fresh deadline → stall abort → timer cleanup.

### 1. `src/range_fetch.rs`

Connect the owned stall timer, progress reset and request cleanup.

<details>
<summary>Locate the existing block</summary>

```rust
    callback:Closure<dyn FnMut()>,
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/67e-time-out-stalled-source-reads-01.rs" skip=37
```

</details>

<a id="checkpoint-67f-bound-incoming-range-body-chunks"></a>

<details>
<summary>Implementation 67f-bound-incoming-range-body-chunks: Bound incoming range body chunks</summary>

Bound incoming range body chunks.

Data flow: incoming chunk → remaining budget → retained original bytes.

### 1. `src/range_fetch.rs`

Read bounded body chunks while retaining the checked source request owner.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn retryable(error:&str)->bool {error.starts_with("network error")}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/67f-bound-incoming-range-body-chunks-01.rs" skip=73
```

</details>

<a id="checkpoint-67g-request-and-accept-one-source-interval"></a>

<details>
<summary>Implementation 67g-request-and-accept-one-source-interval: Request and accept one source interval</summary>

Request and accept one source interval.

Data flow: checked interval → fetch → bounded body → exact response acceptance.

### 1. `src/range_fetch.rs`

Connect HTTP range headers and exact response acceptance.

<details>
<summary>Locate the existing block</summary>

```rust
}

async fn body(reader:&web_sys::ReadableStreamDefaultReader,limit:u64,read:&mut Read)->Result<Vec<u8>,String> {
    let mut bytes=Vec::new();loop {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/67g-request-and-accept-one-source-interval-01.rs"
```

</details>

<a id="checkpoint-67h-keep-range-revisions-and-background-results"></a>

<details>
<summary>Implementation 67h-keep-range-revisions-and-background-results: Keep range revisions and background results</summary>

Keep range revisions and background results.

Data flow: source ETag → complete range → retained result → caller ownership.

### 1. `src/range_fetch.rs`

Validate source revisions and provide background range reads.

<details>
<summary>Locate the existing block</summary>

```rust
    }Ok(bytes)
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/67h-keep-range-revisions-and-background-results-01.rs" skip=17
```

</details>

<a id="checkpoint-67hza-queue-original-source-range-reads"></a>

<details>
<summary>Implementation 67hza-queue-original-source-range-reads: Queue original source range reads</summary>

Write the browser range-read owner and cached metadata-window read.

Data flow: bounded HTTP read → revision and layout → source-array slice → decoded points.

### 1. `src/stream.rs`

Type today’s complete browser source readers; reuse the decoding and queue helpers already written.

<details>
<summary>Locate the existing block</summary>

```rust
}

#[cfg(test)]
mod tests {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/67hza-queue-original-source-range-reads-1.rs"
```

</details>

<a id="checkpoint-67hzb-probe-original-source-array-layouts"></a>

<details>
<summary>Implementation 67hzb-probe-original-source-array-layouts: Probe original source array layouts</summary>

Start owned asynchronous range tasks and inspect the bounded file probe.

Data flow: bounded HTTP read → revision and layout → source-array slice → decoded points.

### 1. `src/stream.rs`

Type today’s complete browser source readers; reuse the decoding and queue helpers already written.

<details>
<summary>Locate the existing block</summary>

```rust
}

#[cfg(test)]
mod tests {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/67hzb-probe-original-source-array-layouts-1.rs"
```

</details>

<a id="checkpoint-67hzc-read-original-cloud-node-tables"></a>

<details>
<summary>Implementation 67hzc-read-original-cloud-node-tables: Read original cloud node tables</summary>

Read original node-table fields through the cached metadata window.

Data flow: bounded HTTP read → revision and layout → source-array slice → decoded points.

### 1. `src/stream.rs`

Type today’s complete browser source readers; reuse the decoding and queue helpers already written.

<details>
<summary>Locate the existing block</summary>

```rust
}

#[cfg(test)]
mod tests {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/67hzc-read-original-cloud-node-tables-1.rs"
```

</details>

<a id="checkpoint-67i-read-source-arrays-through-the-bounded-queue"></a>

<details>
<summary>Implementation 67i-read-source-arrays-through-the-bounded-queue: Fetch original coordinates normals and colours</summary>

Fetch bounded source-array slices through the queue already written.

Data flow: bounded HTTP read → revision and layout → source-array slice → decoded points.

### 1. `src/stream.rs`

Type today’s complete browser source readers; reuse the decoding and queue helpers already written.

<details>
<summary>Locate the existing block</summary>

```rust
}

#[cfg(test)]
mod tests {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/67i-read-source-arrays-through-the-bounded-queue-1.rs"
```

</details>

<a id="checkpoint-67j-keep-original-ranged-source-identity"></a>

<details>
<summary>Implementation 67j-keep-original-ranged-source-identity: Keep original ranged source identity</summary>

Keep original ranged source identity.

Data flow: small source header → original GUID → stable remote source identity.

### 1. `src/stream.rs`

Read original source identity independently of display pages.

<details>
<summary>Locate the existing block</summary>

```rust
    let (at, length, _) = cloud_layout(head)?;
    Some((at, length))
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/67j-keep-original-ranged-source-identity-01.rs" skip=73
```

</details>

<a id="checkpoint-67k-prepare-a-revision-bound-remote-source"></a>

<details>
<summary>Implementation 67k-prepare-a-revision-bound-remote-source: Prepare a revision bound remote source</summary>

Prepare a revision bound remote source.

Data flow: stable URL revision → checked original layout → remote source owner.

### 1. `src/range_source.rs`

Prepare a revision-bound original remote source descriptor.

Create the file and type:

```rust
--8<-- "journey/code/67k-prepare-a-revision-bound-remote-source-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(target_arch = "wasm32")]
pub mod range_fetch;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/67k-prepare-a-revision-bound-remote-source-02.rs" skip=52
```

</details>

<a id="checkpoint-67l-fetch-precise-coordinates-and-original-ids"></a>

<details>
<summary>Implementation 67l-fetch-precise-coordinates-and-original-ids: Fetch precise coordinates and original IDs</summary>

Fetch precise coordinates and original IDs.

Data flow: source row interval → coordinate and ID ranges → checked original page.

### 1. `src/range_source.rs`

Fetch matching original coordinate and ID pages under the source revision.

<details>
<summary>Locate the existing block</summary>

```rust
        Ok(Self {guid,name,url,fields,lod})
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/67l-fetch-precise-coordinates-and-original-ids-01.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/range_source.rs`

Copy actual kernel-protobuf identity, precision and page-layout regression fixtures.

<details>
<summary>Locate the existing block</summary>

```text
    }
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/67l-fetch-precise-coordinates-and-original-ids-02.rs" skip=8
```

</details>

</details>

<a id="checkpoint-67m-cancel-range-work-when-its-runtime-closes"></a>

<details>
<summary>Implementation 67m-cancel-range-work-when-its-runtime-closes: Cancel range work when its runtime closes</summary>

Cancel range work when its runtime closes.

Data flow: Close or device disposal → closed gate → aborted reads → released owners.

### 1. `src/browser_runtime.rs`

Cancel queued and active range work with its browser runtime.

<details>
<summary>Locate the existing block</summary>

```rust
impl Drop for Runtime {
    fn drop(&mut self) {
        self.request.borrow_mut().cancel();
        self.reload.borrow_mut().cancel();
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/67m-cancel-range-work-when-its-runtime-closes-01.rs"
```

### 2. `src/browser.rs`

Cancel queued and active range work with its browser runtime.

<details>
<summary>Locate the existing block</summary>

```rust
            Err(error) => { panel.result(&error); None }
            Ok(crate::commands::Command::Editor(mut action)) => {
                if matches!(action, Action::Close) { publication_control.cancel();request.borrow_mut().cancel();let _={text_fonts.clear();renderer.clear_text_fonts();renderer.set_text(Vec::new())}; }
                if shortcut.is_some() && editor.selected.is_some() && matches!(action, Action::Fit) { action = Action::FitSelected; }
                Some(action)
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/67m-cancel-range-work-when-its-runtime-closes-02.rs"
```

</details>

## Run, check and save

From `session_viewer`, after restoring any experimental edits:

```sh
npm --prefix ../session_tests run course -- dependencies 67m-cancel-range-work-when-its-runtime-closes
npm --prefix ../session_tests run course -- check 67m-cancel-range-work-when-its-runtime-closes
npm --prefix ../session_tests run course -- save 67m-cancel-range-work-when-its-runtime-closes
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running so saving rebuilds it. `check` compares your typed source with the selected endpoint without replacing your project; `save` records that source for recovery. The commands are learner instructions, not a claim that this consolidation batch has rerun GPU or browser acceptance. [Checkpoint recovery](../recovery.md) explains restoring a saved state.

## Result and experiment

The mixed archive picture remains unchanged while bounded range requests become available. Trace one queued and one active interval through Close: both lose delivery authority, permits release and no late reply can adopt a source. Inspect exact Content-Range/revision and timeout rejection paths; visible streamed pages connect in the next lesson.

Recorded screenshot paths are identified in the chapter coverage map as historical, pending fresh acceptance. No uninspected screenshot is embedded here. Source comparison, a successful build and visual behavior are separate checks.

Continue with [Charged residency and current streamed pages](68-reserve-page-bytes-before-reading.md).
