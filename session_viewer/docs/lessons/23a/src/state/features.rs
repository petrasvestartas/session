// --8<-- [start:features-struct]
use super::State;
use super::drag; // register:object_drag
use super::drawing; // register:drawing
use super::edit; // register:gizmo_drag
use super::hydrate; // register:hydrate
use crate::app::gizmo::Gizmo; // register:gizmo
use crate::app::snap::Snapping; // register:snap

/// What each feature keeps between frames; a feature adds its own file and one line here.
// Every field starts from its Default, so `State::new` never names one.
#[derive(Default)]
pub(crate) struct Features {
    pub(super) cloud_query: Option<crate::app::cloud_query::Query>, // a point-cloud pick in flight; register:cloud_query
    #[cfg(target_arch = "wasm32")] // register:cloud_query
    pub(super) query_generation: u64, // counts cloud queries, old answers dropped; register:cloud_query
    pub(super) sheet_query: Option<crate::app::sheet_query::Query>, // a sheet pick in flight; register:sheets
    pub(super) sheet_generation: u64, // counts sheet queries, old answers dropped; register:sheets
    pub(super) resume: Vec<hydrate::Resume>, // register:hydrate
    pub(super) clip_hidden: usize,    // register:clipping
    pub(super) control_drag: Option<edit::ControlDrag>, // register:control_drag
    pub(crate) gizmo: Option<Gizmo>,  // register:gizmo
    pub(super) dragging: Option<edit::GizmoDrag>, // register:gizmo_drag
    pub(super) object_drag: Option<drag::ObjectDrag>, // register:object_drag
    pub(crate) draft: Option<drawing::Draft>, // register:drawing
    pub(crate) snap: Snapping,        // register:snap
}
// --8<-- [end:features-struct]

// --8<-- [start:features-hooks]
// Each list starts empty; a later lesson adds one line per hook.
// `fn(&mut State)` is a function pointer; a method such as `State::purge_idle` is one, with `self` as its first argument.
/// Feature work on every frame, before the pick answers are applied.
pub(super) const BEFORE_PICKS: &[fn(&mut State)] = &[
    State::catch_unsynced,   // register:editing
    State::fetch_if_wanting, // register:editing
    State::update_clipping,  // register:clipping
];

/// Feature work on every frame, once the pick answers are applied.
pub(super) const AFTER_PICKS: &[fn(&mut State)] = &[
    State::purge_idle, // register:hydrate
];

/// Features that take a pick answer before the selection does, in this order.
pub(super) const TAKE_PICK: &[fn(&mut State, Option<crate::engine::gpu::Pick>) -> bool] = &[
    State::take_drag_pick,  // register:editing
    State::take_tool_pick,  // register:tools
    #[cfg(target_arch = "wasm32")] // register:cloud_query
    State::take_cloud_pick, // register:cloud_query
];

/// Features that widen what a viewport click on a row selects, e.g. to its whole group.
pub(super) const CLICK_ROWS: &[fn(&State, u32) -> Option<Vec<u32>>] = &[
    State::group_of, // register:editing
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
