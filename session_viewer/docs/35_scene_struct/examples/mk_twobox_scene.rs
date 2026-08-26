// Occlusion probe: a grey 400 box in FRONT, and 400 mm of red linework BEHIND it (mesh edges as
// free Lines at x = -600, so they ride the ribbon lane with FACING_UNKNOWN). From the +x side
// the red ink must be fully hidden; any red pixel is ink through a face, and countable.
//
// cargo run --example mk_twobox_scene --target x86_64-unknown-linux-gnu --release -- <out.pb>
fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "target/twobox.pb".into());
    let front = session_rust::Mesh::create_box(400.0, 400.0, 400.0);
    let back = session_rust::Mesh::create_box(400.0, 400.0, 400.0);
    let edges = back.edges();
    let mut lines = Vec::new();
    for (a, b) in &edges {
        let p0 = back.vertex_point(*a).unwrap();
        let p1 = back.vertex_point(*b).unwrap();
        lines.push(session_rust::Line::new(p0[0] - 600.0, p0[1], p0[2], p1[0] - 600.0, p1[1], p1[2]));
    }
    let mut s = session_rust::Session::new("twobox");
    s.add_mesh(front, None);
    for mut l in lines {
        l.linecolor = session_rust::Color::red();
        s.add_line(l, None);
    }
    s.pb_dump(&out);
    println!("wrote {out}");
}
