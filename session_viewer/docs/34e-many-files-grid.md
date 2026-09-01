# 34e Nine drawings, one wall — streaming multi-file load + the cycling grid

> **Big picture.** The stress gate graduates: not one drawing tiled nine times, but **nine
> different real construction drawings** (Treppenhaus sections at ~120k objects each, Grundrisse,
> Schalungsbilder — converted from a real project PDF set) in a 3×3 wall: **503,516 objects,
> 598,604 segments, still 4 draw calls**. Getting there killed two dragons: a wasm
> out-of-memory crash (`RuntimeError: unreachable` — the 1GB default linear-memory ceiling), and
> a load path that held every parsed Session in memory at once. The cure for both is the same
> shape as 34c's anchor: keep only what rendering needs.

<svg viewBox="0 0 680 100" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="each file is fetched, parsed, walked into compact tables, then dropped; the grid cycles the compact tables into cells" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g stroke="#6fb3ff" stroke-width="1.5" fill="none">
    <rect x="10"  y="24" width="130" height="34"/>
    <rect x="190" y="24" width="150" height="34"/>
    <rect x="390" y="24" width="120" height="34"/>
    <rect x="560" y="24" width="110" height="34"/>
  </g>
  <g fill="#d7dae0" text-anchor="middle">
    <text x="75"  y="45">fetch + parse</text>
    <text x="265" y="45">walk_session → tables</text>
    <text x="450" y="45">DROP session</text>
    <text x="615" y="45">grid cells</text>
  </g>
  <text x="340" y="84" fill="#888" text-anchor="middle">peak memory = ONE parsed session + compact tables — not nine sessions at once</text>
</svg>

## The plan in one paragraph

**Step 0 performs the streaming split** — the piece no earlier lesson taught: the object walk
leaves the middle of `Gpu::new` and becomes `Gpu::walk_session(&session) -> SceneTables`, one
file at a time; `state.rs` fetches→parses→walks→DROPS each session in a loop; and
`Gpu::new(window, &[SceneTables])` merges the walked files — flatly at first, every file stacked
at the origin. The rest of the lesson turns that merge into a **cycling grid** (a different
drawing per cell, `STRESS_GRID`² cells minimum), makes the device request **adapter-real storage
limits**, grows the fixture list to **all nine drawings**, and adds two build-config unlocks that
keep debug loads fast and alive.

## Where you already are (end of 34d)

`gpu/mod.rs` has ONE `impl Gpu` block, and inside it `Gpu::new(window, session:
&session_rust::Session)` still does everything: the wgpu setup, then 34b's `match` over
`Geometry` walking that ONE session straight into the arena vectors, then a bounding-box scan
over the merged arrays, then every buffer and bind group. `rebase_anchor` (33/34c) follows
`new()`. `state.rs` loads exactly one `DEMO_SESSION_URL`. There is no `SceneTables` and no
`walk_session` yet — Step 0 creates both, and every later lesson anchors on the layout it leaves
(35 deletes `SceneTables` from the top of the file and `walk_session` from that exact seam).

## The map — `gpu/mod.rs` by line number (AFTER Step 0, before Step 1)

```
  29   const SPH_LATS: usize = 6;              ← Step 1 inserts right below this
  34   pub struct SceneTables                   Step 0a — nothing more today
  45   pub struct Gpu
  93   impl Gpu · pub async fn new(            ← Steps 2a/2b/2c all live INSIDE new()
 122       let (device, queue) = adapter       ← 2a inserts 4 lines ABOVE this
 218       for t in files { … }                ← 2b replaces this whole loop
 283       log::info!("scene: …")              ← 2c replaces this call
 290       let instance_buffer = …               end of the edit zone — rest of new() untouched
 534   pub fn walk_session                      Step 0e — nothing more today
 601   pub fn rebase_anchor                     33/34c — untouched
```

Every insert shifts everything below it down a few lines — the numbers are exact only for the
FIRST edit you make; after that, trust the named landmark over the number.

## Files we touch

```
src/engine/gpu/mod.rs   # Step 0: SceneTables + walk_session + the flat merge
                        # Steps 1-2: STRESS_GRID, adapter-real limits, the grid merge loop
src/state.rs            # Step 0: the streaming load loop
                        # Step 3: nine-file DEMO_SESSION_URLS + total load timing
index.html              # Step 4: copy-file per fixture
Cargo.toml              # Step 5a: [profile.dev.package."*"] opt-level = 3 (parse ~5× in debug)
.cargo/config.toml      # Step 5b: wasm max-memory 4GB (the OOM ceiling)
```

> Step 0 is atomic — its eight edits are one compile. After it, every step compiles on its own
> (only 2b is one atomic replace); `cargo check` freely in between.

## Step 0 — the walk moves out of `Gpu::new`: `SceneTables` + `Gpu::walk_session`

34d ended with `Gpu::new(window, &session)` — ONE parsed `Session`, walked into the arena inline,
halfway down a function that also builds every pipeline, buffer and bind group. Nine files cannot
go through that door: `new()` would have to keep nine parsed sessions alive at once just to reach
the walk, and a parsed `Session` (halfedge maps, per-object `String`s) runs ~10× the size of the
arrays the GPU actually wants.

So the walk moves out first, into the shape the whole rest of the lesson builds on:

```
34d:  fetch → parse → Gpu::new(&session)        the walk is inline; the session lives through new()
34e:  per file: fetch → parse → walk → DROP      then Gpu::new(&[SceneTables]) merges the tables
```

`SceneTables` is what survives one file: the six GPU-shaped arrays plus that FILE's own
`min`/`max` extents. The `Session` dies at the end of each loop iteration, so peak memory is ONE
parsed file plus N compact tables — never N sessions.

**0a. The table struct.**

**Where:** top of `src/engine/gpu/mod.rs`, **line 29** — right below the three `SPH_*`/`CYL_*`
consts and above `pub struct Gpu`.

**Find** in `src/engine/gpu/mod.rs`:

```rust
const SPH_LONS: usize = 12;
const SPH_LATS: usize = 6;
```

**Add below it:**

```rust

/// One loaded file, walked into GPU-ready tables. Built by [`Gpu::walk_session`] BEFORE
/// `Gpu::new`, so the parsed `Session` (often 10× larger than these tables) can be dropped
/// before the next file is fetched — peak memory holds ONE session at a time, not all of them.
pub struct SceneTables {
    verts: Vec<RenderVertex>,
    vids: Vec<u32>,
    idx: Vec<u32>,
    segments: Vec<CylinderSegment>,
    glyphs: Vec<GlyphPoint>,
    objects: Vec<(Xform, [f32; 4])>,
    min: [f32; 3],
    max: [f32; 3],
}
```

The fields are private: nothing outside this module builds a `SceneTables` by hand, only
`walk_session` does. `min`/`max` are the FILE's own extents with no placement applied — Step 2b
is what offsets them per grid cell.

**0b. `Gpu::new` takes walked files, not a session.**

**Where:** the `new()` signature, **≈ line 93** — the whole three-line wrapped signature is the
anchor, so the parameter swap cannot land on the wrong `new`.

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub async fn new(
        window: std::sync::Arc<winit::window::Window>,
        session: &session_rust::Session) -> anyhow::Result<Self> {
```

**Replace with**:

```rust
    pub async fn new(
        window: std::sync::Arc<winit::window::Window>,
        files: &[SceneTables]) -> anyhow::Result<Self> {
```

`session` is now undefined inside `new()`, so the compiler names every line that still reads it:
two in the walk (0c) and one in the log (0d). Those are the only three.

**0c. The inline walk becomes a merge over the walked files.**

**Where:** inside `Gpu::new`, **≈ line 207** — the block of `let mut` arena declarations,
immediately after the Time bind group. The first five (`verts`, `vids`, `idx`, `segments`,
`glyphs`) stay exactly as they are; a header comment lands above them, and `objects_base` plus
the walk under it is replaced.

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let mut verts: Vec<RenderVertex> = Vec::new(); // slot 0 - every mesh's vertices, concatenated
```

**Add above it:**

```rust
        // Merge the per-file tables into one arena: mesh indices shift by the vertex base,
        // row ids (vids / instance_id) by the objects base, so every file keeps distinct rows.
```

Now the walk itself. `objects_base` can no longer size itself from `session.lookup`, the scene
bounds move up here (0d takes them out of the bottom of `new()`), and the whole `match` over
`Geometry` leaves — 0e is where it lands.

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let mut objects_base: Vec<(Xform, [f32; 4])> = Vec::with_capacity(session.lookup.len());

        // Each object's placement lives in its xform (kernel convention) - `to_render()`/
        // `start()`/`get_points()` read stored coordinates and ignore it, so the xform IS the
        // instance model. `ri` is the row in objects_base, not the lookup index - skipped
        // variants (Plane/OBB/...) leave no hole.
        for geom in session.lookup.values() {
            let ri = objects_base.len() as u32;
            match geom{
                Geometry::Mesh(m) => {
                    objects_base.push((m.xform.clone(), m.objectcolor().to_f32()));
                    push_mesh(m, ri, &mut verts, &mut vids, &mut idx, &mut segments, &mut glyphs);
                }
                Geometry::BRep(b) => {
                    let bm = b.mesh();
                    objects_base.push((b.xform.clone(), b.surfacecolor.to_f32()));
                    push_mesh(&bm, ri, &mut verts, &mut vids, &mut idx, &mut segments, &mut glyphs);
                }
                Geometry::Line(l) => {
                    objects_base.push((l.xform.clone(), l.linecolor.to_f32()));
                    segments.push(line_to_segment(l, ri));
                }
                Geometry::Polyline(pl) => {
                    objects_base.push((pl.xform.clone(), pl.linecolor.to_f32()));
                    segments.extend(polyline_to_segments(pl, ri));
                }
                Geometry::Point(p) => {
                    objects_base.push((p.xform.clone(), p.pointcolor.to_f32()));
                    glyphs.push(point_to_glyph(p, ri));
                }
                // Later lessons - the match must stay exhaustive over all 11 variants
                Geometry::Plane(_) |
                Geometry::OBB(_) |
                Geometry::PointCloud(_) |
                Geometry::Element(_) |
                Geometry::NurbsCurve(_) |
                Geometry::NurbsSurface(_) => {}
            }
        }
```

**Replace with**:

```rust
        let mut objects_base: Vec<(Xform, [f32; 4])> = Vec::new();
        let mut scene_min = [f32::INFINITY; 3];
        let mut scene_max = [f32::NEG_INFINITY; 3];

        for t in files {
            let vbase = verts.len() as u32;
            let obase = objects_base.len() as u32;
            verts.extend_from_slice(&t.verts);
            vids.extend(t.vids.iter().map(|r| r + obase));
            idx.extend(t.idx.iter().map(|i| i + vbase));
            segments.extend(t.segments.iter().map(|s| CylinderSegment { instance_id: s.instance_id + obase, ..*s }));
            glyphs.extend(t.glyphs.iter().map(|g| GlyphPoint { instance_id: g.instance_id + obase, ..*g }));
            objects_base.extend(t.objects.iter().cloned());
            for k in 0..3 {
                scene_min[k] = scene_min[k].min(t.min[k]);
                scene_max[k] = scene_max[k].max(t.max[k]);
            }
        }

        if !scene_min[0].is_finite() { // no geometry at all - the box the old padded scan produced
            scene_min = [0.0; 3];
            scene_max = [0.0; 3];
        }
```

That loop is the FLAT merge: every file is appended at its own coordinates, so nine drawings land
on top of each other at the origin. Correct, and useless as a stress gate — Step 2b is the fix.
The `is_finite` guard replaces what the old scan gave for free: with no files at all the min/max
are still ±∞, and `F` would fit an infinite box.

**0d. The extents come from the walk, so `new()` stops scanning.**

**Where:** still inside `Gpu::new`, **≈ line 282** — the `// Bounding Box` block plus the log
call under it, between `let arena_index_count = idx.len() as u32;` and `let instance_buffer = …`.
The scan is redundant now — 0c already filled `scene_min`/`scene_max` from the per-file extents
— and it would be WRONG after 2b: arena vertices stay at their file's own coordinates, the cell
offset rides the instance model, so a scan over `verts` can only ever see cell one. (It also
reads the PADDED arrays, placeholders and all.)

**Find** in `src/engine/gpu/mod.rs`:

```rust
        // Bounding Box
        let mut scene_min = [f32::INFINITY; 3];
        let mut scene_max = [f32::NEG_INFINITY; 3];
        for v in &verts{
            for k in 0..3{
                scene_min[k] = scene_min[k].min(v.position[k]);
                scene_max[k] = scene_max[k].max(v.position[k]);
            }
        }
        for s in &segments{
            for p in [s.p0, s.p1]{
                for k in 0..3 {
                    scene_min[k] = scene_min[k].min(p[k]);
                    scene_max[k] = scene_max[k].max(p[k]);
                }
            }
        }
        for g in &glyphs{
            for k in 0..3{
                scene_min[k] = scene_min[k].min(g.center[k]);
                scene_max[k] = scene_max[k].max(g.center[k]);
            }
        }

        log::info!("session '{}': {} objects, {} arena verts, {} segments, {} glyphs",
            session.name, instances.len(), verts.len(), segments.len(), glyphs.len());
```

**Replace with**:

```rust
        log::info!("scene: {} files, {} objects, {} arena verts, {} segments, {} glyphs",
            files.len(), instances.len(), verts.len(), segments.len(), glyphs.len());
```

**0e. `walk_session` — the walk, now a function.**

**Where:** the seam between the end of `new()` and `rebase_anchor` — the tail of the
`Ok(Self { … })` literal is the anchor, and the new function goes after `new()`'s closing brace.
It is the same `match` 34b wrote, with `objects_base`/`verts`/`vids`/`idx`/`segments`/`glyphs`
rewritten as fields of one `t`, plus the per-file extent scan the merge no longer does.

**Find** in `src/engine/gpu/mod.rs`:

```rust
            scene_min,
            scene_max,
         })

    }
```

**Add below it:**

```rust

    /// One file → compact tables. Called from state.rs BEFORE Gpu::new, so the parsed
    /// Session (and its bytes) can be dropped before the next file is fetched.
    pub fn walk_session(session: &session_rust::Session) -> SceneTables {
        let mut t = SceneTables {
            verts: Vec::new(),
            vids: Vec::new(),
            idx: Vec::new(),
            segments: Vec::new(),
            glyphs: Vec::new(),
            objects: Vec::with_capacity(session.lookup.len()),
            min: [f32::INFINITY; 3],
            max: [f32::NEG_INFINITY; 3],
        };
        // Each object's placement lives in its xform (kernel convention) - `to_render()`/
        // `start()`/`get_points()` read stored coordinates and ignore it, so the xform IS the
        // instance model. `ri` is the row in objects, not the lookup index - skipped
        // variants (Plane/OBB/...) leave no hole.
        for geom in session.lookup.values() {
            let ri = t.objects.len() as u32;
            match geom {
                Geometry::Mesh(m) => {
                    t.objects.push((m.xform.clone(), m.objectcolor().to_f32()));
                    push_mesh(m, ri, &mut t.verts, &mut t.vids, &mut t.idx,
                        &mut t.segments, &mut t.glyphs);
                }
                Geometry::BRep(b) => {
                    let bm = b.mesh();
                    t.objects.push((b.xform.clone(), b.surfacecolor.to_f32()));
                    push_mesh(&bm, ri, &mut t.verts, &mut t.vids, &mut t.idx,
                        &mut t.segments, &mut t.glyphs);
                }
                Geometry::Line(l) => {
                    t.objects.push((l.xform.clone(), l.linecolor.to_f32()));
                    t.segments.push(line_to_segment(l, ri));
                }
                Geometry::Polyline(pl) => {
                    t.objects.push((pl.xform.clone(), pl.linecolor.to_f32()));
                    t.segments.extend(polyline_to_segments(pl, ri));
                }
                Geometry::Point(p) => {
                    t.objects.push((p.xform.clone(), p.pointcolor.to_f32()));
                    t.glyphs.push(point_to_glyph(p, ri));
                }
                // Later lessons - the match must stay exhaustive over all 11 variants
                Geometry::Plane(_) | Geometry::OBB(_) |
                Geometry::PointCloud(_) | Geometry::Element(_) |
                Geometry::NurbsCurve(_) | Geometry::NurbsSurface(_) => {}
            }
        }
        // This FILE's extents (no placement yet) — new() offsets them per grid cell.
        for v in &t.verts { for k in 0..3 {
            t.min[k] = t.min[k].min(v.position[k]);
            t.max[k] = t.max[k].max(v.position[k]);
        } }
        for s in &t.segments { for p in [s.p0, s.p1] { for k in 0..3 {
            t.min[k] = t.min[k].min(p[k]);
            t.max[k] = t.max[k].max(p[k]);
        } } }
        for g in &t.glyphs { for k in 0..3 {
            t.min[k] = t.min[k].min(g.center[k]);
            t.max[k] = t.max[k].max(g.center[k]);
        } }
        t
    }
```

It is an associated function, not a method — there is no `&self`, because it runs BEFORE any
`Gpu` exists. That is the whole point: the session is walked and dropped while the GPU is still
just a plan.

**0f. `src/state.rs` streams the files.**

**Where:** `src/state.rs`, **line 15** (the const above `pub struct State`) and then the body of
`State::new`.

**Find** in `src/state.rs`:

```rust
const DEMO_SESSION_URL: &str = "session_data/30700_querschnitt_gg.pb";
```

**Replace with**:

```rust
const DEMO_SESSION_URLS: &[&str] = &[
    "session_data/30700_querschnitt_gg.pb",
    "session_data/draw_pj_treppenhaus_a.pb",
    // …one line per fixture; each must match an index.html copy-file target
];
```

Two entries prove the loop — the second one does not resolve yet (Trunk copies it only from
Step 4), which is exactly what the `is_empty` skip below is for. Step 3a grows the list to nine.
The comment line above the const keeps its old wording; only the const itself changes.

Then the load becomes a loop. The single `t0…t3` ladder goes; per-file timing stays inside the
loop, and the totals arrive in Step 3b.

**Find** in `src/state.rs`:

```rust
        let t0 = now_ms();
        let bytes = persistence::fetch_bytes(DEMO_SESSION_URL).await.unwrap_or_default();
        let t1 = now_ms();
        let session = persistence::session_from_bytes(DEMO_SESSION_URL, &bytes);
        let t2 = now_ms();
        let gpu = Gpu::new(window.clone(), &session).await?;
        let t3 = now_ms();
        log::info!("loaded '{}': {} objects, {} bytes | fetch {:.0}ms · parse {:.0}ms · gpu {:.0}ms · total {:.0}ms",
            session.name, session.lookup.len(), bytes.len(), t1 - t0, t2 - t1, t3 - t2, t3 - t0);
```

**Replace with**:

```rust

        let mut files = Vec::new();

        for url in DEMO_SESSION_URLS{

            let f0 = now_ms();
            let bytes = persistence::fetch_bytes(url).await.unwrap_or_default();
            let f1 = now_ms();
            let session = persistence::session_from_bytes(url, &bytes);
            log::info!("loaded '{}': {} objects, {} bytes | fetch {:.0}ms · parse {:.0}ms",
                session.name, session.lookup.len(), bytes.len(), f1 - f0, now_ms() - f1);
            if !session.lookup.is_empty(){
                files.push(Gpu::walk_session(&session)); // failed fetch = skipped file
            }
            // `session` + `bytes` DROP here - peak memory holds one parsed file, not all of them
        }

        let gpu = Gpu::new(window.clone(), &files).await?;

```

`session` and `bytes` are declared INSIDE the loop body, so each iteration's parsed file is freed
before the next fetch starts — that is the whole streaming property, and it is a scope, not an
API. An empty `lookup` means the fetch failed (`unwrap_or_default`); skipping it keeps a missing
fixture from adding an empty cell to the grid.

**Checkpoint:** `cargo check` passes (the two pre-existing 34c warnings — unused `origin` in
`clear()`, unused first `view_proj` in `render()` — are fine). The frame looks exactly like 34d's:
the console says `scene: 1 files, …`, because the second URL 404s and gets skipped until Step 4
copies it. Same picture, new plumbing — everything below is the grid.

## Step 1 — `STRESS_GRID` at the top of `src/engine/gpu/mod.rs`

**Where:** top of the file, **line 29** — the last of the three `SPH_*`/`CYL_*` consts, just
before the `SceneTables` doc comment (`/// One loaded file, walked into GPU-ready tables.`), which
is what the new const lands in front of.

**Find** in `src/engine/gpu/mod.rs`:

```rust
const SPH_LONS: usize = 12;
const SPH_LATS: usize = 6;
```

**Add below it:**

```rust

/// Grid floor for load testing: at least STRESS_GRID² cells, cycling the loaded files.
const STRESS_GRID: u32 = 3;
```

`SceneTables` itself is untouched — it already carries what the grid needs: each FILE's own
`min`/`max` extents (computed at the end of `walk_session`, no placement), which `new()` will
offset per cell.

## Step 2 — `Gpu::new`: real limits, then the flat merge becomes the grid

**2a. Real GPU limits.** Baseline WebGPU caps any storage-buffer binding at 128MB — a hard wall
at ~1.4M objects (96B instance rows). Desktop adapters allow far more; request it.

**Where:** inside `Gpu::new` (starts line 93), **≈ line 122** — step 4 of new()'s setup ladder
(Instance → Surface → Adapter → **Device**), right after the `.request_adapter(…).await?;` call.

**Find** in `src/engine/gpu/mod.rs`:

```rust
        // 4. Device (creates resources) + Queue (submits work).
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),  // unlock the WEBGpu storage buffers
```

Four lines go ABOVE `let (device, queue)`, and the `required_limits` value is swapped.

**Replace with**:

```rust
        let mut limits = wgpu::Limits::default();
        let hw = adapter.limits();
        limits.max_storage_buffer_binding_size = hw.max_storage_buffer_binding_size;
        limits.max_buffer_size = hw.max_buffer_size;

        // 4. Device (creates resources) + Queue (submits work).
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: limits,  // adapter-real storage limits, not the 128MB baseline
```

(the lines after `required_limits` — `memory_hints`, `..Default::default()`, `})`, `.await?;`,
`device.on_uncaptured_error(…)` — stay as they are).

**2b. The flat merge becomes the cycling grid.**

**Where:** still inside `Gpu::new`, **≈ line 218** — past the MVP and Time uniform blocks, under
the `// Merge the per-file tables into one arena…` comment (≈ line 207). That comment is followed
by six `let mut …` declarations plus `scene_min`/`scene_max` (ALL of these stay), then the flat
loop that stacks every file at the origin.

**Find** in `src/engine/gpu/mod.rs`:

```rust
        for t in files {
            let vbase = verts.len() as u32;
            let obase = objects_base.len() as u32;
            verts.extend_from_slice(&t.verts);
            vids.extend(t.vids.iter().map(|r| r + obase));
            idx.extend(t.idx.iter().map(|i| i + vbase));
            segments.extend(t.segments.iter().map(|s| CylinderSegment { instance_id: s.instance_id + obase, ..*s }));
            glyphs.extend(t.glyphs.iter().map(|g| GlyphPoint { instance_id: g.instance_id + obase, ..*g }));
            objects_base.extend(t.objects.iter().cloned());
            for k in 0..3 {
                scene_min[k] = scene_min[k].min(t.min[k]);
                scene_max[k] = scene_max[k].max(t.max[k]);
            }
        }
```

The grid version cycles the files, each cell's objects get a translated model, and bounds now
INCLUDE the offset.

**Replace with**:

```rust
        // The merge loop: cells cycle through the loaded files (a different drawing per cell),
        // STRESS_GRID² floors the cell count, cell size = largest file extent + 5% gutters.
        // Bounds accumulate per PLACED cell (offset included) — F fits the whole wall.
        let cells = if files.is_empty() { 0 }
                    else { ((STRESS_GRID * STRESS_GRID) as usize).max(files.len()) };
        if cells > 0 {
            let cols = (cells as f64).sqrt().ceil() as usize;
            let (mut cell_w, mut cell_h) = (0.0_f64, 0.0_f64);
            for t in files {
                cell_w = cell_w.max((t.max[0] - t.min[0]) as f64);
                cell_h = cell_h.max((t.max[1] - t.min[1]) as f64);
            }
            let (dx, dy) = (cell_w * 1.05, cell_h * 1.05);   // 5% gutters
            for cell in 0..cells {
                let t = &files[cell % files.len()];
                let o = [(cell % cols) as f64 * dx, (cell / cols) as f64 * dy, 0.0];
                let off = Xform::translation(o[0], o[1], o[2]);
                let ri0 = objects_base.len() as u32;   // this cell's first instance row
                let vbase = verts.len() as u32;        // this cell's first arena vertex
                for (m, c) in &t.objects { objects_base.push((&off * m, *c)); }
                verts.extend_from_slice(&t.verts);
                for id in &t.vids { vids.push(id + ri0); }
                for i in &t.idx { idx.push(i + vbase); }
                for s in &t.segments { let mut s2 = *s; s2.instance_id += ri0; segments.push(s2); }
                for g in &t.glyphs { let mut g2 = *g; g2.instance_id += ri0; glyphs.push(g2); }
                // Scene bounds INCLUDE the cell offset — F fits the whole grid.
                for k in 0..3 {
                    scene_min[k] = scene_min[k].min(t.min[k] + o[k] as f32);
                    scene_max[k] = scene_max[k].max(t.max[k] + o[k] as f32);
                }
            }
        }
```

The `if !scene_min[0].is_finite()` fallback below the loop stays — with zero files
(`cells == 0`) it still catches the infinite box. Everything after it — `let mut instances…`,
the four `is_empty()` padding guards, `let arena_index_count…` — is untouched.

> The offset-inside-the-fold lines fix a latent bug of the flat merge: bounds were computed from
> raw table coordinates, ignoring placements — `F` only ever fitted tile one. Accumulating per
> placed cell (`t.min + offset`) makes `F` fit the wall.

**2c. The log learns the grid.**

**Where:** still inside `Gpu::new`, **≈ line 282** — below the four `is_empty()` padding guards,
sandwiched between `let arena_index_count = idx.len() as u32;` (≈ 280) and
`let instance_buffer = …` (≈ 289).

**Find** in `src/engine/gpu/mod.rs`:

```rust
        log::info!("scene: {} files, {} objects, {} arena verts, {} segments, {} glyphs",
            files.len(), instances.len(), verts.len(), segments.len(), glyphs.len());
```

**Replace with**:

```rust
        log::info!("grid: {} cells x {} files: {} objects, {} arena verts, {} segments, {} glyphs",
            cells, files.len(), instances.len(), verts.len(), segments.len(), glyphs.len());
```

The next line after the log is `let instance_buffer =  storage_buffer(…)` — from here to the end
of `new()` nothing changes, and `walk_session` (Step 0e) needs no further edit.

## Step 3 — `src/state.rs`: all nine drawings + load timing

**3a. The URL list grows to nine.**

**Where:** `src/state.rs`, **line 15** — the two-entry const Step 0f left above
`pub struct State`.

**Find** in `src/state.rs`:

```rust
const DEMO_SESSION_URLS: &[&str] = &[
    "session_data/30700_querschnitt_gg.pb",
    "session_data/draw_pj_treppenhaus_a.pb",
    // …one line per fixture; each must match an index.html copy-file target
];
```

**Replace with**:

```rust
const DEMO_SESSION_URLS: &[&str] = &[
    "session_data/30700_querschnitt_gg.pb",
    "session_data/draw_pb_haus25.pb",
    "session_data/draw_pc_gru_og2.pb",
    "session_data/draw_pd_treppenhaus04.pb",
    "session_data/draw_pe_schalungsbild.pb",
    "session_data/draw_pf_he.pb",
    "session_data/draw_pi_laengsschnitt.pb",
    "session_data/draw_pj_grundriss_og2.pb",
    "session_data/draw_pj_treppenhaus_a.pb",
];
```

**3b. Total timing around the loop.** Step 0f's per-file log stays; the totals wrap it.

**Where:** `src/state.rs`, inside `State::new` — first line of the body (**≈ line 37**), then the
`Gpu::new` call right after the loop's closing `}` (**≈ line 53**).

**Find** in `src/state.rs`:

```rust
        let mut files = Vec::new();
```

**Add above it:**

```rust
        // Total timing around the loop
        let t0 = now_ms();

```

Then the `Gpu::new` call below the loop grows to three lines.

**Find** in `src/state.rs`:

```rust
        let gpu = Gpu::new(window.clone(), &files).await?;
```

**Replace with**:

```rust
        let t1 = now_ms();
        let gpu = Gpu::new(window.clone(), &files).await?;
        log::info!("{} files | load {:.0}ms · gpu {:.0}ms", files.len(), t1 - t0, now_ms() - t1);
```

The `Ok(Self {window, gpu, camera: Camera::new() })` line below stays.

**Checkpoint:** `cargo check` passes (the two pre-existing 34c warnings — unused `origin` in
`clear()`, unused first `view_proj` in `render()` — are fine).

## Step 4 — Trunk copies the fixtures: `index.html`

**Where:** `index.html` (viewer crate root), **line 20** — inside `<body>`, in the
`<link data-trunk rel="copy-file" …>` block between the Trunk rust link and
`<canvas id="canvas">`. The existing fixture line is the anchor; the `floor_model.pb` line above
it stays.

**Find** in `index.html`:

```html
   <link data-trunk rel="copy-file" href="../session_data/30700_querschnitt_gg.pb" data-target-path="session_data"/>
```

One line per new drawing, before `<canvas id="canvas"></canvas>`.

**Add below it:**

```html
   <link data-trunk rel="copy-file" href="../session_data/draw_pb_haus25.pb" data-target-path="session_data"/>
   <link data-trunk rel="copy-file" href="../session_data/draw_pc_gru_og2.pb" data-target-path="session_data"/>
   <link data-trunk rel="copy-file" href="../session_data/draw_pd_treppenhaus04.pb" data-target-path="session_data"/>
   <link data-trunk rel="copy-file" href="../session_data/draw_pe_schalungsbild.pb" data-target-path="session_data"/>
   <link data-trunk rel="copy-file" href="../session_data/draw_pf_he.pb" data-target-path="session_data"/>
   <link data-trunk rel="copy-file" href="../session_data/draw_pi_laengsschnitt.pb" data-target-path="session_data"/>
   <link data-trunk rel="copy-file" href="../session_data/draw_pj_grundriss_og2.pb" data-target-path="session_data"/>
   <link data-trunk rel="copy-file" href="../session_data/draw_pj_treppenhaus_a.pb" data-target-path="session_data"/>
```

## Step 5 — the two build-config unlocks

**5a. `Cargo.toml`** — parse speed in debug builds. Dependencies (prost above all) run optimized
while your own crate keeps fast rebuilds — measured **3.9s → 0.6s** parsing the 20MB querschnitt.

**Where:** `Cargo.toml` (viewer crate root), **line 44** — below the dependency tables.

**Find** in `Cargo.toml`:

```toml
[profile.release]
strip = true
```

**Add above it:**

```toml
[profile.dev.package."*"]
opt-level = 3

```

**5b. `.cargo/config.toml`** — the OOM killer.

**Where:** `.cargo/config.toml` (viewer crate root, next to `Cargo.toml`). The file already
exists (it pins every `cargo` command to the wasm32 target) and is 4 lines long. The default
wasm linear-memory ceiling (1GB) dies loading multi-hundred-MB fixture sets with
`RuntimeError: unreachable` (an allocator abort, no panic message). The new table goes at the end,
below the `[build]` table.

**Append** to `.cargo/config.toml`:

```toml
[target.wasm32-unknown-unknown]
rustflags = ["-C", "link-arg=--max-memory=4294967296"]
```

(Both trigger one slow full-rebuild of dependencies, then everything is cached.)

## Verify

`trunk serve`, open the console; expect the shape:

```
loaded 'my_session': 42232 objects, 20778146 bytes | fetch 114ms · parse 972ms
…eight more…
grid: 9 cells x 9 files: 503516 objects, 1 arena verts, 598604 segments, 1 glyphs
9 files | load ~17s · gpu ~600ms
```

(`1 arena verts` / `1 glyphs` = the padding placeholders — pure line drawings have no mesh
vertices.) `F` fits the whole wall; every cell is a different drawing with its own colors. Draw
count: 4. Orbit/pan stay anchor-cheap (34c). The frame is now genuinely GPU-bound — 598k cylinder
segments ≈ 14M triangles — which is 34f's problem.

## Recap

```
Ch 34e: CYCLE THE WALK. Step 0 splits the walk out of Gpu::new (walk one file into SceneTables,
        DROP the session, peak = one file) and feeds Gpu::new(&[SceneTables]), whose merge then
        stops stacking at the origin and lays cells out cycling the files (STRESS_GRID²
        floor, 5% gutters, bounds INCLUDE offsets — F fits the wall). wasm max-memory 4GB kills
        the allocator abort; dev.package opt-level 3 makes debug parse ~5× faster; adapter-real
        storage limits defuse the 128MB binding wall (~1.4M objects) before it's ever hit.
        503,516 objects, 4 draw calls.
```

## Next

`34f-flat-linework.md` — 14M triangles for 2px-wide lines is the last wall. Pay per pixel:
capsule ribbons, glyph dots, and a switch that keeps the 3D pipes one constant away.
