fn editor()->viewer_journey::editor::Editor {
    use viewer_journey::{editor::{Editor,Action},commands::{parse,Command}};
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();
    for line in ["CPlane XY","Nurbs Curve Circle w-2,0,0 0.5","Nurbs Curve Ellipse w-0.7,0,0 0.55 0.3","Nurbs Curve Arc Center w0.7,0,0 0.5 270","Nurbs Curve Parabola w1.5,0,0 1 0.5","Curve w-0.5,1,0 w0,1.6,0 w0.5,1,0","View Reset","Fit"] {
        let Command::Editor(action)=parse(line).unwrap() else{panic!("Editor command")};editor.apply(action).unwrap();
    }editor
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer {let editor=editor();Renderer::new(device,queue,format,&editor.scene)}
fn draw(renderer:&Renderer,view:&wgpu::TextureView) {let editor=editor();renderer.draw(view,&editor.background,&editor.camera.uniform());assert_eq!(editor.scene.paths().len(),5);assert!(editor.scene.paths().iter().all(|row|matches!(row.prepared.source,viewer_journey::chain::Source::Curve(..))));assert!(renderer.path_usage()[0]>0);}
fn verify_pixels(pixels:&[u8]) {let ink=pixels.chunks_exact(4).filter(|p|p[0]<220&&p[1]<220&&p[2]<220).count();assert!(ink>500,"Actual original exact curves: {ink}");}
