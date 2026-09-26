use crate::State;
use crate::app::command::tool::{Next, Tool, translation};
use crate::app::command::{Action, Spec, offset};
use session_rust::{Plane, Point, Xform};

pub const SPEC: Spec = Spec {
    names: &["Move"],
    aliases: &["m"],
    hint: "Move · pick a base point, then the target point · Move 10,0,0 moves by a typed offset",
    options: &[],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// Pick a base and a target point, or shift the selection by a typed offset.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    if rest.is_empty() {
        return Ok(Box::new(Moving));
    }

    Ok(Box::new(Move(offset(rest)?)))
}

#[derive(Debug)]
struct Move([f64; 3]);

impl Action for Move {
    /// Translate the selection and record it in history.
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.apply(Xform::translation(self.0[0], self.0[1], self.0[2]), "move")
    }

    fn needs_selection(&self) -> bool {
        true
    }
}

/// Move by two picked points; the selection follows the cursor.
#[derive(Clone, Debug)]
struct Moving;

impl Action for Moving {
    /// Start asking for the points.
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.start_tool(Box::new(self.clone()))
    }

    fn needs_selection(&self) -> bool {
        true
    }
}

impl Tool for Moving {
    fn name(&self) -> &'static str {
        "Move"
    }

    fn prompt(&self, points: &[Point]) -> String {
        match points.len() {
            0 => "Point to move from".into(),
            _ => "Point to move to, or a distance along the band".into(),
        }
    }

    fn preview(&self, points: &[Point], cursor: &Point, _plane: &Plane) -> Option<Xform> {
        Some(translation(points.first()?, cursor))
    }

    fn readout(&self, points: &[Point], cursor: &Point, _plane: &Plane) -> String {
        points
            .first()
            .map(|base| format!("{:.3}", base.distance(cursor, None)))
            .unwrap_or_default()
    }

    /// The second point moves the selection in one undo step.
    fn placed(
        &mut self,
        state: &mut State,
        points: &[Point],
        _plane: &Plane,
    ) -> Result<Next, String> {
        let [from, to] = points else {
            return Ok(Next::More);
        };
        state.apply(translation(from, to), "move").map(Next::Done)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The selection follows the cursor from the base point; the band and readout say how far.
    #[test]
    fn moving_previews_the_offset_from_the_base_point() {
        let plane = Plane::new(
            Point::new(0.0, 0.0, 0.0),
            session_rust::Vector::new(1.0, 0.0, 0.0),
            session_rust::Vector::new(0.0, 1.0, 0.0),
        );
        let base = [Point::new(0.0, 0.0, 0.0)];
        let cursor = Point::new(10.0, 5.0, 0.0);
        let delta = Moving.preview(&base, &cursor, &plane).unwrap();
        assert_eq!([delta.m[12], delta.m[13], delta.m[14]], [10.0, 5.0, 0.0]);
        assert!(Moving.preview(&[], &cursor, &plane).is_none());
        assert_eq!(Moving.guide(&base, Some(&cursor)).len(), 2);
        assert_eq!(
            Moving.readout(&base, &cursor, &plane),
            format!("{:.3}", 125f64.sqrt())
        );
        assert_eq!(Moving.prompt(&[]), "Point to move from");
        assert!(Moving.prompt(&base).starts_with("Point to move to"));
    }
}
