
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
