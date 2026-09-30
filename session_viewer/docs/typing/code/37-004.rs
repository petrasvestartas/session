//! Render one headless frame of the given scenes and print the ink count.

use session_viewer::selftest::{SceneFile, render_scene};

/// A logger that prints every record to stderr.
struct StderrLog;

impl log::Log for StderrLog {
    /// Every level is on.
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    /// Print one record.
    fn log(&self, r: &log::Record) {
        eprintln!("[{}] {}", r.level(), r.args());
    }

    /// Nothing is buffered.
    fn flush(&self) {}
}

/// Parse `<out.ppm> <scenes>...` and the VIEWER_W / VIEWER_H size, then render.
fn main() {
    let _ = log::set_logger(&StderrLog);
    log::set_max_level(log::LevelFilter::Info);
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out = args.first().cloned().unwrap_or_else(|| "out.ppm".into());
    let files = SceneFile::from_args(&args[1.min(args.len())..]);
    let w = std::env::var("VIEWER_W")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(900);
    let h = std::env::var("VIEWER_H")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(700);
    print!("{}", render_scene(&files, w, h, &out));
}
