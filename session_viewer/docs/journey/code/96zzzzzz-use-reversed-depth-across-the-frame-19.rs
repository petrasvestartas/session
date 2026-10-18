use crate::camera::{Camera,Projection,Unit,NEAR_FRACTION};
use session_rust::{Point,Vector};
#[test]
fn units_projections_and_distant_origins_keep_near_one_far_zero_and_forward_rays(){
 for unit in [Unit::Millimeters,Unit::Meters]{for projection in [Projection::Perspective,Projection::Orthographic]{for target in [[0.;3],[1e9,-1e9,1e9]]{
  let mut camera=Camera::default();camera.unit=unit;camera.projection=projection;camera.grid=None;camera.target=target;camera.distance=4000.;camera.radius=1000.;
  let direction=camera.orientation.rotate_vector(Vector::y_axis());
  let reach=camera.radius;let range=camera.distance+2.*reach;
  let (near,far)=match projection{Projection::Perspective=>(camera.distance*NEAR_FRACTION,(camera.distance*10.).max(range)),Projection::Orthographic=>(-range,range)};
  let point=|gap:f64|{let delta=gap-camera.distance;Point::new(direction[0]*delta,direction[1]*delta,direction[2]*delta)};
  let matrix=camera.view_projection_anchored(target);
  assert!((matrix.transform_point(&point(near))[2]-1.).abs()<1e-9);
  assert!(matrix.transform_point(&point(far))[2].abs()<1e-9);
  let a=matrix.transform_point(&point(camera.distance))[2];let b=matrix.transform_point(&point(camera.distance+100.))[2];assert!(a>b&&b>0.&&a<1.);
  let ray=camera.ray([0.,0.]).unwrap();assert!(ray.direction.dot(&direction)>0.999999);assert!((ray.max_distance-(far-near)).abs()<1e-4);
  let expected=point(near);for i in 0..3{assert!((ray.origin[i]-target[i]-expected[i]).abs()<1e-6);}
 }}}
}
