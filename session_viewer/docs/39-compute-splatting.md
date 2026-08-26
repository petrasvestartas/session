# 39 The production splatter — tint, the static skip, and the trap

> Direct-path chain (36-41); every step below is replay-verified against a clean
> end-of-35 checkout.

## Goal

Lesson [36](36-cloud-tables.md)'s version-0 splatter draws one cloud beautifully and
falls over on a real scene, twice. This lesson fixes both: per-cloud TINT (the instance
colour never reached the splats), the STATIC SKIP (idle frames re-splatted millions of
points for nothing), and the dispatch trap — the best bug of this whole build.

## The trap, first

Load the three-scan stress scene on version 0 and the clouds FREEZE in screen space
while everything else orbits — or the frame goes black entirely. The console says why:

```
wgpu on_uncaptured_error: Dispatch workgroup count X (108600) exceeds max compute
workgroups per dimension (65535).
[Invalid CommandBuffer "clear encoder"] is invalid due to a previous error.
```

A 1D dispatch caps at **65,535 workgroups** — 4.2 M threads at 64/group, well under a
7 M-point frame — and an oversized dispatch does not clamp: it **invalidates the whole
command buffer**, so the frame silently never draws and the screen keeps showing the
LAST good frame. Frozen clouds, black offscreen renders, symptoms that masquerade as
matrix bugs. The fix is a 2D grid.

## Step 1 — the shader: 24-word records, 2D indexing

Six find/replace pairs in `src/shaders/splat.wgsl`.

**Find** (records grow a 4-word tint row, so every offset past the matrix moves by 4):

```wgsl
const REC_WORDS: u32 = 20u;
```

**Replace with:**

```wgsl
const REC_WORDS: u32 = 24u;
```

Fix the comment above it while you are there — the record is 24 words now, not 20: 16 matrix,
then **4 tint**, then `{first, count, cum, rbits}`.

**Find** (the record scan in `project`):

```wgsl
        let cum = table[b + 18u];
        let count = table[b + 17u];
        if (gid >= cum && gid < cum + count) { i = table[b + 16u] + (gid - cum); base = b; break; }
```

**Replace with:**

```wgsl
        let cum = table[b + 22u];
        let count = table[b + 21u];
        if (gid >= cum && gid < cum + count) { i = table[b + 20u] + (gid - cum); base = b; break; }
```

**Find**:

```wgsl
    s.r = bitcast<f32>(table[base + 19u]);
```

**Replace with:**

```wgsl
    s.r = bitcast<f32>(table[base + 23u]);
```

**Find**:

```wgsl
    s.color = colors[i];
```

**Replace with** (the tint is the instance colour, folded into the record):

```wgsl
    let tint = vec4<f32>(rec_f(base, 16u), rec_f(base, 17u), rec_f(base, 18u), 1.0);
    s.color = pack4x8unorm(unpack4x8unorm(colors[i]) * tint);
```

**Find** (the first entry point):

```wgsl
@compute @workgroup_size(64)
fn cs_depth(@builtin(global_invocation_id) g: vec3<u32>) {
    let s = project(g.x);
```

**Replace with:**

```wgsl
// Dispatched as a 2D grid: 4096 workgroups wide, as many rows as needed - a 1D dispatch
// caps at 65535 workgroups (4.2M threads), well under a 7M-point frame, and an oversized
// dispatch INVALIDATES the whole command buffer: the frame silently never draws.
const STRIDE: u32 = 4096u * 64u; // threads per grid row

@compute @workgroup_size(64)
fn cs_depth(@builtin(global_invocation_id) g: vec3<u32>) {
    let s = project(g.y * STRIDE + g.x);
```

**Find** (the second entry point):

```wgsl
fn cs_color(@builtin(global_invocation_id) g: vec3<u32>) {
    let s = project(g.x);
```

**Replace with:**

```wgsl
fn cs_color(@builtin(global_invocation_id) g: vec3<u32>) {
    let s = project(g.y * STRIDE + g.x);
```

## Step 2 — the CPU side: tint, skip, 2D

All in `src/engine/gpu/mod.rs`.

**Find** in the `Gpu` struct fields:

```rust
    splat_total: u32,
```

**Add below it:**

```rust
    splat_state: Option<([f32; 16], f32)>, // (mvp, cloud_size) the buffers were built for; None = stale
```

**Find** in the struct literal:

```rust
            splat_total: 0,
```

**Add below it:**

```rust
            splat_state: None,
```

**Find** in `encode_frame`'s record builder (the tint row goes between the matrix and
the meta words):

```rust
                    recs.extend_from_slice(bytemuck::cast_slice(&m));
                    recs.extend_from_slice(bytemuck::cast_slice(&[first, count, cum, (px * 0.5).to_bits()]));
```

**Replace with:**

```rust
                    recs.extend_from_slice(bytemuck::cast_slice(&m));
                    let tint = [row.color[0], row.color[1], row.color[2], 1.0f32];
                    recs.extend_from_slice(bytemuck::cast_slice(&tint));
                    recs.extend_from_slice(bytemuck::cast_slice(&[first, count, cum, (px * 0.5).to_bits()]));
```

**Find** (version 0's dispatch tail):

```rust
            if cum > 0 {
                self.queue.write_buffer(&self.splat_recs, 0, bytemuck::bytes_of(&header));
                self.queue.write_buffer(&self.splat_recs, 16, &recs);
                encoder.clear_buffer(&self.splat_depth_buf, 0, None); // 0 bits = reverse-Z far = empty
                encoder.clear_buffer(&self.splat_color_buf, 0, None);
                let groups = cum.div_ceil(64);
                let mut cp = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
                cp.set_bind_group(0, &self.splat_group0, &[]);
                cp.set_bind_group(1, &self.splat_group1, &[]);
                cp.set_pipeline(&self.splat_depth_pipeline);
                cp.dispatch_workgroups(groups, 1, 1);
                cp.set_pipeline(&self.splat_color_pipeline);
                cp.dispatch_workgroups(groups, 1, 1);
            }
```

**Replace with** (both fixes at once):

```rust
            // Static skip: camera still, same scale, nothing rebuilt - the buffers already
            // hold this exact frame's splats, so the whole compute prelude is free.
            let state = (self.mvp_f32, self.cloud_size);
            if cum > 0 && self.splat_state != Some(state) {
                self.queue.write_buffer(&self.splat_recs, 0, bytemuck::bytes_of(&header));
                self.queue.write_buffer(&self.splat_recs, 16, &recs);
                encoder.clear_buffer(&self.splat_depth_buf, 0, None); // 0 bits = reverse-Z far = empty
                encoder.clear_buffer(&self.splat_color_buf, 0, None);
                // 2D grid: a 1D dispatch caps at 65535 workgroups (~4.2M threads) and an
                // oversized dispatch invalidates the WHOLE command buffer - the frame
                // silently never draws. 4096-wide rows cover any point count.
                let groups = cum.div_ceil(64);
                let gx = groups.min(4096);
                let gy = groups.div_ceil(4096);
                let mut cp = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
                cp.set_bind_group(0, &self.splat_group0, &[]);
                cp.set_bind_group(1, &self.splat_group1, &[]);
                cp.set_pipeline(&self.splat_depth_pipeline);
                cp.dispatch_workgroups(gx, gy, 1);
                cp.set_pipeline(&self.splat_color_pipeline);
                cp.dispatch_workgroups(gx, gy, 1);
                self.splat_state = Some(state);
            }
```

## Step 3 — the invalidation hooks

The skip is only correct if every input change resets it. Three one-liners, same file.

**Find** in `set_scene` (the call right after the cloud-table appends — `resize` has the
same line one indent deeper, so take this one with its preceding line):

```rust
        self.cloud_draws.extend_from_slice(&up.cloud_draws);
        self.rebuild_splat_groups();
```

**Add below it:**

```rust
        self.splat_state = None;
```

**Find** in `resize` (one indent deeper):

```rust
            self.rebuild_splat_groups();
```

**Add below it:**

```rust
            self.splat_state = None;
```

**Find** the end of `rebuild_instances` (`update_inside_flags` writes the same buffer —
leave that one):

```rust
            self.instances[i].model = m;
        }
        self.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&self.instances));
```

**Add below it:**

```rust
        self.splat_state = None; // instance models moved - splats are stale
```

## Expected state

- `naga src/shaders/splat.wgsl`: `Validation successful`; wasm check clean.
- The lion render is unchanged: `non-background pixels: 189148 (19.7%)` — tint is white
  for an untinted cloud, and the skip/2D change WHEN work happens, not what it draws.
- The three-scan stress scene now renders and orbits instead of freezing; the console
  error above is gone.
- Idle frames cost the resolve triangle and nothing else — the compute prelude only runs
  when the camera, the scale, or the scene actually changed.
