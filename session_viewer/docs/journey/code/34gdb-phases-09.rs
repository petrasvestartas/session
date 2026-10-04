use crate::{load_phase::Phase, diagnostic_shape_tests::report};

fn phase(i: usize) -> Phase {
    Phase { name: format!("decode {i}"), duration_ms: 2.0, bytes: 100,
        source: "scene.pb".into(), elapsed_ms: i as f64 + 2.0 }
}

#[test]
fn retention_survives_event_rotation_and_counts_evictions() {
    let mut report = report();
    for i in 0..270 { report.phase("later".into(), phase(i)).unwrap(); }
    let value = serde_json::to_value(&report).unwrap();
    assert_eq!(value["phases"].as_array().unwrap().len(), 256);
    assert_eq!(value["phases"][0]["name"], "decode 14");
    assert_eq!(value["phasesDropped"], 14);
    for i in 0..30 { report.record("later".into(), i as f64, "activity", "camera").unwrap(); }
    assert_eq!(serde_json::to_value(&report).unwrap()["phases"], value["phases"]);
    assert!(report.valid());
}

#[test]
fn bad_phases_are_refused_without_mutation() {
    let mut report = report(); let before = report.clone();
    for change in 0..9 {
        let mut invalid = phase(0);
        match change {
            0 => invalid.name.clear(), 1 => invalid.name = "x".repeat(65),
            2 => invalid.source = "😀".repeat(1025), 3 => invalid.source = "file?secret".into(),
            4 => invalid.source = "file#secret".into(), 5 => invalid.duration_ms = -1.0,
            6 => invalid.duration_ms = f64::NAN, 7 => invalid.elapsed_ms = f64::INFINITY,
            _ => invalid.elapsed_ms = 1.0,
        }
        assert!(report.phase("later".into(), invalid).is_err()); assert_eq!(report, before);
    }
    for time in [String::new(), "x".repeat(65)] {
        assert!(report.phase(time, phase(0)).is_err()); assert_eq!(report, before);
    }
    let mut unicode = phase(0); unicode.source = "😀".repeat(1024);
    report.phase("later".into(), unicode).unwrap();
}

#[test]
fn old_reports_decode_and_nested_shape_is_strict() {
    let mut report = report(); let old = serde_json::to_string(&report).unwrap();
    assert!(serde_json::to_value(&report).unwrap().get("phases").is_none());
    assert_eq!(crate::report_store::decode(&old), Some(report.clone()));
    report.phase("later".into(), phase(0)).unwrap();
    let text = serde_json::to_string_pretty(&report).unwrap();
    assert_eq!(crate::report_store::decode(&text), Some(report.clone()));
    let mut value = serde_json::to_value(report).unwrap(); value["phases"][0]["unknown"] = true.into();
    assert!(crate::report_store::decode(&value.to_string()).is_none());
    value["phases"][0].as_object_mut().unwrap().remove("unknown");
    value["phases"] = serde_json::json!(vec![phase(0); 257]);
    assert!(crate::report_store::decode(&value.to_string()).is_none());
}

#[test]
fn encoded_budget_preserves_first_failure_and_recent_events() {
    let mut report = report();
    for i in 0..256 {
        let mut measured = phase(i); measured.name = "\u{0001}".repeat(64);
        measured.source = "\u{0001}".repeat(1024); report.phase("later".into(), measured).unwrap();
    }
    let before = serde_json::to_value(&report).unwrap()["phasesDropped"].as_u64().unwrap();
    assert!(before > 0);
    for i in 0..30 { report.record("later".into(), i as f64, if i == 0 { "fatal" } else { "activity" }, &"\u{0001}".repeat(4096)).unwrap(); }
    let failure = report.failure().unwrap().clone();
    let mut context = report.context.clone(); context.page = "\u{0001}".repeat(4096); context.browser = "\u{0001}".repeat(4096);
    report.refresh_context(context); report.heartbeat("x".repeat(64)).unwrap();
    let text = serde_json::to_string_pretty(&report).unwrap();
    assert!(text.len() <= crate::report_store::MAX_BYTES); assert!(report.valid());
    assert_eq!(crate::report_store::decode(&text), Some(report.clone()));
    let value = serde_json::to_value(&report).unwrap();
    assert_eq!(value["events"].as_array().unwrap().len(), 24);
    assert!(value["phasesDropped"].as_u64().unwrap() > before);
    assert_eq!(report.failure(), Some(&failure));
}
