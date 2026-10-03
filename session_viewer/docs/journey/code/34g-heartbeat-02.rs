use crate::{diagnostic::Outcome, diagnostic_shape_tests::report};

#[test]
fn heartbeat_preserves_everything_except_last_seen() {
    for outcome in [Outcome::Running, Outcome::Ready, Outcome::Closed, Outcome::Failed] {
        let mut value = report();
        if outcome == Outcome::Failed { value.record("first".into(), 1.0, "fatal", "original failure").unwrap(); }
        value.outcome = outcome; let mut expected = value.clone(); expected.last_seen = "heartbeat".into();
        value.heartbeat("heartbeat".into()).unwrap(); assert_eq!(value, expected); assert!(value.valid());
    }
}

#[test]
fn heartbeat_rejects_invalid_text_without_mutation() {
    let mut value = report(); let before = value.clone();
    for bad in [String::new(), "x".repeat(65)] { assert!(value.heartbeat(bad).is_err()); assert_eq!(value, before); }
    value.heartbeat("x".repeat(64)).unwrap(); assert!(value.valid());
}
