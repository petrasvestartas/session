fn original_scene()->viewer_journey::scene::Scene{
    use std::rc::Rc;
    let mut scene=viewer_journey::scene::Scene::empty();
    for(mut source,z)in[
        (session_rust::BRep::create_box(1.5,1.5,0.1),-0.5),
        (session_rust::BRep::create_box(0.35,0.35,0.7),-0.1),
        (session_rust::BRep::create_cylinder(0.12,0.45),0.25),
        (session_rust::BRep::create_cylinder(0.65,0.12),0.7),
    ]{source.surfacecolor=session_rust::Color::new(0.35,0.65,0.85,1.);let id=scene.insert_cad(Rc::new(source)).unwrap();scene.place_owner(id,session_rust::Xform::translation(0.,0.,z)).unwrap();}
    scene
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer{
    let mut renderer={let math_source=original_scene();let mut camera=viewer_journey::camera::Camera::default();camera.isometric();camera.aspect=4./3.;camera.target=[0.,0.,0.2];camera.distance=4.;
    let mut renderer=Renderer::new(device,queue,format,&math_source);
    let mut math_quality=viewer_journey::math_view::Presentation::default();
    math_quality.drain_matrix(&mut renderer,&math_source,&camera.uniform(),[WIDTH,HEIGHT],None).expect("Native mathematical display must meet the requested physical-pixel quality before capture");
    let math_scene=math_quality.scene(&math_source);
    renderer};renderer.set_outlines(true);renderer.set_opacity(0.4);renderer
}
fn draw(renderer:&Renderer,view:&wgpu::TextureView){
    let mut camera=viewer_journey::camera::Camera::default();camera.isometric();camera.aspect=4./3.;camera.target=[0.,0.,0.2];camera.distance=4.;renderer.draw(view,&Default::default(),&camera.uniform());assert_eq!(renderer.global_outline_usage()[1],1);
}
fn verify_pixels(pixels:&[u8]){
    let blue=pixels.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(10)&&p[2]>p[1].saturating_add(3)&&p[0]<230).count();assert!(blue>20000,"original curved and flat surfaces: {blue}");
    let black=pixels.chunks_exact(4).filter(|p|p[..3].iter().all(|v|*v<20)).count();assert!(black>700,"continuous black outline: {black}");
    for(x,y)in[(20,20),(620,20)]{assert!(pixels[(y*640+x)*4..][..3].iter().all(|v|*v>250));}
}
