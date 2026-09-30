fn draw(renderer: &Renderer, view: &wgpu::TextureView) {
    renderer.draw(view, &viewer_journey::background::Background::default());
}

fn verify_pixels(pixels: &[u8]) {
    let pink = |x: usize, y: usize| {
        let offset = (y * 640 + x) * 4;
        pixels[offset..offset + 3].iter().zip([243u8, 137, 179])
            .all(|(&actual, expected)| actual.abs_diff(expected) <= 2)
    };
    assert!(pink(320, 130) && pink(320, 350), "Both indexed triangles must appear");
    assert!(pink(160, 240) && pink(480, 240), "The shared edge must have no gap");
    assert!(!pink(160, 130) && !pink(480, 350), "The old rectangle must be gone");
    let coverage = (0..480).flat_map(|y| (0..640).map(move |x| (x, y)))
        .filter(|&(x, y)| pink(x, y)).count();
    assert!((55_000..56_000).contains(&coverage), "Diamond area: {coverage}");
}
