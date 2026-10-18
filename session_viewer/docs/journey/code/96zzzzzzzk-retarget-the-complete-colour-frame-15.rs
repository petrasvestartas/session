use super::*;
use crate::{
    camera::{Camera, Projection, View},
    scene::Scene,
};
use session_rust::{Point, Xform};
use std::rc::Rc;

fn composition() -> (Scene, Vec<(ObjectId, [f64; 3])>) {
    let mut scene = Scene::empty();
    let cad = scene
        .insert_cad(Rc::new(crate::cad_display::example()))
        .unwrap();
    scene.place(cad, Xform::translation(-3., 1.5, 0.)).unwrap();
    let original = crate::surface::example();
    let roof = scene.insert_surface(Rc::new(original.clone())).unwrap();
    let roof_place = Xform::translation(2.5, 1.5, 0.);
    scene.place(roof, roof_place.clone()).unwrap();
    for direction in 0..2 {
        for parameter in [0.25, 0.5, 0.75] {
            let mut curve = original.iso_curve(direction, parameter).unwrap();
            curve.width = 1.5;
            curve.name = format!("Original roof iso {direction} at {parameter}");
            let id = scene
                .insert_path(crate::chain::PreparedChain::curve(Rc::new(curve)).unwrap())
                .unwrap();
            scene.place_path(id, roof_place.clone()).unwrap();
        }
    }
    let mut cube = session_rust::Mesh::create_box(1.2, 1.2, 1.2);
    cube.name = "Original green cube".into();
    cube.set_objectcolor(session_rust::Color::new(0.15, 0.7, 0.25, 1.));
    let cube = scene
        .insert(crate::prepared::PreparedMesh::new(cube).unwrap())
        .unwrap();
    scene.place(cube, Xform::translation(0., 1.8, 0.)).unwrap();
    crate::scene_document::import(
        &mut scene,
        &crate::instance_specimen::display_bytes(),
        "Shared originals",
        &Xform::translation(0., -1.8, 0.8),
        0.,
    )
    .unwrap();
    let cloud = scene
        .insert_cloud(crate::cloud::Prepared::new(Rc::new(crate::cloud::example())).unwrap())
        .unwrap();
    scene
        .place_cloud(cloud, Xform::translation(-3., -2., 0.3))
        .unwrap();
    let sheet = scene
        .insert_sheet(Rc::new(
            crate::sheet_source::Prepared::new(Rc::new(crate::sheet_example::source())).unwrap(),
        ))
        .unwrap();
    scene
        .place_sheet(sheet, Xform::translation(2.5, -2., 0.))
        .unwrap();
    let mut line = session_rust::Line::new(-4.4, -3.5, 0.2, -1.3, -3.5, 0.2);
    line.width = 2.;
    let line = scene
        .insert_line(crate::stroke::PreparedLine::new(Rc::new(line)).unwrap())
        .unwrap();
    let point = scene
        .insert_point(
            crate::marker::PreparedPoint::new(Rc::new(Point::new(4.2, -3.4, 0.3))).unwrap(),
        )
        .unwrap();
    for (text, placement) in [
        (
            "CAD bore",
            crate::text::TextPlacement::WorldBillboard {
                world: [-3., 3.5, 0.8],
                world_height: 0.25,
            },
        ),
        (
            "Original NURBS",
            crate::text::TextPlacement::WorldPlane {
                world: [1.5, 3.1, 0.4],
                right: [1., 0., 0.],
                up: [0., 1., 0.],
                world_height: 0.22,
            },
        ),
        (
            "Drawing",
            crate::text::TextPlacement::Nameplate {
                world: [2.5, -4.2, 0.5],
                padding: [5., 3.],
                rounded: true,
            },
        ),
    ] {
        let mut label = crate::scene_label::example(placement);
        label.label.text = text.into();
        label.label.color = [255; 4];
        scene
            .insert_text(crate::scene_label::Prepared::new(Rc::new(label)).unwrap())
            .unwrap();
    }
    (
        scene,
        vec![
            (cad, [-4.4, 1.5, 0.3]),
            (roof, [2.7, 1.7, 0.2]),
            (cloud, [-3., -2., 0.3]),
            (sheet, [2.5, -2.5, 0.3]),
            (line, [-2.5, -3.5, 0.2]),
            (point, [4.2, -3.4, 0.3]),
            (cube, [0., 1.8, 0.6]),
        ],
    )
}

fn pixels(r: &Renderer, matrix: [f32; 16]) -> Vec<u8> {
    let texture = r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("actual combined original viewer frame"),
        size: wgpu::Extent3d {
            width: 640,
            height: 480,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: r.output_format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[r.format],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor {
        format: Some(r.format),
        ..Default::default()
    });
    r.draw(&view, &Default::default(), &matrix);
    let read = r.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: 2560 * 480,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut encoder = r.device.create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(
        texture.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &read,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(2560),
                rows_per_image: Some(480),
            },
        },
        texture.size(),
    );
    r.queue.submit([encoder.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    read.slice(..)
        .map_async(wgpu::MapMode::Read, move |v| tx.send(v).unwrap());
    crate::cap_gpu_tests::wait(r);
    rx.recv().unwrap().unwrap();
    let pixels = read.slice(..).get_mapped_range().to_vec();
    read.unmap();
    pixels
}

fn ndc(matrix: [f32; 16], point: [f64; 3]) -> [f32; 2] {
    let clip = std::array::from_fn::<_, 4, _>(|i| {
        matrix[12 + i]
            + (0..3)
                .map(|j| matrix[j * 4 + i] * point[j] as f32)
                .sum::<f32>()
    });
    assert!(clip[3] > 0.);
    [clip[0] / clip[3], clip[1] / clip[3]]
}

fn archive_contents(bytes: &[u8]) -> (Vec<session_rust::proto::Session>, serde_json::Value) {
    use prost::Message;
    let bundle = crate::archive::Bundle::decode(bytes).unwrap();
    (bundle.documents.iter().map(|bytes| session_rust::proto::Session::decode(bytes.as_slice()).unwrap()).collect(), serde_json::to_value(bundle.metadata).unwrap())
}

#[test]
fn combined_original_families_keep_holes_ink_text_queries_and_exact_sample_returns() {
    let (mut scene, owners) = composition();
    let archive = crate::scene_archive::save(&scene).unwrap();
    crate::sample_test_proof::write("composition.session", &archive).unwrap();
    crate::sample_test_proof::write("composition-owners.json", serde_json::to_vec_pretty(&owners.iter().map(|(id, point)| serde_json::json!({"id": id.value(), "guid": scene.row_guid(*id).unwrap(), "world": point})).collect::<Vec<_>>()).unwrap()).unwrap();
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let adapter = pollster::block_on(instance.request_adapter(&Default::default())).unwrap();
    let (device, queue) = pollster::block_on(adapter.request_device(&Default::default())).unwrap();
    device.on_uncaptured_error(std::sync::Arc::new(|e| {
        panic!("actual composed frame: {e}")
    }));
    let mut r = Renderer::with_formats(
        device,
        queue,
        wgpu::TextureFormat::Rgba8Unorm,
        wgpu::TextureFormat::Rgba8UnormSrgb,
        &scene,
    );
    r.resize(crate::viewport::Viewport {
        width: 640,
        height: 480,
    });
    let mut camera = Camera::default();
    camera.aspect = 4. / 3.;
    camera.fit(&scene.bounds().unwrap());
    let geometry = r
        .meshes
        .iter()
        .map(|m| Rc::clone(&m.geometry))
        .collect::<Vec<_>>();
    let uploads = r.stats()[0];
    for projection in [Projection::Orthographic, Projection::Perspective] {
        camera.set_projection_framed(projection, scene.bounds().as_ref());
        let matrix = camera.uniform();
        r.cloud_camera(&camera);
        r.set_outlines(true);
        r.set_ambient(true);
        r.set_samples(4);
        let first = pixels(&r, matrix);
        assert_eq!(r.ambient_error(), None);
        assert_eq!(r.text_error(), None);
        assert!(
            first
                .chunks_exact(4)
                .filter(|p| p[0].abs_diff(p[2]) > 20)
                .count()
                > 5000,
            "Original coloured geometry is visible"
        );
        crate::sample_test_proof::write(format!("composition-{projection:?}-four.rgba"), &first)
            .unwrap();
        for count in [1, 4] {
            r.set_samples(count);
            let current = pixels(&r, matrix);
            assert_eq!(r.ambient_error(), None);
            assert_eq!(r.text_error(), None);
            if count == 4 {
                assert!(current == first, "Composed4→1→4 restores every scene pixel");
            } else {
                crate::sample_test_proof::write(
                    format!("composition-{projection:?}-one.rgba"),
                    &current,
                )
                .unwrap();
            }
            assert_eq!(r.stats()[0], uploads);
            for (mesh, before) in r.meshes.iter().zip(&geometry) {
                assert!(Rc::ptr_eq(&mesh.geometry, before));
            }
            let after_archive = crate::scene_archive::save(&scene).unwrap();
            crate::sample_test_proof::write("composition-after.session", &after_archive).unwrap();
            assert!(archive_contents(&after_archive) == archive_contents(&archive), "Every source field, ordered sequence and presentation field is unchanged");
        }
    }
    r.set_ambient(false);
    r.set_outlines(false);
    camera.set_projection_framed(Projection::Orthographic, scene.bounds().as_ref());
    camera.set_view(View::Top);
    camera.fit(&scene.bounds().unwrap());
    let matrix = camera.uniform();
    r.cloud_camera(&camera);
    let at = ndc(matrix, [-3., 1.5, 0.]);
    let x = ((at[0] + 1.) * 320.).floor() as usize;
    let y = ((1. - at[1]) * 240.).floor() as usize;
    for count in [1, 4] {
        r.set_samples(count);
        let frame = pixels(&r, matrix);
        for dy in -2..=2 {
            for dx in -2..=2 {
                let i = ((y as isize + dy) as usize * 640 + (x as isize + dx) as usize) * 4;
                assert!(
                    frame[i..i + 3].iter().all(|v| *v > 250),
                    "Original BRep bore remains open at{count}x"
                );
            }
        }
        crate::sample_test_proof::write(format!("composition-top-{count}.rgba"), &frame).unwrap();
        for (owner, world) in &owners {
            r.request_pick(ndc(matrix, *world), matrix).unwrap();
            crate::cap_gpu_tests::wait(&r);
            assert_eq!(
                r.take_pick(matrix).unwrap().unwrap().unwrap()[0],
                owner.value(),
                "Fresh original owner at{count}x for{world:?}"
            );
        }
    }
    let plane = crate::clipping_geometry::plane_from(
        crate::clipping_geometry::Mode::Xy,
        &[[0., 0., 0.3]],
        6.,
    )
    .unwrap();
    crate::plane_original::insert(&mut scene, Rc::new(plane)).unwrap();
    let clipped_archive = archive_contents(&crate::scene_archive::save(&scene).unwrap());
    r.set_scene(&scene, None);
    r.set_clipping(true, false);
    r.set_outlines(true);
    r.set_ambient(true);
    r.set_grid(Some(crate::grid::Settings {
        spacing: 1.,
        half_cells: 6,
    }))
    .unwrap();
    r.set_measurement(Some(&crate::measure_mark::Mark {
        points: vec![[-4.4, -3.5, 0.2], [-1.3, -3.5, 0.2]],
        label: "Original distance".into(),
    }))
    .unwrap();
    let guide = crate::guide_gpu::Overlay {
        segments: vec![crate::guide_gpu::Segment {
            stroke: crate::stroke::Stroke {
                start: [1., -3.5, 0.5],
                end: [3.7, -3.5, 0.5],
                width: 1.5,
                colour: [0.8, 0.15, 0.15, 1.],
            },
            dashed: true,
        }],
        ..Default::default()
    };
    r.set_guides(&guide).unwrap();
    let mut preview = crate::tool::Draft::new(crate::tool::Shape::Line);
    preview.submit([1., -3.1, 0.6]).unwrap();
    preview.hover(Some([3.7, -3.1, 0.6])).unwrap();
    r.set_preview(Some(&preview)).unwrap();
    camera.set_view(View::Iso);
    camera.fit(&scene.bounds().unwrap());
    let matrix = camera.uniform();
    r.cloud_camera(&camera);
    r.set_samples(4);
    let clipped = pixels(&r, matrix);
    assert_eq!(r.cap_error(), None);
    assert!(
        r.cap_usage()[1] > 0,
        "Actual closed originals participate in cap composition"
    );
    crate::sample_test_proof::write("composition-clipped-four.rgba", &clipped).unwrap();
    r.set_samples(1);
    let single = pixels(&r, matrix);
    crate::sample_test_proof::write("composition-clipped-one.rgba", &single).unwrap();
    r.set_samples(4);
    assert!(
        pixels(&r, matrix) == clipped,
        "Caps, outlines, ambient, grid, preview, guides and text restore together"
    );
    assert!(archive_contents(&crate::scene_archive::save(&scene).unwrap()) == clipped_archive);
    r.set_grid(None).unwrap();r.set_ambient(false);r.set_outlines(false);
    r.set_measurement(None).unwrap();r.set_guides(&crate::guide_gpu::Overlay::default()).unwrap();r.set_preview(None).unwrap();
    camera.set_view(View::Top);camera.fit(&scene.bounds().unwrap());let matrix=camera.uniform();r.cloud_camera(&camera);
    let at=ndc(matrix,[-3.15,1.5,0.3]);let x=((at[0]+1.)*320.).floor() as usize;let y=((1.-at[1])*240.).floor() as usize;
    for count in [1,4] {
        r.set_samples(count);let frame=pixels(&r,matrix);
        for dy in -2..=2 {for dx in -2..=2 {let i=((y as isize+dy) as usize*640+(x as isize+dx) as usize)*4;assert!(frame[i..i+3].iter().all(|v|*v>250),"Clipped original BRep bore must remain open at{count}x; pixel{:?}",&frame[i..i+4]);}}
        crate::sample_test_proof::write(format!("composition-clipped-top-{count}.rgba"),&frame).unwrap();
    }

}
