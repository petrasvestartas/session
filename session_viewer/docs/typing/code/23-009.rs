
#[cfg(test)]
mod editing_tests_23 {
    use super::*;

    /// A clipping plane is one undo step and survives a save with its name and size.
    #[test]
    fn a_clipping_plane_undoes_and_saves() {
        use crate::app::scene::Scene;
        use session_rust::{Geometry, Session};

        let mut scene = Scene::new();
        let plane = plane_from(Mode::Xy, &[[0.0, 0.0, 50.0]], 300.0).unwrap();
        let (doc, guid) = scene
            .create_geometry(Geometry::Plane(std::rc::Rc::new(plane)))
            .unwrap();
        assert!(scene.undo());
        assert!(!scene.docs[doc].session.lookup.contains_key(guid.as_str()));
        assert!(scene.redo());
        let mut session = (*scene.docs[doc].session).clone();
        let saved = Session::pb_loads(&session.pb_dumps()).unwrap();
        let Some(Geometry::Plane(loaded)) = saved.lookup.get(guid.as_str()) else {
            panic!("the plane is saved");
        };
        assert!(is_clipping(loaded));
        assert!((length(array(&loaded.x_axis())) - 300.0).abs() < 1e-9);
        assert_eq!(array(&loaded.origin()), [0.0, 0.0, 50.0]);
    }
}
