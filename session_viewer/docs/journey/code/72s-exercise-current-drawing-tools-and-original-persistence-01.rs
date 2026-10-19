use crate::{editor::{Action,Change,Editor},cplane::CPlane,tool::Shape};
use session_rust::Point;

fn editor()->Editor {let mut e=Editor::default();e.apply(Action::Close).unwrap();e}
fn command(e:&mut Editor,text:&str)->Result<Change,String> {
    let action=if let Some(action)=e.tool_line(text) {action.map_err(str::to_owned)?}else{
        match crate::commands::parse(text)? {crate::commands::Command::Editor(action)=>action,_=>return Err("Expected editor command".into())}
    };e.apply(action).map_err(str::to_owned)
}
fn tilted()->CPlane {CPlane::from_3_points(&Point::new(4.,5.,6.),&Point::new(5.,5.,7.),&Point::new(4.,6.,6.)).unwrap()}
fn near(a:[f64;3],b:[f64;3]) {for i in 0..3 {assert!((a[i]-b[i]).abs()<1e-12,"{a:?} {b:?}");}}

#[test]
fn finished_world_prefixed_drafts_do_not_apply_the_fixed_plane_twice() {
    let mut e=editor();let plane=tilted();e.apply(Action::ConstructionPlane(Some(plane))).unwrap();
    command(&mut e,"Line").unwrap();command(&mut e,"1,0,0").unwrap();
    let a=plane.world([1.,0.,0.]);let b=[9.123456789012345,8.,7.];
    let draft=e.draft.as_ref().unwrap();assert!(draft.command(&[[a[0],a[1],a[2]],b]).starts_with("Line w"));
    e.apply(Action::ToolPoint(b)).unwrap();let source=&e.scene.lines()[0].prepared.source;
    near([source.start()[0],source.start()[1],source.start()[2]],[a[0],a[1],a[2]]);
    near([source.end()[0],source.end()[1],source.end()[2]],b);
    let bytes=crate::scene_archive::save(&e.scene).unwrap();e.apply(Action::Undo).unwrap();assert!(e.scene.lines().is_empty());e.apply(Action::Redo).unwrap();
    e.apply(Action::Close).unwrap();crate::scene_document::open(&mut e.scene,&bytes).unwrap();
    assert_eq!(e.scene.lines()[0].prepared.source.end()[0],b[0]);
}

#[test]
fn direct_and_interactive_coordinates_share_plane_relative_polar_and_world_rules() {
    let mut e=editor();command(&mut e,"CPlane XZ").unwrap();command(&mut e,"Line 1,2 @3,4").unwrap();
    let line=&e.scene.lines()[0].prepared.source;near([line.start()[0],line.start()[1],line.start()[2]],[1.,0.,2.]);near([line.end()[0],line.end()[1],line.end()[2]],[4.,0.,6.]);
    command(&mut e,"Line").unwrap();command(&mut e,"w1,2,3").unwrap();command(&mut e,"5<90").unwrap();
    let line=&e.scene.lines()[1].prepared.source;near([line.end()[0],line.end()[1],line.end()[2]],[1.,2.,8.]);
    command(&mut e,"CPlane View").unwrap();command(&mut e,"Point w1000000.001,0,0").unwrap();assert_eq!(e.scene.points()[0].prepared.source[0],1000000.001);
    assert!(e.construction_plane.is_none());
}

#[test]
fn typed_batches_validate_before_changing_a_draft_or_history() {
    let mut e=editor();command(&mut e,"Line").unwrap();
    for text in ["0,0,0 NaN,1,0","0,0,0 1,0,0 2,0,0","@1,0","5"] {assert!(command(&mut e,text).is_err(),"{text}");assert!(e.draft.as_ref().unwrap().points.is_empty());}
    command(&mut e,"0,0,0").unwrap();assert!(command(&mut e,"0,0,0").is_err());assert_eq!(e.draft.as_ref().unwrap().points,vec![[0.;3]]);
    e.apply(Action::ToolHover(Some([0.,4.,0.]))).unwrap();command(&mut e,"2").unwrap();
    assert_eq!(e.scene.lines()[0].prepared.source.end()[1],2.);command(&mut e,"Undo").unwrap();assert!(e.scene.lines().is_empty());
    command(&mut e,"Line NaN,0 1,0").unwrap_err();assert!(e.draft.is_none());command(&mut e,"Redo").unwrap();assert_eq!(e.scene.lines().len(),1);
}

#[test]
fn an_open_polyline_commits_once_on_enter_and_preserves_redo_when_cancelled() {
    let mut e=editor();command(&mut e,"Polyline").unwrap();command(&mut e,"0,0").unwrap();
    assert!(command(&mut e,"").is_err());assert!(e.scene.paths().is_empty());
    command(&mut e,"@1,0 @0,1").unwrap();assert!(e.scene.paths().is_empty());
    for x in 0..30 {e.apply(Action::ToolHover(Some([x as f64/30.,1.,0.]))).unwrap();}
    command(&mut e,"").unwrap();assert_eq!(e.scene.paths()[0].prepared.source.coordinates(),vec![[0.,0.,0.],[1.,0.,0.],[1.,1.,0.]]);
    command(&mut e,"Undo").unwrap();assert!(e.scene.paths().is_empty());command(&mut e,"Polyline").unwrap();command(&mut e,"0,0 1,1").unwrap();command(&mut e,"Escape").unwrap();command(&mut e,"Redo").unwrap();assert_eq!(e.scene.paths().len(),1);
}

#[test]
fn polyline_limits_and_direct_commands_leave_original_geometry_atomic() {
    let mut e=editor();command(&mut e,"Polyline w0,0,0 w1,0,0 w1,1,0").unwrap();assert_eq!(e.scene.paths().len(),1);
    command(&mut e,"Polyline").unwrap();let points:Vec<_>=(0..crate::tool::MAX_POINTS).map(|i|[i as f64,0.,0.]).collect();
    e.draft.as_mut().unwrap().submit_many(&points).unwrap();assert!(e.apply(Action::ToolPoint([4096.,0.,0.])).is_err());assert_eq!(e.draft.as_ref().unwrap().points.len(),4096);
    command(&mut e,"Escape").unwrap();command(&mut e,"Undo").unwrap();assert!(e.scene.paths().is_empty());
}

#[test]
fn construction_plane_chooser_and_three_points_do_not_create_geometry_or_undo_entries() {
    let mut e=editor();command(&mut e,"Point w0,0,0").unwrap();command(&mut e,"CPlane").unwrap();assert!(e.plane_menu);
    let buttons=crate::drawing_input::buttons();assert_eq!(buttons.len(),6);command(&mut e,"3 Point").unwrap();
    command(&mut e,"w4,5,6").unwrap();command(&mut e,"w5,5,7").unwrap();assert!(command(&mut e,"w6,5,8").is_err());
    command(&mut e,"w4,6,6").unwrap();assert_eq!(e.construction_plane,Some(tilted()));assert!(e.draft.is_none());assert_eq!(e.scene.points().len(),1);
    command(&mut e,"Undo").unwrap();assert!(e.scene.points().is_empty());command(&mut e,"Redo").unwrap();assert_eq!(e.scene.points().len(),1);
}

#[test]
fn a_plane_can_change_mid_draft_without_reinterpreting_accepted_world_points() {
    let mut e=editor();command(&mut e,"Line").unwrap();command(&mut e,"w2,3,4").unwrap();command(&mut e,"3 Point").unwrap();
    command(&mut e,"w4,5,6").unwrap();command(&mut e,"w5,5,7").unwrap();command(&mut e,"w4,6,6").unwrap();assert_eq!(e.draft.as_ref().unwrap().points,vec![[2.,3.,4.]]);
    command(&mut e,"1,0,0").unwrap();let expected=tilted().world([1.,0.,0.]);let line=&e.scene.lines()[0].prepared.source;
    near([line.start()[0],line.start()[1],line.start()[2]],[2.,3.,4.]);near([line.end()[0],line.end()[1],line.end()[2]],[expected[0],expected[1],expected[2]]);
}

#[test]
fn plane_clicks_grid_and_editor_coordinate_context_use_current_owners() {
    let mut a=editor();a.apply(Action::ConstructionPlane(Some(tilted()))).unwrap();
    let mut b=editor();command(&mut b,"Point 1,2,3").unwrap();assert_eq!(b.scene.points()[0].prepared.source[0],1.);
    command(&mut a,"Point 1,0,0").unwrap();let expected=tilted().world([1.,0.,0.]);assert_eq!(a.scene.points()[0].prepared.source[0],expected[0]);
    a.apply(Action::ConstructionPlane(Some(CPlane::Xy))).unwrap();a.apply(Action::BeginTool(Shape::Point)).unwrap();
    let hit=a.snap_point([0.,0.],[640.,480.]).unwrap();assert!(hit.point[2].abs()<1e-12);
    a.apply(Action::Close).unwrap();assert!(a.draft.is_none());
}

#[test]
fn closing_a_polyline_is_one_transaction_and_the_last_point_is_original() {
    let mut e=editor();e.apply(Action::ConstructionPlane(Some(tilted()))).unwrap();command(&mut e,"Polyline").unwrap();command(&mut e,"w0,0 w1,0").unwrap();assert!(command(&mut e,"Close").is_err());
    command(&mut e,"w1,1").unwrap();command(&mut e,"Close").unwrap();let points=e.scene.paths()[0].prepared.source.coordinates();assert_eq!(points.len(),4);assert_eq!(points[0],points[3]);
    command(&mut e,"Undo").unwrap();assert!(e.scene.paths().is_empty());command(&mut e,"Redo").unwrap();assert_eq!(e.scene.paths()[0].prepared.source.coordinates(),points);
}

#[test]
fn edge_on_planes_and_view_depth_keep_a_finite_click_point() {
    let camera=crate::camera::Camera::default();let point=crate::drawing_input::on_plane(&camera,[0.,0.],CPlane::Xz).unwrap();assert_eq!(point[1],0.);assert!(point.iter().all(|v|v.is_finite()));
    let point=crate::drawing_input::on_at(&camera,[0.,0.],CPlane::Xy,Some([0.,0.,0.5])).unwrap();assert!((point[2]-0.5).abs()<1e-12);
}
