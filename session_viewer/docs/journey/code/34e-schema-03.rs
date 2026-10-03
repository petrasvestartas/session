use crate::diagnostic::{Context, Report};

pub(crate) fn report() -> Report {
    Report::new("tab".into(), "2026-10-03T18:00:00.000Z".into(), Context {
        page: "https://example.test/".into(), browser: "Test".into(), secure_context: true,
        webgpu: true, viewport: [900, 760], canvas: [900, 760], device_pixel_ratio: 1.0 })
}

#[test]
fn shape_rejects_bad_metadata() {
    let good = serde_json::to_value(report()).unwrap();
    for (key, bad) in [("version", serde_json::json!(2)), ("tab", serde_json::json!("")),
        ("page", serde_json::json!("https://example.test/?private=1")), ("devicePixelRatio", serde_json::json!(0)),
        ("outcome", serde_json::json!("failed"))] {
        let mut value = good.clone(); value[key] = bad;
        assert!(!serde_json::from_value::<Report>(value).unwrap().valid(), "{key}");
    }
    assert!(report().valid());
}

#[test]
fn shape_bounds_events_and_failure() {
    let mut failure = report(); failure.record("later".into(), 1.0, "fatal", "first").unwrap();
    assert!(failure.valid());
    let mut value = serde_json::to_value(failure).unwrap();
    let event = serde_json::json!({"time":"now","kind":"phase","message":"test","elapsedMs":1.0});
    value["events"] = serde_json::json!(vec![event; 25]);
    let oversized: Report = serde_json::from_value(value.clone()).unwrap(); assert!(!oversized.valid());
    value["events"] = serde_json::json!([]); value["failure"]["message"] = serde_json::json!("😀".repeat(4097));
    assert!(!serde_json::from_value::<Report>(value.clone()).unwrap().valid());
    value["failure"]["message"] = serde_json::json!("first"); value["failure"]["kind"] = serde_json::json!("phase");
    assert!(!serde_json::from_value::<Report>(value).unwrap().valid());
}
