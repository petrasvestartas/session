use crate::diagnostic::{Context, Outcome, Report};

fn context() -> Context {
    Context { page: "https://example.test/viewer/".into(), browser: "Test browser".into(),
        secure_context: true, webgpu: true, viewport: [900, 760], canvas: [900, 760], device_pixel_ratio: 1.0 }
}

#[test]
fn report_starts_running_with_flat_browser_context() {
    let report = Report::new("tab-a".into(), "2026-10-03T18:00:00.000Z".into(), context());
    let value = serde_json::to_value(&report).unwrap();
    assert_eq!(value["version"], 1); assert_eq!(value["outcome"], "running");
    assert_eq!(value["started"], value["lastSeen"]);
    assert_eq!(value["secureContext"], true); assert_eq!(value["devicePixelRatio"], 1.0);
    assert_eq!(value["viewport"], serde_json::json!([900, 760]));
    assert!(value.get("context").is_none());
    assert_eq!(serde_json::from_value::<Report>(value).unwrap(), report);
}

#[test]
fn report_snapshot_keeps_its_own_context() {
    let mut live = Report::new("tab-a".into(), "start".into(), context()); let snapshot = live.clone();
    live.context.canvas = [618, 1373]; live.context.device_pixel_ratio = 2.625;
    live.outcome = Outcome::Ready;
    assert_eq!(snapshot.context.canvas, [900, 760]); assert_eq!(snapshot.outcome, Outcome::Running);
    assert_eq!(serde_json::from_str::<Report>(&serde_json::to_string(&live).unwrap()).unwrap(), live);
}


#[test]
fn first_failure_survives_later_events() {
    let mut report = Report::new("tab".into(), "start".into(), context());
    report.record("first".into(), 1.0, "fatal", "device lost").unwrap();
    let first = report.failure().unwrap().clone();
    for i in 0..30 { report.record(format!("late-{i}"), i as f64 + 2.0, "fatal", &format!("later-{i}")).unwrap(); }
    assert_eq!(report.failure(), Some(&first));
    let value = serde_json::to_value(&report).unwrap();
    assert_eq!(value["events"].as_array().unwrap().len(), 24);
    assert_eq!(value["events"][0]["message"], "later-6");
    report.record("drawn".into(), 40.0, "milestone", "geometry on screen").unwrap();
    assert_eq!(report.outcome, Outcome::Failed); assert_eq!(report.failure(), Some(&first));
    assert_eq!(serde_json::from_str::<Report>(&serde_json::to_string(&report).unwrap()).unwrap(), report);
}

#[test]
fn invalid_time_is_atomic_and_text_is_bounded() {
    let mut report = Report::new("tab".into(), "start".into(), context());
    report.record("drawn".into(), 1.0, "milestone", "geometry on screen").unwrap();
    assert_eq!(report.outcome, Outcome::Ready);
    report.record("later".into(), 2.0, &"x".repeat(65), &"😀".repeat(4097)).unwrap();
    let value = serde_json::to_value(&report).unwrap();
    assert_eq!(value["events"][1]["kind"].as_str().unwrap().chars().count(), 64);
    assert_eq!(value["events"][1]["message"].as_str().unwrap().chars().count(), 4096);
    let before = report.clone();
    for invalid in [f64::NAN, f64::INFINITY, -1.0] { assert!(report.record("bad".into(), invalid, "fatal", "bad time").is_err()); }
    assert_eq!(report, before);
}
