use super::Gesture;

/// A plain mouse drag on an object moves it, or the selection it belongs to.
pub const GESTURE: Gesture = Gesture {
    name: "object",
    press: None, // a finger on an object orbits
    start: Some(|state, down, at| state.start_object_drag(down, at)),
    drag: |state, at| state.drag_object(at),
    release: |state, _, _| state.end_object_drag(),
};
