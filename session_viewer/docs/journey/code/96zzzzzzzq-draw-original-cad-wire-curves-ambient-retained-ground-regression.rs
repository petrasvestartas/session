use crate::{scene::Scene,cap_gpu_tests::{source,renderer,wait}};
fn pixels(r:&super::Renderer,matrix:&[f32;16])->Vec<u8>{
 let texture=r.device.create_texture(&wgpu::TextureDescriptor{label:Some("actual original ambient fixture"),size:wgpu::Extent3d{width:320,height:240,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::COPY_SRC,view_formats:&[]});r.draw(&texture.create_view(&Default::default()),&Default::default(),matrix);let read=r.device.create_buffer(&wgpu::BufferDescriptor{label:None,size:1280*240,usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let mut encoder=r.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&read,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(1280),rows_per_image:Some(240)}},texture.size());r.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|v|tx.send(v).unwrap());wait(r);rx.recv().unwrap().unwrap();let data=read.slice(..).get_mapped_range().to_vec();read.unmap();data
}
#[test]
fn actual_ambient_darkens_original_contacts_in_one_frame_caches_and_releases_large_images(){
 let mut scene=Scene::empty();let owner=scene.insert(crate::prepared::PreparedMesh::new(source(false)).unwrap()).unwrap();let original=scene.objects()[0].geometry().unwrap().to_proto();let mut camera=crate::camera::Camera::default();camera.isometric();camera.aspect=4./3.;camera.target=[0.,0.,-0.5];camera.distance=3.;let matrix=camera.uniform();let mut r=renderer(&scene);let plain=pixels(&r,&matrix);let uploads=r.stats()[0];r.set_ambient(true);let shaded=pixels(&r,&matrix);assert_eq!(r.ambient_error(),None);let usage=r.ambient_usage();assert!(usage[0]>100_000);assert_eq!(usage[1],1);assert_eq!(usage[3],1);let second=pixels(&r,&matrix);assert_eq!(second,shaded,"one frame completes ambient shading");assert_eq!(&r.ambient_usage()[1..4],&usage[1..4]);assert_eq!(r.stats()[0],uploads);let darker=plain.chunks_exact(4).zip(shaded.chunks_exact(4)).filter(|(a,b)|a[..3]!=[255,255,255]&&a[..3].iter().zip(&b[..3]).any(|(a,b)|a.saturating_sub(*b)>3)).count();assert!(darker>100,"actual ambient shading: {darker} pixels");let shadow=plain.chunks_exact(4).zip(shaded.chunks_exact(4)).filter(|(a,b)|a[..3]==[255,255,255]&&b[..3].iter().any(|v|*v<235)).count();assert!(shadow>20,"actual virtual-ground contacts: {shadow}");assert_eq!(scene.objects()[0].id,owner);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);r.set_ambient(false);assert_eq!(r.ambient_usage()[0],320);assert_eq!(pixels(&r,&matrix),plain);r.set_ambient(true);assert_eq!(pixels(&r,&matrix),shaded);assert_eq!(r.ambient_usage()[1],1,"style toggles retain compiled pipelines");assert_eq!(r.stats()[0],uploads);
 r.set_ambient(false);r.instances.ordinary.clear();r.set_ambient(true);assert_eq!(pixels(&r,&matrix),shaded,"Exact uniform and shared ambient properties");r.set_scene(&scene,None);r.set_ambient(false);r.set_ambient(true);assert_eq!(pixels(&r,&matrix),shaded);assert_eq!(r.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
}

#[test]
fn actual_retained_pedestal_ambient_reaches_visible_ground_and_preserves_cache_and_sources() {
    use prost::Message;
    let bytes = include_bytes!("../assets/pb/course-comparison.pb");
    assert_eq!(bytes.len(), 18_736);
    let document = session_rust::proto::Session::decode(bytes.as_slice()).unwrap();
    assert_eq!(document.objects.as_ref().unwrap().breps.len(), 4);
    let mut scene = Scene::empty();
    crate::scene_document::import(&mut scene, bytes, "original pedestal", &session_rust::Xform::identity(), 0.).unwrap();
    assert_eq!(scene.objects().len(), 4);
    let owners = scene.objects().iter().map(|row| (row.id, row.guid.clone(), row.model.m)).collect::<Vec<_>>();
    let snapshot = |scene: &Scene| {
        let bundle = crate::archive::Bundle::decode(&crate::scene_archive::save(scene).unwrap()).unwrap();
        let documents = bundle.documents.iter().map(|raw| session_rust::proto::Session::decode(raw.as_slice()).unwrap()).collect::<Vec<_>>();
        (documents, serde_json::to_value(bundle.metadata).unwrap())
    };
    let original = snapshot(&scene);
    let mut history = crate::history::History::default();
    history.edit(&mut scene, |scene| { assert!(scene.remove(owners[0].0)); });
    assert!(history.undo(&mut scene));
    assert_eq!(snapshot(&scene), original);
    let retained_history = history.scenes().map(&snapshot).collect::<Vec<_>>();
    assert_eq!(retained_history.len(), 1);
    let mut camera = crate::camera::Camera::default();
    camera.isometric();
    camera.aspect = 4. / 3.;
    camera.fit(&scene.bounds().unwrap());
    let matrix = camera.uniform();
    let adapter = crate::gpu_test_adapter::adapter();
    let (device, queue) = pollster::block_on(adapter.request_device(&Default::default())).expect("Actual GPU device");
    let format = wgpu::TextureFormat::Rgba8UnormSrgb;
    let mut renderer = super::Renderer::new(device, queue, format, &scene);
    renderer.resize(crate::viewport::Viewport { width: 640, height: 480 });
    renderer.set_density(1.);
    renderer.set_samples(1);
    renderer.set_opacity(1.);
    renderer.set_lighting(false, false);
    renderer.set_outlines(false);
    renderer.set_ink_visibility(crate::ink_ranges::Visibility { edges: false, lines: false, points: false, vertices: false });
    let off = crate::widget_tests::pixels_format(&mut renderer, &matrix, [640, 480], format);
    let uploads = renderer.stats()[0];
    renderer.set_ambient(true);
    let on = crate::widget_tests::pixels_format(&mut renderer, &matrix, [640, 480], format);
    assert_eq!(renderer.ambient_error(), None);
    assert_ne!(on, off);
    let ground = off.chunks_exact(4).zip(on.chunks_exact(4)).filter(|(plain, shaded)| {
        plain[..3] == [255, 255, 255]
            && shaded[..3].iter().all(|value| *value < 235)
            && shaded[0].abs_diff(shaded[1]) <= 2
            && shaded[1].abs_diff(shaded[2]) <= 2
            && shaded[0].abs_diff(shaded[2]) <= 2
    }).count();
    assert!(ground > 8_000, "Retained pedestal casts a broad visible gray ground shadow: {ground} pixels");
    let usage = renderer.ambient_usage();
    assert!(usage[2] > 0 && usage[3] > 0);
    assert_eq!(crate::widget_tests::pixels_format(&mut renderer, &matrix, [640, 480], format), on);
    assert_eq!(&renderer.ambient_usage()[1..4], &usage[1..4], "Unchanged On frame reuses pipelines, preparation and uniform writes");
    assert_eq!(renderer.stats()[0], uploads);
    renderer.set_ambient(false);
    assert_eq!(crate::widget_tests::pixels_format(&mut renderer, &matrix, [640, 480], format), off);
    renderer.set_ambient(true);
    assert_eq!(crate::widget_tests::pixels_format(&mut renderer, &matrix, [640, 480], format), on);
    assert_eq!(renderer.ambient_error(), None);
    assert_eq!(renderer.stats()[0], uploads);
    assert_eq!(scene.objects().iter().map(|row| (row.id, row.guid.clone(), row.model.m)).collect::<Vec<_>>(), owners);
    assert_eq!(snapshot(&scene), original);
    assert_eq!(history.scenes().map(&snapshot).collect::<Vec<_>>(), retained_history);
    assert!(history.redo(&mut scene));
    assert_eq!(scene.objects().len(), 3);
    assert!(!scene.contains(owners[0].0));
    assert!(history.undo(&mut scene));
    assert_eq!(snapshot(&scene), original);
}
