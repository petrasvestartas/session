// Shade probe: ONE smooth object alone in a file, so the harness's fit puts it across the whole
// frame and a scanline through it measures the shading. The argument picks the object: the
// sphere (default) for the scanline check, the torus and the block with hole for the seam and
// inner-wire loops, the dome for the NURBS surface path. Sizes and colours are the mixed
// scene's (`mk_mixed_solids`), so what is measured here is what that scene shows.
//
// cargo run --release --target x86_64-unknown-linux-gnu --example mk_shade_probe -- <out.pb> [sphere|torus|hole|dome]
use session_rust::brep::BRep;
use session_rust::{Color, NurbsSurface, Point, Session, Xform};

/// Half-width of the dome patch, as in the mixed scene.
const HALF: f64 = 300.0;

/// The 4x4 control grid of the mixed scene's dome: z = 300 + 200 * cos(r / 250).
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

/// The BRep named by `which`, lifted so its lowest point sits on z = 0, in the mixed scene's colour.
fn solid(which: &str) -> BRep {
    let (mut b, up, color) = match which {
        "torus" => (BRep::create_torus(220.0, 70.0), 70.0, Color::new(0.91, 0.83, 0.80, 1.0)),
        "hole" => (BRep::create_block_with_hole(500.0, 300.0, 200.0, 80.0), 100.0, Color::grey()),
        _ => (BRep::create_sphere(180.0), 180.0, Color::new(0.82, 0.85, 0.91, 1.0)),
    };
    b.transform(&Xform::translation(0.0, 0.0, up));
    b.surfacecolor = color;
    b
}

fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "target/shade_probe.pb".into());
    let which = std::env::args().nth(2).unwrap_or_else(|| "sphere".into());
    let mut s = Session::new("shade_probe");
    if which == "dome" {
        let mut d = NurbsSurface::create(false, false, 3, 3, 4, 4, &dome_points()).expect("dome patch");
        d.facecolors = vec![Color::grey()];
        d.name = "dome".to_string();
        s.add_nurbssurface(d, None);
    } else {
        s.add_brep(solid(&which), None);
    }
    s.pb_dump(&out);
    println!("wrote {out}");
}
