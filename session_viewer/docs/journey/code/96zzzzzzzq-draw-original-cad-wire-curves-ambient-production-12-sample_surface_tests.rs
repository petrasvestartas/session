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


fn retarget_basic_fixture(r:&mut Renderer,count:u32) {r.set_samples(count)}

#[test]
fn cached_clouds_and_original_sheets_draw_in_the_actual_four_sample_main_frame() {
 let mut scene=Scene::empty();let mesh=scene.insert(crate::prepared::PreparedMesh::triangle([[-0.7,-0.7,0.6],[0.7,-0.7,0.6],[0.,0.7,0.6]],[0.1,0.3,0.8]).unwrap()).unwrap();
 let mut original=session_rust::PointCloud::from_coords(vec![0.,0.,0.2],vec![255,0,0,255],vec![]);original.point_size=16.;let original=Rc::new(original);let cloud=scene.insert_cloud(crate::cloud::Prepared::new(original.clone()).unwrap()).unwrap();let cloud_before=original.to_proto();let sheet=Rc::new(crate::sheet_example::source());let drawing=scene.insert_sheet(Rc::new(crate::sheet_source::Prepared::new(sheet.clone()).unwrap())).unwrap();
 let mut r=crate::cap_gpu_tests::renderer(&scene);let transform=crate::cap_gpu_tests::reversed(session_rust::Xform::identity().m.map(|v|v as f32));let first=crate::cap_gpu_tests::pixels_at(&r,transform);let uploads=(r.stats()[0],r.sheet_usage()[3],r.cloud_usage()[2]);let preparations=r.cloud_usage()[3];let geometry=r.meshes.iter().map(|m|Rc::clone(&m.geometry)).collect::<Vec<_>>();
 let root=std::path::Path::new("sheet-cloud-pixels");crate::sample_test_proof::write(root.join("one.rgba"),&first).unwrap();let mut four=Vec::new();
 for count in [4,1,4] {
  retarget_basic_fixture(&mut r,count);let pixels=crate::cap_gpu_tests::pixels_at(&r,transform);let centre=crate::cap_gpu_tests::at(&pixels,160,120);assert!(centre[0]>240&&centre[1]<10&&centre[2]<10,"Original cloud correctly covers the face and sheet at{count}x");assert!(pixels.chunks_exact(4).filter(|p|p[..3].iter().all(|v|*v<20)).count()>400,"Drawing sheet remains visible");
  assert_eq!((r.stats()[0],r.sheet_usage()[3],r.cloud_usage()[2]),uploads);assert_eq!(r.cloud_usage()[3],preparations,"Changing main sample count must not rebuild cached cloud pixels");if count==1 {assert!(pixels==first);}else if four.is_empty(){four=pixels;}else{assert!(pixels==four);}
  for (at,id) in [([0.,0.],cloud),([0.,-0.5],drawing),([0.,-0.2],mesh)] {r.request_pick(at,transform).unwrap();crate::cap_gpu_tests::wait(&r);assert_eq!(r.take_pick(transform).unwrap().unwrap().unwrap()[0],id.value(),"Original owner after main{count}x switch");}
 }
 for (mesh,before)in r.meshes.iter().zip(&geometry){assert!(Rc::ptr_eq(&mesh.geometry,before));}assert_eq!(original.to_proto(),cloud_before);assert!(Rc::ptr_eq(&scene.sheets()[0].prepared.original,&sheet));crate::sample_test_proof::write(root.join("four.rgba"),four).unwrap();
}


#[test]
fn actual_text_sample_switches_keep_font_raster_sources_and_fresh_owner_queries() {
 use crate::{scene_label::{Prepared,example},text::TextPlacement};
 let mut scene=Scene::empty();let mesh=scene.insert(crate::prepared::PreparedMesh::triangle([[-0.8,-0.7,0.6],[0.8,-0.7,0.6],[0.,0.8,0.6]],[0.1,0.3,0.8]).unwrap()).unwrap();let mut r=crate::cap_gpu_tests::renderer(&scene);retarget_basic_fixture(&mut r,4);
 let mut label=example(TextPlacement::WorldPlane {world:[-0.5,0.25,0.2],right:[1.,0.,0.],up:[0.,1.,0.],world_height:0.18});label.label.text="Original".into();let original=Rc::new(label);scene.insert_text(Prepared::new(original.clone()).unwrap()).unwrap();
 let mut name=example(TextPlacement::Nameplate {world:[0.4,-0.35,0.2],padding:[8.,3.],rounded:true});name.label.text="Owner".into();let owner=scene.insert_text(Prepared::new(Rc::new(name)).unwrap()).unwrap();r.set_scene(&scene,None);let source=serde_json::to_string(&*original).unwrap();
 let mut overlay=crate::text_example::label();overlay.text="Sampled text".into();overlay.font_size=18.;r.set_text(vec![overlay]).unwrap();let transform=crate::cap_gpu_tests::reversed(session_rust::Xform::identity().m.map(|v|v as f32));let first=crate::cap_gpu_tests::pixels_at(&r,transform);assert_eq!(r.text_error(),None);let stats=r.text_stats().unwrap();assert!(stats.world_plane_rasterizations>0&&stats.raster_images>0);let usage=r.text_scene_usage();let gpu=r.stats();let geometry=r.meshes[0].geometry.clone();
 let root=std::path::Path::new("text-pixels");crate::sample_test_proof::write(root.join("four.rgba"),&first).unwrap();let mut single=Vec::new();
 for count in [4,1,4] {retarget_basic_fixture(&mut r,count);let pixels=crate::cap_gpu_tests::pixels_at(&r,transform);assert_eq!(r.text_error(),None);if count==4 {assert!(pixels==first);}else{single=pixels;}let current=r.text_stats().unwrap();assert_eq!(current.shape_count,stats.shape_count);assert_eq!(current.world_plane_rasterizations,stats.world_plane_rasterizations);assert_eq!(current.raster_images,stats.raster_images);assert_eq!(current.atlas_resets,stats.atlas_resets);assert_eq!(r.text_scene_usage(),usage);assert_eq!(r.stats(),gpu);assert!(Rc::ptr_eq(&r.meshes[0].geometry,&geometry));
  for(at,id)in [([0.4,-0.35],owner),([-0.2,-0.2],mesh)] {r.request_pick(at,transform).unwrap();crate::cap_gpu_tests::wait(&r);assert_eq!(r.take_pick(transform).unwrap().unwrap().unwrap()[0],id.value(),"Original1x owner query after{count}x text");}
 }
 assert_eq!(serde_json::to_string(&*original).unwrap(),source);crate::sample_test_proof::write(root.join("one.rgba"),single).unwrap();
}


#[test]
fn actual_four_sample_outline_and_section_masks_keep_single_blend_and_original_owners() {
 let (scene,owner)=crate::cap_gpu_tests::scene(false);let mut r=crate::cap_gpu_tests::renderer(&scene);r.select_objects(&scene,&[owner]).unwrap();r.set_outlines(true);let first=crate::cap_gpu_tests::pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().filter(|v|**v>0).count()>300);assert!(r.outline_alpha()==alpha);let usage=r.global_outline_usage()[0];let geometry=r.meshes.iter().map(|m|Rc::clone(&m.geometry)).collect::<Vec<_>>();let uploads=r.stats()[0];let original=scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>();let root=std::path::Path::new("outline-pixels");crate::sample_test_proof::write(root.join("selected-one.rgba"),&first).unwrap();let mut four=Vec::new();
 for count in [4,1,4] {retarget_basic_fixture(&mut r,count);let pixels=crate::cap_gpu_tests::pixels(&r);assert!(r.outline_alpha()==r.global_outline_alpha(),"Selected and global actual{count}x cap/face coverage remains identical");let expected=usage+if count==4 {4*320*240}else{0};assert_eq!(r.global_outline_usage()[0],expected,"Only the active4x coverage image adds memory;1x search images persist");assert_eq!(r.stats()[0],uploads);if count==1 {assert!(pixels==first);}else if four.is_empty(){four=pixels;}else{assert!(pixels==four);}
  let matrix=crate::cap_gpu_tests::matrix();r.request_pick([0.3,0.2],matrix).unwrap();crate::cap_gpu_tests::wait(&r);assert_eq!(r.take_pick(matrix).unwrap().unwrap().unwrap()[0],owner.value());
 }
 crate::sample_test_proof::write(root.join("selected-four.rgba"),&four).unwrap();r.select_objects(&scene,&[]).unwrap();let global=crate::cap_gpu_tests::pixels(&r);crate::sample_test_proof::write(root.join("global-four.rgba"),&global).unwrap();let alpha=r.global_outline_alpha();let mut boundary=0;for (i,coverage)in alpha.iter().enumerate(){if *coverage==255 {boundary+=1;assert!(global[i*4..i*4+3].iter().all(|v|*v<12));assert!(four[i*4..i*4+3].iter().all(|v|*v<12));}}assert!(boundary>300);
 for ((mesh,geometry),source)in r.meshes.iter().zip(&geometry).zip(&original){assert!(Rc::ptr_eq(&mesh.geometry,geometry));assert_eq!(&scene.objects().iter().find(|o|o.id==mesh.id).unwrap().geometry().unwrap().to_proto(),source);}
 r.set_clipping(false,true);let uncut=crate::cap_gpu_tests::pixels(&r);assert!(r.global_outline_alpha()==alpha,"Actual4x horizontal section and full solid share the same outer silhouette");crate::sample_test_proof::write(root.join("uncut-four.rgba"),uncut).unwrap();
}


fn read_ambient_samples(r:&Renderer,metadata:&std::collections::HashMap<u32,(f32,[u8;2])>)->serde_json::Value {
 let (properties,work,_)=r.ambient.borrow().as_ref().unwrap().actual_sample_inputs();
 let module=r.device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("actual four ambient source samples"),source:wgpu::ShaderSource::Wgsl(r#"
@group(0) @binding(0) var depths:texture_depth_multisampled_2d;
@group(0) @binding(1) var primitives:texture_multisampled_2d<u32>;
@group(0) @binding(2) var radii:texture_multisampled_2d<f32>;
@group(0) @binding(3) var normals:texture_multisampled_2d<f32>;
@group(0) @binding(9) var normals_y:texture_multisampled_2d<f32>;
@group(0) @binding(10) var normals_z:texture_multisampled_2d<f32>;
@group(0) @binding(4) var<storage,read_write> output:array<vec4u>;
@group(0) @binding(5) var<storage,read> corrections:array<u32>;
@group(0) @binding(6) var<storage,read> flags:array<u32>;
@group(0) @binding(7) var<storage,read> tiles:array<u32>;
@group(0) @binding(8) var<storage,read> draw:array<u32>;
@compute @workgroup_size(64) fn read_samples(@builtin(global_invocation_id) id:vec3u) {
 let i=id.x;if i>=320u*240u*4u{return;}let p=i/4u;let at=vec2i(i32(p%320u),i32(p/320u));let sample=i%4u;
 output[i*2u]=vec4u(textureLoad(primitives,at,i32(sample)).xy,bitcast<u32>(textureLoad(depths,at,i32(sample))),bitcast<u32>(textureLoad(radii,at,i32(sample)).x));
 let n=vec3f(textureLoad(normals,at,i32(sample)).x,textureLoad(normals_y,at,i32(sample)).x,textureLoad(normals_z,at,i32(sample)).x);
 let sum=abs(n.x)+abs(n.y)+abs(n.z);let e=n.xy/max(sum,1e-30);let folded=(1.-abs(e.yx))*select(vec2f(-1.),vec2f(1.),e>=vec2f(0.));let encoded=select(select(folded,e,n.z>=0.)*0.5+0.5,vec2f(0.),sum==0.);
 output[i*2u+1u]=vec4u(bitcast<vec2u>(encoded),corrections[p],flags[(p/320u/16u)*20u+(p%320u)/16u]);
 if i==0u {output[614400u]=vec4u(draw[0],draw[1],arrayLength(&flags),arrayLength(&tiles));output[614401u]=vec4u(draw[2],draw[3],0,0);}
 if i<300u {let tile=tiles[i];output[614402u+i]=vec4u(tile,flags[min(tile,299u)],0,0);}
}"#.into())});
 let pipeline=r.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {label:None,layout:None,module:&module,entry_point:Some("read_samples"),compilation_options:Default::default(),cache:None});let size=(614402+300)*16;
 let output=r.device.create_buffer(&wgpu::BufferDescriptor {label:None,size,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_SRC,mapped_at_creation:false});let read=r.device.create_buffer(&wgpu::BufferDescriptor {label:None,size,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::MAP_READ,mapped_at_creation:false});
 let views=[&r.targets.depth,&r.targets.primitive.view,&properties[0],&properties[1]];let mut entries=views.into_iter().enumerate().map(|(i,view)|wgpu::BindGroupEntry {binding:i as u32,resource:wgpu::BindingResource::TextureView(view)}).collect::<Vec<_>>();entries.push(wgpu::BindGroupEntry {binding:4,resource:output.as_entire_binding()});for (binding,view) in [(9,&properties[2]),(10,&properties[3])] {entries.push(wgpu::BindGroupEntry {binding,resource:wgpu::BindingResource::TextureView(view)});}for(i,buffer)in work.iter().enumerate(){entries.push(wgpu::BindGroupEntry {binding:5+i as u32,resource:buffer.as_entire_binding()});}
 let group=r.device.create_bind_group(&wgpu::BindGroupDescriptor {label:None,layout:&pipeline.get_bind_group_layout(0),entries:&entries});let mut encoder=r.device.create_command_encoder(&Default::default());{let mut pass=encoder.begin_compute_pass(&Default::default());pass.set_pipeline(&pipeline);pass.set_bind_group(0,&group,&[]);pass.dispatch_workgroups((320*240*4u32).div_ceil(64),1,1);}encoder.copy_buffer_to_buffer(&output,0,&read,0,size);r.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|v|tx.send(v).unwrap());crate::cap_gpu_tests::wait(r);rx.recv().unwrap().unwrap();let bytes=read.slice(..).get_mapped_range();let word=|at:usize|u32::from_ne_bytes(bytes[at*4..at*4+4].try_into().unwrap());let mut covered=0;let mut mixed=0;let mut distinct_radii=0;let mut corrected=0;
 for pixel in 0..320*240 {let mut ids=[0;4];let mut radii=[0;4];for sample in 0..4 {let at=(pixel*4+sample)*8;let id=word(at)|(word(at+1)<<16);let depth=f32::from_bits(word(at+2));let radius=f32::from_bits(word(at+3));let n=[f32::from_bits(word(at+4)),f32::from_bits(word(at+5))];assert!(depth.is_finite()&&(0.0..=1.0).contains(&depth));ids[sample]=id;radii[sample]=radius.to_bits();
  if id==0 {assert_eq!(depth,0.);assert_eq!(radius,0.);assert_eq!(n,[0.;2]);}else{covered+=1;let (expected,normal)=metadata.get(&id).unwrap_or_else(||panic!("Invented physical address{id}"));assert_eq!(radius,*expected,"Actual original radius at physical{id},sample{sample}");for j in 0..2 {assert!((((n[j]*255.).round()as i32)-i32::from(normal[j])).abs()<=1,"Actual geometric normal at physical{id},sample{sample}: {n:?} vs{normal:?}");}}
 }if ids.iter().any(|id|*id!=ids[0]){mixed+=1;}if radii.iter().filter(|r|**r!=0).any(|a|radii.iter().any(|b|*b!=0&&b!=a)){distinct_radii+=1;}if word(pixel*32+6)!=0{corrected+=1;}}
 let header=614400*4;let vertices=word(header);let instances=word(header+1);assert_eq!(vertices,6);assert!(instances>0&&instances<=300*4&&instances%4==0,"Real sparse correction draw: {instances}");assert_eq!(word(header+2),300);assert_eq!(word(header+3),300);assert_eq!([word(header+4),word(header+5)],[0,0]);let mut tiles=std::collections::HashSet::new();for i in 0..instances as usize/4 {let at=(614402+i)*4;let tile=word(at);assert!(tile<300&&tiles.insert(tile),"Unique bounded correction tiles");assert_eq!(word(at+1),1);}
 assert!(covered>10000&&mixed>100&&distinct_radii>0&&corrected>0,"Actual4samples: covered{covered},mixed{mixed},distinct original radii{distinct_radii},corrected{corrected}");drop(bytes);read.unmap();serde_json::json!({"coveredSamples":covered,"mixedPrimitivePixels":mixed,"mixedOriginalRadiusPixels":distinct_radii,"correctedPixels":corrected,"indirectInstances":instances,"uniqueEdgeTiles":tiles.len()})
}
fn ambient_face_metadata(r:&Renderer,scene:&Scene)->std::collections::HashMap<u32,(f32,[u8;2])> {
 let mut out=std::collections::HashMap::new();for mesh in &r.meshes {let object=scene.objects().iter().find(|o|o.id==mesh.id).unwrap();let radius=f32::from_ne_bytes(mesh.packed()[crate::object_record::RANGE+8..crate::object_record::RANGE+12].try_into().unwrap());assert!(radius>0.);for(i,tri)in mesh.geometry.source.indices().chunks_exact(3).enumerate(){let p=tri.iter().map(|index|{let v=mesh.geometry.source.vertices()[*index as usize];object.model.transform_point(&session_rust::Point::new(v[0]as f64,v[1]as f64,v[2]as f64))}).collect::<Vec<_>>();let a=std::array::from_fn::<_,3,_>(|j|p[1][j]-p[0][j]);let b=std::array::from_fn::<_,3,_>(|j|p[2][j]-p[0][j]);let n=[a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]];let sum=n.iter().map(|x|x.abs()).sum::<f64>();let e=[n[0]/sum,n[1]/sum];let e=if n[2]<0. {[(1.-e[1].abs())*if e[0]>=0.{1.}else{-1.},(1.-e[0].abs())*if e[1]>=0.{1.}else{-1.}]}else{e};let normal=e.map(|v|((v*0.5+0.5)*255.).round()as u8);assert!(out.insert(mesh.primitive_base()+1+i as u32,(radius,normal)).is_none());}}
 out
}

#[test]
fn actual_four_sample_ambient_reads_original_properties_and_executes_sparse_corrections() {
 let mut scene=Scene::empty();let mut big=crate::cap_gpu_tests::source(false);big.set_objectcolor(session_rust::Color::new(0.2,0.4,0.6,1.));scene.insert(crate::prepared::PreparedMesh::new(big).unwrap()).unwrap();let mut small=crate::cap_gpu_tests::source(false);small.transform(&(&session_rust::Xform::translation(0.4,0.25,0.35)*&session_rust::Xform::scale_xyz(0.45,0.45,0.45)));small.set_objectcolor(session_rust::Color::new(0.25,0.7,0.2,1.));scene.insert(crate::prepared::PreparedMesh::new(small).unwrap()).unwrap();
 let mut r=crate::cap_gpu_tests::renderer(&scene);retarget_basic_fixture(&mut r,4);r.set_ambient(true);let mut camera=crate::camera::Camera::default();camera.isometric();camera.aspect=4./3.;camera.target=[0.,0.,-0.3];camera.distance=3.;let transform=camera.uniform();let first=crate::cap_gpu_tests::pixels_at(&r,transform);assert_eq!(r.ambient_error(),None);let metadata=ambient_face_metadata(&r,&scene);let samples=read_ambient_samples(&r,&metadata);let (_,_,coarse)=r.ambient.borrow().as_ref().unwrap().actual_sample_inputs();let uploads=r.stats()[0];let geometry=r.meshes.iter().map(|m|Rc::clone(&m.geometry)).collect::<Vec<_>>();let original=scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>();
 let root=std::path::Path::new("ambient-pixels");crate::sample_test_proof::write(root.join("four.rgba"),&first).unwrap();crate::sample_test_proof::write(root.join("samples.json"),serde_json::to_vec_pretty(&samples).unwrap()).unwrap();let mut single=Vec::new();
 for count in [4,1,4] {retarget_basic_fixture(&mut r,count);let pixels=crate::cap_gpu_tests::pixels_at(&r,transform);assert_eq!(r.ambient_error(),None);let (_,_,retained)=r.ambient.borrow().as_ref().unwrap().actual_sample_inputs();assert_eq!(retained,coarse,"Changing count retains all actual coarse ambient images");if count==4 {assert!(pixels==first);}else{single=pixels;}assert_eq!(r.stats()[0],uploads);}
 crate::sample_test_proof::write(root.join("one.rgba"),single).unwrap();let repeated=read_ambient_samples(&r,&metadata);assert_eq!(repeated,samples,"Actual correction counters reset per changed frame and return to the same bounded work");
 r.instances.ordinary.clear();r.set_ambient(false);r.set_ambient(true);let uniforms=crate::cap_gpu_tests::pixels_at(&r,transform);assert!(uniforms==first,"Exact4x ordinary/shared ambient source properties");r.set_ambient(false);let plain=crate::cap_gpu_tests::pixels_at(&r,transform);assert!(plain!=first);crate::sample_test_proof::write(root.join("plain-four.rgba"),plain).unwrap();for ((mesh,before),source)in r.meshes.iter().zip(&geometry).zip(&original){assert!(Rc::ptr_eq(&mesh.geometry,before));assert_eq!(&scene.objects().iter().find(|o|o.id==mesh.id).unwrap().geometry().unwrap().to_proto(),source);}
}


#[test]
fn a_colour_sample_count_change_preserves_an_inflight_independent_owner_query() {
 let (scene,owner)=crate::cap_gpu_tests::scene(false);let mut r=crate::cap_gpu_tests::renderer(&scene);let matrix=crate::cap_gpu_tests::matrix();assert_eq!(r.samples(),1);r.request_pick([0.3,0.2],matrix).unwrap();let usage=r.pick_usage();r.set_samples(4);assert_eq!(r.samples(),4);assert_eq!(r.pick_usage(),usage,"Colour-only retarget must retain its independent1x query and ticket");crate::cap_gpu_tests::wait(&r);assert_eq!(r.take_pick(matrix).unwrap().unwrap().unwrap()[0],owner.value());let first=crate::cap_gpu_tests::pixels(&r);let targets=(r.targets.depth.clone(),r.targets.primitive.view.clone());let stats=r.stats();r.set_samples(4);assert_eq!((&r.targets.depth,&r.targets.primitive.view),(&targets.0,&targets.1));assert_eq!(r.stats(),stats);assert!(crate::cap_gpu_tests::pixels(&r)==first);
 r.request_pick([-0.3,0.2],matrix).unwrap();let usage=r.pick_usage();r.set_samples(1);assert_eq!(r.pick_usage(),usage);crate::cap_gpu_tests::wait(&r);assert_eq!(r.take_pick(matrix).unwrap().unwrap().unwrap()[0],owner.value());
}


#[test]
fn automatic_sampling_eligibility_follows_active_faces_and_world_lettering() {
 let mut scene=Scene::empty();let mut r=crate::cap_gpu_tests::renderer(&scene);assert!(!r.has_solids());let sheet=scene.insert_sheet(Rc::new(crate::sheet_source::Prepared::new(Rc::new(crate::sheet_example::source())).unwrap())).unwrap();r.set_scene(&scene,None);assert!(!r.has_solids(),"Original drawing pens are ribbons, not the reference arena's sheet fills");
 let mesh=scene.insert(crate::prepared::PreparedMesh::triangle([[-0.5,-0.5,0.5],[0.5,-0.5,0.5],[0.,0.5,0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();r.set_scene(&scene,None);assert!(r.has_solids());r.set_opacity(0.);assert!(r.has_solids(),"Eligibility describes geometry, not the current opacity");Rc::make_mut(&mut scene.view).hide(&[mesh],true);r.set_scene(&scene,None);assert!(!r.has_solids());
 let mut isolated=session_rust::Mesh::new();isolated.add_vertex(session_rust::Point::new(0.,0.,0.),Some(17));let isolated=crate::prepared::PreparedMesh::new(isolated).unwrap();assert!(isolated.display.parts.edges.is_empty()&&isolated.display.parts.dots.is_empty(),"Isolated source vertices do not produce mesh pipes or glyphs");let mut empty_faces=Scene::empty();empty_faces.insert(isolated).unwrap();r.set_scene(&empty_faces,None);assert!(!r.has_solids());
 for kind in ["surface","brep","plane"] {let mut sample=Scene::empty();match kind {"surface"=>{sample.insert_surface(Rc::new(crate::surface::example())).unwrap();},"brep"=>{sample.insert_cad(Rc::new(session_rust::BRep::create_box(1.,1.,1.))).unwrap();},_=>{crate::plane_original::insert(&mut sample,Rc::new(session_rust::Plane::xy_plane())).unwrap();}}let owner=sample.objects()[0].id;let source=Rc::clone(&sample.objects()[0].mesh);r.set_scene(&sample,None);assert_eq!(r.has_solids(),kind!="plane","Validated surface and CAD boundaries accompany faces; original plane pens are ribbons");assert!(Rc::ptr_eq(&source,&sample.objects()[0].mesh));Rc::make_mut(&mut sample.view).hide(&[owner],true);r.set_scene(&sample,None);assert!(!r.has_solids(),"Hidden original source families do not qualify");}
 r.set_scene(&scene,None);
 let mut text=crate::text_example::label();r.set_text(vec![text.clone()]).unwrap();assert!(!r.has_solids(),"Screen overlay lettering is not world geometry");text.placement=crate::text::TextPlacement::WorldBillboard {world:[0.;3],world_height:0.1};r.set_text(vec![text]).unwrap();assert!(r.has_solids(),"World lettering is a filled world surface");assert!(scene.visible(sheet));
}
