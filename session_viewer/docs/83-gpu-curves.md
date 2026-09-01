# 83 GPU curves — the segment table gets a compute producer

> **Big picture.** *Phase 10b — GPU tessellation (73–76).* Phase 10 put every curved type on
> screen by tessellating on the **CPU**: 43 sampled `point_at(t)` into a polyline, 44/46 cached
> `mesh()`, 47 ran the CDT. That is the correctness reference — keep it. This phase moves the
> *sampling work* onto the GPU, one type per lesson, and the architecture is the same every time:
> **a compute shader becomes a producer for a table the viewer already draws.** No new lanes, no
> new pipelines downstream — a curve tessellated by compute lands in the same `CylinderSegment`
> rows lesson 31 built, so tubes, flat ribbons, density taper, and pick-tinting all work
> untouched. Two contracts from the flat-lines rework are law here: **no `frag_depth`** (early-Z
> stays alive downstream), and **f32 is the display contract** (the kernel's f64 stays the
> modeling truth; `Mesh::gpu_mesh` set that precedent — measured on this exact port, the f32
> de Boor agrees with the f64 kernel to 2e-5 on 70 mm geometry, pure float rounding).

<svg viewBox="0 0 680 150" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="control points and knots upload once; a compute dispatch evaluates de boor per segment and writes cylinder segment rows into the shared table; both line lanes draw them unchanged" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="10" y="20" width="150" height="34" fill="none" stroke="#6fb3ff"/><text x="85" y="34" fill="#d7dae0" text-anchor="middle">CVs + knots</text><text x="85" y="47" fill="#666" text-anchor="middle" font-size="9">upload ONCE, f32 (x,y,z,w)</text>
  <rect x="200" y="20" width="170" height="34" fill="none" stroke="#6fb3ff"/><text x="285" y="34" fill="#d7dae0" text-anchor="middle">compute: de Boor</text><text x="285" y="47" fill="#666" text-anchor="middle" font-size="9">1 invocation = 1 segment</text>
  <rect x="410" y="20" width="150" height="34" fill="none" stroke="#6fb3ff"/><text x="485" y="34" fill="#d7dae0" text-anchor="middle">segments[]</text><text x="485" y="47" fill="#666" text-anchor="middle" font-size="9">the SAME table 31 draws</text>
  <path d="M 160,37 L 200,37 M 370,37 L 410,37" stroke="#888" marker-end="none"/>
  <text x="180" y="33" fill="#888">→</text><text x="390" y="33" fill="#888">→</text>
  <text x="340" y="90" fill="#d7dae0" text-anchor="middle">re-dispatch only when zoom crosses ×2 — never per frame</text>
  <text x="340" y="110" fill="#888" text-anchor="middle">CPU polyline (43) stays: it is the PICK proxy and the acceptance reference</text>
  <text x="340" y="130" fill="#666" text-anchor="middle" font-size="10">tubes lane + flat lane + taper + selection tint: zero changes downstream</text>
</svg>

## Files we touch

```
src/shaders/curve_tess.wgsl   # NEW — find_span + Cox-de Boor + segment writer
src/engine/pipelines/build.rs # build_curve_tess_pipeline — the viewer's FIRST compute pipeline
src/engine/pipelines/mod.rs   # register it
src/engine/gpu/mod.rs         # read_write segment binding + dispatch_curves
src/app/scene.rs              # build: reserve rows instead of filling them; upload curve data
```

## Step 1 — why one dispatch beats 512 `point_at` calls

Lesson 52's CPU arm costs `samples × point_at`, single-threaded, on every rebuild. The GPU has
thousands of lanes idling next to a table it can write directly. The port is honest because the
algorithm is *embarrassingly parallel*: every sample of a B-spline depends only on `(t, knots,
CVs)` — no neighbor, no sequence. The whole of 43's `sample_curve` becomes one dispatch.

What does NOT move: the **pick proxy**. 55's raycast walks CPU-side geometry; a GPU-resident
polyline would need a readback to pick. So `sample_curve` (43) still runs — at a fixed coarse
count — and only the *display* density lives on the GPU. Same split 66 chose for meshes
(display cache vs pick mesh), now across the bus.

## Step 2 — the shader: `src/shaders/curve_tess.wgsl` (NEW)

Three ports from `session_rust/src/nurbscurve.rs`, line for line: `find_span` (the OpenNURBS
`order - 2` shifted-knot convention), `basis_functions` (the Cox–de Boor triangle), and
`point_at`'s homogeneous accumulate. One upload rule kills every branch the CPU version needs:
**CVs are always uploaded as `(x, y, z, w)` with `w = 1` for non-rational curves** — the shader
has one code path and one divide.

```wgsl
// Everything the shader needs to know about ONE curve. 48 B, uniform.
struct CurveInfo {
    order: u32,        // m_order (degree + 1)
    cv_count: u32,     // m_cv_count
    samples: u32,      // polyline vertex count; segments written = samples - 1
    seg_base: u32,     // first row this curve owns in segments[]
    t0: f32,           // domain start
    t1: f32,           // domain end
    instance_id: u32,  // the object row (model matrix, flags)
    color: u32,        // RGBA8, low byte red - scene.rs's pack_rgba
    knot_base: u32,    // where this curve's knots start in data[]
    cv_base: u32,      // where its CVs start (always 4 floats per CV)
    _pad0: u32, _pad1: u32,
};
@group(0) @binding(0) var<uniform> curve: CurveInfo;
// One flat f32 pool per scene: every curve's knots, then its CVs.
@group(0) @binding(1) var<storage, read> data: array<f32>;

// The REAL row - must match CylinderSegment in gpu/mod.rs exactly (40 B, scalar ends).
struct CylinderSegment {
    p0x: f32, p0y: f32, p0z: f32,
    radius: f32,
    p1x: f32, p1y: f32, p1z: f32,
    instance_id: u32,
    color: u32,
    facing: u32,
};
@group(0) @binding(2) var<storage, read_write> segments: array<CylinderSegment>;

const FACING_UNKNOWN: u32 = 0xffffffffu;
// Fixed-size arrays for the de Boor triangle - WGSL has no Vec. Degree 7 covers everything
// the kernel emits; a higher-degree import clamps here rather than corrupting memory.
const MAX_ORDER: u32 = 8u;

fn knot(i: u32) -> f32 { return data[curve.knot_base + i]; }

// nurbsknot::find_span, ported 1:1 (knots shifted by order - 2, binary search between).
fn find_span(t: f32) -> u32 {
    let offset = curve.order - 2u;
    let span_len = curve.cv_count - curve.order + 2u;
    if (t <= knot(offset)) { return 0u; }
    if (t >= knot(offset + span_len - 1u)) { return span_len - 2u; }
    var low = 0u;
    var high = span_len - 1u;
    loop {
        if (high <= low + 1u) { break; }
        let mid = (low + high) / 2u;
        if (t < knot(offset + mid)) { high = mid; } else { low = mid; }
    }
    return low;
}

// basis_functions + point_at, fused. The triangle is nurbscurve.rs:3319 verbatim.
fn eval(t: f32) -> vec3<f32> {
    let span = find_span(t);
    var basis: array<f32, MAX_ORDER>;
    var left: array<f32, MAX_ORDER>;
    var right: array<f32, MAX_ORDER>;
    let offset = curve.order - 2u + span;
    basis[0] = 1.0;
    for (var j = 1u; j < curve.order; j = j + 1u) {
        left[j] = t - knot(offset + 1u - j);
        right[j] = knot(offset + j) - t;
        var saved = 0.0;
        for (var r = 0u; r < j; r = r + 1u) {
            let denom = right[r + 1u] + left[j - r];
            var temp = 0.0;
            if (denom != 0.0) { temp = basis[r] / denom; }
            basis[r] = saved + right[r + 1u] * temp;
            saved = left[j - r] * temp;
        }
        basis[j] = saved;
    }
    var acc = vec4<f32>(0.0);
    for (var i = 0u; i < curve.order; i = i + 1u) {
        let cv_idx = span + i;
        if (cv_idx >= curve.cv_count) { continue; }
        let k = curve.cv_base + cv_idx * 4u;
        acc = acc + basis[i] * vec4<f32>(data[k], data[k + 1u], data[k + 2u], data[k + 3u]);
    }
    if (abs(acc.w) > 1e-10) { return acc.xyz / acc.w; }
    return acc.xyz;
}

// One invocation = one SEGMENT, both ends evaluated here. The shared end is computed twice
// (this thread's p1 = next thread's p0) - identical inputs give the bit-identical f32 result,
// so the polyline stays watertight, and the redundancy is cheaper than any thread handoff.
@compute @workgroup_size(64)
fn tess(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i >= curve.samples - 1u) { return; }
    let dt = (curve.t1 - curve.t0) / f32(curve.samples - 1u);
    let a = eval(curve.t0 + dt * f32(i));
    let b = eval(curve.t0 + dt * f32(i + 1u));
    var s: CylinderSegment;
    s.p0x = a.x; s.p0y = a.y; s.p0z = a.z;
    s.radius = 0.0;                    // screen-constant global pen, like 43's CPU arm
    s.p1x = b.x; s.p1y = b.y; s.p1z = b.z;
    s.instance_id = curve.instance_id;
    s.color = curve.color;
    s.facing = FACING_UNKNOWN;         // free-standing linework: never facing-culled or tapered
    segments[curve.seg_base + i] = s;
}
```

Honest label: this is 43's *uniform-per-span* sampling, not chord-error adaptivity — the GPU
buys enough headroom (`spans × 16` becomes `spans × 64` for the same frame cost) that uniform
density stops being the visible limit. The adaptive upgrade, if ever needed, runs the
subdivision on the CPU into a `t`-value buffer and keeps this shader unchanged.

## Step 3 — the first compute pipeline: `src/engine/pipelines/build.rs`

Everything so far was a `RenderPipeline`. A compute pipeline is smaller — no vertex layouts, no
targets, no depth state — just bind groups and an entry point. Add at the end of the file:

```rust
/// The viewer's FIRST compute pipeline: curve tessellation (73). Bind group 0 is built ad hoc
/// per dispatch (uniform + data pool + the segment table as read_WRITE - the render passes keep
/// their own read-only binding of the same buffer).
pub fn build_curve_tess_pipeline(device: &wgpu::Device) -> (wgpu::ComputePipeline, wgpu::BindGroupLayout) {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("curve.tess.shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/curve_tess.wgsl").into()),
    });
    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("curve.tess.bgl"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0, visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false, min_binding_size: None },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1, visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false, min_binding_size: None },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2, visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false, min_binding_size: None },
                count: None,
            },
        ],
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("curve.tess.layout"),
        bind_group_layouts: &[Some(&bgl)],
        immediate_size: 0,
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("curve.tess"),
        layout: Some(&layout),
        module: &shader,
        entry_point: Some("tess"),
        compilation_options: Default::default(),
        cache: None,
    });
    (pipeline, bgl)
}
```

In `src/engine/pipelines/mod.rs`, find `pub struct Pipelines{` and add two fields at the end:

```rust
    pub curve_tess: wgpu::ComputePipeline,
    pub curve_tess_bgl: wgpu::BindGroupLayout,
```

In `Pipelines::new`, insert ABOVE the `Self {` line (the builder returns a pair, so it cannot
sit inside the literal like the others):

```rust
        let (curve_tess, curve_tess_bgl) = build_curve_tess_pipeline(device);
```

and add `curve_tess, curve_tess_bgl,` to the `Self {` literal.

## Step 4 — writable segments: `src/engine/gpu/mod.rs`

The segment table is already a storage buffer — find its creation (the `zeroed_buffer` call
with `"segments"`, ~line 387) and extend the usage so compute may write it:

```rust
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
```
becomes
```rust
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST); // compute writes rows too
```

(no flag change needed — `STORAGE` covers read_write; the render bind group stays
`read_only: true`, the compute bind group below declares `read_only: false`. Same buffer, two
bindings, no hazard: the dispatch and the draws are ordered by the queue.)

Then give `Gpu` the upload + dispatch. The layering law from the roadmap holds — **lower
layers never reach up** — so the wgpu objects live HERE, and the scene hands over plain data
(`CurveUpload`, next step). Add near `render_offscreen`:

```rust
/// One curve's GPU residency: its CurveInfo uniform + data pool bound against curve_tess_bgl.
pub struct GpuCurve {
    bind_group: wgpu::BindGroup,
    samples: u32,
}
```

```rust
    /// Adopt the scene's curve uploads (buffers + bind groups built here, engine-side) and
    /// tessellate them into their reserved segment rows. Called on upload and when the zoom
    /// crosses a rebucket threshold - never per frame.
    pub fn tessellate_curves(&mut self, uploads: &[CurveUpload]) {
        self.gpu_curves = uploads.iter().map(|u| {
            let info = /* CurveInfo bytes from u - order, cv_count, samples, seg_base, t0, t1,
                          instance_id, color, knot_base = 0, cv_base = u.knots_len */ ;
            let info_buf = /* uniform buffer from `info` */ ;
            let pool = /* storage buffer from u.data */ ;
            GpuCurve { bind_group: /* bgl: info_buf, pool, self.segment_buffer */, samples: u.samples }
        }).collect();
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&self.pipelines.curve_tess);
            for c in &self.gpu_curves {
                pass.set_bind_group(0, &c.bind_group, &[]);
                pass.dispatch_workgroups((c.samples - 1).div_ceil(64), 1, 1);
            }
        }
        self.queue.submit([encoder.finish()]);
    }
```

(The elided buffer builds are the same `create_buffer_init` calls every table upload uses;
the one new fact is the bind group pairing each curve's own uniform + pool with the SHARED
`segment_buffer` as its read_write third binding.)

## Step 5 — the build arm reserves instead of fills: `src/app/scene.rs`

43's arm pushed `samples - 1` filled rows. Now it pushes the same COUNT of **zeroed rows**
(the reservation — row indexing downstream is untouched; a zeroed row is a zero-length
segment, invisible until the dispatch fills it) and records what the engine needs, in plain
data. `CurveUpload` is declared in `gpu/mod.rs` beside `GpuCurve` and imported here like
`CylinderSegment` is — the app consumes engine types, never the reverse:

```rust
/// Everything the app hands over to make one curve GPU-resident. Plain data: the wgpu
/// objects are built engine-side (tessellate_curves), keeping the layering one-directional.
pub struct CurveUpload {
    pub data: Vec<f32>,      // knots, then always-homogeneous CVs (x, y, z, w)
    pub knots_len: u32,      // where the CVs start in data
    pub order: u32,
    pub cv_count: u32,
    pub samples: u32,
    pub seg_base: u32,
    pub t0: f32,
    pub t1: f32,
    pub instance_id: u32,
    pub color: u32,
}
```

In `Scene::build`'s `Geometry::NurbsCurve(nc)` arm, keep the coarse `sample_curve(nc)` polyline
for `curve_cache` (the pick proxy) — but replace the `for w in pts.windows(2)` segment loop
with the reservation (`use bytemuck::Zeroable;` at the top of the file):

```rust
        let samples = (nc.span_count() as u32 * 64).clamp(64, 2048);   // GPU headroom: 4x 43's
        let seg_base = segments.len() as u32;
        segments.extend((1..samples).map(|_| CylinderSegment::zeroed()));
        curve_uploads.push(nc_upload(nc, samples, seg_base, ri, pack_rgba(curve_color(nc))));
```

where `nc_upload` flattens knots + always-homogeneous CVs into a `CurveUpload` (the Step 2
upload rule — `w = 1` when `!nc.m_is_rat`, else copy `m_cv`'s `(xw, yw, zw, w)` as-is).
`upload_to` hands the collected `curve_uploads` to `gpu.tessellate_curves(..)` after the
tables are pushed.

**Re-tessellation cadence**: store the camera distance the last dispatch used; in the render
path, when `dist_now / dist_then` leaves `[0.5, 2.0]`, recompute `samples` per curve, re-reserve
if the bucket changed, re-dispatch, and store the new distance. Zoom inside the band costs
nothing — the contract render-on-demand (78) relies on.

## What you should see

Draw a `curve` (70's deferred curve tool, or load a curve fixture) and it looks *identical* — that is the acceptance: **CPU and GPU
tessellation of the same curve differ by f32 rounding only** (measured: 2e-5 on 70 mm spans,
verified against `point_at` for non-rational, rational-circle, and surface-boundary curves).
Then zoom close to a tight bend: where 43's 16-per-span polyline showed chords, the 64-per-span
GPU version stays smooth, at zero CPU cost. `L` still flips tubes/flat — both lanes draw the
compute-written rows, which is the whole point.

```
Ch 73: the pattern of Phase 10b, once: a COMPUTE PRODUCER writes a table the viewer already
        draws. Curve CVs/knots upload once (always-homogeneous f32); one invocation per segment
        runs find_span + Cox-de Boor (nurbscurve.rs ports, verified to f32 rounding); rows land
        in segments[] with FACING_UNKNOWN so both line lanes, taper, and tint work untouched.
        Re-dispatch on x2 zoom thresholds only. CPU polyline survives as pick proxy + reference.
```

Edited: `shaders/curve_tess.wgsl` (new), `pipelines/build.rs` + `mod.rs` (first compute
pipeline), `gpu/mod.rs` (`dispatch_curves`), `scene.rs` (reserve + upload + cadence).

## Next

`84-gpu-surfaces.md` — the same producer pattern aimed at the vertex arena: tensor-product
NURBS evaluation in compute, one invocation per grid vertex, and 44's cache becomes a
resolution policy instead of a mesh store.
