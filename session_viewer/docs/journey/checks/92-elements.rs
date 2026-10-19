fn original_scene()->viewer_journey::scene::Scene{
    use viewer_journey::element_input::{Kind,Param};
    use std::rc::Rc;
    use session_rust::{Geometry,Line,Polyline,Point,Xform};
    let mut scene=viewer_journey::scene::Scene::empty();
    for y in [-250.,250.]{let sources=vec![(Geometry::Line(Rc::new(Line::new(-500.,y,300.,500.,y,300.))),Xform::identity())];let made=viewer_journey::element_input::build(Kind::Beam,&[Param{name:"Width",value:100.},Param{name:"Height",value:150.}],&sources).unwrap();viewer_journey::element_adopt::adopt(&mut scene,made,&[]).unwrap();}
    for x in [-400.,400.]{for y in [-250.,250.]{let sources=vec![(Geometry::Line(Rc::new(Line::new(x,y,-100.,x,y,225.))),Xform::identity())];let made=viewer_journey::element_input::build(Kind::Column,&[Param{name:"Width",value:100.},Param{name:"Depth",value:100.}],&sources).unwrap();viewer_journey::element_adopt::adopt(&mut scene,made,&[]).unwrap();}}
    let outline=Polyline::new([[-600.,-400.],[600.,-400.],[600.,400.],[-600.,400.],[-600.,-400.]].map(|p|Point::new(p[0],p[1],-150.)).to_vec());let made=viewer_journey::element_input::build(Kind::Plate,&[Param{name:"Thickness",value:50.}],&[(Geometry::Polyline(Rc::new(outline)),Xform::identity())]).unwrap();viewer_journey::element_adopt::adopt(&mut scene,made,&[]).unwrap();scene
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer{let renderer=Renderer::new(device,queue,format,&original_scene());renderer.set_outlines(true);renderer.set_ambient(true);renderer}
fn draw(renderer:&Renderer,view:&wgpu::TextureView){let mut camera=viewer_journey::camera::Camera::default();camera.isometric();camera.aspect=4./3.;camera.target=[0.,0.,100.];camera.distance=2000.;camera.radius=1000.;renderer.draw(view,&Default::default(),&camera.uniform());assert_eq!(renderer.ambient_error(),None);assert_eq!(renderer.global_outline_usage()[1],1);}
fn verify_pixels(pixels:&[u8]){
    let gray=pixels.chunks_exact(4).filter(|p|p[..3].iter().max().unwrap()-p[..3].iter().min().unwrap()<=2&&(80..245).contains(&p[0])).count();assert!(gray>10000,"original typed Element solids: {gray}");
    let black=pixels.chunks_exact(4).filter(|p|p[..3].iter().all(|v|*v<20)).count();assert!(black>800,"original Element black outlines: {black}");
    for(x,y)in[(20,20),(620,20)]{assert!(pixels[(y*640+x)*4..][..3].iter().all(|v|*v>250));}
}
