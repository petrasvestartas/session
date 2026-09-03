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
    let mut owned: Vec<(String, session_rust::Xform, f32, bool)> = Vec::new();
    for p in std::env::args().skip(1) {
        if p.ends_with(".json") || p.ends_with(".toml") {
            let bytes = std::fs::read(&p).unwrap_or_else(|e| panic!("cannot read manifest {p}: {e}"));
            let man = session_viewer::app::scene::Manifest::parse(&bytes)
                .unwrap_or_else(|| panic!("cannot parse manifest {p}"));
            let root = std::path::Path::new(&p).parent().and_then(|d| d.parent())
                .unwrap_or(std::path::Path::new(".")).to_path_buf();
            let count = man.items.len();
            for (i, item) in man.items.iter().enumerate() {
                let place = item.placement()
                    .unwrap_or_else(|| session_viewer::app::scene::auto_grid(i, count, [3000.0, 3000.0]));
                owned.push((root.join(&item.file).to_string_lossy().into_owned(), place, item.point_size as f32, item.display_only));
            }
        } else {
            owned.push((p, session_rust::Xform::identity(), 0.0, false));
        }
    }
    let files: Vec<(&str, session_rust::Xform, f32, bool)> =
        owned.iter().map(|(p, x, px, d)| (p.as_str(), x.clone(), *px, *d)).collect();
    print!("{}", session_viewer::selftest::frame_profile(
        &files,
        std::env::var("VIEWER_W").ok().and_then(|v| v.parse().ok()).unwrap_or(900),
        std::env::var("VIEWER_H").ok().and_then(|v| v.parse().ok()).unwrap_or(700),
    ));
}
