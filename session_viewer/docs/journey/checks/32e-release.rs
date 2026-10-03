fn editor() -> viewer_journey::editor::Editor {
    use viewer_journey::editor::{Action, Editor};
    let mut editor = Editor::default();
    let bytes = viewer_journey::specimen::bytes();
    for action in [Action::Import(bytes.clone()), Action::Replace(bytes), Action::SelectNext, Action::SelectNext, Action::Translate([0.35, 0.0, 0.25]), Action::Isometric, Action::Orbit(std::f64::consts::FRAC_PI_4, 0.0), Action::Orbit(0.0, std::f64::consts::FRAC_PI_6), Action::Translate([0.36, 0.0, 0.15]), Action::Fit] { editor.apply(action).unwrap(); }
    editor
}

fn make_renderer(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Renderer {
    let editor = editor();
    prove_buffer_accounting(&device, &editor.scene);
    prove_close(&device, &queue, format);
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

fn prove_buffer_accounting(device: &wgpu::Device, scene: &viewer_journey::scene::Scene) {
    use std::rc::Rc;
    use viewer_journey::{gpu_geometry::GpuGeometry, gpu_mesh::GpuMesh, memory, mesh::Mesh};
    let source = Rc::new(Mesh::new(vec![[0.0; 6]; 3], vec![0, 1, 2]).unwrap());
    let geometry = Rc::new(GpuGeometry::upload(device, source));
    let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("buffer accounting proof"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0, visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false, min_binding_size: std::num::NonZeroU64::new(80) },
            count: None,
        }],
    });
    let a = GpuMesh::with_geometry(device, &layout, &scene.objects()[0], false, Rc::clone(&geometry));
    let b = GpuMesh::with_geometry(device, &layout, &scene.objects()[1], true, geometry);
    assert_eq!(memory::gpu([&a, &b]), [1, 80, 2, 160], "Shared geometry and padded index size are counted once");
}

fn prove_close(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) {
    use std::{collections::HashSet, rc::Rc};
    use viewer_journey::{editor::{Action, Editor}, memory};
    let mut editor = Editor::default();
    let bytes = viewer_journey::specimen::bytes();
    editor.apply(Action::Import(bytes.clone())).unwrap();
    editor.apply(Action::Replace(bytes.clone())).unwrap();
    editor.apply(Action::AddBox).unwrap(); editor.apply(Action::Undo).unwrap();
    let mut seen = HashSet::new();
    let displays: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter())
        .filter_map(|row| seen.insert(Rc::as_ptr(&row.mesh)).then(|| Rc::downgrade(&row.mesh))).collect();
    let sources: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter())
        .map(|row| Rc::downgrade(&row.geometry)).collect();
    let documents: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter())
        .filter_map(|row| row.source.as_ref().map(|source| Rc::downgrade(&source.document))).collect();
    let mut renderer = Renderer::new(device.clone(), queue.clone(), format, &editor.scene);
    let gpu: Vec<_> = renderer.geometry_owners().map(Rc::downgrade).collect();
    let counters = renderer.stats();
    assert_eq!(renderer.usage()[0], 3); assert_eq!(renderer.usage()[2], 3);
    editor.apply(Action::Close).unwrap();
    assert!(sources.iter().all(|owner| owner.upgrade().is_none()));
    assert!(documents.iter().all(|owner| owner.upgrade().is_none()));
    assert_eq!(displays.iter().filter(|owner| owner.strong_count() > 0).count(), 3,
        "Only GPU-held displays remain between editor close and renderer synchronization");
    let cpu = memory::cpu(editor.scenes(), renderer.geometry_owners().map(|owner| &owner.source));
    assert_eq!(&cpu[..4], &[0, 0, 0, 3]); assert!(cpu[4] > 0);
    renderer.set_scene(&editor.scene, editor.selected);
    assert_eq!(renderer.usage(), [0; 4]); assert_eq!(renderer.stats(), counters);
    assert!(displays.iter().all(|owner| owner.upgrade().is_none()));
    assert!(gpu.iter().all(|owner| owner.upgrade().is_none()));
    assert_eq!(memory::cpu(editor.scenes(), renderer.geometry_owners().map(|owner| &owner.source)), [0, 0, 0, 0, 0, 1]);
    editor.apply(Action::Import(bytes)).unwrap();
    renderer.set_scene(&editor.scene, editor.selected);
    assert_eq!(renderer.usage()[0], 3); assert_eq!(renderer.usage()[2], 3);
    assert_eq!(renderer.stats()[0], counters[0] + 3);
}
