# 36 The raw cloud lane — one vertex, one pixel

## Goal

Draw a 13.8-million-point LiDAR scan at interactive frame rates, by giving dense clouds
their own lane instead of sending them through the glyph dots that 32b built.

## Why the glyph lane cannot do this

[35](35-scene-struct.md) routed `Geometry::PointCloud` into the flat glyph lane —
the same SDF-circle billboards that draw a `Point`. That is the right answer for
32b's demo clouds and it is the wrong answer for a scan, for two reasons that compound:

```
        GLYPH LANE per point                 RAW LANE per point
        ────────────────────                 ──────────────────
        3 vertices (one triangle)            1 vertex
        a ~3 px round dot   ≈ 38 px          exactly 1 px
        ALPHA_BLENDING → early-Z OFF         opaque → early-Z ON
```

At 13.8M points that is **41.4M vertices** instead of 13.8M, and — far worse — roughly
**520M blended fragments per frame** against a 2.5M-pixel screen. Two hundred times
overdraw, every one of which has to be shaded and blended *in submission order*, because
blending is what turns early-Z off. Measured at **~100 ms/frame**, and it did not just
drop the frame rate: it stalled the desktop.

Opaque points get the depth test back. A sample hidden behind a closer sample is
rejected *before* the fragment shader runs, so a dense cloud costs roughly its visible
silhouette rather than its total point count.

The trade is that WebGPU rasterises a point-list point as **exactly one pixel** — WGSL
has no `gl_PointSize`, and there is no way to ask for a bigger one. So this is not a
replacement for the glyph dots, it is a second lane. Small clouds keep their round
sized dots; big ones become a clump of pixels, which is what a scan actually looks like.

**The lane is chosen by point COUNT, never by camera state.** That matters: a cloud that
switched lanes as you zoomed would change appearance mid-orbit, and nothing here is
allowed to degrade while the view is moving.

## Files we touch

| file | change |
|---|---|
| `src/app/scene.rs` | `CLOUD_RAW_MIN`, `push_cloud`, a second `PointCloud` match arm |
| `src/engine/gpu/mod.rs` | a `points` table in `ArenaUpload`, the upload, the draw |
| `src/engine/pipelines/build.rs` | `PointList`, opaque, depth-writing |
| `src/shaders/point.wgsl` | one vertex per point, no SDF |

---

## Step 1 — the table: `src/engine/gpu/mod.rs`

`Gpu` has carried a `CloudPoint` row struct and an empty `point_buffer` since before
35 — machinery with nothing to put in it. Give `ArenaUpload` the rows.

**Find** in `ArenaUpload`:

```rust
    pub glyphs: Vec<GlyphPoint>, // Flat lane: points, draw as SDF dots,
```

**Add below it:**

```rust
    pub points: Vec<CloudPoint>, // Raw lane: scanned clouds, one vertex and one pixel per point
```

and the matching `points: Vec::new(),` in `ArenaUpload::new()`, next to `glyphs`.

The row itself is already there; it is worth reading:

```rust
pub struct CloudPoint{
    pub position: [f32; 3], // 12 B - mesh local
    pub instance_id: u32,   //  4 B - fills position's tail
    pub color: [f32; 4],    // 16 B
} // 32 B total, two 16-byte rows, zero padding
```

32 B against `GlyphPoint`'s 48, and **no radius field** — a cloud has one global size,
not a pen per point. ([38](38-sixteen-bytes.md) takes it to 16.)

## Step 2 — the walk: `src/app/scene.rs`

**Find** the point-cloud arm:

```rust
                Geometry::PointCloud(pc) => t.glyphs.extend(pointcloud_to_glyphs(pc, ri)),
```

**Replace with** two arms — the guarded one must come **first**, because Rust takes the
first arm that matches:

```rust
                // A cloud picks its lane by SIZE, not by camera state - so nothing changes while
                // you orbit. A handful of points are worth round sized dots (32b's demo clouds);
                // a scan is a clump, and the raw lane draws it one vertex and one pixel per point.
                Geometry::PointCloud(pc) if pc.len() >= CLOUD_RAW_MIN => {
                    push_cloud(pc, ri, &mut t.points)
                }
                Geometry::PointCloud(pc) => t.glyphs.extend(pointcloud_to_glyphs(pc, ri)),
```

**Add** at the bottom of the file, next to `pointcloud_to_glyphs`:

```rust
/// Above this many points a cloud stops being decorated dots and becomes a raw clump: one
/// vertex, one pixel, opaque. Below it the sized round dots of the glyph lane still read better,
/// and 100k of them is a frame cost nobody notices.
const CLOUD_RAW_MIN: usize = 100_000;

/// The raw lane's rows. Same walk as the glyph version, minus the radius - a cloud has no pen
/// per point - and 32 B per row instead of 48.
///
/// It writes STRAIGHT into the shared table instead of collecting a Vec the caller then extends:
/// `Vec::extend` from an owned iterator always memcpies into the destination and drops the
/// source, so a 13.8M-point scan built the same 441 MB table twice and peaked at 843 MB against
/// a heap that practically ends around 2 GB. Reserving once and pushing peaks at 423 MB.
///
/// It also reads the kernel's FLAT arrays rather than `get_point`/`get_color`, which each build a
/// `Point`/`Color` - three String allocations per point, measured at 1.08 s against 0.24 s for
/// the flat walk on this scan, all of it allocator churn on the wasm main thread.
fn push_cloud(pc: &PointCloud, instance_id: u32, out: &mut Vec<CloudPoint>){
    let coords = pc.coords();
    let colors = pc.colors();
    let n = pc.len();
    out.reserve(n);
    for i in 0..n {
        let c = i * 4;
        out.push(CloudPoint{
            position: [coords[i * 3] as f32, coords[i * 3 + 1] as f32, coords[i * 3 + 2] as f32],
            instance_id,
            color: if c + 3 < colors.len() {
                [colors[c] as f32 / 255.0, colors[c + 1] as f32 / 255.0,
                 colors[c + 2] as f32 / 255.0, colors[c + 3] as f32 / 255.0]
            } else {
                [0.0, 0.0, 0.0, 1.0]
            },
        });
    }
}
```

and put `CloudPoint` in the import at the top of the file.

Those two doc-comment paragraphs are the whole reason this function does not look like
`pointcloud_to_glyphs`. `get_point(i)` builds a `Point`, and a `Point` owns a name and a
`Color` that owns another name — three heap allocations, thirteen million times. The
flat-slice accessors `coords()` / `colors()` exist for exactly this, in all three
languages.

## Step 3 — the upload and the draw: `src/engine/gpu/mod.rs`

In `set_scene`, after the glyph block, **add**:

```rust
        // Raw cloud lane: one row per scanned point, uploaded like any other table. Until now
        // this buffer was built empty in new() and never refilled - the machinery existed, the
        // rows never arrived.
        self.point_count = up.points.len() as u32;
        self.point_buffer = storage_buffer(&self.device, "points.buffer", &up.points);
        self.point_bind_group = self.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("points.bind_group"),
                layout: &self.glyph_layout,
                entries: &[wgpu::BindGroupEntry{ binding: 0, resource: self.point_buffer.as_entire_binding() }],
        });
```

Then **move** the point draw. It currently sits at the very end of the pass, where it was
left when it was a blended overlay. **Cut** it from there and **paste** it right after the
cylinder draw, before the flat-ink prepass:

```rust
            // Raw cloud lane, drawn WITH THE SOLIDS: it is opaque and writes depth, so it belongs
            // before the flat ink, not after it. Here, ink in front of the cloud composites over
            // it and ink behind is rejected by the ribbon/glyph depth test. Drawn last - where it
            // sat while it was a blended overlay - an opaque cloud would instead overpaint every
            // polyline in front of it, because flat ink writes no depth of its own.
            if self.point_count > 0 {
                pass.set_pipeline(&self.pipelines.point);
                pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                pass.set_bind_group(1, &self.cloud_bind_group, &[]);
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.point_bind_group, &[]);
                pass.draw(0..self.point_count, 0..1); // ONE vertex per point (PointList)
                draws += 1;
            }
```

The order is the whole point, and it is a consequence of the cloud going opaque. The pass
now reads: background → grid → triangles → cylinders → **CLOUD** → ink prepass → ribbon →
sphere → glyph. Everything that **writes** depth comes first; the flat ink lanes read that
depth and never write it.

Also fix a latent bug two lines up while you are here — `std::mem::size_of::<GlyphPoint>`
without the parentheses is a *function pointer*, not a size:

```rust
rows * std::mem::size_of::<GlyphPoint>() as u64
```

## Step 4 — the pipeline: `src/engine/pipelines/build.rs`

Three changes in `build_point_pipeline`, and each one is doing real work.

**Find** `blend: Some(wgpu::BlendState::ALPHA_BLENDING),` and **replace with:**

```rust
                // OPAQUE. Blending is what made a dense cloud unaffordable: it turns off early-Z,
                // so every one of ~520M overlapping fragments had to be shaded and blended in
                // submission order. Written opaque, the depth test rejects occluded samples first.
                blend: None,
```

**Find** `topology: wgpu::PrimitiveTopology::TriangleList,` and **replace with:**

```rust
            // ONE vertex per point, rasterised as exactly one pixel (WebGPU has no point size).
            topology: wgpu::PrimitiveTopology::PointList,
```

**Find** `depth_write_enabled: Some(false),` and **replace with:**

```rust
            // Writes depth like any other solid: a cloud occludes what is behind it, and points
            // behind it are rejected before shading. The flat-ink lanes stay depth-read-only.
            depth_write_enabled: Some(true),
```

## Step 5 — the shader: `src/shaders/point.wgsl`

The old shader expanded each point into a triangle and carved a circle out of it with an
SDF. All of that goes. **Replace the whole file:**

```wgsl
@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;

struct Instance{
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
};
@group(2) @binding(0) var<storage, read> instances: array<Instance>;

struct CloudPoint {
    position: vec3<f32>,
    instance_id: u32,
    color: vec4<f32>,
};
@group(3) @binding(0) var<storage, read> points: array<CloudPoint>;

const FLAG_HIDDEN: u32 = 2u;   // Instance::FLAG_HIDDEN, bit 1

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut{
    let p = points[vid];        // ONE vertex per point - vid IS the point
    let inst = instances[p.instance_id];
    let world = (inst.model * vec4<f32>(p.position, 1.0)).xyz;

    var o: VsOut;
    o.pos = mvp * vec4<f32>(world, 1.0);
    // A hidden cloud leaves the same way hidden geometry leaves every other lane: pushed behind
    // the near plane so the clip stage drops it, no per-fragment test.
    if ((inst.flags & FLAG_HIDDEN) != 0u) {
        o.pos = vec4<f32>(0.0, 0.0, -1.0, 1.0);
    }
    o.color = p.color * inst.color;
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color.rgb, 1.0);
}
```

Two things this shader deliberately does **not** do.

It does not `discard`, and it does not output a `sample_mask`. The shader mask is ANDed
with rasterizer coverage on every backend, so it can only clear bits, never set them —
writing all-ones is inert, and *merely declaring it* makes coverage shader-dependent,
which demotes an early-Z rejection to late-Z. Early-Z is the entire point of this lane.

And it is the first lane to actually read `Instance.flags`. Every other shader declares
the field (the struct layout has to match) and ignores it, so hiding a mesh today does
nothing. Here, `FLAG_HIDDEN` collapses the vertex behind the near plane and the clip stage
drops it — no per-fragment cost at all.

## Verify

Point `DEMO_SCENE_URL` at a cloud manifest and load a scan. You should see:

- the console's `scene: … N cloud points` line reporting the full count;
- the cloud drawn as a dense clump of single pixels, **not** round dots;
- polylines and other flat ink in front of the cloud still visible — that is the draw-order
  move in step 3. If ink vanishes behind the cloud, the draw is still at the end of the pass.
- a small cloud (under `CLOUD_RAW_MIN`) unchanged from 32b — still round dots.

Frame times should be single-digit milliseconds where the glyph lane was near 100.

## What this costs, and what comes next

It draws. It does not load well: three scans of 3.5M points each measure **2490 MB in the
renderer and 1034 MB in the GPU process**, and the tab gets OOM-killed. Every point is
materialised five times on the way in, and the upload path mirrors the whole table into the
wasm heap.

That is [37](37-cloud-memory.md), and it is a bigger problem than this lesson was.

## Recap

```
Ch 35:  a cloud went through the GLYPH lane - 3 verts and a blended ~38 px dot per point.
        Fine for a demo, ~100 ms/frame and a stalled desktop for a 13.8M-point scan.
Ch 36:  the RAW lane. One vertex per point, PrimitiveTopology::PointList, opaque, depth-
        writing - so the depth test rejects occluded samples before shading, which blending
        had made impossible. Chosen by point COUNT (CLOUD_RAW_MIN), never by camera state,
        so nothing changes mid-orbit. push_cloud reads the kernel's FLAT coords()/colors()
        slices, not get_point/get_color, which built three Strings per point. The draw moves
        up next to the solids because an opaque, depth-writing lane must run before the flat
        ink that only READS depth. First lane to actually honour Instance::FLAG_HIDDEN.
```

## Next

[`37-cloud-memory.md`](37-cloud-memory.md) — it draws beautifully and it will not load.
Where 3.5 GB goes for 323 MB of GPU data, and how much of it is a copy nobody asked for.
