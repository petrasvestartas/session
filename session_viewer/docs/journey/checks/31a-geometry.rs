fn editor() -> viewer_journey::editor::Editor {
    use viewer_journey::editor::{Action, Editor};
    let mut editor = Editor::default();
    let bytes = viewer_journey::specimen::bytes();
    for action in [Action::Import(bytes.clone()), Action::Replace(bytes), Action::SelectNext, Action::SelectNext, Action::Translate([0.35, 0.0, 0.25]), Action::Isometric, Action::Orbit(std::f64::consts::FRAC_PI_4, 0.0), Action::Orbit(0.0, std::f64::consts::FRAC_PI_6), Action::Fit] { editor.apply(action).unwrap(); }
    editor
}

fn make_renderer(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Renderer {
    let editor = editor();
    prove_shared_rows(&device, &editor.scene);
    let mut renderer = Renderer::new(device, queue, format, &editor.scene);
    renderer.set_scene(&editor.scene, editor.selected);
    renderer
}

fn draw(renderer: &Renderer, view: &wgpu::TextureView) {
    let editor = editor();
    renderer.draw(view, &editor.background, &editor.camera.uniform());
}

fn verify_pixels(pixels: &[u8]) {
    let editor = editor();
    let bounds = editor.scene.selected_bounds(editor.selected.unwrap()).unwrap();
    let mut p = bounds.centre(); p[0] = bounds.max[0];
    let point = session_rust::Point::new(p[0], p[1], p[2]);
    let screen = editor.camera.view_projection().transform_point(&point);
    assert!(screen[0].abs() < 0.95 && screen[1].abs() < 0.95);
    let ray = editor.camera.ray([screen[0] as f32, screen[1] as f32]).unwrap();
    assert_eq!(viewer_journey::picking::pick(&editor.scene, &ray), editor.selected);
    let offset = (((1.0 - screen[1]) * 240.0) as usize * 640 + ((screen[0] + 1.0) * 320.0) as usize) * 4;
    let pixel = &pixels[offset..offset + 3];
    assert!(pixel[0] > pixel[1].saturating_add(10) && pixel[1] > pixel[2].saturating_add(50) && pixel[1] > pixel[2].saturating_mul(3),
        "Source-backed object must draw at its world pick point: {pixel:?}");
    let gold = pixels.chunks_exact(4).filter(|p| p[0] > p[1].saturating_add(10) && p[1] > p[2].saturating_add(50) && p[1] > p[2].saturating_mul(3)).count();
    assert!(gold > 1500, "Selected source-backed object has visible area: {gold}");
}

fn prove_shared_rows(device: &wgpu::Device, scene: &viewer_journey::scene::Scene) {
    use std::rc::Rc;
    use viewer_journey::{gpu_geometry::GpuGeometry, gpu_mesh::GpuMesh};
    let first = &scene.objects()[0];
    let colours = first.mesh.vertices().to_vec();
    let mut second = first.clone();
    second.model = session_rust::Xform::translation(0.4, 0.0, 0.0);
    let geometry = Rc::new(GpuGeometry::upload(device, Rc::clone(&first.mesh)));
    let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("shared geometry proof"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0, visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false, min_binding_size: std::num::NonZeroU64::new(80) },
            count: None,
        }],
    });
    let a = GpuMesh::with_geometry(device, &layout, first, false, Rc::clone(&geometry));
    let b = GpuMesh::with_geometry(device, &layout, &second, true, Rc::clone(&geometry));
    assert!(Rc::ptr_eq(&a.geometry, &b.geometry));
    assert!(Rc::ptr_eq(&a.geometry.source, &first.mesh));
    assert_eq!(Rc::strong_count(&geometry), 3);
    assert_eq!(first.mesh.vertices(), colours);
    let weak = Rc::downgrade(&geometry);
    drop(geometry); drop(a);
    assert_eq!(weak.strong_count(), 1);
    drop(b);
    assert!(weak.upgrade().is_none(), "Dropping the final row releases geometry ownership");
}
