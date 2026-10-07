use crate::recovery::Recovery;

#[test]
fn only_device_loss_can_request_one_recovery_reload() {
    let mut policy = Recovery::default();
    assert!(!policy.reduced()); assert!(!policy.request(false));
    assert!(policy.request(true)); assert!(!policy.request(true));
    assert!(!policy.request(false));
}

#[test]
fn recovered_runs_refuse_repeated_reload_until_an_ordinary_reload() {
    let mut recovered = Recovery::adopted();
    assert!(recovered.reduced()); assert!(!recovered.request(true));
    let mut fresh = Recovery::default();
    assert!(!fresh.reduced()); assert!(fresh.request(true));
}
