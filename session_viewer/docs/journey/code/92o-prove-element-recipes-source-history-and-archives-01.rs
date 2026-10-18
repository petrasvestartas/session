use crate::{editor::{Editor,Action},scene::Scene};
use session_rust::{Geometry,Polyline,Point};
use std::rc::Rc;
use wood::WoodElement;
pub fn square(side: f64, z: f64) -> Polyline {
        let corners = [
            (0.0, 0.0),
            (side, 0.0),
            (side, side),
            (0.0, side),
            (0.0, 0.0),
        ];
        Polyline::new(corners.iter().map(|&(x, y)| Point::new(x, y, z)).collect())
    }

fn command(e:&mut Editor,text:&str)->Result<crate::editor::Change,&'static str>{let action=if let Some(a)=e.tool_line(text){a?}else{let crate::commands::Command::Editor(a)=crate::commands::parse(text).map_err(|_|"Invalid command")?else{return Err("Editor command")};a};e.apply(action)}
fn line(e:&mut Editor)->crate::scene::ObjectId{let source=Rc::new(session_rust::Line::new(0.,0.,0.,1000.,0.,0.));e.scene.insert_line(crate::stroke::PreparedLine::new(source).unwrap()).unwrap()}
fn original(e:&Editor)->Rc<session_rust::Element>{e.scene.objects().iter().find_map(|r|if let Some(Geometry::Element(v))=&r.original{Some(v.clone())}else{None}).unwrap()}
#[test]
fn element_current_defaults_named_values_and_atomic_input_are_preserved(){
 let mut e=Editor::default();e.apply(Action::Close).unwrap();let id=line(&mut e);e.select_objects([id],false);command(&mut e,"Element Beam").unwrap();let draft=e.element.as_ref().unwrap();assert_eq!(draft.params.iter().map(|p|p.value).collect::<Vec<_>>(),[120.,200.]);assert_eq!(draft.ids(),[id]);
 command(&mut e,"Width").unwrap();command(&mut e,"").unwrap();assert!(e.element.is_some());command(&mut e,"Width=100 Height=150").unwrap();let before=e.element.as_ref().unwrap().params.clone();assert!(command(&mut e,"Width=99 Height=-1").is_err());assert_eq!(e.element.as_ref().unwrap().params,before);
 command(&mut e,"").unwrap();assert!(e.element.is_none());let source=original(&e);let beam=wood::Beam::from_element(&source).unwrap();assert!((beam.solid().volume().abs()-1000.*100.*150.).abs()<1e-6);assert!(!e.scene.contains(id));
}
#[test]
fn element_world_placements_consumed_sources_and_one_history_step_roundtrip(){
 let mut e=Editor::default();e.apply(Action::Close).unwrap();let id=line(&mut e);e.scene.place_line(id,session_rust::Xform::translation(0.125,2.,3.)).unwrap();let source=crate::source_edit::original(&e.scene,id).unwrap();let Geometry::Line(before)=source else{unreachable!()};let before_proto=before.to_proto();e.select_objects([id],false);command(&mut e,"Element Beam 100 200").unwrap();let made=original(&e);let _=made.guid();let made_proto=made.to_proto();let beam=wood::Beam::from_element(&made).unwrap();assert_eq!(beam.axis.get_point(0).unwrap(),Point::new(0.125,2.,3.));assert!(!e.scene.contains(id));let parent=e.selected_objects()[0];let attrs=e.scene.with_descendants(&[parent]);assert!(attrs.len()>2);assert!(attrs.iter().filter(|id|**id!=parent).all(|id|e.scene.forest.readonly(*id)));
 e.apply(Action::Undo).unwrap();assert!(e.scene.contains(id));assert_eq!(before.to_proto(),before_proto);assert!(e.scene.objects().iter().all(|r|!matches!(r.original,Some(Geometry::Element(_)))));e.apply(Action::Redo).unwrap();assert!(Rc::ptr_eq(&original(&e),&made));assert_eq!(made.to_proto(),made_proto);
 let saved=crate::scene_archive::save(&e.scene).unwrap();let mut restored=Scene::empty();crate::scene_document::open(&mut restored,&saved).unwrap();let elem=restored.objects().iter().find_map(|r|if let Some(Geometry::Element(v))=&r.original{Some(v)}else{None}).unwrap();assert_eq!(elem.to_proto(),made_proto);let parent=restored.objects().iter().find(|r|matches!(r.original,Some(Geometry::Element(_)))).unwrap().id;assert!(restored.with_descendants(&[parent]).len()>2);let child=restored.with_descendants(&[parent])[1];assert!(restored.forest.readonly(child));assert!(restored.place_owner(child,session_rust::Xform::identity()).is_err());
}
#[test]
fn element_picked_stale_source_rejects_creation_without_consumption(){
 let mut e=Editor::default();e.apply(Action::Close).unwrap();let id=line(&mut e);command(&mut e,"Element Beam").unwrap();e.apply(Action::ElementPick(Some(id))).unwrap();e.scene.place_line(id,session_rust::Xform::translation(1.,0.,0.)).unwrap();assert!(command(&mut e,"").is_err());assert!(e.scene.contains(id));assert!(e.element.is_some());assert!(e.scene.objects().is_empty());command(&mut e,"Escape").unwrap();assert!(e.element.is_none());
}
#[test]
fn element_attributes_toggle_follows_parent_and_preserves_original_payload(){
 let mut e=Editor::default();e.apply(Action::Close).unwrap();let id=line(&mut e);e.select_objects([id],false);command(&mut e,"Element Beam 100 200").unwrap();let source=original(&e);let before=source.to_proto();let parent=e.selected_objects()[0];let children=crate::element_adopt::attribute_ids(&e.scene);assert!(children.len()>1);assert!(children.iter().all(|id|!e.scene.visible(*id)));command(&mut e,"Element Attributes On").unwrap();assert!(children.iter().all(|id|e.scene.visible(*id)));command(&mut e,"Element Attributes Off").unwrap();assert!(children.iter().all(|id|!e.scene.visible(*id)));
 let old=children.iter().map(|id|e.scene.placement(*id).unwrap()).collect::<Vec<_>>();e.select_objects([parent],false);e.apply(Action::Translate([3.,4.,5.])).unwrap();for(id,old)in children.iter().zip(old){let matrix=e.scene.placement(*id).unwrap();assert_eq!(matrix.m[12],old.m[12]+3.);assert_eq!(matrix.m[13],old.m[13]+4.);assert_eq!(matrix.m[14],old.m[14]+5.);}assert_eq!(source.to_proto(),before);
}
#[test]
fn element_strict_arguments_longest_command_and_selection_order(){
 for bad in ["Element Beam 1 2 3","Element Plate -1","Element Support 1","Element Block 3","Element Beam Variable 1","Element Column NaN","Element Attributes What"]{assert!(crate::commands::parse(bad).is_err(),"{bad}");}
 let mut e=Editor::default();e.apply(Action::Close).unwrap();let mut ids=Vec::new();for z in [0.,250.,500.]{ids.push(e.scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(square(100.,z))).unwrap()).unwrap());}e.select_objects(ids.clone(),false);command(&mut e,"Element Beam Variable").unwrap();let source=original(&e);assert_eq!(source.element_type,wood::BeamVariable::TYPE);let beam=wood::BeamVariable::from_element(&source).unwrap();assert!((beam.axis.length()-500.).abs()<1e-9);assert!(ids.iter().all(|id|!e.scene.contains(*id)));
 let mut e=Editor::default();e.apply(Action::Close).unwrap();let id=line(&mut e);e.select_objects([id],false);command(&mut e,"Element Plate 40").unwrap_err();assert!(e.scene.contains(id));assert!(e.scene.objects().is_empty());
}
#[test]
fn element_bounding_box_measurement_and_delete_keep_parent_ownership(){
 let mut e=Editor::default();e.apply(Action::Close).unwrap();let id=line(&mut e);e.select_objects([id],false);command(&mut e,"Element Beam 100 200").unwrap();let source=original(&e);let before=source.to_proto();let owner=e.selected_objects()[0];command(&mut e,"Length BoundingBox").unwrap();assert_eq!(e.mark.as_ref().unwrap().label,"X 1000 · Y 100 · Z 200 mm");command(&mut e,"BoundingBox").unwrap();assert_eq!(source.to_proto(),before);e.apply(Action::Undo).unwrap();e.select_objects([owner],false);let all=e.scene.with_descendants(&[owner]);command(&mut e,"Delete").unwrap();assert!(all.iter().all(|id|!e.scene.contains(*id)));e.apply(Action::Undo).unwrap();assert!(all.iter().all(|id|e.scene.contains(*id)));assert!(Rc::ptr_eq(&original(&e),&source));
}

#[test]
fn attributes_and_interactions_have_independent_defaults_and_keep_originals() {
    use session_rust::element::ElementFeature;
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    assert!(!e.element_attributes && e.element_interactions);
    let mut source = session_rust::Element::new("host");
    source.set_geometry(session_rust::Mesh::create_box(100., 100., 100.));
    for kind in ["axis", "contact", "joint", "cut", "drill"] {
        source.add_feature(ElementFeature::new(kind, -1, vec![square(100., 0.)], kind));
    }
    let source = Rc::new(source);
    let _ = source.guid();
    let before = source.to_proto();
    let made = crate::element_input::Made { source: source.clone(), plane: None };
    crate::element_adopt::adopt(&mut e.scene, vec![made], &[]).unwrap();
    let attributes = crate::element_adopt::attribute_ids(&e.scene);
    let interactions = crate::element_adopt::interaction_ids(&e.scene);
    assert_eq!(attributes.len(), 1);
    assert_eq!(interactions.len(), 4);
    assert!(attributes.iter().all(|id| !e.scene.visible(*id)));
    assert!(interactions.iter().all(|id| e.scene.visible(*id)));
    command(&mut e, "Element Attributes On").unwrap();
    command(&mut e, "Element Interactions Off").unwrap();
    assert!(attributes.iter().all(|id| e.scene.visible(*id)));
    assert!(interactions.iter().all(|id| !e.scene.visible(*id)));
    command(&mut e, "Element Interactions").unwrap();
    assert!(interactions.iter().all(|id| e.scene.visible(*id)));
    use prost::Message;
    let mut after = source.to_proto();
    // Protobuf map entry order is not geometry identity; compare the decoded mesh fields exactly.
    let original_mesh = session_rust::proto::Mesh::decode(before.geometry_data.as_slice()).unwrap();
    let current_mesh = session_rust::proto::Mesh::decode(after.geometry_data.as_slice()).unwrap();
    assert_eq!(current_mesh, original_mesh);
    after.geometry_data = before.geometry_data.clone();
    assert_eq!(after, before);
    assert!(crate::commands::parse("Element Interactions Maybe").is_err());
    assert!(crate::commands::parse("Element Features On").is_err());
}
