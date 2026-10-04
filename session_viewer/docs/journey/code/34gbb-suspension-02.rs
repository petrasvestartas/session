use crate::suspension::{Reason::{Cached, Frozen, Hidden}, Suspension};

#[test]
fn every_remaining_reason_prevents_resumption() {
    for order in [[Hidden, Frozen, Cached], [Cached, Hidden, Frozen], [Frozen, Cached, Hidden]] {
        let mut state = Suspension::new(false);
        assert!(state.permits());
        for reason in order { assert!(!state.change(reason, true)); }
        assert!(!state.change(order[0], false));
        assert!(!state.change(order[1], false));
        assert!(state.change(order[2], false));
    }
}

#[test]
fn initial_hidden_and_duplicate_events_remain_paused() {
    let mut state = Suspension::new(true);
    assert!(!state.permits());
    assert!(!state.change(Frozen, true));
    assert!(!state.change(Frozen, true));
    assert!(!state.change(Frozen, false));
    assert!(!state.change(Frozen, false));
    assert!(state.change(Hidden, false));
}

#[test]
fn final_closure_cannot_be_resumed() {
    let mut state = Suspension::new(true);
    state.change(Frozen, true); state.change(Cached, true);
    state.close(); state.close();
    for reason in [Hidden, Frozen, Cached] { assert!(!state.change(reason, false)); }
    assert!(!state.permits());
}
