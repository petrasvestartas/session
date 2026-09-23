use crate::app::walk::Row;
use crate::app::walk::encode::{FACING_UNKNOWN, encode_width, pack_rgba};
use crate::engine::gpu::clip::ClipPlane;
use crate::engine::gpu::segments::SegRows;
use crate::engine::gpu::{CylinderSegment, Instance};
use session_rust::{AABB, Mesh, Plane, Point, Vector, Xform};
use std::sync::atomic::{AtomicBool, Ordering};

/// The name that makes a plane a clipping plane.
pub const NAME: &str = "clipping_plane";

/// Arrow length over the rectangle's smaller half size.
const ARROW: f64 = 0.2;

/// Set by the first cut: from then on every mesh is checked for closedness as it is walked.
static SOLIDS: AtomicBool = AtomicBool::new(false);

/// How a clipping plane is placed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Normal, // an origin, then a point on the side to cut away
    Points, // three points; the cut side by the right-hand rule
    Xy,     // the world XY plane through a point, +Z cut away
    Yz,     // the world YZ plane through a point, +X cut away
    Zx,     // the world ZX plane through a point, +Y cut away
}

impl Mode {
    /// Every mode, in the order the options list them.
    pub const ALL: [Mode; 5] = [Mode::Normal, Mode::Points, Mode::Xy, Mode::Yz, Mode::Zx];

    /// The word typed for it.
    pub fn word(self) -> &'static str {
        match self {
            Mode::Normal => "Normal",
            Mode::Points => "3Point",
            Mode::Xy => "XY",
            Mode::Yz => "YZ",
            Mode::Zx => "ZX",
        }
    }

    /// Points the mode takes.
    pub fn points(self) -> usize {
        match self {
            Mode::Normal => 2,
            Mode::Points => 3,
            _ => 1,
        }
    }

    /// The mode a typed word names, any case.
    pub fn parse(word: &str) -> Option<Mode> {
        Mode::ALL
            .into_iter()
            .find(|mode| mode.word().eq_ignore_ascii_case(word))
    }

    /// What each picked point is for.
    pub fn prompts(self) -> &'static [&'static str] {
        match self {
            Mode::Normal => &[
                "First point: a point on the cutting plane (or choose XY, YZ, ZX, 3Point)",
                "Second point: toward the side to remove; origin-to-point sets the perpendicular direction (e.g. @0,0,1 removes above)",
            ],
            Mode::Points => &[
                "First point",
                "Second point, along the plane",
                "Third point: the cut side follows the right-hand rule",
            ],
            Mode::Xy => &["Origin point: the +Z side is cut away"],
            Mode::Yz => &["Origin point: the +X side is cut away"],
            Mode::Zx => &["Origin point: the +Y side is cut away"],
        }
    }
}

/// True for a plane that cuts the scene.
pub fn is_clipping(plane: &Plane) -> bool {
    plane.name == NAME
}

/// Turn on the closedness check of meshes; true the first time.
pub fn verify_solids() -> bool {
    !SOLIDS.swap(true, Ordering::Relaxed)
}

/// True once meshes are checked for closedness as they are walked.
pub fn solids_verified() -> bool {
    SOLIDS.load(Ordering::Relaxed)
}

/// A clipping plane through picked points, its rectangle `half` wide each way from the origin.
pub fn plane_from(mode: Mode, points: &[[f64; 3]], half: f64) -> Result<Plane, String> {
    if points.len() != mode.points() {
        return Err(format!(
            "clipping_plane {} needs {} point{}",
            mode.word(),
            mode.points(),
            if mode.points() == 1 { "" } else { "s" }
        ));
    }

    let origin = points[0];
    let (x, y, z) = match mode {
        Mode::Normal => {
            let n = sub(points[1], origin);

            if length(n) <= 1e-9 {
                return Err("the normal point must differ from the origin".into());
            }

            let frame = Plane::from_point_normal(
                Point::new(origin[0], origin[1], origin[2]),
                Vector::new(n[0], n[1], n[2]),
                Some(true),
            );
            (
                array(&frame.x_axis()),
                array(&frame.y_axis()),
                array(&frame.z_axis()),
            )
        }
        Mode::Points => {
            let along = sub(points[1], origin);
            let z = cross(along, sub(points[2], origin));

            if length(along) <= 1e-9 || length(z) <= 1e-9 * length(along).max(1.0) {
                return Err("the three points must not lie on one line".into());
            }

            let x = unit(along);
            let z = unit(z);
            (x, cross(z, x), z)
        }
        Mode::Xy => ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]),
        Mode::Yz => ([0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
        Mode::Zx => ([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    };
    let mut plane = Plane::from_frame(
        Point::new(origin[0], origin[1], origin[2]),
        Vector::new(x[0] * half, x[1] * half, x[2] * half),
        Vector::new(y[0] * half, y[1] * half, y[2] * half),
        Vector::new(z[0], z[1], z[2]),
    );
    plane.name = NAME.into();
    Ok(plane)
}

/// The plane turned around: the other side is cut away.
pub fn flipped(plane: &Plane) -> Plane {
    let mut plane = plane.clone();
    plane.reverse();
    plane
}

/// The world clipping plane of `plane` placed at `place`; None when its rectangle is flat.
pub fn clip_plane(plane: &Plane, place: &Xform) -> Option<ClipPlane> {
    let origin = array(&place.transform_point(&plane.origin()));
    let x = array(&place.transform_vector(&plane.x_axis()));
    let y = array(&place.transform_vector(&plane.y_axis()));
    let cut = cross(x, y);

    if !(length(cut) > 1e-300) || !origin.iter().all(|v| v.is_finite()) {
        return None;
    }

    let normal = unit(cut).map(|v| -v);
    // across the 45 degree lines, in the plane
    let across = sub(unit(x), unit(y));
    let hatch = if length(across) > 1e-9 {
        unit(across)
    } else {
        unit(x)
    };
    Some(ClipPlane {
        normal,
        offset: -dot(normal, origin),
        origin,
        hatch,
    })
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
fn array<T: std::ops::Index<usize, Output = f64>>(v: &T) -> [f64; 3] {
    [v[0], v[1], v[2]]
}

/// a + b.
fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

/// a - b.
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// a * s.
fn scale(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}

/// a · b.
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// a × b.
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// |a|.
fn length(a: [f64; 3]) -> f64 {
    dot(a, a).sqrt()
}

/// a at unit length; zero stays zero.
fn unit(a: [f64; 3]) -> [f64; 3] {
    let l = length(a);

    if l > 0.0 { scale(a, 1.0 / l) } else { a }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Kept or cut side of a world point.
    fn kept(plane: &ClipPlane, p: [f64; 3]) -> bool {
        plane.distance(p) >= 0.0
    }

    /// Each mode puts the plane where it says and cuts the side it says.
    #[test]
    fn modes_place_the_plane_and_the_cut_side() {
        let normal = plane_from(Mode::Normal, &[[1.0, 2.0, 3.0], [1.0, 2.0, 13.0]], 50.0).unwrap();
        assert_eq!(array(&normal.z_axis()), [0.0, 0.0, 1.0]);
        assert_eq!(array(&normal.origin()), [1.0, 2.0, 3.0]);
        assert!((length(array(&normal.x_axis())) - 50.0).abs() < 1e-9);
        assert!((length(array(&normal.y_axis())) - 50.0).abs() < 1e-9);
        assert!(is_clipping(&normal));

        for (mode, cut) in [
            (Mode::Xy, [0.0, 0.0, 1.0]),
            (Mode::Yz, [1.0, 0.0, 0.0]),
            (Mode::Zx, [0.0, 1.0, 0.0]),
        ] {
            let plane = plane_from(mode, &[[0.0, 0.0, 5.0]], 10.0).unwrap();
            let clip = clip_plane(&plane, &Xform::identity()).unwrap();
            assert!(
                !kept(&clip, add([0.0, 0.0, 5.0], cut)),
                "{mode:?} cuts its +axis side"
            );
            assert!(
                kept(&clip, sub([0.0, 0.0, 5.0], cut)),
                "{mode:?} keeps the other"
            );
        }

        let three = plane_from(
            Mode::Points,
            &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            10.0,
        )
        .unwrap();
        let clip = clip_plane(&three, &Xform::identity()).unwrap();
        assert!(!kept(&clip, [0.0, 0.0, 1.0]), "right-hand rule: +Z is cut");
        assert!(kept(&clip, [0.0, 0.0, -1.0]));
    }

    /// Equal or collinear points and wrong counts are refused.
    #[test]
    fn degenerate_picks_are_refused() {
        assert!(plane_from(Mode::Normal, &[[1.0, 1.0, 1.0], [1.0, 1.0, 1.0]], 5.0).is_err());
        assert!(
            plane_from(
                Mode::Points,
                &[[0.0, 0.0, 0.0], [1.0, 1.0, 1.0], [2.0, 2.0, 2.0]],
                5.0
            )
            .is_err()
        );
        assert!(plane_from(Mode::Xy, &[], 5.0).is_err());
        assert!(plane_from(Mode::Normal, &[[0.0; 3]], 5.0).is_err());
    }

    /// The placement moves, turns and stretches the cut exactly.
    #[test]
    fn the_cut_follows_the_placement() {
        let plane = plane_from(Mode::Xy, &[[0.0, 0.0, 0.0]], 10.0).unwrap();
        let moved = clip_plane(&plane, &Xform::translation(0.0, 0.0, 10.0)).unwrap();
        assert!(kept(&moved, [0.0, 0.0, 9.999]) && !kept(&moved, [0.0, 0.0, 10.001]));

        // turned about x: +Z becomes -Y
        let turned = clip_plane(&plane, &Xform::rotation_x(90.0, true)).unwrap();
        assert!(!kept(&turned, [0.0, -0.001, 0.0]) && kept(&turned, [0.0, 0.001, 0.0]));

        // stretched unevenly: the normal still comes out exact
        let stretched = clip_plane(&plane, &Xform::scale_xyz(2.0, 1.0, 1.0)).unwrap();
        assert!((stretched.normal[2] + 1.0).abs() < 1e-12);

        for clip in [moved, turned, stretched] {
            assert!(
                (length(clip.hatch) - 1.0).abs() < 1e-12,
                "unit hatch direction"
            );
            assert!(
                dot(clip.hatch, clip.normal).abs() < 1e-12,
                "hatch lies in the plane"
            );
        }
    }

    /// Flipping swaps the kept side.
    #[test]
    fn a_flip_swaps_the_sides() {
        let plane = plane_from(Mode::Xy, &[[0.0, 0.0, 0.0]], 10.0).unwrap();
        let before = clip_plane(&plane, &Xform::identity()).unwrap();
        let after = clip_plane(&flipped(&plane), &Xform::identity()).unwrap();
        assert!(kept(&before, [0.0, 0.0, -1.0]) && !kept(&after, [0.0, 0.0, -1.0]));
        assert!(is_clipping(&flipped(&plane)));
    }

    /// The rectangle is four ribbons, the arrow three, all on the row and never cut.
    #[test]
    fn the_walk_draws_a_rectangle_and_an_arrow() {
        let plane = plane_from(Mode::Xy, &[[0.0, 0.0, 0.0]], 10.0).unwrap();
        let mut seg = SegRows::default();
        let row = walk(&mut seg, &plane, 7);
        assert_eq!(seg.ribbons.len(), 7);
        assert!(seg.ribbons.iter().all(|r| r.instance_id == 7));
        assert_ne!(row.flags & Instance::FLAG_CLIPPING_PLANE, 0);
        assert_eq!(row.bounds.min_point()[0], -10.0);
        assert_eq!(
            row.bounds.max_point()[2],
            2.0,
            "the arrow points +Z, 0.2 of the half size"
        );
        assert_eq!(row.bounds.center()[2], 0.0, "the box is centred on the plane");
    }

    /// A clipping plane is one undo step and survives a save with its name and size.
    #[test]
    fn a_clipping_plane_undoes_and_saves() {
        use crate::app::scene::Scene;
        use session_rust::{Geometry, Session};

        let mut scene = Scene::new();
        let plane = plane_from(Mode::Xy, &[[0.0, 0.0, 50.0]], 300.0).unwrap();
        let (doc, guid) = scene
            .create_geometry(Geometry::Plane(std::rc::Rc::new(plane)))
            .unwrap();
        assert!(scene.undo());
        assert!(!scene.docs[doc].session.lookup.contains_key(guid.as_str()));
        assert!(scene.redo());
        let mut session = (*scene.docs[doc].session).clone();
        let saved = Session::pb_loads(&session.pb_dumps()).unwrap();
        let Some(Geometry::Plane(loaded)) = saved.lookup.get(guid.as_str()) else {
            panic!("the plane is saved");
        };
        assert!(is_clipping(loaded));
        assert!((length(array(&loaded.x_axis())) - 300.0).abs() < 1e-9);
        assert_eq!(array(&loaded.origin()), [0.0, 0.0, 50.0]);
    }

    /// A box is closed and outward; turned inside out it is inward; with a face gone it is open.
    #[test]
    fn closedness_and_winding_of_meshes() {
        let cube = Mesh::create_box(2.0, 2.0, 2.0);
        assert_eq!(solid_orientation(&cube), Some(false));

        let mut inside_out = cube.clone();
        for corners in inside_out.face.values_mut() {
            corners.reverse();
        }
        assert_eq!(solid_orientation(&inside_out), Some(true));

        let mut open = cube.clone();
        let first = *open.face.keys().min().unwrap();
        open.face.remove(&first);
        assert_eq!(solid_orientation(&open), None);

        let mut mixed = cube.clone();
        mixed.face.get_mut(&first).unwrap().reverse();
        assert_eq!(
            solid_orientation(&mixed),
            None,
            "one face wound the wrong way"
        );
    }
}
