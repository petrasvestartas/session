fn editor() -> viewer_journey::editor::Editor {
    use viewer_journey::editor::{Action,Editor};
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();
    editor.apply(Action::AddNested).unwrap();let parent=editor.scene.objects()[0].id;
    editor.apply(Action::TreePick {ids:vec![parent],additive:false}).unwrap();
    editor.apply(Action::Fit).unwrap();editor.apply(Action::Isometric).unwrap();editor
}

fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer {
    let editor=editor();let mut renderer=Renderer::new(device,queue,format,&editor.scene);
    let mut math_quality=viewer_journey::math_view::Presentation::default();
    math_quality.drain_matrix(&mut renderer,&editor.scene,&editor.camera.uniform(),[WIDTH,HEIGHT],editor.selected).expect("Native mathematical display must meet the requested physical-pixel quality before capture");
    let math_scene=math_quality.scene(&editor.scene);
    renderer.select_objects(&math_scene,&editor.selected_objects()).unwrap();
    renderer.set_gizmo(&editor.camera,editor.gizmo_origin(),editor.gizmo_active());
    let attribute=editor.scene.objects()[1].id;
    assert!(editor.scene.visible(attribute));assert_eq!(editor.scene.pick_id(attribute.value()),None);
    assert_eq!(renderer.pick_usage()[3],0);renderer
}

fn draw(renderer:&Renderer,view:&wgpu::TextureView) {
    let editor=editor();renderer.draw(view,&editor.background,&editor.camera.uniform());
}

fn verify_pixels(pixels:&[u8]) {
    let mut gold=0;let mut brown=0;let mut ink=0;
    for p in pixels.chunks_exact(4) {
        if p[0]>200 && p[1]>170 && p[2]<150 {gold+=1;}
        if p[0]>p[1].saturating_add(35) && p[1]>p[2].saturating_add(35) {brown+=1;}
        if p[..3].iter().all(|v|*v<150) {ink+=1;}
    }
    assert!(gold>500,"The editable parent draws selected: {gold}");
    assert!(brown>1000,"Readonly attributes and other original sources remain visible: {brown}");
    assert!(ink>100,"Original source boundaries remain black: {ink}");
}
