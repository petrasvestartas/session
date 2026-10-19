use crate::{editor::{Action, Editor}, scene::{ObjectId, Scene}};
use session_rust::{proto, Element, Geometry, Xform};
use prost::Message;
use std::rc::Rc;

fn owner(scene: &Scene, name: &str) -> ObjectId {
    scene.objects().iter().find(|row| row.metadata.name == name).unwrap().id
}
fn original(scene: &Scene, id: ObjectId) -> Rc<Element> {
    match scene.objects().iter().find(|row| row.id == id).unwrap().original.as_ref().unwrap() {
        Geometry::Element(source) => source.clone(), _ => panic!("Element expected"),
    }
}
fn same_element(mut a: proto::Element, mut b: proto::Element) {
    assert_eq!(proto::Mesh::decode(a.geometry_data.as_slice()).unwrap(),
        proto::Mesh::decode(b.geometry_data.as_slice()).unwrap());
    a.geometry_data.clear(); b.geometry_data.clear();
    assert_eq!(a, b);
}
fn imported() -> Scene {
    let mut scene = Scene::empty();
    crate::scene_document::import(&mut scene, &crate::interaction_example::bytes(), "fixture",
        &Xform::translation(1_000_000_000.125, 0., 0.), 0.).unwrap();
    scene
}

#[test]
fn hosted_features_keep_full_type_source_name_edge_order_and_orphans() {
    let session = crate::interaction_example::session();
    let host = session.objects.elements.iter().find(|e| e.name == "host").unwrap();
    assert_eq!(crate::feature_plan::own(host), [0]);
    let before = host.to_proto();
    let groups = crate::feature_plan::hosted(&session, host);
    same_element(host.to_proto(), before);
    assert_eq!(groups.iter().map(|g| (g.label.as_str(), g.features.clone())).collect::<Vec<_>>(), [
        ("InteractionContactFace seam", vec![1]),
        ("InteractionFeatureSolid connector", vec![2]),
        ("cut unclaimed · face 5", vec![3]),
    ]);
    assert_eq!(crate::feature_plan::label(&host.features[1]), "contact side_side · face 2");
}

#[test]
fn imported_children_stay_editable_and_features_follow_the_host_once() {
    let mut e = Editor::default(); e.scene = imported();
    let host = owner(&e.scene, "host"); let child = owner(&e.scene, "child");
    assert!(!e.scene.forest.readonly(child));
    assert_eq!(e.scene.pick_id(child.value()), Some(child));
    assert_eq!(e.scene.paths().len(), 6);
    let attributes = crate::element_adopt::attribute_ids(&e.scene);
    let interactions = crate::element_adopt::interaction_ids(&e.scene);
    assert_eq!(attributes.len(), 2); assert_eq!(interactions.len(), 4);
    assert!(!attributes.contains(&child));
    assert!(attributes.iter().all(|id| !e.scene.visible(*id)));
    assert!(interactions.iter().all(|id| e.scene.visible(*id)));
    assert!(e.scene.paths().iter().all(|row| e.scene.forest.readonly(row.id) && e.scene.forest.derived(row.id)));
    let labels = crate::hierarchy::rows(&e.scene).into_iter().map(|row| row.label).collect::<Vec<_>>();
    assert!(labels.contains(&"InteractionContactFace seam".into()));
    assert!(labels.contains(&"InteractionFeatureSolid connector".into()));
    assert_eq!(e.scene.placement(host).unwrap().m[12], 1_000_000_010.125);
    assert_eq!(e.scene.placement(child).unwrap().m[12], 1_000_000_012.125);
    e.apply(Action::ElementAttributes(Some(true))).unwrap();
    e.apply(Action::ElementInteractions(Some(false))).unwrap();
    assert!(attributes.iter().all(|id| e.scene.visible(*id)));
    assert!(interactions.iter().all(|id| !e.scene.visible(*id)));
    assert!(e.scene.visible(child));
    let ids = e.scene.with_descendants(&[host]);
    let before = ids.iter().map(|id| e.scene.placement(*id).unwrap()).collect::<Vec<_>>();
    let source = original(&e.scene, host); let original_bytes = source.to_proto();
    e.select_objects([host, child], false);
    e.apply(Action::Translate([3., 4., 5.])).unwrap();
    for (id, old) in ids.iter().zip(before) {
        let now = e.scene.placement(*id).unwrap();
        assert_eq!(now.m[12], old.m[12] + 3.);
        assert_eq!(now.m[13], old.m[13] + 4.);
        assert_eq!(now.m[14], old.m[14] + 5.);
    }
    same_element(source.to_proto(), original_bytes);
    e.apply(Action::Undo).unwrap();
    assert_eq!(e.scene.placement(child).unwrap().m[12], 1_000_000_012.125);
    e.select_objects([host], false); e.apply(Action::Delete).unwrap();
    assert!(ids.iter().all(|id| !e.scene.contains(*id)));
    e.apply(Action::Undo).unwrap();
    assert!(ids.iter().all(|id| e.scene.contains(*id)));
    assert!(Rc::ptr_eq(&source, &original(&e.scene, host)));
}

#[test]
fn repeated_archives_keep_original_features_and_do_not_accumulate_display_paths() {
    let session = crate::interaction_example::session(); let message = session.to_proto();
    let mut scene = Scene::empty();
    crate::scene_document::import(&mut scene, &message.encode_to_vec(), "original", &Xform::identity(), 0.).unwrap();
    for _ in 0..3 {
        let bytes = crate::scene_archive::save(&scene).unwrap();
        let bundle = crate::archive::Bundle::decode(&bytes).unwrap();
        assert_eq!(bundle.documents.len(), 1);
        let saved = proto::Session::decode(bundle.documents[0].as_slice()).unwrap();
        let objects = saved.objects.as_ref().unwrap();
        assert_eq!(objects.elements.len(), 3);
        assert!(objects.polylines.is_empty());
        assert_eq!(saved.interactions, message.interactions);
        for element in &objects.elements {
            let original = message.objects.as_ref().unwrap().elements.iter().find(|e| e.guid == element.guid).unwrap();
            same_element(element.clone(), original.clone());
        }
        crate::scene_document::open(&mut scene, &bytes).unwrap();
        assert_eq!(scene.paths().len(), 6);
        assert_eq!(crate::element_adopt::attribute_ids(&scene).len(), 2);
        assert_eq!(crate::element_adopt::interaction_ids(&scene).len(), 4);
        assert!(!scene.forest.readonly(owner(&scene, "child")));
    }
}

#[test]
fn malformed_feature_projection_does_not_replace_the_open_scene() {
    let mut message = crate::interaction_example::session().to_proto();
    message.objects.as_mut().unwrap().elements.iter_mut().find(|e| e.name == "host").unwrap()
        .features[1].outlines[0].coords[0] = f64::NAN;
    let mut scene = imported();
    let before = scene.all_ids(); let paths = scene.paths().len();
    assert!(crate::scene_document::import(&mut scene, &message.encode_to_vec(), "invalid", &Xform::identity(), 0.).is_err());
    assert_eq!(scene.all_ids(), before); assert_eq!(scene.paths().len(), paths);
}
