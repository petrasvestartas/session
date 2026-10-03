use crate::{diagnostic::Report, diagnostic_shape_tests::report, report_store::{decode, MAX_BYTES}};

#[test]
fn stored_json_rejects_invalid_shape() {
    let good = serde_json::to_value(report()).unwrap();
    for (key, bad) in [("version", serde_json::json!(2)), ("tab", serde_json::json!("")),
        ("page", serde_json::json!("https://example.test/?private=1")), ("devicePixelRatio", serde_json::json!(0)),
        ("events", serde_json::json!([{"time":"now","kind":"fatal","message":"bad","elapsedMs":null}])),
        ("outcome", serde_json::json!("failed")), ("unknown", serde_json::json!(true))] {
        let mut value = good.clone(); value[key] = bad;
        assert!(decode(&value.to_string()).is_none(), "{key}");
    }
    assert!(decode("{").is_none());
}

#[test]
fn stored_json_bounds_bytes_events_and_text() {
    let json = serde_json::to_string(&report()).unwrap();
    let padding = MAX_BYTES - json.len(); let padded = json + &" ".repeat(padding);
    assert!(decode(&padded).is_some()); assert!(decode(&(padded + " ")).is_none());
    let mut value = serde_json::to_value(report()).unwrap();
    let event = serde_json::json!({"time":"now","kind":"phase","message":"test","elapsedMs":1.0});
    value["events"] = serde_json::json!(vec![event; 25]); assert!(decode(&value.to_string()).is_none());
    let mut bypass: Report = serde_json::from_value(value).unwrap();
    bypass.record("later".into(), 2.0, "phase", "bounded").unwrap();
    assert_eq!(serde_json::to_value(bypass).unwrap()["events"].as_array().unwrap().len(), 24);
    let mut failure = report(); failure.record("later".into(), 1.0, "fatal", "first reason").unwrap();
    let json = serde_json::to_string(&failure).unwrap(); assert_eq!(decode(&json), Some(failure));
    let mut value: serde_json::Value = serde_json::from_str(&json).unwrap();
    value["failure"]["message"] = serde_json::json!("😀".repeat(4097)); assert!(decode(&value.to_string()).is_none());
}

#[test]
fn unsupported_telemetry_can_be_retained_without_adoption() {
    let mut value = serde_json::to_value(report()).unwrap(); value["lastSeen"] = serde_json::json!("seen");
    value["futureTelemetry"] = serde_json::json!({"resources":["keep original"]});
    for version in [1, 2] {
        value["version"] = serde_json::json!(version); let text = value.to_string(); let before = text.clone();
        assert!(decode(&text).is_none());
        assert_eq!(crate::report_store::retention_time(&text, 2000.0, |_| 1000.0), Some(1000.0));
        assert_eq!(text, before);
    }
}

#[test]
fn retention_headers_reject_invalid_bounds_and_clocks() {
    let text = serde_json::to_string(&report()).unwrap();
    let retain = |text: &str, now: f64, parsed: f64| crate::report_store::retention_time(text, now, |_| parsed);
    for now in [f64::NAN, f64::INFINITY, -1.0] { assert!(retain(&text, now, 0.0).is_none()); }
    for seen in [f64::NAN, f64::INFINITY, 2001.0] { assert!(retain(&text, 2000.0, seen).is_none()); }
    for bad in ["{", "{}", r#"{"version":0,"lastSeen":"seen"}"#] { assert!(retain(bad, 2000.0, 0.0).is_none()); }
    let mut value = serde_json::to_value(report()).unwrap(); value["lastSeen"] = serde_json::json!("x".repeat(65));
    assert!(retain(&value.to_string(), 2000.0, 0.0).is_none());
    let padded = text.clone() + &" ".repeat(MAX_BYTES - text.len());
    assert!(retain(&padded, 2000.0, 0.0).is_some()); assert!(retain(&(padded + " "), 2000.0, 0.0).is_none());
}
