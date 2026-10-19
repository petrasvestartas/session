use crate::{
    cap_gpu_tests::{pixels_at, renderer, wait},
    scene::Scene,
};
use prost::Message;
use session_rust::{Color, Mesh, Session, Xform, proto};
use std::rc::Rc;
fn original() -> Mesh {
    let mut mesh = crate::cap_gpu_tests::source(false);
    mesh.set_objectcolor(Color::new(0.2, 0.4, 0.6, 1.));
    for key in mesh.vertices() {
        mesh.set_vertex_attribute(key, "nx", 1.);
        mesh.set_vertex_attribute(key, "ny", 0.);
        mesh.set_vertex_attribute(key, "nz", 0.);
    }
    mesh
}
fn document() -> Vec<u8> {
    let mut doc = Session::new("Original lighting normals and geometric contacts");
    doc.add_mesh(original(), None);
    doc.to_proto().encode_to_vec()
}
fn scene(bytes: &[u8]) -> Scene {
    let mut s = Scene::empty();
    crate::scene_document::import(&mut s, bytes, "", &Xform::identity(), 0.).unwrap();
    s
}
fn snapshot(s: &Scene) -> (Vec<proto::Session>, serde_json::Value) {
    let b = crate::archive::Bundle::decode(&crate::scene_archive::save(s).unwrap()).unwrap();
    (
        b.documents
            .iter()
            .map(|v| proto::Session::decode(v.as_slice()).unwrap())
            .collect(),
        serde_json::to_value(b.metadata).unwrap(),
    )
}
fn camera() -> crate::camera::Camera {
    let mut c = crate::camera_defaults_tests::top_fixture();
    c.distance = 3.;
    c.radius = 1.;
    c.target = [0., 0., -0.5];
    c.aspect = 4. / 3.;
    c
}
fn texture(g: &super::Renderer, format: wgpu::TextureFormat) -> wgpu::Texture {
    g.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("actual geometric ambient property readback"),
        size: wgpu::Extent3d {
            width: 320,
            height: 240,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    })
}
fn read(g: &super::Renderer, t: &wgpu::Texture, pitch: u32) -> Vec<u8> {
    let b = g.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: u64::from(pitch) * 240,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut e = g.device.create_command_encoder(&Default::default());
    e.copy_texture_to_buffer(
        t.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &b,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(pitch),
                rows_per_image: Some(240),
            },
        },
        t.size(),
    );
    g.queue.submit([e.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    b.slice(..)
        .map_async(wgpu::MapMode::Read, move |v| tx.send(v).unwrap());
    wait(g);
    rx.recv().unwrap().unwrap();
    let v = b.slice(..).get_mapped_range().to_vec();
    b.unmap();
    v
}
fn pixel(m: [f32; 16], p: [f64; 3]) -> [usize; 2] {
    let clip: [f64; 4] = std::array::from_fn(|i| {
        m[12 + i] as f64 + (0..3).map(|j| m[j * 4 + i] as f64 * p[j]).sum::<f64>()
    });
    [
        ((clip[0] / clip[3] + 1.) * 160.).floor() as usize,
        ((1. - clip[1] / clip[3]) * 120.).floor() as usize,
    ]
}
fn properties(g: &super::Renderer, m: [f32; 16], points: &[[f64; 3]]) -> Vec<([u8; 2], f32)> {
    let radius = texture(g, wgpu::TextureFormat::R32Float);
    let normal = texture(g, wgpu::TextureFormat::Rg8Unorm);
    let mut source = crate::ambient_source_gpu::Sources::new(&g.device);
    let mut e = g.device.create_command_encoder(&Default::default());
    source.encode(
        &mut e,
        &radius.create_view(&Default::default()),
        &normal.create_view(&Default::default()),
        &g.depth,
        &g.view_group,
        &g.clipping.borrow().group,
        &g.meshes,
        Some(&g.instances),
    );
    g.queue.submit([e.finish()]);
    let normals = read(g, &normal, 768);
    let radii = read(g, &radius, 1280);
    points
        .iter()
        .map(|&p| {
            let [x, y] = pixel(m, p);
            assert!(x < 320 && y < 240);
            let at = y * 768 + x * 2;
            let n = [normals[at], normals[at + 1]];
            let at = y * 1280 + x * 4;
            (n, f32::from_ne_bytes(radii[at..at + 4].try_into().unwrap()))
        })
        .collect()
}
fn export(name: &str, s: &Scene, image: &[u8]) {
    if let Some(root) = std::env::var_os("VIEWER_AMBIENT_INPUT_PROOF_DIR") {
        use std::io::Write;
        let p = std::path::Path::new(&root);
        std::fs::create_dir_all(p).unwrap();
        let archive = crate::scene_archive::save(s).unwrap();
        let bundle = crate::archive::Bundle::decode(&archive).unwrap();
        assert_eq!(bundle.documents.len(), 1);
        std::fs::write(p.join(format!("{name}.pb")), &bundle.documents[0]).unwrap();
        std::fs::write(p.join(format!("{name}.session")), archive).unwrap();
        let mut f = std::fs::File::create(p.join(format!("{name}.ppm"))).unwrap();
        write!(f, "P6\n320 240\n255\n").unwrap();
        for p in image.chunks_exact(4) {
            f.write_all(&p[..3]).unwrap();
        }
    }
}
#[test]
fn actual_geometric_normals_preserve_authored_lighting_in_both_object_paths() {
    let s = scene(&document());
    let original = snapshot(&s);
    let owner = s.objects()[0].id;
    assert!(
        s.objects()[0]
            .mesh
            .normals()
            .iter()
            .all(|n| n[0] > 0.99 && n[2].abs() < 0.01)
    );
    let mut g = renderer(&s);
    g.set_opacity(1.);
    g.set_ambient(true);
    let uploads = g.stats()[0];
    let mut c = camera();
    let p = [0.11, 0.13, -0.2];
    for projection in [
        crate::camera::Projection::Orthographic,
        crate::camera::Projection::Perspective,
    ] {
        c.projection = projection;
        let m = c.uniform();
        for density in [1., 2.] {
            g.set_density(density);
            let image = pixels_at(&g, m);
            let sample = properties(&g, m, &[p])[0];
            assert!(
                sample.0.iter().all(|v| v.abs_diff(128) <= 1),
                "Geometric ambient normal must follow the actual top face: {:?}",
                sample.0
            );
            assert_eq!(sample.1, g.ambient_rows.specs[0].1);
            let [x, y] = pixel(m, p);
            let rgb = crate::cap_gpu_tests::at(&image, x, y);
            assert!(
                rgb[..3]
                    .iter()
                    .zip([44u8, 88, 132])
                    .all(|(a, b)| a.abs_diff(b) <= 2),
                "Authored+x lighting remains intact without false geometric self-shadow: {rgb:?}"
            );
            g.set_pick_mode(crate::pick_sources::Mode::Object);
            g.request_pick(super::contact_feature_tests::project(m, p), m)
                .unwrap();
            wait(&g);
            let picked = g
                .take_pick(m)
                .unwrap()
                .unwrap()
                .and_then(|v| g.resolve_pick(&s, v));
            assert_eq!(
                picked,
                Some(crate::pick_sources::Target {
                    owner,
                    part: crate::pick_sources::Part::Object
                })
            );
            g.set_ambient(false);
            g.use_uniform_surface_records();
            g.set_ambient(true);
            assert!(
                pixels_at(&g, m) == image,
                "Real uniform and storage object paths retain identical whole-frame pixels"
            );
            assert_eq!(properties(&g, m, &[p])[0], sample);
            g.set_scene(&s, None);
            assert!(pixels_at(&g, m) == image);
            assert_eq!(g.stats()[0], uploads);
            assert_eq!(snapshot(&s), original);
            if density == 1. {
                export(
                    if projection == crate::camera::Projection::Orthographic {
                        "authored-normal-top-dpr1"
                    } else {
                        "authored-normal-perspective-dpr1"
                    },
                    &s,
                    &image,
                );
            }
            if density == 2. {
                export(
                    if projection == crate::camera::Projection::Orthographic {
                        "authored-normal-top"
                    } else {
                        "authored-normal-perspective"
                    },
                    &s,
                    &image,
                );
            }
        }
    }
}
#[test]
fn contact_radii_reuse_records_for_shared_placements_and_released_sources() {
    let mesh = original();
    let guid = mesh.guid().to_owned();
    let mut proto = crate::instance_specimen::message();
    proto.definitions.as_mut().unwrap().meshes = vec![mesh.to_proto()];
    for row in &mut proto.objects.as_mut().unwrap().instances {
        row.definition_guid = guid.clone();
        row.xform = None;
        row.flags = 0;
    }
    proto.xforms = vec![
        proto::XformEntry {
            guid: "first".into(),
            xform: Some(Xform::translation(-1.4, 0., 0.).to_proto()),
        },
        proto::XformEntry {
            guid: "second".into(),
            xform: Some(
                (&Xform::translation(1.4, 0., 0.) * &Xform::scale_xyz(-0.8, 1.2, 1.4)).to_proto(),
            ),
        },
    ];
    let bytes = proto.encode_to_vec();
    let mut e = crate::editor::Editor::default();
    e.apply(crate::editor::Action::ReplaceAt(
        bytes.clone(),
        Rc::new(crate::reload_url::ReloadUrl::new(
            "test:ambient-inputs".into(),
        )),
    ))
    .unwrap();
    assert_eq!(e.scene.instances.len(), 2);
    let second = e.scene.objects()[1].id;
    assert!(Rc::ptr_eq(
        &e.scene.objects()[0].mesh,
        &e.scene.objects()[1].mesh
    ));
    let weak = Rc::downgrade(e.scene.objects()[0].geometry().unwrap());
    Rc::make_mut(&mut e.scene.view).hide(&[second], true);
    let mut g = renderer(&e.scene);
    g.set_opacity(1.);
    g.set_ambient(true);
    let mut c = camera();
    c.distance = 6.;
    c.radius = 3.;
    c.projection = crate::camera::Projection::Orthographic;
    let m = c.uniform();
    pixels_at(&g, m);
    let owned = g.ambient_usage()[0];
    let uploads = g.stats()[0];
    Rc::make_mut(&mut e.scene.view).hide(&[second], false);
    g.set_scene(&e.scene, None);
    let image = pixels_at(&g, m);
    assert_eq!(
        g.ambient_usage()[0],
        owned,
        "Another placed object must not allocate another ambient radius buffer or bind-group owner"
    );
    assert_eq!(g.stats()[0], uploads);
    let original = snapshot(&e.scene);
    let points = e
        .scene
        .objects()
        .iter()
        .map(|r| {
            let p = r
                .model
                .transform_point(&session_rust::Point::new(0.11, 0.13, -0.2));
            [p[0], p[1], p[2]]
        })
        .collect::<Vec<_>>();
    let values = properties(&g, m, &points);
    for (row, (normal, radius)) in e.scene.objects().iter().zip(&values) {
        assert!(
            (normal[0].abs_diff(128) <= 1 && normal[1].abs_diff(128) <= 1)
                || normal.iter().all(|v| *v >= 254),
            "Actual mirrored top normal stays geometric up to winding: {normal:?}"
        );
        assert_eq!(
            *radius,
            g.ambient_rows
                .specs
                .iter()
                .find(|(id, _)| *id == row.id.value())
                .unwrap()
                .1
        );
        assert!(*radius > 0.);
    }
    assert_ne!(
        values[0].1, values[1].1,
        "Nonuniform placements retain their distinct world contact radii"
    );
    export("shared-contact-radii", &e.scene, &image);
    assert_eq!(
        e.apply(crate::editor::Action::UnloadSources).err(),
        Some("No eligible imported sources to unload")
    );
    assert_eq!(snapshot(&e.scene), original);
    assert!(pixels_at(&g, m) == image);
    assert_eq!(properties(&g, m, &points), values);
    drop(e);
    assert!(
        weak.upgrade().is_none(),
        "Discarding definition instances releases their original geometry"
    );
    let bytes = document();
    let mut e = crate::editor::Editor::default();
    e.apply(crate::editor::Action::ReplaceAt(
        bytes.clone(),
        Rc::new(crate::reload_url::ReloadUrl::new(
            "test:ambient-original".into(),
        )),
    ))
    .unwrap();
    let weak = Rc::downgrade(e.scene.objects()[0].geometry().unwrap());
    let original = snapshot(&e.scene);
    c.distance = 3.;
    c.radius = 1.;
    let m = c.uniform();
    g.set_scene(&e.scene, None);
    let image = pixels_at(&g, m);
    let uploads = g.stats()[0];
    let point = [0.11, 0.13, -0.2];
    let values = properties(&g, m, &[point]);
    e.apply(crate::editor::Action::UnloadSources).unwrap();
    assert!(
        weak.upgrade().is_none(),
        "Ambient inputs retain no eligible original source mesh owner"
    );
    g.set_scene(&e.scene, None);
    assert!(pixels_at(&g, m) == image);
    assert_eq!(properties(&g, m, &[point]), values);
    assert_eq!(g.stats()[0], uploads);
    let keys = e.reload_keys();
    e.hydrate(keys.into_iter().map(|k| (k, bytes.clone())).collect())
        .unwrap();
    assert_eq!(snapshot(&e.scene), original);
    g.set_scene(&e.scene, None);
    assert!(pixels_at(&g, m) == image);
    assert_eq!(g.stats()[0], uploads);
    export("released-original-inputs", &e.scene, &image);
}
