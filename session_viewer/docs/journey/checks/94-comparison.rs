fn original_scene()->viewer_journey::scene::Scene{viewer_journey::comparison::original_scene(&viewer_journey::comparison::specimen()).unwrap()}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer{
    let renderer=Renderer::new(device,queue,format,&original_scene());renderer.set_outlines(true);renderer
}
fn draw(renderer:&Renderer,view:&wgpu::TextureView){
    let mut camera=viewer_journey::camera::Camera::default();camera.isometric();camera.aspect=4./3.;camera.fit(&original_scene().bounds().unwrap());renderer.draw(view,&Default::default(),&camera.uniform());assert_eq!(renderer.global_outline_usage()[1],1);
}
fn verify_pixels(pixels:&[u8]){
    let blue=pixels.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(20)&&p[2]>p[1].saturating_add(10)&&p[0]<220).count();assert!(blue>5000,"original curved and flat surfaces: {blue}");
    let black=pixels.chunks_exact(4).filter(|p|p[..3].iter().all(|v|*v<20)).count();assert!(black>400,"continuous black outline: {black}");
    for(x,y)in[(20,20),(620,20)]{assert!(pixels[(y*640+x)*4..][..3].iter().all(|v|*v>250));}
}
