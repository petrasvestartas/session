fn make_renderer(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Renderer {
    let mut scene = viewer_journey::scene::Scene::demo();
    let original: Vec<_> = scene.meshes.iter().map(|mesh| (mesh.vertices().to_vec(), mesh.indices().to_vec())).collect();
    assert_eq!(original.len(), 2);
    scene.toggle_extra();
    assert_eq!(scene.meshes.len(), 3);
    for (mesh, (vertices, indices)) in scene.meshes.iter().zip(&original) {
        assert_eq!(mesh.vertices(), vertices);
        assert_eq!(mesh.indices(), indices);
    }
    scene.toggle_extra();
    assert_eq!(scene.meshes.len(), 2);
    for (mesh, (vertices, indices)) in scene.meshes.iter().zip(&original) {
        assert_eq!(mesh.vertices(), vertices);
        assert_eq!(mesh.indices(), indices);
    }
    Renderer::new(device, queue, format)
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
    assert!(colour(320, 240, [243, 137, 179]), "The nearer triangle must survive a later far draw");
    assert!(colour(256, 380, [243, 137, 179]), "Near triangle outside the overlap");
    assert!(colour(440, 240, [63, 218, 218]), "Far triangle outside the overlap");
    assert!(colour(384, 70, [63, 218, 218]), "Far triangle above the near triangle");
    assert!(colour(30, 30, [255, 255, 255]), "Background remains visible");
}
