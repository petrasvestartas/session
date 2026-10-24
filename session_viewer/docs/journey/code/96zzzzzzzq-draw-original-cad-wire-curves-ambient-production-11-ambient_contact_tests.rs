use crate::{
    camera::{Camera, Projection},
    cap_gpu_tests::{renderer, wait},
    editor::{Action, Editor},
    scene::Scene,
};
use prost::Message;
use session_rust::{Session, Xform, proto};
use std::rc::Rc;

fn document() -> proto::Session {
    let (_, source) = super::contact_feature_tests::imported(false);
    let mut d = Session::from_proto(source).unwrap();
    let mut cube = crate::cap_gpu_tests::source(false);
    assert!(
        cube.transform(
            &(&Xform::translation(280., 200., 80.) * &Xform::scale_xyz(100., 100., 100.))
        )
    );
    cube.set_objectcolor(session_rust::Color::new(0.3, 0.3, 0.3, 1.));
    d.add_mesh(cube, None);
    d.to_proto()
}
fn editor(bytes: &[u8]) -> Editor {
    let mut e = Editor::default();
    e.apply(Action::ReplaceAt(
        bytes.to_vec(),
        Rc::new(crate::reload_url::ReloadUrl::new(
            "test:contact-ambient".into(),
        )),
    ))
    .unwrap();
    e
}
fn camera(projection: Projection) -> Camera {
    let mut c = crate::camera_defaults_tests::top_fixture();
    c.distance = 3000.;
    c.radius = 1500.;
    c.aspect = 4. / 3.;
    c.projection = projection;
    c
}
fn snapshot(
    s: &Scene,
) -> (
    Vec<proto::Session>,
    Vec<(String, proto::Mesh)>,
    serde_json::Value,
) {
    let b = crate::archive::Bundle::decode(&crate::scene_archive::save(s).unwrap()).unwrap();
    let mut embedded = Vec::new();
    let documents = b
        .documents
        .iter()
        .enumerate()
        .map(|(document, v)| {
            let mut d = proto::Session::decode(v.as_slice()).unwrap();
            for (branch, objects) in [
                ("objects", d.objects.as_mut()),
                ("definitions", d.definitions.as_mut()),
            ] {
                if let Some(objects) = objects {
                    for (index, element) in objects.elements.iter_mut().enumerate() {
                        if element.geometry_type == "Mesh" {
                            embedded.push((
                                format!("{document}/{branch}/{index}/{}", element.guid),
                                proto::Mesh::decode(element.geometry_data.as_slice()).unwrap(),
                            ));
                            element.geometry_data.clear();
                        }
                    }
                }
            }
            d
        })
        .collect();
    (
        documents,
        embedded,
        serde_json::to_value(b.metadata).unwrap(),
    )
}
fn low_renderer(s: &Scene) -> super::Renderer {
    let adapter = crate::gpu_test_adapter::adapter();
    let mut request = wgpu::DeviceDescriptor::default();
    request.required_limits.max_storage_buffer_binding_size = 64 * 1024;
    let (device, queue) = pollster::block_on(adapter.request_device(&request)).unwrap();
    device.on_uncaptured_error(std::sync::Arc::new(|error| {
        panic!("Actual contact ambient device: {error}")
    }));
    let mut g = super::Renderer::new(device, queue, wgpu::TextureFormat::Rgba8Unorm, s);
    g.resize(crate::viewport::Viewport {
        width: 320,
        height: 240,
    });
    g
}
fn pixels(g: &mut super::Renderer, c: &Camera, size: [u32; 2]) -> Vec<u8> {
    crate::widget_tests::pixels(g, &c.uniform(), size)
}
fn read(g: &super::Renderer, t: &wgpu::Texture, bpp: u32) -> Vec<u8> {
    let size = t.size();
    let pitch = (size.width * bpp).div_ceil(256) * 256;
    let b = g.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: u64::from(pitch) * u64::from(size.height),
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
                rows_per_image: Some(size.height),
            },
        },
        size,
    );
    g.queue.submit([e.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    b.slice(..)
        .map_async(wgpu::MapMode::Read, move |v| tx.send(v).unwrap());
    wait(g);
    rx.recv().unwrap().unwrap();
    let mapped = b.slice(..).get_mapped_range();
    let value = mapped
        .chunks_exact(pitch as usize)
        .flat_map(|row| row[..(size.width * bpp) as usize].iter().copied())
        .collect();
    drop(mapped);
    b.unmap();
    value
}
fn properties(g: &super::Renderer, size: [u32; 2], shared: bool) -> (Vec<u8>, Vec<u8>) {
    let texture = |format| {
        g.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("actual contact ambient property readback"),
            size: wgpu::Extent3d {
                width: size[0],
                height: size[1],
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        })
    };
    let radii = texture(wgpu::TextureFormat::R32Float);
    let normals: [wgpu::Texture; 3] = std::array::from_fn(|_| texture(wgpu::TextureFormat::R32Float));
    let mut sources = crate::ambient_source_gpu::Sources::new(&g.device);
    let mut e = g.device.create_command_encoder(&Default::default());
    sources.encode(
        &mut e,
        &radii.create_view(&Default::default()),
        normals.each_ref().map(|texture| texture.create_view(&Default::default())).each_ref(),
        &g.targets.depth,
        &g.view_group,
        &g.clipping.borrow().group,
        &g.meshes,
        shared.then_some(&g.instances),
    );
    g.queue.submit([e.finish()]);
    let normals = normals.each_ref().map(|texture| read(g, texture, 4));
    let normals = (0..size[0] as usize * size[1] as usize).flat_map(|i| normals.iter().flat_map(move |channel| channel[i*4..i*4+4].iter().copied())).collect();
    (read(g, &radii, 4), normals)
}
fn sample(c: &Camera, size: [u32; 2], p: [f64; 3]) -> usize {
    let ndc = super::contact_feature_tests::project(c.uniform(), p);
    let x = ((ndc[0] + 1.) * size[0] as f32 / 2.).floor() as usize;
    let y = ((1. - ndc[1]) * size[1] as f32 / 2.).floor() as usize;
    assert!(x < size[0] as usize && y < size[1] as usize);
    y * size[0] as usize + x
}
fn radius(data: &[u8], index: usize) -> f32 {
    f32::from_ne_bytes(data[index * 4..index * 4 + 4].try_into().unwrap())
}
fn pick(
    g: &mut super::Renderer,
    s: &Scene,
    c: &Camera,
    p: [f64; 3],
) -> Option<crate::pick_sources::Target> {
    g.set_pick_mode(crate::pick_sources::Mode::Face);
    g.request_pick(
        super::contact_feature_tests::project(c.uniform(), p),
        c.uniform(),
    )
    .unwrap();
    wait(g);
    g.take_pick(c.uniform())
        .unwrap()
        .unwrap()
        .and_then(|v| g.resolve_pick(s, v))
}
fn export(name: &str, s: &Scene, bytes: &[u8], image: &[u8], size: [u32; 2]) {
    if let Some(root) = std::env::var_os("VIEWER_AMBIENT_CONTACT_PROOF_DIR") {
        use std::io::Write;
        let root = std::path::Path::new(&root);
        std::fs::create_dir_all(root).unwrap();
        std::fs::write(root.join(format!("{name}.pb")), bytes).unwrap();
        std::fs::write(
            root.join(format!("{name}.session")),
            crate::scene_archive::save(s).unwrap(),
        )
        .unwrap();
        let mut f = std::fs::File::create(root.join(format!("{name}.ppm"))).unwrap();
        write!(f, "P6\n{} {}\n255\n", size[0], size[1]).unwrap();
        for p in image.chunks_exact(4) {
            f.write_all(&p[..3]).unwrap();
        }
    }
}

#[test]
fn contact_properties_match_original_radius_and_shared_geometry_in_both_paths() {
    let bytes = document().encode_to_vec();
    let mut e = editor(&bytes);
    let parent = e
        .scene
        .objects()
        .iter()
        .find(|r| {
            matches!(
                r.original.as_ref(),
                Some(session_rust::Geometry::Element(_))
            )
        })
        .unwrap()
        .id;
    let child = e
        .scene
        .objects()
        .iter()
        .find(|r| e.scene.forest.derived(r.id))
        .unwrap()
        .id;
    let original = snapshot(&e.scene);
    let point = [450., 200., 0.];
    for reduced in [false, true] {
        let mut g = if reduced {
            low_renderer(&e.scene)
        } else {
            renderer(&e.scene)
        };
        g.set_opacity(1.);
        g.set_ambient(true);
        let uploads = g.stats()[0];
        for projection in [Projection::Orthographic, Projection::Perspective] {
            let c = camera(projection);
            for density in [1u32, 2] {
                let size = [320 * density, 240 * density];
                g.resize(crate::viewport::Viewport {
                    width: size[0],
                    height: size[1],
                });
                g.set_density(density as f32);
                g.set_scene(&e.scene, None);
                let image = pixels(&mut g, &c, size);
                let expected = g
                    .ambient_rows
                    .specs
                    .iter()
                    .find(|r| r.0 == parent.value())
                    .unwrap()
                    .1;
                assert_eq!(
                    g.ambient_rows
                        .specs
                        .iter()
                        .find(|r| r.0 == child.value())
                        .unwrap()
                        .1,
                    expected
                );
                let legacy = properties(&g, size, false);
                let shared = properties(&g, size, true);
                assert!(
                    legacy == shared,
                    "Every actual legacy/shared source property byte agrees"
                );
                let index = sample(&c, size, point);
                assert_eq!(
                    radius(&shared.0, index),
                    expected,
                    "Actual contact radius follows its original visual owner"
                );
                assert!(
                    super::ambient_input_tests::encoded_normal(std::array::from_fn(|j| f32::from_ne_bytes(shared.1[index * 12 + j * 4..index * 12 + j * 4 + 4].try_into().unwrap())))
                        .iter()
                        .all(|v| v.abs_diff(128) <= 1),
                    "Ambient source normals remain geometric"
                );
                if !reduced {
                    assert!(
                        g.meshes
                            .iter()
                            .all(|m| g.instances.current_row(m).is_some()),
                        "The ordinary device exercises actual storage object records"
                    );
                } else {
                    assert_eq!(
                        g.tile_usage()[..2],
                        [96, 16],
                        "Real64KiB device retains physical fallback"
                    );
                }
                assert_eq!(
                    pick(&mut g, &e.scene, &c, point),
                    Some(crate::pick_sources::Target {
                        owner: parent,
                        part: crate::pick_sources::Part::Face(0)
                    })
                );
                assert!(snapshot(&e.scene) == original);
                Rc::make_mut(&mut e.scene.view).lock(&[parent], true);
                assert_eq!(
                    crate::feature_owner::visual_owner(&e.scene, child),
                    Some(parent)
                );
                assert_eq!(crate::feature_owner::display_owner(&e.scene, child), None);
                g.set_scene(&e.scene, None);
                assert!(
                    pixels(&mut g, &c, size) == image,
                    "Locking changes picking, not visible contact shading"
                );
                assert_eq!(radius(&properties(&g, size, false).0, index), expected);
                assert_eq!(pick(&mut g, &e.scene, &c, point), None);
                Rc::make_mut(&mut e.scene.view).lock(&[parent], false);
                g.set_scene(&e.scene, None);
                assert!(pixels(&mut g, &c, size) == image);
                assert!(snapshot(&e.scene) == original);
                let ordinary = std::mem::take(&mut g.instances.ordinary);
                g.set_ambient(false);
                g.set_ambient(true);
                assert!(
                    pixels(&mut g, &c, size) == image,
                    "Actual complete renderer uniform fallback preserves every pixel"
                );
                g.instances.ordinary = ordinary;
                assert_eq!(g.stats()[0], uploads);
                if !reduced {
                    let name = format!(
                        "ordinary-{}-dpr{density}",
                        if projection == Projection::Orthographic {
                            "orthographic"
                        } else {
                            "perspective"
                        }
                    );
                    export(&name, &e.scene, &bytes, &image, size);
                }
            }
        }
    }
}

#[test]
fn placed_contacts_follow_original_instances_without_changing_pickability_or_geometry() {
    let element = document().objects.unwrap().elements.remove(0);
    let guid = element.guid.clone();
    let mut source = crate::instance_specimen::message();
    source.definitions = Some(proto::Objects {
        elements: vec![element],
        ..Default::default()
    });
    for row in &mut source.objects.as_mut().unwrap().instances {
        row.definition_guid = guid.clone();
        row.xform = None;
        row.flags = 0;
    }
    source.xforms = vec![
        proto::XformEntry {
            guid: "first".into(),
            xform: Some(Xform::translation(-1800., 0., 0.).to_proto()),
        },
        proto::XformEntry {
            guid: "second".into(),
            xform: Some(
                (&Xform::translation(1800., 0., 0.) * &Xform::scale_xyz(-0.8, 1.2, 1.4)).to_proto(),
            ),
        },
    ];
    let bytes = source.encode_to_vec();
    let mut e = editor(&bytes);
    let parents = e
        .scene
        .objects()
        .iter()
        .filter(|r| e.scene.instances.contains_key(&r.id))
        .map(|r| r.id)
        .collect::<Vec<_>>();
    assert_eq!(parents.len(), 2);
    let rows = parents
        .iter()
        .map(|id| e.scene.objects().iter().find(|r| r.id == *id).unwrap())
        .collect::<Vec<_>>();
    assert!(Rc::ptr_eq(&rows[0].mesh, &rows[1].mesh));
    let weak = Rc::downgrade(rows[0].geometry().unwrap());
    let original = snapshot(&e.scene);
    let mut g = renderer(&e.scene);
    g.set_opacity(1.);
    g.set_ambient(true);
    let uploads = g.stats()[0];
    for projection in [Projection::Orthographic, Projection::Perspective] {
        let mut c = camera(projection);
        c.distance = 6000.;
        c.radius = 3000.;
        for density in [1u32, 2] {
            let size = [320 * density, 240 * density];
            g.resize(crate::viewport::Viewport {
                width: size[0],
                height: size[1],
            });
            g.set_density(density as f32);
            g.set_scene(&e.scene, None);
            let image = pixels(&mut g, &c, size);
            let properties = properties(&g, size, true);
            let mut expected = Vec::new();
            for &parent in &parents {
                let r = g
                    .ambient_rows
                    .specs
                    .iter()
                    .find(|r| r.0 == parent.value())
                    .unwrap()
                    .1;
                expected.push(r);
                let placed = e
                    .scene
                    .placement(parent)
                    .unwrap()
                    .transform_point(&session_rust::Point::new(450., 200., 0.));
                let point = [placed[0], placed[1], placed[2]];
                assert_eq!(radius(&properties.0, sample(&c, size, point)), r);
                assert_eq!(pick(&mut g, &e.scene, &c, point).unwrap().owner, parent);
                for child in e.scene.objects().iter().filter(|r| {
                    e.scene.forest.derived(r.id)
                        && crate::feature_owner::visual_owner(&e.scene, r.id) == Some(parent)
                }) {
                    assert_eq!(
                        g.ambient_rows
                            .specs
                            .iter()
                            .find(|r| r.0 == child.id.value())
                            .unwrap()
                            .1,
                        r
                    );
                }
            }
            assert_ne!(
                expected[0], expected[1],
                "World-scaled original instances retain distinct contact radii"
            );
            assert!(snapshot(&e.scene) == original);
            let old = e.scene.placement(parents[0]).unwrap();
            e.scene.translate(parents[0], [100., 0., 0.]).unwrap();
            g.set_scene(&e.scene, None);
            assert!(pixels(&mut g, &c, size) != image);
            e.scene.place_owner(parents[0], old).unwrap();
            g.set_scene(&e.scene, None);
            assert!(pixels(&mut g, &c, size) == image);
            assert_eq!(g.stats()[0], uploads);
            assert!(snapshot(&e.scene) == original);
            let name = format!(
                "instances-{}-dpr{density}",
                if projection == Projection::Orthographic {
                    "orthographic"
                } else {
                    "perspective"
                }
            );
            export(&name, &e.scene, &bytes, &image, size);
        }
    }
    let before = snapshot(&e.scene);
    assert_eq!(
        e.apply(Action::UnloadSources).err(),
        Some("No eligible imported sources to unload")
    );
    assert!(snapshot(&e.scene) == before);
    assert!(weak.upgrade().is_some());
    drop(e);
    assert!(
        weak.upgrade().is_none(),
        "Numeric visual radii do not retain original definition sources"
    );
    g.set_scene(&Scene::empty(), None);
    g.resize(crate::viewport::Viewport {
        width: 320,
        height: 240,
    });
    let c = camera(Projection::Orthographic);
    pixels(&mut g, &c, [320, 240]);
    assert_eq!(g.ambient_usage()[0], 320);
}
