fn editor()->viewer_journey::editor::Editor {
    use viewer_journey::editor::{Editor,Action};
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();editor.apply(Action::AddCloud).unwrap();
    editor.apply(Action::Fit).unwrap();editor.apply(Action::Isometric).unwrap();editor
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer {
    let editor=editor();let renderer=Renderer::new(device,queue,format,&editor.scene);renderer.cloud_camera(&editor.camera);assert_eq!(editor.scene.clouds()[0].prepared.source.len(),1681);renderer
}
fn draw(renderer:&Renderer,view:&wgpu::TextureView) {
    let editor=editor();renderer.cloud_camera(&editor.camera);renderer.draw(view,&editor.background,&editor.camera.uniform());let usage=renderer.cloud_usage();assert_eq!(usage[0],1681);assert_eq!(usage[2],1);assert_eq!(usage[3],1);assert_eq!(renderer.cloud_error(),None);
}
fn verify_pixels(pixels:&[u8]) {
    let colored=pixels.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(20)&&p[2]>p[1].saturating_add(10)).count();
    assert!(colored>1000,"Actual original coloured cloud discs remain visible: {colored}");
}
