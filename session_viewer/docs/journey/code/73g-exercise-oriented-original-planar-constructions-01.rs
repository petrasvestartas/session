use crate::{cplane::CPlane,editor::{Action,Editor},tool::{Draft,Shape}};
use session_rust::Point;
fn editor()->Editor {let mut e=Editor::default();e.apply(Action::Close).unwrap();e}
fn command(e:&mut Editor,text:&str)->Result<(),String> {
    let action=if let Some(action)=e.tool_line(text){action.map_err(str::to_owned)?}else{let crate::commands::Command::Editor(action)=crate::commands::parse(text)? else{return Err("Editor command".into())};action};e.apply(action).map_err(str::to_owned)?;Ok(())
}
fn tilted()->CPlane {CPlane::from_3_points(&Point::new(4.,5.,6.),&Point::new(5.,5.,7.),&Point::new(4.,6.,6.)).unwrap()}
#[test]
fn a_rectangle_uses_plane_axes_and_retains_closed_original_points() {
    let mut e=editor();command(&mut e,"CPlane XZ").unwrap();command(&mut e,"Polyline Rectangle 1,2 @3,4").unwrap();
    assert_eq!(e.scene.paths()[0].prepared.source.coordinates(),vec![[1.,0.,2.],[4.,0.,2.],[4.,0.,6.],[1.,0.,6.],[1.,0.,2.]]);
    command(&mut e,"Undo").unwrap();assert!(e.scene.paths().is_empty());command(&mut e,"Redo").unwrap();assert_eq!(e.scene.paths().len(),1);
}
#[test]
fn polygon_sides_are_bounded_and_one_corner_lands_on_the_second_point() {
    let mut e=editor();command(&mut e,"CPlane XY").unwrap();command(&mut e,"Polyline Polygon").unwrap();command(&mut e,"Sides 5").unwrap();command(&mut e,"w0,0,0").unwrap();
    for value in ["Sides 2","Sides 4096","Sides -1","Sides NaN","Sides 4 extra"] {assert!(command(&mut e,value).is_err());assert_eq!(e.draft.as_ref().unwrap().tool.sides(),Some(5));}
    command(&mut e,"w0,2,0").unwrap();let points=e.scene.paths()[0].prepared.source.coordinates();assert_eq!(points.len(),6);assert_eq!(points[0],points[5]);
    assert!((points[0][0]).abs()<1e-12 && (points[0][1]-2.).abs()<1e-12);for p in points {assert!((p[0].hypot(p[1])-2.).abs()<1e-12);}
}
#[test]
fn tilted_constructions_preserve_world_prefixes_and_original_archive_columns() {
    let mut e=editor();let plane=tilted();e.apply(Action::ConstructionPlane(Some(plane))).unwrap();command(&mut e,"Polyline Rectangle").unwrap();command(&mut e,"0,0").unwrap();command(&mut e,"1,2").unwrap();
    let points=e.scene.paths()[0].prepared.source.coordinates();for p in &points {assert!(plane.local(&Point::new(p[0],p[1],p[2]))[2].abs()<1e-12);}
    assert_eq!(points[0],[4.,5.,6.]);let bytes=crate::scene_archive::save(&e.scene).unwrap();e.apply(Action::Close).unwrap();crate::scene_document::open(&mut e.scene,&bytes).unwrap();assert_eq!(e.scene.paths()[0].prepared.source.coordinates(),points);
}
#[test]
fn invalid_construction_keeps_accepted_points_and_existing_redo() {
    let mut e=editor();command(&mut e,"Polyline Rectangle w0,0 w1,2").unwrap();command(&mut e,"Undo").unwrap();command(&mut e,"Polyline Rectangle").unwrap();command(&mut e,"w0,0").unwrap();
    assert!(command(&mut e,"w1,0").is_err());assert_eq!(e.draft.as_ref().unwrap().points,vec![[0.;3]]);command(&mut e,"Escape").unwrap();command(&mut e,"Redo").unwrap();assert_eq!(e.scene.paths().len(),1);
    command(&mut e,"Polyline Polygon w0,0 w0,0").unwrap_err();assert!(e.draft.is_none());assert_eq!(e.scene.paths().len(),1);
}
#[test]
fn planar_preview_is_temporary_and_constructs_the_complete_boundary() {
    let mut draft=Draft::new(Shape::Rectangle);draft.submit([0.;3]).unwrap();draft.hover(Some([1.,2.,0.])).unwrap();assert_eq!(draft.outline().len(),5);assert_eq!(draft.points.len(),1);
    draft.hover(Some([1.,0.,0.])).unwrap();assert!(draft.outline().is_empty());assert_eq!(draft.points.len(),1);
    let mut polygon=Draft::new(Shape::Polygon {sides:5});polygon.submit([0.;3]).unwrap();polygon.hover(Some([1.,0.,0.])).unwrap();assert_eq!(polygon.outline().len(),6);assert_eq!(polygon.points.len(),1);
}
#[test]
fn polygon_maximum_leaves_space_for_the_closing_original_point() {
    let points=crate::planar::points("polygon",CPlane::Xy,4095,&[[0.;3],[1.,0.,0.]]).unwrap();assert_eq!(points.len(),4096);assert_eq!(points[0],points[4095]);assert!(crate::planar::points("polygon",CPlane::Xy,4096,&[[0.;3],[1.,0.,0.]]).is_err());
}
