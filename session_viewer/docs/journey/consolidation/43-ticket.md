# 6.2 · Accept a hit only for its current view

**Steps:** 3 · **Source:** [engine/gpu/pick.rs](../../../src/engine/gpu/pick.rs) · [app/selection.rs](../../../src/app/selection.rs) · [engine/gpu/faces.rs](../../../src/engine/gpu/faces.rs) · [app/walk/mesh_topology.rs](../../../src/app/walk/mesh_topology.rs)

## <a id="checkpoint-43-ticket"></a>1. Name the request and view that produced a hit

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="214" viewBox="0 0 1120 214" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr-7ad2abe3" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arr-r-7ad2abe3" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arr-g-7ad2abe3" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arr-b-7ad2abe3" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="grid-7ad2abe3" width="20" height="20" patternUnits="userSpaceOnUse">
      <path d="M 20 0 L 0 0 0 20" fill="none" stroke="#f1f5f9" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="210" rx="8" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="2" y="2" width="1116" height="28" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="2" y1="30" x2="1118" y2="30" stroke="#111111" stroke-width="1.5"/>
  <rect x="12" y="7" width="176" height="18" rx="3" fill="#111111"/>
  <text x="100" y="19.5" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#ffffff" paint-order="normal" stroke="none">43-TICKET · PICK GUARD</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="518" y="20" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Single-Flight map_async Readback Lock &amp; Camera/Scene Generation Validation</text>
  <rect x="838" y="7" width="268" height="18" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="972" y="19.5" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">valid &lt;=&gt; (ticket.cam_gen == cam.gen) ∧ (ticket.scene_rev == scene.rev)</text>
  <rect x="14" y="38" width="344" height="164" rx="6" fill="url(#grid-7ad2abe3)" stroke="#111111" stroke-width="1.5"/>
  <line x1="14" y1="60" x2="358" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1 · Why Unchecked Async Readback Races</text>
  <rect x="44" y="76" width="284" height="104" rx="4" fill="#fee2e2" stroke="#dc2626" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="98" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">Frame 10: Click -&gt; map_async(staging)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="122" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">Frame 11: User orbits camera (view changes)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="144" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">Frame 12: Old map_async resolves</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="166" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#dc2626">Stale pick must be rejected! ✗</text>
  <line x1="358" y1="120" x2="384" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-7ad2abe3)"/>
  <rect x="388" y="38" width="344" height="164" rx="6" fill="url(#grid-7ad2abe3)" stroke="#111111" stroke-width="1.5"/>
  <line x1="388" y1="60" x2="732" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2 · PickTicket { seq, cam_gen, scene_rev }</text>
  <rect x="414" y="76" width="292" height="104" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Staging Buffer State Machine:</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="122" text-anchor="middle" font-family="monospace" font-size="9" fill="#2563eb">• Idle -&gt; CopySubmitted(ticket) -&gt; Mapping</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="144" text-anchor="middle" font-family="monospace" font-size="9" fill="#16a34a">• Always buffer.unmap() before reuse</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="166" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">• Commit selection only if gens match ✓</text>
  <line x1="732" y1="120" x2="758" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-7ad2abe3)"/>
  <rect x="762" y="38" width="344" height="164" rx="6" fill="url(#grid-7ad2abe3)" stroke="#111111" stroke-width="1.75"/>
  <line x1="762" y1="60" x2="1106" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3 · Zero Double-Map Panics</text>
  <rect x="806" y="68" width="256" height="122" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="114" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#16a34a">Safe Async GPU Readback ✓</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="142" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">Rapid clicks queued cleanly</text>
</svg>
</div>

### 1. `src/pick_gate.rs`

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">GPU Offscreen Rg32Uint Texture (owner_id, sub_id)</text>
  <rect x="74" y="48" width="224" height="106" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <rect x="146" y="76" width="80" height="50" fill="#fef08a" stroke="#dc2626" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="105" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">5×5 px Window</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="144" text-anchor="middle" font-family="monospace" font-size="9" fill="#2563eb">COPY_SRC Attachment</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">copy_texture_to_buffer (256B Row Pitch) -&gt; Staging</text>
  <rect x="424" y="52" width="272" height="96" rx="6" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="78" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Staging Buffer (MAP_READ | COPY_DST)</text>
  <rect x="446" y="90" width="228" height="24" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="106" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">bytes_per_row = 256 (WebGPU alignment)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="134" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">slice.map_async(MapMode::Read, ...)</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU Callback Reads Mapped [u32; 2] Winner</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="826" y="58" width="216" height="44" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="78" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">get_mapped_range() -&gt; (id, sub)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="94" text-anchor="middle" font-family="monospace" font-size="9" fill="#16a34a">staging.unmap()</text>
  <rect x="846" y="114" width="176" height="32" rx="4" fill="#fef08a" stroke="#dc2626" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="134" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Selected Owner Updated</text>
</svg>
</div>

### 1. `src/pick_gpu.rs`

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Cursor Pick Crop Window (r_px Radius)</text>
  <rect x="68" y="46" width="236" height="114" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="104" y1="138" x2="268" y2="66" stroke="#2563eb" stroke-width="3"/>
  <circle cx="186" cy="102" r="5" fill="#dc2626"/>
  <rect x="158" y="74" width="56" height="56" fill="none" stroke="#111111" stroke-width="2" stroke-dasharray="3 2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="66" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">15×15 Crop</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Rg32Uint Attachment: vec2&lt;u32&gt;(owner, part)</text>
  <rect x="466" y="46" width="188" height="112" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <line x1="528" y1="46" x2="528" y2="158" stroke="#111111" stroke-width="1"/>
  <line x1="592" y1="46" x2="592" y2="158" stroke="#111111" stroke-width="1"/>
  <line x1="466" y1="83" x2="654" y2="83" stroke="#111111" stroke-width="1"/>
  <line x1="466" y1="121" x2="654" y2="121" stroke="#111111" stroke-width="1"/>
  <rect x="528" y="83" width="64" height="38" fill="#fef08a" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="106" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">[7, 2]</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="497" y="68" text-anchor="middle" font-family="monospace" font-size="9" fill="#64748b">[0,0]</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="623" y="144" text-anchor="middle" font-family="monospace" font-size="9" fill="#16a34a">[7,1]</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Priority: Point &gt; Edge &gt; Face Hit</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="866,138 934,62 1002,138" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <line x1="866" y1="138" x2="934" y2="62" stroke="#f59e0b" stroke-width="4"/>
  <circle cx="934" cy="62" r="7" fill="#fef08a" stroke="#dc2626" stroke-width="2.5"/>
</svg>
</div>

### 1. `src/pick_gpu.rs`

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

![Recorded visual output for Lesson 6.2 (43b-view)](../../screenshots/journey/43b-view.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 6.2
npm --prefix ../session_tests run course -- save 6.2
```

Continue with [6.3 · Pick original faces, edges and controls](44-topology.md).
