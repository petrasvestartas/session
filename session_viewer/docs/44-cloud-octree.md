# 44 Cloud octree — Potree's LOD, on our splat lane

> Replay-verified against the end-of-43 tree; every number below measured twice on it
> (both passes agreed). **The build half is now obsolete — see below.**

## Goal

Lessons 40/41 left one Potree feature on the table: the octree. Each node owns a
spacing-limited SUBSAMPLE of the cloud; per frame a screen-error walk picks nodes, so far
clouds stop at coarse ancestors and near clouds descend to raw leaves. Zoomed out a 13.8 M
scene costs tens of thousands of points; zoomed in you get every point back, pixel-exact.

## What this lesson still teaches, and what moved (2026-09-03)

The tree is no longer built in the browser. `PointCloud` carries it (lesson
[43](43-streaming-cloud.md), *What changed in the kernel*), so a `.pb` arrives with the node
table already in it.

- **Obsolete: Step 7**, the walk building the tree with `SpatialOctree::from_coords`. The
  exporter does it once, offline, via `build_lod`. A browser paying 10 s per 14 M cloud to
  recompute what the file could have carried was always the wrong trade.
- **Obsolete: the old premise** — *"streamed clouds have no CPU points, so there is nothing
  to build a tree from"*. Nothing needs building. Points arrive in node order, so LOD and
  streaming stop being alternatives.
- **Still exact: Steps 1-6** — `CloudDraw`, `LodNode`, the node table beside the draw list,
  the eye reaching the record builder, and the screen-error selection in `splat_records`.
  That is the half of the lesson that renders, and none of it cares where the tree came from.
- **Better than before:** because points are stored in node order, a `Range` request can
  fetch exactly the nodes the camera wants. The old design downloaded every point and then
  drew a subsample of it; far detail is now never downloaded at all.

Read Steps 1-6 as written. Read Step 7 as history: substitute "read `lod_*` off the decoded
`PointCloud`" for "call `from_coords` and permute", and the rest of the lane is unchanged —
`LodNode` has the same six fields the proto now carries.

## How the kernel class maps to the lane

`SpatialOctree::from_coords(&coords, root_spacing, leaf_capacity)` reads the flat array
(stores nothing), grid-accepts at most one point per `spacing` cell per node (first point
wins — deterministic), sends the leftovers into octants at HALF the spacing, and absorbs
whole nodes below `leaf_capacity`. It hands back:

- `order()` — the permutation that makes every node's points contiguous,
- per node: `node_cube` (centre + edge), `node_spacing`, `node_range`, `children`.

It is a real 3-language kernel class with its own minitests
(`./bash/quicktest.sh spatial_octree`), read-only here. Do not change it.

## The tuple has to go first

The draw record has been a `(first, count, instance, spacing)` tuple since lesson 36. The
octree gives every cloud a SECOND range — its slice of the node table — and six positional
fields stop being readable. So step 1 converts the tuple to a named struct across all three
holders (`ArenaUpload`, the walked lane, the stream lane) before anything else happens.

## Step 1 — the two row types: `src/engine/gpu/mod.rs`

**Find** in `src/engine/gpu/mod.rs`:

```rust
pub struct ArenaUpload{
```

**Add above it:**

```rust
/// One cloud's contiguous point range, as the record builder sees it. It was a
/// `(first, count, instance, spacing)` tuple until the octree gave every cloud a second
/// range - its slice of the LOD node table - and six positional fields is where a tuple
/// stops being readable.
#[derive(Clone, Copy)]
pub struct CloudDraw {
    pub first: u32,      // absolute first row in the cloud tables
    pub count: u32,
    pub instance: u32,   // the instance row this cloud draws against
    pub spacing: f32,    // measured point spacing, world units (0 = unknown)
    pub node_first: u32, // first LodNode of this cloud in the nodes table (walked lane)
    pub node_count: u32, // 0 = no octree (streamed clouds) - the record covers everything
}

/// One octree node of a WALKED cloud (kernel `SpatialOctree`): its own spacing-limited
/// subsample as a row range, its cube for the screen-error test, and the accept spacing
/// that drives the attenuated splat radius. `first` is RELATIVE to the cloud's own first
/// point and `children` are indices RELATIVE to the cloud's node slice; -1 = none.
#[derive(Clone, Copy)]
pub struct LodNode {
    pub center: [f32; 3], // cube centre, cloud-LOCAL units
    pub size: f32,        // cube edge, cloud-local units
    pub spacing: f32,     // accept spacing, cloud-local units
    pub first: u32,       // row offset from the draw's own `first`
    pub count: u32,
    pub children: [i32; 8],
}

```

Both rows are `Copy` on purpose: `set_scene` rebases every draw on the way in, and the
record walk copies one node per visit.

**Replace-all** `src/engine/gpu/mod.rs` `Vec<(u32, u32, u32, f32)>` → `Vec<CloudDraw>` (3 hits)

Three holders, one rename: the upload table, the walked lane's draw list and the stream
lane's.

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub cloud_draws: Vec<CloudDraw>,
```

**Add above it:**

```rust
    pub cloud_nodes: Vec<LodNode>, // every walked cloud's octree nodes; a draw owns one slice
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    cloud_draws: Vec<CloudDraw>,
```

**Add above it:**

```rust
    cloud_nodes: Vec<LodNode>,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            idx_print: Vec::new(),
```

**Add above it:**

```rust
            cloud_nodes: Vec::new(),
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            point_count,
```

**Add above it:**

```rust
            cloud_nodes: Vec::new(),
```

The node table is a fifth cloud column beside `cloud_pos`/`cloud_col`/`cloud_nrm`/
`cloud_draws`, mirrored on the `Gpu`. It never reaches the GPU: the screen-error walk is a
CPU pass over a few thousand rows per frame.

## Step 2 — the node table reaches the lane: `src/engine/gpu/mod.rs`

**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.cloud_draws.extend_from_slice(&up.cloud_draws);
```

**Replace with:**

```rust
        // The walk numbers a cloud's nodes from the start of ITS upload; the lane's table is
        // cumulative, so every draw's node slice is rebased on the way in - the same thing
        // `Scene::cloud_base` already does for the point rows.
        let node_base = self.cloud_nodes.len() as u32;
        self.cloud_nodes.extend_from_slice(&up.cloud_nodes);
        self.cloud_draws.extend(up.cloud_draws.iter().map(|d| CloudDraw { node_first: d.node_first + node_base, ..*d }));
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.cloud_draws.clear();
```

**Add below it:**

```rust
        self.cloud_nodes.clear();
```

A node slice that outlived its draws would hand the record walk indices into a table that
no longer describes the scene.

## Step 3 — the tuple's remaining users

Five positional field accesses. Naming them is the whole change.

**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.stream_draws.push((self.stream_count, count, instance, 0.0));
```

**Replace with:**

```rust
        self.stream_draws.push(CloudDraw { first: self.stream_count, count, instance, spacing: 0.0, node_first: 0, node_count: 0 });
```

`node_count: 0` is how a streamed cloud opts out: no CPU points, no tree, one record for
the whole run — an empty slice, not a special case in the record builder.

**Find** in `src/engine/gpu/mod.rs`:

```rust
            if d.3 == 0.0 && self.stream_pos_at == d.0 && pos.len() >= 6 {
```

**Replace with:**

```rust
            if d.spacing == 0.0 && self.stream_pos_at == d.first && pos.len() >= 6 {
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
                    d.3 = gaps[gaps.len() / 2];
```

**Replace with:**

```rust
                    d.spacing = gaps[gaps.len() / 2];
```

**Find** in `src/app/scene.rs`:

```rust
                d.2 = row;
```

**Replace with:**

```rust
                d.instance = row;
```

**Find** in `src/app/scene.rs`:

```rust
        for &(first, count, inst, _) in t.cloud_draws.iter().skip(draw0){
```

**Replace with:**

```rust
        for &CloudDraw { first, count, instance: inst, .. } in t.cloud_draws.iter().skip(draw0){
```

The bounds sweep wants three of six fields, so it names three and `..` the rest — the read
a six-field tuple could not have expressed.

## Step 4 — the eye reaches the record builder: `src/engine/gpu/mod.rs`

The screen-error test needs the camera position in anchored world units — the same eye
`write_frame_uniforms` already solves for the pen uniform, now cached.

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub cloud_bind_group: wgpu::BindGroup,
```

**Add above it:**

```rust
    last_eye: [f32; 3], // eye in anchored world units, for the LOD screen-error test
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            cloud_bind_group,
```

**Add above it:**

```rust
            last_eye: [0.0; 3],
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.last_ortho_h = Self::ortho_half_height(view_proj);
```

**Add below it:**

```rust
        self.last_eye = Self::eye_from_view_proj(view_proj);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            eye: Self::eye_from_view_proj(view_proj),
```

**Replace with:**

```rust
            eye: self.last_eye,
```

The pen uniform used to solve the eye a second time. Both readers now take the cached one,
so there is exactly one place where "this frame's eye" is decided.

## Step 5 — the knob: `src/engine/gpu/mod.rs`

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub cloud_bind_group: wgpu::BindGroup,
```

**Add above it:**

```rust
    pub lod_split_px: f32, // octree LOD cutoff: descend while a node's spacing projects wider; 0 = off (VIEWER_LOD)
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            cloud_bind_group,
```

**Add above it:**

```rust
            lod_split_px: std::env::var("VIEWER_LOD").ok().and_then(|v| v.parse().ok()).unwrap_or(1.0),
```

It lands in the cloud knob block beside `cloud_size` and `edl_strength`. `VIEWER_LOD=0`
turns selection off, which is what makes the reference switch below testable.

## Step 6 — the selection: `splat_records` learns LOD

The record builder gains the node table and, per cloud, a screen-error walk over it.

**Find** in `src/engine/gpu/mod.rs`:

```rust
draws: &[(u32, u32, u32, f32)]
```

**Replace with:**

```rust
draws: &[CloudDraw], nodes: &[LodNode]
```

**Remove** `src/engine/gpu/mod.rs` `        let mut header = [0u32; 4];` **through** `    }`

That cuts the old body out from under the signature you just renamed; the replacement goes
straight back in below it.

**Find** in `src/engine/gpu/mod.rs`:

```rust
-> ([u32; 4], Vec<u8>, u32) {
```

**Add below it:**

```rust
        let mut header = [0u32; 4];
        let mut recs: Vec<u8> = Vec::new();
        let mut cum = 0u32;
        let ortho_h = self.last_ortho_h as f64;
        let vp_h = self.config.height as f64;
        let aspect = self.config.width as f64 / self.config.height as f64;
        let eye = self.last_eye;
        for &CloudDraw { first, count, instance: inst, spacing, node_first, node_count } in draws {
            let Some(row) = self.instances.get(inst as usize) else { continue };
            if row.flags & Instance::FLAG_HIDDEN != 0 { continue; }
            let px = if row.spacing > 0.0 { row.spacing } else { 3.0 } * self.cloud_size;
            if px <= 0.0 || header[0] >= 256 { continue; }
            // column-major 4x4: combined = mvp x model - one per cloud, shared by every
            // record the cloud emits
            let (a, b) = (&self.mvp_f32, &row.model);
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
                    // node centre in anchored world units - the eye's space
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
                    // `nd.first` is relative to this cloud's own first point
                    emit(first + nd.first, nd.count, sp, &mut recs, &mut header, &mut cum);
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

Three design points, each paid for in debugging:

- **The frustum cull is load-bearing, not an optimisation.** A close zoom pushes most of
  the tree off screen, and every off-screen node the walk visits still spends a record out
  of 256. On the lion close-up, `VIEWER_ZOOM=60` costs 64 records with the cull and 84
  without, for the SAME 663485 ink pixels. Twenty wasted records is nothing at 342k points
  and everything on a scene of scans, where a saturated table truncates the picture with no
  error anywhere.
- **A refined node renders at the MEASURED spacing.** Its region also receives all its
  deeper points, so give it the coarse node spacing and its big dots blob OVER the fine
  layer (the depth race lets the nearer big dot win). Only the unrefined fringe keeps the
  coarse spacing that closes the gaps.
- **The walk emits VISITED nodes, not just leaves.** Ancestors' subsamples plus the fringe
  are the whole picture at bounded density — Potree's additive hierarchy.

**Find** in `src/engine/gpu/mod.rs`:

```rust
self.splat_records(&self.cloud_draws)
```

**Replace with:**

```rust
self.splat_records(&self.cloud_draws, &self.cloud_nodes)
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
self.splat_records(&self.stream_draws)
```

**Replace with:**

```rust
self.splat_records(&self.stream_draws, &[])
```

The stream lane passes an empty node table and every one of its draws carries
`node_count: 0`, so the walk is never entered. One function, two lanes, no flag.

## Step 7 — the walk builds the tree: `src/app/scene.rs`

**Find** in `src/app/scene.rs`:

```rust
PointCloud, Vector, Tolerance};
```

**Replace with:**

```rust
PointCloud, SpatialOctree, Vector, Tolerance};
```

**Find** in `src/app/scene.rs`:

```rust
use crate::engine::gpu::{ArenaUpload,
```

**Replace with:**

```rust
use crate::engine::gpu::{ArenaUpload, CloudDraw, LodNode,
```

**Find** in `src/app/scene.rs`:

```rust
                    let first = cb + (t.cloud_pos.len() / 3) as u32;
```

**Add below it:**

```rust
                    let node_first = t.cloud_nodes.len() as u32;
```

**Find** in `src/app/scene.rs`:

```rust
&mut t.cloud_col, &mut t.cloud_nrm);
```

**Replace with:**

```rust
&mut t.cloud_col, &mut t.cloud_nrm, &mut t.cloud_nodes);
```

**Find** in `src/app/scene.rs`:

```rust
                    t.cloud_draws.push((first, pc.len() as u32, ri, cloud_spacing(pc)));
```

**Replace with:**

```rust
                    let node_count = t.cloud_nodes.len() as u32 - node_first;
                    t.cloud_draws.push(CloudDraw { first, count: pc.len() as u32, instance: ri, spacing: cloud_spacing(pc), node_first, node_count });
```

The cloud arm brackets `push_cloud` with the node table's length: what the walk appended in
between IS this cloud's slice. No counter, no second pass.

**Find** in `src/app/scene.rs`:

```rust
        drop_rows(&mut t.cloud_draws);
```

**Add below it:**

```rust
        drop_rows(&mut t.cloud_nodes);
```

The node column is a per-upload delta like the point columns — the walk counts from zero
every upload, which is why `set_scene` rebases `node_first` when it takes it.

**Find** in `src/app/scene.rs`:

```rust
nrm: &mut Vec<u32>){
```

**Replace with:**

```rust
nrm: &mut Vec<u32>, nodes: &mut Vec<LodNode>){
```

**Find** in `src/app/scene.rs`:

```rust
    nrm.reserve(n);
```

**Add below it:**

```rust
    // The LOD octree, built ONCE and read twice: `order()` is the permutation that makes
    // every node's points contiguous, and the node table is this walk's second output.
    // Root accept spacing = the cube over 64 (the root's own subsample is a coarse
    // sketch); leaves absorb below 8192 points, so a shallow cloud stays one node.
    let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
    for i in 0..n {
        for k in 0..3 {
            lo[k] = lo[k].min(coords[i * 3 + k]);
            hi[k] = hi[k].max(coords[i * 3 + k]);
        }
    }
    let size = (hi[0] - lo[0]).max(hi[1] - lo[1]).max(hi[2] - lo[2]).max(1.0e-9);
    let tree = SpatialOctree::from_coords(coords, size / 64.0, 8192);
    // `first` is RELATIVE to this cloud's own first point, exactly as `children` are
    // relative to the cloud's node slice: the record builder adds the draw's base, so a
    // cloud can be re-uploaded at a different offset without rewriting its nodes.
    for ni in 0..tree.node_count() {
        let (center, sz) = tree.node_cube(ni);
        let (f, count) = tree.node_range(ni);
        // `children` hands back only the octants that exist, so the empty slots stay -1
        // and the record walk skips them; which octant a child was is nothing the
        // screen-error test asks about.
        let mut children = [-1i32; 8];
        for (slot, &ch) in tree.children(ni).iter().enumerate() {
            children[slot] = ch as i32;
        }
        nodes.push(LodNode {
            center: [center[0] as f32, center[1] as f32, center[2] as f32],
            size: sz as f32,
            spacing: tree.node_spacing(ni) as f32,
            first: f as u32,
            count: count as u32,
            children,
        });
    }
```

**Find** in `src/app/scene.rs`:

```rust
    for i in 0..n{
```

**Replace with:**

```rust
    for &i in tree.order(){
```

That one-line swap is the whole upload change: rows go out in octree order, and `i` still
indexes the kernel's flat arrays, so every push below it is untouched. Still no per-point
allocation and still one peak — the tree stores nothing, it only borrows `coords` while it
builds.

## Why LOD off is the reference switch

`VIEWER_LOD=0` emits one whole-cloud record exactly as before, but with the points in
octree order. That cannot change the image: the depth pass keeps the MAX reverse-Z bits per
pixel over the same point set, and max is order-free. Only the colour claim can notice, and
only where two points tie on depth — the same race two runs of the unchanged binary already
lose to each other. So ink with LOD off is the end-of-43 ink, and any ink that moves with it
ON was moved by selection, not by the reorder.

## Expected state

- Kernel: `./bash/quicktest.sh spatial_octree` — 9/9 in all three languages.
- `cargo check --target wasm32-unknown-unknown --lib` clean; shaders untouched.
- The SELECTION, at default `VIEWER_LOD=1.0` and 1200x800, two runs agreeing to the record:

```
lion.toml       (VIEWER_ZOOM=6, close-up)   127 records    341,989 of    341,989 points
cloud_mix.toml  (fit)                        66 records     92,331 of  7,492,706 points
lidar14.toml    (fit)                        29 records     37,972 of 13,793,783 points
```

  The lion close-up covers ALL its points — full detail because you are close. The fit
  views draw **81× and 363× fewer points** for **99% and 93% of the ink**.
- The INK, same harness, each row measured twice:

```
scene                        end-of-43   VIEWER_LOD=0   LOD on (1.0)
lion.toml   (ZOOM=6)            319965        319965         319965
cloud_mix.toml (fit)              9009          9009           8899
lidar14.toml   (fit)              4611          4612           4294
```

  `VIEWER_LOD=0` reproduces the end-of-43 ink exactly on two scenes and to a single pixel
  on lidar14, and that pixel is not the reorder: two runs of the SAME end-of-43 binary
  already differ by a few pixels (3 on the lion, 0-2 elsewhere), because `cs_color` claims
  a pixel through an atomic race. Compare ink counts, not file hashes — the .ppm sha is not
  reproducible run to run, with or without this lesson.
- The octree is the walk's new cost, paid on the CPU at load: the lion's 342k points go
  from ~14-28 ms to ~85-107 ms, and the 13.8M scan walks in 10.3 s (10.36 / 10.33 on two
  runs). Native only — a cloud that big STREAMS in the browser and skips the tree, and the
  browser's walked clouds (≤1.5M) build in well under a second.
- Knobs: `VIEWER_LOD=<px>` — descend while a node's spacing projects wider than this
  (bigger = coarser = faster; 0 = off). `[` `]` still scale dot size on top.
- **Every number above is the NATIVE harness.** `lion.toml` / `cloud_mix.toml` /
  `lidar14.toml` are local manifests only it reads. No browser equivalent: `VIEWER_LOD` and
  `VIEWER_ZOOM` are env vars, and `std::env::var` always fails on wasm.

  ```
  VIEWER_LOD=1.0 VIEWER_ZOOM=6 cargo run -q --example selftest \
      --target x86_64-unknown-linux-gnu --release -- /tmp/out.ppm assets/scenes/lion.toml
  ```

- **In the browser** the scene comes from the branch (lesson
  [43](43-streaming-cloud.md), *Where a scene comes from*). Branch clouds get LOD like local
  ones, up to ~1.5M points — larger streams and skips the tree, and >100 MB cannot be committed.
  `?scene=scenes/lion.toml` pins a local manifest instead.

## Recap

- The kernel owns the octree; the viewer owns the selection. `order()` is the whole
  contract — write the points in it and a node becomes a splat record.
- A tuple stops being a data structure at about four fields. Name it before you grow it.
- Emit every VISITED node, not just the leaves: the additive hierarchy is what makes the
  fringe's coarse dots and the interior's fine dots add up to one picture.
- Cull the subtree before you test its error, or a close zoom starves the record table.
- `VIEWER_LOD=0` is a reference switch, not a fallback. Keep it working.

## Next

Lesson [44](45-pipeline-descs.md) — **pipelines are data.** This closes the point-cloud chain and
opens the refactor block: lessons 44-50 split `gpu/mod.rs` into one file per row family and
`scene.rs` into one file per geometry type under a pixel gate, then lesson 50 fixes what the
performance audit found.
