fn original_scene()->viewer_journey::scene::Scene{
    use std::rc::Rc;
    let mut scene=viewer_journey::scene::Scene::empty();
    scene.insert(viewer_journey::prepared::PreparedMesh::triangle([[-0.8,-0.8,0.5],[0.8,-0.8,0.5],[0.,0.8,0.5]],[0.1,0.5,0.8]).unwrap()).unwrap();
    let mut line=session_rust::Line::new(-0.7,0.2,0.2,0.7,0.2,0.2);line.width=4.;line.linecolor=session_rust::Color::new(1.,0.,0.,1.);scene.insert_line(viewer_journey::stroke::PreparedLine::new(Rc::new(line)).unwrap()).unwrap();
    let plane=viewer_journey::clipping_geometry::plane_from(viewer_journey::clipping_geometry::Mode::Yz,&[[0.,0.,0.5]],0.9).unwrap();viewer_journey::plane_original::insert(&mut scene,Rc::new(plane)).unwrap();scene
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer{{let math_source=original_scene();let matrix=session_rust::Xform::identity().m.map(|v|v as f32);
    let mut renderer=Renderer::new(device,queue,format,&math_source);
    let mut math_quality=viewer_journey::math_view::Presentation::default();
    math_quality.drain_matrix(&mut renderer,&math_source,&matrix,[WIDTH,HEIGHT],None).expect("Native mathematical display must meet the requested physical-pixel quality before capture");
    let math_scene=math_quality.scene(&math_source);
    renderer}}
fn draw(renderer:&Renderer,view:&wgpu::TextureView){let matrix=session_rust::Xform::identity().m.map(|v|v as f32);renderer.draw(view,&Default::default(),&matrix);assert_eq!(&renderer.clipping_usage()[..3],&[1,1,1]);assert_eq!(renderer.clipping_error(),None);}
fn verify_pixels(pixels:&[u8]){
    let at=|x:usize,y:usize|&pixels[(y*640+x)*4..(y*640+x+1)*4];
    let kept=at(192,192);assert!(kept[0]>200&&kept[1]<60&&kept[2]<60,"retained original line: {kept:?}");
    for (x,y) in [(440,192),(400,300)]{let p=at(x,y);assert!(p[0]>230&&p[1]>230&&p[2]>230,"cut original color at{x},{y}: {p:?}");}
    let blue=at(270,300);assert!(blue[2]>blue[0]&&blue[2]>blue[1],"retained original solid: {blue:?}");
    assert!(pixels.chunks_exact(4).filter(|p|p[0]>200&&p[1]<60&&p[2]<60).count()>600);
}
