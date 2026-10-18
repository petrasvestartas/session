use crate::{editor::{Action, Editor}, scene::Scene};
use session_rust::{Element, Xform, Point, Polyline, proto, element::ElementFeature};
use prost::Message;

fn message() -> proto::Session {
    let mut source = crate::instance_specimen::message();
    let mesh = session_rust::Mesh::from_proto(source.definitions.as_ref().unwrap().meshes[0].clone());
    let mut definition = Element::new("prototype");
    definition.set_guid("prototype".into());
    definition.set_geometry(mesh);
    let outline = || Polyline::new(vec![Point::new(-0.2, 0., 0.), Point::new(0.2, 0., 0.)]);
    let mut attribute = ElementFeature::new("axis", -1, vec![outline()], "prototype axis");
    attribute.set_guid("prototype-axis".into());
    definition.features.push(attribute);
    source.definitions = Some(proto::Objects {elements: vec![definition.to_proto()], ..Default::default()});
    for (i, instance) in source.objects.as_mut().unwrap().instances.iter_mut().enumerate() {
        instance.definition_guid = "prototype".into();
        let mut feature = ElementFeature::new("cut", -1, vec![outline()], &format!("cut {i}"));
        feature.set_guid(format!("instance-feature-{i}"));
        instance.features.push(feature.to_proto());
    }
    source
}

fn same_definition(actual: &proto::Element, expected: &proto::Element) {
    assert_eq!(proto::Mesh::decode(actual.geometry_data.as_slice()).unwrap(),
        proto::Mesh::decode(expected.geometry_data.as_slice()).unwrap());
    let mut a = actual.clone(); let mut b = expected.clone();
    a.geometry_data.clear(); b.geometry_data.clear();
    assert_eq!(a, b);
}

#[test]
fn instance_projection_keeps_shared_sources_and_independent_feature_owners() {
    let source = message();
    let mut e = Editor::default(); e.scene = Scene::empty();
    crate::scene_document::import(&mut e.scene, &source.encode_to_vec(), "instances", &Xform::identity(), 0.).unwrap();
    assert_eq!(e.scene.instances.len(), 2); assert_eq!(e.scene.paths().len(), 4);
    let ids = e.scene.objects().iter().map(|row| row.id).collect::<Vec<_>>();
    let (session_rust::Geometry::Element(a), session_rust::Geometry::Element(b)) =
        (&e.scene.instances[&ids[0]].definition, &e.scene.instances[&ids[1]].definition)
        else {panic!("Element definitions")};
    assert!(std::rc::Rc::ptr_eq(a, b));
    assert_eq!(crate::feature_owner::highlighted(&e.scene, &[ids[0]]).len(), 2);
    e.apply(Action::ElementAttributes(Some(true))).unwrap();
    assert_eq!(crate::feature_owner::highlighted(&e.scene, &[ids[0]]).len(), 3);
    for path in e.scene.paths() {
        let doc = &e.scene.forest.documents[0];
        let index = doc.nodes.iter().position(|node| node.object == Some(path.id)).unwrap();
        let key = format!("{}/{}", doc.key, doc.nodes[index].source.guid);
        let owner = crate::feature_owner::row_owner(&e.scene, &key);
        let host = if path.model.m[12] == e.scene.placement(ids[0]).unwrap().m[12] {ids[0]} else {ids[1]};
        assert_eq!(path.model.m, e.scene.placement(host).unwrap().m);
        assert_eq!(owner, (host == ids[0]).then_some(host), "locked instance does not become editable");
    }
    let original = e.scene.instances[&ids[0]].document.to_proto();
    same_definition(&original.definitions.unwrap().elements[0], &source.definitions.as_ref().unwrap().elements[0]);
    assert_eq!(original.objects.unwrap().instances[0].features, source.objects.as_ref().unwrap().instances[0].features);
}

#[test]
fn saved_instances_regenerate_features_without_archiving_display_paths() {
    let source = message(); let mut e = Editor::default(); e.scene = Scene::empty();
    crate::scene_document::import(&mut e.scene, &source.encode_to_vec(), "instances", &Xform::identity(), 0.).unwrap();
    let first = e.scene.objects()[0].id;
    e.apply(Action::TreePick {ids: vec![first], additive: false}).unwrap();
    e.apply(Action::Delete).unwrap(); assert_eq!(e.scene.paths().len(), 2);
    e.apply(Action::Undo).unwrap(); assert_eq!(e.scene.paths().len(), 4);
    for _ in 0..3 {
        let saved = crate::scene_archive::save(&e.scene).unwrap();
        let bundle = crate::archive::Bundle::decode(&saved).unwrap();
        let document = proto::Session::decode(bundle.documents[0].as_slice()).unwrap();
        assert!(document.objects.as_ref().unwrap().polylines.is_empty());
        assert_eq!(document.objects.as_ref().unwrap().instances.len(), 2);
        same_definition(&document.definitions.as_ref().unwrap().elements[0], &source.definitions.as_ref().unwrap().elements[0]);
        crate::scene_document::open(&mut e.scene, &saved).unwrap();
        assert_eq!(e.scene.paths().len(), 4);
    }
}
