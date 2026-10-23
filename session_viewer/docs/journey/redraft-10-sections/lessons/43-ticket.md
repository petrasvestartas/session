# 6.2 · Accept a hit only for its current view

**Goal:** Reject results from an old view or interrupted input. · **Steps:** 3 · **Source:** [engine/gpu/pick.rs](../../../../src/engine/gpu/pick.rs) · [app/selection.rs](../../../../src/app/selection.rs) · [engine/gpu/faces.rs](../../../../src/engine/gpu/faces.rs) · [app/walk/mesh_topology.rs](../../../../src/app/walk/mesh_topology.rs) (`src/engine/gpu/pick.rs,src/app/selecting.rs,src/shaders/scene.wgsl`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 6.2 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/pick_gate.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/pick_view.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/pick_gate_tests.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">3 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/engine/gpu/pick.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/selecting.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Name the request and view…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Rg32Uint [R:owner, G:sub]</text>
  <rect x="624" y="50" width="152" height="92" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="624" y="50" width="50" height="30" fill="#fee2e2" stroke="#111111" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="649" y="69" text-anchor="middle" font-family="monospace" font-size="8.5" font-weight="700" fill="#dc2626">R:id=4</text>
  <rect x="674" y="50" width="52" height="30" fill="#dcfce7" stroke="#111111" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="69" text-anchor="middle" font-family="monospace" font-size="8.5" font-weight="700" fill="#16a34a">G:sub=2</text>
  <line x1="674" y1="50" x2="674" y2="142" stroke="#111111" stroke-width="1"/>
  <line x1="726" y1="50" x2="726" y2="142" stroke="#111111" stroke-width="1"/>
  <line x1="624" y1="80" x2="776" y2="80" stroke="#111111" stroke-width="1"/>
  <line x1="624" y1="111" x2="776" y2="111" stroke="#111111" stroke-width="1"/>
  <rect x="674" y="80" width="52" height="31" fill="#fef08a" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">[7, 3]</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Accept a hit only for its c…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Priority &amp; 256B Readback</text>
  <rect x="880" y="54" width="188" height="36" rx="5" fill="#fef08a" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="974" y="76" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Point &gt; Wire &gt; Face</text>
  <rect x="880" y="100" width="188" height="36" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="974" y="122" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">256B Aligned Staging</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Reject results from an ol…</text>
</svg>
</div>

## <a id="checkpoint-43-ticket"></a>1. Name the request and view that produced a hit

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 3</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/pick_gate.rs, src/pick_…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">request sequence and epoch</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">camera/viewport/density s…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/pick…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">accepted current result</text>
</svg>
</div>

**What this code does:** Name the request and view that produced a hit.

### 1. `src/pick_gate.rs`

Create checked request freshness and view signatures.

Create the file and type:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ticket {
    epoch: u64,
    request: u64,
}

#[derive(Default)]
pub struct Gate {
    epoch: u64,
    request: u64,
    latest: Option<Ticket>,
    closed: bool,
}

impl Gate {
    // A ticket names one request in one scene/view epoch; both must still match before its asynchronous result can change selection.
    pub fn issue(&mut self) -> Result<Ticket, &'static str> {
        if self.closed {
            return Err("Picking owner is closed");
        }
        let Some(request) = self.request.checked_add(1) else {
            self.close();
            return Err("Pick requests exhausted");
        };
        self.request = request;
        let ticket = Ticket {
            epoch: self.epoch,
            request,
        };
        self.latest = Some(ticket);
        Ok(ticket)
    }

    pub fn invalidate(&mut self) {
        self.latest = None;
        match self.epoch.checked_add(1) {
            Some(epoch) => self.epoch = epoch,
            None => self.close(),
        }
    }

    pub fn accept(&mut self, ticket: Ticket) -> bool {
        if self.closed || self.latest != Some(ticket) {
            return false;
        }
        self.latest = None;
        true
    }

    pub fn close(&mut self) {
        self.closed = true;
        self.latest = None;
    }
}

#[cfg(test)]
#[path = "pick_gate_tests.rs"]
mod tests;
```

### 2. `src/pick_view.rs`

Create checked request freshness and view signatures.

Create the file and type:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct View {
    transform: [u32; 16],
    size: [u32; 2],
    density: u32,
}

impl View {
    // Readback belongs to the camera matrix, physical viewport and CSS density that produced it, even when the document has not changed.
    pub fn new(
        transform: [f32; 16],
        size: [u32; 2],
        density: f32,
    ) -> Result<Self, &'static str> {
        if transform.iter().any(|n| !n.is_finite())
            || size.contains(&0)
            || !density.is_finite()
            || density <= 0.0
        {
            return Err("Invalid pick view");
        }
        Ok(Self {
            transform: transform.map(f32::to_bits),
            size,
            density: density.to_bits(),
        })
    }
}
```

### 3. `src/pick_gate_tests.rs`

Copy one-use, superseded, invalidated, closed and exhausted ticket checks.

Create the file and type:

```rust
use super::*;

#[test]
fn only_the_latest_request_in_the_current_epoch_can_be_accepted_once() {
    let mut gate = Gate::default();
    let old = gate.issue().unwrap();
    let latest = gate.issue().unwrap();
    assert!(!gate.accept(old));
    assert!(gate.accept(latest));
    assert!(!gate.accept(latest));
    let before_edit = gate.issue().unwrap();
    gate.invalidate();
    assert!(!gate.accept(before_edit));
    let after_edit = gate.issue().unwrap();
    assert_ne!(before_edit, after_edit);
    assert!(gate.accept(after_edit));
    let pending = gate.issue().unwrap();
    gate.close();
    assert!(!gate.accept(pending));
    assert!(gate.issue().is_err());
}

#[test]
fn exhausted_tickets_close_without_wrapping_to_an_old_identity() {
    let mut gate = Gate::default();
    gate.request = u64::MAX - 1;
    let last = gate.issue().unwrap();
    assert!(gate.issue().is_err());
    assert!(!gate.accept(last));
    let mut gate = Gate::default();
    gate.epoch = u64::MAX;
    let last = gate.issue().unwrap();
    gate.invalidate();
    assert!(!gate.accept(last));
    assert!(gate.issue().is_err());
}
```

### 4. `src/pick_view_tests.rs`

Copy camera, viewport, density and invalid-view checks.

Create the file and type:

```rust
use crate::pick_view::View;

#[test]
fn a_camera_viewport_or_density_change_invalidates_the_readback_view() {
    let transform = session_rust::Xform::identity()
        .m
        .map(|n| n as f32);
    let base = View::new(transform, [640, 480], 1.0).unwrap();
    assert_eq!(
        base,
        View::new(transform, [640, 480], 1.0).unwrap()
    );
    let mut orbit = transform;
    orbit[0] = 0.9;
    assert_ne!(
        base,
        View::new(orbit, [640, 480], 1.0).unwrap()
    );
    let mut pan = transform;
    pan[12] = 0.01;
    assert_ne!(
        base,
        View::new(pan, [640, 480], 1.0).unwrap()
    );
    assert_ne!(
        base,
        View::new(transform, [641, 480], 1.0).unwrap()
    );
    assert_ne!(
        base,
        View::new(transform, [640, 480], 2.0).unwrap()
    );
    let mut invalid = transform;
    invalid[3] = f32::NAN;
    assert!(View::new(invalid, [640, 480], 1.0).is_err());
    assert!(View::new(transform, [0, 480], 1.0).is_err());
    assert!(View::new(transform, [640, 480], f32::INFINITY).is_err());
}
```

### 5. `src/lib.rs`

Register the freshness component and its checks.

**Locate this existing block:**

```rust
#[cfg(test)]
mod pick_input_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod pick_input_tests;

pub mod pick_gate;
pub mod pick_view;
#[cfg(test)]
mod pick_view_tests;
```

## <a id="checkpoint-43a-flight"></a>2. Carry checked tickets through GPU mapping

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2 of 3</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/pick_gpu.rs, src/render…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">checked admission</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">map callback</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/pick…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">latest pending cursor</text>
</svg>
</div>

**What this code does:** Carry checked tickets through GPU mapping.

### 1. `src/pick_gpu.rs`

Carry and check the current request ticket through mapping.

**Locate this existing block:**

```rust
}

pub struct Picker {
    pipeline: wgpu::RenderPipeline,
    view: wgpu::Buffer,
    view_group: wgpu::BindGroup,
    crop: wgpu::Buffer,
    crop_group: wgpu::BindGroup,
    image: Option<crate::pick_image::Image>,
    active: Option<Arc<AtomicU8>>,
    pending: Option<Request>,
    alive: Arc<AtomicBool>,
    pub submitted: u64,
    pub coalesced: u64,
}

impl Drop for Picker {
    fn drop(&mut self) {
        self.alive.store(false, Ordering::Release);
    }
}

impl Picker {
    pub fn new(
        device: &wgpu::Device,
        view_layout: &wgpu::BindGroupLayout,
        model_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let crop_layout = crate::gpu_layout::uniform(device, 64);
        let pipeline = crate::pick_pipeline::mesh(
            device,
            view_layout,
            model_layout,
            &crop_layout,
        );
        let buffer = |size| {
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("pick view"),
                size,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
        };
        let view = buffer(80);
        let crop = buffer(64);
        let view_group = crate::gpu_layout::group(device, view_layout, &view);
        let crop_group = crate::gpu_layout::group(device, &crop_layout, &crop);
        Self {
            pipeline,
            view,
            view_group,
            crop,
            crop_group,
            image: None,
            active: None,
            pending: None,
            alive: Arc::new(AtomicBool::new(true)),
            submitted: 0,
            coalesced: 0,
        }
    }

    pub fn queue(&mut self, request: Request) {
        if self.pending.replace(request).is_some() {
            self.coalesced += 1;
        }
    }

    pub fn queued(&self) -> bool {
        self.pending.is_some()
    }

    pub fn cancel(&mut self) {
        self.pending = None;
        let mapped = self
            .active
            .take()
            .is_some_and(|ready| ready.load(Ordering::Acquire) <= 1);
        if let Some(image) = self.image.take() {
            if mapped {
                image.readback.unmap();
            }
        }
    }

    pub fn next(&mut self) -> Option<Request> {
        if self.active.is_some() {
            None
        } else {
            self.pending.take()
        }
    }

    pub fn submit(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        request: Request,
        meshes: &[GpuMesh],
        ink: &[&Lane],
    ) {
        assert!(
            self.active.is_none(),
            "Only one pick may be in flight"
        );
        let window = request.window;
        if self.image.as_ref().is_none_or(|image| {
            image.window.draw[2..] != window.draw[2..]
                || image.readback.size() != window.bytes()
        }) {
            self.image = Some(crate::pick_image::Image::new(device, window));
        }
        let image = self.image.as_mut().unwrap();
        image.window = window;
        let mut bytes: Vec<u8> = request
            .transform
            .iter()
            .flat_map(|n| n.to_ne_bytes())
            .collect();
        bytes.resize(80, 0);
        queue.write_buffer(&self.view, 0, &bytes);
        let bytes: Vec<u8> = window
            .crop()
            .iter()
            .flat_map(|n| n.to_ne_bytes())
            .collect();
        queue.write_buffer(&self.crop, 0, &bytes);
        for lane in ink {
            lane.view(
                queue,
                &request.transform,
                window.full,
                request.density,
            );
        }
        queue.submit([crate::pick_encode::draw(
            device,
            image,
            window,
            &self.pipeline,
            &self.view_group,
            &self.crop_group,
            meshes,
            ink,
        )]);
        let ready = Arc::new(AtomicU8::new(0));
        self.active = Some(Arc::clone(&ready));
        let alive = Arc::clone(&self.alive);
        // Mapping finishes asynchronously; the callback publishes readiness, and the owner reads the bytes without blocking browser input.
        image
            .readback
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                ready.store(
                    if result.is_ok() {
                        1
                    } else {
                        2
                    },
                    Ordering::Release,
                );
                if alive.load(Ordering::Acquire) {
                    #[cfg(target_arch = "wasm32")]
                    crate::browser_pick::wake();
                }
            });
        self.submitted += 1;
    }

    pub fn take(&mut self) -> Option<Result<Option<[u32; 2]>, &'static str>> {
        let status = self.active.as_ref()?.load(Ordering::Acquire);
        if status == 0 {
            return None;
        }
        self.active = None;
        let image = self.image.as_ref().unwrap();
        if status != 1 {
            return Some(Err("Cannot read GPU pick image"));
        }
        let bytes = image.readback.slice(..).get_mapped_range();
        let result = image.window.closest(&bytes);
        drop(bytes);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub(crate) struct Queued {
    request: Request,
    ticket: crate::pick_gate::Ticket,
}
struct Flight {
    ready: Arc<AtomicU8>,
    ticket: crate::pick_gate::Ticket,
}

pub struct Picker {
    pipeline: wgpu::RenderPipeline,
    view: wgpu::Buffer,
    view_group: wgpu::BindGroup,
    crop: wgpu::Buffer,
    crop_group: wgpu::BindGroup,
    image: Option<crate::pick_image::Image>,
    active: Option<Flight>,
    pending: Option<Queued>,
    alive: Arc<AtomicBool>,
    gate: crate::pick_gate::Gate,
    pub submitted: u64,
    pub coalesced: u64,
}

impl Drop for Picker {
    fn drop(&mut self) {
        self.alive.store(false, Ordering::Release);
        self.gate.close();
    }
}

impl Picker {
    pub fn new(
        device: &wgpu::Device,
        view_layout: &wgpu::BindGroupLayout,
        model_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let crop_layout = crate::gpu_layout::uniform(device, 64);
        let pipeline = crate::pick_pipeline::mesh(
            device,
            view_layout,
            model_layout,
            &crop_layout,
        );
        let buffer = |size| {
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("pick view"),
                size,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
        };
        let view = buffer(80);
        let crop = buffer(64);
        let view_group = crate::gpu_layout::group(device, view_layout, &view);
        let crop_group = crate::gpu_layout::group(device, &crop_layout, &crop);
        Self {
            pipeline,
            view,
            view_group,
            crop,
            crop_group,
            image: None,
            active: None,
            pending: None,
            alive: Arc::new(AtomicBool::new(true)),
            gate: Default::default(),
            submitted: 0,
            coalesced: 0,
        }
    }

    pub fn queue(&mut self, request: Request) -> Result<(), &'static str> {
        crate::pick_view::View::new(
            request.transform,
            request.window.full,
            request.density,
        )?;
        let ticket = self.gate.issue()?;
        if self
            .pending
            .replace(Queued { request, ticket })
            .is_some()
        {
            self.coalesced += 1;
        }
        Ok(())
    }

    pub fn queued(&self) -> bool {
        self.pending.is_some()
    }

    pub fn cancel(&mut self) {
        self.gate.invalidate();
        self.pending = None;
        let mapped = self
            .active
            .take()
            .is_some_and(|flight| flight.ready.load(Ordering::Acquire) <= 1);
        if let Some(image) = self.image.take() {
            if mapped {
                image.readback.unmap();
            }
        }
    }

    pub(crate) fn next(&mut self) -> Option<Queued> {
        if self.active.is_some() {
            None
        } else {
            self.pending.take()
        }
    }

    pub(crate) fn submit(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        queued: Queued,
        meshes: &[GpuMesh],
        ink: &[&Lane],
    ) {
        assert!(
            self.active.is_none(),
            "Only one pick may be in flight"
        );
        let request = queued.request;
        let window = request.window;
        if self.image.as_ref().is_none_or(|image| {
            image.window.draw[2..] != window.draw[2..]
                || image.readback.size() != window.bytes()
        }) {
            self.image = Some(crate::pick_image::Image::new(device, window));
        }
        let image = self.image.as_mut().unwrap();
        image.window = window;
        let mut bytes: Vec<u8> = request
            .transform
            .iter()
            .flat_map(|n| n.to_ne_bytes())
            .collect();
        bytes.resize(80, 0);
        queue.write_buffer(&self.view, 0, &bytes);
        let bytes: Vec<u8> = window
            .crop()
            .iter()
            .flat_map(|n| n.to_ne_bytes())
            .collect();
        queue.write_buffer(&self.crop, 0, &bytes);
        for lane in ink {
            lane.view(
                queue,
                &request.transform,
                window.full,
                request.density,
            );
        }
        queue.submit([crate::pick_encode::draw(
            device,
            image,
            window,
            &self.pipeline,
            &self.view_group,
            &self.crop_group,
            meshes,
            ink,
        )]);
        let ready = Arc::new(AtomicU8::new(0));
        self.active = Some(Flight {
            ready: Arc::clone(&ready),
            ticket: queued.ticket,
        });
        let alive = Arc::clone(&self.alive);
        // Mapping finishes asynchronously; the callback publishes readiness, and the owner reads the bytes without blocking browser input.
        image
            .readback
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                ready.store(
                    if result.is_ok() {
                        1
                    } else {
                        2
                    },
                    Ordering::Release,
                );
                if alive.load(Ordering::Acquire) {
                    #[cfg(target_arch = "wasm32")]
                    crate::browser_pick::wake();
                }
            });
        self.submitted += 1;
    }

    pub fn take(&mut self) -> Option<Result<Option<[u32; 2]>, &'static str>> {
        let status = self
            .active
            .as_ref()?
            .ready
            .load(Ordering::Acquire);
        if status == 0 {
            return None;
        }
        let flight = self.active.take().unwrap();
        let current = self.gate.accept(flight.ticket);
        let image = self.image.as_ref().unwrap();
        if status != 1 {
            return Some(if current {
                Err("Cannot read GPU pick image")
            } else {
                Ok(None)
            });
        }
        let bytes = image.readback.slice(..).get_mapped_range();
        let result = if current {
            image.window.closest(&bytes)
        } else {
            Ok(None)
        };
        drop(bytes);
```

### 2. `src/renderer.rs`

Propagate checked request-admission failures before GPU submission.

**Locate this existing block:**

```rust
            density: self.density,
        });
        self.pump_pick();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            density: self.density,
        })?;
        self.pump_pick();
```

## <a id="checkpoint-43b-view"></a>3. Reject results from an old view or interrupted input

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3 of 3</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/pick_gpu.rs, src/render…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">current scene epoch and v…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">stale rejection</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/pick…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">valid next request</text>
</svg>
</div>

**What this code does:** Reject results from an old view or interrupted input.

### 1. `src/pick_gpu.rs`

Adopt only the current view and cancel interrupted input.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            return None;
        }
        let flight = self.active.take().unwrap();
        let current = self.gate.accept(flight.ticket);
        let image = self.image.as_ref().unwrap();
        if status != 1 {
            return Some(if current {
                Err("Cannot read GPU pick image")
            } else {
                Ok(None)
            });
        }
        let bytes = image.readback.slice(..).get_mapped_range();
        let result = if current {
            image.window.closest(&bytes)
        } else {
            Ok(None)
        };
```

Replace it with:

```rust
            return None;
        }
        let flight = self.active.take().unwrap();
        let current = self.gate.accept(flight.ticket) && flight.view == view;
        let image = self.image.as_ref().unwrap();
        if status != 1 {
            return Some(if current {
                Err("Cannot read GPU pick image")
            } else {
                Ok(Reply::Stale)
            });
        }
        let bytes = image.readback.slice(..).get_mapped_range();
        let result = if current {
            image.window.closest(&bytes).map(Reply::Hit)
        } else {
            Ok(Reply::Stale)
        };
```

**Locate this block:**

```rust
        self.submitted += 1;
    }

    pub fn take(&mut self) -> Option<Result<Option<[u32; 2]>, &'static str>> {
        let status = self
            .active
            .as_ref()?
```

Replace it with:

```rust
        self.submitted += 1;
    }

    pub fn take(
        &mut self,
        view: crate::pick_view::View,
    ) -> Option<Result<Reply, &'static str>> {
        let status = self
            .active
            .as_ref()?
```

**Locate this block:**

```rust
        self.active = Some(Flight {
            ready: Arc::clone(&ready),
            ticket: queued.ticket,
        });
        let alive = Arc::clone(&self.alive);
        // Mapping finishes asynchronously; the callback publishes readiness, and the owner reads the bytes without blocking browser input.
```

Replace it with:

```rust
        self.active = Some(Flight {
            ready: Arc::clone(&ready),
            ticket: queued.ticket,
            view: queued.view,
        });
        let alive = Arc::clone(&self.alive);
        // Mapping finishes asynchronously; the callback publishes readiness, and the owner reads the bytes without blocking browser input.
```

**Locate this block:**

```rust
        }
    }

    pub(crate) fn next(&mut self) -> Option<Queued> {
        if self.active.is_some() {
            None
        } else {
            self.pending.take()
        }
    }
```

Replace it with:

```rust
        }
    }

    pub(crate) fn next(&mut self, view: crate::pick_view::View) -> Option<Queued> {
        if self.active.is_some() {
            return None;
        }
        let queued = self.pending.take()?;
        if queued.view == view {
            Some(queued)
        } else {
            self.gate.invalidate();
            None
        }
    }
```

**Locate this block:**

```rust
        let ticket = self.gate.issue()?;
        if self
            .pending
            .replace(Queued { request, ticket })
            .is_some()
        {
            self.coalesced += 1;
```

Replace it with:

```rust
        let ticket = self.gate.issue()?;
        if self
            .pending
            .replace(Queued {
                request,
                ticket,
                view,
            })
            .is_some()
        {
            self.coalesced += 1;
```

**Locate this block:**

```rust
    }

    pub fn queue(&mut self, request: Request) -> Result<(), &'static str> {
        crate::pick_view::View::new(
            request.transform,
            request.window.full,
            request.density,
```

Replace it with:

```rust
    }

    pub fn queue(&mut self, request: Request) -> Result<(), &'static str> {
        let view = crate::pick_view::View::new(
            request.transform,
            request.window.full,
            request.density,
```

**Locate this block:**

```rust
    ticket: crate::pick_gate::Ticket,
}
struct Flight {
    ready: Arc<AtomicU8>,
    ticket: crate::pick_gate::Ticket,
}

pub struct Picker {
    pipeline: wgpu::RenderPipeline,
```

Replace it with:

```rust
    ticket: crate::pick_gate::Ticket,
    view: crate::pick_view::View,
}
struct Flight {
    ready: Arc<AtomicU8>,
    ticket: crate::pick_gate::Ticket,
    view: crate::pick_view::View,
}
pub enum Reply {
    Hit(Option<[u32; 2]>),
    Stale,
}

pub struct Picker {
    pipeline: wgpu::RenderPipeline,
```

### 2. `src/renderer.rs`

Adopt only the current view and cancel interrupted input.

**Locate this existing block:**

```rust
        })?;
        self.pump_pick();
        Ok(())
    }

    fn pump_pick(&mut self) {
        if let Some(request) = self.picker.next() {
            self.picker.submit(
                &self.device,
                &self.queue,
                request,
                &self.meshes,
                &[&self.strokes, &self.paths, &self.markers],
            );
        }
    }

    pub fn take_pick(&mut self) -> Option<Result<Option<[u32; 2]>, &'static str>> {
        let result = self.picker.take();
        if result.is_some() && self.picker.queued() {
            self.pump_pick();
            return None;
        }
        result
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        })?;
        self.pump_pick(transform);
        Ok(())
    }

    fn pump_pick(&mut self, transform: [f32; 16]) {
        let Ok(view) = crate::pick_view::View::new(transform, self.size, self.density)
        else {
            self.picker.cancel();
            return;
        };
        if let Some(request) = self.picker.next(view) {
            self.picker.submit(
                &self.device,
                &self.queue,
                request,
                &self.meshes,
                &[&self.strokes, &self.paths, &self.markers],
            );
        }
    }

    pub fn take_pick(
        &mut self,
        transform: [f32; 16],
    ) -> Option<Result<Option<[u32; 2]>, &'static str>> {
        let view = match crate::pick_view::View::new(transform, self.size, self.density)
        {
            Ok(view) => view,
            Err(error) => {
                self.picker.cancel();
                return Some(Err(error));
            }
        };
        match self.picker.take(view) {
            Some(Ok(crate::pick_gpu::Reply::Hit(hit))) => Some(Ok(hit)),
            Some(Ok(crate::pick_gpu::Reply::Stale)) => {
                self.pump_pick(transform);
                None
            }
            Some(Err(error)) => Some(Err(error)),
            None => None,
        }
    }
```

### 3. `src/browser.rs`

Adopt only the current view and cancel interrupted input.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                None
            };
            let action = if event.type_() == "viewer-pick" {
                match renderer.take_pick() {
                    Some(Ok(hit)) => Some(Action::Picked(
```

Replace it with:

```rust
                None
            };
            let action = if event.type_() == "viewer-pick" {
                match renderer.take_pick(editor.camera.uniform()) {
                    Some(Ok(hit)) => Some(Action::Picked(
```

**Locate this block:**

```rust
            let mut editor = shared.borrow_mut();
            if event.type_() == "pagehide" {
                gesture.cancel();
                if event
```

Replace it with:

```rust
            let mut editor = shared.borrow_mut();
            if matches!(
                event.type_().as_str(),
                "blur" | "pagehide" | "viewer-reload"
            ) {
                renderer.cancel_pick();
            }
            if event.type_() == "pagehide" {
                gesture.cancel();
                if event
```

### 4. `src/pick_gpu_tests.rs`

Copy actual GPU old-view and cancelled-request checks.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    assert!(renderer.take_pick().is_none());
    assert_eq!(renderer.stats(), uploads);
}
```

Replace it with:

```rust
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    assert!(renderer.take_pick(identity()).is_none());
    assert_eq!(renderer.stats(), uploads);
}

#[test]
fn a_real_readback_from_an_old_camera_cannot_select_in_the_current_view() {
    let mut scene = Scene::demo();
    scene.clear();
    scene.insert(triangle(0.2)).unwrap();
    let mut renderer = renderer(&scene);
    renderer
        .request_pick([0.0, 0.0], identity())
        .unwrap();
    renderer
        .device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    let mut moved = identity();
    moved[12] = 0.2;
    assert!(
        renderer.take_pick(moved).is_none(),
        "Same scene, different camera: no accepted hit"
    );
    assert_eq!(&renderer.pick_usage()[..2], &[0, 0]);
    let submitted = renderer.pick_usage()[3];
    let mut invalid = moved;
    invalid[0] = f32::NAN;
    assert!(
        renderer
            .request_pick([0.0, 0.0], invalid)
            .is_err()
    );
    assert_eq!(renderer.pick_usage()[3], submitted);
    renderer
        .request_pick([0.0, 0.0], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_some());
    renderer
        .request_pick([0.0, 0.0], identity())
        .unwrap();
    renderer.set_density(2.0);
    renderer
        .device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    assert!(
        renderer.take_pick(identity()).is_none(),
        "Density changed while the map was pending"
    );
}
```

**Locate this block:**

```rust
        .unwrap();
    assert_eq!(finish(&mut renderer), Some([new.value(), 0]));
    assert!(
        renderer.take_pick().is_none(),
        "The cancelled map has no result for the replacement"
    );
    let uploads = renderer.stats();
```

Replace it with:

```rust
        .unwrap();
    assert_eq!(finish(&mut renderer), Some([new.value(), 0]));
    assert!(
        renderer.take_pick(identity()).is_none(),
        "The cancelled map has no result for the replacement"
    );
    let uploads = renderer.stats();
```

**Locate this block:**

```rust
        })
        .unwrap();
    assert!(
        renderer.take_pick().is_none(),
        "A newer cursor supersedes the first result"
    );
    assert_eq!(finish(&mut renderer), None);
```

Replace it with:

```rust
        })
        .unwrap();
    assert!(
        renderer.take_pick(identity()).is_none(),
        "A newer cursor supersedes the first result"
    );
    assert_eq!(finish(&mut renderer), None);
```

**Locate this block:**

```rust
    renderer
        .take_pick()
        .expect("Map callback completed")
        .unwrap()
}
```

Replace it with:

```rust
    renderer
        .take_pick(identity())
        .expect("Map callback completed")
        .unwrap()
}
```

## Build & Run

![Recorded visual output for Lesson 6.2 (43b-view)](../../../screenshots/journey/43b-view.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 6.2
npm --prefix ../session_tests run course -- save 6.2
```

Continue with [6.3 · Pick original faces, edges and controls](../../consolidation/44-topology.md).
