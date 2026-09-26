// --8<-- [start:arrowhead-parse]
use crate::State;
use crate::app::command::{Action, Spec};
use session_rust::{Arrowhead, Geometry};
use std::rc::Rc;

pub const SPEC: Spec = Spec {
    names: &["Arrowhead"],
    aliases: &[],
    hint: "Arrowhead: heads on the selected lines, polylines and curves · None, Start, End or Both · Example: Arrowhead End",
    options: &[
        "Arrowhead None",
        "Arrowhead Start",
        "Arrowhead End",
        "Arrowhead Both",
    ],
    arity: Some(1), // exactly one word after the name
    wait_for_option: true,
    wait_after_option: false,
    parse,
};

/// Each option's word and the heads it sets.
const HEADS: [(&str, Arrowhead); 4] = [
    ("None", Arrowhead::NONE),
    ("Start", Arrowhead::START),
    ("End", Arrowhead::END),
    ("Both", Arrowhead::BOTH),
];

/// None, Start, End or Both.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    HEADS
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(rest[0]))
        .map(|&(name, head)| Box::new(Heads { name, head }) as Box<dyn Action>)
        .ok_or_else(|| "try Arrowhead None, Start, End or Both".into())
}
// --8<-- [end:arrowhead-parse]

// --8<-- [start:heads]
/// Set the arrowheads of every selected curve.
#[derive(Debug)]
struct Heads {
    name: &'static str, // the option as shown
    head: Arrowhead,    // the heads to set
}

impl Action for Heads {
    /// Replace every selected curve whose heads differ, in one undo step.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let rows = state.selected_rows();

        // A streamed file, or a released document still loading, refuses the whole command before anything changes.
        if let Some(reason) = state.locked_reason(&rows) {
            return Err(reason);
        }

        let mut curves = 0;
        let mut edits = Vec::new();

        for &row in &rows {
            let Some((geometry, changed)) =
                state.scene.geometry(row).and_then(|g| headed(g, self.head))
            else {
                continue;
            };
            curves += 1;

            if changed {
                edits.push((row, geometry));
            }
        }

        if curves == 0 {
            return Err("Arrowhead needs a selected line, polyline or curve".into());
        }

        let count = if edits.is_empty() {
            0
        } else {
            // Every changed curve is swapped in one document edit, so a single Undo takes all the heads off.
            state.scene.replace_rows(edits, "arrowhead")?
        };
        state.commit_rows();
        let plural = if curves == 1 { "" } else { "s" };
        Ok(format!(
            "Arrowhead {} on {curves} curve{plural}, {count} changed",
            self.name
        ))
    }

    fn needs_selection(&self) -> bool {
        true
    }
}
// --8<-- [end:heads]

// --8<-- [start:headed]
/// The curve with `head` and whether that changed it; None when it is no curve.
fn headed(geometry: &Geometry, head: Arrowhead) -> Option<(Geometry, bool)> {
    let out = match geometry {
        Geometry::Line(line) => {
            // `**line` goes through the reference and the `Rc` to the Line itself, so `clone` copies the Line, not the pointer.
            let mut next = (**line).clone();
            next.arrowhead = head;
            (Geometry::Line(Rc::new(next)), line.arrowhead != head)
        }
        Geometry::Polyline(polyline) => {
            let mut next = (**polyline).clone();
            next.arrowhead = head;
            (
                Geometry::Polyline(Rc::new(next)),
                polyline.arrowhead != head,
            )
        }
        Geometry::NurbsCurve(curve) => {
            let mut next = (**curve).clone();
            next.arrowhead = head;
            (Geometry::NurbsCurve(Rc::new(next)), curve.arrowhead != head)
        }
        _ => return None,
    };
    Some(out)
}
// --8<-- [end:headed]

// --8<-- [start:arrowhead-tests]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::scene::{FileDoc, Scene};
    use session_rust::{Line, NurbsCurve, Point, Polyline, Session, Xform};

    /// The four options parse in any case; other words are refused.
    #[test]
    fn options_parse() {
        let parsed = |word: &str| parse("Arrowhead", &[word]).map(|action| format!("{action:?}"));
        assert!(parsed("both").unwrap().contains("BOTH"));
        assert!(parsed("START").unwrap().contains("START"));
        assert!(parsed("None").unwrap().contains("NONE"));
        assert!(parsed("End").unwrap().contains("END"));
        assert!(parsed("sideways").is_err());
        assert!(crate::app::command::parse("arrow head end").is_ok());
        assert!(crate::app::command::parse("Arrowhead").is_err());
    }

    /// Lines, polylines and curves take the heads; other kinds are left alone.
    #[test]
    fn only_curves_take_heads() {
        let p = |x: f64, y: f64| Point::new(x, y, 0.0);
        let line = Geometry::Line(Rc::new(Line::from_points(&p(0.0, 0.0), &p(1.0, 0.0))));
        let polyline = Geometry::Polyline(Rc::new(Polyline::new(vec![p(0.0, 0.0), p(1.0, 1.0)])));
        let curve = Geometry::NurbsCurve(Rc::new(NurbsCurve::create(
            false,
            2,
            &[p(0.0, 0.0), p(1.0, 1.0), p(2.0, 0.0)],
        )));

        for geometry in [line, polyline, curve] {
            let Some((out, true)) = headed(&geometry, Arrowhead::BOTH) else {
                panic!("a curve takes heads")
            };
            let head = match &out {
                Geometry::Line(l) => l.arrowhead,
                Geometry::Polyline(pl) => pl.arrowhead,
                Geometry::NurbsCurve(c) => c.arrowhead,
                _ => unreachable!(),
            };
            assert_eq!(head, Arrowhead::BOTH);
            assert!(
                matches!(headed(&out, Arrowhead::BOTH), Some((_, false))),
                "the same heads change nothing"
            );
        }

        assert!(headed(&Geometry::Point(Rc::new(p(0.0, 0.0))), Arrowhead::END).is_none());
    }

    /// Setting heads is one undo step; undo takes them off, redo puts them back.
    #[test]
    fn heads_undo_and_redo() {
        let mut session = Session::new("test");
        let polyline = Polyline::new(vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(10.0, 10.0, 0.0),
        ]);
        session.add_polyline(polyline, None).unwrap();
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "test".into(),
            session: Rc::new(session),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        let heads = |scene: &Scene| match scene.geometry(0) {
            Some(Geometry::Polyline(pl)) => pl.arrowhead,
            _ => panic!("the polyline"),
        };
        let (edit, _) = headed(scene.geometry(0).unwrap(), Arrowhead::END).unwrap();
        assert_eq!(scene.replace_rows(vec![(0, edit)], "arrowhead"), Ok(1));
        scene.sync();
        assert_eq!(heads(&scene), Arrowhead::END);
        assert!(scene.undo());
        scene.sync();
        assert_eq!(heads(&scene), Arrowhead::NONE);
        assert!(scene.redo());
        scene.sync();
        assert_eq!(heads(&scene), Arrowhead::END);
    }
}
// --8<-- [end:arrowhead-tests]
