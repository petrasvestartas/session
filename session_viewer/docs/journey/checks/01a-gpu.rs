fn draw(renderer: &Renderer, view: &wgpu::TextureView) {
    renderer.draw(view);
}

fn verify_pixels(pixels: &[u8]) {
    assert!(pixels.chunks_exact(4).all(|pixel| pixel == [255, 255, 255, 255]),
        "The first GPU clear must fill every pixel with opaque white");
}
