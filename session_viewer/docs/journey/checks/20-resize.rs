fn make_renderer(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Renderer {
    use viewer_journey::{history::History, scene::Scene};
    let mut scene = Scene::demo();
    let mut renderer = Renderer::new(device, queue, format, &scene);
    History::default().try_edit(&mut scene, |scene| scene.add_box().map(|_| ())).unwrap();
    renderer.set_scene(&scene, None);
    for (width, height) in [(320.0, 240.0), (1.0, 1.0), (770.0, 385.0)] {
        renderer.resize(viewer_journey::viewport::Viewport::from_css(width, height, 1.0, 4096).unwrap());
    }
    assert_eq!(scene.objects().len(), 3);
    renderer
}

fn draw(renderer: &Renderer, view: &wgpu::TextureView) {
    let mut camera = viewer_journey::camera::Camera::default();
    camera.isometric();
    camera.aspect = 2.0;
    renderer.draw(view, &viewer_journey::background::Background::default(), &camera.uniform());
}

fn verify_pixels(pixels: &[u8]) {
    let mut camera = viewer_journey::camera::Camera::default();
    camera.isometric();
    camera.aspect = 2.0;
    let length = 1.52_f64.sqrt();
    let faces = [
        ([0.5, 0.0, 0.4], 0.4 / length),
        ([0.9, -0.4, 0.4], 0.6 / length),
        ([0.9, 0.0, 0.8], 1.0 / length),
    ];
    let mut shades = Vec::new();
    for (position, alignment) in faces {
        let linear = 0.65 * (0.3 + 0.7 * alignment);
        let expected = (255.0 * (1.055 * linear.powf(1.0 / 2.4) - 0.055)).round() as u8;
        let point = session_rust::Point::new(position[0], position[1], position[2]);
        let screen = camera.view_projection().transform_point(&point);
        let x = ((screen[0] + 1.0) * 385.0) as usize;
        let y = ((1.0 - screen[1]) * 192.5) as usize;
        let offset = (y * 770 + x) * 4;
        let colour = &pixels[offset..offset + 3];
        assert!(colour.iter().all(|&value| value.abs_diff(expected) <= 2),
            "Face at {position:?}: expected {expected}, got {colour:?}");
        shades.push(colour[0]);
    }
    assert!(shades[0] + 10 < shades[1] && shades[1] + 10 < shades[2]);
}
