# 15 Fit / zoom-extents

Named views (lesson 14) point the camera the right *way*; **Fit** gets it the right *distance*.
Tap **F** and the whole scene jumps to fill the view — Rhino's `Zoom Extents`, every CAD app's
"frame everything" key. The job is two numbers: move `target` to the centre of what you want to
see, and set `distance` so that thing exactly fills the frame. No new matrix, no new state — just
arithmetic on the camera fields we already have.

## Why

We already turn `(yaw, pitch, distance, target)` into a `view_proj`. "Fit a box" is the inverse
question: *what `target` and `distance` make this box fill the screen?*

- **`target`** is easy — the box centre.
- **`distance`** comes from the box's **bounding sphere** (radius = half the diagonal) and the
  camera's field of view. A sphere of radius `r` exactly fills a half-angle `θ` when it sits at
  `distance = r / sin θ`. Using the sphere, not the box corners, makes the fit **orientation-free**:
  it frames correctly from *any* yaw/pitch, so we never have to rotate the box into view space.

The only subtlety is *which* half-angle. Our projection uses a 60° **vertical** FOV, but on a tall,
narrow canvas the **horizontal** FOV is smaller — and the smaller angle is the one that clips. So we
take `min(vertical, horizontal)` and fit to that.

```
fit a box:                       r = ½‖max − min‖   (bounding-sphere radius)
   target  ← (min + max) / 2                         eye
   distance ← r / sin(θ)          θ = min half-FOV    •────distance────◯ r   ⟵ sphere fills the cone
```

"Scene **or** selection" is the same call with a different box: feed it the whole-scene AABB, or
just the selected objects' AABB. Selection doesn't exist yet (Phase 7), so today **F frames the one
scene** — but `fit()` stays a pure `(min, max) → camera` function, so the selection branch later is
one `if` at the call site, not a change to the math.

## Files we touch

```
src/camera.rs   # a `fit(min, max, aspect)` method on the existing `impl Camera`
src/lib.rs      # bind `F` to fit the scene's bounding box
```

## Step 1 — the fit: `src/camera.rs`

Add `fit` inside the existing `impl Camera` block. It keeps the current `yaw`/`pitch` (you fit
*from where you're standing*) and only rewrites `target` and `distance`:

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

The `1.1` is a small margin so the geometry doesn't kiss the screen edge; the `clamp` mirrors
`zoom()` so a fit can never push the eye through the near plane or past the far plane.

## Step 2 — bind the key: `src/lib.rs`

Add the scene's bounding box as a const near the top (the only geometry so far is the two
triangles from lesson 6; this is their AABB). Phase 4's `Scene` will compute this live from real
objects — and Phase 7 will pass the *selection* AABB instead — but the shape of the call is final:

```rust
// The scene's axis-aligned bounds. Today: the two triangles (lesson 6). Phase 4's `Scene`
// computes this from real geometry; Phase 7 swaps in the selection's box when something is picked.
const SCENE_MIN: [f32; 3] = [-0.7, -0.4, -0.3];
const SCENE_MAX: [f32; 3] = [ 0.7,  0.5,  0.3];
```

Then add one arm to the keyboard `match` (next to the digit views). Fit needs the aspect ratio, so
read it from the live surface size:

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

Zoom way out with the scroll wheel until the triangles are specks, then press **F** — they snap back
to fill the frame. Zoom in until they overflow the edges and press **F** again — they pull back to
fit. Try it after a named view (`1`–`7`): the box stays centred and framed at that angle. Resize the
window narrow and tall, hit **F**, and the camera pulls back further so nothing clips the sides —
that's the horizontal-FOV branch earning its keep.

## Recap

```
Ch 14: named views set the camera's *angle* (yaw/pitch) and flip to ortho.
Ch 15: fit() sets its *position* — target ← box centre, distance ← r / sin(half-FOV) — so the
       bounding sphere fills the view from any angle. Pure (min,max)→camera; F frames the scene.
```

Edited: `camera.rs` (`fit`), `lib.rs` (`SCENE_MIN`/`SCENE_MAX` + the `F` key arm).

## Next

`16-projection-polish.md` — seamless persp↔ortho (`ortho_scale = distance` on the switch), an
**adaptive near/far** tied to `distance` for depth precision, and the **mm→unit** scale baked into
`view_proj` — the camera section's final polish before real geometry arrives.
