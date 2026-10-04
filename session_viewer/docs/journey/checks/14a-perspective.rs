fn make_renderer(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Renderer {
    let scene = viewer_journey::scene::Scene::demo();
    Renderer::new(device, queue, format, &scene)
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
    assert!(colour(320, 240, [63, 218, 218]), "The nearer turquoise surface must own the centre pixel");
    assert!(colour(290, 320, [243, 137, 179]), "The lower pink surface remains visible");
    assert!(colour(30, 30, [255, 255, 255]));
}
