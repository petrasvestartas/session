use super::*;
use std::rc::Rc;
fn surface_frame(r:&Renderer,target:&crate::frame_targets::Targets,shared:bool,sections:bool)->Vec<u8> {
 let output=r.device.create_texture(&wgpu::TextureDescriptor {label:Some("actual multisampled original surfaces"),size:wgpu::Extent3d {width:320,height:240,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:r.format,usage:wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::COPY_SRC,view_formats:&[]});let view=output.create_view(&Default::default());
 let matrix=if sections {crate::cap_gpu_tests::matrix()}else{crate::cap_gpu_tests::reversed(session_rust::Xform::identity().m.map(|v|v as f32))};r.queue.write_buffer(&r.uniform,0,&crate::surface_view::bytes(matrix,[0.,1.,0.,0.],[320,240],1.));
 r.clipping.borrow_mut().frame_anchored(&r.queue,matrix,r.anchor(),240,1.);
 if sections {r.caps.borrow_mut().prepare(&r.device,&r.queue,r.format,[320,240],true);}
 let mut encoder=r.device.create_command_encoder(&Default::default());{
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("original and shared surface count witness"),color_attachments:&[Some(wgpu::RenderPassColorAttachment {view:target.color(&view),resolve_target:None,depth_slice:None,ops:wgpu::Operations {load:wgpu::LoadOp::Clear(wgpu::Color::WHITE),store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment {view:&target.primitive.view,resolve_target:None,depth_slice:None,ops:wgpu::Operations {load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment {view:&target.depth,depth_ops:Some(wgpu::Operations {load:wgpu::LoadOp::Clear(0.),store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});
  pass.set_bind_group(0,&r.view_group,&[]);pass.set_bind_group(2,&r.clipping.borrow().group,&[]);
  for mesh in r.meshes.iter().filter(|mesh|!shared||!r.instance_rows.contains(&mesh.id)) {pass.set_pipeline(&r.surfaces.active.0);r.instances.bind_own(&mut pass);mesh.draw_pulled(&mut pass);}
  if shared {r.instances.draw(&mut pass);}
 }
 if sections {let clip=r.clipping.borrow();r.caps.borrow_mut().draw(&r.device,&mut encoder,target.color(&view),&target.primitive.view,&target.depth,&r.view_group,&clip.group,&r.meshes,clip.rows.len()as u32,Some(&r.instances));}
 target.resolve(&mut encoder,&view);
 let buffer=r.device.create_buffer(&wgpu::BufferDescriptor {label:None,size:320*240*4,usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});encoder.copy_texture_to_buffer(output.as_image_copy(),wgpu::TexelCopyBufferInfo {buffer:&buffer,layout:wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(1280),rows_per_image:Some(240)}},output.size());r.queue.submit([encoder.finish()]);
 let(tx,rx)=std::sync::mpsc::channel();buffer.slice(..).map_async(wgpu::MapMode::Read,move|v|tx.send(v).unwrap());crate::cap_gpu_tests::wait(r);rx.recv().unwrap().unwrap();let pixels=buffer.slice(..).get_mapped_range().to_vec();buffer.unmap();pixels
}
fn actual_physical_samples(r:&Renderer,target:&crate::frame_targets::Targets)->usize {
 let module=r.device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("read every physical original sample"),source:wgpu::ShaderSource::Wgsl(r#"
@group(0) @binding(0) var primitive:texture_multisampled_2d<u32>;
@group(0) @binding(1) var depth:texture_depth_multisampled_2d;
@group(0) @binding(2) var<storage,read_write> output:array<vec4u>;
@compute @workgroup_size(64) fn read_samples(@builtin(global_invocation_id) id:vec3u) {
 if(id.x>=320u*240u*4u){return;}let p=id.x/4u;let at=vec2i(i32(p%320u),i32(p/320u));let sample=id.x%4u;
 output[id.x]=vec4u(textureLoad(primitive,at,i32(sample)).xy,bitcast<u32>(textureLoad(depth,at,i32(sample))),sample);
}"#.into())});
 let pipeline=r.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {label:None,layout:None,module:&module,entry_point:Some("read_samples"),compilation_options:Default::default(),cache:None});
 let size=320*240*4*16;let output=r.device.create_buffer(&wgpu::BufferDescriptor {label:None,size,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_SRC,mapped_at_creation:false});let read=r.device.create_buffer(&wgpu::BufferDescriptor {label:None,size,usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});
 let group=r.device.create_bind_group(&wgpu::BindGroupDescriptor {label:None,layout:&pipeline.get_bind_group_layout(0),entries:&[wgpu::BindGroupEntry {binding:0,resource:wgpu::BindingResource::TextureView(&target.primitive.view)},wgpu::BindGroupEntry {binding:1,resource:wgpu::BindingResource::TextureView(&target.depth)},wgpu::BindGroupEntry {binding:2,resource:output.as_entire_binding()}]});let mut encoder=r.device.create_command_encoder(&Default::default());{
  let mut pass=encoder.begin_compute_pass(&Default::default());pass.set_pipeline(&pipeline);pass.set_bind_group(0,&group,&[]);pass.dispatch_workgroups((320*240*4u32).div_ceil(64),1,1);
 }encoder.copy_buffer_to_buffer(&output,0,&read,0,size);r.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|v|tx.send(v).unwrap());crate::cap_gpu_tests::wait(r);rx.recv().unwrap().unwrap();let bytes=read.slice(..).get_mapped_range();let mut mixed=0;
 for pixel in bytes.chunks_exact(64) {let mut addresses=[0;4];for (i,sample) in pixel.chunks_exact(16).enumerate(){let words=std::array::from_fn::<_,4,_>(|j|u32::from_ne_bytes(sample[j*4..j*4+4].try_into().unwrap()));assert_eq!(words[3],i as u32);let address=words[0]|(words[1]<<16);let depth=f32::from_bits(words[2]);assert_eq!(depth,if address==0 {0.}else{1.});addresses[i]=address;}if addresses.iter().any(|a|*a==0)&&addresses.iter().any(|a|*a!=0){mixed+=1;}}
 drop(bytes);read.unmap();assert!(mixed>100,"Actual original silhouettes have mixed covered and uncovered samples");mixed
}
#[test]
fn original_and_shared_meshes_switch_samples_without_source_or_geometry_rebuilds() {
 let mut scene=Scene::empty();crate::scene_document::import(&mut scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();scene.insert(crate::prepared::PreparedMesh::triangle([[-0.3,-0.85,0.],[0.3,-0.85,0.],[0.,-0.3,0.]],[0.1,0.8,0.3]).unwrap()).unwrap();
 let mut r=crate::cap_gpu_tests::renderer(&scene);let geometry=r.meshes.iter().map(|m|Rc::clone(&m.geometry)).collect::<Vec<_>>();let sources=scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>();let stats=r.stats();let usage=r.instance_usage();
 let one=crate::frame_targets::Targets::new(&r.device,[320,240],r.format,1);let four=crate::frame_targets::Targets::new(&r.device,[320,240],r.format,4);let first=surface_frame(&r,&one,true,false);assert!(first.chunks_exact(4).filter(|p|p[..3]!=[255;3]).count()>5000);let surface1=r.surfaces.active.0.clone();
 r.surfaces.select(4,||crate::surface_pipelines::pipelines(&r.device,r.format,&crate::surface_view::layout(&r.device),&crate::gpu_layout::uniform(&r.device,crate::object_record::BYTES as u64),4));r.instances.set_samples(&r.device,r.format,4);let surface4=r.surfaces.active.0.clone();assert_ne!(surface1,surface4);let shared=surface_frame(&r,&four,true,false);assert!(surface_frame(&r,&four,false,false)==shared,"Original uniforms and shared records have identical4x colour");let mixed=actual_physical_samples(&r,&four);assert!(first!=shared,"Actual colour changes on silhouette coverage");
 for count in [4,1,4] {r.surfaces.select(count,||panic!("A retained surface variant must not compile again"));r.instances.set_samples(&r.device,r.format,count);let pixels=surface_frame(&r,if count==1 {&one}else{&four},true,false);assert!(pixels==if count==1 {first.clone()}else{shared.clone()});assert_eq!(r.surfaces.active.0,if count==1 {surface1.clone()}else{surface4.clone()});}
 let matrix=crate::cap_gpu_tests::reversed(session_rust::Xform::identity().m.map(|v|v as f32));for (i,x) in [-0.6,0.6].into_iter().enumerate(){r.request_pick([x,0.],matrix).unwrap();crate::cap_gpu_tests::wait(&r);let pick=r.take_pick(matrix).unwrap().unwrap().unwrap();assert_eq!(pick[0],scene.objects()[i].id.value(),"Fresh owner queries remain single-sample after surface4x switching");}
 assert_eq!(r.stats(),stats);assert_eq!(r.instance_usage(),usage);for ((gpu,before),source) in r.meshes.iter().zip(&geometry).zip(&sources){assert!(Rc::ptr_eq(&gpu.geometry,before));let row=scene.objects().iter().find(|o|o.id==gpu.id).unwrap();assert_eq!(&row.geometry().unwrap().to_proto(),source);}
 let root=std::path::Path::new("surface-pixels");crate::sample_test_proof::write(root.join("one.rgba"),first).unwrap();crate::sample_test_proof::write(root.join("four.rgba"),shared).unwrap();crate::sample_test_proof::write(root.join("physical.json"),format!("{{\"actualMixedCoveragePixels\":{mixed}}}\n")).unwrap();
}

#[test]
fn actual_section_caps_retarget_crossings_and_colour_without_changing_originals() {
 let (scene,owner)=crate::cap_gpu_tests::scene(false);let mut r=crate::cap_gpu_tests::renderer(&scene);
 let geometry=r.meshes.iter().map(|m|Rc::clone(&m.geometry)).collect::<Vec<_>>();let stats=r.stats();let sources=scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>();
 let one=crate::frame_targets::Targets::new(&r.device,[320,240],r.format,1);let four=crate::frame_targets::Targets::new(&r.device,[320,240],r.format,4);
 let first=surface_frame(&r,&one,true,true);let surface1=r.surfaces.active.0.clone();
 let root=std::path::Path::new("cap-pixels");
 let mut four_pixels=Vec::new();
 for count in [4,1,4] {
  r.surfaces.select(count,||crate::surface_pipelines::pipelines(&r.device,r.format,&crate::surface_view::layout(&r.device),&crate::gpu_layout::uniform(&r.device,crate::object_record::BYTES as u64),count));r.instances.set_samples(&r.device,r.format,count);r.caps.borrow_mut().set_samples(count);
  let pixels=surface_frame(&r,if count==1 {&one}else{&four},true,true);let centre=crate::cap_gpu_tests::at(&pixels,208,96);assert!(centre[..3].iter().all(|v|(150..=155).contains(v)),"Actual closed cap remains grey at{count}x: {centre:?}");
  if count==1 {assert!(pixels==first);assert_eq!(r.surfaces.active.0,surface1);}else if four_pixels.is_empty(){four_pixels=pixels;}else{assert!(pixels==four_pixels);}
 }
 crate::sample_test_proof::write(root.join("solid-four.rgba"),&four_pixels).unwrap();r.set_clipping(true,false);let hatch=surface_frame(&r,&four,true,true);assert!(hatch!=four_pixels);assert!(hatch.chunks_exact(4).filter(|p|p[..3].iter().all(|v|*v<20)).count()>400);crate::sample_test_proof::write(root.join("hatch-four.rgba"),hatch).unwrap();
 assert_eq!(r.stats(),stats);for ((gpu,before),source)in r.meshes.iter().zip(&geometry).zip(&sources){assert!(Rc::ptr_eq(&gpu.geometry,before));assert_eq!(&scene.objects().iter().find(|o|o.id==gpu.id).unwrap().geometry().unwrap().to_proto(),source);}
 r.select_objects(&scene,&[owner]).unwrap();let selected=surface_frame(&r,&four,true,true);let centre=crate::cap_gpu_tests::at(&selected,208,96);assert!(centre[0]>240&&centre[1]>240&&centre[2]<20);crate::sample_test_proof::write(root.join("selected-four.rgba"),selected).unwrap();
 let matrix=crate::cap_gpu_tests::matrix();r.request_pick([0.3,0.2],matrix).unwrap();crate::cap_gpu_tests::wait(&r);assert_eq!(r.take_pick(matrix).unwrap().unwrap().unwrap()[0],owner.value(),"Single-sample closed-cap owner query is independent of4x crossing resources");
}
