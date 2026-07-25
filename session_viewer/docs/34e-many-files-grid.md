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

## Files we touch

```
src/engine/gpu/mod.rs   # SceneTables (pub), walk_session (walk moves OUT of new), grid layout
src/state.rs            # DEMO_SESSION_URLS list; fetch→parse→walk→drop loop
src/index.html          # copy-file per fixture
.cargo/config.toml      # wasm max-memory 4GB (the OOM ceiling)
Cargo.toml              # [profile.dev.package."*"] opt-level = 3 (parse ~5× in debug)
```

## Step 1 — the walk becomes a per-file step: `gpu/mod.rs`

**1a. A module-level table struct** (fields private — callers pass it opaquely):

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

**1b. `Gpu::walk_session(session) -> SceneTables`** — 34b's whole `match` over the 11 `Geometry`
variants moves verbatim out of `new()` into this associated fn (pushing into `t.*` instead of
locals), followed by a min/max extent pass over verts/segments/glyphs. `Gpu::new`'s signature
becomes:

```rust
    pub async fn new(
        window: std::sync::Arc<winit::window::Window>,
        files: &[SceneTables]) -> anyhow::Result<Self> {
```

## Step 2 — the streaming loader: `state.rs`

`DEMO_SESSION_URL` becomes a LIST, and the loop drops each parsed session before the next fetch:

```rust
const DEMO_SESSION_URLS: &[&str] = &[
    "session_data/30700_querschnitt_gg.pb",
    "session_data/draw_pj_treppenhaus_a.pb",
    // …one line per fixture; each must match an index.html copy-file target
];
```

```rust
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
        let gpu = Gpu::new(window.clone(), &files).await?;
```

`index.html` gets one `copy-file` link per fixture (34a Step 2's pattern).

## Step 3 — the grid cycles the files: `gpu/mod.rs` `new()`

Where the old single-session walk sat, the merge loop now places files into cells — **cells cycle
through the loaded files** (different drawing per cell), `STRESS_GRID²` floors the cell count for
load testing, and the cell size is the largest file extent + 5%:

```rust
        let cells = ((STRESS_GRID * STRESS_GRID) as usize).max(files.len());
        let cols = (cells as f64).sqrt().ceil() as usize;
        // cell_w/cell_h = max (t.max - t.min) over files; dx/dy = ×1.05
        for cell in 0..cells {
            let t = &files[cell % files.len()];
            let off = Xform::translation((cell % cols) as f64 * dx, (cell / cols) as f64 * dy, 0.0);
            let ri0 = objects_base.len() as u32; // this cell's first instance row
            for (m, c) in &t.objects { objects_base.push((&off * m, *c)); }
            for s in &t.segments { let mut s2 = *s; s2.instance_id += ri0; segments.push(s2); }
            // …glyphs likewise; verts/vids/idx with a vbase index shift…
            // Scene bounds INCLUDE the cell offset — F fits the whole grid.
        }
```

> That last line fixes a latent 34b bug: bounds were computed from raw table coordinates,
> ignoring placements — `F` only ever fitted tile one. Accumulating per placed cell
> (`t.min + offset`) makes `F` fit the wall.

## Step 4 — the two build-config unlocks

**4a. `.cargo/config.toml`** — the OOM killer. The default wasm linear-memory ceiling (1GB) dies
loading multi-hundred-MB fixture sets with `RuntimeError: unreachable` (an allocator abort, no
panic message):

```toml
[target.wasm32-unknown-unknown]
rustflags = ["-C", "link-arg=--max-memory=4294967296"]
```

**4b. `Cargo.toml`** — parse speed in debug builds. Dependencies (prost above all) run optimized
while your own crate keeps fast rebuilds — measured **3.9s → 0.6s** parsing the 20MB querschnitt:

```toml
[profile.dev.package."*"]
opt-level = 3
```

(Both trigger one slow full-rebuild of dependencies, then everything is cached.)

**4c. Ask the GPU for its real limits** — baseline WebGPU caps any storage-buffer BINDING at
128MB: a hard wall at ~1.4M objects (96B instance rows) / ~2.8M segments (48B rows). Desktop
adapters allow far more — request it, but only for the two limits we lean on. **In `Gpu::new`,
replace `required_limits: wgpu::Limits::default(),`:**

```rust
        let mut limits = wgpu::Limits::default();
        let hw = adapter.limits();
        limits.max_storage_buffer_binding_size = hw.max_storage_buffer_binding_size;
        limits.max_buffer_size = hw.max_buffer_size;
        // …
                required_limits: limits,
```

## Verify

```
loaded 'my_session': 42232 objects, 20778146 bytes | fetch 114ms · parse 972ms
…eight more…
grid: 9 cells x 9 files: 503516 objects, 1 arena verts, 598604 segments, 1 glyphs
9 files | load ~17s · gpu ~600ms
```

`F` fits the whole wall; every cell is a different drawing with its own colors. Draw count: 4.
Orbit/pan stay anchor-cheap (34c). The frame is now genuinely GPU-bound — 598k cylinder segments
≈ 14M triangles — which is 34f's problem.

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
