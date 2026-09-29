fn editor() -> viewer_journey::editor::Editor {
    use viewer_journey::editor::{Action, Editor};
    let mut editor = Editor::default();
    editor.apply(Action::AddBox).unwrap();
    editor.apply(Action::Isometric).unwrap();
    for _ in 0..3 {
        editor.apply(Action::SelectNext).unwrap();
    }
    assert_eq!(editor.selected, Some(editor.scene.objects()[2].id));
    editor
}

fn make_renderer(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Renderer {
    let editor = editor();
    let mut renderer = Renderer::new(device, queue, format, &editor.scene);
    renderer.set_scene(&editor.scene, editor.selected);
    renderer
}

fn draw(renderer: &Renderer, view: &wgpu::TextureView) {
    let editor = editor();
    renderer.draw(view, &editor.background, &editor.camera.uniform());
}

fn verify_pixels(pixels: &[u8]) {
    let editor = editor();
    let screen = editor.camera.view_projection().transform_point(&session_rust::Point::new(0.9, 0.0, 0.8));
    let offset = (((1.0 - screen[1]) * 240.0) as usize * 640 + ((screen[0] + 1.0) * 320.0) as usize) * 4;
    let top = &pixels[offset..offset + 3];
    assert!(top[0] > 220 && top[1] > 190 && top[2] < 80, "Selection colour must reach the lit box: {top:?}");
    let yellow = pixels.chunks_exact(4).filter(|p| p[0] > 175 && p[1] > 150 && p[2] < 80).count();
    assert!(yellow > 1_000, "Selection must cover the solid: {yellow}");
}
