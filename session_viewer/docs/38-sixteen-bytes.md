# 38 Sixteen bytes a point — split the row, one draw per cloud

## Goal

Halve the GPU table. `CloudPoint` is 32 B; a scanned point needs 16. The GPU buffers for
the full 13.8M-point scan go **421 → 221 MB**, and the three-scan scene **323 → 162 MB**.

The split also turns out to be the thing that makes [39](39-streaming-cloud.md) possible,
which is the real reason to do it now rather than later.

## What is actually in the row

```rust
pub struct CloudPoint{
    position: [f32; 3], // 12 B  <- earns its place
    instance_id: u32,   //  4 B  <- the same number, 13.8 million times
    color: [f32; 4],    // 16 B  <- four floats to carry four BYTES
} // 32 B
```

Two thirds of that is waste, and each half has its own fix.

**`instance_id` is per-point and it never varies within a cloud.** There are three clouds
in this scene, not 13.8 million. It is in the row only because one shared table serves many
objects and the shader had no other way to know which instance a vertex belonged to.

**The colour is 8-bit at the source.** The proto carries `0-255` and the kernel stores
`i32`; `push_cloud` was widening that to four `f32`s — 16 B to hold 4 B of information.

## The layout

```
        BEFORE  one interleaved array          AFTER  two parallel arrays
        ────────────────────────────          ──────────────────────────
        array<CloudPoint>   32 B/pt           array<f32>   12 B/pt   positions
                                              array<u32>    4 B/pt   RGBA8
```

The stride question people get wrong here: in the **storage** address space `array<f32>`
has element stride 4 and `array<u32>` stride 4, so three floats really do cost 12 B. The
16-byte-stride rule you may be remembering applies to the **uniform** address space and to
`vec3`, and neither is in play. Index the array as `3*vid + k` and there is no padding
anywhere.

**Why split rather than one interleaved 16 B row?** Because `coords` and `colours` are
separate contiguous runs on the protobuf wire, and `queue.write_buffer` cannot do strided
writes. Interleaving means assembling rows on the CPU, which means holding both arrays at
once, which is exactly what 39 exists to stop doing. Two buffers cost one extra binding and
one extra sequential read stream on a lane that is rasterisation-bound anyway.

## Files we touch

| file | change |
|---|---|
| `src/engine/gpu/mod.rs` | two buffers, a `cloud_layout`, `CloudDraw`, a draw per cloud |
| `src/app/scene.rs` | `push_cloud` writes two arrays; bounds walk follows |
| `src/engine/pipelines/{mod,build}.rs` | the point pipeline binds the 2-entry layout |
| `src/shaders/point.wgsl` | two arrays, `instance_index`, `unpack4x8unorm` |

---

## Step 1 — the tables: `src/engine/gpu/mod.rs`

**Find** in `ArenaUpload`:

```rust
    pub points: Vec<CloudPoint>, // Raw lane: scanned clouds, one vertex and one pixel per point
```

**Replace with:**

```rust
    // Raw lane, SPLIT: 3 floats + 1 packed RGBA8 per point = 16 B, against CloudPoint's 32.
    pub cloud_pos: Vec<f32>,      // 3 per point
    pub cloud_col: Vec<u32>,      // RGBA8, 1 per point
    pub clouds: Vec<CloudDraw>,   // one entry per cloud - the instance rides here, not per point
```

and in `ArenaUpload::new()`, replace `points: Vec::new(),` with the three matching lines.

**Find** the `CloudPoint` struct and **replace the whole thing** — the comment above it
and the `#[repr(C)]`/`derive` attributes included — **with:**

```rust
// The raw cloud lane has no row STRUCT any more - it has two parallel arrays, 12 B of position
// and 4 B of packed RGBA8 per point. What is left per CLOUD is this, and only this.
#[derive(Clone, Copy)]
pub struct CloudDraw {
    pub base: u32,     // first point row, absolute in the shared buffers
    pub count: u32,    // how many points
    pub instance: u32, // which instance row - once per cloud instead of once per point
}
```

## Step 2 — one draw per cloud

This is the step that retires `instance_id`, and it rests on two WebGPU facts.

**`vertex_index` is absolute.** It counts from the draw's `first_vertex`, so
`draw(base..base+count, …)` gives the shader `vid` values that index straight into a shared
buffer holding every cloud. No per-point base, no offset uniform.

**`first_instance` lands on `instance_index`.** `draw(…, inst..inst+1)` makes
`@builtin(instance_index)` equal `inst` — so the cloud's instance row arrives once per draw
call instead of once per point. (wgpu only lowers its `instance_limit` below `u64::MAX` for
instance-rate *vertex* buffers, and this lane has none; the WebGPU spec gates non-zero
`firstInstance` only for **indirect** draws.)

**Find** the point draw block — the `if self.point_count > 0 { … }` from 36; the
"drawn WITH THE SOLIDS" comment above it stays — and **replace with:**

```rust
            if !self.cloud_draws.is_empty() {
                pass.set_pipeline(&self.pipelines.point);
                pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                pass.set_bind_group(1, &self.cloud_bind_group, &[]);
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.point_bind_group, &[]);
                // ONE draw per cloud, not one per point: the draw's first_vertex makes
                // vertex_index absolute into the shared buffers, and first_instance puts the
                // cloud's instance row on instance_index. That pair is what let the per-point
                // instance_id (4 B x 13.8M) leave the row.
                for c in &self.cloud_draws {
                    pass.draw(c.base..c.base + c.count, c.instance..c.instance + 1);
                    draws += 1;
                }
            }
```

## Step 3 — two buffers and their own layout

`glyph_layout` has one binding; the split cloud needs two, so it gets its own. In `Gpu::new`,
**replace** the single point-buffer creation with:

```rust
        let point_count = 0u32;
        let point_capacity = 1u64;
        let cloud_draws: Vec<CloudDraw> = Vec::new();

        let storage_entry = |binding: u32| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };
        let cloud_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("cloud.layout"),
            entries: &[storage_entry(0), storage_entry(1)],
        });

        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let point_pos_buffer = zeroed_buffer(&device, "cloud.pos", point_capacity * 12, usage);
        let point_col_buffer = zeroed_buffer(&device, "cloud.col", point_capacity * 4, usage);
        let point_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("points.bind_group"),
            layout: &cloud_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: point_pos_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: point_col_buffer.as_entire_binding() },
            ],
        });
```

Update the `Gpu` struct fields to match (`point_pos_buffer`, `point_col_buffer`,
`cloud_layout`, `cloud_draws`, keeping `point_count`/`point_capacity`), and add them to the
struct literal at the end of `new()`.

Thread the layout through: `Pipelines::new` takes a `cloud_layout: &wgpu::BindGroupLayout`,
passes it to `build_point_pipeline` in place of `glyph_layout`, and
`build_point_pipeline`'s `bind_group_layouts` array uses it at index 3. Both
`Pipelines::new` call sites in `gpu/mod.rs` pass `&cloud_layout` / `&self.cloud_layout`.

## Step 4 — the append, split in two

In `set_scene`, the point block becomes a pair of writes plus the draw records. Pull the
growth out into a helper, because [39](39-streaming-cloud.md) needs it too. **Add** to
`impl Gpu`, next to `set_scene`:

```rust
    /// Make room for `need` point rows total, copying the live prefix GPU-side.
    ///
    /// EXACT, not doubling: appends here are few and huge, so doubling would waste 122 MB of
    /// buffer on the three-scan scene AND take the worse worst-transient (668 MB of old+new
    /// live at once against 540 MB). What doubling avoids is a GPU-side copy - the one thing
    /// here that never touches wasm memory.
    fn cloud_reserve(&mut self, need: u64) {
        if need <= self.point_capacity { return; }
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let pos = zeroed_buffer(&self.device, "cloud.pos", need * 12, usage);
        let col = zeroed_buffer(&self.device, "cloud.col", need * 4, usage);
        if self.point_count > 0 {
            let mut enc = self.device.create_command_encoder(&Default::default());
            enc.copy_buffer_to_buffer(&self.point_pos_buffer, 0, &pos, 0, self.point_count as u64 * 12);
            enc.copy_buffer_to_buffer(&self.point_col_buffer, 0, &col, 0, self.point_count as u64 * 4);
            self.queue.submit([enc.finish()]);
        }
        self.point_pos_buffer = pos;
        self.point_col_buffer = col;
        self.point_capacity = need;
        self.point_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("points.bind_group"),
            layout: &self.cloud_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: self.point_pos_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: self.point_col_buffer.as_entire_binding() },
            ],
        });
    }
```

Then **replace** 37's whole `if !up.points.is_empty() { … }` block in `set_scene`
(keep the "READ THIS BEFORE THE CODE" comment above it) with:

```rust
        if !up.clouds.is_empty() {
            let need = self.point_count as u64 + (up.cloud_pos.len() / 3) as u64;
            self.cloud_reserve(need);
            self.queue.write_buffer(&self.point_pos_buffer, self.point_count as u64 * 12, bytemuck::cast_slice(&up.cloud_pos));
            self.queue.write_buffer(&self.point_col_buffer, self.point_count as u64 * 4, bytemuck::cast_slice(&up.cloud_col));
            // The delta's bases are relative to the delta; shift them into the shared buffers.
            for c in &up.clouds {
                self.cloud_draws.push(CloudDraw { base: self.point_count + c.base, count: c.count, instance: c.instance });
            }
            self.point_count += (up.cloud_pos.len() / 3) as u32;
        }
```

## Step 5 — `push_cloud` writes two arrays

**Replace** `push_cloud` with:

```rust
fn push_cloud(pc: &PointCloud, instance_id: u32, t: &mut ArenaUpload){
    let coords = pc.coords();
    let colors = pc.colors();
    let n = pc.len();
    let base = (t.cloud_pos.len() / 3) as u32;
    t.cloud_pos.reserve_exact(n * 3);
    t.cloud_col.reserve_exact(n);
    for i in 0..n {
        t.cloud_pos.push(coords[i * 3] as f32);
        t.cloud_pos.push(coords[i * 3 + 1] as f32);
        t.cloud_pos.push(coords[i * 3 + 2] as f32);
        let c = i * 4;
        // RGBA8 in one u32, little-endian byte order so the shader's unpack4x8unorm reads
        // x=r y=g b=b w=a. 4 B a point instead of four f32s: the colour is 8-bit at the source
        // (the proto carries 0-255) and it was being widened to 16 B for nothing.
        t.cloud_col.push(if c + 3 < colors.len() {
            ((colors[c] as u32) & 255)
                | (((colors[c + 1] as u32) & 255) << 8)
                | (((colors[c + 2] as u32) & 255) << 16)
                | (((colors[c + 3] as u32) & 255) << 24)
        } else {
            0xff00_0000
        });
    }
    t.clouds.push(CloudDraw { base, count: n as u32, instance: instance_id });
}
```

Three follow-ups in `src/app/scene.rs`, all mechanical:

- the import at the top: `CloudPoint` becomes `CloudDraw`;
- the call site in the match arm: `push_cloud(pc, ri, &mut t.points)` becomes
  `push_cloud(pc, ri, t)` — and mind the borrow: `t` is already the `&mut` you need;
- `Scene::upload_to`: the two `points` lines become all three tables —

```rust
        self.tables.cloud_pos.clear();
        self.tables.cloud_pos.shrink_to_fit();
        self.tables.cloud_col.clear();
        self.tables.cloud_col.shrink_to_fit();
        self.tables.clouds.clear();
        self.tables.clouds.shrink_to_fit();
```

In `Scene::rebuild`, the reset line 37 added grows a sibling — **find**
`gpu.point_count = 0;` and **add below it** `gpu.cloud_draws.clear();`.

And the bounds walk from 36 follows the data. **Replace** the `point0` mark with
`let cloud0 = self.tables.clouds.len();` and the `t.points` loop with:

```rust
        // Bases in the delta table are delta-relative (upload shifts them), so this walk
        // indexes t.cloud_pos directly - no point_count offset here.
        for c in t.clouds.iter().skip(cloud0){
            if let Some((xf, _, _)) = t.objects.get(c.instance as usize){
                for i in c.base as usize..(c.base + c.count) as usize {
                    let p = [t.cloud_pos[i * 3], t.cloud_pos[i * 3 + 1], t.cloud_pos[i * 3 + 2]];
                    grow_bounds(&mut fmin, &mut fmax, xform_point(xf, p));
                }
            }
        }
```

## Step 6 — the shader

Group 3 becomes two bindings, and the vertex entry point gains `instance_index`:

```wgsl
@group(3) @binding(0) var<storage, read> positions: array<f32>;  // 3 per point
@group(3) @binding(1) var<storage, read> colors: array<u32>;     // RGBA8, 1 per point

@vertex
fn vs_main(@builtin(vertex_index) vid: u32, @builtin(instance_index) iid: u32) -> VsOut{
    let i = vid * 3u;
    let local = vec3<f32>(positions[i], positions[i + 1u], positions[i + 2u]);
    let inst = instances[iid];
    let world = (inst.model * vec4<f32>(local, 1.0)).xyz;

    var o: VsOut;
    o.pos = mvp * vec4<f32>(world, 1.0);
    if ((inst.flags & FLAG_HIDDEN) != 0u) {
        o.pos = vec4<f32>(0.0, 0.0, -1.0, 1.0);
    }
    o.color = unpack4x8unorm(colors[vid]) * inst.color;
    return o;
}
```

`unpack4x8unorm` is a WGSL builtin: one `u32` in, a `vec4<f32>` of `0..1` out, in the byte
order `push_cloud` packed. The `CloudPoint` struct declaration goes away entirely.

## Verify

```bash
naga src/shaders/point.wgsl        # or: naga --stdin-file-path point.wgsl < point.wgsl
cargo check --target wasm32-unknown-unknown
```

Then load the scene. The picture must be **identical** — same points, same colours, same
frame rate. What changed is only the arithmetic:

| | before | after |
|---|---|---|
| bytes/point | 32 | **16** |
| GPU buffers, 3 scans | 323 MB | **162 MB** |
| GPU buffers, 14M scan | 421 MB | **221 MB** |
| draw calls for the lane | 1 | one per cloud (3) |

If colours come out wrong, the byte order in the `u32` pack is reversed — `unpack4x8unorm`
reads the **low** byte as `x`.

## Recap

```
Ch 37:  the copies stopped multiplying, but a point still cost 32 B on the GPU: 12 B of
        position, 4 B of an instance id that is the same for every point in a cloud, and
        16 B of float holding 4 B of colour.
Ch 38:  two parallel arrays instead of one struct - array<f32> at 12 B and array<u32> RGBA8
        at 4 B, both stride-4 in the STORAGE address space (the 16-byte rule is a UNIFORM
        rule). The per-point instance_id leaves entirely: ONE draw per cloud, where
        first_vertex makes vertex_index absolute into the shared buffers and first_instance
        puts the cloud's row on instance_index. Colour unpacks in the shader with
        unpack4x8unorm. 32 B -> 16 B, and the split is precisely what lets 39 stream a file
        in one forward pass.
```

## Next

[`39-streaming-cloud.md`](39-streaming-cloud.md) — the GPU side is now as small as it is
going to get. The peak is still a gigabyte, because the file still exists whole in wasm
memory before any of it reaches the GPU. That is the last copy.
