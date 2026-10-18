use crate::text_atlas::Lane;

fn device()->(wgpu::Device,wgpu::Queue) {
    let instance=wgpu::Instance::new(wgpu::InstanceDescriptor {backends:wgpu::Backends::VULKAN,flags:Default::default(),memory_budget_thresholds:Default::default(),backend_options:Default::default(),display:None});
    let adapter=pollster::block_on(instance.request_adapter(&Default::default())).unwrap();pollster::block_on(adapter.request_device(&Default::default())).unwrap()
}

fn pixels(device:&wgpu::Device,queue:&wgpu::Queue,lane:&Lane,size:[u32;2])->Vec<u8> {
    let [width,height]=size;let texture=device.create_texture(&wgpu::TextureDescriptor {label:Some("actual glyph coverage"),size:wgpu::Extent3d {width,height,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::COPY_SRC,view_formats:&[]});
    let view=texture.create_view(&Default::default());let stride=width*4;assert_eq!(stride%256,0);let read=device.create_buffer(&wgpu::BufferDescriptor {label:Some("glyph pixels"),size:stride as u64*height as u64,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::MAP_READ,mapped_at_creation:false});let mut encoder=device.create_command_encoder(&Default::default());
    {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("text coverage"),color_attachments:&[Some(wgpu::RenderPassColorAttachment {view:&view,depth_slice:None,resolve_target:None,ops:wgpu::Operations {load:wgpu::LoadOp::Clear(wgpu::Color::WHITE),store:wgpu::StoreOp::Store}})],..Default::default()});lane.draw(&mut pass).unwrap();}
    encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo {buffer:&read,layout:wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(stride),rows_per_image:Some(height)}},wgpu::Extent3d {width,height,depth_or_array_layers:1});queue.submit([encoder.finish()]);let (tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|result|tx.send(result).unwrap());device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();rx.recv().unwrap().unwrap();let bytes=read.slice(..).get_mapped_range().to_vec();read.unmap();bytes
}

#[test]
fn actual_glyph_atlas_has_antialiased_coverage_and_preserves_css_size_at_two_densities() {
    let (device,queue)=device();let mut lane=Lane::new(&device,&queue,wgpu::TextureFormat::Rgba8Unorm);lane.document.set_labels(vec![crate::text_example::label()]).unwrap();let mut bounds=Vec::new();
    for density in [1.,2.] {
        let size=[(320.*density) as u32,(240.*density) as u32];lane.prepare(&device,&queue,size,density).unwrap();let bytes=pixels(&device,&queue,&lane,size);let mut box_=[f32::INFINITY,f32::INFINITY,0.,0.];let (mut ink,mut coverage)=(0,0);
        for (i,p) in bytes.chunks_exact(4).enumerate() {if p[0]<240 {let x=(i%size[0] as usize) as f32/density;let y=(i/size[0] as usize) as f32/density;box_[0]=box_[0].min(x);box_[1]=box_[1].min(y);box_[2]=box_[2].max(x);box_[3]=box_[3].max(y);ink+=1;}if p[0]>0&&p[0]<240 {coverage+=1;}}
        assert!(ink>200&&coverage>50,"Actual glyph ink and fractional coverage must render");bounds.push(box_);lane.prepare(&device,&queue,size,density).unwrap();assert_eq!(pixels(&device,&queue,&lane,size),bytes);
    }
    assert!(bounds[0].iter().zip(bounds[1]).all(|(a,b)|(a-b).abs()<1.5));assert_eq!(lane.document.shape_count,1);assert_eq!(lane.preparations,2);assert_eq!(lane.skips,2);assert_eq!(lane.raster_usage(),[0,0],"Glyphon 0.11 uploads temporary raster images without retaining them in the CPU image cache");
}

#[test]
fn actual_glyph_clip_and_font_reload_preserve_source_words_and_reset_only_accepted_fonts() {
    let (device,queue)=device();let mut lane=Lane::new(&device,&queue,wgpu::TextureFormat::Rgba8Unorm);let mut label=crate::text_example::label();label.clip=Some([16.,12.,120.,42.]);let words=label.text.clone();lane.document.set_labels(vec![label.clone()]).unwrap();lane.prepare(&device,&queue,[320,240],1.).unwrap();let before=pixels(&device,&queue,&lane,[320,240]);
    for (i,p) in before.chunks_exact(4).enumerate() {let (x,y)=(i%320,i/320);if x<16||x>=120||y<12||y>=42 {assert_eq!(p,[255;4]);}}
    assert!(before.chunks_exact(4).filter(|p|p[0]<240).count()>100);label.placement=crate::text::TextPlacement::Screen {left:17.,top:13.};lane.document.set_labels(vec![label]).unwrap();assert_eq!(lane.document.shape_count,1);lane.prepare(&device,&queue,[320,240],1.).unwrap();let moved=pixels(&device,&queue,&lane,[320,240]);assert_ne!(moved,before);let rasters=lane.raster_usage();
    assert!(lane.document.replace_fonts(vec![vec![0;8]]).is_err());lane.prepare(&device,&queue,[320,240],1.).unwrap();assert_eq!(lane.resets,0);assert_eq!(pixels(&device,&queue,&lane,[320,240]),moved);assert_eq!(lane.raster_usage(),rasters);
    lane.document.replace_fonts(vec![crate::text::FONT_BYTES.to_vec(),crate::text::FALLBACK_BYTES.to_vec(),crate::text::SYMBOL_BYTES.to_vec()]).unwrap();lane.prepare(&device,&queue,[320,240],1.).unwrap();assert_eq!(lane.resets,1);assert_eq!(lane.document.shape_count,2);assert_eq!(lane.document.runs[0].label.text,words);assert_eq!(pixels(&device,&queue,&lane,[320,240]),moved);
}
