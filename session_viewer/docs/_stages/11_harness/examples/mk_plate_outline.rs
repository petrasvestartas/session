// Depth-fight probe: three grey plates, each with its TOP outline as a closed pure-blue
// polyline on the rim of its top face and its BOTTOM outline as a closed pure-magenta polyline
// on its bottom face, inset 20 mm inside the footprint so it is hidden by the plate from every
// angle above - a magenta pixel seen from above is therefore ink THROUGH the plate, never a
// silhouette. Plate 1: 4000 x 300 x 40 mm flat at y = 0 (the thin regime). Plate 2: the same
// 200 mm thick at y = 600. Plate 3: the 40 mm plate rotated 30 deg about its long axis at
// y = 1200, baked into its vertices (no xform): its axis-aligned box is 185 mm thick, the plate
// 40, which is the case a box-based thickness gets wrong. Render with VIEWER_NO_EDGES=1 so the
// meshes' own black wireframe stays out of the count.
//
// cargo run --example mk_plate_outline --target x86_64-unknown-linux-gnu --release -- <out.pb>
use session_rust::{Color, Mesh, Point, Polyline, Session, Xform};

const INSET: f64 = 20.0;

/// A closed rectangle at height `z`, `inset` inside the plate footprint, as a polyline.
fn outline(y0: f64, z: f64, inset: f64, color: Color) -> Polyline {
    let (x0, x1, ya, yb) = (inset, 4000.0 - inset, y0 + inset, y0 + 300.0 - inset);
    let mut pl = Polyline::new(vec![Point::new(x0, ya, z), Point::new(x1, ya, z), Point::new(x1, yb, z), Point::new(x0, yb, z), Point::new(x0, ya, z)]);
    pl.linecolor = color;
    pl
}

fn main() {
    let out = std::env::args().nth(1).unwrap_or("target/plate_outline.pb".to_string());
    let mut s = Session::new("plate_outline");
    for (y0, dz, tilt) in [(0.0, 40.0, 0.0), (600.0, 200.0, 0.0), (1200.0, 40.0, 30.0)] {
        let mut plate = Mesh::create_box(4000.0, 300.0, dz);
        plate.transform(&Xform::translation(2000.0, y0 + 150.0, dz * 0.5));
        plate.set_objectcolor(Color::grey());
        let mut top = outline(y0, dz, 0.0, Color::blue());
        let mut bottom = outline(y0, 0.0, INSET, Color::magenta());
        if tilt != 0.0 {
            let about = Xform::translation(0.0, y0 + 150.0, dz * 0.5) * Xform::rotation_x(tilt, true) * Xform::translation(0.0, -(y0 + 150.0), -dz * 0.5);
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
