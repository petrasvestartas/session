fn editor()->viewer_journey::editor::Editor {
    use viewer_journey::editor::{Editor,Action};
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();editor.apply(Action::AddNested).unwrap();
    let key=viewer_journey::hierarchy::rows(&editor.scene).into_iter().find(|row|row.label=="Posts").unwrap().key;
    editor.apply(Action::LayerProperty {key,property:viewer_journey::layer_properties::Property::Colour {edge:false,rgb:Some([65,130,225])}}).unwrap();
    editor.apply(Action::Fit).unwrap();editor.apply(Action::Isometric).unwrap();editor
}

fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer {
    let editor=editor();let renderer=Renderer::new(device,queue,format,&editor.scene);
    let attribute=editor.scene.objects()[1].id;assert!(editor.scene.visible(attribute));assert_eq!(editor.scene.pick_id(attribute.value()),None);
    assert!(editor.scene.objects().iter().take(2).all(|row|editor.scene.view.faces[&row.id]==[65,130,225]));
    assert_eq!(renderer.pick_usage()[3],0);renderer
}

fn draw(renderer:&Renderer,view:&wgpu::TextureView) {let editor=editor();renderer.draw(view,&editor.background,&editor.camera.uniform());}

fn verify_pixels(pixels:&[u8]) {
    let (mut blue,mut brown,mut ink)=(0,0,0);
    for p in pixels.chunks_exact(4) {
        if p[2]>p[1].saturating_add(35)&&p[1]>p[0].saturating_add(25) {blue+=1;}
        if p[0]>p[1].saturating_add(35)&&p[1]>p[2].saturating_add(35) {brown+=1;}
        if p[..3].iter().all(|v|*v<100) {ink+=1;}
    }
    assert!(blue>1000,"The derived Posts face colour is visible: {blue}");
    assert!(brown>500,"The unrelated original source retains its colour: {brown}");
    assert!(ink>100,"Original source boundaries remain black: {ink}");
}
