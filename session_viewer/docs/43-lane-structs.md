# 43 Lane structs — the flat `Gpu` earns its split

> Replay-verified against the end-of-42 tree: every hunk below was applied mechanically
> and the three golden renders came back PIXEL-IDENTICAL (twice). This lesson changes no
> behavior — it changes where every field lives. **Every lesson from here on is written
> against the NEW paths.**

## Goal

`Gpu` has grown to ~70 flat fields and `ArenaUpload` carries a four-field cloud quadruple
plus two anonymous tuples. Nothing says which fields belong together — yet nine of them
(`instances`, `objects_base`, `base_f32`, `bounded_rows`, …) are ONE invariant that
`set_scene` must rebuild as a unit, and every lane (segments, glyphs, clouds, stream) is
a buffer+bind-group+count family scattered across the list. This lesson groups them **by
lane, not by type**: named row structs replace the tuples, and each lane becomes a
sub-struct mirrored on the CPU (`ArenaUpload`) and GPU (`Gpu`) side.

The one Rust rule that shapes it: sub-struct methods can't reach `self.device` through
`&mut self`, so **lanes never own device/queue** — anything that needs them takes
`(&wgpu::Device, &wgpu::Queue)` as parameters. Field-path splits (`&mut self.objects.rows`
while reading `self.objects.base`) stay free: the borrow checker tracks nested paths.

## Files we touch

| file | change |
|---|---|
| `src/engine/gpu/mod.rs` | the new structs; `Gpu` + `ArenaUpload` declarations; constructor; ~280 field-path renames |
| `src/app/scene.rs` | `CloudDraw`/`ObjectBase` at every push/destructure site |
| `src/lib.rs`, `src/selftest.rs`, `examples/*` | a handful of renamed `gpu.` paths |

The build only compiles again at the END of the lesson — do all steps, then check.

## Step 1 — the new structs

In `src/engine/gpu/mod.rs`, **find** the end of the `LineStyle` enum:

```rust
    /// A camera-facing quad per edge: 6 vertices, the flat lane's own shader. Cheaper, and it
    /// lies IN the surface rather than proud of it.
    Flat,
}
```

**Add below it:**

```rust
/// One draw record for a cloud lane: which rows, which object, and the measured spacing.
#[derive(Clone, Copy)]
pub struct CloudDraw {
    pub first: u32,    // first point row in the lane's tables
    pub count: u32,    // number of points
    pub instance: u32, // object row - instances[] / InstanceTable.base
    pub spacing: f32,  // measured point spacing, world units (0 = unknown)
}

/// The TRUE per-object placement + tint + flags, as walked from the documents.
/// `InstanceTable.rows` is rebased from this every re-anchor; this never moves.
#[derive(Clone)]
pub struct ObjectBase {
    pub model: Xform,
    pub color: [f32; 4],
    pub flags: u32,
}

/// The cloud lane's CPU tables: three parallel flat arrays + one record per cloud.
pub struct CloudTables {
    pub pos: Vec<f32>,  // 3 floats per point, 12 B
    pub col: Vec<u32>,  // RGBA8 per point, 4 B
    pub nrm: Vec<u32>,  // oct16 normal per point (u32::MAX = none), 4 B -> 20 B/pt
    pub draws: Vec<CloudDraw>,
}

impl CloudTables {
    pub fn new() -> Self {
        Self { pos: Vec::new(), col: Vec::new(), nrm: Vec::new(), draws: Vec::new() }
    }
}

/// Per-frame shared uniforms: one buffer + bind group per block, written by
/// `write_frame_uniforms`, bound by every pass.
pub struct FrameUniforms {
    pub mvp_buffer: wgpu::Buffer,
    pub mvp_bind_group: wgpu::BindGroup,
    pub line_buffer: wgpu::Buffer, // px-sizing for every ink lane
    pub line_bind_group: wgpu::BindGroup,
    pub time: f32,
    pub time_buffer: wgpu::Buffer,
    pub time_bind_group: wgpu::BindGroup,
    pub cloud_buffer: wgpu::Buffer, // cloud size scale + viewport + EDL strength
    pub cloud_bind_group: wgpu::BindGroup,
    pub mvp_f32: [f32; 16], // this frame's matrix, CPU-side - the splat records fold it
    pub last_ortho_h: f32,  // ortho half-height (0 = perspective), for the splat k
}

/// Bind-group layouts that survive init, so set_scene/resize can rebuild bind groups
/// and pipelines (an MSAA flip rebuilds every pipeline).
pub struct Layouts {
    pub mvp: wgpu::BindGroupLayout,
    pub time: wgpu::BindGroupLayout,
    pub instance: wgpu::BindGroupLayout,
    pub line: wgpu::BindGroupLayout,
    pub segment: wgpu::BindGroupLayout,
    pub glyph: wgpu::BindGroupLayout,
}

/// The mesh arena: every tessellated triangle in the scene, appended per file.
pub struct Arena {
    pub vbo: wgpu::Buffer,
    pub vids: wgpu::Buffer,
    pub ibo: wgpu::Buffer,
    pub index_count: u32,
    pub vert_count: u32, // rows already on the GPU - the base for the next append
    pub vert_cap: u64,
    pub index_cap: u64,
}

/// The per-object instance table and everything that rebuilds it: the TRUE bases, the
/// cached f32 casts, the world AABBs for FLAG_INSIDE, and the rebase throttle. These
/// fields are ONE invariant - set_scene rebuilds them together, rebase re-patches rows.
pub struct InstanceTable {
    pub rows: Vec<Instance>,
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub base: Vec<ObjectBase>,    // TRUE world model+color+flags; rows are rebased from this
    pub base_f32: Vec<[f32; 16]>, // model.to_f32() cached once - rebase re-patches 3 slots
    pub bounded_rows: Vec<u32>,   // rows with Some(world AABB) - the inside test walks only these
    pub bounds_world: Vec<Option<([f64; 3], [f64; 3])>>,
    pub inside: Vec<bool>,        // FLAG_INSIDE state per row, for change detection
    pub last_origin: Option<Point>, // rebuild skips while the camera target has not moved
    pub last_rebase_ms: f64,      // throttle - a 210k-row rebase costs ~25 ms
}

/// A segment lane: mesh/BRep edge pipes in rows [0..pipe_count], flat ribbons after,
/// ONE buffer - switching tube/flat style costs a branch at the draw site, no memory.
pub struct SegmentLane {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub count: u32,
    pub pipe_count: u32, // rows [0..pipe_count] are the SOLID lane, the rest are ribbons
    pub template_vbo: wgpu::Buffer, // unit cylinder
    pub template_ibo: wgpu::Buffer,
    pub template_index_count: u32,
}

/// The glyph lane: mesh/BRep vertex markers in rows [0..sphere_count], flat SDF dots after.
pub struct GlyphLane {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub count: u32,
    pub sphere_count: u32, // rows [0..sphere_count] are the SOLID lane, the rest flat dots
    pub template_vbo: wgpu::Buffer, // unit quad
    pub template_ibo: wgpu::Buffer,
    pub template_index_count: u32,
}

/// The WALKED cloud lane on the GPU: three flat buffers + the CPU-side draw records.
/// Rebuilt whole by every set_scene - which is why streamed clouds have their own lane.
pub struct CloudLane {
    pub pos: wgpu::Buffer, // positions, array<f32>
    pub col: wgpu::Buffer, // colours, array<u32> RGBA8
    pub nrm: wgpu::Buffer, // normals, array<u32> oct16 (u32::MAX = none)
    pub count: u32,
    pub draws: Vec<CloudDraw>,
}

/// The compute splatter: per-pixel depth/colour buffers, the record table, and the
/// groups/pipelines that run it. Both cloud lanes write the SAME per-pixel buffers.
pub struct Splat {
    pub depth_buf: wgpu::Buffer, // one u32 per pixel: winning reverse-Z bits (0 = empty)
    pub color_buf: wgpu::Buffer, // one u32 per pixel: winner's RGBA8
    pub recs: wgpu::Buffer,      // header + one Rec per cloud, written per frame
    pub group0_layout: wgpu::BindGroupLayout,
    pub group1_layout: wgpu::BindGroupLayout,
    pub resolve_layout: wgpu::BindGroupLayout,
    pub group0: wgpu::BindGroup,
    pub group1: wgpu::BindGroup,
    pub resolve_group: wgpu::BindGroup,
    pub depth_pipeline: wgpu::ComputePipeline,
    pub color_pipeline: wgpu::ComputePipeline,
    pub total: u32,
    pub state: Option<([f32; 16], f32)>, // (mvp, cloud_size) the buffers hold; None = stale
}

/// The STREAM lane: clouds whose points never existed on the CPU. Its own three buffers
/// and record table - the walked lane is rebuilt whole by every set_scene, so a streamed
/// cloud cannot live there. The two lanes meet in the shared per-pixel depth/colour
/// buffers: atomics compose across dispatches.
pub struct StreamLane {
    pub pos: wgpu::Buffer,
    pub col: wgpu::Buffer,
    pub nrm: wgpu::Buffer,
    pub capacity: u64, // rows
    pub count: u32,
    pub pos_at: u32,
    pub col_at: u32,
    pub draws: Vec<CloudDraw>,
    pub recs: wgpu::Buffer,
    pub group0: wgpu::BindGroup,
    pub group1: wgpu::BindGroup,
}
```

## Step 2 — `ArenaUpload` regroups its cloud quadruple

**Find**:

```rust
    pub cloud_pos: Vec<f32>,  // Raw lane: 3 floats per point, 12 B
    pub cloud_col: Vec<u32>,  // Raw lane: RGBA8 per point, 4 B
    pub cloud_nrm: Vec<u32>,  // Raw lane: oct16 normal per point (u32::MAX = none), 4 B -> 20 B/pt
    pub cloud_draws: Vec<(u32, u32, u32, f32)>, // (first, count, instance, point spacing world units)
    pub objects: Vec<(Xform, [f32; 4], u32)>,
```

**Replace with:**

```rust
    pub clouds: CloudTables,  // Raw lane: flat point rows + one draw record per cloud
    pub objects: Vec<ObjectBase>,
```

**Find** in `ArenaUpload::new()`:

```rust
            cloud_pos: Vec::new(),
            cloud_col: Vec::new(),
            cloud_nrm: Vec::new(),
            cloud_draws: Vec::new(),
            objects: Vec::new(),
```

**Replace with:**

```rust
            clouds: CloudTables::new(),
            objects: Vec::new(),
```

## Step 3 — the `Gpu` declaration, wholesale

**Find** `pub struct Gpu {` and **replace the whole declaration** — everything down to and
including the closing `}` after `pub scene_max: [f32; 3],` (~100 lines) — **with:**

```rust
pub struct Gpu {
    pub surface: Option<wgpu::Surface<'static>>, // Screen to draw pixels on; None when headless.
    pub device: wgpu::Device,               // Handle to the GPU, used to create resources.
    pub queue: wgpu::Queue,                 // Used to submit work to the GPU.
    pub config: wgpu::SurfaceConfiguration, // Settings for Surface: size, pixel format
    pub pipelines: Pipelines,
    pub frame: FrameUniforms, // per-frame shared uniforms (camera, pens, time, cloud)
    pub layouts: Layouts,     // bind-group layouts that survive init
    pub arena: Arena,         // the mesh triangle arena
    pub objects: InstanceTable, // per-object rows + rebase state
    pub seg: SegmentLane,     // edges: solid pipes + flat ribbons, one table
    pub glyphs: GlyphLane,    // vertex markers + flat dots, one table
    pub cloud: CloudLane,     // WALKED point clouds
    pub splat: Splat,         // the compute splatter both cloud lanes share
    pub stream: StreamLane,   // STREAMED point clouds
    /// Solid-lane style; `VIEWER_LINE_STYLE=flat` picks Flat at startup.
    pub line_style: LineStyle,
    pub cloud_size: f32,   // global SCALE on per-cloud sizes, [ and ] keys
    pub edl_strength: f32, // Eye-Dome Lighting strength; 0 = off (VIEWER_EDL)
    pub depth_view: wgpu::TextureView,
    pub msaa_view: wgpu::TextureView,
    pub samples: u32, // MSAA sample count this scene chose (see `msaa_for`)
    pub performance: Performance,
    pub scene_min: [f32; 3],
    pub scene_max: [f32; 3],
}
```

## Step 4 — the constructor

The locals in `Gpu::build` keep their names; only the final expression changes.

**Find** `Ok(Self { ` (the constructor's last expression) and **replace the whole literal**
— everything down to and including its closing `})` — **with:**

```rust
        Ok(Self {
            surface,
            device,
            queue,
            config,
            pipelines,
            frame: FrameUniforms {
                mvp_buffer,
                mvp_bind_group,
                line_buffer,
                line_bind_group,
                time: 0.0,
                time_buffer,
                time_bind_group,
                cloud_buffer,
                cloud_bind_group,
                mvp_f32: [0.0; 16],
                last_ortho_h: 0.0,
            },
            layouts: Layouts {
                mvp: mvp_layout,
                time: time_layout,
                instance: instance_layout,
                line: line_layout,
                segment: segment_layout,
                glyph: glyph_layout,
            },
            arena: Arena {
                vbo: arena_vbo,
                vids: arena_vids,
                ibo: arena_ibo,
                index_count: arena_index_count,
                vert_count: arena_vert_count,
                vert_cap: arena_vert_cap,
                index_cap: arena_index_cap,
            },
            objects: InstanceTable {
                rows: instances,
                buffer: instance_buffer, // must live on GPU: rebuild_instances rewrites it
                bind_group: instance_bind_group,
                base: objects_base,
                base_f32,
                bounded_rows,
                bounds_world: Vec::new(),
                inside: Vec::new(),
                last_origin: None,
                last_rebase_ms: 0.0,
            },
            seg: SegmentLane {
                buffer: segment_buffer,
                bind_group: segment_bind_group,
                count: segment_count,
                pipe_count,
                template_vbo: cyl_template_vbo,
                template_ibo: cyl_template_ibo,
                template_index_count: cyl_index_count,
            },
            glyphs: GlyphLane {
                buffer: glyph_buffer,
                bind_group: glyph_bind_group,
                count: glyph_count,
                sphere_count,
                template_vbo: sph_template_vbo,
                template_ibo: sph_template_ibo,
                template_index_count: sph_index_count,
            },
            cloud: CloudLane {
                pos: point_buffer,
                col: point_col_buffer,
                nrm: point_nrm_buffer,
                count: point_count,
                draws: Vec::new(),
            },
            splat: Splat {
                depth_buf: splat_depth_buf,
                color_buf: splat_color_buf,
                recs: splat_recs,
                group0_layout: splat_group0_layout,
                group1_layout: splat_group1_layout,
                resolve_layout: splat_resolve_layout,
                group0: splat_group0,
                group1: splat_group1,
                resolve_group: splat_resolve_group,
                depth_pipeline: splat_depth_pipeline,
                color_pipeline: splat_color_pipeline,
                total: 0,
                state: None,
            },
            stream: StreamLane {
                pos: stream_pos_buf,
                col: stream_col_buf,
                nrm: stream_nrm_buf,
                capacity: 1,
                count: 0,
                pos_at: 0,
                col_at: 0,
                draws: Vec::new(),
                recs: splat_stream_recs,
                group0: splat_group0_stream,
                group1: splat_group1_stream,
            },
            line_style: if std::env::var("VIEWER_LINE_STYLE").map(|v| v.eq_ignore_ascii_case("tubes")).unwrap_or(false) {
                LineStyle::Tubes
            } else {
                LineStyle::Flat
            },
            cloud_size: std::env::var("VIEWER_CLOUD_SCALE").ok().and_then(|v| v.parse().ok()).unwrap_or(1.0),
            edl_strength: std::env::var("VIEWER_EDL").ok().and_then(|v| v.parse().ok()).unwrap_or(0.25),
            depth_view,
            msaa_view,
            samples,
            performance: Performance::new(),
            scene_min,
            scene_max,
        })
```

One local's type annotation changes with it. **Find**:

```rust
        let objects_base: Vec<(Xform, [f32; 4], u32)> = Vec::new();
```

**Replace with:**

```rust
        let objects_base: Vec<ObjectBase> = Vec::new();
```

## Step 5 — the rename pass

Every use site re-paths mechanically. In your editor, scope the search to `src/` +
`examples/`, turn **Match Whole Word ON**, and run each row as a Replace-All. The hits
column is the exact count each row must report — a different number means a typo.

The prefix matters: plain `point_buffer` would also hit the constructor LOCALS, which
step 4 already consumed — that is why every row is anchored on `self.` (or `gpu.`).

| find (whole word) | replace with | hits |
|---|---|---|
| `self.mvp_buffer` | `self.frame.mvp_buffer` | 3 |
| `self.mvp_bind_group` | `self.frame.mvp_bind_group` | 8 |
| `self.line_buffer` | `self.frame.line_buffer` | 1 |
| `self.line_bind_group` | `self.frame.line_bind_group` | 7 |
| `self.time_buffer` | `self.frame.time_buffer` | 1 |
| `self.time_bind_group` | `self.frame.time_bind_group` | 1 |
| `self.time` | `self.frame.time` | 2 |
| `self.cloud_buffer` | `self.frame.cloud_buffer` | 3 |
| `self.cloud_bind_group` | `self.frame.cloud_bind_group` | 1 |
| `self.mvp_f32` | `self.frame.mvp_f32` | 4 |
| `self.last_ortho_h` | `self.frame.last_ortho_h` | 2 |
| `self.mvp_layout` | `self.layouts.mvp` | 1 |
| `self.time_layout` | `self.layouts.time` | 1 |
| `self.instance_layout` | `self.layouts.instance` | 2 |
| `self.line_layout` | `self.layouts.line` | 1 |
| `self.segment_layout` | `self.layouts.segment` | 2 |
| `self.glyph_layout` | `self.layouts.glyph` | 2 |
| `self.arena_vbo` | `self.arena.vbo` | 4 |
| `self.arena_vids` | `self.arena.vids` | 4 |
| `self.arena_ibo` | `self.arena.ibo` | 4 |
| `self.arena_index_count` | `self.arena.index_count` | 7 |
| `self.arena_vert_count` | `self.arena.vert_count` | 10 |
| `self.arena_vert_cap` | `self.arena.vert_cap` | 3 |
| `self.arena_index_cap` | `self.arena.index_cap` | 3 |
| `self.instances` | `self.objects.rows` | 15 |
| `self.instance_buffer` | `self.objects.buffer` | 7 |
| `self.instance_bind_group` | `self.objects.bind_group` | 8 |
| `self.objects_base` | `self.objects.base` | 3 |
| `self.base_f32` | `self.objects.base_f32` | 2 |
| `self.bounded_rows` | `self.objects.bounded_rows` | 3 |
| `self.object_bounds_world` | `self.objects.bounds_world` | 4 |
| `self.inside` | `self.objects.inside` | 4 |
| `self.last_origin` | `self.objects.last_origin` | 7 |
| `self.last_rebase_ms` | `self.objects.last_rebase_ms` | 2 |
| `self.segment_buffer` | `self.seg.buffer` | 4 |
| `self.segment_bind_group` | `self.seg.bind_group` | 4 |
| `self.segment_count` | `self.seg.count` | 7 |
| `self.pipe_count` | `self.seg.pipe_count` | 11 |
| `self.cyl_template_vbo` | `self.seg.template_vbo` | 1 |
| `self.cyl_template_ibo` | `self.seg.template_ibo` | 1 |
| `self.cyl_index_count` | `self.seg.template_index_count` | 1 |
| `self.glyph_buffer` | `self.glyphs.buffer` | 4 |
| `self.glyph_bind_group` | `self.glyphs.bind_group` | 4 |
| `self.glyph_count` | `self.glyphs.count` | 7 |
| `self.sphere_count` | `self.glyphs.sphere_count` | 10 |
| `self.sph_template_vbo` | `self.glyphs.template_vbo` | 1 |
| `self.sph_template_ibo` | `self.glyphs.template_ibo` | 1 |
| `self.sph_index_count` | `self.glyphs.template_index_count` | 2 |
| `self.point_buffer` | `self.cloud.pos` | 2 |
| `self.point_col_buffer` | `self.cloud.col` | 2 |
| `self.point_nrm_buffer` | `self.cloud.nrm` | 2 |
| `self.point_count` | `self.cloud.count` | 2 |
| `self.cloud_draws` | `self.cloud.draws` | 2 |
| `self.splat_depth_buf` | `self.splat.depth_buf` | 5 |
| `self.splat_color_buf` | `self.splat.color_buf` | 5 |
| `self.splat_recs` | `self.splat.recs` | 3 |
| `self.splat_group0_layout` | `self.splat.group0_layout` | 2 |
| `self.splat_group1_layout` | `self.splat.group1_layout` | 2 |
| `self.splat_resolve_layout` | `self.splat.resolve_layout` | 2 |
| `self.splat_group0_stream` | `self.stream.group0` | 3 |
| `self.splat_group1_stream` | `self.stream.group1` | 3 |
| `self.splat_group0` | `self.splat.group0` | 3 |
| `self.splat_group1` | `self.splat.group1` | 3 |
| `self.splat_resolve_group` | `self.splat.resolve_group` | 2 |
| `self.splat_depth_pipeline` | `self.splat.depth_pipeline` | 1 |
| `self.splat_color_pipeline` | `self.splat.color_pipeline` | 1 |
| `self.splat_total` | `self.splat.total` | 3 |
| `self.splat_state` | `self.splat.state` | 8 |
| `self.splat_stream_recs` | `self.stream.recs` | 3 |
| `self.stream_pos_buf` | `self.stream.pos` | 4 |
| `self.stream_col_buf` | `self.stream.col` | 4 |
| `self.stream_nrm_buf` | `self.stream.nrm` | 3 |
| `self.stream_capacity` | `self.stream.capacity` | 2 |
| `self.stream_count` | `self.stream.count` | 10 |
| `self.stream_pos_at` | `self.stream.pos_at` | 4 |
| `self.stream_col_at` | `self.stream.col_at` | 3 |
| `self.stream_draws` | `self.stream.draws` | 3 |
| `gpu.stream_draws` | `gpu.stream.draws` | 1 |

And the `ArenaUpload` side (`up.` in gpu/mod.rs, `t.`/`tables.` in scene.rs). Do **NOT**
touch the streaming METHOD calls `gpu.cloud_pos(...)`/`gpu.cloud_col(...)` — the rows
below are field reads, never followed by `(`:

| find (whole word) | replace with | hits |
|---|---|---|
| `up.cloud_pos` | `up.clouds.pos` | 2 |
| `up.cloud_col` | `up.clouds.col` | 1 |
| `up.cloud_nrm` | `up.clouds.nrm` | 1 |
| `up.cloud_draws` | `up.clouds.draws` | 1 |
| `t.cloud_pos` | `t.clouds.pos` | 5 |
| `t.cloud_col` | `t.clouds.col` | 1 |
| `t.cloud_nrm` | `t.clouds.nrm` | 1 |
| `t.cloud_draws` | `t.clouds.draws` | 2 |
| `tables.cloud_draws` | `tables.clouds.draws` | 1 |

## Step 6 — the tuple sites become named

The renames left exactly the places where a tuple was built or taken apart. Seventeen
hunks; the pattern trick `ObjectBase { model: xf, .. }` keeps each body byte-identical.

**6a-6i: `src/engine/gpu/mod.rs`.**

**Find**:

```rust
        self.objects.base_f32 = up.objects.iter().map(|(m, _, _)| m.to_f32()).collect();
```

**Replace with:**

```rust
        self.objects.base_f32 = up.objects.iter().map(|ob| ob.model.to_f32()).collect();
```

**Find**:

```rust
        self.objects.bounds_world = up.objects.iter().zip(&up.object_bounds).map(|((m, _, _), b)| {
```

**Replace with:**

```rust
        self.objects.bounds_world = up.objects.iter().zip(&up.object_bounds).map(|(ObjectBase { model: m, .. }, b)| {
```

**Find**:

```rust
.map(|(i, (m, c, f))| Instance {
            model: m.to_f32(),
            color: *c,
            flags: *f,
```

**Replace with:**

```rust
.map(|(i, ob)| Instance {
            model: ob.model.to_f32(),
            color: ob.color,
            flags: ob.flags,
```

**Find** (in `rebuild_instances`):

```rust
        for (i, (model, _, _)) in self.objects.base.iter().enumerate() {
```

**Replace with:**

```rust
        for (i, ObjectBase { model, .. }) in self.objects.base.iter().enumerate() {
```

**Find** (in `cloud_begin`):

```rust
        self.stream.draws.push((self.stream.count, count, instance, 0.0));
```

**Replace with:**

```rust
        self.stream.draws.push(CloudDraw { first: self.stream.count, count, instance, spacing: 0.0 });
```

**Find** (in `cloud_pos`):

```rust
            if d.3 == 0.0 && self.stream.pos_at == d.0 && pos.len() >= 6 {
```

**Replace with:**

```rust
            if d.spacing == 0.0 && self.stream.pos_at == d.first && pos.len() >= 6 {
```

**Find**:

```rust
                    d.3 = gaps[gaps.len() / 2];
```

**Replace with:**

```rust
                    d.spacing = gaps[gaps.len() / 2];
```

**Find**:

```rust
    fn splat_records(&self, draws: &[(u32, u32, u32, f32)]) -> ([u32; 4], Vec<u8>, u32) {
```

**Replace with:**

```rust
    fn splat_records(&self, draws: &[CloudDraw]) -> ([u32; 4], Vec<u8>, u32) {
```

**Find** (its loop — the copy-bindings keep the body untouched):

```rust
        for &(first, count, inst, spacing) in draws {
```

**Replace with:**

```rust
        for &CloudDraw { first, count, instance: inst, spacing } in draws {
```

**6j-6q: `src/app/scene.rs`.**

**Find**:

```rust
use crate::engine::gpu::{ArenaUpload, Instance, CylinderSegment, GlyphPoint};
```

**Replace with:**

```rust
use crate::engine::gpu::{ArenaUpload, CloudDraw, Instance, CylinderSegment, GlyphPoint, ObjectBase};
```

**Find** (in `rebuild`):

```rust
                d.2 = row;
```

**Replace with:**

```rust
                d.instance = row;
```

**Find** (in `begin_cloud`):

```rust
        self.tables.objects.push((place.clone(), [1.0; 4], 0));
```

**Replace with:**

```rust
        self.tables.objects.push(ObjectBase { model: place.clone(), color: [1.0; 4], flags: 0 });
```

**Find** (in `add_file`):

```rust
            t.objects.push((placed, [1.0; 4], flags));
```

**Replace with:**

```rust
            t.objects.push(ObjectBase { model: placed, color: [1.0; 4], flags });
```

**Replace-all** (3 hits — the flags writes):

```rust
t.objects.last_mut().unwrap().2 |=
```

**with:**

```rust
t.objects.last_mut().unwrap().flags |=
```

(and the comment above the first one drops its trailing `- .2 is flags`.)

**Find** (the cloud arm):

```rust
                    t.clouds.draws.push((first, pc.len() as u32, ri, cloud_spacing(pc)));
```

**Replace with:**

```rust
                    t.clouds.draws.push(CloudDraw { first, count: pc.len() as u32, instance: ri, spacing: cloud_spacing(pc) });
```

**Find** (the cloud bounds walk):

```rust
        for &(first, count, inst, _) in t.clouds.draws.iter().skip(draw0){
```

**Replace with:**

```rust
        for &CloudDraw { first, count, instance: inst, .. } in t.clouds.draws.iter().skip(draw0){
```

**Replace-all** (7 hits — every object-transform lookup; the field binding keeps `xf` and
the whole body identical):

```rust
Some((xf, _, _)) = t.objects.get
```

**with:**

```rust
Some(ObjectBase { model: xf, .. }) = t.objects.get
```

## The map, for every lesson after this one

| old flat field | new path |
|---|---|
| `mvp/line/time/cloud` buffers + groups, `time`, `mvp_f32`, `last_ortho_h` | `gpu.frame.*` |
| `*_layout` | `gpu.layouts.*` |
| `arena_*` | `gpu.arena.*` |
| `instances`, `instance_buffer/bind_group`, `objects_base`→`base`, `base_f32`, `bounded_rows`, `object_bounds_world`→`bounds_world`, `inside`, `last_origin`, `last_rebase_ms` | `gpu.objects.*` |
| `segment_*`, `pipe_count`, `cyl_*`→`template_*` | `gpu.seg.*` |
| `glyph_*`, `sphere_count`, `sph_*`→`template_*` | `gpu.glyphs.*` |
| `point_*`→`pos/col/nrm/count`, `cloud_draws`→`draws` | `gpu.cloud.*` |
| `splat_*` | `gpu.splat.*` |
| `stream_*`, `splat_*_stream`→`recs/group0/group1` | `gpu.stream.*` |
| `ArenaUpload.cloud_*` | `up.clouds.*` |
| `(Xform, [f32;4], u32)` object tuple | `ObjectBase { model, color, flags }` |
| `(u32, u32, u32, f32)` draw tuple | `CloudDraw { first, count, instance, spacing }` |

Still top-level on `Gpu`: `surface`, `device`, `queue`, `config`, `pipelines`, the three
user knobs (`line_style`, `cloud_size`, `edl_strength`), the render targets
(`depth_view`, `msaa_view`, `samples`), `performance`, `scene_min/max`.

## Expected state

- `cargo check --target x86_64-unknown-linux-gnu --all-targets`: clean.
- `cargo check --target wasm32-unknown-unknown --lib`: clean. Shaders untouched — no naga run needed.
- All three goldens PIXEL-IDENTICAL (run each twice; both passes must agree):

```
VIEWER_W=1200 VIEWER_H=800 VIEWER_ZOOM=6 VIEWER_ORBIT="25,-10" \
cargo run --example selftest --target x86_64-unknown-linux-gnu --release -- out.ppm assets/scenes/lion.toml
# => non-background pixels: 325369 (33.9%)

VIEWER_W=1600 VIEWER_H=700 VIEWER_ZOOM=3 \
cargo run --example selftest --target x86_64-unknown-linux-gnu --release -- out.ppm assets/scenes/cloud_mix.toml
# => non-background pixels: 12143 (1.1%)

VIEWER_W=1600 VIEWER_H=700 \
cargo run --example selftest --target x86_64-unknown-linux-gnu --release -- out.ppm assets/scenes/lidar14.toml
# => non-background pixels: 3798 (0.3%)
```

A refactor that changes any of those numbers changed behavior — go find the typo before
moving on. From [44](44-cloud-octree.md) onward, every lesson speaks the new paths.
