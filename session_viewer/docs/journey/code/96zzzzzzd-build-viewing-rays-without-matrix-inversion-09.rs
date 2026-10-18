use session_rust::Point;
#[test]
fn equivalent_millimetre_and_metre_scenes_share_screen_and_ray_geometry(){
 for unit in [Unit::Millimeters,Unit::Meters]{for projection in [Projection::Perspective,Projection::Orthographic]{let mut camera=Camera::default();fixed_plane(&mut camera,[0.;3]);camera.unit=unit;camera.projection=projection;assert!(camera.view_projection().inverse().is_some());assert!(camera.ray([0.2,-0.1]).is_some());}}
 for projection in [Projection::Perspective,Projection::Orthographic]{
  let mut mm=Camera::default();mm.projection=projection;mm.target=[2e6,-3e6,4e6];mm.distance=5000.;mm.radius=20e6;
  let mut m=Camera::default();m.projection=projection;m.unit=Unit::Meters;m.target=mm.target.map(|x|x*0.001);m.distance=5.;m.radius=20000.;
  let a=mm.view_projection_anchored(mm.target);let b=m.view_projection_anchored(m.target);
  let pa=a.transform_point(&Point::new(500.,250.,100.));let pb=b.transform_point(&Point::new(0.5,0.25,0.1));for i in 0..3{assert!((pa[i]-pb[i]).abs()<1e-12);}
  let ra=mm.ray([0.2,-0.1]).unwrap();let rb=m.ray([0.2,-0.1]).unwrap();for i in 0..3{assert!((ra.origin[i]*0.001-rb.origin[i]).abs()<1e-6);assert!((ra.direction[i]-rb.direction[i]).abs()<1e-6);}assert!((ra.max_distance*0.001-rb.max_distance).abs()<1e-5);
