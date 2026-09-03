// cargo run --example bench_frame --target x86_64-unknown-linux-gnu --release -- assets/scenes/<scene>.toml
//
// Splits a frame into uniforms / encode / gpu, for a still and a moving camera. `bench_lines`
// answers how fast the frame is; this answers which of the three legs owns the milliseconds.

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
        std::env::var("VIEWER_W").ok().and_then(|v| v.parse().ok()).unwrap_or(900),
        std::env::var("VIEWER_H").ok().and_then(|v| v.parse().ok()).unwrap_or(700),
    );
    print!("{}", session_viewer::selftest::frame_profile(&files, size));
}
