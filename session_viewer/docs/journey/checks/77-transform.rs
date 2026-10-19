fn editor()->viewer_journey::editor::Editor {
    use viewer_journey::{editor::{Editor,Action},commands::{parse,Command}};
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();
    for line in ["CPlane XY","Box w0,0,0 1 1 1","Select All","Copy 2,0,0","Scale 0.8","Rotate z 30","Copy -2,0,0","View Isometric","Select None","Fit"] {
        let Command::Editor(action)=parse(line).unwrap() else{panic!("Editor command")};editor.apply(action).unwrap();
    }editor
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer {let editor=editor();Renderer::new(device,queue,format,&editor.scene)}
fn draw(renderer:&Renderer,view:&wgpu::TextureView) {let editor=editor();renderer.draw(view,&editor.background,&editor.camera.uniform());let rows=editor.scene.objects();assert_eq!(rows.len(),3);assert!(rows.iter().all(|row|std::rc::Rc::ptr_eq(&row.cad.as_ref().unwrap().source,&rows[0].cad.as_ref().unwrap().source)));assert_eq!(rows[1].model.m[12],2.);assert_ne!(rows[0].model.m,rows[1].model.m);for row in rows{let bounds=editor.scene.selected_bounds(row.id).unwrap();let center=bounds.centre();let projected=editor.camera.view_projection().transform_point(&session_rust::Point::new(center[0],center[1],center[2]));assert!(projected[0].abs()<0.9&&projected[1].abs()<0.9,"Each copied box stays in the frame");}assert_eq!(renderer.stats()[0],1,"Copied rows share one geometry upload");}
fn verify_pixels(pixels:&[u8]) {let ink=pixels.chunks_exact(4).filter(|p|p[0]<220&&p[1]<220&&p[2]<220).count();assert!(ink>1000,"Actual independently placed original boxes: {ink}");let black=pixels.chunks_exact(4).filter(|p|p[0]<100&&p[1]<100&&p[2]<100).count();assert!(black>100,"Actual original box boundaries: {black}");}
