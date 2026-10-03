use crate::{diagnostic::Outcome, diagnostic_shape_tests::report};

#[test]
fn final_close_keeps_failure_and_observations() {
    for outcome in [Outcome::Running, Outcome::Ready, Outcome::Closed, Outcome::Failed] {
        let mut value = report();
        if outcome == Outcome::Failed { value.record("first".into(), 1.0, "fatal", "original failure").unwrap(); }
        value.outcome = outcome; let mut expected = value.clone(); expected.last_seen = "closed".into();
        if expected.failure().is_none() { expected.outcome = Outcome::Closed; }
        value.close("closed".into()).unwrap(); assert_eq!(value, expected); assert!(value.valid());
        value.close("closed".into()).unwrap(); assert_eq!(value, expected);
    }
}

#[test]
fn close_rejects_invalid_time_atomically() {
    let mut value = report(); let before = value.clone();
    for bad in [String::new(), "x".repeat(65)] { assert!(value.close(bad).is_err()); assert_eq!(value, before); }
}

#[test]
fn late_ready_milestone_cannot_reopen_final_run() {
    let mut value = report(); value.close("closed".into()).unwrap();
    value.record("later".into(), 1.0, "milestone", "geometry on screen").unwrap();
    assert_eq!(value.outcome, Outcome::Closed); assert!(value.valid());
}
