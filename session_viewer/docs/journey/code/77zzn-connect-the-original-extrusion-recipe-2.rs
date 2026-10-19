/// Every picked curve extruded by the answered distance, as one undo step.
fn build(input: &Input) -> Result<Made, String> {
    let (_, direction) = axis(input);
    let offset = input
        .along(0, &direction)
        .ok_or("Extrude needs a distance")?;

    if !(offset.magnitude() > 1e-12 && (0..3).all(|i| offset[i].is_finite())) {
        return Err("The extrusion distance must not be zero".into());
    }

    let cap = input.choice != "Cap Off";
    let mut geometries = Vec::new();
    let mut skipped = [0; 3];

    for picked in &input.curves[0] {
        let (geometry, skip) = extrude(picked, &offset, cap)?;

        match skip {
            Skip::Open if cap => skipped[0] += 1,
            Skip::Bent if cap => skipped[1] += 1,
            Skip::Flat if cap => skipped[2] += 1,
            _ => {}
        }

        geometries.push(geometry);
    }

    let solids = geometries
        .iter()
        .filter(|geometry| matches!(geometry, Geometry::BRep(brep) if brep.is_solid()))
        .count();
    let mut message = format!(
        "Extruded {}: {}, {}",
        count(geometries.len(), "curve"),
        count(solids, "solid"),
        count(geometries.len() - solids, "open surface")
    );

    for (number, why) in skipped
        .iter()
        .zip(["open", "not planar", "along its plane"])
    {
        if *number > 0 {
            message.push_str(&format!(" · no cap on {} ({why})", count(*number, "curve")));
        }
    }

    Ok(Made {
        geometries,
        message,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gather_input::Answer;
    use crate::surfacing::picked;
    use crate::surfacing::tests::p;
    use session_rust::Line;

    /// The input of `geometries` extruded by `answer`.
    fn input(geometries: &[Geometry], answer: Answer, choice: &'static str) -> Input {
        Input {
            curves: vec![
                geometries
                    .iter()
                    .map(|geometry| picked(geometry, &Xform::identity()).unwrap())
                    .collect(),
            ],
            answers: vec![answer],
            choice,
            normal: Vector::new(0.0, 0.0, 1.0),
        }
    }

    /// A polyline geometry.
    fn polyline(points: &[Point]) -> Geometry {
        Geometry::Polyline(Rc::new(Polyline::new(points.to_vec())))
    }

    /// The one made geometry.
    fn one(input: &Input) -> Geometry {
        build(input).unwrap().geometries.remove(0)
    }

    /// A closed rectangle becomes a box; open or uncapped it stays a sheet.
    #[test]
    fn polylines_become_boxes_and_sheets() {
        let rectangle = polyline(&[
            p(0., 0., 0.),
            p(40., 0., 0.),
            p(40., 30., 0.),
            p(0., 30., 0.),
            p(0., 0., 0.),
        ]);
        let Geometry::BRep(solid) =
            one(&input(&[rectangle.clone()], Answer::Number(25.0), "Cap On"))
        else {
            panic!()
        };
        assert!(solid.is_solid());
        assert_eq!(solid.face_count(), 6);
        assert!((solid.volume().abs() - 40.0 * 30.0 * 25.0).abs() < 1e-6);
        let Geometry::BRep(sheet) = one(&input(&[rectangle], Answer::Number(25.0), "Cap Off"))
        else {
            panic!()
        };
        assert_eq!((sheet.face_count(), sheet.is_solid()), (4, false));
        let open = polyline(&[
            p(0., 0., 0.),
            p(10., 0., 0.),
            p(10., 10., 0.),
            p(0., 10., 0.),
        ]);
        let Geometry::BRep(open) = one(&input(&[open], Answer::Number(5.0), "Cap On")) else {
            panic!()
        };
        assert_eq!((open.face_count(), open.is_solid()), (3, false));
        let bent = polyline(&[
            p(0., 0., 0.),
            p(40., 0., 0.),
            p(40., 30., 10.),
            p(0., 30., 0.),
            p(0., 0., 0.),
        ]);
        let made = build(&input(&[bent], Answer::Number(20.0), "Cap On")).unwrap();
        assert!(made.message.contains("not planar"));
        let Geometry::BRep(bent) = &made.geometries[0] else {
            panic!()
        };
        assert_eq!((bent.face_count(), bent.is_solid()), (4, false));
    }

    /// A closed circle-like curve becomes a solid cylinder in either winding.
    #[test]
    fn a_closed_curve_becomes_a_solid() {
        let ring = |turn: f64| {
            let points: Vec<Point> = (0..8)
                .map(|i| {
                    let angle = turn * std::f64::consts::TAU * i as f64 / 8.0;
                    p(10.0 * angle.cos(), 10.0 * angle.sin(), 0.0)
                })
                .collect();
            let curve = NurbsCurve::create(true, 3, &points);
            Geometry::NurbsCurve(Rc::new(curve))
        };
        let mut volume = None;

        for turn in [1.0, -1.0] {
            let Geometry::BRep(solid) = one(&input(&[ring(turn)], Answer::Number(30.0), "Cap On"))
            else {
                panic!()
            };
            assert!(solid.is_solid());
            assert_eq!(solid.face_count(), 3);
            let v = solid.volume();
            assert!(v > 0.0, "{v}");
            let area = picked(&ring(turn), &Xform::identity()).unwrap().outline();
            let area = 0.5
                * area
                    .windows(2)
                    .map(|w| w[0][0] * w[1][1] - w[1][0] * w[0][1])
                    .sum::<f64>()
                    .abs();
            assert!(
                (v - area * 30.0).abs() < 0.01 * area * 30.0,
                "{v} {}",
                area * 30.0
            );
            volume = Some(v);
        }

        assert!(volume.is_some());
        let Geometry::NurbsSurface(tube) =
            one(&input(&[ring(1.0)], Answer::Number(30.0), "Cap Off"))
        else {
            panic!()
        };
        assert!(tube.is_closed(0));
    }

    /// A line gives a flat strip; a zero distance is refused.
    #[test]
    fn a_line_gives_a_strip() {
        let line = Geometry::Line(Rc::new(Line::from_points(&p(0., 0., 0.), &p(50., 0., 0.))));
        let Geometry::NurbsSurface(strip) = one(&input(
            &[line.clone()],
            Answer::Vector(Vector::new(0., 0., 10.)),
            "Cap On",
        )) else {
            panic!()
        };
        assert_eq!((strip.degree(0), strip.degree(1)), (1, 1));
        assert!(build(&input(&[line], Answer::Number(0.0), "Cap On")).is_err());
    }

    /// The default direction is the plane's normal, facing the construction plane.
    #[test]
    fn the_direction_follows_the_construction_plane() {
        let square = polyline(&[
            p(0., 0., 0.),
            p(0., 10., 0.),
            p(10., 10., 0.),
            p(10., 0., 0.),
            p(0., 0., 0.),
        ]);
        let (_, direction) = axis(&input(&[square], Answer::Number(1.0), "Cap On"));
        assert!((direction[2] - 1.0).abs() < 1e-12);
        let line = Geometry::Line(Rc::new(Line::from_points(&p(0., 0., 0.), &p(50., 0., 0.))));
        let (_, direction) = axis(&input(&[line], Answer::Number(1.0), "Cap On"));
        assert!((direction[2] - 1.0).abs() < 1e-12);
    }
}
