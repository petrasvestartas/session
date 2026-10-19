use crate::{editor::{Action,Editor},view_flags::Flags};
use std::rc::Rc;

#[test]
fn hide_show_and_undo_keep_original_sources_and_presentation_separate() {
    let mut e=Editor::default();e.apply(Action::Close).unwrap();e.apply(Action::AddNested).unwrap();let id=e.scene.objects()[0].id;let child=e.scene.objects()[1].id;
    let source=Rc::clone(e.scene.objects()[0].geometry().unwrap());let original=source.to_proto();
    e.select_objects([id],false);e.apply(Action::Hide).unwrap();assert!(!e.scene.visible(id)&&!e.scene.visible(child));assert!(e.selected_objects().is_empty());
    assert_eq!(e.scene.objects()[0].geometry().unwrap().to_proto(),original);
    e.apply(Action::AddBox).unwrap();e.apply(Action::Undo).unwrap();assert!(!e.scene.visible(id)&&!e.scene.visible(child));
    e.apply(Action::Show).unwrap();assert!(e.scene.visible(id)&&e.scene.visible(child));e.apply(Action::Redo).unwrap();assert!(e.scene.visible(id));assert_eq!(e.scene.objects().len(),4);
    assert!(Rc::ptr_eq(&source,e.scene.objects()[0].geometry().unwrap()));e.apply(Action::Undo).unwrap();e.apply(Action::Undo).unwrap();assert!(e.scene.objects().is_empty(),"Hide and Show created no document transaction");
}

#[test]
fn show_reveals_initially_hidden_originals_without_changing_saved_flags() {
    let mut e=Editor::default();e.apply(Action::Close).unwrap();let mut mesh=session_rust::Mesh::create_box(1.,1.,1.);mesh.is_visible=false;
    let id=e.scene.insert(crate::prepared::PreparedMesh::new(mesh).unwrap()).unwrap();assert!(!e.scene.visible(id));e.apply(Action::Show).unwrap();assert!(e.scene.visible(id));assert!(!e.scene.objects()[0].geometry().unwrap().is_visible);
    assert_eq!(e.scene.pick_id(id.value()),Some(id));assert!(e.scene.bounds().is_some());
}

#[test]
fn presentation_flags_clone_without_pinning_sources_or_mutating_other_snapshots() {
    let mut e=Editor::default();let id=e.scene.objects()[0].id;let saved=e.scene.clone();let source=Rc::downgrade(e.scene.objects()[0].geometry().unwrap());
    assert!(Rc::ptr_eq(&saved.view,&e.scene.view));Rc::make_mut(&mut e.scene.view).lock(&[id],true);assert!(!saved.view.locked.contains(&id));assert_eq!(e.scene.pick_id(id.value()),None);
    let flags=Rc::clone(&e.scene.view);e.apply(Action::Close).unwrap();drop(saved);assert!(source.upgrade().is_none());assert!(flags.locked.contains(&id));assert!(e.scene.view.locked.is_empty());
}

#[test]
fn view_flags_hold_plain_ids_and_rgb_channels_and_restore_originals_independently() {
    let e=Editor::default();let id=e.scene.objects()[0].id;let mut flags=Flags::default();flags.initial(id,false,true);assert!(flags.hidden.contains(&id)&&flags.locked.contains(&id));
    flags.hide(&[id],false);assert!(!flags.hidden.contains(&id));flags.lock(&[id],false);assert!(!flags.locked.contains(&id));flags.faces.insert(id,[255,0,0]);flags.edges.insert(id,[0,0,255]);flags.faces.remove(&id);assert_eq!(flags.edges[&id],[0,0,255]);
}

#[test]
fn layer_row_properties_follow_descendants_and_protect_the_current_layer() {
    let mut e=Editor::default();e.apply(Action::Close).unwrap();e.apply(Action::AddNested).unwrap();let rows=crate::hierarchy::rows(&e.scene);let group=rows.iter().find(|r|r.label=="Posts").unwrap();let key=group.key.clone();let ids=group.ids.clone();
    Rc::make_mut(&mut e.scene.view).current=Some(key.clone());assert!(e.apply(Action::LayerProperty {key:key.clone(),property:crate::layer_properties::Property::Hidden(true)}).is_err());assert!(ids.iter().all(|id|e.scene.visible(*id)));
    Rc::make_mut(&mut e.scene.view).current=None;e.apply(Action::LayerProperty {key:key.clone(),property:crate::layer_properties::Property::Hidden(true)}).unwrap();assert!(ids.iter().all(|id|!e.scene.visible(*id)));e.apply(Action::Show).unwrap();
    e.apply(Action::LayerProperty {key:key.clone(),property:crate::layer_properties::Property::Locked(true)}).unwrap();assert!(ids.iter().all(|id|e.scene.pick_id(id.value()).is_none()));
    for edge in [false,true] {e.apply(Action::LayerProperty {key:key.clone(),property:crate::layer_properties::Property::Colour {edge,rgb:Some([65,130,225])}}).unwrap();}
    assert!(ids.iter().all(|id|e.scene.view.faces[id]==[65,130,225]&&e.scene.view.edges[id]==[65,130,225]));e.apply(Action::LayerProperty {key,property:crate::layer_properties::Property::Colour {edge:false,rgb:None}}).unwrap();assert!(ids.iter().all(|id|!e.scene.view.faces.contains_key(id)&&e.scene.view.edges.contains_key(id)));
    e.apply(Action::Undo).unwrap();assert!(e.scene.objects().is_empty(),"Presentation properties created no document history");
}

#[test]
fn nearest_source_node_colours_seed_faces_and_original_clears_only_the_view_override() {
    use prost::Message;
    let mut source=session_rust::proto::Session::decode(crate::tree_specimen::nested(true).as_slice()).unwrap();let root=source.tree.as_mut().unwrap().root.as_mut().unwrap();let posts=&mut root.children[0];
    posts.color=Some(session_rust::Color::red().to_proto());posts.children[0].children[0].children[0].color=Some(session_rust::Color::blue().to_proto());
    let original=source.objects.as_ref().unwrap().meshes.clone();let mut e=Editor::default();e.apply(Action::Replace(source.encode_to_vec())).unwrap();let id=e.scene.objects().iter().map(|row|row.id).collect::<Vec<_>>();
    assert_eq!(e.scene.view.faces[&id[0]],[255,0,0]);assert_eq!(e.scene.view.faces[&id[1]],[0,0,255]);assert!(!e.scene.view.faces.contains_key(&id[2]));assert!(e.scene.view.edges.is_empty());
    let key=crate::hierarchy::rows(&e.scene).into_iter().find(|row|row.label=="Posts").unwrap().key;e.apply(Action::LayerProperty {key,property:crate::layer_properties::Property::Colour {edge:false,rgb:None}}).unwrap();assert!(e.scene.view.faces.is_empty());
    for (row,mesh) in e.scene.objects().iter().zip(original) {assert_eq!(row.geometry().unwrap().to_proto(),mesh);}
    let group=e.scene.forest.documents[0].nodes.iter().find(|node|node.label=="Posts").unwrap();assert_eq!(group.source.color.as_ref().unwrap().r,1.);
}

#[test]
fn parent_lamp_preserves_child_visibility_and_child_lamps() {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e.apply(Action::AddNested).unwrap();
    let rows = crate::hierarchy::rows(&e.scene);
    let posts = rows.iter().find(|row| row.label == "Posts").unwrap();
    let root = rows[0].key.clone();
    let first = posts.ids[0];
    Rc::make_mut(&mut e.scene.view).hide(&[first], true);
    let set = |e: &mut Editor, key: String, on| e.apply(Action::LayerProperty {
        key, property: crate::layer_properties::Property::Hidden(on),
    }).unwrap();
    set(&mut e, posts.key.clone(), true);
    set(&mut e, root.clone(), true);
    set(&mut e, root, false);
    assert!(e.scene.view.lamps.contains(&posts.key));
    assert!(posts.ids.iter().all(|id| !e.scene.visible(*id)));
    set(&mut e, posts.key.clone(), false);
    assert!(!e.scene.visible(first));
    assert!(posts.ids.iter().filter(|id| **id != first).all(|id| e.scene.visible(*id)));
    e.apply(Action::Show).unwrap();
    assert!(e.scene.view.lamps.is_empty() && e.scene.view.held.is_empty());
    assert!(posts.ids.iter().all(|id| e.scene.visible(*id)));
}
