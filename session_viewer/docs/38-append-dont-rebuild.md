# 38 Append, don't rebuild — Mat4 rows and growable lanes

A scene loads file by file. Until now each new file made every GPU lane rebuild its whole
buffer, so the fifth file re-sent the first file's rows for the fifth time — and it could only
do that because the CPU-side table was still there to re-send from. Both copies go in this
lesson.

## Goal

Every lane grows and appends: only the new file's rows travel, the existing prefix is copied
GPU-side, and the CPU table is dropped as soon as it is uploaded.

Measured on the ten-sheet `drawings` scene, 2311 MB resident → 881 MB. On the 13.8 M-point
scan the CPU mirror alone was 263 MB.

Three other things ride along, because they are the same walk and the same frame:

- the object row stops being a kernel `Xform` — `Xform::identity()` heap-allocates twice, and
  a 90k-line sheet paid ~400k allocations to carry 128 bytes of numbers;
- the mesh walk asks the kernel for edges, adjacency, normals and closedness in ONE pass
  instead of four (123 ms of the bunny's 137 ms walk);
- a drawing sheet stops asking the depth buffer to order things it cannot order.

## Step 1 — `Mat4`, and one grow function for every lane

`append_rows` is the whole lesson in forty lines: double the capacity when it runs out, move
the prefix GPU-side, write only the new rows, and report whether the buffer moved so the caller
knows to rebuild the bind group pointing at it.

**Find** in `src/engine/gpu/mod.rs`:

```rust
use crate::engine::pipelines::Pipelines;
```

**Replace with:**

```rust
 

use crate::engine::pipelines::Pipelines;
 
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
/// No Mesh, no Session, no wgpu type on the app side of this line.
```

**Add below it:**

```rust
/// One object's world placement as the 16 raw column-major doubles the GPU row needs.
///
/// NOT a kernel `Xform`: that struct carries `typ`/`name` Strings and a guid `OnceLock`, so
/// `Xform::identity()` heap-allocates TWICE per call and every arena row cost two more on the
/// clone into `objects_base`. On a 90k-line sheet that was ~400k allocations - 300 ms of the
/// walk - to carry 128 bytes of numbers nothing downstream ever reads a name off.
pub type Mat4 = [f64; 16];

/// `a * b` in the kernel's convention: column-major, index = col * 4 + row.
/// Matches `impl Mul for &Xform` element for element - and allocates nothing.
pub fn mat_mul(a: &Mat4, b: &Mat4) -> Mat4 {
    let mut out = [0.0f64; 16];
    for i in 0..4 {
        for j in 0..4 {
            let mut sum = 0.0;
            for k in 0..4 {
                sum += a[k * 4 + i] * b[j * 4 + k];
            }
            out[j * 4 + i] = sum;
        }
    }
    out
}

/// The GPU edge: f64 world math stays CPU-side, the instance row is f32.
pub fn mat_to_f32(m: &Mat4) -> [f32; 16] {
    std::array::from_fn(|i| m[i] as f32)
}

/// Grow-and-append one index run. Same shape as the solid arena's own append: the existing
/// prefix is copied GPU-side, never back through wasm memory.
/// Append rows to a growable STORAGE buffer: double the capacity when it runs out, move the
/// prefix GPU-side, and write only the new rows. Returns `true` when the buffer was replaced, so
/// the caller knows to rebuild the bind group pointing at it.
///
/// This is the same deal the mesh arena already struck, extended to the lanes that had not taken
/// it: a lane that rebuilds its whole buffer per file re-sends every earlier file's rows (five
/// files means the last one travels once and the first one five times), and it can only do that
/// because the CPU-side table is still there to re-send FROM - so the rows are held twice, in
/// wasm memory and on the GPU, for the whole session. On a 13.8 M-point scan that second copy is
/// 280 MB of browser heap.
fn append_rows<T: bytemuck::Pod>(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    label: &str,
    buf: &mut wgpu::Buffer,
    count: &mut u32,
    cap: &mut u64,
    data: &[T],
) -> bool {
    if data.is_empty() {
        return false;
    }
    let stride = std::mem::size_of::<T>() as u64;
    let need = *count as u64 + data.len() as u64;
    let mut grew = false;
    if need > *cap {
        let new_cap = need.max(*cap * 2);
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let nb = zeroed_buffer(device, label, new_cap * stride, usage);
        if *count > 0 {
            // the prefix moves GPU-side; it never travels back through wasm memory
            let mut enc = device.create_command_encoder(&Default::default());
            enc.copy_buffer_to_buffer(buf, 0, &nb, 0, *count as u64 * stride);
            queue.submit([enc.finish()]);
        }
        *buf = nb;
        *cap = new_cap;
        grew = true;
    }
    queue.write_buffer(buf, *count as u64 * stride, bytemuck::cast_slice(data));
    *count += data.len() as u32;
    grew
}

fn append_index_run(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    label: &str,
    ibo: &mut wgpu::Buffer,
    count: &mut u32,
    cap: &mut u64,
    data: &[u32],
) {
    if data.is_empty() {
        return;
    }
    let need = *count as u64 + data.len() as u64;
    if need > *cap {
        let new_cap = need.max(*cap * 2);
        let iu = wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let nb = zeroed_buffer(device, label, new_cap * 4, iu);
        if *count > 0 {
            let mut enc = device.create_command_encoder(&Default::default());
            enc.copy_buffer_to_buffer(ibo, 0, &nb, 0, *count as u64 * 4);
            queue.submit([enc.finish()]);
        }
        *ibo = nb;
        *cap = new_cap;
    }
    queue.write_buffer(ibo, *count as u64 * 4, bytemuck::cast_slice(data));
    *count += data.len() as u32;
}

```


## Step 2 — every lane carries a capacity

A lane that grows needs to know how much room it has. Each of the six row tables gains a
`*_cap`, and `new()` reserves instead of building from data it does not have yet.

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub cloud_pos: Vec<f32>,  // Raw lane: 3 floats per point, 12 B
    pub cloud_col: Vec<u32>,  // Raw lane: RGBA8 per point, 4 B
    pub cloud_nrm: Vec<u32>,  // Raw lane: oct16 normal per point (u32::MAX = none), 4 B -> 20 B/pt
    pub cloud_draws: Vec<(u32, u32, u32, f32)>, // (first, count, instance, point spacing world units)
    pub objects: Vec<(Xform, [f32; 4], u32)>,
```

**Replace with:**

```rust
    pub cloud_pos: Vec<f32>, // Raw lane: 3 floats per point, 12 B
    pub cloud_col: Vec<u32>, // Raw lane: RBGA8 per point, 4 B
    pub cloud_nrm: Vec<u32>, // Raw lane: oct16 normal per point (u32::MAX = none), 4 B -> 20 B/pt
    pub cloud_draws: Vec<(u32, u32, u32, f32)>, // first, count, instance, point spacing world units
    /// Sheet lanes. A PDF's fills are exactly coplanar, so they must NOT arbitrate by depth -
    /// they are split off the solid index run and drawn in document order with depth write off.
    /// `idx_text` is the lettering, drawn LAST of all, after the ink lanes, because a page puts
    /// its text on top of both its hatching and its linework.
    pub idx_print: Vec<u32>,
    pub idx_text: Vec<u32>,
    pub objects: Vec<(Mat4, [f32; 4], u32)>,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            cloud_nrm: Vec::new(),
            cloud_draws: Vec::new(),
```

**Add below it:**

```rust
            idx_print: Vec::new(),
            idx_text: Vec::new(),
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub arena_index_count: u32,
```

**Add below it:**

```rust
    // The two sheet index runs, appended exactly like the solid one.
    arena_ibo_print: wgpu::Buffer,
    arena_print_count: u32,
    arena_print_cap: u64,
    arena_ibo_text: wgpu::Buffer,
    arena_text_count: u32,
    arena_text_cap: u64,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    objects_base: Vec<(Xform, [f32; 4], u32)>, // TRUE world model+color; isntance[] is rebased from this
```

**Replace with:**

```rust
    objects_base: Vec<(Mat4, [f32; 4], u32)>, // TRUE world model+color; isntance[] is rebased from this
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    instance_buffer: wgpu::Buffer, // new() builds this storage buffer as a local and drops it, only the bidn group survives; rebuild_instances() reuploads into it every frame, so the buffer handle itself must live on GPU, not vanish atht eh of new()
```

**Add below it:**

```rust
    instance_rows: u32, // instance rows already ON the GPU - the base for the next append
    instance_cap: u64,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub segment_buffer: wgpu::Buffer,
    pub segment_bind_group: wgpu::BindGroup,
    pub segment_count: u32,
    pub pipe_count: u32,  // segments[0..pipe_count] are the SOLID lane, the rest are ribbons
    pub sph_template_vbo: wgpu::Buffer,
    pub sph_template_ibo: wgpu::Buffer,
    pub sph_index_count: u32,
    pub glyph_buffer: wgpu::Buffer,
    pub glyph_bind_group: wgpu::BindGroup,
    pub glyph_count: u32,
    pub sphere_count: u32, // glyphs[0..sphere_count] are the SOLID lane, the rest are flat dots
    pub point_buffer: wgpu::Buffer,     // positions, array<f32>
    pub point_col_buffer: wgpu::Buffer, // colours, array<u32> RGBA8
    pub point_nrm_buffer: wgpu::Buffer, // normals, array<u32> oct16 (u32::MAX = none)
    // compute splatting for the cloud lane
    splat_depth_buf: wgpu::Buffer,    // one u32 per pixel: winning reverse-Z bits (0 = empty)
    splat_color_buf: wgpu::Buffer,    // one u32 per pixel: winner's RGBA8
    splat_recs: wgpu::Buffer,         // header + one Rec per cloud, written per frame
```

**Replace with:**

```rust
    /// The SOLID lane (mesh/BRep edges -> cylinders) and the FLAT lane (line/polyline ->
    /// ribbons) used to share one buffer, solid rows first. One buffer meant one splice point,
    /// and a splice point moves whenever either half grows - so appending a file was impossible
    /// and every upload rebuilt the whole table from the CPU copy. Two buffers, same layout and
    /// same shader (each lane indexes from row 0), and both grow by appending.
    pub pipe_buffer: wgpu::Buffer,
    pub pipe_bind_group: wgpu::BindGroup,
    pub pipe_count: u32,
    pub pipe_cap: u64,
    pub segment_buffer: wgpu::Buffer,
    pub segment_bind_group: wgpu::BindGroup,
    pub segment_count: u32,
    pub segment_cap: u64,
    pub sph_template_vbo: wgpu::Buffer,
    pub sph_template_ibo: wgpu::Buffer,
    pub sph_index_count: u32,
    /// Vertex ink, split the same way: spheres are mesh/BRep vertices, glyphs are flat dots.
    pub sphere_buffer: wgpu::Buffer,
    pub sphere_bind_group: wgpu::BindGroup,
    pub sphere_count: u32,
    pub sphere_cap: u64,
    pub glyph_buffer: wgpu::Buffer,
    pub glyph_bind_group: wgpu::BindGroup,
    pub glyph_count: u32,
    pub glyph_cap: u64,
    pub point_buffer: wgpu::Buffer, // positions, array<f32>
    pub point_col_buffer: wgpu::Buffer, // colours, array<u32> RGBA8
    pub point_nrm_buffer: wgpu::Buffer, // normals, array<u32> oct16 (u32::MAX = none)
    pub point_cap: u64,     // capacity in POINTS; positions hold 3 floats each
    pub point_col_cap: u64,
    pub point_nrm_cap: u64,
    splat_depth_buf: wgpu::Buffer, // one u32 per pixel: winning reverse-Z bits (0 = empty)
    splat_color_buf: wgpu::Buffer, // one u32 per pixel: winner's RBGA8
    splat_recs: wgpu::Buffer,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let instance_buffer =  storage_buffer(&device, "instance.buffer", &instances);
        let objects_base: Vec<(Xform, [f32; 4], u32)> = Vec::new();
        let (pipe_count, segment_count, sphere_count, glyph_count) = (0u32, 0u32, 0u32, 0u32);
        let arena_index_count = 0u32;
```

**Replace with:**

```rust
        // COPY_SRC because the table GROWS by appending: when it outgrows its buffer the prefix
        // is copied GPU-side into the bigger one, and a buffer without COPY_SRC cannot be the
        // source of that copy.
        let instance_buffer = zeroed_buffer(
            &device, 
            "instance.buffer",
            std::mem::size_of::<Instance>() as u64,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC);
        let objects_base: Vec<(Mat4, [f32; 4], u32)> = Vec::new();
        let (pipe_count, segment_count, sphere_count, glyph_count) = (0u32, 0u32, 0u32, 0u32);
        let arena_index_count = 0u32;
        let iu_sheet = wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let arena_ibo_print = zeroed_buffer(&device, "arena.ibo.print", 4, iu_sheet);
        let arena_ibo_text = zeroed_buffer(&device, "arena.ibo.text", 4, iu_sheet);
        let (arena_print_count, arena_print_cap) = (0u32, 1u64);
        let (arena_text_count, arena_text_cap) = (0u32, 1u64);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        // One storage row per edge (VERTEX-visible, read-only) - the segment table.
        let segment_buffer =  zeroed_buffer(
            &device, "segments.buffer", 
            std::mem::size_of::<CylinderSegment>() as u64, 
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
```

**Replace with:**

```rust
        // One storage row per edge (VERTEX-visible, read-only) - the two segment tables. Both
        // start at one row and grow by appending; COPY_SRC lets a grown buffer take the old
        // prefix straight from the old one without a round trip through wasm memory.
        let pipe_cap = 1u64;
        let segment_cap = 1u64;
        let pipe_buffer = zeroed_buffer(
            &device, "pipes.buffer",
            std::mem::size_of::<CylinderSegment>() as u64,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC);
        let segment_buffer =  zeroed_buffer(
            &device, "segments.buffer", 
            std::mem::size_of::<CylinderSegment>() as u64, 
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let segment_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("segments.bind_group"),
            layout: &segment_layout,
            entries: &[wgpu::BindGroupEntry{
                binding: 0,
                resource: segment_buffer.as_entire_binding() 
            }]
        });
```

**Replace with:**

```rust
        let pipe_bind_group = Self::mk_rows_group(&device, &segment_layout, "pipes.bind_group", &pipe_buffer);
        let segment_bind_group = Self::mk_rows_group(&device, &segment_layout, "segments.bind_group", &segment_buffer);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            contents: bytemuck::cast_slice(&sph_i),
            usage: wgpu::BufferUsages::INDEX,
        });
```

**Add below it:**

```rust
        let sphere_cap = 1u64;
        let glyph_cap = 1u64;
        let sphere_buffer = zeroed_buffer(
            &device,
            "spheres.buffer",
            std::mem::size_of::<GlyphPoint>() as u64,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            std::mem::size_of::<GlyphPoint>() as u64,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        let glyph_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
```

**Replace with:**

```rust
            std::mem::size_of::<GlyphPoint>() as u64,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC);
        let glyph_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let glyph_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("glyphs.bind_group"),
            layout: &glyph_layout,
            entries: &[wgpu::BindGroupEntry{
                binding: 0, 
                resource: glyph_buffer.as_entire_binding()
            }],
        });
```

**Replace with:**

```rust
        let sphere_bind_group = Self::mk_rows_group(&device, &glyph_layout, "spheres.bind_group", &sphere_buffer);
        let glyph_bind_group = Self::mk_rows_group(&device, &glyph_layout, "glyphs.bind_group", &glyph_buffer);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        // Point cloud tables - empty until set_scene fills them from ArenaUpload
        let point_count = 0u32;
        let point_buffer = storage_buffer(&device, "points.buffer", &[0f32]);
        let point_col_buffer = storage_buffer(&device, "points.col.buffer", &[0u32]);
        let point_nrm_buffer = storage_buffer(&device, "points.nrm.buffer", &[u32::MAX]);
```

**Replace with:**

```rust
        // Point cloud tables - empty until set_scene fill them from ArenaUpload
        let point_count = 0u32;
        let (point_cap, point_col_cap, point_nrm_cap) = (3u64, 1u64, 1u64);
        let point_buffer = zeroed_buffer(&device, "points.buffer", 12, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC);
        let point_col_buffer = zeroed_buffer(&device, "points.col.buffer", 4, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC);
        let point_nrm_buffer = zeroed_buffer(&device, "points.nrm.buffer", 4, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        // The per-pixel buffers are framebuffer-sized u32s; clear_buffer needs COPY_DST.
        let pixels = (config.width.max(1) * config.height.max(1)) as u64 * 4;
        let splat_depth_buf = zeroed_buffer(&device, "splat.depth", pixels,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        let splat_color_buf = zeroed_buffer(&device, "splat.color", pixels,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        let splat_recs = zeroed_buffer(&device, "splat.recs", 16 + 256 * 144,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
```

**Replace with:**

```rust
        // the per-pixel buffers are framebuffer-sized u32s;
        // clear_buffer COPY_DST
        let pixels = (config.width.max(1) * config.height.max(1)) as u64 * 4;
        let splat_depth_buf = zeroed_buffer(&device, "splat.depth", pixels, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        let splat_color_buf = zeroed_buffer(&device, "splat.color", pixels, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        let splat_recs = zeroed_buffer(&device, "splat.rescales", 16 + 256 * 144, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        // ── TWIN TRAP ── group0/group1 machinery comes in near-identical pairs (layouts,
        // mk_ helpers, calls). The three bugs you will most likely type are all unedited
        // copies of the FIRST twin: group0's Uniform entries left in group1's layout, the
        // second mk_ helper never renamed, rebuild calling mk_splat_group0 for group1.
        // Symptom for all of them: a wgpu VALIDATION ERROR naming the exact label — and the
        // frame silently shows the LAST GOOD image (or 100% painted in the headless
        // harness), because an invalid bind group invalidates the whole submit. Read the
        // console before touching the math.
        let splat_group1_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("splat.group1.layout"),
            entries: &[
                Self::splat_entry(0, wgpu::BufferBindingType::Storage { read_only: true }),
                Self::splat_entry(1, wgpu::BufferBindingType::Storage { read_only: true }),
                Self::splat_entry(2, wgpu::BufferBindingType::Storage { read_only: false }),
                Self::splat_entry(3, wgpu::BufferBindingType::Storage { read_only: false }),
            ],
        });
        let splat_resolve_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("splat.resolve.layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry{
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None },
```

**Replace with:**

```rust
        let splat_group1_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("splat.group1.layout"),
            entries: &[
                Self::splat_entry(0, wgpu::BufferBindingType::Storage { read_only: true }), // pos
                Self::splat_entry(1, wgpu::BufferBindingType::Storage { read_only: true }), // col
                Self::splat_entry(2, wgpu::BufferBindingType::Storage { read_only: false }), // sdepth
                Self::splat_entry(3, wgpu::BufferBindingType::Storage { read_only: false }), // scolor
            ],
        });

        let splat_resolve_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("splat.resolve.layout"),
            entries: & [
                wgpu::BindGroupLayoutEntry{
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None },
```

**Replace with:**

```rust
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Storage { read_only: true }, 
                        has_dynamic_offset: false, 
                        min_binding_size: None 
                    },
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let splat_group0 = Self::mk_splat_group0(&device, &splat_group0_layout, &mvp_buffer, &cloud_buffer, &instance_buffer, &splat_recs);
        let splat_group1 = Self::mk_splat_group1(&device, &splat_group1_layout, &point_buffer, &point_col_buffer, &splat_depth_buf, &splat_color_buf);
        let splat_resolve_group = Self::mk_splat_resolve_group(&device, &splat_resolve_layout, &splat_depth_buf, &splat_color_buf);
```

**Replace with:**

```rust

        let splat_group0 = Self::mk_splat_group0(
            &device, 
            &splat_group0_layout,
            &mvp_buffer,
            &cloud_buffer,
            &instance_buffer,
            &splat_recs
        );

        let splat_group1 = Self::mk_splat_group1(
            &device, 
            &splat_group1_layout,
            &point_buffer,
            &point_col_buffer,
            &splat_depth_buf,
            &splat_color_buf,
        );

        let splat_resolve_group = Self::mk_splat_resolve_group(
            &device,
            &splat_resolve_layout,
            &splat_depth_buf,
            &splat_color_buf,
        );

```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/splat.wgsl").into()),
        });
```

**Add below it:**

```rust

```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            immediate_size: 0,
        });
```

**Add below it:**

```rust

```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let splat_color_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor{
```

**Replace with:**

```rust

         let splat_color_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor{
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            entry_point: Some("cs_color"),
            compilation_options: Default::default(),
            cache: None,
        });
```

**Add below it:**

```rust

```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            arena_index_count,
```

**Add below it:**

```rust
            arena_ibo_print,
            arena_print_count,
            arena_print_cap,
            arena_ibo_text,
            arena_text_count,
            arena_text_cap,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            instance_buffer, // was a dropped local in new(), now moved onto GPU so rebuild_instances() can write into every frame
```

**Add below it:**

```rust
            instance_rows: 0,
            instance_cap: 1,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            segment_buffer,
            segment_bind_group,
            segment_count,
            pipe_count,
            sph_template_vbo,
            sph_template_ibo,
            sph_index_count,
            glyph_buffer,
            glyph_bind_group,
            glyph_count,
            sphere_count,
            point_buffer,
            point_col_buffer,
            point_nrm_buffer,
```

**Replace with:**

```rust
            pipe_buffer,
            pipe_bind_group,
            pipe_count,
            pipe_cap,
            segment_buffer,
            segment_bind_group,
            segment_count,
            segment_cap,
            sph_template_vbo,
            sph_template_ibo,
            sph_index_count,
            sphere_buffer,
            sphere_bind_group,
            sphere_count,
            sphere_cap,
            glyph_buffer,
            glyph_bind_group,
            glyph_count,
            glyph_cap,
            point_buffer,
            point_col_buffer,
            point_nrm_buffer,
            point_cap,
            point_col_cap,
            point_nrm_cap,
```


## Step 3 — `set_scene` appends

This is the payoff. Every `= up.something.clone()` becomes an `append_rows` over this file's
slice, and the bind group is rebuilt only when the buffer behind it actually moved.

**Find** in `src/engine/gpu/mod.rs`:

```rust
        // Instance rows: rebuilt from the true transforms (rebase stete, must live CPU-side).
        self.objects_base = up.objects.clone();
        debug_assert_eq!(up.objects.len(), up.object_bounds.len());
        self.object_bounds_world = up.objects.iter().zip(&up.object_bounds).map(|((m, _, _), b)| {
```

**Replace with:**

```rust
        // Instance rows: rebuilt from the true transforms (rebase state, must live CPU-side).
        //
        // `up.objects` is the ONE table the walk keeps cumulative - the bounds sweep and the
        // per-file sheet pass both index it by global row - so this is the one lane that gets a
        // full table every time instead of a delta. Only the NEW rows are turned into instances
        // and sent: cloning 148k rows per file was 22 MB of memcpy and a full re-upload, for a
        // tail that had not changed since the file before.
        let base = self.objects_base.len();
        if base == 0 {
            // First upload, or a rebuild that rewound everything: start the GPU table over too,
            // which also drops the one-row placeholder an empty scene leaves behind.
            self.instances.clear();
            self.instance_rows = 0;
        }
        debug_assert_eq!(up.objects.len(), up.object_bounds.len());
        debug_assert!(up.objects.len() >= base, "the object table only ever grows");
        self.objects_base.extend_from_slice(&up.objects[base..]);
        self.object_bounds_world.extend(up.objects[base..].iter().zip(&up.object_bounds[base..]).map(|((m, _, _), b)| {
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
                    m.m[0] * x + m.m[4] * y + m.m[8] * z + m.m[12],
                    m.m[1] * x + m.m[5] * y + m.m[9] * z + m.m[13],
                    m.m[2] * x + m.m[6] * y + m.m[10] * z + m.m[14],
```

**Replace with:**

```rust
                    m[0] * x + m[4] * y + m[8] * z + m[12],
                    m[1] * x + m[5] * y + m[9] * z + m[13],
                    m[2] * x + m[6] * y + m[10] * z + m[14],
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        }).collect();
        self.inside = vec![false; up.objects.len()];
        self.instances.clear();
        // `object_bounds_world` was just rebuilt above, so each row's extent comes from the same
        // AABB FLAG_INSIDE uses. The diagonal, not an axis: a flat sheet has a zero-thickness axis
        // and would clamp its ink lift to nothing.
        self.instances.extend(up.objects.iter().enumerate().map(|(i, (m, c, f))| Instance {
            model: m.to_f32(),
            color: *c,
            flags: *f,
            extent: self.object_bounds_world.get(i).and_then(|b| *b).map_or(0.0, |(lo, hi)| {
                ((hi[0] - lo[0]).powi(2) + (hi[1] - lo[1]).powi(2) + (hi[2] - lo[2]).powi(2)).sqrt() as f32
            }),
            spacing: up.object_spacing.get(i).copied().unwrap_or(0.0),
```

**Replace with:**

```rust
        }));
        self.inside.resize(self.objects_base.len(), false);
        // `object_bounds_world` was just extended above, so each row's extent comes from the same
        // AABB FLAG_INSIDE uses. The diagonal, not an axis: a flat sheet has a zero-thickness axis
        // and would clamp its ink lift to nothing.
        let bounds = &self.object_bounds_world;
        self.instances.extend(up.objects[base..].iter().enumerate().map(|(i, (m, c, f))| Instance {
            model: mat_to_f32(m),
            color: *c,
            flags: *f,
            extent: bounds.get(base + i).and_then(|b| *b).map_or(0.0, |(lo, hi)| {
                ((hi[0] - lo[0]).powi(2) + (hi[1] - lo[1]).powi(2) + (hi[2] - lo[2]).powi(2)).sqrt() as f32
            }),
            spacing: up.object_spacing.get(base + i).copied().unwrap_or(0.0),
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.instance_buffer = storage_buffer(&self.device, "instance.buffer", &self.instances);
        self.instance_bind_group = self.device.create_bind_group( &wgpu::BindGroupDescriptor{
            label: Some("instances.bind_group"),
            layout: &self.instance_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: self.instance_buffer.as_entire_binding()
            }],
        });
```

**Replace with:**

```rust
        let mut rows = self.instance_rows;
        let fresh = &self.instances[rows as usize..];
        if append_rows(&self.device, &self.queue, "instance.buffer",
            &mut self.instance_buffer, &mut rows, &mut self.instance_cap, fresh) {
            self.instance_bind_group = Self::mk_rows_group(&self.device, &self.instance_layout, "instances.bind_group", &self.instance_buffer);
        }
        self.instance_rows = rows;
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        }

        // The two lane tables: one buffer each, solid rows firstm spliced by two writes.
        self.pipe_count = up.pipes.len() as u32;
        self.segment_count = (up.pipes.len() + up.segments.len()) as u32;
        let rows = (self.segment_count as u64).max(1);
        self.segment_buffer = zeroed_buffer(
            &self.device, "segments.buffer", 
            rows * std::mem::size_of::<CylinderSegment>() as u64, 
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        self.queue.write_buffer(
            &self.segment_buffer, 
            0, 
            bytemuck::cast_slice(&up.pipes));
        self.queue.write_buffer(
            &self.segment_buffer, 
            up.pipes.len() as u64 * std::mem::size_of::<CylinderSegment>() as u64, 
            bytemuck::cast_slice(&up.segments));
        self.segment_bind_group = self.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("segments.bind_group"),
                layout: &self.segment_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.segment_buffer.as_entire_binding()
                }],
        });

        self.sphere_count = up.spheres.len() as u32;
        self.glyph_count = (up.spheres.len() + up.glyphs.len()) as u32;
        let rows = (self.glyph_count as u64).max(1);
        self.glyph_buffer = zeroed_buffer(
            &self.device,
            "glyphs.buffer",
            rows * std::mem::size_of::<GlyphPoint>() as u64,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST
        );
        self.queue.write_buffer(
            &self.glyph_buffer,
            0,
            bytemuck::cast_slice(&up.spheres),
        );
        self.queue.write_buffer(
            &self.glyph_buffer,
            up.spheres.len() as u64 * std::mem::size_of::<GlyphPoint>() as u64,
            bytemuck::cast_slice(&up.glyphs),
        );
        self.glyph_bind_group = self.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("glyphs.bind_group"),
                layout: &self.glyph_layout,
                entries: &[wgpu::BindGroupEntry{
                    binding: 0,
                    resource: self.glyph_buffer.as_entire_binding()
                }],
        });

        // Raw cloud lane: one row per scanned point, uploaded like any other table.
        self.cloud_draws = up.cloud_draws.clone();
        self.point_count = (up.cloud_pos.len() / 3) as u32;
        self.point_buffer = storage_buffer(&self.device, "points.buffer", &up.cloud_pos);
        self.point_col_buffer = storage_buffer(&self.device, "points.col.buffer", &up.cloud_col);
        self.point_nrm_buffer = storage_buffer(&self.device, "points.nrm.buffer", &up.cloud_nrm);
        self.rebuild_splat_groups(); // instance + point buffers are fresh
```

**Replace with:**

```rust

            // The sheet runs grow and append the same way; they index the SAME vertex table, so
            // splitting them costs one buffer each and no duplicated geometry.
            append_index_run(&self.device, &self.queue, "arena.ibo.print",
                &mut self.arena_ibo_print, &mut self.arena_print_count, &mut self.arena_print_cap, &up.idx_print);
            append_index_run(&self.device, &self.queue, "arena.ibo.text",
                &mut self.arena_ibo_text, &mut self.arena_text_count, &mut self.arena_text_cap, &up.idx_text);
        }

        // The four ink lanes, each a DELTA like the mesh arena: only this file's rows travel,
        // and the bind group is rebuilt only when the buffer behind it actually grew.
        if append_rows(&self.device, &self.queue, "pipes.buffer",
            &mut self.pipe_buffer, &mut self.pipe_count, &mut self.pipe_cap, &up.pipes) {
            self.pipe_bind_group = Self::mk_rows_group(&self.device, &self.segment_layout, "pipes.bind_group", &self.pipe_buffer);
        }
        if append_rows(&self.device, &self.queue, "segments.buffer",
            &mut self.segment_buffer, &mut self.segment_count, &mut self.segment_cap, &up.segments) {
            self.segment_bind_group = Self::mk_rows_group(&self.device, &self.segment_layout, "segments.bind_group", &self.segment_buffer);
        }
        if append_rows(&self.device, &self.queue, "spheres.buffer",
            &mut self.sphere_buffer, &mut self.sphere_count, &mut self.sphere_cap, &up.spheres) {
            self.sphere_bind_group = Self::mk_rows_group(&self.device, &self.glyph_layout, "spheres.bind_group", &self.sphere_buffer);
        }
        if append_rows(&self.device, &self.queue, "glyphs.buffer",
            &mut self.glyph_buffer, &mut self.glyph_count, &mut self.glyph_cap, &up.glyphs) {
            self.glyph_bind_group = Self::mk_rows_group(&self.device, &self.glyph_layout, "glyphs.bind_group", &self.glyph_buffer);
        }

        // Raw cloud lane, same deal. `cloud_draws` carries each cloud's absolute first-point
        // offset, which `Scene` keeps running across files - so the draw records append too.
        let mut pos_rows = self.point_count * 3;
        append_rows(&self.device, &self.queue, "points.buffer",
            &mut self.point_buffer, &mut pos_rows, &mut self.point_cap, &up.cloud_pos);
        let mut col_rows = self.point_count;
        append_rows(&self.device, &self.queue, "points.col.buffer",
            &mut self.point_col_buffer, &mut col_rows, &mut self.point_col_cap, &up.cloud_col);
        let mut nrm_rows = self.point_count;
        append_rows(&self.device, &self.queue, "points.nrm.buffer",
            &mut self.point_nrm_buffer, &mut nrm_rows, &mut self.point_nrm_cap, &up.cloud_nrm);
        self.point_count = pos_rows / 3;
        self.cloud_draws.extend_from_slice(&up.cloud_draws);
        self.rebuild_splat_groups();


```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            self.instances.len(), self.arena_vert_count, self.segment_count, self.pipe_count, self.glyph_count, self.sphere_count, self.point_count
```

**Replace with:**

```rust
            self.instances.len(), self.arena_vert_count, self.pipe_count + self.segment_count, self.pipe_count,
            self.sphere_count + self.glyph_count, self.sphere_count, self.point_count
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
                &self.splat_resolve_layout
```

**Replace with:**

```rust
                &self.splat_resolve_layout,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            let mut m = model.to_f32();
            m[12] = (model.m[12] - origin[0]) as f32;
            m[13] = (model.m[13] - origin[1]) as f32;
            m[14] = (model.m[14] - origin[2]) as f32;
```

**Replace with:**

```rust
            let mut m = mat_to_f32(model);
            m[12] = (model[12] - origin[0]) as f32;
            m[13] = (model[13] - origin[1]) as f32;
            m[14] = (model[14] - origin[2]) as f32;
```


## Step 4 — the bind groups get named builders

Six lanes rebuilding bind groups inline is six chances to pass the wrong buffer. One builder
per group shape, called from both `new()` and `set_scene`.

**Find** in `src/engine/gpu/mod.rs`:

```rust
    // rebuilt whenever any bound buffer is recreated (set_scene, resize).
    fn splat_entry(binding: u32, ty: wgpu::BufferBindingType) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry{
            binding,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer { ty, has_dynamic_offset: false, min_binding_size: None },
            count: None,
        }
    }
    fn mk_splat_group0(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, mvp: &wgpu::Buffer, cloud: &wgpu::Buffer, instances: &wgpu::Buffer, recs: &wgpu::Buffer) -> wgpu::BindGroup {
```

**Replace with:**

```rust
    // rebuilt whenever any bound buffer is recreated (set_scene, resize)
    fn splat_entry(
        binding: u32, 
        ty: wgpu::BufferBindingType) -> wgpu::BindGroupLayoutEntry{
        wgpu::BindGroupLayoutEntry { 
            binding, 
            visibility: wgpu::ShaderStages::COMPUTE, 
            ty: wgpu::BindingType::Buffer { ty, has_dynamic_offset: false, min_binding_size: None }, 
            count: None }
    }

    fn mk_splat_group0(
        device: &wgpu::Device, 
        layout: &wgpu::BindGroupLayout, 
        mvp: &wgpu::Buffer, 
        cloud: &wgpu::Buffer,
        instances: &wgpu::Buffer,
        recs: &wgpu::Buffer
    ) -> wgpu::BindGroup {
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    fn mk_splat_group1(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, pos: &wgpu::Buffer, col: &wgpu::Buffer, sdepth: &wgpu::Buffer, scolor: &wgpu::Buffer) -> wgpu::BindGroup {
```

**Replace with:**

```rust

    fn mk_splat_group1(
        device: &wgpu::Device, 
        layout: &wgpu::BindGroupLayout, 
        pos: &wgpu::Buffer,
        col: &wgpu::Buffer,
        sdepth: &wgpu::Buffer,
        scolor: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    fn mk_splat_resolve_group(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, sdepth: &wgpu::Buffer, scolor: &wgpu::Buffer) -> wgpu::BindGroup {
```

**Replace with:**

```rust

    fn mk_splat_resolve_group(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        sdepth: &wgpu::Buffer,
        scolor: &wgpu::Buffer,
    ) -> wgpu::BindGroup{
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    fn rebuild_splat_groups(&mut self) {
        self.splat_group0 = Self::mk_splat_group0(&self.device, &self.splat_group0_layout, &self.mvp_buffer, &self.cloud_buffer, &self.instance_buffer, &self.splat_recs);
        self.splat_group1 = Self::mk_splat_group1(&self.device, &self.splat_group1_layout, &self.point_buffer, &self.point_col_buffer, &self.splat_depth_buf, &self.splat_color_buf);
        self.splat_resolve_group = Self::mk_splat_resolve_group(&self.device, &self.splat_resolve_layout, &self.splat_depth_buf, &self.splat_color_buf);
    }
```

**Replace with:**

```rust

    /// One read-only storage buffer at binding 0 - the shape every ink lane's bind group has.
    fn mk_rows_group(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, label: &str, buf: &wgpu::Buffer) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(label),
            layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: buf.as_entire_binding() }],
        })
    }

    fn rebuild_splat_groups(&mut self){
        self.splat_group0 = Self::mk_splat_group0(&self.device, &self.splat_group0_layout, &self.mvp_buffer, &self.cloud_buffer, &self.instance_buffer, &self.splat_recs);
        self.splat_group1 = Self::mk_splat_group1(&self.device, &self.splat_group1_layout, &self.point_buffer, &self.point_col_buffer, &self.splat_depth_buf, &self.splat_color_buf);
        self.splat_resolve_group = Self::mk_splat_resolve_group(&self.device, &self.splat_resolve_layout, &self.splat_depth_buf, &self.splat_color_buf);

    }

```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            self.splat_depth_buf = zeroed_buffer(&self.device, "splat.depth", pixels,
                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
            self.splat_color_buf = zeroed_buffer(&self.device, "splat.color", pixels,
                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
            self.rebuild_splat_groups();
```

**Replace with:**

```rust
            self.splat_depth_buf = zeroed_buffer(&self.device, "splat.depth", pixels, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
            self.splat_color_buf = zeroed_buffer(&self.device, "splat.color", pixels, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
            self.rebuild_splat_groups();

```


## Step 5 — the draw split

The solid and flat halves of the segment and glyph tables are now separated by a count rather
than by living in different buffers, so each lane is one draw over a row range.

**Find** in `src/engine/gpu/mod.rs`:

```rust
        // Splat the clouds by COMPUTE before the render pass. One thread per point,
        // twice (depth race, then colour claim); the render pass composites the result
        // with one fullscreen triangle.
```

**Replace with:**

```rust

        // Splat the clouds by compute before the render pass.
        // One thread per point, twice (depth race, then colour claim);
        // the rende rpass composites the result with one fullscreen triangle
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            // Linework, ONE draw per lane over the SAME segment table.
            // segments[0..pipe_count] = mesh/BRep edges -> real cylinders: the tube radius lifts
            // the ink off the surface it sits on, so silhouette edges never lose the depth test.
            // segments[pipe_count..] = line/polyline -> flat ribbons: nothing to fight with, and
            // they stay screen-constant and cheap.
```

**Replace with:**

```rust
            // SHEET FILLS, second. Same vertex table, depth WRITE off, so a page's exactly
            // coplanar regions composite in document order instead of flickering over one shared
            // depth value. They still depth-TEST, so 3D geometry in front of the sheet occludes.
            if self.arena_print_count > 0 {
                pass.set_pipeline(&self.pipelines.triangle_sheet);
                pass.set_vertex_buffer(0, self.arena_vbo.slice(..));
                pass.set_vertex_buffer(1, self.arena_vids.slice(..));
                pass.set_index_buffer(self.arena_ibo_print.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..self.arena_print_count, 0, 0..1);
                draws += 1;
            }

            // Linework, ONE draw per lane, each over its OWN table.
            // pipes = mesh/BRep edges -> real cylinders: the tube radius lifts the ink off the
            // surface it sits on, so silhouette edges never lose the depth test.
            // segments = line/polyline -> flat ribbons: nothing to fight with, and they stay
            // screen-constant and cheap.
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.segment_bind_group, &[]);
                match self.line_style {
```

**Replace with:**

```rust
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.pipe_bind_group, &[]);
                match self.line_style {
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
                    // The flat lane's own shader over the SOLID half of the same table. vid/6
                    // picks the row, so the range is simply the pipes prefix. DEPTH PREPASS
```

**Replace with:**

```rust
                    // The flat lane's own shader over the SOLID table. DEPTH PREPASS
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            // The cloud lane, drawn WITH THE SOLIDS: the compute splatter already resolved
            // every cloud into the per-pixel depth/colour buffers, so the whole lane is ONE
            // fullscreen triangle that composites them - depth-writing via frag_depth, so
            // splats and solids occlude each other exactly.
```

**Replace with:**

```rust
            // The cloud lane. drawn with the solids: the compute splatter already resovled
            // every cloud into the per-pixel depth/color buffers, so the whoel lane is one fullscreen triangle
            // that composites them - depth-writing via frag_depth, so splat and solids occlude each other exactly.
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
                pass.draw(0..3, 0..1);
                draws += 1;
            }
```

**Add below it:**

```rust

```

**Find** in `src/engine/gpu/mod.rs`:

```rust
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.glyph_bind_group, &[]);
                pass.set_vertex_buffer(0, self.sph_template_vbo.slice(..));
```

**Replace with:**

```rust
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.sphere_bind_group, &[]);
                pass.set_vertex_buffer(0, self.sph_template_vbo.slice(..));
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            if INK_DEPTH_PREPASS && self.segment_count > self.pipe_count {
```

**Replace with:**

```rust
            if INK_DEPTH_PREPASS && self.segment_count > 0 {
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
                pass.draw(0..4, self.pipe_count..self.segment_count);
                draws += 1;
            }
            if INK_DEPTH_PREPASS && self.glyph_count > self.sphere_count {
```

**Replace with:**

```rust
                pass.draw(0..4, 0..self.segment_count);
                draws += 1;
            }
            if INK_DEPTH_PREPASS && self.glyph_count > 0 {
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
                pass.draw(3 * self.sphere_count..3 * self.glyph_count, 0..1);
                draws += 1;
            }

            if self.segment_count > self.pipe_count {
```

**Replace with:**

```rust
                pass.draw(0..3 * self.glyph_count, 0..1);
                draws += 1;
            }

            if self.segment_count > 0 {
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
                // instance_index carries the base, so the row is just the instance id
                pass.draw(0..4, self.pipe_count..self.segment_count);
                draws += 1;
            }

            // Vertex ink, same split: glyphs[0..sphere_count] = mesh/BRep vertices -> markers
            // (DRAWN EARLIER - right after the faces; see there), the rest -> flat SDF dots.
            if self.glyph_count > self.sphere_count {
```

**Replace with:**

```rust
                // instance_index IS the row: this table holds nothing but flat-lane segments
                pass.draw(0..4, 0..self.segment_count);
                draws += 1;
            }

            // LETTERING, last of everything. A page paints its text on top of its hatching AND
            // its linework, so it lands after the ink lanes above - the one thing draw order can
            // express that a depth buffer cannot, since all of it is coplanar at z = 0.
            if self.arena_text_count > 0 {
                pass.set_pipeline(&self.pipelines.triangle_sheet);
                pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                pass.set_bind_group(1, &self.time_bind_group, &[]);
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_vertex_buffer(0, self.arena_vbo.slice(..));
                pass.set_vertex_buffer(1, self.arena_vids.slice(..));
                pass.set_index_buffer(self.arena_ibo_text.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..self.arena_text_count, 0, 0..1);
                draws += 1;
            }

            // Vertex ink, same split: the sphere table is mesh/BRep vertices -> markers (DRAWN
            // EARLIER - right after the faces; see there), this one is flat SDF dots.
            if self.glyph_count > 0 {
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
                pass.draw(3 * self.sphere_count..3 * self.glyph_count, 0..1); // 3 verts/dot, no template
                draws += 1;
            }









```

**Replace with:**

```rust
                pass.draw(0..3 * self.glyph_count, 0..1); // 3 verts/dot, no template
                draws += 1;
            }
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.arena_index_count = 0;
```

**Add below it:**

```rust
        self.arena_print_count = 0;
        self.arena_text_count = 0;
        // Every lane appends now, so a rebuild has to rewind every lane - leaving these set
        // would append the re-walked scene BEHIND the copy already there. Capacity stays, so a
        // rebuild costs no allocation.
        self.pipe_count = 0;
        self.segment_count = 0;
        self.sphere_count = 0;
        self.glyph_count = 0;
        self.point_count = 0;
        self.cloud_draws.clear();
        self.objects_base.clear();
        self.object_bounds_world.clear();
        self.inside.clear();
        self.instances.clear();
        self.instance_rows = 0;
```


## Step 6 — `FLAG_SHEET`

One more instance flag: this row belongs to a drawing sheet. Step 9 is what reads it.

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub const FLAG_OPEN: u32 = 1 << 4;
```

**Add below it:**

```rust

    /// This row belongs to a PLANAR file - a drawing sheet. Its fills write no depth (they are
    /// exactly coplanar and composite in document order instead), so the sheet's ink has nothing
    /// to fight and takes NO lift: ribbon.wgsl reads this bit and keeps the pen on the page. That
    /// is what lets the lettering pass, drawn last with a >= depth test, land on top of the
    /// linework the way the page draws it.
    pub const FLAG_SHEET: u32 = 1 << 5;
```

**Find** in `src/engine/gpu/mod.rs`:

```rust



// Points global attributes
```

**Replace with:**

```rust

// Points global attributes
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
/// A storage buffer filled by `write_buffer`, NOT `create_buffer_init`: init maps the whole
/// buffer at creation, and on wgpu's web backend that allocates a FULL-SIZE mirror of the
/// contents in the wasm heap - a 127 MB cloud table briefly costs 254 MB, three times per
/// scene load. `write_buffer` stages through the queue instead; an empty `data` leaves the
/// minimum-size buffer zero-initialized (a WebGPU guarantee).
fn storage_buffer<T: bytemuck::Pod>(device: &wgpu::Device, queue: &wgpu::Queue, label: &str, data: &[T]) -> wgpu::Buffer {
```

**Replace with:**

```rust
/// A storage bufffer filled by  `write buffer`, not `create_buffer_init`: init maps the whole buffer at a creation
/// and on wgpu's web backend that allocates a full-size mirror of the contents in the wasm heap costs three times per scene load.
/// `ẁrite_buffer` stages through the queue instead
/// empty data leaves the minimum buffer zeri-initialized.
fn storage_buffer<T: bytemuck::Pod>(device: &wgpu::Device, queue: &wgpu::Queue, label: &str, data: &[T]) -> wgpu::Buffer{
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    if !data.is_empty() {
```

**Replace with:**

```rust

    if !data.is_empty(){
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    buf
}

```

**Add below it:**

```rust

```


## Step 7 — the walk

`add_file` stops handing the GPU a table to copy and starts handing it a delta. `MeshTopo` is
the four-passes-into-one; its doc comment records exactly which kernel calls it replaces and
why the viewer, not the kernel, is the right place to fuse them.

**Find** in `src/app/scene.rs`:

```rust
    pub point_size: f64,              // raw-cloud px for this file; 0 = keep the pb's own
```

**Replace with:**

```rust
    pub point_size: f64,              // raw-cloud px for this file; 0 = keep the pb'own
    /// `display_only = true` releases this file's kernel `Session` once it has been walked into
    /// the GPU tables. It is the single biggest memory lever a scene has, and it is a scene's
    /// call to make rather than the loader's, because of exactly what it gives up.
    ///
    /// What a Doc's `Session` is FOR, once the walk is done, is reading geometry back: picking
    /// (ray against the kernel meshes), editing, saving, and `Scene::rebuild`. A drawing sheet
    /// does none of those - it is ink on paper that is looked at - and it is also where the
    /// memory is: 10 sheets of the `drawings` scene hold 1.2 GB of kernel documents to draw
    /// tables the GPU already owns. Measured on that scene: 2056 MB resident -> 899 MB, frame
    /// byte-identical.
    ///
    /// A model file (the bunny, a BRep, anything the user will click) must NOT set this.
    #[serde(default)]
    pub display_only: bool,
```

**Find** in `src/app/scene.rs`:

```rust
    pub fn parse(bytes: &[u8]) -> Option<Self> {
                serde_json::from_slice(bytes).ok()
```

**Replace with:**

```rust
    /// JSON first (every existing scene), TOML as the fallback - a .toml manifest gets
    /// real comments and no trailing-comma landmines; both land in the same structs.
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        serde_json::from_slice(bytes).ok()
```

**Find** in `src/app/scene.rs`:

```rust
use session_rust::{Session, Geometry, Mesh, Line, Point, Polyline, NurbsCurve, RenderVertex, Plane, OBB, PointCloud, Vector};
use session_rust::element::ElementGeometry;
use session_rust::mesh::ColorMode;
use crate::engine::gpu::{ArenaUpload, Instance, CylinderSegment, GlyphPoint};
```

**Replace with:**

```rust
use session_rust::{Session, Geometry, Mesh, Line, Point, Polyline, NurbsCurve, RenderVertex, Plane, OBB, PointCloud, Vector, Color, Tolerance};
use session_rust::element::ElementGeometry;
use session_rust::mesh::ColorMode;
use crate::engine::gpu::{ArenaUpload, Instance, CylinderSegment, GlyphPoint, Mat4, mat_mul};
```

**Find** in `src/app/scene.rs`:

```rust
    pub cloud_px: f32, // per-file raw_cloud point size, px; 0 = pb's own
```

**Add below it:**

```rust
    /// This doc's `session` was RELEASED after the walk (manifest `display_only`), so it is an
    /// empty shell: it still names the document and holds its placement, but there is no geometry
    /// behind it any more. `rebuild` cannot bring it back.
    pub display_only: bool,
```

**Find** in `src/app/scene.rs`:

```rust
    vert_base: u32,             // arena rows already uploaded - push_mesh bases its indices on this
```

**Add below it:**

```rust
    cloud_base: u32,            // cloud points already uploaded - a draw record's `first` counts from here
```

**Find** in `src/app/scene.rs`:

```rust
        vert_base: 0,
```

**Add below it:**

```rust
        cloud_base: 0,
```

**Find** in `src/app/scene.rs`:

```rust
        gpu.reset_arena();

        for d in docs {
            self.add_file(d.name, d.session, d.place, d.cloud_px);
```

**Replace with:**

```rust
        self.cloud_base = 0;
        gpu.reset_arena();

        for d in docs {
            if d.display_only {
                // Nothing to re-walk - the kernel document was released after the first walk.
                // Saying so beats silently dropping the sheet out of the frame.
                log::warn!("rebuild: '{}' is display_only, its geometry was released", d.name);
            }
            self.add_file(d.name, d.session, d.place, d.cloud_px, d.display_only);
```

**Find** in `src/app/scene.rs`:

```rust
    /// Upload the walked tables, then FORGET the arena rows: the GPU is their only holder.
    pub fn upload_to(&mut self, gpu: &mut crate::engine::gpu::Gpu) {
        gpu.set_scene(&self.tables);
        // The arena rows are on the GPU now and nothing reads them back - picking goes through
        // the kernel Meshes in Doc.session, never through these flattened rows. Keep only the
        // running vertex base, so the next file's indices still land in the right place.
        self.vert_base += self.tables.verts.len() as u32;
        self.tables.verts.clear();
        self.tables.verts.shrink_to_fit();
        self.tables.vids.clear();
        self.tables.vids.shrink_to_fit();
        self.tables.idx.clear();
        self.tables.idx.shrink_to_fit();
```

**Replace with:**

```rust
    /// Upload the walked tables, then FORGET the rows: the GPU is their only holder.
    ///
    /// EVERY drawn table goes, not just the arena. Nothing reads any of them back - picking goes
    /// through the kernel Meshes in Doc.session, never through these flattened rows - and holding
    /// them cost twice over: the wasm heap kept a full second copy of the scene for the whole
    /// session (280 MB on a 13.8 M-point scan), and having that copy is exactly what let the ink
    /// and cloud lanes rebuild their whole buffer per file instead of appending. Keep only the
    /// running bases, so the next file's indices still land in the right place.
    pub fn upload_to(&mut self, gpu: &mut crate::engine::gpu::Gpu) {
        gpu.set_scene(&self.tables);
        self.vert_base += self.tables.verts.len() as u32;
        self.cloud_base += (self.tables.cloud_pos.len() / 3) as u32;
        let t = &mut self.tables;
        drop_rows(&mut t.verts);
        drop_rows(&mut t.vids);
        drop_rows(&mut t.idx);
        drop_rows(&mut t.idx_print);
        drop_rows(&mut t.idx_text);
        drop_rows(&mut t.pipes);
        drop_rows(&mut t.segments);
        drop_rows(&mut t.spheres);
        drop_rows(&mut t.glyphs);
        drop_rows(&mut t.cloud_pos);
        drop_rows(&mut t.cloud_col);
        drop_rows(&mut t.cloud_nrm);
        drop_rows(&mut t.cloud_draws);
        // `objects`, `object_bounds` and `object_spacing` STAY: they are per-object rows the
        // instance table is rebased from every time the camera re-anchors, and the walk indexes
        // them by global row - they are the one table the GPU is not the only holder of.
```

**Find** in `src/app/scene.rs`:

```rust
    pub fn add_file(&mut self, name: String, session: Session, place: Xform, cloud_px: f32){

```

**Replace with:**

```rust
    pub fn add_file(&mut self, name: String, session: Session, place: Xform, cloud_px: f32, display_only: bool){

        let cb = self.cloud_base; // read before `t` borrows self.tables
```

**Find** in `src/app/scene.rs`:

```rust
        let placement = |guid: &str| world.get(guid).cloned().unwrap_or_else(Xform::identity);
```

**Replace with:**

```rust
        // The 99% path (a flat sheet, a mesh file) has NO local transforms, so every row's
        // placement IS the file placement - composed once here instead of 90k times inside the
        // loop, where `Xform::identity()` + `&place * &..` cost four heap allocations apiece.
        let place_m = place.m;
        let placement = |guid: &str| match world.get(guid) {
            Some(local) => mat_mul(&place_m, &local.m),
            None => place_m,
        };
        // VIEWER_PROFILE=1 splits the walk into "per-object push" vs "bounds sweep". Same
        // wasm32 caveat as push_mesh: Instant::now() PANICS in the browser, so it is cfg'd out.
        #[cfg(not(target_arch = "wasm32"))]
        let wprof = env_flag("VIEWER_PROFILE", &VIEWER_PROFILE);
        #[cfg(not(target_arch = "wasm32"))]
        let mut wlap = std::time::Instant::now();
```

**Find** in `src/app/scene.rs`:

```rust
            let placed = &place * &placement(&guid);
```

**Replace with:**

```rust
            let placed = placement(&guid);
```

**Find** in `src/app/scene.rs`:

```rust
                Geometry::Mesh(m) => {

                    let b = push_mesh(
                        m, 
```

**Replace with:**

```rust
                Geometry::Mesh(m) => {
                    // Which index run this mesh's triangles join decides WHEN it is drawn, and
                    // for a drawing that is the whole answer: sheet fills composite in document
                    // order with no depth arbitration, and lettering goes last of all - after the
                    // ink lanes. `is_print_fill` is the sheet test the walk already uses; the
                    // "text" name is set by the PDF importer, which knows a glyph from a region.
                    let idx_lane = if is_print_fill(m) {
                        if m.name == "text" { &mut t.idx_text } else { &mut t.idx_print }
                    } else {
                        &mut t.idx
                    };
                    let (b, closed) = push_mesh(
                        m, 
```

**Find** in `src/app/scene.rs`:

```rust
                        ri,
                        vb, 
                        &mut t.verts, 
                        &mut t.vids, 
                        &mut t.idx, 
                        &mut t.pipes,
                        &mut t.spheres
                    );
                    if is_print_fill(m) {
```

**Replace with:**

```rust
                        ri,
                        vb, 
                        &mut t.verts, 
                        &mut t.vids, 
                        idx_lane, 
                        &mut t.pipes,
                        &mut t.spheres
                    );
                    if is_print_fill(m) {
```

**Find** in `src/app/scene.rs`:

```rust
                    if !m.is_closed() {
```

**Replace with:**

```rust
                    // ONLY when this mesh actually drew a wireframe. `b` is None for a print
                    // fill and for a dense mesh - neither emits pipes or dots, and FLAG_OPEN is
                    // read by nothing else (cylinder/sphere/ribbon shaders only). The answer rides
                    // out of push_mesh because the fused topology pass already knows it - an edge
                    // walked by a face in only one direction IS a border. `Mesh::is_closed()` was a
                    // SECOND full sweep, two more HashSets over every directed face edge: 10 ms on
                    // the bunny, 91 ms on one sheet's 21 fill meshes, every millisecond of it
                    // thrown away.
                    if b.is_some() && !closed {
```

**Find** in `src/app/scene.rs`:

```rust
                    bm.set_objectcolor(b.surfacecolor.clone());
                    let bb = push_mesh(
                        &bm, 
```

**Replace with:**

```rust
                    bm.set_objectcolor(b.surfacecolor.clone());
                    let (bb, _) = push_mesh(
                        &bm, 
```

**Find** in `src/app/scene.rs`:

```rust
                // EVERY cloud takes the splat lane: split flat rows into the shared tables,
                // one draw record per cloud, and the per-cloud point size rides the spacing
                // row (unused for clouds - the ink lanes read 0 there as "never cull").
                Geometry::PointCloud(pc) => {
                    let first = (t.cloud_pos.len() / 3) as u32;
                    push_cloud(pc, &mut t.cloud_pos, &mut t.cloud_col, &mut t.cloud_nrm);
                    t.cloud_draws.push((first, pc.len() as u32, ri, cloud_spacing(pc)));
                    let px = if cloud_px > 0.0 { cloud_px } else { pc.point_size as f32 };
                    t.object_bounds.push(None); t.object_spacing.push(px);
```

**Replace with:**

```rust
                // EVERY cloud takes the splat lane: split flat rows into share tables,
                // one draw record per cloud, and the per cloud point size rides the spacing spacing
                Geometry::PointCloud(pc) => {
                    // ABSOLUTE first point, counted from the start of the scene: the GPU table is
                    // cumulative while `cloud_pos` is only this upload's delta.
                    let first = cb + (t.cloud_pos.len() / 3) as u32;
                    push_cloud(pc, &mut t.cloud_pos, &mut t.cloud_col, &mut t.cloud_nrm);
                    t.cloud_draws.push((first, pc.len() as u32, ri, cloud_spacing(pc)));
                    let px = if cloud_px > 0.0 { cloud_px } else { pc.point_size as f32 };
                    t.object_bounds.push(None);
                    t.object_spacing.push(px);
```

**Find** in `src/app/scene.rs`:

```rust
                    }
                    let b = push_mesh(
                        &sm, 
```

**Replace with:**

```rust
                    }
                    let (b, _) = push_mesh(
                        &sm, 
```

**Find** in `src/app/scene.rs`:

```rust
                        let b = push_mesh(
```

**Replace with:**

```rust
                        let idx_lane = if is_print_fill(&m) {
                            if m.name == "text" { &mut t.idx_text } else { &mut t.idx_print }
                        } else {
                            &mut t.idx
                        };
                        let (b, _) = push_mesh(
```

**Find** in `src/app/scene.rs`:

```rust
                            ri,
                        vb, 
                            &mut t.verts, 
                            &mut t.vids, 
                            &mut t.idx, 
                            &mut t.pipes,
                            &mut t.spheres
                        );
                        if is_print_fill(&m) {
```

**Replace with:**

```rust
                            ri,
                        vb, 
                            &mut t.verts, 
                            &mut t.vids, 
                            idx_lane, 
                            &mut t.pipes,
                            &mut t.spheres
                        );
                        if is_print_fill(&m) {
```

**Find** in `src/app/scene.rs`:

```rust
                        let bb = push_mesh(
```

**Replace with:**

```rust
                        let (bb, _) = push_mesh(
```

**Find** in `src/app/scene.rs`:

```rust
            self.order.push(guid);
        }
```

**Add below it:**

```rust
        #[cfg(not(target_arch = "wasm32"))]
        if wprof { eprintln!("  walk objects {:?}", wlap.elapsed()); wlap = std::time::Instant::now(); }
```

**Find** in `src/app/scene.rs`:

```rust
        for &(first, count, inst, _) in t.cloud_draws.iter().skip(draw0){
            let Some((xf, _, _)) = t.objects.get(inst as usize) else { continue };
            for i in first as usize..(first + count) as usize {
                let p = [t.cloud_pos[i * 3], t.cloud_pos[i * 3 + 1], t.cloud_pos[i * 3 + 2]];
```

**Replace with:**

```rust

        for &(first, count, inst, _) in t.cloud_draws.iter().skip(draw0){
            let Some((xf, _, _)) = t.objects.get(inst as usize) else { continue };
            // `first` is absolute; `cloud_pos` starts at `cb`.
            for i in (first - cb) as usize..(first - cb + count) as usize {
                let p = [t.cloud_pos[i*3], t.cloud_pos[i*3+1], t.cloud_pos[i*3 + 2]];
```

**Find** in `src/app/scene.rs`:

```rust
                grow_bounds(&mut fmin, &mut fmax, xform_point(xf, p));
            }
        }

```

**Add below it:**

```rust
        #[cfg(not(target_arch = "wasm32"))]
        if wprof { eprintln!("  walk bounds  {:?}", wlap.elapsed()); wlap = std::time::Instant::now(); }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = &wlap;
```

**Find** in `src/app/scene.rs`:

```rust
        if planar {
```

**Add below it:**

```rust
            // Every row of this file is page content. The ink lanes read the bit to drop their
            // lift (a sheet's fills no longer write depth, so there is nothing to lift off), and
            // that is what lets the lettering pass sit on top of the linework.
            for o in t.objects.iter_mut().skip(obj0) {
                o.2 |= Instance::FLAG_SHEET;
            }
```

**Find** in `src/app/scene.rs`:

```rust
        let _ = obj0;
```

**Replace with:**

```rust
        // The walk is done and the tables are about to be uploaded, so a display-only document
        // has nothing left to answer: release it here rather than at the call site, because this
        // is the exact point after which nothing reads it. VIEWER_DROP_SESSIONS=1 forces it on
        // for every file, which is how the number in `Item::display_only` was measured.
        let display_only = display_only || env_flag("VIEWER_DROP_SESSIONS", &VIEWER_DROP_SESSIONS);
        let session = if display_only { Session::new(&name) } else { session };
```

**Find** in `src/app/scene.rs`:

```rust
            cloud_px
```

**Replace with:**

```rust
            cloud_px,
            display_only,
```

**Find** in `src/app/scene.rs`:

```rust
fn oct16(n: &Vector) -> Option<u32> {
```

**Replace with:**

```rust
fn oct16(n: &[f64; 3]) -> Option<u32> {
```

**Find** in `src/app/scene.rs`:

```rust
    Some(q(x) | q(y) << 8)
}

```

**Add below it:**

```rust
/// Opaque black, packed. The wireframe's default pen, and what a dense mesh's edges draw as.
const BLACK: u32 = 0xff00_0000;

```

**Find** in `src/app/scene.rs`:

```rust
fn pack_facing(n0: Option<Vector>, n1: Option<Vector>) -> u32 {
    let pair = match (n0, n1) {
        (Some(a), Some(b)) => (oct16(&a), oct16(&b)),
        // A naked edge is visible whenever its single face is, so duplicating the one normal is
        // the correct answer and needs no special case in the shader.
        (Some(a), None) | (None, Some(a)) => (oct16(&a), oct16(&a)),
```

**Replace with:**

```rust
fn pack_facing(n0: Option<&[f64; 3]>, n1: Option<&[f64; 3]>) -> u32 {
    let pair = match (n0, n1) {
        (Some(a), Some(b)) => (oct16(a), oct16(b)),
        // A naked edge is visible whenever its single face is, so duplicating the one normal is
        // the correct answer and needs no special case in the shader.
        (Some(a), None) | (None, Some(a)) => (oct16(a), oct16(a)),
```

**Find** in `src/app/scene.rs`:

```rust
static VIEWER_NO_EDGES: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
static VIEWER_NO_DOTS: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
```

**Replace with:**

```rust
static VIEWER_DROP_SESSIONS: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
static VIEWER_NO_EDGES: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
static VIEWER_NO_DOTS: std::sync::OnceLock<bool> = std::sync::OnceLock::new();

/// Everything the ink lanes need to know about a mesh's topology, built in ONE pass over the
/// faces: the edge list (unique, with its pen color), each edge's two adjacent faces, the per-face
/// normal, and whether the mesh is closed.
///
/// It exists because the kernel answers the same four questions in four independent passes -
/// `edges_with_colors`, `edge_face_map`, `face_normals`, `is_closed` - each rebuilding its own
/// hash table over the same faces, and `face_normals` allocating a `Vector` (String name + guid
/// OnceLock) per face on top. That is the kernel's business: three languages share those APIs and
/// they answer honestly on their own. The VIEWER walks every mesh in a scene through all four at
/// once, so it pays for the repetition four times over - 123 ms of the bunny's 137 ms walk.
///
/// Byte-identical to the kernel by construction: same sorted-face-key order, same "first face to
/// walk a directed edge keeps it" rule, same `linecolors[i]` indexing by first-seen edge, same
/// cross-product-and-normalize as `Mesh::face_normal`.
struct MeshTopo {
    /// Unique edges as (low, high) vertex key + PACKED pen color, in `edges_with_colors` order.
    /// Packed here rather than kept as a kernel `Color`, which carries a `name` String and a guid
    /// OnceLock: cloning one per edge was 104k String allocations on the bunny, for four bytes.
    edges: Vec<(usize, usize, u32)>,
    /// Per edge: the face walking (low, high) and the face walking (high, low), as SLOTS into
    /// `normals` (u32::MAX = none). Compacted the way the old two-lookup loop compacted: a lone
    /// face always lands in slot 0.
    edge_faces: Vec<[u32; 2]>,
    /// Per face slot, in sorted-face-key order. `None` for a degenerate face.
    normals: Vec<Option<[f64; 3]>>,
    /// Every edge walked in BOTH directions, i.e. no border. Meshes with declared hole rings fall
    /// back to the kernel, which knows that a ring's own edges are not borders.
    closed: bool,
}

/// One face's normal, from the by-slot position table - no `Point`, no `Vector`, no allocation
/// and no map lookup. Same arithmetic and the same `ZERO_TOLERANCE` cut-off as `Mesh::face_normal`.
fn face_normal_raw(vs: &[usize], vpos: &[[f64; 3]], slot: &impl Fn(usize) -> usize) -> Option<[f64; 3]> {
    if vs.len() < 3 { return None }
    let (p0, p1, p2) = (vpos[slot(vs[0])], vpos[slot(vs[1])], vpos[slot(vs[2])]);
    let u = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
    let v = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
    let n = [
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ];
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if len > Tolerance::ZERO_TOLERANCE { Some([n[0] / len, n[1] / len, n[2] / len]) } else { None }
}

/// The fused pass. No hash table at all: edges hang off their LOW vertex on an intrusive chain
/// (`head` per vertex slot, `next` per edge), so finding whether (lo, hi) already exists is a walk
/// of the two or three edges that share `lo` - array reads, no hashing, and deterministic by
/// construction, where a HashMap's order depends on a per-process random seed.
fn mesh_topology(m: &Mesh, keys: &[usize], vpos: &[[f64; 3]], slot: &impl Fn(usize) -> usize) -> MeshTopo {
    // SORTED face keys: the kernel sorts too, and it is what makes the pen colors and the packed
    // `facing` words reproducible - `m.face` is a HashMap, so its own iteration order changes
    // between runs of the same binary on the same file.
    // Sorted (key, vertex list) pairs, not sorted keys re-looked-up: `m.face` is a HashMap, so
    // indexing it per face is one hash per face on top of the walk.
    let mut faces: Vec<(usize, &Vec<usize>)> = m.face.iter().map(|(k, v)| (*k, v)).collect();
    faces.sort_unstable_by_key(|f| f.0);
    let cols = m.get_linecolors();

    let mut normals: Vec<Option<[f64; 3]>> = Vec::with_capacity(faces.len());
    let mut edges: Vec<(usize, usize, u32)> = Vec::new();
    let mut edge_faces: Vec<[u32; 2]> = Vec::new();
    let mut head: Vec<u32> = vec![u32::MAX; keys.len()];
    let mut next: Vec<u32> = Vec::new();

    for (fs, (_, vs)) in faces.iter().enumerate() {
        normals.push(face_normal_raw(vs, vpos, slot));
        let n = vs.len();
        for i in 0..n {
            let (u, v) = (vs[i], vs[(i + 1) % n]);
            // dir 0 = this face walks the edge low -> high, dir 1 = high -> low. The two are the
            // two SIDES of the edge, which is exactly what the facing test needs.
            let (lo, hi, dir) = if u < v { (u, v, 0) } else { (v, u, 1) };
            let ls = slot(lo);
            let mut ei = head[ls];
            while ei != u32::MAX && edges[ei as usize].1 != hi {
                ei = next[ei as usize];
            }
            if ei == u32::MAX {
                ei = edges.len() as u32;
                let pen = cols.get(edges.len()).map_or(BLACK, |c| pack_rgba(c.to_f32()));
                edges.push((lo, hi, pen));
                edge_faces.push([u32::MAX; 2]);
                next.push(head[ls]);
                head[ls] = ei;
            }
            // FIRST face wins, like the kernel's `or_insert`: on an inconsistently wound or
            // non-manifold patch two faces walk the same directed edge, and letting the last one
            // win makes the packed `facing` word depend on which face was visited first.
            let f = &mut edge_faces[ei as usize][dir];
            if *f == u32::MAX { *f = fs as u32; }
        }
    }

    // The chain is built by pushing to the FRONT, so edges come out newest-first per vertex -
    // which is not the kernel's order. `edges_with_colors` emits them in first-seen order, and the
    // pen colors are indexed by that, so the list is built in first-seen order too (above) and the
    // chain is only ever used for lookup. Nothing to re-sort.
    let mut closed = !m.vertex.is_empty();
    for f in edge_faces.iter_mut() {
        if f[0] == u32::MAX || f[1] == u32::MAX { closed = false }
        // A lone face moves to slot 0 - the old two-lookup loop filled the slots in lookup order
        // and stopped at the first miss, so a border edge's single normal was always `normal_of(0)`.
        if f[0] == u32::MAX { f[0] = f[1]; f[1] = u32::MAX; }
    }
    // A declared hole ring's edges are borders by this test but not by the kernel's, and only the
    // kernel knows the rings. Rare (PDF poche fills), and it never reaches here anyway - a fill
    // returns before the topology pass.
    if !closed && !m.face_holes.is_empty() { closed = m.is_closed(); }

    MeshTopo { edges, edge_faces, normals, closed }
}
```

**Find** in `src/app/scene.rs`:

```rust
) -> Option<([f32; 3], [f32; 3])> {
```

**Replace with:**

```rust
) -> (Option<([f32; 3], [f32; 3])>, bool) {
```

**Find** in `src/app/scene.rs`:

```rust
    if rm.indices.len() / 3 > MESH_RAW_MIN {
        return None;
    }
```

**Replace with:**

```rust
    if rm.indices.len() / 3 > MESH_RAW_MIN {
        return (None, false);
    }
```

**Find** in `src/app/scene.rs`:

```rust
    if is_print_fill(m) { return None }

    if env_flag("VIEWER_NO_EDGES", &VIEWER_NO_EDGES) { return None }

    // ONE edge walk, shared by the pipes below and the vertex widths further down.
    let edges = m.edges_with_colors();
    mark("edges_with_colors", &mut lap);

    // Face normals once for the whole mesh, so the per-edge adjacency lookup below is two map
    // reads instead of a cross product each time. These are MESH-LOCAL, matching p0/p1 - the
    // shader rotates them by the instance model the same way it transforms the endpoints.
    let fnormals = m.face_normals();
    mark("face_normals", &mut lap);

```

**Replace with:**

```rust
    if is_print_fill(m) { return (None, false) }

    if env_flag("VIEWER_NO_EDGES", &VIEWER_NO_EDGES) { return (None, false) }

    // ONE face walk builds all three things the lanes need: the edge list with its pen colors,
    // each edge's two adjacent faces, and the face normals (MESH-LOCAL, matching p0/p1 - the
    // shader rotates them by the instance model the same way it transforms the endpoints).
    //
    // The kernel answers the same three questions in three separate passes over the faces, each
    // building its own hash table - and `face_normals` allocates a `Vector` per face, which
    // carries a `name` String and a guid OnceLock. On the bunny (69k faces, 104k edges) that was
    // 39 ms (edges_with_colors) + 43 ms (face_normals) + 28 ms (edge_face_map) + 13 ms of lookups
    // against 30 ms for the fused pass - the single biggest cost in the whole walk. Same walk
    // order (sorted face keys, first face to walk a directed edge keeps it), so the pen colors
    // and the `facing` words come out byte-identical.
```

**Find** in `src/app/scene.rs`:

```rust
    let vpos: Vec<[f32; 3]> = keys.iter().map(|&k| m.vertex_point(k).unwrap().to_f32()).collect();
```

**Replace with:**

```rust
    // Straight out of the vertex table, not `vertex_point`, which builds a `Point` (name String
    // + guid OnceLock) per vertex only to read three numbers back off it. Kept in f64 as well:
    // the face normals below are computed from these, and rounding to f32 first would change the
    // sign of a near-degenerate cross product, i.e. the packed `facing` word.
    let vpos64: Vec<[f64; 3]> = keys.iter().map(|&k| { let v = &m.vertex[&k]; [v.x, v.y, v.z] }).collect();
    let vpos: Vec<[f32; 3]> = vpos64.iter().map(|p| [p[0] as f32, p[1] as f32, p[2] as f32]).collect();

    let topo = mesh_topology(m, &keys, &vpos64, &slot);
    let edges = &topo.edges;
    let closed = topo.closed;
    mark("topology", &mut lap);
```

**Find** in `src/app/scene.rs`:

```rust
    let mut edge_faces: Vec<Vec<usize>> = Vec::with_capacity(edges.len());
```

**Replace with:**

```rust
    // `topo.edge_faces` holds two slots per edge, not a Vec: an edge has at most two adjacent
    // faces and the code below reads at most two. The old `Vec<usize>` per edge heap-allocated
    // once per edge - 104k allocations on the bunny alone, 87 ms of the pipe loop. u32::MAX = no
    // face in that slot, and the entries are face SLOTS (index into `topo.normals`), not keys.
    let edge_faces = &topo.edge_faces;
```

**Find** in `src/app/scene.rs`:

```rust
    for (i, (a, b, col)) in edges.iter().cloned().enumerate(){
        let f = m.edge_faces(a, b).unwrap_or_default();
```

**Replace with:**

```rust
    for (i, (a, b, col)) in edges.iter().enumerate(){
        let f = edge_faces[i];
```

**Find** in `src/app/scene.rs`:

```rust
        let facing = pack_facing(
            f.first().and_then(|&k| fnormals.get(&k).cloned()),
            f.get(1).and_then(|&k| fnormals.get(&k).cloned()),
        );
        edge_faces.push(f);
```

**Replace with:**

```rust
        // Borrowed, never cloned: `Vector` carries a `name` String and a guid OnceLock, so a
        // `.cloned()` here was two heap allocations per edge - 200k on the bunny's wireframe.
        let normal_of = |slot: usize| -> Option<&[f64; 3]> {
            if f[slot] == u32::MAX { None } else { topo.normals[f[slot] as usize].as_ref() }
        };
        let facing = pack_facing(normal_of(0), normal_of(1));
```

**Find** in `src/app/scene.rs`:

```rust
                p0: vpos[slot(a)],
                radius: encode_width(width_at(i)),
                p1: vpos[slot(b)],
                instance_id: ri,
                color: if black_wire { pack_rgba([0.0, 0.0, 0.0, 1.0]) } else { pack_rgba(col.to_f32()) },
```

**Replace with:**

```rust
                p0: vpos[slot(*a)],
                radius: encode_width(width_at(i)),
                p1: vpos[slot(*b)],
                instance_id: ri,
                color: if black_wire { BLACK } else { *col },
```

**Find** in `src/app/scene.rs`:

```rust
    for (i, (a, b, _)) in edges.iter().cloned().enumerate(){
```

**Replace with:**

```rust
    for (i, (a, b, _)) in edges.iter().enumerate(){
```

**Find** in `src/app/scene.rs`:

```rust
        for vk in [a, b] {
```

**Replace with:**

```rust
        for vk in [*a, *b] {
```

**Find** in `src/app/scene.rs`:

```rust
    if env_flag("VIEWER_NO_DOTS", &VIEWER_NO_DOTS) { return local_bounds }
```

**Replace with:**

```rust
    if env_flag("VIEWER_NO_DOTS", &VIEWER_NO_DOTS) { return (local_bounds, closed) }
```

**Find** in `src/app/scene.rs`:

```rust
            for &fk in &edge_faces[ei] {
```

**Replace with:**

```rust
            for &f in edge_faces[ei].iter() {
                if f == u32::MAX { continue }
                let fk = f as usize;
```

**Find** in `src/app/scene.rs`:

```rust
                .filter_map(|fk| fnormals.get(fk))
```

**Replace with:**

```rust
                .filter_map(|fk| topo.normals[*fk].as_ref())
```

**Find** in `src/app/scene.rs`:

```rust
    local_bounds
}

pub fn xform_point(xf: &Xform, p: [f32; 3]) -> [f32; 3] {
```

**Replace with:**

```rust
    (local_bounds, closed)
}

/// Empty a table AND hand its allocation back. `clear()` alone keeps the capacity, which on
/// these tables is the whole point of the exercise - a scan's cleared-but-capacious `cloud_pos`
/// holds exactly as much wasm heap as a full one.
fn drop_rows<T>(v: &mut Vec<T>) {
    v.clear();
    v.shrink_to_fit();
}

pub fn xform_point(m: &Mat4, p: [f32; 3]) -> [f32; 3] {
```

**Find** in `src/app/scene.rs`:

```rust
        (xf.m[0] * x + xf.m[4] * y + xf.m[8] * z + xf.m[12]) as f32,
        (xf.m[1] * x + xf.m[5] * y + xf.m[9] * z + xf.m[13]) as f32,
        (xf.m[2] * x + xf.m[6] * y + xf.m[10] * z + xf.m[14]) as f32,
```

**Replace with:**

```rust
        (m[0] * x + m[4] * y + m[8] * z + m[12]) as f32,
        (m[1] * x + m[5] * y + m[9] * z + m[13]) as f32,
        (m[2] * x + m[6] * y + m[10] * z + m[14]) as f32,
```

**Find** in `src/app/scene.rs`:

```rust
/// The raw lane's rows, written STRAIGHT into the shared table (one 423 MB peak, not two),
/// reading the kernel's FLAT arrays rather than get_point/get_color (no per-point allocs).
```

**Replace with:**

```rust

/// The raw lane's rows, written straight into the shared table,
/// reading the kernel's flat arrays rather than get_point/get_color (no per_point allocs)
```

**Find** in `src/app/scene.rs`:

```rust
    pos.reserve(n * 3);
    col.reserve(n);
    nrm.reserve(n);
    for i in 0..n {
        pos.push(coords[i * 3] as f32);
        pos.push(coords[i * 3 + 1] as f32);
        pos.push(coords[i * 3 + 2] as f32);
        // Normal, oct16-packed into 16 bits (same encoding as the edge facing words).
        // All-ones = this point HAS no normal: a scan without them still pays the 4 B,
        // but the shading branch stays uniform per cloud, which is what the GPU wants.
        nrm.push(if i * 3 + 2 < normals.len() {
            let v = Vector::new(normals[i * 3], normals[i * 3 + 1], normals[i * 3 + 2]);
            oct16(&v).unwrap_or(u32::MAX)
```

**Replace with:**

```rust
    pos.reserve(n*3);
    col.reserve(n);
    nrm.reserve(n);
    for i in 0..n{
        pos.push(coords[i*3] as f32);
        pos.push(coords[i*3+1] as f32);
        pos.push(coords[i*3+2] as f32);

        // Normal, oct16-packed into 16 bits
        // All-ones = this point has nor normal: a scan without them still pays the 4 B,
        // but the shading branch stays uniform per cloud, which is what the GPU wants.
        // Three f64s, not a `Vector`: the kernel type carries a `name` String and a guid
        // OnceLock, so building one per point cost two heap allocations per scanned point -
        // 27 million of them on the 13.8 M-point lidar scan, for a value read once and dropped.
        nrm.push(if i*3 + 2 < normals.len() {
            oct16(&[normals[i*3], normals[i*3+1], normals[i*3+2]]).unwrap_or(u32::MAX)
```

**Find** in `src/app/scene.rs`:

```rust
        // The colour is 8-bit at the source (proto 0-255): pack it back to the four bytes it
        // is, instead of four f32s carrying four bytes of information.
        col.push(if c + 3 < colors.len() {
            (colors[c] as u32 & 255) | (colors[c + 1] as u32 & 255) << 8
                | (colors[c + 2] as u32 & 255) << 16 | (colors[c + 3] as u32 & 255) << 24
```

**Replace with:**

```rust

        // The colour is 8-bit at the source (proto 0-255):
        // pack it back to the four bytes it is, instrad of four f32s carying four bytes of information
        col.push(if c + 3 < colors.len() {
            (colors[c] as u32 & 255) | (colors[c + 1] as u32 & 255) << 8 | (colors[c+2] as u32 & 255) << 16 | (colors[c + 3] as u32 & 255) << 24
```

**Find** in `src/app/scene.rs`:

```rust
}

/// Median distance between CONSECUTIVE points - a scanner emits angular neighbours in order,
/// so successive points are usually adjacent on the surface, which makes this a cheap and
/// honest estimate of the cloud's point spacing (world units). Potree never measures this:
/// it PRESCRIBES a spacing per octree node at conversion time. Lesson 44's octree does the
/// same for its coarse nodes - and still needs this MEASURED number for the raw points in
/// its leaves. Drives the attenuated (world-sized) splat radius.
fn cloud_spacing(pc: &PointCloud) -> f32 {
    let c = pc.coords();
    let n = pc.len();
    if n < 2 { return 20.0; }
```

**Replace with:**

```rust
   
}

/// Median distance between consecutive points - a scanner emits angular neighbours in order,
/// so successive points are usually adjacent on the surface, which makes this a cheap and
/// honest estimate of the clouds's point spacing (world units). 
/// Potree gets the same number from its octree, we sample it.
/// Drives the attenuated world-sized splat radius.
fn cloud_spacing(pc: &PointCloud) -> f32{
    let c = pc.coords();
    let n = pc.len();
    if n < 2 {
        return 20.0;
    }
```

**Find** in `src/app/scene.rs`:

```rust
        let (a, b) = (i * 3, (i + 1) * 3);
        let dd = (c[a] - c[b]).powi(2) + (c[a + 1] - c[b + 1]).powi(2) + (c[a + 2] - c[b + 2]).powi(2);
        if dd > 0.0 { d.push(dd.sqrt()); }
        i += step;
    }
    if d.is_empty() { return 20.0; }
```

**Replace with:**

```rust
        let  (a, b) = (i * 3, (i + 1) * 3);
        let dd = (c[a] - c[b]).powi(2) + (c[a + 1] - c[b + 1]).powi(2) + (c[a + 2] - c[b + 2]).powi(2);
        if dd> 0.0 {
            d.push(dd.sqrt());
        }
        i += step;
    }
    if d.is_empty() {
        return 20.0;
    }
```

**Find** in `src/app/scene.rs`:

```rust

    let radius = encode_width(pc.point_size);
    let colors = pc.color_count();
    (0..pc.len()).map(|i| GlyphPoint{
        center: pc.get_point(i).to_f32(),
        radius,
        color: if i < colors {pc.get_color(i).to_f32()} else { [0.0, 0.0, 0.0, 1.0]},
        instance_id,
        facing: FACING_UNKNOWN, // a cloud point has no surface to hug
        facing_ext: [FACING_UNKNOWN; 2],
    }).collect()
}
```

**Delete**

**Find** in `src/app/scene.rs`:

```rust
    d[d.len() / 2] as f32
}


```

**Replace with:**

```rust
    d[d.len() / 2] as f32
}

```


## Step 8 — the lean decode

The kernel's `Mesh` proto carries a halfedge map the viewer throws away — but prost decodes it
into a nested `HashMap` first. A wire-identical mirror with that one tag left out skips it with
a length jump instead.

`LeanMesh` mirrors `proto::Mesh` as the kernel spelled it here, so it only compiles against the
kernel this lesson's tree pins. If yours has moved — the colour fields were later repacked to
`*_rgba`, and the halfedge tag went away entirely — mirror the fields your `proto::Mesh`
actually has, and keep the rule rather than the field list: every tag the viewer reads gets its
exact type so `into_proto` MOVES it, and every tag it does not read is simply absent.

**Find** in `src/app/persistence.rs`:

```rust
use session_rust::graph::Graph;
```

**Delete**

**Find** in `src/app/persistence.rs`:

```rust


/// Objects converted per slice before the loader hands the browser one macrotask — the whole
```

**Replace with:**

```rust

/// Objects converted per slice before the loader hands the browser one macrotask — the whole
```

**Find** in `src/app/persistence.rs`:

```rust
    let _ = JsFuture::from(p).await;
}

```

**Add below it:**

```rust
/// Wire-identical mirror of `proto::Mesh` with ONE field left out: `halfedges` (tag 5).
///
/// `Mesh::from_proto` discards that map - topology is rebuilt from faces on the first edit - but
/// prost still decoded it into a nested `HashMap<u64, HashMap<u64, ..>>` first: 208k entries on
/// the bunny, allocated and dropped. An unlisted tag is skipped with a length jump instead.
/// Every other field keeps `proto::Mesh`'s exact type, so `into_proto` below MOVES them - no
/// copy, no second hash, and the kernel's own `from_proto` stays the single source of truth for
/// what a mesh means.
#[derive(Clone, PartialEq, prost::Message)]
pub struct LeanMesh {
    #[prost(string, tag = "1")]
    pub guid: String,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(map = "uint64, message", tag = "3")]
    pub vertices: std::collections::HashMap<u64, proto::VertexData>,
    #[prost(map = "uint64, message", tag = "4")]
    pub faces: std::collections::HashMap<u64, proto::FaceData>,
    // tag 5 (halfedges) intentionally absent - see the doc comment.
    #[prost(message, repeated, tag = "6")]
    pub edge_data: Vec<proto::EdgeData>,
    #[prost(btree_map = "string, double", tag = "7")]
    pub default_vertex_attributes: std::collections::BTreeMap<String, f64>,
    #[prost(btree_map = "string, double", tag = "8")]
    pub default_face_attributes: std::collections::BTreeMap<String, f64>,
    #[prost(btree_map = "string, double", tag = "9")]
    pub default_edge_attributes: std::collections::BTreeMap<String, f64>,
    #[prost(message, repeated, tag = "10")]
    pub pointcolors: Vec<proto::Color>,
    #[prost(message, repeated, tag = "11")]
    pub facecolors: Vec<proto::Color>,
    #[prost(message, repeated, tag = "12")]
    pub linecolors: Vec<proto::Color>,
    #[prost(double, repeated, tag = "13")]
    pub widths: Vec<f64>,
    #[prost(message, optional, tag = "15")]
    pub objectcolor: Option<proto::Color>,
    #[prost(int32, tag = "16")]
    pub color_mode: i32,
    #[prost(map = "uint64, message", tag = "17")]
    pub triangulation: std::collections::HashMap<u64, proto::TriList>,
}

impl LeanMesh {
    /// Hand the decoded fields to the kernel unchanged. `halfedges` is the empty map the kernel
    /// would have ignored anyway.
    pub fn into_proto_pub(self) -> proto::Mesh { self.into_proto() }

    fn into_proto(self) -> proto::Mesh {
        proto::Mesh {
            guid: self.guid,
            name: self.name,
            vertices: self.vertices,
            faces: self.faces,
            halfedges: Default::default(),
            edge_data: self.edge_data,
            default_vertex_attributes: self.default_vertex_attributes,
            default_face_attributes: self.default_face_attributes,
            default_edge_attributes: self.default_edge_attributes,
            pointcolors: self.pointcolors,
            facecolors: self.facecolors,
            linecolors: self.linecolors,
            widths: self.widths,
            objectcolor: self.objectcolor,
            color_mode: self.color_mode,
            triangulation: self.triangulation,
        }
    }
}

/// `proto::Objects` with the mesh lane swapped for [`LeanMesh`]; every other lane keeps the
/// generated type, so nothing else about the decode changes.
#[derive(Clone, PartialEq, prost::Message)]
pub struct LeanObjects {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(string, tag = "2")]
    pub guid: String,
    #[prost(message, repeated, tag = "3")]
    pub points: Vec<proto::Point>,
    #[prost(message, repeated, tag = "4")]
    pub lines: Vec<proto::Line>,
    #[prost(message, repeated, tag = "5")]
    pub planes: Vec<proto::Plane>,
    #[prost(message, repeated, tag = "6")]
    pub bboxes: Vec<proto::BoundingBox>,
    #[prost(message, repeated, tag = "7")]
    pub polylines: Vec<proto::Polyline>,
    #[prost(message, repeated, tag = "8")]
    pub pointclouds: Vec<proto::PointCloud>,
    #[prost(message, repeated, tag = "9")]
    pub meshes: Vec<LeanMesh>,
    #[prost(message, repeated, tag = "12")]
    pub nurbscurves: Vec<proto::NurbsCurve>,
    #[prost(message, repeated, tag = "13")]
    pub nurbssurfaces: Vec<proto::NurbsSurface>,
    #[prost(message, repeated, tag = "14")]
    pub breps: Vec<proto::BRep>,
    #[prost(message, repeated, tag = "15")]
    pub elements: Vec<proto::Element>,
}

/// The `Session` fields the viewer actually READS - same wire tags as `proto::Session`, so this
/// decodes the same bytes, but prost skips an unlisted field with a cheap length-delimited jump
/// instead of allocating it.
///
/// `tree` (tag 4) and `graph` (tag 5) are 21.7 MB of the 52 MB Treppenhaus sheet - 42% of the
/// file - and NOTHING in the viewer reads either one: the walk orders objects by
/// `Session::order()`, which is built from the object vectors, and `world_xforms()` consults the
/// tree only when `xforms` is non-empty. `TreeOnly` below covers exactly that case, and skipping
/// `objects` in turn makes it cheap.
/// Same shape, public, so the native `bench_load` harness can time this decode against the full
/// one. The loader below uses the private alias.
#[derive(Clone, PartialEq, prost::Message)]
pub struct LeanSessionProbe {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(string, tag = "2")]
    pub guid: String,
    #[prost(message, optional, tag = "3")]
    pub objects: Option<LeanObjects>,
    #[prost(message, repeated, tag = "7")]
    pub xforms: Vec<proto::XformEntry>,
}

/// Second pass for the rare file that carries local transforms: the tree, with the 30 MB of
/// objects skipped rather than decoded twice.
#[derive(Clone, PartialEq, prost::Message)]
pub struct TreeOnlyProbe {
    #[prost(message, optional, tag = "4")]
    pub tree: Option<proto::Tree>,
}

```

**Find** in `src/app/persistence.rs`:

```rust
    let Ok(p) = proto::Session::decode(bytes) else { return Session::default() };
```

**Replace with:**

```rust
    let Ok(p) = LeanSessionProbe::decode(bytes) else { return Session::default() };
```

**Find** in `src/app/persistence.rs`:

```rust
                let g = Rc::new($ty::from_proto(x));
                s.lookup.insert(g.guid().to_string(), Geometry::$variant(Rc::clone(&g)));
                s.objects.$slot.push(g);
                n += 1;
                if n % CHUNK == 0 { next_tick().await; }
            }
        };
```

**Add below it:**

```rust
        // the mesh lane arrives as LeanMesh (halfedges skipped); the kernel's from_proto still
        // does the building
        (lean $vec:expr, $ty:ident, $variant:ident, $slot:ident) => {
            for x in $vec {
                let g = Rc::new($ty::from_proto(x.into_proto()));
                s.lookup.insert(g.guid().to_string(), Geometry::$variant(Rc::clone(&g)));
                s.objects.$slot.push(g);
                n += 1;
                if n % CHUNK == 0 { next_tick().await; }
            }
        };
```

**Find** in `src/app/persistence.rs`:

```rust
        chunk!(o.meshes, Mesh, Mesh, meshes);
```

**Replace with:**

```rust
        chunk!(lean o.meshes, Mesh, Mesh, meshes);
```

**Find** in `src/app/persistence.rs`:

```rust
    // Tree / graph / xforms - rebuilt synchronously exactly as pb_loads does
```

**Replace with:**

```rust
    // Xforms first: they decide whether the tree is needed at all.
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

    // The tree is rebuilt ONLY to compose those transforms down the hierarchy - see
    // `Session::world_xforms`, which returns an empty map on the same test. A flat sheet or a
    // mesh file lands here with nothing to compose and pays neither the decode nor the 90k
    // Rc<RefCell<TreeNode>> allocations.
    if s.xforms.is_empty() {
        return s;
    }
    let p = match TreeOnlyProbe::decode(bytes) { Ok(t) => t, Err(_) => return s };
```

**Find** in `src/app/persistence.rs`:

```rust
    if let Some(gp) = &p.graph{
        s.graph = Graph::new(&gp.name);
        s.graph.set_guid(gp.guid.clone());
        for (name, v) in &gp.vertices {
            s.graph.add_node(name, &v.attribute);
        }
        for e in &gp.edges{
            s.graph.add_edge(&e.v0, &e.v1, &e.attribute);
        }
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
```

**Delete**

**Find** in `src/app/persistence.rs`:

```rust


    s
```

**Replace with:**

```rust

    s
```


## Step 9 — the page composites in order

A PDF sheet's fills are exactly coplanar: 362,581 vertices of a sheet, ONE distinct z. The
depth buffer cannot order them — equal depth fails a strict `Greater`, and the depths are not
even reliably equal, because positions are camera-relative and re-rounded to f32 every frame.
Whichever fill won flipped as the camera moved, and that flip is the flicker between lettering
and hatching.

So the sheet lanes draw through the same program with depth WRITE off. They are still
depth-TESTED, so 3D geometry in front still occludes the sheet; among themselves they composite
in draw order, which is what a page is.

**Find** in `src/engine/pipelines/build.rs`:

```rust
/// Pipeline for solid mesh triangles — reverse-Z depth (write on) + MSAA; reads mvp / time / instances.
```

**Replace with:**

```rust
/// Pipeline for solid mesh triangles — reverse-Z depth + MSAA; reads mvp / time / instances.
///
/// `depth_write` is off for the SHEET lanes. A drawing's fills are exactly coplanar (every vertex
/// of a PDF sheet sits at z = 0 - measured: 362,581 vertices, ONE distinct z), so the depth buffer
/// cannot order them: equal depth fails a strict `Greater`, and the depths are not even reliably
/// equal, because positions are camera-relative and re-rounded to f32 every frame. Whichever fill
/// won flipped as the camera moved - that flip is the flicker between lettering and hatching.
/// With no depth WRITE the fills stop arbitrating and composite in draw order, which is what a
/// page is: a painter's-algorithm document. They are still depth-TESTED, so 3D geometry in front
/// still occludes the sheet.
```

**Find** in `src/engine/pipelines/build.rs`:

```rust
    time_layout: &wgpu::BindGroupLayout,
    instance_layout: &wgpu::BindGroupLayout,
```

**Add below it:**

```rust
    depth_write: bool,
```

**Find** in `src/engine/pipelines/build.rs`:

```rust
            label: Some("triangle"),
```

**Replace with:**

```rust
            label: Some(if depth_write { "triangle" } else { "triangle.sheet" }),
```

**Find** in `src/engine/pipelines/build.rs`:

```rust
                depth_write_enabled: Some(true),
```

**Replace with:**

```rust
                depth_write_enabled: Some(depth_write),
```

**Find** in `src/engine/pipelines/build.rs`:

```rust
    })

}
```

**Add below it:**

```rust


```

**Find** in `src/engine/pipelines/build.rs`:

```rust
    splat_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline{
```

**Add below it:**

```rust

```

**Find** in `src/engine/pipelines/build.rs`:

```rust
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
        label: Some("splat.resolve.layout"),
        bind_group_layouts: &[Some(line_layout), Some(splat_layout)],
        immediate_size: 0,
    });
```

**Replace with:**

```rust
        }
    );

    let layout = device.create_pipeline_layout(
        &wgpu::PipelineLayoutDescriptor{
            label: Some("splat.resolve.layout"),
            bind_group_layouts: &[Some(line_layout), Some(splat_layout)],
            immediate_size: 0,
        }    
    );

```

**Find** in `src/engine/pipelines/build.rs`:

```rust
            targets: &[Some(wgpu::ColorTargetState{
                format: color_format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
        compilation_options: Default::default(),
```

**Replace with:**

```rust
            targets: &[
                Some(wgpu::ColorTargetState{
                    format: color_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL
                })
            ],
            compilation_options: Default::default(),
```

**Find** in `src/engine/pipelines/build.rs`:

```rust
            depth_compare: Some(wgpu::CompareFunction::Greater), // reverse-Z
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState{ count: samples, mask: !0, alpha_to_coverage_enabled: false},
        multiview_mask: None,
        cache: None,
    })
}
```

**Replace with:**

```rust
            depth_compare: Some(wgpu::CompareFunction::Greater), // reverse-Z
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: samples,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview_mask: None,
        cache: None,
    })
}
```

**Find** in `src/engine/pipelines/build.rs`:

```rust
            alpha_to_coverage_enabled: false,
        },
        multiview_mask: None,
        cache: None,
    })
}

```

**Replace with:**

```rust
            alpha_to_coverage_enabled: false,
        },
        multiview_mask: None,
        cache: None,
    })
}
```

**Find** in `src/engine/pipelines/mod.rs`:

```rust
use build::build_sphere_pipeline;

use build::build_ribbon_pipeline;
```

**Replace with:**

```rust
use build::build_sphere_pipeline;
use build::build_ribbon_pipeline;
```

**Find** in `src/engine/pipelines/mod.rs`:

```rust
    pub triangle: wgpu::RenderPipeline,
```

**Add below it:**

```rust
    /// Same program, depth WRITE off: the sheet lanes (print fills, then lettering) composite in
    /// draw order instead of fighting over one coplanar depth value. See build_triangle_pipeline.
    pub triangle_sheet: wgpu::RenderPipeline,
```

**Find** in `src/engine/pipelines/mod.rs`:

```rust
    pub sphere: wgpu::RenderPipeline,

    pub ribbon: wgpu::RenderPipeline,
```

**Replace with:**

```rust
    pub sphere: wgpu::RenderPipeline,
    pub ribbon: wgpu::RenderPipeline,
```

**Find** in `src/engine/pipelines/mod.rs`:

```rust
            triangle: build_triangle_pipeline(device, samples, color_format, aspect_layout, time_layout, instance_layout),
```

**Replace with:**

```rust
            triangle: build_triangle_pipeline(device, samples, color_format, aspect_layout, time_layout, instance_layout, true),
            triangle_sheet: build_triangle_pipeline(device, samples, color_format, aspect_layout, time_layout, instance_layout, false),
```

**Find** in `src/engine/pipelines/mod.rs`:

```rust
            sphere: build_sphere_pipeline(device, samples, color_format, aspect_layout, line_layout, instance_layout, glyph_layout),

            ribbon: build_ribbon_pipeline(device, samples, color_format, aspect_layout, line_layout, instance_layout, segment_layout),
```

**Replace with:**

```rust
            sphere: build_sphere_pipeline(device, samples, color_format, aspect_layout, line_layout, instance_layout, glyph_layout),
            ribbon: build_ribbon_pipeline(device, samples, color_format, aspect_layout, line_layout, instance_layout, segment_layout),
```


**Find** in `src/shaders/triangle.wgsl`:

```wgsl
    
}

```

**Add below it:**

```wgsl
/// The colour every back-facing triangle takes. Bright enough to read against the arctic
/// background at any lighting, dark enough not to bloom.
const BACKFACE_COLOR = vec3<f32>(0.80, 0.05, 0.05);

```

**Find** in `src/shaders/triangle.wgsl`:

```wgsl
    // Print (FLAG_PRINT) is paper, not surface: its authored colour is the final colour, and it
    // must read the same from the back of the sheet - where the flipped normal above collapses
    // lit to the 0.20 hemisphere floor - as from the front. Everything else keeps the model.
    return vec4<f32>(in.color * select(lit, 1.0, in.print > 0.5), 1.0);
```

**Replace with:**

```wgsl

    // A surface showing its BACK is either a flipped normal or a look inside an open solid, and
    // both are things to see rather than to shade smoothly - so the back reads red, always.
    // Print (FLAG_PRINT) is the one exclusion: a PDF sheet is paper, it has no inside, and it is
    // read from behind as often as from the front (see the paper branch below).
    let backface = !front && in.print <= 0.5;
    let base = select(in.color, BACKFACE_COLOR, backface);

    // Print (FLAG_PRINT) is paper, not surface: its authored colour is the final colour, and it
    // must read the same from the back of the sheet - where the flipped normal above collapses
    // lit to the 0.20 hemisphere floor - as from the front. Everything else keeps the model.
    return vec4<f32>(base * select(lit, 1.0, in.print > 0.5), 1.0);
```

**Find** in `src/shaders/ribbon.wgsl`:

```wgsl
const FLAG_OPEN: u32 = 16u;
```

**Add below it:**

```wgsl
// Instance::FLAG_SHEET - the row belongs to a planar drawing sheet (see gpu/mod.rs).
const FLAG_SHEET: u32 = 32u;
```

**Find** in `src/shaders/ribbon.wgsl`:

```wgsl
    if (line.ortho_h > 0.0) {
```

**Replace with:**

```wgsl
    // A SHEET takes no lift. The lift exists to keep a wireframe in front of the SURFACE it
    // decorates; a drawing's fills no longer write depth at all (they are exactly coplanar and
    // composite in document order), so there is no surface to clear - and a lifted pen would sit
    // in front of the lettering that the page draws on top of it. On the page, ink and letters
    // share the one plane and the ORDER decides, which is what a page means.
    let sheet = (instances[seg.instance_id].flags & FLAG_SHEET) != 0u;
    if (sheet) {
        // nothing to do: zn/wn stay the unlifted projection
    } else if (line.ortho_h > 0.0) {
```


## Step 10 — the splat shaders

Comment and struct cleanups in the two compute-splat shaders, plus the `ok` field that lets the
splat kernel bail without a branch in the caller.

**Find** in `src/shaders/splat.wgsl`:

```wgsl
// Compute-shader point splatting for the cloud lane (Schutz-style).
// One thread per point. Pass 1 (cs_depth): atomicMax the point's reverse-Z depth into a
// per-pixel u32 buffer for every pixel of its disc - bigger f32 bits = closer, and positive
// f32s compare correctly as u32s. Pass 2 (cs_color): re-project, and the thread whose depth
// WON a pixel stores its colour there. No rasterizer, no per-point vertices, no discard.

struct CloudUniform {
```

**Replace with:**

```wgsl
// Computer-shader point splatting for the cloud lane (Schutz-style https://github.com/m-schuetz/compute_rasterizer)
// One thread per point. Pass 1 (cs_depth): atomicMax the point's reverse-Z depth into a
// per-pixel u32 buffer for every pixel of its disc - bigger f32 bits = closer, and positive
// f32 compare correctly as u32s. Pass2 (cs_color): re-project, and the thread those depth
// won a pixel stores its colour there. No rasteriser, no per-point vertices, no discard.

// splat.wgsl is COMPUTE (cs_depth, cs_color). Compute shaders have no framebuffer
// they cannot draw a pixel to the screen or touch the depth attachment at all. 
// What they can do is hammer atomics into plain storage buffers, so they build a hand-made z-buffer: 
// sdepth (per-pixel winning reverse-Z bits via atomicMax) and scolor (the winner's colour). 
// That's the whole trick of the lane — the "rasterizer" is these two dispatches, and it runs in the compute prelude before the render pass.

struct CloudUniform{
```

**Find** in `src/shaders/splat.wgsl`:

```wgsl
// The record table is read as RAW WORDS - 4-word header {n, total, 0, 0}, then 20 words per
// record: 16 matrix (mvp x model, column-major) and {first, count, cum, rbits}. Raw
// indexing sidesteps every struct-layout question between Rust packing and WGSL rules.
```

**Replace with:**

```wgsl
// The record table is read as raw words - 4-word header {n, total, 0, 0}, then 20 words per record:
// 16 matrix (mvp x model, column-major) and {first, count, cum, rbits}.
// Raw indexing sidesteps every struct-layout question between Rust pacaking and WGSL rules.
```

**Find** in `src/shaders/splat.wgsl`:

```wgsl
struct Splat { px: vec2<i32>, r: f32, dbits: u32, color: u32, ok: bool };

fn rec_f(base: u32, w: u32) -> f32 { return bitcast<f32>(table[base + w]); }
```

**Replace with:**

```wgsl
struct Splat {
    px: vec2<i32>,
    r: f32,
    dbits: u32,
    color: u32,
    ok: bool,
};

fn rec_f(base: u32, w: u32) -> f32 {
    return bitcast<f32>(table[base + w]);
}
```

**Find** in `src/shaders/splat.wgsl`:

```wgsl
            if (atomicLoad(&sdepth[idx]) == s.dbits) { scolor[idx] = s.color; }
        }
    }
}

```

**Replace with:**

```wgsl
            if (atomicLoad(&sdepth[idx]) == s.dbits) { scolor[idx] = s.color; }
        }
    }
}
```

**Find** in `src/shaders/splat_resolve.wgsl`:

```wgsl
// Composite the splat buffers into the frame: ONE fullscreen triangle. Each fragment looks up
// its pixel; no splat (depth bits 0 = reverse-Z far) discards, a splat emits the colour and
// exports the splat's depth via frag_depth - so points and solids depth-test each other
// exactly, and later passes (markers, flat ink) see real cloud depth. frag_depth costs
// early-Z only for THIS one triangle, ~2M cheap fragments.

struct CloudUniform {
```

**Replace with:**

```wgsl
// Composite the splate buffers into frame: one fullscreen triangle.
// Each fragment looks up its pixel
// no splat (depth bits = 0 = reverse-Z far) discards, a splat emits the colour and 
// exports the splat's depth via frag_depth - so points and solids depth-test each other
// exactly, and late passes (markers, flat ink) see real cloud depth.
// frag_depths costs early-Z only this one triangle, ~2M cheap fragments.
// splat_resolve.wgsl is a RENDER shader (vs + fs). 
// Only a render pipeline can write the swapchain texture and the real depth buffer. 
// So one fullscreen triangle, drawn inside the render pass with the solids, looks up each pixel in those two storage buffers, 
// discards empties, emits the colour, and exports the splat's depth via frag_depth
//which is what lets splats and meshes occlude each other exactly.

struct CloudUniform{
```

**Find** in `src/shaders/splat_resolve.wgsl`:

```wgsl
    _pad: f32,
};
```

**Add below it:**

```wgsl

```

**Find** in `src/shaders/splat_resolve.wgsl`:

```wgsl
struct VsOut { @builtin(position) pos: vec4<f32> };
```

**Replace with:**

```wgsl
struct VsOut {
    @builtin(position) pos: vec4<f32>
};
```

**Find** in `src/shaders/splat_resolve.wgsl`:

```wgsl
    o.pos = vec4<f32>(x, y, 0.0, 1.0); // (-1,-1) (3,-1) (-1,3): one triangle covers the screen
```

**Replace with:**

```wgsl
    o.pos = vec4<f32>(x, y, 0.0, 1.0); // (-1, 1) (3, -1) (-1, 3): one triangle covers the screen
```

**Find** in `src/shaders/splat_resolve.wgsl`:

```wgsl
    o.depth = bitcast<f32>(d);
    return o;
}

```

**Replace with:**

```wgsl
    o.depth = bitcast<f32>(d);
    return o;
}
```


## Step 11 — plumbing

`add_file` gained a parameter, so the two harnesses and the message enum follow it, and
`performance.rs` learns to print what the new counters know.

**Find** in `src/engine/performance.rs`:

```rust
            log::info!("perf: {:.1} fps | {:.2} | {} draws | {} objects", fps, self.frame_ms, draws, objects);
```

**Replace with:**

```rust
            log::info!("perf: {:.1} fps | {:.2} | {} draws | {} objects | heap {:.0} MB", fps, self.frame_ms, draws, objects, heap_mb());
```

**Find** in `src/engine/performance.rs`:

```rust
    .as_secs_f64() * 1000.0
}

```

**Add below it:**

```rust
/// How much memory this viewer is holding, MB.
///
/// A wasm heap NEVER SHRINKS: `WebAssembly.Memory` only ever grows, and freeing a Vec hands the
/// pages back to the allocator, not to the browser. So this number is the high-water mark, which
/// is the honest budget - and printing it once a second is the only way to tell a scene that
/// costs 500 MB to LOAD from one that costs 500 MB to HOLD, or to catch a leak that adds a few
/// MB per frame.
#[cfg(target_arch = "wasm32")]
pub fn heap_mb() -> f64 {
    use wasm_bindgen::JsCast;
    wasm_bindgen::memory()
        .dyn_into::<js_sys::WebAssembly::Memory>()
        .ok()
        .map(|m| m.buffer().unchecked_into::<js_sys::ArrayBuffer>().byte_length() as f64 / 1.048576e6)
        .unwrap_or(0.0)
}

/// Native: resident set size from /proc, the closest thing to the same measure.
#[cfg(all(not(target_arch = "wasm32"), target_os = "linux"))]
pub fn heap_mb() -> f64 {
    std::fs::read_to_string("/proc/self/statm")
        .ok()
        .and_then(|s| s.split_whitespace().nth(1).and_then(|v| v.parse::<f64>().ok()))
        .map(|pages| pages * 4096.0 / 1.048576e6)
        .unwrap_or(0.0)
}

#[cfg(all(not(target_arch = "wasm32"), not(target_os = "linux")))]
pub fn heap_mb() -> f64 { 0.0 }

```

**Find** in `src/selftest.rs`:

```rust
pub fn render_scene(files: &[(&str, Xform, f32)], w: u32, h: u32, out: &str) -> String {
    let mut gpu = pollster::block_on(Gpu::new_headless(w, h)).expect("headless gpu");
    let mut scene = Scene::new();
```

**Replace with:**

```rust
pub fn render_scene(files: &[(&str, Xform, f32, bool)], w: u32, h: u32, out: &str) -> String {
    let mut gpu = pollster::block_on(Gpu::new_headless(w, h)).expect("headless gpu");
    let mut scene = Scene::new();
    let incremental = std::env::var("VIEWER_INCREMENTAL").is_ok();
```

**Find** in `src/selftest.rs`:

```rust
    for (path, place, px) in files {
```

**Replace with:**

```rust
    for (path, place, px, only) in files {
```

**Find** in `src/selftest.rs`:

```rust
        scene.add_file(name, session, place.clone(), *px);
        println!("  after walk into GPU tables: {:.1} MB | walk {:?}", rss_mb() - rss0, t0.elapsed() - t_read - t_decode);
```

**Replace with:**

```rust
        scene.add_file(name, session, place.clone(), *px, *only);
        println!("  after walk into GPU tables: {:.1} MB | walk {:?}", rss_mb() - rss0, t0.elapsed() - t_read - t_decode);
        // VIEWER_INCREMENTAL=1 uploads after EVERY file, which is what the browser does - each
        // fetched document is appended live. Batching all the files and uploading once (the
        // default here) hides exactly the cost that matters there: whether a lane re-sends the
        // whole scene per file or only the new rows.
        if incremental {
            let tu = std::time::Instant::now();
            scene.upload_to(&mut gpu);
            println!("  upload {:?} | RSS {:.1} MB", tu.elapsed(), rss_mb() - rss0);
        }
```

**Find** in `src/selftest.rs`:

```rust
    }

    scene.upload_to(&mut gpu);

    let mut camera = Camera::new();
```

**Replace with:**

```rust
    }

    if !incremental { scene.upload_to(&mut gpu); }

    // VIEWER_REBUILD=1 re-walks every document from its kernel Session and re-uploads from
    // scratch - the path a visibility toggle or a geometry edit takes. Every lane appends now, so
    // a rebuild has to REWIND every lane; forget one and the re-walked scene lands behind the copy
    // already on the GPU. The frame must come out pixel-identical to the same scene loaded once.
    if std::env::var("VIEWER_REBUILD").is_ok() {
        let t = std::time::Instant::now();
        scene.rebuild(&mut gpu);
        println!("rebuild {:?} | RSS {:.1} MB", t.elapsed(), rss_mb() - rss0);
    }

    let mut camera = Camera::new();
```

**Find** in `src/selftest.rs`:

```rust
        scene.add_file(name, session, place.clone(), 0.0);
```

**Replace with:**

```rust
        scene.add_file(name, session, place.clone(), 0.0, false);
```

**Find** in `src/lib.rs`:

```rust
    File(String, session_rust::Session, session_rust::Xform, f32),
```

**Replace with:**

```rust
    File(String, session_rust::Session, session_rust::Xform, f32, bool),
```

**Find** in `src/lib.rs`:

```rust
                        scene.add_file(name, session, place, item.point_size as f32);
```

**Replace with:**

```rust
                        scene.add_file(name, session, place, item.point_size as f32, item.display_only);
```

**Find** in `src/lib.rs`:

```rust
                        let _ = proxy.send_event(Msg::File(name, session, place, item.point_size as f32));
```

**Replace with:**

```rust
                        let _ = proxy.send_event(Msg::File(name, session, place, item.point_size as f32, item.display_only));
```

**Find** in `src/lib.rs`:

```rust
            Msg::File(name, session, place, cloud_px) => {
```

**Replace with:**

```rust
            Msg::File(name, session, place, cloud_px, display_only) => {
```

**Find** in `src/lib.rs`:

```rust
                state.scene.add_file(name, session, place, cloud_px);
                let t1 = crate::engine::performance::now_ms();
                state.scene.upload_to(&mut state.gpu);
                log::info!("appended: walk {:.0}ms · upload {:.0}ms | {} docs",
                    t1 - t0, crate::engine::performance::now_ms() - t1, state.scene.docs.len());
```

**Replace with:**

```rust
                state.scene.add_file(name, session, place, cloud_px, display_only);
                let t1 = crate::engine::performance::now_ms();
                state.scene.upload_to(&mut state.gpu);
                log::info!("appended: walk {:.0}ms · upload {:.0}ms | {} docs | heap {:.0} MB",
                    t1 - t0, crate::engine::performance::now_ms() - t1, state.scene.docs.len(),
                    crate::engine::performance::heap_mb());
```

**Find** in `examples/selftest.rs`:

```rust
    // A .json argument is a SCENE MANIFEST, not a mesh: resolve it the way the browser does, so
    // what the harness renders is what the viewer renders. Without this a manifest's placements
    // are silently dropped and every file lands at its own native origin and scale - which is how
    // a 0.156-unit bunny turns into an invisible speck sitting on a 1000 mm box.
    let mut owned: Vec<(String, session_rust::Xform, f32)> = Vec::new();
    for p in a.iter().skip(1) {
        if p.ends_with(".json") {
```

**Replace with:**

```rust
    // A .json/.toml argument is a SCENE MANIFEST, not a mesh: resolve it the way the browser does, so
    // what the harness renders is what the viewer renders. Without this a manifest's placements
    // are silently dropped and every file lands at its own native origin and scale - which is how
    // a 0.156-unit bunny turns into an invisible speck sitting on a 1000 mm box.
    let mut owned: Vec<(String, session_rust::Xform, f32, bool)> = Vec::new();
    for p in a.iter().skip(1) {
        if p.ends_with(".json") || p.ends_with(".toml") {
```

**Find** in `examples/selftest.rs`:

```rust
                owned.push((root.join(&item.file).to_string_lossy().into_owned(), place, item.point_size as f32));
            }
        } else {
            owned.push((p.clone(), session_rust::Xform::identity(), 0.0));
        }
    }
    let files: Vec<(&str, session_rust::Xform, f32)> =
        owned.iter().map(|(p, x, px)| (p.as_str(), x.clone(), *px)).collect();
```

**Replace with:**

```rust
                owned.push((root.join(&item.file).to_string_lossy().into_owned(), place, item.point_size as f32, item.display_only));
            }
        } else {
            owned.push((p.clone(), session_rust::Xform::identity(), 0.0, false));
        }
    }
    let files: Vec<(&str, session_rust::Xform, f32, bool)> =
        owned.iter().map(|(p, x, px, d)| (p.as_str(), x.clone(), *px, *d)).collect();
```

**Find** in `Cargo.toml`:

```toml
serde = { version = "1.0", features = ["derive"] }
```

**Add below it:**

```toml
toml = "0.8"
```

**Find** in `Cargo.toml`:

```toml
pollster = "0.4"

```

**Add below it:**

```toml
[[example]]
name = "bench_load"
path = "examples/bench_load.rs"

[[example]]
name = "mk_facing_probe"
path = "examples/mk_facing_probe.rs"

[[example]]
name = "check_lean"
path = "examples/check_lean.rs"

[[example]]
name = "check_determinism"
path = "examples/check_determinism.rs"

```


## What this does NOT fix

The arena's own vertex table still doubles when it grows, which on the biggest scene is a
127 MB copy. That is the right trade — the alternative is reserving for the worst case — but
it is the largest single allocation left in a load.

## Expected state

```
cargo run --release --example selftest -- out.ppm assets/scenes/bunny_drawings.toml
```
