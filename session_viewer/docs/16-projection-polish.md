# 16 Projection polish

The camera *works*, but it has three rough edges we want gone before real geometry arrives.
**One:** flipping perspective↔ortho (Space) jumps the apparent size — the ortho frustum is a fixed
`h = 1.0`, unrelated to where you're standing. **Two:** in ortho, the scroll wheel does nothing —
zoom changes `distance`, but the ortho size ignores it. **Three:** `near`/`far` are fixed at
`0.1`/`100`, so depth precision rots as you zoom, and the world has no real unit. All three live in
one method — `view_proj` — and all three are a few lines of arithmetic on numbers we already have.

## Why

- **Seamless switch.** A 60° perspective shows, at the target plane (`distance` away), a half-height
  of `distance · tan 30°`. Make the ortho half-height *exactly that* and the two projections agree
  at the target — toggling no longer pops. Because it's derived from `distance`, ortho also gains
  zoom for free. (The archive keeps a separate `ortho_scale` field to decouple dolly from zoom; with
  our single-`distance` camera, deriving it is equivalent and needs no new state.)
- **Adaptive depth.** The depth buffer spreads a fixed number of bits across `[near, far]`; in
  perspective they bunch up near the camera, and the `far/near` *ratio* sets the quality. Pin both
  ends to `distance` (`near = distance·0.001`, `far = distance·100`) and the ratio is **constant
  (~100 000) at every zoom** — precision never degrades. (Cost: geometry past 100× your focus
  distance clips. The archive lifts that with reverse-Z, a later lesson.) Ortho depth is *linear*,
  so a wide symmetric range is free.
- **Real units, set in code.** The viewer renders in **metres**, but geometry can be authored in
  **millimetres** (session geometry) *or* metres. Rather than bake one magic `0.001`, we make the
  unit an **enum** the camera carries — each variant knows its scale to metres — and bake that scale
  into `view_proj`. The unit is a property of the model, so it's **fixed in code** (the `new()`
  default, or `set_unit` when your API loads geometry) — not a runtime key.

```
perspective:  near = distance × 0.001    far = distance × 100     →  far/near ≈ 100 000, always
ortho:        half-height = distance × tan 30°   (what perspective shows at the target plane)
units:        Unit::Millimeters → × 0.001 │ Unit::Meters → × 1.0   (scale baked into view_proj)
```

## Files we touch

```
src/camera.rs   # a `Unit` enum + `unit` field (+ set_unit), rewritten view_proj, unit-aware fit()
src/engine/gpu.rs   # the demo triangles, now in millimetres (× 1000)
src/lib.rs      # SCENE_* in millimetres
```

## Step 1 — the unit: `src/camera.rs`

Add a `Unit` enum above `impl Camera`. Each variant carries its own scale-to-metres, so adding cm
or inches later is one line — no scattered constants. Give `Camera` a `unit` field (default
`Millimeters`), and a `set_unit` setter your API calls when it loads a model of a known unit:

```rust
/// The unit the loaded geometry is authored in. The viewer renders in metres, so each variant
/// knows its scale *to* metres — `view_proj` bakes it in. Extend with Centimeters/Inches as needed.
#[derive(Clone, Copy, PartialEq)]
pub enum Unit { Millimeters, Meters }

impl Unit {
    pub fn to_meters(self) -> f64 {
        match self {
            Unit::Millimeters => 0.001,
            Unit::Meters      => 1.0,
        }
    }
}

impl Camera {
    /// Set the authoring unit in code — call this from the API when a model's unit is known.
    /// It takes any variant, so it never needs editing when you add Centimeters/Inches.
    pub fn set_unit(&mut self, unit: Unit) {
        self.unit = unit;
    }
}
```

Add the field to the struct and initialise it in `new()`:

```rust
pub struct Camera {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub target: [f32; 3],
    pub perspective: bool,
    pub unit: Unit,                 // authoring unit of the geometry
}

// in new():
// DEFAULT UNIT — session geometry is millimetres, so the viewer starts in mm.
// To make the viewer default to metres, change `Unit::Millimeters` → `Unit::Meters` here.
// (Or flip it at runtime any time with the `U` key — see Step 5.)
Self { yaw: 0.6, pitch: 0.5, distance: 3.0, target: [0.0, 0.0, 0.0], perspective: true,
       unit: Unit::Millimeters }
```

## Step 2 — the polish: `src/camera.rs`

Rewrite `view_proj`. The `eye`/`target`/`view` middle is exactly lesson 13's — only the projection
ends and the final unit scale are new:

```rust
    pub fn view_proj(&self, aspect: f64) -> Xform {
        let dist = self.distance as f64;
        let projection = if self.perspective {
            // near & far both track distance → far/near ratio is pinned (~1e5): depth precision is
            // identical at every zoom. (No reverse-Z yet, so a constant ratio is what we rely on.)
            Xform::perspective(f64::to_radians(60.0), aspect, dist * 0.001, dist * 100.0)
        } else {
            // ortho half-height = what perspective shows at the target plane → the switch is
            // seamless, and because it tracks `distance`, scrolling now zooms in ortho too.
            let h = dist * f64::to_radians(30.0).tan();
            let r = dist * 100.0;                 // ortho depth is linear → a wide range is free
            Xform::orthographic(-aspect * h, aspect * h, -h, h, -r, r)
        };

        let (cp, sp) = (self.pitch.cos(), self.pitch.sin());
        let (cy, sy) = (self.yaw.cos(), self.yaw.sin());
        let eye = Point::new(
            self.target[0] as f64 + (self.distance * cp * sy) as f64,
            self.target[1] as f64 + (self.distance * sp) as f64,
            self.target[2] as f64 + (self.distance * cp * cy) as f64,
        );
        let target = Point::new(self.target[0] as f64, self.target[1] as f64, self.target[2] as f64);
        let up = Vector::new(0.0, 1.0, 0.0);
        let view = Xform::look_at_right_handed(&eye, &target, &up);

        // authoring unit → metre, applied to geometry first: projection · view · scale.
        let s = self.unit.to_meters();
        let scale = Xform::scale_xyz(s, s, s);
        projection * view * scale
    }
```

## Step 3 — fit follows the unit: `src/camera.rs`

`fit` receives a box in the geometry's **authoring unit**, but `target`/`distance` are the camera's
**metres** — so convert with the *same* `to_meters` scale. Now F frames correctly in either unit:

```rust
    pub fn fit(&mut self, min: [f32; 3], max: [f32; 3], aspect: f64){
        let s = self.unit.to_meters() as f32;     // authoring unit → metre camera
        self.target = [
            (min[0] + max[0]) * 0.5 * s,
            (min[1] + max[1]) * 0.5 * s,
            (min[2] + max[2]) * 0.5 * s,
        ];
        let dx = (max[0] - min[0]) * 0.5 * s;
        let dy = (max[1] - min[1]) * 0.5 * s;
        let dz = (max[2] - min[2]) * 0.5 * s;
        let radius = (dx*dx + dy*dy + dz*dz).sqrt();
        if radius <= 0.0 { return; }

        let half_fov_y = f64::to_radians(60.0) * 0.5;
        let half_fov_x = (aspect * half_fov_y.tan()).atan();
        let half_fov = half_fov_y.min(half_fov_x) as f32;
        self.distance = (radius / half_fov.sin() * 1.1).clamp(0.2, 100.0);
    }
```

## Step 4 — the demo geometry, in millimetres: `src/engine/gpu.rs`

The two triangles were in arbitrary units; make them real by multiplying every coordinate by
**1000** — now they're a ~1.4 m scene in mm, the camera's default unit. After the `0.001` scale they
land at exactly the same place on screen, so nothing *looks* different — the numbers are just
meaningful now:

```rust
        const TRIANGLES: &[Vertex] = &[
            Vertex { position: [-200.0,  500.0,  300.0], color: [1.0, 0.5, 0.1] },
            Vertex { position: [-700.0, -400.0,  300.0], color: [1.0, 0.5, 0.1] },
            Vertex { position: [ 300.0, -400.0,  300.0], color: [1.0, 0.5, 0.1] },

            Vertex { position: [ 200.0,  500.0, -300.0], color: [0.1, 0.5, 1.0] },
            Vertex { position: [-300.0, -400.0, -300.0], color: [0.1, 0.5, 1.0] },
            Vertex { position: [ 700.0, -400.0, -300.0], color: [0.1, 0.5, 1.0] },
        ];
```

## Step 5 — scene bounds in mm: `src/lib.rs`

`SCENE_MIN`/`SCENE_MAX` describe those same triangles, so they move to mm too — `× 1000`:

```rust
const SCENE_MIN: [f32; 3] = [-700.0, -400.0, -300.0];
const SCENE_MAX: [f32; 3] = [ 700.0,  500.0,  300.0];
```

## Step 6 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

The picture is identical to lesson 15 — but the rough edges are gone. Press **Space**: perspective
and ortho no longer jump in size. **Scroll in ortho**: it finally zooms. Orbit far out and back: the
orange triangle stays cleanly in front of the blue one at every distance — adaptive near/far holding
depth precision steady. The world is now honest millimetres viewed by a metre camera; to render a
model authored in metres, set `Unit::Meters` in `new()` (or call `set_unit` from your API) — no other
change needed.

## Recap

```
Ch 15: fit framed the scene, but the projection had three rough edges.
Ch 16: view_proj fixes all three — adaptive near/far (constant depth ratio), ortho half-height from
       distance (seamless + zoomable), and a Unit enum (mm / m) whose scale-to-metres it bakes in.
       The unit is fixed in code (new() default / set_unit), fit honours it. Steady depth, real units.
```

Edited: `camera.rs` (`Unit` enum + `unit` field + `set_unit`, `view_proj`, `fit`), `gpu.rs`
(triangles → mm), `lib.rs` (`SCENE_*` → mm).

## Next

`17-quaternion-camera.md` — replace spherical yaw/pitch with a **quaternion** turntable (+ a
`last_right` vector) so Top/Bottom are pole-stable and named views become single rotations — the
last piece of the archive-grade camera before real geometry in Phase 2.
