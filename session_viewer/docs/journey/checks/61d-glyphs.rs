fn editor()->viewer_journey::editor::Editor {
    use viewer_journey::editor::{Editor,Action};
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();
    editor.apply(Action::AddCad).unwrap();editor.apply(Action::Fit).unwrap();
    editor.apply(Action::Isometric).unwrap();editor
}

fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer {
    let editor=editor();let row=&editor.scene.objects()[0];let cad=row.cad.as_ref().unwrap();
    assert_eq!(row.mesh.parts.faces.len(),7);assert_eq!(cad.edges.len(),15);
    assert_eq!(cad.source.m_faces[6].wires.len(),2);
    assert_eq!(row.metadata.source_guid,cad.source.guid());
    let renderer=Renderer::new(device,queue,format,&editor.scene);renderer.text_demo().unwrap();assert_eq!(renderer.text_usage()[0],1);renderer
}

fn draw(renderer:&Renderer,view:&wgpu::TextureView) {
    let editor=editor();renderer.draw(view,&editor.background,&editor.camera.uniform());
}

fn verify_pixels(pixels:&[u8]) {
    let (mut blue,mut ink)=(0,0);
    for p in pixels.chunks_exact(4) {
        if p[2]>p[1].saturating_add(20)&&p[1]>p[0].saturating_add(20) {blue+=1;}
        if p[..3].iter().all(|v|*v<100) {ink+=1;}
    }
    let font=pixels.chunks_exact(4).take(640*50).filter(|p|p[0]<200&&p[1]<200&&p[2]<200).count();
    assert!(font>200,"Real Noto specimen glyphs render at the top left: {font}");
    assert!(blue>1000,"The original CAD bore has blue faces: {blue}");
    assert!(ink>100,"Original CAD borders remain black: {ink}");
}
