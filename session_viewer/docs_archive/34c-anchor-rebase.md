# 34c Floating anchor — pan for the price of one uniform

> **Big picture.** The 42k-object PDF drawing loaded… at 0.1 fps. Not the GPU — it drew the same
> triangles at 120 fps moments later. The wall was lesson 33's per-frame rebase:
> `rebuild_instances` re-multiplied every object's f64 matrix and re-uploaded the whole 4MB
> instance table EVERY frame, "to be safe". 491 objects hid it; 42,232 made it a 300ms/frame CPU
> loop feeding an idle GPU. The fix is what every large-world engine ships (Cesium, Unreal LWC):
> a **floating anchor** — instances stay rebased about a *snapped* anchor; the camera's drift goes
> into the VIEW matrix (64 bytes); a full rebuild happens only when the target strays far.

<svg viewBox="0 0 680 96" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="camera drift is absorbed by the view matrix; the instance table only rebuilds when drift exceeds the re-anchor distance" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g stroke="#6fb3ff" stroke-width="1.5" fill="none">
    <rect x="10"  y="24" width="180" height="34"/>
    <rect x="250" y="24" width="180" height="34"/>
    <rect x="490" y="24" width="180" height="34"/>
  </g>
  <g fill="#d7dae0" text-anchor="middle">
    <text x="100" y="45">anchor (snapped)</text>
    <text x="340" y="45">drift &lt; 100m → view matrix</text>
    <text x="580" y="45">drift &gt; 100m → re-anchor</text>
  </g>
  <text x="340" y="82" fill="#888" text-anchor="middle">orbit never moves the target · pan/zoom drift is ONE 64-byte uniform · rebuild is the rare case</text>
</svg>

## Files we touch

```
src/engine/gpu/mod.rs   # Steps 1-2: consts to the top, REANCHOR_DIST, last_origin, rebase_anchor,
                        #            rebuild loses the f64 multiply, clear() stops rebuilding
src/camera.rs           # Step 3: view_proj_anchored — camera-relative to a caller-supplied anchor
src/state.rs            # Step 4: render(): anchor dance before clear
```

## Step 1 — housekeeping first: the template consts move to the top of `gpu/mod.rs`

Right now `CYL_SIDES` hides at the BOTTOM of the file (above `fn unit_cylinder`) and
`SPH_LONS`/`SPH_LATS` below that — this lesson adds a third const, so gather them all at the top
where the next lessons expect them.

**1a.** Near the bottom of the file, under the `/// Primitives` banner. The
`/// Unit-cylinder template mesh…` doc comment below it stays, on top of `fn unit_cylinder`.

**Find** in `src/engine/gpu/mod.rs`:

```rust
const CYL_SIDES: u32 = 12;
```

**Delete**

**1b.** A screen further down, between `fn unit_cylinder`'s closing `}` and the
`// Unit sphere on the origin…` comment.

**Find** in `src/engine/gpu/mod.rs`:

```rust
const SPH_LONS: usize = 12;
const SPH_LATS: usize = 6;
```

**Delete**

**1c.** At the TOP of the file, the import block's last line. The three consts return below it,
plus this lesson's new one.

**Find** in `src/engine/gpu/mod.rs`:

```rust
use session_rust::{Mesh, Xform, RenderVertex, Point, Geometry};
```

**Add below it:**

```rust

/// Re-anchor distance: the instance table is rebased about a snapped anchor.
/// The camera can drift this far (mm) before a full rebuild.
/// Within it, pan/zoom only changes the view matrix.
/// f32 error at 1e5 mm from the anchor = 6e-3 mm - far below a pixel.
const REANCHOR_DIST: f64 = 1.0e5;

/// const for the unit_cylinder method
const CYL_SIDES: u32 = 12;

/// const for the unit_sphere method
const SPH_LONS: usize = 12;
const SPH_LATS: usize = 6;
```

## Step 2 — the anchor state + the 20× cheaper rebuild: `gpu/mod.rs`

**2a. The field.** In `pub struct Gpu`, between `instances` and the `objects_base` line under it.

**Find** in `src/engine/gpu/mod.rs`:

```rust
    instances: Vec<Instance>,
```

**Add below it:**

```rust
    last_origin: Option<Point>, // rebuild_instances skips when the camera target did not move
```

**2b. The initializer.** In the `Ok(Self { … })` at the end of `new()`, between `instances,` and
the `objects_base,` line under it.

**Find** in `src/engine/gpu/mod.rs`:

```rust
            instances,
```

**Add below it:**

```rust
            last_origin: None,
```

**2c. The public entry.** The seam between `new()`'s end and `rebuild_instances`: the new method
goes between `new()`'s closing `}` and `rebuild_instances`'s doc comment, which is the anchor.

**Find** in `src/engine/gpu/mod.rs`:

```rust
    /// Rebase every instance's translation around 'origin' - an f64 subtract agains the TRUE world transfrom in 'objects_base'
```

**Add above it:**

```rust
    /// The anchor the instance table is rebased about.
    /// A full rebuild (42 000 x at stress scale) runs only when the camera target strays
    /// REANCHOR_DIST from the current anchor - orbit never moves the target,
    /// and pan/zoom within the budget just changes the view matrix.
    pub fn rebase_anchor(&mut self, origin: &Point) -> Point{
        let need = match &self.last_origin {
            None => true,
            Some(a) => {
                let (dx, dy, dz) = (a[0] - origin[0], a[1] - origin[1], a[2] - origin[2]);
                (dx * dx + dy * dy + dz * dz).sqrt() > REANCHOR_DIST
            }
        };
        if need {
            self.rebuild_instances(origin);
        }
        self.last_origin.clone().unwrap()
    }

```

**2d. The rebuild gets 20× cheaper.** `T(-origin) × M` only changes the TRANSLATION column of a
column-major matrix — the old code paid a full 4×4 f64 multiply per object for three
subtractions' worth of change.

**Find** in `src/engine/gpu/mod.rs`:

```rust
    fn rebuild_instances(&mut self, origin: &Point){
        let shift = Xform::translation(-origin[0], -origin[1], -origin[2]);
        for (i, (model, color)) in self.objects_base.iter().enumerate() {
            self.instances[i].model = (&shift * model).to_f32(); // f64 multiply, f32 cast last
            self.instances[i].color = *color;
        }
        self.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&self.instances));
    }
```

Keep the f64-subtract-then-cast order — that's lesson 33's precision guarantee; it also records
the anchor now.

**Replace with**:

```rust
    fn rebuild_instances(&mut self, origin: &Point){
        self.last_origin = Some(origin.clone());
        for (i, (model, color)) in self.objects_base.iter().enumerate() {
            let mut m = model.to_f32();
            m[12] = (model.m[12] - origin[0]) as f32; // f64 subtract, f32 cast last
            m[13] = (model.m[13] - origin[1]) as f32;
            m[14] = (model.m[14] - origin[2]) as f32;
            self.instances[i].model = m;
            self.instances[i].color = *color;
        }
        self.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&self.instances));
    }
```

**2e. `clear()` stops rebuilding.** Inside `pub fn clear`, the line lesson 33 added between the
time write and the mvp write.

**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.rebuild_instances(origin); // Make sure objects are displayed within limits, we rebuild buffers here to avoid camera wiggle!
```

**Delete**

Rebasing is `rebase_anchor`'s job now, called from `render()` (Step 4).
`clear()`'s signature keeps its (now unused) `origin: &Point` parameter — expect an
`unused variable: origin` warning from here on; `state.rs` keeps passing `&origin`, so no call
site changes.

## Step 3 — the camera renders about the anchor: `src/camera.rs`

Lesson 33's `view_proj` built eye/target relative to `origin()` (the target). Generalize it.

**Find** in `src/camera.rs`:

```rust
    pub fn view_proj(&self, aspect: f64) -> Xform {
        let dist = self.distance;
        let projection = if self.perspective {
            Xform::perspective(f64::to_radians(60.0), aspect, dist * 10.0, dist * 0.01)
        } else {
            let h = dist * f64::to_radians(30.0).tan();
            let r = dist * 100.0;
            Xform::orthographic(-aspect * h, aspect * h, -h, h, r, -r)
        };

        let origin = self.origin();
        let eye    = Point::new(self.position[0] - origin[0], self.position[1] - origin[1], self.position[2] - origin[2]);
        let target = Point::new(self.target[0]   - origin[0], self.target[1]   - origin[1], self.target[2]   - origin[2]);
        let up     = Vector::new(self.up[0], self.up[1], self.up[2]);
        let view   = Xform::look_at_right_handed(&eye, &target, &up);

        // units
        let s = self.unit.to_meters();
        let scale = Xform::scale_xyz(s, s, s);

        projection * view * scale
    }
```

A wrapper plus the anchored variant takes its place — the body is the same, with `origin` renamed
to the `anchor` parameter and the `let origin = self.origin();` line gone.

**Replace with**:

```rust
    pub fn view_proj(&self, aspect: f64) -> Xform {
        self.view_proj_anchored(aspect, &self.origin())
    }

    /// Camera-relative to a caller-supplied ANCHOR instead of the target.
    /// Instances rebased about the same anchor stay valid while the target drifts (pan/zoom) —
    /// panning then costs ONE uniform write instead of an instance-table rebuild.
    pub fn view_proj_anchored(&self, aspect: f64, anchor: &Point) -> Xform {
        let dist = self.distance;
        let projection = if self.perspective {
            Xform::perspective(f64::to_radians(60.0), aspect, dist * 10.0, dist * 0.01)
        } else {
            let h = dist * f64::to_radians(30.0).tan();
            let r = dist * 100.0;
            Xform::orthographic(-aspect * h, aspect * h, -h, h, r, -r)
        };

        let eye    = Point::new(self.position[0] - anchor[0], self.position[1] - anchor[1], self.position[2] - anchor[2]);
        let target = Point::new(self.target[0]   - anchor[0], self.target[1]   - anchor[1], self.target[2]   - anchor[2]);
        let up     = Vector::new(self.up[0], self.up[1], self.up[2]);
        let view   = Xform::look_at_right_handed(&eye, &target, &up);

        // units
        let s = self.unit.to_meters();
        let scale = Xform::scale_xyz(s, s, s);

        projection * view * scale
    }
```

## Step 4 — the wiring: `src/state.rs` `render()`

Two lines go between `let origin = …;` and the `clear` call, so the tail of `render()` reads as
below.

**Find** in `src/state.rs`:

```rust
        let view_proj = self.camera.view_proj(aspect);
        let origin = self.camera.origin();
        self.gpu.clear(wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 }, &view_proj, &origin)
    }
```

**Replace with**:

```rust
        let view_proj = self.camera.view_proj(aspect);
        let origin = self.camera.origin();
        let anchor = self.gpu.rebase_anchor(&origin);
        let view_proj = self.camera.view_proj_anchored(aspect, &anchor);
        self.gpu.clear(wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 }, &view_proj, &origin)
    }
```

The second `let view_proj` shadows the first — the anchored matrix is the one `clear` receives
(the compiler warns `unused variable: view_proj` on the first; that's expected). The `clear` call
itself is untouched (it still passes `&origin`, per Step 2e).

The math: when `anchor == origin` this is bit-identical to lesson 33. When the target drifts, the
instances stay put and the view matrix absorbs the difference — same final transform, since
`P·V(eye−a, t−a) · T(−a)·M ≡ P·V(eye−t, 0) · T(−t)·M`.

## Verify

`cargo check --target wasm32-unknown-unknown` — clean except the two expected warnings (unused
`origin` in `clear()`, unused first `view_proj` in `render()`). Load
`30700_querschnitt_gg.pb` (42k objects). Before: 0.1 fps. After: **~120 fps orbit and idle**,
pan/zoom smooth (the only rebuild left is crossing 100m of target drift — try panning kilometers
to see one). Measured on the 9-drawing wall (34e): 503k objects, rebuild never fires in normal use.

## Recap

```
Ch 33: camera-relative f64 — correct, but rebased EVERY frame "to be safe".
Ch 34c: FLOATING ANCHOR. rebuild_instances = translation-column subtract (f64, cast last), and it
        runs only when the target strays REANCHOR_DIST (100m) from the snapped anchor — orbit
        never moves the target, pan/zoom ride the view matrix (view_proj_anchored). 42k objects:
        300ms/frame → GPU-bound. The precision story is unchanged: everything the GPU sees is
        still small numbers near the anchor.
```

Edited: `engine/gpu/mod.rs` (consts to the top, `REANCHOR_DIST`, `last_origin`, `rebase_anchor`,
cheap rebuild, `clear()` drops its rebuild call), `camera.rs` (`view_proj_anchored`), `state.rs`
(anchor dance).

## Next

`34d-proper-reader.md` — the drawing renders fast but all-black and hairline: the kernel's Line
was silently dropping color and width in serialization. A schema fix across three languages.
