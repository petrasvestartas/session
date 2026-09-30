fn make_renderer(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Renderer {
    let scene = viewer_journey::scene::Scene::demo();
    let mut camera = viewer_journey::camera::Camera::default();
    camera.isometric();
    let ray = camera.ray([0.0, 0.0]).unwrap();
    assert_eq!(viewer_journey::picking::pick(&scene, &ray), Some(scene.objects()[0].id));
    Renderer::new(device, queue, format, &scene)
}

fn draw(renderer: &Renderer, view: &wgpu::TextureView) {
    let mut camera = viewer_journey::camera::Camera::default();
    camera.isometric();
    renderer.draw(view, &viewer_journey::background::Background::default(), &camera.uniform());
}

fn verify_pixels(pixels: &[u8]) {
    let near = |pixel: &[u8], expected: [u8; 3]| pixel.iter().zip(expected)
        .all(|(&actual, wanted)| actual.abs_diff(wanted) <= 2);
    let centre = (240 * 640 + 320) * 4;
    assert!(near(&pixels[centre..centre + 3], [243, 137, 179]), "Orbit must change visible overlap consistently with picking");
    for colour in [[243, 137, 179], [63, 218, 218]] {
        let count = pixels.chunks_exact(4).filter(|pixel| near(&pixel[..3], colour)).count();
        assert!(count > 200, "Both surfaces must remain visible: {count}");
    }
}
