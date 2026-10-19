use crate::{editor::{Editor,Action},document,hierarchy,scene::ObjectId};
use session_rust::{proto,Xform};
use prost::Message;
use std::rc::Rc;

use crate::tree_specimen::nested;
fn editor(attribute:bool)->Editor {let mut e=Editor::default();e.apply(Action::Replace(nested(attribute))).unwrap();e}
fn ids(e:&Editor)->Vec<ObjectId> {e.scene.objects().iter().map(|r|r.id).collect()}
fn models(e:&Editor)->Vec<[f64;16]> {e.scene.objects().iter().map(|r|r.model.m).collect()}

#[test]
fn nested_tree_rows_keep_original_guids_precise_world_placements_and_readonly_attributes() {
    let mut e=editor(true);let id=ids(&e);let rows=hierarchy::rows(&e.scene);
    assert_eq!(rows.iter().map(|r|r.depth).collect::<Vec<_>>(),vec![0,1,2,3,4,1]);
    assert_eq!(rows[1].ids,vec![id[0],id[1]]);assert!(rows[4].readonly);assert!(!rows[2].readonly);
    let model=models(&e);assert_eq!(model[0][12],0.123456789012345);assert_eq!(model[1][12],model[0][12]);assert_eq!(model[1][13],0.25);
    assert!(e.scene.visible(id[1]));assert_eq!(e.scene.pick_id(id[1].value()),None);
    let before=e.scene.objects()[1].geometry().unwrap().to_proto();
    assert!(e.apply(Action::TreePick {ids:vec![id[1]],additive:false}).is_err());
    assert!(e.scene.place(id[1],Xform::identity()).is_err());assert!(e.scene.translate(id[1],[1.,0.,0.]).is_err());assert!(e.delete_object(id[1]).is_err());
    assert_eq!(e.scene.objects()[1].geometry().unwrap().to_proto(),before);
}

#[test]
fn parent_and_child_selection_preview_moves_each_descendant_once_and_commits_one_undo() {
    for attribute in [false,true] {
        let mut e=editor(attribute);let id=ids(&e);let before=models(&e);let roots=if attribute {vec![id[0]]} else {vec![id[0],id[1]]};
        let mut preview=crate::edit_preview::Preview::new(&e.scene,roots).unwrap();assert_eq!(preview.ids(),&id[..2]);
        preview.set(Xform::translation(0.5,0.,0.)).unwrap();assert_eq!(models(&e),before);
        for index in 0..2 {assert_eq!(preview.scene.objects()[index].model.m[12],before[index][12]+0.5);assert!(Rc::ptr_eq(preview.scene.objects()[index].geometry().unwrap(),e.scene.objects()[index].geometry().unwrap()));}
        e.apply(Action::TreePick {ids:if attribute {vec![id[0]]} else {vec![id[0],id[1]]},additive:false}).unwrap();e.apply(Action::Translate([0.5,0.,0.])).unwrap();let after=models(&e);assert_eq!(after[2],before[2]);
        e.apply(Action::Undo).unwrap();assert_eq!(models(&e),before);e.apply(Action::Redo).unwrap();assert_eq!(models(&e),after);
        e.apply(Action::Undo).unwrap();e.apply(Action::Undo).unwrap();assert_eq!(e.scene.objects().len(),2,"One parent drag added one transaction after Replace");
    }
}

#[test]
fn nested_save_reopen_preserves_source_metadata_hierarchy_and_world_placements() {
    let mut e=editor(true);let id=ids(&e);e.apply(Action::TreePick {ids:vec![id[0]],additive:false}).unwrap();e.apply(Action::Translate([0.5,0.,0.])).unwrap();
    let before=models(&e);let original=e.scene.objects().iter().map(|r|r.geometry().unwrap().to_proto()).collect::<Vec<_>>();
    let saved=document::snapshot(&e.scene).unwrap();let mut reopened=Editor::default();reopened.apply(Action::Replace(saved)).unwrap();assert_eq!(models(&reopened),before);
    let rows=hierarchy::rows(&reopened.scene);assert!(rows.iter().any(|r|r.label=="Posts"));assert!(rows.iter().any(|r|r.label=="attributes"));
    for (index,row) in reopened.scene.objects().iter().enumerate() {assert_eq!(row.geometry().unwrap().to_proto(),original[index]);}
    assert!(reopened.scene.forest.readonly(reopened.scene.objects()[1].id));
    for expected in [reopened.scene.objects()[0].id,reopened.scene.objects()[2].id,reopened.scene.objects()[0].id] {reopened.apply(Action::SelectNext).unwrap();assert_eq!(reopened.selected,Some(expected));}
}

#[test]
fn repeated_documents_keep_distinct_tree_keys_and_undo_keeps_expansion_identity() {
    let mut e=editor(true);let first=Rc::clone(&e.scene.forest.documents[0]);let keys=hierarchy::rows(&e.scene).iter().map(|r|r.key.clone()).collect::<Vec<_>>();
    e.apply(Action::Import(nested(true))).unwrap();assert_eq!(e.scene.forest.documents.len(),2);assert_ne!(e.scene.forest.documents[0].key,e.scene.forest.documents[1].key);
    let rows=hierarchy::rows(&e.scene);let unique=rows.iter().map(|r|&r.key).collect::<std::collections::HashSet<_>>();assert_eq!(unique.len(),rows.len());
    e.apply(Action::Undo).unwrap();assert!(Rc::ptr_eq(&first,&e.scene.forest.documents[0]));assert_eq!(hierarchy::rows(&e.scene).iter().map(|r|r.key.clone()).collect::<Vec<_>>(),keys);
}

#[test]
fn deleting_a_parent_removes_attached_attributes_and_undo_restores_the_same_sources() {
    let mut e=editor(true);let id=ids(&e);let source=Rc::clone(e.scene.objects()[1].geometry().unwrap());let tree=Rc::clone(&e.scene.forest.documents[0]);
    e.apply(Action::TreePick {ids:vec![id[0]],additive:false}).unwrap();e.apply(Action::Delete).unwrap();assert_eq!(e.scene.objects().len(),1);
    assert!(hierarchy::rows(&e.scene).iter().all(|r|!r.ids.contains(&id[0])&&!r.ids.contains(&id[1])));
    e.apply(Action::Undo).unwrap();assert_eq!(ids(&e),id);assert!(Rc::ptr_eq(&source,e.scene.objects()[1].geometry().unwrap()));assert!(Rc::ptr_eq(&tree,&e.scene.forest.documents[0]));
}

#[test]
fn nested_validation_rejects_missing_duplicate_and_excessively_deep_source_rows() {
    let original=proto::Session::decode(nested(true).as_slice()).unwrap();
    for case in 0..3 {
        let mut m=original.clone();let root=m.tree.as_mut().unwrap().root.as_mut().unwrap();
        match case {0=>{root.children.pop();},1=>{root.children.push(root.children[1].clone());},_=>{let mut node=root.children.pop().unwrap();for _ in 0..=hierarchy::MAX_DEPTH {node=proto::TreeNode {name:"group".into(),children:vec![node],..Default::default()};}root.children.push(node);}}
        assert!(document::load(&m.encode_to_vec()).is_err());
    }
}

#[test]
fn tree_metadata_does_not_pin_a_closed_document_or_source_mesh() {
    let mut e=editor(true);let tree=Rc::clone(&e.scene.forest.documents[0]);let doc=Rc::downgrade(&e.scene.objects()[0].source().unwrap().document);let source=Rc::downgrade(e.scene.objects()[0].geometry().unwrap());
    e.apply(Action::Close).unwrap();assert!(doc.upgrade().is_none());assert!(source.upgrade().is_none());assert!(!tree.nodes.is_empty());assert!(e.scene.forest.documents.is_empty());
}

#[test]
fn actual_tree_panel_expands_indents_and_refuses_readonly_source_rows() {
    let e=editor(true);let mut model=crate::tree_panel::TreePanel::default();model.refresh(&e.scene,vec![]);model.open=true;
    assert_eq!(model.visible(),vec![0]);model.all();assert_eq!(model.visible().len(),6);
    let context=egui::Context::default();let mut controls=Some(Vec::new());let mut drawn=egui::Rect::NOTHING;
    for _ in 0..2 {controls=Some(Vec::new());let input=egui::RawInput {screen_rect:Some(egui::Rect::from_min_size(egui::Pos2::ZERO,egui::vec2(640.,480.))),..Default::default()};let _=context.run_ui(input,|ui|crate::tree_panel::draw(ui,&mut model,&mut controls));drawn=model.rect.unwrap();}
    assert!(drawn.width()>=240.&&drawn.height()>100.);let controls=controls.unwrap();
    let select=controls.iter().filter(|c|c.key.starts_with("tree/select/")).collect::<Vec<_>>();assert_eq!(select.len(),6);
    assert!(select[4].rect[0]>select[2].rect[0],"Nested original rows are indented in the actual egui layout");
    let at=egui::pos2((select[4].rect[0]+select[4].rect[2])/2.,(select[4].rect[1]+select[4].rect[3])/2.);
    for pressed in [true,false] {let input=egui::RawInput {events:vec![egui::Event::PointerMoved(at),egui::Event::PointerButton {pos:at,button:egui::PointerButton::Primary,pressed,modifiers:Default::default()}],screen_rect:Some(egui::Rect::from_min_size(egui::Pos2::ZERO,egui::vec2(640.,480.))),..Default::default()};let _=context.run_ui(input,|ui|crate::tree_panel::draw(ui,&mut model,&mut Some(Vec::new())));}
    assert!(model.action.is_none(),"Actual disabled attribute button cannot select its source");
    model.open=false;let _=context.run_ui(egui::RawInput::default(),|ui|crate::tree_panel::draw(ui,&mut model,&mut Some(Vec::new())));assert!(model.rect.is_none());
}
