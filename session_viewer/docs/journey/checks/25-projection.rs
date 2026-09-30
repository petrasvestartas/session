fn editor() -> viewer_journey::editor::Editor {
    use viewer_journey::editor::{Action, Editor};
    let mut editor = Editor::default();
    editor.apply(Action::Import(std::fs::read("sample.pb").unwrap())).unwrap();
    editor.apply(Action::Isometric).unwrap();
    editor.apply(Action::Projection(viewer_journey::camera::Projection::Orthographic)).unwrap();
    editor.apply(Action::Fit).unwrap();
    editor
}

fn make_renderer(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Renderer {
    std::fs::write("sample.pb", viewer_journey::specimen::bytes()).unwrap();
    Renderer::new(device, queue, format, &editor().scene)
}

fn draw(renderer: &Renderer, view: &wgpu::TextureView) {
    let editor = editor();
    renderer.draw(view, &editor.background, &editor.camera.uniform());
}

fn verify_pixels(pixels: &[u8]) {
    let editor = editor();
    for object in editor.scene.objects() {
        for vertex in object.mesh.vertices() {
            let point = session_rust::Point::new(vertex[0] as f64, vertex[1] as f64, vertex[2] as f64);
            let screen = editor.camera.view_projection().transform_point(&point);
            assert!(screen[0].abs() < 0.95 && screen[1].abs() < 0.95);
            assert!((0.0..1.0).contains(&screen[2]));
        }
    }
    let point = session_rust::Point::new(0.0, -0.175, 1.125);
    let screen = editor.camera.view_projection().transform_point(&point);
    let ray = editor.camera.ray([screen[0] as f32, screen[1] as f32]).unwrap();
    let hit = viewer_journey::picking::pick(&editor.scene, &ray).unwrap();
    let beam = editor.scene.objects().iter().find(|object| object.id == hit).unwrap();
    assert!(beam.mesh.vertices().iter().any(|vertex| vertex[2] >= 1.25));
    let offset = (((1.0 - screen[1]) * 240.0) as usize * 640
        + ((screen[0] + 1.0) * 320.0) as usize) * 4;
    let pixel = &pixels[offset..offset + 3];
    assert!(pixel[0] > pixel[1] + 30 && pixel[1] > pixel[2] + 30,
        "Fitted beam must occupy its projected position: {pixel:?}");
    let orange = pixels.chunks_exact(4).filter(|p| p[0] > p[1].saturating_add(30)
        && p[1] > p[2].saturating_add(30)).count();
    assert!(orange > 2000, "Fitted frame must cover visible area: {orange}");
}
