fn editor() -> viewer_journey::editor::Editor {
    use viewer_journey::{editor::{Action,Editor},tool::Shape};
    let mut editor=Editor::default(); editor.apply(Action::Close).unwrap();
    for action in [Action::AddBox,Action::SelectAll,Action::Translate([-1.8,0.,0.]),Action::AddBox,Action::SelectAll,Action::Fit] { editor.apply(action).unwrap(); }
    let before: Vec<_>=editor.scene.objects().iter().map(|r| r.model.m).collect();
    editor.apply(Action::BeginTool(Shape::Move)).unwrap(); editor.apply(Action::ToolPoint([0.;3])).unwrap();
    editor.apply(Action::ToolHover(Some([0.4,0.2,0.]))).unwrap();
    assert_eq!(editor.scene.objects().iter().map(|r| r.model.m).collect::<Vec<_>>(),before);
    for (row,model) in editor.display_scene().objects().iter().zip(before) { assert_eq!(row.model.m[12],model[12]+0.4); }
    editor
}

fn make_renderer(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Renderer {
    let editor=editor(); let mut renderer=Renderer::new(device,queue,format,&editor.scene); let before=renderer.stats();
    renderer.set_scene(editor.display_scene(),editor.selected); renderer.select_objects(editor.display_scene(),&editor.selected_objects()).unwrap();
    let after=renderer.stats(); assert_eq!(&after[..2],&before[..2],"Preview reuses original mesh uploads and existing row settings allocations");
    renderer.set_preview(editor.draft.as_ref()).unwrap(); renderer
}

fn draw(renderer: &Renderer, view: &wgpu::TextureView) {
    let editor=editor(); renderer.draw(view,&editor.background,&editor.camera.uniform());
}

fn verify_pixels(pixels: &[u8]) {
    let gold=pixels.chunks_exact(4).filter(|p| p[0]>p[1].saturating_add(10) && p[1]>p[2].saturating_add(50)).count();
    let ink=pixels.chunks_exact(4).filter(|p| p[..3].iter().all(|v| *v<180)).count();
    assert!(gold>5000,"The actual moved selected boxes and temporary band draw: {gold}");
    assert!(ink>100,"Original boundary ink remains on the moved display: {ink}");
}
