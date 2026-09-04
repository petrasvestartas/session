# 07 Point clouds: the splat lane and its octree

- At the end the lion cloud of the local scene draws as round, lit splats with Eye-Dome Lighting, `[` and `]` scale every splat, and a cloud that carries the kernel's octree draws by level of detail under `?lod=`.
- Points get a lane of their own, and the lane gets a pass of its own: one pixel-aligned quad per point into a private 1x depth + colour pair, then ONE fullscreen triangle copies the nearest point per pixel into the scene pass through `frag_depth`, so points and solids occlude each other exactly.
- A record per visible cloud (or per selected octree node) folds camera x placement, tint and radius on the CPU once per frame; the shader does one mat-vec per point and finds its record by binary search. The point pass is skipped while the camera, the knobs and the tables are what they were.
- The octree is read off the file, never built here: the kernel orders the points so that every node's range is its own subsample, and the walk descends only where a node's spacing still projects wider than the cutoff.
- The producer stays app-side (`walk/cloud.rs` fills `CloudRows`) and the lane stays engine-side (`gpu/cloud.rs`, `lod.rs`, `splat.rs`); the manifest's `point_size` rides `FileDoc` -> `WalkCx` -> the object row, so the engine never reads a manifest.

<svg viewBox="0 0 720 340" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="lesson 7 on the two-halves map: walk/cloud.rs fills Upload.cloud; the engine side gains cloud.rs, lod.rs, splat.rs and two shaders; the frame runs the point pass before the scene pass" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <defs><marker id="a7" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#444"/></marker></defs>
  <g fill="#222" font-size="11">
    <text x="14" y="18">app/  knows what a Geometry IS</text>
    <text x="266" y="18">Upload</text>
    <text x="418" y="18">engine/  knows what a ROW IS</text>
  </g>
  <line x1="14" y1="24" x2="706" y2="24" stroke="#444"/>
  <g fill="none" stroke="#1a7f4b" stroke-width="1.3"><rect x="14" y="36" width="230" height="52"/></g>
  <g fill="#222" font-size="11"><text x="22" y="52">walk/cloud.rs  (new)</text></g>
  <g fill="#555" font-size="10">
    <text x="22" y="67">walk_cloud: push_points, push_nodes</text>
    <text x="22" y="80">cloud_spacing -> Row.spacing = point px</text>
  </g>
  <g fill="none" stroke="#444"><rect x="14" y="100" width="230" height="52"/></g>
  <g fill="#222" font-size="11"><text x="22" y="116">manifest.rs -> loader.rs -> scene.rs</text></g>
  <g fill="#555" font-size="10">
    <text x="22" y="131">point_size -> FileDoc.point_px</text>
    <text x="22" y="144">-> Doc.point_px -> WalkCx.cloud_px</text>
  </g>
  <g fill="none" stroke="#444"><rect x="14" y="164" width="230" height="40"/></g>
  <g fill="#222" font-size="11"><text x="22" y="180">input.rs / state.rs</text></g>
  <g fill="#555" font-size="10"><text x="22" y="195">[ ] -> State::set_cloud_size</text></g>
  <line x1="244" y1="62" x2="262" y2="62" stroke="#444" marker-end="url(#a7)"/>
  <g fill="none" stroke="#444"><rect x="264" y="36" width="130" height="80"/></g>
  <g fill="#222" font-size="11"><text x="272" y="52">upload.rs</text></g>
  <g fill="#555" font-size="10">
    <text x="272" y="67">cloud: CloudRows</text>
    <text x="272" y="82">pos, col, nrm</text>
    <text x="272" y="95">draws: CloudDraw</text>
    <text x="272" y="108">nodes: LodNode</text>
  </g>
  <line x1="394" y1="62" x2="414" y2="62" stroke="#444" marker-end="url(#a7)"/>
  <g fill="none" stroke="#1a7f4b" stroke-width="1.3">
    <rect x="416" y="36" width="290" height="52"/>
    <rect x="416" y="100" width="140" height="52"/>
    <rect x="566" y="100" width="140" height="52"/>
    <rect x="416" y="164" width="290" height="40"/>
  </g>
  <g fill="#222" font-size="11">
    <text x="424" y="52">gpu/cloud.rs  (new)</text>
    <text x="424" y="116">gpu/lod.rs  (new)</text>
    <text x="574" y="116">gpu/splat.rs  (new)</text>
    <text x="424" y="180">shaders/splat.wgsl · splat_resolve.wgsl  (new)</text>
  </g>
  <g fill="#555" font-size="10">
    <text x="424" y="67">CloudLane: pos/col/nrm GrowBuf, clouds, nodes</text>
    <text x="424" y="80">append rebases upload rows -> global rows</text>
    <text x="424" y="131">LodWalk::select -> ranges</text>
    <text x="424" y="144">projected_spacing, radius_factor</text>
    <text x="574" y="131">SplatRecord x N, Key</text>
    <text x="574" y="144">prelude, draw_resolve</text>
    <text x="424" y="195">vs_point/fs_point · fs_main + EDL + frag_depth</text>
  </g>
  <g fill="none" stroke="#444"><rect x="416" y="216" width="290" height="52"/></g>
  <g fill="#222" font-size="11"><text x="424" y="232">frame.rs · layouts.rs · view.rs · mod.rs</text></g>
  <g fill="#555" font-size="10">
    <text x="424" y="247">CloudUniform 16 B · points + resolve layouts</text>
    <text x="424" y="260">cloud_size, edl_strength, lod_px · Gpu.cloud, Gpu.splat</text>
  </g>
  <line x1="14" y1="284" x2="706" y2="284" stroke="#444"/>
  <g fill="#222" font-size="11"><text x="14" y="302">render.rs - the frame</text></g>
  <g fill="#555" font-size="10">
    <text x="14" y="318">point_pass: records -> own 1x depth + colour   ->   scene pass: … 5 mesh edges · 6 clouds (resolve) · 7 markers · 8 lines · 9 lettering · 10 dots</text>
    <text x="14" y="333">green = created in this lesson</text>
  </g>
</svg>

## Step 1 - Lay the cloud lane's tables

- The lane is defined by the rows it owns: three flat point tables (positions, packed colours, oct16 normals), the node table, and one `Cloud` per draw. `CloudRows` is one upload's delta with upload-local `first`s; `CloudLane::append` rebases them to global rows, so a producer never knows what is already on the GPU.

_Paste it._
**Create `src/engine/gpu/cloud.rs`**

```rust
//! The cloud lane's tables: positions, colours, optional normals, the octree nodes, and one
//! `Cloud` record per cloud. `CloudRows` is one upload's delta; `CloudLane` is the GPU side.

use super::buffers::{GpuCtx, GrowBuf, ROWS};
use super::upload::drop_rows;

/// `nrm_first` value of a cloud without normals.
pub const NO_NORMALS: u32 = u32::MAX;

/// One cloud in an upload: `count` points landing at upload-local row `first`, with its node
/// table and spacing.
pub struct CloudDraw {
    pub instance: u32,
    pub count: u32,
    pub first: u32,
    /// Measured point spacing, cloud-local units (drives the splat radius).
    pub spacing: f32,
    /// This cloud's slice of the upload's node table; `node_count` 0 = no octree.
    pub node_first: u32,
    pub node_count: u32,
    /// Upload-local first row in the normals table, or `NO_NORMALS`.
    pub nrm_first: u32,
}

/// One octree node, read off the file: `first`/`count` are RELATIVE to the cloud's point 0
/// and `children` are indices RELATIVE to the cloud's node slice (-1 = none).
#[derive(Clone, Copy)]
pub struct LodNode {
    pub center: [f32; 3],
    pub size: f32,
    pub spacing: f32,
    pub first: u32,
    pub count: u32,
    pub children: [i32; 8],
}

/// One cloud as the lane knows it: its object row, its node slice, and its rows
/// `[first, first + count)` in the lane.
pub struct Cloud {
    pub instance: u32,
    pub spacing: f32,
    pub node_first: u32,
    pub node_count: u32,
    pub nrm_first: u32,
    pub first: u32,
    pub count: u32,
}

/// One upload's clouds: this file's rows only.
#[derive(Default)]
pub struct CloudRows {
    pub pos: Vec<f32>,
    pub col: Vec<u32>,
    pub nrm: Vec<u32>,
    pub draws: Vec<CloudDraw>,
    pub nodes: Vec<LodNode>,
}

impl CloudRows {
    /// Points in this upload so far - the `first` of the next draw.
    pub fn point_count(&self) -> u32 {
        (self.pos.len() / 3) as u32
    }

    /// Empty every table and hand the allocations back.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.pos);
        drop_rows(&mut self.col);
        drop_rows(&mut self.nrm);
        drop_rows(&mut self.draws);
        drop_rows(&mut self.nodes);
    }
}

/// The three point buffers as the point lane binds them.
pub struct PointBufs<'a> {
    pub pos: &'a wgpu::Buffer,
    pub col: &'a wgpu::Buffer,
    pub nrm: &'a wgpu::Buffer,
}

/// The cloud lane on the GPU: three append-only tables, the node table, the clouds.
pub struct CloudLane {
    pos: GrowBuf,
    col: GrowBuf,
    nrm: GrowBuf,
    pub clouds: Vec<Cloud>,
    pub nodes: Vec<LodNode>,
    pub point_count: u32,
}

impl CloudLane {
    /// Three one-row tables - empty until the first upload fills them.
    pub fn new(ctx: &GpuCtx) -> Self {
        Self {
            pos: GrowBuf::new(ctx, "points.buffer", 4, ROWS),
            col: GrowBuf::new(ctx, "points.col.buffer", 4, ROWS),
            nrm: GrowBuf::new(ctx, "points.nrm.buffer", 4, ROWS),
            clouds: Vec::new(),
            nodes: Vec::new(),
            point_count: 0,
        }
    }

    /// Append one upload: rows to the tables, nodes to the node table, one cloud per draw.
    /// Returns whether a buffer moved (the point lane must rebind).
    pub fn append(&mut self, ctx: &GpuCtx, up: &CloudRows) -> bool {
        debug_assert_eq!(up.col.len() * 3, up.pos.len());
        let point_base = self.point_count;
        let nrm_base = self.nrm.len();
        let node_base = self.nodes.len() as u32;

        let mut moved = self.pos.append(ctx, &up.pos);
        moved |= self.col.append(ctx, &up.col);
        moved |= self.nrm.append(ctx, &up.nrm);
        self.point_count = self.pos.len() / 3;
        self.nodes.extend_from_slice(&up.nodes);

        for d in &up.draws {
            let nrm_first = if d.nrm_first == NO_NORMALS { NO_NORMALS } else { nrm_base + d.nrm_first };
            self.clouds.push(Cloud {
                instance: d.instance,
                spacing: d.spacing,
                node_first: d.node_first + node_base,
                node_count: d.node_count,
                nrm_first,
                first: point_base + d.first,
                count: d.count,
            });
        }
        moved
    }

    /// Which cloud a global point row belongs to: (object row, index within that cloud).
    pub fn row_of(&self, row: u32) -> Option<(u32, u32)> {
        for c in &self.clouds {
            if row >= c.first && row < c.first + c.count {
                return Some((c.instance, row - c.first));
            }
        }
        None
    }

    /// The three point buffers.
    pub fn buffers(&self) -> PointBufs<'_> {
        PointBufs { pos: &self.pos.buf, col: &self.col.buf, nrm: &self.nrm.buf }
    }

    /// Forget every row and record; capacity stays.
    pub fn reset(&mut self) {
        self.pos.reset();
        self.col.reset();
        self.nrm.reset();
        self.point_count = 0;
        self.clouds.clear();
        self.nodes.clear();
    }

    /// Hand every buffer and both lists back; the caller rebinds the point lane.
    pub fn release(&mut self, ctx: &GpuCtx) {
        self.reset();
        self.pos.release(ctx);
        self.col.release(ctx);
        self.nrm.release(ctx);
        self.clouds.shrink_to_fit();
        self.nodes.shrink_to_fit();
    }
}
```

## Step 2 - Give Upload a cloud table

- `Upload` is the contract between the halves; a lane joins by adding its rows here and dropping them after the upload.

_Type it._
**Find** in `src/engine/gpu/upload.rs`:

```rust
use super::arena::ArenaRows;
```

**Add below it:**

```rust
use super::cloud::CloudRows;
```

_Type it._
**Find** in `src/engine/gpu/upload.rs`:

```rust
    pub glyph: GlyphRows,
```

**Add below it:**

```rust
    pub cloud: CloudRows,
```

_Type it._
**Find** in `src/engine/gpu/upload.rs`:

```rust
            glyph: GlyphRows::default(),
```

**Add below it:**

```rust
            cloud: CloudRows::default(),
```

_Type it._
**Find** in `src/engine/gpu/upload.rs`:

```rust
        self.glyph.drop_rows();
```

**Add below it:**

```rust
        self.cloud.drop_rows();
```

## Step 3 - Walk a PointCloud into the tables

- The producer reads the kernel's flat arrays straight into the tables and reports one `CloudDraw`; the spacing it measures from the cloud's density drives the splat radius, and the per-file pixel size rides the object row's `spacing` slot.
- The nodes are copied as the file holds them - ranges relative to the cloud's point 0, children relative to its node slice - so a cloud's octree is valid wherever its rows land.

_Type it._
**Create `src/app/walk/cloud.rs`**

```rust
//! Point clouds into the cloud lane: a walked kernel `PointCloud` (points, optional normals,
//! the octree it carries, one draw).

use session_rust::PointCloud;
use crate::engine::gpu::cloud::CloudRows;
use crate::engine::gpu::{CloudDraw, LodNode, NO_NORMALS};
use crate::math::Aabb;
use super::{Row, WalkCx};
use super::encode::oct16;

/// Spacing reported when a cloud is too small to measure.
const DEFAULT_SPACING: f32 = 20.0;

/// The points, the nodes, then the draw record; the per-file point size rides the row.
pub fn walk_cloud(c: &mut CloudRows, pc: &PointCloud, cx: &WalkCx) -> Row {
    let first = c.point_count();
    let node_first = c.nodes.len() as u32;
    let nrm_first = if pc.normals().len() >= pc.len() * 3 { c.nrm.len() as u32 } else { NO_NORMALS };
    let bounds = push_points(c, pc);
    push_nodes(c, pc);
    c.draws.push(CloudDraw {
        instance: cx.row,
        count: pc.len() as u32,
        first,
        spacing: cloud_spacing(pc, &bounds),
        node_first,
        node_count: pc.lod_node_count() as u32,
        nrm_first,
    });
    let px = if cx.cloud_px > 0.0 { cx.cloud_px } else { pc.point_size as f32 };
    Row { bounds, spacing: px, flags: 0, faces: false, thickness: bounds.thinnest() }
}

/// Positions, colours and (when every point has one) normals, from the kernel's flat arrays.
fn push_points(rows: &mut CloudRows, pc: &PointCloud) -> Aabb {
    let coords = pc.coords();
    let colors = pc.colors();
    let normals = pc.normals();
    let n = pc.len();
    let has_normals = normals.len() >= n * 3;
    rows.pos.reserve(n * 3);
    rows.col.reserve(n);
    let mut bounds = Aabb::empty();
    for i in 0..n {
        let p = [coords[i * 3] as f32, coords[i * 3 + 1] as f32, coords[i * 3 + 2] as f32];
        bounds.grow(p);
        rows.pos.extend_from_slice(&p);
        let c = i * 4;
        rows.col.push(if c + 3 < colors.len() { pack_color(&colors[c..c + 4]) } else { 0xff00_0000 });
        if has_normals {
            rows.nrm.push(oct16(&[normals[i * 3], normals[i * 3 + 1], normals[i * 3 + 2]]).unwrap_or(0));
        }
    }
    bounds
}

/// The octree nodes the file carries, relative to this cloud's own rows and node slice.
fn push_nodes(rows: &mut CloudRows, pc: &PointCloud) {
    for k in 0..pc.lod_node_count() {
        let (c, size) = pc.lod_cube(k);
        let (nf, nc) = pc.lod_range(k);
        let mut children = [-1i32; 8];
        for (slot, v) in pc.lod_children(k).into_iter().enumerate().take(8) {
            children[slot] = v;
        }
        rows.nodes.push(LodNode {
            center: [c[0] as f32, c[1] as f32, c[2] as f32],
            size: size as f32,
            spacing: pc.lod_spacing(k) as f32,
            first: nf as u32,
            count: nc as u32,
            children,
        });
    }
}

/// Four 0-255 channels to one RGBA8 word.
fn pack_color(c: &[i32]) -> u32 {
    (c[0] as u32 & 255) | (c[1] as u32 & 255) << 8 | (c[2] as u32 & 255) << 16 | (c[3] as u32 & 255) << 24
}

/// The cloud's point spacing from its density: `sqrt(area / n)` over the two longest box
/// edges - a scan samples a surface. Invariant to point order.
fn cloud_spacing(pc: &PointCloud, bounds: &Aabb) -> f32 {
    let n = pc.len();
    if n < 2 || !bounds.is_finite() {
        return DEFAULT_SPACING;
    }
    let mut e = [bounds.max[0] - bounds.min[0], bounds.max[1] - bounds.min[1], bounds.max[2] - bounds.min[2]];
    e.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let area = e[0] as f64 * e[1] as f64;
    if area <= 0.0 || !area.is_finite() {
        return DEFAULT_SPACING;
    }
    (area / n as f64).sqrt() as f32
}
```

## Step 4 - Dispatch clouds to the producer

- `Walk` lends the cloud table, `WalkCx` carries the file's pixel-size override, and the `PointCloud` arm that used to report an empty box now calls the producer.

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
use crate::engine::gpu::arena::ArenaRows;
```

**Add below it:**

```rust
use crate::engine::gpu::cloud::CloudRows;
```

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
use brep::{walk_brep, walk_surface};
```

**Add below it:**

```rust
use cloud::walk_cloud;
```

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
pub mod brep;
```

**Add below it:**

```rust
pub mod cloud;
```

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
    pub glyph: &'a mut GlyphRows,
```

**Add below it:**

```rust
    pub cloud: &'a mut CloudRows,
```

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
        Self { arena: &mut t.arena, seg: &mut t.seg, glyph: &mut t.glyph }
```

**Replace with:**

```rust
        Self { arena: &mut t.arena, seg: &mut t.seg, glyph: &mut t.glyph, cloud: &mut t.cloud }
```

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
/// indices on it) and the object row.
pub struct WalkCx<'a> {
    pub vert_base: u32,
```

**Replace with:**

```rust
/// indices on it), the file's point-size override in px (0 = the pb's own) and the object row.
pub struct WalkCx<'a> {
    pub vert_base: u32,
    pub cloud_px: f32,
```

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
/// and points the FLAT lane.
pub fn walk_geometry(w: &mut Walk, cx: &WalkCx, geom: &Geometry) -> Row {
```

**Replace with:**

```rust
/// and points the FLAT lane; every cloud the point lane.
pub fn walk_geometry(w: &mut Walk, cx: &WalkCx, geom: &Geometry) -> Row {
```

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
        Geometry::PointCloud(_) => Row::thin(Aabb::empty()),
```

**Replace with:**

```rust
        Geometry::PointCloud(pc) => walk_cloud(w.cloud, pc, cx),
```

## Step 5 - Read the manifest's point size

- A scene file may say how big a cloud's points are on screen; 0 means the pb's own `point_size`. The loader turns it into a float once and hands it to the document.

_Type it._
**Find** in `src/app/manifest.rs`:

```rust
    pub xform: Option<[f64; 16]>,
```

**Add below it:**

```rust
    /// Cloud point size in px for this file; 0 = the pb's own.
    #[serde(default)]
    pub point_size: f64,
```

_Type it._
**Find** in `src/app/loader.rs`:

```rust
        let place = manifest.place(i, AUTO_GRID);
```

**Add below it:**

```rust
        let point_px = item.point_size as f32;
```

_Type it._
**Find** in `src/app/loader.rs`:

```rust
        post(Msg::File(FileDoc { name, session: Rc::new(session), place, display_only: item.display_only }));
```

**Replace with:**

```rust
        post(Msg::File(FileDoc { name, session: Rc::new(session), place, point_px, display_only: item.display_only }));
```

## Step 6 - Carry the point size through the scene

- The size lives on the document so a rebuild walks with the same value, and reaches the producer through `WalkCx` - the scene never names a cloud.

_Type it._
**Find** in `src/app/scene.rs`:

```rust
    /// The session was RELEASED after the walk (manifest `display_only`): an empty shell.
    pub display_only: bool,
```

**Add above it:**

```rust
    pub point_px: f32,
```

_Type it._
**Find** in `src/app/scene.rs`:

```rust
    pub place: Xform,
    pub display_only: bool,
```

**Replace with:**

```rust
    pub place: Xform,
    pub point_px: f32,
    pub display_only: bool,
```

_Type it._
**Find** in `src/app/scene.rs`:

```rust
            self.add_file(FileDoc { name: d.name, session: d.session, place: d.place, display_only: d.display_only });
```

**Replace with:**

```rust
            self.add_file(FileDoc { name: d.name, session: d.session, place: d.place, point_px: d.point_px, display_only: d.display_only });
```

_Type it._
**Find** in `src/app/scene.rs`:

```rust
        let FileDoc { name, session, place, display_only } = doc;
```

**Replace with:**

```rust
        let FileDoc { name, session, place, point_px, display_only } = doc;
```

_Type it._
**Find** in `src/app/scene.rs`:

```rust
            let cx = WalkCx { vert_base: self.bases.vert, row, hosts: &hosts };
```

**Replace with:**

```rust
            let cx = WalkCx { vert_base: self.bases.vert, cloud_px: point_px, row, hosts: &hosts };
```

_Type it._
**Find** in `src/app/scene.rs`:

```rust
        self.docs.push(Doc { name, place, session, display_only });
```

**Replace with:**

```rust
        self.docs.push(Doc { name, place, session, point_px, display_only });
```

## Step 7 - Add the cloud block to the frame uniforms

- Both point shaders read one 16 B block: the global size scale, the viewport in pixels (the quads are pixel-aligned) and the EDL strength. It binds through the existing `line` layout - one uniform buffer at binding 0 is the same shape.

_Type it._
**Find** in `src/engine/gpu/frame.rs`:

```rust
//! The per-frame uniforms every shader reads: the camera matrix (group 0) and the line/pen block
//! (group 1), written once per frame from a `FrameInput`. The eye and the
//! ortho half-height are solved here ONCE and read by the inside test.
```

**Replace with:**

```rust
//! The per-frame uniforms every shader reads: the camera matrix (group 0), the line/pen block
//! and the cloud block (group 1), written once per frame from a `FrameInput`. The eye and the
//! ortho half-height are solved here ONCE and read by the point records and the inside test.
```

_Type it._
**Find** in `src/engine/gpu/frame.rs`:

```rust
/// The two uniform buffers with their bind groups, plus this frame's solved camera facts.
pub struct FrameUniforms {
    mvp_buffer: wgpu::Buffer,
    line_buffer: wgpu::Buffer,
    pub mvp_group: wgpu::BindGroup,
    pub line_group: wgpu::BindGroup,
    /// This frame's camera matrix as f32.
    pub mvp_f32: [f32; 16],
    /// Ortho half-height this frame (0 = perspective).
    pub ortho_h: f32,
    /// Eye in anchored world units, for the inside test.
    pub eye: [f32; 3],
}
```

**Replace with:**

```rust
/// The cloud block (group 1 of the point lane), 16 B.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CloudUniform {
    pub size: f32, // global scale on per-cloud point sizes
    pub vp_w: f32,
    pub vp_h: f32,
    pub edl: f32,  // Eye-Dome Lighting strength; 0 = off
}

/// The three uniform buffers with their bind groups, plus this frame's solved camera facts.
pub struct FrameUniforms {
    mvp_buffer: wgpu::Buffer,
    line_buffer: wgpu::Buffer,
    cloud_buffer: wgpu::Buffer,
    pub mvp_group: wgpu::BindGroup,
    pub line_group: wgpu::BindGroup,
    pub cloud_group: wgpu::BindGroup,
    /// This frame's camera matrix as f32: the point lane's static-skip key and record fold.
    pub mvp_f32: [f32; 16],
    /// Ortho half-height this frame (0 = perspective).
    pub ortho_h: f32,
    /// Eye in anchored world units, for the inside test and the LOD screen-error test.
    pub eye: [f32; 3],
}
```

_Type it._
**Find** in `src/engine/gpu/frame.rs`:

```rust
    /// The two buffers and bind groups with no camera yet.
```

**Replace with:**

```rust
    /// The three buffers and bind groups with no camera yet.
```

_Type it._
**Find** in `src/engine/gpu/frame.rs`:

```rust
        let line_buffer = uniform_buffer(&ctx.device, "line.buffer", &line);
```

**Add below it:**

```rust
        let cloud = CloudUniform { size: 1.0, vp_w: size.0 as f32, vp_h: size.1 as f32, edl: 0.0 };
        let cloud_buffer = uniform_buffer(&ctx.device, "cloud.buffer", &cloud);
```

_Type it._
**Find** in `src/engine/gpu/frame.rs`:

```rust
        let line_group = bind_group(ctx, &l.line, "line.bind_group", &[&line_buffer]);
```

**Add below it:**

```rust
        let cloud_group = bind_group(ctx, &l.line, "cloud.bind_group", &[&cloud_buffer]);
```

_Type it._
**Find** in `src/engine/gpu/frame.rs`:

```rust
        Self { mvp_buffer, line_buffer, mvp_group, line_group, mvp_f32: [0.0; 16], ortho_h: 0.0, eye: [0.0; 3] }
```

**Replace with:**

```rust
        Self { mvp_buffer, line_buffer, cloud_buffer, mvp_group, line_group, cloud_group, mvp_f32: [0.0; 16], ortho_h: 0.0, eye: [0.0; 3] }
```

_Type it._
**Find** in `src/engine/gpu/frame.rs`:

```rust
    /// Per-frame uniforms: camera and the line/pen block. The eye and the
```

**Replace with:**

```rust
    /// Per-frame uniforms: camera, the line/pen block, and the cloud block. The eye and the
```

_Type it._
**Find** in `src/engine/gpu/frame.rs`:

```rust
        ctx.queue.write_buffer(&self.line_buffer, 0, bytemuck::bytes_of(&line));
    }
```

**Replace with:**

```rust
        ctx.queue.write_buffer(&self.line_buffer, 0, bytemuck::bytes_of(&line));

        let cloud = CloudUniform { size: cx.view.cloud_size, vp_w: cx.size.0 as f32, vp_h: cx.size.1 as f32, edl: cx.view.edl_strength };
        ctx.queue.write_buffer(&self.cloud_buffer, 0, bytemuck::bytes_of(&cloud));
    }
```

## Step 8 - Declare the point and resolve layouts

- The point pass binds the record table and the three point tables as one group; the resolve pass binds the lane's two targets as textures readable from the fragment stage.

_Paste it._
**Find** in `src/engine/pipelines/layouts.rs`:

```rust
/// The bind-group layouts every lane shares.
pub struct Layouts {
    pub mvp: wgpu::BindGroupLayout,
    pub line: wgpu::BindGroupLayout,
    pub instance: wgpu::BindGroupLayout,
    pub rows: wgpu::BindGroupLayout,
}
```

**Replace with:**

```rust
/// The point lane's group: the record table at 0, then positions, colours, normals.
fn points_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("points.layout"),
        entries: &[storage_entry(0), storage_entry(1), storage_entry(2), storage_entry(3)],
    })
}

/// The resolve pass reads the point lane's depth and colour targets from its fragment stage.
fn resolve_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("splat.resolve.layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Depth,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
        ],
    })
}

/// The bind-group layouts every lane shares.
pub struct Layouts {
    pub mvp: wgpu::BindGroupLayout,
    pub line: wgpu::BindGroupLayout,
    pub instance: wgpu::BindGroupLayout,
    pub rows: wgpu::BindGroupLayout,
    pub points: wgpu::BindGroupLayout,
    pub resolve: wgpu::BindGroupLayout,
}
```

_Type it._
**Find** in `src/engine/pipelines/layouts.rs`:

```rust
            rows: rows_layout(device, "rows.layout"),
```

**Add below it:**

```rust
            points: points_layout(device),
            resolve: resolve_layout(device),
```

## Step 9 - Walk the octree by screen error

- A cloud under `LOD_MIN_POINTS`, or a frame with `lod_px` off, is one range: the whole cloud. Otherwise the walk descends from the root while a node's spacing still projects wider than the cutoff, and every visited node becomes a range sized by the finest spacing selected beneath it.
- `radius_factor` folds the world radius with the projection so the shader divides once; an octree node's radius is floored at half its pitch so its discs still tile.

_Type it._
**Create `src/engine/gpu/lod.rs`**

```rust
//! The level-of-detail walk over one cloud's octree: which node ranges to draw this frame,
//! given how wide each node's point spacing projects on screen. Pure CPU; the point lane
//! turns the ranges into records.

use crate::math::mat_scale;
use super::cloud::{Cloud, LodNode};

/// Clouds smaller than this draw WHOLE whatever the LOD cutoff says: nothing to save, and a
/// node drawn at its own coarser spacing is fatter than the whole cloud.
const LOD_MIN_POINTS: u32 = 2_000_000;

/// A point range one record covers, cloud-local: rows, spacing, and whether it is an octree
/// NODE (whose spacing is the real pitch between its points, so the radius has a coverage floor).
pub struct Range {
    pub first: u32,
    pub count: u32,
    pub spacing: f32,
    pub tile: bool,
}

/// One visited octree node during the walk: its range and its parent's slot, so the finest
/// spacing found below a node can travel back up to it.
struct Visit {
    first: u32,
    count: u32,
    spacing: f32,
    parent: usize,
}

/// What the walk needs from the frame: the eye, the projection, the viewport height, the
/// cutoff in pixels, and the lane's node table.
pub struct Projection<'a> {
    pub eye: [f32; 3],
    /// Ortho half-height in world mm; 0 in perspective.
    pub ortho_h: f32,
    pub height_px: u32,
    pub lod_px: f32,
    pub nodes: &'a [LodNode],
}

/// The walk's scratch and its output, kept between frames so nothing is reallocated.
#[derive(Default)]
pub struct LodWalk {
    pub ranges: Vec<Range>,
    stack: Vec<(usize, usize)>,
    visits: Vec<Visit>,
}

impl LodWalk {
    /// The ranges one cloud contributes: the whole cloud, or the octree nodes whose
    /// spacing still projects wider than `lod_px` pixels (each node OWNS its subsample, so
    /// descending only adds detail), every node sized by the finest spacing selected beneath
    /// it.
    pub fn select(&mut self, p: &Projection, c: &Cloud, model: &[f32; 16]) {
        self.ranges.clear();
        if c.node_count == 0 || p.lod_px <= 0.0 || c.count < LOD_MIN_POINTS {
            self.ranges.push(Range { first: 0, count: c.count, spacing: c.spacing, tile: false });
            return;
        }

        let base = c.node_first as usize;
        let scale = mat_scale(model);
        self.stack.clear();
        self.visits.clear();
        self.stack.push((0, usize::MAX));
        while let Some((n, parent)) = self.stack.pop() {
            let Some(node) = p.nodes.get(base + n) else { continue };
            let count = node.count;
            let slot = self.visits.len();
            self.visits.push(Visit { first: node.first, count, spacing: node.spacing, parent });
            if projected_spacing(p, node, model, scale) > p.lod_px as f64 {
                for &child in &node.children {
                    if child >= 0 {
                        self.stack.push((child as usize, slot));
                    }
                }
            }
        }

        for i in (0..self.visits.len()).rev() {
            let (fine, parent) = (self.visits[i].spacing, self.visits[i].parent);
            if parent != usize::MAX && fine < self.visits[parent].spacing {
                self.visits[parent].spacing = fine;
            }
        }
        for v in &self.visits {
            if v.count > 0 {
                self.ranges.push(Range { first: v.first, count: v.count, spacing: v.spacing, tile: true });
            }
        }
    }
}

/// How wide a node's spacing projects on screen, in pixels. Everything in metres: the
/// spacing through the placement scale, the eye distance, and the ortho half-height.
fn projected_spacing(p: &Projection, node: &LodNode, model: &[f32; 16], scale: f64) -> f64 {
    let world = node.spacing as f64 * scale * 0.001;
    let c = node.center;
    let wx = (model[0] * c[0] + model[4] * c[1] + model[8] * c[2] + model[12]) as f64;
    let wy = (model[1] * c[0] + model[5] * c[1] + model[9] * c[2] + model[13]) as f64;
    let wz = (model[2] * c[0] + model[6] * c[1] + model[10] * c[2] + model[14]) as f64;
    let e = p.eye;
    let dist = ((wx - e[0] as f64).powi(2) + (wy - e[1] as f64).powi(2) + (wz - e[2] as f64).powi(2)).sqrt().max(1.0e-6) * 0.001;
    let frac = if p.ortho_h > 0.0 { world / (2.0 * p.ortho_h as f64 * 0.001) } else { world * 1.7320508 * 0.5 / dist };
    frac * p.height_px as f64
}

/// The radius factor `k` of a range: world radius = spacing x scale x px / 6 (a manifest
/// size of 6 is a full spacing), floored to spacing / 2 for an octree node so discs on a
/// pitch of `spacing` still tile; then folded with the projection so the shader divides once.
/// `ortho_h` is in world mm, the radius in metres.
pub fn radius_factor(r: &Range, px: f32, scale: f64, ortho_h: f32) -> f32 {
    let mut world_r = (r.spacing as f64).max(1.0e-9) * scale * 0.001 * (px as f64) / 6.0;
    if r.tile {
        world_r = world_r.max(r.spacing as f64 * scale * 0.001 * 0.5);
    }
    let k = if ortho_h > 0.0 { world_r / (2.0 * ortho_h as f64 * 0.001) } else { world_r * 1.7320508 * 0.5 };
    k as f32
}
```

## Step 10 - Splat one quad per point

- No vertex buffer: `vs_point` pulls point `vid / 6` from the storage tables, finds its record by binary search over the `cum` column, projects it once, and emits a pixel-aligned box over the disc's footprint. `fs_point` rounds the box; the hardware depth test keeps the nearest point.

_Paste it._
**Create `src/shaders/splat.wgsl`**

```wgsl
// The point lane: one pixel-aligned quad per point, pulled by vertex index, into the lane's
// own 1x depth + colour targets. Group 0 = the cloud uniform, group 1 = records + tables.

struct CloudUniform {
    size: f32,
    vp_w: f32,
    vp_h: f32,
    edl: f32,
};
@group(0) @binding(0) var<uniform> cloud: CloudUniform;

// The record table as raw words: a 4-word header {n, total, 0, 0}, then REC_WORDS per record:
// 0-15 mvp x model (column-major), 16-19 tint (.a = min radius px), 20 first, 21 count,
// 22 cum, 23 k bits, 24-35 rotation columns (3 x vec4), 36 nrm_first, 37 instance, 38 flags.
const REC_WORDS: u32 = 40u;
const NO_NORMALS: u32 = 0xffffffffu;
@group(1) @binding(0) var<storage, read> table: array<u32>;
@group(1) @binding(1) var<storage, read> positions: array<f32>;
@group(1) @binding(2) var<storage, read> colors: array<u32>;
@group(1) @binding(3) var<storage, read> normals: array<u32>;

struct Splat {
    px: vec2<i32>,
    r: f32,
    z: f32,
    color: u32,
    row: u32,
    instance: u32,
    ok: bool,
};

fn rec_f(base: u32, w: u32) -> f32 {
    return bitcast<f32>(table[base + w]);
}

// The record holding global point index `gid`: records are in `cum` order, so a binary
// search over the header count finds it in log steps.
fn record_of(gid: u32) -> u32 {
    let n = table[0];
    var lo = 0u;
    var hi = n;
    while (hi - lo > 1u) {
        let mid = (lo + hi) / 2u;
        if (table[4u + mid * REC_WORDS + 22u] <= gid) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    return lo;
}

fn oct16_decode(p: u32) -> vec3<f32> {
    let e = vec2<f32>(f32(i32(p << 24u) >> 24u) / 127.0, f32(i32(p << 16u) >> 24u) / 127.0);
    var n = vec3<f32>(e, 1.0 - abs(e.x) - abs(e.y));
    if (n.z < 0.0) {
        let s = vec2<f32>(select(1.0, -1.0, n.x < 0.0), select(1.0, -1.0, n.y < 0.0));
        n = vec3<f32>((1.0 - abs(n.y)) * s.x, (1.0 - abs(n.x)) * s.y, n.z);
    }
    return normalize(n);
}

// Point `gid` projected: pixel centre, radius, depth, lit colour, and its table row.
fn project(gid: u32) -> Splat {
    var s: Splat;
    s.ok = false;
    if (gid >= table[1]) {
        return s;
    }
    let base = 4u + record_of(gid) * REC_WORDS;
    let offset = gid - table[base + 22u];
    let i = table[base + 20u] + offset;
    let m = mat4x4<f32>(
        vec4<f32>(rec_f(base, 0u), rec_f(base, 1u), rec_f(base, 2u), rec_f(base, 3u)),
        vec4<f32>(rec_f(base, 4u), rec_f(base, 5u), rec_f(base, 6u), rec_f(base, 7u)),
        vec4<f32>(rec_f(base, 8u), rec_f(base, 9u), rec_f(base, 10u), rec_f(base, 11u)),
        vec4<f32>(rec_f(base, 12u), rec_f(base, 13u), rec_f(base, 14u), rec_f(base, 15u)),
    );
    s.row = i;
    s.instance = table[base + 37u];
    let clip = m * vec4<f32>(positions[i * 3u], positions[i * 3u + 1u], positions[i * 3u + 2u], 1.0);
    if (clip.w <= 0.0) {
        return s;
    }
    let ndc = clip.xyz / clip.w;
    if (ndc.z < 0.0 || ndc.z > 1.0) {
        return s;
    }

    // Attenuated radius: k folds the world footprint and the projection; floored at the
    // manifest px so a far cloud never turns to dust, capped at 8 px.
    let r_min = rec_f(base, 19u);
    s.r = clamp(bitcast<f32>(table[base + 23u]) * cloud.vp_h / clip.w, r_min, 8.0);
    let x = (ndc.x * 0.5 + 0.5) * cloud.vp_w;
    let y = (0.5 - ndc.y * 0.5) * cloud.vp_h;
    if (x < -s.r || y < -s.r || x >= cloud.vp_w + s.r || y >= cloud.vp_h + s.r) {
        return s;
    }
    s.px = vec2<i32>(i32(x), i32(y));
    s.z = ndc.z;

    let tint = vec4<f32>(rec_f(base, 16u), rec_f(base, 17u), rec_f(base, 18u), 1.0);
    var rgba = unpack4x8unorm(colors[i]) * tint;
    let nrm_first = table[base + 36u];
    if (nrm_first != NO_NORMALS) {
        let packed_n = normals[nrm_first + offset];
        let rot = mat3x3<f32>(
            vec3<f32>(rec_f(base, 24u), rec_f(base, 25u), rec_f(base, 26u)),
            vec3<f32>(rec_f(base, 28u), rec_f(base, 29u), rec_f(base, 30u)),
            vec3<f32>(rec_f(base, 32u), rec_f(base, 33u), rec_f(base, 34u)),
        );
        let nw = normalize(rot * oct16_decode(packed_n));
        let light = normalize(vec3<f32>(0.4, 0.4, 0.8));
        let lambert = 0.25 + 0.75 * abs(dot(nw, light));
        rgba = vec4<f32>(rgba.rgb * lambert, rgba.a);
    }
    s.color = pack4x8unorm(rgba);
    s.ok = true;
    return s;
}

struct PointOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) @interpolate(flat) center: vec2<i32>,
    @location(1) @interpolate(flat) rr: f32,
    @location(2) @interpolate(flat) color: vec4<f32>,
    @location(3) @interpolate(flat) row: u32,
    @location(4) @interpolate(flat) instance: u32,
};

// A pixel-aligned box over the disc's footprint; the fragment rounds it. A disc big enough
// to swallow its own box is a square, so the corners are kept outside the radius.
@vertex
fn vs_point(@builtin(vertex_index) vid: u32) -> PointOut {
    var o: PointOut;
    let s = project(vid / 6u);
    if (!s.ok) {
        o.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
        return o;
    }
    let ir = i32(ceil(s.r - 0.5));
    let corner_rr = 2.0 * f32(ir * ir) - 0.001;
    let lo = vec2<f32>(f32(s.px.x - ir), f32(s.px.y - ir));
    let hi = vec2<f32>(f32(s.px.x + ir + 1), f32(s.px.y + ir + 1));
    let c = vid % 6u;
    let right = c == 1u || c == 4u || c == 5u;
    let bottom = c == 2u || c == 3u || c == 5u;
    let p = vec2<f32>(select(lo.x, hi.x, right), select(lo.y, hi.y, bottom));
    o.pos = vec4<f32>(p.x / cloud.vp_w * 2.0 - 1.0, 1.0 - p.y / cloud.vp_h * 2.0, s.z, 1.0);
    o.center = s.px;
    o.rr = select(s.r * s.r, min(s.r * s.r, corner_rr), ir >= 1);
    o.color = unpack4x8unorm(s.color);
    o.row = s.row;
    o.instance = s.instance;
    return o;
}

// Round dot: pixels outside the radius are discarded.
fn outside(in: PointOut) -> bool {
    let q = vec2<i32>(floor(in.pos.xy));
    let d = q - in.center;
    return f32(d.x * d.x + d.y * d.y) > in.rr;
}

@fragment
fn fs_point(in: PointOut) -> @location(0) vec4<f32> {
    if (outside(in)) {
        discard;
    }
    return in.color;
}
```

## Step 11 - Resolve the points into the scene with EDL

- One fullscreen triangle per frame: the pixel's point depth is exported through `frag_depth` so solids and points occlude each other in the scene's own depth buffer, and the four neighbouring depths darken edges (Eye-Dome Lighting) without needing normals.

_Type it._
**Create `src/shaders/splat_resolve.wgsl`**

```wgsl
// Composite the point pass into the scene: one fullscreen triangle looks up its pixel in the
// lane's depth + colour targets, discards empties, applies Eye-Dome Lighting from the four
// neighbouring depths, and exports the point's depth through frag_depth so points and solids
// occlude each other exactly.

struct CloudUniform {
    size: f32,
    vp_w: f32,
    vp_h: f32,
    edl: f32,
};
@group(0) @binding(0) var<uniform> cloud: CloudUniform;
@group(1) @binding(0) var sdepth: texture_depth_2d;
@group(1) @binding(1) var scolor: texture_2d<f32>;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    var o: VsOut;
    let x = f32(i32(vid & 1u) * 4 - 1);
    let y = f32(i32(vid >> 1u) * 4 - 1);
    o.pos = vec4<f32>(x, y, 0.0, 1.0);
    return o;
}

struct FsOut {
    @location(0) color: vec4<f32>,
    @builtin(frag_depth) depth: f32,
};

// -log2 of a reverse-Z depth grows with distance, like Potree's log depth.
fn log_depth(d: f32) -> f32 {
    return -log2(max(d, 1.0e-7));
}

@fragment
fn fs_main(in: VsOut) -> FsOut {
    let pix = vec2<i32>(in.pos.xy);
    let d = textureLoad(sdepth, pix, 0);
    if (d == 0.0) {
        discard;
    }
    var o: FsOut;
    var rgb = textureLoad(scolor, pix, 0).rgb;

    if (cloud.edl > 0.0) {
        let w = i32(cloud.vp_w);
        let h = i32(cloud.vp_h);
        let me = log_depth(d);
        var sum = 0.0;
        var taps = array<vec2<i32>, 4>(vec2<i32>(-1, 0), vec2<i32>(1, 0), vec2<i32>(0, -1), vec2<i32>(0, 1));
        for (var k = 0; k < 4; k++) {
            let q = pix + taps[k];
            if (q.x < 0 || q.y < 0 || q.x >= w || q.y >= h) {
                continue;
            }
            let nd = textureLoad(sdepth, q, 0);
            if (nd == 0.0) {
                continue;
            }
            sum += max(0.0, me - log_depth(nd));
        }
        // Floored at 0.25: an edge darkens, it never goes black.
        let shade = max(exp(-sum * 75.0 * cloud.edl), 0.25);
        rgb *= shade;
    }

    o.color = vec4<f32>(rgb, 1.0);
    o.depth = d;
    return o;
}
```

## Step 12 - Let a pipeline name its vertex entry

- Every pipeline so far entered at `vs_main`; the point pass enters at `vs_point`, so the desc builder gains the one setter it lacked.

_Type it._
**Find** in `src/engine/pipelines/mod.rs`:

```rust
        d.fs = fs;
        d
    }
```

**Replace with:**

```rust
        d.fs = fs;
        d
    }

    /// The same desc with another vertex entry.
    pub fn vertex(mut self, vs: &'a str) -> Self {
        self.vs = vs;
        self
    }
```

## Step 13 - Build the point lane's renderer

- `Splat` owns the records, the LOD walk, the record buffer, the two pipelines and - lazily - its two 1x targets: they are made on the first frame that has points and dropped on resize, so a scene without a cloud never pays for them.
- `prelude` is the point pass: it compares a `Key` (camera, knobs, point count) with the last one and returns early when nothing changed; `draw_resolve` is the one draw the scene list sees.

_Paste it._
**Create `src/engine/gpu/splat.rs`**

```rust
//! The point lane's renderer: one pixel-aligned quad per point into the lane's OWN 1x depth +
//! colour targets (the hardware depth test keeps the nearest point), then a fullscreen resolve
//! into the scene pass with EDL and `frag_depth`. A record per visible cloud (or octree node)
//! folds camera x placement, tint and radius; the point pass is skipped while nothing changed.

use crate::engine::pipelines::{build, module, DepthMode, Layouts, PipelineDesc, Target};
use crate::math::{mat_mul_f32, mat_scale};
use super::buffers::{bind_group, zeroed_buffer, GpuCtx};
use super::cloud::{Cloud, LodNode, PointBufs, NO_NORMALS};
use super::instance::Instance;
use super::lod::{radius_factor, LodWalk, Projection};
use super::objects::InstanceTable;
use super::targets::{texture_view, TextureSpec};
use wgpu::PrimitiveTopology::TriangleList;

/// Records the lane can hold in one frame: one per cloud, or one per selected octree node.
pub const MAX_RECORDS: usize = 4096;

/// The point pass draws into linear RGBA8; the resolve reads it back as-is.
const COLOR_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

/// Vertices per point: one quad pulled by vertex index.
const POINT_VERTS: u32 = 6;

/// Header words before the records: {count, total points, 0, 0}.
const HEADER_BYTES: u64 = 16;

/// One record, 160 B (40 words), read as raw words by splat.wgsl.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SplatRecord {
    /// mvp x anchored model: one mat-vec per point.
    pub mvp_model: [f32; 16],
    /// Instance tint; `.a` = the minimum radius in px.
    pub tint: [f32; 4],
    pub first: u32,
    pub count: u32,
    /// Points before this record: the vertex index minus `cum` is the offset into the range.
    pub cum: u32,
    /// Radius factor: screen radius = k * vp_h / clip.w (perspective) or k * vp_h (ortho).
    pub k: f32,
    /// The model's rotation columns (translation-free), three vec4 slots, for the normals.
    pub rot: [f32; 12],
    /// First row in the normals table, or `NO_NORMALS`.
    pub nrm_first: u32,
    /// The object row, written by the id pass.
    pub instance: u32,
    pub flags: u32,
    pub _pad: u32,
}

const _: () = assert!(std::mem::size_of::<SplatRecord>() == 160);

/// What the record builder needs from the frame: camera facts, size, the two cloud knobs,
/// the object rows and the cloud lane's clouds and nodes.
pub struct RecordCx<'a> {
    pub mvp: &'a [f32; 16],
    pub ortho_h: f32,
    pub eye: [f32; 3],
    pub size: (u32, u32),
    pub cloud_size: f32,
    pub lod_px: f32,
    pub objects: &'a InstanceTable,
    pub clouds: &'a [Cloud],
    pub nodes: &'a [LodNode],
}

/// The key the point pass was last drawn for; a frame with the same key skips it.
#[derive(Clone, PartialEq)]
struct Key {
    mvp: [f32; 16],
    cloud_size: f32,
    lod_px: f32,
    point_count: u32,
}

/// The two point-pass targets, 1x, sized to the surface, and the resolve group over them.
/// Made on the first frame that has points and dropped on resize, so a scene without a
/// cloud never pays 8 B/px for them.
struct SplatTargets {
    depth: wgpu::TextureView,
    color: wgpu::TextureView,
    size: (u32, u32),
    resolve_group: wgpu::BindGroup,
}

impl SplatTargets {
    /// Depth (nearest point per pixel, 0 = empty) and its colour, both bindable.
    fn new(ctx: &GpuCtx, l: &Layouts, size: (u32, u32)) -> Self {
        let usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING;
        let depth = texture_view(ctx, "splat.depth", &TextureSpec { size, format: wgpu::TextureFormat::Depth32Float, samples: 1, usage });
        let color = texture_view(ctx, "splat.color", &TextureSpec { size, format: COLOR_FORMAT, samples: 1, usage });
        let resolve_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("splat.resolve.group"),
            layout: &l.resolve,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&depth) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&color) },
            ],
        });
        Self { depth, color, size, resolve_group }
    }
}

/// One of the two point pipelines: the colour pass or the id pass.
struct PointVariant {
    target: Target,
    label: &'static str,
    fs: &'static str,
}

/// The point lane's renderer.
pub struct Splat {
    records: Vec<SplatRecord>,
    walk: LodWalk,
    record_buf: wgpu::Buffer,
    total: u32,
    key: Option<Key>,
    targets: Option<SplatTargets>,
    points_group: wgpu::BindGroup,
    resolve_shader: wgpu::ShaderModule,
    point_pipeline: wgpu::RenderPipeline,
    resolve_pipeline: wgpu::RenderPipeline,
}

impl Splat {
    /// The record buffer, the points group over the lane's placeholder buffers, and the
    /// two pipelines; the targets wait for the first cloud.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target, bufs: PointBufs) -> Self {
        let record_buf = zeroed_buffer(&ctx.device, "splat.records", HEADER_BYTES + MAX_RECORDS as u64 * 160, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        let points_group = points_group(ctx, l, &record_buf, &bufs);
        let point_shader = module(&ctx.device, "splat.shader", include_str!("../../shaders/splat.wgsl"));
        let resolve_shader = module(&ctx.device, "splat.resolve.shader", include_str!("../../shaders/splat_resolve.wgsl"));
        let point_pipeline = build_point(ctx, l, &point_shader, &PointVariant { target: Target { format: COLOR_FORMAT, samples: 1 }, label: "splat.points", fs: "fs_point" });
        let resolve_pipeline = build_resolve(ctx, l, &resolve_shader, target);

        Self {
            records: Vec::new(),
            walk: LodWalk::default(),
            record_buf,
            total: 0,
            key: None,
            targets: None,
            points_group,
            resolve_shader,
            point_pipeline,
            resolve_pipeline,
        }
    }

    /// Rebuild the resolve pipeline for a new scene sample count (the point pass stays 1x).
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.resolve_pipeline = build_resolve(ctx, l, &self.resolve_shader, target);
    }

    /// Drop the targets: the next point pass makes them at the new size.
    pub fn resize(&mut self) {
        self.targets = None;
        self.key = None;
    }

    /// Re-point the points group at the lane's current buffers (a table grew or was released).
    pub fn rebind(&mut self, ctx: &GpuCtx, l: &Layouts, bufs: PointBufs) {
        self.points_group = points_group(ctx, l, &self.record_buf, &bufs);
        self.key = None;
    }

    /// Force the next frame to rebuild the records and redraw the point pass.
    pub fn invalidate(&mut self) {
        self.key = None;
    }

    /// Drop the targets with the scene.
    pub fn release(&mut self) {
        self.targets = None;
        self.total = 0;
        self.key = None;
    }

    /// Points the last point pass drew; 0 = nothing to resolve.
    pub fn total(&self) -> u32 {
        self.total
    }

    /// The point pass: skipped while the key matches, else records rebuilt, written and drawn.
    /// `cloud_group` is the cloud uniform (group 0).
    pub fn prelude(&mut self, ctx: &GpuCtx, l: &Layouts, encoder: &mut wgpu::CommandEncoder, cx: &RecordCx, cloud_group: &wgpu::BindGroup) {
        let mut point_count = 0u32;
        for c in cx.clouds {
            point_count += c.count;
        }
        let key = Key { mvp: *cx.mvp, cloud_size: cx.cloud_size, lod_px: cx.lod_px, point_count };
        if self.key.as_ref() == Some(&key) {
            return;
        }
        self.key = Some(key);
        self.build_records(cx);
        if self.total == 0 {
            return;
        }
        if self.targets.as_ref().map(|t| t.size) != Some(cx.size) {
            self.targets = Some(SplatTargets::new(ctx, l, cx.size));
        }

        let header = [self.records.len() as u32, self.total, 0, 0];
        ctx.queue.write_buffer(&self.record_buf, 0, bytemuck::bytes_of(&header));
        ctx.queue.write_buffer(&self.record_buf, HEADER_BYTES, bytemuck::cast_slice(&self.records));

        let Some(targets) = &self.targets else { return };
        let mut pass = begin_point_pass(encoder, targets);
        pass.set_pipeline(&self.point_pipeline);
        pass.set_bind_group(0, cloud_group, &[]);
        pass.set_bind_group(1, &self.points_group, &[]);
        pass.draw(0..POINT_VERTS * self.total, 0..1);
    }

    /// The fullscreen resolve inside the scene pass: 1 draw, or 0 with no points.
    pub fn draw_resolve(&self, pass: &mut wgpu::RenderPass<'_>, cloud_group: &wgpu::BindGroup) -> u32 {
        let Some(targets) = &self.targets else { return 0 };
        if self.total == 0 {
            return 0;
        }
        pass.set_pipeline(&self.resolve_pipeline);
        pass.set_bind_group(0, cloud_group, &[]);
        pass.set_bind_group(1, &targets.resolve_group, &[]);
        pass.draw(0..3, 0..1);
        1
    }

    /// One record per visible cloud, or per selected octree node when the LOD walk is on.
    fn build_records(&mut self, cx: &RecordCx) {
        self.records.clear();
        let p = Projection { eye: cx.eye, ortho_h: cx.ortho_h, height_px: cx.size.1, lod_px: cx.lod_px, nodes: cx.nodes };
        let mut cum = 0u32;
        for c in cx.clouds {
            let Some(row) = cx.objects.row(c.instance) else { continue };
            if row.flags & Instance::FLAG_HIDDEN != 0 {
                continue;
            }
            let Some(model) = cx.objects.anchored_model(c.instance) else { continue };
            let px = if row.spacing > 0.0 { row.spacing } else { 3.0 } * cx.cloud_size;

            self.walk.select(&p, c, &model);
            let m = mat_mul_f32(cx.mvp, &model);
            let rot = [model[0], model[1], model[2], 0.0, model[4], model[5], model[6], 0.0, model[8], model[9], model[10], 0.0];
            let scale = mat_scale(&model);
            let tint = [row.color[0], row.color[1], row.color[2], (px * 0.5).max(0.5)];

            for r in &self.walk.ranges {
                if self.records.len() >= MAX_RECORDS {
                    break;
                }
                let k = radius_factor(r, px, scale, cx.ortho_h);
                let nrm_first = if c.nrm_first == NO_NORMALS { NO_NORMALS } else { c.nrm_first + r.first };
                self.records.push(SplatRecord {
                    mvp_model: m,
                    tint,
                    first: c.first + r.first,
                    count: r.count,
                    cum,
                    k,
                    rot,
                    nrm_first,
                    instance: c.instance,
                    flags: row.flags,
                    _pad: 0,
                });
                cum += r.count;
            }
        }
        self.total = cum;
    }
}

/// The point pass over the lane's own targets: colour cleared transparent, depth to 0.
/// Group 0 (the cloud uniform) is set by the caller.
fn begin_point_pass<'a>(encoder: &'a mut wgpu::CommandEncoder, t: &'a SplatTargets) -> wgpu::RenderPass<'a> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("splat.points"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &t.color,
            resolve_target: None,
            depth_slice: None,
            ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store },
        })],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: &t.depth,
            depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(0.0), store: wgpu::StoreOp::Store }),
            stencil_ops: None,
        }),
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    })
}

/// Group 1 of the point pass: the records, then positions, colours, normals.
fn points_group(ctx: &GpuCtx, l: &Layouts, records: &wgpu::Buffer, bufs: &PointBufs) -> wgpu::BindGroup {
    bind_group(ctx, &l.points, "splat.points.group", &[records, bufs.pos, bufs.col, bufs.nrm])
}

/// The point pass pipeline: quads, depth written (nearest wins), no blending.
fn build_point(ctx: &GpuCtx, l: &Layouts, shader: &wgpu::ShaderModule, v: &PointVariant) -> wgpu::RenderPipeline {
    let groups = [&l.line, &l.points];
    let desc = PipelineDesc::new(shader, &groups, &[], TriangleList).with(v.label, v.fs).vertex("vs_point");
    build(&ctx.device, v.target, &desc)
}

/// The resolve pipeline: a fullscreen triangle writing colour and `frag_depth` under the
/// scene's depth test.
fn build_resolve(ctx: &GpuCtx, l: &Layouts, shader: &wgpu::ShaderModule, target: Target) -> wgpu::RenderPipeline {
    let groups = [&l.line, &l.resolve];
    let desc = PipelineDesc::new(shader, &groups, &[], TriangleList).with("splat.resolve", "fs_main").depth(DepthMode::Opaque);
    build(&ctx.device, target, &desc)
}
```

## Step 14 - Register the lane in Gpu

- The lane and its renderer become two fields of `Gpu`, built after the other lanes so `Splat::new` can bind the cloud lane's placeholder buffers.

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
pub mod buffers;
```

**Add below it:**

```rust
pub mod cloud;
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
pub mod segments;
```

**Add below it:**

```rust
pub mod lod;
pub mod splat;
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
use buffers::GpuCtx;
```

**Add below it:**

```rust
use cloud::CloudLane;
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
use segments::SegmentLane;
```

**Add below it:**

```rust
use splat::Splat;
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
pub use frame::FrameInput;
```

**Add above it:**

```rust
pub use cloud::{CloudDraw, LodNode, NO_NORMALS};
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub glyphs: GlyphLane,
```

**Add below it:**

```rust
    pub cloud: CloudLane,
    pub splat: Splat,
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
        let glyphs = GlyphLane::new(&ctx, &layouts, target);
```

**Add below it:**

```rust
        let cloud = CloudLane::new(&ctx);
        let splat = Splat::new(&ctx, &layouts, target, cloud.buffers());
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
            glyphs,
```

**Add below it:**

```rust
            cloud,
            splat,
```

## Step 15 - Append clouds and keep the point lane current

- A table that grew has a new buffer, so the point group is rebound; anything that moves a model (an upload, a re-anchor, a reset) invalidates the key so the next frame redraws the point pass; a resize drops the targets; a release hands everything back and rebinds the one-row tables.

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.glyphs.append(&self.ctx, &self.layouts, &up.glyph);
```

**Add below it:**

```rust
        if self.cloud.append(&self.ctx, &up.cloud) {
            self.splat.rebind(&self.ctx, &self.layouts, self.cloud.buffers());
        }
        self.splat.invalidate();
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
            "scene: {} objects, {} verts, {} pipes, {} ribbons, {} markers, {} dots",
            self.objects.len(), self.arena.vert_count(), self.segments.pipe_count(), self.segments.ribbon_count(),
            self.glyphs.sphere_count(), self.glyphs.dot_count()
```

**Replace with:**

```rust
            "scene: {} objects, {} verts, {} pipes, {} ribbons, {} markers, {} dots, {} points",
            self.objects.len(), self.arena.vert_count(), self.segments.pipe_count(), self.segments.ribbon_count(),
            self.glyphs.sphere_count(), self.glyphs.dot_count(), self.cloud.point_count
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
            self.glyphs.retarget(&self.ctx, &self.layouts, target);
```

**Add below it:**

```rust
            self.splat.retarget(&self.ctx, &self.layouts, target);
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
    /// pipes, spheres) and a canvas MSAA can afford; a pure sheet stays at 1x.
```

**Replace with:**

```rust
    /// pipes, spheres) and a canvas MSAA can afford; a pure sheet or cloud stays at 1x.
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
    /// The anchor the instance table is rebased about. `now` is the frame's one timestamp (ms).
    pub fn rebase_anchor(&mut self, origin: &Point, view_dist: f64, now: f64) -> Rebase {
        let rebase = self.objects.rebase_anchor(&self.ctx, origin, view_dist, now);
```

**Replace with:**

```rust
    /// The anchor the instance table is rebased about. A rebase moves every model, so the
    /// point pass is stale. `now` is the frame's one timestamp (ms).
    pub fn rebase_anchor(&mut self, origin: &Point, view_dist: f64, now: f64) -> Rebase {
        let rebase = self.objects.rebase_anchor(&self.ctx, origin, view_dist, now);
        if rebase.moved {
            self.splat.invalidate();
        }
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.retarget(true);
```

**Add below it:**

```rust
        self.splat.resize();
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.glyphs.reset();
```

**Add below it:**

```rust
        self.cloud.reset();
        self.splat.invalidate();
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.glyphs.release(&self.ctx, &self.layouts);
```

**Add below it:**

```rust
        self.cloud.release(&self.ctx);
        self.splat.release();
        self.splat.rebind(&self.ctx, &self.layouts, self.cloud.buffers());
```

## Step 16 - Put the point pass in the frame list

- The point pass runs before the scene pass because the resolve reads its targets; the resolve draws right after the mesh edges, with the other depth writers, so the blended ink after it composites over points too.

_Type it._
**Find** in `src/engine/gpu/render.rs`:

```rust
//! The frame list. `encode_frame` runs ONE scene pass whose `scene_list`
```

**Replace with:**

```rust
//! The frame list. `encode_frame` runs the point pass, then ONE scene pass whose `scene_list`
```

_Type it._
**Find** in `src/engine/gpu/render.rs`:

```rust
use super::frame::Binds;
```

**Add below it:**

```rust
use super::splat::RecordCx;
```

_Type it._
**Find** in `src/engine/gpu/render.rs`:

```rust
        let draws = {
```

**Replace with:**

```rust
        self.point_pass(encoder);

        let draws = {
```

_Type it._
**Find** in `src/engine/gpu/render.rs`:

```rust
        (draws, self.objects.len())
    }

    /// The scene list, in order:
    /// 1 background · 2 grid · 3 faces · 4 sheet fills · 5 mesh edges · 6 vertex
    /// markers · 7 lines · 8 lettering · 9 point dots. Lines write no depth: two lines on one
```

**Replace with:**

```rust
        (draws, self.objects.len())
    }

    /// The point lane's own pass, skipped while the camera, the knobs and the tables are what
    /// they were - a still cloud costs one fullscreen resolve.
    fn point_pass(&mut self, encoder: &mut wgpu::CommandEncoder) {
        let cx = RecordCx {
            mvp: &self.frame.mvp_f32,
            ortho_h: self.frame.ortho_h,
            eye: self.frame.eye,
            size: (self.config.width, self.config.height),
            cloud_size: self.view.cloud_size,
            lod_px: self.view.lod_px,
            objects: &self.objects,
            clouds: &self.cloud.clouds,
            nodes: &self.cloud.nodes,
        };
        self.splat.prelude(&self.ctx, &self.layouts, encoder, &cx, &self.frame.cloud_group);
    }

    /// The scene list, in order:
    /// 1 background · 2 grid · 3 faces · 4 sheet fills · 5 mesh edges · 6 clouds · 7 vertex
    /// markers · 8 lines · 9 lettering · 10 point dots. Lines write no depth: two lines on one
```

_Type it._
**Find** in `src/engine/gpu/render.rs`:

```rust
        if v.show_mesh_edges {
            draws += self.segments.draw_pipes(pass, b, v.line_style);
        }
```

**Add below it:**

```rust
        draws += self.splat.draw_resolve(pass, &self.frame.cloud_group);
```

## Step 17 - The three cloud knobs

- `cloud_size` scales every cloud's pixel size, `edl_strength` 0 turns the lighting off, `lod_px` 0 draws every cloud whole; all three read once at startup from `?cloud=`, `?edl=`, `?lod=` (or the `VIEWER_*` variables natively).

_Type it._
**Find** in `src/engine/gpu/view.rs`:

```rust
//! pen weight. Read ONCE at startup from the query string
```

**Replace with:**

```rust
//! cloud / EDL / LOD scalars and the pen weight. Read ONCE at startup from the query string
```

_Type it._
**Find** in `src/engine/gpu/view.rs`:

```rust
    pub line_style: LineStyle,
```

**Add below it:**

```rust
    /// Global scale on per-cloud point sizes, `[` and `]` (`VIEWER_CLOUD_SCALE`).
    pub cloud_size: f32,
    /// Eye-Dome Lighting strength; 0 = off (`VIEWER_EDL`).
    pub edl_strength: f32,
    /// Octree LOD cutoff in projected pixels; 0 = off, draw every cloud whole (`?lod=` / `VIEWER_LOD`).
    pub lod_px: f32,
```

_Type it._
**Find** in `src/engine/gpu/view.rs`:

```rust
            line_style: if tubes { LineStyle::Tubes } else { LineStyle::Flat },
```

**Add below it:**

```rust
            cloud_size: knob_f32("VIEWER_CLOUD_SCALE", "cloud", 1.0),
            edl_strength: knob_f32("VIEWER_EDL", "edl", 0.25),
            lod_px: knob_f32("VIEWER_LOD", "lod", 0.0),
```

## Step 18 - Keys [ and ]

- The scale is clamped in one place on `State`, which also asks for the frame; the key handler only says by how much.

_Type it._
**Find** in `src/state.rs`:

```rust
        self.gpu.resize(width, height);
        self.needs_frame = true;
    }
```

**Replace with:**

```rust
        self.gpu.resize(width, height);
        self.needs_frame = true;
    }

    /// The global cloud point-size scale, clamped.
    pub fn set_cloud_size(&mut self, size: f32) {
        self.gpu.view.cloud_size = size.clamp(0.25, 8.0);
        self.needs_frame = true;
    }
```

_Type it._
**Find** in `src/app/input.rs`:

```rust
//! 1-7 named views, Space projection, C reset, F fit, Q/W/E lane toggles, L line style.
//! Fingers go to `touch.rs`.
```

**Replace with:**

```rust
//! 1-7 named views, Space projection, C reset, F fit, Q/W/E lane toggles, L line style,
//! [ ] cloud size. Fingers go to `touch.rs`.
```

_Type it._
**Find** in `src/app/input.rs`:

```rust
            Key::Character("l" | "L") => state.gpu.view.toggle_line_style(),
```

**Add below it:**

```rust
            Key::Character("[") => state.set_cloud_size(state.gpu.view.cloud_size - 0.25),
            Key::Character("]") => state.set_cloud_size(state.gpu.view.cloud_size + 0.25),
```

## Run

```bash
trunk serve
```

- Open `http://127.0.0.1:8770/`: the lion of `assets/view_local.toml` draws as round, Lambert-lit splats at the manifest's `point_size = 4`, edges darkened by EDL; `[` and `]` scale them, `?edl=0` turns the lighting off, `?cloud=2` doubles the size at startup.
- `?lod=<px>` turns the octree walk on for clouds over `LOD_MIN_POINTS`; the local lion is below it and draws whole, so the cutoff first shows on the streamed scan of lesson 8.

## Why

- A point is not a triangle: rasterising a million quads under MSAA with blending would cost more than the whole scene, so the point pass draws into a private 1x depth + colour pair where the hardware depth test keeps the nearest point, and the scene pays one fullscreen triangle.
- `frag_depth` in the resolve is what makes points and solids occlude each other exactly: the scene's own depth test runs on the point's depth, so no ordering trick is needed between the lanes.
- Records fold camera x placement, tint and radius on the CPU once per frame so the shader does one mat-vec per point; `Key` skips the whole pass while nothing moved, so a still cloud costs a resolve and nothing else.
- The splat radius is world-sized through the placement scale and floored at the manifest's pixels: a far cloud never turns to dust, a near cloud is discs, not squares, and an octree node's discs still tile at their own pitch.
- EDL darkens from four neighbouring depths, so a scan with no normals still reads as a surface; Lambert is only added when every point carries an oct16 normal (4 B each).
- The octree is the kernel's: its point order makes every node's range its own subsample, so the viewer only chooses a cutoff by projected spacing - the same test serves perspective and ortho, and clouds under `LOD_MIN_POINTS` skip it because a node drawn at its coarser spacing is fatter than the whole cloud.
- Upload-local rows keep the append-only rule: `CloudDraw` and `LodNode` are relative to the upload and the cloud, `CloudLane::append` rebases once, and a table that grew rebinds one group instead of rebuilding anything.
- The point lane's targets are lazy: a scene with no cloud never allocates 8 B per pixel it will not read.
