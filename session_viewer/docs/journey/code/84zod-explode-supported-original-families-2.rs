use session_rust::{Geometry,Line,Mesh,BRep,BRepRef};
use std::{rc::Rc,collections::HashMap};
use crate::interval_geometry::MAX_POINTS;
pub fn pieces(geometry: &Geometry) -> Result<Vec<Geometry>, String> {
    let count = match geometry {
        Geometry::Polyline(line) => line.point_count().saturating_sub(1),
        Geometry::BRep(brep) => brep.face_count(),
        Geometry::Mesh(mesh) => mesh.number_of_faces(),
        Geometry::PointCloud(cloud) => cloud.point_count(),
        _ => 0,
    };

    // counted before anything is made
    if count > MAX_POINTS {
        return Err(format!(
            "Explode makes at most {MAX_POINTS} objects at once"
        ));
    }

    let parts: Vec<Geometry> = match geometry {
        Geometry::Polyline(line) => line
            .get_points()
            .windows(2)
            .filter(|pair| pair[0].distance(&pair[1], None) > 1e-12)
            .map(|pair| {
                let mut part = Line::from_points(&pair[0], &pair[1]);
                part.name = line.name.clone();
                part.width = line.width;
                part.dash = line.dash.clone();
                part.linecolor = line.linecolor.clone();
                Geometry::Line(Rc::new(part))
            })
            .collect(),
        Geometry::BRep(brep) if brep.face_count() > 1 => (0..brep.face_count())
            .map(|face| face_brep(brep, face).map(|part| Geometry::BRep(Rc::new(part))))
            .collect::<Option<_>>()
            .ok_or("This BRep's tables do not resolve")?,
        Geometry::Mesh(mesh) if mesh.number_of_faces() > 1 => mesh
            .faces()
            .into_iter()
            .map(|face| face_mesh(mesh, face).map(|part| Geometry::Mesh(Rc::new(part))))
            .collect::<Option<_>>()
            .ok_or("This mesh's faces do not resolve")?,
        Geometry::PointCloud(cloud) => (0..cloud.point_count())
            .map(|index| {
                let mut point = cloud.get_point(index);
                point.name = cloud.name.clone();
                point.width = cloud.point_size;

                if index < cloud.color_count() {
                    point.pointcolor = cloud.get_color(index);
                }

                Geometry::Point(Rc::new(point))
            })
            .collect(),
        _ => Vec::new(),
    };

    if parts.len() < 2 {
        return Err(format!(
            "{} cannot be exploded",
            crate::source_shape::Shape::of(geometry).label()
        ));
    }

    Ok(parts)
}

/// One face of a mesh as a mesh of its own, holes included.
fn face_mesh(mesh: &Mesh, face: usize) -> Option<Mesh> {
    let outer = mesh.face_vertices(face)?;
    let holes = mesh.face_holes.get(&face);
    let mut index = HashMap::new(); // old vertex key to new
    let mut points = Vec::new();

    for &key in outer.iter().chain(holes.into_iter().flatten().flatten()) {
        if !index.contains_key(&key) {
            index.insert(key, points.len());
            points.push(mesh.vertex_point(key)?);
        }
    }

    let ring = outer.iter().map(|key| index[key]).collect();
    let mut part = Mesh::from_vertices_and_faces(points, vec![ring]);

    if let Some(holes) = holes {
        let rings = holes
            .iter()
            .map(|hole| hole.iter().map(|key| index[key]).collect())
            .collect();
        let key = *part.faces().first()?;
        part.set_face_holes(key, rings);
    }

    part.name = mesh.name.clone();
    part.set_objectcolor(mesh.get_objectcolor().clone());
    Some(part)
}

/// One face of a BRep as a BRep of its own: its surface, wires, edges, vertices and pcurves on that surface.
fn face_brep(brep: &BRep, index: usize) -> Option<BRep> {
    let face = brep.m_faces.get(index)?;
    let surface = face.surface_index;
    let mut part = BRep::new();
    part.name = brep.name.clone();
    part.width = brep.width;
    part.surfacecolor = brep.surfacecolor.clone();
    part.add_surface(brep.m_surfaces.get(usize::try_from(surface).ok()?)?);
    let mut vertices: HashMap<i32, i32> = HashMap::new(); // old index to new
    let mut edges: HashMap<i32, i32> = HashMap::new();
    let mut wires = Vec::with_capacity(face.wires.len());

    for wire in &face.wires {
        let uses = &brep.m_wires.get(usize::try_from(wire.index).ok()?)?.edges;
        let mut refs = Vec::with_capacity(uses.len());

        for used in uses {
            let edge = match edges.get(&used.index) {
                Some(&edge) => edge,
                None => {
                    let edge = copy_edge(brep, &mut part, used.index, surface, &mut vertices)?;
                    edges.insert(used.index, edge);
                    edge
                }
            };
            refs.push(BRepRef::new(edge, used.orientation));
        }

        wires.push(BRepRef::new(part.add_wire(&refs) as i32, wire.orientation));
    }

    let made = part.add_face(0, &wires, face.tolerance);
    part.m_faces[made].facecolor = face.facecolor.clone();
    part.add_shell(&[BRepRef::new(0, brep.face_orientation(index))]);
    Some(part)
}

/// Copy edge `old` into `part`: its curve, its two vertices once each, and its pcurves on `surface`.
fn copy_edge(
    brep: &BRep,
    part: &mut BRep,
    old: i32,
    surface: i32,
    vertices: &mut HashMap<i32, i32>,
) -> Option<i32> {
    let edge = brep.m_edges.get(usize::try_from(old).ok()?)?;
    let curve = match usize::try_from(edge.curve_3d_index) {
        Ok(curve) if !edge.degenerated => part.add_curve_3d(brep.m_curves_3d.get(curve)?) as i32,
        _ => -1, // a pole or apex has no curve
    };
    let mut ends = [0; 2];

    for (end, old) in ends.iter_mut().zip([edge.start_vertex, edge.end_vertex]) {
        *end = match vertices.get(&old) {
            Some(&vertex) => vertex,
            None => {
                let source = brep.m_vertices.get(usize::try_from(old).ok()?)?;
                let vertex = part.add_vertex(&source.point, source.tolerance) as i32;
                vertices.insert(old, vertex);
                vertex
            }
        };
    }

    let copied = part.add_edge(curve, ends[0], ends[1]);
    part.m_edges[copied].tolerance = edge.tolerance;
    part.m_edges[copied].degenerated = edge.degenerated;

    for pcurve in edge
        .pcurves
        .iter()
        .filter(|pcurve| pcurve.surface_index == surface)
    {
        let first = part.add_curve_2d(
            brep.m_curves_2d
                .get(usize::try_from(pcurve.curve_2d_index).ok()?)?,
        ) as i32;
        // a seam runs twice over the surface, once each way
        let second = match usize::try_from(pcurve.curve_2d_index_2) {
            Ok(seam) => part.add_curve_2d(brep.m_curves_2d.get(seam)?) as i32,
            Err(_) => -1,
        };
        part.add_pcurve(copied, 0, first, second);
    }

    Some(copied as i32)
}


#[cfg(test)]
mod tests{use super::*;use session_rust::{Point,Vector,Color,PointCloud};
#[test]
    fn a_brep_becomes_its_faces() {
        let area = |brep: &BRep| {
            brep.face_meshes_q(None)
                .iter()
                .map(|mesh| mesh.area())
                .sum::<f64>()
        };

        for brep in [
            BRep::create_box(2.0, 3.0, 4.0),
            BRep::create_cylinder(1.0, 2.0),
            BRep::create_sphere(1.0),
        ] {
            let parts: Vec<BRep> = (0..brep.face_count())
                .map(|face| face_brep(&brep, face).unwrap())
                .collect();
            let mut total = 0.0;

            for part in &parts {
                assert_eq!(part.face_count(), 1);
                assert!(part.is_valid());
                assert!(!part.face_meshes_q(None).is_empty());
                total += area(part);
            }

            // a smaller object tessellates a little finer
            assert!(
                (total - area(&brep)).abs() < 1e-2 * area(&brep),
                "{total} {}",
                area(&brep)
            );
        }

        let sphere = Geometry::BRep(Rc::new(BRep::create_sphere(1.0)));
        assert!(pieces(&sphere).is_err(), "one face is nothing to explode");
        assert_eq!(
            pieces(&Geometry::BRep(Rc::new(BRep::create_box(2.0, 3.0, 4.0))))
                .unwrap()
                .len(),
            6
        );
    }

    /// Clouds give coloured points, meshes their faces; a line is one piece and refused.
    #[test]
    fn clouds_and_meshes_explode_and_lines_do_not() {
        let points: Vec<Point> = (0..25).map(|i| Point::new(i as f64, 0.0, 0.0)).collect();
        let colors = vec![Color::red(); 25];
        let mut cloud = PointCloud::new(points, vec![Vector::new(0.0, 0.0, 1.0); 25], colors);
        cloud.name = "scan".into();
        let parts = pieces(&Geometry::PointCloud(Rc::new(cloud))).unwrap();
        assert_eq!(parts.len(), 25);
        let Geometry::Point(point) = &parts[3] else {
            panic!()
        };
        assert_eq!(
            (point.name.as_str(), point[0], point.pointcolor.r),
            ("scan", 3.0, 1.0)
        );
        assert_eq!(
            pieces(&Geometry::Mesh(Rc::new(Mesh::create_box(1.0, 1.0, 1.0))))
                .unwrap()
                .len(),
            6
        );
        let line = Geometry::Line(Rc::new(Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0)));
        assert_eq!(
            pieces(&line).err().as_deref(),
            Some("Line cannot be exploded")
        );
    }

}
