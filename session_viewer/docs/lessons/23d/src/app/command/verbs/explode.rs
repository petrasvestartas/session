use crate::State;
use crate::app::command::{Action, Spec};
use crate::app::layers::{self, owned};
use crate::app::modeling::MAX_POINTS;
use crate::app::scene::{Scene, Shape, sync};
use session_rust::{BRep, BRepRef, Geometry, Line, Mesh};
use std::collections::HashMap;
use std::rc::Rc;

pub const SPEC: Spec = Spec {
    names: &["Explode"],
    aliases: &[],
    hint: "Explode · a polyline into lines, a BRep into faces, a mesh into faces, a point cloud into points",
    options: &[],
    arity: Some(0),
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// Break the selection into its parts.
fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Explode))
}

#[derive(Debug)]
struct Explode;

impl Action for Explode {
    /// Replace each selected object by its parts in one undo step; the parts end up selected.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let rows = state.selected_rows();

        if let Some(reason) = state.locked_reason(&rows) {
            return Err(reason);
        }

        if crate::app::deform::Target::selected(&state.selection).is_some() {
            return Err("Explode works on whole objects; press Esc to leave the face, edge or control point".into());
        }

        let (pieces, message) = state.scene.explode_rows(&rows)?;
        state.commit_rows();
        let rows = pieces
            .iter()
            .filter_map(|(doc, guid)| state.scene.row_of(*doc, guid))
            .collect();
        state.select_rows(rows, false);
        Ok(message)
    }

    fn needs_selection(&self) -> bool {
        true
    }
}

impl Scene {
    /// Replace each of `rows` by its parts under the same parent, with its local transform, name and colours.
    pub(crate) fn explode_rows(
        &mut self,
        rows: &[u32],
    ) -> Result<(Vec<(usize, Rc<str>)>, String), String> {
        let mut sources = Vec::with_capacity(rows.len()); // (document, guid, parts)
        let mut refused = None;
        let mut total = 0;

        for &row in rows {
            let (doc, guid) = self.identity_of(row).ok_or("An object no longer exists")?;

            let file = self.docs.get(doc).ok_or("This object cannot be exploded")?;

            if file.display_only {
                return Err(crate::app::scene::READ_ONLY.into());
            }

            if self
                .node_of(row)
                .is_some_and(|(node, _)| !node.borrow().is_leaf())
            {
                return Err("Explode needs an object without child objects".into());
            }

            let geometry = self.geometry(row).ok_or("This object cannot be exploded")?;

            match pieces(geometry) {
                Ok(parts) => {
                    total += parts.len();
                    sources.push((doc, guid, parts));
                }
                Err(reason) => refused = Some(reason),
            }

            if total > MAX_POINTS {
                return Err(format!(
                    "Explode makes at most {MAX_POINTS} objects at once"
                ));
            }
        }

        if sources.is_empty() {
            return Err(refused.unwrap_or_else(|| "Nothing to explode".into()));
        }

        let mut docs: Vec<usize> = sources.iter().map(|source| source.0).collect();
        docs.sort_unstable();
        docs.dedup();
        let mut made = Vec::with_capacity(total);

        for &doc in &docs {
            let mine: Vec<_> = sources.iter().filter(|source| source.0 == doc).collect();
            let parents: Vec<_> = mine
                .iter()
                .map(|(_, guid, _)| self.parent_of(doc, guid))
                .collect();
            let session = Rc::make_mut(&mut self.docs[doc].session);
            let mut nodes = Vec::with_capacity(mine.len());
            session.begin("explode");

            for ((_, guid, parts), parent) in mine.iter().zip(parents) {
                let local = session.xform(guid); // the parts keep the placement
                let Some(parent) = owned(session, parent).or_else(|| session.tree.root()) else {
                    continue;
                };

                if !session.remove_object(guid) {
                    continue;
                }

                for part in parts {
                    let Some(node) = layers::add(session, part, &parent, false) else {
                        continue;
                    };
                    let id: Rc<str> = Rc::from(node.borrow().name.as_str());
                    session.set_xform(&id, local.clone());
                    nodes.push((node, (doc, Rc::clone(guid)), (doc, id)));
                }
            }

            let notes = sync::commit(session);
            self.noted(doc, notes);

            for (node, from, to) in nodes {
                self.hint(doc, &node);
                self.inherit(&from, &to, false);
                made.push(to);
            }
        }

        self.edited(&docs);
        let mut message = format!("Exploded into {} objects", made.len());

        if let Some(reason) = refused {
            message = format!("{message} · {reason}");
        }

        Ok((made, message))
    }
}

/// The parts of one geometry: segments, faces or points.
fn pieces(geometry: &Geometry) -> Result<Vec<Geometry>, String> {
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
            Shape::of(geometry).label()
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
mod tests {
    use super::*;
    use crate::app::scene::FileDoc;
    use session_rust::{Color, Point, PointCloud, Polyline, Session, Vector, Xform};

    /// A scene of one document holding `session`.
    fn scene(session: Session) -> Scene {
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "test".into(),
            session: Rc::new(session),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        scene
    }

    /// The first row holding a geometry of this kind.
    fn row(scene: &Scene, kind: fn(&Geometry) -> bool) -> u32 {
        (0..scene.object_count() as u32)
            .find(|&row| scene.geometry(row).is_some_and(kind))
            .unwrap()
    }

    /// A square as three points; the corner repeated makes a zero-length segment.
    fn square() -> Polyline {
        Polyline::new(vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
            Point::new(0.0, 0.0, 0.0),
        ])
    }

    /// Explode is one undo step and keeps the placement.
    #[test]
    fn explode_is_one_transaction_and_preserves_placement() {
        let mut session = Session::new("test");
        session
            .add_polyline(
                Polyline::new(vec![
                    Point::new(0.0, 0.0, 0.0),
                    Point::new(1.0, 0.0, 0.0),
                    Point::new(1.0, 1.0, 0.0),
                ]),
                None,
            )
            .unwrap();
        let mut scene = scene(session);
        let (_, guid) = scene.identity_of(0).unwrap();
        Rc::make_mut(&mut scene.docs[0].session)
            .set_xform(&guid, Xform::translation(5.0, 0.0, 0.0));
        scene.explode_rows(&[0]).unwrap();
        let session = &scene.docs[0].session;
        assert_eq!(session.lookup.len(), 2);

        for guid in session.lookup.keys() {
            assert_eq!(session.world_xform(guid).m[12], 5.0);
        }

        assert!(scene.undo());
        assert_eq!(scene.docs[0].session.lookup.len(), 1);
        assert!(matches!(
            scene.docs[0].session.lookup.get(guid.as_ref()),
            Some(Geometry::Polyline(_))
        ));
        assert!(scene.redo());
        assert_eq!(scene.docs[0].session.lookup.len(), 2);
    }

    /// The lines stay in the polyline's group with its name and colour override; zero-length segments are skipped.
    #[test]
    fn a_polyline_becomes_its_segments_in_its_group() {
        let mut session = Session::new("test");
        let group = session.add_group("parts");
        let mut line = square();
        line.name = "outline".into();
        session.add_polyline(line, Some(&group)).unwrap();
        let mut scene = scene(session);
        let target = row(&scene, |g| matches!(g, Geometry::Polyline(_)));
        let original = scene.identity_of(target).unwrap();
        scene.edge_colors.insert(original.clone(), [230, 65, 55]);
        let (made, _) = scene.explode_rows(&[target]).unwrap();
        assert_eq!(made.len(), 4);
        let session = &scene.docs[0].session;
        let parts = session.tree.get_node_by_name("parts").unwrap();
        assert_eq!(parts.borrow().children().len(), 4);

        for id in &made {
            assert_eq!(session.lookup[id.1.as_ref()].name(), "outline");
            assert_eq!(scene.edge_colors.get(id), Some(&[230, 65, 55]));
        }

        assert!(scene.undo());
        assert!(
            scene.docs[0]
                .session
                .lookup
                .contains_key(original.1.as_ref())
        );
    }

    /// Every face comes out a valid one-face BRep and the areas add up; seams and poles survive too.
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

    /// A parent object and a cap on the part count are refused, and nothing changes.
    #[test]
    fn parents_and_huge_results_are_refused() {
        let mut session = Session::new("test");
        let outer = session.add_line(Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0), None);
        session.add_polyline(square(), Some(&outer)).unwrap();
        let mut scene = scene(session);
        let parent = row(&scene, |g| matches!(g, Geometry::Line(_)));
        assert!(scene.explode_rows(&[parent]).is_err());
        let long: Vec<Point> = (0..=MAX_POINTS + 1)
            .map(|i| Point::new(i as f64, 0.0, 0.0))
            .collect();
        assert!(pieces(&Geometry::Polyline(Rc::new(Polyline::new(long)))).is_err());
        assert_eq!(scene.docs[0].session.lookup.len(), 2);
    }
}
