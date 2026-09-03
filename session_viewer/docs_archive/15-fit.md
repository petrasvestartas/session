# 15 Fit / zoom-extents

Named views (14) point the camera the right *way*; **Fit** gets the right *distance*. Tap **F** and
the scene fills the view — Rhino's `Zoom Extents`. Two numbers: `target` moves to the centre of what
you want to see, `distance` is set so it fills the frame. No new matrix, no new state — just
arithmetic on fields we already have.

## Why

`(yaw, pitch, distance, target)` already makes a `view_proj`. "Fit a box" inverts that: *what
`target`/`distance` fill the screen with this box?*

- **`target`** — the box centre.
- **`distance`** — from the box's **bounding sphere** (radius = half the diagonal) and the FOV: a
  sphere of radius `r` fills half-angle `θ` at `distance = r / sin θ`. Using the sphere instead of
  box corners keeps the fit **orientation-free** — it frames correctly from *any* yaw/pitch.

One catch: *which* half-angle. Our FOV is 60° **vertical**, but a tall canvas has a smaller
**horizontal** FOV — the smaller one clips, so we take `min(vertical, horizontal)`.

```
fit a box:                       r = ½‖max − min‖   (bounding-sphere radius)
   target  ← (min + max) / 2                         eye
   distance ← r / sin(θ)          θ = min half-FOV    •────distance────◯ r   ⟵ sphere fills the cone
```

<svg viewBox="0 0 400 150" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="distance derived from bounding-sphere radius and half field of view" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="200" y="16" fill="#888" text-anchor="middle">distance = r / sin(θ)</text>
  <line x1="50" y1="95" x2="330" y2="95" stroke="#555" stroke-width="1" stroke-dasharray="3,3"/>
  <text x="190" y="112" fill="#666" text-anchor="middle" font-size="10">distance</text>
  <line x1="50" y1="95" x2="300" y2="45" stroke="#6fb3ff" stroke-width="1.5"/>
  <text x="80" y="80" fill="#d7dae0" font-size="10">θ</text>
  <circle cx="50" cy="95" r="3" fill="#6fb3ff"/>
  <text x="50" y="118" fill="#666" text-anchor="middle" font-size="10">eye</text>
  <circle cx="330" cy="95" r="55" fill="none" stroke="#6fb3ff" stroke-width="1.5"/>
  <circle cx="330" cy="95" r="2" fill="#6fb3ff"/>
  <text x="330" y="120" fill="#666" text-anchor="middle" font-size="10">target</text>
  <text x="365" y="70" fill="#d7dae0" font-size="10">r</text>
  <line x1="330" y1="95" x2="330" y2="40" stroke="#3a3a3a" stroke-width="1"/>
</svg>

"Scene **or** selection" is the same call, a different box: whole-scene AABB, or the selected
objects' AABB. Selection doesn't exist yet (Phase 7), so today **F frames the scene** — but `fit()`
stays a pure `(min, max) → camera` function, so selection later is one `if` at the call site.

## Files we touch

```
src/camera.rs   # a `fit(min, max, aspect)` method on the existing `impl Camera`
src/lib.rs      # bind `F` to fit the scene's bounding box
```

## Step 1 — the fit: `src/camera.rs`

Add `fit` inside the existing `impl Camera` block — it keeps the current `yaw`/`pitch` (fit *from
where you're standing*) and only rewrites `target` and `distance`:

```rust
    /// Frame an axis-aligned box: centre `target` on it and pull `distance` back so the box's
    /// bounding sphere fills the view. Keeps the current yaw/pitch and projection — works the same
    /// in perspective or ortho. `aspect` = width / height, to pick the limiting field of view.
    pub fn fit(&mut self, min: [f32; 3], max: [f32; 3], aspect: f64) {
        // target ← box centre
        self.target = [
            (min[0] + max[0]) * 0.5,
            (min[1] + max[1]) * 0.5,
            (min[2] + max[2]) * 0.5,
        ];

        // bounding-sphere radius = half the diagonal (orientation-free: fits from any angle)
        let dx = (max[0] - min[0]) * 0.5;
        let dy = (max[1] - min[1]) * 0.5;
        let dz = (max[2] - min[2]) * 0.5;
        let radius = (dx * dx + dy * dy + dz * dz).sqrt();
        if radius <= 0.0 { return; }              // empty box / single point → leave distance be

        // limiting half-FOV: 60° vertical, narrower horizontal on a tall canvas → take the smaller
        let half_fov_y = f64::to_radians(60.0) * 0.5;
        let half_fov_x = (aspect * half_fov_y.tan()).atan();
        let half_fov = half_fov_y.min(half_fov_x) as f32;

        // distance so the sphere fills that half-angle, plus 10% breathing room
        self.distance = (radius / half_fov.sin() * 1.1).clamp(0.2, 100.0);
    }
```

`1.1` is a small edge margin; the `clamp` mirrors `zoom()` so a fit never pushes the eye past the
near or far plane.

## Step 2 — bind the key: `src/lib.rs`

Add the scene's bounding box as a const near the top — today just the two triangles from lesson 6.
Phase 4's `Scene` will compute this live from real objects, Phase 7 will pass the *selection* AABB
instead — but the call's shape is final:

```rust
// The scene's axis-aligned bounds. Today: the two triangles (lesson 6). Phase 4's `Scene`
// computes this from real geometry; Phase 7 swaps in the selection's box when something is picked.
const SCENE_MIN: [f32; 3] = [-0.7, -0.4, -0.3];
const SCENE_MAX: [f32; 3] = [ 0.7,  0.5,  0.3];
```

Then add one arm to the keyboard `match` (next to the digit views) — fit needs the aspect ratio,
read from the live surface size:

```rust
                        Key::Character("f" | "F") => {
                            let aspect = state.gpu.config.width as f64
                                       / state.gpu.config.height as f64;
                            state.camera.fit(SCENE_MIN, SCENE_MAX, aspect);
                        }
```

## Step 3 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Zoom out until the triangles are specks, press **F** — they snap back to fill the frame; zoom in
past the edges, **F** again pulls back to fit. After a named view (`1`–`7`) the box stays centred at
that angle. Resize the window narrow and tall and hit **F** — the camera pulls back further so
nothing clips, the horizontal-FOV branch earning its keep.

## Recap

```
Ch 14: named views set the camera's *angle* (yaw/pitch) and flip to ortho.
Ch 15: fit() sets its *position* — target ← box centre, distance ← r / sin(half-FOV) — so the
       bounding sphere fills the view from any angle. Pure (min,max)→camera; F frames the scene.
```

Edited: `camera.rs` (`fit`), `lib.rs` (`SCENE_MIN`/`SCENE_MAX` + the `F` key arm).

## Next

`16-projection-polish.md` — seamless persp↔ortho (`ortho_scale = distance`), an **adaptive
near/far** tied to `distance` for depth precision, and the **mm→unit** scale baked into
`view_proj` — final polish before real geometry arrives.
