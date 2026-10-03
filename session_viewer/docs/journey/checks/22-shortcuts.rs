fn editor() -> viewer_journey::editor::Editor {
    use viewer_journey::{editor::{Action, Editor}, navigation::wheel};
    let mut editor = Editor::default();
    editor.apply(Action::AddBox).unwrap();
    editor.apply(Action::Isometric).unwrap();
    editor.apply(wheel(-120.0, 0, 480.0).unwrap()).unwrap();
    editor.apply(Action::Pan(0.25, 0.0)).unwrap();
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
    let point = session_rust::Point::new(0.5, 0.0, 0.4);
    let screen = editor.camera.view_projection().transform_point(&point);
    assert!(screen[0].abs() < 1.0 && screen[1].abs() < 1.0);
    let offset = (((1.0 - screen[1]) * 240.0) as usize * 640
        + ((screen[0] + 1.0) * 320.0) as usize) * 4;
    let linear = 0.65 * (0.3 + 0.7 * 0.4 / 1.52_f64.sqrt());
    let expected = (255.0 * (1.055 * linear.powf(1.0 / 2.4) - 0.055)).round() as u8;
    let face = &pixels[offset..offset + 3];
    assert!(face.iter().all(|&value| value.abs_diff(expected) <= 2),
        "The zoomed and panned box face should be {expected}, got {face:?}");
    let solid = pixels.chunks_exact(4).filter(|p| p[0] > 140 && p[0] == p[1] && p[1] == p[2]).count();
    assert!(solid > 1000, "The solid must cover more than a single test pixel");
}
