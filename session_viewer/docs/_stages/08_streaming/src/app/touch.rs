//! Touch gestures — the phone half of the camera bindings.
//!
//! winit's web backend splits pointers by `pointerType`: a pointer whose type is `"touch"` is
//! routed to `WindowEvent::Touch` and NEVER to `CursorMoved` / `MouseInput`
//! (`winit-0.30.13/src/platform_impl/web/web_sys/pointer.rs`, the `match pointer_type` arms —
//! the comment there says duplicate mouse events would be "inconsistent with other platforms").
//! So a finger cannot also reach the mouse arms in `lib.rs`, and the two sets of bindings can be
//! read, and changed, independently. (The runner registers a SECOND, unfiltered set of pointer
//! listeners on the window — `event_loop/runner.rs`, "pointermove"/"pointerdown"/"pointerup" —
//! but those raise `DeviceEvent`, which this viewer does not implement, and they return early
//! unless device events are switched on. They are not a second route into anything here.)
//!
//! | gesture | camera | the mouse binding it mirrors |
//! |---|---|---|
//! | one finger, drag | `orbit` | right-drag |
//! | two fingers, slide | `pan` | middle-drag |
//! | two fingers, spread / close | `zoom_at` their midpoint | wheel |
//! | double tap | `fit` | `F` |
//!
//! Two conversions have to happen here, or the same hand movement means different things on
//! different phones.
//!
//! FINGER TRAVEL IS IN CSS PIXELS. winit reports PHYSICAL pixels (`to_physical(scale_factor)`,
//! same file), so one centimetre of glass is three times the number on a dpr-3 phone that it is
//! on a dpr-1 laptop. Orbit is a fixed radians-per-unit, so the raw figure would spin the model
//! three times as fast for the same movement — and differently again on the next phone. Dividing
//! by the device pixel ratio makes the gesture mean one thing everywhere; on a dpr-1 screen it
//! is then exactly the mouse.
//!
//! PAN IS FINGER-EXACT. `Camera::pan` scales its argument by a hard-coded `distance * 0.0015`,
//! which equals the `2·tan(30°)` the projection really spans only when the viewport is 770 px
//! tall — anywhere else the model slides faster or slower than the hand holding it. A mouse does
//! not notice, because the cursor is not the thing being dragged. A finger IS on the thing, so
//! the error reads as the model slipping. Scaling by the real viewport height (`PAN_PER_PX`)
//! removes it, in both projections: the orthographic branch of `view_proj_anchored` uses the
//! same `distance * tan(30°)` half-height as the perspective one.

use winit::event::{Touch, TouchPhase};

use crate::camera::Camera;
use crate::engine::performance::now_ms;

/// `Camera::pan` moves the target by `arg * distance * 0.0015`, while one physical pixel is
/// worth `distance * 2·tan(30°) / viewport_height` of target motion. Their ratio is this
/// constant over the viewport height — the number of pan units one pixel of finger is worth.
const PAN_PER_PX: f64 = 2.0 * 0.577_350_269_189_625_7 / 0.001_5; // 769.8 — pan is exact at that height

/// `Camera::zoom_at` takes a WHEEL DETENT and scales distance by `1 - amount * 0.1`. A pinch
/// gives a ratio `r` instead, so invert it: `1 - amount * 0.1 = 1/r`, hence `PINCH_GAIN`.
/// Spreading the fingers (`r > 1`) shortens the distance, which is zooming in — the same sign
/// as a wheel push.
const PINCH_GAIN: f64 = 10.0;

/// Biggest span change one event may claim. A finger the browser loses and re-delivers, or a
/// third finger landing between two samples, otherwise teleports the camera.
const PINCH_MAX: f64 = 2.0;

/// A finger that never travelled this far (CSS px) and lifted within `TAP_MS` is a tap …
///
/// Both clocks are read when the event is HANDLED, not when it happened — winit's `Touch` carries
/// no timestamp — so a main thread stalled longer than the window turns a real double tap into
/// two singles. At 30-60 fps the stall is a frame and it does not matter; in a BACKGROUND tab,
/// where the browser throttles the frame loop to 1 Hz, it always will. That is a measurement
/// trap, not a bug: a viewer nobody is looking at has no gestures to miss.
const TAP_SLOP: f64 = 12.0;
const TAP_MS: f64 = 300.0;
/// … and a second tap this soon after it, and this near it, is a double tap. Both windows are
/// wider than the single-tap ones: the second tap of a real double tap is the sloppier of the two.
const DOUBLE_TAP_MS: f64 = 320.0;
const DOUBLE_TAP_SLOP: f64 = 40.0;

/// What one touch event asked for. `Fit` needs the scene bounds, which live a layer up, so it is
/// reported rather than done here — this file knows the camera and nothing else.
pub enum Act {
    None,
    Moved,
    Fit,
}

/// One finger, from its `Started` to its `Ended`. Physical pixels throughout.
struct Finger {
    id: u64,
    pos: (f64, f64),  // where it is now
    down: (f64, f64), // where it landed — a tap is a finger that never left this
    t0: f64,          // when it landed, ms
}

/// Every finger on the glass, plus what the last two-finger sample measured.
pub struct Touches {
    fingers: Vec<Finger>,
    /// Distance between the first two fingers at the previous event, and their midpoint.
    /// `span == 0.0` means NOT SEEDED: the next two-finger move records and does nothing else.
    /// Every change in finger count clears it, and that is what stops the model jumping when a
    /// second finger joins or leaves halfway through a gesture.
    span: f64,
    mid: (f64, f64),
    /// When and where the last tap lifted, for the double tap.
    tap: Option<(f64, (f64, f64))>,
}

impl Touches {
    /// No fingers down, no tap pending.
    pub fn new() -> Self {
        Self { fingers: Vec::new(), span: 0.0, mid: (0.0, 0.0), tap: None }
    }

    /// Fold one `WindowEvent::Touch` into the gesture and move the camera. `vp` is the surface
    /// size and `t.location` the finger, both in physical pixels; `dpr` is the device pixel
    /// ratio that turns physical travel back into the CSS pixels a hand feels.
    pub fn event(&mut self, cam: &mut Camera, t: &Touch, vp: (f64, f64), dpr: f64) -> Act {
        let p = (t.location.x, t.location.y);
        match t.phase {
            TouchPhase::Started => {
                self.fingers.push(Finger { id: t.id, pos: p, down: p, t0: now_ms() });
                self.span = 0.0; // the gesture just changed shape — re-seed on the next move
                Act::None
            }
            TouchPhase::Moved => self.moved(cam, t.id, p, vp, dpr),
            TouchPhase::Ended => self.lifted(t.id, p, dpr),
            // A cancel is the browser taking the gesture away — a scroll it decided to own, a
            // system edge swipe, a call arriving. Drop the finger, and never read it as a tap.
            TouchPhase::Cancelled => {
                self.drop_finger(t.id);
                self.tap = None;
                Act::None
            }
        }
    }

    /// A finger travelled. One finger orbits; two pan by their midpoint and zoom by their span.
    fn moved(&mut self, cam: &mut Camera, id: u64, p: (f64, f64), vp: (f64, f64), dpr: f64) -> Act {
        let Some(i) = self.fingers.iter().position(|f| f.id == id) else { return Act::None };
        let d = (p.0 - self.fingers[i].pos.0, p.1 - self.fingers[i].pos.1);
        self.fingers[i].pos = p;

        if self.fingers.len() == 1 {
            cam.orbit((d.0 / dpr) as f32, (d.1 / dpr) as f32);
            return Act::Moved;
        }

        // Three fingers or more still drive the two-finger gesture, off the first two down: a
        // hand resting a third finger on the glass should not stop the pan it is already doing.
        let (a, b) = (self.fingers[0].pos, self.fingers[1].pos);
        let span = (b.0 - a.0).hypot(b.1 - a.1).max(1.0);
        let mid = ((a.0 + b.0) * 0.5, (a.1 + b.1) * 0.5);
        if self.span == 0.0 {
            self.span = span; // first sample of this shape: record, do not act
            self.mid = mid;
            return Act::None;
        }

        // Pan first, then zoom about the NEW midpoint: the pan slides the model with the hand,
        // the zoom then keeps whatever is under the midpoint under it.
        let h = vp.1.max(1.0);
        cam.pan(((mid.0 - self.mid.0) * PAN_PER_PX / h) as f32, ((mid.1 - self.mid.1) * PAN_PER_PX / h) as f32);
        let r = (span / self.span).clamp(1.0 / PINCH_MAX, PINCH_MAX);
        cam.zoom_at((PINCH_GAIN * (1.0 - 1.0 / r)) as f32, mid, vp);

        self.span = span;
        self.mid = mid;
        Act::Moved
    }

    /// A finger left the glass cleanly. Only the LAST one up can be a tap — a lift that leaves
    /// other fingers down is the tail of a two-finger gesture, not a tap on anything.
    fn lifted(&mut self, id: u64, p: (f64, f64), dpr: f64) -> Act {
        let Some(f) = self.drop_finger(id) else { return Act::None };
        if !self.fingers.is_empty() {
            self.tap = None;
            return Act::None;
        }
        let now = now_ms();
        if (p.0 - f.down.0).hypot(p.1 - f.down.1) / dpr > TAP_SLOP || now - f.t0 > TAP_MS {
            self.tap = None; // a drag, or a press held long enough to mean something else
            return Act::None;
        }
        let second = self.tap.take().is_some_and(|(t0, at)| {
            now - t0 < DOUBLE_TAP_MS && (p.0 - at.0).hypot(p.1 - at.1) / dpr < DOUBLE_TAP_SLOP
        });
        if second {
            return Act::Fit; // `self.tap` is already cleared, so three taps are not two doubles
        }
        self.tap = Some((now, p));
        Act::None
    }

    /// Forget one finger and re-seed the pinch, whatever ended it.
    fn drop_finger(&mut self, id: u64) -> Option<Finger> {
        let i = self.fingers.iter().position(|f| f.id == id)?;
        self.span = 0.0;
        Some(self.fingers.remove(i))
    }
}

impl Default for Touches {
    fn default() -> Self {
        Self::new()
    }
}
