//! The side panel's model: what rows it shows and which objects each one controls.
//!
//! A panel row is a FILTER over the rows that already exist, never a second copy of the scene.
//! It owns no visibility of its own either: hiding a layer puts the same guids in the same
//! `Scene.hidden` set that `H` uses, so a rebuild re-applies it and the two cannot disagree.
//!
//! Pure, so what the panel would show is testable without a browser.

use crate::app::scene::Scene;

/// What a row stands for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layer {
    /// One loaded file, by its index in `Scene.docs`.
    Document(usize),
    /// Every object of one kernel type, whichever file it came from.
    Kind(Kind),
}

/// The geometry kinds the panel groups by: the viewer's lanes, named as a person would.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Solids,
    Surfaces,
    Meshes,
    Curves,
    Points,
    Clouds,
}

impl Kind {
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

    /// Which bucket one object falls in.
    ///
    /// The match is exhaustive on purpose, with no catch-all: a kernel type added later stops
    /// the build here, where someone has to decide which bucket it belongs in, rather than
    /// falling into an "other" row nobody would notice was wrong.
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
    /// The panel is one element with one listener, and a row says which layer it is through
    /// this key: a closure per row would be a closure per scene change.
    pub fn key(self) -> String {
        match self {
            Layer::Document(index) => format!("doc:{index}"),
            Layer::Kind(kind) => format!("kind:{}", kind.label()),
        }
    }

    /// The layer a clicked row names, or `None` for anything that is not a row.
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
    pub layer: Layer,
    pub label: String,
    /// How many object rows it controls. A row that controls none is not shown at all.
    pub count: usize,
    /// Every one of them is hidden.
    pub hidden: bool,
}

/// The rows a panel would show: the documents in load order, then the kinds present.
pub fn rows(scene: &Scene) -> Vec<Row> {
    let mut out = Vec::new();
    for (index, doc) in scene.docs.iter().enumerate() {
        let rows = of_layer(scene, Layer::Document(index));
        if rows.is_empty() {
            continue;
        }
        out.push(Row {
            layer: Layer::Document(index),
            label: doc.name.clone(),
            count: rows.len(),
            hidden: all_hidden(scene, &rows),
        });
    }
    let mut kinds: Vec<Kind> = Vec::new();
    for row in 0..scene.object_count() as u32 {
        if let Some(geometry) = scene.geometry(row) {
            let kind = Kind::of(geometry);
            if !kinds.contains(&kind) {
                kinds.push(kind);
            }
        }
    }
    kinds.sort();
    for kind in kinds {
        let rows = of_layer(scene, Layer::Kind(kind));
        if rows.is_empty() {
            continue;
        }
        out.push(Row {
            layer: Layer::Kind(kind),
            label: kind.label().to_string(),
            count: rows.len(),
            hidden: all_hidden(scene, &rows),
        });
    }
    out
}

/// The object rows one layer controls.
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

fn all_hidden(scene: &Scene, rows: &[u32]) -> bool {
    !rows.is_empty()
        && rows.iter().all(|&row| {
            scene
                .identity_of(row)
                .is_some_and(|identity| scene.hidden.contains(&identity))
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::scene::FileDoc;
    use session_rust::{Point, Polyline, Session, Xform};
    use std::rc::Rc;

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

    /// Documents first, in load order, then the kinds that are actually present - never a row
    /// for a kind nothing in the scene has.
    #[test]
    fn the_panel_lists_documents_then_the_kinds_present() {
        let scene = scene_with_two_files();
        let rows = rows(&scene);
        let labels: Vec<&str> = rows.iter().map(|r| r.label.as_str()).collect();
        assert_eq!(labels, vec!["left", "right", "curves", "points"]);
        assert_eq!(rows[0].count, 2, "left holds a point and a polyline");
        assert_eq!(rows[3].count, 2, "one point in each file");
    }

    /// A layer is a filter over rows that already exist, so it can name rows from more than
    /// one document, and a document row names only its own.
    #[test]
    fn a_layer_names_the_rows_it_controls() {
        let scene = scene_with_two_files();
        assert_eq!(of_layer(&scene, Layer::Document(1)), vec![2]);
        assert_eq!(of_layer(&scene, Layer::Kind(Kind::Points)), vec![0, 2]);
        assert!(of_layer(&scene, Layer::Kind(Kind::Clouds)).is_empty());
    }

    /// A row carries its layer as text, and the text has to come back as the same layer -
    /// otherwise a click lands on a different row than the one under the pointer.
    #[test]
    fn a_row_key_survives_the_round_trip() {
        for layer in [Layer::Document(0), Layer::Document(17), Layer::Kind(Kind::Clouds)] {
            assert_eq!(Layer::from_key(&layer.key()), Some(layer));
        }
        assert_eq!(Layer::from_key("kind:sandwiches"), None);
        assert_eq!(Layer::from_key("nonsense"), None);
    }

    /// The panel reads the same hide set `H` writes, so a layer reads as hidden exactly when
    /// every row it controls is.
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
