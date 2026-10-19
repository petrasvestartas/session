use super::*;
use std::rc::Rc;

pub(super) fn retarget(_r:&mut Renderer,count:u32){assert_eq!(count,1);}
fn physical(r: &Renderer) -> Vec<[u32; 4]> {
    let count = r.targets.samples;
    let mut code = format!(
        r#"
@group(0) @binding(0) var ids: texture_2d<u32>;
@group(0) @binding(1) var depth: texture_depth_2d;
@group(0) @binding(2) var<storage,read_write> output: array<vec4u>;
@compute @workgroup_size(4) fn read_samples(@builtin(global_invocation_id) gid: vec3u) {{
 if gid.x >= {count}u {{return;}}
 let at = vec2i(208,96);
 output[gid.x] = vec4u(textureLoad(ids,at,SAMPLE).xy,bitcast<u32>(textureLoad(depth,at,SAMPLE)),gid.x);
}}
"#
    );
    if count == 4 {
        code = code
            .replace("texture_2d<u32>", "texture_multisampled_2d<u32>")
            .replace("texture_depth_2d", "texture_depth_multisampled_2d");
    }
    code = code.replace("SAMPLE", if count == 4 { "i32(gid.x)" } else { "0" });
    let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("actual cloud physical address witness"),
        source: wgpu::ShaderSource::Wgsl(code.into()),
    });
    let pipeline = r
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: None,
            module: &module,
            entry_point: Some("read_samples"),
            compilation_options: Default::default(),
            cache: None,
        });
    let output = r.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: 64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let read = r.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: 64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let group = r.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&r.targets.primitive.view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&r.targets.depth),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: output.as_entire_binding(),
            },
        ],
    });
    let mut encoder = r.device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.dispatch_workgroups(1, 1, 1);
    }
    encoder.copy_buffer_to_buffer(&output, 0, &read, 0, 64);
    r.queue.submit([encoder.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    read.slice(..)
        .map_async(wgpu::MapMode::Read, move |v| tx.send(v).unwrap());
    crate::cap_gpu_tests::wait(r);
    rx.recv().unwrap().unwrap();
    let values = read
        .slice(..)
        .get_mapped_range()
        .chunks_exact(16)
        .take(count as usize)
        .map(|b| {
            std::array::from_fn(|i| u32::from_ne_bytes(b[i * 4..i * 4 + 4].try_into().unwrap()))
        })
        .collect();
    read.unmap();
    values
}
#[test]
fn opaque_cloud_depth_clears_occluded_physical_triangle_and_cap_addresses() {
    for clipped in [false, true] {
        for count in [1] {
            let (mut scene, _) = crate::cap_gpu_tests::scene(false);
            if clipped {
                let plane = crate::clipping_geometry::plane_from(
                    crate::clipping_geometry::Mode::Points,
                    &[[0., 0., -0.3], [1., 0., -1.3], [0., 1., -0.3]],
                    2.,
                )
                .unwrap();
                crate::plane_original::insert(&mut scene, Rc::new(plane)).unwrap();
            }
            let mut r = crate::cap_gpu_tests::renderer(&scene);
            r.set_clipping(clipped, true);
            retarget(&mut r,count);
            let matrix = crate::cap_gpu_tests::matrix();
            let before_pixels = crate::cap_gpu_tests::pixels_at(&r, matrix);
            let before = physical(&r);
            for value in &before {
                let address = value[0] | (value[1] << 16);
                assert!(address > 0);
                assert_eq!(address >= crate::primitive_image::CAP_START, clipped);
            }
            let coords = if clipped {
                vec![0.28, 0.2, -0.585]
            } else {
                vec![0.3, 0.2, -0.1]
            };
            let mut source =
                session_rust::PointCloud::from_coords(coords, vec![0, 0, 255, 255], vec![]);
            source.point_size = 28.;
            let source = Rc::new(source);
            let _ = source.guid();
            let original = source.to_proto();
            let owner = scene
                .insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap())
                .unwrap();
            r.set_scene(&scene, None);
            r.set_clipping(clipped, true);
            let after_pixels = crate::cap_gpu_tests::pixels_at(&r, matrix);
            let after = physical(&r);
            crate::sample_test_proof::write(
                format!("cloud-address-{clipped}-{count}.json"),
                serde_json::to_vec_pretty(&serde_json::json!({"before": before, "after": after}))
                    .unwrap(),
            )
            .unwrap();
            crate::sample_test_proof::write(
                format!("cloud-address-{clipped}-{count}.rgba"),
                &after_pixels,
            )
            .unwrap();
            assert!(before_pixels != after_pixels);
            let rgba = crate::cap_gpu_tests::at(&after_pixels, 208, 96);
            assert!(
                rgba[2] > 250 && rgba[0] < 5 && rgba[1] < 5,
                "The actual cloud lies in front of the original face/cap"
            );
            for (prior, current) in before.iter().zip(&after) {
                assert!(
                    f32::from_bits(current[2]) > f32::from_bits(prior[2]),
                    "Cloud writes its nearer actual depth"
                );
                assert_eq!(
                    [current[0], current[1]],
                    [0, 0],
                    "Cloud depth must not retain an occluded triangle or cap address"
                );
            }
            r.request_pick([0.3, 0.2], matrix).unwrap();
            crate::cap_gpu_tests::wait(&r);
            assert_eq!(
                r.take_pick(matrix).unwrap().unwrap().unwrap()[0],
                owner.value()
            );
            assert_eq!(source.to_proto(), original);
        }
    }
}

#[test]
fn actual_nearer_cloud_hides_original_ink_without_hiding_front_sources() {
 let mut rows=Vec::new();
 for density in [1.,2.] {for count in [1] {
  let mut scene=Scene::empty();let face=scene.insert(crate::prepared::PreparedMesh::triangle([[-2.,-2.,0.6],[2.,-2.,0.6],[0.,2.,0.6]],[0.1,0.6,0.1]).unwrap()).unwrap();
  let mut line=session_rust::Line::new(-0.4,0.2,0.6,0.6,0.2,0.6);line.width=6.;line.linecolor=session_rust::Color::new(1.,0.,0.,1.);let line=Rc::new(line);let _=line.guid();let line_before=line.to_proto();let ink=scene.insert_line(crate::stroke::PreparedLine::new(line.clone()).unwrap()).unwrap();
  let mut r=crate::cap_gpu_tests::renderer(&scene);r.set_density(density);retarget(&mut r,count);let matrix=crate::cap_gpu_tests::reversed(session_rust::Xform::identity().m.map(|v|v as f32));let before=crate::cap_gpu_tests::pixels_at(&r,matrix);let z_before=physical(&r);let pixel=crate::cap_gpu_tests::at(&before,208,96);assert!(pixel[0]>240&&pixel[1]<10&&pixel[2]<10,"Original front red line exists before cloud: {pixel:?}");
  let coords=(-3..=3).flat_map(|y|(-3..=3).flat_map(move|x|[0.3+x as f64*0.03,0.2+y as f64*0.03,0.2])).collect();let mut points=session_rust::PointCloud::from_coords(coords,[0,0,255,255].repeat(49),vec![]);points.point_size=96.;let points=Rc::new(points);let _=points.guid();let cloud_before=points.to_proto();let cloud=scene.insert_cloud(crate::cloud::Prepared::new(points.clone()).unwrap()).unwrap();r.set_scene(&scene,None);let actual=crate::cap_gpu_tests::pixels_at(&r,matrix);let z_after=physical(&r);for (prior,current)in z_before.iter().zip(&z_after){assert!(f32::from_bits(current[2])>f32::from_bits(prior[2]));assert_eq!([current[0],current[1]],[0,0]);}
  let pixel=crate::cap_gpu_tests::at(&actual,208,96);assert!(pixel[2]>240&&pixel[0]<10&&pixel[1]<10,"Nearer opaque blue cloud hides the original red line: {pixel:?}");r.request_pick([0.3,0.2],matrix).unwrap();crate::cap_gpu_tests::wait(&r);let picked=r.take_pick(matrix).unwrap().unwrap();assert_eq!(picked.unwrap()[0],cloud.value());
  let geometry=r.meshes[0].geometry.clone();let cloud_uploads=r.cloud_usage()[2];
  scene.translate(ink,[0.,0.,-0.5]).unwrap();r.set_scene(&scene,None);let front=crate::cap_gpu_tests::pixels_at(&r,matrix);let visible=crate::cap_gpu_tests::at(&front,208,96);assert!(visible[0]>240&&visible[1]<10&&visible[2]<10,"A genuinely nearer original line remains visible: {visible:?}");r.request_pick([0.3,0.2],matrix).unwrap();crate::cap_gpu_tests::wait(&r);assert_eq!(r.take_pick(matrix).unwrap().unwrap().unwrap()[0],ink.value());
  scene.translate(ink,[0.,0.,0.5]).unwrap();r.set_scene(&scene,None);assert!(crate::cap_gpu_tests::pixels_at(&r,matrix)==actual,"Restoring original placement restores exact hidden-ink pixels");
  assert_eq!(r.cloud_usage()[2],cloud_uploads,"Original cloud source upload is retained through line placement changes");let cached=r.cloud_usage()[..4].to_vec();
  assert!(crate::cap_gpu_tests::pixels_at(&r,matrix)==actual,"Fresh owner queries retain exact repeated-frame pixels");
  assert!(Rc::ptr_eq(&r.meshes[0].geometry,&geometry));assert_eq!(r.cloud_usage()[..4],cached);crate::sample_test_proof::write(format!("cloud-ink-front-{density}-{count}.rgba"),&front).unwrap();assert_eq!(line.to_proto(),line_before);assert_eq!(points.to_proto(),cloud_before);
  crate::sample_test_proof::write(format!("cloud-ink-fixed-{density}-{count}.rgba"),&actual).unwrap();rows.push(serde_json::json!({"density":density,"samples":count,"physicalBefore":z_before,"physicalAfter":z_after,"actualRgba":pixel,"freshOwner":picked,"cloud":cloud.value(),"ink":ink.value(),"face":face.value()}));
 }}
 for density in [1.,2.] {for count in [1] {
  let mut scene=Scene::empty();scene.insert(crate::prepared::PreparedMesh::triangle([[-2.,-2.,0.6],[2.,-2.,0.6],[0.,2.,0.6]],[0.1,0.6,0.1]).unwrap()).unwrap();
  let mut source=session_rust::Line::new(-0.4,0.,0.2,0.6,0.,0.2);source.width=60.;source.linecolor=session_rust::Color::new(1.,0.,0.,1.);let owner=scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(source)).unwrap()).unwrap();
  let mut r=crate::cap_gpu_tests::renderer(&scene);r.set_density(density);retarget(&mut r,count);let matrix=crate::cap_gpu_tests::reversed(session_rust::Xform::identity().m.map(|v|v as f32));let actual=crate::cap_gpu_tests::pixels_at(&r,matrix);let visible=crate::cap_gpu_tests::at(&actual,208,96);assert!(visible[0]>240&&visible[1]<10&&visible[2]<10,"Actual wide source fringe is visible outside its query-axis halo: {visible:?}");
  r.request_pick([0.3,0.2],matrix).unwrap();crate::cap_gpu_tests::wait(&r);let picked=r.take_pick(matrix).unwrap().unwrap();crate::sample_test_proof::write(format!("cloud-ink-wide-fringe-{density}-{count}.rgba"),&actual).unwrap();crate::sample_test_proof::write(format!("cloud-ink-wide-fringe-{density}-{count}.json"),serde_json::to_vec_pretty(&serde_json::json!({"density":density,"samples":count,"pixel":visible,"freshOwner":picked,"expected":owner.value()})).unwrap()).unwrap();
  assert_eq!(picked.unwrap()[0],owner.value(),"A visible wide source fringe retains its original owner even when its axis lies outside the query halo");
 }}

 crate::sample_test_proof::write("cloud-ink-fixed.json",serde_json::to_vec_pretty(&rows).unwrap()).unwrap();
}
