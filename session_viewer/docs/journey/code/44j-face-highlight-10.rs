    renderer.set_pick_mode(crate::pick_sources::Mode::Control); renderer.request_pick([0., 0.6], identity()).unwrap(); assert!(finish(&mut renderer).is_none());
}

fn colour_pixels(renderer: &mut Renderer) -> Vec<u8> {
    let texture = renderer.device.create_texture(&wgpu::TextureDescriptor { label: Some("selection pixels"),
        size: wgpu::Extent3d { width: 320, height: 240, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1,
        dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC, view_formats: &[] });
    renderer.draw_mode(&texture.create_view(&Default::default()), &Default::default(), &identity(), false);
    let bytes = renderer.device.create_buffer(&wgpu::BufferDescriptor { label: Some("selection readback"), size: 320 * 240 * 4,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
    let mut encoder = renderer.device.create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(texture.as_image_copy(), wgpu::TexelCopyBufferInfo { buffer: &bytes,
        layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(1280), rows_per_image: Some(240) } }, texture.size());
    renderer.queue.submit([encoder.finish()]); let (send, receive) = std::sync::mpsc::channel();
    bytes.slice(..).map_async(wgpu::MapMode::Read, move |result| { let _ = send.send(result); });
    renderer.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: Some(std::time::Duration::from_secs(5)) }).unwrap();
    receive.recv_timeout(std::time::Duration::from_secs(5)).unwrap().unwrap();
    let result = bytes.slice(..).get_mapped_range().to_vec(); bytes.unmap(); result
}

#[test]
fn actual_part_highlights_are_local_leave_black_edges_and_release_derived_data() {
    use crate::pick_sources::{Part, Target};
    let mut source = session_rust::Mesh::new();
    for (key, p) in [(10, [-0.8, -0.6, 0.5]), (20, [0., -0.6, 0.5]), (30, [0., 0.6, 0.5]), (40, [-0.8, 0.6, 0.5]),
                     (50, [0.8, -0.6, 0.5]), (60, [0.8, 0.6, 0.5])] {
        source.add_vertex(session_rust::Point::new(p[0], p[1], p[2]), Some(key));
    }
    source.add_face(vec![10, 20, 30, 40], Some(37)); source.add_face(vec![20, 50, 60, 30], Some(91));
    let mut scene = Scene::demo(); scene.clear(); let owner = scene.insert(PreparedMesh::new(source).unwrap()).unwrap();
    let weak = Rc::downgrade(&scene.objects()[0].mesh); let mut renderer = renderer(&scene); let stats = renderer.stats();
    let before = colour_pixels(&mut renderer);
    renderer.set_highlight(&scene, Some(Target { owner, part: Part::Face(37) })).unwrap();
    let face = colour_pixels(&mut renderer); assert!(renderer.highlight_bytes() > 0); let mut changed = 0;
    for (index, (a, b)) in before.chunks_exact(4).zip(face.chunks_exact(4)).enumerate() { if a != b {
        let x = index % 320; let y = index / 320; assert!((31..=160).contains(&x) && (47..=192).contains(&y)); changed += 1;
    } }
    assert!(changed > 10000, "The original face, rather than the whole object, is visibly selected");
    renderer.set_highlight(&scene, Some(Target { owner, part: Part::MeshEdge(10, 20) })).unwrap();
    let edge = colour_pixels(&mut renderer); let mut changed = 0; let mut black = 0;
    for (index, (a, b)) in before.chunks_exact(4).zip(edge.chunks_exact(4)).enumerate() {
        let x = index % 320; let y = index / 320;
        if a != b { assert!((30..=161).contains(&x) && (190..=193).contains(&y)); changed += 1; }
        if (35..150).contains(&x) && (191..=192).contains(&y) && b[0] < 160 && b[1] < 140 && b[2] < 80 { black += 1; }
    }
    assert!(changed > 80 && black > 80, "A yellow fringe surrounds the original black edge: changed={changed}, dark={black}");
    assert_eq!(renderer.stats(), stats, "Highlights reuse primary geometry");
    renderer.set_highlight(&scene, None).unwrap(); assert_eq!(renderer.highlight_bytes(), 0); assert_eq!(colour_pixels(&mut renderer), before);
    scene.clear(); renderer.set_scene(&scene, None); assert!(weak.upgrade().is_none()); assert_eq!(renderer.highlight_bytes(), 0);
}

#[test]
fn actual_control_highlight_surrounds_only_the_original_marker_and_keeps_source_precision() {
    let mut editor = crate::editor::Editor::default(); editor.apply(crate::editor::Action::Close).unwrap();
    editor.apply(crate::editor::Action::AddCurve).unwrap(); editor.apply(crate::editor::Action::Controls(true)).unwrap();
    let owner = editor.scene.paths()[0].id; let original = editor.scene.paths()[0].points(); let mut renderer = renderer(&editor.scene);
    let before = colour_pixels(&mut renderer); renderer.set_highlight(&editor.scene, Some(crate::pick_sources::Target {
        owner, part: crate::pick_sources::Part::Control(1) })).unwrap(); let after = colour_pixels(&mut renderer); let mut changed = 0;
    for (index, (a, b)) in before.chunks_exact(4).zip(after.chunks_exact(4)).enumerate() { if a != b {
        let x = index % 320; let y = index / 320; assert!((151..=168).contains(&x) && (39..=56).contains(&y)); changed += 1;
    } }
    assert!(changed > 20, "The halo belongs to the original control rather than a curve sample");
    assert_eq!(editor.scene.paths()[0].points(), original);
    editor.apply(crate::editor::Action::Close).unwrap(); renderer.set_scene(&editor.scene, None); assert_eq!(renderer.highlight_bytes(), 0);
}
