// Hidden line behind a BRep cylinder: the case the ink suite never covered. A magenta polyline
// stands behind the cylinder (seen from the front camera, VIEWER_ORBIT=0,60) so that the
// cylinder's curved side must hide its middle; its ends stick out on both sides so a run that
// hides everything is caught too. Zero magenta pixels in the frame at distance 1 and 4 is the
// acceptance, read from the colour frame the way the joint probe reads it.
//
// cargo run --release --target x86_64-unknown-linux-gnu --example mk_cylinder_hidden_probe -- <out.pb>
use session_rust::brep::BRep;
use session_rust::{Color, Point, Polyline, Session};

/// Write the cylinder and its fully-hidden line into one session at `argv[1]`.
fn main() {
    let out = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "target/cylinder_hidden.pb".into());
    let mut s = Session::new("cylinder_hidden");
    let mut cylinder = BRep::create_cylinder(150.0, 400.0);
    cylinder.surfacecolor = Color::grey();
    s.add_brep(cylinder, None);
    // 60 mm behind the axis, at half height, 100 mm short of each silhouette: hidden entirely.
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
