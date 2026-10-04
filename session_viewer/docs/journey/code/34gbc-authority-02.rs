use crate::startup::Startup;

#[test]
fn close_revokes_every_waiting_ticket() {
    let owner = Startup::default();
    let adapter = owner.clone(); let device = owner.clone();
    assert!(adapter.permits()); assert!(device.permits());
    owner.revoke();
    assert!(!adapter.permits()); assert!(!device.permits());
    assert!(!owner.clone().permits());
}

#[test]
fn repeated_revocation_never_restores_permission() {
    let owner = Startup::default(); let waiting = owner.clone();
    waiting.revoke(); owner.revoke();
    assert!(!owner.permits()); assert!(!waiting.permits());
}

#[test]
fn a_replacement_has_its_own_permission() {
    let previous = Startup::default(); let replacement = Startup::default();
    previous.revoke();
    assert!(!previous.permits()); assert!(replacement.permits());
    replacement.revoke(); assert!(!replacement.permits());
}
