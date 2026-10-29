fn editor() -> viewer_journey::editor::Editor {
    use viewer_journey::{editor::{Action, Editor}, tool::Shape};
    let mut editor = Editor::default(); editor.apply(Action::Close).unwrap();
    editor.apply(Action::BeginTool(Shape::Line)).unwrap();
    editor.apply(Action::ToolPoint([-0.65, -0.35, 0.])).unwrap();
    editor.apply(Action::ToolHover(Some([0.65, 0.35, 0.]))).unwrap();
    assert!(editor.scene.lines().is_empty()); editor
}

fn make_renderer(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Renderer {
    let editor = editor(); let mut renderer = Renderer::new(device, queue, format, &editor.scene);
    let mut math_quality=viewer_journey::math_view::Presentation::default();
    math_quality.drain_matrix(&mut renderer,&editor.scene,&editor.camera.uniform(),[WIDTH,HEIGHT],editor.selected).expect("Native mathematical display must meet the requested physical-pixel quality before capture");
    let math_scene=math_quality.scene(&editor.scene);
    renderer.set_preview(editor.draft.as_ref()).unwrap(); assert_eq!(renderer.preview_bytes(), 76); renderer
}

fn draw(renderer: &Renderer, view: &wgpu::TextureView) {
    let editor = editor(); renderer.draw(view, &editor.background, &editor.camera.uniform());
}

fn verify_pixels(pixels: &[u8]) {
    let yellow = pixels.chunks_exact(4).filter(|p| p[0] > p[1].saturating_add(10) && p[1] > p[2].saturating_add(50)).count();
    assert!(yellow > 150, "The unfinished line and cursor render without any saved line: area={yellow}");
}
