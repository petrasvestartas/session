# 35 Scene struct — the document comes back, and loading stops hurting

> **Big picture.** Since 34e the viewer has NO document. Each `Session` is parsed, walked into
> flat GPU tables, and **thrown away** — deliberately, to survive the stress wall. That was right
> for the wall and is wrong for everything ahead: picking (42) must answer "which OBJECT did the
> ray hit" (a guid), undo (51) must snapshot geometry, save (39) must write a `.pb` back — all of
> that needs the real `Session` alive in memory. And the load path hurts three ways you can watch
> on every reload: ten sheets take ~24 s and **nothing renders until the last one lands**; each
> file's parse is one synchronous block that would freeze any UI drawn during it; and every
> upload clones whole tables just to hand wgpu a contiguous slice. This lesson fixes all of it in
> one continuous pass: `Scene` (app layer) owns the documents AND the merged tables, files append
> one at a time (first sheet on screen in ~2-3 s), the parse is sliced so frames render *during*
> it, and the upload copies nothing.

## What holds the geometry now? (read this first)

| type | where | what it is | lifetime |
|---|---|---|---|
| `Session` | kernel (`session_rust`) | THE document: `lookup: HashMap<guid → Geometry>` + `xforms` (placements live in the SESSION since the Xform refactor) | 34e: dies right after the walk. **35: lives forever, inside `Scene`** |
| `SceneTables` | engine (34e) | flat GPU tables for ONE walked file, anonymous — no guids, no way back to objects | **deleted this lesson** |
| `Scene` | app (**NEW**) | `docs: Vec<Doc { name, place, session }>` + the MERGED tables + viewer bookkeeping: `guid_to_row`, `hidden` | owned by `State`, lives as long as the viewer |
| `ArenaUpload` | engine (**NEW**) | `SceneTables` reborn as the app→engine HANDOFF: `Scene` owns one, `Gpu::set_scene` borrows it and uploads | as long as the Scene |

So nothing exotic replaces `Session` — `Session` IS the data structure, kept alive per file in
`state.scene.docs`. When later lessons say "the document", they mean the sessions in
`scene.docs`; when they say "row" or "instance id", they mean the GPU-side index
`scene.guid_to_row` translates guids into. Placement is layered exactly like the data:
`instance model = manifest place × session.world_xforms()[guid]` — the manifest says where a
SHEET sits, the session says where an OBJECT sits inside it.

<svg viewBox="0 0 680 200" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="before: Sessions are dropped after walking and nothing draws until all files load; after: Scene keeps every Session, appends files into shared tables, parse is sliced, upload is zero-copy" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="150" y="16" fill="#888" text-anchor="middle">34e — no document, no pixels until file 10</text>
  <rect x="30" y="26" width="240" height="120" fill="none" stroke="#3a3a3a"/>
  <text x="150" y="48" fill="#d7dae0" text-anchor="middle">fetch → parse → walk ×10</text>
  <text x="150" y="70" fill="#c66" text-anchor="middle">Session DROPPED ×10</text>
  <text x="150" y="88" fill="#666" text-anchor="middle">merge ALL → Gpu::new → first frame</text>
  <text x="150" y="130" fill="#6fb3ff" text-anchor="middle">~24 s of white screen</text>
  <text x="300" y="90" fill="#6fb3ff" font-size="16" text-anchor="middle">▶</text>
  <text x="480" y="16" fill="#888" text-anchor="middle">35 — app owns the documents</text>
  <rect x="360" y="26" width="240" height="56" fill="none" stroke="#6fb3ff"/>
  <text x="480" y="46" fill="#d7dae0" text-anchor="middle">app::Scene</text>
  <text x="480" y="64" fill="#666" text-anchor="middle">docs (KEPT) · tables · guid_to_row · hidden</text>
  <text x="480" y="98" fill="#6fb3ff" text-anchor="middle">↓ &amp;ArenaUpload, once per appended file</text>
  <text x="480" y="112" fill="#555" text-anchor="middle">sliced parse · zero-copy set_scene</text>
  <rect x="360" y="122" width="240" height="48" fill="none" stroke="#3a3a3a"/>
  <text x="480" y="142" fill="#d7dae0" text-anchor="middle">engine::Gpu — new() empty · set_scene(all)</text>
  <text x="480" y="160" fill="#6fb3ff" text-anchor="middle">first sheet ~2-3 s, rest stream in, UI live</text>
  <text x="480" y="192" fill="#555" text-anchor="middle">engine code names no Session / Mesh / BRep</text>
</svg>

**The 34e stress wall retires here** (the `STRESS_GRID` cell machinery is deleted — the manifest
already says where every sheet sits), but the ten-sheet scene itself stays: it is now the
progressive-loading test bed.

Two design facts the whole lesson leans on, stated once:

- **Why not a Web Worker for the parse?** The kernel `Session` is built on `Rc` — not `Send` —
  so it cannot cross or share a thread boundary; moving parse off-thread would mean serializing
  the result back, i.e. parsing twice. Slicing it ON the main thread is the honest fix, and the
  kernel's `to_proto`/`from_proto` split (34h pt3) exists precisely so a caller can own the
  pacing.
- **WebGPU zero-initializes buffers.** That one guarantee pays twice below: lanes can be spliced
  GPU-side by two `write_buffer` calls into a right-sized buffer (no CPU-side concat clone), and
  an "empty category" is just a 1-row zeroed buffer — the placeholder 34e used to push by hand.

## Files we touch

```
src/engine/gpu/mod.rs   # Step 1: SceneTables/walk_session/push_mesh/merge OUT; ArenaUpload +
                        #         zero-copy set_scene IN; new() starts EMPTY; row structs pub
src/engine/gpu/adapters.rs  # Step 1: DELETED — converters move to scene.rs
src/app/scene.rs        # Step 2: Scene { docs, tables, guid_to_row, hidden } + add_file()
                        #         appended BELOW the manifest code that already lives there
Cargo.toml              # Step 3: + prost (decode the proto in the app layer)
src/app/persistence.rs  # Step 3: session_from_bytes_chunked + next_tick; session_from_bytes OUT
src/state.rs            # Step 4: State gains `scene`; the load loop moves OUT (to lib.rs)
src/lib.rs              # Step 5: Msg::Ready/Msg::File — the progressive loader
```

> ⚠ Nothing compiles between Step 1 and the end of Step 5 — the walk is changing owners. Type it
> all, then `cargo check`.

## Step 1 — `Gpu` forgets the document: `src/engine/gpu/mod.rs`

Eleven edits, strictly top to bottom.

**1a. Imports.** Find at the top:

```rust
mod adapters;
use adapters::{line_to_segment, nurbscurve_to_segments, point_to_glyph, polyline_to_segments, encode_width};
use bytemuck::Zeroable;
use session_rust::{Mesh, Xform, RenderVertex, Point, Geometry};
use session_rust::mesh::ColorMode;
```

Replace all five lines with (one import only — `Zeroable` goes too: the by-hand `::zeroed()`
placeholders disappear with the zero-init trick, and `storage_buffer`'s `T::zeroed()` resolves
through its `Pod` bound without the import):

```rust
use session_rust::{Xform, RenderVertex, Point};
```

Also fix one comment: find `/// Routing lives in `walk_session`, one draw per lane in `clear`.`
and make it `/// Routing lives in `app::scene::Scene::add_file`, one draw per lane in `clear`.`

**1b. `STRESS_GRID` dies, `SceneTables` becomes `ArenaUpload`.** Find:

```rust
/// Grid floor for load testing: at least STREE_GRID2 cells, cycling the loaded files
const STRESS_GRID: u32 = 1;
```

Delete both lines. Then find the `SceneTables` block (the doc comment beginning
`/// One loaded file, walked into GPU-ready tables.` down to the struct's closing `}`) and
replace the whole thing with:

```rust
/// Everything `Gpu` needs to fill its buffers, built and OWNED by `app::scene::Scene` — the
/// engine borrows it, uploads, and forgets. Lanes stay apart (SOLID pipes/spheres vs FLAT
/// segments/glyphs) and are spliced solid-first at upload. `objects` holds the TRUE
/// per-object transform + tint + flags; `Gpu` builds instance rows from it and rebases them as
/// the camera moves (33). No Mesh, no Session, no wgpu type on the app side of this line.
pub struct ArenaUpload {
    pub verts: Vec<RenderVertex>,
    pub vids: Vec<u32>,
    pub idx: Vec<u32>,
    pub pipes: Vec<CylinderSegment>,   // SOLID lane: mesh/BRep edges, drawn as 3D cylinders
    pub spheres: Vec<GlyphPoint>,      // SOLID lane: mesh/BRep vertices, radius matched to the pipes
    pub segments: Vec<CylinderSegment>,// FLAT lane: line/polyline, drawn as camera-facing ribbons
    pub glyphs: Vec<GlyphPoint>,       // FLAT lane: points, drawn as SDF dots
    pub objects: Vec<(Xform, [f32; 4], u32)>,   // true model, tint, flags
    pub min: [f32; 3],
    pub max: [f32; 3],
}

impl ArenaUpload {
    pub fn new() -> Self {
        Self {
            verts: Vec::new(),
            vids: Vec::new(),
            idx: Vec::new(),
            pipes: Vec::new(),
            spheres: Vec::new(),
            segments: Vec::new(),
            glyphs: Vec::new(),
            objects: Vec::new(),
            min: [f32::INFINITY; 3],
            max: [f32::NEG_INFINITY; 3],
        }
    }
}
```

**1c. The field grows flags, the layouts survive.** In `pub struct Gpu`, find:

```rust
    objects_base: Vec<(Xform, [f32; 4])>, // TRUE world model+color; isntance[] is rebased from this
```

Replace that ONE line with the seven below — the `objects_base` tuple grows a `u32` (flags), and
the six `*_layout` fields are NEW, inserted right after it. Delete nothing: `instance_buffer` and
everything under it stays. (Today the layouts are locals inside `new()`, dropped when it returns;
`set_scene` must rebuild bind groups after buffer recreation, and pipelines when MSAA flips, so
they move into the struct — step 1g stores them in `Ok(Self { … })`.)

```rust
    objects_base: Vec<(Xform, [f32; 4], u32)>, // TRUE world model+tint+flags; instances[] is rebased from this
    // Layouts survive so set_scene can rebuild bind groups (and pipelines on an MSAA change).
    mvp_layout: wgpu::BindGroupLayout,
    time_layout: wgpu::BindGroupLayout,
    instance_layout: wgpu::BindGroupLayout,
    line_layout: wgpu::BindGroupLayout,
    segment_layout: wgpu::BindGroupLayout,
    glyph_layout: wgpu::BindGroupLayout,
```

**1d. The signature — `new()` takes NOTHING but the window.** Find:

```rust
    /// Set up the five wgpu objects, in order: Instance → Surface → Adapter → Device + Queue → configure.
    /// `files` carries each loaded file WITH the placement the scene manifest gave it - the
    /// viewer no longer decides where a sheet goes (see `app::scene`).
    pub async fn new(
        window: std::sync::Arc<winit::window::Window>,
        files: &[(SceneTables, Xform)]) -> anyhow::Result<Self> {
```

Replace with:

```rust
    /// Set up the five wgpu objects, in order: Instance → Surface → Adapter → Device + Queue → configure.
    /// The scene starts EMPTY — every upload, including the first file, goes through `set_scene`
    /// (progressive loading calls it once per appended file). One code path, not two.
    pub async fn new(
        window: std::sync::Arc<winit::window::Window>) -> anyhow::Result<Self> {
```

Then a few pages down find:

```rust
        // Depth and MSAA
        let samples = Self::msaa_for(files);
        log::info!("msaa: {}x", samples);
```

Replace with:

```rust
        // Depth and MSAA — the empty scene starts flat (1x); set_scene flips to 4x when the
        // first solid geometry arrives (sample count belongs to the render PASS).
        let samples = 1;
```

**1e. The merge loop dies — empty placeholders take its place.** Find the block that starts at:

```rust
        // Merge the per-file tables into one arena: mesh indices shift by the vertex base,
        // row ids (vids / instance_id) by the objects base, so every file keeps distinct rows.
```

and **delete everything from that comment down to and including the grid log**:

```rust
        log::info!("grid: {} cells x {} files, {} objects {} arena verts {} segments ({} pipes) {} glyphs ({} spheres)",
            cells, files.len(), instances.len(), verts.len(), segments.len(), pipe_count, glyphs.len(), sphere_count);
```

The deletion is ~155 lines. Check off what it swallows, top to bottom: the eight `let mut …`
table declarations (`verts` … `objects_base`) plus `scene_min`/`scene_max`, the commented-out
flat merge, the whole `cells`/`cols` stress-grid machinery, the `is_finite()` bounds fallback,
the instances build, the lane concat (`let mut segments = { pipes.extend(segments); pipes };` and
its glyph twin), `segment_count`/`glyph_count`, the `points` declaration, the four `is_empty()`
padding guards, and `arena_index_count`. The grid log is the LAST line deleted; the
`let instance_buffer = …` line below it is the first survivor. **In the hole, insert:**

```rust
        // The scene-shaped fields start as EMPTY placeholders (WebGPU zero-initializes
        // buffers, and every *_count is 0, so the first frames draw nothing) — the loader calls
        // set_scene the moment the first file's tables exist.
        let instances: Vec<Instance> = vec![Instance {
            model: Xform::identity().to_f32(), color: [0.5, 0.5, 0.5, 1.0], flags: 0, _pad: [0; 3],
        }];
        let objects_base: Vec<(Xform, [f32; 4], u32)> = Vec::new();
        let (pipe_count, segment_count, sphere_count, glyph_count) = (0u32, 0u32, 0u32, 0u32);
        let arena_index_count = 0u32;
        let (scene_min, scene_max) = ([0.0f32; 3], [0.0f32; 3]);
```

One casualty of the deletion must come back. The deletion removed the `points` declaration, but
its user SURVIVED: `let point_count = points.len() as u32;` sits much further down in `new()`,
under the `// Point buffer + the cloud uniform` comment (past the arena/template/segment/glyph
buffer creation). Find it there and put the declaration back directly above it:

```rust
        let points: Vec<CloudPoint> = Vec::new();
        let point_count = points.len() as u32;
```

**1f. The scene buffers in `new()` become zeroed placeholders** (their real creation now lives
in `set_scene`). Three replacements. Find:

```rust
        let arena_vbo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("arena.vbo"),
            contents: bytemuck::cast_slice(&verts), usage: wgpu::BufferUsages::VERTEX,
        });

        let arena_vids = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("arena.vids"),
            contents: bytemuck::cast_slice(&vids), usage: wgpu::BufferUsages::VERTEX,
        });

        let arena_ibo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("arena.ibo"),
            contents: bytemuck::cast_slice(&idx), usage: wgpu::BufferUsages::INDEX,
        });
```

Replace with:

```rust
        let arena_vbo = zeroed_buffer(&device, "arena.vbo", std::mem::size_of::<RenderVertex>() as u64, wgpu::BufferUsages::VERTEX);
        let arena_vids = zeroed_buffer(&device, "arena.vids", 4, wgpu::BufferUsages::VERTEX);
        let arena_ibo = zeroed_buffer(&device, "arena.ibo", 12, wgpu::BufferUsages::INDEX);
```

Find:

```rust
        // One storage row per edge (VERTEX-visible, read-only) - the segment table.
        let segment_buffer =  storage_buffer(&device, "segments.buffer", &segments);
```

Replace with:

```rust
        // One storage row per edge (VERTEX-visible, read-only) - the segment table.
        let segment_buffer = zeroed_buffer(&device, "segments.buffer", std::mem::size_of::<CylinderSegment>() as u64,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
```

Find `let glyph_buffer =  storage_buffer(&device, "glyphs.buffer", &glyphs);` — careful, the
source has TWO spaces after the `=` (this find and the `segment_buffer` one above), so search for
`"glyphs.buffer"` if a pasted line misses — and replace with:

```rust
        let glyph_buffer = zeroed_buffer(&device, "glyphs.buffer", std::mem::size_of::<GlyphPoint>() as u64,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
```

(`let instance_buffer =  storage_buffer(…)` — the first line after the 1e hole, ABOVE the three
arena lines you just replaced — stays as is: it uploads the one placeholder instance).

**1g. `Ok(Self { … })` stores the layouts.** In the struct literal at the end of `new()`, find:

```rust
            instances,
            last_origin: None,
            objects_base,
```

and insert the six layouts after `objects_base,`:

```rust
            mvp_layout,
            time_layout,
            instance_layout,
            line_layout,
            segment_layout,
            glyph_layout,
```

**1h. `walk_session` becomes the zero-copy `set_scene`.** Find the doc comment beginning
`/// One file → compact tables.` (it continues `Called from state.rs BEFORE Gpu::new …` — delete
all of it) and **delete from it down to and including `walk_session`'s closing `}`** (the last lines of the deletion are the planar block and `        t\n    }`).
In its place, insert:

```rust
    /// Replace the whole scene from the app's tables — called once per file while progressive
    /// loading appends. ZERO-COPY: lanes are written straight from the Scene's Vecs into fresh
    /// buffers (two write_buffer calls splice SOLID-first), so nothing is cloned per append.
    /// WebGPU zero-initializes buffers, so an empty category is just a 1-row zeroed buffer.
    /// An MSAA flip (first solid file after flat-only ones) also rebuilds the depth/msaa targets
    /// and every pipeline, since sample count belongs to the render PASS.
    pub fn set_scene(&mut self, up: &ArenaUpload) {
        use wgpu::util::DeviceExt;

        // Instance rows: rebuilt from the true transforms (rebase state, must live CPU-side).
        self.objects_base = up.objects.clone();
        self.instances.clear();
        self.instances.extend(up.objects.iter().map(|(m, c, f)| Instance {
            model: m.to_f32(), color: *c, flags: *f, _pad: [0; 3],
        }));
        if self.instances.is_empty() {
            self.instances.push(Instance { model: Xform::identity().to_f32(), color: [0.5, 0.5, 0.5, 1.0], flags: 0, _pad: [0; 3] });
        }
        self.instance_buffer = storage_buffer(&self.device, "instance.buffer", &self.instances);
        self.instance_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("instances.bind_group"),
            layout: &self.instance_layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: self.instance_buffer.as_entire_binding() }],
        });

        // Mesh arena — straight from the Scene's Vecs; the empty case is a zeroed placeholder.
        if up.verts.is_empty() {
            self.arena_vbo = zeroed_buffer(&self.device, "arena.vbo", std::mem::size_of::<RenderVertex>() as u64, wgpu::BufferUsages::VERTEX);
            self.arena_vids = zeroed_buffer(&self.device, "arena.vids", 4, wgpu::BufferUsages::VERTEX);
            self.arena_ibo = zeroed_buffer(&self.device, "arena.ibo", 12, wgpu::BufferUsages::INDEX);
            self.arena_index_count = 3;
        } else {
            self.arena_vbo = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("arena.vbo"),
                contents: bytemuck::cast_slice(&up.verts), usage: wgpu::BufferUsages::VERTEX,
            });
            self.arena_vids = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("arena.vids"),
                contents: bytemuck::cast_slice(&up.vids), usage: wgpu::BufferUsages::VERTEX,
            });
            self.arena_ibo = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("arena.ibo"),
                contents: bytemuck::cast_slice(&up.idx), usage: wgpu::BufferUsages::INDEX,
            });
            self.arena_index_count = up.idx.len() as u32;
        }

        // The two lane tables: one buffer each, SOLID rows first, spliced by two writes.
        self.pipe_count = up.pipes.len() as u32;
        self.segment_count = (up.pipes.len() + up.segments.len()) as u32;
        let rows = (self.segment_count as u64).max(1);
        self.segment_buffer = zeroed_buffer(&self.device, "segments.buffer", rows * std::mem::size_of::<CylinderSegment>() as u64,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        self.queue.write_buffer(&self.segment_buffer, 0, bytemuck::cast_slice(&up.pipes));
        self.queue.write_buffer(&self.segment_buffer, up.pipes.len() as u64 * std::mem::size_of::<CylinderSegment>() as u64,
            bytemuck::cast_slice(&up.segments));
        self.segment_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("segments.bind_group"),
            layout: &self.segment_layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: self.segment_buffer.as_entire_binding() }],
        });

        self.sphere_count = up.spheres.len() as u32;
        self.glyph_count = (up.spheres.len() + up.glyphs.len()) as u32;
        let rows = (self.glyph_count as u64).max(1);
        self.glyph_buffer = zeroed_buffer(&self.device, "glyphs.buffer", rows * std::mem::size_of::<GlyphPoint>() as u64,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        self.queue.write_buffer(&self.glyph_buffer, 0, bytemuck::cast_slice(&up.spheres));
        self.queue.write_buffer(&self.glyph_buffer, up.spheres.len() as u64 * std::mem::size_of::<GlyphPoint>() as u64,
            bytemuck::cast_slice(&up.glyphs));
        self.glyph_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("glyphs.bind_group"),
            layout: &self.glyph_layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: self.glyph_buffer.as_entire_binding() }],
        });

        self.last_origin = None;   // force the next frame to rebase against the new table
        self.scene_min = if up.min[0].is_finite() { up.min } else { [0.0; 3] };
        self.scene_max = if up.min[0].is_finite() { up.max } else { [0.0; 3] };

        log::info!("scene: {} objects {} arena verts {} segments ({} pipes) {} glyphs ({} spheres)",
            self.instances.len(), up.verts.len(), self.segment_count, self.pipe_count, self.glyph_count, self.sphere_count);

        let samples = Self::msaa_for(up);
        if samples != self.samples {
            self.samples = samples;
            self.depth_view = Self::create_depth_view(&self.device, &self.config, samples);
            self.msaa_view = Self::create_msaa_view(&self.device, &self.config, samples);
            self.pipelines = Pipelines::new(&self.device, samples, self.config.format,
                &self.mvp_layout, &self.time_layout, &self.instance_layout,
                &self.line_layout, &self.segment_layout, &self.glyph_layout);
            log::info!("msaa: {}x", samples);
        }
    }
```

**1i. `msaa_for` reads the upload.** Two-line change: the signature and the `solid` line (the
`if solid` line is shown only as context — it stays). Find:

```rust
    fn msaa_for(files: &[(SceneTables, Xform)]) -> u32 {
        let solid = files.iter().any(|(f, _)| !f.verts.is_empty() || !f.pipes.is_empty() || !f.spheres.is_empty());
        if solid { 4 } else { 1 }
```

Replace the first two lines with:

```rust
    fn msaa_for(up: &ArenaUpload) -> u32 {
        let solid = !up.verts.is_empty() || !up.pipes.is_empty() || !up.spheres.is_empty();
        if solid { 4 } else { 1 }
```

**1j. `rebuild_instances` learns the third tuple element.** Find the LIVE loop (not the
commented-out copy just above it):

```rust
        for (i, (model, color)) in self.objects_base.iter().enumerate() {
            let mut m = model.to_f32();
```

Replace the first line with (the flag is set once at build; rebasing never touches it):

```rust
        for (i, (model, color, _)) in self.objects_base.iter().enumerate() {
```

**1k. Row structs go `pub`, `zeroed_buffer` appears, movers leave.** Near the bottom:

- `struct Instance {` → `pub struct Instance {` and give it the flag const right after the
  struct (fields stay private — only `Gpu` builds rows; `Scene` just names the flag):

```rust
impl Instance {
    /// Row is skipped by the draw (46). Bit 0 is reserved for FLAG_SELECTED (45).
    pub const FLAG_HIDDEN: u32 = 1 << 1;
}
```

- `struct CylinderSegment{` and `struct GlyphPoint{` get `pub` on the struct keyword **and on
  every field** (Scene constructs them field-by-field across the module boundary; keep each
  field's comment, only visibility changes).
- **Delete** `fn push_mesh(…)`, `fn xform_point(…)` and `fn grow_bounds(…)` whole — they name
  document types and move to `scene.rs` in Step 2.
- **Delete the file `src/engine/gpu/adapters.rs`** — its converters and `encode_width` also
  reappear in `scene.rs`.
- Finally insert the zero-init helper directly above `fn storage_buffer<`:

```rust
/// A fresh buffer of `size` bytes, zero-initialized by WebGPU — the write_buffer splice and the
/// empty-category placeholders both rely on that guarantee.
fn zeroed_buffer(device: &wgpu::Device, label: &str, size: u64, usage: wgpu::BufferUsages) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size,
        usage,
        mapped_at_creation: false,
    })
}
```

## Step 2 — `Scene` owns the documents: `src/app/scene.rs`

The file already holds the MANIFEST (which files, where). The document side is APPENDED below
`auto_grid` — manifest above says WHERE, `Scene` below owns WHAT. Read it as four parts: `Doc`
(one kept session + its placement), `Scene` (docs + the merged tables + bookkeeping),
`add_file()` (34e's walk, appending into the SHARED tables so each file costs only its own
walk), and the converters (34's `adapters.rs` + `push_mesh` + the bounds helpers, moved
verbatim). Append exactly this:

```rust
// ─────────────────────────────────────────────────────────────────────────────────────────────
// The DOCUMENT side of the scene: manifest above says WHERE, `Scene` below owns WHAT.

use std::collections::{HashMap, HashSet};
use session_rust::{Session, Geometry, Mesh, Line, Point, Polyline, NurbsCurve, RenderVertex};
use session_rust::mesh::ColorMode;
use crate::engine::gpu::{ArenaUpload, Instance, CylinderSegment, GlyphPoint};

/// One loaded file: the kernel `Session` (kept ALIVE — picking/undo/save need it) plus the
/// placement the manifest gave it.
pub struct Doc {
    pub name: String,
    pub place: Xform,
    pub session: Session,
}

/// The open document set + the merged GPU tables. `add_file` walks ONE new session straight
/// into the shared tables (rows appended, never rebuilt), so progressive loading costs each
/// file only its own walk. Viewer-only bookkeeping (row order, guid→row, hidden) lives here —
/// never in the kernel type that three languages share.
pub struct Scene {
    pub docs: Vec<Doc>,
    pub tables: ArenaUpload,
    order: Vec<String>,                    // renderable guids, global row order across docs
    pub guid_to_row: HashMap<String, u32>,
    pub hidden: HashSet<String>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            docs: Vec::new(),
            tables: ArenaUpload::new(),
            order: Vec::new(),
            guid_to_row: HashMap::new(),
            hidden: HashSet::new(),
        }
    }

    /// Walk one session into the shared tables. The lesson-34 walk, moved out of `Gpu`:
    /// - placement = manifest `place` × the session's own `world_xforms()` (one downward pass;
    ///   per-object `world_xform()` rescans the tree each call — quadratic over a session)
    /// - `session.order()` is the kernel's CANONICAL order, deterministic across runs and
    ///   languages — the row a guid gets here is the row it keeps (picking/selection rely on it)
    /// - per-FILE planar test: a z≡0 sheet flips its linework into the world-mm lane (34f)
    pub fn add_file(&mut self, name: String, session: Session, place: Xform) {
        let seg0 = self.tables.segments.len();
        let pipe0 = self.tables.pipes.len();
        let vert0 = self.tables.verts.len();
        let sphere0 = self.tables.spheres.len();
        let glyph0 = self.tables.glyphs.len();
        let obj0 = self.tables.objects.len();

        let world = session.world_xforms();
        let placement = |guid: &str| world.get(guid).cloned().unwrap_or_else(Xform::identity);
        let t = &mut self.tables;
        for guid in session.order() {
            let Some(geom) = session.lookup.get(&guid) else { continue };
            let ri = t.objects.len() as u32;
            let flags = if self.hidden.contains(&guid) { Instance::FLAG_HIDDEN } else { 0 };
            let placed = &place * &placement(&guid);
            match geom {
                // 3D geometry takes the SOLID lane: edges are real cylinders and vertices real
                // spheres, so ink is lifted off the surface by its own radius instead of being
                // a flat quad at the surface's depth (which loses the depth test at silhouettes).
                Geometry::Mesh(m) => {
                    t.objects.push((placed, [1.0; 4], flags));
                    push_mesh(m, ri, &mut t.verts, &mut t.vids, &mut t.idx,
                        &mut t.pipes, &mut t.spheres);
                }
                Geometry::BRep(b) => {
                    let mut bm = b.mesh();
                    bm.set_objectcolor(b.surfacecolor.clone());
                    t.objects.push((placed, [1.0; 4], flags));
                    push_mesh(&bm, ri, &mut t.verts, &mut t.vids, &mut t.idx,
                        &mut t.pipes, &mut t.spheres);
                }
                Geometry::Line(l) => {
                    t.objects.push((placed, [1.0; 4], flags));
                    t.segments.push(line_to_segment(l, ri));
                }
                Geometry::Polyline(pl) => {
                    t.objects.push((placed, [1.0; 4], flags));
                    t.segments.extend(polyline_to_segments(pl, ri));
                }
                // Curves ride the FLAT lane too - sampled to segments, they ARE polylines by
                // the time the GPU sees them.
                Geometry::NurbsCurve(c) => {
                    t.objects.push((placed, [1.0; 4], flags));
                    t.segments.extend(nurbscurve_to_segments(c, ri));
                }
                Geometry::Point(p) => {
                    t.objects.push((placed, [1.0; 4], flags));
                    t.glyphs.push(point_to_glyph(p, ri));
                }
                // Later lessons - the match must stay exhaustive over all 11 variants
                Geometry::Plane(_) | Geometry::OBB(_) |
                Geometry::PointCloud(_) | Geometry::Element(_) |
                Geometry::NurbsSurface(_) => { continue }
            }
            self.guid_to_row.insert(guid.clone(), ri);
            self.order.push(guid);
        }

        // This FILE's extents in WORLD placement (each row through its object's full xform) —
        // both the planar test and the scene bounds see what is actually drawn.
        let (mut fmin, mut fmax) = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);
        for (i, v) in t.verts.iter().enumerate().skip(vert0) {
            if let Some(&ri) = t.vids.get(i) {
                if let Some((xf, _, _)) = t.objects.get(ri as usize) {
                    grow_bounds(&mut fmin, &mut fmax, xform_point(xf, v.position));
                }
            }
        }
        for s in t.pipes.iter().skip(pipe0).chain(t.segments.iter().skip(seg0)) {
            if let Some((xf, _, _)) = t.objects.get(s.instance_id as usize) {
                grow_bounds(&mut fmin, &mut fmax, xform_point(xf, s.p0));
                grow_bounds(&mut fmin, &mut fmax, xform_point(xf, s.p1));
            }
        }
        for g in t.spheres.iter().skip(sphere0).chain(t.glyphs.iter().skip(glyph0)) {
            if let Some((xf, _, _)) = t.objects.get(g.instance_id as usize) {
                grow_bounds(&mut fmin, &mut fmax, xform_point(xf, g.center));
            }
        }
        for k in 0..3 {
            t.min[k] = t.min[k].min(fmin[k]);
            t.max[k] = t.max[k].max(fmax[k]);
        }

        // 2D drawing sheets (exactly planar, z = 0 - every PDF conversion gets paper space)
        // lineweights: kernel width (mm on the sheet) - the radius world lane, so zooming out
        // thins the ink like a real print. 3D model files keep screen-constant px linework.
        let planar = fmin[2].is_finite() && (fmax[2] - fmin[2]).abs() < 1e-3;
        if planar {
            for s in t.pipes.iter_mut().skip(pipe0).chain(t.segments.iter_mut().skip(seg0)) {
                s.radius = if s.radius < 0.0 { -s.radius * 0.5 } else { 0.5 }
            }
        }

        let _ = obj0;
        self.docs.push(Doc { name, place, session });
    }
}
```

Then, below that, paste the converters — `line_to_segment`, `polyline_to_segments`,
`nurbscurve_to_segments`, `point_to_glyph`, `encode_width` (bodies UNCHANGED from the deleted
`adapters.rs`, `pub`/`pub(super)` dropped — they're file-local now) and, verbatim from the
deleted engine code, `push_mesh`, `xform_point`, `grow_bounds`. Nothing in their bodies changes;
they name `Mesh`/`Line`/`Point` — document types — which is exactly why they now live in the app
layer.

Four things to notice while typing:

- The import block adds no `Xform` — the manifest code already at the top of this file imports
  it (`use session_rust::Xform;`), so `Doc { place: Xform }` resolves.
- `add_file` walks straight into the GLOBAL tables, so `push_mesh`'s `base = verts.len()`
  index-rebasing works unchanged — there is no separate per-file merge step anymore, and
  appending file #7 never touches rows 1-6.
- The planar test is per FILE (its own new rows only): a 2D sheet keeps paper-space lineweights
  even when a 3D model is loaded next to it.
- `obj0` is captured alongside the other bases but nothing reads it yet — `let _ = obj0;` at the
  bottom keeps the unused-variable warning away until a later lesson needs the per-file row base.

## Step 3 — the sliced parse: `src/app/persistence.rs`

`Session::pb_loads` is one synchronous block — 0.7-2.6 s per sheet during which the browser
cannot paint. The kernel's `from_proto` split (34h pt3) lets the app own the pacing instead:
decode the proto whole (prost is fast), then convert objects 25k at a time with a REAL browser
yield between slices.

First the dependency: in `Cargo.toml`, find `js-sys = "0.3"` and add below it (the SAME major
version the kernel uses, or the `Message` trait won't line up):

```toml
prost = "0.14"
```

Then in `persistence.rs`, **delete `session_from_bytes` whole** (with its doc comment — the
chunked version replaces it) and append at the end of the file:

```rust
// ── chunked parsing: convert the decoded proto in slices, yielding between them ──

use std::rc::Rc;
use prost::Message;
use session_rust::proto;
use session_rust::{Geometry, Line, Mesh, NurbsCurve, NurbsSurface, OBB, Plane, Point, Polyline, PointCloud, BRep, Element, Xform};
use session_rust::tree::{Tree, TreeNode};
use session_rust::graph::Graph;

/// Objects converted per slice before the loader hands the browser one macrotask — the whole
/// point is that a frame can render BETWEEN slices, so a 250k-object parse stops freezing the UI.
const CHUNK: usize = 25_000;

/// One macrotask (setTimeout 0). A microtask (Promise.resolve) would NOT let the browser paint.
async fn next_tick() {
    let p = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 0)
            .unwrap();
    });
    let _ = JsFuture::from(p).await;
}

/// `Session::pb_loads`, unrolled with awaits: decode the proto whole (one short block — prost is
/// fast), then convert objects CHUNK at a time. Same result, no multi-second freeze. `.json`
/// files stay on the synchronous path (they are small).
pub async fn session_from_bytes_chunked(url: &str, bytes: &[u8]) -> Session {
    if url.ends_with(".json") {
        return Session::file_json_loads(&String::from_utf8_lossy(bytes));
    }
    let Ok(p) = proto::Session::decode(bytes) else { return Session::default() };
    let mut s = Session::new(&p.name);
    s.set_guid(p.guid.clone());

    let mut n = 0usize;
    macro_rules! chunk {
        ($vec:expr, $ty:ident, $variant:ident, $slot:ident) => {
            for x in $vec {
                let g = Rc::new($ty::from_proto(x));
                s.lookup.insert(g.guid().to_string(), Geometry::$variant(Rc::clone(&g)));
                s.objects.$slot.push(g);
                n += 1;
                if n % CHUNK == 0 { next_tick().await; }
            }
        };
        // from_proto -> Result for the nested types; a bad object is skipped, not fatal
        (fallible $vec:expr, $ty:ident, $variant:ident, $slot:ident) => {
            for x in $vec {
                let Ok(v) = $ty::from_proto(x) else { continue };
                let g = Rc::new(v);
                s.lookup.insert(g.guid().to_string(), Geometry::$variant(Rc::clone(&g)));
                s.objects.$slot.push(g);
                n += 1;
                if n % CHUNK == 0 { next_tick().await; }
            }
        };
    }

    if let Some(o) = p.objects {
        s.objects.set_guid(o.guid);
        s.objects.name = o.name;
        chunk!(o.points, Point, Point, points);
        chunk!(o.lines, Line, Line, lines);
        chunk!(o.planes, Plane, Plane, planes);
        chunk!(fallible o.bboxes, OBB, OBB, bboxes);
        chunk!(o.polylines, Polyline, Polyline, polylines);
        chunk!(o.pointclouds, PointCloud, PointCloud, pointclouds);
        chunk!(o.meshes, Mesh, Mesh, meshes);
        chunk!(o.nurbscurves, NurbsCurve, NurbsCurve, nurbscurves);
        chunk!(fallible o.nurbssurfaces, NurbsSurface, NurbsSurface, nurbssurfaces);
        chunk!(fallible o.breps, BRep, BRep, breps);
        chunk!(fallible o.elements, Element, Element, elements);
    }

    // Tree / graph / xforms — small, rebuilt synchronously exactly as pb_loads does.
    if let Some(tp) = &p.tree {
        s.tree = Tree::new(&tp.name);
        s.tree.set_guid(tp.guid.clone());
        if let Some(rp) = &tp.root {
            fn build(proto: &proto::TreeNode) -> Rc<std::cell::RefCell<TreeNode>> {
                let node = TreeNode::new(&proto.name);
                for c in &proto.children {
                    let child = build(c);
                    node.borrow_mut().add(&child);
                }
                node
            }
            let root = build(rp);
            s.tree.add(&root, None);
        }
    }
    if let Some(gp) = &p.graph {
        s.graph = Graph::new(&gp.name);
        s.graph.set_guid(gp.guid.clone());
        for (name, v) in &gp.vertices { s.graph.add_node(name, &v.attribute); }
        for e in &gp.edges { s.graph.add_edge(&e.v0, &e.v1, &e.attribute); }
    }
    for entry in &p.xforms {
        if let Some(xf) = &entry.xform {
            let mut xform = Xform::identity();
            xform.set_guid(xf.guid.clone());
            xform.name = xf.name.clone();
            for (i, val) in xf.matrix.iter().enumerate().take(16) {
                xform.m[i] = *val;
            }
            s.xforms.insert(entry.guid.clone(), xform);
        }
    }
    s
}
```

Three details worth pausing on:

- **Why not a Web Worker?** Answered at the top: `Rc` keeps `Session` on the main thread —
  slicing IS the fix.
- **The `fallible` matcher word is not `try`** — `try` is a reserved keyword and won't parse as
  a macro literal in that position.
- **`next_tick` must be a macrotask.** `Promise.resolve().await` yields only the microtask
  queue — the browser still cannot paint. `setTimeout(0)` gives up the event loop turn. Cost:
  ~1 ms × (objects/25k) per file — a 250k-object sheet spends ~10 ms extra and stops blocking
  for 1.6 s.

## Step 4 — `State` holds the document set: `src/state.rs`

The load loop LEAVES this file (it becomes the progressive loader in lib.rs). Keep the `//!`
module header; replace everything between it and `/// Forward a canvas resize` — the whole `use`
block, the `DEMO_SCENE_URL` const, `pub struct State`, and all of `State::new` — with the block
below. It ends at `new`'s closing brace: `impl State {` stays open, `resize`/`render` follow
unchanged inside it.

```rust
use std::sync::Arc;
use winit::window::Window;

use crate::engine::gpu::Gpu;
use crate::camera::Camera;
use crate::app::scene::Scene;
use crate::engine::performance::now_ms;

pub struct State {
    pub window: Arc<Window>,
    pub gpu: Gpu,
    pub camera: Camera,
    pub scene: Scene, // the DOCUMENT set (kernel Sessions + placements + row/hidden bookkeeping)
}

impl State {

    /// Wire the stack around an already-populated `Scene` (the loader in lib.rs builds it from
    /// the manifest's FIRST file, then streams the rest through `Gpu::set_scene`).
    pub async fn new(window: Arc<Window>, scene: Scene) -> anyhow::Result<Self>{
        let t0 = now_ms();
        let mut gpu = Gpu::new(window.clone()).await?;
        gpu.set_scene(&scene.tables);
        log::info!("gpu init {:.0}ms", now_ms() - t0);
        Ok(Self {window, gpu, camera: Camera::new(), scene })
    }
```

`resize`/`render` are untouched — `render()` still does 34c's anchor dance against `gpu` and
`camera` only. (`persistence` and the manifest imports move to lib.rs with the loop.) Note the
shape: `Gpu::new` builds an EMPTY viewer, `set_scene` fills it — the same two calls the loader
makes for every later file, so there is exactly one upload path to get right.

## Step 5 — the progressive loader: `src/lib.rs`

**5a. The message type.** Find:

```rust
pub use state::State;
use crate::camera::View;
```

and extend it to:

```rust
pub use state::State;
use crate::camera::View;
use crate::app::persistence;
use crate::app::scene::{auto_grid, Manifest, Scene};

// The scene: which sheets, and where each one sits. Fetched at runtime, so re-arranging the
// scene is a text edit in assets/scenes/, not a rebuild (app/scene.rs).
const DEMO_SCENE_URL: &str = "scenes/drawings.json";

/// Async init → event-loop messages. `Ready` carries the State built around the FIRST file
/// (pixels in ~2s); each `File` is one more parsed document, appended live.
pub enum Msg {
    Ready(Box<State>),
    File(String, session_rust::Session, session_rust::Xform),
}
```

**5b. The loop speaks `Msg`.** Three one-line changes:
`proxy: Option<winit::event_loop::EventLoopProxy<State>>` → `…EventLoopProxy<Msg>>`,
`EventLoop::<State>::with_user_event()` → `EventLoop::<Msg>::with_user_event()`,
`impl ApplicationHandler<State> for App` → `impl ApplicationHandler<Msg> for App`.

**5c. The loader.** In `resumed`, find:

```rust
        if let Some(proxy) = self.proxy.take() {
            wasm_bindgen_futures::spawn_local(async move {
                let state = State::new(window).await.expect("State init failed");
                let _ = proxy.send_event(state);
            });
        }
```

Replace with — this is 34e's loop from state.rs, PIPELINED (34h pt3's eager `fetch_start`),
CHUNKED (Step 3's parse) and PROGRESSIVE (`Ready` fires after file one; the loop keeps running
behind the live viewer):

```rust
        if let Some(proxy) = self.proxy.take() {
            wasm_bindgen_futures::spawn_local(async move {
                // Manifest, then the files — PIPELINED (fetch_start is eager: the browser
                // request for file N+1 is in flight while file N parses) and PROGRESSIVE
                // (Ready after the FIRST file; every later file streams in as a Msg::File).
                let t0 = crate::engine::performance::now_ms();
                let manifest_bytes = persistence::fetch_bytes(DEMO_SCENE_URL).await.unwrap_or_default();
                let manifest = Manifest::parse(&manifest_bytes)
                    .unwrap_or_else(|| panic!("cannot read the scene manifest at {DEMO_SCENE_URL}"));
                log::info!("scene '{}': {} items", manifest.name, manifest.items.len());
                let count = manifest.items.len();
                let mut next = manifest.items.first().map(|it| persistence::fetch_start(&it.file));
                let mut sent_ready = false;
                for (i, item) in manifest.items.iter().enumerate() {
                    let f0 = crate::engine::performance::now_ms();
                    let cur = next.take();
                    next = manifest.items.get(i + 1).map(|it| persistence::fetch_start(&it.file));
                    let bytes = match cur {
                        Some(Ok(f)) => persistence::fetch_finish(f).await.unwrap_or_default(),
                        _ => Vec::new(),
                    };
                    let f1 = crate::engine::performance::now_ms();
                    let session = persistence::session_from_bytes_chunked(&item.file, &bytes).await;
                    let name = if item.name.is_empty() { session.name.clone() } else { item.name.clone() };
                    log::info!("loaded '{}': {} objects, {} bytes | fetch {:.0}ms · parse {:.0}ms",
                        name, session.lookup.len(), bytes.len(), f1 - f0,
                        crate::engine::performance::now_ms() - f1);
                    if session.lookup.is_empty() { continue } // failed fetch = skipped file
                    let place = item.placement().unwrap_or_else(|| auto_grid(i, count, [7500.0, 4800.0]));
                    if !sent_ready {
                        sent_ready = true;
                        let mut scene = Scene::new();
                        scene.add_file(name, session, place);
                        let state = State::new(window.clone(), scene).await.expect("State init failed");
                        log::info!("first file on screen {:.0}ms after manifest fetch", crate::engine::performance::now_ms() - t0);
                        let _ = proxy.send_event(Msg::Ready(Box::new(state)));
                    } else {
                        let _ = proxy.send_event(Msg::File(name, session, place));
                    }
                }
            });
        }
```

**5d. Receiving.** Replace the whole `user_event` fn (its signature changes) with:

```rust
    /// `Ready`: adopt the State built around the first file, size it, fit the camera, draw.
    /// `File`: append one more document — walk it into the shared tables, re-upload, redraw.
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, msg: Msg) {
        match msg {
            Msg::Ready(state) => {
                let mut state = *state;
                let (w, h) = desired_canvas_size()
                    .unwrap_or_else(|| { let s = state.window.inner_size(); (s.width, s.height) });
                state.resize(w, h);
                let aspect = w as f64 / h as f64;
                state.camera.fit(state.gpu.scene_min, state.gpu.scene_max, aspect);
                state.window.request_redraw();
                self.state = Some(state);
            }
            Msg::File(name, session, place) => {
                let Some(state) = &mut self.state else { return };
                let t0 = crate::engine::performance::now_ms();
                state.scene.add_file(name, session, place);
                state.gpu.set_scene(&state.scene.tables);
                log::info!("appended: walk+upload {:.0}ms | {} docs",
                    crate::engine::performance::now_ms() - t0, state.scene.docs.len());
                state.window.request_redraw();
            }
        }
    }
```

Note the free win in the `Ready` arm: the camera FITS the first sheet — no more pressing `F` at
a grey void. (Later files widen `scene_min/max`; the camera stays where you are, which is what
you want while reading sheet one.)

## Memory honesty

`Scene` now RETAINS every parsed `Session`. One sheet costs tens to hundreds of MB in kernel
form — all ten at once flirts with the wasm heap ceiling (~1-2 GB practical). That is a known,
accepted cost of having the document back, and the fix is already planned: the P2/P3
single-storage refactor in `.claude/SESSION_DATASTRUCTURE_PLAN.md` shrinks the Rust Session to
what C++/Python already pay. Until it lands, the ten-sheet manifest is a stress test; the
two-sheet manifest is the comfortable working set for the editing lessons. If the tab dies with
the full ten, that is the heap, not your code.

## What is deliberately NOT here

- **A flat GPU-ready sidecar format** (bake `.pb` → raw tables offline, load ≈ memcpy). Biggest
  possible lever, rejected: a second format to keep in sync, and it bypasses the `Session` this
  whole lesson exists to bring back.
- **gzip assets.** `.pb` compresses ~2.5-3×. It does nothing on localhost (fetch is already
  hidden behind parse) but matters on a real network — a deployment concern for `serve_dist`,
  not viewer code.
- **Hatch entities.** On the heaviest sheet 59% of all lines are sub-2 mm hatch strokes; a
  boundary+angle+spacing representation with a stripe shader would collapse them by an order of
  magnitude. Needs a new kernel type across 3 languages + proto — importer-side detection is the
  first step, queued behind the kernel freeze.
- **Smaller Sessions** (the real memory fix): `.claude/SESSION_DATASTRUCTURE_PLAN.md` P2/P3.

## Verify

`trunk serve` → the FIRST sheet appears in ~2-3 s, already fitted, while the console keeps
logging `loaded '…'` / `appended: walk+upload …ms` lines and sheets pop in one by one. **DRAG
while it loads**: the view keeps orbiting through all ten parses — brief dips are fine,
multi-second freezes are not (that is what the sliced parse bought; before it, every arriving
sheet locked the canvas for 1-2.6 s). When the last one lands:

- `colors_widths` shows THREE separated boxes (placements now ride `Session.xforms`).
- A drawing sheet still shows paper-space pen weights (the planar lane moved with the walk).
- The litmus test:

```bash
grep -rn "Session\|Mesh\|BRep" src/engine/ | grep -v "//"
```

Empty — `engine/` compiles against `Xform`, `RenderVertex`, `Point`, and its own
`Instance`/`CylinderSegment`/`GlyphPoint`/`ArenaUpload`, nothing that knows what a document is.
And the document is BACK: `state.scene.docs[0].session` holds every object by guid, ready for
picking, undo, and save.

## Recap

```
Ch 34e: STREAM, WALK, DROP — fast, but the viewer held no document, and nothing drew until the
        LAST file finished (~24 s of white screen on ten sheets).
Ch 35:  THE DOCUMENT RETURNS, AND SO DO THE PIXELS. Scene (app layer) owns docs: Vec<Doc{name,
        place, session}> — the kernel Session KEPT per file — plus the merged ArenaUpload tables
        and viewer-only bookkeeping (guid_to_row, hidden). add_file walks ONE session into the
        shared tables (placement = manifest place × session.world_xforms()), applies the 34f
        paper lane per file, and never rebuilds old rows. Gpu::new starts EMPTY; set_scene is
        the ONE upload path — zero-copy (write_buffer lane splice into zeroed buffers; WebGPU
        zero-init), MSAA flip rebuilds pipelines (sample count belongs to the PASS). The parse
        is SLICED: proto decoded whole, objects converted 25k per setTimeout(0) macrotask
        (microtasks don't paint; Rc keeps Session off workers — slicing IS the fix), built on
        the kernel's from_proto split. lib.rs turns the 34e loop into Msg::Ready (first file,
        ~2-3 s, camera auto-fit) + Msg::File (append live, UI never freezes).
        Litmus: engine/ names no Session/Mesh/BRep in code.
```

Edited: `engine/gpu/mod.rs` (imports trimmed, STRESS_GRID+SceneTables+merge+walk_session+
push_mesh deleted; ArenaUpload + zero-copy set_scene + zeroed_buffer + pub row structs +
FLAG_HIDDEN + stored layouts; new() starts empty), `engine/gpu/adapters.rs` (DELETED),
`app/scene.rs` (+Doc/Scene/add_file + converters), `Cargo.toml` (+prost), `app/persistence.rs`
(chunked parse in, session_from_bytes out), `state.rs` (loop out, `State.scene` in), `lib.rs`
(Msg enum + progressive loader).

## Next

`36-scene-bvh.md` — `Scene` now has a fixed, ordered object list; the next lesson gives it a
broad-phase AABB BVH over their world boxes. One BVH, reused by frustum culling (37), picking
(42), and box-select (45) — the "one acceleration structure, many uses" principle, and the reason
the object list had to stabilize here first.
