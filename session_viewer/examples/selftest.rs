// cargo run --example selftest --target x86_64-unknown-linux-gnu --release -- <out.ppm> <scene.yaml | file.pb>...
//
// Renders one headless frame and prints the ink count; VIEWER_FRAMES=N times N frames first,
// VIEWER_PICK="x,y" reports what the id pass finds under a pixel. VIEWER_W / VIEWER_H size it.

use session_viewer::selftest::{render_scene, SceneFile};

/// wgpu reports validation errors through `log`; without a logger a broken shader renders black.
struct StderrLog;

impl log::Log for StderrLog {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, r: &log::Record) {
        eprintln!("[{}] {}", r.level(), r.args());
    }

    fn flush(&self) {}
}

fn main() {
    let _ = log::set_logger(&StderrLog);
    log::set_max_level(log::LevelFilter::Info);
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out = args.first().cloned().unwrap_or_else(|| "out.ppm".into());
    let files = SceneFile::from_args(&args[1.min(args.len())..]);
    let w = std::env::var("VIEWER_W").ok().and_then(|v| v.parse().ok()).unwrap_or(900);
    let h = std::env::var("VIEWER_H").ok().and_then(|v| v.parse().ok()).unwrap_or(700);
    print!("{}", render_scene(&files, w, h, &out));
}
