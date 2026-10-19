fn editor()->viewer_journey::editor::Editor {
    use viewer_journey::editor::{Editor,Action};
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();editor.apply(Action::AddCad).unwrap();let cad=editor.scene.objects()[0].id;editor.scene.translate(cad,[-4.,0.,0.]).unwrap();editor.apply(Action::AddInstances).unwrap();let instances=editor.scene.objects()[1..].iter().map(|row|row.id).collect::<Vec<_>>();for id in instances {editor.scene.translate(id,[2.,0.,0.]).unwrap();}editor.apply(Action::Fit).unwrap();editor.apply(Action::Isometric).unwrap();let id=editor.scene.objects()[1].id;editor.apply(Action::Picked(Some(id))).unwrap();editor
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer {
    let editor=editor();let mut renderer=Renderer::new(device,queue,format,&editor.scene);renderer.select_objects(&editor.scene,&editor.selected_objects()).unwrap();renderer
}
fn draw(renderer:&Renderer,view:&wgpu::TextureView) {
    let editor=editor();renderer.draw(view,&editor.background,&editor.camera.uniform());assert_eq!(&renderer.instance_usage()[..3],&[1,2,320]);
}
fn verify_pixels(pixels:&[u8]) {
    let blue=pixels.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(20)&&p[2]>p[1].saturating_add(10)).count();let yellow=pixels.chunks_exact(4).filter(|p|p[0]>180&&p[1]>140&&p[2]<100).count();let dark=pixels.chunks_exact(4).filter(|p|p[0]<65&&p[1]<65&&p[2]<65).count();assert!(blue>100,"Independent blue original instance: {blue}");assert!(yellow>100,"Selected shared-definition instance: {yellow}");assert!(dark>100,"Original CAD boundaries: {dark}");
}
