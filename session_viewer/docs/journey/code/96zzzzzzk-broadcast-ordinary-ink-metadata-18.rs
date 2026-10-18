use crate::{renderer::Renderer,scene::Scene,stroke_gpu::Lane};
fn renderer(scene:&Scene)->Renderer{crate::cap_gpu_tests::renderer(scene)}
#[test]
fn ordinary_ink_broadcasts_one_record_and_releases_it(){
    let r=renderer(&Scene::empty());let device=&r.device;
    let stroke=crate::stroke::Stroke{start:[0.;3],end:[1.,0.,0.],colour:[0.,0.,0.,1.],width:1.};
    let mut lines=Lane::new(device,wgpu::TextureFormat::Rgba8Unorm);lines.enable_finite(device,wgpu::TextureFormat::Rgba8Unorm,false);lines.set(device,std::iter::repeat_n(stroke,8192)).unwrap();assert_eq!(lines.clipping_bytes(),[4,4],"8192ordinary line instances need one CPU/GPU flags record, previously32768bytes each");assert!(lines.set_flags(device,vec![0;8192]).is_err());lines.set(device,[]).unwrap();assert_eq!(lines.clipping_bytes(),[0,0]);
    let mut paths=Lane::joined(device,wgpu::TextureFormat::Rgba8Unorm);paths.enable_finite(device,wgpu::TextureFormat::Rgba8Unorm,false);paths.set_chains(device,std::iter::repeat_n(crate::chain::Segment{stroke,previous:stroke.start,next:stroke.end,heads:3},8192)).unwrap();assert_eq!(paths.clipping_bytes(),[4,4]);paths.set_chains(device,[]).unwrap();assert_eq!(paths.clipping_bytes(),[0,0]);
    let mut points=Lane::markers(device,wgpu::TextureFormat::Rgba8Unorm);points.enable_finite(device,wgpu::TextureFormat::Rgba8Unorm,false);points.set_points(device,std::iter::repeat_n(crate::marker::Marker{center:[0.;3],diameter:4.,colour:[0.,0.,0.,1.]},8192)).unwrap();assert_eq!(points.clipping_bytes(),[4,4]);points.set_points(device,[]).unwrap();assert_eq!(points.clipping_bytes(),[0,0]);
    let mut varying=Lane::new(device,wgpu::TextureFormat::Rgba8Unorm);varying.enable_finite(device,wgpu::TextureFormat::Rgba8Unorm,true);varying.set(device,std::iter::repeat_n(stroke,3)).unwrap();varying.set_flags(device,vec![0,4096,0]).unwrap();assert_eq!(varying.clipping_bytes(),[12,12]);assert!(varying.set_flags(device,vec![0]).is_err());varying.set(device,[]).unwrap();assert_eq!(varying.clipping_bytes(),[0,0]);
}
#[test]
fn actual_zero_stride_ink_draws_and_picks_every_original(){
    let mut scene=Scene::empty();let mut ids=Vec::new();
    for i in 0..32{let y=-0.8+i as f64*0.05;let line=session_rust::Line::new(-0.7,y,-0.5,0.7,y,-0.5);let row=scene.insert_line(crate::stroke::PreparedLine::new(std::rc::Rc::new(line)).unwrap()).unwrap();ids.push(row);}
    let mut r=renderer(&scene);r.set_opacity(1.);let matrix=crate::cap_gpu_tests::matrix();
    for density in [1.,2.]{r.set_density(density);let image=crate::cap_gpu_tests::pixels_at(&r,matrix);for(i,id)in ids.iter().enumerate(){let y=-0.8+i as f32*0.05;let row=((1.-y)*120.).floor()as usize;let pixel=&image[(row*320+160)*4..(row*320+161)*4];assert!(pixel[..3].iter().all(|v|*v<150),"Actual zero-stride line{i} remains visible at density{density}: {pixel:?}");r.request_pick([0.,y],matrix).unwrap();crate::cap_gpu_tests::wait(&r);assert_eq!(r.take_pick(matrix).unwrap().unwrap().unwrap()[0],id.value(),"The actual first, middle and last original instances retain their GPU IDs");}}
    scene.clear();r.set_scene(&scene,None);crate::cap_gpu_tests::pixels_at(&r,matrix);assert_eq!(r.clipping_ink_bytes(),[0,0]);
}
