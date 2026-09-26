// --8<-- [start:length]
use crate::State;
use crate::app::command::verbs::measure::{self, plural, skipped_text, to_text, unit_suffix};
use crate::app::command::{Action, Spec};
use session_rust::{Geometry, Xform};

pub const SPEC: Spec = Spec {
    names: &["Length"],
    aliases: &[],
    hint: "Length · total length of the selected lines, polylines and NURBS curves",
    options: &[],
    arity: Some(0),
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Length))
}

/// The summed world length of the selected curves.
#[derive(Debug)]
struct Length;

impl Action for Length {
    fn run(&self, state: &mut State) -> Result<String, String> {
        // `?` stops here with the loading message while a released document comes back.
        measure::loading(state)?;
        let (targets, mut skipped) = measure::selected(state);
        let mut total = 0.0;
        let mut curves = 0;

        for target in &targets {
            match compute_length(target.geometry, &target.place) {
                Some(length) => {
                    total += length;
                    curves += 1;
                }
                None => skipped += 1,
            }
        }

        if curves == 0 {
            return Err(
                "Length measures lines, polylines and NURBS curves; none is selected".into(),
            );
        }

        let value = format!("{} {}", to_text(total), unit_suffix(state, 1));
        state.mark_selection(value.clone());
        Ok(format!(
            "Length {value} · {}{}",
            plural(curves, "curve"),
            skipped_text(skipped)
        ))
    }

    fn needs_selection(&self) -> bool {
        true
    }
}

/// World length of a line, polyline or NURBS curve placed by `place`; None for other kinds.
fn compute_length(geometry: &Geometry, place: &Xform) -> Option<f64> {
    match geometry {
        Geometry::Line(line) => Some(line.transformed(place).length()),
        Geometry::Polyline(polyline) => Some(polyline.transformed(place).length()),
        Geometry::NurbsCurve(curve) => Some(curve.transformed(place).length(None)), // placed first, so a block scaled 1.5 times reports 1.5 times the length
        _ => None,
    }
}
// --8<-- [end:length]

// --8<-- [start:length-tests]
#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::{Line, Mesh, NurbsCurve, Point, Polyline};
    use std::rc::Rc;

    #[test]
    fn curves_measure_in_world_units() {
        let identity = Xform::identity();
        let line = Geometry::Line(Rc::new(Line::new(0.0, 0.0, 0.0, 3.0, 4.0, 0.0)));
        assert_eq!(compute_length(&line, &identity), Some(5.0));
        let polyline = Geometry::Polyline(Rc::new(Polyline::new(vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(60.0, 0.0, 0.0),
            Point::new(60.0, 60.0, 0.0),
        ])));
        let place = &Xform::translation(20.0, 0.0, 0.0) * &Xform::scale_xyz(1.5, 1.5, 1.5);
        let placed = compute_length(&polyline, &place).unwrap();
        assert!((placed - 180.0).abs() < 1e-9);
        let points = [
            Point::new(8.0, 3.0, 0.0),
            Point::new(9.0, 4.0, 0.0),
            Point::new(10.0, 3.0, 0.0),
        ];
        let curve = Geometry::NurbsCurve(Rc::new(NurbsCurve::create(false, 2, &points)));
        let exact = (2.0 * 2.0_f64.sqrt() + 2.0 * 1.0_f64.asinh()) / 2.0;
        assert!((compute_length(&curve, &identity).unwrap() - exact).abs() < 1e-9);
        let mesh = Geometry::Mesh(Rc::new(Mesh::create_box(1.0, 1.0, 1.0)));
        assert_eq!(compute_length(&mesh, &identity), None);
    }
}
// --8<-- [end:length-tests]
