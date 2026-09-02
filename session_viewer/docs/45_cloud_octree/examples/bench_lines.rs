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
    let mut owned: Vec<(String, session_rust::Xform)> = Vec::new();
    for p in &a {
        if p.ends_with(".toml") {
            let bytes = std::fs::read(p).unwrap_or_else(|e| panic!("cannot read manifest {p}: {e}"));
            let man = session_viewer::app::scene::Manifest::parse(&bytes)
                .unwrap_or_else(|| panic!("cannot parse manifest {p}"));
            let root = std::path::Path::new(p).parent().and_then(|d| d.parent())
                .unwrap_or(std::path::Path::new(".")).to_path_buf();
            let count = man.items.len();
            for (i, item) in man.items.iter().enumerate() {
                let place = item.placement()
                    .unwrap_or_else(|| session_viewer::app::scene::auto_grid(i, count, [3000.0, 3000.0]));
                owned.push((root.join(&item.file).to_string_lossy().into_owned(), place));
            }
        } else {
            owned.push((p.clone(), session_rust::Xform::identity()));
        }
    }
    let files: Vec<(&str, session_rust::Xform)> =
        owned.iter().map(|(p, x)| (p.as_str(), x.clone())).collect();
    print!("{}", session_viewer::selftest::bench_scene(
        &files,
        std::env::var("VIEWER_W").ok().and_then(|v| v.parse().ok()).unwrap_or(1568),
        std::env::var("VIEWER_H").ok().and_then(|v| v.parse().ok()).unwrap_or(724),
    ));
}
