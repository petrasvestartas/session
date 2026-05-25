//! Gumball 3D manipulation widget.
//! Pure CPU logic — no wgpu dependency. All coordinates in mm (session units).

use crate::pick::Ray;

const ARROW_LEN: f32 = 150.0;
const ARROW_CAP: f32 = 18.0;
const ARC_RADIUS: f32 = 120.0;
const SPHERE_R: f32 = 16.0;
const ARC_SEGS: usize = 36;
const PICK_TOL: f32 = 12.0;

pub const SCREEN_PX: f32 = 160.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HandleKind {
    TranslateX,
    TranslateY,
    TranslateZ,
    RotateX,
    RotateY,
    RotateZ,
    ScaleUniform,
}

#[derive(Clone, Copy)]
pub struct GumballLine {
    pub a: [f32; 3],
    pub b: [f32; 3],
    pub color: [u8; 4],
}

#[derive(Clone)]
pub struct DragState {
    pub handle: HandleKind,
    pub start_world: [f32; 3],
    pub start_angle: f32,
    pub plane_normal: [f32; 3],
    pub start_dist: f32,
    pub drag_start_origin: [f32; 3],
}

pub struct Gumball {
    pub origin: [f32; 3],
    pub drag: Option<DragState>,
    pub hovered: Option<HandleKind>,
}

impl Gumball {
    pub fn new(origin: [f32; 3]) -> Self {
        Self { origin, drag: None, hovered: None }
    }

    pub fn set_origin(&mut self, origin: [f32; 3]) {
        self.origin = origin;
        self.drag = None;
        self.hovered = None;
    }

    pub fn hit_test(&self, ray: Ray, scale: f32) -> Option<HandleKind> {
        hit_test_at(ray, self.origin, scale)
    }
}

pub fn compute_scale(
    cam_pos_mm: [f32; 3],
    origin: [f32; 3],
    fov_y: f32,
    vp_h: f32,
) -> f32 {
    let d = sub3(cam_pos_mm, origin);
    let dist = len3(d).max(1.0);
    let world_per_px = 2.0 * dist * (fov_y * 0.5).tan() / vp_h.max(1.0);
    (SCREEN_PX * world_per_px) / ARC_RADIUS
}

pub fn hit_test_at(ray: Ray, origin: [f32; 3], scale: f32) -> Option<HandleKind> {
    let tol = PICK_TOL * scale;
    let tol_arc = tol * 0.5;

    if ray_vs_sphere(ray, origin, SPHERE_R * scale) {
        return Some(HandleKind::ScaleUniform);
    }

    let mut best_dist = f32::MAX;
    let mut best: Option<HandleKind> = None;
    for (kind, axis) in [
        (HandleKind::TranslateX, [1.0f32, 0.0, 0.0]),
        (HandleKind::TranslateY, [0.0, 1.0, 0.0]),
        (HandleKind::TranslateZ, [0.0, 0.0, 1.0]),
    ] {
        if let Some(d) = ray_vs_arrow(ray, origin, axis, ARROW_LEN * scale, tol) {
            if d < best_dist { best_dist = d; best = Some(kind); }
        }
    }
    if best.is_some() { return best; }

    for (kind, axis) in [
        (HandleKind::RotateX, [1.0f32, 0.0, 0.0]),
        (HandleKind::RotateY, [0.0, 1.0, 0.0]),
        (HandleKind::RotateZ, [0.0, 0.0, 1.0]),
    ] {
        if let Some(d) = ray_vs_arc(ray, origin, axis, ARC_RADIUS * scale, tol_arc) {
            if d < best_dist { best_dist = d; best = Some(kind); }
        }
    }
    best
}

pub fn begin_drag(handle: HandleKind, ray: Ray, origin: [f32; 3], _scale: f32) -> DragState {
    match handle {
        HandleKind::TranslateX => {
            let p = project_ray_on_axis(ray, origin, [1.0, 0.0, 0.0]).unwrap_or(origin);
            DragState { handle, start_world: p, start_angle: 0.0, plane_normal: [0.0; 3], start_dist: 0.0, drag_start_origin: origin }
        }
        HandleKind::TranslateY => {
            let p = project_ray_on_axis(ray, origin, [0.0, 1.0, 0.0]).unwrap_or(origin);
            DragState { handle, start_world: p, start_angle: 0.0, plane_normal: [0.0; 3], start_dist: 0.0, drag_start_origin: origin }
        }
        HandleKind::TranslateZ => {
            let p = project_ray_on_axis(ray, origin, [0.0, 0.0, 1.0]).unwrap_or(origin);
            DragState { handle, start_world: p, start_angle: 0.0, plane_normal: [0.0; 3], start_dist: 0.0, drag_start_origin: origin }
        }
        HandleKind::RotateX => {
            let a = ray_plane_point(ray, origin, [1.0, 0.0, 0.0])
                .map(|p| rot_plane_angle(p, origin, [1.0, 0.0, 0.0]))
                .unwrap_or(0.0);
            DragState { handle, start_world: origin, start_angle: a, plane_normal: [0.0; 3], start_dist: 0.0, drag_start_origin: origin }
        }
        HandleKind::RotateY => {
            let a = ray_plane_point(ray, origin, [0.0, 1.0, 0.0])
                .map(|p| rot_plane_angle(p, origin, [0.0, 1.0, 0.0]))
                .unwrap_or(0.0);
            DragState { handle, start_world: origin, start_angle: a, plane_normal: [0.0; 3], start_dist: 0.0, drag_start_origin: origin }
        }
        HandleKind::RotateZ => {
            let a = ray_plane_point(ray, origin, [0.0, 0.0, 1.0])
                .map(|p| rot_plane_angle(p, origin, [0.0, 0.0, 1.0]))
                .unwrap_or(0.0);
            DragState { handle, start_world: origin, start_angle: a, plane_normal: [0.0; 3], start_dist: 0.0, drag_start_origin: origin }
        }
        HandleKind::ScaleUniform => {
            let pn = best_plane_normal(ray.direction);
            let p = ray_plane_point_n(ray, origin, pn).unwrap_or(origin);
            let dist = len3(sub3(p, origin)).max(1e-4);
            DragState { handle, start_world: p, start_angle: 0.0, plane_normal: pn, start_dist: dist, drag_start_origin: origin }
        }
    }
}

pub fn update_drag(ds: &DragState, ray: Ray, origin: [f32; 3], _scale: f32) -> Option<[[f32; 4]; 4]> {
    match ds.handle {
        HandleKind::TranslateX => translate_drag(ds, ray, origin, [1.0, 0.0, 0.0]),
        HandleKind::TranslateY => translate_drag(ds, ray, origin, [0.0, 1.0, 0.0]),
        HandleKind::TranslateZ => translate_drag(ds, ray, origin, [0.0, 0.0, 1.0]),
        HandleKind::RotateX => rotate_drag(ds, ray, origin, [1.0, 0.0, 0.0]),
        HandleKind::RotateY => rotate_drag(ds, ray, origin, [0.0, 1.0, 0.0]),
        HandleKind::RotateZ => rotate_drag(ds, ray, origin, [0.0, 0.0, 1.0]),
        HandleKind::ScaleUniform => scale_drag(ds, ray, origin),
    }
}

pub fn build_lines(origin: [f32; 3], scale: f32, hovered: Option<HandleKind>) -> Vec<GumballLine> {
    let mut out = Vec::with_capacity(256);
    let r = |rgb: [u8; 3], h: HandleKind| -> [u8; 4] {
        if hovered == Some(h) { [255, 220, 0, 255] } else { [rgb[0], rgb[1], rgb[2], 255] }
    };

    for (axis, rgb, kind) in [
        ([1.0f32, 0.0, 0.0], [220u8, 60, 60], HandleKind::TranslateX),
        ([0.0, 1.0, 0.0],    [60, 200, 60],   HandleKind::TranslateY),
        ([0.0, 0.0, 1.0],    [60, 100, 220],  HandleKind::TranslateZ),
    ] {
        let col = r(rgb, kind);
        let tip = add3(origin, scale3(axis, ARROW_LEN * scale));
        let cap = ARROW_CAP * scale;
        out.push(GumballLine { a: origin, b: tip, color: col });
        let (pa, pb) = perp_pair(axis);
        for perp in [pa, pb, scale3(pa, -1.0), scale3(pb, -1.0)] {
            let base = add3(tip, add3(scale3(axis, -cap), scale3(perp, cap * 0.5)));
            out.push(GumballLine { a: base, b: tip, color: col });
        }
    }

    for (axis, rgb, kind) in [
        ([1.0f32, 0.0, 0.0], [220u8, 60, 60], HandleKind::RotateX),
        ([0.0, 1.0, 0.0],    [60, 200, 60],   HandleKind::RotateY),
        ([0.0, 0.0, 1.0],    [60, 100, 220],  HandleKind::RotateZ),
    ] {
        let col = r(rgb, kind);
        let rad = ARC_RADIUS * scale;
        let (u, v) = perp_pair(axis);
        for i in 0..ARC_SEGS {
            let a0 = std::f32::consts::TAU * i as f32 / ARC_SEGS as f32;
            let a1 = std::f32::consts::TAU * (i + 1) as f32 / ARC_SEGS as f32;
            let p0 = add3(origin, add3(scale3(u, rad * a0.cos()), scale3(v, rad * a0.sin())));
            let p1 = add3(origin, add3(scale3(u, rad * a1.cos()), scale3(v, rad * a1.sin())));
            out.push(GumballLine { a: p0, b: p1, color: col });
        }
    }

    let col_sph = r([200, 200, 200], HandleKind::ScaleUniform);
    let sph_r = SPHERE_R * scale;
    for (u, v) in [
        ([1.0f32, 0.0, 0.0], [0.0f32, 1.0, 0.0]),
        ([1.0, 0.0, 0.0],    [0.0, 0.0, 1.0]),
        ([0.0, 1.0, 0.0],    [0.0, 0.0, 1.0]),
    ] {
        for i in 0..24 {
            let a0 = std::f32::consts::TAU * i as f32 / 24.0;
            let a1 = std::f32::consts::TAU * (i + 1) as f32 / 24.0;
            let p0 = add3(origin, add3(scale3(u, sph_r * a0.cos()), scale3(v, sph_r * a0.sin())));
            let p1 = add3(origin, add3(scale3(u, sph_r * a1.cos()), scale3(v, sph_r * a1.sin())));
            out.push(GumballLine { a: p0, b: p1, color: col_sph });
        }
    }

    out
}

pub fn mat4_mul(a: &[[f32; 4]; 4], b: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut out = [[0.0f32; 4]; 4];
    for c in 0..4 {
        for row in 0..4 {
            let mut s = 0.0f32;
            for k in 0..4 { s += a[k][row] * b[c][k]; }
            out[c][row] = s;
        }
    }
    out
}

fn translate_drag(ds: &DragState, ray: Ray, origin: [f32; 3], axis: [f32; 3]) -> Option<[[f32; 4]; 4]> {
    let cur = project_ray_on_axis(ray, origin, axis)?;
    let delta = sub3(cur, ds.start_world);
    Some(mat4_translation(delta[0], delta[1], delta[2]))
}

fn rotate_drag(ds: &DragState, ray: Ray, origin: [f32; 3], axis: [f32; 3]) -> Option<[[f32; 4]; 4]> {
    let p = ray_plane_point(ray, origin, axis)?;
    let angle = rot_plane_angle(p, origin, axis);
    let dtheta = angle - ds.start_angle;
    let rot = mat4_rot_axis(axis, dtheta);
    let tp = mat4_translation(origin[0], origin[1], origin[2]);
    let tn = mat4_translation(-origin[0], -origin[1], -origin[2]);
    Some(mat4_mul(&mat4_mul(&tp, &rot), &tn))
}

fn scale_drag(ds: &DragState, ray: Ray, origin: [f32; 3]) -> Option<[[f32; 4]; 4]> {
    let p = ray_plane_point_n(ray, origin, ds.plane_normal)?;
    let dist = len3(sub3(p, origin)).max(1e-8);
    let ratio = (dist / ds.start_dist).max(0.01);
    let sc = mat4_scale_uniform(ratio);
    let tp = mat4_translation(origin[0], origin[1], origin[2]);
    let tn = mat4_translation(-origin[0], -origin[1], -origin[2]);
    Some(mat4_mul(&mat4_mul(&tp, &sc), &tn))
}

fn ray_vs_arrow(ray: Ray, origin: [f32; 3], axis: [f32; 3], length: f32, tol: f32) -> Option<f32> {
    let w = sub3(ray.origin, origin);
    let d = ray.direction;
    let b = dot3(d, axis);
    let dd = dot3(d, w);
    let e = dot3(axis, w);
    let denom = 1.0 - b * b;
    let (s, tc) = if denom.abs() < 1e-8 {
        (0.0f32, dd)
    } else {
        let s = ((b * e - dd) / denom).clamp(0.0, length);
        let tc = b * s - dd;
        (s, tc)
    };
    if tc < 0.0 { return None; }
    let cs = add3(origin, scale3(axis, s));
    let cr = add3(ray.origin, scale3(d, tc));
    let dist = len3(sub3(cs, cr));
    if dist < tol { Some(dist) } else { None }
}

fn ray_vs_sphere(ray: Ray, center: [f32; 3], radius: f32) -> bool {
    let oc = sub3(ray.origin, center);
    let b = 2.0 * dot3(oc, ray.direction);
    let c = dot3(oc, oc) - radius * radius;
    b * b - 4.0 * c >= 0.0
}

fn ray_vs_arc(ray: Ray, origin: [f32; 3], axis: [f32; 3], radius: f32, tol: f32) -> Option<f32> {
    let p = ray_plane_point(ray, origin, axis)?;
    let dist = (len3(sub3(p, origin)) - radius).abs();
    if dist < tol { Some(dist) } else { None }
}

fn project_ray_on_axis(ray: Ray, origin: [f32; 3], axis: [f32; 3]) -> Option<[f32; 3]> {
    let w = sub3(ray.origin, origin);
    let b = dot3(ray.direction, axis);
    let dd = dot3(ray.direction, w);
    let e = dot3(axis, w);
    let denom = 1.0 - b * b;
    if denom.abs() < 1e-8 { return None; }
    let t = (e - b * dd) / denom;
    Some(add3(origin, scale3(axis, t)))
}

fn ray_plane_point(ray: Ray, origin: [f32; 3], normal: [f32; 3]) -> Option<[f32; 3]> {
    ray_plane_point_n(ray, origin, normal)
}

fn ray_plane_point_n(ray: Ray, origin: [f32; 3], normal: [f32; 3]) -> Option<[f32; 3]> {
    let denom = dot3(ray.direction, normal);
    if denom.abs() < 1e-8 { return None; }
    let t = dot3(sub3(origin, ray.origin), normal) / denom;
    if t < 0.0 { return None; }
    Some(add3(ray.origin, scale3(ray.direction, t)))
}

fn rot_plane_angle(p: [f32; 3], origin: [f32; 3], axis: [f32; 3]) -> f32 {
    let ref_v: [f32; 3] = if axis[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
    let u = normalize3(cross3(axis, ref_v));
    let v = cross3(axis, u);
    let dp = sub3(p, origin);
    f32::atan2(dot3(dp, v), dot3(dp, u))
}

fn best_plane_normal(dir: [f32; 3]) -> [f32; 3] {
    let ax = dir[0].abs();
    let ay = dir[1].abs();
    let az = dir[2].abs();
    if ax >= ay && ax >= az { [1.0, 0.0, 0.0] }
    else if ay >= az { [0.0, 1.0, 0.0] }
    else { [0.0, 0.0, 1.0] }
}

fn perp_pair(axis: [f32; 3]) -> ([f32; 3], [f32; 3]) {
    let ref_v: [f32; 3] = if axis[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
    let u = normalize3(cross3(axis, ref_v));
    let v = cross3(axis, u);
    (u, v)
}

fn mat4_translation(tx: f32, ty: f32, tz: f32) -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [tx,  ty,  tz,  1.0],
    ]
}

fn mat4_scale_uniform(s: f32) -> [[f32; 4]; 4] {
    [
        [s,   0.0, 0.0, 0.0],
        [0.0, s,   0.0, 0.0],
        [0.0, 0.0, s,   0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

fn mat4_rot_axis(axis: [f32; 3], theta: f32) -> [[f32; 4]; 4] {
    let c = theta.cos();
    let s = theta.sin();
    let t = 1.0 - c;
    let [x, y, z] = axis;
    [
        [t*x*x+c,   t*x*y+s*z, t*x*z-s*y, 0.0],
        [t*x*y-s*z, t*y*y+c,   t*y*z+s*x, 0.0],
        [t*x*z+s*y, t*y*z-s*x, t*z*z+c,   0.0],
        [0.0,       0.0,       0.0,        1.0],
    ]
}

fn add3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] { [a[0]+b[0], a[1]+b[1], a[2]+b[2]] }
fn sub3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] { [a[0]-b[0], a[1]-b[1], a[2]-b[2]] }
fn scale3(a: [f32; 3], s: f32) -> [f32; 3] { [a[0]*s, a[1]*s, a[2]*s] }
fn dot3(a: [f32; 3], b: [f32; 3]) -> f32 { a[0]*b[0] + a[1]*b[1] + a[2]*b[2] }
fn len3(a: [f32; 3]) -> f32 { dot3(a, a).sqrt() }
fn cross3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
}
fn normalize3(a: [f32; 3]) -> [f32; 3] {
    let l = len3(a).max(1e-30);
    [a[0]/l, a[1]/l, a[2]/l]
}
