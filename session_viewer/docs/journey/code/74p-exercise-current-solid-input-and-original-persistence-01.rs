use crate::{editor::{Action,Editor},primitive_shape::{Answer,Kind},primitive_draft::Draft,cplane::CPlane};
use session_rust::Point;
fn editor()->Editor {let mut e=Editor::default();e.apply(Action::Close).unwrap();e}
fn command(e:&mut Editor,line:&str)->Result<(),String> {
    let action=if let Some(action)=e.tool_line(line){action.map_err(str::to_owned)?}else{let crate::commands::Command::Editor(action)=crate::commands::parse(line)? else{return Err("Editor command".into())};action};e.apply(action).map_err(|s|if s=="Shape input invalid" {e.tool_status.clone()}else{s.into()})?;Ok(())
}
#[test]
fn direct_maintained_solid_builders_create_original_geometry_as_one_transaction() {
    let mut e=editor();for (line,faces) in [("Box 0,0,0 2 3 4",6),("Pyramid 4,0,0 2 3",5),("Block With Hole 8,0,0 4 3 2 0.4",7),("Tetrahedron 12,0,0 2",4),("Octahedron 16,0,0 2",8),("Icosahedron 20,0,0 2",20),("Dodecahedron 24,0,0 2",12)] {
        let count=e.scene.objects().len();command(&mut e,line).unwrap();assert!(e.draft.is_none());let cad=e.scene.objects().last().unwrap().cad.as_ref().unwrap();assert_eq!(cad.source.face_count(),faces);command(&mut e,"Undo").unwrap();assert_eq!(e.scene.objects().len(),count);command(&mut e,"Redo").unwrap();assert_eq!(e.scene.objects().len(),count+1);
    }
    let sources=e.scene.objects().iter().map(|row|row.cad.as_ref().unwrap().source.to_proto()).collect::<Vec<_>>();let bytes=crate::scene_archive::save(&e.scene).unwrap();e.apply(Action::Close).unwrap();crate::scene_document::open(&mut e.scene,&bytes).unwrap();assert_eq!(e.scene.objects().iter().map(|row|row.cad.as_ref().unwrap().source.to_proto()).collect::<Vec<_>>(),sources);
}
#[test]
fn typed_defaults_and_brep_mesh_choices_preserve_the_current_answers() {
    let mut e=editor();command(&mut e,"Box").unwrap();command(&mut e,"0,0,0").unwrap();assert!(command(&mut e,"").is_err());command(&mut e,"2").unwrap();command(&mut e,"Mesh").unwrap();assert_eq!(e.draft.as_ref().unwrap().primitive.as_ref().unwrap().answers.len(),2);command(&mut e,"").unwrap();assert!(e.scene.objects().is_empty());command(&mut e,"").unwrap();assert_eq!(e.scene.objects().len(),1);assert!(e.scene.objects()[0].cad.is_none());
    let bounds=e.scene.bounds().unwrap();assert_eq!(bounds.min,[-1.,-1.,0.]);assert_eq!(bounds.max,[1.,1.,2.]);command(&mut e,"Undo").unwrap();assert!(e.scene.objects().is_empty());command(&mut e,"Redo").unwrap();assert_eq!(e.scene.objects().len(),1);
}
#[test]
fn clicked_corners_and_signed_heights_follow_the_current_fixed_plane() {
    let mut e=editor();command(&mut e,"CPlane XZ").unwrap();command(&mut e,"Box").unwrap();e.apply(Action::ToolPoint([1.,2.,3.])).unwrap();e.apply(Action::ToolPoint([2.,2.,5.])).unwrap();assert!(e.scene.objects().is_empty());command(&mut e,"-3").unwrap();
    let source=&e.scene.objects()[0].cad.as_ref().unwrap().source;let bounds=crate::primitive_shape::tests::bounds(&source.vertex_points());assert_eq!(bounds,[[0.,2.,1.],[2.,5.,5.]]);
}
#[test]
fn invalid_direct_and_batched_shape_input_preserves_draft_points_and_redo() {
    let mut e=editor();command(&mut e,"Box 0,0,0 2 3 4").unwrap();command(&mut e,"Undo").unwrap();command(&mut e,"Line").unwrap();command(&mut e,"w0,0,0").unwrap();
    assert!(command(&mut e,"Box 0,0,0 0 3 4").is_err());assert_eq!(e.draft.as_ref().unwrap().tool.name(),"Line");assert_eq!(e.draft.as_ref().unwrap().points,vec![[0.;3]]);
    command(&mut e,"Escape").unwrap();command(&mut e,"Box").unwrap();for text in ["0,0,0 2 3 4 extra","0,0,0 2 3 4 Mesh","0,0,0 NaN","0,0,0 2 0"] {assert!(command(&mut e,text).is_err(),"{text}");assert!(e.draft.as_ref().unwrap().primitive.as_ref().unwrap().answers.is_empty());assert!(e.scene.objects().is_empty());}
    command(&mut e,"Escape").unwrap();command(&mut e,"Redo").unwrap();assert_eq!(e.scene.objects().len(),1);
}
#[test]
fn height_clicks_use_the_normal_axis_and_refuse_an_axis_aligned_view() {
    let mut draft=Draft::new(&crate::primitive_box::SHAPE,CPlane::Xy,0).unwrap();draft.accept(Answer::Point(Point::new(0.,0.,0.))).unwrap();draft.accept(Answer::Number(2.)).unwrap();draft.accept(Answer::Number(3.)).unwrap();assert_eq!(draft.next().unwrap().kind,Kind::Height);
    let mut camera=crate::camera::Camera::default();assert!(draft.on_axis(&camera,[0.,0.]).is_none());camera.orbit(0.,std::f64::consts::FRAC_PI_4);let point=draft.on_axis(&camera,[0.,0.2]).unwrap();assert!(point[0].abs()<1e-12 && point[1].abs()<1e-12);assert!(point[2].is_finite());
}
#[test]
fn pending_solid_wires_and_cancel_never_create_saved_geometry() {
    let mut e=editor();command(&mut e,"Box 0,0,0 2 3").unwrap();let draft=e.draft.as_ref().unwrap();assert_eq!(draft.primitive.as_ref().unwrap().answers.len(),3);assert!(e.scene.objects().is_empty());assert_eq!(draft.outlines().len(),1);
    e.apply(Action::ToolHover(Some([0.,0.,4.]))).unwrap();assert_eq!(e.draft.as_ref().unwrap().outlines().len(),6);assert!(e.scene.objects().is_empty());command(&mut e,"Escape").unwrap();assert!(e.draft.is_none());assert!(e.scene.objects().is_empty());
}
