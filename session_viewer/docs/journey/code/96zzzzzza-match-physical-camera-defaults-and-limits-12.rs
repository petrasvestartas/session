use crate::{editor::{Action,Change,Editor},camera::{Camera,Projection},gesture::{Gesture,Motion}};

#[test]
fn cursor_zoom_holds_the_target_plane_point_in_both_projections_and_frames() {
    for projection in [Projection::Perspective,Projection::Orthographic] {
        let mut camera=crate::camera_defaults_tests::top_fixture();camera.projection=projection;camera.frame([0.,0.,0.7,0.8]);
        let at=[0.2,-0.1];let ray=camera.ray(at).unwrap();let forward=camera.orientation.rotate_vector(session_rust::Vector::y_axis());
        let centre=session_rust::Point::new(camera.target[0],camera.target[1],camera.target[2]);let t=(&centre-&ray.origin).dot(&forward)/ray.direction.dot(&forward);
        let point=session_rust::Point::new(ray.origin[0]+ray.direction[0]*t,ray.origin[1]+ray.direction[1]*t,ray.origin[2]+ray.direction[2]*t);
        let before=camera.view_projection().transform_point(&point);camera.zoom_at(1.4,at);let after=camera.view_projection().transform_point(&point);
        assert!((before[0]-after[0]).abs()<1e-7&&(before[1]-after[1]).abs()<1e-7);assert!((camera.distance-3./1.4_f32 as f64).abs()<1e-9);
        let saved=camera.view_projection().m;camera.zoom_at(2.,[f32::NAN,0.]);assert_eq!(camera.view_projection().m,saved);
    }
}

#[test]
fn slop_pan_and_free_canvas_are_css_based_without_document_history() {
    let mut input=Gesture::default();input.press_with(1,2,[10.,20.],true);assert_eq!(input.move_to(1,[13.,23.]),None);assert_eq!(input.move_to(1,[15.,24.]),Some(Motion::Pan([5.,4.])));
    let mut camera=Camera::default();let old=camera.target;camera.pan_screen([20.,-10.],[640.,480.]);assert_ne!(camera.target,old);camera.pan_screen([-20.,10.],[640.,480.]);assert!(camera.target.into_iter().zip(old).all(|(a,b)|(a-b).abs()<1e-12));
    assert!(camera.frame([0.,0.,0.7,0.8]));let target=session_rust::Point::new(camera.target[0],camera.target[1],camera.target[2]);let projected=camera.view_projection().transform_point(&target);assert!((projected[0]+0.3).abs()<1e-12&&(projected[1]-0.2).abs()<1e-12);
    let frame=camera.frame;assert!(!camera.frame([0.,0.,0.,1.]));assert_eq!(camera.frame,frame);
}

#[test]
fn select_all_and_remove_preserve_sources_and_undo_while_respecting_current_flags() {
    let mut e=Editor::default();let ids=e.scene.objects().iter().map(|r|r.id).collect::<Vec<_>>();let source=e.scene.objects()[0].geometry().unwrap().clone();
    std::rc::Rc::make_mut(&mut e.scene.view).lock(&[ids[1]],true);assert_eq!(e.apply(Action::SelectAll).unwrap(),Change::View);assert_eq!(e.selected_objects(),vec![ids[0]]);
    e.apply(Action::SelectRows {ids:vec![ids[0]],add:true,remove:true}).unwrap();assert!(e.selected_objects().is_empty());assert!(std::rc::Rc::ptr_eq(&source,e.scene.objects()[0].geometry().unwrap()));
    assert_eq!(crate::shortcuts::key("a",true,false,false,false,false,false),Some(crate::shortcuts::Shortcut::SelectAll));assert_eq!(crate::shortcuts::key("a",true,false,false,false,true,false),None);
}

fn start(e:&mut Editor,from:[f32;2],to:[f32;2]) {e.apply(Action::CanvasStart {from,to,size:[640.,480.],modified:false,pointer:1}).unwrap();}

#[test]
fn a_released_hit_commits_the_entire_move_once_and_shares_original_source() {
    let mut e=Editor::default();let id=e.scene.objects()[0].id;let original=e.scene.placement(id).unwrap().m;let source=e.scene.objects()[0].geometry().unwrap().clone();
    start(&mut e,[-0.3,0.],[0.1,0.]);assert_eq!(e.apply(Action::CanvasMove {screen:[0.2,0.],size:[640.,480.]}).unwrap(),Change::View);
    e.apply(Action::CanvasRelease {screen:[0.4,0.],size:[640.,480.],add:false,remove:false}).unwrap();let released=e.canvas_drag.as_ref().unwrap().to;
    e.apply(Action::CanvasMove {screen:[0.6,0.],size:[640.,480.]}).unwrap();assert_eq!(e.canvas_drag.as_ref().unwrap().to,released);
    assert_eq!(e.apply(Action::CanvasHit(Some(id))).unwrap(),Change::Scene);let moved=e.scene.placement(id).unwrap().m;assert_ne!(moved,original);assert!(e.canvas_drag.is_none());assert!(std::rc::Rc::ptr_eq(&source,e.scene.objects()[0].geometry().unwrap()));
    e.apply(Action::Undo).unwrap();assert_eq!(e.scene.placement(id).unwrap().m,original);e.apply(Action::Redo).unwrap();assert_eq!(e.scene.placement(id).unwrap().m,moved);
}

#[test]
fn a_released_miss_uses_release_modifiers_and_new_press_discards_the_old_drag() {
    let mut e=Editor::default();e.apply(Action::SelectAll).unwrap();let before=e.selected_objects();
    start(&mut e,[-1.,1.],[1.,-1.]);e.apply(Action::CanvasRelease {screen:[1.,-1.],size:[640.,480.],add:true,remove:true}).unwrap();
    assert_eq!(e.apply(Action::CanvasHit(None)).unwrap(),Change::View);assert!(e.selected_objects().is_empty());assert!(e.canvas_drag.is_none());assert!(!before.is_empty());
    start(&mut e,[-0.5,0.],[0.2,0.]);e.apply(Action::CanvasRelease {screen:[0.3,0.],size:[640.,480.],add:false,remove:false}).unwrap();start(&mut e,[0.,0.],[0.1,0.]);assert_eq!(e.canvas_drag.as_ref().unwrap().from,[0.,0.]);
    e.apply(Action::CanvasCancel).unwrap();let placements=e.scene.objects().iter().map(|r|r.model.m).collect::<Vec<_>>();e.apply(Action::CanvasHit(Some(e.scene.objects()[0].id))).unwrap();assert_eq!(e.scene.objects().iter().map(|r|r.model.m).collect::<Vec<_>>(),placements);
}

#[test]
fn touch_pinch_pan_wait_for_all_lifts_and_double_tap_fits() {
    let rect=[0.,0.,640.,480.];let mut touch=crate::touch::Touch::default();assert!(touch.press(1,[200.,200.],0.));assert!(touch.move_to(1,[203.,202.],rect).is_empty());assert!(matches!(touch.move_to(1,[210.,205.],rect).as_slice(),[Action::Orbit(..)]));
    assert!(touch.press(2,[300.,200.],20.));let actions=touch.move_to(2,[340.,220.],rect);assert!(matches!(actions.as_slice(),[Action::PanScreen {..},Action::ZoomAt {..}]));
    touch.release(2,[340.,220.],30.,rect);assert!(touch.move_to(1,[230.,205.],rect).is_empty());assert!(touch.release(1,[230.,205.],40.,rect).is_empty());assert_eq!(touch.count(),0);
    touch.press(3,[200.,200.],100.);assert!(matches!(touch.release(3,[200.,200.],120.,rect).as_slice(),[Action::Pick(_)]));touch.press(4,[210.,200.],150.);assert!(matches!(touch.release(4,[210.,200.],180.,rect).as_slice(),[Action::Fit]));
    touch.press(5,[200.,200.],200.);touch.cancel();assert!(touch.release(5,[200.,200.],220.,rect).is_empty());assert!(!touch.press(6,[f64::NAN,0.],230.));
}

#[test]
fn a_failed_cold_source_drag_releases_its_preview_and_keeps_redo() {
    let mut e=Editor::default();let location=std::rc::Rc::new(crate::reload_url::ReloadUrl::new("test:drag".into()));
    e.apply(Action::ReplaceAt(crate::specimen::bytes(),location)).unwrap();let id=e.scene.objects()[0].id;
    e.apply(Action::AddBox).unwrap();e.apply(Action::Undo).unwrap();e.apply(Action::UnloadSources).unwrap();let before=e.scene.placement(id).unwrap().m;
    start(&mut e,[0.,0.],[0.2,0.]);assert!(e.apply(Action::CanvasHit(Some(id))).unwrap_err().contains("Reload"));assert!(e.canvas_drag.is_none());assert_eq!(e.scene.placement(id).unwrap().m,before);
    let count=e.scene.objects().len();e.apply(Action::Redo).unwrap();assert_eq!(e.scene.objects().len(),count+1);
}

#[test]
fn panel_free_fit_contains_every_corner_in_perspective_and_orthographic() {
    let mut bounds=crate::bounds::Bounds::point([-3.,-2.,-1.]);bounds.include([3.,2.,1.]);
    for projection in [Projection::Perspective,Projection::Orthographic] {
        let mut camera=Camera::default();camera.projection=projection;camera.isometric();camera.frame([0.,0.,0.6,0.75]);camera.fit(&bounds);
        for x in [bounds.min[0],bounds.max[0]] {for y in [bounds.min[1],bounds.max[1]] {for z in [bounds.min[2],bounds.max[2]] {
            let p=camera.view_projection().transform_point(&session_rust::Point::new(x,y,z));assert!(p[0]>=-1.&&p[0]<=0.2&&p[1]>=-0.5&&p[1]<=1.);
        }}}
    }
}

#[test]
fn only_an_unmoved_right_click_repeats_and_cancel_clears_the_request() {
    let mut gesture=Gesture::default();gesture.press(1,2,[0.,0.]);assert_eq!(gesture.release(1,[2.,2.]),None);assert!(gesture.take_repeat());assert!(!gesture.take_repeat());
    gesture.press(1,2,[0.,0.]);gesture.move_to(1,[10.,0.]);gesture.release(1,[0.,0.]);assert!(!gesture.take_repeat());gesture.press(1,2,[0.,0.]);gesture.cancel();assert!(!gesture.take_repeat());
}
