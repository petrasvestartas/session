use crate::gpu_fault::Fault;

#[test]
fn cloned_callback_keeps_the_first_reason() {
    let fault = Fault::default(); let callback = fault.clone();
    assert!(fault.message().is_none());
    assert!(callback.same_device(&fault));
    assert!(callback.remember("device lost: original reason".into()));
    assert!(!fault.remember("validation error after loss".into()));
    assert_eq!(fault.message().as_deref(), Some("device lost: original reason"));
}

#[test]
fn replacement_device_has_an_independent_fault() {
    let old = Fault::default(); let replacement = Fault::default();
    assert!(!old.same_device(&replacement));
    assert!(old.remember("late failure from old device".into()));
    assert!(replacement.message().is_none());
    assert!(replacement.remember("new device failed".into()));
    assert_eq!(old.message().as_deref(), Some("late failure from old device"));
}

#[test]
fn concurrent_callbacks_accept_exactly_one_reason() {
    let fault = Fault::default(); let a = fault.clone(); let b = fault.clone();
    let a = std::thread::spawn(move || a.remember("first contender".into()));
    let b = std::thread::spawn(move || b.remember("second contender".into()));
    assert_ne!(a.join().unwrap(), b.join().unwrap());
    assert!(matches!(fault.message().as_deref(), Some("first contender" | "second contender")));
}
