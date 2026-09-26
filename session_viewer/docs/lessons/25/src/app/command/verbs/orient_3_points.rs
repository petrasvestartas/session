use crate::State;
use crate::app::command::tool::{Next, Tool, translation};
use crate::app::command::{Action, Spec};
use crate::app::coords;
use session_rust::{Plane, Point, Vector, Xform};

pub const SPEC: Spec = Spec {
    names: &["Orient 3 Points"],
    aliases: &["orient3pt"],
    hint: "Orient 3 Points · pick three reference points, then three target points · the selection moves rigidly",
    options: &[],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// Pick the points, or orient at once by six typed x,y,z points.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    if rest.is_empty() {
        return Ok(Box::new(Orienting));
    }

    let points = rest
        .iter()
        .map(|word| match coords::parse(word) {
            Some(coords::Typed::Absolute { x, y, z }) => Ok([x, y, z.unwrap_or(0.0)]),
            _ => Err(
                "Orient 3 Points takes six x,y,z points: three references, then three targets"
                    .to_string(),
            ),
        })
        .collect::<Result<Vec<_>, _>>()?;
    let points: [[f64; 3]; 6] = points.try_into().map_err(
        |_| "Orient 3 Points takes six x,y,z points: three references, then three targets",
    )?;
    Ok(Box::new(Orient(points)))
}

#[derive(Debug)]
struct Orient([[f64; 3]; 6]);

impl Action for Orient {
    /// Orient the selection by the typed points in one undo step.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let p: Vec<Point> = self
            .0
            .iter()
            .map(|p| Point::new(p[0], p[1], p[2]))
            .collect();
        let xform =
            orient(&p[..3], &p[3..]).ok_or("The reference or target points are in a line")?;
        state.apply(xform, "orient")
    }

    fn needs_selection(&self) -> bool {
        true
    }
}

/// Rigidly move three reference points onto three target points; the selection follows the cursor.
#[derive(Clone, Debug)]
struct Orienting;

impl Action for Orienting {
    /// Start asking for the points.
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.start_tool(Box::new(self.clone()))
    }

    fn needs_selection(&self) -> bool {
        true
    }
}

impl Tool for Orienting {
    fn name(&self) -> &'static str {
        "Orient 3 Points"
    }

    fn prompt(&self, points: &[Point]) -> String {
        match points.len() {
            0 => "Point to orient from".into(),
            1 => "Second reference point".into(),
            2 => "Third reference point".into(),
            3 => "Point to orient to".into(),
            4 => "Second target point".into(),
            _ => "Third target point".into(),
        }
    }

    /// Slide after the first target, turn onto the second, then the full orientation.
    fn preview(&self, points: &[Point], cursor: &Point, _plane: &Plane) -> Option<Xform> {
        match points {
            [r0, _, _] => Some(translation(r0, cursor)),
            [r0, r1, _, t0] => Some(align(r0, r1, t0, cursor)),
            [r0, r1, r2, t0, t1] => orient(
                &[r0.clone(), r1.clone(), r2.clone()],
                &[t0.clone(), t1.clone(), cursor.clone()],
            )
            .or_else(|| Some(align(r0, r1, t0, t1))),
            _ => None,
        }
    }

    /// Two points must differ and three must not be in a line; the sixth orients the selection.
    fn placed(
        &mut self,
        state: &mut State,
        points: &[Point],
        _plane: &Plane,
    ) -> Result<Next, String> {
        match points {
            [a, b] | [_, _, _, a, b] if a.distance(b, None) <= 1e-12 => {
                Err("Pick a point away from the first one".into())
            }
            [a, b, c] | [_, _, _, a, b, c] if frame(a, b, c).is_none() => {
                Err("The three points are in a line; pick a point off it".into())
            }
            [_, _, _, _, _, _] => {
                let xform = orient(&points[..3], &points[3..]).ok_or("The points are in a line")?;
                state.apply(xform, "orient").map(Next::Done)
            }
            _ => Ok(Next::More),
        }
    }
}

/// An orthonormal frame at `a`: x toward `b`, y toward `c` in their plane; None when they are in a line.
fn frame(a: &Point, b: &Point, c: &Point) -> Option<[Vector; 3]> {
    let (x, y) = (b - a, c - a);
    let z = x.cross(&y);

    if z.magnitude() <= 1e-9 * x.magnitude() * y.magnitude() || x.magnitude() <= 1e-12 {
        return None;
    }

    let x = x.normalized();
    let z = z.normalized();
    let y = z.cross(&x);
    Some([x, y, z])
}

/// The rigid transform taking the reference frame onto the target frame.
fn orient(from: &[Point], to: &[Point]) -> Option<Xform> {
    let a = frame(&from[0], &from[1], &from[2])?;
    let b = frame(&to[0], &to[1], &to[2])?;
    let into = Xform::world_to_frame(&from[0], &a[0], &a[1], &a[2]);
    let out = Xform::frame_to_world(&to[0], &b[0], &b[1], &b[2]);
    Some(&out * &into)
}

/// Move `r0` onto `t0` and turn so r0→r1 points along t0→t1.
fn align(r0: &Point, r1: &Point, t0: &Point, t1: &Point) -> Xform {
    let (a, b) = ((r1 - r0).normalized(), (t1 - t0).normalized());
    let axis = a.cross(&b);
    let angle = axis.magnitude().atan2(a.dot(&b));
    // opposite directions turn half way about any axis across them
    let axis = if axis.magnitude() <= 1e-12 && a.dot(&b) < 0.0 {
        Plane::from_point_normal(r0.clone(), a.clone(), None).x_axis()
    } else {
        axis
    };
    let turn = Xform::rotation(&axis, angle, false);
    let back = Xform::translation(-r0[0], -r0[1], -r0[2]);
    let to = Xform::translation(t0[0], t0[1], t0[2]);
    &(&to * &turn) * &back
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A point from coordinates.
    fn p(x: f64, y: f64, z: f64) -> Point {
        Point::new(x, y, z)
    }

    /// Close to within 1e-9.
    fn near(a: &Point, b: &Point) -> bool {
        a.distance(b, None) < 1e-9
    }

    /// The reference frame lands on the target frame, and z follows the right hand.
    #[test]
    fn three_references_land_on_three_targets() {
        let xform = orient(
            &[p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0)],
            &[p(5.0, 5.0, 0.0), p(5.0, 6.0, 0.0), p(4.0, 5.0, 0.0)],
        )
        .unwrap();
        assert!(near(
            &xform.transform_point(&p(1.0, 0.0, 0.0)),
            &p(5.0, 6.0, 0.0)
        ));
        assert!(near(
            &xform.transform_point(&p(0.0, 1.0, 0.0)),
            &p(4.0, 5.0, 0.0)
        ));
        assert!(near(
            &xform.transform_point(&p(0.0, 0.0, 1.0)),
            &p(5.0, 5.0, 1.0)
        ));
    }

    /// Distances stay: the move is rigid even when the targets are spaced differently.
    #[test]
    fn orienting_is_rigid() {
        let from = [p(0.0, 0.0, 0.0), p(10.0, 0.0, 0.0), p(10.0, 5.0, 0.0)];
        let to = [p(50.0, 50.0, 0.0), p(50.0, 60.0, 0.0), p(45.0, 60.0, 0.0)];
        let xform = orient(&from, &to).unwrap();

        for (a, b) in from.iter().zip(&to) {
            assert!(near(&xform.transform_point(a), b));
        }

        let wide = orient(
            &from,
            &[p(0.0, 0.0, 0.0), p(0.0, 0.0, 99.0), p(3.0, 1.0, 50.0)],
        )
        .unwrap();
        let (a, b) = (p(1.0, 2.0, 3.0), p(-4.0, 7.0, 0.5));
        let before = a.distance(&b, None);
        let after = wide
            .transform_point(&a)
            .distance(&wide.transform_point(&b), None);
        assert!((before - after).abs() < 1e-9);
    }

    /// Points in a line give no frame.
    #[test]
    fn points_in_a_line_are_refused() {
        assert!(frame(&p(0.0, 0.0, 0.0), &p(10.0, 0.0, 0.0), &p(20.0, 0.0, 0.0)).is_none());
        assert!(frame(&p(0.0, 0.0, 0.0), &p(0.0, 0.0, 0.0), &p(0.0, 1.0, 0.0)).is_none());
        assert!(
            orient(
                &[p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0)],
                &[p(0.0, 0.0, 0.0), p(1.0, 1.0, 1.0), p(2.0, 2.0, 2.0)]
            )
            .is_none()
        );
        assert!(parse("Orient 3 Points", &["0,0,0"]).is_err());
        assert!(
            parse(
                "Orient 3 Points",
                &["0,0,0", "1,0,0", "0,1,0", "5,5,0", "5,6,0", "4,5,0"]
            )
            .is_ok()
        );
    }

    /// The preview slides after the first target, then turns r0→r1 onto t0→cursor.
    #[test]
    fn the_preview_follows_each_stage() {
        let refs = [p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0)];
        let slide = Orienting
            .preview(&refs, &p(3.0, 4.0, 0.0), &Plane::default())
            .unwrap();
        assert!(near(&slide.transform_point(&refs[1]), &p(4.0, 4.0, 0.0)));
        let points = [
            refs[0].clone(),
            refs[1].clone(),
            refs[2].clone(),
            p(5.0, 5.0, 0.0),
        ];
        let turn = Orienting
            .preview(&points, &p(5.0, 9.0, 0.0), &Plane::default())
            .unwrap();
        assert!(near(&turn.transform_point(&refs[1]), &p(5.0, 6.0, 0.0)));
        let back = align(&refs[0], &refs[1], &p(0.0, 0.0, 0.0), &p(-3.0, 0.0, 0.0));
        assert!(
            near(&back.transform_point(&refs[1]), &p(-1.0, 0.0, 0.0)),
            "opposite directions"
        );
    }
}
