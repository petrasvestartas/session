# 35 Scene struct, and the mesh edge lane

> Two parts, one lesson. **Part 1** brings the document back and stops the load path hurting.
> **Part 2** replaces the 3D tube on every mesh edge with two triangles, and works out what it
> takes to make a flat pen occlude correctly. The end state of both is the snapshot crate in
> [`35_scene_struct/`](35_scene_struct/).

> **Big picture.** Since 34e the viewer has NO document. Each `Session` is parsed, walked into
> flat GPU tables, and **thrown away** — deliberately, to survive the stress wall. That was right
> for the wall and is wrong for everything ahead: picking (55) must answer "which OBJECT did the
> ray hit" (a guid), undo (64) must snapshot geometry, save (50) must write a `.pb` back — all of
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
assets/scenes/drawings_rotated.toml  # Step 6: rotated-sheet verify scene (no code)
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
    pub points: Vec<CloudPoint>,       // RAW lane: scanned clouds, one vertex and one pixel each
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

(Empty here, and it stayed empty for a long time — the raw cloud lane existed in the engine but
nothing ever filled it. `set_scene` fills it now; see the note at the end of Step 2.)

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

- Find the row struct `struct Instance {` and add `pub` to the struct keyword only — its fields
  stay private, since only `Gpu` builds rows and `Scene` merely names the flag:

```rust
pub struct Instance {
```

  Then insert the flag const directly below that struct's closing `}`:

```rust
impl Instance {
    /// Row is skipped by the draw (51). Bit 0 is reserved for FLAG_SELECTED (50).
    pub const FLAG_HIDDEN: u32 = 1 << 1;
}
```

- Find `struct CylinderSegment{` and `struct GlyphPoint{` and add `pub` to the struct keyword
  **and to every field** — `Scene` constructs these field-by-field across the module boundary.
  Nothing else changes; keep each field's comment exactly as it is:

```rust
pub struct CylinderSegment{
    pub p0: [f32; 3],
    pub radius: f32,
    pub p1: [f32; 3],
    pub instance_id: u32,
    pub color: [f32; 4],
}

pub struct GlyphPoint{
    pub center: [f32; 3],
    pub radius: f32,
    pub color: [f32; 4],
    pub instance_id: u32,
    pub _pad: [u32; 3],
}
```
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

Nothing in this file changes — it stays the manifest (WHICH files, WHERE they sit). You only
ADD code at the very bottom, after `auto_grid`'s closing `}`. Three pastes:

**2a.** Paste the block below at the end of the file. It reads as three parts: `Doc` (one kept
session + its placement), `Scene` (the docs + the merged GPU tables + `guid_to_row`/`hidden`
bookkeeping), and `add_file()` — 34e's walk moved out of `Gpu`, appending into the SHARED
tables so each file costs only its own walk. The walk also GROWS here: since 34b it rendered 6
of the kernel's 11 geometry types and skipped the rest — now that it lives in the app layer it
covers all 11 (surfaces tessellate like BReps, elements delegate to their baked geometry,
planes/boxes/clouds get the three new converters of 2c).

**2b.** Below that, the converters rescued from Step 1's deletions (full bodies after the
block — nothing to dig out of git).

**2c.** Last, the three NEW converters the full coverage needs — plane, box, point cloud (full
code after 2b).

```rust
// ─────────────────────────────────────────────────────────────────────────────────────────────
// The DOCUMENT side of the scene: manifest above says WHERE, `Scene` below owns WHAT.

use std::collections::{HashMap, HashSet};
use session_rust::{Session, Geometry, Mesh, Line, Point, Polyline, NurbsCurve, Plane, OBB, PointCloud, RenderVertex, Vector};
use session_rust::element::ElementGeometry;
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
    /// - per-FILE planar test: a sheet flat along its OWN normal (place·ẑ, any orientation)
    ///   flips its linework into the world-mm lane (34f)
    /// - unlike the 34 walk (6 of 11 types), EVERY kernel geometry type now renders
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
            // The ONE skip left: an Element with no baked geometry draws nothing. Screening it
            // BEFORE the row push is what lets the push live outside the match — every arm
            // below draws, so every arm shares the same object row.
            if let Geometry::Element(e) = geom {
                if matches!(e.geometry(), ElementGeometry::None) { continue }
            }
            let ri = t.objects.len() as u32;
            let flags = if self.hidden.contains(&guid) { Instance::FLAG_HIDDEN } else { 0 };
            let placed = &place * &placement(&guid);
            t.objects.push((placed, [1.0; 4], flags));
            match geom {
                // 3D geometry takes the SOLID lane: edges are real cylinders and vertices real
                // spheres, so ink is lifted off the surface by its own radius instead of being
                // a flat quad at the surface's depth (which loses the depth test at silhouettes).
                Geometry::Mesh(m) => {
                    push_mesh(m, ri, &mut t.verts, &mut t.vids, &mut t.idx,
                        &mut t.pipes, &mut t.spheres);
                }
                Geometry::BRep(b) => {
                    let mut bm = b.mesh();
                    bm.set_objectcolor(b.surfacecolor.clone());
                    push_mesh(&bm, ri, &mut t.verts, &mut t.vids, &mut t.idx,
                        &mut t.pipes, &mut t.spheres);
                }
                Geometry::Line(l) => t.segments.push(line_to_segment(l, ri)),
                Geometry::Polyline(pl) => t.segments.extend(polyline_to_segments(pl, ri)),
                // Curves ride the FLAT lane too - sampled to segments, they ARE polylines by
                // the time the GPU sees them.
                Geometry::NurbsCurve(c) => t.segments.extend(nurbscurve_to_segments(c, ri)),
                Geometry::Point(p) => t.glyphs.push(point_to_glyph(p, ri)),
                Geometry::PointCloud(pc) => t.glyphs.extend(pointcloud_to_glyphs(pc, ri)),
                // A surface tessellates exactly like a BRep face (mesh() caches; a planar
                // surface is just two triangles), so it rides the SOLID lane.
                Geometry::NurbsSurface(s) => {
                    let mut sm = s.mesh();
                    if let Some(c) = s.facecolors.first() { sm.set_objectcolor(c.clone()); }
                    push_mesh(&sm, ri, &mut t.verts, &mut t.vids, &mut t.idx,
                        &mut t.pipes, &mut t.spheres);
                }
                // Construction types draw as linework: a plane is infinite, so it gets a fixed
                // PLANE_SIZE rectangle spanned by its x/y axes; a box is its 12 edges.
                Geometry::Plane(p) => t.segments.extend(plane_to_segments(p, ri)),
                Geometry::OBB(b) => t.segments.extend(obb_to_segments(b, ri)),
                // An element IS its baked geometry. NO wildcard arm anywhere in this match: a
                // 12th kernel type must fail to compile until the walk decides how to draw it.
                Geometry::Element(e) => match e.geometry() {
                    ElementGeometry::Mesh(m) => {
                        push_mesh(m, ri, &mut t.verts, &mut t.vids, &mut t.idx,
                            &mut t.pipes, &mut t.spheres);
                    }
                    ElementGeometry::BRep(b) => {
                        let mut bm = b.mesh();
                        bm.set_objectcolor(b.surfacecolor.clone());
                        push_mesh(&bm, ri, &mut t.verts, &mut t.vids, &mut t.idx,
                            &mut t.pipes, &mut t.spheres);
                    }
                    ElementGeometry::None => (), // screened before the row push above
                },
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

        // 2D drawing sheets (flat linework - every PDF conversion gets paper space) keep kernel
        // widths (mm on the sheet) via the radius world lane, so zooming out thins the ink like
        // a real print; 3D model files keep screen-constant px linework. Planar = thin along the
        // SHEET's normal (place·ẑ; place is rigid — the manifest never scales), not world z, so
        // a rotated placement stays paper. The 99% path (translation-only place, auto_grid:
        // normal still ±Z) reuses the z-extent accumulated above — no extra work at all; only a
        // rotated placement pays one dot-product pass over this file's new rows.
        let n = place.transform_vector(&Vector::new(0.0, 0.0, 1.0));
        let thickness = if n[0].abs() < 1e-9 && n[1].abs() < 1e-9 {
            fmax[2] - fmin[2]
        } else {
            let (nx, ny, nz) = (n[0] as f32, n[1] as f32, n[2] as f32);
            let (mut dmin, mut dmax) = (f32::INFINITY, f32::NEG_INFINITY);
            let mut span = |p: [f32; 3]| {
                let d = p[0] * nx + p[1] * ny + p[2] * nz;
                dmin = dmin.min(d);
                dmax = dmax.max(d);
            };
            for (i, v) in t.verts.iter().enumerate().skip(vert0) {
                if let Some(&ri) = t.vids.get(i) {
                    if let Some((xf, _, _)) = t.objects.get(ri as usize) {
                        span(xform_point(xf, v.position));
                    }
                }
            }
            for s in t.pipes.iter().skip(pipe0).chain(t.segments.iter().skip(seg0)) {
                if let Some((xf, _, _)) = t.objects.get(s.instance_id as usize) {
                    span(xform_point(xf, s.p0));
                    span(xform_point(xf, s.p1));
                }
            }
            for g in t.spheres.iter().skip(sphere0).chain(t.glyphs.iter().skip(glyph0)) {
                if let Some((xf, _, _)) = t.objects.get(g.instance_id as usize) {
                    span(xform_point(xf, g.center));
                }
            }
            dmax - dmin
        };
        let planar = thickness.is_finite() && thickness.abs() < 1e-3;
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

**2b (the rescued converters).** Below the block you just pasted, add eight functions, bodies
UNCHANGED: `line_to_segment`, `polyline_to_segments`, `nurbscurve_to_segments`,
`point_to_glyph`, `encode_width` from the deleted `adapters.rs` (drop the `pub`/`pub(super)` —
they're file-local now), and `push_mesh`, `xform_point`, `grow_bounds` from the deleted engine
code. They name `Mesh`/`Line`/`Point` — document types — which is exactly why they now live in
the app layer. After Step 1's deletions their only other copy is git history
(`git show a6a33a8b:./src/engine/gpu/adapters.rs` / `…gpu/mod.rs`), so here they are in full —
the ONLY edits vs the originals are the dropped `pub`/`pub(super)` and `encode_width`'s comment
saying `add_file` instead of the deleted `walk_session`:

```rust
fn line_to_segment(l: &Line, instance_id: u32) -> CylinderSegment {
    CylinderSegment {
        p0: l.start().to_f32(),
        radius: encode_width(l.width),
        p1: l.end().to_f32(),
        instance_id,
        color: l.linecolor.to_f32(),
    }
}

fn polyline_to_segments(pl: &Polyline, instance_id: u32) -> Vec<CylinderSegment> {
    let pts = pl.get_points();
    let color = pl.linecolor.to_f32();
    // windows(2) = every OVERLAPPING pair [p_i, p_i+1]: N points -> N-1 connected edges
    // (chunks(2) would skip every other edge; < 2 points yields nothing, no panic)
    pts.windows(2).map(|w| CylinderSegment {
        p0: w[0].to_f32(),
        radius: encode_width(pl.width),
        p1: w[1].to_f32(),
        instance_id,
        color,
    }).collect()
}

/// A curve becomes a polyline of ribbon segments. Sample count follows the curve's SIZE, not a
/// fixed number: a PDF sheet is mostly 1-2 mm glyph outlines (4 segments is already smoother
/// than a pixel) next to metre-long arcs (which need ~50), and a flat count would either
/// shatter the budget or visibly facet the big ones.
fn nurbscurve_to_segments(c: &NurbsCurve, instance_id: u32) -> Vec<CylinderSegment> {
    let (mut lo, mut hi) = ([f64::MAX; 3], [f64::MIN; 3]);
    for i in 0..c.m_cv_count {
        if let Some(cv) = c.cv(i) {
            let w = if c.m_is_rat && cv.len() > 3 && cv[3] != 0.0 { cv[3] } else { 1.0 };
            for k in 0..3 { lo[k] = lo[k].min(cv[k] / w); hi[k] = hi[k].max(cv[k] / w); }
        }
    }
    if lo[0] > hi[0] { return Vec::new(); }
    let size = ((hi[0]-lo[0]).powi(2) + (hi[1]-lo[1]).powi(2) + (hi[2]-lo[2]).powi(2)).sqrt();
    let n = ((size / 0.2).sqrt().ceil() as usize).clamp(4, 64);

    let (t0, t1) = c.domain();
    let color = c.linecolors.first().map(|c| c.to_f32()).unwrap_or([0.0, 0.0, 0.0, 1.0]);
    let radius = encode_width(c.width);
    let pts: Vec<[f32; 3]> = (0..=n)
        .map(|i| c.point_at(t0 + (t1 - t0) * i as f64 / n as f64).to_f32())
        .collect();
    pts.windows(2).map(|w| CylinderSegment {
        p0: w[0],
        radius,
        p1: w[1],
        instance_id,
        color,
    }).collect()
}

fn point_to_glyph(p: &Point, instance_id: u32) -> GlyphPoint {
    GlyphPoint {
        center: p.to_f32(),
        radius: encode_width(p.width),
        color: p.pointcolor.to_f32(),
        instance_id,
        _pad: [0; 3],
    }
}

/// Kernel width - the radius encoding's negative lane (px multiplier); 0.0 = global default.
/// Radius 0.0 and -1.0 render identically (mult = select(1.0, -r, r<0)), so every w > 0 encodes
/// as-is - a special case for 1.0 would silently lose a real 1.0 pen (PDF widths are mm now).
/// add_file flips negatives into the positive world-mm lane for planar 2d drawings:
/// paper-space lineweights that scale with zoom.
fn encode_width(w: f64) -> f32 {
    if w.is_finite() && w > 0.0 {
        -(w as f32)
    } else {
        0.0
    }
}

fn push_mesh(
    m: &Mesh,
    ri: u32,
    verts: &mut Vec<RenderVertex>,
    vids: &mut Vec<u32>,
    idx: &mut Vec<u32>,
    segments: &mut Vec<CylinderSegment>,
    glyphs: &mut Vec<GlyphPoint>
){
    let base = verts.len() as u32;
    let rm = m.to_render();
    for v in &rm.vertices{
        verts.push(*v);
        vids.push(ri);
    }
    for &i in &rm.indices{
        idx.push(base+i);
    }

    // Edge width 0 = HIDDEN wireframe. A mesh only has explicit widths if someone called
    // set_linecolors, so the 1.0 default below leaves every ordinary mesh untouched - but a
    // triangulated PDF fill (a letter, a poché region) asks for no wireframe at all, and without
    // this every glyph would render outlined in tubes and dotted at each vertex.
    // A single width BROADCASTS to every edge - one entry instead of one per edge, which for
    // thousands of small glyph meshes is the difference between a lean .pb and a fat one.
    let width_at = |i: usize| -> f64 {
        let w = m.widths();
        if w.len() == 1 { w[0] } else { w.get(i).copied().unwrap_or(1.0) }
    };
    let hidden = |i: usize| width_at(i) == 0.0;

    // A fill (every PDF glyph, every poché region) broadcasts a single width of 0 - no wireframe
    // at all. Leave BEFORE edges_with_colors, which builds a HashSet over the faces: on a sheet
    // made of hundreds of thousands of tiny fills that set is the walk's biggest single cost,
    // and every edge it produces is skipped one line later anyway.
    if m.widths().len() == 1 && m.widths()[0] == 0.0 { return }

    // ONE edge walk, shared by the pipes below and the vertex widths further down.
    let edges = m.edges_with_colors();

    for (i, (a, b, col)) in edges.iter().cloned().enumerate(){
        if hidden(i) { continue }
        let pa = m.vertex_point(a).unwrap();
        let pb = m.vertex_point(b).unwrap();
        segments.push(
            CylinderSegment{
                p0: pa.to_f32(),
                radius: encode_width(width_at(i)),
                p1: pb.to_f32(),
                instance_id: ri,
                color: col.to_f32()
            }
        )
    }

    // Dots honor user-set pointcolors.
    // The auto-seeded white vec is filtered by the MODE gate.
    // m.vertices() is sorted - the same order to_render indexes pointcolors by.
    let pc = m.get_pointcolors();
    let dots_colored = m.color_mode == ColorMode::POINTCOLORS && pc.len() == m.number_of_vertices();
    // A vertex sphere must be as fat as the pipes that meet there, or the joint shows a pinch
    // (thinner sphere) or a bead (fatter one). The kernel has no per-vertex width, so take the
    // widest incident edge - the same encoding the pipes above are pushed with.
    let mut vwidth: std::collections::HashMap<usize, f64> = std::collections::HashMap::new();
    for (i, (a, b, _)) in edges.iter().cloned().enumerate(){
        if hidden(i) { continue }   // a vertex whose every edge is hidden gets no dot either
        let w = width_at(i);
        for vk in [a, b] {
            let e = vwidth.entry(vk).or_insert(w);
            if w > *e { *e = w; }
        }
    }
    for (i,vk) in m.vertices().into_iter().enumerate(){
        let Some(&vw) = vwidth.get(&vk) else { continue };
        let p = m.vertex_point(vk).unwrap();
        glyphs.push(
            GlyphPoint {
                center: p.to_f32(),
                radius: encode_width(vw),
                color: if dots_colored { pc[i].to_f32() } else { [0.1, 0.1, 0.1, 1.0] },
                instance_id: ri,
                _pad: [0;3] }
        );
    }
}

fn xform_point(xf: &Xform, p: [f32; 3]) -> [f32; 3] {
    let x = p[0] as f64;
    let y = p[1] as f64;
    let z = p[2] as f64;
    [
        (xf.m[0] * x + xf.m[4] * y + xf.m[8] * z + xf.m[12]) as f32,
        (xf.m[1] * x + xf.m[5] * y + xf.m[9] * z + xf.m[13]) as f32,
        (xf.m[2] * x + xf.m[6] * y + xf.m[10] * z + xf.m[14]) as f32,
    ]
}

fn grow_bounds(min: &mut [f32; 3], max: &mut [f32; 3], p: [f32; 3]) {
    for k in 0..3 {
        min[k] = min[k].min(p[k]);
        max[k] = max[k].max(p[k]);
    }
}
```

Two things to notice: `push_mesh`'s last two parameters are NAMED `segments`/`glyphs` but every
call site passes `&mut t.pipes, &mut t.spheres` — mesh edges and vertices ride the SOLID lane;
the parameter names just came along with the body. And `xform_point`'s column indices
(`m[0]/m[4]/m[8]/m[12]` across a row) are the kernel `Xform`'s COLUMN-major layout — the same
convention Step 6's hand-written manifest matrices use.

**2c (the new converters).** The three types no walk ever drew before. Each reuses a lane that
already exists — no shader, no pipeline, no engine change:

```rust
/// A plane is infinite — draw a fixed square around its origin, spanned by its x/y axes.
/// Half-extent in world mm (a 1 m square); the plane's own pen (width, linecolor) draws it.
const PLANE_SIZE: f64 = 500.0;

fn plane_to_segments(pl: &Plane, instance_id: u32) -> Vec<CylinderSegment> {
    let (o, x, y) = (pl.origin(), pl.x_axis(), pl.y_axis());
    let corner = |sx: f64, sy: f64| -> [f32; 3] {
        [0usize, 1, 2].map(|k| (o[k] + (x[k] * sx + y[k] * sy) * PLANE_SIZE) as f32)
    };
    let c = [corner(1.0, 1.0), corner(-1.0, 1.0), corner(-1.0, -1.0), corner(1.0, -1.0)];
    let color = pl.linecolor.to_f32();
    let radius = encode_width(pl.width);
    (0..4).map(|i| CylinderSegment { p0: c[i], radius, p1: c[(i + 1) % 4], instance_id, color })
        .collect()
}

/// A box is its 12 edges: bottom loop, top loop, four verticals — `corners()` orders the bottom
/// face 0-3 and the top 4-7 with i / i+4 vertically aligned. The OBB type carries no pen, so
/// the edges draw black at screen-constant width (radius 0.0 = the global default).
fn obb_to_segments(b: &OBB, instance_id: u32) -> Vec<CylinderSegment> {
    const EDGES: [[usize; 2]; 12] = [[0, 1], [1, 2], [2, 3], [3, 0],
        [4, 5], [5, 6], [6, 7], [7, 4], [0, 4], [1, 5], [2, 6], [3, 7]];
    let c = b.corners_f32();
    EDGES.iter().map(|&[i, j]| CylinderSegment {
        p0: c[i], radius: 0.0, p1: c[j], instance_id, color: [0.0, 0.0, 0.0, 1.0],
    }).collect()
}

/// One glyph per point. `point_size` rides the same width encoding as every other pen, and a
/// cloud with fewer colors than points falls back to black for the tail.
fn pointcloud_to_glyphs(pc: &PointCloud, instance_id: u32) -> Vec<GlyphPoint> {
    let radius = encode_width(pc.point_size);
    let colors = pc.color_count();
    (0..pc.len()).map(|i| GlyphPoint {
        center: pc.get_point(i).to_f32(),
        radius,
        color: if i < colors { pc.get_color(i).to_f32() } else { [0.0, 0.0, 0.0, 1.0] },
        instance_id,
        _pad: [0; 3],
    }).collect()
}
```

Five things to notice while typing:

- The import block adds no `Xform` — the manifest code already at the top of this file imports
  it (`use session_rust::Xform;`), so `Doc { place: Xform }` resolves.
- `add_file` walks straight into the GLOBAL tables, so `push_mesh`'s `base = verts.len()`
  index-rebasing works unchanged — there is no separate per-file merge step anymore, and
  appending file #7 never touches rows 1-6.
- The planar test is per FILE (its own new rows only) and orientation-independent: thickness is
  measured along the sheet's own normal (`place·ẑ`), so a rotated sheet keeps paper-space
  lineweights next to a 3D model — and the ±Z fast path means the common flat layout pays
  nothing for that generality.
- `obj0` is captured alongside the other bases but nothing reads it yet — `let _ = obj0;` at the
  bottom keeps the unused-variable warning away until a later lesson needs the per-file row base.
- The five new arms cost three small converters and zero engine changes because every lane they
  need already exists: surfaces and elements reuse the BRep pattern (`mesh()` → `push_mesh`),
  planes and boxes are plain segments, and a cloud goes through the GLYPH lane — deliberately
  not `Gpu`'s dormant `CloudPoint` machinery, so the walk keeps one path per category.
  That last choice is right for the demo clouds of 32b and wrong for a real scan — which is
  exactly what [36](36-cloud-tables.md) is about. It stays as written here: a lesson that
  quietly patches itself teaches nothing about why the first answer looked reasonable.

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
    // The same conversion loop for all 11 types, written once: proto -> object, stored, paused
    // every CHUNK so the browser can paint.
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

The load loop LEAVES this file (it becomes the progressive loader in lib.rs), so the top half of
`state.rs` is replaced and the bottom half is untouched. Three moves:

1. **KEEP** the four `//!` header lines at the very top.
2. **DELETE** everything from `use std::sync::Arc;` down to and including the `}` that closes
   `new` — that is the line directly above `/// Forward a canvas resize`. You are deleting: the
   whole `use` block, the `DEMO_SCENE_URL` const, `pub struct State`, the `impl State {` line,
   and the entire body of `new` (manifest fetch, pipelined loop, `Gpu::new`, `Ok(Self …)`).
3. **PASTE** the block below where they were. It re-opens `impl State {` itself and ends at
   `new`'s closing `}` — deliberately unbalanced, because `resize` and `render` are still sitting
   below untouched, and the file's existing last `}` still closes the impl.

The file afterwards, top to bottom: `//!` header → the pasted block (`use`s, `struct State`,
`impl State {`, `new`) → `resize` → `render` → `}`.

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
const DEMO_SCENE_URL: &str = "scenes/drawings.toml";

/// Async init → event-loop messages. `Ready` carries the State built around the FIRST file
/// (pixels in ~2s); each `File` is one more parsed document, appended live.
pub enum Msg {
    Ready(Box<State>),
    File(String, session_rust::Session, session_rust::Xform),
}
```

**5b. The loop speaks `Msg`.** The event loop was parameterised on `State` — the type it carries
from the async init back to the handler. Now it carries `Msg` instead, which takes three
separate one-line edits further down `lib.rs`. `State` stays imported and `App` still holds one
in `state: Option<State>`; only the message type changes.

First, inside `pub struct App {`, find:

```rust
    proxy: Option<winit::event_loop::EventLoopProxy<State>>,
```

and change `State` to `Msg`:

```rust
    proxy: Option<winit::event_loop::EventLoopProxy<Msg>>,
```

Second, inside `impl App`'s `run()`, find:

```rust
        let event_loop = EventLoop::<State>::with_user_event().build()?;
```

and change `State` to `Msg`:

```rust
        let event_loop = EventLoop::<Msg>::with_user_event().build()?;
```

Third, the `impl` header just below `run()`'s closing braces, find:

```rust
impl ApplicationHandler<State> for App {
```

and change `State` to `Msg`:

```rust
impl ApplicationHandler<Msg> for App {
```

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

**5d. Receiving.** Below `resumed`, find the whole `user_event` fn — doc comment through closing
brace, seven lines:

```rust
    /// Receive the initialized `State`, size it to the canvas, and start drawing.
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut state: State) {
        let (w, h) = desired_canvas_size()
            .unwrap_or_else(|| { let s = state.window.inner_size(); (s.width, s.height) });
        state.resize(w, h);
        state.window.request_redraw();
        self.state = Some(state);
    }
```

Delete all of it and paste in its place (the third parameter is a `Msg` now, so the body becomes
a match — the old body survives as the `Ready` arm plus the camera fit):

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

## Step 6 — sheets in space: verify the rotated planar lane

The planar test claims orientation doesn't matter — prove it with your eyes. Nothing compiles
here: one new manifest, one const flip, reload.

**6a.** Create `assets/scenes/drawings_rotated.toml`. The kernel `Xform` is COLUMN-major
(`m[0..3]` = X column, `m[4..7]` = Y, `m[8..11]` = Z, `m[12..14]` = translation), so each
`xform` below reads as four columns of four:

```json
{
  "name": "rotated sheets",
  "comment": "Planar-lane torture test: flat / standing / tilted / in-plane-rotated sheets + one 3D control. Xform is column-major, translation in slots 12-14.",

  "items": [
    { "file": "pb/30700_querschnitt_gg.pb", "name": "flat reference",
      "at": [0, 0, 0] },

    { "file": "pb/draw_pc_gru_og2.pb",      "name": "standing (90 deg about X)",
      "xform": [1,0,0,0,  0,0,1,0,  0,-1,0,0,  3400,0,0,1] },

    { "file": "pb/draw_pb_haus25.pb",       "name": "drafting tilt (45 deg about X)",
      "xform": [1,0,0,0,  0,0.7071068,0.7071068,0,  0,-0.7071068,0.7071068,0,  7200,0,0,1] },

    { "file": "pb/draw_pe_schalungsbild.pb","name": "in-plane spin (30 deg about Z)",
      "xform": [0.8660254,0.5,0,0,  -0.5,0.8660254,0,0,  0,0,1,0,  10000,0,0,1] },

    { "file": "pb/colors_widths.pb",        "name": "3D control (must stay px pens)",
      "at": [0, 4200, 0] }
  ]
}
```

**6b.** In `lib.rs`, flip `DEMO_SCENE_URL` to `"scenes/drawings_rotated.toml"`, reload. Flip it
back when done — the manifest stays in `assets/scenes/` as a permanent regression scene.

**What to verify, sheet by sheet:**

- **Rendering is orientation-independent by construction** — nothing should look different on
  the rotated sheets. FLAT-lane ribbons are built camera-facing per frame from world-space
  endpoints, glyphs are camera-facing SDF dots, and the SOLID lane is real 3D cylinders — none
  of them care what plane the endpoints lie in.
- **Pens are the actual test.** Zoom out: ink must thin like a real print on the flat, the
  standing, the tilted AND the spun sheet alike. The standing and tilted sheets are exactly the
  cases the old world-z test misclassified (their world z-extent is their full height — they'd
  have silently switched to screen-px pens); the normal-aligned test keeps them paper.
- **The fast path isn't fooled.** The 30°-about-Z sheet is rotated but its normal is still +Z —
  it must classify planar through the free `fmax[2]−fmin[2]` path, no dot-product pass. (Log a
  temporary `log::info!("rotated: {}", rotated)` if you want to see the branch taken: false for
  it, true only for the standing/tilted two.)
- **The 3D control stays px.** `colors_widths` has solid boxes → not planar → its linework stays
  screen-constant, and its arrival flips MSAA 1x→4x exactly as before. The four drawing sheets
  alone run at 1x — edge quality on their linework comes from 34f's SDF ramp, which is
  per-pixel and angle-independent, so rotated edges look no different from flat ones.
- **Bounds and fit still hold**: the camera fits the flat reference first, later sheets widen
  `scene_min/max` (the bounds loop runs every point through its full placement, rotation
  included), and `F` frames the whole arrangement.

## What loading actually costs (measured)

The parse yields, the walk does not — `add_file` runs to completion inside `user_event`, a
synchronous winit callback, so each appended file is one uninterrupted block. Native release
timings for the 69 MB / 129k-object sheet, which is the honest budget this lesson leaves behind:

| stage | native release | native debug |
|---|---|---|
| prost decode | 357 ms | — |
| from_proto conversion (sliced) | ~530 ms | — |
| mesh walk (`to_render` + edges) | 226 ms | 2007 ms |
| row bookkeeping (lookup + 2 guid clones each) | 92 ms | 546 ms |
| `world_xforms()` | 84 ms | 275 ms |

Two things follow. First, `trunk serve` builds UNOPTIMIZED wasm — 5-9× slower than release on
exactly this code — so judge load times with `trunk serve --release` and use the dev build only
for iterating. Second, making the viewer stay interactive DURING a load needs the walk to become
resumable (a job with a cursor, stepped a few ms per frame from `RedrawRequested`) and the upload
to become append-only, which is lesson 48a's arena. Both are out of scope here: this lesson's
contract is a first sheet on screen in seconds and whole documents appearing one at a time.

## Memory honesty

> **Followed up in [37](37-cloud-memory.md), `38` and
> `39`.** The paragraph below was written before anyone
> measured it. The measurement, when it came, was worse than this guess: three LiDAR
> scans put the tab at 3.5 GB and the Linux OOM killer took it. The retained `Session`
> is only part of it — the upload path was mirroring each whole table into the wasm
> heap as well. Read this, then read 37.

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
        paper lane per file, never rebuilds old rows — and now renders ALL 11 kernel types
        (planes as PLANE_SIZE squares, boxes as 12 edges, clouds by COUNT — glyph dots
        below CLOUD_RAW_MIN, the raw one-vertex-one-pixel lane above it, surfaces and
        elements through mesh() like BReps). Gpu::new starts EMPTY; set_scene is
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
`app/scene.rs` (+Doc/Scene/add_file, all 11 types + converters), `Cargo.toml` (+prost),
`app/persistence.rs`
(chunked parse in, session_from_bytes out), `state.rs` (loop out, `State.scene` in), `lib.rs`
(Msg enum + progressive loader).

---

# Part 2 — the mesh edge lane

> Everything above gave the viewer a document and a load path that does not hurt. What you will
> notice next is what it spends its triangles on: mesh edges are 3D tubes, twelve triangles an
> edge, ninety times the geometry they decorate. The rest of this lesson replaces them with two
> triangles — and spends most of its length on why a flat rectangle of nonzero width is so much
> harder to occlude correctly than a tube, because that argument IS the lesson.

## Part 2 goal

Draw a mesh's edges as **camera-facing rectangles** — two triangles an edge instead of a
tessellated tube — at the correct pen width, with occlusion that holds at every zoom, every
angle and every pen width. And, just as important, understand *why* the obvious fixes fail, so
the next person does not spend a day rediscovering it.

> **Big picture.** 31 gave edges a 3D tube: twelve triangles per edge, and the tube's radius
> lifts the ink off the surface it decorates, so it never loses the depth test. It looks right
> and it costs 90× the geometry it decorates. A flat rectangle is 2 triangles — but a flat
> rectangle *lies in the plane through the edge*, and at a convex edge that plane cuts into the
> wedge the two faces form. Half the pen ends up inside the solid. Everything in this lesson
> follows from that single sentence.

<svg viewBox="0 0 680 210" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a tube bulges toward the eye by its radius so it is never buried; a flat quad lies in the plane through the edge and half its width sits inside the wedge the two faces form" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="170" y="16" fill="#888" text-anchor="middle">tube — proud by r in every direction</text>
  <path d="M60 150 L280 90" stroke="#3a3a3a" stroke-width="1"/>
  <path d="M60 150 L280 190" stroke="#3a3a3a" stroke-width="1"/>
  <circle cx="60" cy="150" r="16" fill="none" stroke="#6fb3ff" stroke-width="2"/>
  <text x="98" y="150" fill="#6fb3ff">r</text>
  <text x="170" y="120" fill="#666" text-anchor="middle">face A</text>
  <text x="170" y="182" fill="#666" text-anchor="middle">face B</text>
  <text x="170" y="205" fill="#5c5" text-anchor="middle">nothing can bury it</text>

  <text x="510" y="16" fill="#888" text-anchor="middle">flat quad — a PLANE through the edge</text>
  <path d="M400 150 L620 90" stroke="#3a3a3a" stroke-width="1"/>
  <path d="M400 150 L620 190" stroke="#3a3a3a" stroke-width="1"/>
  <line x1="384" y1="150" x2="416" y2="150" stroke="#c66" stroke-width="3"/>
  <text x="510" y="120" fill="#666" text-anchor="middle">face A</text>
  <text x="510" y="182" fill="#666" text-anchor="middle">face B</text>
  <text x="510" y="205" fill="#c66" text-anchor="middle">both halves cut INTO the solid</text>
</svg>

## Files we touch

| file | change |
|---|---|
| `src/engine/gpu/mod.rs` | `LineStyle`, `CylinderSegment` repacked 48 → 40 B, `Instance.extent`/`.spacing`, `eye_from_view_proj` |
| `src/app/scene.rs` | `pack_rgba`, `oct16`, `pack_facing`, `mesh_spacing`, adjacency in `push_mesh` |
| `src/shaders/ribbon.wgsl` | the whole flat lane: width, near-plane clip, facing cull, plane hug, density LOD |
| `src/shaders/sphere.wgsl` | vertex markers: the same hug, drawn last, their own cull and LOD |
| `src/shaders/cylinder.wgsl` | the tube lane reads the same repacked table |
| `src/selftest.rs`, `examples/selftest.rs` | `VIEWER_ORBIT`, `VIEWER_ZOOM`, manifest loading, table footprint |

---

## Step 1 — two lanes over one table

Both lanes read the **same** `CylinderSegment` rows, so switching costs one branch at the draw
site and nothing in memory.

**Find** in `src/engine/gpu/mod.rs`, near the other GPU enums:

```rust
pub struct Gpu {
```

**Add above it:**

```rust
/// How the SOLID lane draws mesh/BRep edges. Both read the SAME `CylinderSegment` table, so
/// switching costs one branch at the draw site and nothing in memory.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LineStyle {
    /// A real 3D tube per edge: 12 triangles, and the radius lifts the ink off the surface.
    Tubes,
    /// A camera-facing quad per edge: 6 vertices, the flat lane's own shader.
    Flat,
}
```

At the draw site in `encode_frame`, the two lanes differ by one `match`:

```rust
match self.line_style {
    LineStyle::Tubes => {
        pass.set_pipeline(&self.pipelines.cylinder);
        pass.draw_indexed(0..self.cyl_index_count, 0, 0..self.pipe_count);
    }
    LineStyle::Flat => {
        pass.set_pipeline(&self.pipelines.ribbon_solid);
        pass.draw(0..6 * self.pipe_count, 0..1);   // vid/6 picks the row
    }
}
```

Bind **L** in `lib.rs` to flip it. Keep the tube lane forever: it is *real geometry*, so it is
the only ground truth you can hold a screen-space construction against. Every width bug below
was caught by measuring the flat lane against it.

## Step 2 — the row: 48 → 40 bytes, and where the adjacency lives

**Replace** `CylinderSegment` in `src/engine/gpu/mod.rs` with:

```rust
pub struct CylinderSegment{
    // FLAT f32s, not `[f32; 3]`: WGSL aligns `vec3<f32>` to 16, so a struct containing one is
    // padded to a multiple of 16 - this table was 48 B and could not have been 40 whatever else
    // was packed. Scalars align to 4, so the stride is the honest sum of the fields.
    pub p0: [f32; 3],   // 12 B
    pub radius: f32,    // 4 B - 0.0 = screen-constant px; > 0 = world-mm override
    pub p1: [f32; 3],   // 12 B
    pub instance_id: u32,  // 4 B
    pub color: u32,     // 4 B - RGBA8, low byte red. Was `[f32; 4]` carrying 8-bit colour.
    pub facing: u32,    // 4 B - two oct16 adjacent face normals
}                       // 40 B
```

Packing the colour paid for `facing` **and** took 8 B off every row: on the bunny that is
104,288 edges, 4.0 MB where 48 B would have been 4.8.

The `facing` word is two octahedral normals, 16 bits each — about 1.4°, when all that is ever
asked of them is the **sign** of a dot product. In `src/app/scene.rs`:

```rust
fn oct16(n: &Vector) -> Option<u32> {
    let l = n[0].abs() + n[1].abs() + n[2].abs();
    if !(l > 0.0) { return None; }
    let (mut x, mut y) = (n[0] / l, n[1] / l);
    if n[2] < 0.0 {
        // signNotZero, NOT signum. `f64::signum(0.0)` is 0.0, which folds (0,0,-1) onto (0,0) -
        // the code for (0,0,+1) - so the two poles collide. On an axis-aligned box that is the
        // top and bottom faces, i.e. most of its edges.
        let s = |v: f64| if v < 0.0 { -1.0 } else { 1.0 };
        let (ax, ay) = (x.abs(), y.abs());
        (x, y) = ((1.0 - ay) * s(x), (1.0 - ax) * s(y));
    }
    let q = |v: f64| (((v.clamp(-1.0, 1.0) * 127.0).round() as i32) as u32) & 0xff;
    Some(q(x) | q(y) << 8)
}

/// "No adjacency, always draw". It CANNOT be 0: (0,0) is the honest encoding of +Z.
pub const FACING_UNKNOWN: u32 = u32::MAX;
```

> **The bug that hid for a day.** Both of those comments are scar tissue. `signum(0.0) == 0.0`
> made ±Z encode identically, and that collision landed on an all-zeros sentinel — so the facing
> test was silently inert for most of a box's edges, and an experiment that depended on it
> "proved" nothing. If you take one habit from this lesson: **a sentinel must be a value the
> encoder can never produce.**

In `push_mesh`, fill it from the halfedge the kernel already has:

**Find:**

```rust
    let edges = m.edges_with_colors();
```

**Add below it:**

```rust
    // Face normals once for the whole mesh, so the per-edge lookup is two map reads.
    let fnormals = m.face_normals();
```

and inside the edge loop, before the `segments.push`:

```rust
        let f = m.edge_faces(a, b).unwrap_or_default();
        let facing = pack_facing(
            f.first().and_then(|&k| fnormals.get(&k).cloned()),
            f.get(1).and_then(|&k| fnormals.get(&k).cloned()),
        );
```

A naked edge (the bunny has 223 of them) gets its single normal duplicated — a boundary edge is
visible whenever its one face is, which needs no special case in the shader.

## Step 3 — the width was twice the pen, twice over

**NDC spans [-1, 1] across `vp_h` pixels, so one NDC unit is `vp_h/2` px, not `vp_h`:**

```
y_ndc = (y_eye / d) * cot(fovy/2)        px = y_ndc * vp_h/2 = y*cot*vp_h / (2*d)
```

The lane divided by `vp_h`. And separately used `thickness` — documented as an on-screen
**width** — as a **half**-width. Same factor twice.

**Replace** the width helper in `src/shaders/ribbon.wgsl`:

```wgsl
fn half_width_px(radius: f32, w: f32) -> f32 {
    if (radius > 0.0){
        if (line.ortho_h > 0.0){
            return radius * line.vp_h * 0.5 / line.ortho_h;
        }
        return radius * line.proj_y * line.vp_h * 0.5 / w;
    }
    return line.thickness * 0.5 * select(1.0, -radius, radius < 0.0);
}
```

How to know you got it right: **measure against the tube lane**, which is real geometry of
radius `r` and cannot be argued with. Before this a mesh edge measured 8 px flat against 4 px as
a tube; after, 4 against 4. This is also why the depth artifacts below were so violent — the
wedge is proportional to band *width*, so a pen at twice its size fights twice as hard.

## Step 4 — the quad is a TRAPEZOID, so the width cannot be a varying

Under perspective the two ends project to different widths, so half-width is a function of the
along-coordinate — which over a trapezoid is **projective, not affine**. Hand the rasterizer a
per-vertex `hw` and each of the quad's two triangles builds its own affine approximation; they
agree only on the diagonal they share, and the seam shows as a **triangular bite** out of the
band along that diagonal.

**Change** the varyings:

```wgsl
    // Half-width in px at each END, both FLAT. Never interpolated.
    @location(4) @interpolate(flat) hw0: f32,
    @location(5) @interpolate(flat) hw1: f32,
```

and resolve per fragment, at the SDF's own along-parameter `h`:

```wgsl
 fn resolve_width(in: VsOut, h: f32) -> vec2<f32> {
    let raw = mix(in.hw0, in.hw1, h);
    return vec2<f32>(floor_hairline(raw), hairline_fade(raw));
 }
```

Exact, and independent of how the quad happens to be triangulated. The centreline depth gets the
same treatment (`zend`).

## Step 5 — clip against the near plane before dividing by w

This lane projects by hand, and a hand divide is only valid **in front of the eye**. The old
`c.xy / max(abs(c.w), 1e-6)` does not clip a vertex behind the eye — it **mirrors** it through
the screen centre, and the quad splays off across the model.

**Add** before the screen-space mapping:

```wgsl
    // In CLIP space `z - w` is linear along the segment and the near plane is exactly z - w = 0
    // (reverse-Z depth z/w = 1), visible side <= 0. Closed form, no uniform, and it needs to know
    // neither the near distance nor the scene scale.
    let f0 = c0.z - c0.w;
    let f1 = c1.z - c1.w;
    if (f0 > 0.0 && f1 > 0.0){ return dead_vertex(); }
    let e0 = select(c0, mix(c0, c1, f0 / (f0 - f1)), f0 > 0.0);
    let e1 = select(c1, mix(c1, c0, f1 / (f1 - f0)), f1 > 0.0);
```

The tube lane never had this bug because the hardware clips real geometry for you.

## Step 6 — hidden edges never reach the rasterizer

An edge belongs to two faces. If **both** turn away, it is inside the solid.

```wgsl
fn edge_faces_camera(facing: u32, n0: vec3<f32>, n1: vec3<f32>, to_eye: vec3<f32>) -> bool {
    if (facing == FACING_UNKNOWN){ return true; }
    return dot(n0, to_eye) > 0.0 || dot(n1, to_eye) > 0.0;
}
```

The eye it needs is recovered from the view-projection alone — **the eye is the one point that
projects to nothing**, where clip x, y and w all vanish. Three rows, one 3×3 solve
(`eye_from_view_proj`), so it works for any caller including the headless harness. It must be
the real eye, not a constant forward direction: at 60° FOV a constant is off by 30° at the frame
corner, and near-silhouette edges are exactly the ones that would flip.

> **`FLAG_INSIDE`.** From inside a solid *every* face points away, so this cull would delete the
> whole object the moment the camera crosses a face. A per-edge test cannot tell "far side of the
> solid" from "eye inside it" — that difference is global, so it rides the instance row as a
> per-frame CPU flag from the object's world AABB.

## Step 7 — the depth trade, and why no offset can win

Here is the load-bearing paragraph of the whole lesson.

A band's depth is its **centreline's** — one value across the whole width. The depth test runs
**per fragment**. At screen distance `d` from the centreline the adjacent face has already risen
toward the eye by `d · tan θ`, θ being the angle between that face's normal and the view ray. So
the offset the ink needs is proportional to the **pen width**, and it is **unbounded** as the
face turns edge-on.

And the trade is symmetric: any offset large enough to clear a pen makes the two faces meeting at
an edge fight each other over a band of the same width. One artifact converts into the other.

| attempt | what happens |
|---|---|
| constant ink lift toward the camera | clears mild cases; a grazing face needs an unbounded value |
| relative face push (`clip.xy*K, clip.z, clip.w*K`) | same limitation — it is a constant |
| hardware `DepthBiasState` slope_scale | works, but `constant`'s units on a float depth format are implementation-defined |
| `dpdx/dpdy` slope bias in the face shader | at the strength that kills the wedge, 16 px slivers along every shared edge |
| per-edge secant lift `r·tan θ` | correct law, still a race against the same unbounded quantity |

**If a constant is being tuned, the model is wrong.** That is the tell.

## Step 8 — the fix: the ink HUGS the surface it decorates

Stop choosing a distance. The adjacent faces are **planes**, their normals are already in the
table, and a plane's depth at a pixel is closed form. Write, per fragment, the depth of whichever
front-facing adjacent plane is nearer here, one epsilon in front.

Build the planes in **clip space**, as the homogeneous join of three transformed points:

```wgsl
// The plane three clip-space points span, as four signed 3x3 minors (each a dot with a cross).
// No matrix inverse and no normalize - the fragment's solve divides the overall scale back out.
fn join3(a: vec4<f32>, b: vec4<f32>, c: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(
        dot(a.yzw, cross(b.yzw, c.yzw)),
        -dot(a.xzw, cross(b.xzw, c.xzw)),
        dot(a.xyw, cross(b.xyw, c.xyw)),
        -dot(a.xyz, cross(b.xyz, c.xyz)),
    );
}
```

The three points are the two endpoints (both lie on both faces) plus one stepped
`cross(n, edir) * elen` off the midpoint. Near-plane clipping is irrelevant to them: a clip-space
point with `w < 0` is still algebraically on the plane.

Then the fragment solves it:

```wgsl
//     pl.x*nx + pl.y*ny + pl.z*nz + pl.w = 0   =>   nz = -(pl.x*nx + pl.y*ny + pl.w) / pl.z
```

Three rules make it work:

1. **`max()` against the centreline, never `min`.** A plane must not pull ink *behind* the
   centreline, or a silhouette edge loses its outer half.
2. **Back-facing planes are skipped.** Past the silhouette they continue through space that is
   not the object, and would drag the ink forward over things it should not cover.
3. **The epsilon is derived, not tuned** — `HUG_ABS + HUG_PIX·slope + HUG_REL·rise`: float
   disagreement between the plane solve and the face rasterizer; the plane's own ndc-z change per
   pixel (under MSAA the ink solves once at the pixel centre while the face holds a depth *per
   sample* — `glPolygonOffset`'s slope term, in closed form); and a fraction of the local rise,
   which covers the oct16 normals' 1.4° quantization.

**The way to see what this is doing:** it makes the flat band compute, per fragment, the depth
the *tube* would have had. The tube gets that from geometry; the ribbon gets it from algebra.

## Step 9 — vertex markers are the topmost ink

Markers ride the same hug. Two rules beyond it:

- **Draw them LAST of the solid lane, and compare `GreaterEqual`.** Drawn first they must win
  *strictly*, because the band testing `GreaterEqual` against them takes any tie. Drawn last they
  only have to match — a strictly weaker condition.
- **Bound the band's own epsilon.** The band references its centreline depth *at the fragment*,
  and a fragment on the disc is up to one marker radius along the band from the vertex, where
  that centreline has moved by the plane's screen slope times the distance:

```wgsl
            let band_span = slope_px * (in.px + 0.5);
            let eps = HUG_ABS + HUG_PIX * slope_px
                + HUG_REL * (abs(zp - z_band) + band_span) + SPHERE_TIE;
```

A trihedral corner needs **three** face pairs, not one — a marker hugging only the widest
incident edge's two faces still loses a sector of its disc to the third face's band. `GlyphPoint`
carries `facing` + `facing_ext[2]`, up to six incident normals.

## Step 10 — density: the part that is not a depth problem at all

Zoom out and a dense mesh goes *see-through*. It is tempting to read that as another depth bug.
It is not, and no depth fix touches it: 104,288 edges and 35,947 markers at **screen-constant**
width over a bunny 100 px tall is ink on every pixel several times over, and a thin feature's
front and back land within a pixel where 4× MSAA resolves both. That is why you can see the
inside of an ear through its near side.

A 2 px pen does not shrink with the model. So stop drawing wires once they fall below the
density the screen can carry.

The threshold is the part worth getting right, and the obvious version is wrong twice over.

First, measuring against an absolute pixel count asks "can I see one edge" — but what makes a
wireframe readable is **room between the wires**. A 2 px pen on edges 4 px apart still covers the
whole surface. Measure in **pen widths** instead and it is scale-free: a fat pen needs more room
than a hairline before it reads as a line rather than as fill.

Second — and this is the part to take seriously — **do not cull.** Dropping edges makes them pop
out of existence as you zoom, which is worse than the problem it solves, and this viewer's rule is
that geometry is never hidden. **Thin** them instead: the ink shrinks, the surface reads through,
and below 1 px the hairline rule already in this lane carries the remainder into alpha. Nothing
disappears, and there is no visible threshold to notice.

**Add** to `src/shaders/ribbon.wgsl`:

```wgsl
const WIRE_MIN_PENS = 3.0;
const TAPER_MIN = 0.15;   // a wire never thins past this fraction of its pen

 fn density_taper(facing: u32, len_px: f32, px: f32) -> f32 {
    if (facing == FACING_UNKNOWN){
        return 1.0;   // free-standing linework is never thinned - the user drew it deliberately
    }
    let room = WIRE_MIN_PENS * 2.0 * max(px, 1e-6);
    return clamp(len_px / room, TAPER_MIN, 1.0);
 }
```

applied to the RAW widths, so the hairline floor and its alpha fade still run afterwards:

```wgsl
    let crowd = density_taper(seg.facing, len, px);
    let raw0t = raw0 * crowd;
    let raw1t = raw1 * crowd;
```

A marker cannot measure its own length, so it uses the object's vertex **spacing** —
`extent / sqrt(vertices)`, computed in `mesh_spacing` and shipped on the instance row — against
its own diameter, and scales `px` by the same clamp.

> **Do it in ALL THREE lanes.** Ribbons, markers *and* tubes. Fixing only the flat lane leaves the
> tube lane — which is the DEFAULT — painting a dense mesh solid black, and it looks exactly like a
> depth bug when it is nothing of the kind. `cylinder.wgsl` projects its own two endpoints and
> scales its radius; a tube is opaque so it cannot fade, but a thin tube still marks the edge.

Free-standing linework is exempt on both counts (`facing == FACING_UNKNOWN`, `spacing == 0`): a
short polyline segment is a real line the user drew, and a drawing is full of them.

**And one more unbounded quantity while you are here.** The lift is a fraction of *eye depth*, so
world lift = lift × eye depth grows with camera distance while an object's size does not. On a
1000 mm box with a 2 px pen it exceeds the box at **242 m** for a band and **91 m** for a
marker — ordinary zoom-out. `lift_capped` clamps it to a tenth of the object's world AABB
diagonal, which the CPU already computes for `FLAG_INSIDE`.

## Step 11 — the harness, and the acceptance test that ends the argument

None of the above can be judged by eye. Give `selftest` three knobs — `VIEWER_ORBIT`,
`VIEWER_ZOOM`, and a `.json` argument it resolves as a **scene manifest the way the browser
does** — and one comparison:

> Render with the ink's depth test on, and again with `VIEWER_NO_DEPTH=1` forcing `Always`. On
> genuinely visible edges the two must match. They may differ only where an edge is truly
> occluded.

That number is the bug. It went **1804 → 12** differing px of 675,000 at zoom 19.

```bash
cargo build --release --example selftest --target x86_64-unknown-linux-gnu
VIEWER_W=900 VIEWER_H=750 VIEWER_ZOOM=19 VIEWER_ORBIT="10,-8" \
  ./target/x86_64-unknown-linux-gnu/release/examples/selftest out.ppm assets/scenes/bunny.toml
```

Two more measurements worth keeping in the harness, both one line of output:

- **table footprint** before upload — bunny: 1.4 MB verts + 0.8 indices + 4.0 edges + 1.6 markers
  = **7.7 MB**
- **staged RSS** — 17.2 MB file → +74.1 MB decode+build → +17.1 MB walk = **108.5 MB**. The
  connectivity, not the render data, is the whole cost: ~3.4 KB per vertex, which is the
  `HashMap<usize, HashMap<usize, Option<usize>>>` halfedge with one allocation per vertex.

## Faster loading

`trunk build` is a **debug** wasm build. Release is 7.1 MB against 10.8, and `session_viewer`'s
own walk — `push_mesh` over 104,288 edges — stops running unoptimized. (`[profile.dev.package."*"]
opt-level = 3` already optimizes *dependencies*, so the kernel parse was never the slow part.)

```bash
trunk serve --release     # not just `trunk serve`
```

## Verify

- **L** toggles tubes ↔ flat. On a free-standing polyline the two lanes are pixel-identical; on a
  box edge they agree to a pixel. If flat is twice as wide, Step 3 is missing.
- Zoom into a box corner: bands solid, no wedge of surface inside them, the vertex marker a clean
  disc.
- Zoom out on the bunny: a smooth shaded surface, not speckle you can see through.
- A 2D drawing sheet is untouched — 52,244 ink px before and after every step here.

## Reference — checking what you typed

The end state of this lesson is the snapshot crate **`35_scene_struct/`** next to this file — a
standalone copy of `session_viewer` as it stands at the end of Part 2, so you can diff a file you
typed against the finished one:

```bash
diff -u docs/35_scene_struct/src/shaders/ribbon.wgsl src/shaders/ribbon.wgsl
```

Every step of Part 2 also landed as its own commit on `origin/pointcloud-memory`, which is more
useful while you are typing — it shows the *change*, not the finished file:

```bash
git log --oneline main..origin/pointcloud-memory   # the whole arc, newest first
git show cd4af476 -- session_viewer/src/shaders/ribbon.wgsl   # e.g. the 2x width fix
```

| step | commit | what it changed |
|---|---|---|
| 1 — two lanes, L to toggle | `eee380c3` | `LineStyle`, the draw-site match |
| 3 — width 2x, and the oct poles | `cd4af476` | `half_width_px`, `oct16` signNotZero, all-ones sentinel |
| 4 — trapezoid width | `c9dfddf6` | `hw0`/`hw1` flat + `resolve_width` |
| 5 — near-plane clip | `c9dfddf6` | clip against `z - w = 0` |
| 2, 6 — 40 B row + facing cull | `ff86cfab` | repacked `CylinderSegment`, `edge_faces_camera`, `eye_from_view_proj` |
| 7, 8 — the hug | `6c8f8c50` | `join3`, `ink_depth`, the derived epsilon, `FLAG_INSIDE` |
| 9 — markers on top | `2fbd7cd2` | draw last + `GreaterEqual`, the `band_span` bound |
| 10 — density LOD + lift cap | `4f1ae1d1` | `WIRE_MIN_PX`, `MARKER_MIN_PX`, `lift_capped` |
| 11 — harness | `662c099f`, `e13ab7b3` | `VIEWER_ORBIT`/`VIEWER_ZOOM`, manifest loading |

Two commits in that range are **dead ends kept on purpose** — `b50e78fd` (lift + hardware slope
bias) and `04b17444` (the `dpdx/dpdy` slope bias, reverted because at the strength that kills the
wedge it puts 16 px slivers along every shared edge). If you find yourself reinventing either,
their messages say what happened.

---

## Next

[`36-cloud-tables.md`](36-cloud-tables.md) — meshes now have a lane that scales. Point clouds
do not: this lesson routes `Geometry::PointCloud` into the flat glyph dots, which is the right
answer for 32b's demo clouds and the wrong one for a 13.8M-point scan. 36 gives dense clouds their
own lane; 37 measures where 3.5 GB went for 323 MB of GPU data and takes the peak down by a third;
38 halves the GPU table; 39 streams the file straight into GPU memory so the peak stops growing
with the scene.

Then [`66-scene-bvh.md`](66-scene-bvh.md) — `Scene` now has a fixed, ordered object list; that
lesson gives it a broad-phase AABB BVH over their world boxes. One BVH, reused by frustum culling
(41), picking (47), and box-select (50) — the "one acceleration structure, many uses" principle,
and the reason the object list had to stabilize here first.
