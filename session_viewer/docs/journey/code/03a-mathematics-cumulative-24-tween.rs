// --8<-- [start:clock]
use super::tex::Rgba;

/// Smootherstep easing of `t` in 0..1.
pub fn smooth(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    let s = 1.0 - t;
    t * t * t * (10.0 * s * s + 5.0 * s * t + t * t)
}

/// Ease up to 1 at the midpoint and back to 0.
pub(super) fn there_and_back(t: f64) -> f64 {
    smooth(if t < 0.5 { 2.0 * t } else { 2.0 * (1.0 - t) })
}

/// Blend two colours.
pub(super) fn mix(a: Rgba, b: Rgba, t: f32) -> Rgba {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
        a[3] + (b[3] - a[3]) * t,
    ]
}

/// Blend two numbers.
pub(super) fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// The colour with its alpha scaled by `a`.
pub(super) fn with_alpha(mut c: Rgba, a: f32) -> Rgba {
    c[3] *= a;
    c
}

/// A value moving from `from` to `to`.
#[derive(Clone, Copy, Debug)]
pub struct Tween<T: Copy> {
    from: T,
    pub(super) to: T,
    t0: f64,
    dur: f64,
}

/// Values a tween can blend.
pub trait Lerp: Copy {
    fn lerp(a: Self, b: Self, t: f64) -> Self;
}

impl Lerp for f32 {
    fn lerp(a: f32, b: f32, t: f64) -> f32 {
        a + (b - a) * t as f32
    }
}

impl Lerp for [f32; 2] {
    fn lerp(a: Self, b: Self, t: f64) -> Self {
        [f32::lerp(a[0], b[0], t), f32::lerp(a[1], b[1], t)]
    }
}

impl Lerp for [f64; 3] {
    fn lerp(a: Self, b: Self, t: f64) -> Self {
        [
            a[0] + (b[0] - a[0]) * t,
            a[1] + (b[1] - a[1]) * t,
            a[2] + (b[2] - a[2]) * t,
        ]
    }
}

impl Lerp for Rgba {
    fn lerp(a: Self, b: Self, t: f64) -> Self {
        mix(a, b, t as f32)
    }
}

impl<T: Lerp> Tween<T> {
    /// A value at rest.
    pub(super) fn fixed(v: T) -> Self {
        Tween {
            from: v,
            to: v,
            t0: 0.0,
            dur: 0.0,
        }
    }

    /// The value at time `now`.
    pub(super) fn at(&self, now: f64) -> T {
        if self.dur <= 0.0 || now >= self.t0 + self.dur {
            return self.to;
        }
        if now <= self.t0 {
            return self.from;
        }
        T::lerp(self.from, self.to, smooth((now - self.t0) / self.dur))
    }

    /// Move from the current value to `to` over `dur` milliseconds.
    pub(super) fn set(&mut self, to: T, now: f64, dur: f64) {
        self.from = self.at(now);
        self.to = to;
        self.t0 = now;
        self.dur = dur;
    }

    /// Move to `to` when `animate`, otherwise jump there.
    pub(super) fn retarget(&mut self, to: T, now: f64, dur: f64, animate: bool) {
        if animate {
            self.set(to, now, dur);
        } else {
            *self = Tween::fixed(to);
        }
    }

    /// Jump to the end.
    pub(super) fn snap(&mut self) {
        self.from = self.to;
        self.dur = 0.0;
    }

    /// Whether the value is still moving at `now`.
    pub(super) fn busy(&self, now: f64) -> bool {
        self.dur > 0.0 && now < self.t0 + self.dur
    }
}
// --8<-- [end:clock]
