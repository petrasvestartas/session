//! Scene of seven BReps, four curves, two surfaces and a polyline in two rows.
use session_rust::brep::BRep;
use session_rust::{Color, NurbsCurve, NurbsSurface, Point, Polyline, Session, Xform};

/// Slot spacing along a row, mm.
const STEP: f64 = 700.0;

/// Distance between the rows, mm.
const ROW: f64 = 900.0;

/// Slots per row.
const PER_ROW: usize = 7;

/// Half width of the patches and the cube.
const HALF: f64 = 300.0;

/// The x of slot `i`.
fn slot(i: usize) -> f64 {
    (i % PER_ROW) as f64 * STEP
}

/// The y of slot `i`.
fn row(i: usize) -> f64 {
    (i / PER_ROW) as f64 * ROW
}

/// Move `b` into slot `i`, lift it by `up`, colour it.
fn place(b: &mut BRep, i: usize, up: f64, color: Color) {
    b.transform(&Xform::translation(slot(i), row(i), up));
    b.surfacecolor = color;
}

/// A two-turn helix in slot `i`.
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

/// An S curve in slot `i`.
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

/// A tilted rational circle in slot `i`.
fn circle(i: usize) -> NurbsCurve {
    let mut circle = session_rust::Primitives::circle(0.0, 0.0, 0.0, 250.0);
    circle.transform(&Xform::rotation_x((120.0_f64 / 250.0).atan(), false));
    circle.transform(&Xform::translation(slot(i), row(i), 600.0));
    circle.linecolors = vec![Color::blue()];
    circle.name = "circle".into();
    circle
}

/// Points on the box top in slot `i`.
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

/// A cubic curve through `points`.
fn curve(points: &[Point], color: Color, name: &str) -> NurbsCurve {
    let mut c = NurbsCurve::create(false, 3, points);
    c.linecolors = vec![color];
    c.name = name.to_string();
    c
}

/// The dome's control grid in slot `i`.
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

/// The saddle's control grid in slot `i`.
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

/// A grey bicubic patch over `points`.
fn surface(points: &[Point], name: &str) -> NurbsSurface {
    let mut s = NurbsSurface::create(false, false, 3, 3, 4, 4, points).expect("mixed patch");
    s.facecolors = vec![Color::grey()];
    s.name = name.to_string();
    s
}

/// All cube edges in slot `i` as one polyline.
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

/// Write the whole scene to `argv[1]`.
fn main() {
    let out = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "target/mixed_solids.pb".into());
    let mut s = Session::new("mixed_solids");

    // slots 0..6: the BReps
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

    // slots 7..9: curves; the red one lies on the box top
    s.add_nurbscurve(curve(&helix_points(7), Color::blue(), "helix"), None);
    s.add_nurbscurve(curve(&s_curve_points(8), Color::blue(), "s_curve"), None);
    s.add_nurbscurve(circle(9), None);
    s.add_nurbscurve(
        curve(&box_top_points(0, 250.0), Color::red(), "on_box_top"),
        None,
    );

    // slots 10..11: the two surfaces
    s.add_nurbssurface(surface(&dome_points(10), "dome"), None);
    s.add_nurbssurface(surface(&saddle_points(11), "saddle"), None);

    // slot 12: the polyline cube
    s.add_polyline(cube_polyline(12), None);

    s.pb_dump(&out);
    println!("wrote {out}");
}
