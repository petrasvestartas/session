fn editor() -> viewer_journey::editor::Editor {
    use viewer_journey::{editor::{Action, Editor}, pick_sources::{Mode, Part, Target}};
    let mut editor = Editor::default(); editor.apply(Action::Close).unwrap(); editor.apply(Action::AddBox).unwrap();
    editor.apply(Action::Isometric).unwrap(); editor.apply(Action::Fit).unwrap();
    let owner = editor.scene.objects()[0].id;
    editor.apply(Action::PickMode(Mode::Face)).unwrap(); editor.apply(Action::PickedTarget(Some(Target { owner, part: Part::Face(1) }))).unwrap();
    editor
}

fn make_renderer(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Renderer {
    let editor = editor(); let mut renderer = Renderer::new(device, queue, format, &editor.scene);
    let mut math_quality=viewer_journey::math_view::Presentation::default();
    math_quality.drain_matrix(&mut renderer,&editor.scene,&editor.camera.uniform(),[WIDTH,HEIGHT],editor.selected).expect("Native mathematical display must meet the requested physical-pixel quality before capture");
    let math_scene=math_quality.scene(&editor.scene);
    renderer.set_pick_mode(editor.pick_mode); renderer.set_highlight(&math_scene, editor.selection()).unwrap(); renderer
}

fn draw(renderer: &Renderer, view: &wgpu::TextureView) {
    let editor = editor(); renderer.draw(view, &editor.background, &editor.camera.uniform());
}

fn verify_pixels(pixels: &[u8]) {
    let gold = pixels.chunks_exact(4).filter(|p| p[0] > p[1].saturating_add(10) && p[1] > p[2].saturating_add(50)).count();
    let grey = pixels.chunks_exact(4).filter(|p| p[0] > 80 && p[0] < 230 && p[0] == p[1] && p[1] == p[2]).count();
    let black = pixels.chunks_exact(4).filter(|p| p[0] < 80 && p[1] < 80 && p[2] < 80).count();
    assert!(gold > 1000 && grey > 1000 && black > 100, "A local original face is gold, neighbouring faces stay grey, and boundaries are dark: {gold}/{grey}/{black}");
}
