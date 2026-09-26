//! Probe scene: a magenta line fully hidden behind a BRep cylinder.
use session_rust::brep::BRep;
use session_rust::{Color, Point, Polyline, Session};

/// Write the cylinder and the hidden line to `argv[1]`.
fn main() {
    let out = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "target/cylinder_hidden.pb".into());
    let mut s = Session::new("cylinder_hidden");
    let mut cylinder = BRep::create_cylinder(150.0, 400.0);
    cylinder.surfacecolor = Color::grey();
    s.add_brep(cylinder, None);
    // behind the axis, short of both silhouettes
    let mut line = Polyline::new(vec![
        Point::new(-50.0, 60.0, 200.0),
        Point::new(50.0, 60.0, 200.0),
    ]);
    line.linecolor = Color::magenta();
    line.name = "hidden".to_string();
    s.add_polyline(line, None);
    s.pb_dump(&out);
    println!("wrote {out}");
}
