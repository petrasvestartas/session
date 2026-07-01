# 14 Named views & reset

Orbit/pan/zoom let you *find* a viewpoint; CAD also wants to *snap* to the canonical ones —
**Front, Back, Left, Right, Top, Bottom, Iso** — and a single key to get **home** again. Each
named view is just a fixed `(yaw, pitch)` pair fed into the camera we already have, plus one
convention: a named view switches to **orthographic** (a measured, parallel projection — the whole
point of looking "straight on"). No new camera math, no new matrix — only presets and key bindings.

## Why

A named view is the camera's `yaw`/`pitch` set to known angles. "Front" means looking down −Z
with the eye on +Z; "Right" means the eye on +X; "Iso" is the classic 35.26° three-quarter angle.
Because our `Camera` already turns `(yaw, pitch, distance, target)` into a `view_proj`, the feature
collapses to: *pick the two angles, flip to ortho.* The quaternion turntable (lesson 17) will make
Top/Bottom perfectly pole-stable; for now we sit a hair under the pole, which is why this lesson is
a handful of lines, not a rewrite.

```
Front  (1)  yaw 0       pitch 0          Top    (5)  yaw 0   pitch ~+90°
Back   (2)  yaw 180°    pitch 0          Bottom (6)  yaw 0   pitch ~−90°
Left   (3)  yaw −90°    pitch 0          Iso    (7)  yaw 45° pitch 35.26°
Right  (4)  yaw +90°    pitch 0          Home   (C)  reset → new() (perspective)
```

## Files we touch

```
src/camera.rs   # a `View` enum + `set_view()` (presets → ortho) + `reset()` (home)
src/lib.rs      # bind digit keys 1–7 to the views and `C` to reset
```

## Step 1 — the views: `src/camera.rs`

Add a `View` enum above `impl Camera`, and a `set_view` method inside the existing `impl Camera`
block. The match picks the two angles; the last line flips to orthographic. `Top`/`Bottom` sit at
`±(π/2 − 0.001)` — dead-on vertical is a gimbal pole for our fixed `up = +Y` (lesson 17 fixes it
properly with a quaternion):

```rust
/// The seven canonical CAD views, as (yaw, pitch) presets for our spherical camera.
#[derive(Clone, Copy)]
pub enum View { Front, Back, Left, Right, Top, Bottom, Iso }

impl Camera {
    /// Snap to a named view and switch to orthographic — the conventional CAD behaviour:
    /// a named view is a flat, measured projection, not a perspective one.
    pub fn set_view(&mut self, view: View) {
        use std::f32::consts::{PI, FRAC_PI_2, FRAC_PI_4};
        let top = FRAC_PI_2 - 0.001;              // dead-on top/bottom is a gimbal pole — lesson 17
        let (yaw, pitch) = match view {
            View::Front  => (0.0,        0.0),
            View::Back   => (PI,         0.0),
            View::Left   => (-FRAC_PI_2, 0.0),
            View::Right  => (FRAC_PI_2,  0.0),
            View::Top    => (0.0,        top),
            View::Bottom => (0.0,       -top),
            View::Iso    => (FRAC_PI_4,  0.6155),  // classic 35.26° isometric
        };
        self.yaw = yaw;
        self.pitch = pitch;
        self.perspective = false;
    }
}
```

## Step 2 — home: `src/camera.rs`

`reset` goes back to the exact state `new()` builds — angles, distance, target, and perspective
on. Since `new()` already encodes "home", reset is one line:

```rust
    /// Reset to the home view — the angles, distance and target `new()` starts at, perspective on.
    pub fn reset(&mut self) {
        *self = Camera::new();
    }
```

## Step 3 — bind the keys: `src/lib.rs`

Bring the `View` enum into scope next to the other `use crate::…` lines:

```rust
use crate::camera::View;
```

Then widen the keyboard arm. It currently only handles `Space`; turn the single `if` into a
`match` on the (pressed, non-repeat) logical key — `Space` still toggles projection, digits `1`–`7`
snap to a view, and `C` goes home:

```rust
            WindowEvent::KeyboardInput { event, ..} => {
                if event.state == ElementState::Pressed && !event.repeat {
                    match event.logical_key.as_ref() {
                        Key::Named(NamedKey::Space) => state.camera.toggle_projection(),
                        Key::Character("1") => state.camera.set_view(View::Front),
                        Key::Character("2") => state.camera.set_view(View::Back),
                        Key::Character("3") => state.camera.set_view(View::Left),
                        Key::Character("4") => state.camera.set_view(View::Right),
                        Key::Character("5") => state.camera.set_view(View::Top),
                        Key::Character("6") => state.camera.set_view(View::Bottom),
                        Key::Character("7") => state.camera.set_view(View::Iso),
                        Key::Character("c" | "C") => state.camera.reset(),
                        _ => {}
                    }
                }
            }
```

(`logical_key.as_ref()` borrows the key as `Key<&str>`, so a `Character` matches a plain string
literal like `"1"`. `Key` and `NamedKey` are already imported from lesson 9.)

## Step 4 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Tap `1`–`7` to snap through Front/Back/Left/Right/Top/Bottom/Iso — each one jumps to ortho, so the
two triangles project flat. Orbit with the right mouse to leave the snap (perspective stays off
until you hit `Space`), and press `C` to fly home to the three-quarter perspective view. Orbiting
after a Top view re-clamps the pitch back under the pole — expected until the quaternion camera.

## Recap

```
Ch 13: Camera owns yaw/pitch/distance/target + view_proj; orbit/pan/zoom drive it by hand.
Ch 14: the same fields get named presets — set_view(View) snaps (yaw,pitch)+ortho, reset() = home —
       wired to keys 1–7 and C. No new matrix; just known angles.
```

Edited: `camera.rs` (`View` enum, `set_view`, `reset`), `lib.rs` (`use crate::camera::View;` + the
digit/`C` key arms).

## Next

`15-fit.md` — **F** frames the scene (or the selection) by setting `target` to the bounding-box
centre and `distance` so it fills the view — the other half of "get me to a useful viewpoint".
