use crate::{scene::Scene,renderer::Renderer,viewport::Viewport};
fn renderer(scene: &Scene) -> Renderer {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: wgpu::Backends::VULKAN, flags: Default::default(), memory_budget_thresholds: Default::default(), backend_options: Default::default(), display: None });
    let adapter = pollster::block_on(instance.request_adapter(&Default::default())).expect("Actual GPU adapter");
    let (device, queue) = pollster::block_on(adapter.request_device(&Default::default())).expect("Actual GPU device");
    let mut renderer = Renderer::new(device, queue, wgpu::TextureFormat::Rgba8Unorm, scene);
    renderer.resize(Viewport { width: 320, height: 240 }); renderer
}

pub(crate) fn pixels(renderer: &mut Renderer, transform:&[f32;16], size:[u32;2]) -> Vec<u8> {
    pixels_format(renderer,transform,size,wgpu::TextureFormat::Rgba8Unorm)
}
pub(crate) fn pixels_format(renderer: &mut Renderer, transform:&[f32;16], size:[u32;2],format:wgpu::TextureFormat) -> Vec<u8> {
    let texture = renderer.device.create_texture(&wgpu::TextureDescriptor { label: Some("selection pixels"),
        size: wgpu::Extent3d { width: size[0], height: size[1], depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1,
        dimension: wgpu::TextureDimension::D2, format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC, view_formats: &[] });
    renderer.draw_mode(&texture.create_view(&Default::default()), &Default::default(), transform, false);
    let bytes = renderer.device.create_buffer(&wgpu::BufferDescriptor { label: Some("selection readback"), size: (u64::from(size[0])*4).div_ceil(256)*256*u64::from(size[1]),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
    let mut encoder = renderer.device.create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(texture.as_image_copy(), wgpu::TexelCopyBufferInfo { buffer: &bytes,
        layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some((size[0]*4).div_ceil(256)*256), rows_per_image: Some(size[1]) } }, texture.size());
    renderer.queue.submit([encoder.finish()]); let (send, receive) = std::sync::mpsc::channel();
    bytes.slice(..).map_async(wgpu::MapMode::Read, move |result| { let _ = send.send(result); });
    renderer.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: Some(std::time::Duration::from_secs(5)) }).unwrap();
    receive.recv_timeout(std::time::Duration::from_secs(5)).unwrap().unwrap();
    let mapped=bytes.slice(..).get_mapped_range(); let pitch=(size[0]*4).div_ceil(256)*256; let result=mapped.chunks_exact(pitch as usize).flat_map(|row|row[..size[0] as usize*4].iter().copied()).collect();drop(mapped); bytes.unmap(); result
}

#[test]
fn all_handles_have_finite_twenty_byte_vertices_and_layouts_keep_css_size() {
    let vertices=crate::widget_mesh::vertices();assert!(vertices.len()<20_000);let mut handles=[false;10];
    for vertex in vertices {handles[vertex.handle as usize]=true;assert_eq!(vertex.bytes().len(),20);assert!(vertex.position.iter().all(|v|v.is_finite() && v.abs()<=98.));}assert!(handles.into_iter().all(|v|v));
    let mut camera=crate::camera::Camera::default();camera.isometric();
    for projection in [crate::camera::Projection::Perspective,crate::camera::Projection::Orthographic] {camera.projection=projection;
        for distance in [3.,9.,30.] {camera.distance=distance;let matrix=camera.view_projection();let scale=crate::widget_layout::world_per_pixel(&matrix,[0.;3],480.).unwrap();
            let centre=matrix.transform_point(&session_rust::Point::new(0.,0.,0.));let world=camera.orientation.rotate_vector(session_rust::Vector::z_axis())*(96.*scale);let tip=matrix.transform_point(&session_rust::Point::new(world[0],world[1],world[2]));assert!(((tip[1]-centre[1])*240.-96.).abs()<1e-9);
            for density in [1u32,2] {let layout=crate::widget_layout::layout(&matrix,[0.;3],scale,[640*density,480*density],-1.).unwrap();assert!(layout.extent.into_iter().all(|v|v<=1024 && v%64==0));}
        }
    }
    assert!(crate::widget_layout::world_per_pixel(&session_rust::Xform::identity(),[0.;3],0.).is_none());
    let mut invalid=session_rust::Xform::identity();invalid.m[15]=-1.;assert!(crate::widget_layout::layout(&invalid,[0.;3],1.,[640,480],-1.).is_none());
}

#[test]
fn actual_widget_gpu_draws_over_geometry_reuses_tile_and_never_creates_pick_owners() {
    let mut scene=Scene::demo();scene.clear();let mut renderer=renderer(&scene);let mut camera=crate::camera_defaults_tests::top_fixture();camera.isometric();camera.aspect=320./240.;
    for projection in [crate::camera::Projection::Perspective,crate::camera::Projection::Orthographic] {camera.projection=projection;
        let inverse=camera.view_projection().inverse().unwrap();
        let points=[[-1.,-1.,0.001],[3.,-1.,0.001],[-1.,3.,0.001]].map(|p| {let p=inverse.transform_point(&session_rust::Point::new(p[0],p[1],p[2]));[p[0],p[1],p[2]]});
        scene.clear();scene.insert(crate::prepared::PreparedMesh::triangle(points,[0.5;3]).unwrap()).unwrap();renderer.set_scene(&scene,None);
        let stats=renderer.stats();let pick=renderer.pick_usage();
        for density in [1u32,2] {let size=[320*density,240*density];renderer.resize(Viewport {width:size[0],height:size[1]});renderer.set_density(density as f32);
            renderer.set_gizmo(&camera,Some([0.;3]),-1.);let usage=renderer.gizmo_usage();assert!(usage[0]<400_100 && usage[1]<=1024*1024*36 && usage[1]>0);
            let image=pixels(&mut renderer,&camera.uniform(),size);let mut colours=[0;3];
            for p in image.chunks_exact(4) {for i in 0..3 {if p[i]>140 && p[(i+1)%3]<130 && p[(i+2)%3]<130 {colours[i]+=1;}}}
            assert!(colours.into_iter().all(|n|n>100),"Three visible handle colours: {colours:?}");
            renderer.set_gizmo(&camera,Some([0.;3]),0.);assert_eq!(renderer.gizmo_usage()[2],usage[2],"Hover reuses the tile");let active=pixels(&mut renderer,&camera.uniform(),size);assert_ne!(active,image);
            let orange=active.chunks_exact(4).filter(|p|p[0]>180 && p[1]>100 && p[1]<210 && p[2]<90).count();assert!(orange>100);
            assert_eq!(renderer.stats(),stats);assert_eq!(renderer.pick_usage(),pick);
            renderer.set_gizmo(&camera,None,-1.);assert_eq!(renderer.gizmo_usage()[1],0);assert!(renderer.gizmo_extent().is_none());
        }
    }
}
