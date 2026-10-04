use crate::{adapter_info::AdapterInfo, diagnostic_shape_tests::report};

fn identity() -> AdapterInfo {
    AdapterInfo { vendor: "vendor".into(), architecture: "family".into(), device: "model".into(), description: "GPU".into() }
}

#[test]
fn identity_survives_event_rotation_and_failure() {
    let mut report = report(); let info = identity(); report.set_adapter(info.clone()).unwrap();
    report.record("first".into(), 1.0, "fatal", "original failure").unwrap();
    let first = report.failure().unwrap().clone();
    for i in 0..30 { report.record("later".into(), i as f64 + 2.0, "phase", "later activity").unwrap(); }
    let value = serde_json::to_value(&report).unwrap();
    assert_eq!(value["events"].as_array().unwrap().len(), 24);
    assert_eq!(value["adapter"], serde_json::to_value(info).unwrap());
    assert_eq!(report.failure(), Some(&first));
}

#[test]
fn old_reports_and_new_identity_round_trip() {
    let mut report = report(); let old = serde_json::to_string(&report).unwrap();
    assert!(serde_json::to_value(&report).unwrap().get("adapter").is_none());
    assert_eq!(crate::report_store::decode(&old), Some(report.clone()));
    report.set_adapter(identity()).unwrap();
    assert_eq!(crate::report_store::decode(&serde_json::to_string(&report).unwrap()), Some(report.clone()));
    let mut value = serde_json::to_value(report).unwrap(); value["adapter"]["unknown"] = 1.into();
    assert!(crate::report_store::decode(&value.to_string()).is_none());
}

#[test]
fn invalid_identity_is_atomic_and_empty_fields_are_valid() {
    let mut report = report(); let before = report.clone();
    for field in ["vendor", "architecture", "device", "description"] {
        let mut value = serde_json::to_value(identity()).unwrap(); value[field] = "😀".repeat(257).into();
        let invalid: AdapterInfo = serde_json::from_value(value).unwrap();
        assert!(report.set_adapter(invalid).is_err()); assert_eq!(report, before);
    }
    let mut info = identity(); info.vendor = "😀".repeat(256); info.architecture.clear();
    info.device.clear(); info.description.clear(); assert!(info.valid());
    report.set_adapter(info).unwrap(); assert!(report.valid());
}
