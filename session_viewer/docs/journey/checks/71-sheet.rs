fn editor()->viewer_journey::editor::Editor {
    use viewer_journey::editor::{Editor,Action};
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();editor.apply(Action::AddSheet).unwrap();editor.apply(Action::Fit).unwrap();editor
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer {let editor=editor();{let mut renderer=Renderer::new(device,queue,format,&editor.scene);
    let mut math_quality=viewer_journey::math_view::Presentation::default();
    math_quality.drain_matrix(&mut renderer,&editor.scene,&editor.camera.uniform(),[WIDTH,HEIGHT],editor.selected).expect("Native mathematical display must meet the requested physical-pixel quality before capture");
    let math_scene=math_quality.scene(&editor.scene);
    renderer}}
fn draw(renderer:&Renderer,view:&wgpu::TextureView) {let editor=editor();renderer.sheet_camera(&editor.camera);renderer.draw(view,&editor.background,&editor.camera.uniform());assert_eq!(&renderer.sheet_usage()[..2],&[1,1]);}
fn verify_pixels(pixels:&[u8]) {
    let black=pixels.chunks_exact(4).filter(|p|p[0]<30&&p[1]<30&&p[2]<30).count();let blue=pixels.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(40)&&p[2]>p[1].saturating_add(20)).count();assert!(black>1000,"Original black physical pens: {black}");assert!(blue>100,"Original blue hairline: {blue}");
}
