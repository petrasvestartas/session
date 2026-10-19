use crate::{editor::{Action,Editor},scene_label::{Prepared,example},text::TextPlacement};
use std::rc::Rc;
fn plane(world:[f64;3])->TextPlacement {TextPlacement::WorldPlane {world,right:[1.,0.,0.],up:[0.,1.,0.],world_height:0.22}}

#[test]
fn original_scene_words_precision_placement_and_history_keep_one_source_owner() {
 let mut e=Editor::default();e.apply(Action::Close).unwrap();let source=Rc::new(example(plane([1_000_000_000.125,2.25,0.5])));let saved=serde_json::to_string(&*source).unwrap();let prepared=Prepared::new(source.clone()).unwrap();let id=e.scene.insert_text(prepared).unwrap();
 e.apply(Action::Picked(Some(id))).unwrap();let intent=crate::edit_intent::Intent::Move{id,offset:[0.25,-0.5,1.]};assert!(intent.keys(&e).unwrap().is_empty(),"An already loaded text needs no mesh source fetch");assert!(matches!(intent.replay(&mut e).unwrap(),crate::edit_replay::Reply::Changed(crate::editor::Change::Scene)));assert_eq!(e.scene.texts()[0].label(false,true).unwrap().placement,plane([1_000_000_000.375,1.75,1.5]));assert!(Rc::ptr_eq(&e.scene.texts()[0].prepared.source,&source));assert_eq!(serde_json::to_string(&*source).unwrap(),saved);assert_eq!(crate::memory::texts(e.presentation_scenes())[1],1);
 e.apply(Action::Undo).unwrap();assert_eq!(e.scene.texts()[0].label(false,true).unwrap().placement,plane([1_000_000_000.125,2.25,0.5]));e.apply(Action::Redo).unwrap();
 let original=e.scene.texts()[0].model.m;let mut scale=session_rust::Xform::identity();scale.m[0]=2.;assert!(crate::history::History::default().try_edit(&mut e.scene,|s|s.place_text(id,scale)).is_err());assert_eq!(e.scene.texts()[0].model.m,original);assert_eq!(serde_json::to_string(&*source).unwrap(),saved);assert!(crate::document::snapshot(&e.scene).is_err(),"Mixed text persistence connects later; never silently omit it");
}

#[test]
fn text_rows_follow_layers_selection_delete_undo_copy_and_close_lifetime() {
 let mut e=Editor::default();e.apply(Action::Close).unwrap();e.apply(Action::AddText(plane([0.;3]))).unwrap();let id=e.scene.texts()[0].id;let weak=Rc::downgrade(&e.scene.texts()[0].prepared.source);let rows=crate::hierarchy::rows(&e.scene);let key=rows.iter().find(|row|row.ids==[id]&&row.depth==1).unwrap().key.clone();
 e.apply(Action::LayerProperty {key:key.clone(),property:crate::layer_properties::Property::Locked(true)}).unwrap();e.apply(Action::SelectAll).unwrap();assert!(e.selected_objects().is_empty());assert_eq!(e.scene.text_labels(&[]).unwrap()[0].object.unwrap().row,u32::MAX);
 e.apply(Action::LayerProperty {key:key.clone(),property:crate::layer_properties::Property::Locked(false)}).unwrap();e.apply(Action::SelectAll).unwrap();assert_eq!(e.selected_objects(),vec![id]);let source=e.scene.texts()[0].prepared.source.clone();let copied=e.scene.copy_row(id,false).unwrap();assert_ne!(copied,id);assert_ne!(e.scene.texts()[0].guid,e.scene.texts()[1].guid);assert!(Rc::ptr_eq(&source,&e.scene.texts()[1].prepared.source));e.scene.remove(copied);drop(source);
 e.apply(Action::LayerProperty {key,property:crate::layer_properties::Property::Hidden(true)}).unwrap();assert!(e.scene.text_labels(&[]).unwrap().is_empty());e.apply(Action::Show).unwrap();e.apply(Action::Picked(Some(id))).unwrap();e.apply(Action::Delete).unwrap();assert!(e.scene.texts().is_empty());assert!(weak.upgrade().is_some());e.apply(Action::Undo).unwrap();assert_eq!(e.scene.texts()[0].id,id);e.apply(Action::Redo).unwrap();e.apply(Action::Close).unwrap();assert!(weak.upgrade().is_none());assert_eq!(crate::memory::texts(e.presentation_scenes()),[0;4]);
}

#[test]
fn world_text_bounds_and_rectangles_use_the_original_words_and_plate_extent() {
 let mut e=Editor::default();e.apply(Action::Close).unwrap();e.apply(Action::AddText(plane([0.,0.,0.5]))).unwrap();let row=&e.scene.texts()[0];let id=row.id;let bounds=e.scene.selected_bounds(id).unwrap();assert!(bounds.max[0]>1.&&bounds.min[1]<0.&&bounds.max[2]==0.5);assert_eq!(e.scene.bounds().unwrap().min,bounds.min);
 let matrix=session_rust::Xform::identity().m;let points=crate::scene_label_screen::polygon(row,&matrix,[800.,600.]).unwrap();let small=crate::rect_select::Rectangle::new([405.,305.],[401.,301.]).unwrap();assert!(crate::rect_select::found(&e.scene,&matrix,[800.,600.],small).contains(&id),"Crossing rectangle wholly inside the plate selects its source row");
 let tiny=crate::rect_select::Rectangle::new([401.,301.],[405.,305.]).unwrap();assert!(!crate::rect_select::found(&e.scene,&matrix,[800.,600.],tiny).contains(&id));let min=[points.iter().map(|p|p[0]).fold(f64::INFINITY,f64::min)-1.,points.iter().map(|p|p[1]).fold(f64::INFINITY,f64::min)-1.];let max=[points.iter().map(|p|p[0]).fold(f64::NEG_INFINITY,f64::max)+1.,points.iter().map(|p|p[1]).fold(f64::NEG_INFINITY,f64::max)+1.];assert_eq!(crate::rect_select::found(&e.scene,&matrix,[800.,600.],crate::rect_select::Rectangle::new(min,max).unwrap()),[id]);
}

#[test]
fn invalid_scene_labels_do_not_consume_history_or_turn_screen_overlays_into_sources() {
 let mut e=Editor::default();e.apply(Action::Close).unwrap();e.apply(Action::AddText(plane([0.;3]))).unwrap();e.apply(Action::Undo).unwrap();let source=Rc::new(example(TextPlacement::Screen {left:0.,top:0.}));assert!(Prepared::new(source).is_err());let source=Rc::new(example(plane([f64::NAN,0.,0.])));assert!(Prepared::new(source).is_err());assert!(e.apply(Action::Redo).is_ok());assert_eq!(e.scene.texts().len(),1);assert!(e.scene.bounds().is_some());
}
