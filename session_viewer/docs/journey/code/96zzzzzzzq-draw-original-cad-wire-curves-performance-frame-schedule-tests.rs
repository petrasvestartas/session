use super::Schedule;

#[test]
fn bursts_keep_ordered_state_and_only_present_the_latest_after_gpu_completion() {
    let mut frames = Schedule::after_submit(true);
    let mut camera = 0;
    for delta in [1, 2, -4, 8] {
        camera += delta;
        assert_eq!(frames.request(), None);
    }
    assert_eq!(camera, 7);
    let first = frames.complete(0).unwrap();
    for delta in [3, -1, 9] {
        camera += delta;
        assert_eq!(frames.request(), None);
    }
    assert!(frames.begin(first));
    assert_eq!(camera, 18);
    assert!(!frames.begin(first));
    for _ in 0..100 { camera += 1; assert_eq!(frames.request(), None); }
    assert_eq!(frames.complete(first + 1), None);
    let second = frames.complete(first).unwrap();
    assert_eq!(frames.complete(first), None);
    assert!(frames.begin(second));
    assert_eq!(camera, 118);
    assert_eq!(frames.complete(second), None);
    assert_eq!(frames.complete(second), None);
}

#[test]
fn suspension_revokes_queued_callbacks_and_preserves_one_dirty_frame() {
    let mut frames = Schedule::after_submit(true);
    assert_eq!(frames.complete(0), None);
    let old = frames.request().unwrap();
    assert_eq!(frames.allow(false), None);
    assert!(!frames.begin(old));
    for _ in 0..100 { assert_eq!(frames.request(), None); }
    let current = frames.allow(true).unwrap();
    assert_ne!(old, current);
    assert!(!frames.begin(old));
    assert!(frames.begin(current));
    assert_eq!(frames.request(), None);
    assert_eq!(frames.allow(false), None);
    assert_eq!(frames.complete(current), None);
    let resumed = frames.allow(true).unwrap();
    assert!(frames.begin(resumed));
    assert_eq!(frames.complete(resumed), None);
}

#[test]
fn disposal_and_ticket_exhaustion_never_revive_a_frame() {
    let mut frames = Schedule::after_submit(true);
    frames.complete(0);
    let ticket = frames.request().unwrap();
    frames.close();
    assert!(!frames.begin(ticket));
    assert_eq!(frames.complete(ticket), None);
    assert_eq!(frames.allow(true), None);
    assert_eq!(frames.request(), None);
    let mut exhausted = Schedule::after_submit(true);
    exhausted.complete(0);
    exhausted.next = u64::MAX;
    assert_eq!(exhausted.request(), None);
    assert_eq!(exhausted.allow(true), None);
    assert!(!exhausted.begin(0));
}
