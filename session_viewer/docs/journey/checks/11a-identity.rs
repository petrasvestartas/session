fn make_renderer(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Renderer {
    let mut scene = viewer_journey::scene::Scene::demo();
    let mut renderer = Renderer::new(device, queue, format, &scene);
    scene.toggle_extra();
    renderer.set_scene(&scene);
    assert_eq!(scene.objects().len(), 3);
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
    assert!(colour(100, 100, [124, 231, 149]), "The third mesh must be uploaded and drawn");
    assert!(colour(320, 240, [243, 137, 179]), "Near mesh retained");
    assert!(colour(440, 240, [63, 218, 218]), "Far mesh retained");
}
