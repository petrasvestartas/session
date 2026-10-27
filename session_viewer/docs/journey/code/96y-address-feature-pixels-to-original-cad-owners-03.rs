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

#[test]
fn tree_original_targets_preserve_complete_feature_presentation_membership() {
    use crate::layer_properties::{apply, Property};
    let mut e = Editor::default(); e.scene = Scene::empty();
    crate::scene_document::import(&mut e.scene, &crate::interaction_example::bytes(), "source", &Xform::identity(), 0.).unwrap();
    e.apply(Action::ElementAttributes(Some(true))).unwrap();
    let child = e.scene.objects().iter().find(|row|row.metadata.name == "child").unwrap().id;
    let source = e.scene.objects().iter().map(|row|row.geometry().unwrap().to_proto()).collect::<Vec<_>>();
    let rows = crate::hierarchy::rows(&e.scene);
    let parent = rows.iter().find(|row|row.label == "child").unwrap();
    assert_eq!(parent.ids, [child]);
    assert!(parent.members.len() > parent.ids.len());
    assert!(rows.iter().filter(|row|!row.readonly).flat_map(|row|&row.ids).all(|id|!e.scene.forest.derived(*id)));
    let pieces = rows.iter().filter(|row|row.label.starts_with("piece ")
        && crate::feature_owner::row_owner(&e.scene, &row.key) == Some(child)).collect::<Vec<_>>();
    assert_eq!(pieces.len(), 1);
    let piece = pieces[0];
    assert!(piece.readonly);
    assert_eq!(piece.ids, piece.members);
    assert_eq!(piece.ids.len(), 1);
    assert!(e.scene.forest.derived(piece.ids[0]));
    assert!(parent.members.contains(&piece.ids[0]));
    e.apply(Action::TreePick {ids:parent.ids.clone(), additive:false}).unwrap();
    assert_eq!(e.selected_objects(), [child]);
    assert!(e.highlighted_objects().contains(&piece.ids[0]));
    apply(&mut e.scene, &parent.key, Property::Locked(true)).unwrap();
    assert!(parent.members.iter().all(|id|e.scene.view.locked.contains(id)));
    assert_eq!(crate::feature_owner::display_owner(&e.scene, piece.ids[0]), None);
    apply(&mut e.scene, &parent.key, Property::Locked(false)).unwrap();
    assert!(parent.members.iter().all(|id|!e.scene.view.locked.contains(id)));
    assert_eq!(crate::feature_owner::display_owner(&e.scene, piece.ids[0]), Some(child));
    for edge in [false, true] {
        apply(&mut e.scene, &parent.key, Property::Colour {edge, rgb:Some([12, 34, 56])}).unwrap();
        let colours = if edge {&e.scene.view.edges} else {&e.scene.view.faces};
        assert!(parent.members.iter().all(|id|colours.get(id) == Some(&[12, 34, 56])));
    }
    apply(&mut e.scene, &parent.key, Property::Hidden(true)).unwrap();
    assert!(parent.members.iter().all(|id|!e.scene.visible(*id)));
    assert_eq!(crate::feature_owner::row_owner(&e.scene, &piece.key), None);
    apply(&mut e.scene, &parent.key, Property::Hidden(false)).unwrap();
    assert!(parent.members.iter().all(|id|e.scene.visible(*id)));
    assert_eq!(crate::feature_owner::row_owner(&e.scene, &piece.key), Some(child));
    assert_eq!(e.selected_objects(), [child]);
    assert_eq!(e.scene.objects().iter().map(|row|row.geometry().unwrap().to_proto()).collect::<Vec<_>>(), source);
}
