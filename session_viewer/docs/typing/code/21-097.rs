    pub(super) control_drag: Option<edit::ControlDrag>, // register:control_drag
    pub(crate) gizmo: Option<Gizmo>,  // register:gizmo
    pub(super) dragging: Option<edit::GizmoDrag>, // register:gizmo_drag
    pub(super) object_drag: Option<drag::ObjectDrag>, // register:object_drag
    pub(crate) snap: Snapping,        // register:snap
