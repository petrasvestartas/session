use crate::cap_gpu_tests::{scene,renderer,pixels,wait};
fn retarget(r:&mut crate::renderer::Renderer,count:u32) {
 assert!(matches!(count,1|4));if r.targets.samples==count{return;}
 r.surfaces.select(count,||crate::surface_pipelines::pipelines(&r.device,r.format,&crate::surface_view::layout(&r.device),&crate::gpu_layout::uniform(&r.device,crate::object_record::BYTES as u64),count));
 r.instances.set_samples(&r.device,r.format,count);
 for lane in [&mut r.mesh_ink,&mut r.strokes,&mut r.paths,&mut r.markers,&mut r.grid] {lane.set_samples(&r.device,count);}
 r.highlight.set_samples(&r.device,count);r.preview.set_samples(&r.device,count);r.measurement.set_samples(&r.device,count);r.guides.set_samples(&r.device,count);r.caps.borrow_mut().set_samples(count);
 if let Some(cloud)=r.clouds.borrow_mut().as_mut(){cloud.set_samples(&r.text_context(),count);}
 r.targets=crate::frame_targets::Targets::with_formats(&r.device,r.size,r.output_format,r.format,count);
}
fn level(r:&crate::renderer::Renderer,texture:&wgpu::Texture,mip:u32,bytes:u32)->Vec<u8>{
 let width=texture.width()>>mip;let height=texture.height()>>mip;let pitch=(width*bytes).div_ceil(256)*256;let buffer=r.device.create_buffer(&wgpu::BufferDescriptor{label:Some("actual pyramid fixture"),size:u64::from(pitch)*u64::from(height),usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let mut encoder=r.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(wgpu::TexelCopyTextureInfo{mip_level:mip,..texture.as_image_copy()},wgpu::TexelCopyBufferInfo{buffer:&buffer,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},wgpu::Extent3d{width,height,depth_or_array_layers:1});r.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();buffer.slice(..).map_async(wgpu::MapMode::Read,move|v|tx.send(v).unwrap());wait(r);rx.recv().unwrap().unwrap();let view=buffer.slice(..).get_mapped_range();let data=view.chunks_exact(pitch as usize).flat_map(|r|r[..(width*bytes)as usize].iter().copied()).collect();drop(view);buffer.unmap();data
}
#[test]
fn actual_pyramid_uses_six_nearest_positive_depth_levels_and_keeps_radius_addresses(){
 for samples in [1,4]{let(scene,_)=scene(false);let mut r=renderer(&scene);retarget(&mut r,samples);r.set_depth_pyramid(true);r.pyramid.borrow_mut().as_mut().unwrap().set_samples(&r.device,samples);pixels(&r);assert_eq!(r.depth_pyramid_error(),None);let owner=r.pyramid.borrow();let owner=owner.as_ref().unwrap();let linear=owner.linear().unwrap();let radii=owner.radii().unwrap();let mut previous:Option<(Vec<f32>,Vec<u16>,usize,usize)>=None;
 for mip in 0..6{let width=(linear.texture.width()>>mip)as usize;let height=(linear.texture.height()>>mip)as usize;let depths=level(&r,&linear.texture,mip,4).chunks_exact(4).map(|b|f32::from_le_bytes(b.try_into().unwrap())).collect::<Vec<_>>();let encoded=level(&r,&radii.texture,mip,2).chunks_exact(2).map(|b|u16::from_le_bytes(b.try_into().unwrap())).collect::<Vec<_>>();assert!(depths.iter().all(|z|z.is_finite()&&*z>=0.));if let Some((before,radius,w,h))=&previous{for y in 0..height{for x in 0..width{let mut nearest=1e30;let mut chosen=0;for i in 0..4{let q=(y*2+i/2).min(h-1)*w+(x*2+i%2).min(w-1);if before[q]>0.&&before[q]<nearest{nearest=before[q];chosen=radius[q];}}let expected=if nearest==1e30{0.}else{nearest};assert_eq!(depths[y*width+x],expected,"actual reduction mip{mip} at{x},{y}");assert_eq!(encoded[y*width+x],chosen);}}}else{let index=48*width+104;assert!((depths[index]-0.5).abs()<1e-5,"original section depth: {}",depths[index]);assert_eq!(depths[0],0.,"empty depth stays absent");assert!(encoded[index]>0);}previous=Some((depths,encoded,width,height));}
 assert_eq!(r.depth_pyramid_usage()[3],6);
 }
}
#[test]
fn actual_pyramid_caches_resting_views_rebuilds_on_resize_and_releases_on_disable(){
 let(scene,_)=scene(false);let mut r=renderer(&scene);r.set_depth_pyramid(true);pixels(&r);let first=r.depth_pyramid_usage();pixels(&r);assert_eq!(r.depth_pyramid_usage(),first);r.resize(crate::viewport::Viewport{width:640,height:480});r.set_density(2.);let texture=r.device.create_texture(&wgpu::TextureDescriptor{label:None,size:wgpu::Extent3d{width:640,height:480,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});r.draw(&texture.create_view(&Default::default()),&Default::default(),&crate::cap_gpu_tests::matrix());assert_eq!(r.depth_pyramid_usage()[1],2);assert_eq!(r.depth_pyramid_usage()[3],12);r.set_depth_pyramid(false);assert_eq!(r.depth_pyramid_usage(),[0;4]);
}
