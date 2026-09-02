// Grey-cube probe for the vertex-marker check: ONE default box (grey faces, black edges - the
// colors an unauthored mesh gets), so the corner markers come out black on the pens' own color
// path. Same anchor trick as mk_wedge_scene, so VIEWER_ZOOM hovers outside the west face.
//
// cargo run --example mk_cube_scene --target x86_64-unknown-linux-gnu --release -- <out.pb>
fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "target/wedge/greycube.pb".into());

    let mut m = session_rust::Mesh::create_box(400.0, 400.0, 400.0);
    let n = m.edges_with_colors().len();
    // Grey faces, black 10 mm world pens - wide enough that a corner marker that is NOT fully in
    // front shows a bite out of its disc.
    m.set_linecolors(vec![session_rust::Color::black(); n], vec![10.0; n]);
    let guid = m.guid().to_string();

    let mut s = session_rust::Session::new("grey_box");
    s.add_mesh(m, None);
    s.set_xform(&guid, session_rust::Xform::translation(600.0, 0.0, 0.0));
    s.add_point(session_rust::Point::new(0.0, 0.0, 0.0), None);
    s.pb_dump(&out);
    println!("wrote {out}");
}
