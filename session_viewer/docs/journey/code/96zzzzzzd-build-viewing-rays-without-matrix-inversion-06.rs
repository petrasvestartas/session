use crate::camera::{Camera,Projection,Unit};
use crate::camera_reference_tests as current;
use crate::cplane::CPlane;
use session_rust::{Point};
fn fixed(center:[f64;3])->CPlane{CPlane::Frame{origin:center,x:[0.,1.,0.],y:[0.,0.,1.]}}
#[test]
fn ground_and_fixed_plane_depth_match_current_in_both_units_even_when_hidden(){
 for unit in [Unit::Millimeters,Unit::Meters]{for projection in [Projection::Perspective,Projection::Orthographic]{for physical_target in [[0.;3],[10.,-2.,3.],[1e6,-2e6,3e6]]{for extent in [0.,0.2,20000.]{for center in [None,Some([7.,-11.,13.])]{
  let scale=unit.to_meters();let mut camera=Camera::default();camera.unit=unit;camera.target=physical_target.map(|v|v/scale);camera.distance=3./scale;camera.radius=extent/scale;camera.projection=projection;camera.aspect=1.6;camera.frame=[0.1,0.05,0.8,0.9];camera.grid_plane=center.map(|c|fixed(c.map(|v|v/scale)));
  let mut reference=current::Camera::new();reference.unit=match unit{Unit::Millimeters=>current::Unit::Millimeters,Unit::Meters=>current::Unit::Meters};reference.target=physical_target;reference.distance=3.;reference.scene_extent=extent;reference.grid=center.map(|c|c.map(|v|v/scale));reference.perspective=projection==Projection::Perspective;reference.frame=camera.frame;reference.update_position();
  let anchor=camera.target;let expected=reference.view_proj_anchored(camera.aspect,&Point::new(anchor[0],anchor[1],anchor[2]));let actual=camera.view_projection_anchored(anchor);
  for p in [[0.;3],[0.2,-0.1,0.05],[-0.3,0.2,-0.1]]{let point=Point::new(p[0]/scale,p[1]/scale,p[2]/scale);let a=actual.transform_point(&point);let b=expected.transform_point(&point);for i in 0..3{assert!((a[i]-b[i]).abs()<1e-8,"Current camera mismatch in {unit:?} {projection:?}: {a:?} {b:?}");}}
  for settings in [None,Some(crate::grid::Settings::default()),Some(crate::grid::Settings{spacing:0.25,half_cells:2})]{camera.grid=settings;assert_eq!(camera.view_projection_anchored(anchor).m,actual.m,"Showing or hiding the grid cannot alter camera depth");}
 }}}}}
}
#[test]
fn translating_the_fixed_plane_with_the_scene_preserves_camera_and_ray_ranges(){
 for unit in [Unit::Millimeters,Unit::Meters]{for projection in [Projection::Perspective,Projection::Orthographic]{
  let mut camera=Camera::default();camera.unit=unit;camera.projection=projection;camera.distance=3./unit.to_meters();camera.target=[0.125,-0.25,0.5];camera.grid_plane=Some(fixed([0.;3]));let matrix=camera.view_projection_anchored(camera.target).m;let ray=camera.ray([0.2,-0.1]).unwrap();let shift=[1e9,-1e9,1e9];camera.target=std::array::from_fn(|i|camera.target[i]+shift[i]);camera.grid_plane=Some(fixed(shift));let far=camera.ray([0.2,-0.1]).unwrap();
  let after=camera.view_projection_anchored(camera.target);for i in 0..16{assert!((after.m[i]-matrix[i]).abs()<=matrix[i].abs()*1e-7+1e-9);}
  for i in 0..3{assert!((far.origin[i]-shift[i]-ray.origin[i]).abs()<1e-5);assert!((far.direction[i]-ray.direction[i]).abs()<1e-12);}assert!((far.max_distance-ray.max_distance).abs()<1e-5);
  let mut reference=current::Camera::new();reference.unit=match unit{Unit::Millimeters=>current::Unit::Millimeters,Unit::Meters=>current::Unit::Meters};reference.distance=3.;reference.target=camera.target.map(|v|v*unit.to_meters());reference.grid=Some(shift);reference.perspective=projection==Projection::Perspective;reference.update_position();let a=after.transform_point(&Point::new(0.,0.,0.));let b=reference.view_proj_anchored(camera.aspect,&reference.origin()).transform_point(&Point::new(0.,0.,0.));for i in 0..3{assert!((a[i]-b[i]).abs()<1e-8);}
 }
 }
}
