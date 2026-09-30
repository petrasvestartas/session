fn make_renderer(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Renderer {
    use viewer_journey::{history::History, scene::Scene};
    let mut scene = Scene::demo();
    let mut renderer = Renderer::new(device, queue, format, &scene);
    History::default().try_edit(&mut scene, |scene| scene.add_box().map(|_| ())).unwrap();
    renderer.set_scene(&scene, None);
    assert_eq!(scene.objects().len(), 3);
    renderer
}

fn draw(renderer: &Renderer, view: &wgpu::TextureView) {
    let mut camera = viewer_journey::camera::Camera::default();
    camera.isometric();
    renderer.draw(view, &viewer_journey::background::Background::default(), &camera.uniform());
}

fn verify_pixels(pixels: &[u8]) {
    let mut camera = viewer_journey::camera::Camera::default();
    camera.isometric();
    let point = camera.view_projection().transform_point(&session_rust::Point::new(0.9, 0.0, 0.4));
    let x = ((point[0] + 1.0) * 320.0) as usize;
    let y = ((1.0 - point[1]) * 240.0) as usize;
    let offset = (y * 640 + x) * 4;
    assert!(pixels[offset..offset + 3].iter().all(|&value| value.abs_diff(211) <= 2), "Box must occupy its projected position");
    let grey = pixels.chunks_exact(4).filter(|pixel| pixel[..3].iter().all(|&value| value.abs_diff(211) <= 2)).count();
    assert!(grey > 1_000, "The solid must cover a visible area: {grey}");
}
