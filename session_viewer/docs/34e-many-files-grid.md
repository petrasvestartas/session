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

The streaming split already happened in 34d: `Gpu::walk_session(&session) -> SceneTables` walks
ONE file into compact tables, `state.rs` fetches→parses→walks→DROPS each session in a loop, and
`Gpu::new(window, &[SceneTables])` merges the walked files — but the merge just stacks every file
at the origin, on top of each other. Today the merge becomes a **cycling grid** (a different
drawing per cell, `STRESS_GRID`² cells minimum), the device requests **adapter-real storage
limits**, the fixture list grows to **all nine drawings**, and two build-config unlocks keep
debug loads fast and alive.

## Where you already are (the 34d checkpoint)

`gpu/mod.rs` has, top to bottom: `pub struct SceneTables` (right above `pub struct Gpu`),
`Gpu::new(window, files: &[SceneTables])` with a flat merge loop, and `Gpu::walk_session` sitting
between the end of `new()` and `rebase_anchor` — ONE `impl Gpu` block. `state.rs` already streams
`DEMO_SESSION_URLS` (two entries so far). If any of these live elsewhere in your copy, move them
to match — every later lesson anchors on this layout (35 deletes `SceneTables` from the top of
the file and `walk_session` from that exact seam).

## The map — `gpu/mod.rs` by line number (BEFORE this lesson's edits)

```
  29   const SPH_LATS: usize = 6;              ← Step 1 inserts right below this
  34   pub struct SceneTables                   34d — untouched today
  45   pub struct Gpu
  93   impl Gpu · pub async fn new(            ← Steps 2a/2b/2c all live INSIDE new()
 122       let (device, queue) = adapter       ← 2a inserts 4 lines ABOVE this
 218       for t in files { … }                ← 2b replaces this whole loop
 282       log::info!("scene: …")              ← 2c replaces this call
 289       let instance_buffer = …               end of the edit zone — rest of new() untouched
 533   pub fn walk_session                      34d — untouched today
 600   pub fn rebase_anchor                     33/34c — untouched
```

Every insert shifts everything below it down a few lines — the numbers are exact only for the
FIRST edit you make; after that, trust the named landmark over the number.

## Files we touch

```
src/engine/gpu/mod.rs   # Steps 1-2: STRESS_GRID, adapter-real limits, the grid merge loop
src/state.rs            # Step 3: nine-file DEMO_SESSION_URLS + total load timing
index.html              # Step 4: copy-file per fixture
Cargo.toml              # Step 5a: [profile.dev.package."*"] opt-level = 3 (parse ~5× in debug)
.cargo/config.toml      # Step 5b: wasm max-memory 4GB (the OOM ceiling)
```

> Every step compiles on its own — only 2b is one atomic replace; `cargo check` freely in
> between.

## Step 1 — `STRESS_GRID` at the top of `src/engine/gpu/mod.rs`

**Where:** top of the file, **line 29** — the last of the three `SPH_*`/`CYL_*` consts, just
before the `SceneTables` doc comment. Find:

```rust
const SPH_LONS: usize = 12;
const SPH_LATS: usize = 6;

/// One loaded file, walked into GPU-ready tables. Built by [`Gpu::walk_session`] BEFORE
```

**Insert between `const SPH_LATS…` and the doc comment:**

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
Find:

```rust
        // 4. Device (creates resources) + Queue (submits work).
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
loop that stacks every file at the origin. **Find the whole loop:**

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

**Replace it with the grid version** — cells cycle the files, each cell's objects get a
translated model, bounds now INCLUDE the offset:

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
`let instance_buffer = …` (≈ 289). Find:

```rust
        log::info!("scene: {} files, {} objects, {} arena verts, {} segments, {} glyphs",
            files.len(), instances.len(), verts.len(), segments.len(), glyphs.len());
```

Replace with:

```rust
        log::info!("grid: {} cells x {} files: {} objects, {} arena verts, {} segments, {} glyphs",
            cells, files.len(), instances.len(), verts.len(), segments.len(), glyphs.len());
```

The next line after the log is `let instance_buffer =  storage_buffer(…)` — from here to the end
of `new()` nothing changes. `walk_session` needs NO edit today.

## Step 3 — `src/state.rs`: all nine drawings + load timing

**3a. The URL list grows to nine.**

**Where:** `src/state.rs`, **line 15** — the const right above `pub struct State`. Find:

```rust
// Runtime fetch path — must match an index.html copy-file target (data-target-path + filename).
const DEMO_SESSION_URLS: &[&str] = &[ 
    "session_data/30700_querschnitt_gg.pb",
    "session_data/draw_pj_treppenhaus_a.pb",
    // …one line per fixture; each must match an index.html copy-file target
];
```

Replace with:

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

**3b. Total timing around the loop.** The per-file log inside the loop already exists; add the
totals.

**Where:** `src/state.rs`, inside `State::new` — first line of the body (**≈ line 30**), then the
`Gpu::new` call right after the loop's closing `}` (**≈ line 46**). Find:

```rust
        let mut files = Vec::new();
```

Insert ABOVE it:

```rust
        let t0 = now_ms();
```

Then find (below the loop):

```rust
        let gpu = Gpu::new(window.clone(), &files).await?;
```

and grow it to three lines:

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
`<canvas id="canvas">`. Find the existing fixture line (the `floor_model.pb` line above it
stays):

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

## Step 5 — the two build-config unlocks

**5a. `Cargo.toml`** — parse speed in debug builds. Dependencies (prost above all) run optimized
while your own crate keeps fast rebuilds — measured **3.9s → 0.6s** parsing the 20MB querschnitt.

**Where:** `Cargo.toml` (viewer crate root), **line 44** — below the dependency tables. Find:

```toml
[profile.release]
strip = true
```

Insert ABOVE it (blank line between the tables):

```toml
[profile.dev.package."*"]
opt-level = 3
```

**5b. `.cargo/config.toml`** — the OOM killer.

**Where:** `.cargo/config.toml` (viewer crate root, next to `Cargo.toml`). The file already
exists (it pins every `cargo` command to the wasm32 target) and is 4 lines long. The default
wasm linear-memory ceiling (1GB) dies loading multi-hundred-MB fixture sets with
`RuntimeError: unreachable` (an allocator abort, no panic message). **Append at the end, below
the `[build]` table:**

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
Ch 34e: CYCLE THE WALK. 34d's stream (walk one file, DROP the session, peak = one file) feeds
        Gpu::new(&[SceneTables]), whose merge now lays cells out cycling the files (STRESS_GRID²
        floor, 5% gutters, bounds INCLUDE offsets — F fits the wall). wasm max-memory 4GB kills
        the allocator abort; dev.package opt-level 3 makes debug parse ~5× faster; adapter-real
        storage limits defuse the 128MB binding wall (~1.4M objects) before it's ever hit.
        503,516 objects, 4 draw calls.
```

## Next

`34f-flat-linework.md` — 14M triangles for 2px-wide lines is the last wall. Pay per pixel:
capsule ribbons, glyph dots, and a switch that keeps the 3D pipes one constant away.
