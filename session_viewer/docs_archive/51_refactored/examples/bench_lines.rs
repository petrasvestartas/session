// cargo run --example bench_lines --target x86_64-unknown-linux-gnu --release -- <file.pb | scene.json>...
// Times BOTH line styles (tubes vs flat) on the same scene, fit and far views.

struct StderrLog;
impl log::Log for StderrLog {
    fn enabled(&self, _: &log::Metadata) -> bool { true }
    fn log(&self, r: &log::Record) { eprintln!("[{}] {}", r.level(), r.args()); }
    fn flush(&self) {}
}

fn main() {
    let _ = log::set_logger(&StderrLog);
    log::set_max_level(log::LevelFilter::Warn);
    let a: Vec<String> = std::env::args().skip(1).collect();
    let files = session_viewer::selftest::SceneFile::from_args(&a);
    let size = (
        std::env::var("VIEWER_W").ok().and_then(|v| v.parse().ok()).unwrap_or(1568),
        std::env::var("VIEWER_H").ok().and_then(|v| v.parse().ok()).unwrap_or(724),
    );
    print!("{}", session_viewer::selftest::bench_scene(&files, size));
}
