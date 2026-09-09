// Mixed solids: the curved lanes the mixed scene was missing. Seven BReps (box, cylinder, cone,
// sphere, torus, block-with-hole, pyramid), four NURBS curves, two NURBS surfaces and one
// polyline, in two rows 700 mm apart - the seven BReps along y = 0, the curves, surfaces and
// the polyline along y = ROW - every solid standing on z = 0. The point is
// coverage, not a model: a seam edge (cylinder, sphere, torus), a degenerated apex (cone,
// pyramid), an inner wire (block-with-hole), a smooth-tessellation border (both surfaces) and a
// curve lying exactly on a BRep face (the red one on the box top) each exercise a different path
// of the ink rule, and a single file puts them all under the same orbit.
//
// cargo run --release --target x86_64-unknown-linux-gnu --example mk_mixed_solids -- <out.pb>
use session_rust::brep::BRep;
use session_rust::{Color, NurbsCurve, NurbsSurface, Point, Polyline, Session, Xform};

/// Centre-to-centre spacing along a row: the widest item (the torus, 580 mm) leaves 120 mm.
const STEP: f64 = 700.0;

/// Distance between the two rows: the deepest items (the patches, 600 mm) leave 300 mm.
const ROW: f64 = 900.0;

/// Items per row: the seven BReps fill the first row, the six others the second.
const PER_ROW: usize = 7;

/// Half-width of both NURBS patches and of the polyline cube, so each fits inside one slot.
const HALF: f64 = 300.0;

/// The x of slot `i`: slots wrap after PER_ROW.
fn slot(i: usize) -> f64 {
    (i % PER_ROW) as f64 * STEP
}

/// The y of slot `i`: the first PER_ROW slots on y = 0, the rest one ROW behind.
fn row(i: usize) -> f64 {
    (i / PER_ROW) as f64 * ROW
}

/// Move `b` into slot `i`, lift it by `up` so its lowest point sits on z = 0, and colour it.
fn place(b: &mut BRep, i: usize, up: f64, color: Color) {
    b.transform(&Xform::translation(slot(i), row(i), up));
    b.surfacecolor = color;
}

/// 12 points of a two-turn helix rising from z = 300 to z = 900 in slot `i`.
fn helix_points(i: usize) -> Vec<Point> {
    let mut points = Vec::with_capacity(12);
    for k in 0..12 {
        let t = k as f64 / 11.0;
        let a = t * 4.0 * std::f64::consts::PI;
        points.push(Point::new(
            slot(i) + 200.0 * a.cos(),
            row(i) + 200.0 * a.sin(),
            300.0 + 600.0 * t,
        ));
    }
    points
}

/// 7 points of an S laid in the xz plane of slot `i`, swinging ±250 mm about the slot centre.
fn s_curve_points(i: usize) -> Vec<Point> {
    let mut points = Vec::with_capacity(7);
    for k in 0..7 {
        let t = k as f64 / 6.0;
        let a = t * 2.0 * std::f64::consts::PI;
        points.push(Point::new(
            slot(i) + 250.0 * a.sin(),
            row(i),
            300.0 + 600.0 * t,
        ));
    }
    points
}

/// An exact rational circle, tilted as one rigid plane with a smooth closed seam.
fn circle(i: usize) -> NurbsCurve {
    let mut circle = session_rust::Primitives::circle(0.0, 0.0, 0.0, 250.0);
    circle.transform(&Xform::rotation_x((120.0_f64 / 250.0).atan(), false));
    circle.transform(&Xform::translation(slot(i), row(i), 600.0));
    circle.linecolors = vec![Color::blue()];
    circle.name = "circle".into();
    circle
}

/// 5 points lying exactly on the top face of the box in slot `i`, whose top is at `z`.
fn box_top_points(i: usize, z: f64) -> Vec<Point> {
    let mut points = Vec::with_capacity(5);
    for k in 0..5 {
        let t = k as f64 / 4.0;
        points.push(Point::new(
            slot(i) - 150.0 + 300.0 * t,
            row(i) + 100.0 * (t * 6.0).sin(),
            z,
        ));
    }
    points
}

/// A degree-3 open NURBS curve through `points`, drawn in `color`.
fn curve(points: &[Point], color: Color, name: &str) -> NurbsCurve {
    let mut c = NurbsCurve::create(false, 3, points);
    c.linecolors = vec![color];
    c.name = name.to_string();
    c
}

/// The 4x4 control grid of the dome in slot `i`: z = 300 + 200 * cos(r / 250).
fn dome_points(i: usize) -> Vec<Point> {
    let coord = [-HALF, -HALF / 3.0, HALF / 3.0, HALF];
    let mut points = Vec::with_capacity(16);
    for &x in &coord {
        for &y in &coord {
            let r = (x * x + y * y).sqrt();
            points.push(Point::new(
                slot(i) + x,
                row(i) + y,
                300.0 + 200.0 * (r / 250.0).cos(),
            ));
        }
    }
    points
}

/// The 4x4 control grid of the saddle in slot `i`: z = 250 + 0.0006 * (x*x - y*y).
fn saddle_points(i: usize) -> Vec<Point> {
    let coord = [-HALF, -HALF / 3.0, HALF / 3.0, HALF];
    let mut points = Vec::with_capacity(16);
    for &x in &coord {
        for &y in &coord {
            points.push(Point::new(
                slot(i) + x,
                row(i) + y,
                250.0 + 0.0006 * (x * x - y * y),
            ));
        }
    }
    points
}

/// A bicubic 4x4 NURBS patch over `points` (row-major, u slowest), grey like the BRep faces.
fn surface(points: &[Point], name: &str) -> NurbsSurface {
    let mut s = NurbsSurface::create(false, false, 3, 3, 4, 4, points).expect("mixed patch");
    s.facecolors = vec![Color::grey()];
    s.name = name.to_string();
    s
}

/// All 12 edges of a cube of side `2 * HALF` in slot `i` as ONE stroke: a cube has eight odd
/// vertices, so the shortest single path retraces three of its edges.
fn cube_polyline(i: usize) -> Polyline {
    let h = HALF;
    let corner = [
        [-h, -h, 0.0],
        [h, -h, 0.0],
        [h, h, 0.0],
        [-h, h, 0.0],
        [-h, -h, 2.0 * h],
        [h, -h, 2.0 * h],
        [h, h, 2.0 * h],
        [-h, h, 2.0 * h],
    ];
    let route = [0, 1, 2, 3, 0, 4, 5, 1, 5, 6, 2, 6, 7, 3, 7, 4];
    let mut points = Vec::with_capacity(route.len());
    for &v in &route {
        let c = corner[v];
        points.push(Point::new(slot(i) + c[0], row(i) + c[1], c[2]));
    }
    let mut p = Polyline::new(points);
    p.name = "cube_edges".to_string();
    p
}

fn main() {
    let out = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "target/mixed_solids.pb".into());
    let mut s = Session::new("mixed_solids");

    // Slots 0..6: the BReps. create_box, create_sphere, create_torus and create_block_with_hole
    // are centred on the origin; create_cylinder, create_cone and create_pyramid already stand
    // on z = 0. The lift is whatever puts the lowest point at z = 0.
    let mut solid = BRep::create_box(400.0, 300.0, 250.0);
    place(&mut solid, 0, 125.0, Color::new(0.86, 0.81, 0.72, 1.0));
    s.add_brep(solid, None);

    let mut solid = BRep::create_cylinder(150.0, 400.0);
    place(&mut solid, 1, 0.0, Color::grey());
    s.add_brep(solid, None);

    let mut solid = BRep::create_cone(150.0, 400.0);
    place(&mut solid, 2, 0.0, Color::new(0.78, 0.87, 0.82, 1.0));
    s.add_brep(solid, None);

    let mut solid = BRep::create_sphere(180.0);
    place(&mut solid, 3, 180.0, Color::new(0.82, 0.85, 0.91, 1.0));
    s.add_brep(solid, None);

    let mut solid = BRep::create_torus(220.0, 70.0);
    place(&mut solid, 4, 70.0, Color::new(0.91, 0.83, 0.80, 1.0));
    s.add_brep(solid, None);

    let mut solid = BRep::create_block_with_hole(500.0, 300.0, 200.0, 80.0);
    place(&mut solid, 5, 100.0, Color::grey());
    s.add_brep(solid, None);

    let mut solid = BRep::create_pyramid(400.0, 350.0);
    place(&mut solid, 6, 0.0, Color::new(0.87, 0.89, 0.76, 1.0));
    s.add_brep(solid, None);

    // Slots 7..9: free curves in the air, blue. The fourth curve is hosted: it lies exactly on
    // the box top at z = 250, so its ink must survive the face it sits on.
    s.add_nurbscurve(curve(&helix_points(7), Color::blue(), "helix"), None);
    s.add_nurbscurve(curve(&s_curve_points(8), Color::blue(), "s_curve"), None);
    s.add_nurbscurve(circle(9), None);
    s.add_nurbscurve(
        curve(&box_top_points(0, 250.0), Color::red(), "on_box_top"),
        None,
    );

    // Slots 10..11: the smooth lane, where the border edges are ink and the seam grid is not.
    s.add_nurbssurface(surface(&dome_points(10), "dome"), None);
    s.add_nurbssurface(surface(&saddle_points(11), "saddle"), None);

    // Slot 12: free linework of the same shape as a BRep's edges, so both meet in one scene.
    s.add_polyline(cube_polyline(12), None);

    s.pb_dump(&out);
    println!("wrote {out}");
}
