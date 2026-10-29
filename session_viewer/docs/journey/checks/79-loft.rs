fn editor()->viewer_journey::editor::Editor {
    use viewer_journey::{editor::{Editor,Action},commands::{parse,Command}};
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();
    for line in ["Curve w-1,-1,0 w0,-1,1 w1,-1,0","Curve w-1,0,0 w0,0,1.5 w1,0,0","Curve w-1,1,0 w0,1,0.75 w1,1,0","Select All","Nurbs Surface Loft Smooth","View Isometric","Select None","Fit"] {
        let Command::Editor(action)=parse(line).unwrap() else{panic!("Editor command")};editor.apply(action).unwrap();
    }editor
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer {let editor=editor();{let mut renderer=Renderer::new(device,queue,format,&editor.scene);
    let mut math_quality=viewer_journey::math_view::Presentation::default();
    math_quality.drain_matrix(&mut renderer,&editor.scene,&editor.camera.uniform(),[WIDTH,HEIGHT],editor.selected).expect("Native mathematical display must meet the requested physical-pixel quality before capture");
    let math_scene=math_quality.scene(&editor.scene);
    renderer}}
fn draw(renderer:&Renderer,view:&wgpu::TextureView) {let editor=editor();renderer.draw(view,&editor.background,&editor.camera.uniform());assert_eq!(editor.scene.paths().len(),3);assert_eq!(editor.scene.objects().len(),1);let source=&editor.scene.objects()[0].surface.as_ref().unwrap().source;assert!(source.is_valid());assert!(source.degree(1)>1);assert_eq!(renderer.stats()[0],1);}
fn verify_pixels(pixels:&[u8]) {let ink=pixels.chunks_exact(4).filter(|p|p[0]<220&&p[1]<220&&p[2]<220).count();assert!(ink>1000,"Actual original loft surface: {ink}");let black=pixels.chunks_exact(4).filter(|p|p[0]<100&&p[1]<100&&p[2]<100).count();assert!(black>75,"Actual section and original domain boundaries: {black}");}
