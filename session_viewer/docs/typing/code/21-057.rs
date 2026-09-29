
#[cfg(test)]
mod editing_tests {
    use super::tests::file;
    use super::*;
    use session_rust::Point;

    /// A created text is an edit: the GPU anchor stays where it was.
    #[test]
    fn a_created_text_keeps_the_anchor() {
        let mut scene = Scene::new();
        scene.add_text(crate::app::edit::tests::text(1.0));
        assert!(!scene.loaded);
    }

    /// Element features draw in the element's row and move with it.
    #[test]
    fn attributes_share_the_element_row_and_its_placement() {
        use session_rust::element::ElementFeature;
        use session_rust::{Element, Mesh, Polyline};

        let mut element = Element::new("beam");
        element.set_geometry(Mesh::create_box(10.0, 10.0, 10.0));
        let axis = Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(100.0, 0.0, 0.0)]);
        element.add_feature(ElementFeature::new("axis", -1, vec![axis], "axis"));
        let mut source = Session::new("attributes");
        source.add_element(element, None);
        let mut scene = Scene::new();
        scene.attributes = false;
        scene.add_file(file("beam", Rc::new(source), false));
        let plain = scene.tables.seg.ribbons.len();
        assert_eq!(scene.object_count(), 1);

        scene.attributes = true;
        scene.rewalk_cpu();
        assert_eq!(scene.object_count(), 1);
        let range = scene.ribbon_range(0).unwrap();
        assert_eq!(range.len(), plain + 1);

        let moved = scene.transform_rows(&[0], &Xform::translation(5.0, 0.0, 0.0), "Move");
        assert_eq!(moved.map(|m| m.len()), Some(1));
        assert_eq!(
            scene
                .placement_of(0)
                .unwrap()
                .transform_point(&Point::new(100.0, 0.0, 0.0))[0],
            105.0
        );
    }
}

/// A released document fetched and decoded again.
pub struct Hydrated {
    pub doc: usize,                       // the document
    pub token: u64,                       // the release it answers
    pub session: Result<Session, String>, // its objects, or why not
    pub ms: f64,                          // fetch and decode time
}
