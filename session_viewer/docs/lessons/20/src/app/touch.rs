//! Turns raw touch points into one camera gesture: one finger orbits, two fingers pan and zoom.
use winit::event::{Touch, TouchPhase};

use crate::camera::Camera;
use crate::engine::performance::now_ms;

/// 2·tan(30°) / 0.0015: with a 60° view the point under the finger stays under it; `pan` multiplies by 0.0015.
const PAN_PER_PX: f64 = 2.0 * 0.577_350_269_189_625_7 / 0.001_5; // 769.8

/// ln 0.9: zoom counts wheel steps of x0.9, so a pinch ratio r is -ln r / ln 0.9 steps, i.e. distance / r.
const PINCH_LOG: f64 = -0.105_360_515_657_826_28;

/// Largest pinch ratio one event may apply.
const PINCH_MAX: f64 = 2.0;

/// A finger moving less than this many pixels is a tap.
const TAP_SLOP: f64 = 12.0;

/// A finger lifting within this many ms is a tap.
const TAP_MS: f64 = 300.0;

/// A second tap within this many ms is a double tap.
const DOUBLE_TAP_MS: f64 = 320.0;

/// A second tap within this many pixels is a double tap.
const DOUBLE_TAP_SLOP: f64 = 40.0;

/// What one touch event asks the caller to do.
pub enum Act {
    None,
    Moved, // the camera moved, redraw
    Fit, // double tap: fit the scene
    Tap((f64, f64)), // single tap: pick at these pixels
}

/// One finger on the screen, in physical pixels.
struct Finger {
    id: u64, // the same while this finger stays down
    pos: (f64, f64),
    down: (f64, f64), // where it landed
    t0: f64, // when it landed, ms
}

/// Every finger down and the last two-finger measurement.
pub struct Touches {
    fingers: Vec<Finger>,
    span: f64, // last distance between the first two, 0 = not yet measured
    mid: (f64, f64), // last midpoint of the first two
    tap: Option<(f64, (f64, f64))>, // when and where the last tap lifted
}

impl Touches {
    /// No fingers down, no tap pending.
    pub fn new() -> Self {
        Self {
            fingers: Vec::new(),
            span: 0.0,
            mid: (0.0, 0.0),
            tap: None,
        }
    }

    /// Apply one touch event to the camera; `vp` is the viewport size in pixels.
    pub fn event(&mut self, cam: &mut Camera, t: &Touch, vp: (f64, f64), dpr: f64) -> Act {
        let p = (t.location.x, t.location.y);

        match t.phase {
            TouchPhase::Started => {
                self.fingers.push(Finger {
                    id: t.id,
                    pos: p,
                    down: p,
                    t0: now_ms(),
                });
                self.span = 0.0; // finger count changed, measure again
                Act::None
            }
            TouchPhase::Moved => self.moved(cam, t.id, p, vp, dpr),
            TouchPhase::Ended => self.lifted(t.id, p, dpr),
            // the browser took the gesture away
            TouchPhase::Cancelled => {
                self.drop_finger(t.id);
                self.tap = None;
                Act::None
            }
        }
    }

    /// One finger orbits; two pan by their midpoint and zoom by their distance.
    fn moved(&mut self, cam: &mut Camera, id: u64, p: (f64, f64), vp: (f64, f64), dpr: f64) -> Act {
        let Some(i) = self.finger_index(id) else {
            return Act::None;
        };
        let d = (p.0 - self.fingers[i].pos.0, p.1 - self.fingers[i].pos.1);
        self.fingers[i].pos = p;

        if self.fingers.len() == 1 {
            cam.orbit((d.0 / dpr) as f32, (d.1 / dpr) as f32);
            return Act::Moved;
        }

        // always the first two fingers, even with more down
        let (a, b) = (self.fingers[0].pos, self.fingers[1].pos);
        let span = (b.0 - a.0).hypot(b.1 - a.1).max(1.0);
        let mid = ((a.0 + b.0) * 0.5, (a.1 + b.1) * 0.5);

        if self.span == 0.0 {
            self.span = span; // first measurement: record only
            self.mid = mid;
            return Act::None;
        }

        // pan by the midpoint's move, then zoom about it
        let h = vp.1.max(1.0);
        cam.pan(
            ((mid.0 - self.mid.0) * PAN_PER_PX / h) as f32,
            ((mid.1 - self.mid.1) * PAN_PER_PX / h) as f32,
        );
        let r = (span / self.span).clamp(1.0 / PINCH_MAX, PINCH_MAX);
        cam.zoom_at((-r.ln() / PINCH_LOG) as f32, mid, vp);

        self.span = span;
        self.mid = mid;
        Act::Moved
    }

    /// A finger lifted; the last one up may be a tap.
    fn lifted(&mut self, id: u64, p: (f64, f64), dpr: f64) -> Act {
        let Some(f) = self.drop_finger(id) else {
            return Act::None;
        };

        if !self.fingers.is_empty() {
            self.tap = None;
            return Act::None;
        }

        let now = now_ms();

        if (p.0 - f.down.0).hypot(p.1 - f.down.1) / dpr > TAP_SLOP || now - f.t0 > TAP_MS {
            self.tap = None; // a drag or a long press
            return Act::None;
        }

        let second = match self.tap.take() {
            Some((t0, at)) => {
                now - t0 < DOUBLE_TAP_MS && (p.0 - at.0).hypot(p.1 - at.1) / dpr < DOUBLE_TAP_SLOP
            }
            None => false,
        };

        if second {
            return Act::Fit; // double tap
        }

        self.tap = Some((now, p));
        Act::Tap(p)
    }

    fn finger_index(&self, id: u64) -> Option<usize> {
        for (index, finger) in self.fingers.iter().enumerate() {
            if finger.id == id {
                return Some(index);
            }
        }

        None
    }

    /// Remove one finger and reset the pinch.
    fn drop_finger(&mut self, id: u64) -> Option<Finger> {
        let i = self.finger_index(id)?;
        self.span = 0.0;
        Some(self.fingers.remove(i))
    }
}

impl Default for Touches {
    /// Clippy asks for Default whenever a type has a no-argument `new`.
    fn default() -> Self {
        Self::new()
    }
}
