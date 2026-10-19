use super::*;
use wgpu::util::DeviceExt;
const SIZE:[u32;2]=[128,96];
fn read(device:&wgpu::Device,queue:&wgpu::Queue,image:&wgpu::Texture,bytes:u32)->Vec<u8> {
 let pitch=SIZE[0]*bytes;
 let buffer=device.create_buffer(&wgpu::BufferDescriptor {label:Some("actual ink pixels"),size:u64::from(pitch*SIZE[1]),usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::MAP_READ,mapped_at_creation:false});
 let mut encoder=device.create_command_encoder(&Default::default());
 encoder.copy_texture_to_buffer(image.as_image_copy(),wgpu::TexelCopyBufferInfo {buffer:&buffer,layout:wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(SIZE[1])}},image.size());
 queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();buffer.slice(..).map_async(wgpu::MapMode::Read,move|v|tx.send(v).unwrap());
 device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();rx.recv().unwrap().unwrap();let pixels=buffer.slice(..).get_mapped_range().to_vec();buffer.unmap();pixels
}
fn output(device:&wgpu::Device,format:wgpu::TextureFormat)->wgpu::Texture {
 device.create_texture(&wgpu::TextureDescriptor {label:Some("resolved actual ink"),size:wgpu::Extent3d {width:SIZE[0],height:SIZE[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format,usage:wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::COPY_SRC,view_formats:&[]})
}
fn geometry_bytes(stride:u64)->Vec<u8> {
 let mut values=if stride==32 {vec![0.,0.,0.5,36.,1.,0.,0.,1.]}else{vec![-0.7,0.,0.5,0.7,0.,0.5,1.,0.,0.,1.,12.]};
 if stride==72 {values.extend([-0.7,0.,0.5,0.7,0.,0.5,f32::from_bits(3)]);}
 values.into_iter().flat_map(f32::to_ne_bytes).collect()
}
fn geometry(device:&wgpu::Device,stride:u64)->wgpu::Buffer {
 device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:Some("retained original ink vertex"),contents:&geometry_bytes(stride),usage:wgpu::BufferUsages::VERTEX})
}
fn occlusion(device:&wgpu::Device,queue:&wgpu::Queue,target:&crate::frame_targets::Targets,visible:u32) {
 let source=format!(r#"
@vertex fn vertex(@builtin(vertex_index) i:u32)->@builtin(position) vec4f {{
 let p=array<vec2f,3>(vec2f(-1,-1),vec2f(3,-1),vec2f(-1,3));return vec4f(p[i],0.5,1);
}}
struct Hit {{@location(0) primitive:vec2u,@builtin(frag_depth) depth:f32}}
@fragment fn fragment(@builtin(sample_index) sample:u32)->Hit {{
 return Hit(vec2u(0),select(0.9,0.0,sample<{visible}u));
}}"#);
 let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("different real depth in each sample"),source:wgpu::ShaderSource::Wgsl(source.into())});
 let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("actual ink occluder"),layout:None,vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(wgpu::DepthStencilState {format:crate::depth::FORMAT,depth_write_enabled:Some(true),depth_compare:Some(wgpu::CompareFunction::Always),stencil:Default::default(),bias:Default::default()}),multisample:wgpu::MultisampleState {count:target.samples,..Default::default()},multiview_mask:None,cache:None});
 let mut encoder=device.create_command_encoder(&Default::default());{
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("fresh physical sample depths"),color_attachments:&[Some(wgpu::RenderPassColorAttachment {view:&target.primitive.view,resolve_target:None,depth_slice:None,ops:wgpu::Operations {load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment {view:&target.depth,depth_ops:Some(wgpu::Operations {load:wgpu::LoadOp::Clear(0.),store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&pipeline);pass.draw(0..3,0..1);
 }queue.submit([encoder.finish()]);
}
fn draw(device:&wgpu::Device,queue:&wgpu::Queue,ink:&Ink,target:&crate::frame_targets::Targets,tables:&Tables,view:&wgpu::BindGroup,clip:&wgpu::BindGroup,vertices:&wgpu::Buffer,flags:&wgpu::Buffer,stride:u64)->Vec<u8> {
 let image=output(device,wgpu::TextureFormat::Rgba8Unorm);let single=image.create_view(&Default::default());ink.set(device,tables,&target.depth);
 let mut encoder=device.create_command_encoder(&Default::default());{
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("real finite colour samples"),color_attachments:&[Some(wgpu::RenderPassColorAttachment {view:target.color(&single),resolve_target:None,depth_slice:None,ops:wgpu::Operations {load:wgpu::LoadOp::Clear(wgpu::Color::WHITE),store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment {view:&target.depth,depth_ops:None,stencil_ops:None}),..Default::default()});pass.set_bind_group(2,clip,&[]);assert!(ink.draw(&mut pass,view,vertices,flags,1,if stride==72 {18}else{6}));
 }target.resolve(&mut encoder,&single);queue.submit([encoder.finish()]);read(device,queue,&image,4)
}
fn query(device:&wgpu::Device,queue:&wgpu::Queue,ink:&Ink,target:&crate::frame_targets::Targets,tables:&Tables,view:&wgpu::BindGroup,clip:&wgpu::BindGroup,vertices:&wgpu::Buffer,flags:&wgpu::Buffer,stride:u64)->[u32;2] {
 let image=output(device,wgpu::TextureFormat::Rg32Uint);let single=image.create_view(&Default::default());
 let ids=device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:None,contents:&[17u32,1].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>(),usage:wgpu::BufferUsages::VERTEX});
 let mut bytes=vec![0u8;80];for i in 0..4 {bytes[i*20..i*20+4].copy_from_slice(&1f32.to_ne_bytes());}
 let crop=device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:None,contents:&bytes,usage:wgpu::BufferUsages::UNIFORM});let crop=crate::gpu_layout::group(device,&crate::gpu_layout::uniform(device,80),&crop);
 let mut encoder=device.create_command_encoder(&Default::default());{
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("fresh independent single sample owner query"),color_attachments:&[Some(wgpu::RenderPassColorAttachment {view:&single,resolve_target:None,depth_slice:None,ops:wgpu::Operations {load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment {view:&target.depth,depth_ops:None,stencil_ops:None}),..Default::default()});pass.set_bind_group(3,clip,&[]);ink.pick(device,&mut pass,view,&crop,vertices,&ids,flags,1,if stride==72 {18}else{6},tables,&target.depth);
 }queue.submit([encoder.finish()]);let pixels=read(device,queue,&image,8);let at=(48*128+64)*8;[u32::from_ne_bytes(pixels[at..at+4].try_into().unwrap()),u32::from_ne_bytes(pixels[at+4..at+8].try_into().unwrap())]
}
#[test]
fn actual_four_sample_ink_reads_each_depth_and_keeps_queries_independent() {
 let renderer=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let device=&renderer.device;let queue=&renderer.queue;
 let buffer=|bytes:&[u8],usage|device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:None,contents:bytes,usage});
 let projected=buffer(&[0;96],wgpu::BufferUsages::STORAGE);let tiles=buffer(&[0;32],wgpu::BufferUsages::STORAGE);
 let matrix=session_rust::Xform::identity().m.map(|v|v as f32);
 let uniform=buffer(&crate::surface_view::bytes(matrix,[0.,1.,0.,0.],SIZE,1.),wgpu::BufferUsages::UNIFORM);let view=crate::gpu_layout::group(device,&crate::surface_view::layout(device),&uniform);
 let clipping=crate::clipping_gpu::Clipping::new(device);
 let one=crate::frame_targets::Targets::new(device,SIZE,wgpu::TextureFormat::Rgba8Unorm,1);let four=crate::frame_targets::Targets::new(device,SIZE,wgpu::TextureFormat::Rgba8Unorm,4);
 let table=|target:&crate::frame_targets::Targets|Tables {projected:projected.clone(),tiles:tiles.clone(),primitive:target.primitive.view.clone()};
 let path=std::path::Path::new("ink-pixels");
 for (case,(stride,grid,varying)) in [(44,false,false),(72,false,false),(32,false,false),(44,true,false),(44,false,true),(32,false,true)].into_iter().enumerate() {
  let vertices=geometry(device,stride);let flags=buffer(&[0;8],wgpu::BufferUsages::VERTEX);let mut ink=Ink::with_samples(device,wgpu::TextureFormat::Rgba8Unorm,stride,grid,varying,4);
  let mut mixed=Vec::new();
  for visible in 0..=4 {
   occlusion(device,queue,&four,visible);let pixels=draw(device,queue,&ink,&four,&table(&four),&view,&clipping.group,&vertices,&flags,stride);let centre=&pixels[(48*128+64)*4..(48*128+65)*4];let expected=(255.*(1.-visible as f32/4.)).round()as i32;
   assert_eq!(centre[0],255,"family{case} visible{visible}");assert!((i32::from(centre[1])-expected).abs()<=1,"family{case} visible{visible}: actual{centre:?}, expected green{expected}");assert_eq!(centre[1],centre[2]);
   if visible==2 {crate::sample_test_proof::write(path.join(format!("family-{case}.rgba")),&pixels).unwrap();mixed=pixels;}
  }
  occlusion(device,queue,&one,1);assert_eq!(query(device,queue,&ink,&one,&table(&one),&view,&clipping.group,&vertices,&flags,stride),[17,1],"query uses fresh1x depth while4x colour depth is unrelated");
  occlusion(device,queue,&one,0);assert_eq!(query(device,queue,&ink,&one,&table(&one),&view,&clipping.group,&vertices,&flags,stride),[0,0],"fresh1x occlusion controls query ownership");
  occlusion(device,queue,&four,2);ink.set_samples(device,1);occlusion(device,queue,&one,1);let single=draw(device,queue,&ink,&one,&table(&one),&view,&clipping.group,&vertices,&flags,stride);assert_eq!(&single[(48*128+64)*4..(48*128+65)*4],&[255,0,0,255]);
  ink.set_samples(device,4);let returned=draw(device,queue,&ink,&four,&table(&four),&view,&clipping.group,&vertices,&flags,stride);assert!(returned==mixed,"family{case}:4→1→4 returns every actual colour pixel exactly");
 }
}
