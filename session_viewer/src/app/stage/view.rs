use super::deck::CameraSpec;
use super::eval;
use super::run::{H, Stage, TOP, W};
use super::tween::Lerp;

/// An orthographic view: angles in degrees, zoom in slide pixels per world unit.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cam {
    pub yaw: f64,
    pub pitch: f64,
    pub zoom: f64,
    pub center: [f64; 3],
}

impl Lerp for Cam {
    fn lerp(a: Self, b: Self, t: f64) -> Self {
        Cam {
            yaw: a.yaw + (b.yaw - a.yaw) * t,
            pitch: a.pitch + (b.pitch - a.pitch) * t,
            zoom: a.zoom * (b.zoom / a.zoom).powf(t),
            center: <[f64; 3]>::lerp(a.center, b.center, t),
        }
    }
}

/// The projection of one frame.
#[derive(Clone, Copy)]
pub struct View {
    right: [f64; 3],     // World direction of slide x.
    up: [f64; 3],        // World direction of slide up.
    pub(super) cam: Cam, // The authored camera.
    origin: [f32; 2],    // Slide point of the camera centre.
    lens: Option<Lens>,  // The real camera, used instead when set.
}

/// The real camera as an affine map from world to slide pixels: `a * p + b`.
#[derive(Clone, Copy, Debug)]
pub struct Lens {
    pub a: [[f64; 3]; 2],
    pub b: [f64; 2],
}

/// Dot product.
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

impl View {
    /// The orthographic view of `cam` with its centre at slide point `origin`.
    fn new(cam: Cam, origin: [f32; 2]) -> Self {
        let (y, p) = (cam.yaw.to_radians(), cam.pitch.to_radians());
        let eye = [p.cos() * y.cos(), p.cos() * y.sin(), p.sin()];
        let right = [-y.sin(), y.cos(), 0.0];
        let up = [
            eye[1] * right[2] - eye[2] * right[1],
            eye[2] * right[0] - eye[0] * right[2],
            eye[0] * right[1] - eye[1] * right[0],
        ];
        View {
            right,
            up,
            cam,
            origin,
            lens: None,
        }
    }

    /// A world point on the slide.
    pub fn project(&self, p: [f64; 3]) -> [f32; 2] {
        if let Some(l) = &self.lens {
            return [
                (dot(l.a[0], p) + l.b[0]) as f32,
                (dot(l.a[1], p) + l.b[1]) as f32,
            ];
        }
        let d = [
            p[0] - self.cam.center[0],
            p[1] - self.cam.center[1],
            p[2] - self.cam.center[2],
        ];
        [
            self.origin[0] + (dot(d, self.right) * self.cam.zoom) as f32,
            self.origin[1] - (dot(d, self.up) * self.cam.zoom) as f32,
        ]
    }
}

impl Stage {
    /// The authored camera with the user's orbit and zoom applied.
    pub(super) fn view(&self, now: f64) -> View {
        let mut cam = self.cam.at(now);
        cam.yaw += self.user[0];
        cam.pitch = (cam.pitch + self.user[1]).clamp(-89.0, 89.9);
        cam.zoom *= self.user[2];
        let mut v = View::new(cam, self.origin());
        v.lens = self.lens;
        v
    }

    /// The authored camera at `now`, and whether it is still moving.
    pub fn camera(&self, now: f64) -> (Cam, bool) {
        (self.cam.at(now), self.cam.busy(now))
    }

    /// Where the camera centre sits on the slide.
    pub fn origin(&self) -> [f32; 2] {
        [self.split * W / 2.0, (TOP - 30.0 + H - 70.0) / 2.0]
    }

    /// The camera a spec asks for from `cur`; `fit_all` frames every point by default.
    pub(super) fn camera_for(&self, spec: &CameraSpec, cur: Cam, fit_all: bool) -> Cam {
        let mut cam = cur;
        if let Some(v) = &spec.view {
            let (y, p) = match v.as_str() {
                "front" => (-90.0, 0.0),
                "back" => (90.0, 0.0),
                "top" => (-90.0, 89.9),
                "side" | "right" => (0.0, 0.0),
                "left" => (180.0, 0.0),
                _ => (-58.0, 26.0),
            };
            cam.yaw = y;
            cam.pitch = p;
        }
        if let Some(y) = spec.yaw {
            cam.yaw = y;
        }
        if let Some(p) = spec.pitch {
            cam.pitch = p;
        }
        let ps = self.points_at(self.now);
        let mut fit: Vec<[f64; 3]> = spec
            .fit
            .iter()
            .filter_map(|p| eval::position(p, &ps).ok())
            .collect();
        if fit.is_empty() && fit_all && spec.zoom.is_none() {
            fit = self.points.values().map(|t| t.to).collect();
        }
        if !fit.is_empty() {
            let n = fit.len() as f64;
            let mut c = [0.0; 3];
            for p in &fit {
                for k in 0..3 {
                    c[k] += p[k] / n;
                }
            }
            cam.center = c;
            cam.zoom = 1.0;
            let v = View::new(cam, [0.0, 0.0]);
            let pr: Vec<[f32; 2]> = fit.iter().map(|&p| v.project(p)).collect();
            let (mut lo, mut hi) = ([f32::MAX; 2], [f32::MIN; 2]);
            for p in &pr {
                for k in 0..2 {
                    lo[k] = lo[k].min(p[k]);
                    hi[k] = hi[k].max(p[k]);
                }
            }
            let (bx, by) = (
                ((lo[0] + hi[0]) / 2.0) as f64,
                ((lo[1] + hi[1]) / 2.0) as f64,
            );
            for k in 0..3 {
                cam.center[k] += v.right[k] * bx - v.up[k] * by;
            }
            let pw = self.split * W - 120.0;
            let ph = H - TOP - 70.0 - 90.0;
            let zx = pw / (hi[0] - lo[0]).max(1e-3);
            let zy = ph / (hi[1] - lo[1]).max(1e-3);
            cam.zoom = zx.min(zy) as f64;
        }
        if let Some(c) = &spec.center
            && let Ok(p) = eval::position(c, &ps)
        {
            cam.center = p;
        }
        if let Some(z) = spec.zoom {
            cam.zoom = z;
        }
        cam
    }
}
