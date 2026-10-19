use crate::{editor::{Action, Editor}, pick_sources::{Mode, Part, Sources, Target}, scene::Scene};
use session_rust::Xform;
use std::rc::Rc;

#[test]
fn derived_path_pick_addresses_target_only_original_owners() {
    let mut e = Editor::default(); e.scene = Scene::empty();
    crate::scene_document::import(&mut e.scene, &crate::interaction_example::bytes(), "source", &Xform::identity(), 0.).unwrap();
    e.apply(Action::ElementAttributes(Some(true))).unwrap();
    let table = Sources::new(&e.scene).unwrap();
    let mut offset = 0;
    for path in e.scene.paths().iter().filter(|row|e.scene.visible(row.id)) {
        let owner = crate::feature_owner::display_owner(&e.scene, path.id).unwrap();
        assert_ne!(owner, path.id);
        for pair in path.points().windows(2).filter(|pair|pair[0].to_f32()!=pair[1].to_f32()) {
            let _ = pair;
            assert_eq!(table.path_ids[offset], [owner.value(), 0]);
            assert_eq!(table.resolve(&e.scene, table.path_ids[offset], Mode::Object), Some(Target {owner,part:Part::Object}));
            assert_eq!(table.resolve(&e.scene, table.path_ids[offset], Mode::Edge), None);
            offset += 1;
        }
    }
    assert_eq!(offset, 6);
    let child = e.scene.objects().iter().find(|row|row.metadata.name == "child").unwrap().id;
    let child_path = e.scene.paths().iter().find(|row|crate::feature_owner::display_owner(&e.scene,row.id)==Some(child)).unwrap().id;
    Rc::make_mut(&mut e.scene.view).lock(&[child], true);
    assert_eq!(crate::feature_owner::display_owner(&e.scene, child_path), None);
    assert!(Sources::new(&e.scene).unwrap().path_ids.iter().any(|ids|*ids==[0,0]));
    Rc::make_mut(&mut e.scene.view).lock(&[child], false);
    e.select_objects([child], false);
    e.apply(Action::Hide).unwrap();
    assert!(!e.scene.visible(child_path));
}
