fn editor()->viewer_journey::editor::Editor {
    use viewer_journey::{editor::{Editor,Action},pick_sources::{Target,Part}};
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();editor.apply(Action::AddCloud).unwrap();
    editor.apply(Action::Fit).unwrap();editor.apply(Action::Isometric).unwrap();let owner=editor.scene.clouds()[0].id;
    editor.apply(Action::Picked(Some(owner))).unwrap();editor.apply(Action::Controls(true)).unwrap();
    editor.apply(Action::PickedTarget(Some(Target {owner,part:Part::CloudPoint(840)}))).unwrap();editor
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer {
    let editor=editor();let mut renderer=Renderer::new(device,queue,format,&editor.scene);renderer.cloud_camera(&editor.camera);
    renderer.set_highlight(&editor.scene,editor.selection()).unwrap();assert!(renderer.highlight_bytes()>0);renderer
}
fn draw(renderer:&Renderer,view:&wgpu::TextureView) {
    let editor=editor();renderer.cloud_camera(&editor.camera);renderer.draw(view,&editor.background,&editor.camera.uniform());
    let usage=renderer.cloud_usage();assert_eq!(usage[0],1681);assert_eq!(usage[2],1);assert_eq!(renderer.cloud_error(),None);
    assert!(editor.selection().unwrap().describe().contains("original point 840"));
}
fn verify_pixels(pixels:&[u8]) {
    let colored=pixels.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(20)&&p[2]>p[1].saturating_add(10)).count();
    let yellow=pixels.chunks_exact(4).filter(|p|p[0]>180&&p[1]>120&&p[2]<80).count();
    assert!(colored>1000,"Original coloured cloud remains visible: {colored}");
    assert!(yellow>10,"Local original point highlight remains visible: {yellow}");
}
