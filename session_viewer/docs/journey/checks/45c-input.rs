fn editor() -> viewer_journey::editor::Editor {
    use viewer_journey::editor::{Action, Editor};
    let mut editor = Editor::default(); editor.apply(Action::Close).unwrap();
    for action in [Action::AddBox, Action::SelectNext, Action::Translate([-2., 0., 0.]), Action::AddBox, Action::Isometric, Action::Fit, Action::SelectAll] {
        editor.apply(action).unwrap();
    }
    assert_eq!(editor.selected_objects().len(), 2); editor
}

fn make_renderer(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Renderer {
    let editor = editor(); let mut renderer = Renderer::new(device, queue, format, &editor.scene);
    let mut math_quality=viewer_journey::math_view::Presentation::default();
    math_quality.drain_matrix(&mut renderer,&editor.scene,&editor.camera.uniform(),[WIDTH,HEIGHT],editor.selected).expect("Native mathematical display must meet the requested physical-pixel quality before capture");
    let math_scene=math_quality.scene(&editor.scene);
    renderer.select_objects(&math_scene, &editor.selected_objects()).unwrap(); renderer
}

fn draw(renderer: &Renderer, view: &wgpu::TextureView) {
    let editor = editor(); renderer.draw(view, &editor.background, &editor.camera.uniform());
}

fn verify_pixels(pixels: &[u8]) {
    for side in [0, 1] {
        let gold = pixels.chunks_exact(4).enumerate().filter(|(i, p)| (*i % 640) / 320 == side && p[0] > p[1].saturating_add(10) && p[1] > p[2].saturating_add(50)).count();
        assert!(gold > 1000, "Both original owners are visibly selected: side={side}, area={gold}");
    }
}
