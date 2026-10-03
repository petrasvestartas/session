fn editor() -> viewer_journey::editor::Editor {
    use viewer_journey::editor::{Action, Editor};
    let mut editor = Editor::default(); let bytes = viewer_journey::specimen::precise_bytes();
    editor.apply(Action::ReplaceAt(bytes.clone(), std::rc::Rc::new(viewer_journey::reload_url::ReloadUrl::new("test:auto-frame".into())))).unwrap();
    for action in [Action::SelectNext, Action::Translate([0.35, 0.0, 0.25]), Action::Isometric, Action::Fit, Action::UnloadSources] { editor.apply(action).unwrap(); }
    let intent = viewer_journey::edit_intent::Intent::Move { id: editor.selected.unwrap(), offset: [0.25, 0.0, 0.15] };
    let reply = viewer_journey::reload_reply::Reply::new(intent.keys(&editor).unwrap(), Some(intent), Ok(vec![bytes]));
    assert!(matches!(reply.complete(&mut editor).unwrap(), Some(viewer_journey::edit_replay::Reply::Changed(_))));
    editor.apply(Action::Fit).unwrap();
    editor.apply(Action::Orbit(0.0, std::f64::consts::FRAC_PI_6)).unwrap();
    editor.apply(Action::Translate([0.0, -0.5, 0.0])).unwrap();
    editor.apply(Action::Fit).unwrap();
    editor
}

fn make_renderer(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Renderer {
    let editor = editor();
    prove_buffer_accounting(&device, &editor.scene);
    prove_close(&device, &queue, format);
    prove_unload(&device, &queue, format);
    prove_hydrate(&device, &queue, format);
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
        .map(|row| Rc::downgrade(row.geometry().unwrap())).collect();
    let documents: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter())
        .filter_map(|row| row.source().map(|source| Rc::downgrade(&source.document))).collect();
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
fn prove_unload(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) {
    use std::rc::Rc;
    use viewer_journey::{editor::{Action, Editor}, reload_url::ReloadUrl};
    let mut editor = Editor::default();
    let url = Rc::new(ReloadUrl::new("test:gpu-source".into()));
    editor.apply(Action::ReplaceAt(viewer_journey::specimen::bytes(), url)).unwrap();
    editor.apply(Action::SelectNext).unwrap();
    editor.apply(Action::Translate([0.25, 0.0, 0.0])).unwrap();
    let sources: Vec<_> = editor.scenes().flat_map(|scene| scene.objects()).filter_map(|row|
        row.source().map(|_| Rc::downgrade(row.geometry().unwrap()))).collect();
    let document = Rc::downgrade(&editor.scene.objects()[0].source().unwrap().document);
    let mut renderer = Renderer::new(device.clone(), queue.clone(), format, &editor.scene);
    renderer.set_scene(&editor.scene, editor.selected);
    let gpu: Vec<_> = renderer.geometry_owners().map(Rc::downgrade).collect();
    let displays: Vec<_> = renderer.geometry_owners().map(|g| Rc::downgrade(&g.source)).collect();
    let counters = renderer.stats(); let usage = renderer.usage();
    editor.apply(Action::UnloadSources).unwrap();
    assert!(document.upgrade().is_none() && sources.iter().all(|source| source.upgrade().is_none()));
    assert!(editor.scene.objects().iter().all(|row| row.geometry().is_none() && row.release_epoch() == Some(1)));
    renderer.set_scene(&editor.scene, editor.selected);
    assert_eq!(renderer.stats(), counters); assert_eq!(renderer.usage(), usage);
    for (geometry, old) in renderer.geometry_owners().zip(&gpu) {
        assert!(Rc::ptr_eq(geometry, &old.upgrade().unwrap()));
    }
    assert!(displays.iter().all(|display| display.upgrade().is_some()));
    editor.apply(Action::Undo).unwrap(); renderer.set_scene(&editor.scene, editor.selected);
    let now = renderer.stats(); assert_eq!(&now[..2], &counters[..2]);
    assert_eq!(now[2], counters[2] + 1); assert_eq!(now[3], counters[3] + 64);
    assert!(document.upgrade().is_none());
}

fn prove_hydrate(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) {
    use std::rc::Rc;
    use viewer_journey::{editor::{Action, Editor}, reload_url::ReloadUrl};
    let mut editor = Editor::default(); let bytes = viewer_journey::specimen::bytes();
    let url = Rc::new(ReloadUrl::new("test:gpu-hydrate".into()));
    editor.apply(Action::ReplaceAt(bytes.clone(), url)).unwrap(); editor.apply(Action::SelectNext).unwrap();
    editor.apply(Action::Translate([0.25, 0.0, 0.0])).unwrap();
    let old_kernel = Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
    let display = Rc::clone(&editor.scene.objects()[0].mesh);
    let mut renderer = Renderer::new(device.clone(), queue.clone(), format, &editor.scene);
    renderer.set_scene(&editor.scene, editor.selected);
    let gpu: Vec<_> = renderer.geometry_owners().map(Rc::downgrade).collect();
    let counters = renderer.stats(); let usage = renderer.usage();
    editor.apply(Action::UnloadSources).unwrap(); assert!(old_kernel.upgrade().is_none());
    let key = editor.reload_keys().pop().unwrap(); assert!(editor.hydrate(vec![(key, bytes)]).unwrap());
    renderer.set_scene(&editor.scene, editor.selected);
    assert_eq!(renderer.stats(), counters); assert_eq!(renderer.usage(), usage);
    assert!(Rc::ptr_eq(&editor.scene.objects()[0].mesh, &display));
    for (current, old) in renderer.geometry_owners().zip(gpu) { assert!(Rc::ptr_eq(current, &old.upgrade().unwrap())); }
    assert!(old_kernel.upgrade().is_none());
    let roots = editor.scenes().count(); editor.apply(Action::Translate([0.1, 0.0, 0.0])).unwrap();
    assert_eq!(editor.scenes().count(), roots + 1);
    renderer.set_scene(&editor.scene, editor.selected);
    let now = renderer.stats(); assert_eq!(&now[..2], &counters[..2]);
    assert_eq!(now[2], counters[2] + 1); assert_eq!(now[3], counters[3] + 64);
}
