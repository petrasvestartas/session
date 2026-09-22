//! Probe scene: three plates with a blue top outline and a hidden magenta bottom outline.
use session_rust::{Color, Mesh, Point, Polyline, Session, Xform};

const INSET: f64 = 20.0; // bottom outline inset, mm

/// A closed rectangle at height `z`, inset inside the plate.
fn outline(y0: f64, z: f64, inset: f64, color: Color) -> Polyline {
    let (x0, x1, ya, yb) = (inset, 4000.0 - inset, y0 + inset, y0 + 300.0 - inset);
    let mut pl = Polyline::new(vec![
        Point::new(x0, ya, z),
        Point::new(x1, ya, z),
        Point::new(x1, yb, z),
        Point::new(x0, yb, z),
        Point::new(x0, ya, z),
    ]);
    pl.linecolor = color;
    pl
}

/// Write the three plates and their outlines to `argv[1]`.
fn main() {
    let out = std::env::args()
        .nth(1)
        .unwrap_or("target/plate_outline.pb".to_string());
    let mut s = Session::new("plate_outline");
    for (y0, dz, tilt) in [(0.0, 40.0, 0.0), (600.0, 200.0, 0.0), (1200.0, 40.0, 30.0)] {
        let mut plate = Mesh::create_box(4000.0, 300.0, dz);
        plate.transform(&Xform::translation(2000.0, y0 + 150.0, dz * 0.5));
        plate.set_objectcolor(Color::grey());
        let mut top = outline(y0, dz, 0.0, Color::blue());
        let mut bottom = outline(y0, 0.0, INSET, Color::magenta());
        if tilt != 0.0 {
            let about = Xform::translation(0.0, y0 + 150.0, dz * 0.5)
                * Xform::rotation_x(tilt, true)
                * Xform::translation(0.0, -(y0 + 150.0), -dz * 0.5);
            plate.transform(&about);
            top.transform(&about);
            bottom.transform(&about);
        }
        s.add_mesh(plate, None);
        s.add_polyline(top, None);
        s.add_polyline(bottom, None);
    }
    s.pb_dump(&out);
    println!("wrote {out}");
}
