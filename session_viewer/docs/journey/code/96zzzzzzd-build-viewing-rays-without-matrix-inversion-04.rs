use crate::camera::{Camera,Projection,Unit};
use crate::camera_reference_tests as current;
#[test]
fn ordinary_large_orthographic_rays_match_current_lines_in_both_units(){
 for unit in [Unit::Millimeters,Unit::Meters]{for projection in [Projection::Perspective,Projection::Orthographic]{
  let mut camera=Camera::default();camera.unit=unit;let scale=unit.to_meters();camera.distance=20./scale;camera.target=[2./scale,-3./scale,4./scale];camera.projection=projection;camera.aspect=1.6;camera.frame=[0.1,0.05,0.8,0.9];
  let mut reference=current::Camera::new();reference.unit=match unit{Unit::Millimeters=>current::Unit::Millimeters,Unit::Meters=>current::Unit::Meters};reference.distance=20.;reference.target=[2.,-3.,4.];reference.perspective=projection==Projection::Perspective;reference.frame=camera.frame;reference.update_position();
  for screen in [[0.,0.],[0.7,-0.4],[-1.3,1.1]]{
   let ray=camera.ray_unbounded(screen).unwrap();let pixel=((screen[0]as f64+1.)*400.,(1.-screen[1]as f64)*250.);
   let(origin,direction)=reference.ray(pixel,(800.,500.)).unwrap();assert!((&ray.origin-&origin).cross(&direction).magnitude()*scale<1e-10);assert!((&ray.direction-&direction).magnitude()<1e-12);
   let matrix=camera.view_projection();let near=matrix.transform_point(&ray.origin);let end=&ray.origin+&(&ray.direction*ray.max_distance);let far=matrix.transform_point(&end);
   for axis in 0..2{assert!((near[axis]-screen[axis]as f64).abs()<1e-9);assert!((far[axis]-screen[axis]as f64).abs()<1e-9);}assert!((near[2]-1.).abs()<1e-9);assert!(far[2].abs()<1e-9);
  }
 }}
}
#[test]
fn analytic_rays_keep_precise_direction_frame_and_finite_endpoint_guards(){
 for projection in [Projection::Perspective,Projection::Orthographic]{let mut local=Camera::default();local.projection=projection;local.distance=20000.;local.frame=[0.2,0.1,0.9,0.8];local.grid_plane=Some(crate::cplane::CPlane::Frame{origin:[0.;3],x:[1.,0.,0.],y:[0.,1.,0.]});let mut far=Camera::default();far.projection=local.projection;far.distance=local.distance;far.frame=local.frame;far.target=[1e12,-1e12,1e12];far.grid_plane=Some(crate::cplane::CPlane::Frame{origin:far.target,x:[1.,0.,0.],y:[0.,1.,0.]});
  for screen in [[0.,0.],[0.2,-0.1],[1.,-1.]]{let a=local.ray(screen).unwrap();let b=far.ray(screen).unwrap();for i in 0..3{assert!((b.origin[i]-far.target[i]-a.origin[i]).abs()<0.00025);assert_eq!(a.direction[i],b.direction[i]);}assert_eq!(a.max_distance,b.max_distance);assert!((a.direction.magnitude()-1.).abs()<1e-12);}
 }
 let mut camera=Camera::default();for invalid in [[f32::NAN,0.],[0.,f32::INFINITY]]{assert!(camera.ray_unbounded(invalid).is_none());}assert!(camera.ray([1.1,0.]).is_none());assert!(camera.ray_unbounded([1.1,0.]).is_some());for distance in [0.,-1.,f64::NAN,f64::INFINITY]{camera.distance=distance;assert!(camera.ray([0.,0.]).is_none());}
}
