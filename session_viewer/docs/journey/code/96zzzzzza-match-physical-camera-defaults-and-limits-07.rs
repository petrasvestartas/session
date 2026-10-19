use crate::{camera::{Camera,Unit},camera_reference_tests as current};
use session_rust::{AABB,Point,Vector};
use sha2::{Digest,Sha256};
#[test]
fn current_default_pose_uses_metres_and_the_shared_isometric_owner(){
 let manifest:serde_json::Value=serde_json::from_str(include_str!("camera_reference_fixture.json")).unwrap();
 assert_eq!(format!("{:x}",Sha256::digest(include_bytes!("camera_reference_tests.rs"))),manifest["fixtureSha256"].as_str().unwrap());
 let actual=Camera::default();let expected=current::Camera::new();let scale=actual.unit.to_meters();
 assert_eq!(actual.distance*scale,expected.distance);assert_eq!(actual.radius*scale,expected.scene_extent);
 assert_eq!(actual.target.map(|v|v*scale),expected.target);assert_eq!(actual.unit.to_meters(),expected.unit.to_meters());
 for [x,y,z] in [[1.,0.,0.],[0.,1.,0.],[0.,0.,1.]]{let a=actual.orientation.rotate_vector(Vector::new(x,y,z));let b=expected.orientation.rotate_vector(Vector::new(x,y,z));for i in 0..3{assert!((a[i]-b[i]).abs()<1e-12);}}
}
#[test]
fn physical_zoom_bounds_and_tiny_fit_match_the_maintained_camera(){
 for unit in [Unit::Millimeters,Unit::Meters]{
  let mut actual=Camera::default();actual.unit=unit;let scale=unit.to_meters();let mut expected=current::Camera::new();expected.unit=match unit{Unit::Millimeters=>current::Unit::Millimeters,Unit::Meters=>current::Unit::Meters};actual.distance=expected.distance/scale;
  actual.zoom(1e-30);for _ in 0..1000{expected.zoom(-10.);}assert!((actual.distance*scale-expected.distance).abs()<=expected.distance*1e-14);
  actual.zoom(1e30);for _ in 0..1000{expected.zoom(10.);}assert!((actual.distance*scale-expected.distance).abs()<=expected.distance*1e-14);
  let bounds=crate::bounds::Bounds{min:[-1e-12;3],max:[1e-12;3]};actual.fit(&bounds);
  expected.fit(&AABB::from_points(&[Point::new(-1e-12,-1e-12,-1e-12),Point::new(1e-12,1e-12,1e-12)],0.),actual.aspect);
  assert!((actual.distance*scale-expected.distance).abs()<1e-18);
 }
}

// Explicit three-scene-unit Top pose for pre-existing unit-sized fixtures.
pub(crate) fn top_fixture()->Camera {
 let mut camera=Camera::default();camera.distance=3.;camera.radius=1.;
 camera.orientation=session_rust::Quaternion::from_axis_angle(Vector::x_axis(),-std::f64::consts::FRAC_PI_2);camera
}
pub(crate) fn editor_fixture()->crate::editor::Editor {
 let mut editor=crate::editor::Editor::default();editor.camera=top_fixture();editor
}
