use crate::text_gpu_support::GpuCtx;
use std::rc::Rc;
fn mesh(center:f32,color:[f32;3])->Rc<crate::mesh::Mesh>{
 Rc::new(crate::mesh::Mesh::new(vec![[center-0.4,-0.6,0.,color[0],color[1],color[2]],[center+0.4,-0.6,0.,color[0],color[1],color[2]],[center,0.6,0.,color[0],color[1],color[2]]],vec![0,1,2]).unwrap())
}
#[test]
fn actual_gpu_arena_appends_exact_records_reuses_identity_and_releases(){
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut arena=crate::mesh_arena::Arena::new(&ctx);
 let a=mesh(-0.45,[1.,0.,0.]);let b=mesh(0.45,[0.,1.,0.]);let first=arena.upload(&ctx,&a).unwrap();let revision=arena.revision;
 assert_eq!(arena.upload(&ctx,&a).unwrap(),first);assert_eq!(arena.revision,revision);
 let second=arena.upload(&ctx,&b).unwrap();assert_eq!(second.vertices,3..6);assert_eq!(second.indices,3..6);assert!(arena.revision>revision);
 assert_eq!(crate::growing_buffer_tests::read(&ctx,&arena.vertices.buf,120),[crate::packed_vertex::bytes(&a),crate::packed_vertex::bytes(&b)].concat());
 assert_eq!(crate::growing_buffer_tests::read(&ctx,&arena.indices.buf,24),arena.rows.index_bytes());
 let allocated=arena.allocated_bytes();let revision=arena.revision;arena.reset();assert_eq!(arena.rows.source_count(),0);assert!(arena.vertices.is_empty());assert!(arena.indices.is_empty());
 assert_eq!(arena.upload(&ctx,&b).unwrap().vertices,0..3);assert_eq!(arena.allocated_bytes(),allocated);assert_eq!(arena.revision,revision);
 arena.release(&ctx);assert_eq!(arena.allocated_bytes(),24);assert!(arena.vertices.is_empty());assert!(arena.indices.is_empty());
}
fn render(ctx:&GpuCtx,arena:&crate::mesh_arena::Arena,group:&wgpu::BindGroup)->Vec<u8>{
 let shader=ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("arena binding fixture"),source:wgpu::ShaderSource::Wgsl(format!("{}\n{}",include_str!("packed_vertex.wgsl"),r#"
 @group(0) @binding(0) var<storage,read> vertices:array<PackedVertex>;
 @group(0) @binding(2) var<storage,read> indices:array<u32>;
 struct Out{@builtin(position) position:vec4<f32>,@location(0) color:vec4<f32>}
 @vertex fn vertex(@builtin(vertex_index) occurrence:u32)->Out{
  let v=vertices[indices[occurrence]];return Out(vec4<f32>(packed_position(v),1.),unpack4x8unorm(v.color));
 }
 @fragment fn fragment(input:Out)->@location(0) vec4<f32>{return input.color;}
 "#).into())});
 let layout=ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:None,bind_group_layouts:&[Some(&arena.layout)],immediate_size:0});
 let pipeline=ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:None,layout:Some(&layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[]},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::Rgba8Unorm,blend:None,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:None,multisample:Default::default(),multiview_mask:None,cache:None});
 let texture=ctx.device.create_texture(&wgpu::TextureDescriptor{label:Some("actual arena colors"),size:wgpu::Extent3d{width:64,height:64,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::COPY_SRC,view_formats:&[]});let view=texture.create_view(&Default::default());
 let mut encoder=ctx.device.create_command_encoder(&Default::default());{
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::WHITE),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&pipeline);pass.set_bind_group(0,group,&[]);pass.draw(0..arena.indices.len(),0..1);
 }
 let output=ctx.device.create_buffer(&wgpu::BufferDescriptor{label:None,size:64*256,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::COPY_SRC,mapped_at_creation:false});
 encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&output,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(256),rows_per_image:Some(64)}},texture.size());ctx.queue.submit([encoder.finish()]);crate::growing_buffer_tests::read(ctx,&output,64*256)
}
#[test]
fn actual_arena_draw_rebuilds_immutable_bindings_after_growth(){
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut arena=crate::mesh_arena::Arena::new(&ctx);
 let a=mesh(-0.45,[1.,0.,0.]);let b=mesh(0.45,[0.,1.,0.]);arena.upload(&ctx,&a).unwrap();let old=arena.group.clone();arena.upload(&ctx,&b).unwrap();
 let fresh=render(&ctx,&arena,&arena.group);let left=(32*64+16)*4;let right=(32*64+48)*4;
 assert_eq!(&fresh[left..left+4],&[255,0,0,255]);assert_eq!(&fresh[right..right+4],&[0,255,0,255]);
 let stale=render(&ctx,&arena,&old);assert_eq!(&stale[left..left+4],&[255,0,0,255]);assert_eq!(&stale[right..right+4],&[255;4],"An old binding cannot read the appended right-hand range");
}

#[test]
fn actual_gpu_arena_reuses_ranges_preserves_bindings_and_counts_exact_writes(){
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut arena=crate::mesh_arena::Arena::new(&ctx);let a=mesh(-0.45,[1.,0.,0.]);let b=mesh(0.45,[0.,1.,0.]);let history=Rc::clone(&a);let first=arena.upload(&ctx,&a).unwrap();let survivor=arena.upload(&ctx,&b).unwrap();let group=arena.group.clone();let buffers=[arena.vertices.buf.clone(),arena.indices.buf.clone()];let revision=arena.revision;assert_eq!((arena.uploads,arena.uploaded_bytes),(2,144));
 assert!(arena.rows.retire(Rc::as_ptr(&a)as usize));let c=mesh(-0.45,[0.,0.,1.]);assert_eq!(arena.upload(&ctx,&c).unwrap(),first);assert_eq!(arena.upload(&ctx,&b).unwrap(),survivor);assert_eq!(arena.group,group);assert_eq!(arena.revision,revision);assert_eq!([arena.vertices.buf.clone(),arena.indices.buf.clone()],buffers);assert_eq!((arena.uploads,arena.uploaded_bytes),(3,216));assert_eq!(history.indices(),&[0,1,2]);
 assert_eq!(crate::growing_buffer_tests::read(&ctx,&arena.vertices.buf,120),[crate::packed_vertex::bytes(&c),crate::packed_vertex::bytes(&b)].concat());assert_eq!(crate::growing_buffer_tests::read(&ctx,&arena.indices.buf,24),[0u32,1,2,3,4,5].into_iter().flat_map(u32::to_le_bytes).collect::<Vec<_>>());let image=render(&ctx,&arena,&group);let left=(32*64+16)*4;let right=(32*64+48)*4;let center=(32*64+32)*4;assert_eq!(&image[left..left+4],&[0,0,255,255]);assert_eq!(&image[right..right+4],&[0,255,0,255]);
 let d=mesh(0.,[1.,1.,0.]);arena.upload(&ctx,&d).unwrap();assert_ne!(arena.group,group);assert!(arena.revision>revision);assert_eq!((arena.uploads,arena.uploaded_bytes),(4,288));let fresh=render(&ctx,&arena,&arena.group);let stale=render(&ctx,&arena,&group);assert_eq!(&fresh[center..center+4],&[255,255,0,255]);assert_eq!(&stale[center..center+4],&[255;4]);assert_eq!(&stale[left..left+4],&[0,0,255,255]);
 let before=crate::growing_buffer_tests::read(&ctx,&arena.vertices.buf,180);assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||arena.vertices.write_or_append(&ctx,u32::MAX,&[0;20]))).is_err());assert_eq!(crate::growing_buffer_tests::read(&ctx,&arena.vertices.buf,180),before);assert!(!arena.vertices.write_or_append(&ctx,9,&[]));
}
