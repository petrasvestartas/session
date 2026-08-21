// Probe: a grey 400 mm box PLUS the same 12 edges as free Lines (FACING_UNKNOWN, so the facing
// cull never fires and hidden-line removal must come from the depth test alone). Any far-side
// line drawn over the box is ink lifted in front of the surface it decorates.
//
// cargo run --example mk_freelines_scene --target x86_64-unknown-linux-gnu --release -- <out.pb>
fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "target/freelines.pb".into());
    let m = session_rust::Mesh::create_box(400.0, 400.0, 400.0);
    let edges = m.edges();
    let mut lines = Vec::new();
    for (a, b) in &edges {
        lines.push({ let p0 = m.vertex_point(*a).unwrap(); let p1 = m.vertex_point(*b).unwrap(); session_rust::Line::new(p0[0], p0[1], p0[2], p1[0], p1[1], p1[2]) });
    }
    let mut s = session_rust::Session::new("freelines");
    s.add_mesh(m, None);
    for l in lines {
        s.add_line(l, None);
    }
    s.pb_dump(&out);
    println!("wrote {out} ({} lines)", edges.len());
}
