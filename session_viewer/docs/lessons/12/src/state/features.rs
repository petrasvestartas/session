use super::State;

/// What each feature keeps between frames; a feature adds its own file and one line here.
#[derive(Default)]
pub(crate) struct Features {
}

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
