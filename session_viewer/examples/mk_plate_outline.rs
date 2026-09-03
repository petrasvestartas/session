// Depth-fight probe: two grey plates lying flat (thickness along z), each with its TOP outline
// as a closed pure-blue polyline and its BOTTOM outline as a closed pure-magenta one, both
// EXACTLY on the faces they trace. Seen from above every magenta pixel is the bottom outline
// surfacing through the plate, every blue pixel is the top outline drawn where it belongs.
// Plate 1 is 4000 x 300 x 40 mm at y = 0 (the thin regime: an ink lift or face push over
// 40 mm reaches the far face); plate 2 is 4000 x 300 x 200 mm at y = 600. Default pens: render
// with VIEWER_NO_EDGES=1 so the box's own black wireframe, which shares both outlines with the
// coloured ink, stays out of the count.
//
// cargo run --example mk_plate_outline --target x86_64-unknown-linux-gnu --release -- <out.pb>
fn main() {
    let out = std::env::args().nth(1).unwrap_or("target/plate_outline.pb".to_string());
    let mut s = session_rust::Session::new("plate_outline");
    for (y0, dz) in [(0.0, 40.0), (600.0, 200.0)] {
        let mut plate = session_rust::Mesh::create_box(4000.0, 300.0, dz);
        plate.transform(&session_rust::Xform::translation(2000.0, y0 + 150.0, dz * 0.5));
        plate.set_objectcolor(session_rust::Color::grey());
        s.add_mesh(plate, None);
        for (z, color) in [(dz, session_rust::Color::blue()), (0.0, session_rust::Color::magenta())] {
            let mut outline = session_rust::Polyline::new(vec![
                session_rust::Point::new(0.0, y0, z),
                session_rust::Point::new(4000.0, y0, z),
                session_rust::Point::new(4000.0, y0 + 300.0, z),
                session_rust::Point::new(0.0, y0 + 300.0, z),
                session_rust::Point::new(0.0, y0, z),
            ]);
            outline.linecolor = color;
            s.add_polyline(outline, None);
        }
    }
    s.pb_dump(&out);
    println!("wrote {out}");
}
