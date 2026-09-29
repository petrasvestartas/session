
#[cfg(test)]
mod commands_tests {
    use super::tests::{children, site};
    use super::*;
    use session_rust::Xform;
    use std::rc::Rc;

    /// New objects go to the current layer, keeping the typed world coordinates.
    #[test]
    fn drawing_lands_on_the_current_layer() {
        let mut scene = site();
        Rc::make_mut(&mut scene.docs[0].session)
            .xforms
            .insert("roof".into(), Xform::translation(0.0, 0.0, 10.0));
        scene.set_current_layer(0, "roof").unwrap();
        let (doc, guid) = scene
            .model(&crate::app::command::verbs::point::SPEC, &[[1.0, 2.0, 3.0]])
            .unwrap();
        assert_eq!(doc, 0);
        assert_eq!(children(&scene, 0, "roof"), vec![guid.clone()]);
        let world = scene.docs[0].session.world_xform(&guid);
        assert_eq!([world.m[12], world.m[13], world.m[14]], [0.0, 0.0, 0.0]);
        assert!(scene.created_doc.is_none());
        assert!(
            scene.set_current_layer(0, &guid).is_err(),
            "an object is not a layer"
        );
        let session = Rc::make_mut(&mut scene.docs[0].session);
        let point = session.tree.get_node_by_name(&guid).unwrap();
        session.add(&TreeNode::new("features"), Some(&point));
        assert!(
            scene.set_current_layer(0, "features").is_err(),
            "nor a group inside an object"
        );
    }
}
