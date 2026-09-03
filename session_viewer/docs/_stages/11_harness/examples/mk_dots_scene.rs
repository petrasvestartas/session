// Dots-only probe for the glyph-size investigation: the SAME box corners as mk_wedge_scene,
// but as 8 Point objects with a 10 mm pen and no mesh - so the glyph lane renders alone and
// the disc's intended size can be measured without the bands drawing over it.
//
// cargo run --example mk_dots_scene --target x86_64-unknown-linux-gnu --release -- <out.pb>
fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "target/wedge/dots.pb".into());

    let mut s = session_rust::Session::new("dots_box");
    for &sx in &[0.0, 400.0] {
        for &sy in &[0.0, 400.0] {
            for &sz in &[0.0, 400.0] {
                let mut p = session_rust::Point::new(600.0 - 200.0 + sx, -200.0 + sy, -200.0 + sz);
                p.width = 10.0;
                p.pointcolor = session_rust::Color::red();
                s.add_point(p, None);
            }
        }
    }
    s.add_point(session_rust::Point::new(0.0, 0.0, 0.0), None); // anchor, same fit as onebox.pb
    s.pb_dump(&out);
    println!("wrote {out}");
}
