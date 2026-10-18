use crate::camera::{Camera,Projection,Unit};
use session_rust::Point;
#[test]
fn equivalent_millimetre_and_metre_scenes_share_screen_and_ray_geometry(){
 for unit in [Unit::Millimeters,Unit::Meters]{for projection in [Projection::Perspective,Projection::Orthographic]{let mut camera=Camera::default();camera.unit=unit;camera.projection=projection;assert!(camera.view_projection().inverse().is_some());assert!(camera.ray([0.2,-0.1]).is_some());}}
 for projection in [Projection::Perspective,Projection::Orthographic]{
  let mut mm=Camera::default();mm.projection=projection;mm.target=[2e6,-3e6,4e6];mm.distance=5000.;mm.radius=1000.;
  let mut m=Camera::default();m.projection=projection;m.unit=Unit::Meters;m.target=mm.target.map(|x|x*0.001);m.distance=5.;m.radius=1.;
  let a=mm.view_projection_anchored(mm.target);let b=m.view_projection_anchored(m.target);
  let pa=a.transform_point(&Point::new(500.,250.,100.));let pb=b.transform_point(&Point::new(0.5,0.25,0.1));for i in 0..3{assert!((pa[i]-pb[i]).abs()<1e-12);}
  let ra=mm.ray([0.2,-0.1]).unwrap();let rb=m.ray([0.2,-0.1]).unwrap();for i in 0..3{assert!((ra.origin[i]*0.001-rb.origin[i]).abs()<1e-6);assert!((ra.direction[i]-rb.direction[i]).abs()<1e-6);}assert!((ra.max_distance*0.001-rb.max_distance).abs()<1e-5);
 }
}
#[test]
fn anchored_camera_keeps_submillimetre_detail_at_a_distant_origin(){
 let mut camera=Camera::default();camera.target=[1e12,-1e12,1e12];camera.distance=1.;camera.radius=1.;
 for projection in [Projection::Perspective,Projection::Orthographic]{camera.projection=projection;let local=camera.view_projection_anchored(camera.target).to_f32();let matrix=session_rust::Xform::from_matrix(local.map(f64::from));let a=matrix.transform_point(&Point::new(0.,0.,0.));let b=matrix.transform_point(&Point::new(0.000125,0.,0.));assert!(b[0]-a[0]>0.0001);assert!((0.0..1.0).contains(&a[2]));}
 for projection in [Projection::Perspective,Projection::Orthographic]{camera.projection=projection;let mut local=Camera::default();local.projection=projection;local.distance=1.;local.radius=1.;for screen in [[0.,0.],[0.2,-0.1]]{let a=camera.ray(screen).unwrap();let b=local.ray(screen).unwrap();for i in 0..3{assert!((a.origin[i]-camera.target[i]-b.origin[i]).abs()<0.00025);assert!((a.direction[i]-b.direction[i]).abs()<1e-10);}assert!((a.max_distance-b.max_distance).abs()<1e-8);}}
 assert_eq!(camera.target,[1e12,-1e12,1e12]);
}

#[test]
fn cpu_screen_queries_keep_distant_pixels_and_original_clip_guards(){
 use crate::snap_math::Screen;let anchor=[1e12,-1e12,1e12];for projection in [Projection::Perspective,Projection::Orthographic]{let mut camera=Camera::default();camera.projection=projection;camera.distance=2.;let baseline=Screen::camera(&camera,[640.,480.]);camera.target=anchor;let far=Screen::camera(&camera,[640.,480.]);assert_eq!(baseline.matrix,far.matrix);assert!(baseline!=far);for point in [[0.25,0.125,-0.5],[-0.125,-0.25,0.125],[0.,0.,0.]]{let world=std::array::from_fn(|i|anchor[i]+point[i]);assert_eq!(far.point(world),baseline.point(point));assert_eq!(far.origin,anchor);}}
 let mut view=Screen{matrix:session_rust::Xform::identity().m,size:[200.,100.],origin:anchor};assert_eq!(view.point([anchor[0]+0.25,anchor[1]+0.125,anchor[2]+0.5]),Some([125.,43.75]));for point in [[anchor[0],anchor[1],anchor[2]-0.5],[anchor[0],anchor[1],anchor[2]+1.5],[f64::NAN,0.,0.]]{assert_eq!(view.point(point),None);}view.size[0]=0.;assert_eq!(view.point(anchor),None);view.size[0]=200.;view.origin[0]=f64::NAN;assert_eq!(view.point(anchor),None);view.origin=anchor;view.matrix[15]=0.;assert_eq!(view.point(anchor),None);
}
