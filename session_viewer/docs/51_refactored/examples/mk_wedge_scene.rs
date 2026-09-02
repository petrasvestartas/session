// Generates the wedge acceptance scene: ONE box with red 10 mm world pens, plus a tiny anchor
// point at the origin. The anchor pulls the fit target ~400 mm off the box, so VIEWER_ZOOM
// dollies the camera to a hover just OUTSIDE the box's west face instead of inside the solid
// (where the facing cull would legitimately remove every edge). No third object means no
// genuine occlusion: the depth-on vs depth-forced-Always comparison isolates the bug.
//
// cargo run --example mk_wedge_scene --target x86_64-unknown-linux-gnu --release -- <out.pb>
fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "target/wedge/onebox.pb".into());

    let mut m = session_rust::Mesh::create_box(400.0, 400.0, 400.0);
    let n = m.edges_with_colors().len();
    m.set_linecolors(vec![session_rust::Color::red(); n], vec![10.0; n]); // 10 mm plot pen
    let guid = m.guid().to_string();

    let mut s = session_rust::Session::new("wedge_box");
    s.add_mesh(m, None);
    s.set_xform(&guid, session_rust::Xform::translation(600.0, 0.0, 0.0));
    s.add_point(session_rust::Point::new(0.0, 0.0, 0.0), None);
    s.pb_dump(&out);
    println!("wrote {out}");
}
