fn editor() -> viewer_journey::editor::Editor {
    use viewer_journey::editor::{Action, Editor};
    let mut editor = Editor::default();
    for action in [Action::AddBox, Action::Isometric, Action::SelectNext, Action::FitSelected] {
        editor.apply(action).unwrap();
    }
    editor
}

fn make_renderer(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Renderer {
    Renderer::new(device, queue, format, &editor().scene)
}

fn draw(renderer: &Renderer, view: &wgpu::TextureView) {
    let editor = editor();
    renderer.draw(view, &editor.background, &editor.camera.uniform());
}

fn verify_pixels(pixels: &[u8]) {
    let editor = editor();
    let selected = editor.scene.objects().iter().find(|o| Some(o.id) == editor.selected).unwrap();
    for vertex in selected.mesh.vertices() {
        let point = session_rust::Point::new(vertex[0] as f64, vertex[1] as f64, vertex[2] as f64);
        let screen = editor.camera.view_projection().transform_point(&point);
        assert!(screen[0].abs() < 0.95 && screen[1].abs() < 0.95);
        assert!((0.0..1.0).contains(&screen[2]));
    }
    let ink = pixels.chunks_exact(4).filter(|p| p[0..3].iter().any(|&v| v < 240)).count();
    assert!(ink > 3000, "Selected fit should draw a substantial model area: {ink}");
    assert_eq!(editor.scene.objects().len(), 3);
}
