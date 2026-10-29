fn original_scene()->viewer_journey::scene::Scene{
    use std::rc::Rc;
    let mut scene=viewer_journey::scene::Scene::empty();
    let mut source=session_rust::BRep::create_block_with_hole(1.2,1.2,0.6,0.2);source.surfacecolor=session_rust::Color::new(1.,0.,0.,1.);
    let owner=scene.insert_cad(Rc::new(source)).unwrap();scene.place_owner(owner,session_rust::Xform::translation(0.,0.,-0.5)).unwrap();
    let plane=viewer_journey::clipping_geometry::plane_from(viewer_journey::clipping_geometry::Mode::Xy,&[[0.,0.,-0.5]],0.9).unwrap();viewer_journey::plane_original::insert(&mut scene,Rc::new(plane)).unwrap();scene
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer{{let math_source=original_scene();let matrix=session_rust::Xform::orthographic(-1.,1.,-1.,1.,0.,1.).m.map(|v|v as f32);
    let mut renderer=Renderer::new(device,queue,format,&math_source);
    let mut math_quality=viewer_journey::math_view::Presentation::default();
    math_quality.drain_matrix(&mut renderer,&math_source,&matrix,[WIDTH,HEIGHT],None).expect("Native mathematical display must meet the requested physical-pixel quality before capture");
    let math_scene=math_quality.scene(&math_source);
    renderer}}
fn draw(renderer:&Renderer,view:&wgpu::TextureView){let matrix=session_rust::Xform::orthographic(-1.,1.,-1.,1.,0.,1.).m.map(|v|v as f32);renderer.draw(view,&Default::default(),&matrix);assert_eq!(renderer.cap_usage()[1],1);assert_eq!(renderer.cap_error(),None);}
fn verify_pixels(pixels:&[u8]){
    let at=|x:usize,y:usize|&pixels[(y*640+x)*4..(y*640+x+1)*4];
    let solid=at(416,192);assert!(solid[..3].iter().all(|v|(201..=205).contains(v)),"closed original section: {solid:?}");
    let bore=at(346,221);assert!(bore[..3].iter().all(|v|*v>240),"original bore remains empty: {bore:?}");
    assert!(pixels.chunks_exact(4).filter(|p|p[..3].iter().all(|v|(201..=205).contains(v))).count()>65_000);
    assert!(pixels.chunks_exact(4).filter(|p|p[..3].iter().all(|v|*v<20)).count()>300,"black section and bore boundaries");
}
