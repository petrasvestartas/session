fn original_scene()->viewer_journey::scene::Scene{
    use std::rc::Rc;
    let mut scene=viewer_journey::scene::Scene::empty();
    let mut source=session_rust::BRep::create_block_with_hole(1.2,1.2,0.6,0.2);source.surfacecolor=session_rust::Color::new(1.,0.,0.,1.);
    let owner=scene.insert_cad(Rc::new(source)).unwrap();scene.place_owner(owner,session_rust::Xform::translation(0.,0.,-0.5)).unwrap();
    let plane=viewer_journey::clipping_geometry::plane_from(viewer_journey::clipping_geometry::Mode::Xy,&[[0.,0.,-0.5]],0.9).unwrap();viewer_journey::plane_original::insert(&mut scene,Rc::new(plane)).unwrap();scene
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer{{let renderer=Renderer::new(device,queue,format,&original_scene());renderer.set_depth_pyramid(true);renderer}}
fn draw(renderer:&Renderer,view:&wgpu::TextureView){let matrix=session_rust::Xform::orthographic(-1.,1.,-1.,1.,0.,1.).m.map(|v|v as f32);renderer.draw(view,&Default::default(),&matrix);assert_eq!(renderer.cap_usage()[1],1);assert_eq!(renderer.cap_error(),None);assert_eq!(renderer.depth_pyramid_error(),None);assert_eq!(renderer.depth_pyramid_usage()[3],6);assert!(renderer.depth_pyramid_usage()[0]>600_000);}
fn verify_pixels(pixels:&[u8]){
    let at=|x:usize,y:usize|&pixels[(y*640+x)*4..(y*640+x+1)*4];
    let solid=at(416,192);assert!(solid[..3].iter().all(|v|(201..=205).contains(v)),"closed original section: {solid:?}");
    let bore=at(346,221);assert!(bore[..3].iter().all(|v|*v>240),"original bore remains empty: {bore:?}");
    assert!(pixels.chunks_exact(4).filter(|p|p[..3].iter().all(|v|(201..=205).contains(v))).count()>65_000);
    assert!(pixels.chunks_exact(4).filter(|p|p[..3].iter().all(|v|*v<20)).count()>300,"black section and bore boundaries");
}
