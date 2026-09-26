use super::{State, drag, drawing, edit, hydrate, splitting};
use crate::app::command::verbs::measure::Mark;
use crate::app::gizmo::Gizmo;
use crate::app::hierarchy::Hierarchy;
use crate::app::snap::Snapping;

/// What each feature keeps between frames; a feature adds its own file and one line here.
#[derive(Default)]
pub(crate) struct Features {
    pub(super) resume: Vec<hydrate::Resume>, // register:hydrate
    pub(super) clip_hidden: usize,           // register:clipping
    pub(super) control_drag: Option<edit::ControlDrag>, // register:control_drag
    pub(crate) gizmo: Option<Gizmo>,         // register:gizmo
    pub(super) dragging: Option<edit::GizmoDrag>, // register:gizmo_drag
    pub(super) object_drag: Option<drag::ObjectDrag>, // register:object_drag
    pub(crate) draft: Option<drawing::Draft>, // register:drawing
    pub(crate) snap: Snapping,               // register:snap
    pub(crate) mark: Option<Mark>,           // register:measure
    pub(super) hierarchy: Hierarchy,         // register:hierarchy
    pub(super) pending_split: Option<splitting::Pending>, // register:split
    pub(super) opacity_chosen: bool,         // register:opacity
}

/// Feature work on every frame, before the pick answers are applied.
pub(super) const BEFORE_PICKS: &[fn(&mut State)] = &[
    State::update_clipping, // register:clipping
];

/// Feature work on every frame, once the pick answers are applied.
pub(super) const AFTER_PICKS: &[fn(&mut State)] = &[
    State::purge_idle, // register:hydrate
];
