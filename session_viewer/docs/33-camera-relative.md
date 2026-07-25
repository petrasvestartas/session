# 33 Camera-Relative — subtract the origin before the f32 cast

> **Big picture.** *Phase 4 — one scene, one draw call.* Before real files arrive (34), one precision
> landmine has to go: the kernel is f64, the GPU is f32, and a real project can sit kilometres from
> the world origin. This is the "floating origin" technique every large-world renderer uses — scenes
> near the origin draw exactly as before, but far ones stop shimmering. Skip it and the first real
> file at real coordinates would jitter on every orbit.

Move the demo scene ten thousand kilometres from the world origin and every edge starts to crawl —
not because the kernel is imprecise (it's f64 throughout), but because the GPU only ever sees f32,
and f32 runs out of digits exactly where the camera is looking.

## Why

f32 carries about 7 significant decimal digits, everywhere, all the time. At `x = 1e7` mm (10 km)
that budget is already spent on the integer part — the smallest step float32 can represent near
10,000,000 is roughly 1 mm. `Camera::position` is computed in f64 every frame (`update_position`),
but it gets cast to f32 the moment it reaches the GPU; two consecutive frames of an f64 orbit each
round to a different ~1 mm bucket, and the whole scene visibly shimmers. The kernel was never the
problem — the cast was. Fix: never cast a *big* number. Subtract the camera's own target (f64) from
everything — the view matrix's eye/target, and every instance's world position — so only the
*small* leftover (bounded by how far the camera actually is from what it's looking at) ever meets
`as f32`.

<svg viewBox="0 0 680 150" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="casting a huge f64 coordinate to f32 loses precision; subtracting the origin first keeps it exact" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="10" y="20" fill="#888">cast directly (f64 → f32)</text>
  <rect x="10" y="30" width="235" height="26" fill="none" stroke="#3a3a3a"/>
  <text x="20" y="47" fill="#d7dae0">10 000 000.4  (f64)</text>
  <text x="258" y="47" fill="#6fb3ff" font-size="14">▶</text>
  <rect x="292" y="30" width="235" height="26" fill="none" stroke="#5a4a2b"/>
  <text x="302" y="47" fill="#d7dae0">10 000 000.0  (f32)</text>
  <text x="540" y="47" fill="#888" font-size="10">±1 mm/frame — crawls</text>

  <text x="10" y="94" fill="#888">subtract origin FIRST (f64), then cast</text>
  <rect x="10" y="104" width="235" height="26" fill="none" stroke="#3a3a3a"/>
  <text x="20" y="121" fill="#d7dae0">10 000 000.4 − 10 000 000.0</text>
  <text x="258" y="121" fill="#6fb3ff" font-size="14">▶</text>
  <rect x="292" y="104" width="235" height="26" fill="none" stroke="#2b4a63"/>
  <text x="302" y="121" fill="#d7dae0">0.4  (f32, exact)</text>
  <text x="540" y="121" fill="#666" font-size="10">rock solid</text>
</svg>

## Files we touch

```
src/camera.rs     # origin() — the rebase point; view_proj() builds eye/target relative to it
# objects_base (TRUE absolute transforms) + rebuild_instances() (f64 rebase, f32 cast last)
src/engine/gpu.rs
src/state.rs      # render() threads camera.origin() into gpu.clear() alongside view_proj
```

Mesh vertices are already **local** — `Mesh::create_box` builds them centred on `(0,0,0)`; the demo
scene's world placement lives entirely in each object's `model` Xform (`Xform::translation(-2400.0,
0.0, 0.0)`, …). That half of "vertices stay local + per-object matrix" was true from lesson 30 on.
What's missing is keeping `model` itself small — today it holds the raw, possibly enormous, world
translation, cast straight to f32 in `Instance.model`.

## Step 1 — `origin()`: `src/camera.rs`

**1a. Find `pub fn view_proj` and add a new method directly above it:**

```rust
    /// Camera-relative origin: the world point the view matrix and every instance transform
    /// get rebased around before anything touches f32. Tracks `target` every frame — a
    /// coarser "only re-snap past N mm of drift" origin is a later optimization, not a
    /// different formula.
    pub fn origin(&self) -> Point {
        Point::new(self.target[0], self.target[1], self.target[2])
    }
```

**1b. Inside `view_proj`, replace the eye/target/up block.** Find:

```rust
        let eye = Point::new(self.position[0], self.position[1], self.position[2]);
        let target = Point::new(self.target[0], self.target[1], self.target[2]);
        let up = Vector::new(self.up[0], self.up[1], self.up[2]);
        let view = Xform::look_at_right_handed(&eye, &target, &up);
```

with:

```rust
        let origin = self.origin();
        let eye    = Point::new(self.position[0] - origin[0], self.position[1] - origin[1],
                                self.position[2] - origin[2]);
        let target = Point::new(self.target[0]   - origin[0], self.target[1]   - origin[1],
                                self.target[2]   - origin[2]);
        let up     = Vector::new(self.up[0], self.up[1], self.up[2]);
        let view   = Xform::look_at_right_handed(&eye, &target, &up);
```

`target − origin` is always `(0,0,0)` today — `origin` *is* `target` — but it's written as a
subtraction, not hardcoded to zero, so a future decoupled origin (re-snapped only past a drift
threshold, the way flight-sim "floating origin" engines do it) needs no change here, only a
different `origin()`. `eye − origin` is the part that matters: it's `position − target`, bounded by
`distance` (0.2–100 units, clamped), **never** by how far the scene sits from world `(0,0,0)`.

## Step 2 — keep the true transforms, rebase a copy every frame: `src/engine/gpu.rs`

`Gpu` currently bakes each object's absolute `model` straight into the GPU-facing `Instance` row
once, in `new()`, and never touches it again. Camera-relative rendering needs that absolute
transform to survive (the camera's `origin` moves every time you pan), so it can be re-rebased and
re-cast every frame.

**2a. Add `Point` to the kernel import** at the top of the file — only `Point` is new here
(`rebuild_instances` takes an `&Point`); the rest of the line is untouched:

```rust
use session_rust::{Mesh, BRep, Xform, RenderVertex, Point};
```

**2b. Store the absolute transforms.** Find the `instances` field on `struct Gpu` and add two after
it:

```rust
    instances: Vec<Instance>,
    // ← ADD — TRUE world model+color; instances[] is rebased FROM this
    objects_base: Vec<(Xform, [f32; 4])>,
    // ← ADD — new() builds this storage buffer as a LOCAL and drops it (only the bind
    //         group survives); rebuild_instances() reuploads into it every frame, so the
    //         buffer handle itself must live on Gpu, not vanish at the end of new()
    instance_buffer: wgpu::Buffer,
```

**2c. Keep a copy while building the arena.** Find the loop in `Gpu::new`
(`for (ri, (mesh, model, color)) in objects.into_iter().enumerate(){`) and its `instances` vec:

```rust
        let mut instances: Vec<Instance> = Vec::with_capacity(objects.len());
        let mut objects_base: Vec<(Xform, [f32; 4])> = Vec::with_capacity(objects.len());   // ← ADD

        for (ri, (mesh, model, color)) in objects.into_iter().enumerate(){
            objects_base.push((model.duplicate(), color));                                  // ← ADD
            instances.push(Instance{model: model.to_f32(), color, flags: 0, _pad: [0; 3]});
```

(`duplicate()` — Xform's own copy method — over `clone()`; it's what the kernel already uses when a
transform needs to live on past its original owner.) At startup `Camera::new()`'s `target` is
`[0,0,0]`, so `origin` is zero and this first, absolute upload is already correct for frame 0 —
`clear()` (Step 3) takes over from frame 1.

**2d. Store the fields.** Find the `Ok(Self { … })` initializer at the end of `new` and add both
new fields beside `instances,`:

```rust
            instances,
            objects_base,    // ← ADD
            instance_buffer, // ← ADD — was a dropped local in new(); now moved onto Gpu so
                             //         rebuild_instances() can write into it every frame
```

**2e. Add the rebase.** Right after `new` (before `pub fn resize`), add:

```rust
    /// Rebase every instance's translation around `origin` — an f64 subtract against the TRUE
    /// world transform in `objects_base`, THEN cast to f32. `instances` (what the GPU actually
    /// sees) never holds a coordinate bigger than the camera's distance from `origin`, no
    /// matter how far the scene sits from world (0,0,0).
    fn rebuild_instances(&mut self, origin: &Point) {
        let shift = Xform::translation(-origin[0], -origin[1], -origin[2]);
        for (i, (model, color)) in self.objects_base.iter().enumerate() {
            self.instances[i].model = (&shift * model).to_f32();   // f64 multiply, f32 cast LAST
            self.instances[i].color = *color;
        }
        self.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&self.instances));
    }
```

`&shift * model` is plain `session_rust::Xform` matrix multiplication (`impl Mul for &Xform`) — the
same operator `state.rs` already chains for `projection * view * scale`. No manual `[f64;16]`
poking, no external maths crate.

## Step 3 — call it from `clear()`: `src/engine/gpu.rs`

**3a.** Find the `clear` signature and its first two `write_buffer` calls:

```rust
    pub fn clear(&mut self, color: wgpu::Color, view_proj: &Xform) -> anyhow::Result<()> {

        // Time for triangle wgsl buffer.
        self.time += 1.0 / 60.0;
        self.queue.write_buffer(&self.time_buffer, 0, bytemuck::bytes_of(&self.time));
        self.queue.write_buffer(&self.mvp_buffer, 0, bytemuck::cast_slice(&view_proj.to_f32()));
```

and change them to:

```rust
    pub fn clear(&mut self, color: wgpu::Color, view_proj: &Xform,
                 origin: &Point) -> anyhow::Result<()> {

        // Time for triangle wgsl buffer.
        self.time += 1.0 / 60.0;
        self.queue.write_buffer(&self.time_buffer, 0, bytemuck::bytes_of(&self.time));
        // ← ADD — same origin as view_proj below
        self.rebuild_instances(origin);
        self.queue.write_buffer(&self.mvp_buffer, 0, bytemuck::cast_slice(&view_proj.to_f32()));
```

`view_proj` (Step 1) and `instances[].model` (Step 2e) must be built from the **same** `origin` —
otherwise `world = mvp · model · vertex` recombines two different rebasings and the scene tears
apart instead of jittering. Step 4 is what guarantees the same call supplies both.

<svg viewBox="0 0 560 150" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="the clip-space transform chain: view and model each subtract the same origin, so a vertex's clip position is built entirely from small numbers" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="10" y="20" fill="#888">clip = projection · view · model · vertex — the two −origin subtractions must agree</text>
  <text x="10" y="63" fill="#d7dae0">clip =</text>

  <rect x="60" y="44" width="92" height="30" fill="none" stroke="#3a3a3a"/>
  <text x="106" y="63" fill="#d7dae0" text-anchor="middle">projection</text>
  <text x="158" y="63" fill="#666" text-anchor="middle">·</text>

  <rect x="170" y="44" width="118" height="30" fill="none" stroke="#6fb3ff"/>
  <text x="229" y="63" fill="#6fb3ff" text-anchor="middle">view(−origin)</text>
  <text x="294" y="63" fill="#666" text-anchor="middle">·</text>

  <rect x="306" y="44" width="122" height="30" fill="none" stroke="#6fb3ff"/>
  <text x="367" y="63" fill="#6fb3ff" text-anchor="middle">model(−origin)</text>
  <text x="434" y="63" fill="#666" text-anchor="middle">·</text>

  <rect x="446" y="44" width="80" height="30" fill="none" stroke="#5bbf87"/>
  <text x="486" y="63" fill="#5bbf87" text-anchor="middle">vertex</text>

  <line x1="170" y1="88" x2="428" y2="88" stroke="#6fb3ff"/>
  <line x1="170" y1="88" x2="170" y2="82" stroke="#6fb3ff"/>
  <line x1="428" y1="88" x2="428" y2="82" stroke="#6fb3ff"/>
  <text x="299" y="104" fill="#6fb3ff" text-anchor="middle">both rebased by the SAME origin() (Step 4)</text>
  <text x="486" y="90" fill="#5bbf87" text-anchor="middle">already local</text>

  <text x="10" y="138" fill="#888">mismatched origins → the chain recombines two rebasings → scene tears, not jitters</text>
</svg>

> **Later:** orbiting and zooming never touch `target`, so most frames' `origin` is identical to the
> last one and this rewrite is wasted work — cache the last `origin`, skip `rebuild_instances` (and
> its `write_buffer`) when it hasn't moved. Free at 5 objects; worth doing once 34+ loads a real
> scene.

## Step 4 — thread it through: `src/state.rs`

Find `render`:

```rust
    pub fn render(&mut self) -> anyhow::Result<()> {
        self.window.request_redraw();
        let aspect = self.gpu.config.width as f64 / self.gpu.config.height as f64;
        let view_proj = self.camera.view_proj(aspect);
        self.gpu.clear(wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 }, &view_proj)
    }
```

and make **two** changes: a new `origin` line, **and** pass it as `clear`'s new third argument. Miss the
second and `clear` still gets 2 args → *"this method takes 3 arguments but 2 were supplied"*:

```rust
    pub fn render(&mut self) -> anyhow::Result<()> {
        self.window.request_redraw();
        let aspect = self.gpu.config.width as f64 / self.gpu.config.height as f64;
        let view_proj = self.camera.view_proj(aspect);
        let origin = self.camera.origin();                                                    // ← ADD (1/2)
        self.gpu.clear(wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 }, &view_proj, &origin)    // ← ADD &origin (2/2)
    }
```

## Cylinders and glyphs get this for free

31's cylinder shader and 32's sphere shader never read `Camera` or `origin` — they read
`instances[seg.instance_id].model`, exactly like the triangle arena:

```wgsl
let model = instances[seg.instance_id].model;
let w0 = (model * vec4<f32>(seg.p0, 1.0)).xyz;
```

Once that `model` row is origin-relative (Step 2e), `w0`/`w1` — and 32's `GlyphPoint` centres — land
near zero automatically. Zero shader changes in `cylinder.wgsl`, `sphere.wgsl`, or 32b's
`point.wgsl` (its billboard clouds read the same `instances[p.instance_id].model`); the fix lives
entirely in the one place all three pipelines already share.

## Step 5 — verify

Temporarily push the scene out to `x = 1e7` mm and point the camera at it — add a sixth object in
`Gpu::new`'s `objects` array, `(Mesh::create_box(600.0, 600.0, 600.0), Xform::translation(1.0e7, 0.0,
0.0), [1.0, 1.0, 1.0, 1.0])`, and set `Camera::new()`'s `target` to `[1.0e7, 0.0, 0.0]`. Orbit with
the mouse:

- **Comment out Step 1b's `- origin` terms** (back to raw `self.position[…]`/`self.target[…]`) and
  the box visibly crawls/shimmers a pixel or two as you orbit — the f32 cast rounding differently
  frame to frame.
- **Put them back** and the same orbit holds rock-solid, at any distance.

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Revert the temporary sixth object and target once you've seen both sides — the fix itself needs no
demo-scene change to ship.

## Recap

```
Ch 32: POINTS, two ways — sphere glyphs (handles/endpoints) and billboard circles (clouds), each
       one template + one row + one draw, reusing 31's instances[] and line uniform.
Ch 33: CAMERA-RELATIVE. origin() = camera target (f64). view_proj() now builds eye/target
       RELATIVE to it — look_at(eye−origin, target−origin, up) — so the view matrix's own
       numbers stay bounded by camera distance, never by how far target sits from world
       (0,0,0). Gpu keeps objects_base: the TRUE absolute model transforms; rebuild_instances()
       rebases a COPY of each one every frame — Xform::translation(-origin) * model, entirely in
       f64 — and casts to f32 only in the last line, right before write_buffer. Mesh vertices,
       31's cylinder segments, and 32's sphere/billboard glyphs needed ZERO shader changes: all
       three read instances[].model, which is origin-relative for free. No new draws — the fix
       is what the SAME instance buffer holds each frame, not a new pipeline.
```

Edited: `camera.rs` (`Camera::origin()`, `view_proj()` rebased eye/target), `engine/gpu.rs`
(`objects_base`, `rebuild_instances()`, `clear()` gains an `origin` parameter), `state.rs`
(`render()` passes `camera.origin()` through).

## Next

`34a-load-file.md` — the demo's five hardcoded meshes give way to the real kernel file format:
fetch bytes (or `<input type=file>`) → `Session` from `.pb`/`.json` → iterate its objects into the
arena and instance table via this chapter's and 31/32's adapters. Stress gate: a real PDF technical
drawing converted to a `.pb` — the first scene this viewer didn't build by hand.
