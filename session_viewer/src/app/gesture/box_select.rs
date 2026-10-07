use super::Gesture;

/// A left drag on empty canvas, or with Shift or Ctrl held, draws a selection rectangle.
pub const GESTURE: Gesture = Gesture {
    name: "box",
    press: None, // only a drag past the click slop draws a rectangle
    start: Some(|state, down, at| state.start_box(down, at)),
    drag: |state, at| state.drag_box(at),
    release: |state, _, _| state.end_box(),
};
