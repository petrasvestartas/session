use crate::{scene::Scene,cap_gpu_tests::{renderer,pixels_at,wait},camera::Projection};
use session_rust::{Point,Polyline};
fn project(matrix:[f32;16],p:[f32;3])->[f32;2]{let c:[f32;4]=std::array::from_fn(|row|p.iter().enumerate().fold(matrix[12+row],|s,(i,v)|s+matrix[i*4+row]*v));[c[0]/c[3],c[1]/c[3]]}
#[test]
fn shared_color_and_request_frames_preserve_actual_original_ink(){
    let mut scene=Scene::empty();scene.insert(crate::prepared::PreparedMesh::triangle([[-0.8,-0.7,0.],[0.8,-0.7,0.],[0.,0.8,0.]],[0.2,0.4,0.7]).unwrap()).unwrap();
    let mut path=Polyline::new(vec![Point::new(-0.7,-0.3,0.25),Point::new(0.,0.1,0.25),Point::new(0.7,0.3,0.25)]);path.width=3.;let _=path.guid();let source=path.to_proto();scene.insert_path(crate::chain::PreparedChain::polyline(std::rc::Rc::new(path)).unwrap()).unwrap();
    let mut point=Point::new(0.55,-0.45,0.3);point.width=12.;let owner=scene.insert_point(crate::marker::PreparedPoint::new(std::rc::Rc::new(point)).unwrap()).unwrap();
    let mut r=renderer(&scene);let uploads=r.stats()[0];let path_uploads=r.path_usage();let mut c=crate::camera_defaults_tests::top_fixture();c.aspect=4./3.;
    for projection in [Projection::Orthographic,Projection::Perspective]{c.projection=projection;for density in [1.,2.]{r.set_density(density);let matrix=c.uniform();let before=pixels_at(&r,matrix);assert!(before.chunks_exact(4).filter(|p|p[..3]!=[255;3]).count()>2000);
        let target=project(matrix,[0.55,-0.45,0.3]);r.request_pick(target,matrix).unwrap();
        let mut later=crate::camera_defaults_tests::top_fixture();later.aspect=c.aspect;later.projection=c.projection;later.pan(0.1,0.);let _=pixels_at(&r,later.uniform());wait(&r);
        assert_eq!(r.take_pick(matrix).unwrap().unwrap().unwrap()[0],owner.value(),"The request retains its original transform/viewport while the later color frame changes");
        assert_eq!(pixels_at(&r,matrix),before,"Color frame restores exact original surface and ink pixels");
    }}
    assert_eq!(r.stats()[0],uploads);assert_eq!(r.path_usage(),path_uploads);assert_eq!(match &scene.paths()[0].prepared.source{crate::chain::Source::Polyline(p)=>p.to_proto(),_=>panic!()},source);
}
