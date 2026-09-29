
#[cfg(test)]
mod document_tests {
    use super::tests::file;
    use super::*;
    use session_rust::Point;

    /// Baked attribute copies never get a row.
    #[test]
    fn baked_attributes_never_get_a_row() {
        use session_rust::{Element, Mesh, Polyline};

        let mut element = Element::new("beam");
        element.set_geometry(Mesh::create_box(10.0, 10.0, 10.0));
        let mut source = Session::new("attributes");
        source.add_element(element, None);
        let group = source.add_group("attributes");
        let axis = Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(100.0, 0.0, 0.0)]);
        source.add_polyline(axis, Some(&group));
        assert_eq!(source.lookup.len(), 2);
        let mut scene = Scene::new();
        scene.add_file(file("beam", Rc::new(source), false));
        assert_eq!(scene.object_count(), 1);

        scene.attributes = true;
        scene.rewalk_cpu();
        assert_eq!(scene.object_count(), 1);
    }
}
