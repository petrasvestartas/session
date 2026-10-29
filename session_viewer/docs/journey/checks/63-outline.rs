fn editor()->viewer_journey::editor::Editor {
    use viewer_journey::editor::{Editor,Action};
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();editor.apply(Action::AddCad).unwrap();
    editor.apply(Action::Fit).unwrap();editor.apply(Action::Isometric).unwrap();let id=editor.scene.objects()[0].id;editor.apply(Action::Picked(Some(id))).unwrap();editor
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer {
    let editor=editor();let mut renderer=Renderer::new(device,queue,format,&editor.scene);
    let mut math_quality=viewer_journey::math_view::Presentation::default();
    math_quality.drain_matrix(&mut renderer,&editor.scene,&editor.camera.uniform(),[WIDTH,HEIGHT],editor.selected).expect("Native mathematical display must meet the requested physical-pixel quality before capture");
    let math_scene=math_quality.scene(&editor.scene);renderer.select_objects(&math_scene,&editor.selected_objects()).unwrap();renderer
}
fn draw(renderer:&Renderer,view:&wgpu::TextureView) {
    let editor=editor();renderer.draw(view,&editor.background,&editor.camera.uniform());let usage=renderer.outline_usage();assert!(usage[0]>0);assert_eq!(usage[1],1);assert_eq!(usage[3],1);
}
fn verify_pixels(pixels:&[u8]) {
    let gold=pixels.chunks_exact(4).filter(|p|p[0]>110&&p[1]>90&&p[2]<100&&p[0]>p[1]&&p[0]>p[2].saturating_mul(2)&&p[1]>p[2].saturating_mul(2)).count();
    let ink=pixels.chunks_exact(4).filter(|p|p[..3].iter().all(|v|*v<50)).count();
    assert!(gold>1000,"Selected original CAD faces remain visible: {gold}");assert!(ink>100,"Actual outer and bore contour ink remains black: {ink}");
}
