# 40 The production splatter — tint, the static skip, and the trap

> Direct-path chain (36-42). Re-verified 2026-08-27 against the tree lesson 39 leaves
> behind: every anchor matches, the result compiles, `naga` validates the shader.

## Goal

Lesson [36](36-cloud-tables.md)'s splatter draws one cloud well. Put it in a real scene and
three things go wrong. This lesson fixes all three.

- **Tint.** An object's colour never reached its splats, so a cloud ignored whatever the
  scene told it to be.
- **The static skip.** A still camera re-splatted every point, every frame, to produce the
  picture that was already on screen.
- **The dispatch trap.** Past roughly 4 million points the frame stops drawing at all.

## The trap, first

Open the three-scan scene on lesson 36's code. The clouds sit frozen in screen space while
everything else orbits. Sometimes the frame is black. The console says why:

```
wgpu on_uncaptured_error: Dispatch workgroup count X (108600) exceeds max compute
workgroups per dimension (65535).
[Invalid CommandBuffer "clear encoder"] is invalid due to a previous error.
```

One dispatch may ask for **65,535 workgroups**, no more. At 64 threads each that covers
4.2 million points. A 7-million-point scene asks for more.

Asking for more does not clamp to the limit. It throws the whole command buffer away. The
frame never draws, so the screen keeps showing the last good one — which is why the clouds
look frozen rather than missing. Nothing on screen points at the dispatch. It reads like a
broken matrix, and that is where you will waste the afternoon.

A 2D grid fixes it: the same threads, laid out in rows instead of one long line.

## Step 1 — the shader: 24-word records, 2D indexing

Six find/replace pairs in `src/shaders/splat.wgsl`.

A record grows from 20 words to 24. The four new ones hold the tint and sit straight after
the matrix, so every offset past the matrix moves up by 4.

**Find**:

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

A cached frame is only safe while everything that feeds it holds still. Three places clear
the cache.

**Find** in `set_scene`. `resize` calls the same function one indent deeper, so this anchor
takes the line above it too:

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

**Find** the end of `rebuild_instances`. `update_inside_flags` writes the same buffer, but
it only ever changes flags — leave that one alone:

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

- `naga src/shaders/splat.wgsl` says `Validation successful`. The wasm check is clean.
- The lion render does not move: `non-background pixels: 189148 (19.7%)`. An untinted cloud
  tints by white, and the other two fixes change WHEN the work happens, not what it draws.
- The three-scan scene renders and orbits instead of freezing. The console error is gone.
- A still camera costs one resolve triangle. The compute prelude runs only when the camera,
  the scale, or the scene changed.

## Next

Lesson [41](41-potree-look.md) — **The Potree look: EDL and attenuated splats.** Eye-dome lighting gives an unlit cloud its shape back.
