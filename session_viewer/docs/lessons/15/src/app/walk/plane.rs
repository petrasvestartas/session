use crate::app::walk::Row;
use crate::app::walk::encode::{FACING_UNKNOWN, encode_width, pack_rgba};
use crate::engine::gpu::segments::SegRows;
use crate::engine::gpu::{CylinderSegment, Instance};
use session_rust::{AABB, Mesh, Plane};
use std::sync::atomic::{AtomicBool, Ordering};

/// The name that makes a plane a clipping plane.
pub const NAME: &str = "Clipping Plane";

/// Arrow length over the rectangle's smaller half size.
const ARROW: f64 = 0.2;

/// Set by the first cut: from then on every mesh is checked for closedness as it is walked.
static SOLIDS: AtomicBool = AtomicBool::new(false);

/// True for a plane that cuts the scene.
pub fn is_clipping(plane: &Plane) -> bool {
    plane.name == NAME || plane.name == "clipping_plane" // the name before 2026-09-24
}

/// Turn on the closedness check of meshes; true the first time.
pub fn verify_solids() -> bool {
    !SOLIDS.swap(true, Ordering::Relaxed)
}

/// True once meshes are checked for closedness as they are walked.
pub fn solids_verified() -> bool {
    SOLIDS.load(Ordering::Relaxed)
}

/// The rectangle and an arrow toward the cut side, as ribbons of `row`.
pub fn walk(seg: &mut SegRows, plane: &Plane, row: u32) -> Row {
    let o = array(&plane.origin());
    let x = array(&plane.x_axis());
    let y = array(&plane.y_axis());
    let corner = |a: f64, b: f64| add(o, add(scale(x, a), scale(y, b)));
    let arrow = ARROW * length(x).min(length(y));
    let z = unit(cross(x, y));
    let tip = add(o, scale(z, arrow));
    let wing = scale(unit(x), arrow * 0.25);
    let back = add(o, scale(z, arrow * 0.7));
    let rectangle = [
        corner(1.0, 1.0),
        corner(-1.0, 1.0),
        corner(-1.0, -1.0),
        corner(1.0, -1.0),
    ];
    let mut lines: Vec<[[f64; 3]; 2]> = (0..4)
        .map(|i| [rectangle[i], rectangle[(i + 1) % 4]])
        .collect();
    lines.push([o, tip]);
    lines.push([tip, add(back, wing)]);
    lines.push([tip, sub(back, wing)]);
    let radius = encode_width(plane.width);
    let color = pack_rgba(plane.linecolor.to_f32());
    // mirrored below the plane too, so the gumball sits on the plane's origin
    let below = sub(o, scale(z, arrow));
    let mut bounds = AABB::empty();
    bounds.union_with_point(below[0], below[1], below[2]);

    for [a, b] in lines {
        bounds.union_with_point(a[0], a[1], a[2]);
        bounds.union_with_point(b[0], b[1], b[2]);
        seg.ribbons.push(CylinderSegment {
            p0: a.map(|v| v as f32),
            radius,
            p1: b.map(|v| v as f32),
            instance_id: row,
            color,
            facing: FACING_UNKNOWN, // no face orientation
        });
    }

    Row {
        flags: Instance::FLAG_CLIPPING_PLANE,
        ..Row::thin(bounds)
    }
}

/// Closed and wound one way, every edge walked as often each way: Some(inward), else None.
pub fn solid_orientation(mesh: &Mesh) -> Option<bool> {
    if mesh.vertex.is_empty() {
        return None;
    }

    // hole rings: the kernel decides
    if !mesh.face_holes.is_empty() {
        return mesh.is_closed().then(|| six_volume(mesh) < 0.0);
    }

    let mut edges: Vec<u64> = Vec::with_capacity(mesh.face.values().map(Vec::len).sum());

    for corners in mesh.face.values() {
        for (i, &a) in corners.iter().enumerate() {
            let b = corners[(i + 1) % corners.len()];
            let (lo, hi) = (a.min(b), a.max(b));

            if lo == hi {
                continue;
            }

            // keys too big to pack: the kernel decides
            if hi >= 1 << 31 {
                return mesh.is_closed().then(|| six_volume(mesh) < 0.0);
            }

            // the edge, then which way this face walks it
            edges.push((lo as u64) << 33 | (hi as u64) << 1 | u64::from(a > b));
        }
    }

    edges.sort_unstable();

    for run in edges.chunk_by(|a, b| a >> 1 == b >> 1) {
        let forward = run.iter().filter(|edge| *edge & 1 == 0).count();

        if forward * 2 != run.len() {
            return None;
        }
    }

    Some(six_volume(mesh) < 0.0)
}

/// Row flags of a solid: closed, and inward when its faces wind inward; none when not closed.
pub fn solid_flags(solid: Option<bool>) -> u32 {
    match solid {
        Some(true) => Instance::FLAG_CLOSED | Instance::FLAG_INWARD,
        Some(false) => Instance::FLAG_CLOSED,
        None => 0,
    }
}

/// Six times the signed volume a mesh's faces enclose, fanned from each face's first corner.
fn six_volume(mesh: &Mesh) -> f64 {
    let mut volume = 0.0;

    for corners in mesh.face.values() {
        let Some(first) = corners.first().and_then(|key| mesh.vertex.get(key)) else {
            continue;
        };
        let a = [first.x, first.y, first.z];

        for pair in corners[1..].windows(2) {
            if let (Some(b), Some(c)) = (mesh.vertex.get(&pair[0]), mesh.vertex.get(&pair[1])) {
                volume += dot(a, cross([b.x, b.y, b.z], [c.x, c.y, c.z]));
            }
        }
    }

    volume
}

/// A point or vector as three numbers.
pub(crate) fn array<T: std::ops::Index<usize, Output = f64>>(v: &T) -> [f64; 3] {
    [v[0], v[1], v[2]]
}

/// a + b.
pub(crate) fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

/// a - b.
pub(crate) fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// a * s.
pub(crate) fn scale(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}

/// a · b.
pub(crate) fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// a × b.
pub(crate) fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// |a|.
pub(crate) fn length(a: [f64; 3]) -> f64 {
    dot(a, a).sqrt()
}

/// a at unit length; zero stays zero.
pub(crate) fn unit(a: [f64; 3]) -> [f64; 3] {
    let l = length(a);

    if l > 0.0 { scale(a, 1.0 / l) } else { a }
}
