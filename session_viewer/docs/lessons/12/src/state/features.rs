// --8<-- [start:features-struct]
use super::State;

/// What each feature keeps between frames; a feature adds its own file and one line here.
// Every field starts from its Default, so `State::new` never names one.
#[derive(Default)]
pub(crate) struct Features {
}
// --8<-- [end:features-struct]

// --8<-- [start:features-hooks]
// Each list starts empty; a later lesson adds one line per hook.
// `fn(&mut State)` is a function pointer; a method such as `State::purge_idle` is one, with `self` as its first argument.
/// Feature work on every frame, before the pick answers are applied.
pub(super) const BEFORE_PICKS: &[fn(&mut State)] = &[
];

/// Feature work on every frame, once the pick answers are applied.
pub(super) const AFTER_PICKS: &[fn(&mut State)] = &[
];

/// Features that take a pick answer before the selection does, in this order.
pub(super) const TAKE_PICK: &[fn(&mut State, Option<crate::engine::gpu::Pick>) -> bool] = &[
];

/// Features that widen what a viewport click on a row selects, e.g. to its whole group.
pub(super) const CLICK_ROWS: &[fn(&State, u32) -> Option<Vec<u32>>] = &[
];

impl State {
    /// Feature work before the pick answers are applied.
    pub(super) fn before_picks(&mut self) {
        for hook in BEFORE_PICKS {
            hook(self);
        }
    }

    /// Feature work once the pick answers are applied.
    pub(super) fn after_picks(&mut self) {
        for hook in AFTER_PICKS {
            hook(self);
        }
    }
}
// --8<-- [end:features-hooks]
