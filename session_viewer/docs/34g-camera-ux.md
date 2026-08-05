# 34g Camera UX — cursor zoom, no stops, middle-mouse pan

> **Big picture.** Three small changes that separate "a renderer" from "a CAD viewport": zoom
> must dolly **toward the cursor** (the point under the mouse stays under the mouse — the one
> trick every CAD app shares), zoom must never hit an artificial wall, and **middle-mouse must
> pan**. Also buried here: the bug where deep zooming made the whole scene vanish — a demo-era
> `distance` clamp of 0.2–100 left over from lesson 10, while a fitted drawing wall sits far
> beyond it; one wheel tick snapped the far plane (10×distance) inside the scene and culled
> everything.

## Files we touch

```
src/camera.rs   # Step 1: zoom loses its clamp · Step 2: zoom_at — cursor-centered dolly
src/lib.rs      # Step 3: MMB pan state + wheel wired to zoom_at
```

## Step 1 — zoom without stops: `src/camera.rs`

Multiplicative zoom (×0.9 per detent) is naturally asymptotic — it approaches zero but never
reaches it, and our near/far planes scale WITH distance (`dist*0.01 … dist*10`), so no absolute
range is ever needed. **Find the whole function (including its doc comment):**

```rust
    /// Dolly in/out by scaling `distance` (clamped to 0.2–100).
    pub fn zoom(&mut self, amount: f32) {
        self.distance = (self.distance * (1.0 - amount as f64 * 0.1)).clamp(0.2, 100.0);
        self.update_position();
    }
```

Replace with:

```rust
    /// Dolly in/out by scaling `distance`. NO range clamp — zoom is multiplicative (×0.9 per
    /// detent) so it approaches but never reaches zero, and near/far planes scale with distance;
    /// only a not-zero guard remains. (The old 0.2–100 clamp made deep zooms cull the scene.)
    pub fn zoom(&mut self, amount: f32) {
        self.distance = (self.distance * (1.0 - amount as f64 * 0.1)).max(1.0e-6);
        self.update_position();
    }
```

## Step 2 — zoom toward the cursor: `src/camera.rs`

The math: unproject the cursor to its world point `p` on the target plane (right/up frame ×
frustum half-extents × NDC), then scale the distance by `k` while pulling the target so `p` stays
fixed — `target' = target + (p − target)·(1 − k)`. Our ortho height matches perspective at the
target distance, so ONE `half_h` formula serves both projections.

**Insert directly BELOW the `zoom` function you just replaced** (between `zoom`'s closing `}` and
`/// Flip between perspective and orthographic projection.`):

```rust
    /// CAD zoom: dolly toward the CURSOR — the point under the mouse stays under the mouse.
    /// The cursor's world point on the target plane is computed from the view frame, then the
    /// target is pulled toward it by the zoom factor. `cursor`/`viewport` in physical px.
    pub fn zoom_at(&mut self, amount: f32, cursor: (f64, f64), viewport: (f64, f64)) {
        let new_dist = (self.distance * (1.0 - amount as f64 * 0.1)).max(1.0e-6);
        let k = new_dist / self.distance; // actual factor after the guard
        let ndc_x = 2.0 * cursor.0 / viewport.0 - 1.0;
        let ndc_y = 1.0 - 2.0 * cursor.1 / viewport.1;
        // Frustum half-extents at the target plane (ortho h matches perspective at the target)
        let half_h = self.distance * f64::to_radians(30.0).tan();
        let half_w = half_h * (viewport.0 / viewport.1);
        let right = self.orientation.rotate_vector(Vector::x_axis());
        for i in 0..3 {
            let cursor_off = right[i] * ndc_x * half_w + self.up[i] * ndc_y * half_h;
            self.target[i] += cursor_off * (1.0 - k); // keeps the cursor's world point fixed
        }
        self.distance = new_dist;
        self.update_position();
    }
```

(`Vector` is already imported at the top of `camera.rs` — nothing to add.)

> Zoom moves the target now — 34c's anchor absorbs it: within 100m of drift it's still just a
> view-matrix change, no instance rebuild.

## Step 3 — middle-mouse pan + the wiring: `src/lib.rs`

**3a. The field.** In `pub struct App`, find:

```rust
    orbiting: bool,
    last_cursor: (f64, f64),
```

Insert between the two lines:

```rust
    panning: bool,
```

**3b. The initializer.** In `App::run()`, find the `let app = App { … }` literal:

```rust
            orbiting: false,
            last_cursor: (0.0, 0.0),
```

Insert between the two lines:

```rust
            panning: false,
```

**3c. The MMB handler.** Find the RMB arm in `window_event`:

```rust
            WindowEvent::MouseInput {state: btn, button: MouseButton::Right, ..} => {
                self.orbiting = btn == ElementState::Pressed; // hold RMB to orbit
            }
```

Insert after its closing `}`:

```rust
            WindowEvent::MouseInput {state: btn, button: MouseButton::Middle, ..} => {
                self.panning = btn == ElementState::Pressed; // hold MMB to pan (CAD standard)
            }
```

**3d. The move handler decides by mode** — MMB pans, RMB orbits, Ctrl+RMB still pans. In the
`WindowEvent::CursorMoved` arm, find:

```rust
                if self.orbiting {
                    let dx = (position.x - self.last_cursor.0) as f32;
                    let dy = (position.y - self.last_cursor.1) as f32;
                    if self.ctrl {
                        state.camera.pan(dx, dy);
                    } else {
                        state.camera.orbit(dx, dy)
                    };
                }
```

Replace the whole if-block with (two condition lines change, the deltas stay):

```rust
                if self.orbiting || self.panning {
                    let dx = (position.x - self.last_cursor.0) as f32;
                    let dy = (position.y - self.last_cursor.1) as f32;
                    if self.panning || self.ctrl {
                        state.camera.pan(dx, dy);
                    } else {
                        state.camera.orbit(dx, dy)
                    };
                }
```

(the `self.last_cursor = (position.x, position.y);` line below stays.)

**3e. The wheel calls the new zoom** with the tracked cursor. In the `WindowEvent::MouseWheel`
arm, find:

```rust
                state.camera.zoom(amount);
```

Replace with:

```rust
                // Zoom TOWARD THE CURSOR — the point under the mouse stays put (CAD standard)
                let vp = (state.gpu.config.width as f64, state.gpu.config.height as f64);
                state.camera.zoom_at(amount, self.last_cursor, vp);
```

(`zoom` itself stays — keyboard/fallback callers and the no-cursor case still use it.)

## Verify

Put the cursor on one drawing detail and wheel in — the detail grows *under the cursor* without
drifting to the view center. Wheel out far past the wall, then all the way back into a single
line's cap: nothing ever disappears at either extreme. Hold MMB and drag: pan. RMB: orbit.
On the paper-width drawings (34f), zooming in fattens the pens under your cursor like leaning
into a print.

## Recap

```
Ch 34g: A CAD VIEWPORT, NOT A DEMO. Zoom is unlimited (multiplicative + scaling near/far needs
        no clamp; the old 0.2–100 clamp was culling fitted scenes) and cursor-centered
        (unproject to the target plane, pull the target by 1−k). MMB pans; RMB orbits; Ctrl+RMB
        pans. The anchor (34c) makes all of it rebuild-free.
```

## Next

`34h-colors-widths.md` — the remaining color channels (FACECOLORS, POINTCOLORS dots, the
instance tint) and width plumbing that 34d/34f didn't already land.
