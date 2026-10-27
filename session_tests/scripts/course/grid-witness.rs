use anyhow::{ensure, Result};
use sha2::{Digest, Sha256};
use std::{io::Write, path::Path};
use viewer_journey::{renderer::Renderer, viewport::Viewport};

fn digest(bytes: impl AsRef<[u8]>) -> String { format!("{:x}", Sha256::digest(bytes.as_ref())) }

fn pixels(renderer: &Renderer, matrix: [f32; 16], size: u32) -> Result<Vec<u8>> {
    let texture = renderer.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("paired native grid"), size: wgpu::Extent3d {width:size,height:size,depth_or_array_layers:1},
        mip_level_count:1, sample_count:1, dimension:wgpu::TextureDimension::D2,
        format:wgpu::TextureFormat::Rgba8UnormSrgb, usage:wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::COPY_SRC, view_formats:&[],
    });
    renderer.draw(&texture.create_view(&Default::default()), &Default::default(), &matrix);
    let pitch = (size*4).div_ceil(256)*256;
    let read = renderer.device.create_buffer(&wgpu::BufferDescriptor {label:None,size:u64::from(pitch)*u64::from(size),usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});
    let mut encoder = renderer.device.create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo {buffer:&read,layout:wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(size)}},texture.size());
    renderer.queue.submit([encoder.finish()]);
    let(tx,rx)=std::sync::mpsc::channel();
    read.slice(..).map_async(wgpu::MapMode::Read,move|result|{let _=tx.send(result);});
    renderer.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(60))})?;
    rx.recv()??;
    let mapped=read.slice(..).get_mapped_range();
    let image=mapped.chunks_exact(pitch as usize).flat_map(|row|row[..size as usize*4].iter().copied()).collect();
    drop(mapped);read.unmap();Ok(image)
}

fn ppm(path:&Path,image:&[u8],size:u32)->Result<()> {
    let mut file=std::io::BufWriter::new(std::fs::File::create(path)?);
    write!(file,"P6\n{size} {size}\n255\n")?;
    for pixel in image.chunks_exact(4){file.write_all(&pixel[..3])?;}
    file.flush()?;Ok(())
}

fn main()->Result<()> {
    let output=std::env::args_os().nth(1).ok_or_else(||anyhow::anyhow!("grid-witness OUTPUT"))?;
    let output=Path::new(&output);std::fs::create_dir_all(output)?;
    let mut cases=Vec::new();
    for density in [1u32,2] {
        let size=256*density;
        let (production, transform, device, queue, identity)=pollster::block_on(current_viewer::selftest::stroke_witness::render_grid(size))?;
        let scene=viewer_journey::scene::Scene::empty();
        let mut course=Renderer::new(device,queue,wgpu::TextureFormat::Rgba8UnormSrgb,&scene);
        course.resize(Viewport {width:size,height:size});course.set_density(density as f32);course.set_grid(Some(Default::default())).map_err(anyhow::Error::msg)?;
        let actual=pixels(&course,transform,size)?;
        let changed=production.chunks_exact(4).zip(actual.chunks_exact(4)).filter(|(a,b)|a!=b).count();
        let production_path=output.join(format!("grid-{size}-production.ppm"));
        ppm(&production_path,&production,size)?;ppm(&output.join(format!("grid-{size}-course.ppm")),&actual,size)?;
        cases.push(serde_json::json!({"physical_size":size,"density":density,"samples":1,"production":identity,"camera_matrix":transform,"encoding":"Rgba8UnormSrgb","same_device_and_queue":true,"changed_pixels":changed,"production_ppm_sha256":digest(std::fs::read(production_path)?)}));
        std::fs::write(output.join("manifest.json"),serde_json::to_vec_pretty(&serde_json::json!({"scope":"Independent actual maintained renderer and course, identical camera/device/queue, complete RGBA comparison","cases":cases}))?)?;
        eprintln!("grid {size}: {changed} changed pixels");
    }
    ensure!(cases.iter().all(|case|case["changed_pixels"]==0),"Paired grid differs; see manifest");Ok(())
}
