fn editor() -> viewer_journey::editor::Editor {
    use viewer_journey::editor::{Action,Editor};
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();
    for action in [Action::AddBox,Action::SelectAll,Action::Translate([-1.8,0.,0.]),Action::AddBox,Action::SelectAll,Action::Fit,Action::Isometric] {editor.apply(action).unwrap();}
    editor
}

fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer {
    let editor=editor();let mut renderer=Renderer::new(device,queue,format,&editor.scene);
    let mut math_quality=viewer_journey::math_view::Presentation::default();
    math_quality.drain_matrix(&mut renderer,&editor.scene,&editor.camera.uniform(),[WIDTH,HEIGHT],editor.selected).expect("Native mathematical display must meet the requested physical-pixel quality before capture");
    let math_scene=math_quality.scene(&editor.scene);
    renderer.select_objects(&math_scene,&editor.selected_objects()).unwrap();
    renderer.set_gizmo(&editor.camera,editor.gizmo_origin(),editor.gizmo_active());
    assert!(renderer.gizmo_usage()[1]>0 && renderer.gizmo_usage()[1]<=1024*1024*36);
    assert_eq!(renderer.pick_usage()[3],0,"Handles never submit source integer picks");renderer
}

fn draw(renderer:&Renderer,view:&wgpu::TextureView) {
    let editor=editor();renderer.draw(view,&editor.background,&editor.camera.uniform());
}

fn verify_pixels(pixels:&[u8]) {
    let mut colours=[0;3];let mut gold=0;let mut ink=0;
    for p in pixels.chunks_exact(4) {
        for i in 0..3 {if p[i]>p[(i+1)%3].saturating_add(50) && p[i]>p[(i+2)%3].saturating_add(50) {colours[i]+=1;}}
        if p[0]>p[1].saturating_add(10) && p[1]>p[2].saturating_add(50) {gold+=1;}
        if p[..3].iter().all(|v|*v<150) {ink+=1;}
    }
    assert!(colours.into_iter().all(|n|n>100),"All three handle colours draw over the selected boxes: {colours:?}");
    assert!(gold>3000,"Original selected boxes remain beneath the independent widget: {gold}");
    assert!(ink>100,"Original black source boundaries remain: {ink}");
}
