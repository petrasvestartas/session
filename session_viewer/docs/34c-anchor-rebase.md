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
src/engine/gpu/mod.rs   # REANCHOR_DIST, last_origin, rebase_anchor; rebuild loses the f64 multiply
src/camera.rs           # view_proj_anchored — camera-relative to a caller-supplied anchor
src/state.rs            # render(): anchor dance before clear; clear() loses the origin param
```

## Step 1 — the rebuild gets 20× cheaper: `gpu/mod.rs`

`T(-origin) × M` only changes the TRANSLATION column of a column-major matrix — the old code paid
a full 4×4 f64 multiply per object for three subtractions' worth of change. **Replace
`rebuild_instances`' loop body** (keep the f64-subtract-then-cast order — that's lesson 33's
precision guarantee):

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

## Step 2 — the anchor: `gpu/mod.rs`

**2a. A constant next to the others**, and a field next to `instances`:

```rust
/// Re-anchor distance: the instance table is rebased about a snapped ANCHOR; the camera target can
/// drift this far (mm) before a full rebuild. Within it, pan/zoom only changes the view matrix.
/// f32 error at 1e5 mm from the anchor ≈ 6e-3 mm — far below a pixel.
const REANCHOR_DIST: f64 = 1.0e5;
```

```rust
    last_origin: Option<Point>, // rebuild_instances skips when the camera target didn't move
```

(add `last_origin: None,` to the `Ok(Self { … })` initializer.)

**2b. The public entry — add above `rebuild_instances`:**

```rust
    /// The anchor the instance table is rebased about. A full rebuild (42k× at stress scale) runs
    /// ONLY when the camera target strays REANCHOR_DIST from the current anchor — orbit never
    /// moves the target, and pan/zoom within the budget just changes the view matrix.
    pub fn rebase_anchor(&mut self, origin: &Point) -> Point {
        let need = match &self.last_origin {
            None => true,
            Some(a) => {
                let (dx, dy, dz) = (a[0] - origin[0], a[1] - origin[1], a[2] - origin[2]);
                (dx * dx + dy * dy + dz * dz).sqrt() > REANCHOR_DIST
            }
        };
        if need { self.rebuild_instances(origin); }
        self.last_origin.clone().unwrap()
    }
```

**2c. `clear()` stops rebuilding.** Remove the `self.rebuild_instances(origin);` line and the
`origin: &Point` parameter — the signature becomes
`pub fn clear(&mut self, color: wgpu::Color, view_proj: &Xform)`.

## Step 3 — the camera renders about the anchor: `camera.rs`

Lesson 33's `view_proj` built eye/target relative to `origin()` (the target). Generalize it —
**replace `view_proj` with a wrapper + the anchored variant** (body identical, `origin` → the
`anchor` parameter):

```rust
    pub fn view_proj(&self, aspect: f64) -> Xform {
        self.view_proj_anchored(aspect, &self.origin())
    }

    /// Like `view_proj`, but camera-relative to a caller-supplied ANCHOR instead of the target.
    /// Instances rebased about the same anchor stay valid while the target drifts (pan/zoom) —
    /// panning then costs ONE uniform write instead of an instance-table rebuild.
    pub fn view_proj_anchored(&self, aspect: f64, anchor: &Point) -> Xform {
        // …identical body, with:
        let eye    = Point::new(self.position[0] - anchor[0], self.position[1] - anchor[1], self.position[2] - anchor[2]);
        let target = Point::new(self.target[0]   - anchor[0], self.target[1]   - anchor[1], self.target[2]   - anchor[2]);
        // …
    }
```

## Step 4 — the wiring: `state.rs` `render()`

```rust
        let origin = self.camera.origin();
        let anchor = self.gpu.rebase_anchor(&origin);
        let view_proj = self.camera.view_proj_anchored(aspect, &anchor);
        self.gpu.clear(wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 }, &view_proj)
```

The math: when `anchor == origin` this is bit-identical to lesson 33. When the target drifts, the
instances stay put and the view matrix absorbs the difference — same final transform, since
`P·V(eye−a, t−a) · T(−a)·M ≡ P·V(eye−t, 0) · T(−t)·M`.

## Verify

Load `30700_querschnitt_gg.pb` (42k objects). Before: 0.1 fps. After: **~120 fps orbit and idle**,
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

Edited: `gpu/mod.rs` (`REANCHOR_DIST`, `last_origin`, `rebase_anchor`, cheap rebuild, `clear()`
signature), `camera.rs` (`view_proj_anchored`), `state.rs` (anchor dance).

## Next

`34d-proper-reader.md` — the drawing renders fast but all-black and hairline: the kernel's Line
was silently dropping color and width in serialization. A schema fix across three languages.
