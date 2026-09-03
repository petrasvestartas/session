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
    // A .json/.toml argument is a SCENE MANIFEST, not a mesh: resolve it the way the browser does, so
    // what the harness renders is what the viewer renders. Without this a manifest's placements
    // are silently dropped and every file lands at its own native origin and scale - which is how
    // a 0.156-unit bunny turns into an invisible speck sitting on a 1000 mm box.
    let mut owned: Vec<(String, session_rust::Xform, f32, bool)> = Vec::new();
    for p in a.iter().skip(1) {
        if p.ends_with(".json") || p.ends_with(".toml") {
            let bytes = std::fs::read(p).unwrap_or_else(|e| panic!("cannot read manifest {p}: {e}"));
            let man = session_viewer::app::scene::Manifest::parse(&bytes)
                .unwrap_or_else(|| panic!("cannot parse manifest {p}"));
            // Manifest paths are relative to the assets root - the directory `pb/` sits in.
            // That is the manifest's OWN directory for `assets/view_local.toml`, and its
            // grandparent for the `assets/scenes/*.toml` layout this used to assume. Pick
            // whichever actually holds the files instead of hard-coding one shape, or moving a
            // manifest one level silently breaks every render with `could not read pb/...`.
            let here = std::path::Path::new(p).parent().unwrap_or(std::path::Path::new("."));
            let first = man.items.first().map(|i| i.file.clone()).unwrap_or_default();
            let root = [here.to_path_buf(), here.join("..")]
                .into_iter()
                .find(|r| r.join(&first).exists())
                .unwrap_or_else(|| here.to_path_buf());
            let count = man.items.len();
            for (i, item) in man.items.iter().enumerate() {
                let place = item.placement()
                    .unwrap_or_else(|| session_viewer::app::scene::auto_grid(i, count, [3000.0, 3000.0]));
                owned.push((root.join(&item.file).to_string_lossy().into_owned(), place, item.point_size as f32, item.display_only));
            }
        } else {
            owned.push((p.clone(), session_rust::Xform::identity(), 0.0, false));
        }
    }
    let files: Vec<(&str, session_rust::Xform, f32, bool)> =
        owned.iter().map(|(p, x, px, d)| (p.as_str(), x.clone(), *px, *d)).collect();
    print!("{}", session_viewer::selftest::render_scene(
        &files,
        std::env::var("VIEWER_W").ok().and_then(|v| v.parse().ok()).unwrap_or(900),
        std::env::var("VIEWER_H").ok().and_then(|v| v.parse().ok()).unwrap_or(700),
        &out,
    ));
}
