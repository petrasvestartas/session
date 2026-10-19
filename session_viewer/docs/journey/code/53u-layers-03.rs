use crate::{editor::{Action,Editor},layer_edit::{self,Edit}};
use std::rc::Rc;

fn editor()->Editor {let mut e=Editor::default();e.apply(Action::Close).unwrap();e.apply(Action::AddNested).unwrap();e}
fn key(e:&Editor,label:&str)->String {crate::hierarchy::rows(&e.scene).into_iter().find(|row|row.label==label).unwrap().key}
fn edit(e:&mut Editor,key:&str,edit:Edit)->Result<crate::editor::Change,&'static str> {e.apply(Action::LayerEdit {key:key.to_owned(),edit})}

#[test]
fn new_and_renamed_layers_keep_unrelated_keys_sources_and_one_undo() {
    let mut e=editor();let root=crate::hierarchy::rows(&e.scene)[0].key.clone();let original=crate::hierarchy::rows(&e.scene).into_iter().map(|row|row.key).collect::<Vec<_>>();
    let sources=e.scene.objects().iter().map(|row|Rc::clone(row.geometry().unwrap())).collect::<Vec<_>>();
    edit(&mut e,&root,Edit::New {sublayer:false}).unwrap();let layer=key(&e,"Layer 01");edit(&mut e,&layer,Edit::Current).unwrap();
    edit(&mut e,&layer,Edit::Rename(" Timber ".into())).unwrap();assert_eq!(key(&e,"Timber"),layer);assert_eq!(e.scene.view.current.as_ref(),Some(&layer));
    assert!(original.iter().all(|key|crate::hierarchy::rows(&e.scene).iter().any(|row|&row.key==key)));
    assert!(e.scene.objects().iter().zip(sources).all(|(row,source)|Rc::ptr_eq(row.geometry().unwrap(),&source)));
    e.apply(Action::Undo).unwrap();assert_eq!(key(&e,"Layer 01"),layer);e.apply(Action::Undo).unwrap();assert!(layer_edit::locate(&e.scene,&layer).is_err());e.apply(Action::Redo).unwrap();assert_eq!(key(&e,"Layer 01"),layer);
}

#[test]
fn layer_errors_preserve_redo_and_current_ancestors_cannot_be_hidden_or_deleted() {
    let mut e=editor();let root=crate::hierarchy::rows(&e.scene)[0].key.clone();edit(&mut e,&root,Edit::New {sublayer:true}).unwrap();let parent=key(&e,"Layer 01");
    edit(&mut e,&parent,Edit::New {sublayer:true}).unwrap();let child=key(&e,"Layer 02");edit(&mut e,&child,Edit::Current).unwrap();
    for target in [&parent,&child] {assert!(edit(&mut e,target,Edit::Delete).is_err());assert!(e.apply(Action::LayerProperty {key:target.clone(),property:crate::layer_properties::Property::Hidden(true)}).is_err());}
    assert!(edit(&mut e,&root,Edit::Rename("Root".into())).is_err());assert!(edit(&mut e,&root,Edit::Delete).is_err());
    e.apply(Action::Undo).unwrap();assert!(edit(&mut e,&parent,Edit::Rename(" / ".into())).is_err());assert!(edit(&mut e,&parent,Edit::Rename("Posts".into())).is_err());e.apply(Action::Redo).unwrap();assert_eq!(key(&e,"Layer 02"),child);
    let doc=&e.scene.forest.documents[0];let node=doc.nodes.iter().find(|node|node.object==Some(e.scene.objects()[0].id)).unwrap();let object=format!("{}/{}",doc.key,node.source.guid);assert!(edit(&mut e,&object,Edit::Current).is_err());
}

#[test]
fn deleting_a_populated_group_restores_descendants_and_readonly_ownership_on_undo() {
    let mut e=editor();let group=key(&e,"Posts");let rows=crate::hierarchy::rows(&e.scene);let ids=rows.iter().find(|row|row.key==group).unwrap().ids.clone();
    let child=e.scene.objects()[1].id;assert!(e.scene.forest.readonly(child));let source=Rc::clone(e.scene.objects()[0].geometry().unwrap());
    edit(&mut e,&group,Edit::Delete).unwrap();assert!(ids.iter().all(|id|!e.scene.contains(*id)));assert!(!e.scene.forest.readonly(child));
    e.apply(Action::Undo).unwrap();assert_eq!(key(&e,"Posts"),group);assert!(ids.iter().all(|id|e.scene.contains(*id)));assert!(e.scene.forest.readonly(child));assert_eq!(e.scene.pick_id(child.value()),None);assert!(Rc::ptr_eq(e.scene.objects()[0].geometry().unwrap(),&source));
    e.apply(Action::Redo).unwrap();assert!(ids.iter().all(|id|!e.scene.contains(*id)));
}

#[test]
fn created_and_current_layers_receive_every_generated_geometry_lane_and_survive_undo() {
    let mut e=editor();e.apply(Action::AddBox).unwrap();let created=key(&e,"Created");assert_eq!(layer_edit::current_key(&e.scene),Some(created.clone()));
    assert!(edit(&mut e,&created,Edit::Delete).is_err());let root=crate::hierarchy::rows(&e.scene)[0].key.clone();edit(&mut e,&root,Edit::New {sublayer:true}).unwrap();let layer=key(&e,"Layer 01");edit(&mut e,&layer,Edit::Current).unwrap();
    let count=e.scene.objects().len();for action in [Action::AddBox,Action::AddLine,Action::AddPoint,Action::AddCurve] {e.apply(action).unwrap();}
    let row=crate::hierarchy::rows(&e.scene).into_iter().find(|row|row.key==layer).unwrap();assert_eq!(row.ids.len(),4);assert_eq!(e.scene.objects().len(),count+1);
    for _ in 0..4 {e.apply(Action::Undo).unwrap();}assert!(crate::hierarchy::rows(&e.scene).iter().find(|row|row.key==layer).unwrap().ids.is_empty());assert_eq!(layer_edit::current_key(&e.scene),Some(layer));
    e.apply(Action::Close).unwrap();assert!(layer_edit::current_key(&e.scene).is_none());assert!(e.scene.view.current.is_none());
}

#[test]
fn changing_layers_across_documents_keeps_world_placement_and_attached_attributes_exact() {
    let mut e=editor();let ids=e.scene.objects().iter().map(|row|row.id).collect::<Vec<_>>();let source=Rc::clone(e.scene.objects()[0].geometry().unwrap());let models=e.scene.objects().iter().map(|row|row.model.m).collect::<Vec<_>>();
    e.apply(Action::Import(crate::tree_specimen::nested(false))).unwrap();let target_doc=&e.scene.forest.documents[1];let target=format!("{}/{}",target_doc.key,target_doc.nodes[0].source.guid);
    edit(&mut e,&target,Edit::Move(vec![ids[0]])).unwrap();assert!(e.scene.forest.documents[0].nodes.iter().all(|node|node.object!=Some(ids[0])&&node.object!=Some(ids[1])));
    assert!(e.scene.forest.documents[1].nodes.iter().any(|node|node.object==Some(ids[1])&&node.readonly));assert!(Rc::ptr_eq(e.scene.objects()[0].geometry().unwrap(),&source));assert_eq!(e.scene.objects()[0].model.m,models[0]);assert_eq!(e.scene.objects()[1].model.m,models[1]);
    let saved=crate::document::snapshot(&e.scene).unwrap();let mut reopened=Editor::default();reopened.apply(Action::Replace(saved)).unwrap();assert_eq!(reopened.scene.objects()[0].model.m,models[0]);assert_eq!(reopened.scene.objects()[1].model.m,models[1]);assert!(reopened.scene.forest.readonly(reopened.scene.objects()[1].id));
    assert!(edit(&mut e,&target,Edit::Move(vec![ids[0]])).is_err());e.apply(Action::Undo).unwrap();assert!(e.scene.forest.documents[0].nodes.iter().any(|node|node.object==Some(ids[0])));e.apply(Action::Redo).unwrap();assert!(e.scene.forest.documents[1].nodes.iter().any(|node|node.object==Some(ids[0])));
}

#[test]
fn duplicate_layers_copy_descendants_and_view_flags_but_share_precise_source_allocations() {
    let mut e=editor();let group=key(&e,"Posts");let parent=e.scene.objects()[0].clone();let child=e.scene.objects()[1].clone();
    e.apply(Action::LayerProperty {key:group.clone(),property:crate::layer_properties::Property::Hidden(true)}).unwrap();e.apply(Action::LayerProperty {key:group.clone(),property:crate::layer_properties::Property::Locked(true)}).unwrap();e.apply(Action::LayerProperty {key:group.clone(),property:crate::layer_properties::Property::Colour {edge:true,rgb:Some([17,29,231])}}).unwrap();
    edit(&mut e,&group,Edit::Duplicate).unwrap();let copy=key(&e,"Posts copy 01");let copied=crate::hierarchy::rows(&e.scene).into_iter().find(|row|row.key==copy).unwrap().ids;assert_eq!(copied.len(),2);
    for (id,original) in copied.iter().zip([parent,child]) {let row=e.scene.objects().iter().find(|row|row.id==*id).unwrap();assert_ne!(row.guid,original.guid);assert!(Rc::ptr_eq(row.geometry().unwrap(),original.geometry().unwrap()));assert!(Rc::ptr_eq(&row.mesh,&original.mesh));assert_eq!(row.model.m,original.model.m);assert!(!e.scene.visible(*id));assert!(e.scene.view.locked.contains(id));assert_eq!(e.scene.view.edges[id],[17,29,231]);}
    assert!(e.scene.forest.readonly(copied[1]));e.apply(Action::Undo).unwrap();assert_eq!(e.scene.objects().len(),3);e.apply(Action::Redo).unwrap();assert_eq!(key(&e,"Posts copy 01"),copy);
}

#[test]
fn copy_object_layer_keeps_originals_and_rejects_readonly_or_released_sources_without_history() {
    let mut e=editor();let root=crate::hierarchy::rows(&e.scene)[0].key.clone();let parent=e.scene.objects()[0].clone();let child=e.scene.objects()[1].id;
    assert!(edit(&mut e,&root,Edit::Copy(vec![child])).is_err());edit(&mut e,&root,Edit::Copy(vec![parent.id])).unwrap();let copy=e.scene.objects().last().unwrap();assert_ne!(copy.id,parent.id);assert!(Rc::ptr_eq(copy.geometry().unwrap(),parent.geometry().unwrap()));assert_eq!(copy.model.m,parent.model.m);assert!(!e.scene.forest.readonly(copy.id));assert_eq!(e.scene.objects().len(),4);
    e.apply(Action::Undo).unwrap();let origins=e.scene.objects().iter().filter_map(|row|row.origin().map(|origin|origin.id)).collect();e.scene.unload(&origins,7);
    assert!(edit(&mut e,&root,Edit::Current).is_err());assert!(edit(&mut e,&root,Edit::New {sublayer:true}).is_err());assert!(edit(&mut e,&root,Edit::Copy(vec![parent.id])).is_err());e.apply(Action::Redo).unwrap();assert_eq!(e.scene.objects().len(),4);
}

#[test]
fn object_copy_keeps_each_explicit_selection_while_parent_move_deduplicates_followers() {
    let mut e=Editor::default();e.apply(Action::Replace(crate::tree_specimen::nested(false))).unwrap();let root=crate::hierarchy::rows(&e.scene)[0].key.clone();let ids=e.scene.objects().iter().map(|row|row.id).collect::<Vec<_>>();
    edit(&mut e,&root,Edit::Copy(vec![ids[0],ids[1],ids[1]])).unwrap();assert_eq!(e.scene.objects().len(),5);assert_eq!(e.scene.objects()[3].model.m,e.scene.objects()[0].model.m);assert_eq!(e.scene.objects()[4].model.m,e.scene.objects()[1].model.m);
    e.apply(Action::Undo).unwrap();edit(&mut e,&root,Edit::New {sublayer:true}).unwrap();let target=key(&e,"Layer 01");let models=e.scene.objects().iter().map(|row|row.model.m).collect::<Vec<_>>();
    edit(&mut e,&target,Edit::Move(vec![ids[0],ids[1]])).unwrap();let row=crate::hierarchy::rows(&e.scene).into_iter().find(|row|row.key==target).unwrap();assert_eq!(row.ids,ids[..2]);assert_eq!(e.scene.objects().len(),3);assert_eq!(e.scene.objects().iter().map(|row|row.model.m).collect::<Vec<_>>(),models);
}

#[test]
fn portable_tree_identity_is_stable_across_repeated_saves_and_distinct_across_imports() {
    let mut e=editor();e.apply(Action::Import(crate::tree_specimen::nested(true))).unwrap();let first=crate::document::snapshot(&e.scene).unwrap();
    use prost::Message;let decoded=|bytes:&[u8]|session_rust::proto::Session::decode(bytes).unwrap();assert_eq!(decoded(&first),decoded(&crate::document::snapshot(&e.scene).unwrap()));
    let tree=decoded(&first).tree.unwrap().root.unwrap();let mut nodes=vec![&tree];let mut guids=std::collections::HashSet::new();while let Some(node)=nodes.pop() {if !node.guid.is_empty() {assert!(uuid::Uuid::parse_str(&node.guid).is_ok());assert!(guids.insert(&node.guid));}nodes.extend(node.children.iter());}
    let mut reopened=Editor::default();reopened.apply(Action::Replace(first)).unwrap();assert_eq!(reopened.scene.objects().len(),6);
}
