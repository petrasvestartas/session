use super::Gesture;
use crate::State;

/// A gumball handle drags the selection; a click opens its number box.
pub const GESTURE: Gesture = Gesture {
    name: "gizmo",
    press: Some(|state, at, reach| state.begin_gizmo_with_radius(at.0, at.1, reach)),
    start: None,
    drag: |state, at| state.drag_gizmo(at.0, at.1),
    release,
};

/// Let go: a drag records one undo step, a click asks for a number instead.
fn release(state: &mut State, at: (f64, f64), click: bool) -> bool {
    if click {
        return state.click_gizmo();
    }

    state.end_gizmo(at.0, at.1)
}
