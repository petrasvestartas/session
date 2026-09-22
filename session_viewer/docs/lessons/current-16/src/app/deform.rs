use super::selection::{ControlId, Controls, SelectionMode};
use session_rust::{Geometry, Mesh, NurbsSurface, Point, Xform};
use std::collections::HashSet;
use std::rc::Rc;

/// The part of a geometry an edit applies to.
#[derive(Clone, Copy, Debug)]
pub enum Target {
    Control(ControlId), // one control point or vertex
    Edge(u32),          // one edge by index
    Face(usize),        // one face by key
}

impl Target {
    /// The target the current selection names, if any.
    pub fn selected(mode: &SelectionMode) -> Option<Self> {
        match *mode {
            SelectionMode::Controls {
                selected: Some(id), ..
            } => Some(Self::Control(id)),
            SelectionMode::Edge { edge, .. } => Some(Self::Edge(edge)),
            SelectionMode::Face { face, .. } => Some(Self::Face(face)),
            _ => None,
        }
    }
}

// --8<-- [start:step-2a]
/// The mesh vertex keys a target covers.
pub(crate) fn mesh_keys(mesh: &Mesh, target: Target) -> Result<Vec<usize>, String> {
// --8<-- [end:step-2a]
    match target {
        Target::Control(ControlId::Vertex(key)) if mesh.vertex.contains_key(&key) => Ok(vec![key]),
        Target::Face(key) => {
            let mut keys = mesh.face.get(&key).ok_or("Unknown mesh face")?.clone();

            if let Some(holes) = mesh.face_holes.get(&key) {
                keys.extend(holes.iter().flatten()); // hole rims move with the face
            }

            keys.sort_unstable();
            keys.dedup();
            Ok(keys)
        }
        Target::Edge(index) => {
            // edges are numbered in order of first appearance
            let mut seen = HashSet::new();
            let mut at = 0;

            for face in mesh.faces() {
                let keys = &mesh.face[&face];

                for i in 0..keys.len() {
                    let (a, b) = (keys[i], keys[(i + 1) % keys.len()]);
                    let pair = (a.min(b), a.max(b));

                    if seen.insert(pair) {
                        if at == index {
                            return Ok(vec![pair.0, pair.1]);
                        }

                        at += 1;
                    }
                }
            }

            Err("Unknown mesh edge".into())
        }
        _ => Err("Select a mesh vertex, edge or face".into()),
    }
}

/// The surface control (u, v) pairs a target covers.
fn surface_keys(surface: &NurbsSurface, target: Target) -> Result<Vec<(usize, usize)>, String> {
    let [nu, nv] = surface.m_cv_count; // control grid size
    let all = || (0..nu).flat_map(|u| (0..nv).map(move |v| (u, v))).collect();

    match target {
        Target::Control(ControlId::Surface { surface: 0, u, v }) if u < nu && v < nv => {
            Ok(vec![(u, v)])
        }
        Target::Face(0) => Ok(all()),
        // edge 0..3: the four sides of the control grid
        Target::Edge(edge) if edge < 4 => Ok((0..nu)
            .flat_map(|u| (0..nv).map(move |v| (u, v)))
            .filter(|&(u, v)| match edge {
                0 => u == 0,
                1 => u + 1 == nu,
                2 => v == 0,
                _ => v + 1 == nv,
            })
            .collect()),
        _ => Err("Unknown surface control, boundary or face".into()),
    }
}

/// The world points a target covers.
pub fn points(geometry: &Geometry, target: Target) -> Result<Vec<Point>, String> {
    match geometry {
        Geometry::Mesh(mesh) => mesh_keys(mesh, target)?
            .into_iter()
            .map(|k| mesh.vertex_point(k).ok_or("Missing mesh vertex".into()))
            .collect(),
        Geometry::NurbsSurface(surface) => surface_keys(surface, target)?
            .into_iter()
            .map(|(u, v)| surface.get_cv(u, v).ok_or("Missing surface control".into()))
            .collect(),
        Geometry::BRep(brep) => match target {
            Target::Face(face) => {
                let face = brep.m_faces.get(face).ok_or("Unknown BRep face")?;
                let surface = brep
                    .m_surfaces
                    .get(face.surface_index as usize)
                    .ok_or("Missing face surface")?;
                Ok((0..surface.m_cv_count[0])
                    .flat_map(|u| {
                        (0..surface.m_cv_count[1]).filter_map(move |v| surface.get_cv(u, v))
                    })
                    .collect())
            }
            Target::Edge(edge) => {
                let edge = brep.m_edges.get(edge as usize).ok_or("Unknown BRep edge")?;
                let curve = brep
                    .m_curves_3d
                    .get(edge.curve_3d_index as usize)
                    .ok_or("Degenerate edge has no movable curve")?;
                Ok((0..curve.cv_count())
                    .filter_map(|i| curve.get_cv(i))
                    .collect())
            }
            Target::Control(id) => control_point(geometry, id).map(|p| vec![p]),
        },
        Geometry::Element(element) => match element.geometry() {
            session_rust::element::ElementGeometry::Mesh(mesh) => {
                points(&Geometry::Mesh(Rc::new(mesh.clone())), target)
            }
            session_rust::element::ElementGeometry::BRep(brep) => {
                points(&Geometry::BRep(Rc::new(brep.clone())), target)
            }
            _ => Err("Element has no source geometry".into()),
        },
        _ => match target {
            Target::Control(id) => control_point(geometry, id).map(|p| vec![p]),
            _ => Err("This source has no editable faces or edges".into()),
        },
    }
}

/// One control point of any geometry.
fn control_point(geometry: &Geometry, id: ControlId) -> Result<Point, String> {
    Controls::from_geometry(geometry)
        .points
        .into_iter()
        .find(|p| p.id == id)
        .map(|p| Point::new(p.position[0], p.position[1], p.position[2]))
        .ok_or("Unknown source control".into())
}

/// A copy of the geometry with the target moved by `delta`.
pub fn transform(geometry: &Geometry, target: Target, delta: &Xform) -> Result<Geometry, String> {
    if !delta.m.iter().all(|v| v.is_finite()) {
        return Err("Transform must be finite".into());
    }

    let edited = match geometry {
        Geometry::Mesh(source) => {
            let mut mesh = (**source).clone();

            for key in mesh_keys(&mesh, target)? {
                let point = mesh
                    .vertex_point(key)
                    .ok_or("Missing vertex")?
                    .transformed(delta);
                mesh.vertex
                    .get_mut(&key)
                    .ok_or("Missing vertex")?
                    .set_position(point);
            }

            // --8<-- [start:step-2b]
            mesh.clear_triangle_bvh(); // stale after moving vertices
            // --8<-- [end:step-2b]
            Geometry::Mesh(Rc::new(mesh))
        }
        Geometry::NurbsSurface(source) => {
            let mut surface = (**source).clone();

            for (u, v) in surface_keys(&surface, target)? {
                let p = surface
                    .get_cv(u, v)
                    .ok_or("Missing surface control")?
                    .transformed(delta);

                if !surface.set_cv(u, v, &p) {
                    return Err("Cannot set surface control".into());
                }
            }

            surface.m_mesh = None; // drop the cached mesh
            Geometry::NurbsSurface(Rc::new(surface))
        }
        Geometry::Polyline(source) => {
            let Target::Control(ControlId::Vertex(i)) = target else {
                return Err("Select a polyline vertex".into());
            };
            let p = source
                .get_point(i)
                .ok_or("Unknown polyline vertex")?
                .transformed(delta);
            let mut next = (**source).clone();
            next.set_point(i, &p);
            Geometry::Polyline(Rc::new(next))
        }
        Geometry::NurbsCurve(source) => {
            let Target::Control(ControlId::Curve { curve: 0, point }) = target else {
                return Err("Select a curve control point".into());
            };
            let p = source
                .get_cv(point)
                .ok_or("Unknown curve control")?
                .transformed(delta);
            let mut next = (**source).clone();

            if !next.set_cv_point(point, &p) {
                return Err("Cannot set curve control".into());
            }

            Geometry::NurbsCurve(Rc::new(next))
        }
        Geometry::Line(source) => {
            let Target::Control(ControlId::Vertex(i @ 0..=1)) = target else {
                return Err("Select a line endpoint".into());
            };
            let mut ends = [source.start(), source.end()];
            ends[i] = ends[i].transformed(delta);
            let mut next = session_rust::Line::from_points(&ends[0], &ends[1]);
            next.set_guid(source.guid().to_string());
            next.name = source.name.clone();
            next.width = source.width;
            next.dash = source.dash.clone();
            next.linecolor = source.linecolor.clone();
            Geometry::Line(Rc::new(next))
        }
        Geometry::Point(source) => {
            let Target::Control(ControlId::Vertex(0)) = target else {
                return Err("Unknown point".into());
            };
            Geometry::Point(Rc::new(source.transformed(delta)))
        }
        Geometry::BRep(source) => {
            let selected = points(geometry, target)?;
            let mut next = (**source).clone();
            // every vertex, curve or surface point at a selected position moves
            let matches = |p: &Point| {
                selected
                    .iter()
                    .any(|s| (0..3).all(|i| (s[i] - p[i]).abs() <= 1e-8))
            };

            for vertex in &mut next.m_vertices {
                if matches(&vertex.point) {
                    vertex.point = vertex.point.transformed(delta);
                }
            }

            let mut changed_curves = HashSet::new(); // curves touched
            let mut changed_surfaces = HashSet::new(); // surfaces touched

            for (curve_index, curve) in next.m_curves_3d.iter_mut().enumerate() {
                for i in 0..curve.cv_count() {
                    if let Some(p) = curve.get_cv(i)
                        && matches(&p)
                    {
                        curve.set_cv_point(i, &p.transformed(delta));
                        changed_curves.insert(curve_index);
                    }
                }
            }

            for (surface_index, surface) in next.m_surfaces.iter_mut().enumerate() {
                for u in 0..surface.m_cv_count[0] {
                    for v in 0..surface.m_cv_count[1] {
                        if let Some(p) = surface.get_cv(u, v)
                            && matches(&p)
                        {
                            surface.set_cv(u, v, &p.transformed(delta));
                            changed_surfaces.insert(surface_index);
                        }
                    }
                }

                if changed_surfaces.contains(&surface_index) {
                    surface.m_mesh = None; // drop the cached mesh
                }
            }

            validate_boundaries(&next, &changed_curves, &changed_surfaces)?;
            Geometry::BRep(Rc::new(next))
        }
        Geometry::Element(source) => {
            let mut next = (**source).clone();

            match source.geometry() {
                session_rust::element::ElementGeometry::Mesh(mesh) => {
                    let Geometry::Mesh(mesh) =
                        transform(&Geometry::Mesh(Rc::new(mesh.clone())), target, delta)?
                    else {
                        unreachable!()
                    };
                    next.set_geometry((*mesh).clone());
                }
                session_rust::element::ElementGeometry::BRep(brep) => {
                    let Geometry::BRep(brep) =
                        transform(&Geometry::BRep(Rc::new(brep.clone())), target, delta)?
                    else {
                        unreachable!()
                    };
                    next.set_brep_geometry((*brep).clone());
                }
                _ => return Err("Element has no source geometry".into()),
            }

            Geometry::Element(Rc::new(next))
        }
        _ => return Err("This source control cannot be transformed".into()),
    };
    Ok(edited)
}

/// Refuse an edit whose edges no longer lie on their surfaces.
fn validate_boundaries(
    brep: &session_rust::BRep,
    changed_curves: &HashSet<usize>,
    changed_surfaces: &HashSet<usize>,
) -> Result<(), String> {
    if !brep.is_valid() {
        return Err("Edit would invalidate BRep topology".into());
    }

    for edge in &brep.m_edges {
        if edge.degenerated
            || (!changed_curves.contains(&(edge.curve_3d_index as usize))
                && !edge
                    .pcurves
                    .iter()
                    .any(|pc| changed_surfaces.contains(&(pc.surface_index as usize))))
        {
            continue;
        }

        let curve = &brep.m_curves_3d[edge.curve_3d_index as usize];
        let (a, b) = curve.domain();

        // sample the 3D edge against each surface's 2D curve
        for pc in &edge.pcurves {
            let surface = &brep.m_surfaces[pc.surface_index as usize];

            for ci in [pc.curve_2d_index, pc.curve_2d_index_2] {
                if ci < 0 {
                    continue;
                }

                let uv = &brep.m_curves_2d[ci as usize];
                let (u0, u1) = uv.domain();

                for sample in 0..=16 {
                    let t = sample as f64 / 16.0;
                    let p = curve.point_at(a + (b - a) * t);
                    let q = uv.point_at(u0 + (u1 - u0) * t);
                    let actual = surface
                        .point_at(q[0], q[1])
                        .ok_or("Cannot evaluate incident surface")?;
                    let tolerance = edge.tolerance.max(1e-6) * 10.0;

                    if (0..3).any(|i| (p[i] - actual[i]).abs() > tolerance) {
                        return Err("This BRep edit requires rebuilding adjacent trims; the original solid was preserved".into());
                    }
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A quad and a triangle sharing an edge.
    fn mesh() -> Geometry {
        let mut mesh = Mesh::new();

        for (key, p) in [
            (10, [0., 0., 0.]),
            (20, [10., 0., 0.]),
            (30, [10., 10., 0.]),
            (40, [0., 10., 0.]),
            (90, [0., 0., 10.]),
        ] {
            mesh.add_vertex(Point::new(p[0], p[1], p[2]), Some(key));
        }

        mesh.add_face(vec![10, 20, 30, 40], Some(7));
        mesh.add_face(vec![10, 90, 20], Some(19));
        Geometry::Mesh(Rc::new(mesh))
    }

    /// Moving a face moves its vertices only.
    #[test]
    fn mesh_face_moves_shared_source_vertices_not_unrelated_vertices() {
        let source = mesh();
        let Geometry::Mesh(next) =
            transform(&source, Target::Face(7), &Xform::translation(0., 0., 3.)).unwrap()
        else {
            panic!()
        };

        for key in [10, 20, 30, 40] {
            assert_eq!(next.vertex[&key].z, 3.);
        }

        assert_eq!(next.vertex[&90].z, 10.);
        assert_eq!(next.face[&19], vec![10, 90, 20]);
        let Geometry::Mesh(original) = source else {
            panic!()
        };
        assert_eq!(original.vertex[&10].z, 0.);
    }

    /// Edge numbering matches the display edges.
    #[test]
    fn edge_ids_match_the_display_producer_even_with_sparse_keys() {
        let Geometry::Mesh(mesh) = mesh() else {
            panic!()
        };
        let keys = mesh.vertices();
        let positions: Vec<_> = keys
            .iter()
            .map(|k| {
                let p = mesh.vertex_point(*k).unwrap();
                [p[0], p[1], p[2]]
            })
            .collect();
        let slots = super::super::walk::mesh_topology::SlotMap::new(&keys);
        let topo =
            super::super::walk::mesh_topology::mesh_topology(&mesh, &keys, &positions, &slots);

        for (i, &(a, b, _)) in topo.edges.iter().enumerate() {
            assert_eq!(
                mesh_keys(&mesh, Target::Edge(i as u32)).unwrap(),
                vec![a, b]
            );
        }
    }

    /// Moving one surface edge keeps weights and the other edge.
    #[test]
    fn surface_boundary_preserves_weights_and_the_opposite_boundary() {
        let mut surface = NurbsSurface::create(
            false,
            false,
            1,
            1,
            2,
            2,
            &[
                Point::new(0., 0., 0.),
                Point::new(1., 0., 0.),
                Point::new(0., 1., 0.),
                Point::new(1., 1., 0.),
            ],
        )
        .unwrap();
        assert!(surface.make_rational());

        for u in 0..2 {
            for v in 0..2 {
                surface.set_cv_4d(u, v, u as f64 * 2., v as f64 * 2., 0., 2.);
            }
        }

        let Geometry::NurbsSurface(next) = transform(
            &Geometry::NurbsSurface(Rc::new(surface)),
            Target::Edge(0),
            &Xform::translation(0., 0., 5.),
        )
        .unwrap() else {
            panic!()
        };
        assert_eq!(next.get_cv(0, 0).unwrap()[2], 5.);
        assert_eq!(next.get_cv(1, 0).unwrap()[2], 0.);
        assert!(next.m_is_rat);
        assert_eq!(next.weight(0, 0), 2.);
        assert_eq!(next.weight(1, 1), 2.);
    }

    /// Moving a box face keeps the solid valid.
    #[test]
    fn moving_box_face_keeps_edges_on_incident_surfaces() {
        let source = Geometry::BRep(Rc::new(session_rust::BRep::create_box(10., 20., 30.)));
        let Geometry::BRep(next) =
            transform(&source, Target::Face(0), &Xform::translation(0., 0., 2.)).unwrap()
        else {
            panic!()
        };
        assert!(next.is_valid());
        assert!(next.is_closed(0));
        validate_boundaries(
            &next,
            &(0..next.m_curves_3d.len()).collect(),
            &(0..next.m_surfaces.len()).collect(),
        )
        .unwrap();
    }
}
