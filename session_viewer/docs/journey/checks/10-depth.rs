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
