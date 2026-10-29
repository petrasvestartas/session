fn original_scene()->viewer_journey::scene::Scene{
    use std::rc::Rc;
    let mut scene=viewer_journey::scene::Scene::empty();
    let mut source=session_rust::BRep::create_block_with_hole(1.2,1.2,0.6,0.2);source.surfacecolor=session_rust::Color::new(0.1,0.5,0.8,1.);
    let owner=scene.insert_cad(Rc::new(source)).unwrap();scene.place_owner(owner,session_rust::Xform::translation(0.,0.,-0.5)).unwrap();scene
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer{let renderer={let math_source=original_scene();let mut camera=viewer_journey::camera::Camera::default();camera.isometric();camera.aspect=4./3.;camera.target=[0.,0.,-0.5];camera.distance=3.;
    let mut renderer=Renderer::new(device,queue,format,&math_source);
    let mut math_quality=viewer_journey::math_view::Presentation::default();
    math_quality.drain_matrix(&mut renderer,&math_source,&camera.uniform(),[WIDTH,HEIGHT],None).expect("Native mathematical display must meet the requested physical-pixel quality before capture");
    let math_scene=math_quality.scene(&math_source);
    renderer};renderer.set_ambient(true);renderer}
fn draw(renderer:&Renderer,view:&wgpu::TextureView){let mut camera=viewer_journey::camera::Camera::default();camera.isometric();camera.aspect=4./3.;camera.target=[0.,0.,-0.5];camera.distance=3.;renderer.draw(view,&Default::default(),&camera.uniform());assert_eq!(renderer.ambient_error(),None);assert_eq!(&renderer.ambient_usage()[1..4],&[1,1,1]);}
fn verify_pixels(pixels:&[u8]){
    let blue=pixels.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(20)&&p[2]>p[1].saturating_add(10)&&p[0]<180).count();assert!(blue>8000,"original blue geometry: {blue}");
    let shadow=pixels.chunks_exact(4).filter(|p|p[..3].iter().max().unwrap()-p[..3].iter().min().unwrap()<=2&&(80..248).contains(&p[0])).count();assert!(shadow>100,"soft virtual-ground contact shading: {shadow}");
    assert!(pixels.chunks_exact(4).filter(|p|p[..3].iter().all(|v|*v<80)).count()>200,"original antialiased black boundary ink");
    for(x,y)in[(20,20),(620,20)]{let p=&pixels[(y*640+x)*4..][..3];assert!(p.iter().all(|v|*v>250),"distant background remains clear: {p:?}");}
}
