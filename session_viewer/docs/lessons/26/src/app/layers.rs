use crate::app::scene::Scene;

/// What one panel row controls.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layer {
    Document(usize), // one loaded file, by index
    Kind(Kind),      // every object of one kind
}

/// The kinds the panel groups objects by.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Solids,   // BReps, boxes, elements
    Surfaces, // NURBS surfaces, planes
    Meshes,
    Curves,   // lines, polylines, NURBS curves
    Points,
    Clouds,   // point clouds
}

impl Kind {
    /// The row label.
    pub fn label(self) -> &'static str {
        match self {
            Kind::Solids => "solids",
            Kind::Surfaces => "surfaces",
            Kind::Meshes => "meshes",
            Kind::Curves => "curves",
            Kind::Points => "points",
            Kind::Clouds => "clouds",
        }
    }

    /// The kind of one geometry.
    fn of(geometry: &session_rust::Geometry) -> Self {
        use session_rust::Geometry as G;

        match geometry {
            G::BRep(_) | G::OBB(_) | G::Element(_) => Kind::Solids,
            G::NurbsSurface(_) | G::Plane(_) => Kind::Surfaces,
            G::Mesh(_) => Kind::Meshes,
            G::Line(_) | G::Polyline(_) | G::NurbsCurve(_) => Kind::Curves,
            G::Point(_) => Kind::Points,
            G::PointCloud(_) => Kind::Clouds,
        }
    }
}

impl Layer {
    /// The row's text key, e.g. `doc:0` or `kind:curves`.
    pub fn key(self) -> String {
        match self {
            Layer::Document(index) => format!("doc:{index}"),
            Layer::Kind(kind) => format!("kind:{}", kind.label()),
        }
    }

    /// The layer from a row key.
    pub fn from_key(key: &str) -> Option<Self> {
        if let Some(index) = key.strip_prefix("doc:") {
            return index.parse().ok().map(Layer::Document);
        }

        let label = key.strip_prefix("kind:")?;

        for kind in [
            Kind::Solids,
            Kind::Surfaces,
            Kind::Meshes,
            Kind::Curves,
            Kind::Points,
            Kind::Clouds,
        ] {
            if kind.label() == label {
                return Some(Layer::Kind(kind));
            }
        }

        None
    }
}

/// One line of the panel.
pub struct Row {
    pub layer: Layer,  // what it controls
    pub label: String, // text shown
    pub count: usize,  // objects it controls
    pub hidden: bool,  // all of them hidden
}

// --8<-- [start:step-5a]
/// The panel rows: documents, then the kinds present.
pub fn rows(scene: &Scene) -> Vec<Row> {
    let mut documents = vec![(0, 0); scene.docs.len()]; // (count, hidden) per document
    let mut kinds = [(0, 0); 6]; // (count, hidden) per kind

    for row in 0..scene.object_count() as u32 {
        let Some(identity) = scene.identity_of(row) else {
            continue;
        };
        let hidden = usize::from(scene.hidden.contains(&identity));

        if let Some(count) = documents.get_mut(identity.0) {
            count.0 += 1;
            count.1 += hidden;
        }

        if let Some(geometry) = scene.geometry(row) {
            let count = &mut kinds[Kind::of(geometry) as usize];
            count.0 += 1;
            count.1 += hidden;
        }
    }

    let mut out = Vec::new();

    for (index, &(count, hidden)) in documents.iter().enumerate() {
        if count > 0 {
            out.push(Row {
                layer: Layer::Document(index),
                label: scene.docs[index].name.clone(),
                count,
                hidden: hidden == count,
            });
        }
    }

    for kind in [
        Kind::Solids,
        Kind::Surfaces,
        Kind::Meshes,
        Kind::Curves,
        Kind::Points,
        Kind::Clouds,
    ] {
        let (count, hidden) = kinds[kind as usize];

        if count > 0 {
            out.push(Row {
                layer: Layer::Kind(kind),
                label: kind.label().into(),
                count,
                hidden: hidden == count,
            });
        }
        // --8<-- [end:step-5a]
    }

    out
}

/// The object rows of one layer.
pub fn of_layer(scene: &Scene, layer: Layer) -> Vec<u32> {
    let mut rows = Vec::new();

    for row in 0..scene.object_count() as u32 {
        let matches = match layer {
            Layer::Document(index) => scene.identity_of(row).map(|(doc, _)| doc) == Some(index),
            Layer::Kind(kind) => scene.geometry(row).map(Kind::of) == Some(kind),
        };

        if matches {
            rows.push(row);
        }
    }

    rows
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::scene::FileDoc;
    use session_rust::{Point, Polyline, Session, Xform};
    use std::rc::Rc;

    /// Two files: a point and a polyline, then a point.
    fn scene_with_two_files() -> Scene {
        let mut left = Session::new("left");
        left.add_point(Point::new(0.0, 0.0, 0.0), None);
        left.add_polyline(
            Polyline::new(vec![
                Point::new(0.0, 0.0, 0.0),
                Point::new(1.0, 0.0, 0.0),
                Point::new(1.0, 1.0, 0.0),
            ]),
            None,
        );
        let mut right = Session::new("right");
        right.add_point(Point::new(5.0, 0.0, 0.0), None);
        let mut scene = Scene::new();

        for (name, session) in [("left", left), ("right", right)] {
            scene.add_file(FileDoc {
                name: name.into(),
                session: Rc::new(session),
                place: Xform::identity(),
                point_px: 0.0,
                display_only: false,
            });
        }

        scene
    }

    /// Documents first, then only the kinds present.
    #[test]
    fn the_panel_lists_documents_then_the_kinds_present() {
        let scene = scene_with_two_files();
        let rows = rows(&scene);
        let labels: Vec<&str> = rows.iter().map(|r| r.label.as_str()).collect();
        assert_eq!(labels, vec!["left", "right", "curves", "points"]);
        assert_eq!(rows[0].count, 2, "left holds a point and a polyline");
        assert_eq!(rows[3].count, 2, "one point in each file");
    }

    // --8<-- [start:step-5c]
    /// Row counts match the rows a layer controls.
    #[test]
    fn bucket_counts_match_membership_with_mixed_visibility() {
        let mut scene = scene_with_two_files();
        scene.hidden.insert(scene.identity_of(0).unwrap());
        scene.hidden.insert(scene.identity_of(2).unwrap());

        for row in rows(&scene) {
            let members = of_layer(&scene, row.layer);
            assert_eq!(row.count, members.len());
            let hidden = members
                .iter()
                .all(|row| scene.hidden.contains(&scene.identity_of(*row).unwrap()));
            assert_eq!(row.hidden, hidden);
        }
    }

    /// A kind spans documents; a document is only its own.
// --8<-- [end:step-5c]
    #[test]
    fn a_layer_names_the_rows_it_controls() {
        let scene = scene_with_two_files();
        assert_eq!(of_layer(&scene, Layer::Document(1)), vec![2]);
        assert_eq!(of_layer(&scene, Layer::Kind(Kind::Points)), vec![0, 2]);
        assert!(of_layer(&scene, Layer::Kind(Kind::Clouds)).is_empty());
    }

    /// A key parses back to its layer.
    #[test]
    fn a_row_key_survives_the_round_trip() {
        for layer in [
            Layer::Document(0),
            Layer::Document(17),
            Layer::Kind(Kind::Clouds),
        ] {
            assert_eq!(Layer::from_key(&layer.key()), Some(layer));
        }

        assert_eq!(Layer::from_key("kind:sandwiches"), None);
        assert_eq!(Layer::from_key("nonsense"), None);
    }

    /// A layer is hidden only when all its objects are.
    #[test]
    fn a_layer_is_hidden_when_all_of_it_is() {
        let mut scene = scene_with_two_files();
        assert!(!rows(&scene)[1].hidden);
        let identity = scene.identity_of(2).expect("row 2 exists");
        scene.hidden.insert(identity);
        let rows = rows(&scene);
        assert!(rows[1].hidden, "the whole of `right` is hidden");
        assert!(!rows[3].hidden, "only one of the two points is");
    }
}
