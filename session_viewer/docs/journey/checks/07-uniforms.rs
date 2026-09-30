fn draw(renderer: &Renderer, view: &wgpu::TextureView) {
    renderer.draw(view, &viewer_journey::background::Background::default(), &[0.75, 0.75, 0.25, 0.15]);
}

fn verify_pixels(pixels: &[u8]) {
    let pink = |x: usize, y: usize| {
        let offset = (y * 640 + x) * 4;
        pixels[offset..offset + 3].iter().zip([243u8, 137, 179])
            .all(|(&actual, expected)| actual.abs_diff(expected) <= 2)
    };
    assert!(pink(400, 204) && pink(520, 204) && pink(400, 110));
    assert!(!pink(160, 240) && !pink(320, 350), "The old larger diamond must be gone");
    let coverage = (0..480).flat_map(|y| (0..640).map(move |x| (x, y)))
        .filter(|&(x, y)| pink(x, y)).count();
    assert!((30_800..31_400).contains(&coverage), "Scale must affect both axes: {coverage}");
}
