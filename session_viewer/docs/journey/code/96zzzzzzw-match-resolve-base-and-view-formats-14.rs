use crate::frame_targets::Targets;

fn read(device: &wgpu::Device, buffer: &wgpu::Buffer) -> Vec<u8> {
    let (tx, rx) = std::sync::mpsc::channel();
    buffer
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |v| tx.send(v).unwrap());
    device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    rx.recv().unwrap().unwrap();
    let data = buffer.slice(..).get_mapped_range().to_vec();
    buffer.unmap();
    data
}

#[test]
fn actual_samples_keep_depth_and_integer_addresses_and_resolve_only_colour() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let adapter = pollster::block_on(instance.request_adapter(&Default::default())).unwrap();
    let (device, queue) = pollster::block_on(adapter.request_device(&Default::default())).unwrap();
    device.on_uncaptured_error(std::sync::Arc::new(|e| panic!("actual frame targets: {e}")));
    for (base, format) in [
        (wgpu::TextureFormat::Rgba8Unorm, wgpu::TextureFormat::Rgba8Unorm),
        (wgpu::TextureFormat::Rgba8Unorm, wgpu::TextureFormat::Rgba8UnormSrgb),
        (wgpu::TextureFormat::Rgba8UnormSrgb, wgpu::TextureFormat::Rgba8UnormSrgb),
    ] {
    for samples in [1, 4] {
        let mut targets = Targets::with_formats(&device, [8, 8], base, format, samples);
        assert_eq!(targets.primitive.texture.sample_count(), samples);
        assert_eq!(
            targets.primitive.texture.format(),
            crate::primitive_image::FORMAT
        );
        assert_eq!(
            targets
                .primitive
                .texture
                .usage()
                .contains(wgpu::TextureUsages::COPY_SRC),
            samples == 1
        );
        let output = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("resolved float colour"),
            size: wgpu::Extent3d {
                width: 8,
                height: 8,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: base,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[format],
        });
        let single = output.create_view(&wgpu::TextureViewDescriptor { format: Some(format), ..Default::default() });
        assert_eq!(targets.color(&single) == &single, samples == 1);
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("actual distinct samples"),
            source: wgpu::ShaderSource::Wgsl(r#"
@vertex fn vertex(@builtin(vertex_index) i:u32)->@builtin(position) vec4f {
 let p=array<vec2f,3>(vec2f(-1,-1),vec2f(3,-1),vec2f(-1,3));return vec4f(p[i],0.5,1);
}
struct Hit { @location(0) color:vec4f, @location(1) primitive:vec2u, @builtin(frag_depth) depth:f32 }
@fragment fn fragment(@builtin(sample_index) sample:u32)->Hit {
 return Hit(vec4f(f32(sample&1u),f32((sample>>1u)&1u),0,1),vec2u(sample+1u,50000u+sample),f32(sample+1u)*0.125);
}"#.into()),
        });
        let pipe = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("per-sample colour, integer and depth"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[
                    Some(wgpu::ColorTargetState {
                        format,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    }),
                    Some(crate::primitive_image::target(true)),
                ],
            }),
            primitive: Default::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: crate::depth::FORMAT,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Always),
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: samples,
                ..Default::default()
            },
            multiview_mask: None,
            cache: None,
        });
        let storage = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let mut source = format!(
            r#"
@group(0) @binding(0) var depth:texture_depth_2d;
@group(0) @binding(1) var physical:texture_2d<u32>;
@group(0) @binding(2) var<storage,read_write> values:array<vec4u>;
@compute @workgroup_size(4) fn read_samples(@builtin(global_invocation_id) id:vec3u) {{
 if(id.x>={samples}u){{return;}}
 let primitive=textureLoad(physical,vec2i(2),SAMPLE).xy;
 values[id.x]=vec4u(bitcast<u32>(textureLoad(depth,vec2i(2),SAMPLE)),primitive,id.x);
}}
"#
        );
        if samples == 4 {
            source = source
                .replace("texture_depth_2d", "texture_depth_multisampled_2d")
                .replace("texture_2d<u32>", "texture_multisampled_2d<u32>");
        }
        source = source.replace("SAMPLE", if samples == 4 { "i32(id.x)" } else { "0" });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("actual sample readback"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let compute = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: None,
            module: &module,
            entry_point: Some("read_samples"),
            compilation_options: Default::default(),
            cache: None,
        });
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &compute.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&targets.depth),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&targets.primitive.view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: storage.as_entire_binding(),
                },
            ],
        });
        let result = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let pixels = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 256 * 8,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut encoder = device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("actual matching sample attachments"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: targets.color(&single),
                        resolve_target: None,
                        depth_slice: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                            store: wgpu::StoreOp::Store,
                        },
                    }),
                    Some(wgpu::RenderPassColorAttachment {
                        view: &targets.primitive.view,
                        resolve_target: None,
                        depth_slice: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                    }),
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &targets.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(0.),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            pass.set_pipeline(&pipe);
            pass.draw(0..3, 0..1);
        }
        targets.resolve(&mut encoder, &single);
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&compute);
            pass.set_bind_group(0, &group, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        encoder.copy_buffer_to_buffer(&storage, 0, &result, 0, 64);
        encoder.copy_texture_to_buffer(
            output.as_image_copy(),
            wgpu::TexelCopyBufferInfo {
                buffer: &pixels,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(256),
                    rows_per_image: Some(8),
                },
            },
            output.size(),
        );
        queue.submit([encoder.finish()]);
        let actual = read(&device, &result);
        for sample in 0..samples as usize {
            let words = actual[sample * 16..sample * 16 + 16]
                .chunks_exact(4)
                .map(|b| u32::from_ne_bytes(b.try_into().unwrap()))
                .collect::<Vec<_>>();
            assert_eq!(f32::from_bits(words[0]), (sample + 1) as f32 * 0.125);
            assert_eq!(
                &words[1..],
                &[sample as u32 + 1, 50000 + sample as u32, sample as u32]
            );
        }
        let actual = read(&device, &pixels);
        let expected = if samples == 4 {
            let midpoint = if format.is_srgb() { 188 } else { 128 };
            [midpoint, midpoint, 0, 255]
        } else {
            [0, 0, 0, 255]
        };
        assert!(actual[2 * 256 + 2 * 4..2 * 256 + 2 * 4 + 4]
            .iter()
            .zip(expected)
            .all(|(&a, b)| a.abs_diff(b) <= 1));
        let depth = targets.depth.clone();
        let primitive = targets.primitive.texture.clone();
        let other = if samples == 1 { 4 } else { 1 };
        targets = Targets::with_formats(&device, [17, 13], base, format, other);
        assert_eq!(targets.primitive.texture.sample_count(), other);
        assert_eq!(
            [
                targets.primitive.texture.width(),
                targets.primitive.texture.height()
            ],
            [17, 13]
        );
        assert_ne!(targets.depth, depth);
        assert_ne!(targets.primitive.texture, primitive);
        let mut encoder = device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&compute);
            pass.set_bind_group(0, &group, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        encoder.copy_buffer_to_buffer(&storage, 0, &result, 0, 64);
        queue.submit([encoder.finish()]);
        let retained = read(&device, &result);
        for sample in 0..samples as usize {
            let at = sample * 16;
            assert_eq!(
                f32::from_bits(u32::from_ne_bytes(retained[at..at + 4].try_into().unwrap())),
                (sample + 1) as f32 * 0.125
            );
            assert_eq!(
                u32::from_ne_bytes(retained[at + 4..at + 8].try_into().unwrap()),
                sample as u32 + 1
            );
            assert_eq!(
                u32::from_ne_bytes(retained[at + 8..at + 12].try_into().unwrap()),
                50000 + sample as u32
            );
        }
        targets = Targets::with_formats(&device, [8, 8], base, format, samples);
        assert_eq!(targets.primitive.texture.sample_count(), samples);
    }
    }
}

#[test]
fn identical_resize_retains_targets_geometry_sources_and_exact_single_sample_pixels() {
    use crate::{
        editor::{Action, Editor},
        viewport::Viewport,
    };
    let mut editor = Editor::default();
    editor
        .apply(Action::Replace(crate::specimen::bytes()))
        .unwrap();
    let source = editor.scene.objects()[0].geometry().unwrap().clone();
    let weak = std::rc::Rc::downgrade(&source);
    let original =
        crate::archive::Bundle::decode(&crate::scene_archive::save(&editor.scene).unwrap())
            .unwrap();
    let documents = |b: &crate::archive::Bundle| {
        use prost::Message;
        b.documents
            .iter()
            .map(|d| session_rust::proto::Session::decode(d.as_slice()).unwrap())
            .collect::<Vec<_>>()
    };
    let before = documents(&original);
    let metadata = serde_json::to_value(&original.metadata).unwrap();
    let mut renderer = crate::cap_gpu_tests::renderer(&editor.scene);
    let geometry = renderer.geometry_owners().cloned().collect::<Vec<_>>();
    assert_eq!(geometry.len(), editor.scene.objects().len());
    assert!(!geometry.is_empty());
    let mut camera = crate::camera::Camera::default();
    camera.aspect = 4. / 3.;
    camera.fit(&editor.scene.bounds().unwrap());
    let transform = renderer.camera_matrix(&camera);
    let baseline = crate::cap_gpu_tests::pixels_at(&renderer, transform);
    assert!(
        baseline
            .chunks_exact(4)
            .filter(|p| p[..3] != [255, 255, 255])
            .count()
            > 1000
    );
    export("original-frame-before", &baseline);
    let primitive = renderer.targets.primitive.texture.clone();
    let depth = renderer.targets.depth.clone();
    let stats = renderer.stats();
    let uploads = renderer.uploaded_bytes();
    for _ in 0..4 {
        renderer.resize(Viewport {
            width: 320,
            height: 240,
        });
    }
    assert_eq!(renderer.targets.depth, depth);
    assert_eq!(renderer.targets.primitive.texture, primitive);
    assert_eq!(renderer.targets.samples, 1);
    assert!(crate::cap_gpu_tests::pixels_at(&renderer, transform) == baseline);
    renderer.resize(Viewport {
        width: 640,
        height: 480,
    });
    assert_ne!(renderer.targets.depth, depth);
    assert_ne!(renderer.targets.primitive.texture, primitive);
    renderer.resize(Viewport {
        width: 320,
        height: 240,
    });
    assert!(crate::cap_gpu_tests::pixels_at(&renderer, transform) == baseline);
    export(
        "original-frame-after",
        &crate::cap_gpu_tests::pixels_at(&renderer, transform),
    );
    assert_eq!(renderer.geometry_owners().count(), geometry.len());
    assert_eq!(renderer.stats(), stats);
    assert_eq!(renderer.uploaded_bytes(), uploads);
    assert!(renderer
        .geometry_owners()
        .zip(&geometry)
        .all(|(a, b)| std::rc::Rc::ptr_eq(a, b)));
    let after = crate::archive::Bundle::decode(&crate::scene_archive::save(&editor.scene).unwrap())
        .unwrap();
    assert!(documents(&after) == before);
    assert_eq!(serde_json::to_value(after.metadata).unwrap(), metadata);
    drop(source);
    drop(geometry);
    drop(renderer);
    drop(editor);
    assert!(weak.upgrade().is_none());
}

fn export(name: &str, image: &[u8]) {
    if let Some(root) = std::env::var_os("VIEWER_FRAME_TARGET_PROOF_DIR") {
        use std::io::Write;
        std::fs::create_dir_all(&root).unwrap();
        let mut file =
            std::fs::File::create(std::path::Path::new(&root).join(format!("{name}.ppm"))).unwrap();
        write!(file, "P6\n320 240\n255\n").unwrap();
        for p in image.chunks_exact(4) {
            file.write_all(&p[..3]).unwrap();
        }
    }
}
