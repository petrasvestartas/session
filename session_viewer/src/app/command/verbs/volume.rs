use crate::State;
use crate::app::command::verbs::measure::{
    self, compute_swept, plural, skipped_text, to_text, unit_suffix,
};
use crate::app::command::{Action, Spec};
use crate::app::walk::{brep_edges::edge_chains, brep_orient::face_signs};
use session_rust::element::ElementGeometry;
use session_rust::{BRep, Geometry, Mesh, Point, Xform};

pub const SPEC: Spec = Spec {
    names: &["Volume"],
    aliases: &[],
    hint: "Volume · volume of the selected closed meshes, solid BReps and elements; open ones are refused",
    options: &[],
    arity: Some(0),
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

const OPEN: &str = "open"; // a volume kind that encloses nothing

const SHOWN: usize = 3; // open object names listed in an answer

fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Volume))
}

/// The summed world volume of the selected closed objects.
#[derive(Debug)]
struct Volume;

impl Action for Volume {
    fn run(&self, state: &mut State) -> Result<String, String> {
        measure::loading(state)?;
        let (targets, mut skipped) = measure::selected(state);
        let mut total = 0.0;
        let mut closed = 0;
        let mut open = Vec::new(); // names of open objects

        for target in &targets {
            match compute_volume(target.geometry, &target.place) {
                Some(Ok(volume)) => {
                    total += volume;
                    closed += 1;
                }
                Some(Err(_)) => open.push(state.scene.object_name(target.row).to_owned()),
                None => skipped += 1,
            }
        }

        let names = match open.len() {
            0..=SHOWN => open.join(", "),
            count => format!("{} and {} more", open[..SHOWN].join(", "), count - SHOWN),
        };

        if closed == 0 && open.is_empty() {
            return Err("Volume measures closed meshes and solid BReps; none is selected".into());
        }

        if closed == 0 {
            return Err(format!(
                "Volume needs closed objects; open or inconsistently wound: {names}"
            ));
        }

        let refused = match open.len() {
            0 => String::new(),
            _ => format!(" · open, not measured: {names}"),
        };
        let value = format!("{} {}", to_text(total), unit_suffix(state, 3));
        state.mark_selection(value.clone());
        Ok(format!(
            "Volume {value} · {}{refused}{}",
            plural(closed, "object"),
            skipped_text(skipped)
        ))
    }

    fn needs_selection(&self) -> bool {
        true
    }
}

/// World volume of a closed mesh or solid BRep; Err when open, None when the kind has no volume.
fn compute_volume(geometry: &Geometry, place: &Xform) -> Option<Result<f64, &'static str>> {
    match geometry {
        Geometry::Mesh(mesh) => Some(compute_mesh_volume(mesh, place)),
        Geometry::BRep(brep) => Some(compute_brep_volume(brep, place)),
        Geometry::NurbsSurface(_) => Some(Err(OPEN)),
        Geometry::Element(element) => match element.geometry() {
            ElementGeometry::Mesh(mesh) => Some(compute_mesh_volume(mesh, place)),
            ElementGeometry::BRep(brep) => Some(compute_brep_volume(brep, place)),
            ElementGeometry::None => None,
        },
        _ => None,
    }
}

/// Volume of a closed mesh, from its drawn triangles.
fn compute_mesh_volume(mesh: &Mesh, place: &Xform) -> Result<f64, &'static str> {
    if !mesh.is_closed() {
        return Err(OPEN);
    }

    let origin = origin_of(mesh, place);
    Ok(compute_swept(mesh, place, &origin).abs() / 6.0)
}

/// Volume of a solid BRep as the viewer meshes it, each face turned outward first.
fn compute_brep_volume(brep: &BRep, place: &Xform) -> Result<f64, &'static str> {
    if !brep.is_solid() {
        return Err(OPEN);
    }

    let meshes = brep.face_meshes_q(Some(crate::app::walk::brep::QUALITY));
    let chains = edge_chains(brep, &meshes);
    let signs = face_signs(brep, &meshes, &chains);
    let Some(origin) = meshes.iter().find(|mesh| !mesh.vertex.is_empty()) else {
        return Err(OPEN);
    };
    let origin = origin_of(origin, place);
    let swept: f64 = meshes
        .iter()
        .zip(&signs)
        .map(|(mesh, sign)| sign * compute_swept(mesh, place, &origin))
        .sum();
    Ok(swept.abs() / 6.0)
}

/// A world point on the mesh, so far-away models keep their precision.
fn origin_of(mesh: &Mesh, place: &Xform) -> Point {
    mesh.faces()
        .first()
        .and_then(|face| mesh.vertex_point(*mesh.face[face].first()?))
        .map_or(Point::new(0.0, 0.0, 0.0), |p| p.transformed(place))
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::{Element, Line, NurbsSurface};
    use std::rc::Rc;

    fn near(value: f64, expected: f64, relative: f64) -> bool {
        (value - expected).abs() <= relative * expected.abs().max(1.0)
    }

    fn measured(geometry: Geometry, place: &Xform) -> Option<Result<f64, &'static str>> {
        compute_volume(&geometry, place)
    }

    #[test]
    fn closed_meshes_measure_and_open_ones_are_refused() {
        let identity = Xform::identity();
        let mesh = Mesh::create_box(2.0, 2.0, 0.4);
        let volume = measured(Geometry::Mesh(Rc::new(mesh.clone())), &identity);
        assert!(near(volume.unwrap().unwrap(), 1.6, 1e-9));
        let far = &Xform::translation(1e6, -2e6, 5e5) * &Xform::scale_xyz(2.0, 2.0, 2.0);
        let volume = measured(Geometry::Mesh(Rc::new(mesh)), &far);
        assert!(near(volume.unwrap().unwrap(), 12.8, 1e-9));
        let corners = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
            Point::new(1.0, 0.0, 1.0),
            Point::new(1.0, 1.0, 1.0),
            Point::new(0.0, 1.0, 1.0),
        ];
        let sides = vec![
            vec![0, 3, 2, 1],
            vec![0, 1, 5, 4],
            vec![1, 2, 6, 5],
            vec![2, 3, 7, 6],
            vec![3, 0, 4, 7],
        ];
        let lidless = Mesh::from_vertices_and_faces(corners, sides);
        let volume = measured(Geometry::Mesh(Rc::new(lidless)), &identity);
        assert_eq!(volume, Some(Err(OPEN)));
    }

    #[test]
    fn solid_breps_measure_whatever_their_face_flags() {
        let identity = Xform::identity();
        let brep = BRep::create_box(2.0, 2.0, 0.4);
        let volume = measured(Geometry::BRep(Rc::new(brep.clone())), &identity);
        assert!(near(volume.unwrap().unwrap(), 1.6, 1e-9));
        let mut flipped = brep.clone();
        let face = &mut flipped.m_shells[0].faces[0];
        face.orientation = session_rust::brep::brep_reverse(face.orientation);
        let volume = measured(Geometry::BRep(Rc::new(flipped)), &identity);
        assert!(near(volume.unwrap().unwrap(), 1.6, 1e-9));
        let mut shell = brep.clone();
        shell.m_solids.clear();
        let volume = measured(Geometry::BRep(Rc::new(shell)), &identity);
        assert_eq!(volume, Some(Err(OPEN)));
        let cylinder = Geometry::BRep(Rc::new(BRep::create_cylinder(150.0, 400.0)));
        let exact = std::f64::consts::PI * 150.0 * 150.0 * 400.0;
        assert!(near(
            measured(cylinder, &identity).unwrap().unwrap(),
            exact,
            5e-3
        ));
        let element = Geometry::Element(Rc::new(Element::from_brep(brep, "beam")));
        assert!(near(
            measured(element, &identity).unwrap().unwrap(),
            1.6,
            1e-9
        ));
        let corners = [
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
        ];
        let surface = NurbsSurface::create(false, false, 1, 1, 2, 2, &corners).unwrap();
        let surface = Geometry::NurbsSurface(Rc::new(surface));
        assert_eq!(measured(surface, &identity), Some(Err(OPEN)));
        let line = Geometry::Line(Rc::new(Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0)));
        assert_eq!(measured(line, &identity), None);
    }
}
