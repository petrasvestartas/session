fn make_renderer(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Renderer {
    let mut camera = viewer_journey::camera::Camera::default();
    camera.pan(0.1, -0.2);
    camera.zoom(0.5);
    camera.rotate(0.7);
    let point = [0.4, 0.0];
    let matrix = camera.uniform();
    let screen = [matrix[0] * point[0] + matrix[4] * point[1] + matrix[12],
        matrix[1] * point[0] + matrix[5] * point[1] + matrix[13]];
    let restored = camera.world_from_screen(screen);
    assert!((restored[0] - point[0]).abs() < 1.0e-6);
    assert!((restored[1] - point[1]).abs() < 1.0e-6);
    let scene = viewer_journey::scene::Scene::demo();
    let mut renderer = Renderer::new(device, queue, format, &scene);
    let selected = scene.next(None);
    renderer.set_scene(&scene, selected);
    assert_eq!(scene.objects()[0].mesh.vertices()[0][3..], [0.9, 0.25, 0.45]);
    renderer
}

fn draw(renderer: &Renderer, view: &wgpu::TextureView) {
    let camera = viewer_journey::camera::Camera::default();
    renderer.draw(view, &viewer_journey::background::Background::default(), &camera.uniform());
}

fn verify_pixels(pixels: &[u8]) {
    let colour = |x: usize, y: usize, expected: [u8; 3]| {
        let offset = (y * 640 + x) * 4;
        pixels[offset..offset + 3].iter().zip(expected)
            .all(|(&actual, wanted)| actual.abs_diff(wanted) <= 2)
    };
    assert!(colour(320, 240, [255, 225, 63]), "Selected object uses its display tint");
    assert!(colour(440, 240, [63, 218, 218]), "Unselected object keeps its colour");
}
