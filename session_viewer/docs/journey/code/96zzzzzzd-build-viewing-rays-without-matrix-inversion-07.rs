#[test]
fn units_projections_and_distant_origins_keep_near_one_far_zero_and_forward_rays(){
 for unit in [Unit::Millimeters,Unit::Meters]{for projection in [Projection::Perspective,Projection::Orthographic]{for target in [[0.;3],[1e9,-1e9,1e9]]{
  let mut camera=Camera::default();camera.unit=unit;camera.projection=projection;camera.grid=None;camera.target=target;camera.distance=4000.;camera.radius=20000.;camera.grid_plane=Some(crate::cplane::CPlane::Frame{origin:target,x:[1.,0.,0.],y:[0.,1.,0.]});
  let direction=camera.orientation.rotate_vector(Vector::y_axis());
  let reach=camera.radius;let range=camera.distance+2.*reach;
  let (near,far)=match projection{Projection::Perspective=>(camera.distance*NEAR_FRACTION,(camera.distance*10.).max(range)),Projection::Orthographic=>(-range,range)};
