// --8<-- [start:loft-recipe]
use crate::app::command::tool::gather::{self, Input, Made, Recipe, Step};
use crate::app::command::tool::surfacing::{self, checked, count, curves};
use crate::app::command::{Action, Spec};
use session_rust::{Point, Vector};

pub const SPEC: Spec = Spec {
    names: &["Loft"],
    aliases: &[],
    hint: "Loft (Open Closed): click curves in section order, Enter lofts · Example: select curves, then Loft Closed",
    options: &["Loft Open", "Loft Closed"],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

// Loft is a Recipe from lesson 23a, not a Shape: it gathers picked curves instead of answering questions with points.
pub static RECIPE: Recipe = Recipe {
    name: "Loft",
    chips: &[
        ("Open", "Open"),
        ("Closed", "Closed"),
        ("Finish", ""), // an empty line is Enter
        ("Cancel", "Escape"),
    ],
    steps: &[Step::Curves {
        prompt: "select curves in order",
        min: 2,
        max: usize::MAX, // no upper limit: Enter ends the picking
    }],
    axis: no_axis,
    build,
};

/// Start lofting; the selection in pick order gives the sections.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    gather::start(&RECIPE, rest)
}

/// Loft has no distance to slide.
pub fn no_axis(_input: &Input) -> (Point, Vector) {
    (Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0))
}
// --8<-- [end:loft-recipe]

// --8<-- [start:loft-build]
/// A cubic loft through the curves in pick order; Closed returns to the first.
fn build(input: &Input) -> Result<Made, String> {
    let sections = curves(&input.curves[0]);
    let surface = surfacing::loft(&sections, input.choice == "Closed", 3)?; // 3 = cubic across the sections
    Ok(Made {
        geometries: vec![checked(surface, "loft")?],
        message: format!(
            "Lofted {} into a NURBS surface",
            count(sections.len(), "curve")
        ),
    })
}
// --8<-- [end:loft-build]

// --8<-- [start:loft-tests]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::tool::surfacing::Picked;
    use crate::app::command::tool::surfacing::tests::{curve, p, square};
    use session_rust::{Geometry, NurbsCurve};

    /// The input of curves picked in this order.
    fn input(sections: Vec<NurbsCurve>, choice: &'static str) -> Input {
        Input {
            curves: vec![
                sections
                    .into_iter()
                    .map(|curve| Picked {
                        curve,
                        corners: None,
                    })
                    .collect(),
            ],
            answers: Vec::new(),
            choice,
            normal: Vector::new(0.0, 0.0, 1.0),
        }
    }

    /// Three arcs give a surface through the middle one.
    #[test]
    fn three_arcs_loft_through_the_middle_section() {
        let arcs = vec![
            curve(&[p(0., 0., 0.), p(50., 0., 20.), p(100., 0., 0.)]),
            curve(&[p(0., 50., 10.), p(50., 50., 40.), p(100., 50., 10.)]),
            curve(&[p(0., 100., 0.), p(50., 100., 20.), p(100., 100., 0.)]),
        ];
        let middle = arcs[1].point_at(0.5);
        let made = build(&input(arcs, "Open")).unwrap();
        let Geometry::NurbsSurface(surface) = &made.geometries[0] else {
            panic!()
        };
        assert!(surface.is_valid());
        assert_eq!(surface.cv_count(1), 3);
        assert!(surface.closest_point(&middle).distance(&middle, None) < 1e-6);
    }

    /// Closed returns to the first section and needs three.
    #[test]
    fn closed_loops_back_and_needs_three() {
        let squares = vec![square(20.0, 0.0), square(20.0, 30.0), square(20.0, 15.0)];
        let made = build(&input(squares.clone(), "Closed")).unwrap();
        let Geometry::NurbsSurface(surface) = &made.geometries[0] else {
            panic!()
        };
        assert!(surface.is_closed(1));
        assert!(surface.is_closed(0));
        assert!(build(&input(squares[..2].to_vec(), "Closed")).is_err());
        assert!(build(&input(squares[..1].to_vec(), "Open")).is_err());
    }
}
// --8<-- [end:loft-tests]
