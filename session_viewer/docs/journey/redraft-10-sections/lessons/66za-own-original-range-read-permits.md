# 12.3 · Bounded scheduling, exact replies and cancellation

**Section 7: Out-of-Core HTTP Range Streaming, Shared Instances, and Drawing Sheets** · **Lesson 12.3** (Redraft `7.3`) · **17 checkpoints (`66za-own-original-range-read-permits` → `67m-cancel-range-work-when-its-runtime-closes`)**

> **Learning Goal:** Write the per-scene gate, queued turn, waiting-read and permit types.

## Visual WebGPU & Architecture Schematic

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1px solid #cbd5e1;border-radius:10px;background:#f8fafc;">
<svg viewBox="0 0 840 240" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#475569"/>
    </marker>
    <marker id="arrow-blue" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <marker id="arrow-green" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#059669"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="232" rx="8" fill="#ffffff" stroke="#e2e8f0" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#0f172a">Section 7 WebGPU Streaming & Instancing: HTTP Range Requests, Shared Source Meshes & 2D Drawing Sheets</text>
  <text x="20" y="46" font-size="11.5" fill="#475569">How massive multi-gigabyte scenes stream out-of-core via HTTP 206 Partial Content and draw 1,000+ placements from 1 GPU buffer.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#e2e8f0" stroke-width="1"/>
  
    <rect x="18" y="68" width="245" height="145" rx="6" fill="#eff6ff" stroke="#93c5fd" stroke-width="1.5"/>
    <text x="140" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#1e40af">1. HTTP 206 Range Streaming (Ch 12)</text>
    <text x="28" y="108" font-size="10.5" fill="#1e3a8a">• Fetch compact Manifest + World Bounds</text>
    <text x="28" y="126" font-size="10.5" font-weight="600" fill="#1d4ed8">• Prioritize visible frustum chunks</text>
    <text x="28" y="144" font-size="10.5" fill="#1e3a8a">• Range: bytes=start-end (bounded memory)</text>
    <text x="28" y="162" font-size="10.5" fill="#1e3a8a">• Evict cold off-screen chunks under</text>
    <text x="28" y="178" font-size="10.5" fill="#1e3a8a">  memory budget without losing metadata</text>

    <line x1="263" y1="138" x2="293" y2="138" stroke="#2563eb" stroke-width="2" marker-end="url(#arrow-blue)"/>

    <rect x="295" y="68" width="265" height="145" rx="6" fill="#fff7ed" stroke="#fdba74" stroke-width="1.5"/>
    <text x="427" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#9a3412">2. GPU Hardware Instancing (Ch 13)</text>
    <text x="307" y="108" font-size="10.5" fill="#7c2d12">• 1 Shared Definition Buffer (e.g. Bolt)</text>
    <text x="307" y="126" font-size="10.5" font-weight="600" fill="#c2410c">• N Instance Rows (@builtin(instance_index)):</text>
    <text x="307" y="142" font-size="10" fill="#7c2d12">  Per-instance mat4x4 transform, tint,</text>
    <text x="307" y="157" font-size="10" fill="#7c2d12">  and distinct instance_owner_id</text>
    <text x="307" y="176" font-size="10.5" fill="#7c2d12">• draw_indexed(0..idx_cnt, 0, 0..N_inst)</text>

    <line x1="560" y1="138" x2="590" y2="138" stroke="#059669" stroke-width="2" marker-end="url(#arrow-green)"/>

    <rect x="592" y="68" width="228" height="145" rx="6" fill="#ecfdf5" stroke="#6ee7b7" stroke-width="1.5"/>
    <text x="706" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#065f46">3. 2D Drawing Sheets &amp; Blocks</text>
    <text x="604" y="108" font-size="10.5" fill="#064e3b">• Paper-space Sheet frames &amp; title</text>
    <text x="604" y="124" font-size="10.5" fill="#064e3b">  blocks (`SheetInit / SheetRows`)</text>
    <text x="604" y="144" font-size="10.5" font-weight="600" fill="#047857">• Calibrated mm/inch pen widths</text>
    <text x="604" y="162" font-size="10.5" fill="#064e3b">• Switch smoothly between 3D Model</text>
    <text x="604" y="178" font-size="10.5" fill="#064e3b">  view and 2D Sheet layouts</text>
    
</svg>
</div>

### Lesson Data Flow (`Rust` → `WebGPU` → `WGSL`)

```
queued range read → permit → browser request → release.
checked interval → fetch → bounded body → exact response acceptance.
Close or device disposal → closed gate → aborted reads → released owners.
```

- Write the per-scene gate, queued turn, waiting-read and permit types.
- Keep range revisions and background results.
- Cancel range work when its runtime closes.
- **Why it works (Why does a permit own the gate?):** The permit releases its slot when dropped. Its gate must remain alive until that release, even if the caller has already discarded its scene reference.

### Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `Browser`)

| Layer | Construct | First-Year Student Explanation |
| :--- | :--- | :--- |
| **HTTP / Fetch** | `Range: bytes=start-end` -> `206 Partial Content` | Fetches only the exact byte slice of an octree node or sheet column needed for the current camera view. |
| **Protobuf Wire** | Varint tag + length prefix scanner | Skips over multi-megabyte coordinate arrays by reading their byte range offsets without allocating their contents. |
| **Rust (Residency)** | `budget.reserve(bytes)?` -> LRU eviction | Guarantees resident + in-flight pages never exceed the memory cap; coarse root pages stay pinned so the scene is never blank. |
| **GPU Instancing** | Shared definition buffer + per-instance record | Draws N placed copies of a complex CAD component using 1 shared vertex/index allocation and N compact instance records. |

**Production source (`session_viewer/src`):** [app/manifest.rs](../../../../src/app/manifest.rs), [app/session_io.rs](../../../../src/app/session_io.rs), [app/stream.rs](../../../../src/app/stream.rs), [app/range_gate.rs](../../../../src/app/range_gate.rs), [app/fetch.rs](../../../../src/app/fetch.rs), [app/loader.rs](../../../../src/app/loader.rs), [app/live.rs](../../../../src/app/live.rs).

## Implementation Checkpoints

Apply these checkpoints in order inside `workspace/journey`. Expand each block to view the Rust/WGSL/HTML edits:

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

## Run, Check, and Save

From `session_viewer`:

```sh
npm --prefix ../session_tests run course -- dependencies 67m-cancel-range-work-when-its-runtime-closes
npm --prefix ../session_tests run course -- check 67m-cancel-range-work-when-its-runtime-closes
npm --prefix ../session_tests run course -- save 67m-cancel-range-work-when-its-runtime-closes
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Use [checkpoint recovery](../../recovery.md) to restore any saved checkpoint.

## Expected Visual Result & Experiment

- **Try this:** Close with queued and active reads; keep the mixed archive picture unchanged until streamed page display connects.

Continue with [Charged residency and current streamed pages](68-reserve-page-bytes-before-reading.md).
