fn make_renderer(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Renderer {
    use viewer_journey::{history::History, scene::Scene};
    let mut scene = Scene::demo();
    let mut history = History::default();
    let mut renderer = Renderer::new(device, queue, format, &scene);
    history.edit(&mut scene, Scene::toggle_extra);
    let extra = scene.objects()[2].id;
    history.edit(&mut scene, |scene| { scene.remove(extra); });
    renderer.set_scene(&scene, None);
    assert!(history.undo(&mut scene));
    assert!(scene.contains(extra));
    renderer.set_scene(&scene, scene.next(None));
    renderer
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
    assert!(colour(100, 100, [124, 231, 149]), "Undo must restore the deleted mesh to the GPU scene");
    assert!(colour(320, 240, [255, 225, 63]), "Selection remains independent of restored geometry");
    assert!(colour(440, 240, [63, 218, 218]));
}
