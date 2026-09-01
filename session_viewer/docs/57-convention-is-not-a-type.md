# 57 A naming convention is not a type

> Six structs lose twenty-one fields between them, and one six-parameter helper disappears.
> Nothing you can see changes.

## 1. The tell

Look for the same stem appearing with different suffixes:

```
mvp_buffer      mvp_bind_group
pos_cap         col_cap         nrm_cap
splat_group0    splat_group1    splat_resolve
buffer          on_gpu          cap
```

Every one of those is a type the code has not written down, spelled in a naming convention
instead. The convention works fine while you are reading the struct. It fails the moment you
want to PASS the thing: a convention cannot be passed, so the function grows one parameter per
field. That is exactly how `append_rows` came to take six - and its own doc comment said so,
naming a lesson to fix it that never happened:

> Six parameters, against the house limit of five, and deliberately so: three raw cloud lanes are
> still a loose (buffer, count, cap) triple rather than a `GrowBuf`.

`GrowBuf` already existed. `segments.rs`, `glyphs.rs` and `arena.rs` had adopted it. Three lanes
had not, and one helper carried the cost for all of them.

## 2. A point lane is three tables

Both point lanes - the walked one and the streamed one - are exactly three buffers: positions,
colours, normals. The walked lane held them as three buffers plus three capacities plus a count;
the streamed one as three buffers plus a shared capacity plus two cursors. Same shape, two
spellings, and `StreamLane::reserve` hand-rolled a second copy of the grow-and-copy that
`GrowBuf` already did.

**Find** in `src/engine/gpu/cloud.rs`:

```rust
use super::buffers::{GpuCtx, append_rows, zeroed_buffer};
```

**Replace with:**

```rust
use super::buffers::{GpuCtx, GrowBuf, zeroed_buffer};
```

**Find** in `src/engine/gpu/cloud.rs`:

```rust
pub struct CloudLane {
    pub(super) pos: wgpu::Buffer,
    pub(super) col: wgpu::Buffer,
    pub(super) nrm: wgpu::Buffer,
    pos_cap: u64,   // capacity in POINTS; the positions buffer holds three floats each
    col_cap: u64,
    nrm_cap: u64,
    count: u32,
```

**Replace with:**

```rust
/// A point lane's three tables, as one value.
///
/// Both point lanes - the walked one here and the streamed one in `stream.rs` - have exactly
/// these three buffers and nothing else that is a buffer. They were six loose fields each
/// (three buffers, three capacities) plus a count, and every append had to fake up a rows
/// counter per buffer, call a six-parameter helper three times, and divide to get the point
/// count back. Three `GrowBuf`s carry their own capacity and their own count, so the append is
/// three calls and no arithmetic.
///
/// The one asymmetry worth knowing: `pos` counts FLOATS - three per point - while `col` and
/// `nrm` count POINTS. That is why `points()` reads the colour table.
pub struct PointTables {
    pub pos: GrowBuf,
    pub col: GrowBuf,
    pub nrm: GrowBuf,
}

impl PointTables {
    pub fn new(device: &wgpu::Device, labels: [&'static str; 3]) -> Self {
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let mk = |label: &'static str, bytes: u64| GrowBuf {
            buf: zeroed_buffer(device, label, bytes, usage), count: 0, cap: 0, usage, label,
        };
        Self { pos: mk(labels[0], 12), col: mk(labels[1], 4), nrm: mk(labels[2], 4) }
    }

    /// Points on the GPU.
    pub fn points(&self) -> u32 {
        self.col.count
    }

    /// Append one file's rows to all three. `true` if any buffer was replaced, so the caller
    /// knows the bind groups pointing at them are stale.
    pub fn append(&mut self, ctx: &GpuCtx, pos: &[f32], col: &[u32], nrm: &[u32]) -> bool {
        let mut grew = self.pos.append(ctx, pos);
        grew |= self.col.append(ctx, col);
        grew |= self.nrm.append(ctx, nrm);
        grew
    }

    /// Room for `points` points in all three, written later at explicit cursors.
    pub fn reserve(&mut self, ctx: &GpuCtx, points: u64) -> bool {
        let mut grew = self.pos.reserve::<f32>(ctx, points * 3);
        grew |= self.col.reserve::<u32>(ctx, points);
        grew |= self.nrm.reserve::<u32>(ctx, points);
        grew
    }

    /// Rewind. Buffers and capacity stay; only the counters move.
    pub fn reset(&mut self) {
        self.pos.count = 0;
        self.col.count = 0;
        self.nrm.count = 0;
    }
}

pub struct CloudLane {
    pub(super) pts: PointTables,
```

**Find** in `src/engine/gpu/cloud.rs`:

```rust
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        Self {
            pos: zeroed_buffer(device, "points.buffer", 12, usage),
            col: zeroed_buffer(device, "points.col.buffer", 4, usage),
            nrm: zeroed_buffer(device, "points.nrm.buffer", 4, usage),
            pos_cap: 0,
            col_cap: 0,
            nrm_cap: 0,
            count: 0,
```

**Replace with:**

```rust
        Self {
            pts: PointTables::new(device, ["points.buffer", "points.col.buffer", "points.nrm.buffer"]),
```

**Find** in `src/engine/gpu/cloud.rs`:

```rust
    pub fn points(&self) -> u32 {
        self.count
    }
```

**Replace with:**

```rust
    pub fn points(&self) -> u32 {
        self.pts.points()
    }
```

**Find** in `src/engine/gpu/cloud.rs`:

```rust
        let mut pos_rows = self.count * 3;
        let mut grew = append_rows(ctx, "points.buffer", &mut self.pos, &mut pos_rows, &mut self.pos_cap, &up.pos);
        let mut col_rows = self.count;
        grew |= append_rows(ctx, "points.col.buffer", &mut self.col, &mut col_rows, &mut self.col_cap, &up.col);
        let mut nrm_rows = self.count;
        grew |= append_rows(ctx, "points.nrm.buffer", &mut self.nrm, &mut nrm_rows, &mut self.nrm_cap, &up.nrm);
        self.count = pos_rows / 3;
```

**Replace with:**

```rust
        let grew = self.pts.append(ctx, &up.pos, &up.col, &up.nrm);
```

**Find** in `src/engine/gpu/cloud.rs`:

```rust
        self.count = 0;
```

**Replace with:**

```rust
        self.pts.reset();
```

**Find** in `src/engine/gpu/stream.rs`:

```rust
use super::buffers::{GpuCtx, zeroed_buffer};
use super::cloud::CloudDraw;

/// The streamed point lane on the GPU, plus the write cursors one cloud's slices advance.
pub struct StreamLane {
    pub(super) pos: wgpu::Buffer,
    pub(super) col: wgpu::Buffer,
    pub(super) nrm: wgpu::Buffer,
```

**Replace with:**

```rust
use super::buffers::GpuCtx;
use super::cloud::{CloudDraw, PointTables};

/// The streamed point lane on the GPU, plus the write cursors one cloud's slices advance.
pub struct StreamLane {
    /// The same three tables the walked lane has, and the same value holding them. This lane
    /// writes at explicit cursors instead of appending, but the buffers, their capacity and
    /// their growth are identical - which is why the growth logic below is now one call rather
    /// than a second hand-rolled copy of it.
    pub(super) pts: PointTables,
```

**Find** in `src/engine/gpu/stream.rs`:

```rust
            pos: zeroed_buffer(device, "stream.pos", 12, usage),
            col: zeroed_buffer(device, "stream.col", 4, usage),
            nrm: zeroed_buffer(device, "stream.nrm", 4, usage),
```

**Replace with:**

```rust
            pts: PointTables::new(device, ["stream.pos", "stream.col", "stream.nrm"]),
```

**Find** in `src/engine/gpu/stream.rs`:

```rust
        let cap = need;
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let pos = zeroed_buffer(&ctx.device, "stream.pos", cap * 12, usage);
        let col = zeroed_buffer(&ctx.device, "stream.col", cap * 4, usage);
        let nrm = zeroed_buffer(&ctx.device, "stream.nrm", cap * 4, usage);
        if self.count > 0 {
            let mut enc = ctx.device.create_command_encoder(&Default::default());
            enc.copy_buffer_to_buffer(&self.pos, 0, &pos, 0, self.count as u64 * 12);
            enc.copy_buffer_to_buffer(&self.col, 0, &col, 0, self.count as u64 * 4);
            enc.copy_buffer_to_buffer(&self.nrm, 0, &nrm, 0, self.count as u64 * 4);
            ctx.queue.submit([enc.finish()]);
        }
```

**Replace with:**

```rust
        // The three buffers grow together, prefix copy included - `PointTables` does that.
        self.pts.pos.count = self.count * 3;
        self.pts.col.count = self.count;
        self.pts.nrm.count = self.count;
        self.pts.reserve(ctx, need);
```

**Find** in `src/engine/gpu/stream.rs`:

```rust
        while at < cap {
            let n = (cap - at).min(1 << 20) as usize;
            ctx.queue.write_buffer(&nrm, at * 4, bytemuck::cast_slice(&fill[..n]));
            ctx.queue.submit([]);
            at += n as u64;
        }
        self.pos = pos;
        self.col = col;
        self.nrm = nrm;
        self.capacity = cap;
```

**Replace with:**

```rust
        while at < need {
            let n = (need - at).min(1 << 20) as usize;
            ctx.queue.write_buffer(&self.pts.nrm.buf, at * 4, bytemuck::cast_slice(&fill[..n]));
            ctx.queue.submit([]);
            at += n as u64;
        }
        self.capacity = need;
```

**Find** in `src/engine/gpu/stream.rs`:

```rust
        ctx.queue.write_buffer(&self.pos, self.pos_at as u64 * 12, bytemuck::cast_slice(pos));
```

**Replace with:**

```rust
        ctx.queue.write_buffer(&self.pts.pos.buf, self.pos_at as u64 * 12, bytemuck::cast_slice(pos));
```

**Find** in `src/engine/gpu/stream.rs`:

```rust
        ctx.queue.write_buffer(&self.col, self.col_at as u64 * 4, bytemuck::cast_slice(col));
```

**Replace with:**

```rust
        ctx.queue.write_buffer(&self.pts.col.buf, self.col_at as u64 * 4, bytemuck::cast_slice(col));
```

## 3. The rows on the GPU are one value

**Find** in `src/engine/gpu/objects.rs`:

```rust
use super::buffers::{GpuCtx, append_rows, mk_rows_group, zeroed_buffer};
```

**Replace with:**

```rust
use super::buffers::{GpuCtx, GrowBuf, mk_rows_group, zeroed_buffer};
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
    pub(super) buffer: wgpu::Buffer,
    /// Rows already ON the buffer - the base for the next append.
    on_gpu: u32,
    cap: u64,
```

**Replace with:**

```rust
    /// The rows on the GPU: buffer, how many are up there, and how many fit. One value, because
    /// they only ever change together - and because a helper that took them apart needed six
    /// parameters to do it.
    pub(super) gpu: GrowBuf,
    /// Rows already ON the buffer - the base for the next append.
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
            buffer,
            on_gpu: 0,
            cap: 1,
```

**Replace with:**

```rust
            gpu: GrowBuf {
                buf: buffer, count: 0, cap: 1, label: BUFFER_LABEL,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
            },
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
            self.on_gpu = 0;
```

**Replace with:**

```rust
            self.gpu.count = 0;
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
        let fresh = &self.rows[self.on_gpu as usize..];
        if append_rows(ctx, BUFFER_LABEL, &mut self.buffer, &mut self.on_gpu, &mut self.cap, fresh) {
            self.bind_group = mk_rows_group(&ctx.device, &layouts.instance, GROUP_LABEL, &self.buffer);
```

**Replace with:**

```rust
        let fresh = &self.rows[self.gpu.count as usize..];
        if self.gpu.append(ctx, fresh) {
            self.bind_group = mk_rows_group(&ctx.device, &layouts.instance, GROUP_LABEL, &self.gpu.buf);
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
        }
        ctx.queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&self.rows));
    }
```

**Replace with:**

```rust
        }
        ctx.queue.write_buffer(&self.gpu.buf, 0, bytemuck::cast_slice(&self.rows));
    }
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
            ctx.queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&self.rows));
```

**Replace with:**

```rust
            ctx.queue.write_buffer(&self.gpu.buf, 0, bytemuck::cast_slice(&self.rows));
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
        self.on_gpu = 0;
```

**Replace with:**

```rust
        self.gpu.count = 0;
```

## 4. So the six-parameter helper has no reason to exist

**Find** in `src/engine/gpu/buffers.rs`:

```rust
}

/// Append rows to a growable STORAGE buffer: double the capacity when it runs out, move the
/// prefix GPU-side, and write only the new rows. Returns `true` when the buffer was replaced, so
/// the caller knows to rebuild the bind group pointing at it.
///
/// Six parameters, against the house limit of five, and deliberately so until 49: three raw
/// cloud lanes in `mod.rs` are still a loose (buffer, count, cap) triple rather than a `GrowBuf`.
/// Anything that HAS a `GrowBuf` calls `GrowBuf::append` above instead.
///
/// This is the same deal the mesh arena already struck, extended to the lanes that had not taken
/// it: a lane that rebuilds its whole buffer per file re-sends every earlier file's rows (five
/// files means the last one travels once and the first one five times), and it can only do that
/// because the CPU-side table is still there to re-send FROM - so the rows are held twice, in
/// wasm memory and on the GPU, for the whole session. On a 13.8 M-point scan that second copy is
/// 280 MB of browser heap.
pub fn append_rows<T: bytemuck::Pod>(
    ctx: &GpuCtx,
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
        let nb = zeroed_buffer(&ctx.device, label, new_cap * stride, usage);
        if *count > 0 {
            // the prefix moves GPU-side; it never travels back through wasm memory
            let mut enc = ctx.device.create_command_encoder(&Default::default());
            enc.copy_buffer_to_buffer(buf, 0, &nb, 0, *count as u64 * stride);
            ctx.queue.submit([enc.finish()]);
        }
        *buf = nb;
        *cap = new_cap;
        grew = true;
    }
    ctx.queue.write_buffer(buf, *count as u64 * stride, bytemuck::cast_slice(data));
    *count += data.len() as u32;
    grew
}
```

**Replace with:**

```rust

    /// Grow to hold at least `rows` rows, without writing any. The streamed lane needs this:
    /// it knows a cloud's point count before the points arrive, and then writes them at explicit
    /// cursors as the socket delivers them rather than appending.
    pub fn reserve<T: bytemuck::Pod>(&mut self, ctx: &GpuCtx, rows: u64) -> bool {
        if rows <= self.cap {
            return false;
        }
        let stride = std::mem::size_of::<T>() as u64;
        let new_cap = rows.max(self.cap * 2);
        let nb = zeroed_buffer(&ctx.device, self.label, new_cap * stride, self.usage);
        if self.count > 0 {
            let mut enc = ctx.device.create_command_encoder(&Default::default());
            enc.copy_buffer_to_buffer(&self.buf, 0, &nb, 0, self.count as u64 * stride);
            ctx.queue.submit([enc.finish()]);
        }
        self.buf = nb;
        self.cap = new_cap;
        true
    }
}

```

## 5. A uniform is its buffer and its bind group

They are created together, replaced together and bound together. Four of them sat as eight fields
distinguished by a prefix.

**Find** in `src/engine/gpu/frame.rs`:

```rust
/// The per-frame uniform blocks, as one value on `Gpu`.
pub struct FrameUniforms {
    pub(super) mvp_buffer: wgpu::Buffer,            // Camera matrix
    pub(super) mvp_bind_group: wgpu::BindGroup,     // Camera matrix
    pub(super) line_buffer: wgpu::Buffer, // shared: px-sizing for cylinders + spheres
    pub(super) line_bind_group: wgpu::BindGroup,
    pub(super) time: f32,  // shared: animation
    pub(super) time_buffer: wgpu::Buffer,
    pub(super) time_bind_group: wgpu::BindGroup,
    pub(super) cloud_buffer: wgpu::Buffer,
    pub(super) cloud_bind_group: wgpu::BindGroup,
```

**Replace with:**

```rust
/// One uniform block: the buffer, and the bind group that points at it.
///
/// They are created together, replaced together and bound together - there is no moment in a
/// frame where you want one without the other. Four of them sat here as eight fields named by a
/// prefix convention (`mvp_buffer` / `mvp_bind_group`), which is a struct spelled in field names.
pub struct Uniform {
    pub buf: wgpu::Buffer,
    pub group: wgpu::BindGroup,
}

impl Uniform {
    /// The buffer from `init`, and the group binding it at 0 - the shape all four share.
    fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, label: &'static str, init: &[u8]) -> Self {
        use wgpu::util::DeviceExt;
        let buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label), contents: init,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(label), layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: buf.as_entire_binding() }],
        });
        Self { buf, group }
    }
}

/// The per-frame uniform blocks, as one value on `Gpu`.
pub struct FrameUniforms {
    pub(super) mvp: Uniform,    // camera matrix
    pub(super) line: Uniform,   // px-sizing for cylinders + spheres
    pub(super) cloud: Uniform,  // the cloud lane's own size + viewport
    pub(super) time_u: Uniform, // animation
    pub(super) time: f32,
```

**Find** in `src/engine/gpu/frame.rs`:

```rust
        use wgpu::util::DeviceExt;

        // Camera MVP uniform - buffer + layout + bind group (group 0)
        let mvp_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("mvp.buffer"),
            contents: bytemuck::cast_slice(&Xform::identity().to_f32()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let mvp_bind_group: wgpu::BindGroup = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("mvp.bind_group"),
            layout: &layouts.mvp,
            entries: &[wgpu::BindGroupEntry{
                binding: 0,
                resource: mvp_buffer.as_entire_binding(),
            }],
        });

        // Time Uniform
        let time_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("time.buffer"),
            contents: bytemuck::bytes_of(&0.0f32),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let time_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("time.bind_group"),
            layout: &layouts.time,
            entries: &[wgpu::BindGroupEntry{ binding: 0, resource: time_buffer.as_entire_binding() }],
        });

        // Line uniform - scree-constant thickness
        let line_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("line.buffer"),
            contents: bytemuck::bytes_of(&LineUniform {
```

**Replace with:**

```rust
        // Four blocks, one shape: make a buffer, bind it at 0. This is a list of what the frame
        // holds, not eighty lines of create_buffer_init/create_bind_group pairs differing only in
        // the struct they carry.
        Self {
            mvp: Uniform::new(device, &layouts.mvp, "mvp",
                bytemuck::cast_slice(&Xform::identity().to_f32())),
            time_u: Uniform::new(device, &layouts.time, "time", bytemuck::bytes_of(&0.0f32)),
            line: Uniform::new(device, &layouts.line, "line", bytemuck::bytes_of(&LineUniform {
```

**Find** in `src/engine/gpu/frame.rs`:

```rust
                eye: [0.0; 3],   // no camera until the first frame writes one
                anchor: [0.0; 3],   // no anchor until the first frame rebases the table
                _pad1: 0.0,
            }),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let line_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("line.bind_group"),
            layout: &layouts.line,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: line_buffer.as_entire_binding()
            }],
        });

        // point cloud unioform - the cloud's OWN global size + viewport (reuses layouts.line)
        let cloud_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("cloud.buffer"),
            contents: bytemuck::bytes_of(&CloudUniform {
```

**Replace with:**

```rust
                eye: [0.0; 3],      // no camera until the first frame writes one
                anchor: [0.0; 3],   // no anchor until the first frame rebases the table
                _pad1: 0.0,
            })),
            cloud: Uniform::new(device, &layouts.line, "cloud", bytemuck::bytes_of(&CloudUniform {
```

**Find** in `src/engine/gpu/frame.rs`:

```rust
            }),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let cloud_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("cloud.bind_group"),
            layout: &layouts.line,
            entries: &[wgpu::BindGroupEntry {binding: 0, resource: cloud_buffer.as_entire_binding()}],
        });

        Self {
            mvp_buffer,
            mvp_bind_group,
            line_buffer,
            line_bind_group,
            time: 0.0,
            time_buffer,
            time_bind_group,
            cloud_buffer,
            cloud_bind_group,
```

**Replace with:**

```rust
            })),
            time: 0.0,
```

**Find** in `src/engine/gpu/frame.rs`:

```rust
        ctx.queue.write_buffer(&self.time_buffer, 0, bytemuck::bytes_of(&self.time));
        self.mvp_f32 = f.view_proj.to_f32();
        self.last_ortho_h = ortho_half_height(f.view_proj);
        self.last_eye = eye_from_view_proj(f.view_proj);
        ctx.queue.write_buffer(&self.mvp_buffer, 0, bytemuck::cast_slice(&self.mvp_f32));
```

**Replace with:**

```rust
        ctx.queue.write_buffer(&self.time_u.buf, 0, bytemuck::bytes_of(&self.time));
        self.mvp_f32 = f.view_proj.to_f32();
        self.last_ortho_h = ortho_half_height(f.view_proj);
        self.last_eye = eye_from_view_proj(f.view_proj);
        ctx.queue.write_buffer(&self.mvp.buf, 0, bytemuck::cast_slice(&self.mvp_f32));
```

**Find** in `src/engine/gpu/frame.rs`:

```rust
        ctx.queue.write_buffer(&self.line_buffer, 0, bytemuck::bytes_of(&line));
```

**Replace with:**

```rust
        ctx.queue.write_buffer(&self.line.buf, 0, bytemuck::bytes_of(&line));
```

**Find** in `src/engine/gpu/frame.rs`:

```rust
        ctx.queue.write_buffer(&self.cloud_buffer, 0, bytemuck::bytes_of(&CloudUniform{
```

**Replace with:**

```rust
        ctx.queue.write_buffer(&self.cloud.buf, 0, bytemuck::bytes_of(&CloudUniform{
```

**Find** in `src/engine/gpu/frame.rs`:

```rust
            mvp: &self.mvp_bind_group,
            time: &self.time_bind_group,
            line: &self.line_bind_group,
            cloud: &self.cloud_bind_group,
```

**Replace with:**

```rust
            mvp: &self.mvp.group,
            time: &self.time_u.group,
            line: &self.line.group,
            cloud: &self.cloud.group,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let splat = Splat::new(&device, &layouts, config.width, config.height,
            PointBufs { pos: &cloud.pos, col: &cloud.col, nrm: &cloud.nrm },
            PointBufs { pos: &stream.pos, col: &stream.col, nrm: &stream.nrm },
            SharedBufs { mvp: &frame.mvp_buffer, cloud: &frame.cloud_buffer, instances: &objects.buffer });

```

**Replace with:**

```rust
        let splat = Splat::new(&device, &layouts, config.width, config.height,
            PointBufs { pos: &cloud.pts.pos.buf, col: &cloud.pts.col.buf, nrm: &cloud.pts.nrm.buf },
            PointBufs { pos: &stream.pts.pos.buf, col: &stream.pts.col.buf, nrm: &stream.pts.nrm.buf },
            SharedBufs { mvp: &frame.mvp.buf, cloud: &frame.cloud.buf, instances: &objects.gpu.buf });

```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            PointBufs { pos: &cloud.pos, col: &cloud.col, nrm: &cloud.nrm },
            PointBufs { pos: &stream.pos, col: &stream.col, nrm: &stream.nrm },
            SharedBufs { mvp: &frame.mvp_buffer, cloud: &frame.cloud_buffer, instances: &objects.buffer });
```

**Replace with:**

```rust
            PointBufs { pos: &cloud.pts.pos.buf, col: &cloud.pts.col.buf, nrm: &cloud.pts.nrm.buf },
            PointBufs { pos: &stream.pts.pos.buf, col: &stream.pts.col.buf, nrm: &stream.pts.nrm.buf },
            SharedBufs { mvp: &frame.mvp.buf, cloud: &frame.cloud.buf, instances: &objects.gpu.buf });
```

## 6. And the splat lane's layouts and pipelines

These two were found by the RULE below rather than by reading - which is the point of writing
rules down.

**Find** in `src/engine/pipelines/layouts.rs`:

```rust
/// every pipeline and every rows bind group from these, so they must outlive both.
```

**Add below it:**

```rust
/// The splat lane's three bind-group layouts.
///
/// They were `splat_group0` / `splat_group1` / `splat_resolve` on `Layouts` - a prefix doing a
/// type's job. Grouped, the lane asks for `&layouts.splat` instead of three fields, and adding a
/// fourth is a change to one struct rather than to every signature that passes them along.
pub struct SplatLayouts {
    pub group0: wgpu::BindGroupLayout,
    pub group1: wgpu::BindGroupLayout,
    pub resolve: wgpu::BindGroupLayout,
}

```

**Find** in `src/engine/pipelines/layouts.rs`:

```rust
    pub splat_group0: wgpu::BindGroupLayout,
    pub splat_group1: wgpu::BindGroupLayout,
    /// The fullscreen resolve reads the two per-pixel buffers from the FRAGMENT stage.
    pub splat_resolve: wgpu::BindGroupLayout,
```

**Replace with:**

```rust
    /// The splat lane's three, as one value - see `SplatLayouts`.
    pub splat: SplatLayouts,
```

**Find** in `src/engine/pipelines/layouts.rs`:

```rust
        Self { mvp, time, instance, segment, glyph, line, splat_group0, splat_group1, splat_resolve }
```

**Replace with:**

```rust
        Self { mvp, time, instance, segment, glyph, line,
            splat: SplatLayouts { group0: splat_group0, group1: splat_group1, resolve: splat_resolve } }
```

**Find** in `src/engine/pipelines/mod.rs`:

```rust
/// `arena`, which is where lessons 49 and 50 take the other twelve.
```

**Add below it:**

```rust
/// The splat lane's three pipelines: one render pass that composites, and the two compute passes
/// that rasterize. Same reason as `SplatLayouts` - a shared prefix is a type in waiting.
pub struct SplatPipes {
    /// Fullscreen composite of the splat buffers.
    pub resolve: wgpu::RenderPipeline,
    pub depth: wgpu::ComputePipeline,
    pub color: wgpu::ComputePipeline,
}

```

**Find** in `src/engine/pipelines/mod.rs`:

```rust
    pub splat_resolve: wgpu::RenderPipeline, // fullscreen composite of the splat buffers
    // The splat rasterizer is COMPUTE: two passes over one shader, depth for every point first,
    // then colour for every point, composing into the two per-pixel atomics buffers.
    pub splat_depth: wgpu::ComputePipeline,
    pub splat_color: wgpu::ComputePipeline,
```

**Replace with:**

```rust
    /// The splat lane's three pipelines, as one value - see `SplatPipes`.
    pub splat: SplatPipes,
    // The splat rasterizer is COMPUTE: two passes over one shader, depth for every point first,
    // then colour for every point, composing into the two per-pixel atomics buffers.
```

**Find** in `src/engine/pipelines/mod.rs`:

```rust
            splat_resolve: build(device, t, &PipelineDesc::opaque("splat.resolve", SPLAT_RESOLVE, &[&l.line, &l.splat_resolve])),
            splat_depth: build_compute(device, "splat.depth", SPLAT, "cs_depth", &[&l.splat_group0, &l.splat_group1]),
            splat_color: build_compute(device, "splat.color", SPLAT, "cs_color", &[&l.splat_group0, &l.splat_group1]),
```

**Replace with:**

```rust
            splat: SplatPipes {
                resolve: build(device, t, &PipelineDesc::opaque("splat.resolve", SPLAT_RESOLVE, &[&l.line, &l.splat.resolve])),
                depth: build_compute(device, "splat.depth", SPLAT, "cs_depth", &[&l.splat.group0, &l.splat.group1]),
                color: build_compute(device, "splat.color", SPLAT, "cs_color", &[&l.splat.group0, &l.splat.group1]),
            },
```

## 7. The rule

A struct with three or more fields sharing a stem is a struct that has not been written down.

**Find** in `src/architecture.rs`:

```rust
/// All three are the same shape underneath: values that travel together are being passed apart.
const KNOWN_WIDE: &[(&str, usize, &str)] = &[
    ("engine/gpu/buffers.rs:88", 6,
     "append_rows takes buf/count/cap loose - which is exactly `GrowBuf`. The struct EXISTS; the \
      lanes that call this (cloud.rs, objects.rs) just do not use it, keeping parallel fields \
      instead. Fixing the signature means converting those lanes first."),
```

**Replace with:**

```rust
/// Both are the same shape underneath: values that travel together being passed apart.
/// The third entry - append_rows, taking buf/count/cap loose - is GONE: the three lanes that
/// forced it (cloud, stream, objects) now hold `GrowBuf`s, so it became `GrowBuf::append`.
const KNOWN_WIDE: &[(&str, usize, &str)] = &[
```

**Find** in `src/architecture.rs`:

```rust
    assert!(bad.is_empty(), "a file outgrew its budget:\n{}", bad.join("\n"));
}

```

**Add below it:**

```rust
/// The pattern behind every grouping in lesson 58: when the same stem keeps appearing with
/// different suffixes - `mvp_buffer`/`mvp_bind_group`, `pos_cap`/`col_cap`/`nrm_cap` - that is a
/// type asking to exist, spelled in a naming convention instead. A convention cannot be passed
/// to a function, so the function grows a parameter per field, which is how `append_rows` came to
/// take six.
///
/// Flags a struct holding three or more fields that share a stem and differ only by suffix.
#[test]
fn a_naming_convention_is_not_a_type() {
    let mut bad = Vec::new();
    for (rel, t) in all_rs() {
        let mut cur = String::new();
        let mut stems: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for line in t.lines() {
            if let Some(rest) = line.strip_prefix("pub struct ").or_else(|| line.strip_prefix("struct ")) {
                cur = rest.split(|c: char| !c.is_alphanumeric() && c != '_').next().unwrap_or("").to_string();
                stems.clear();
                continue;
            }
            if line == "}" && !cur.is_empty() {
                for (stem, fields) in &stems {
                    if fields.len() >= 3 && stem.len() >= 3 {
                        bad.push(format!("  {rel}: {cur} has {} fields sharing the stem `{stem}` ({}) — one value?",
                            fields.len(), fields.join(", ")));
                    }
                }
                cur.clear();
                continue;
            }
            if cur.is_empty() { continue }
            let f = line.trim().trim_start_matches("pub(super) ").trim_start_matches("pub(crate) ").trim_start_matches("pub ");
            if let Some(name) = f.split(':').next() {
                let name = name.trim();
                if name.is_empty() || !name.chars().all(|c| c.is_ascii_lowercase() || c == '_' || c.is_ascii_digit()) { continue }
                if let Some((stem, _)) = name.rsplit_once('_') {
                    stems.entry(stem.to_string()).or_default().push(name.to_string());
                }
            }
        }
    }
    assert!(bad.is_empty(), "a shared field-name stem is a struct that has not been written down:\n{}", bad.join("\n"));
}

```

## 8. Expected state

```
cargo xtest                                     9 passed, 0 failed
cargo check --target wasm32-unknown-unknown     0 errors
./docs/_gate.sh                                 gate OK
```

| struct | fields |
|---|---|
| `CloudLane` | 9 -> 3 |
| `StreamLane` | 8 -> 6 |
| `InstanceTable` | 11 -> 9 |
| `FrameUniforms` | 11 -> 7 |
| `Layouts` | 9 -> 7 |
| `Pipelines` | 7 -> 5 |

## Recap

When the same names keep appearing beside each other with different prefixes, that is a type
asking to exist. Writing it down is not tidying - it is what lets the thing be passed, which is
what stops the signatures growing.

## Next

Lesson [58](58-nurbscurve.md) - NurbsCurve.
