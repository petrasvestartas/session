use crate::State;
use crate::app::command::tool::{Next, Tool, typed_number};
use crate::app::command::{Action, Spec, number};
use session_rust::{Plane, Point, Vector, Xform};

pub const SPEC: Spec = Spec {
    names: &["Scale"],
    aliases: &["s"],
    hint: "Scale · pick the origin, then type a factor or pick two reference points · 1D, 2D or 3D · Scale 2 scales at once",
    options: &[],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// Pick an origin and a factor, or grow the selection by a typed factor about the gizmo.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    if rest.is_empty() {
        return Ok(Box::new(Scaling { mode: 3 }));
    }

    if rest.len() > 1 {
        return Err("wrong number of arguments for `Scale`".into());
    }

    let k = number(rest.first().copied(), "Scale 2")?;

    if k <= 0.0 {
        return Err("Scale wants a factor above zero".into());
    }

    Ok(Box::new(Scale(k)))
}

#[derive(Debug)]
struct Scale(f64);

impl Action for Scale {
    /// Scale about the gizmo centre, or the world origin without one.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let about = state.features.gizmo.as_ref().map(|g| g.origin.clone());
        state.apply(
            crate::state::edit::scaling_about(self.0, about.as_ref()),
            "scale",
        )
    }

    fn needs_selection(&self) -> bool {
        true
    }
}

/// Scale about a picked origin; the selection follows the cursor.
#[derive(Clone, Debug)]
struct Scaling {
    mode: u8, // 1 along one direction, 2 in the construction plane, 3 uniformly
}

impl Action for Scaling {
    /// Start asking for the points.
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.start_tool(Box::new(self.clone()))
    }

    fn needs_selection(&self) -> bool {
        true
    }
}

impl Tool for Scaling {
    fn name(&self) -> &'static str {
        "Scale"
    }

    fn prompt(&self, points: &[Point]) -> String {
        let step = match (points.len(), self.mode) {
            (0, _) => "Origin point",
            (1, 1) => "First reference point, it sets the direction",
            (1, _) => "Scale factor or first reference point",
            _ => "Second reference point or new length",
        };
        format!("{step} · {}D", self.mode)
    }

    fn options(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("1D", "1D"),
            ("2D", "2D"),
            ("3D", "3D"),
            ("Cancel", "Escape"),
        ]
    }

    /// 1D, 2D or 3D switch the mode; a number is the factor, or the new length after a reference point.
    fn word(
        &mut self,
        state: &mut State,
        word: &str,
        points: &[Point],
        plane: &Plane,
    ) -> Option<Result<Next, String>> {
        if let Some(mode) = ["1d", "2d", "3d"]
            .iter()
            .position(|mode| word.eq_ignore_ascii_case(mode))
        {
            self.mode = mode as u8 + 1;
            return Some(Ok(Next::More));
        }

        let value = typed_number(word)?;
        let origin = points.first()?;

        if value <= 0.0 {
            return Some(Err("Scale wants a value above zero".into()));
        }

        let answer = match points {
            [_] if self.mode == 1 => {
                Err("Scale 1D: pick the first reference point to set the direction".into())
            }
            [_] => Ok(scaling(self.mode, origin, origin, value, plane)),
            [_, from] => match reach(self.mode, origin, from, from, plane) {
                Some(base) if base > 1e-12 => {
                    Ok(scaling(self.mode, origin, from, value / base, plane))
                }
                _ => Err("The first reference point must not be on the origin".into()),
            },
            _ => return None,
        };
        Some(answer.and_then(|xform| commit(state, xform)))
    }

    fn preview(&self, points: &[Point], cursor: &Point, plane: &Plane) -> Option<Xform> {
        let [origin, from] = points else {
            return None;
        };
        let k = factor(self.mode, origin, from, cursor, plane)?;
        Some(scaling(self.mode, origin, from, k, plane))
    }

    fn readout(&self, points: &[Point], cursor: &Point, plane: &Plane) -> String {
        match points {
            [origin, from] => factor(self.mode, origin, from, cursor, plane)
                .map(|k| format!("×{k:.3}"))
                .unwrap_or_default(),
            _ => String::new(),
        }
    }

    /// The reference points' reaches give the factor.
    fn placed(
        &mut self,
        state: &mut State,
        points: &[Point],
        plane: &Plane,
    ) -> Result<Next, String> {
        match points {
            [origin, from]
                if reach(self.mode, origin, from, from, plane).is_none_or(|base| base <= 1e-12) =>
            {
                Err("The first reference point must not be on the origin".into())
            }
            [origin, from, to] => {
                let k = factor(self.mode, origin, from, to, plane)
                    .ok_or("The scale factor must be above zero")?;
                commit(state, scaling(self.mode, origin, from, k, plane))
            }
            _ => Ok(Next::More),
        }
    }
}

/// Scale the selection in one undo step.
fn commit(state: &mut State, xform: Xform) -> Result<Next, String> {
    state.apply(xform, "scale").map(Next::Done)
}

/// How far `p` reaches from `origin`: in space, in the plane, or along origin→`from` for 1D.
fn reach(mode: u8, origin: &Point, from: &Point, p: &Point, plane: &Plane) -> Option<f64> {
    let d = p - origin;

    match mode {
        1 => {
            let along = from - origin;
            let length = along.magnitude();
            (length > 1e-12).then(|| d.dot(&along) / length)
        }
        2 => Some(d.dot(&plane.x_axis()).hypot(d.dot(&plane.y_axis()))),
        _ => Some(d.magnitude()),
    }
}

/// The factor that takes the first reference point's reach to `to`'s.
fn factor(mode: u8, origin: &Point, from: &Point, to: &Point, plane: &Plane) -> Option<f64> {
    let base = reach(mode, origin, from, from, plane)?;
    let k = reach(mode, origin, from, to, plane)? / base;
    (base > 1e-12 && k.is_finite() && k > 1e-12).then_some(k)
}

/// A scale by `k` about `origin`: uniform, in the plane, or along origin→`from`.
fn scaling(mode: u8, origin: &Point, from: &Point, k: f64, plane: &Plane) -> Xform {
    match mode {
        1 => stretch(origin, &[(from - origin).normalized()], k),
        2 => stretch(origin, &[plane.x_axis(), plane.y_axis()], k),
        _ => Xform::scale_uniform(origin, k),
    }
}

/// Scale by `k` along the orthonormal `axes` through `origin`, keeping every other direction.
fn stretch(origin: &Point, axes: &[Vector], k: f64) -> Xform {
    let mut xform = Xform::identity();

    for a in axes {
        for col in 0..3 {
            for row in 0..3 {
                xform.m[col * 4 + row] += (k - 1.0) * a[row] * a[col];
            }
        }
    }

    // the origin stays where it is
    for row in 0..3 {
        let moved: f64 = (0..3).map(|col| xform.m[col * 4 + row] * origin[col]).sum();
        xform.m[12 + row] = origin[row] - moved;
    }

    xform
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The construction plane of the Top view.
    fn top() -> Plane {
        Plane::new(
            Point::new(0.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
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

    /// 3D grows every direction, 2D keeps the plane normal, 1D only its direction.
    #[test]
    fn each_mode_scales_its_own_directions() {
        let origin = Point::new(1.0, 1.0, 1.0);
        assert!(near(
            moved(&scaling(3, &origin, &origin, 2.0, &top()), [2.0, 1.0, 1.0]),
            [3.0, 1.0, 1.0]
        ));
        let flat = scaling(2, &origin, &origin, 3.0, &top());
        assert!(near(moved(&flat, [2.0, 2.0, 5.0]), [4.0, 4.0, 5.0]));
        let zero = Point::new(0.0, 0.0, 0.0);
        let along = scaling(1, &zero, &Point::new(1.0, 1.0, 0.0), 4.0, &top());
        assert!(near(moved(&along, [1.0, 1.0, 0.0]), [4.0, 4.0, 0.0]));
        assert!(near(moved(&along, [1.0, -1.0, 0.0]), [1.0, -1.0, 0.0]));
        assert!(near(moved(&along, [0.0, 0.0, 7.0]), [0.0, 0.0, 7.0]));
    }

    /// The factor is the second reach over the first, in each mode's measure.
    #[test]
    fn the_reference_points_give_the_factor() {
        let zero = Point::new(0.0, 0.0, 0.0);
        let from = Point::new(10.0, 0.0, 0.0);
        let k = |mode, to: [f64; 3]| {
            factor(mode, &zero, &from, &Point::new(to[0], to[1], to[2]), &top())
        };
        assert!((k(3, [5.0, 0.0, 0.0]).unwrap() - 0.5).abs() < 1e-12);
        assert!((k(3, [0.0, 0.0, 30.0]).unwrap() - 3.0).abs() < 1e-12);
        assert!(
            (k(2, [0.0, 20.0, 99.0]).unwrap() - 2.0).abs() < 1e-12,
            "height does not count in 2D"
        );
        assert!(
            (k(1, [30.0, 50.0, 0.0]).unwrap() - 3.0).abs() < 1e-12,
            "only along the direction in 1D"
        );
        assert!(k(1, [-5.0, 0.0, 0.0]).is_none(), "a mirror is no scale");
        assert!(k(3, [0.0, 0.0, 0.0]).is_none());
        assert!(
            factor(3, &zero, &zero, &from, &top()).is_none(),
            "the first reference is on the origin"
        );
        let preview = Scaling { mode: 1 }
            .preview(
                &[zero.clone(), from.clone()],
                &Point::new(30.0, 0.0, 0.0),
                &top(),
            )
            .unwrap();
        assert!(near(moved(&preview, [1.0, 1.0, 1.0]), [3.0, 1.0, 1.0]));
        assert_eq!(
            Scaling { mode: 3 }.readout(&[zero, from], &Point::new(20.0, 0.0, 0.0), &top()),
            "×2.000"
        );
    }

    /// Typed words: the mode in any case, factors above zero only.
    #[test]
    fn the_mode_and_factor_can_be_typed() {
        let mut tool = Scaling { mode: 3 };
        assert!(tool.prompt(&[]).ends_with("3D"));
        assert_eq!(tool.options().len(), 4);
        assert!(parse("Scale", &[]).is_ok());
        assert!(parse("Scale", &["0"]).is_err());
        assert!(parse("Scale", &["-2"]).is_err());
        assert!(parse("Scale", &["2", "extra"]).is_err());
        tool.mode = 1;
        assert!(
            tool.prompt(&[Point::new(0.0, 0.0, 0.0)])
                .contains("direction")
        );
    }
}
