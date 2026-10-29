fn original_scene()->viewer_journey::scene::Scene{
    use std::rc::Rc;
    let mut scene=viewer_journey::scene::Scene::empty();
    scene.insert(viewer_journey::prepared::PreparedMesh::triangle([[-0.8,-0.8,0.5],[0.8,-0.8,0.5],[0.,0.8,0.5]],[0.1,0.5,0.8]).unwrap()).unwrap();
    for (y,z,color) in [(0.1,0.3,[1.,0.,0.]),(-0.2,0.7,[0.,0.,0.])]{let mut line=session_rust::Line::new(-0.25,y,z,0.25,y,z);line.width=4.;line.linecolor=session_rust::Color::new(color[0],color[1],color[2],1.);scene.insert_line(viewer_journey::stroke::PreparedLine::new(Rc::new(line)).unwrap()).unwrap();}
    let mut point=session_rust::Point::new(0.1,0.4,0.3);point.width=10.;point.pointcolor=session_rust::Color::new(1.,0.,0.,1.);scene.insert_point(viewer_journey::marker::PreparedPoint::new(Rc::new(point)).unwrap()).unwrap();scene
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer{{let math_source=original_scene();let matrix=session_rust::Xform::identity().m.map(|v|v as f32);
    let mut renderer=Renderer::new(device,queue,format,&math_source);
    let mut math_quality=viewer_journey::math_view::Presentation::default();
    math_quality.drain_matrix(&mut renderer,&math_source,&matrix,[WIDTH,HEIGHT],None).expect("Native mathematical display must meet the requested physical-pixel quality before capture");
    let math_scene=math_quality.scene(&math_source);
    renderer}}
fn draw(renderer:&Renderer,view:&wgpu::TextureView){let matrix=session_rust::Xform::identity().m.map(|v|v as f32);renderer.draw(view,&Default::default(),&matrix);let first=renderer.tile_usage();assert_eq!(first[3],1);assert_eq!(first[5..],[1,1]);}
fn verify_pixels(pixels:&[u8]){
    let at=|x:usize,y:usize|&pixels[(y*640+x)*4..(y*640+x+1)*4];
    let red=pixels.chunks_exact(4).filter(|p|p[0]>180&&p[1]<80&&p[2]<80).count();assert!(red>650,"Actual front original stroke and point: {red}");
    for (x,y) in [(320,216),(352,144)]{let p=at(x,y);assert!(p[0]>180&&p[1]<80,"front original coverage at {x},{y}: {p:?}");}
    for x in 265..375{let p=at(x,288);assert!(p[2]>p[0]&&p[2]>p[1],"hidden rear original cannot cover the blue solid: {p:?}");}
}
