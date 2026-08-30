# 44 Cloud octree — Potree's LOD, on our splat lane

> Replay-verified against the end-of-43 tree. Numbers measured twice: with LOD off the
> three goldens are PIXEL-IDENTICAL to lesson 43's; with LOD on, the lion close-up is
> STILL pixel-identical (325369) while the fit views draw **92,331 of 7,492,706** points
> (cloud_mix) and **37,972 of 13,793,783** (lidar14).

## Goal

Lessons 40/41 left one Potree feature on the table: the octree. This lesson takes it —
using the kernel's new `SpatialOctree` (a real 3-language class,
`session_rust/src/spatial_octree.rs`, with its own minitests: run
`./bash/quicktest.sh spatial_octree`). Each octree node owns a spacing-limited SUBSAMPLE
of the cloud; the walk uploads points in octree order so every node is one contiguous
`(first, count)` range — which is exactly what a splat record already is. Per frame, a
screen-error walk picks nodes: far clouds stop at coarse ancestors (a few big dots),
near clouds descend to raw leaves. Zoomed out, a 13.8M-point scene costs 38k points;
zoomed in, you get every point back, pixel-exact.

Streamed clouds (lesson [43](43-streaming-cloud.md)) keep their whole-cloud record —
their points never exist on the CPU, so there is nothing to build a tree from. That is
import-side work for another day.

## How the kernel class maps to the lane

`SpatialOctree::from_coords(&coords, root_spacing, leaf_capacity)` reads the flat array
(stores nothing), grid-accepts at most one point per `spacing` cell per node (first point
wins — deterministic), sends the leftovers into octants at HALF the spacing, and absorbs
whole nodes below `leaf_capacity`. It hands back:

- `order()` — the permutation that makes every node's points contiguous,
- per node: `node_cube` (center + edge), `node_spacing`, `node_range`, `children`.

## Step 1 — the node row: `src/engine/gpu/mod.rs`

**Find**:

```rust
    pub spacing: f32,  // measured point spacing, world units (0 = unknown)
}
```

**Replace with:**

```rust
    pub spacing: f32,  // measured point spacing, world units (0 = unknown)
    pub node_first: u32, // first LodNode of this cloud in the nodes table (walked lane)
    pub node_count: u32, // 0 = no octree (streamed clouds) - the record covers everything
}

/// One octree node of a WALKED cloud (kernel SpatialOctree): its own spacing-limited
/// subsample as an absolute row range in the cloud tables, its cube for the screen-error
/// test, and the accept spacing that drives the attenuated splat radius. Children are
/// indices RELATIVE to the cloud's node slice; -1 = none.
#[derive(Clone, Copy)]
pub struct LodNode {
    pub center: [f32; 3], // cube center, cloud-LOCAL units
    pub size: f32,        // cube edge, cloud-local units
    pub spacing: f32,     // accept spacing, cloud-local units
    pub first: u32,       // absolute row in the cloud tables
    pub count: u32,
    pub children: [i32; 8],
}
```

## Step 2 — the tables carry the nodes

**Find**:

```rust
    pub draws: Vec<CloudDraw>,
}

impl CloudTables {
    pub fn new() -> Self {
        Self { pos: Vec::new(), col: Vec::new(), nrm: Vec::new(), draws: Vec::new() }
    }
}
```

**Replace with:**

```rust
    pub draws: Vec<CloudDraw>,
    pub nodes: Vec<LodNode>, // all clouds' octree nodes, sliced per draw by node_first/count
}

impl CloudTables {
    pub fn new() -> Self {
        Self { pos: Vec::new(), col: Vec::new(), nrm: Vec::new(), draws: Vec::new(), nodes: Vec::new() }
    }
}
```

**Find** (in `CloudLane` — the only `count` + `draws` pair):

```rust
    pub count: u32,
    pub draws: Vec<CloudDraw>,
}
```

**Replace with:**

```rust
    pub count: u32,
    pub draws: Vec<CloudDraw>,
    pub nodes: Vec<LodNode>,
}
```

**Find** (the `cloud:` literal in `Gpu::build`):

```rust
                count: point_count,
                draws: Vec::new(),
            },
```

**Replace with:**

```rust
                count: point_count,
                draws: Vec::new(),
                nodes: Vec::new(),
            },
```

**Find** (in `set_scene`):

```rust
        self.cloud.draws = up.clouds.draws.clone();
```

**Add below it:**

```rust
        self.cloud.nodes = up.clouds.nodes.clone();
```

## Step 3 — the eye reaches the record builder

The screen-error test needs the camera position in anchored world units — the same eye
`write_frame_uniforms` already solves for the pen uniforms, now cached.

**Find** (in `FrameUniforms`):

```rust
    pub mvp_f32: [f32; 16], // this frame's matrix, CPU-side - the splat records fold it
    pub last_ortho_h: f32,  // ortho half-height (0 = perspective), for the splat k
}
```

**Replace with:**

```rust
    pub mvp_f32: [f32; 16], // this frame's matrix, CPU-side - the splat records fold it
    pub last_ortho_h: f32,  // ortho half-height (0 = perspective), for the splat k
    pub last_eye: [f32; 3], // eye in anchored world units, for the LOD screen-error test
}
```

**Find** (the `frame:` literal):

```rust
                mvp_f32: [0.0; 16],
                last_ortho_h: 0.0,
            },
```

**Replace with:**

```rust
                mvp_f32: [0.0; 16],
                last_ortho_h: 0.0,
                last_eye: [0.0; 3],
            },
```

**Find** (in `write_frame_uniforms`):

```rust
        self.frame.mvp_f32 = view_proj.to_f32();
        self.frame.last_ortho_h = Self::ortho_half_height(view_proj);
```

**Add below it:**

```rust
        self.frame.last_eye = Self::eye_from_view_proj(view_proj);
```

**Find** (the pen uniform reuses it, a few lines down):

```rust
            eye: Self::eye_from_view_proj(view_proj),
```

**Replace with:**

```rust
            eye: self.frame.last_eye,
```

## Step 4 — the knob

**Find**:

```rust
    pub edl_strength: f32, // Eye-Dome Lighting strength; 0 = off (VIEWER_EDL)
```

**Add below it:**

```rust
    pub lod_split_px: f32, // octree LOD cutoff: descend while a node's spacing projects wider; 0 = off (VIEWER_LOD)
```

**Find** (the struct literal):

```rust
            edl_strength: std::env::var("VIEWER_EDL").ok().and_then(|v| v.parse().ok()).unwrap_or(0.25),
```

**Add below it:**

```rust
            lod_split_px: std::env::var("VIEWER_LOD").ok().and_then(|v| v.parse().ok()).unwrap_or(1.0),
```

## Step 5 — streamed clouds opt out

**Find** (in `cloud_begin`):

```rust
        self.stream.draws.push(CloudDraw { first: self.stream.count, count, instance, spacing: 0.0 });
```

**Replace with:**

```rust
        self.stream.draws.push(CloudDraw { first: self.stream.count, count, instance, spacing: 0.0, node_first: 0, node_count: 0 });
```

## Step 6 — the selection: `splat_records` learns LOD

**Find** `fn splat_records(&self, draws: &[CloudDraw]) -> ([u32; 4], Vec<u8>, u32) {` and
**replace the whole function** — down to and including its closing
`(header, recs, cum)` + `}` — **with:**

```rust
    fn splat_records(&self, draws: &[CloudDraw], nodes: &[LodNode]) -> ([u32; 4], Vec<u8>, u32) {
        let mut header = [0u32; 4];
        let mut recs: Vec<u8> = Vec::new();
        let mut cum = 0u32;
        let ortho_h = self.frame.last_ortho_h as f64;
        let vp_h = self.config.height as f64;
        let aspect = self.config.width as f64 / self.config.height as f64;
        let eye = self.frame.last_eye;
        for &CloudDraw { first, count, instance: inst, spacing, node_first, node_count } in draws {
            let Some(row) = self.objects.rows.get(inst as usize) else { continue };
            if row.flags & Instance::FLAG_HIDDEN != 0 { continue; }
            let px = if row.spacing > 0.0 { row.spacing } else { 3.0 } * self.cloud_size;
            if px <= 0.0 || header[0] >= 256 { continue; }
            // column-major 4x4: combined = mvp x model - the same per cloud, shared by
            // every record the cloud emits
            let (a, b) = (&self.frame.mvp_f32, &row.model);
            let mut m = [0.0f32; 16];
            for col in 0..4 {
                for r in 0..4 {
                    m[col * 4 + r] = (0..4).map(|k| a[k * 4 + r] * b[col * 4 + k]).sum();
                }
            }
            // tint.a smuggles the MINIMUM radius (the manifest px, halved): without a
            // floor, attenuation turns distant clouds to dust. With octree LOD a far node
            // carries BIGGER spacing (Potree's answer), but the floor still guards leaves.
            let tint = [row.color[0], row.color[1], row.color[2], (px * 0.5).max(0.5)];
            // spacing is in the cloud's LOCAL units; col0's length is the model scale
            let mscale = ((row.model[0] as f64).powi(2) + (row.model[1] as f64).powi(2) + (row.model[2] as f64).powi(2)).sqrt();
            // one record = one contiguous range at one spacing. world radius = spacing x
            // (px/6); k folds the projection so the shader only divides by clip.w:
            //   perspective: r_px = world_r * cot(fov/2) * (vp_h/2) / w
            //   ortho:       r_px = world_r * vp_h / (2*ortho_h), and w = 1
            let emit = |f: u32, c: u32, sp: f32, recs: &mut Vec<u8>, header: &mut [u32; 4], cum: &mut u32| {
                if header[0] >= 256 { return; }
                recs.extend_from_slice(bytemuck::cast_slice(&m));
                recs.extend_from_slice(bytemuck::cast_slice(&tint));
                let world_r = (sp as f64).max(1.0e-9) * mscale * 0.001 * (px as f64) / 6.0; // metres
                let k = if ortho_h > 0.0 { world_r / (2.0 * ortho_h) }
                        else { world_r * 1.7320508 * 0.5 }; // cot(30 deg) / 2
                recs.extend_from_slice(bytemuck::cast_slice(&[f, c, *cum, (k as f32).to_bits()]));
                // the MODEL rotation columns (translation-free), so a cloud with
                // normals can rotate them into world space for the lambert term
                recs.extend_from_slice(bytemuck::cast_slice(&[
                    b[0], b[1], b[2], 0.0f32,
                    b[4], b[5], b[6], 0.0,
                    b[8], b[9], b[10], 0.0,
                ]));
                header[0] += 1;
                *cum += c;
            };
            if self.lod_split_px > 0.0 && node_count > 0 {
                // Octree LOD, Potree-style screen-error selection: every VISITED node
                // contributes its own subsample, and the walk descends while the node's
                // projected point spacing is coarser than the cutoff - far nodes stop at
                // the root (a handful of coarse points), near nodes go deep. Coarse nodes
                // carry big spacing, so attenuation grows their dots to close the gaps.
                let slice = &nodes[node_first as usize..(node_first + node_count) as usize];
                let mut stack: Vec<usize> = vec![0];
                while let Some(ni) = stack.pop() {
                    if header[0] >= 256 { break; }
                    let nd = slice[ni];
                    let c = nd.center;
                    // FRUSTUM CULL on the node's bounding sphere, in clip space through the
                    // folded matrix: an off-screen subtree costs nothing - and without this
                    // a close zoom would visit every node and starve the 256-record table.
                    let r_m = nd.size as f64 * 0.8660254 * mscale * 0.001; // sphere radius, metres
                    let cw = (m[3] * c[0] + m[7] * c[1] + m[11] * c[2] + m[15]) as f64;
                    if ortho_h <= 0.0 && cw < -r_m { continue; } // fully behind the eye
                    let cx = (m[0] * c[0] + m[4] * c[1] + m[8] * c[2] + m[12]) as f64;
                    let cy = (m[1] * c[0] + m[5] * c[1] + m[9] * c[2] + m[13]) as f64;
                    let (ndc_x, ndc_y, ry) = if ortho_h > 0.0 {
                        (cx, cy, r_m / ortho_h)
                    } else {
                        let w = cw.max(1.0e-9);
                        (cx / w, cy / w, r_m * 1.7320508 / w)
                    };
                    if ndc_x.abs() > 1.0 + ry / aspect.min(1.0) || ndc_y.abs() > 1.0 + ry {
                        continue; // the whole subtree is outside the view
                    }
                    // node center in anchored world units - the eye's space
                    let w = [
                        row.model[0] * c[0] + row.model[4] * c[1] + row.model[8] * c[2] + row.model[12],
                        row.model[1] * c[0] + row.model[5] * c[1] + row.model[9] * c[2] + row.model[13],
                        row.model[2] * c[0] + row.model[6] * c[1] + row.model[10] * c[2] + row.model[14],
                    ];
                    let dist_m = (((w[0] - eye[0]).powi(2) + (w[1] - eye[1]).powi(2) + (w[2] - eye[2]).powi(2)) as f64).sqrt() * 0.001;
                    let sp_m = nd.spacing as f64 * mscale * 0.001;
                    let sp_px = if ortho_h > 0.0 { sp_m * vp_h / (2.0 * ortho_h) }
                                else { sp_m * 1.7320508 * 0.5 * vp_h / dist_m.max(1.0e-9) };
                    let leaf = nd.children.iter().all(|&ch| ch < 0);
                    let refine = !leaf && sp_px > self.lod_split_px as f64;
                    // Dot size: a REFINED node's region also receives all its deeper
                    // points, so its own subsample renders at the cloud's measured
                    // spacing - otherwise coarse dots blob over the fine layer under
                    // them. Only the unrefined FRINGE keeps its coarse node spacing
                    // (its points are the only ink there - big dots close the gaps);
                    // a node can never be DENSER than the raw cloud, so the measured
                    // spacing is also the floor there. Leaves hold raw points.
                    let sp = if refine || leaf { spacing } else { nd.spacing.max(spacing) };
                    emit(nd.first, nd.count, sp, &mut recs, &mut header, &mut cum);
                    if refine {
                        for &ch in &nd.children {
                            if ch >= 0 { stack.push(ch as usize); }
                        }
                    }
                }
            } else {
                emit(first, count, spacing, &mut recs, &mut header, &mut cum);
            }
        }
        header[1] = cum;
        (header, recs, cum)
    }
```

Three design points hiding in there, each paid for in debugging:

- **The frustum cull is load-bearing, not an optimisation.** Without it a close zoom
  visits every node, and the 256-record table silently truncates — the lion lost 12k
  pixels to exactly this before the cull went in.
- **A refined node renders at the MEASURED spacing.** Its region also receives all its
  deeper points, so its subsample is part of a full-density picture; give it the coarse
  node spacing and its big dots blob OVER the fine layer (the depth race lets the nearer
  big dot win). Only the unrefined fringe — where a node's points are the only ink —
  keeps the coarse spacing that closes the gaps.
- **The walk emits VISITED nodes, not just leaves.** The union of ancestors' subsamples
  plus the fringe is the whole picture at bounded density — that is Potree's additive
  hierarchy.

**Find** (first call site, in `encode_frame`):

```rust
            let (header, recs, cum) = self.splat_records(&self.cloud.draws);
```

**Replace with:**

```rust
            let (header, recs, cum) = self.splat_records(&self.cloud.draws, &self.cloud.nodes);
```

**Find** (the stream lane call — no CPU points, no tree):

```rust
            let (header_s, recs_s, cum_s) = self.splat_records(&self.stream.draws);
```

**Replace with:**

```rust
            let (header_s, recs_s, cum_s) = self.splat_records(&self.stream.draws, &[]);
```

## Step 7 — the walk builds the tree: `src/app/scene.rs`

**Find**:

```rust
use session_rust::{Session, Geometry, Mesh, Line, Point, Polyline, NurbsCurve, RenderVertex, Plane, OBB, PointCloud, Vector};
```

**Replace with:**

```rust
use session_rust::{Session, Geometry, Mesh, Line, Point, Polyline, NurbsCurve, RenderVertex, Plane, OBB, PointCloud, SpatialOctree, Vector};
```

**Find**:

```rust
use crate::engine::gpu::{ArenaUpload, CloudDraw, Instance, CylinderSegment, GlyphPoint, ObjectBase};
```

**Replace with:**

```rust
use crate::engine::gpu::{ArenaUpload, CloudDraw, Instance, CylinderSegment, GlyphPoint, LodNode, ObjectBase};
```

**Find** (the cloud arm):

```rust
                Geometry::PointCloud(pc) => {
                    let first = (t.clouds.pos.len() / 3) as u32;
                    push_cloud(pc, &mut t.clouds.pos, &mut t.clouds.col, &mut t.clouds.nrm);
                    t.clouds.draws.push(CloudDraw { first, count: pc.len() as u32, instance: ri, spacing: cloud_spacing(pc) });
```

**Replace with:**

```rust
                Geometry::PointCloud(pc) => {
                    let first = (t.clouds.pos.len() / 3) as u32;
                    let node_first = t.clouds.nodes.len() as u32;
                    push_cloud(pc, &mut t.clouds);
                    let node_count = t.clouds.nodes.len() as u32 - node_first;
                    t.clouds.draws.push(CloudDraw { first, count: pc.len() as u32, instance: ri, spacing: cloud_spacing(pc), node_first, node_count });
```

**Find** `/// The raw lane's rows, written STRAIGHT into the shared table` and **replace
the whole `push_cloud` function including that comment with:**

```rust
/// The raw lane's rows, written STRAIGHT into the shared table in OCTREE ORDER: the
/// kernel SpatialOctree hands back a permutation where every LOD node's subsample is
/// contiguous, so a node is one (first, count) splat record. Still the kernel's FLAT
/// arrays (no per-point allocs), still one peak, not two.
fn push_cloud(pc: &PointCloud, t: &mut crate::engine::gpu::CloudTables) {
    let coords = pc.coords();
    let colors = pc.colors();
    let normals = pc.normals();
    let n = pc.len();
    // Octree knobs: root accept spacing = cube/64 (the root's own subsample is a coarse
    // sketch), leaves absorb below 8192 points so shallow clouds stay one node.
    let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
    for i in 0..n {
        for k in 0..3 {
            lo[k] = lo[k].min(coords[i * 3 + k]);
            hi[k] = hi[k].max(coords[i * 3 + k]);
        }
    }
    let size = (hi[0] - lo[0]).max(hi[1] - lo[1]).max(hi[2] - lo[2]).max(1.0e-9);
    let tree = SpatialOctree::from_coords(coords, size / 64.0, 8192);
    let base = (t.pos.len() / 3) as u32;
    t.pos.reserve(n * 3);
    t.col.reserve(n);
    t.nrm.reserve(n);
    for &i in tree.order() {
        t.pos.push(coords[i * 3] as f32);
        t.pos.push(coords[i * 3 + 1] as f32);
        t.pos.push(coords[i * 3 + 2] as f32);
        // Normal, oct16-packed into 16 bits (same encoding as the edge facing words).
        // All-ones = this point HAS no normal: a scan without them still pays the 4 B,
        // but the shading branch stays uniform per cloud, which is what the GPU wants.
        t.nrm.push(if i * 3 + 2 < normals.len() {
            let v = Vector::new(normals[i * 3], normals[i * 3 + 1], normals[i * 3 + 2]);
            oct16(&v).unwrap_or(u32::MAX)
        } else {
            u32::MAX
        });
        let c = i * 4;
        // The colour is 8-bit at the source (proto 0-255): pack it back to the four bytes it
        // is, instead of four f32s carrying four bytes of information.
        t.col.push(if c + 3 < colors.len() {
            (colors[c] as u32 & 255) | (colors[c + 1] as u32 & 255) << 8
                | (colors[c + 2] as u32 & 255) << 16 | (colors[c + 3] as u32 & 255) << 24
        } else {
            0xff00_0000
        });
    }
    for ni in 0..tree.node_count() {
        let (center, sz) = tree.node_cube(ni);
        let (f, count) = tree.node_range(ni);
        let mut children = [-1i32; 8];
        for (slot, &ch) in tree.children(ni).iter().enumerate() {
            children[slot] = ch as i32;
        }
        t.nodes.push(LodNode {
            center: [center[0] as f32, center[1] as f32, center[2] as f32],
            size: sz as f32,
            spacing: tree.node_spacing(ni) as f32,
            first: base + f as u32,
            count: count as u32,
            children,
        });
    }
}
```

## Why LOD off is pixel-identical

`VIEWER_LOD=0` emits one whole-cloud record, exactly as before — but the points are now
in octree order. That cannot change the image: the depth pass keeps the MAX reverse-Z
bits per pixel over the same point set, and max is order-free. Which makes LOD off a
true reference switch: any pixel that differs with it ON was changed by selection, not by
the reorder.

## Expected state

- Kernel: `./bash/quicktest.sh spatial_octree` — 9/9 in all three languages.
- `cargo check --target wasm32-unknown-unknown --lib` clean; shaders untouched.
- `VIEWER_LOD=0`: all three lesson-43 goldens EXACTLY — lion 325369, cloud_mix 12143,
  lidar14 3798.
- LOD on (default 1.0), measured twice:

```
lion.json     (ZOOM=6 close-up)  non-background pixels: 325369 (33.9%)  <- IDENTICAL to full
cloud_mix.json (fit)             non-background pixels: 11887 (1.1%)    92,331 of 7,492,706 points
lidar14.json   (fit)             non-background pixels: 3548 (0.3%)     37,972 of 13,793,783 points
```

  The lion close-up selects 127 records that happen to cover all 341,989 points — full
  detail because you are close. The fit views draw 66 and 29 records: **80-360× fewer
  points for ~95% of the ink**.
- The 13.8M native walk now takes ~10.3 s (octree build included) — native only; a cloud
  that big STREAMS in the browser and skips the tree. The browser's walked clouds
  (≤1.5M) build in well under a second.
- Knobs: `VIEWER_LOD=<px>` — descend while a node's spacing projects wider than this
  (bigger = coarser = faster; 0 = off). `[` `]` still scale dot size on top.
