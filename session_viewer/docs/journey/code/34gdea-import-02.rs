pub fn completed(phase: crate::load_measure::Completed<'_>, source: &str) {
    let name = if phase.succeeded { phase.name.to_owned() } else { format!("{} failed", phase.name) };
    let _ = crate::browser_report::phase(crate::load_phase::Phase {
        name, duration_ms: phase.duration_ms, elapsed_ms: phase.elapsed_ms, bytes: phase.bytes,
        source: source.to_owned(),
    });
}

pub fn finish(name: &str, started: f64, bytes: u64, source: &str) {