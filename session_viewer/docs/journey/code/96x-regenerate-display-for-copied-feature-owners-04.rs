use crate::{scene::Scene, editor::{Editor, Action}};
use session_rust::{Xform, Geometry, proto};
use std::rc::Rc;
use prost::Message;

fn same_source(actual: &proto::Element, expected: &proto::Element) {
    assert_eq!(proto::Mesh::decode(actual.geometry_data.as_slice()).unwrap(),
        proto::Mesh::decode(expected.geometry_data.as_slice()).unwrap());
    let mut a = actual.clone(); let mut b = expected.clone();
    a.geometry_data.clear(); b.geometry_data.clear(); assert_eq!(a,b);
}

#[test]
fn copied_elements_get_fresh_original_identity_and_regenerate_only_their_own_display() {
    let mut e = Editor::default(); e.scene = Scene::empty();
    crate::scene_document::import(&mut e.scene, &crate::interaction_example::bytes(), "source", &Xform::identity(), 0.).unwrap();
    let host = e.scene.objects().iter().find(|row| row.metadata.name == "host").unwrap().id;
    let original = match e.scene.objects().iter().find(|row| row.id == host).unwrap().original.as_ref().unwrap() {Geometry::Element(source) => source.clone(), _ => panic!("Element")};
    let before = original.to_proto();
    let child = e.scene.objects().iter().find(|row| row.metadata.name == "child").unwrap().id;
    let connector = e.scene.objects().iter().find(|row| row.metadata.name == "connector").unwrap().id;
    crate::layer_edit::attach_attributes(&mut e.scene, child, vec![connector], "attributes").unwrap();
    let ids = crate::layer_edit::copy_objects(&mut e.scene, &[host], &Xform::translation(4., 0., 0.)).unwrap();
    assert_eq!(ids.len(), 1); let copied = ids[0];
    let row = e.scene.objects().iter().find(|row| row.id == copied).unwrap();
    let Geometry::Element(copy) = row.original.as_ref().unwrap() else {panic!("Copied Element")};
    assert!(!Rc::ptr_eq(&original, copy)); assert_eq!(copy.guid(), row.guid); assert_ne!(copy.guid(), original.guid());
    assert_eq!(copy.features.iter().map(|f|f.to_proto()).collect::<Vec<_>>(), before.features);
    same_source(&original.to_proto(), &before);
    assert_eq!(e.scene.objects().len(), 5); assert_eq!(e.scene.paths().len(), 12);
    assert_eq!(e.scene.objects().iter().filter(|row|row.metadata.name == "connector").count(), 1, "Copy keeps direct source attributes, not their nested tree descendants");
    let tree = &e.scene.forest.documents[0];
    let index = tree.nodes.iter().position(|node| node.object == Some(copied)).unwrap();
    let children = &tree.nodes[index+1..tree.nodes[index].end];
    assert_eq!(children.iter().filter(|node|node.derived && node.object.is_some()).count(), 6);
    assert_eq!(children.iter().filter(|node|!node.derived && node.object.is_some()).count(), 1);
    assert!(children.iter().all(|node|node.label != "InteractionContactFace seam"));
    for node in children.iter().filter(|node|node.derived && node.object.is_some()) {
        let path = e.scene.paths().iter().find(|row| Some(row.id) == node.object).unwrap();
        let owner = crate::feature_owner::row_owner(&e.scene, &format!("{}/{}", tree.key, node.source.guid)).unwrap();
        assert!(owner == copied || e.scene.objects().iter().any(|row| row.id == owner && row.metadata.name == "child"));
        assert_eq!(path.model.m, e.scene.placement(owner).unwrap().m);
    }
    let placements = e.scene.objects().iter().map(|row| (row.guid.clone(), row.model.m)).collect::<std::collections::HashMap<_, _>>();
    for _ in 0..3 {
        let saved = crate::scene_archive::save(&e.scene).unwrap();
        let archive = crate::archive::Bundle::decode(&saved).unwrap();
        let session = proto::Session::decode(archive.documents[0].as_slice()).unwrap();
        assert_eq!(session.objects.as_ref().unwrap().elements.len(), 5);
        assert!(session.objects.as_ref().unwrap().polylines.is_empty());
        crate::scene_document::open(&mut e.scene, &saved).unwrap();
        assert_eq!(e.scene.objects().len(), 5); assert_eq!(e.scene.paths().len(), 12);
        for row in e.scene.objects() { assert_eq!(row.model.m, placements[&row.guid]); }
    }
}

#[test]
fn copied_instances_keep_one_definition_and_separate_reference_identity() {
    let mut e = Editor::default(); e.scene = Scene::empty();
    crate::scene_document::import(&mut e.scene, include_bytes!("../assets/pb/course-instance-features.pb"), "instances", &Xform::identity(), 0.).unwrap();
    let first = e.scene.objects()[0].id; let original = e.scene.instances[&first].clone();
    let copied = crate::layer_edit::copy_objects(&mut e.scene, &[first], &Xform::translation(2., 0., 0.)).unwrap()[0];
    let owner = &e.scene.instances[&copied];
    assert!(!Rc::ptr_eq(&original.instance, &owner.instance)); assert_eq!(owner.instance.guid(), e.scene.row_guid(copied).unwrap());
    assert_ne!(original.instance.guid(), owner.instance.guid());
    let (Geometry::Element(a), Geometry::Element(b)) = (&original.definition, &owner.definition) else {panic!("Element definitions")};
    assert!(Rc::ptr_eq(a,b)); assert_eq!(e.scene.paths().len(), 6);
    e.apply(Action::ElementAttributes(Some(true))).unwrap(); assert_eq!(crate::feature_owner::highlighted(&e.scene, &[copied]).len(), 3);
    let saved = crate::scene_archive::save(&e.scene).unwrap();
    let archive = crate::archive::Bundle::decode(&saved).unwrap();
    let session = proto::Session::decode(archive.documents[0].as_slice()).unwrap();
    assert_eq!(session.definitions.unwrap().elements.len(), 1); assert_eq!(session.objects.unwrap().instances.len(), 3);
    crate::scene_document::open(&mut e.scene, &saved).unwrap(); assert_eq!(e.scene.paths().len(), 6);
}
