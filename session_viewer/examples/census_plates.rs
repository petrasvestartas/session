//! Plate census: per-mesh AABB extents + face-normal thickness, per-polyline distance to the nearest mesh face plane, the fit camera, and the depth rule judged by ray-casting every outline sample against the plates in front of it at 1x, 4x and 16x the fit distance (VIEWER_W/H size the pen; CENSUS_RECOLOR=<out.pb> writes a copy whose outline segments are magenta when covered, blue when visible, cyan when partly covered).

use session_rust::{Color, Mesh, Polyline, Quaternion, Session, Vector, Xform};
use session_viewer::math::{mat_scale, mat_to_f32, xform_point_f64, Mat4};
use std::cmp::Ordering;
use std::collections::HashMap;

// The rule under test (objects.rs, triangle.wgsl, ribbon.wgsl) and the harness pen.
const THICK_FLOOR: f64 = 0.001;
const PUSH_FRAC: f64 = 0.004;
const PUSH_MAX_THICK: f64 = 0.25;
const LIFT_RADII_FREE: f64 = 1.0;
const LIFT_MAX_THICK: f64 = 0.25;
const PEN_PX: f64 = 2.0;
const ON_FACE_TOL: f64 = 0.01;
const SAMPLE_MM: f64 = 50.0;
const SCALES: [f64; 3] = [1.0, 4.0, 16.0];

struct Face {
    n: [f64; 3],
    d: f64,
    behind: f64,
}

struct Plate {
    verts: Vec<[f64; 3]>,
    faces: Vec<Face>,
    tris: Vec<[[f64; 3]; 3]>,
    lo: [f64; 3],
    hi: [f64; 3],
    ext: [f64; 3],
    diag: f64,
    t_rule: f64,
    t_real: f64,
    big_nz: f64,
}

struct Outline {
    pts: Vec<[f64; 3]>,
    samples: Vec<[f64; 3]>,
    lo: [f64; 3],
    hi: [f64; 3],
    ext: [f64; 3],
    diag: f64,
    t_rule: f64,
    nz: f64,
    dist: f64,
    plate: usize,
    face: usize,
}

struct Fit {
    eye: [f64; 3],
    fwd: [f64; 3],
    distance: f64,
}

// One outline sample against the rule: covered by a plate in front?, its eye depth (m), the
// binding cover's separation along the ray, its push, the sample's lift, the margin (mm), the
// binding plate.
#[derive(Clone)]
struct Verdict {
    covered: bool,
    w: f64,
    sep: f64,
    push: f64,
    lift: f64,
    margin: f64,
    plate: usize,
}

fn sub(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn dot(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn norm(a: &[f64; 3]) -> f64 {
    dot(a, a).sqrt()
}

fn unit(a: &[f64; 3]) -> [f64; 3] {
    let l = norm(a).max(1e-300);
    [a[0] / l, a[1] / l, a[2] / l]
}

fn by_first(a: &(f64, usize), b: &(f64, usize)) -> Ordering {
    a.0.total_cmp(&b.0)
}

fn env_f64(name: &str, default: f64) -> f64 {
    match std::env::var(name) {
        Ok(v) => v.trim().parse().unwrap_or(default),
        Err(_) => default,
    }
}

fn grow(lo: &mut [f64; 3], hi: &mut [f64; 3], p: &[f64; 3]) {
    for k in 0..3 {
        lo[k] = lo[k].min(p[k]);
        hi[k] = hi[k].max(p[k]);
    }
}

fn box_of(pts: &[[f64; 3]]) -> ([f64; 3], [f64; 3]) {
    let mut lo = [f64::INFINITY; 3];
    let mut hi = [f64::NEG_INFINITY; 3];
    for p in pts {
        grow(&mut lo, &mut hi, p);
    }
    (lo, hi)
}

fn sorted_extents(lo: &[f64; 3], hi: &[f64; 3]) -> [f64; 3] {
    let mut e = [hi[0] - lo[0], hi[1] - lo[1], hi[2] - lo[2]];
    e.sort_by(f64::total_cmp);
    e
}

fn centroid(pts: &[[f64; 3]]) -> [f64; 3] {
    let mut c = [0.0; 3];
    for p in pts {
        for k in 0..3 {
            c[k] += p[k];
        }
    }
    let n = pts.len().max(1) as f64;
    [c[0] / n, c[1] / n, c[2] / n]
}

fn push_mm(w_m: f64, t_rule: f64) -> f64 {
    (PUSH_FRAC * w_m * 1000.0).min(PUSH_MAX_THICK * t_rule)
}

#[allow(dead_code)]
fn lift_free_mm(w_m: f64, t_rule: f64, vp_h: f64) -> f64 {
    let proj_y = 1.0 / 30.0_f64.to_radians().tan() * 0.001;
    let raw_px = (PEN_PX * 0.5).max(0.5);
    let uncapped = raw_px * LIFT_RADII_FREE * w_m / (proj_y * vp_h);
    uncapped.min(LIFT_MAX_THICK * t_rule)
}

fn stats(label: &str, v: &mut [f64]) {
    if v.is_empty() {
        println!("  {label}: none");
        return;
    }
    v.sort_by(f64::total_cmp);
    let n = v.len();
    let p10 = v[(n - 1) * 10 / 100];
    let med = v[(n - 1) / 2];
    let p90 = v[(n - 1) * 90 / 100];
    println!("  {label}: n={n} min {:.2} p10 {p10:.2} median {med:.2} p90 {p90:.2} max {:.2}", v[0], v[n - 1]);
}

fn zero_translation(m: &Mat4) -> Mat4 {
    let mut out = *m;
    out[12] = 0.0;
    out[13] = 0.0;
    out[14] = 0.0;
    out
}

fn plate_of(m: &Mesh, place: &Mat4) -> Plate {
    let mut local: Vec<[f64; 3]> = Vec::with_capacity(m.vertex.len());
    for key in m.vertices() {
        let v = &m.vertex[&key];
        local.push([v.x, v.y, v.z]);
    }
    let mut verts: Vec<[f64; 3]> = Vec::with_capacity(local.len());
    for p in &local {
        verts.push(xform_point_f64(place, *p));
    }
    let (lo, hi) = box_of(&verts);
    let ext = sorted_extents(&lo, &hi);
    let diag = norm(&sub(&hi, &lo));
    let mc = centroid(&verts);
    let mut faces = Vec::new();
    let mut tris = Vec::new();
    let mut t_real = f64::INFINITY;
    let mut big_area = 0.0;
    let mut big_nz = 0.0;
    for fk in m.faces() {
        let Some(nv) = m.face_normal(fk) else { continue };
        let Some(fpts) = m.face_points(fk) else { continue };
        let mut pts: Vec<[f64; 3]> = Vec::with_capacity(fpts.len());
        for p in &fpts {
            pts.push(xform_point_f64(place, [p[0], p[1], p[2]]));
        }
        for i in 1..pts.len().saturating_sub(1) {
            tris.push([pts[0], pts[i], pts[i + 1]]);
        }
        let mut n = unit(&xform_point_f64(&zero_translation(place), [nv[0], nv[1], nv[2]]));
        let c = centroid(&pts);
        if dot(&n, &sub(&c, &mc)) < 0.0 {
            n = [-n[0], -n[1], -n[2]];
        }
        let d = dot(&n, &c);
        let mut lo_n = f64::INFINITY;
        for v in &verts {
            lo_n = lo_n.min(dot(&n, v));
        }
        let behind = d - lo_n;
        t_real = t_real.min(behind);
        let area = m.face_area(fk).unwrap_or(0.0);
        if area > big_area {
            big_area = area;
            big_nz = n[2].abs();
        }
        faces.push(Face { n, d, behind });
    }
    if !t_real.is_finite() {
        t_real = 0.0;
    }
    let t_rule = t_real.max(THICK_FLOOR * diag);
    Plate { verts, faces, tris, lo, hi, ext, diag, t_rule, t_real, big_nz }
}

fn newell_nz(pts: &[[f64; 3]]) -> f64 {
    let mut n = [0.0; 3];
    for i in 0..pts.len() {
        let a = &pts[i];
        let b = &pts[(i + 1) % pts.len()];
        n[0] += (a[1] - b[1]) * (a[2] + b[2]);
        n[1] += (a[2] - b[2]) * (a[0] + b[0]);
        n[2] += (a[0] - b[0]) * (a[1] + b[1]);
    }
    if norm(&n) <= 0.0 {
        return f64::NAN;
    }
    unit(&n)[2].abs()
}

// The nearest mesh face plane (max point deviation) and the outline's samples: its vertices
// and its edge midpoints.
fn outline_of(pl: &Polyline, place: &Mat4, plates: &[Plate]) -> Outline {
    let mut local: Vec<[f64; 3]> = Vec::with_capacity(pl.coords.len() / 3);
    for c in pl.coords.chunks_exact(3) {
        local.push([c[0], c[1], c[2]]);
    }
    let scale = mat_scale(&mat_to_f32(place));
    let mut pts: Vec<[f64; 3]> = Vec::with_capacity(local.len());
    for p in &local {
        pts.push(xform_point_f64(place, *p));
    }
    let (lo, hi) = box_of(&pts);
    let ext = sorted_extents(&lo, &hi);
    let diag = norm(&sub(&hi, &lo));
    let mut nn = [0.0f64; 3];
    for i in 0..pts.len() {
        let (a, b) = (pts[i], pts[(i + 1) % pts.len()]);
        nn[0] += (a[1] - b[1]) * (a[2] + b[2]);
        nn[1] += (a[2] - b[2]) * (a[0] + b[0]);
        nn[2] += (a[0] - b[0]) * (a[1] + b[1]);
    }
    let nl = norm(&nn);
    let mut spread = 0.0;
    if nl > 0.0 {
        let (mut smin, mut smax) = (f64::INFINITY, f64::NEG_INFINITY);
        for p in &pts {
            let d = (p[0] * nn[0] + p[1] * nn[1] + p[2] * nn[2]) / nl;
            smin = smin.min(d);
            smax = smax.max(d);
        }
        spread = smax - smin;
    }
    let t_rule = (spread * scale).max(THICK_FLOOR * diag);
    let mut best = (f64::INFINITY, usize::MAX, usize::MAX);
    for (pi, plate) in plates.iter().enumerate() {
        for (fi, f) in plate.faces.iter().enumerate() {
            let mut dev: f64 = 0.0;
            for p in &pts {
                dev = dev.max((dot(&f.n, p) - f.d).abs());
            }
            if dev < best.0 {
                best = (dev, pi, fi);
            }
        }
    }
    let mut samples: Vec<[f64; 3]> = Vec::with_capacity(pts.len() * 2);
    for w in pts.windows(2) {
        samples.push(w[0]);
        let steps = (norm(&sub(&w[1], &w[0])) / SAMPLE_MM).ceil().max(1.0) as usize;
        for k in 1..steps {
            let t = k as f64 / steps as f64;
            samples.push([w[0][0] + (w[1][0] - w[0][0]) * t, w[0][1] + (w[1][1] - w[0][1]) * t, w[0][2] + (w[1][2] - w[0][2]) * t]);
        }
    }
    if let Some(last) = pts.last() && pts.first() != Some(last) {
        samples.push(*last);
    }
    let nz = newell_nz(&pts);
    let hosted = best.0 <= ON_FACE_TOL;
    let t_rule = if hosted { plates[best.1].t_real.max(THICK_FLOOR * plates[best.1].diag) } else { t_rule };
    Outline { pts, samples, lo, hi, ext, diag, t_rule, nz, dist: best.0, plate: best.1, face: best.2 }
}

fn iso_frame() -> ([f64; 3], [f64; 3], [f64; 3]) {
    let yaw_q = Quaternion::from_axis_angle(Vector::z_axis(), -std::f64::consts::FRAC_PI_6);
    let rv = yaw_q.rotate_vector(Vector::x_axis());
    let pitch_q = Quaternion::from_axis_angle(rv, -std::f64::consts::FRAC_PI_6);
    let o = (pitch_q * yaw_q).normalized();
    let f = o.rotate_vector(Vector::y_axis());
    let u = o.rotate_vector(Vector::z_axis());
    let r = o.rotate_vector(Vector::x_axis());
    ([f[0], f[1], f[2]], [u[0], u[1], u[2]], [r[0], r[1], r[2]])
}

// Camera::fit (src/camera.rs) at the default iso orientation, 60 deg vertical fov, mm -> m.
fn fit(lo: &[f64; 3], hi: &[f64; 3], aspect: f64) -> Fit {
    let (fwd, up, right) = iso_frame();
    let ty = 30.0_f64.to_radians().tan();
    let tx = aspect * ty;
    let s = 0.001;
    let target = [(lo[0] + hi[0]) * 0.5 * s, (lo[1] + hi[1]) * 0.5 * s, (lo[2] + hi[2]) * 0.5 * s];
    let mut distance: f64 = 0.0;
    for c in 0..8u32 {
        let p = [
            (if c & 1 == 0 { lo[0] } else { hi[0] }) * s - target[0],
            (if c & 2 == 0 { lo[1] } else { hi[1] }) * s - target[1],
            (if c & 4 == 0 { lo[2] } else { hi[2] }) * s - target[2],
        ];
        let (x, y, z) = (dot(&p, &right), dot(&p, &up), dot(&p, &fwd));
        distance = distance.max(x.abs() / tx + z);
        distance = distance.max(y.abs() / ty + z);
    }
    let distance = distance * 1.05;
    let eye = [target[0] - fwd[0] * distance, target[1] - fwd[1] * distance, target[2] - fwd[2] * distance];
    Fit { eye, fwd, distance }
}

fn eye_at(f: &Fit, k: f64) -> [f64; 3] {
    let back = f.distance * (k - 1.0);
    [f.eye[0] - f.fwd[0] * back, f.eye[1] - f.fwd[1] * back, f.eye[2] - f.fwd[2] * back]
}

// Ray `o + t d` against the slab box, true when it can hit for some t in [0, 1].
fn hits_box(o: &[f64; 3], d: &[f64; 3], lo: &[f64; 3], hi: &[f64; 3]) -> bool {
    let mut t0: f64 = 0.0;
    let mut t1: f64 = 1.0;
    for k in 0..3 {
        if d[k].abs() < 1e-300 {
            if o[k] < lo[k] || o[k] > hi[k] {
                return false;
            }
            continue;
        }
        let a = (lo[k] - o[k]) / d[k];
        let b = (hi[k] - o[k]) / d[k];
        t0 = t0.max(a.min(b));
        t1 = t1.min(a.max(b));
    }
    t0 <= t1
}

// Moller-Trumbore: t of the hit of `o + t d` on the triangle, if any.
fn hit_tri(o: &[f64; 3], d: &[f64; 3], tri: &[[f64; 3]; 3]) -> Option<f64> {
    let e1 = sub(&tri[1], &tri[0]);
    let e2 = sub(&tri[2], &tri[0]);
    let p = cross(d, &e2);
    let det = dot(&e1, &p);
    if det.abs() < 1e-18 {
        return None;
    }
    let inv = 1.0 / det;
    let s = sub(o, &tri[0]);
    let u = dot(&s, &p) * inv;
    if !(-1e-9..=1.0 + 1e-9).contains(&u) {
        return None;
    }
    let q = cross(&s, &e1);
    let v = dot(d, &q) * inv;
    if v < -1e-9 || u + v > 1.0 + 1e-9 {
        return None;
    }
    Some(dot(&e2, &q) * inv)
}

// Every plate face between the eye (m) and the sample (mm) along the sample's view ray, as
// (t along the ray, plate) with t < 1: the sample's own face (t = 1) is not a cover.
fn covers(plates: &[Plate], eye: &[f64; 3], s: &[f64; 3]) -> Vec<(f64, usize)> {
    let o = [eye[0] * 1000.0, eye[1] * 1000.0, eye[2] * 1000.0];
    let d = sub(s, &o);
    let mut out = Vec::new();
    for (pi, p) in plates.iter().enumerate() {
        if !hits_box(&o, &d, &p.lo, &p.hi) {
            continue;
        }
        for tri in &p.tris {
            if let Some(t) = hit_tri(&o, &d, tri) && t > 1e-9 && t < 1.0 - 1e-7 {
                out.push((t, pi));
            }
        }
    }
    out
}

// The rule at one sample: the outline surfaces only if EVERY cover is pushed behind it, so
// the margin is the best cover's (separation - push) minus the outline's lift.
fn judge(o: &Outline, s: &[f64; 3], plates: &[Plate], eye: &[f64; 3], fwd: &[f64; 3], vp_h: f64) -> Verdict {
    let s_m = [s[0] * 0.001, s[1] * 0.001, s[2] * 0.001];
    // CENSUS_ORTHO_H=<half height mm>: parallel rays along `fwd` from a virtual eye 1 km back;
    // the shader's implied distance is ortho_h / tan(30 deg) and the lift is the ortho formula.
    let ortho_h = env_f64("CENSUS_ORTHO_H", 0.0);
    let eye_used = if ortho_h > 0.0 { [s_m[0] - fwd[0] * 1000.0, s_m[1] - fwd[1] * 1000.0, s_m[2] - fwd[2] * 1000.0] } else { *eye };
    let eye = &eye_used;
    let to_s = sub(&s_m, eye);
    let w = if ortho_h > 0.0 { ortho_h / 30.0_f64.to_radians().tan() * 0.001 } else { dot(&to_s, fwd) };
    let len_mm = norm(&to_s) * 1000.0;
    // mm per pixel at the sample, the host face's slope to the ray, the lift the ribbon needs.
    let mmpp = if ortho_h > 0.0 { 2.0 * ortho_h / vp_h } else { 2.0 * w * 30.0_f64.to_radians().tan() * 1000.0 / vp_h };
    let ray = if ortho_h > 0.0 { *fwd } else { let l = norm(&to_s); [to_s[0] / l, to_s[1] / l, to_s[2] / l] };
    // The shipped rule: free linework lifts LIFT_RADII_FREE pen HALF-WIDTHS toward the eye
    // (the same number in both projections), capped by a quarter of its thickness.
    let _ = ray;
    let lift = (PEN_PX * 0.5 * LIFT_RADII_FREE * 2.0 * mmpp).min(LIFT_MAX_THICK * o.t_rule);
    let mut best = Verdict { covered: false, w, sep: 0.0, push: 0.0, lift, margin: f64::INFINITY, plate: usize::MAX };
    for (t, pi) in covers(plates, eye, s) {
        let sep = (1.0 - t) * len_mm;
        let push = if ortho_h > 0.0 { push_mm(w, plates[pi].t_rule) } else { push_mm(w * t, plates[pi].t_rule) };
        let margin = sep - push - lift;
        if !best.covered || margin > best.margin {
            best = Verdict { covered: true, w, sep, push, lift, margin, plate: pi };
        }
    }
    best
}

fn placement(world: &HashMap<String, Xform>, guid: &str) -> Mat4 {
    match world.get(guid) {
        Some(x) => x.m,
        None => Xform::identity().m,
    }
}

// The scene box the harness fits: every drawn object's placed points.
fn scene_box(s: &Session, world: &HashMap<String, Xform>, plates: &[Plate], outlines: &[Outline]) -> ([f64; 3], [f64; 3]) {
    let mut lo = [f64::INFINITY; 3];
    let mut hi = [f64::NEG_INFINITY; 3];
    for p in plates {
        grow(&mut lo, &mut hi, &p.lo);
        grow(&mut lo, &mut hi, &p.hi);
    }
    for o in outlines {
        grow(&mut lo, &mut hi, &o.lo);
        grow(&mut lo, &mut hi, &o.hi);
    }
    for p in &s.objects.points {
        grow(&mut lo, &mut hi, &xform_point_f64(&placement(world, p.guid()), [p[0], p[1], p[2]]));
    }
    for l in &s.objects.lines {
        let m = placement(world, l.guid());
        grow(&mut lo, &mut hi, &xform_point_f64(&m, [l[0], l[1], l[2]]));
        grow(&mut lo, &mut hi, &xform_point_f64(&m, [l[3], l[4], l[5]]));
    }
    (lo, hi)
}

// The meshes as they are and every outline recoloured by the fit view: magenta when every
// sample is covered (any magenta pixel is ink through a face), blue when none is, cyan between.
fn recolor(s: &Session, outlines: &[Outline], plates: &[Plate], f0: &Fit, vp_h: f64, out: &str) {
    let mut s2 = Session::new("census");
    for m in &s.objects.meshes {
        s2.add_mesh(m.duplicate(), None);
    }
    let mut counts = [0usize; 3];
    for (i, o) in outlines.iter().enumerate() {
        let mut n = 0;
        for smp in &o.samples {
            if judge(o, smp, plates, &f0.eye, &f0.fwd, vp_h).covered {
                n += 1;
            }
        }
        let class = if n == o.samples.len() { 0 } else if n == 0 { 1 } else { 2 };
        counts[class] += 1;
        let mut p = s.objects.polylines[i].duplicate();
        p.linecolor = [Color::new(1.0, 0.0, 1.0, 1.0), Color::new(0.0, 0.0, 1.0, 1.0), Color::new(0.0, 1.0, 1.0, 1.0)][class].clone();
        s2.add_polyline(p, None);
    }
    s2.pb_dump(out);
    println!("recolored copy: {out}  magenta (fully covered at the fit view) {}  blue (visible) {}  cyan (partly covered) {}", counts[0], counts[1], counts[2]);
}

fn census(path: &str) {
    let vp_w = env_f64("VIEWER_W", 900.0);
    let vp_h = env_f64("VIEWER_H", 700.0);
    let bytes = std::fs::read(path).expect("read pb");
    let s = Session::pb_loads(&bytes).expect("parse pb");
    let world = s.world_xforms();
    println!("== {path}  ({:.2} MB, {} objects, {} xforms, {} meshes, {} polylines, {} points, {} lines)", bytes.len() as f64 / 1.048576e6, s.lookup.len(), s.xforms.len(), s.objects.meshes.len(), s.objects.polylines.len(), s.objects.points.len(), s.objects.lines.len());

    let mut plates: Vec<Plate> = Vec::new();
    for m in &s.objects.meshes {
        plates.push(plate_of(m, &placement(&world, m.guid())));
    }
    println!("meshes: i verts faces | extents sorted (thin mid long) diag | t_rule t_real ratio | big face |nz|");
    for (i, p) in plates.iter().enumerate() {
        println!("  m{i:<3} {:>4} {:>4} | {:8.2} {:8.2} {:8.2} {:8.2} | {:7.2} {:7.2} {:6.2} | {:.3}", p.verts.len(), p.faces.len(), p.ext[0], p.ext[1], p.ext[2], p.diag, p.t_rule, p.t_real, p.t_rule / p.t_real.max(1e-9), p.big_nz);
    }

    let mut outlines: Vec<Outline> = Vec::new();
    for pl in &s.objects.polylines {
        outlines.push(outline_of(pl, &placement(&world, pl.guid()), &plates));
    }
    let (lo, hi) = scene_box(&s, &world, &plates, &outlines);
    let mut f0 = fit(&lo, &hi, vp_w / vp_h);
    if let Ok(e) = std::env::var("CENSUS_EYE") {
        let v: Vec<f64> = e.split(',').filter_map(|t| t.trim().parse().ok()).collect();
        if v.len() == 3 {
            let centre = [(lo[0] + hi[0]) * 0.0005, (lo[1] + hi[1]) * 0.0005, (lo[2] + hi[2]) * 0.0005];
            let to = sub(&centre, &[v[0], v[1], v[2]]);
            let d = norm(&to);
            let mut fwd = [to[0] / d, to[1] / d, to[2] / d];
            if let Ok(f) = std::env::var("CENSUS_FWD") {
                let fv: Vec<f64> = f.split(',').filter_map(|t| t.trim().parse().ok()).collect();
                if fv.len() == 3 {
                    let n = (fv[0] * fv[0] + fv[1] * fv[1] + fv[2] * fv[2]).sqrt();
                    fwd = [fv[0] / n, fv[1] / n, fv[2] / n];
                }
            }
            f0 = Fit { eye: [v[0], v[1], v[2]], fwd, distance: d };
            println!("  CENSUS_EYE override: eye ({:.2}, {:.2}, {:.2}) m, distance {:.3} m", v[0], v[1], v[2], d);
        }
    }
    println!("polylines: i pts | extents sorted diag | t_rule | |newell nz| | nearest face plane: dist plate face, thickness normal to it | samples covered at k=1 4 16 | failing samples at k=1 4 16 | min margin mm at k=1 4 16");
    let mut per_k: Vec<Vec<Verdict>> = Vec::new();
    for _ in SCALES {
        per_k.push(Vec::new());
    }
    for (i, o) in outlines.iter().enumerate() {
        let on = o.dist <= ON_FACE_TOL;
        let behind = if on { plates[o.plate].faces[o.face].behind } else { f64::NAN };
        let mut cov = String::new();
        let mut fail = String::new();
        let mut mins = String::new();
        let mut worst1: Option<(Verdict, [f64; 3])> = None;
        for (ki, k) in SCALES.iter().enumerate() {
            let eye = eye_at(&f0, *k);
            let mut n_cov = 0;
            let mut n_fail = 0;
            let mut min_margin = f64::INFINITY;
            for smp in &o.samples {
                let j = judge(o, smp, &plates, &eye, &f0.fwd, vp_h);
                if j.covered {
                    n_cov += 1;
                    min_margin = min_margin.min(j.margin);
                    if j.margin < 0.0 {
                        n_fail += 1;
                        if ki == 0 && worst1.as_ref().is_none_or(|(w, _)| j.margin < w.margin) {
                            worst1 = Some((j.clone(), *smp));
                        }
                    }
                }
                per_k[ki].push(j);
            }
            cov.push_str(&format!(" {n_cov:>2}"));
            fail.push_str(&format!(" {n_fail:>2}"));
            mins.push_str(&format!(" {:8.2}", if min_margin.is_finite() { min_margin } else { f64::NAN }));
        }
        println!("  p{i:<3} {:>2} | {:8.2} {:8.2} {:8.2} {:8.2} | {:6.2} | {:.3} | {:7.4} m{:<3} f{:<3} {:7.2} |{cov} of {:>2} |{fail} |{mins}", o.pts.len(), o.ext[0], o.ext[1], o.ext[2], o.diag, o.t_rule, o.nz, o.dist, o.plate, o.face, behind, o.samples.len());
        if let Some((w, smp)) = &worst1 {
            println!("    WORST k=1 p{i}: sample ({:.0}, {:.0}, {:.0}) mm on m{} f{}  cover m{} (t_real {:.2}) push {:.2} lift {:.2} sep {:.2} margin {:.2}", smp[0], smp[1], smp[2], o.plate, o.face, w.plate, plates[w.plate].t_real, w.push, w.lift, w.sep, w.margin);
        }
    }

    println!("plates: {}", plates.len());
    let mut v = Vec::new();
    for p in &plates {
        v.push(p.ext[0]);
    }
    stats("thickness mm (thinnest AABB axis = what the rule sees)", &mut v);
    v.clear();
    for p in &plates {
        v.push(p.t_rule);
    }
    stats("thickness mm (rule: max(thinnest, 0.001 x diag))", &mut v);
    v.clear();
    for p in &plates {
        v.push(p.t_real);
    }
    stats("thickness mm (min extent along any face normal = real)", &mut v);
    v.clear();
    for p in &plates {
        v.push(p.t_rule / p.t_real.max(1e-9));
    }
    stats("t_rule / t_real (rule overestimate on rotated plates)", &mut v);
    v.clear();
    for p in &plates {
        v.push(p.ext[2]);
    }
    stats("length mm (longest AABB axis)", &mut v);
    v.clear();
    for p in &plates {
        v.push(p.diag);
    }
    stats("diagonal mm", &mut v);
    v.clear();
    for p in &plates {
        v.push(p.diag / p.ext[0].max(1e-9));
    }
    stats("diagonal / thickness(AABB)", &mut v);
    v.clear();
    for p in &plates {
        v.push(p.diag / p.t_real.max(1e-9));
    }
    stats("diagonal / thickness(real)", &mut v);
    let mut flat = 0;
    let mut rotated = 0;
    let mut tris = 0;
    for p in &plates {
        if p.ext[0] < ON_FACE_TOL {
            flat += 1;
        }
        if p.t_rule > p.t_real * 1.01 {
            rotated += 1;
        }
        tris += p.tris.len();
    }
    println!("  flat meshes (thinnest axis < {ON_FACE_TOL} mm): {flat}   plates whose t_rule exceeds t_real by >1%: {rotated}   triangles ray-cast: {tris}");

    println!("polylines: {}", outlines.len());
    v.clear();
    for o in &outlines {
        v.push(o.dist);
    }
    stats("distance to nearest mesh face plane mm (max point deviation)", &mut v);
    v.clear();
    for o in &outlines {
        v.push(o.ext[0]);
    }
    stats("outline thinnest AABB axis mm (its lift cap is 0.25 x max(this, 0.001 x diag))", &mut v);
    v.clear();
    for o in &outlines {
        v.push(o.t_rule);
    }
    stats("outline t_rule mm", &mut v);
    let mut on = 0;
    v.clear();
    for o in &outlines {
        if o.dist <= ON_FACE_TOL {
            on += 1;
            v.push(plates[o.plate].faces[o.face].behind);
        }
    }
    println!("  on a mesh face (<= {ON_FACE_TOL} mm): {on} of {}", outlines.len());
    stats("that plate's thickness normal to the face mm", &mut v);

    println!("scene AABB mm: min [{:.1}, {:.1}, {:.1}] max [{:.1}, {:.1}, {:.1}]  extent {:.1} x {:.1} x {:.1}  diagonal {:.1}", lo[0], lo[1], lo[2], hi[0], hi[1], hi[2], hi[0] - lo[0], hi[1] - lo[1], hi[2] - lo[2], norm(&sub(&hi, &lo)));
    let proj_y = 1.0 / 30.0_f64.to_radians().tan() * 0.001;
    for (w, h) in [(vp_w, vp_h), (1280.0, 720.0), (1920.0, 1080.0)] {
        let f = fit(&lo, &hi, w / h);
        let px_per_mm = proj_y * h / (2.0 * f.distance);
        println!("  fit {w:.0}x{h:.0}: distance {:.3} m  eye ({:.2}, {:.2}, {:.2}) m  fwd ({:.3}, {:.3}, {:.3})  at target {:.4} px/mm, 1 px = {:.2} mm, uncapped free lift (0.5 px) = {:.2} mm", f.distance, f.eye[0], f.eye[1], f.eye[2], f.fwd[0], f.fwd[1], f.fwd[2], px_per_mm, 1.0 / px_per_mm, 0.5 / px_per_mm * LIFT_RADII_FREE);
    }

    println!("rule at k x fit distance ({vp_w:.0}x{vp_h:.0}), covered outline samples: margin = best cover's (separation along the ray - its face push) - the outline's lift");
    for (ki, k) in SCALES.iter().enumerate() {
        let mut n_cov = 0;
        let mut n_fail = 0;
        let mut seps = Vec::new();
        let mut pushes = Vec::new();
        let mut lifts = Vec::new();
        let mut margins = Vec::new();
        let mut fail_outlines: Vec<usize> = Vec::new();
        let mut cov_outlines = 0;
        let mut grazing = 0;
        let mut at = 0;
        for (i, o) in outlines.iter().enumerate() {
            let mut any_cov = false;
            let mut any_fail = false;
            for _ in &o.samples {
                let j = &per_k[ki][at];
                at += 1;
                if !j.covered {
                    continue;
                }
                any_cov = true;
                n_cov += 1;
                seps.push(j.sep);
                pushes.push(j.push);
                lifts.push(j.lift);
                margins.push(j.margin);
                if j.margin < 0.0 {
                    any_fail = true;
                    n_fail += 1;
                    if j.sep < plates[j.plate].t_real {
                        grazing += 1;
                    }
                }
            }
            if any_cov {
                cov_outlines += 1;
            }
            if any_fail {
                fail_outlines.push(i);
            }
        }
        println!(" k={k:<3} distance {:.2} m  0.4% of it = {:.1} mm  samples {} covered {n_cov} FAIL {n_fail} (of which {grazing} graze a cover's edge: separation < that plate's t_real)  outlines covered {cov_outlines} with a FAIL {}: {fail_outlines:?}", f0.distance * k, PUSH_FRAC * f0.distance * k * 1000.0, per_k[ki].len(), fail_outlines.len());
        stats("separation along the ray mm", &mut seps);
        stats("cover's face push mm", &mut pushes);
        stats("outline lift mm", &mut lifts);
        stats("margin mm", &mut margins);
    }

    let mut order: Vec<(f64, usize)> = Vec::new();
    for (i, p) in plates.iter().enumerate() {
        let mut has_outline = false;
        for o in &outlines {
            has_outline = has_outline || (o.plate == i && o.dist <= ON_FACE_TOL);
        }
        if has_outline && p.t_real > ON_FACE_TOL {
            order.push((p.t_real, i));
        }
    }
    order.sort_by(by_first);
    if order.is_empty() {
        return;
    }
    let picks = [("thinnest outlined plate", order[0].1), ("median outlined plate", order[(order.len() - 1) / 2].1), ("thickest outlined plate", order[order.len() - 1].1)];
    for (label, pi) in picks {
        let p = &plates[pi];
        println!("{label}: m{pi}  extents {:.2} x {:.2} x {:.2}  diag {:.2}  t_rule {:.2} (push cap {:.2})  t_real {:.2}  big face |nz| {:.3}", p.ext[0], p.ext[1], p.ext[2], p.diag, p.t_rule, PUSH_MAX_THICK * p.t_rule, p.t_real, p.big_nz);
        for (oi, o) in outlines.iter().enumerate() {
            if o.plate != pi || o.dist > ON_FACE_TOL {
                continue;
            }
            for k in SCALES {
                let eye = eye_at(&f0, k);
                let mut worst = Verdict { covered: false, w: 0.0, sep: 0.0, push: 0.0, lift: 0.0, margin: f64::INFINITY, plate: usize::MAX };
                let mut n_cov = 0;
                for smp in &o.samples {
                    let j = judge(o, smp, &plates, &eye, &f0.fwd, vp_h);
                    if j.covered {
                        n_cov += 1;
                        if j.margin < worst.margin {
                            worst = j;
                        }
                    }
                }
                if !worst.covered {
                    println!("  p{oi:<3} k={k:<3} no sample covered: the outline is in view");
                    continue;
                }
                let fail = if worst.margin < 0.0 { "FAIL" } else { "" };
                println!("  p{oi:<3} k={k:<3} covered {n_cov}/{}  worst sample: eye depth {:6.2} m  0.4% = {:6.1} mm  cover m{} (t_rule {:.2}, t_real {:.2}) push {:6.2} mm  lift {:5.2} mm (cap {:.2})  separation {:7.2} mm  margin {:8.2} mm {fail}", o.samples.len(), worst.w, PUSH_FRAC * worst.w * 1000.0, worst.plate, plates[worst.plate].t_rule, plates[worst.plate].t_real, worst.push, worst.lift, LIFT_MAX_THICK * o.t_rule, worst.sep, worst.margin);
            }
        }
    }
    if let Ok(out) = std::env::var("CENSUS_RECOLOR") {
        recolor(&s, &outlines, &plates, &f0, vp_h, &out);
    }
}

fn main() {
    for path in std::env::args().skip(1) {
        census(&path);
    }
}
