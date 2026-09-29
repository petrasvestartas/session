
impl State {
    /// L: open or close the layers panel.
    pub fn toggle_layers_panel(&mut self) {
        let open = !crate::app::feedback::layers_open();
        crate::app::feedback::layers_visible(open);

        if open {
            self.refresh_layers(); // register:panel
        }

        self.touch();
    }
}

impl State {
    /// Hide a layer, or show it when it is fully hidden.
    pub fn toggle_layer(&mut self, layer: Layer) {
        let rows = layers::of_layer(&self.scene, layer);

        if rows.is_empty() {
            return;
        }

        // anything still visible: hide the whole layer
        let hide = rows.iter().any(|&row| {
            self.scene
                .identity_of(row)
                .is_some_and(|id| !self.scene.hidden.contains(&id))
        });
        self.set_rows_hidden(&rows, hide);
    }

    /// Refill the layers panel, when it is open.
    pub fn refresh_layers(&mut self) {
        if !crate::app::feedback::layers_open() {
            return;
        }

        self.features.hierarchy.refresh(&self.scene);
        let mut rows = Vec::new();
        self.hierarchy_labels(&mut rows);
        crate::app::feedback::layers_panel(&rows);
        let selected = self.selected_rows(); // sorted
        let guid = |row| {
            self.scene
                .identity_of(row)
                .map(|id| id.1)
                .unwrap_or_default()
        };
        // rows only while the table is unfolded
        let open = crate::app::feedback::graph_open();
        let edges = self
            .features
            .hierarchy
            .edges
            .iter()
            .take(if open { MAX_EDGE_ROWS } else { 0 })
            .map(|&[from, to]| crate::app::feedback::EdgeRow {
                key: format!("pair/{from}/{to}"),
                from: edge_label(&self.scene, from),
                to: edge_label(&self.scene, to),
                guids: format!("From {}\nTo {}", guid(from), guid(to)), // the panel font has no arrow
                selected: selected.binary_search(&from).is_ok()
                    && selected.binary_search(&to).is_ok(),
            })
            .collect();
        crate::app::feedback::graph_panel(edges, self.features.hierarchy.edges.len());
    }
}

/// An edge end in the graph table: the object's own name, else the start of its guid.
fn edge_label(scene: &crate::app::scene::Scene, row: u32) -> String {
    match scene.geometry(row).map(session_rust::Geometry::name) {
        Some(name) if !name.trim().is_empty() => name.to_string(),
        _ => scene
            .identity_of(row)
            .map(|id| id.1.chars().take(8).collect())
            .unwrap_or_default(),
    }
}

#[cfg(test)]
mod panel_tests {
    use super::*;

    /// The graph table shows an object's name as the tree does, and a short guid when it has none.
    #[test]
    fn an_edge_end_is_named_like_its_tree_row() {
        use crate::app::scene::{FileDoc, Scene};
        use session_rust::Session;
        use std::rc::Rc;

        let mut session = Session::new("site");
        session.add_point(Point::new(0.0, 0.0, 0.0), None);
        let mut blank = Point::new(1.0, 0.0, 0.0);
        blank.name = String::new();
        session.add_point(blank, None);
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "site".into(),
            session: Rc::new(session),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        assert_eq!(edge_label(&scene, 0), "my_point");
        let guid = scene.identity_of(1).unwrap().1;
        assert_eq!(edge_label(&scene, 1), guid[..8]);
    }
}
