fn editor()->viewer_journey::editor::Editor {
    use viewer_journey::editor::{Editor,Action};
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();
    editor.apply(Action::AddSurface).unwrap();editor.apply(Action::Fit).unwrap();
    editor.apply(Action::Isometric).unwrap();editor
}

fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer {
    let editor=editor();let row=&editor.scene.objects()[0];
    assert_eq!(row.mesh.parts.faces,vec![0]);assert_eq!(row.mesh.parts.edges.len(),64);
    assert_eq!(row.surface.as_ref().unwrap().grid.boundaries.len(),4);
    {let mut renderer=Renderer::new(device,queue,format,&editor.scene);
    let mut math_quality=viewer_journey::math_view::Presentation::default();
    math_quality.drain_matrix(&mut renderer,&editor.scene,&editor.camera.uniform(),[WIDTH,HEIGHT],editor.selected).expect("Native mathematical display must meet the requested physical-pixel quality before capture");
    let math_scene=math_quality.scene(&editor.scene);
    renderer}
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
    assert!(blue>1000,"The parametric roof is blue: {blue}");
    assert!(ink>100,"Original surface borders remain black: {ink}");
}
