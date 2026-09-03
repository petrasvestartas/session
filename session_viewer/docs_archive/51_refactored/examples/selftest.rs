// cargo run --example selftest --target x86_64-unknown-linux-gnu --release -- <out.ppm> <file.pb>...

// wgpu reports validation errors through `log`; with no logger installed a broken shader just
// renders black. The smallest possible stderr logger, so a broken frame says WHY.
struct StderrLog;
impl log::Log for StderrLog {
    fn enabled(&self, _: &log::Metadata) -> bool { true }
    fn log(&self, r: &log::Record) { eprintln!("[{}] {}", r.level(), r.args()); }
    fn flush(&self) {}
}

fn main() {
    let _ = log::set_logger(&StderrLog);
    log::set_max_level(log::LevelFilter::Info);
    let a: Vec<String> = std::env::args().skip(1).collect();
    let out = a.first().cloned().unwrap_or_else(|| "out.ppm".into());
    // A .json/.toml argument is a SCENE MANIFEST, resolved the way the browser does it.
    let files = session_viewer::selftest::SceneFile::from_args(&a[1.min(a.len())..]);
    let size = (
        std::env::var("VIEWER_W").ok().and_then(|v| v.parse().ok()).unwrap_or(900),
        std::env::var("VIEWER_H").ok().and_then(|v| v.parse().ok()).unwrap_or(700),
    );
    print!("{}", session_viewer::selftest::render_scene(&files, size, &out));
}
