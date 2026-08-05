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

Today `Gpu::new(window, &session)` takes ONE parsed session and walks it into flat tables
(`verts`/`vids`/`idx`/`segments`/`glyphs`/`objects_base`) right inside `new()`. We split that in
two: a new `Gpu::walk_session(&session) -> SceneTables` does the walk for ONE file (so `state.rs`
can walk each file and immediately drop the parsed session), and `Gpu::new(window, &[SceneTables])`
takes the list of walked files and lays them out as grid cells. Six edits in `gpu/mod.rs`, two in
`state.rs`, then three config files.

## Files we touch

```
src/engine/gpu/mod.rs   # Steps 1-3: SceneTables, walk_session, grid merge loop
src/state.rs            # Step 4: DEMO_SESSION_URLS list; fetch→parse→walk→drop loop
index.html              # Step 5: copy-file per fixture
Cargo.toml              # Step 6a: [profile.dev.package."*"] opt-level = 3 (parse ~5× in debug)
.cargo/config.toml      # Step 6b: NEW FILE — wasm max-memory 4GB (the OOM ceiling)
```

> ⚠ `gpu/mod.rs` will NOT compile between Step 1 and the end of Step 4 — the function is being
> cut in half and reassembled. Type everything through Step 4, then `cargo check`.

## Step 1 — two new items at the top of `src/engine/gpu/mod.rs`

**Find these lines near the top of the file** (right below `const CYL_SIDES` / `SPH_LONS` /
`SPH_LATS`):

```rust
const SPH_LONS: usize = 12;
const SPH_LATS: usize = 6;

pub struct Gpu {
```

**Insert between `const SPH_LATS…` and `pub struct Gpu {`:**

```rust
/// Grid floor for load testing: at least STRESS_GRID² cells, cycling the loaded files.
const STRESS_GRID: u32 = 3;

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

It is a plain struct — **no `impl SceneTables` block anywhere**. The walk that fills it becomes a
function on `Gpu` (Step 3); `Gpu::new` reads the fields directly (same module = private fields are
visible; `state.rs` only passes the struct through, opaquely).

## Step 2 — `Gpu::new` stops walking and lays out the grid

Five edits inside `new()`, top to bottom.

**2a. The signature.** Find:

```rust
    pub async fn new(
        window: std::sync::Arc<winit::window::Window>,
        session: &session_rust::Session) -> anyhow::Result<Self> {
```

Replace the `session` parameter line so it reads:

```rust
    pub async fn new(
        window: std::sync::Arc<winit::window::Window>,
        files: &[SceneTables]) -> anyhow::Result<Self> {
```

**2b. Real GPU limits.** Baseline WebGPU caps any storage-buffer binding at 128MB — a hard wall at
~1.4M objects (96B instance rows). Desktop adapters allow far more; request it. Find:

```rust
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),  // unlock the WEBGpu storage buffers
```

Insert 4 lines ABOVE `let (device, queue)` and swap the `required_limits` value, so it reads:

```rust
        let mut limits = wgpu::Limits::default();
        let hw = adapter.limits();
        limits.max_storage_buffer_binding_size = hw.max_storage_buffer_binding_size;
        limits.max_buffer_size = hw.max_buffer_size;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: limits,  // adapter-real storage limits, not the 128MB baseline
```

(the four lines after `required_limits` — `memory_hints`, `..Default::default()`, `})`,
`.await?;` — stay as they are).

**2c. `objects_base` loses its `session` reference.** Scroll down to the six `let mut …`
declarations (after the Time-uniform block). Five of them stay untouched; the LAST one mentions
`session`, which no longer exists here. Find:

```rust
        let mut objects_base: Vec<(Xform, [f32; 4])> = Vec::with_capacity(session.lookup.len());
```

Replace with:

```rust
        let mut objects_base: Vec<(Xform, [f32; 4])> = Vec::new();
```

**2d. The walk goes away; the grid merge loop takes its place.** Directly below the declaration
you just edited sits the whole 34b walk. **Delete everything from the comment line**

```rust
        // Each object's placement lives in its xform (kernel convention) - `to_render()`/
```

**down to and including the `}` that closes `for geom in session.lookup.values() {`** — the last
two lines of the deletion are:

```rust
            }
        }
```

and the next line you KEEP is `let mut instances: Vec<Instance> = objects_base.iter()`. (Don't
throw the walk away — Step 3 pastes it back inside `walk_session`, and you can copy the five
`Geometry::…` arms from it instead of retyping.)

**In the hole you just made, insert the merge loop** (between `let mut objects_base…` and
`let mut instances…`):

```rust
        // The merge loop: cells cycle through the loaded files (a different drawing per cell),
        // STRESS_GRID² floors the cell count, cell size = largest file extent + 5% gutters.
        // Bounds accumulate per PLACED cell (offset included) — F fits the whole wall.
        let mut scene_min = [f32::INFINITY; 3];
        let mut scene_max = [f32::NEG_INFINITY; 3];
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

Everything below — `let mut instances…`, the four `is_empty()` padding guards,
`let arena_index_count…` — is untouched.

> The bounds-inside-the-loop lines fix a latent 34b bug: bounds were computed from raw table
> coordinates, ignoring placements — `F` only ever fitted tile one. Accumulating per placed cell
> (`t.min + offset`) makes `F` fit the wall.

**2e. The old bounds fold + log die.** The merge loop now owns `scene_min`/`scene_max`, so the
old fold is a duplicate declaration (E0428). Find, a screen below the padding guards:

```rust
        let arena_index_count = idx.len() as u32;

        // Bounding Box
        let mut scene_min = [f32::INFINITY; 3];
```

and **delete from `// Bounding Box` down to and including the old log call** — the deletion ends
with these two lines:

```rust
        log::info!("session '{}': {} objects, {} arena verts, {} segments, {} glyphs",
            session.name, instances.len(), verts.len(), segments.len(), glyphs.len());
```

(everything between — the three `for v in &verts` / `for s in &segments` / `for g in &glyphs`
min/max loops — goes too). **In its place, right after `let arena_index_count…`, insert:**

```rust
        log::info!("grid: {} cells x {} files: {} objects, {} arena verts, {} segments, {} glyphs",
            cells, files.len(), instances.len(), verts.len(), segments.len(), glyphs.len());
```

The next line after the new log is `let instance_buffer =  storage_buffer(…)` — from here to the
end of `new()` nothing changes.

## Step 3 — the walk comes back as `Gpu::walk_session`

Find the seam between `new()` and `rebase_anchor` — the end of `new()` looks like:

```rust
            scene_min,
            scene_max,
         })

    }

    /// The anchor the instance table is rebased about.
```

**Insert the whole function between that `}` and the `/// The anchor…` doc comment.** The match
body is EXACTLY the walk you deleted in 2d with three mechanical renames: `objects_base` →
`t.objects`, `&mut verts, &mut vids, &mut idx, &mut segments, &mut glyphs` → the `&mut t.*` forms,
and each bare `segments.push`/`glyphs.push` → `t.segments.push`/`t.glyphs.push`:

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

## Step 4 — the streaming loader: `src/state.rs`

**4a. The URL becomes a list.** Find:

```rust
// Runtime fetch path — must match an index.html copy-file target (data-target-path + filename).
const DEMO_SESSION_URL: &str = "session_data/30700_querschnitt_gg.pb";
```

Replace both lines with:

```rust
// Runtime fetch paths — each must match an index.html copy-file target (data-target-path + filename).
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

**4b. The fetch→parse→gpu block becomes a loop.** In `State::new`, find (the whole body up to the
final `Ok`):

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

Replace all nine lines with — each parsed session dies at the loop's end, before the next fetch:

```rust
        let t0 = now_ms();
        let mut files = Vec::new();
        for url in DEMO_SESSION_URLS {
            let f0 = now_ms();
            let bytes = persistence::fetch_bytes(url).await.unwrap_or_default();
            let f1 = now_ms();
            let session = persistence::session_from_bytes(url, &bytes);
            log::info!("loaded '{}': {} objects, {} bytes | fetch {:.0}ms · parse {:.0}ms",
                session.name, session.lookup.len(), bytes.len(), f1 - f0, now_ms() - f1);
            if !session.lookup.is_empty() {
                files.push(Gpu::walk_session(&session)); // failed fetch = skipped file
            }
            // `session` + `bytes` DROP here — peak memory holds one parsed file, not all nine
        }
        let t1 = now_ms();
        let gpu = Gpu::new(window.clone(), &files).await?;
        log::info!("{} files | load {:.0}ms · gpu {:.0}ms", files.len(), t1 - t0, now_ms() - t1);
```

The `Ok(Self {window, gpu, camera: Camera::new() })` line below stays.

**Checkpoint:** `cargo check --target wasm32-unknown-unknown` now passes (the two pre-existing
34c warnings — unused `origin` in `clear()`, unused first `view_proj` in `render()` — are fine).

## Step 5 — Trunk copies the fixtures: `index.html`

Find the existing fixture line:

```html
   <link data-trunk rel="copy-file" href="../session_data/30700_querschnitt_gg.pb" data-target-path="session_data"/>
```

Insert after it, one line per new drawing (before `<canvas id="canvas"></canvas>`):

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

## Step 6 — the two build-config unlocks

**6a. `Cargo.toml`** — parse speed in debug builds. Dependencies (prost above all) run optimized
while your own crate keeps fast rebuilds — measured **3.9s → 0.6s** parsing the 20MB querschnitt.
Find at the bottom:

```toml
[profile.release]
strip = true
```

Insert ABOVE it (blank line between the tables):

```toml
[profile.dev.package."*"]
opt-level = 3
```

**6b. `.cargo/config.toml`** — the OOM killer. This file does NOT exist yet: create the folder
`.cargo/` next to `Cargo.toml` (inside the viewer crate, NOT the repo root), then create
`config.toml` inside it with exactly this content. The default wasm linear-memory ceiling (1GB)
dies loading multi-hundred-MB fixture sets with `RuntimeError: unreachable` (an allocator abort,
no panic message):

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
Ch 34e: STREAM, WALK, DROP, CYCLE. SceneTables + Gpu::walk_session let each parsed Session die
        right after its walk (peak = one file); Gpu::new takes &[SceneTables] and lays cells out
        cycling the files (STRESS_GRID² floor, 5% gutters, bounds INCLUDE offsets — F fits the
        wall). wasm max-memory 4GB kills the allocator abort; dev.package opt-level 3 makes debug
        parse ~5× faster; adapter-real storage limits defuse the 128MB binding wall (~1.4M
        objects) before it's ever hit. 503,516 objects, 4 draw calls.
```

## Next

`34f-flat-linework.md` — 14M triangles for 2px-wide lines is the last wall. Pay per pixel:
capsule ribbons, glyph dots, and a switch that keeps the 3D pipes one constant away.
