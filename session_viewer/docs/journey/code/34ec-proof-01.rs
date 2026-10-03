use crate::{diagnostic::Outcome, diagnostic_shape_tests::report, report_recency::eligible};

#[test]
fn previous_run_selection_respects_actual_failure_time() {
    let now = 1_000_000_000.0;
    for (name, failed, same, seen_age, failure_age, expected) in [
        ("old failure, fresh heartbeat", true, false, 5000.0, 259_200_000.0, false),
        ("recent failure", true, false, 5000.0, 10000.0, true),
        ("failure at cutoff", true, false, 5000.0, 7_200_000.0, false),
        ("failure after heartbeat", true, false, 5000.0, 1000.0, false),
        ("future heartbeat", true, false, -1.0, 1000.0, false),
        ("future failure", true, false, 0.0, -1.0, false),
        ("active other tab", false, false, 120_000.0, 0.0, false),
        ("interrupted other tab", false, false, 120_001.0, 0.0, true),
        ("stale other tab", false, false, 7_200_000.0, 0.0, false),
        ("interrupted same tab", false, true, 0.0, 0.0, true),
    ] {
        let mut value = report(); value.started = "start".into();
        if failed { value.record("failed".into(), 1.0, "fatal", "original").unwrap(); }
        value.last_seen = "seen".into(); value.tab = if same { "current" } else { "other" }.into();
        let parser = |time: &str| match time { "start" => now - 259_200_000.0, "seen" => now - seen_age,
            "failed" => now - failure_age, _ => f64::NAN };
        let selected = eligible(&value, now, "current", parser);
        assert_eq!(selected.is_some(), expected, "{name}");
        if expected { assert_eq!(selected, Some(now - if failed { failure_age } else { seen_age })); }
    }
}

#[test]
fn invalid_clocks_and_healthy_runs_are_not_interruption_evidence() {
    let mut value = report(); value.started = "start".into(); value.last_seen = "seen".into();
    let clock = |time: &str| if time == "start" { 0.0 } else { 1000.0 };
    for now in [f64::NAN, f64::INFINITY, -1.0] { assert!(eligible(&value, now, "tab", clock).is_none()); }
    assert!(eligible(&value, 2000.0, "tab", |_| f64::NAN).is_none());
    assert!(eligible(&value, 2000.0, "tab", |time| if time == "start" { 3000.0 } else { 1000.0 }).is_none());
    for state in [Outcome::Ready, Outcome::Closed] { value.outcome = state; assert!(eligible(&value, 2000.0, "tab", clock).is_none()); }
}

#[test]
fn failure_before_start_is_rejected_without_mutating_report() {
    let mut value = report(); value.record("failed".into(), 1.0, "fatal", "original").unwrap();
    value.started = "start".into(); value.last_seen = "seen".into(); let before = value.clone();
    let clock = |time: &str| match time { "start" => 1500.0, "seen" => 1900.0, "failed" => 1000.0, _ => f64::NAN };
    assert!(eligible(&value, 2000.0, "tab", clock).is_none()); assert_eq!(value, before);
    assert!(eligible(&value, 2000.0, "tab", |time| if time == "failed" { f64::NAN } else { clock(time) }).is_none());
}
