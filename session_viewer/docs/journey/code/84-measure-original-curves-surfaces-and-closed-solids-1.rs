use crate::measure_math::compute_swept;
use crate::cad_edges::edge_chains;
use crate::source_brep_orient::face_signs;
use session_rust::{BRep,Geometry,Mesh,Point,Xform};
use session_rust::element::ElementGeometry;
const OPEN:&str="open";
/// World volume of a closed mesh or solid BRep; Err when open, None when the kind has no volume.
pub fn compute_volume(geometry: &Geometry, place: &Xform) -> Option<Result<f64, &'static str>> {
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

    let meshes = brep.face_meshes_q(Some(crate::cad_boundary::QUALITY));
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

