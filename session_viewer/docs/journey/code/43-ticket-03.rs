use super::*;

#[test]
fn only_the_latest_request_in_the_current_epoch_can_be_accepted_once() {
    let mut gate = Gate::default(); let old = gate.issue().unwrap(); let latest = gate.issue().unwrap();
    assert!(!gate.accept(old)); assert!(gate.accept(latest)); assert!(!gate.accept(latest));
    let before_edit = gate.issue().unwrap(); gate.invalidate(); assert!(!gate.accept(before_edit));
    let after_edit = gate.issue().unwrap(); assert_ne!(before_edit, after_edit); assert!(gate.accept(after_edit));
    let pending = gate.issue().unwrap(); gate.close(); assert!(!gate.accept(pending)); assert!(gate.issue().is_err());
}

#[test]
fn exhausted_tickets_close_without_wrapping_to_an_old_identity() {
    let mut gate = Gate::default(); gate.request = u64::MAX - 1;
    let last = gate.issue().unwrap(); assert!(gate.issue().is_err()); assert!(!gate.accept(last));
    let mut gate = Gate::default(); gate.epoch = u64::MAX; let last = gate.issue().unwrap();
    gate.invalidate(); assert!(!gate.accept(last)); assert!(gate.issue().is_err());
}
