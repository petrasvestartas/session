// Back-face probe: two coplanar 200 mm quads side by side, wound in OPPOSITE directions.
// From any one camera exactly one of them shows its back — so a single render proves both
// branches of the front_facing test at once.
fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "target/facing.pb".into());
    let p = |x: f64, y: f64| session_rust::Point::new(x, y, 0.0);
    let polys = vec![
        vec![p(0.0, 0.0), p(200.0, 0.0), p(200.0, 200.0), p(0.0, 200.0)],      // CCW seen from +Z
        vec![p(240.0, 0.0), p(240.0, 200.0), p(440.0, 200.0), p(440.0, 0.0)],  // CW  seen from +Z
    ];
    let m = session_rust::Mesh::from_polylines(polys, None);
    let mut s = session_rust::Session::new("facing");
    s.add_mesh(m, None);
    s.pb_dump(&out);
    println!("wrote {out}");
}
