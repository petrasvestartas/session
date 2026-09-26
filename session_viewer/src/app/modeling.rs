use crate::app::scene::FileDoc;
use crate::app::scene::Scene;
use crate::app::scene::rows::DocState;
use crate::app::scene::sync;
use session_rust::Arrowhead;
use session_rust::Geometry;
use session_rust::Line;
use session_rust::Point;
use session_rust::Polyline;
use session_rust::Session;
use session_rust::Xform;
use std::rc::Rc;

/// Most points one command may create.
pub const MAX_POINTS: usize = 4096;

/// One geometry command.
#[derive(Clone, Debug, PartialEq)]
pub enum Modeling {
    Point([f64; 3]),          // create a point
    Line([f64; 3], [f64; 3]), // create a line
    Arrow([f64; 3], [f64; 3]), // create a line with a head at its end
    Polyline(Vec<[f64; 3]>),  // create a polyline
    Curve(Vec<[f64; 3]>),     // create a curve through control points
    Trim(f64, f64),           // keep this part of the selected curve, 0..1
    Extend(f64, f64),         // extend the selected curve to this range
}

impl Scene {
    /// Run one geometry command as one undo step; a new object's (document, guid) comes back.
    pub fn model(&mut self, command: &Modeling) -> Result<Option<(usize, String)>, String> {
        match command {
            Modeling::Point(p) => self
                .create_geometry(Geometry::Point(Rc::new(point(*p)?)))
                .map(Some),
            Modeling::Line(a, b) | Modeling::Arrow(a, b) => {
                let mut line = Line::from_points(&point(*a)?, &point(*b)?);

                if line.length() <= 1e-12 {
                    return Err("line endpoints must differ".into());
                }

                if matches!(command, Modeling::Arrow(..)) {
                    line.arrowhead = Arrowhead::END;
                }

                self.create_geometry(Geometry::Line(Rc::new(line)))
                    .map(Some)
            }
            Modeling::Polyline(points) => {
                if !(2..=MAX_POINTS).contains(&points.len()) {
                    return Err(format!("polyline needs 2–{MAX_POINTS} points"));
                }

                let points = points
                    .iter()
                    .map(|p| point(*p))
                    .collect::<Result<Vec<_>, _>>()?;
                self.create_geometry(Geometry::Polyline(Rc::new(Polyline::new(points))))
                    .map(Some)
            }
            Modeling::Curve(points) => {
                if !(2..=MAX_POINTS).contains(&points.len()) {
                    return Err(format!("curve needs 2–{MAX_POINTS} control points"));
                }

                let points = points
                    .iter()
                    .map(|p| point(*p))
                    .collect::<Result<Vec<_>, _>>()?;
                self.create_geometry(Geometry::NurbsCurve(Rc::new(curve(&points))))
                    .map(Some)
            }
            _ => self.edit_geometry(command).map(|_| None),
        }
    }

    /// Add a geometry to the current layer, else the `Created` document, making it if needed.
    pub(crate) fn create_geometry(
        &mut self,
        geometry: Geometry,
    ) -> Result<(usize, String), String> {
        let mut made = self.create_many(vec![geometry], "create")?;
        Ok(made.remove(0))
    }

    /// Add geometries to the current layer as one undo step; each (document, guid) comes back.
    pub(crate) fn create_many(
        &mut self,
        geometries: Vec<Geometry>,
        label: &str,
    ) -> Result<Vec<(usize, String)>, String> {
        if geometries.is_empty() {
            return Err("nothing to create".into());
        }

        let layer = self
            .current_layer()
            .filter(|(doc, _)| !self.docs[*doc].display_only);
        let doc = match (&layer, self.created_doc) {
            (Some((doc, _)), _) => *doc,
            (None, Some(index)) => index,
            (None, None) => {
                let session = Rc::new(Session::new("Created"));
                let nodes_from = sync::tree_key(&session);
                self.push_doc(
                    FileDoc {
                        name: "Created".into(),
                        session,
                        place: Xform::identity(),
                        point_px: 0.0,
                        display_only: false,
                    },
                    DocState {
                        sheet: None,
                        nodes_from,
                    },
                );
                self.created_doc = Some(self.docs.len() - 1);
                self.docs.len() - 1
            }
        };
        let place = self.docs[doc].place.clone();
        let session = Rc::make_mut(&mut self.docs[doc].session);
        // the current layer, else the root
        let parent = layer
            .as_ref()
            .and_then(|(_, name)| session.tree.get_node_by_name(name))
            .or_else(|| session.tree.root())
            .ok_or("this document has no tree")?;
        let name = parent.borrow().name.clone();
        let back = super::layers::frame(session, &place, &name);
        session.begin(label);
        let mut nodes = Vec::with_capacity(geometries.len());

        for geometry in &geometries {
            let Some(node) = super::layers::add(session, geometry, &parent, true) else {
                continue;
            };

            // world coordinates stay put under a placed document or layer
            if let Some(back) = &back {
                let guid = node.borrow().name.clone();
                super::layers::place(session, &guid, back, &Xform::identity());
            }

            nodes.push(node);
        }

        let notes = sync::commit(session);
        self.noted(doc, notes);

        if nodes.is_empty() {
            return Err("the geometry is empty".into());
        }

        let mut made = Vec::with_capacity(nodes.len());

        for node in &nodes {
            self.hint(doc, node);
            made.push((doc, node.borrow().name.clone()));
        }

        self.edited(&[doc]);
        Ok(made)
    }

    /// Trim or extend the selected object.
    fn edit_geometry(&mut self, command: &Modeling) -> Result<(), String> {
        let row = self.selected.ok_or("select one object first")?;
        let (doc, guid) = self.identity_of(row).ok_or("object no longer exists")?;
        self.editable(doc)?; // a released document comes back first
        let file = self.docs.get(doc).ok_or("this object has no document")?;

        if file.display_only {
            return Err("this document is display only".into());
        }

        let source = self.geometry(row).ok_or("source geometry is unavailable")?;
        let next = edited(source, command)?;
        let session = Rc::make_mut(&mut self.docs[doc].session);
        session.begin("edit geometry");
        let replaced = session.replace(&guid, next);
        let notes = sync::commit(session);
        self.noted(doc, notes);

        if !replaced {
            return Err("object no longer exists".into());
        }

        self.edited(&[doc]);
        Ok(())
    }
}

/// A point from finite, reasonable coordinates.
fn point(p: [f64; 3]) -> Result<Point, String> {
    if p.iter().any(|v| !v.is_finite() || v.abs() > 1e12) {
        return Err("coordinates must be finite and within ±1e12".into());
    }

    Ok(Point::new(p[0], p[1], p[2]))
}

/// The source trimmed or extended to `a..b` of its length.
fn edited(source: &Geometry, command: &Modeling) -> Result<Geometry, String> {
    let (a, b, trim) = match *command {
        Modeling::Trim(a, b) => (a, b, true),
        Modeling::Extend(a, b) => (a, b, false),
        _ => return Err("expected trim or extend".into()),
    };

    if !a.is_finite() || !b.is_finite() || a >= b || a.abs() > 1e6 || b.abs() > 1e6 {
        return Err("parameters must be finite, increasing, and within ±1e6".into());
    }

    if (trim && (a < 0.0 || b > 1.0)) || (!trim && (a > 0.0 || b < 1.0)) {
        return Err("trim keeps 0 <= a < b <= 1; extend needs a <= 0 and b >= 1".into());
    }

    match source {
        Geometry::Line(line) => {
            let mut next = Line::from_points(&line.point_at(a), &line.point_at(b));
            next.name = line.name.clone();
            next.linecolor = line.linecolor.clone();
            next.width = line.width;
            next.dash = line.dash.clone();
            next.arrowhead = line.arrowhead;
            Ok(Geometry::Line(Rc::new(next)))
        }
        Geometry::NurbsCurve(curve) => {
            if curve.m_cv_count > MAX_POINTS {
                return Err(format!("curve edits are limited to {MAX_POINTS} controls"));
            }

            let mut next = (**curve).clone();
            let (lo, hi) = next.domain();
            let a = lo + a * (hi - lo); // 0..1 into the curve domain
            let b = lo + b * (hi - lo);
            let ok = if trim {
                next.trim(a, b)
            } else {
                next.extend(a, b)
            };

            if !ok {
                return Err("kernel refused this curve interval".into());
            }

            Ok(Geometry::NurbsCurve(Rc::new(next)))
        }
        _ => Err("trim and extend currently accept lines and NURBS curves".into()),
    }
}

/// A curve through control points; ending on the start closes it smoothly.
fn curve(points: &[Point]) -> session_rust::NurbsCurve {
    let count = points.len();
    let closed = count >= 4 && points[0].distance(&points[count - 1], None) <= 1e-12;

    if closed {
        return session_rust::NurbsCurve::create(true, (count - 2).min(3), &points[..count - 1]);
    }

    session_rust::NurbsCurve::create(false, (count - 1).min(3), points)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A created point undoes and redoes; NaN is refused.
    #[test]
    fn creation_undo_redo_and_invalid_input() {
        let mut scene = Scene::new();
        assert!(scene.model(&Modeling::Point([f64::NAN, 0.0, 0.0])).is_err());
        assert!(scene.docs.is_empty());
        scene.model(&Modeling::Point([1.0, 2.0, 3.0])).unwrap();
        assert_eq!(scene.docs[0].session.lookup.len(), 1);
        assert!(scene.undo());
        assert!(scene.docs[0].session.lookup.is_empty());
        assert!(scene.redo());
        assert_eq!(scene.docs[0].session.lookup.len(), 1);
    }

    /// Several objects go in as one undo step; an empty list adds nothing.
    #[test]
    fn create_many_adds_a_surface_and_a_brep_in_one_undo_step() {
        let mut scene = Scene::new();
        let surface = session_rust::Primitives::create_extrusion(
            &session_rust::NurbsCurve::create(false, 1, &[Point::new(0.0, 0.0, 0.0), Point::new(5.0, 0.0, 0.0)]),
            &session_rust::Vector::new(0.0, 0.0, 5.0),
        );
        let geometries = vec![
            Geometry::NurbsSurface(Rc::new(surface)),
            Geometry::BRep(Rc::new(session_rust::BRep::create_box(1.0, 2.0, 3.0))),
        ];
        assert_eq!(scene.create_many(geometries, "pair").unwrap().len(), 2);
        assert_eq!(scene.docs[0].session.lookup.len(), 2);
        assert!(scene.undo());
        assert!(scene.docs[0].session.lookup.is_empty());
        assert!(scene.redo());
        assert_eq!(scene.docs[0].session.lookup.len(), 2);
        assert!(scene.create_many(Vec::new(), "none").is_err());
    }

    /// Trim and extend keep the pen and check their ranges.
    #[test]
    fn trim_and_extend_preserve_style_and_intervals() {
        let mut line = Line::new(0.0, 0.0, 0.0, 10.0, 0.0, 0.0);
        line.width = 3.0;
        let source = Geometry::Line(Rc::new(line));
        let Geometry::Line(trim) = edited(&source, &Modeling::Trim(0.2, 0.8)).unwrap() else {
            panic!()
        };
        assert_eq!(trim.start()[0], 2.0);
        assert_eq!(trim.end()[0], 8.0);
        assert_eq!(trim.width, 3.0);
        let Geometry::Line(extend) = edited(&source, &Modeling::Extend(-0.5, 1.5)).unwrap() else {
            panic!()
        };
        assert_eq!(extend.start()[0], -5.0);
        assert_eq!(extend.end()[0], 15.0);
        assert!(edited(&source, &Modeling::Trim(-0.1, 0.8)).is_err());
        assert!(edited(&source, &Modeling::Extend(0.1, 1.5)).is_err());
    }

    /// Curve trim takes 0..1 of the domain.
    #[test]
    fn curve_trim_uses_normalized_domain() {
        let curve = session_rust::NurbsCurve::create(
            false,
            1,
            &[Point::new(0.0, 0.0, 0.0), Point::new(10.0, 0.0, 0.0)],
        );
        let source = Geometry::NurbsCurve(Rc::new(curve));
        let Geometry::NurbsCurve(curve) = edited(&source, &Modeling::Trim(0.2, 0.8)).unwrap()
        else {
            panic!()
        };
        assert!((curve.point_at_start()[0] - 2.0).abs() < 1e-9);
        assert!((curve.point_at_end()[0] - 8.0).abs() < 1e-9);
    }

    /// A curve that ends on its start closes without a kink.
    #[test]
    fn a_curve_ending_on_its_start_is_closed() {
        let square = [
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(10.0, 10.0, 0.0),
            Point::new(0.0, 10.0, 0.0),
            Point::new(0.0, 0.0, 0.0),
        ];
        let closed = curve(&square);
        assert!(closed.is_valid() && closed.is_closed());
        assert!(
            closed
                .point_at_start()
                .distance(&closed.point_at_end(), None)
                < 1e-9
        );
        let triangle = curve(&[
            square[0].clone(),
            square[1].clone(),
            square[2].clone(),
            square[0].clone(),
        ]);
        assert!(triangle.is_valid() && triangle.is_closed());
        let open = curve(&square[..4]);
        assert!(open.is_valid() && !open.is_closed());
    }
}
