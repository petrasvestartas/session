// --8<-- [start:distance-parse]
use crate::State;
use crate::app::command::tool::{Next, Tool};
use crate::app::command::verbs::measure::{to_text, unit_suffix};
use crate::app::command::{Action, Spec};
use crate::app::coords;
use session_rust::{Plane, Point};

pub const SPEC: Spec = Spec {
    names: &["Measure Distance"],
    aliases: &[],
    hint: "Measure Distance · pick two points, snaps on · or type Measure Distance 0,0,0 100,0,0",
    options: &[],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

const USAGE: &str = "Measure Distance takes two points: Measure Distance 0,0,0 100,0,0";

/// Two world points answer at once; fewer, or relative ones, are picked.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    if rest.len() > 2 {
        return Err(USAGE.into());
    }

    let mut points = Vec::with_capacity(2);

    for word in rest {
        let Some(typed) = coords::parse(word) else {
            return Err(format!("`{word}` is not a point; {USAGE}"));
        };

        // A relative point such as `@3,4` needs the point before it, so only two absolute points answer at once.
        if let coords::Typed::Absolute { x, y, z } = typed {
            points.push([x, y, z.unwrap_or(0.0)]);
        }
    }

    match points.as_slice() {
        [a, b] => Ok(Box::new(Distance(*a, *b))),
        _ => Ok(Box::new(Pick(rest.join(" ")))),
    }
}
// --8<-- [end:distance-parse]

// --8<-- [start:distance-actions]
/// The distance between two typed world points.
#[derive(Debug)]
// A tuple struct: its fields have no names and are read as `self.0` and `self.1`.
struct Distance([f64; 3], [f64; 3]);

impl Action for Distance {
    fn run(&self, state: &mut State) -> Result<String, String> {
        let [a, b] = [self.0, self.1].map(|[x, y, z]| Point::new(x, y, z));
        Ok(answer(state, &a, &b))
    }
}

/// Pick the points; typed ones go in first.
#[derive(Debug)]
struct Pick(String);

impl Action for Pick {
    fn run(&self, state: &mut State) -> Result<String, String> {
        let prompt = state.open_tool(Box::new(Measuring))?;

        if self.0.is_empty() {
            return Ok(prompt);
        }

        let answer = state.run_command(&self.0);

        if answer.is_err() {
            state.cancel_drawing();
        }

        answer
    }
}
// --8<-- [end:distance-actions]

// --8<-- [start:measuring-tool]
/// Asks for two points, then answers with their distance.
#[derive(Debug)]
struct Measuring;

impl Tool for Measuring {
    fn name(&self) -> &'static str {
        "Measure Distance"
    }

    fn prompt(&self, points: &[Point]) -> String {
        match points.len() {
            0 => "First point".into(),
            _ => "Second point".into(),
        }
    }

    fn readout(&self, points: &[Point], cursor: &Point, _plane: &Plane) -> String {
        points
            .first()
            .map(|first| to_text(first.distance(cursor, None)))
            .unwrap_or_default()
    }

    fn placed(
        &mut self,
        state: &mut State,
        points: &[Point],
        _plane: &Plane,
    ) -> Result<Next, String> {
        match points {
            // Clicks arrive already snapped, so two clicked line ends measure exactly.
            [a, b] => Ok(Next::Done(answer(state, a, b))),
            _ => Ok(Next::More),
        }
    }

    /// Enter before any point cancels; after one it asks for the second.
    fn enter(&mut self, _state: &mut State, points: &[Point]) -> Result<Next, String> {
        match points.len() {
            0 => Ok(Next::Done("Measure Distance cancelled".into())),
            _ => Err(USAGE.into()),
        }
    }
}
// --8<-- [end:measuring-tool]

// --8<-- [start:distance-answer]
/// `Distance 141.421 mm · dx 100 · dy 100 · dz 0`, drawn between the points until the next command.
fn answer(state: &mut State, a: &Point, b: &Point) -> String {
    let (distance, delta) = compute_distance(a, b);
    let unit = unit_suffix(state, 1);
    let value = format!("{} {unit}", to_text(distance));
    state.set_mark(vec![a.clone(), b.clone()], value.clone());
    format!(
        "Distance {value} · dx {} · dy {} · dz {}",
        to_text(delta[0]),
        to_text(delta[1]),
        to_text(delta[2])
    )
}

/// The distance from `a` to `b` and the signed steps b - a.
fn compute_distance(a: &Point, b: &Point) -> (f64, [f64; 3]) {
    let delta = b - a;
    (delta.magnitude(), [delta[0], delta[1], delta[2]])
}
// --8<-- [end:distance-answer]

// --8<-- [start:distance-tests]
#[cfg(test)]
mod tests {
    use super::*;

    fn parsed(line: &str) -> Result<String, String> {
        crate::app::command::parse(line).map(|action| format!("{action:?}"))
    }

    #[test]
    fn two_points_answer_and_fewer_are_picked() {
        let (distance, delta) =
            compute_distance(&Point::new(0.0, 0.0, 0.0), &Point::new(3.0, 4.0, 0.0));
        assert_eq!(distance, 5.0);
        assert_eq!(delta, [3.0, 4.0, 0.0]);
        assert_eq!(
            parsed("Measure Distance 0,0,0 3,4,0"),
            Ok("Distance([0.0, 0.0, 0.0], [3.0, 4.0, 0.0])".into())
        );
        assert_eq!(
            parsed("measuredistance 0,0 3,4"),
            Ok("Distance([0.0, 0.0, 0.0], [3.0, 4.0, 0.0])".into())
        );
        assert_eq!(parsed("Measure Distance"), Ok("Pick(\"\")".into()));
        assert_eq!(
            parsed("measure distance 5,5,0"),
            Ok("Pick(\"5,5,0\")".into())
        );
        assert_eq!(
            parsed("Measure Distance 0,0,0 @3,4,0"),
            Ok("Pick(\"0,0,0 @3,4,0\")".into())
        );
        assert!(parsed("Measure Distance 1 2 3").is_err());
        assert!(parsed("Measure Distance a b").is_err());
    }

    /// The analysis verbs complete in one Enter and take no arguments but points.
    #[test]
    fn analysis_verbs_complete_and_parse() {
        use crate::app::command::accept;
        assert_eq!(accept("mea"), ("Measure Distance".into(), true));
        assert_eq!(accept("len"), ("Length".into(), true));
        assert_eq!(accept("are"), ("Area".into(), true));
        assert_eq!(accept("vol"), ("Volume".into(), true));
        assert_eq!(accept("Ar"), ("Arctic ".into(), false)); // register:arctic
        assert_eq!(accept("Lin"), ("Line".into(), true));
        assert_eq!(
            crate::app::command::completions("m")[..2],
            ["Move", "Measure Distance"]
        );
        assert_eq!(accept("m").0, "Move");
        assert_eq!(parsed("length"), Ok("Length".into()));
        assert_eq!(parsed("VOLUME"), Ok("Volume".into()));
        assert!(parsed("Area 2").is_err());
    }
}
// --8<-- [end:distance-tests]
