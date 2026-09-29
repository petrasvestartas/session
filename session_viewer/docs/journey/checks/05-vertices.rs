fn draw(renderer: &Renderer, view: &wgpu::TextureView) {
    renderer.draw(view, &viewer_journey::background::Background::default());
}

fn verify_pixels(pixels: &[u8]) {
    let pink = |x: usize, y: usize| {
        let offset = (y * 640 + x) * 4;
        pixels[offset..offset + 3].iter().zip([243u8, 137, 179])
            .all(|(&actual, expected)| actual.abs_diff(expected) <= 2)
    };
    assert!(pink(160, 180) && pink(480, 180) && pink(320, 340));
    assert!(!pink(120, 240) && !pink(520, 240) && !pink(320, 380));
    let coverage = (0..480).flat_map(|y| (0..640).map(move |x| (x, y)))
        .filter(|&(x, y)| pink(x, y)).count();
    assert_eq!(coverage, 384 * 264, "Both triangles must fill the rectangle without a gap");
}
