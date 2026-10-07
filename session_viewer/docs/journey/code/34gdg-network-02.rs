use wasm_bindgen::JsCast;

pub fn source(url: &str) -> String {
    let Ok(value) = web_sys::Url::new(url) else { return "unavailable URL".into(); };
    let _ = value.set_username(""); let _ = value.set_password("");
    value.set_search(""); value.set_hash("");
    value.href().chars().take(1024).collect()
}

pub fn measured(url: &str, started: f64, succeeded: bool, bytes: u64) -> Vec<crate::load_phase::Phase> {
    let ended = crate::browser_phase::now();
    let source = source(url);
    let mut phases = vec![crate::load_phase::Phase {
        name: if succeeded { "source fetch" } else { "source fetch failed" }.into(),
        duration_ms: ended - started, elapsed_ms: ended, bytes, source: source.clone(),
    }];
    if let Some(clock) = web_sys::window().and_then(|window| window.performance()) {
        for entry in clock.get_entries_by_name(url).iter().rev() {
            let Ok(entry) = entry.dyn_into::<web_sys::PerformanceResourceTiming>() else { continue; };
            if entry.start_time() < started || entry.response_end() > ended { continue; }
            phases.push(crate::load_phase::Phase { name: "network response".into(),
                duration_ms: entry.response_end() - entry.fetch_start(), elapsed_ms: entry.response_end(),
                bytes: entry.transfer_size() as u64, source });
            break;
        }
    }
    phases
}
