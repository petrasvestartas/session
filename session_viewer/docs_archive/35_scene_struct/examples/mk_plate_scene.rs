// Thin-shell probe: a 400 x 400 x 8 mm plate, subdivided 8x8 on the big faces, black default
// pens. The regime where ink lift + face push can exceed the OBJECT's own thickness - if they
// do, the far face's wireframe surfaces through the near face (the bunny-ear black-out).
//
// cargo run --example mk_plate_scene --target x86_64-unknown-linux-gnu --release -- <out.pb>
fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "target/plate.pb".into());
    let mut polys = Vec::new();
    let n = 8;
    let s = 400.0 / n as f64;
    for i in 0..n {
        for j in 0..n {
            let (x0, y0) = (i as f64 * s, j as f64 * s);
            for z in [0.0, 8.0] {
                polys.push(vec![
                    session_rust::Point::new(x0, y0, z),
                    session_rust::Point::new(x0 + s, y0, z),
                    session_rust::Point::new(x0 + s, y0 + s, z),
                    session_rust::Point::new(x0, y0 + s, z),
                ]);
            }
        }
    }
    // side walls
    for i in 0..n {
        let x0 = i as f64 * s;
        polys.push(vec![
            session_rust::Point::new(x0, 0.0, 0.0), session_rust::Point::new(x0 + s, 0.0, 0.0),
            session_rust::Point::new(x0 + s, 0.0, 8.0), session_rust::Point::new(x0, 0.0, 8.0)]);
        polys.push(vec![
            session_rust::Point::new(x0, 400.0, 0.0), session_rust::Point::new(x0 + s, 400.0, 0.0),
            session_rust::Point::new(x0 + s, 400.0, 8.0), session_rust::Point::new(x0, 400.0, 8.0)]);
        polys.push(vec![
            session_rust::Point::new(0.0, x0, 0.0), session_rust::Point::new(0.0, x0 + s, 0.0),
            session_rust::Point::new(0.0, x0 + s, 8.0), session_rust::Point::new(0.0, x0, 8.0)]);
        polys.push(vec![
            session_rust::Point::new(400.0, x0, 0.0), session_rust::Point::new(400.0, x0 + s, 0.0),
            session_rust::Point::new(400.0, x0 + s, 8.0), session_rust::Point::new(400.0, x0, 8.0)]);
    }
    let m = session_rust::Mesh::from_polylines(polys, None);
    let mut s2 = session_rust::Session::new("plate");
    s2.add_mesh(m, None);
    s2.pb_dump(&out);
    println!("wrote {out}");
}
