use crate::{editor::{Editor,Action,Change},gizmo::{Gizmo,Handle,Axis},pick_sources::{Part,Target}};
use session_rust::{Point,Xform};
use std::rc::Rc;
const SIZE:[f64;2]=[640.,480.];

fn editor()->Editor {let mut e=Editor::default();e.apply(Action::Close).unwrap();e.apply(Action::AddBox).unwrap();e.apply(Action::AddLine).unwrap();e.apply(Action::AddPoint).unwrap();e.apply(Action::SelectAll).unwrap();e.apply(Action::Fit).unwrap();e}
fn screen(e:&Editor,p:Point)->[f32;2] {let p=e.camera.view_projection().transform_point(&p);[p[0] as f32,p[1] as f32]}
fn handle(e:&Editor,handle:Handle)->[f32;2] {
    let p=e.gizmo_origin().unwrap();let scale=crate::widget_layout::world_per_pixel(&e.camera.view_projection(),p,SIZE[1]).unwrap();
    screen(e,Gizmo::new(Point::new(p[0],p[1],p[2])).handle_point(handle,scale))
}
fn press(e:&mut Editor,handle_:Handle)->[f32;2] {let p=handle(e,handle_);assert_eq!(e.gizmo_hit(p,SIZE),Some(handle_));e.apply(Action::GizmoPress {pointer:7,screen:p,size:SIZE,modified:false}).unwrap();assert_eq!(e.gizmo_edit.as_ref().unwrap().active(),Some(handle_));p}
fn click(e:&mut Editor,h:Handle) {let p=press(e,h);e.apply(Action::GizmoRelease {pointer:7,screen:p,size:SIZE,shift:false}).unwrap();assert_eq!(e.gizmo_edit.as_ref().unwrap().gizmo.typing,Some(h));}
fn models(e:&Editor)->Vec<[f64;16]> {e.scene.objects().iter().map(|r|r.id).chain(e.scene.lines().iter().map(|r|r.id)).chain(e.scene.points().iter().map(|r|r.id)).map(|id|e.scene.placement(id).unwrap().m).collect()}

#[test]
fn handle_drag_shares_original_sources_keeps_save_unchanged_and_commits_once() {
    let mut e=editor();let before=models(&e);let source=Rc::clone(e.scene.objects()[0].geometry().unwrap());let display=Rc::clone(&e.scene.objects()[0].mesh);
    let p=press(&mut e,Handle::Translate(Axis::X));let moved=[p[0]+0.1,p[1]];
    e.apply(Action::GizmoMove {pointer:8,screen:moved,size:SIZE,shift:false}).unwrap();assert_eq!(e.gizmo_edit.as_ref().unwrap().preview.matrix(),&Xform::identity().m);
    assert_eq!(e.apply(Action::GizmoMove {pointer:7,screen:moved,size:SIZE,shift:false}).unwrap(),Change::Preview);
    assert_eq!(models(&e),before);assert!(Rc::ptr_eq(&source,e.display_scene().objects()[0].geometry().unwrap()));assert!(Rc::ptr_eq(&display,&e.display_scene().objects()[0].mesh));
    assert!(e.display_scene().objects()[0].model.m[12]!=before[0][12]);
    e.apply(Action::GizmoRelease {pointer:8,screen:moved,size:SIZE,shift:false}).unwrap();assert!(e.gizmo_edit.is_some());
    e.apply(Action::GizmoRelease {pointer:7,screen:moved,size:SIZE,shift:false}).unwrap();let after=models(&e);assert_ne!(after,before);assert!(e.gizmo_edit.is_none());
    e.apply(Action::Undo).unwrap();assert_eq!(models(&e),before);e.apply(Action::Redo).unwrap();assert_eq!(models(&e),after);
    e.apply(Action::Undo).unwrap();e.apply(Action::Undo).unwrap();assert!(e.scene.points().is_empty(),"The whole handle drag owns exactly one history entry");
}

#[test]
fn numeric_handles_keep_precise_values_and_invalid_answers_preserve_selection_and_redo() {
    let mut e=editor();let before=models(&e);click(&mut e,Handle::Translate(Axis::X));
    for text in ["NaN","1e999","nonsense"] {assert!(e.apply(Action::GizmoNumber(text.into())).is_err());assert!(e.gizmo_edit.is_some());assert_eq!(models(&e),before);}
    let value=0.123456789012345;let action=e.tool_line(&value.to_string()).unwrap().unwrap();e.apply(action).unwrap();let after=models(&e);
    assert!((after[0][12]-before[0][12]-value).abs()<1e-15);e.apply(Action::Undo).unwrap();
    click(&mut e,Handle::ScaleUniform);assert!(e.apply(Action::GizmoNumber("0".into())).is_err());assert!(e.gizmo_edit.is_some());e.apply(Action::GizmoCancel).unwrap();
    click(&mut e,Handle::ScaleUniform);e.apply(Action::GizmoNumber("1".into())).unwrap();e.apply(Action::Redo).unwrap();assert_eq!(models(&e),after);
}

#[test]
fn modified_presses_and_point_drafts_keep_handle_capture_out_of_selection_and_drawing() {
    let mut e=editor();let p=handle(&e,Handle::Translate(Axis::X));e.apply(Action::GizmoPress {pointer:7,screen:p,size:SIZE,modified:true}).unwrap();assert!(e.gizmo_edit.is_none());
    e.apply(Action::BeginTool(crate::tool::Shape::Line)).unwrap();assert!(e.gizmo_origin().is_none());e.apply(Action::GizmoPress {pointer:7,screen:p,size:SIZE,modified:false}).unwrap();assert!(e.gizmo_edit.is_none());
    e.apply(Action::CancelTool).unwrap();e.apply(Action::BeginTool(crate::tool::Shape::Move)).unwrap();e.apply(Action::ToolPoint([0.;3])).unwrap();
    e.apply(Action::GizmoPress {pointer:7,screen:p,size:SIZE,modified:false}).unwrap();assert!(e.edit_preview.is_some());assert_eq!(e.draft.as_ref().unwrap().points,vec![[0.;3]]);
    e.apply(Action::CancelTool).unwrap();press(&mut e,Handle::Translate(Axis::X));e.apply(Action::Close).unwrap();assert!(e.gizmo_edit.is_none());assert!(e.scene.bounds().is_none());
}

#[test]
fn stale_documents_and_invalid_drag_answers_cannot_publish_a_handle_preview() {
    let mut e=editor();let p=press(&mut e,Handle::Translate(Axis::X));let moved=[p[0]+0.08,p[1]];
    e.apply(Action::GizmoMove {pointer:7,screen:moved,size:SIZE,shift:false}).unwrap();let preview=*e.gizmo_edit.as_ref().unwrap().preview.matrix();let before=models(&e);
    assert!(e.apply(Action::GizmoMove {pointer:7,screen:[f32::NAN,0.],size:SIZE,shift:false}).is_err());assert_eq!(*e.gizmo_edit.as_ref().unwrap().preview.matrix(),preview);assert_eq!(models(&e),before);
    let id=e.scene.objects()[0].id;e.scene.translate(id,[0.2,0.,0.]).unwrap();let changed=models(&e);
    assert!(e.apply(Action::GizmoRelease {pointer:7,screen:moved,size:SIZE,shift:false}).is_err());assert_eq!(models(&e),changed);assert!(e.gizmo_edit.is_some());e.apply(Action::GizmoCancel).unwrap();
}

#[test]
fn face_and_edge_previews_use_original_keys_and_world_to_local_placement() {
    let mut e=editor();let id=e.scene.objects()[0].id;let mut model=Xform::scale_xyz(2.,3.,4.);model.m[4]=0.7;model.m[12]=12.3456789012345;e.scene.place(id,model.clone()).unwrap();
    let source=Rc::clone(e.scene.objects()[0].geometry().unwrap());let face=source.faces()[0];let target=Target {owner:id,part:Part::Face(face)};
    let keys=crate::part_preview::keys(&source,target.part).unwrap();let mut p=crate::part_preview::Preview::new(&e.scene,target).unwrap();p.set(Xform::translation(0.123456789012345,0.25,-0.5)).unwrap();
    assert!(Rc::ptr_eq(&source,e.scene.objects()[0].geometry().unwrap()));let edited=p.scene.objects()[0].geometry().unwrap();assert_eq!(edited.guid(),source.guid());assert_eq!(edited.face,source.face);assert_eq!(edited.face_holes,source.face_holes);
    for key in source.vertices() {let a=source.vertex_point(key).unwrap();let b=edited.vertex_point(key).unwrap();if keys.contains(&key) {
        let a=model.transform_point(&a);let b=model.transform_point(&b);for i in 0..3 {assert!((b[i]-a[i]-[0.123456789012345,0.25,-0.5][i]).abs()<1e-12);}
    } else {for i in 0..3 {assert_eq!(a[i],b[i]);}}}
    let mut history=crate::history::History::default();assert!(p.commit(&mut e.scene,&mut history).unwrap());assert!(history.undo(&mut e.scene));assert!(Rc::ptr_eq(&source,e.scene.objects()[0].geometry().unwrap()));assert!(history.redo(&mut e.scene));
    let (a,b)=source.edges()[0];assert_eq!(crate::part_preview::keys(&source,Part::MeshEdge(a,b)).unwrap(),vec![a,b]);
}

#[test]
fn face_hole_keys_deduplicate_and_curve_control_edits_preserve_weights_and_knots() {
    let mut source=session_rust::Mesh::new();for key in 0..8 {source.add_vertex(Point::new(key as f64,0.,0.),Some(key));}
    source.add_face(vec![0,1,2,3],Some(37));source.set_face_holes(37,vec![vec![4,5,6,7,4]]);assert_eq!(crate::part_preview::keys(&source,Part::Face(37)).unwrap(),(0..8).collect::<Vec<_>>());
    let mut e=Editor::default();e.apply(Action::Close).unwrap();let mut curve=session_rust::NurbsCurve::create(true,2,&[Point::new(-0.6,-0.4,0.),Point::new(0.,0.6,0.),Point::new(0.6,-0.4,0.)]);
    assert!(curve.set_cv_4d(1,0.,1.2,0.,2.));let prepared=crate::chain::PreparedChain::curve(Rc::new(curve)).unwrap();e.scene.insert_path(prepared).unwrap();e.apply(Action::Controls(true)).unwrap();let id=e.scene.paths()[0].id;
    let crate::chain::Source::Curve(source,_)=e.scene.paths()[0].prepared.source.clone() else {panic!("Curve");};let target=Target {owner:id,part:Part::Control(1)};
    let mut p=crate::part_preview::Preview::new(&e.scene,target).unwrap();p.set(Xform::translation(0.123456789012345,0.25,0.)).unwrap();
    let crate::chain::Source::Curve(edited,_)=&p.scene.paths()[0].prepared.source else {panic!("Curve");};assert_eq!(edited.guid(),source.guid());assert_eq!(edited.m_nurbsknot,source.m_nurbsknot);assert_eq!(edited.m_is_rat,source.m_is_rat);assert!(edited.m_is_rat);for index in 0..source.m_cv_count {let weight=index*source.m_cv_stride+source.m_dim;assert_eq!(edited.m_cv[weight],source.m_cv[weight]);}
    for index in 0..source.m_cv_count {let a=source.get_cv(index).unwrap();let b=edited.get_cv(index).unwrap();for i in 0..3 {assert!((b[i]-a[i]-if index==1 {[0.123456789012345,0.25,0.][i]} else {0.}).abs()<1e-14);}}
    let mut history=crate::history::History::default();assert!(p.commit(&mut e.scene,&mut history).unwrap());assert!(history.undo(&mut e.scene));let crate::chain::Source::Curve(restored,_)=&e.scene.paths()[0].prepared.source else {panic!("Curve");};assert!(Rc::ptr_eq(restored,&source));
}

#[test]
fn changed_part_source_is_refused_and_cancelled_preview_releases_derived_geometry() {
    let mut e=editor();let id=e.scene.objects()[0].id;let target=Target {owner:id,part:Part::Face(e.scene.objects()[0].geometry().unwrap().faces()[0])};
    let mut p=crate::part_preview::Preview::new(&e.scene,target).unwrap();p.set(Xform::translation(0.1,0.,0.)).unwrap();let derived=Rc::downgrade(p.scene.objects()[0].geometry().unwrap());
    e.scene.deform_part(target,&Xform::translation(0.2,0.,0.)).unwrap();assert!(p.commit(&mut e.scene,&mut crate::history::History::default()).is_err());drop(p);assert!(derived.upgrade().is_none());
}

#[test]
fn part_selection_places_its_handle_at_the_average_original_world_points() {
    let mut e=editor();let id=e.scene.objects()[0].id;let face=e.scene.objects()[0].geometry().unwrap().faces()[0];let target=Target {owner:id,part:Part::Face(face)};
    let model=e.scene.placement(id).unwrap().m;e.apply(Action::PickedTarget(Some(target))).unwrap();assert_eq!(e.gizmo_origin(),crate::part_preview::origin(&e.scene,target));click(&mut e,Handle::Translate(Axis::X));e.apply(Action::GizmoNumber("0.25".into())).unwrap();
    assert_eq!(e.scene.placement(id).unwrap().m,model);e.apply(Action::Undo).unwrap();assert!(e.gizmo_edit.is_none());
}

#[test]
fn release_uses_current_shift_state_for_quarter_turn_commit() {
    let mut e=editor();let origin=e.gizmo_origin().unwrap();let scale=crate::widget_layout::world_per_pixel(&e.camera.view_projection(),origin,SIZE[1]).unwrap();
    let start=press(&mut e,Handle::Rotate(Axis::Z));let angle=(-75f64).to_radians();let p=screen(&e,Point::new(origin[0]+crate::gizmo::ARM*scale*angle.cos(),origin[1]+crate::gizmo::ARM*scale*angle.sin(),origin[2]));
    e.apply(Action::GizmoMove {pointer:7,screen:p,size:SIZE,shift:false}).unwrap();let free=*e.gizmo_edit.as_ref().unwrap().preview.matrix();assert!((free[0]-0.5).abs()<1e-5);
    e.apply(Action::GizmoRelease {pointer:7,screen:p,size:SIZE,shift:true}).unwrap();let first=e.scene.objects()[0].model.clone();assert!(first.m[0].abs()<1e-12 && (first.m[1]-1.).abs()<1e-12);
    e.apply(Action::Undo).unwrap();assert!(e.gizmo_edit.is_none());assert_ne!(start,p);
}

#[test]
fn memory_counts_part_preview_sources_and_releases_them_on_cancel() {
    let mut e=editor();let before=crate::memory::cpu(e.presentation_scenes(),std::iter::empty());let id=e.scene.objects()[0].id;let target=Target {owner:id,part:Part::Face(e.scene.objects()[0].geometry().unwrap().faces()[0])};
    e.apply(Action::PickedTarget(Some(target))).unwrap();let start=press(&mut e,Handle::Translate(Axis::X));e.apply(Action::GizmoMove {pointer:7,screen:[start[0]+0.1,start[1]],size:SIZE,shift:false}).unwrap();
    let during=crate::memory::cpu(e.presentation_scenes(),std::iter::empty());assert_eq!(during[1],before[1]+1);assert_eq!(during[3],before[3]+1);assert_eq!(during[5],before[5]+2);
    e.apply(Action::GizmoCancel).unwrap();assert_eq!(crate::memory::cpu(e.presentation_scenes(),std::iter::empty()),before);
}

#[test]
fn idle_handle_hover_changes_only_highlight_and_releases_on_commands_and_drafts() {
    let mut e=editor();let before=models(&e);let p=handle(&e,Handle::Translate(Axis::X));
    e.hover_gizmo(Some((p,SIZE)));assert_eq!(e.gizmo_active(),0.);assert!(e.gizmo_edit.is_none());assert_eq!(models(&e),before);
    e.hover_gizmo(None);assert_eq!(e.gizmo_active(),-1.);
    e.hover_gizmo(Some((p,SIZE)));e.apply(Action::Zoom(1.)).unwrap();assert_eq!(e.gizmo_active(),-1.);assert_eq!(models(&e),before);
    e.apply(Action::BeginTool(crate::tool::Shape::Line)).unwrap();e.hover_gizmo(Some((p,SIZE)));assert_eq!(e.gizmo_active(),-1.);assert!(e.gizmo_edit.is_none());
    e.apply(Action::CancelTool).unwrap();e.apply(Action::Undo).unwrap();assert!(e.scene.points().is_empty(),"Hover created no history entry");
}
