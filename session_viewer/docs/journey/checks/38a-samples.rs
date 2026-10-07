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
    editor.apply(Action::Fit).unwrap();
    editor.apply(Action::Orbit(std::f64::consts::FRAC_PI_4, 0.0)).unwrap();
    editor.apply(Action::Translate([0.0, -0.5, 0.0])).unwrap();
    editor.apply(Action::Fit).unwrap();
    editor.apply(Action::Orbit(std::f64::consts::FRAC_PI_4, 0.0)).unwrap();
    editor.apply(Action::Fit).unwrap();
    editor.apply(Action::Isometric).unwrap();
    editor.apply(Action::Orbit(std::f64::consts::FRAC_PI_4, 0.0)).unwrap();
    editor.apply(Action::Fit).unwrap();
    editor.apply(Action::Translate([0.1, 0.0, 0.0])).unwrap();
    editor.apply(Action::Fit).unwrap();
    editor.apply(Action::Orbit(0.0, std::f64::consts::FRAC_PI_6)).unwrap();
    editor.apply(Action::Fit).unwrap();
    editor.apply(Action::Orbit(std::f64::consts::FRAC_PI_4, 0.0)).unwrap();
    editor.apply(Action::Fit).unwrap();
    editor.apply(Action::Orbit(0.0, std::f64::consts::FRAC_PI_6)).unwrap();
    editor.apply(Action::Fit).unwrap();
    editor.apply(Action::Orbit(std::f64::consts::FRAC_PI_4, 0.0)).unwrap();
    editor.apply(Action::Fit).unwrap();
    editor.apply(Action::Translate([0.05, 0.0, 0.0])).unwrap();
    editor.apply(Action::Fit).unwrap();
    editor.apply(Action::Translate([0.0, -0.05, 0.0])).unwrap();
    editor.apply(Action::Fit).unwrap();
    editor.apply(Action::Translate([0.05, 0.0, 0.0])).unwrap();
    editor.apply(Action::Fit).unwrap();
    editor.apply(Action::Translate([0.0, -0.05, 0.0])).unwrap();
    editor.apply(Action::Fit).unwrap();
    editor.apply(Action::Translate([0.05, 0.0, 0.0])).unwrap();
    editor.apply(Action::Fit).unwrap();
    editor.apply(Action::Translate([0.0, -0.05, 0.0])).unwrap();
    editor.apply(Action::Fit).unwrap();
    editor.apply(Action::Translate([0.05, 0.0, 0.0])).unwrap();
    editor.apply(Action::Fit).unwrap();
    editor.apply(Action::Translate([0.0, -0.05, 0.0])).unwrap();
    editor.apply(Action::Fit).unwrap();
    editor.apply(Action::Translate([0.05, 0.0, 0.0])).unwrap();
    editor.apply(Action::Fit).unwrap();
    editor.apply(Action::Translate([0.0, -0.05, 0.0])).unwrap();
    editor.apply(Action::Fit).unwrap();
    editor.apply(Action::Translate([0.05, 0.0, 0.0])).unwrap();
    editor.apply(Action::Fit).unwrap();
    editor.apply(Action::Translate([0.0, -0.05, 0.0])).unwrap();
    editor.apply(Action::Fit).unwrap();
    editor.apply(Action::Translate([0.05, 0.0, 0.0])).unwrap();
    editor.apply(Action::Fit).unwrap();
    editor.apply(Action::Translate([0.0, -0.05, 0.0])).unwrap();
    editor.apply(Action::Fit).unwrap();
    let mut line = session_rust::Line::new(-0.35, -0.4, 0.55, 0.35, -0.4, 0.55); line.width = 5.0;
    editor.scene.insert_line(viewer_journey::stroke::PreparedLine::new(std::rc::Rc::new(line)).unwrap()).unwrap();
    let mut path = session_rust::Polyline::new(vec![session_rust::Point::new(-0.6, -0.5, 0.65), session_rust::Point::new(0.0, -0.5, 0.9), session_rust::Point::new(0.6, -0.5, 0.65)]);
    path.width = 5.0; path.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
    editor.scene.insert_path(viewer_journey::chain::PreparedChain::polyline(std::rc::Rc::new(path)).unwrap()).unwrap();
    let mut point = session_rust::Point::new(0.0, -0.5, 0.9); point.width = 12.0;
    point.pointcolor = session_rust::Color::new(0.1, 0.3, 0.8, 1.0);
    editor.scene.insert_point(viewer_journey::marker::PreparedPoint::new(std::rc::Rc::new(point)).unwrap()).unwrap();
    editor
}

fn make_renderer(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Renderer {
    let editor = editor();
    let marker_scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
    let _marker = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("marker validation"), source: wgpu::ShaderSource::Wgsl(viewer_journey::marker::SHADER.into()),
    });
    assert!(pollster::block_on(marker_scope.pop()).is_none(), "Actual marker shader must validate");
    let chain_scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
    let _chain = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("connected stroke validation"), source: wgpu::ShaderSource::Wgsl(viewer_journey::chain::SHADER.into()),
    });
    assert!(pollster::block_on(chain_scope.pop()).is_none(), "Connected shader must validate on the actual drawing device");
    let scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
    let _lane = viewer_journey::stroke_gpu::Lane::new(&device, format);
    let _shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("stroke extrusion check"), source: wgpu::ShaderSource::Wgsl(viewer_journey::stroke::SHADER.into()),
    });
    assert!(pollster::block_on(scope.pop()).is_none(), "The actual drawing device accepts the stroke shader");
    prove_connected_close(&device, &queue, format);
    prove_point_close(&device, &queue, format);
    prove_marker_pixels(&device, &queue, format);
    prove_connected_pixels(&device, &queue, format);
    prove_line_close(&device, &queue, format);
    prove_stroke_pixels(&device, &queue, format);
    prove_document_owner(&device, &queue, format);
    prove_upload_bytes(&device, &queue, format);
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

fn prove_upload_bytes(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) {
    use viewer_journey::editor::{Action, Editor};
    let mut editor = Editor::default();
    let mut renderer = Renderer::new(device.clone(), queue.clone(), format, &editor.scene);
    let expected: u64 = editor.scene.objects().iter().map(|object|
        (object.mesh.vertices().len() * 24 + object.mesh.indices().len() * 2 + 80) as u64).sum();
    assert_eq!(renderer.uploaded_bytes(), expected);
    renderer.set_scene(&editor.scene, editor.selected);
    assert_eq!(renderer.uploaded_bytes(), expected);
    editor.apply(Action::SelectNext).unwrap(); renderer.set_scene(&editor.scene, editor.selected);
    assert_eq!(renderer.uploaded_bytes(), expected + 16);
    editor.apply(Action::Translate([0.5, 0.0, 0.0])).unwrap(); renderer.set_scene(&editor.scene, editor.selected);
    assert_eq!(renderer.uploaded_bytes(), expected + 80);
    editor.apply(Action::Undo).unwrap(); renderer.set_scene(&editor.scene, editor.selected);
    assert_eq!(renderer.uploaded_bytes(), expected + 144);
}

fn prove_document_owner(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) {
    use viewer_journey::{drawing_document, editor::Action};
    let owner = drawing_document::new();
    let weak = std::rc::Rc::downgrade(&owner);
    let (camera, selected, placement, roots) = {
        let mut editor = owner.borrow_mut();
        editor.apply(Action::Replace(viewer_journey::specimen::precise_bytes())).unwrap();
        editor.apply(Action::SelectNext).unwrap(); editor.apply(Action::Translate([0.25, 0.0, 0.0])).unwrap();
        editor.apply(Action::Orbit(0.2, 0.1)).unwrap();
        (editor.camera.uniform(), editor.selected, editor.scene.objects()[0].model.m, editor.scenes().count())
    };
    let renderer = Renderer::new(device.clone(), queue.clone(), format, &owner.borrow().scene);
    let gpu: Vec<_> = renderer.geometry_owners().map(std::rc::Rc::downgrade).collect();
    let retained = std::rc::Rc::clone(&owner); drop(renderer); drop(owner);
    assert!(weak.upgrade().is_some()); assert!(gpu.iter().all(|owner| owner.upgrade().is_none()));
    let mut editor = retained.borrow_mut();
    let mut replacement = Renderer::new(device.clone(), queue.clone(), format, &editor.scene);
    replacement.set_scene(&editor.scene, editor.selected);
    assert_eq!(editor.camera.uniform(), camera); assert_eq!(editor.selected, selected);
    assert_eq!(editor.scene.objects()[0].model.m, placement); assert_eq!(editor.scenes().count(), roots);
    editor.apply(Action::Undo).unwrap(); assert_ne!(editor.scene.objects()[0].model.m, placement);
    editor.apply(Action::Redo).unwrap(); assert_eq!(editor.scene.objects()[0].model.m, placement);
    drop(editor); drop(replacement); drop(retained); assert!(weak.upgrade().is_none());
}

fn prove_stroke_pixels(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) {
    use viewer_journey::{camera::{Camera, Projection}, stroke::Stroke, stroke_gpu::Lane};
    let mut lane = Lane::new(device, format);
    for projection in [Projection::Perspective, Projection::Orthographic] {
        for zoom in [1.0, 2.0] {
            for density in [1, 2] {
                let size = [640 * density, 480 * density];
                for width in [1.0, 3.0, 9.0] {
                    let stroke = Stroke { start: [-0.7, 0.0, 0.0], end: [0.7, 0.0, 0.0],
                        colour: [0.0, 0.0, 0.0, 1.0], width };
                    lane.set(device, [stroke]).unwrap(); let uploads = lane.uploads;
                    lane.set(device, [stroke]).unwrap(); assert_eq!(lane.uploads, uploads);
                    assert_eq!(lane.usage(), [1, 44]);
                    let mut camera = Camera::default(); camera.projection = projection; camera.zoom(zoom);
                    lane.view(queue, &camera.uniform(), size, density as f32);
                    let pixels = stroke_pixels(device, queue, format, size, &lane);
                    let x = size[0] as usize / 2; let mid = size[1] as usize / 2;
                    let mut coverage = 0.0;
                    for y in mid - 24..mid + 24 {
                        let value = pixels[(y * size[0] as usize + x) * 4] as f64 / 255.0;
                        let linear = if value <= 0.04045 { value / 12.92 } else { ((value + 0.055) / 1.055).powf(2.4) };
                        coverage += 1.0 - linear;
                    }
                    assert!((coverage - (width * density as f32) as f64).abs() < 0.08,
                        "Pen width changes with camera or density: {projection:?}, {zoom}, {density}, {width}, {coverage}");
                    assert_eq!(lane.uploads, uploads, "View updates reuse stroke vertices");
                }
            }
        }
    }
    let usage = lane.usage(); let uploads = lane.uploads;
    let invalid = Stroke { start: [f32::NAN; 3], end: [0.0; 3], colour: [0.0; 4], width: 1.0 };
    assert!(lane.set(device, [invalid]).is_err()); assert_eq!(lane.usage(), usage); assert_eq!(lane.uploads, uploads);
    lane.set(device, []).unwrap(); assert_eq!(lane.usage(), [0, 0]);
    assert_eq!(lane.uploaded_bytes, uploads * 44, "Every initialized payload is counted without padding");
}

fn stroke_pixels(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat,
    size: [u32; 2], lane: &viewer_journey::stroke_gpu::Lane,
) -> Vec<u8> {
    let texture = device.create_texture(&wgpu::TextureDescriptor { label: Some("stroke width check"),
        size: wgpu::Extent3d { width: size[0], height: size[1], depth_or_array_layers: 1 },
        mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2, format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC, view_formats: &[] });
    let view = texture.create_view(&Default::default());
    let depth = device.create_texture(&wgpu::TextureDescriptor { label: Some("stroke test depth"),
        size: texture.size(), mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float, usage: wgpu::TextureUsages::RENDER_ATTACHMENT, view_formats: &[] });
    let depth = depth.create_view(&Default::default());
    let mut encoder = device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: &view, depth_slice: None, resolve_target: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::WHITE), store: wgpu::StoreOp::Store } })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &depth,
                depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }), stencil_ops: None }),
            ..Default::default()
        });
        lane.draw(&mut pass);
    }
    let row = size[0] * 4;
    let readback = device.create_buffer(&wgpu::BufferDescriptor { label: Some("stroke pixels"), size: (row * size[1]) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
    encoder.copy_texture_to_buffer(texture.as_image_copy(), wgpu::TexelCopyBufferInfo { buffer: &readback,
        layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(row), rows_per_image: Some(size[1]) } }, texture.size());
    queue.submit([encoder.finish()]);
    let (send, receive) = std::sync::mpsc::channel();
    readback.slice(..).map_async(wgpu::MapMode::Read, move |result| send.send(result).unwrap());
    device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None }).unwrap(); receive.recv().unwrap().unwrap();
    readback.slice(..).get_mapped_range().to_vec()
}

fn prove_line_close(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) {
    use viewer_journey::{editor::{Action, Editor}, stroke::PreparedLine, memory};
    let mut editor = Editor::default();
    let source = std::rc::Rc::new(session_rust::Line::new(-0.5, 0.0, 0.0, 0.5, 0.0, 0.0));
    let weak = std::rc::Rc::downgrade(&source);
    editor.scene.insert_line(PreparedLine::new(source).unwrap()).unwrap();
    editor.apply(Action::AddBox).unwrap();
    let mut renderer = Renderer::new(device.clone(), queue.clone(), format, &editor.scene);
    assert_eq!(renderer.stroke_usage(), [1, 44, 1, 44]);
    let usage = memory::lines(editor.scenes()); assert_eq!(&usage[..2], &[2, 1]); assert!(usage[2] > 0);
    let before = renderer.stroke_usage(); renderer.set_scene(&editor.scene, editor.selected);
    assert_eq!(renderer.stroke_usage(), before);
    editor.apply(Action::Zoom(2.0)).unwrap(); renderer.set_scene(&editor.scene, editor.selected);
    assert_eq!(renderer.stroke_usage(), before);
    editor.apply(Action::Close).unwrap(); assert!(weak.upgrade().is_none());
    renderer.set_scene(&editor.scene, editor.selected);
    assert_eq!(renderer.stroke_usage(), [0, 0, 1, 44]); assert_eq!(memory::lines(editor.scenes()), [0, 0, 0]);
}

fn prove_connected_pixels(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) {
    use viewer_journey::{chain::PreparedChain, stroke_gpu::Lane, camera::{Camera, Projection}};
    use session_rust::{Point, Polyline, Line, Arrowhead, Color};
    use std::rc::Rc;
    let mut lane = Lane::joined(device, format);
    for projection in [Projection::Perspective, Projection::Orthographic] {
        for density in [1, 2] {
            let size = [640 * density, 480 * density]; let mut camera = Camera::default(); camera.projection = projection;
            let mut path = Polyline::new(vec![Point::new(-0.7, -0.5, 0.0), Point::new(0.0, 0.0, 0.0), Point::new(0.7, -0.5, 0.0)]);
            path.width = 6.0; path.linecolor = Color::new(0.0, 0.0, 0.0, 0.5);
            let chain = PreparedChain::polyline(Rc::new(path)).unwrap(); let segments = chain.segments();
            lane.set_chains(device, segments.clone()).unwrap(); let uploads = lane.uploads;
            assert_eq!(lane.usage(), [2, 144]); lane.set_chains(device, segments.clone()).unwrap(); assert_eq!(lane.uploads, uploads);
            lane.view(queue, &camera.uniform(), size, density as f32);
            let pixels = stroke_pixels(device, queue, format, size, &lane);
            let x = size[0] as usize / 2; let y = size[1] as usize / 2;
            let sample = pixels[(y * size[0] as usize + x) * 4] as f64 / 255.0;
            let linear = ((sample + 0.055) / 1.055).powf(2.4);
            assert!((linear - 0.5).abs() < 0.025, "Join must neither leave a gap nor blend twice: {projection:?}, {density}, {linear}");
            let mut invalid = segments[0]; invalid.previous[0] = f32::NAN;
            assert!(lane.set_chains(device, [invalid]).is_err()); assert_eq!(lane.usage(), [2, 144]); assert_eq!(lane.uploads, uploads);
            let mut areas = Vec::new();
            for heads in [Arrowhead::NONE, Arrowhead::START, Arrowhead::END, Arrowhead::BOTH] {
                let mut line = Line::new(-0.6, 0.0, 0.0, 0.6, 0.0, 0.0); line.width = 5.0; line.arrowhead = heads;
                line.linecolor = Color::new(0.0, 0.0, 0.0, 1.0);
                lane.set_chains(device, PreparedChain::line(Rc::new(line)).unwrap().segments()).unwrap();
                lane.view(queue, &camera.uniform(), size, density as f32);
                let pixels = stroke_pixels(device, queue, format, size, &lane);
                areas.push(pixels.chunks_exact(4).filter(|p| p[0] < 100).count());
            }
            assert!(areas[1] > areas[0] && areas[2] > areas[0] && areas[3] > areas[1] && areas[3] > areas[2], "Every flagged head adds real triangular area: {areas:?}");
            lane.set_chains(device, []).unwrap(); assert_eq!(lane.usage(), [0, 0]);
        }
    }
}

fn prove_connected_close(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) {
    use viewer_journey::{editor::{Action, Editor}, chain::PreparedChain, memory};
    use session_rust::{Polyline, Point};
    let mut editor = Editor::default(); editor.apply(Action::Close).unwrap();
    let source = std::rc::Rc::new(Polyline::new(vec![Point::new(-0.6, 0.0, 0.0), Point::new(0.0, 0.0, 0.3), Point::new(0.6, 0.0, 0.0)]));
    let weak = std::rc::Rc::downgrade(&source);
    editor.scene.insert_path(PreparedChain::polyline(source).unwrap()).unwrap(); editor.apply(Action::AddBox).unwrap();
    let mut renderer = Renderer::new(device.clone(), queue.clone(), format, &editor.scene);
    assert_eq!(renderer.path_usage(), [2, 144, 1, 144]);
    assert_eq!(&memory::paths(editor.scenes())[..3], &[2, 1, 1]);
    renderer.set_scene(&editor.scene, editor.selected); let before = renderer.path_usage();
    editor.apply(Action::Zoom(2.0)).unwrap(); renderer.set_scene(&editor.scene, editor.selected); assert_eq!(renderer.path_usage(), before);
    editor.apply(Action::Close).unwrap(); assert!(weak.upgrade().is_none());
    renderer.set_scene(&editor.scene, editor.selected); assert_eq!(renderer.path_usage(), [0, 0, 1, 144]);
    assert_eq!(memory::paths(editor.scenes()), [0; 5]);
}

fn prove_marker_pixels(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) {
    use viewer_journey::{stroke_gpu::Lane, marker::Marker, camera::{Camera, Projection}};
    let mut lane = Lane::markers(device, format);
    for projection in [Projection::Perspective, Projection::Orthographic] {
        for zoom in [1.0, 2.0] {
            for density in [1, 2] {
                let size = [640 * density, 480 * density]; let mut camera = Camera::default(); camera.projection = projection; camera.zoom(zoom);
                for diameter in [6.0, 12.0] {
                    let marker = Marker { center: [0.0; 3], diameter, colour: [0.0, 0.0, 0.0, 0.5] };
                    lane.set_points(device, [marker]).unwrap(); let uploads = lane.uploads;
                    lane.set_points(device, [marker]).unwrap(); assert_eq!(lane.uploads, uploads); assert_eq!(lane.usage(), [1, 32]);
                    lane.view(queue, &camera.uniform(), size, density as f32);
                    let pixels = stroke_pixels(device, queue, format, size, &lane);
                    let linear = |v: u8| { let v = v as f64 / 255.0; if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) } };
                    let x = size[0] as usize / 2; let y = size[1] as usize / 2;
                    let interior = linear(pixels[(y * size[0] as usize + x) * 4]);
                    let area: f64 = pixels.chunks_exact(4).map(|p| (1.0 - linear(p[0])) / (1.0 - interior)).sum();
                    let radius = diameter as f64 * density as f64 * 0.5;
                    for row in 0..size[1] as usize {
                        for col in 0..size[0] as usize {
                            let dx = col as f64 + 0.5 - x as f64;
                            let dy = row as f64 + 0.5 - y as f64;
                            let coverage = (radius + 0.5 - dx.hypot(dy)).clamp(0.0, 1.0);
                            let expected = 1.0 - coverage * 0.5;
                            let encoded = if expected <= 0.0031308 { expected * 12.92 } else { 1.055 * expected.powf(1.0 / 2.4) - 0.055 };
                            let expected_byte = (encoded * 255.0).round() as i16;
                            let actual = pixels[(row * size[0] as usize + col) * 4] as i16;
                            assert!((actual - expected_byte).abs() <= 1, "Disc pixel differs from analytic coverage: {projection:?}, {zoom}, {density}, {diameter}, {col}, {row}, {actual}, {expected_byte}");
                        }
                    }
                    assert!((area - std::f64::consts::PI * radius * radius).abs() < 1.5,
                        "Disc area changes with camera or density: {projection:?}, {zoom}, {density}, {diameter}, {area}");
                    assert!((interior - 0.5).abs() < 0.025, "Disc interior blends once");
                    assert_eq!(lane.uploads, uploads);
                }
            }
        }
    }
    let before = lane.usage(); let uploads = lane.uploads;
    let invalid = Marker { center: [f32::NAN; 3], diameter: 6.0, colour: [0.0; 4] };
    assert!(lane.set_points(device, [invalid]).is_err()); assert_eq!(lane.usage(), before); assert_eq!(lane.uploads, uploads);
    lane.set_points(device, []).unwrap(); assert_eq!(lane.usage(), [0, 0]); assert_eq!(lane.uploaded_bytes, uploads * 32);
}

fn prove_point_close(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) {
    use viewer_journey::{editor::{Action, Editor}, marker::PreparedPoint, memory};
    let mut editor = Editor::default(); editor.apply(Action::Close).unwrap();
    let source = std::rc::Rc::new(session_rust::Point::new(0.0, 0.0, 0.0)); let weak = std::rc::Rc::downgrade(&source);
    editor.scene.insert_point(PreparedPoint::new(source).unwrap()).unwrap(); editor.apply(Action::AddBox).unwrap();
    let mut renderer = Renderer::new(device.clone(), queue.clone(), format, &editor.scene);
    assert_eq!(renderer.point_usage(), [1, 32, 1, 32]); assert_eq!(&memory::points(editor.scenes())[..2], &[2, 1]);
    renderer.set_scene(&editor.scene, editor.selected); let before = renderer.point_usage();
    editor.apply(Action::Zoom(2.0)).unwrap(); renderer.set_scene(&editor.scene, editor.selected); assert_eq!(renderer.point_usage(), before);
    editor.apply(Action::Close).unwrap(); assert!(weak.upgrade().is_none());
    renderer.set_scene(&editor.scene, editor.selected); assert_eq!(renderer.point_usage(), [0, 0, 1, 32]); assert_eq!(memory::points(editor.scenes()), [0; 3]);
}
