fn make_renderer(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Renderer {
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
