//! Probe scene: one smooth solid alone, `sphere` (default), `torus`, `hole` or `dome`.
use session_rust::brep::BRep;
use session_rust::{Color, NurbsSurface, Point, Session, Xform};

/// Half width of the dome patch.
const HALF: f64 = 300.0;

/// The dome's 4x4 control grid.
fn dome_points() -> Vec<Point> {
    let coord = [-HALF, -HALF / 3.0, HALF / 3.0, HALF];
    let mut points = Vec::with_capacity(16);
    for &x in &coord {
        for &y in &coord {
            let r = (x * x + y * y).sqrt();
            points.push(Point::new(x, y, 300.0 + 200.0 * (r / 250.0).cos()));
        }
    }
    points
}

/// The named solid, lifted onto z = 0.
fn solid(which: &str) -> BRep {
    let (mut b, up, color) = match which {
        "torus" => (
            BRep::create_torus(220.0, 70.0),
            70.0,
            Color::new(0.91, 0.83, 0.80, 1.0),
        ),
        "hole" => (
            BRep::create_block_with_hole(500.0, 300.0, 200.0, 80.0),
            100.0,
            Color::grey(),
        ),
        _ => (
            BRep::create_sphere(180.0),
            180.0,
            Color::new(0.82, 0.85, 0.91, 1.0),
        ),
    };
    b.transform(&Xform::translation(0.0, 0.0, up));
    b.surfacecolor = color;
    b
}

/// Write the chosen solid to `argv[1]`.
fn main() {
    let out = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "target/shade_probe.pb".into());
    let which = std::env::args().nth(2).unwrap_or_else(|| "sphere".into());
    let mut s = Session::new("shade_probe");
    if which == "dome" {
        let mut d =
            NurbsSurface::create(false, false, 3, 3, 4, 4, &dome_points()).expect("dome patch");
        d.facecolors = vec![Color::grey()];
        d.name = "dome".to_string();
        s.add_nurbssurface(d, None);
    } else {
        s.add_brep(solid(&which), None);
    }
    s.pb_dump(&out);
    println!("wrote {out}");
}
