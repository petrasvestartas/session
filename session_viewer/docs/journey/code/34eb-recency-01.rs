use crate::diagnostic::{Outcome, Report};

pub fn eligible(report: &Report, now: f64, tab: &str, parse: impl Fn(&str) -> f64) -> Option<f64> {
    if !report.valid() || !now.is_finite() || now < 0.0 { return None; }
    let started = parse(&report.started); let seen = parse(&report.last_seen);
    if !started.is_finite() || !seen.is_finite() || started > seen || seen > now { return None; }
    match report.outcome {
        Outcome::Failed => {
            let failed = parse(&report.failure()?.time);
            (failed.is_finite() && failed >= started && failed <= seen && now - failed < 7_200_000.0).then_some(failed)
        }
        Outcome::Running if now - seen < 7_200_000.0 && (report.tab == tab || now - seen > 120_000.0) => Some(seen),
        _ => None,
    }
}
