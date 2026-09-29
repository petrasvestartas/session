fn make_renderer(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Renderer {
    let scene = viewer_journey::scene::Scene::demo();
    let camera = viewer_journey::camera::Camera::default();
    let selected = viewer_journey::picking::pick(&scene, camera.world_from_screen([0.4, 0.0]));
    assert_eq!(selected, Some(scene.objects()[1].id));
    let mut renderer = Renderer::new(device, queue, format, &scene);
    renderer.set_scene(&scene, selected);
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
    assert!(colour(440, 240, [255, 225, 63]), "The picked far object must be highlighted");
    assert!(colour(320, 240, [243, 137, 179]), "The near object remains unselected and visible");
}
