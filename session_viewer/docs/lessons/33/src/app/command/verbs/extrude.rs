use crate::app::command::tool::gather::{self, Input, Made, Recipe, Step};
use crate::app::command::tool::surfacing::{Picked, checked, count, loop_normal, planar};
use crate::app::command::{Action, Spec};
use session_rust::{
    BRep, BRepOrientation, BRepRef, Geometry, NurbsCurve, NurbsSurface, Point, Polyline,
    Primitives, Vector, Xform,
};
use std::rc::Rc;

pub const SPEC: Spec = Spec {
    names: &["Extrude"],
    aliases: &[],
    hint: "Extrude (Cap On Cap Off): curves, Enter, then a distance or x,y,z · Example: select curves, then Extrude 25",
    options: &["Extrude Cap On", "Extrude Cap Off"],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

pub static RECIPE: Recipe = Recipe {
    name: "Extrude",
    chips: &[
        ("Cap On", "Cap On"),
        ("Cap Off", "Cap Off"),
        ("Finish", ""),
        ("Cancel", "Escape"),
    ],
    steps: &[
        Step::Curves {
            prompt: "select curves to extrude",
            min: 1,
            max: usize::MAX,
        },
        Step::Distance("distance"),
    ],
    axis,
    build,
};

const F: BRepOrientation = BRepOrientation::Forward;
const R: BRepOrientation = BRepOrientation::Reversed;

/// Start extruding; a typed number or x,y,z answers the distance.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    gather::start(&RECIPE, rest)
}

/// The first curve's start and its plane's normal, facing like the construction plane; else that plane's normal.
fn axis(input: &Input) -> (Point, Vector) {
    let Some(first) = input.curves.first().and_then(|curves| curves.first()) else {
        return (Point::new(0.0, 0.0, 0.0), input.normal.clone());
    };
    let outline = first.outline();
    let normal = loop_normal(&outline)
        .filter(|normal| planar(&outline, normal))
        .map(|normal| {
            if normal.dot(&input.normal) < 0.0 {
                -normal
            } else {
                normal
            }
        })
        .unwrap_or_else(|| input.normal.clone());
    (first.curve.point_at_start(), normal)
}

/// Why a cap was left off, if it was.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Skip {
    None,
    Open,
    Bent,
    Flat,
}

/// The normal of a closed planar outline, signed toward `offset`; why not when it cannot be capped.
fn cap_normal(outline: &[Point], closed: bool, offset: &Vector) -> Result<Vector, Skip> {
    if !closed {
        return Err(Skip::Open);
    }

    let normal = loop_normal(outline).ok_or(Skip::Bent)?;

    if !planar(outline, &normal) {
        return Err(Skip::Bent);
    }

    let along = normal.dot(offset);

    if along.abs() <= 1e-9 * offset.magnitude() {
        return Err(Skip::Flat);
    }

    Ok(if along < 0.0 { -normal } else { normal })
}

/// One picked curve extruded by `offset`, capped when asked and possible.
fn extrude(picked: &Picked, offset: &Vector, cap: bool) -> Result<(Geometry, Skip), String> {
    let named = |mut brep: BRep| {
        brep.name = "extrusion".into();
        Geometry::BRep(Rc::new(brep))
    };

    match &picked.corners {
        Some(corners) if corners.len() > 2 => {
            let closed = corners.len() >= 4
                && corners[0].distance(&corners[corners.len() - 1], None) <= 1e-12;
            let lift = |p: &Point| p + offset;
            let mut faces: Vec<Polyline> = corners
                .windows(2)
                .map(|pair| {
                    Polyline::new(vec![
                        pair[0].clone(),
                        pair[1].clone(),
                        lift(&pair[1]),
                        lift(&pair[0]),
                        pair[0].clone(),
                    ])
                })
                .collect();
            let skip = match cap_normal(&corners[..corners.len() - 1], closed, offset) {
                Ok(_) if cap => {
                    faces.push(Polyline::new(corners.clone()));
                    faces.push(Polyline::new(corners.iter().map(lift).collect()));
                    Skip::None
                }
                Ok(_) => Skip::None,
                Err(skip) => skip,
            };
            let brep = BRep::from_polylines(&faces, &[]);

            if brep.face_count() == 0 {
                return Err("The polyline could not be extruded".into());
            }

            Ok((named(brep), skip))
        }
        _ => {
            let outline = picked.outline();
            let normal = cap_normal(&outline, picked.curve.is_closed(), offset);

            if let (true, Ok(normal)) = (cap && picked.corners.is_none(), &normal) {
                return Ok((named(capped(&picked.curve, offset, normal)?), Skip::None));
            }

            let surface = Primitives::create_extrusion(&picked.curve, offset);
            let skip = normal.err().unwrap_or(Skip::None);
            Ok((checked(surface, "extrusion")?, skip))
        }
    }
}

/// A straight pcurve from (u0, v0) to (u1, v1).
fn uv_line(u0: f64, v0: f64, u1: f64, v1: f64) -> NurbsCurve {
    NurbsCurve::create(
        false,
        1,
        &[Point::new(u0, v0, 0.0), Point::new(u1, v1, 0.0)],
    )
}

/// A flat patch in the plane through `origin` with outward `normal` that covers `points`.
fn cap_patch(points: &[Point], origin: &Point, normal: &Vector) -> NurbsSurface {
    let mut x = normal.cross(&Vector::new(0.0, 0.0, 1.0));

    if !x.normalize_self() {
        x = Vector::new(1.0, 0.0, 0.0);
    }

    let y = normal.cross(&x);
    let (mut low, mut high) = ([f64::MAX; 2], [f64::MIN; 2]);

    for p in points {
        let d = p - origin;
        let (u, v) = (d.dot(&x), d.dot(&y));
        low = [low[0].min(u), low[1].min(v)];
        high = [high[0].max(u), high[1].max(v)];
    }

    let pad = 0.01 * (high[0] - low[0]).max(high[1] - low[1]);
    let corner = |u: f64, v: f64| origin + &(&x * u + &y * v);
    let (u0, v0, u1, v1) = (low[0] - pad, low[1] - pad, high[0] + pad, high[1] + pad);
    let points = [
        corner(u0, v0),
        corner(u0, v1),
        corner(u1, v0),
        corner(u1, v1),
    ];
    NurbsSurface::create(false, false, 1, 1, 2, 2, &points).unwrap_or_default()
}

/// The curve in the patch's parameters: the affine image of its control points.
fn on_patch(curve: &NurbsCurve, patch: &NurbsSurface) -> NurbsCurve {
    let p00 = patch.get_cv(0, 0).unwrap_or_default();
    let eu = &patch.get_cv(1, 0).unwrap_or_default() - &p00;
    let ev = &patch.get_cv(0, 1).unwrap_or_default() - &p00;
    let ((u0, u1), (v0, v1)) = (
        patch.domain(0).unwrap_or((0.0, 1.0)),
        patch.domain(1).unwrap_or((0.0, 1.0)),
    );
    let mut flat = curve.clone();

    for i in 0..curve.cv_count() {
        let (wx, wy, wz, w) = curve.get_cv_4d(i).unwrap_or((0.0, 0.0, 0.0, 1.0));
        let d = &Point::new(wx / w, wy / w, wz / w) - &p00;
        let u = u0 + (u1 - u0) * d.dot(&eu) / eu.dot(&eu);
        let v = v0 + (v1 - v0) * d.dot(&ev) / ev.dot(&ev);
        flat.set_cv_4d(i, u * w, v * w, 0.0, w);
    }

    flat
}

/// Twice the signed area a closed pcurve encloses, counter-clockwise positive.
fn winding(curve: &NurbsCurve) -> f64 {
    let points = curve
        .divide_by_count((curve.cv_count() * 4).max(16), true)
        .0;
    points
        .windows(2)
        .map(|pair| pair[0][0] * pair[1][1] - pair[1][0] * pair[0][1])
        .sum()
}

/// A cap face bounded by one closed edge lying on `patch`.
fn cap_face(brep: &mut BRep, patch: &NurbsSurface, curve: &NurbsCurve, edge: usize) -> usize {
    let surface = brep.add_surface(patch);
    let flat = on_patch(curve, patch);
    let orientation = if winding(&flat) > 0.0 { F } else { R };
    let pcurve = brep.add_curve_2d(&flat);
    brep.add_pcurve(edge, surface, pcurve as i32, -1);
    let wire = brep.add_wire(&[BRepRef::new(edge as i32, orientation)]);
    brep.add_face(surface as i32, &[BRepRef::new(wire as i32, F)], 0.0)
}

/// A closed planar curve extruded into a solid: one side face with a seam and two flat caps.
fn capped(curve: &NurbsCurve, offset: &Vector, normal: &Vector) -> Result<BRep, String> {
    let mut bottom = curve.clone();

    // the side faces outward when the curve turns counter-clockwise about the offset
    let outline = bottom.divide_by_count(64, false).0;

    if loop_normal(&outline).is_some_and(|own| own.dot(normal) < 0.0) {
        bottom.reverse();
    }

    bottom.set_domain(0.0, 1.0);
    let top = bottom.transformed(&Xform::translation(offset[0], offset[1], offset[2]));
    let body = Primitives::create_extrusion(&bottom, offset);

    if !body.is_valid() {
        return Err("The curve could not be extruded".into());
    }

    let mut brep = BRep::new();
    let low = bottom.point_at_start();
    let high = &low + offset;
    let start = brep.add_vertex(&low, 0.0) as i32;
    let end = brep.add_vertex(&high, 0.0) as i32;
    let curve_low = brep.add_curve_3d(&bottom) as i32;
    let edge_low = brep.add_edge(curve_low, start, start);
    let curve_high = brep.add_curve_3d(&top) as i32;
    let edge_high = brep.add_edge(curve_high, end, end);
    let curve_seam = brep.add_curve_3d(&NurbsCurve::create(false, 1, &[low.clone(), high])) as i32;
    let edge_seam = brep.add_edge(curve_seam, start, end);
    let surface = brep.add_surface(&body);
    let (u0, u1) = body.domain(0).unwrap_or((0.0, 1.0));
    let (v0, v1) = body.domain(1).unwrap_or((0.0, 1.0));
    let pcurve_low = brep.add_curve_2d(&uv_line(u0, v0, u1, v0));
    brep.add_pcurve(edge_low, surface, pcurve_low as i32, -1);
    let pcurve_high = brep.add_curve_2d(&uv_line(u0, v1, u1, v1));
    brep.add_pcurve(edge_high, surface, pcurve_high as i32, -1);
    let right = brep.add_curve_2d(&uv_line(u1, v0, u1, v1));
    let left = brep.add_curve_2d(&uv_line(u0, v0, u0, v1));
    brep.add_pcurve(edge_seam, surface, right as i32, left as i32);
    let wire = brep.add_wire(&[
        BRepRef::new(edge_low as i32, F),
        BRepRef::new(edge_seam as i32, F),
        BRepRef::new(edge_high as i32, R),
        BRepRef::new(edge_seam as i32, R),
    ]);
    let side = brep.add_face(surface as i32, &[BRepRef::new(wire as i32, F)], 0.0);
    let points: Vec<Point> = (0..bottom.cv_count())
        .filter_map(|i| bottom.get_cv(i))
        .collect();
    let lifted: Vec<Point> = points.iter().map(|p| p + offset).collect();
    let floor = cap_face(
        &mut brep,
        &cap_patch(&points, &low, &-normal),
        &bottom,
        edge_low as usize,
    );
    let roof = cap_face(
        &mut brep,
        &cap_patch(&lifted, &(&low + offset), normal),
        &top,
        edge_high as usize,
    );
    let shell = brep.add_shell(&[
        BRepRef::new(side as i32, F),
        BRepRef::new(floor as i32, F),
        BRepRef::new(roof as i32, F),
    ]);
    brep.add_solid(&[BRepRef::new(shell as i32, F)]);
    Ok(brep)
}

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
    use crate::app::command::tool::gather::Answer;
    use crate::app::command::tool::surfacing::picked;
    use crate::app::command::tool::surfacing::tests::p;
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
