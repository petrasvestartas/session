mod control;
mod gizmo;
mod object;

use crate::State;

/// One left-button tool: which presses it takes, how it follows the pointer, how it ends.
pub struct Gesture {
    pub name: &'static str, // what tests call it
    pub press: Option<fn(&mut State, (f64, f64), f64) -> bool>, // takes a press on its target, reach in CSS px
    pub start: Option<fn(&mut State, (f64, f64), (f64, f64)) -> bool>, // takes a plain press once it leaves the click slop
    pub drag: fn(&mut State, (f64, f64)) -> bool, // the pointer moved; true redraws
    pub release: fn(&mut State, (f64, f64), bool) -> bool, // let go, true when it stayed a click; true redraws
}

/// Every left-button tool, tried in this order.
pub const GESTURES: &[&Gesture] = &[
    &control::GESTURE, // register:control-drag
    &gizmo::GESTURE,   // register:gizmo-drag
    &object::GESTURE,  // register:object-drag
];

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

#[cfg(test)]
mod tests {
    use super::*;

    /// Controls, then handles, then objects; a finger never starts an object drag.
    #[test]
    fn gestures_are_tried_control_then_gizmo_then_object() {
        let names: Vec<_> = GESTURES.iter().map(|gesture| gesture.name).collect();
        assert_eq!(names, ["control", "gizmo", "object"]);
        let pressed: Vec<_> = GESTURES.iter().map(|g| g.press.is_some()).collect();
        assert_eq!(
            pressed,
            [true, true, false],
            "object drags start only past the slop"
        );
        let started: Vec<_> = GESTURES.iter().map(|g| g.start.is_some()).collect();
        assert_eq!(started, [false, false, true]);
    }
}
