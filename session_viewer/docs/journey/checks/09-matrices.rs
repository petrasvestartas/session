fn draw(renderer: &Renderer, view: &wgpu::TextureView) {
    let mut camera = viewer_journey::camera::Camera::default();
    camera.rotate(std::f32::consts::FRAC_PI_4);
    renderer.draw(view, &viewer_journey::background::Background::default(), &camera.uniform());
}

fn verify_pixels(pixels: &[u8]) {
    let pink = |x: usize, y: usize| {
        let offset = (y * 640 + x) * 4;
        pixels[offset..offset + 3].iter().zip([243u8, 137, 179])
            .all(|(&actual, expected)| actual.abs_diff(expected) <= 2)
    };
    assert!(pink(200, 150) && pink(440, 330), "The rotated diamond must fill its new corners");
    assert!(!pink(160, 240) && !pink(320, 110), "The unrotated tips must be gone");
    let coverage = (0..480).flat_map(|y| (0..640).map(move |x| (x, y)))
        .filter(|&(x, y)| pink(x, y)).count();
    assert!((55_000..56_000).contains(&coverage), "Rotation must preserve area: {coverage}");
}
