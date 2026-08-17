// cargo run --example selftest --target x86_64-unknown-linux-gnu --release -- <out.ppm> <file.pb>...
fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let out = a.first().cloned().unwrap_or_else(|| "out.ppm".into());
    let files: Vec<(&str, session_rust::Xform)> =
        a.iter().skip(1).map(|p| (p.as_str(), session_rust::Xform::identity())).collect();
    print!("{}", session_viewer::selftest::render_scene(
        &files,
        std::env::var("VIEWER_W").ok().and_then(|v| v.parse().ok()).unwrap_or(900),
        std::env::var("VIEWER_H").ok().and_then(|v| v.parse().ok()).unwrap_or(700),
        &out,
    ));
}
