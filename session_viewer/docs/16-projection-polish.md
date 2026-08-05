# 16 Projection polish

The camera *works* but has three rough edges before real geometry arrives. **One:**
perspective↔ortho (Space) jumps in size — the ortho frustum is a fixed `h = 1.0`, unrelated to
`distance`. **Two:** in ortho the scroll wheel does nothing — zoom changes `distance`, ortho ignores
it. **Three:** `near`/`far` are fixed at `0.1`/`100`, so depth precision rots as you zoom, with no
real unit. All three live in `view_proj`, fixed with arithmetic on numbers we already have.

<svg viewBox="0 0 680 160" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a perspective frustum and an orthographic box seen from the side; matching the ortho half height to distance times tan of the half fov makes both show the same extent at the target plane" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="170" y="16" fill="#888" text-anchor="middle">perspective — a frustum</text>
  <circle cx="30" cy="80" r="5" fill="none" stroke="#d7dae0"/>
  <path d="M 35,77 L 300,20 M 35,83 L 300,140" fill="none" stroke="#6fb3ff" stroke-width="1.4"/>
  <line x1="230" y1="35" x2="230" y2="125" stroke="#e0b040" stroke-width="1.6"/>
  <text x="230" y="152" fill="#e0b040" text-anchor="middle" font-size="10">target plane: h = distance · tan 30°</text>
  <text x="120" y="70" fill="#666" font-size="10">60° fov</text>
  <text x="510" y="16" fill="#888" text-anchor="middle">ortho — a box</text>
  <circle cx="390" cy="80" r="5" fill="none" stroke="#d7dae0"/>
  <path d="M 395,35 H 660 M 395,125 H 660" fill="none" stroke="#6fb3ff" stroke-width="1.4"/>
  <line x1="590" y1="35" x2="590" y2="125" stroke="#e0b040" stroke-width="1.6"/>
  <text x="590" y="152" fill="#e0b040" text-anchor="middle" font-size="10">same h at the target → NO pop on toggle</text>
</svg>

## Why

- **Seamless switch.** A 60° perspective shows a half-height of `distance · tan 30°` at the target
  plane. Making ortho's half-height match means the two agree at the target — no pop on toggle — and
  since it tracks `distance`, ortho gains zoom for free. (The archive uses a separate `ortho_scale`
  to decouple dolly from zoom; deriving it from our single `distance` needs no new state.)
- **Adaptive depth.** The depth buffer's bits spread across `[near, far]`; in perspective they bunch
  near the camera, and the `far/near` *ratio* sets quality. Pinning both to `distance`
  (`near = distance·0.001`, `far = distance·100`) keeps that ratio **constant (~100 000) at every
  zoom** — precision never degrades. (Cost: geometry past 100× focus distance clips; the archive
  lifts that with reverse-Z, later.) Ortho depth is *linear*, so a wide range is free.
- **Real units, set in code.** The viewer renders in **metres**; geometry may be authored in
  **millimetres** or metres. Instead of one magic `0.001`, the unit is an **enum** the camera
  carries — each variant knows its scale to metres, baked into `view_proj`. It's a model property,
  so it's **fixed in code** (`new()`'s default, or `set_unit` from the API) — not a runtime key.

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

Add a `Unit` enum above `impl Camera` — each variant carries its own scale-to-metres, so cm or
inches later is one line, no scattered constants. Give `Camera` a `unit` field (default
`Millimeters`) and a `set_unit` setter the API calls when a model's unit is known:

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
// To make the viewer default to metres, change `Unit::Millimeters` → `Unit::Meters` here
// (or call `set_unit` at runtime).
Self { yaw: 0.6, pitch: 0.5, distance: 3.0, target: [0.0, 0.0, 0.0], perspective: true,
       unit: Unit::Millimeters }
```

## Step 2 — the polish: `src/camera.rs`

Rewrite `view_proj` — the `eye`/`target`/`view` middle is exactly lesson 13's; only the projection
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
        let target = Point::new(self.target[0] as f64, self.target[1] as f64,
                                self.target[2] as f64);
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
**metres** — convert with the *same* `to_meters` scale, so F frames correctly in either unit:

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

The triangles were in arbitrary units — multiply each coordinate by **1000** to make them real: a
~1.4 m scene in mm, the camera's default unit. After the `0.001` scale they land at the same screen
position, so nothing *looks* different, the numbers are just meaningful now:

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

Same picture as lesson 15, rough edges gone. **Space** no longer jumps size between perspective and
ortho. **Scroll in ortho** finally zooms. Orbit far out and back: orange stays cleanly in front of
blue at every distance — adaptive near/far holding precision steady. The world is honest millimetres
viewed by a metre camera; for metres-authored geometry, set `Unit::Meters` in `new()` (or call
`set_unit`) — nothing else changes.

## Recap

```
Ch 15: fit framed the scene, but the projection had three rough edges.
Ch 16: view_proj fixes all three — adaptive near/far (constant depth ratio), ortho half-height from
       distance (seamless + zoomable), and a Unit enum (mm / m) whose scale-to-metres it bakes in.
       The unit is fixed in code (new() default / set_unit), fit honours it. Steady depth,
       real units.
```

Edited: `camera.rs` (`Unit` enum + `unit` field + `set_unit`, `view_proj`, `fit`), `gpu.rs`
(triangles → mm), `lib.rs` (`SCENE_*` → mm).

## Next

`17-quaternion-camera.md` — replace spherical yaw/pitch with a **quaternion** turntable (+
`last_right` vector) so Top/Bottom are pole-stable and named views become single rotations — the
last piece of the archive-grade camera before Phase 2.
