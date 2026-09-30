// --8<-- [start:gesture-table]

use crate::State;

// Gesture = one left-button tool, e.g. dragging an object; each lives in its own file beside this one.
/// One left-button tool: which presses it takes, how it follows the pointer, how it ends.
pub struct Gesture {
    pub name: &'static str, // what tests call it
    // `Option<fn ...>`: a tool without this step leaves it None and never takes such a press
    pub press: Option<fn(&mut State, (f64, f64), f64) -> bool>, // takes a press on its target, reach in CSS px
    pub start: Option<fn(&mut State, (f64, f64), (f64, f64)) -> bool>, // takes a plain press once it leaves the click slop
    pub drag: fn(&mut State, (f64, f64)) -> bool, // the pointer moved; true redraws
    pub release: fn(&mut State, (f64, f64), bool) -> bool, // let go, true when it stayed a click; true redraws
}

/// Every left-button tool, tried in this order.
// Empty at lesson 12; lesson 21 adds the control, gizmo and object drags, one line each.
pub const GESTURES: &[&Gesture] = &[
];
// --8<-- [end:gesture-table]

// --8<-- [start:gesture-find]
/// The first tool that takes a press at `at`.
pub fn press(state: &mut State, at: (f64, f64), reach: f64) -> Option<&'static Gesture> {
    GESTURES
        .iter()
        .copied()
        .find(|gesture| gesture.press.is_some_and(|take| take(state, at, reach)))
}

/// The first tool that takes a plain press at `down` dragged to `at`.
pub fn start(state: &mut State, down: (f64, f64), at: (f64, f64)) -> Option<&'static Gesture> {
    GESTURES
        .iter()
        .copied()
        .find(|gesture| gesture.start.is_some_and(|take| take(state, down, at)))
}
// --8<-- [end:gesture-find]
