// cargo run --example bench_frame --target x86_64-unknown-linux-gnu --release -- <scene.toml | file.pb>...
//
// Median frame time for a still and a moving camera. BENCH_FRAMES=N frames per leg;
// VIEWER_LINE_STYLE=tubes|flat picks the solid-lane style; VIEWER_W / VIEWER_H size it.

use session_viewer::selftest::{frame_profile, SceneFile};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let files = SceneFile::from_args(&args);
    let w = std::env::var("VIEWER_W").ok().and_then(|v| v.parse().ok()).unwrap_or(900);
    let h = std::env::var("VIEWER_H").ok().and_then(|v| v.parse().ok()).unwrap_or(700);
    print!("{}", frame_profile(&files, w, h));
}
