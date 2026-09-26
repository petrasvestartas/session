use crate::State;
use crate::app::command::verbs::measure::{
    self, compute_mesh_area, plural, skipped_text, to_text, unit_suffix,
};
use crate::app::command::{Action, Spec};
use session_rust::element::ElementGeometry;
use session_rust::{BRep, Geometry, NurbsSurface, Xform};

pub const SPEC: Spec = Spec {
    names: &["Area"],
    aliases: &[],
    hint: "Area · surface area of the selected meshes, BReps, NURBS surfaces and elements, or of one selected face",
    options: &[],
    arity: Some(0),
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Area))
}

/// The summed world area of the selected surfaces, or of the selected face.
#[derive(Debug)]
struct Area;

impl Action for Area {
    fn run(&self, state: &mut State) -> Result<String, String> {
        measure::loading(state)?;
        let (targets, mut skipped) = measure::selected(state);
        let mut total = 0.0;
        let mut objects = 0;
        let mut faces = 0;

        for target in &targets {
            match compute_area(target.geometry, &target.place, target.face) {
                Some(area) => {
                    total += area;
                    objects += 1;
                    faces += usize::from(target.face.is_some());
                }
                None => skipped += 1,
            }
        }

        if objects == 0 {
            return Err(
                "Area measures meshes, BReps, NURBS surfaces and elements; none is selected".into(),
            );
        }

        // a face selected alone counts as a face
        let counted = if faces == objects {
            plural(faces, "face")
        } else {
            plural(objects, "object")
        };
        let value = format!("{} {}", to_text(total), unit_suffix(state, 2));
        state.mark_selection(value.clone());
        Ok(format!("Area {value} · {counted}{}", skipped_text(skipped)))
    }

    fn needs_selection(&self) -> bool {
        true
    }
}

/// World area of a surface kind placed by `place`, all faces or only `face`; None for other kinds.
fn compute_area(geometry: &Geometry, place: &Xform, face: Option<usize>) -> Option<f64> {
    match geometry {
        Geometry::Mesh(mesh) => Some(compute_mesh_area(mesh, place, face)),
        Geometry::BRep(brep) => Some(compute_brep_area(brep, place, face)),
        Geometry::NurbsSurface(surface) => Some(compute_surface_area(&surface.transformed(place))),
        Geometry::Element(element) => match element.geometry() {
            ElementGeometry::Mesh(mesh) => Some(compute_mesh_area(mesh, place, face)),
            ElementGeometry::BRep(brep) => Some(compute_brep_area(brep, place, face)),
            ElementGeometry::None => None,
        },
        _ => None,
    }
}

/// World area of a BRep's faces as the viewer meshes them; planar faces are exact.
fn compute_brep_area(brep: &BRep, place: &Xform, face: Option<usize>) -> f64 {
    let meshes = brep.face_meshes_q(Some(crate::app::walk::brep::QUALITY));
    meshes
        .iter()
        .enumerate()
        .filter(|(index, _)| face.is_none_or(|face| face == *index))
        .map(|(_, mesh)| compute_mesh_area(mesh, place, None))
        .sum()
}

const GAUSS: [(f64, f64); 8] = [
    (-0.9602898564975363, 0.1012285362903763),
    (-0.7966664774136267, 0.2223810344533745),
    (-0.5255324099163290, 0.3137066458778873),
    (-0.1834346424956498, 0.3626837833783620),
    (0.1834346424956498, 0.3626837833783620),
    (0.5255324099163290, 0.3137066458778873),
    (0.7966664774136267, 0.2223810344533745),
    (0.9602898564975363, 0.1012285362903763),
]; // 8-point Gauss-Legendre nodes and weights on [-1, 1]

/// Area of a NURBS surface: |Su x Sv| integrated per knot span with 8 x 8 Gauss points.
fn compute_surface_area(surface: &NurbsSurface) -> f64 {
    let us = surface.get_span_vector(0);
    let vs = surface.get_span_vector(1);
    let mut total = 0.0;

    for u in us.windows(2) {
        let (uh, um) = ((u[1] - u[0]) * 0.5, (u[1] + u[0]) * 0.5); // half width, middle

        for v in vs.windows(2) {
            let (vh, vm) = ((v[1] - v[0]) * 0.5, (v[1] + v[0]) * 0.5);

            for (su, wu) in GAUSS {
                for (sv, wv) in GAUSS {
                    let derivatives = surface.evaluate(um + uh * su, vm + vh * sv, 1); // [S, Sv, Su]

                    if let [_, dv, du] = derivatives.as_slice() {
                        total += wu * wv * uh * vh * du.cross(dv).magnitude();
                    }
                }
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::{Element, Line, Mesh, Point, Primitives};
    use std::f64::consts::PI;
    use std::rc::Rc;

    fn near(value: f64, expected: f64, relative: f64) -> bool {
        (value - expected).abs() <= relative * expected.abs().max(1.0)
    }

    #[test]
    fn meshes_and_breps_measure_in_world_units() {
        let identity = Xform::identity();
        let mesh = Mesh::create_box(2.0, 2.0, 0.4);
        let boxed = Geometry::Mesh(Rc::new(mesh.clone()));
        assert!(near(
            compute_area(&boxed, &identity, None).unwrap(),
            11.2,
            1e-9
        ));
        let moved = Xform::translation(-9.0, 3.0, 0.0);
        assert!(near(
            compute_area(&boxed, &moved, None).unwrap(),
            11.2,
            1e-9
        ));
        let scaled = Xform::scale_xyz(2.0, 2.0, 2.0);
        assert!(near(
            compute_area(&boxed, &scaled, None).unwrap(),
            44.8,
            1e-9
        ));
        let faces: Vec<f64> = mesh
            .faces()
            .into_iter()
            .map(|face| compute_area(&boxed, &identity, Some(face)).unwrap())
            .collect();
        assert!(faces.iter().any(|area| near(*area, 4.0, 1e-9)));
        assert!(faces.iter().any(|area| near(*area, 0.8, 1e-9)));

        let brep = BRep::create_box(2.0, 2.0, 0.4);
        let solid = Geometry::BRep(Rc::new(brep.clone()));
        assert!(near(
            compute_area(&solid, &identity, None).unwrap(),
            11.2,
            1e-9
        ));
        let each: Vec<f64> = (0..brep.m_faces.len())
            .map(|face| compute_area(&solid, &identity, Some(face)).unwrap())
            .collect();
        assert!(near(each.iter().sum(), 11.2, 1e-9));
        assert!(
            each.iter()
                .all(|area| near(*area, 4.0, 1e-9) || near(*area, 0.8, 1e-9))
        );

        let cylinder = Geometry::BRep(Rc::new(BRep::create_cylinder(150.0, 400.0)));
        let exact = 2.0 * PI * 150.0 * 400.0 + 2.0 * PI * 150.0 * 150.0;
        assert!(near(
            compute_area(&cylinder, &identity, None).unwrap(),
            exact,
            5e-3
        ));

        let element = Geometry::Element(Rc::new(Element::from_mesh(mesh, "beam")));
        assert!(near(
            compute_area(&element, &identity, None).unwrap(),
            11.2,
            1e-9
        ));
        let line = Geometry::Line(Rc::new(Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0)));
        assert_eq!(compute_area(&line, &identity, None), None);
    }

    #[test]
    fn nurbs_surfaces_integrate_exactly() {
        let identity = Xform::identity();
        let cylinder = Primitives::cylinder_surface(0.0, 0.0, 0.0, 2.0, 5.0);
        let side = Geometry::NurbsSurface(Rc::new(cylinder));
        assert!(near(
            compute_area(&side, &identity, None).unwrap(),
            20.0 * PI,
            1e-7
        ));
        let scaled = Xform::scale_xyz(2.0, 2.0, 2.0);
        assert!(near(
            compute_area(&side, &scaled, None).unwrap(),
            80.0 * PI,
            1e-7
        ));
        let sphere =
            Geometry::NurbsSurface(Rc::new(Primitives::sphere_surface(0.0, 0.0, 0.0, 3.0)));
        assert!(near(
            compute_area(&sphere, &identity, None).unwrap(),
            36.0 * PI,
            1e-7
        ));
        let corners = [
            Point::new(-1.0, -1.0, 0.0),
            Point::new(-1.0, 1.0, 0.0),
            Point::new(1.0, -1.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
        ];
        let flat = NurbsSurface::create(false, false, 1, 1, 2, 2, &corners).unwrap();
        let flat = Geometry::NurbsSurface(Rc::new(flat));
        assert!(near(
            compute_area(&flat, &identity, None).unwrap(),
            4.0,
            1e-12
        ));
    }
}
