use crate::{snap::{Settings, Setting, Kind}, snap_index::Index, snap_math::Screen, scene::Scene};
use std::rc::Rc;

fn screen() -> Screen { Screen { matrix: session_rust::Xform::identity().m, size: [200.,100.] } }
fn ray(p: [f64; 2]) -> crate::camera::Ray { crate::camera::Ray { origin:session_rust::Point::new(p[0],p[1],-1.),direction:session_rust::Vector::z_axis(),max_distance:2. } }
fn empty() -> Scene { let mut scene=Scene::demo(); scene.clear(); scene }
fn line(scene: &mut Scene, a: [f64; 3], b: [f64; 3]) -> crate::scene::ObjectId {
    scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(a[0],a[1],a[2],b[0],b[1],b[2]))).unwrap()).unwrap()
}

#[test]
fn current_grid_default_and_independent_switches_validate_before_mutation() {
    let mut settings=Settings::default(); assert!(settings.objects && settings.grid); assert_eq!(settings.step,1000.);
    assert_eq!(settings.modes,crate::snap::END|crate::snap::NEAR|crate::snap::MID|crate::snap::CENTER);
    settings.apply(Setting::Objects(Some(false))).unwrap(); assert!(settings.grid && !settings.objects);
    settings.apply(Setting::Grid(Some(false))).unwrap(); settings.apply(Setting::Spacing(50.)).unwrap();
    assert!(settings.grid && !settings.objects); assert_eq!(settings.step,50.); let before=settings;
    for value in [0.,-1.,f64::NAN,f64::INFINITY] { assert!(settings.apply(Setting::Spacing(value)).is_err()); assert_eq!(settings,before); }
    assert!(settings.apply(Setting::Mode(32)).is_err()); assert_eq!(settings,before);
    settings.apply(Setting::Mode(crate::snap::PERP)).unwrap(); assert_ne!(settings.modes & crate::snap::PERP,0);
}

#[test]
fn grid_rounding_preserves_height_and_never_creates_nonfinite_points() {
    assert_eq!(crate::snap_math::grid([149.,51.,33.3],[0.;3],[[1.,0.,0.],[0.,1.,0.]],100.).unwrap(),[100.,100.,33.3]);
    assert_eq!(crate::snap_math::grid([149.,51.,33.3],[0.;3],[[0.,1.,0.],[0.,0.,1.]],10.).unwrap(),[149.,50.,30.]);
    assert_eq!(crate::snap_math::grid([-150.,150.,0.],[0.;3],[[1.,0.,0.],[0.,1.,0.]],100.).unwrap(),[-200.,200.,0.]);
    assert!(crate::snap_math::grid([1.,2.,3.],[0.;3],[[1.,0.,0.],[0.,1.,0.]],0.).is_none());
    assert!(crate::snap_math::grid([f64::NAN,2.,3.],[0.;3],[[1.,0.,0.],[0.,1.,0.]],50.).is_none());
    assert_eq!(crate::snap_math::grid([1e12,2.,3.],[0.;3],[[1.,0.,0.],[0.,1.,0.]],1e-320).unwrap(),[1e12,2.,3.]);
}

#[test]
fn priority_wins_inside_twelve_css_pixels_then_distance_breaks_same_kind_ties() {
    let mut scene=empty(); let owner=line(&mut scene,[-0.5,0.,0.2],[0.5,0.,0.2]); let mut index=Index::new(&scene);
    let hit=index.query(screen(),&ray([0.49,0.]),[149.,50.],crate::snap::END|crate::snap::NEAR,None).unwrap();
    assert_eq!(hit.kind,Kind::End); assert_eq!(hit.point,[0.5,0.,0.2]); assert_eq!(hit.owner,Some(owner));
    let near=index.query(screen(),&ray([0.49,0.]),[149.,50.],crate::snap::NEAR,None).unwrap(); assert_eq!(near.kind,Kind::Near); assert!((near.point[0]-0.49).abs()<1e-12);
    let mid=index.query(screen(),&ray([0.,0.]),[100.,50.],crate::snap::MID,None).unwrap(); assert_eq!(mid.point,[0.,0.,0.2]);
    assert!(index.query(screen(),&ray([0.7,0.]),[170.,50.],crate::snap::END|crate::snap::NEAR,None).is_none());
}

#[test]
fn placed_original_endpoints_curve_domain_midpoints_and_hidden_sources_are_distinct() {
    let mut scene=empty(); let id=line(&mut scene,[-0.5,0.,0.2],[0.5,0.,0.2]);
    let shift=0.123456789012345; scene.place_line(id,session_rust::Xform::translation(shift,0.,0.)).unwrap();
    let p=[0.5+shift,0.,0.2]; let pixel=screen().point(p).unwrap(); let mut index=Index::new(&scene);
    assert_eq!(index.query(screen(),&ray([p[0],p[1]]),pixel,crate::snap::END,None).unwrap().point,p);
    let mut hidden=session_rust::Point::new(0.,0.8,0.2); hidden.is_visible=false; scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(hidden)).unwrap()).unwrap();
    let mut index=Index::new(&scene); assert!(index.query(screen(),&ray([0.,0.8]),[100.,10.],crate::snap::END,None).is_none());
    let mut curve=session_rust::NurbsCurve::create(false,2,&[session_rust::Point::new(-0.5,0.,0.2),session_rust::Point::new(0.,0.6,0.2),session_rust::Point::new(0.5,0.,0.2)]);
    curve.is_locked=true; scene.insert_path(crate::chain::PreparedChain::curve(Rc::new(curve)).unwrap()).unwrap();
    let mut index=Index::new(&scene); let mid=index.query(screen(),&ray([0.,0.3]),[100.,35.],crate::snap::MID,None).unwrap();
    assert_eq!(mid.kind,Kind::Mid); assert!((mid.point[1]-0.3).abs()<1e-12); assert!((mid.point[1]-0.6).abs()>0.1);
    assert!(scene.pick_id(mid.owner.unwrap().value()).is_none(),"Read-only geometry can supply a snap without becoming an editable selection");
}

#[test]
fn perpendicular_uses_the_previous_point_and_excludes_feet_outside_the_segment() {
    let mut scene=empty(); line(&mut scene,[-0.5,0.,0.2],[0.5,0.,0.2]); let mut index=Index::new(&scene);
    let hit=index.query(screen(),&ray([0.,0.01]),[100.,49.5],crate::snap::PERP|crate::snap::NEAR,Some([0.,0.5,0.2])).unwrap();
    assert_eq!(hit.kind,Kind::Perp); assert_eq!(hit.point,[0.,0.,0.2]);
    assert!(crate::snap_math::perpendicular([-0.5,0.,0.2],[0.5,0.,0.2],[1.,0.5,0.2]).is_none());
    assert!(crate::snap_math::perpendicular([0.;3],[0.;3],[1.;3]).is_none());
}

#[test]
fn screen_cells_keep_queries_local_and_rebuild_only_for_a_changed_view() {
    let mut scene=empty();
    for i in 0..2000 { scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(i as f64*2.,0.,0.2))).unwrap()).unwrap(); }
    let weak=Rc::downgrade(&scene.points()[0].prepared.source); let mut index=Index::new(&scene);
    index.query(screen(),&ray([0.,0.]),[100.,50.],crate::snap::END,None).unwrap(); assert_eq!(index.builds,1); assert!(index.visits<5);
    index.query(screen(),&ray([0.01,0.]),[101.,50.],crate::snap::END,None).unwrap(); assert_eq!(index.builds,1);
    let mut changed=screen(); changed.size=[300.,100.]; index.query(changed,&ray([0.,0.]),[150.,50.],crate::snap::END,None).unwrap(); assert_eq!(index.builds,2);
    scene.clear(); assert!(weak.upgrade().is_none(),"Derived snap coordinates do not retain original source owners");
    let mut bins=crate::snap_bins::Bins::default(); bins.insert(1,Some([[0.,0.],[128.,64.]])); bins.insert(2,Some([[-1e50,-1e50],[1e50,1e50]]));
    assert_eq!(bins.near([127.,63.],12.),vec![1,2]); assert_eq!(bins.near([1e6,1e6],12.),vec![2]);
}

#[test]
fn snapping_settings_preserve_a_draft_and_typed_coordinates_bypass_grid_rounding() {
    use crate::editor::{Action,Editor}; let mut editor=Editor::default(); editor.apply(Action::Close).unwrap();
    editor.apply(Action::BeginTool(crate::tool::Shape::Line)).unwrap(); editor.apply(Action::ToolPoint([0.123456789012345,0.25,0.])).unwrap();
    let first=editor.draft.as_ref().unwrap().points.clone(); editor.apply(Action::Snap(Setting::Objects(Some(false)))).unwrap();
    editor.apply(Action::ToolScreen { screen:[0.25,0.25],size:[400.,300.],commit:false }).unwrap();
    assert_eq!(editor.draft.as_ref().unwrap().cursor,Some([0.,0.,0.])); assert_eq!(editor.draft.as_ref().unwrap().snapped,Some(Kind::Grid));
    editor.apply(Action::Snap(Setting::Spacing(50.))).unwrap(); assert_eq!(editor.draft.as_ref().unwrap().points,first);
    editor.apply(Action::Snap(Setting::Grid(Some(false)))).unwrap();
    editor.apply(Action::ToolScreen { screen:[0.25,0.25],size:[400.,300.],commit:false }).unwrap();
    assert_ne!(editor.draft.as_ref().unwrap().cursor,Some([0.,0.,0.])); assert_eq!(editor.draft.as_ref().unwrap().snapped,None);
    editor.apply(Action::ToolPoint([0.987654321012345,0.5,0.])).unwrap(); assert_eq!(editor.scene.lines()[0].prepared.source.start()[0],first[0][0]);
    editor.apply(Action::Undo).unwrap(); assert!(editor.scene.lines().is_empty());
}

#[test]
fn original_transactions_invalidate_snap_positions_without_creating_snap_history() {
    use crate::editor::{Action,Editor}; let mut editor=Editor::default(); editor.apply(Action::Close).unwrap();
    editor.camera.distance=4.; editor.camera.aspect=2.; editor.apply(Action::AddLine).unwrap(); editor.apply(Action::SelectAll).unwrap();
    editor.apply(Action::BeginTool(crate::tool::Shape::Point)).unwrap(); editor.apply(Action::Snap(Setting::Grid(Some(false)))).unwrap(); editor.snapping.modes=crate::snap::END;
    let target=|editor:&Editor,p:[f64;3]| { let p=editor.camera.view_projection().transform_point(&session_rust::Point::new(p[0],p[1],p[2])); [p[0] as f32,p[1] as f32] };
    let old=target(&editor,[0.6,0.,0.]); assert_eq!(editor.snap_point(old,[400.,200.]).unwrap().point,[0.6,0.,0.]);
    editor.apply(Action::Translate([0.5,0.,0.])).unwrap(); let current=target(&editor,[1.1,0.,0.]);
    assert_eq!(editor.snap_point(current,[400.,200.]).unwrap().point,[1.1,0.,0.]); assert!(editor.snap_point(old,[400.,200.]).unwrap().owner.is_none());
    editor.apply(Action::CancelTool).unwrap(); editor.apply(Action::Undo).unwrap(); assert_eq!(editor.scene.lines()[0].endpoints()[1][0],0.6);
    for text in ["Grid Snap 0","Grid Snap NaN","Grid Snap -1","Grid Snap 1 2","Snap Maybe"] { assert!(crate::commands::parse(text).is_err(),"{text}"); }
    assert!(matches!(crate::commands::parse("GridSnap 50").unwrap(),crate::commands::Command::Editor(Action::Snap(Setting::Spacing(50.)))));
}

#[test]
fn a_closed_source_loop_offers_its_center_with_a_small_closure_tolerance() {
    let mut scene=empty(); let points=vec![session_rust::Point::new(-0.5,-0.5,0.2),session_rust::Point::new(0.5,-0.5,0.2),session_rust::Point::new(0.5,0.5,0.2),session_rust::Point::new(-0.5,0.5,0.2),session_rust::Point::new(-0.5+1e-10,-0.5,0.2)];
    scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(session_rust::Polyline::new(points))).unwrap()).unwrap();
    let hit=Index::new(&scene).query(screen(),&ray([0.,0.]),[100.,50.],crate::snap::CENTER,None).unwrap();
    assert_eq!(hit.kind,Kind::Center); assert_eq!(hit.point,[0.,0.,0.2]);
}

#[test]
fn a_tilted_grid_counts_from_its_origin_and_preserves_local_height() {
    let q=2.0_f64.sqrt().recip(); let origin=[10.,0.,0.]; let axes=[[0.,q,q],[-1.,0.,0.]];
    let world=|u:f64,v:f64,w:f64| [10.-v, q*(u-w),q*(u+w)];
    let point=crate::snap_math::grid(world(1490.,520.,7.),origin,axes,1000.).unwrap();
    let expected=world(1000.,1000.,7.);
    assert!((0..3).all(|i| (point[i]-expected[i]).abs()<1e-9));
    assert!(crate::snap_math::grid(world(1.,2.,3.),origin,[[1.,0.,0.],[1.,0.,0.]],10.).is_none());
}
