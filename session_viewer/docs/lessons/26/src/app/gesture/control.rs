use super::Gesture;
use crate::State;

/// The selected control point follows the pointer; a click leaves it where it is.
pub const GESTURE: Gesture = Gesture {
    name: "control",
    press: Some(|state, at, _| state.begin_control_drag(at.0, at.1)),
    start: None,
    drag: |state, at| state.drag_control(at.0, at.1),
    release,
};

/// Let go: a drag moves the point, a click puts it back.
fn release(state: &mut State, at: (f64, f64), click: bool) -> bool {
    if click {
        state.cancel_gesture();
        return true;
    }

    state.end_control_drag(at.0, at.1)
}
