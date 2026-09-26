//! Plate census: measure plates and outlines, judge which samples a plate covers.

use session_rust::{Color, Mesh, Point, Polyline, Quaternion, Session, Vector, Xform};
use std::cmp::Ordering;
use std::collections::HashMap;

// the old offset model, kept for the BEFORE estimates
const THICK_FLOOR: f64 = 0.001; // thickness floor, m
const PUSH_FRAC: f64 = 0.004; // push as a fraction of thickness
const PUSH_MAX_THICK: f64 = 0.25; // push cap as a fraction of thickness
const LIFT_RADII_FREE: f64 = 1.0; // lift of free linework, pen half-widths
const LIFT_MAX_THICK: f64 = 0.25; // lift cap as a fraction of thickness
const PEN_PX: f64 = 2.0; // pen width, pixels
const ON_FACE_TOL: f64 = 0.01; // on-face distance tolerance, mm
const SAMPLE_MM: f64 = 50.0; // outline sample spacing, mm
const SCALES: [f64; 3] = [1.0, 4.0, 16.0]; // fit distances to judge at

/// One plate face plane.
struct Face {
    n: [f64; 3],
    d: f64,
    behind: f64,
}

/// One mesh as a plate: faces, triangles, box, thickness.
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

/// One polyline outline and its samples.
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

/// The fit camera.
struct Fit {
    eye: [f64; 3],
    fwd: [f64; 3],
    distance: f64,
}

/// The rule's answer for one outline sample.
#[derive(Clone)]
struct Verdict {
    covered: bool, // a plate in front covers it
    w: f64,        // eye depth, m
    sep: f64,      // cover separation along the ray
    push: f64,     // the cover's push
    lift: f64,     // the sample's lift
    margin: f64,   // separation - push - lift, mm
    plate: usize,  // the binding plate
}

/// a - b.
fn sub(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// Dot product.
fn dot(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Cross product.
fn cross(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Length.
fn norm(a: &[f64; 3]) -> f64 {
    dot(a, a).sqrt()
}

/// Unit vector.
fn unit(a: &[f64; 3]) -> [f64; 3] {
    let l = norm(a).max(1e-300);
    [a[0] / l, a[1] / l, a[2] / l]
}

/// Order by the first element.
fn by_first(a: &(f64, usize), b: &(f64, usize)) -> Ordering {
    a.0.total_cmp(&b.0)
}

/// An f64 from the environment, or `default`.
fn env_f64(name: &str, default: f64) -> f64 {
    match std::env::var(name) {
        Ok(v) => v.trim().parse().unwrap_or(default),
        Err(_) => default,
    }
}

/// A placed point as an array.
fn placed(place: &Xform, p: [f64; 3]) -> [f64; 3] {
    let q = place.transform_point(&Point::new(p[0], p[1], p[2]));
    [q[0], q[1], q[2]]
}

/// Grow the box to include `p`.
fn grow(lo: &mut [f64; 3], hi: &mut [f64; 3], p: &[f64; 3]) {
    for k in 0..3 {
        lo[k] = lo[k].min(p[k]);
        hi[k] = hi[k].max(p[k]);
    }
}

/// The box of the points.
fn box_of(pts: &[[f64; 3]]) -> ([f64; 3], [f64; 3]) {
    let mut lo = [f64::INFINITY; 3];
    let mut hi = [f64::NEG_INFINITY; 3];
    for p in pts {
        grow(&mut lo, &mut hi, p);
    }
    (lo, hi)
}

/// Box extents, smallest first.
fn sorted_extents(lo: &[f64; 3], hi: &[f64; 3]) -> [f64; 3] {
    let mut e = [hi[0] - lo[0], hi[1] - lo[1], hi[2] - lo[2]];
    e.sort_by(f64::total_cmp);
    e
}

/// Mean of the points.
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

/// The push at eye depth `w_m`, mm.
fn push_mm(w_m: f64, t_rule: f64) -> f64 {
    (PUSH_FRAC * w_m * 1000.0).min(PUSH_MAX_THICK * t_rule)
}

#[allow(dead_code)]
/// The lift of free linework, mm.
fn lift_free_mm(w_m: f64, t_rule: f64, vp_h: f64) -> f64 {
    let proj_y = 1.0 / 30.0_f64.to_radians().tan() * 0.001;
    let raw_px = (PEN_PX * 0.5).max(0.5);
    let uncapped = raw_px * LIFT_RADII_FREE * w_m / (proj_y * vp_h);
    uncapped.min(LIFT_MAX_THICK * t_rule)
}

/// Print min, median, max of `v`.
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
    println!(
        "  {label}: n={n} min {:.2} p10 {p10:.2} median {med:.2} p90 {p90:.2} max {:.2}",
        v[0],
        v[n - 1]
    );
}

/// The plate's triangles, using the mesh's own triangulation.
fn face_triangles(mesh: &Mesh, face: usize) -> Vec<[usize; 3]> {
    let mut triangles = Vec::new();
    if let Some(cached) = mesh.triangulation.get(&face)
        && !cached.is_empty()
    {
        triangles.extend_from_slice(cached);
    } else if let Some(vertices) = mesh.face_vertices(face) {
        for i in 2..vertices.len() {
            triangles.push([vertices[0], vertices[i - 1], vertices[i]]);
        }
    }
    triangles.retain(|triangle| triangle.iter().all(|key| mesh.vertex.contains_key(key)));
    triangles
}

/// A mesh under `place` as a plate.
fn plate_of(m: &Mesh, place: &Xform) -> Plate {
    let mut local: Vec<[f64; 3]> = Vec::with_capacity(m.vertex.len());
    for key in m.vertices() {
        let v = &m.vertex[&key];
        local.push([v.x, v.y, v.z]);
    }
    let mut verts: Vec<[f64; 3]> = Vec::with_capacity(local.len());
    for p in &local {
        verts.push(placed(place, *p));
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
        for triangle in face_triangles(m, fk) {
            tris.push(triangle.map(|key| {
                let point = &m.vertex[&key];
                placed(place, [point.x, point.y, point.z])
            }));
        }
        let Some(fpts) = m.face_points(fk) else {
            continue;
        };
        let mut pts: Vec<[f64; 3]> = Vec::with_capacity(fpts.len());
        for p in &fpts {
            pts.push(placed(place, [p[0], p[1], p[2]]));
        }
        if pts.len() < 3 {
            continue;
        }
        // cross product after placement gives the placed normal
        let mut n = unit(&cross(&sub(&pts[1], &pts[0]), &sub(&pts[2], &pts[0])));
        if norm(&n) == 0.0 {
            continue;
        }
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
    Plate {
        verts,
        faces,
        tris,
        lo,
        hi,
        ext,
        diag,
        t_rule,
        t_real,
        big_nz,
    }
}

/// z of the Newell normal of the polygon.
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

/// An outline: its samples and the nearest plate face.
fn outline_of(pl: &Polyline, place: &Xform, plates: &[Plate]) -> Outline {
    let mut local: Vec<[f64; 3]> = Vec::with_capacity(pl.coords.len() / 3);
    for c in pl.coords.chunks_exact(3) {
        local.push([c[0], c[1], c[2]]);
    }
    let scale = place.uniform_scale();
    let mut pts: Vec<[f64; 3]> = Vec::with_capacity(local.len());
    for p in &local {
        pts.push(placed(place, *p));
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
            samples.push([
                w[0][0] + (w[1][0] - w[0][0]) * t,
                w[0][1] + (w[1][1] - w[0][1]) * t,
                w[0][2] + (w[1][2] - w[0][2]) * t,
            ]);
        }
    }
    if let Some(last) = pts.last()
        && pts.first() != Some(last)
    {
        samples.push(*last);
    }
    let nz = newell_nz(&pts);
    let hosted = best.0 <= ON_FACE_TOL;
    let t_rule = if hosted {
        plates[best.1].t_real.max(THICK_FLOOR * plates[best.1].diag)
    } else {
        t_rule
    };
    Outline {
        pts,
        samples,
        lo,
        hi,
        ext,
        diag,
        t_rule,
        nz,
        dist: best.0,
        plate: best.1,
        face: best.2,
    }
}

/// Right, up and forward of the iso view.
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

/// The fit camera, as Camera::fit at the iso view.
fn fit(lo: &[f64; 3], hi: &[f64; 3], aspect: f64) -> Fit {
    let (fwd, up, right) = iso_frame();
    let ty = 30.0_f64.to_radians().tan();
    let tx = aspect * ty;
    let s = 0.001;
    let target = [
        (lo[0] + hi[0]) * 0.5 * s,
        (lo[1] + hi[1]) * 0.5 * s,
        (lo[2] + hi[2]) * 0.5 * s,
    ];
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
    let eye = [
        target[0] - fwd[0] * distance,
        target[1] - fwd[1] * distance,
        target[2] - fwd[2] * distance,
    ];
    Fit { eye, fwd, distance }
}

/// The eye at `k` times the fit distance.
fn eye_at(f: &Fit, k: f64) -> [f64; 3] {
    let back = f.distance * (k - 1.0);
    [
        f.eye[0] - f.fwd[0] * back,
        f.eye[1] - f.fwd[1] * back,
        f.eye[2] - f.fwd[2] * back,
    ]
}

/// Whether ray `o + t d` can hit the box for t in [0, 1].
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

/// The parallel ray from the ink toward the eye.
fn hits_forward_box(o: &[f64; 3], d: &[f64; 3], lo: &[f64; 3], hi: &[f64; 3]) -> bool {
    let mut t0: f64 = 0.0;
    let mut t1: f64 = f64::INFINITY;
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

/// The t where `o + t d` hits the triangle, if any.
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

/// Every plate between the eye and the sample, as (t, plate).
fn covers(plates: &[Plate], eye: &[f64; 3], s: &[f64; 3]) -> Vec<(f64, usize)> {
    let o = [eye[0] * 1000.0, eye[1] * 1000.0, eye[2] * 1000.0];
    let d = sub(s, &o);
    let mut out = Vec::new();
    for (pi, p) in plates.iter().enumerate() {
        if !hits_box(&o, &d, &p.lo, &p.hi) {
            continue;
        }
        for tri in &p.tris {
            if let Some(t) = hit_tri(&o, &d, tri)
                && t > 1e-9
                && t < 1.0 - 1e-7
            {
                out.push((t, pi));
            }
        }
    }
    out
}

/// Judge one sample: visible only if every cover is pushed behind it.
fn judge(
    o: &Outline,
    s: &[f64; 3],
    plates: &[Plate],
    eye: &[f64; 3],
    fwd: &[f64; 3],
    vp_h: f64,
) -> Verdict {
    let s_m = [s[0] * 0.001, s[1] * 0.001, s[2] * 0.001];
    // CENSUS_ORTHO_H: parallel rays from a far virtual eye
    let ortho_h = env_f64("CENSUS_ORTHO_H", 0.0);
    let eye_used = if ortho_h > 0.0 {
        [
            s_m[0] - fwd[0] * 1000.0,
            s_m[1] - fwd[1] * 1000.0,
            s_m[2] - fwd[2] * 1000.0,
        ]
    } else {
        *eye
    };
    let eye = &eye_used;
    let to_s = sub(&s_m, eye);
    let w = if ortho_h > 0.0 {
        ortho_h / 30.0_f64.to_radians().tan() * 0.001
    } else {
        dot(&to_s, fwd)
    };
    let len_mm = norm(&to_s) * 1000.0;
    // pixel size at the sample, face slope, ribbon lift
    let mmpp = if ortho_h > 0.0 {
        2.0 * ortho_h / vp_h
    } else {
        2.0 * w * 30.0_f64.to_radians().tan() * 1000.0 / vp_h
    };
    let ray = if ortho_h > 0.0 {
        *fwd
    } else {
        let l = norm(&to_s);
        [to_s[0] / l, to_s[1] / l, to_s[2] / l]
    };
    // free linework lifts by pen half-widths, capped by thickness
    let _ = ray;
    let lift = (PEN_PX * 0.5 * LIFT_RADII_FREE * 2.0 * mmpp).min(LIFT_MAX_THICK * o.t_rule);
    let mut best = Verdict {
        covered: false,
        w,
        sep: 0.0,
        push: 0.0,
        lift,
        margin: f64::INFINITY,
        plate: usize::MAX,
    };
    for (t, pi) in covers(plates, eye, s) {
        let sep = (1.0 - t) * len_mm;
        let push = if ortho_h > 0.0 {
            push_mm(w, plates[pi].t_rule)
        } else {
            push_mm(w * t, plates[pi].t_rule)
        };
        let margin = sep - push - lift;
        if !best.covered || margin > best.margin {
            best = Verdict {
                covered: true,
                w,
                sep,
                push,
                lift,
                margin,
                plate: pi,
            };
        }
    }
    best
}

/// The placement of `guid`, identity when none.
fn placement(world: &HashMap<String, Xform>, guid: &str) -> Xform {
    match world.get(guid) {
        Some(x) => x.clone(),
        None => Xform::identity(),
    }
}

/// The box of every placed point, as the harness fits it.
fn scene_box(
    s: &Session,
    world: &HashMap<String, Xform>,
    plates: &[Plate],
    outlines: &[Outline],
) -> ([f64; 3], [f64; 3]) {
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
        grow(
            &mut lo,
            &mut hi,
            &placed(&placement(world, p.guid()), [p[0], p[1], p[2]]),
        );
    }
    for l in &s.objects.lines {
        let m = placement(world, l.guid());
        grow(&mut lo, &mut hi, &placed(&m, [l[0], l[1], l[2]]));
        grow(&mut lo, &mut hi, &placed(&m, [l[3], l[4], l[5]]));
    }
    (lo, hi)
}

/// Write a copy with outlines magenta when covered, blue when visible, cyan when partly.
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
        let class = if n == o.samples.len() {
            0
        } else if n == 0 {
            1
        } else {
            2
        };
        counts[class] += 1;
        let mut p = s.objects.polylines[i].duplicate();
        p.linecolor = [
            Color::new(1.0, 0.0, 1.0, 1.0),
            Color::new(0.0, 0.0, 1.0, 1.0),
            Color::new(0.0, 1.0, 1.0, 1.0),
        ][class]
            .clone();
        s2.add_polyline(p, None);
    }
    s2.pb_dump(out);
    println!(
        "recolored copy: {out}  magenta (fully covered at the fit view) {}  blue (visible) {}  cyan (partly covered) {}",
        counts[0], counts[1], counts[2]
    );
}

/// An edge and the spans of it that triangles cover.
struct ProbeEdge {
    ends: [[f64; 3]; 2],
}

/// The eye point or the parallel view direction, mm.
struct ProbeView {
    eye: [f64; 3],
    parallel: Option<[f64; 3]>,
}

/// One span fixture: its camera and output path.
struct RecolorSettings<'a> {
    fit: &'a Fit,
    out: &'a str,
}

/// A rendered frame and its camera.
struct SampleImage<'a> {
    pixels: Vec<u8>,
    width: usize,
    height: usize,
    right: [f64; 3],
    up: [f64; 3],
    fit: &'a Fit,
    ortho_h: f64,
}

/// One object's id and its ribbon range.
#[derive(serde::Deserialize)]
/// One object id and its ribbon range.
struct RenderedObject {
    object_id: u32,
    ribbon_start: u32,
    ribbon_count: usize,
}

impl<'a> SampleImage<'a> {
    /// Whether a plate triangle covers the point.
    fn covered(&self, point: &[f64; 3], plates: &[Plate]) -> bool {
        let view = ProbeView {
            eye: self.fit.eye.map(|value| value * 1000.0),
            parallel: (self.ortho_h > 0.0).then_some(self.fit.fwd),
        };
        let edge = ProbeEdge {
            ends: [*point, *point],
        };
        plates.iter().any(|plate| {
            let hits = if let Some(direction) = view.parallel {
                hits_forward_box(point, &direction.map(|value| -value), &plate.lo, &plate.hi)
            } else {
                hits_box(&view.eye, &sub(point, &view.eye), &plate.lo, &plate.hi)
            };
            hits && plate
                .tris
                .iter()
                .any(|triangle| triangle_interval(&edge, triangle, &view).is_some())
        })
    }

    /// Read a PPM frame and its camera basis.
    fn read(path: &str, fit: &'a Fit) -> Self {
        let bytes = std::fs::read(path).expect("read CENSUS_RENDERED_SPANS frame");
        let mut end = 0;
        let mut lines = 0;
        for (i, byte) in bytes.iter().enumerate() {
            if *byte == b'\n' {
                lines += 1;
                if lines == 3 {
                    end = i + 1;
                    break;
                }
            }
        }
        let header: Vec<&str> = std::str::from_utf8(&bytes[..end])
            .expect("PPM header")
            .split_whitespace()
            .collect();
        assert_eq!(header.len(), 4, "expected harness PPM header");
        assert_eq!(header[0], "P6");
        assert_eq!(header[3], "255");
        let width: usize = header[1].parse().expect("PPM width");
        let height: usize = header[2].parse().expect("PPM height");
        assert_eq!(bytes.len() - end, width * height * 3, "truncated PPM");
        let mut image = Self::camera(width, height, fit);
        image.pixels = bytes[end..].to_vec();
        image
    }

    /// The camera basis of the capture.
    fn camera(width: usize, height: usize, fit: &'a Fit) -> Self {
        let mut right = cross(&fit.fwd, &[0.0, 0.0, 1.0]);
        if norm(&right) < 1e-8 {
            right = [1.0, 0.0, 0.0];
        }
        if let Ok(value) = std::env::var("CENSUS_UP") {
            let up: Vec<f64> = value
                .split(',')
                .map(str::parse)
                .collect::<Result<_, _>>()
                .expect("CENSUS_UP coordinates");
            assert_eq!(up.len(), 3, "CENSUS_UP requires x,y,z");
            right = cross(&fit.fwd, &[up[0], up[1], up[2]]);
        }
        right = unit(&right);
        let up = unit(&cross(&right, &fit.fwd));
        Self {
            pixels: Vec::new(),
            width,
            height,
            right,
            up,
            fit,
            ortho_h: env_f64("CENSUS_ORTHO_H", 0.0),
        }
    }

    /// Project a point to the pixel and its eye depth.
    fn projected(&self, point: &[f64; 3]) -> ([f64; 2], f64) {
        let eye = [
            self.fit.eye[0] * 1000.0,
            self.fit.eye[1] * 1000.0,
            self.fit.eye[2] * 1000.0,
        ];
        let delta = sub(point, &eye);
        let depth = dot(&delta, &self.fit.fwd);
        let half_height = if self.ortho_h > 0.0 {
            self.ortho_h
        } else {
            depth * 30.0f64.to_radians().tan()
        };
        let mmpp = 2.0 * half_height / self.height as f64;
        (
            [
                self.width as f64 * 0.5 + dot(&delta, &self.right) / mmpp,
                self.height as f64 * 0.5 - dot(&delta, &self.up) / mmpp,
            ],
            depth,
        )
    }

    /// The edge point a fragment represents.
    fn axis_at(&self, edge: &ProbeEdge, pixel: [f64; 2]) -> [f64; 3] {
        let (a, depth_a) = self.projected(&edge.ends[0]);
        let (b, depth_b) = self.projected(&edge.ends[1]);
        let d = [b[0] - a[0], b[1] - a[1]];
        let length2 = d[0] * d[0] + d[1] * d[1];
        let h = (((pixel[0] - a[0]) * d[0] + (pixel[1] - a[1]) * d[1]) / length2.max(1e-6))
            .clamp(0.0, 1.0);
        let t = if self.ortho_h > 0.0 {
            h
        } else {
            h * depth_a / ((1.0 - h) * depth_b + h * depth_a)
        };
        let point = edge_point(edge, t);
        [point[0], point[1], point[2]]
    }

    /// The point on a pixel ray at the edge's depth.
    fn fragment_point(&self, edge: &ProbeEdge, pixel: [f64; 2]) -> [f64; 3] {
        let axis = self.axis_at(edge, pixel);
        let (at, depth) = self.projected(&axis);
        let half_height = if self.ortho_h > 0.0 {
            self.ortho_h
        } else {
            depth * 30.0f64.to_radians().tan()
        };
        let mmpp = 2.0 * half_height / self.height as f64;
        let dx = (pixel[0] - at[0]) * mmpp;
        let dy = -(pixel[1] - at[1]) * mmpp;
        [
            axis[0] + self.right[0] * dx + self.up[0] * dy,
            axis[1] + self.right[1] * dx + self.up[1] * dy,
            axis[2] + self.right[2] * dx + self.up[2] * dy,
        ]
    }

    /// Whether one triangle covers the axis and all pixel corners.
    fn full_pixel_cover(
        &self,
        edge: &ProbeEdge,
        pixel: usize,
        plates: &[Plate],
    ) -> Option<(usize, usize)> {
        let x = (pixel % self.width) as f64;
        let y = (pixel / self.width) as f64;
        let points = [
            self.axis_at(edge, [x + 0.5, y + 0.5]),
            self.fragment_point(edge, [x, y]),
            self.fragment_point(edge, [x + 1.0, y]),
            self.fragment_point(edge, [x, y + 1.0]),
            self.fragment_point(edge, [x + 1.0, y + 1.0]),
        ];
        let view = ProbeView {
            eye: [
                self.fit.eye[0] * 1000.0,
                self.fit.eye[1] * 1000.0,
                self.fit.eye[2] * 1000.0,
            ],
            parallel: if self.ortho_h > 0.0 {
                Some(self.fit.fwd)
            } else {
                None
            },
        };
        for (index, plate) in plates.iter().enumerate() {
            for (triangle_index, triangle) in plate.tris.iter().enumerate() {
                let mut covered = true;
                for point in &points {
                    if triangle_interval(
                        &ProbeEdge {
                            ends: [*point, *point],
                        },
                        triangle,
                        &view,
                    )
                    .is_none()
                    {
                        covered = false;
                        break;
                    }
                }
                if covered {
                    return Some((index, triangle_index));
                }
            }
        }
        None
    }

    /// The pixel centre at the sample's depth.
    fn sample_pixel(&self, point: &[f64; 3]) -> Option<(usize, [f64; 3])> {
        let eye = [
            self.fit.eye[0] * 1000.0,
            self.fit.eye[1] * 1000.0,
            self.fit.eye[2] * 1000.0,
        ];
        let delta = sub(point, &eye);
        let depth = dot(&delta, &self.fit.fwd);
        if self.ortho_h == 0.0 && depth <= 0.0 {
            return None;
        }
        let half_height = if self.ortho_h > 0.0 {
            self.ortho_h
        } else {
            depth * 30.0f64.to_radians().tan()
        };
        let mmpp = 2.0 * half_height / self.height as f64;
        let x = self.width as f64 * 0.5 + dot(&delta, &self.right) / mmpp;
        let y = self.height as f64 * 0.5 - dot(&delta, &self.up) / mmpp;
        if x < 0.0 || y < 0.0 || x >= self.width as f64 || y >= self.height as f64 {
            return None;
        }
        let px = x.floor() as usize;
        let py = y.floor() as usize;
        let dx = (px as f64 + 0.5 - x) * mmpp;
        let dy = -(py as f64 + 0.5 - y) * mmpp;
        let centre = [
            point[0] + self.right[0] * dx + self.up[0] * dy,
            point[1] + self.right[1] * dx + self.up[1] * dy,
            point[2] + self.right[2] * dx + self.up[2] * dy,
        ];
        Some((py * self.width + px, centre))
    }
}

/// Map rendered pixels back to source outlines through the id pass.
fn rendered_ids(s: &Session, outlines: &[Outline], plates: &[Plate], fit: &Fit) {
    let path = std::env::var("CENSUS_RENDERED_IDS").expect("ID frame path");
    let bytes = std::fs::read(&path).expect("read ID frame");
    assert!(bytes.len() >= 12, "truncated ID header");
    assert_eq!(
        &bytes[..4],
        b"HLI2",
        "expected versioned object/segment ID frame"
    );
    let width = u32::from_le_bytes(bytes[4..8].try_into().expect("ID width")) as usize;
    let height = u32::from_le_bytes(bytes[8..12].try_into().expect("ID height")) as usize;
    assert_eq!(bytes.len(), 12 + width * height * 8, "truncated ID frame");
    let mapping: HashMap<String, RenderedObject> = serde_json::from_slice(
        &std::fs::read(format!("{path}.json")).expect("read ID GUID mapping"),
    )
    .expect("parse ID GUID mapping");
    let image = SampleImage::camera(width, height, fit);
    let mut covered = 0;
    let mut surfaced = 0;
    let mut other_legs = 0;
    let mut raw_matches = 0;
    let mut visible_axis = 0;
    let mut partial_pixels = 0;
    let mut locations = Vec::new();
    for (oi, outline) in outlines.iter().enumerate() {
        let expected = mapping
            .get(s.objects.polylines[oi].guid())
            .expect("outline GUID absent from rendered frame");
        assert_eq!(
            expected.ribbon_count,
            outline.pts.len().saturating_sub(1),
            "source span count differs from actual uploaded range"
        );
        for (si, point) in outline.samples.iter().enumerate() {
            if !image.covered(point, plates) {
                continue;
            }
            let Some((pixel, centre)) = image.sample_pixel(point) else {
                continue;
            };
            if !image.covered(&centre, plates) {
                continue;
            }
            covered += 1;
            let offset = 12 + pixel * 8;
            let actual =
                u32::from_le_bytes(bytes[offset..offset + 4].try_into().expect("pixel ID"));
            let sub = u32::from_le_bytes(
                bytes[offset + 4..offset + 8]
                    .try_into()
                    .expect("segment ID"),
            );
            if actual == expected.object_id {
                assert_ne!(sub & 0x8000_0000, 0, "outline pixel lacks segment identity");
                let global = (sub & 0x7fff_ffff)
                    .checked_sub(1)
                    .expect("tagged zero segment");
                let local = global
                    .checked_sub(expected.ribbon_start)
                    .expect("segment before object range") as usize;
                assert!(
                    local < expected.ribbon_count,
                    "segment outside object range"
                );
                let edge = ProbeEdge {
                    ends: [outline.pts[local], outline.pts[local + 1]],
                };
                if !point_on_edge(point, &edge) {
                    other_legs += 1;
                    continue;
                }
                raw_matches += 1;
                let axis = image.axis_at(
                    &edge,
                    [(pixel % width) as f64 + 0.5, (pixel / width) as f64 + 0.5],
                );
                if !image.covered(&axis, plates) {
                    visible_axis += 1;
                    continue;
                }
                let Some((covering, triangle)) = image.full_pixel_cover(&edge, pixel, plates)
                else {
                    partial_pixels += 1;
                    continue;
                };
                surfaced += 1;
                locations.push(format!(
                    "p{oi}/s{si}@{},{}[id={actual};edge={local};cover=m{covering}/t{triangle}]",
                    pixel % width,
                    pixel / width
                ));
                println!(
                    "RENDERED_IDS geometry p{oi}/s{si}: sample={point:?} represented_axis={axis:?} edge={:?} covering_triangle={:?}",
                    edge.ends, plates[covering].tris[triangle]
                );
            }
        }
    }
    println!(
        "RENDERED_IDS covered samples {covered}, raw same-edge matches {raw_matches}, pixel-axis-visible {visible_axis}, partial-pixel matches {partial_pixels}, fully-covered-pixel matches {surfaced}, other-leg aliases {other_legs}; original geometry, exact object and segment identity, picking alpha >= 0.5"
    );
    println!("RENDERED_IDS locations: {}", locations.join(" "));
    if std::env::var("CENSUS_REQUIRE_ZERO").is_ok_and(|value| value == "1") {
        assert_eq!(
            surfaced, 0,
            "renderer exposed covered original-source ink; inspect RENDERED_IDS locations"
        );
    }
}

/// Whether the pixel belongs to this leg, not another visible one.
fn point_on_edge(point: &[f64; 3], edge: &ProbeEdge) -> bool {
    let direction = sub(&edge.ends[1], &edge.ends[0]);
    let length2 = dot(&direction, &direction);
    if length2 == 0.0 {
        return false;
    }
    let t = (dot(&sub(point, &edge.ends[0]), &direction) / length2).clamp(0.0, 1.0);
    let nearest = edge_point(edge, t);
    norm(&sub(point, &[nearest[0], nearest[1], nearest[2]])) < 1e-6
}

/// Count hidden ink pixels at covered samples.
fn rendered_samples(outlines: &[Outline], plates: &[Plate], image: &SampleImage<'_>) {
    let mut covered = 0;
    let mut surfaced = 0;
    let mut faint = 0;
    let mut locations = Vec::new();
    let probe = std::env::var("CENSUS_PROBE_OUTLINE")
        .ok()
        .map(|v| v.parse::<usize>().expect("CENSUS_PROBE_OUTLINE index"));
    for (oi, outline) in outlines.iter().enumerate() {
        if probe.is_some_and(|index| index != oi) {
            continue;
        }
        for (si, point) in outline.samples.iter().enumerate() {
            if !judge(
                outline,
                point,
                plates,
                &image.fit.eye,
                &image.fit.fwd,
                image.height as f64,
            )
            .covered
            {
                continue;
            }
            let Some((pixel, centre)) = image.sample_pixel(point) else {
                continue;
            };
            if !judge(
                outline,
                &centre,
                plates,
                &image.fit.eye,
                &image.fit.fwd,
                image.height as f64,
            )
            .covered
            {
                continue;
            }
            covered += 1;
            let at = pixel * 3;
            let (r, g, b) = (
                image.pixels[at] as i16,
                image.pixels[at + 1] as i16,
                image.pixels[at + 2] as i16,
            );
            if r > g + 5 && b > g + 5 {
                faint += 1;
            }
            if r >= 195 && b >= 195 && g <= 60 {
                surfaced += 1;
                let cover = judge(
                    outline,
                    point,
                    plates,
                    &image.fit.eye,
                    &image.fit.fwd,
                    image.height as f64,
                );
                locations.push(format!(
                    "p{oi}/s{si}@{},{}[world={:.6},{:.6},{:.6};cover=m{};sep={:.6}mm]",
                    pixel % image.width,
                    pixel / image.width,
                    point[0],
                    point[1],
                    point[2],
                    cover.plate,
                    cover.sep
                ));
            }
        }
    }
    println!(
        "RENDERED covered samples {covered}, magenta samples {surfaced}, faint magenta samples {faint}; strict RGB >=195,<=60,>=195; faint R-G>5 and B-G>5"
    );
    println!("RENDERED locations: {}", locations.join(" "));
}

/// The scene with only one outline kept.
fn recolor_single(s: &Session, index: usize, out: &str) {
    assert!(
        index < s.objects.polylines.len(),
        "CENSUS_PROBE_OUTLINE out of range"
    );
    let world = s.world_xforms();
    let mut copy = Session::new("census single original outline");
    for mesh in &s.objects.meshes {
        let mut placed = mesh.duplicate();
        if let Some(transform) = world.get(mesh.guid()) {
            placed.transform(transform);
        }
        copy.add_mesh(placed, None);
    }
    for (i, source) in s.objects.polylines.iter().enumerate() {
        let mut line = source.duplicate();
        if let Some(transform) = world.get(source.guid()) {
            line.transform(transform);
        }
        line.linecolor = if i == index {
            Color::magenta()
        } else {
            Color::black()
        };
        copy.add_polyline(line, None);
    }
    copy.pb_dump(out);
    println!("single outline copy: {out}; original p{index} magenta, other outlines black");
}

/// Clip a parameter interval to a half-space.
fn clip_interval(interval: &mut [f64; 2], values: [f64; 2]) -> bool {
    if values[0] < 0.0 && values[1] < 0.0 {
        return false;
    }
    if values[0] < 0.0 {
        interval[0] = interval[0].max(values[0] / (values[0] - values[1]));
    } else if values[1] < 0.0 {
        interval[1] = interval[1].min(values[0] / (values[0] - values[1]));
    }
    interval[0] < interval[1]
}

/// The exact hidden interval of an edge.
fn triangle_interval(edge: &ProbeEdge, tri: &[[f64; 3]; 3], view: &ProbeView) -> Option<[f64; 2]> {
    let mut interval = [0.0f64, 1.0f64];
    let mut normal = cross(&sub(&tri[1], &tri[0]), &sub(&tri[2], &tri[0]));
    if norm(&normal) < 1e-12 {
        return None;
    }
    normal = unit(&normal);
    let toward_eye = match view.parallel {
        Some(fwd) => [-fwd[0], -fwd[1], -fwd[2]],
        None => sub(&view.eye, &tri[0]),
    };
    let side = dot(&normal, &toward_eye);
    if side.abs() < 1e-12 {
        return None;
    }
    let sign = side.signum();
    // tolerance against coplanar noise only
    let values = [
        -sign * dot(&normal, &sub(&edge.ends[0], &tri[0])) - 1e-7,
        -sign * dot(&normal, &sub(&edge.ends[1], &tri[0])) - 1e-7,
    ];
    if !clip_interval(&mut interval, values) {
        return None;
    }
    for i in 0..3 {
        let a = tri[i];
        let b = tri[(i + 1) % 3];
        let c = tri[(i + 2) % 3];
        let ray = match view.parallel {
            Some(fwd) => fwd,
            None => sub(&view.eye, &a),
        };
        let n = unit(&cross(&sub(&b, &a), &ray));
        let sign = dot(&n, &sub(&c, &a)).signum();
        let values = [
            sign * dot(&n, &sub(&edge.ends[0], &a)),
            sign * dot(&n, &sub(&edge.ends[1], &a)),
        ];
        if !clip_interval(&mut interval, values) {
            return None;
        }
    }
    Some(interval)
}

/// Sort intervals along the edge.
fn by_interval(a: &[f64; 2], b: &[f64; 2]) -> Ordering {
    a[0].total_cmp(&b[0])
}

/// Union of the covered spans of an edge.
fn hidden_intervals(edge: &ProbeEdge, plates: &[Plate], view: &ProbeView) -> Vec<[f64; 2]> {
    let mut intervals = Vec::new();
    for plate in plates {
        for tri in &plate.tris {
            if let Some(interval) = triangle_interval(edge, tri, view) {
                intervals.push(interval);
            }
        }
    }
    intervals.sort_by(by_interval);
    let mut merged: Vec<[f64; 2]> = Vec::new();
    for interval in intervals {
        if let Some(last) = merged.last_mut()
            && interval[0] <= last[1]
        {
            last[1] = last[1].max(interval[1]);
        } else {
            merged.push(interval);
        }
    }
    merged
}

/// The point at parameter `t` along the edge.
fn edge_point(edge: &ProbeEdge, t: f64) -> Point {
    Point::new(
        edge.ends[0][0] + (edge.ends[1][0] - edge.ends[0][0]) * t,
        edge.ends[0][1] + (edge.ends[1][1] - edge.ends[0][1]) * t,
        edge.ends[0][2] + (edge.ends[1][2] - edge.ends[0][2]) * t,
    )
}

/// Write hidden spans magenta and visible spans blue.
fn recolor_spans(
    s: &Session,
    outlines: &[Outline],
    plates: &[Plate],
    settings: &RecolorSettings<'_>,
) {
    let (fit, out) = (settings.fit, settings.out);
    let parallel = if env_f64("CENSUS_ORTHO_H", 0.0) > 0.0 {
        Some(fit.fwd)
    } else {
        None
    };
    let view = ProbeView {
        eye: [
            fit.eye[0] * 1000.0,
            fit.eye[1] * 1000.0,
            fit.eye[2] * 1000.0,
        ],
        parallel,
    };
    let world = s.world_xforms();
    let mut copy = Session::new("census exact hidden spans");
    for mesh in &s.objects.meshes {
        let mut placed = mesh.duplicate();
        if let Some(transform) = world.get(mesh.guid()) {
            placed.transform(transform);
        }
        copy.add_mesh(placed, None);
    }
    let mut counts = [0usize; 2];
    for (i, outline) in outlines.iter().enumerate() {
        for points in outline.pts.windows(2) {
            let edge = ProbeEdge {
                ends: [points[0], points[1]],
            };
            let intervals = hidden_intervals(&edge, plates, &view);
            let mut cursor = 0.0;
            let mut spans = Vec::new();
            for interval in intervals {
                if interval[0] > cursor {
                    spans.push((cursor, interval[0], false));
                }
                spans.push((interval[0], interval[1], true));
                cursor = interval[1];
            }
            if cursor < 1.0 {
                spans.push((cursor, 1.0, false));
            }
            for (start, end, hidden) in spans {
                let mut line =
                    Polyline::new(vec![edge_point(&edge, start), edge_point(&edge, end)]);
                line.width = s.objects.polylines[i].width;
                line.linecolor = if hidden {
                    Color::magenta()
                } else {
                    Color::blue()
                };
                counts[usize::from(hidden)] += 1;
                copy.add_polyline(line, None);
            }
        }
    }
    copy.pb_dump(out);
    println!(
        "exact span copy: {out}; visible spans {}, hidden spans {}. Magenta classifies the underlying axis; silhouette overhang and transition caps require spatial checking.",
        counts[0], counts[1]
    );
}

/// Run the census on one .pb file.
fn census(path: &str) {
    if std::env::var("CENSUS_REQUIRE_ZERO").is_ok_and(|value| value == "1") {
        assert!(
            std::env::var("CENSUS_RENDERED_IDS").is_ok(),
            "CENSUS_REQUIRE_ZERO requires an actual CENSUS_RENDERED_IDS frame"
        );
    }
    let vp_w = env_f64("VIEWER_W", 900.0);
    let vp_h = env_f64("VIEWER_H", 700.0);
    let bytes = std::fs::read(path).expect("read pb");
    let s = Session::pb_loads(&bytes).expect("parse pb");
    let world = s.world_xforms();
    println!(
        "== {path}  ({:.2} MB, {} objects, {} xforms, {} meshes, {} polylines, {} points, {} lines)",
        bytes.len() as f64 / 1.048576e6,
        s.lookup.len(),
        s.xforms.len(),
        s.objects.meshes.len(),
        s.objects.polylines.len(),
        s.objects.points.len(),
        s.objects.lines.len()
    );

    let mut plates: Vec<Plate> = Vec::new();
    for m in &s.objects.meshes {
        plates.push(plate_of(m, &placement(&world, m.guid())));
    }
    println!(
        "meshes: i verts faces | extents sorted (thin mid long) diag | t_rule t_real ratio | big face |nz|"
    );
    for (i, p) in plates.iter().enumerate() {
        println!(
            "  m{i:<3} {:>4} {:>4} | {:8.2} {:8.2} {:8.2} {:8.2} | {:7.2} {:7.2} {:6.2} | {:.3}",
            p.verts.len(),
            p.faces.len(),
            p.ext[0],
            p.ext[1],
            p.ext[2],
            p.diag,
            p.t_rule,
            p.t_real,
            p.t_rule / p.t_real.max(1e-9),
            p.big_nz
        );
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
            let centre = [
                (lo[0] + hi[0]) * 0.0005,
                (lo[1] + hi[1]) * 0.0005,
                (lo[2] + hi[2]) * 0.0005,
            ];
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
            f0 = Fit {
                eye: [v[0], v[1], v[2]],
                fwd,
                distance: d,
            };
            println!(
                "  CENSUS_EYE override: eye ({:.2}, {:.2}, {:.2}) m, distance {:.3} m",
                v[0], v[1], v[2], d
            );
        }
    }
    println!(
        "polylines: i pts | extents sorted diag | t_rule | |newell nz| | nearest face plane: dist plate face, thickness normal to it | samples covered at k=1 4 16 | failing samples at k=1 4 16 | min margin mm at k=1 4 16"
    );
    let mut per_k: Vec<Vec<Verdict>> = Vec::new();
    for _ in SCALES {
        per_k.push(Vec::new());
    }
    for (i, o) in outlines.iter().enumerate() {
        let on = o.dist <= ON_FACE_TOL;
        let behind = if on {
            plates[o.plate].faces[o.face].behind
        } else {
            f64::NAN
        };
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
            mins.push_str(&format!(
                " {:8.2}",
                if min_margin.is_finite() {
                    min_margin
                } else {
                    f64::NAN
                }
            ));
        }
        println!(
            "  p{i:<3} {:>2} | {:8.2} {:8.2} {:8.2} {:8.2} | {:6.2} | {:.3} | {:7.4} m{:<3} f{:<3} {:7.2} |{cov} of {:>2} |{fail} |{mins}",
            o.pts.len(),
            o.ext[0],
            o.ext[1],
            o.ext[2],
            o.diag,
            o.t_rule,
            o.nz,
            o.dist,
            o.plate,
            o.face,
            behind,
            o.samples.len()
        );
        if let Some((w, smp)) = &worst1 {
            println!(
                "    WORST k=1 p{i}: sample ({:.0}, {:.0}, {:.0}) mm on m{} f{}  cover m{} (t_real {:.2}) push {:.2} lift {:.2} sep {:.2} margin {:.2}",
                smp[0],
                smp[1],
                smp[2],
                o.plate,
                o.face,
                w.plate,
                plates[w.plate].t_real,
                w.push,
                w.lift,
                w.sep,
                w.margin
            );
        }
    }

    println!("plates: {}", plates.len());
    let mut v = Vec::new();
    for p in &plates {
        v.push(p.ext[0]);
    }
    stats(
        "thickness mm (thinnest AABB axis = what the rule sees)",
        &mut v,
    );
    v.clear();
    for p in &plates {
        v.push(p.t_rule);
    }
    stats("thickness mm (rule: max(thinnest, 0.001 x diag))", &mut v);
    v.clear();
    for p in &plates {
        v.push(p.t_real);
    }
    stats(
        "thickness mm (min extent along any face normal = real)",
        &mut v,
    );
    v.clear();
    for p in &plates {
        v.push(p.t_rule / p.t_real.max(1e-9));
    }
    stats(
        "t_rule / t_real (rule overestimate on rotated plates)",
        &mut v,
    );
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
    println!(
        "  flat meshes (thinnest axis < {ON_FACE_TOL} mm): {flat}   plates whose t_rule exceeds t_real by >1%: {rotated}   triangles ray-cast: {tris}"
    );

    println!("polylines: {}", outlines.len());
    v.clear();
    for o in &outlines {
        v.push(o.dist);
    }
    stats(
        "distance to nearest mesh face plane mm (max point deviation)",
        &mut v,
    );
    v.clear();
    for o in &outlines {
        v.push(o.ext[0]);
    }
    stats(
        "outline thinnest AABB axis mm (its lift cap is 0.25 x max(this, 0.001 x diag))",
        &mut v,
    );
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
    println!(
        "  on a mesh face (<= {ON_FACE_TOL} mm): {on} of {}",
        outlines.len()
    );
    stats("that plate's thickness normal to the face mm", &mut v);

    println!(
        "scene AABB mm: min [{:.1}, {:.1}, {:.1}] max [{:.1}, {:.1}, {:.1}]  extent {:.1} x {:.1} x {:.1}  diagonal {:.1}",
        lo[0],
        lo[1],
        lo[2],
        hi[0],
        hi[1],
        hi[2],
        hi[0] - lo[0],
        hi[1] - lo[1],
        hi[2] - lo[2],
        norm(&sub(&hi, &lo))
    );
    let proj_y = 1.0 / 30.0_f64.to_radians().tan() * 0.001;
    for (w, h) in [(vp_w, vp_h), (1280.0, 720.0), (1920.0, 1080.0)] {
        let f = fit(&lo, &hi, w / h);
        let px_per_mm = proj_y * h / (2.0 * f.distance);
        println!(
            "  fit {w:.0}x{h:.0}: distance {:.3} m  eye ({:.2}, {:.2}, {:.2}) m  fwd ({:.3}, {:.3}, {:.3})  at target {:.4} px/mm, 1 px = {:.2} mm, uncapped free lift (0.5 px) = {:.2} mm",
            f.distance,
            f.eye[0],
            f.eye[1],
            f.eye[2],
            f.fwd[0],
            f.fwd[1],
            f.fwd[2],
            px_per_mm,
            1.0 / px_per_mm,
            0.5 / px_per_mm * LIFT_RADII_FREE
        );
    }

    println!(
        "LEGACY WORLD-OFFSET ESTIMATE at k x camera distance ({vp_w:.0}x{vp_h:.0}), retained for BEFORE comparison only; margin = best cover's (separation along the ray - its face push) - the outline's lift"
    );
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
        println!(
            " k={k:<3} distance {:.2} m  0.4% of it = {:.1} mm  samples {} covered {n_cov} FAIL {n_fail} (of which {grazing} graze a cover's edge: separation < that plate's t_real)  outlines covered {cov_outlines} with a FAIL {}: {fail_outlines:?}",
            f0.distance * k,
            PUSH_FRAC * f0.distance * k * 1000.0,
            per_k[ki].len(),
            fail_outlines.len()
        );
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
    let picks = [
        ("thinnest outlined plate", order[0].1),
        ("median outlined plate", order[(order.len() - 1) / 2].1),
        ("thickest outlined plate", order[order.len() - 1].1),
    ];
    for (label, pi) in picks {
        let p = &plates[pi];
        println!(
            "{label}: m{pi}  extents {:.2} x {:.2} x {:.2}  diag {:.2}  t_rule {:.2} (push cap {:.2})  t_real {:.2}  big face |nz| {:.3}",
            p.ext[0],
            p.ext[1],
            p.ext[2],
            p.diag,
            p.t_rule,
            PUSH_MAX_THICK * p.t_rule,
            p.t_real,
            p.big_nz
        );
        for (oi, o) in outlines.iter().enumerate() {
            if o.plate != pi || o.dist > ON_FACE_TOL {
                continue;
            }
            for k in SCALES {
                let eye = eye_at(&f0, k);
                let mut worst = Verdict {
                    covered: false,
                    w: 0.0,
                    sep: 0.0,
                    push: 0.0,
                    lift: 0.0,
                    margin: f64::INFINITY,
                    plate: usize::MAX,
                };
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
                println!(
                    "  p{oi:<3} k={k:<3} covered {n_cov}/{}  worst sample: eye depth {:6.2} m  0.4% = {:6.1} mm  cover m{} (t_rule {:.2}, t_real {:.2}) push {:6.2} mm  lift {:5.2} mm (cap {:.2})  separation {:7.2} mm  margin {:8.2} mm {fail}",
                    o.samples.len(),
                    worst.w,
                    PUSH_FRAC * worst.w * 1000.0,
                    worst.plate,
                    plates[worst.plate].t_rule,
                    plates[worst.plate].t_real,
                    worst.push,
                    worst.lift,
                    LIFT_MAX_THICK * o.t_rule,
                    worst.sep,
                    worst.margin
                );
            }
        }
    }
    if let Ok(out) = std::env::var("CENSUS_RECOLOR") {
        recolor(&s, &outlines, &plates, &f0, vp_h, &out);
    }
    if let Ok(out) = std::env::var("CENSUS_RECOLOR_SPANS") {
        recolor_spans(
            &s,
            &outlines,
            &plates,
            &RecolorSettings {
                fit: &f0,
                out: &out,
            },
        );
    }
    if let Ok(path) = std::env::var("CENSUS_RENDERED_SPANS") {
        rendered_samples(&outlines, &plates, &SampleImage::read(&path, &f0));
    }
    if let Ok(out) = std::env::var("CENSUS_RECOLOR_SINGLE") {
        let index: usize = std::env::var("CENSUS_PROBE_OUTLINE")
            .expect("CENSUS_RECOLOR_SINGLE needs CENSUS_PROBE_OUTLINE")
            .parse()
            .expect("outline index");
        recolor_single(&s, index, &out);
    }
    if std::env::var("CENSUS_RENDERED_IDS").is_ok() {
        rendered_ids(&s, &outlines, &plates, &f0);
    }
}

#[cfg(test)]
mod span_tests {
    use super::*;

    /// A concave face keeps its notch open.
    #[test]
    fn cached_concave_triangulation_preserves_notch() {
        let vertices = [
            [0.0, 0.0],
            [3.0, 0.0],
            [3.0, 3.0],
            [2.0, 3.0],
            [2.0, 1.0],
            [1.0, 1.0],
            [1.0, 3.0],
            [0.0, 3.0],
        ];
        let mut mesh = Mesh::from_vertices_and_faces(
            vertices.map(|p| Point::new(p[0], p[1], 0.0)).to_vec(),
            vec![(0..8).collect()],
        );
        mesh.triangulation.insert(
            mesh.faces()[0],
            vec![
                [0, 1, 4],
                [0, 4, 5],
                [1, 2, 3],
                [1, 3, 4],
                [0, 5, 6],
                [0, 6, 7],
            ],
        );
        let place = Xform::identity();
        let view = ProbeView {
            eye: [0.0, 0.0, 10.0],
            parallel: Some([0.0, 0.0, -1.0]),
        };
        let notch = ProbeEdge {
            ends: [[1.4, 2.0, -1.0], [1.6, 2.0, -1.0]],
        };
        let plate = plate_of(&mesh, &place);
        assert_eq!(plate.tris.len(), 6);
        assert!(
            plate
                .tris
                .iter()
                .all(|triangle| triangle_interval(&notch, triangle, &view).is_none())
        );
        mesh.triangulation.clear();
        let fan = plate_of(&mesh, &place);
        assert!(
            fan.tris
                .iter()
                .any(|triangle| triangle_interval(&notch, triangle, &view).is_some())
        );
    }

    /// A placed face normal stays orthogonal to its edges.
    #[test]
    fn placed_face_normal_uses_inverse_transpose() {
        let mesh = Mesh::from_vertices_and_faces(
            vec![
                Point::new(0.0, 0.0, 0.0),
                Point::new(1.0, 0.0, 0.0),
                Point::new(0.0, 1.0, 1.0),
            ],
            vec![vec![0, 1, 2]],
        );
        let place = Xform::from_matrix([
            2.0, 0.0, 0.25, 0.0, 0.5, 3.0, 0.0, 0.0, 0.0, 0.0, 4.0, 0.0, 5.0, -2.0, 8.0, 1.0,
        ]);
        let plate = plate_of(&mesh, &place);
        let face = &plate.faces[0];
        for vertex in &plate.verts {
            assert!((dot(&face.n, vertex) - face.d).abs() < 1e-12);
        }
        assert!((norm(&face.n) - 1.0).abs() < 1e-12);
    }

    /// Perspective cover reaches behind a triangle, not in front.
    #[test]
    fn perspective_triangle_coverage() {
        let triangle = [[-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], [0.0, 1.0, 0.0]];
        let view = ProbeView {
            eye: [0.0, 0.0, 10.0],
            parallel: None,
        };
        let edge = ProbeEdge {
            ends: [[-2.0, 0.0, -1.0], [2.0, 0.0, -1.0]],
        };
        let interval = triangle_interval(&edge, &triangle, &view).expect("covered centre");
        assert!((interval[0] - 0.3625).abs() < 1e-12);
        assert!((interval[1] - 0.6375).abs() < 1e-12);
        for z in [0.0, 1.0] {
            let edge = ProbeEdge {
                ends: [[-2.0, 0.0, z], [2.0, 0.0, z]],
            };
            assert!(triangle_interval(&edge, &triangle, &view).is_none());
        }
    }

    /// Parallel cover is the triangle footprint.
    #[test]
    fn orthographic_triangle_coverage() {
        let triangle = [[-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], [0.0, 1.0, 0.0]];
        let view = ProbeView {
            eye: [0.0, 0.0, 10.0],
            parallel: Some([0.0, 0.0, -1.0]),
        };
        let edge = ProbeEdge {
            ends: [[-2.0, 0.0, -100.0], [2.0, 0.0, -100.0]],
        };
        let interval = triangle_interval(&edge, &triangle, &view).expect("covered centre");
        assert!((interval[0] - 0.375).abs() < 1e-12);
        assert!((interval[1] - 0.625).abs() < 1e-12);
    }

    /// Pixel y runs downward in both projections.
    #[test]
    fn sample_projection_and_pixel_centre() {
        let fit = Fit {
            eye: [0.0, 0.0, 10.0],
            fwd: [0.0, 0.0, -1.0],
            distance: 10.0,
        };
        for ortho_h in [0.0, 10000.0 * 30.0f64.to_radians().tan()] {
            let image = SampleImage {
                pixels: Vec::new(),
                width: 1000,
                height: 800,
                right: [1.0, 0.0, 0.0],
                up: [0.0, 1.0, 0.0],
                fit: &fit,
                ortho_h,
            };
            let (pixel, centre) = image.sample_pixel(&[0.0, 0.0, 0.0]).expect("centre pixel");
            assert_eq!(pixel, 400500);
            assert!(centre[0] > 0.0 && centre[1] < 0.0 && centre[2] == 0.0);
            let (again, same) = image.sample_pixel(&centre).expect("same pixel");
            assert_eq!(again, pixel);
            assert!(norm(&sub(&same, &centre)) < 1e-9);
        }
    }

    /// Axis depth interpolates perspective-correctly.
    #[test]
    fn represented_axis_depth_is_perspective_correct() {
        let fit = Fit {
            eye: [0.0, 0.0, 0.01],
            fwd: [0.0, 0.0, -1.0],
            distance: 0.01,
        };
        let image = SampleImage {
            pixels: Vec::new(),
            width: 100,
            height: 100,
            right: [1.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            fit: &fit,
            ortho_h: 0.0,
        };
        let edge = ProbeEdge {
            ends: [[-1.0, 0.0, -1.0], [1.0, 0.0, -10.0]],
        };
        let (a, _) = image.projected(&edge.ends[0]);
        let (b, _) = image.projected(&edge.ends[1]);
        let axis = image.axis_at(&edge, [(a[0] + b[0]) * 0.5, (a[1] + b[1]) * 0.5]);
        let expected = edge_point(&edge, 11.0 / 31.0);
        assert!(norm(&sub(&axis, &[expected[0], expected[1], expected[2]])) < 1e-12);
    }

    /// Full-pixel cover rejects silhouette pixels.
    #[test]
    fn full_pixel_cover_excludes_silhouette() {
        let fit = Fit {
            eye: [0.0, 0.0, 0.01],
            fwd: [0.0, 0.0, -1.0],
            distance: 0.01,
        };
        let image = SampleImage {
            pixels: Vec::new(),
            width: 100,
            height: 100,
            right: [1.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            fit: &fit,
            ortho_h: 0.0,
        };
        let plate = Plate {
            verts: Vec::new(),
            faces: Vec::new(),
            tris: vec![[[-2.0, -2.0, 0.0], [2.0, -2.0, 0.0], [0.0, 2.0, 0.0]]],
            lo: [0.0; 3],
            hi: [0.0; 3],
            ext: [0.0; 3],
            diag: 0.0,
            t_rule: 0.0,
            t_real: 0.0,
            big_nz: 0.0,
        };
        let plates = [plate];
        let interior = ProbeEdge {
            ends: [[-0.5, 0.0, -1.0], [0.5, 0.0, -1.0]],
        };
        assert_eq!(
            image.full_pixel_cover(&interior, 5050, &plates),
            Some((0, 0))
        );
        let silhouette = ProbeEdge {
            ends: [[1.04, 0.0, -1.0], [1.12, 0.0, -1.0]],
        };
        assert_eq!(image.full_pixel_cover(&silhouette, 5058, &plates), None);
    }

    /// A thin joint still covers in orthographic mode.
    #[test]
    fn physical_coverage_has_no_virtual_eye_distance_tolerance() {
        let mesh = Mesh::from_vertices_and_faces(
            vec![
                Point::new(-2.0, -2.0, 0.0),
                Point::new(2.0, -2.0, 0.0),
                Point::new(0.0, 2.0, 0.0),
            ],
            vec![vec![0, 1, 2]],
        );
        let plates = [plate_of(&mesh, &Xform::identity())];
        for distance in [10.0, 1000.0] {
            let fit = Fit {
                eye: [0.0, 0.0, distance],
                fwd: [0.0, 0.0, -1.0],
                distance,
            };
            let image = SampleImage {
                pixels: Vec::new(),
                width: 100,
                height: 100,
                right: [1.0, 0.0, 0.0],
                up: [0.0, 1.0, 0.0],
                fit: &fit,
                ortho_h: distance * 1000.0,
            };
            assert!(image.covered(&[0.0, 0.0, -0.001], &plates));
            assert!(!image.covered(&[0.0, 0.0, 0.0], &plates));
            assert!(!image.covered(&[0.0, 0.0, 0.001], &plates));
        }
    }
}

/// Run the census on every file in the arguments.
fn main() {
    for path in std::env::args().skip(1) {
        census(&path);
    }
}
