pub fn revision(scene: &crate::scene::Scene) {
    for version in crate::revision::versions(scene) {
        let _ = crate::browser_report::observe("revision", &version);
    }
}

pub fn completed(phase: crate::load_measure::Completed<'_>, source: &str) {