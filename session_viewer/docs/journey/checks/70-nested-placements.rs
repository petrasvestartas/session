fn editor()->viewer_journey::editor::Editor {
    use viewer_journey::editor::{Editor,Action};
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();editor.apply(Action::AddNestedInstances).unwrap();editor.apply(Action::Fit).unwrap();editor.apply(Action::Isometric).unwrap();let id=editor.scene.objects()[1].id;editor.apply(Action::Picked(Some(id))).unwrap();editor
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer {
    let editor=editor();let mut renderer=Renderer::new(device,queue,format,&editor.scene);renderer.select_objects(&editor.scene,&editor.selected_objects()).unwrap();renderer
}
fn draw(renderer:&Renderer,view:&wgpu::TextureView) {
    let editor=editor();renderer.draw(view,&editor.background,&editor.camera.uniform());assert_eq!(&renderer.instance_usage()[..3],&[1,2,320]);
}
fn verify_pixels(pixels:&[u8]) {
    let gold=pixels.chunks_exact(4).filter(|p|p[0]>110&&p[1]>90&&p[2]<100&&p[0]>p[1]&&p[0]>p[2].saturating_mul(2)&&p[1]>p[2].saturating_mul(2)).count();let blue=pixels.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(20)&&p[2]>p[1].saturating_add(10)).count();assert!(gold>1000,"Selected mirrored original reference: {gold}");assert!(blue>1000,"Rotated original reference: {blue}");
}
