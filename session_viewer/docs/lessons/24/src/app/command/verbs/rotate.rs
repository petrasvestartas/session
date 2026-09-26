use crate::State;
use crate::app::command::tool::{Next, Tool, typed_number};
use crate::app::command::{Action, Spec, axis_and_number};
use crate::app::gizmo::Axis;
use session_rust::{Plane, Point, Xform};

pub const SPEC: Spec = Spec {
    names: &["Rotate"],
    aliases: &["rot"],
    hint: "Rotate · pick the center, then type an angle or pick two reference points · Rotate z 45 turns about a world axis",
    options: &["Rotate x", "Rotate y", "Rotate z"],
    arity: None,
    wait_for_option: false,
    wait_after_option: true,
    parse,
};

/// Pick a center and an angle, or turn the selection about a typed world axis.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    if rest.is_empty() {
        return Ok(Box::new(Rotating));
    }

    let (axis, degrees) = axis_and_number(rest, "Rotate x 90")?;

    if rest.len() > 2 {
        return Err("wrong number of arguments for `Rotate`".into());
    }

    Ok(Box::new(Rotate { axis, degrees }))
}

#[derive(Debug)]
struct Rotate {
    axis: Axis,
    degrees: f64,
}

impl Action for Rotate {
    /// Turn about the gizmo centre, or the world origin without one.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let about = state.features.gizmo.as_ref().map(|g| g.origin.clone()); // turn about the gizmo
        let turn = crate::state::edit::rotation_about(self.axis, self.degrees, about.as_ref());
        state.apply(turn, "rotate")
    }

    fn needs_selection(&self) -> bool {
        true
    }
}

/// Turn in the construction plane about a picked center; the selection follows the cursor.
#[derive(Clone, Debug)]
struct Rotating;

impl Action for Rotating {
    /// Start asking for the points.
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.start_tool(Box::new(self.clone()))
    }

    fn needs_selection(&self) -> bool {
        true
    }
}

impl Tool for Rotating {
    fn name(&self) -> &'static str {
        "Rotate"
    }

    fn prompt(&self, points: &[Point]) -> String {
        match points.len() {
            0 => "Center of rotation".into(),
            1 => "Angle or first reference point".into(),
            _ => "Second reference point or angle".into(),
        }
    }

    /// A typed angle turns at once, counter-clockwise as seen from the viewer.
    fn word(
        &mut self,
        state: &mut State,
        word: &str,
        points: &[Point],
        plane: &Plane,
    ) -> Option<Result<Next, String>> {
        let center = points.first()?;
        let degrees = typed_number(word)?;
        Some(commit(
            state,
            &rotation(center, degrees.to_radians(), plane),
        ))
    }

    fn preview(&self, points: &[Point], cursor: &Point, plane: &Plane) -> Option<Xform> {
        let [center, from] = points else {
            return None;
        };
        Some(rotation(center, angle(center, from, cursor, plane)?, plane))
    }

    fn readout(&self, points: &[Point], cursor: &Point, plane: &Plane) -> String {
        match points {
            [center, from] => angle(center, from, cursor, plane)
                .map(|turn| format!("{:.1}°", turn.to_degrees()))
                .unwrap_or_default(),
            _ => String::new(),
        }
    }

    /// The first reference point sets zero, the second the angle.
    fn placed(
        &mut self,
        state: &mut State,
        points: &[Point],
        plane: &Plane,
    ) -> Result<Next, String> {
        match points {
            [center, from] if angle(center, from, from, plane).is_none() => {
                Err("The reference point must not be on the center".into())
            }
            [center, from, to] => {
                let turn = angle(center, from, to, plane)
                    .ok_or("The second point must not be on the center")?;
                commit(state, &rotation(center, turn, plane))
            }
            _ => Ok(Next::More),
        }
    }
}

/// Turn the selection in one undo step.
fn commit(state: &mut State, turn: &Xform) -> Result<Next, String> {
    state.apply(turn.clone(), "rotate").map(Next::Done)
}

/// The signed angle, radians, from `from` to `to` about `center` in the plane; None when either lies on the center.
fn angle(center: &Point, from: &Point, to: &Point, plane: &Plane) -> Option<f64> {
    let (x, y) = (plane.x_axis(), plane.y_axis());
    let (a, b) = (from - center, to - center);
    let (ax, ay, bx, by) = (a.dot(&x), a.dot(&y), b.dot(&x), b.dot(&y));

    if ax.hypot(ay) <= 1e-12 || bx.hypot(by) <= 1e-12 {
        return None;
    }

    Some((ax * by - ay * bx).atan2(ax * bx + ay * by))
}

/// A turn of `radians` about the plane normal through `center`.
fn rotation(center: &Point, radians: f64, plane: &Plane) -> Xform {
    let to = Xform::translation(center[0], center[1], center[2]);
    let back = Xform::translation(-center[0], -center[1], -center[2]);
    &(&to * &Xform::rotation(&plane.z_axis(), radians, false)) * &back
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::Vector;

    /// A construction plane from its two axes.
    fn plane(x: [f64; 3], y: [f64; 3]) -> Plane {
        Plane::new(
            Point::new(0.0, 0.0, 0.0),
            Vector::new(x[0], x[1], x[2]),
            Vector::new(y[0], y[1], y[2]),
        )
    }

    /// Where the transform puts a point.
    fn moved(xform: &Xform, p: [f64; 3]) -> [f64; 3] {
        let q = xform.transform_point(&Point::new(p[0], p[1], p[2]));
        [q[0], q[1], q[2]]
    }

    /// Close to within 1e-9.
    fn near(a: [f64; 3], b: [f64; 3]) -> bool {
        (0..3).all(|i| (a[i] - b[i]).abs() < 1e-9)
    }

    /// Counter-clockwise is positive in Top and in Front.
    #[test]
    fn a_turn_is_counter_clockwise_in_the_construction_plane() {
        let origin = Point::new(0.0, 0.0, 0.0);
        let top = plane([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        let turn = angle(
            &origin,
            &Point::new(10.0, 0.0, 0.0),
            &Point::new(0.0, 10.0, 0.0),
            &top,
        )
        .unwrap();
        assert!((turn.to_degrees() - 90.0).abs() < 1e-9);
        let front = plane([1.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let turn = angle(
            &origin,
            &Point::new(10.0, 0.0, 0.0),
            &Point::new(0.0, 0.0, 10.0),
            &front,
        )
        .unwrap();
        assert!((turn.to_degrees() - 90.0).abs() < 1e-9);
        assert!(near(
            moved(&rotation(&origin, turn, &front), [1.0, 0.0, 0.0]),
            [0.0, 0.0, 1.0]
        ));
        let quarter = rotation(&Point::new(5.0, 5.0, 0.0), 90f64.to_radians(), &top);
        assert!(near(moved(&quarter, [6.0, 5.0, 0.0]), [5.0, 6.0, 0.0]));
        assert!(angle(&origin, &origin, &Point::new(1.0, 0.0, 0.0), &top).is_none());
        assert!(
            angle(
                &origin,
                &Point::new(0.0, 0.0, 3.0),
                &Point::new(1.0, 0.0, 0.0),
                &top
            )
            .is_none(),
            "straight above the center"
        );
    }

    /// The live turn follows the cursor from the first reference point.
    #[test]
    fn the_preview_turns_from_the_reference_to_the_cursor() {
        let top = plane([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        let points = [Point::new(0.0, 0.0, 0.0), Point::new(10.0, 0.0, 0.0)];
        let turn = Rotating
            .preview(&points, &Point::new(0.0, 10.0, 0.0), &top)
            .unwrap();
        assert!(near(moved(&turn, [10.0, 0.0, 0.0]), [0.0, 10.0, 0.0]));
        assert!(
            Rotating
                .preview(&points[..1], &Point::new(0.0, 10.0, 0.0), &top)
                .is_none()
        );
        assert_eq!(
            Rotating.readout(&points, &Point::new(0.0, 10.0, 0.0), &top),
            "90.0°"
        );
    }
}
