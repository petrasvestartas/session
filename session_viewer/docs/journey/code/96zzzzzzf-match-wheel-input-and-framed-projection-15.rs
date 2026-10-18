use crate::{camera::{Camera,Projection,Unit},camera_reference_tests as current,bounds::Bounds,editor::{Action,Editor}};
use session_rust::{AABB,Point};
fn reference(camera:&Camera)->current::Camera {
    let scale=camera.unit.to_meters();let mut out=current::Camera::new();out.unit=match camera.unit{Unit::Millimeters=>current::Unit::Millimeters,Unit::Meters=>current::Unit::Meters};out.target=camera.target.map(|v|v*scale);out.distance=camera.distance*scale;out.scene_extent=camera.radius*scale;out.orientation=camera.orientation.duplicate();out.frame=camera.frame;out.perspective=camera.projection==Projection::Perspective;out.grid=camera.grid_plane.map(|p|{let origin=p.origin();std::array::from_fn(|i|origin[i])});out.update_position();out
}
fn close(a:f64,b:f64){assert!((a-b).abs()<=a.abs().max(b.abs()).max(1.)*1e-10,"{a} != {b}");}
#[test]
fn physical_wheel_modes_match_current_distance_in_both_units(){
    let cases=[(100.,0,1.,-1f32),(100.,0,2.,-2.),(100.,0,3.,-3.),(1.,1,3.,-1.),(-0.125,1,2.,0.125),(1e9,0,1.,-1e7),(-1e9,0,3.,3e7),(0.125,0,2.,-0.0025)];
    for unit in [Unit::Millimeters,Unit::Meters]{for (delta,mode,dpr,amount) in cases {
        let mut actual=Camera::default();actual.unit=unit;actual.distance=3./unit.to_meters();let mut expected=reference(&actual);
        let Some(Action::Zoom(factor))=crate::navigation::wheel(delta,mode,dpr) else{panic!("Valid current wheel domain");};actual.zoom(factor);expected.zoom(amount);close(actual.distance*unit.to_meters(),expected.distance);
    }}
    for mode in [0,1,2,99]{for delta in [f64::NAN,f64::INFINITY,-f64::INFINITY,f64::MAX,-f64::MAX]{assert!(crate::navigation::wheel(delta,mode,2.).is_none());}}
    for mode in [2,99]{assert!(crate::navigation::wheel(100.,mode,2.).is_none());}
    assert!(crate::navigation::wheel(100.,0,0.).is_none());
}
#[test]
fn zoom_at_matches_current_cursor_anchor_and_preserves_originals(){
    for unit in [Unit::Millimeters,Unit::Meters]{for projection in [Projection::Perspective,Projection::Orthographic]{for dpr in [1.,2.,3.]{
        let mut editor=Editor::default();editor.camera.unit=unit;editor.camera.projection=projection;editor.camera.distance=3./unit.to_meters();editor.camera.aspect=1.5;editor.camera.frame=[0.1,0.05,0.9,0.85];
        let originals=editor.scene.objects().iter().map(|r|(r.id,r.geometry().unwrap().to_proto(),r.model.m)).collect::<Vec<_>>();let mut expected=reference(&editor.camera);
        let Some(Action::Zoom(factor))=crate::navigation::wheel(100.,0,dpr)else{panic!()};editor.apply(Action::ZoomAt{factor,screen:[0.25,-0.125]}).unwrap();expected.zoom_at(-(dpr as f32),(600.,360.),(960.,640.));
        close(editor.camera.distance*unit.to_meters(),expected.distance);for i in 0..3{close(editor.camera.target[i]*unit.to_meters(),expected.target[i]);}
        assert_eq!(editor.scene.objects().iter().map(|r|(r.id,r.geometry().unwrap().to_proto(),r.model.m)).collect::<Vec<_>>(),originals);
    }}}
}
#[test]
fn framed_projection_matches_current_clipped_corners_and_mode_guards(){
    for unit in [Unit::Millimeters,Unit::Meters]{for aspect in [0.4,1.5,3.]{for frame in [[0.,0.,1.,1.],[0.15,0.1,0.8,0.95]]{for target in [[0.;3],[10.,-20.,30.]]{
        let scale=unit.to_meters();let mut actual=Camera::default();actual.unit=unit;actual.distance=3./scale;actual.target=target.map(|v|v/scale);actual.aspect=aspect;actual.frame=frame;actual.projection=Projection::Orthographic;
        let bounds=Bounds{min:std::array::from_fn(|i|(target[i]+[-20.,-10.,-5.][i])/scale),max:std::array::from_fn(|i|(target[i]+[30.,15.,12.][i])/scale)};
        let points=bounds.corners().map(|p|Point::new(p[0],p[1],p[2]));let aabb=AABB::from_points(&points,0.);let mut expected=reference(&actual);
        expected.toggle_projection_framed(&aabb,aspect);actual.set_projection_framed(Projection::Perspective,Some(&bounds));expected.grow_extent(&aabb);actual.grow_extent(&bounds);
        close(actual.distance*scale,expected.distance);for i in 0..3{close(actual.target[i]*scale,expected.target[i]);}close(actual.radius*scale,expected.scene_extent);
        let anchor=Point::new(actual.target[0],actual.target[1],actual.target[2]);let a=actual.view_projection_anchored(actual.target);let b=expected.view_proj_anchored(aspect,&anchor);for i in 0..16{close(a.m[i]/a.m[15],b.m[i]/b.m[15]);}
        let before=(actual.target,actual.distance,actual.radius,actual.frame);actual.set_projection_framed(Projection::Perspective,Some(&Bounds::point([1e12;3])));assert_eq!((actual.target,actual.distance,actual.radius,actual.frame),before);
        actual.set_projection_framed(Projection::Orthographic,Some(&bounds));assert_eq!(actual.projection,Projection::Orthographic);assert_eq!((actual.target,actual.distance,actual.radius,actual.frame),before);
    }}}}
}
