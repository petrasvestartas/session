use super::scene::Scene;
use session_rust::simple_split;
use session_rust::{BRep, Geometry, NurbsCurve};
use std::rc::Rc;

/// True for a geometry that can cut: a line, polyline or curve.
pub fn is_cutter(geometry: &Geometry) -> bool {
    matches!(
        geometry,
        Geometry::Line(_) | Geometry::Polyline(_) | Geometry::NurbsCurve(_)
    )
}

/// The face to split: None for a curve, the selected face for a BRep.
pub fn face_index(geometry: &Geometry, selected: Option<usize>) -> Result<Option<usize>, String> {
    match geometry {
        Geometry::Line(_)
        | Geometry::Polyline(_)
        | Geometry::NurbsCurve(_)
        | Geometry::NurbsSurface(_) => Ok(None),
        Geometry::BRep(brep) => brep_face(brep, selected).map(Some),
        Geometry::Element(element) => match element.geometry() {
            session_rust::element::ElementGeometry::BRep(brep) => {
                brep_face(brep, selected).map(Some)
            }
            _ => Err("Split accepts curves and NURBS/BRep faces".into()),
        },
        _ => Err("Split accepts lines, polylines, NURBS curves and surface faces".into()),
    }
}

/// The selected face, or the only one.
fn brep_face(brep: &BRep, selected: Option<usize>) -> Result<usize, String> {
    selected
        .or((brep.face_count() == 1).then_some(0))
        .filter(|face| *face < brep.face_count())
        .ok_or_else(|| {
            "Ctrl+Shift-select one BRep face before Split; the solid stays joined".into()
        })
}

/// A cutter as a NURBS curve.
fn curve(geometry: &Geometry) -> Result<NurbsCurve, String> {
    match geometry {
        Geometry::Line(line) => Ok(NurbsCurve::create(
            false,
            1,
            &[line.point_at(0.), line.point_at(1.)],
        )),
        Geometry::Polyline(polyline) => Ok(NurbsCurve::create(false, 1, &polyline.get_points())),
        Geometry::NurbsCurve(curve) => Ok((**curve).clone()),
        _ => Err("Choose a line, polyline or NURBS curve as cutter".into()),
    }
}

impl Scene {
    /// Split `target` by the cutters; returns how many pieces.
    pub fn split_rows(
        &mut self,
        target: u32,
        face: Option<usize>,
        cutters: &[u32],
    ) -> Result<usize, String> {
        if !self.streamed.is_empty() || !self.sheets.is_empty() {
            return Err("Splitting requires complete retained source documents".into());
        }

        if cutters.is_empty() || cutters.len() > 64 {
            return Err("Choose 1–64 cutter curves".into());
        }

        if !self.selectable(target) {
            return Err("Unlock the target before splitting".into());
        }

        let (doc, guid) = self.identity_of(target).ok_or("Target no longer exists")?;
        let file = self.docs.get(doc).ok_or("Target has no source document")?;

        if file.display_only {
            return Err("Target is display only".into());
        }

        let back = self
            .placement_of(target)
            .ok_or("Target has no placement")?
            .inverse()
            .ok_or("Target placement is singular")?; // world into the target's frame
        let mut tools = Vec::new(); // cutters in the target's frame

        for &row in cutters {
            if row == target || !self.selectable(row) {
                return Err("Choose an unlocked cutter distinct from the target".into());
            }

            let mut cutter = curve(self.geometry(row).ok_or("Cutter no longer exists")?)?;
            let place = self.placement_of(row).ok_or("Cutter has no placement")?;

            if place.inverse().is_none() {
                return Err("Cannot transform the cutter into target coordinates".into());
            }

            cutter.transform(&(&back * &place));
            tools.push(cutter);
        }

        let source = self
            .geometry(target)
            .ok_or("Source geometry is unavailable")?;
        let face = face_index(source, face)?;
        let tolerance = 1e-6;
        // the new geometries and how many regions the cut made
        let (mut pieces, regions) = match source {
            Geometry::Line(line) => {
                let pieces: Vec<_> = simple_split::split_line_by_curves(line, &tools, tolerance)?
                    .into_iter()
                    .map(|p| Geometry::Line(Rc::new(p)))
                    .collect();
                let count = pieces.len();
                (pieces, count)
            }
            Geometry::Polyline(line) => {
                let pieces: Vec<_> =
                    simple_split::split_polyline_by_curves(line, &tools, tolerance)?
                        .into_iter()
                        .map(|p| Geometry::Polyline(Rc::new(p)))
                        .collect();
                let count = pieces.len();
                (pieces, count)
            }
            Geometry::NurbsCurve(curve) => {
                let pieces: Vec<_> = simple_split::split_curve_by_curves(curve, &tools, tolerance)?
                    .into_iter()
                    .map(|p| Geometry::NurbsCurve(Rc::new(p)))
                    .collect();
                let count = pieces.len();
                (pieces, count)
            }
            Geometry::NurbsSurface(surface) => {
                let mut brep = simple_split::split_surface_by_curves(surface, &tools, tolerance)?;
                brep.name = surface.name.clone();
                let count = brep.face_count();
                (vec![Geometry::BRep(Rc::new(brep))], count)
            }
            Geometry::BRep(brep) => {
                let next = simple_split::split_brep_face_by_curves(
                    brep,
                    face.ok_or("Select a face")?,
                    &tools,
                    tolerance,
                )?;
                let count = next.face_count() - brep.face_count() + 1;
                (vec![Geometry::BRep(Rc::new(next))], count)
            }
            Geometry::Element(element) => {
                let session_rust::element::ElementGeometry::BRep(brep) = element.geometry() else {
                    return Err("Element has no BRep".into());
                };
                let next = simple_split::split_brep_face_by_curves(
                    brep,
                    face.ok_or("Select a face")?,
                    &tools,
                    tolerance,
                )?;
                let count = next.face_count() - brep.face_count() + 1;
                let mut result = (**element).clone();
                result.set_brep_geometry(next);
                (vec![Geometry::Element(Rc::new(result))], count)
            }
            _ => return Err("Unsupported split target".into()),
        };

        if regions < 2 {
            return Ok(1); // nothing was cut
        }

        // the pieces inherit the parent, placement and colours
        let parent_name = file
            .session
            .tree
            .get_node_by_name(&guid)
            .and_then(|node| node.borrow().parent())
            .map(|node| node.borrow().name.clone());
        let place = file.session.xform(&guid);
        let color = self.colors.get(&(doc, Rc::clone(&guid))).copied();

        // name the pieces `x (part 1)`, `x (part 2)`...
        if pieces.len() > 1 {
            let name = source.name();

            for (index, piece) in pieces.iter_mut().enumerate() {
                let name = format!("{name} (part {})", index + 1);

                match piece {
                    Geometry::Line(p) => Rc::make_mut(p).name = name,
                    Geometry::Polyline(p) => Rc::make_mut(p).name = name,
                    Geometry::NurbsCurve(p) => Rc::make_mut(p).name = name,
                    _ => unreachable!("only curves create sibling objects"),
                }
            }
        }

        // the first piece replaces the target, the rest are added beside it
        let first = pieces.remove(0);
        let session = Rc::make_mut(&mut self.docs[doc].session);
        let parent = parent_name.and_then(|name| session.tree.get_node_by_name(&name));
        session.begin("split");
        let replaced = session.replace(&guid, first);
        debug_assert!(replaced);

        for piece in pieces {
            let node = match piece {
                Geometry::Line(piece) => session.add_line((*piece).clone(), parent.as_ref()),
                Geometry::Polyline(piece) => session
                    .add_polyline((*piece).clone(), parent.as_ref())
                    .expect("valid split polyline"),
                Geometry::NurbsCurve(piece) => session
                    .add_nurbscurve((*piece).clone(), parent.as_ref())
                    .expect("valid split curve"),
                _ => unreachable!("only curves create sibling objects"),
            };
            let id = node.borrow().name.clone();
            session.set_xform(&id, place.clone());

            if let Some(color) = color {
                self.colors.insert((doc, Rc::from(id)), color);
            }
        }

        session.commit();
        self.last_edited = Some(doc);
        Ok(regions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::scene::FileDoc;
    use session_rust::Xform;
    use session_rust::{Line, Point, Session};

    /// Add one placed document.
    fn add(scene: &mut Scene, session: Rc<Session>, name: &str, place: Xform) {
        scene.add_file(FileDoc {
            name: name.into(),
            session,
            place,
            point_px: 0.,
            display_only: false,
        });
    }

    /// A split keeps the group, placement and other placements; undo reverses it.
    #[test]
    fn split_preserves_tree_placement_and_other_shared_documents_and_undo() {
        let mut session = Session::new("shared");
        let group = session.add_group("parts");
        session.set_xform(&group.borrow().name, Xform::translation(10., 0., 0.));
        let target = session.add_line(
            Line::from_points(&Point::new(-2., 0., 0.), &Point::new(2., 0., 0.)),
            Some(&group),
        );
        let id = target.borrow().name.clone();
        let shared = Rc::new(session);
        let mut scene = Scene::new();
        add(
            &mut scene,
            Rc::clone(&shared),
            "first",
            Xform::translation(100., 0., 0.),
        );
        add(
            &mut scene,
            Rc::clone(&shared),
            "second",
            Xform::translation(200., 0., 0.),
        );
        let mut cutters = Session::new("cutters");
        cutters.add_line(
            Line::from_points(&Point::new(110., -2., 0.), &Point::new(110., 2., 0.)),
            None,
        );
        add(&mut scene, Rc::new(cutters), "cutters", Xform::identity());
        scene
            .colors
            .insert(scene.identity_of(0).unwrap(), [60, 170, 100]);
        assert_eq!(scene.split_rows(0, None, &[2]).unwrap(), 2);
        let first = &scene.docs[0].session;
        assert_eq!(first.objects.lines.len(), 2);
        assert_eq!(scene.docs[1].session.objects.lines.len(), 1);
        assert_eq!(shared.objects.lines.len(), 1);
        let parent = first.tree.get_node_by_name("parts").unwrap();
        assert_eq!(parent.borrow().children().len(), 2);
        assert_eq!(
            shared
                .tree
                .get_node_by_name("parts")
                .unwrap()
                .borrow()
                .children()
                .len(),
            1
        );
        assert!(first.lookup.contains_key(&id));
        let Geometry::Line(line) = &first.lookup[&id] else {
            panic!()
        };
        assert!(line.point_at(1.).distance(&Point::new(0., 0., 0.), None) < 1e-6);
        assert_eq!(scene.colors.len(), 2);
        assert!(scene.undo());
        assert_eq!(scene.docs[0].session.objects.lines.len(), 1);
        assert!(scene.redo());
        assert_eq!(scene.docs[0].session.objects.lines.len(), 2);
        let bytes = crate::app::session_io::save(&scene).unwrap();
        let restored = crate::app::session_io::open(&bytes).unwrap();
        assert_eq!(restored.docs[0].session.objects.lines.len(), 2);
    }

    /// A face split keeps the box solid; a bad cut changes nothing.
    #[test]
    fn split_face_keeps_solid_joined_and_invalid_cut_preserves_source() {
        let brep = BRep::create_box(10., 10., 10.);
        let original_area = brep.face_meshes_q(Some((20., 0.005)))[0].area();
        let s = &brep.m_surfaces[0];
        let a = s.get_cv(0, 0).unwrap();
        let u = s.get_cv(1, 0).unwrap();
        let v = s.get_cv(0, 1).unwrap();
        let p = |x: f64, y: f64| {
            Point::new(
                a[0] + x * (u[0] - a[0]) + y * (v[0] - a[0]),
                a[1] + x * (u[1] - a[1]) + y * (v[1] - a[1]),
                a[2] + x * (u[2] - a[2]) + y * (v[2] - a[2]),
            )
        };
        let mut session = Session::new("box");
        let node = session.add_brep(brep, None).unwrap();
        let id = node.borrow().name.clone();
        session.add_line(Line::from_points(&p(0.5, -1.), &p(0.5, 2.)), None);
        let mut scene = Scene::new();
        add(&mut scene, Rc::new(session), "box", Xform::identity());
        let row = (0..scene.object_count() as u32)
            .find(|&row| matches!(scene.geometry(row), Some(Geometry::BRep(_))))
            .unwrap();
        let cutter = (0..scene.object_count() as u32)
            .find(|&row| matches!(scene.geometry(row), Some(Geometry::Line(_))))
            .unwrap();
        assert!(scene.split_rows(row, None, &[cutter]).is_err());
        assert_eq!(scene.split_rows(row, Some(0), &[cutter]).unwrap(), 2);
        let Geometry::BRep(result) = &scene.docs[0].session.lookup[&id] else {
            panic!()
        };
        assert_eq!(result.face_count(), 7);
        assert!(result.is_solid());
        let meshes = result.face_meshes_q(Some((20., 0.005)));
        assert!(
            (meshes[0].area() - original_area / 2.).abs() < 1e-6,
            "first region area: {} of {original_area}",
            meshes[0].area()
        );
        assert!(
            (meshes[6].area() - original_area / 2.).abs() < 1e-6,
            "second region area: {} of {original_area}",
            meshes[6].area()
        );
        assert!(scene.undo());
        let Geometry::BRep(original) = &scene.docs[0].session.lookup[&id] else {
            panic!()
        };
        assert_eq!(original.face_count(), 6);
        assert!(original.is_solid());
    }
}
