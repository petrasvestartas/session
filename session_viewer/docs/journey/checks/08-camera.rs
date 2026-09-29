fn draw(renderer: &Renderer, view: &wgpu::TextureView) {
    let mut camera = viewer_journey::camera::Camera::default();
    camera.zoom(0.5);
    renderer.draw(view, &viewer_journey::background::Background::default(), &camera.uniform());
}

fn verify_pixels(pixels: &[u8]) {
    let pink = |x: usize, y: usize| {
        let offset = (y * 640 + x) * 4;
        pixels[offset..offset + 3].iter().zip([243u8, 137, 179])
            .all(|(&actual, expected)| actual.abs_diff(expected) <= 2)
    };
    assert!(pink(320, 240) && pink(240, 240) && pink(320, 180));
    assert!(!pink(210, 240) && !pink(320, 150), "Zoom out must reduce the diamond");
    let coverage = (0..480).flat_map(|y| (0..640).map(move |x| (x, y)))
        .filter(|&(x, y)| pink(x, y)).count();
    assert!((13_500..14_000).contains(&coverage), "Half scale must have quarter area: {coverage}");
}
