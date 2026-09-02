// Tube-lane ground truth for mk_twobox_scene: the SAME back box, but as a mesh with red
// screen-px pens, so its edges render as real cylinder geometry with honest depth. Red on the
// front box here = genuinely visible; red in the ribbon scene but not here = penetration.
//
// cargo run --example mk_twobox_mesh --target x86_64-unknown-linux-gnu --release -- <out.pb>
fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "target/twobox_mesh.pb".into());
    let front = session_rust::Mesh::create_box(400.0, 400.0, 400.0);
    let mut back = session_rust::Mesh::create_box(400.0, 400.0, 400.0);
    let n = back.edges_with_colors().len();
    back.set_linecolors(vec![session_rust::Color::red(); n], vec![-1.0; n]); // screen-px pen
    let guid = back.guid().to_string();
    let mut s = session_rust::Session::new("twobox_mesh");
    s.add_mesh(front, None);
    s.add_mesh(back, None);
    s.set_xform(&guid, session_rust::Xform::translation(-600.0, 0.0, 0.0));
    s.pb_dump(&out);
    println!("wrote {out}");
}
