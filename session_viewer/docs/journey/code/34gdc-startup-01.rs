pub fn now() -> f64 {
    web_sys::window().and_then(|window| window.performance()).map_or(0.0, |clock| clock.now())
}

pub fn finish(name: &str, started: f64, bytes: u64, source: &str) {
    let elapsed_ms = now();
    let phase = crate::load_phase::Phase { name: name.chars().take(64).collect(),
        duration_ms: elapsed_ms - started, bytes,
        source: source.split(['?', '#']).next().unwrap_or("").chars().take(1024).collect(), elapsed_ms };
    let _ = crate::browser_report::phase(phase);
}
