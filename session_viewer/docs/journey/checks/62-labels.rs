fn editor()->viewer_journey::editor::Editor {
    use viewer_journey::{editor::{Editor,Action},text::TextPlacement};
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();editor.apply(Action::AddCad).unwrap();
    editor.apply(Action::AddText(TextPlacement::WorldPlane {world:[-1.5,0.,1.],right:[1.,0.,0.],up:[0.,1.,0.],world_height:0.22})).unwrap();
    editor.apply(Action::Fit).unwrap();editor.apply(Action::Isometric).unwrap();let id=editor.scene.texts()[0].id;editor.apply(Action::Picked(Some(id))).unwrap();editor
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer {
    let editor=editor();let mut renderer=Renderer::new(device,queue,format,&editor.scene);renderer.select_objects(&editor.scene,&editor.selected_objects()).unwrap();
    renderer.replace_text_fonts(vec![include_bytes!("../assets/text/NotoSans-Regular.ttf").to_vec(),include_bytes!("../assets/text/NotoSansSymbols2-Regular.ttf").to_vec(),include_bytes!("../assets/text/NotoSansSymbols-Regular.ttf").to_vec()]).unwrap();
    assert_eq!(editor.scene.texts()[0].prepared.source.label.text,"Original CAD · Ø25 · Ω");assert!(editor.scene.texts()[0].prepared.source.label.object.is_none());renderer
}
fn draw(renderer:&Renderer,view:&wgpu::TextureView) {let editor=editor();renderer.draw(view,&editor.background,&editor.camera.uniform());assert!(renderer.text_scene_usage()[1]>0);assert_eq!(renderer.text_error(),None);assert_eq!(renderer.text_stats().unwrap().missing_glyphs,0);}
fn verify_pixels(pixels:&[u8]) {
    let blue=pixels.chunks_exact(4).filter(|p|p[2]>p[1].saturating_add(20)&&p[1]>p[0].saturating_add(20)).count();
    let gold=pixels.chunks_exact(4).filter(|p|p[0]>200&&p[1]>200&&p[2]<50).count();
    let ink=pixels.chunks_exact(4).filter(|p|p[..3].iter().all(|v|*v<100)).count();
    assert!(blue>1000,"Original CAD faces remain visible: {blue}");assert!(gold>100,"The selected original world-plane text has a real yellow plate: {gold}");assert!(ink>100,"Actual text ink and CAD borders remain black: {ink}");
}
