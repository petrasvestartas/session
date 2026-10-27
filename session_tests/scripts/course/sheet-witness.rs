use anyhow::{ensure, Result};
use current_viewer::app::{scene::SheetInit, stream::SheetFields, walk::sheet::SheetRows};
use prost::Message;
use session_rust::{proto, Xform};
use sha2::{Digest, Sha256};
use std::{io::Write, path::Path, rc::Rc};
use viewer_journey::{renderer::Renderer, viewport::Viewport};

fn digest(bytes: impl AsRef<[u8]>) -> String { format!("{:x}", Sha256::digest(bytes.as_ref())) }

fn pixels(renderer: &Renderer, matrix: [f32; 16], size: [u32; 2]) -> Result<Vec<u8>> {
    let [width, height] = size;
    let texture = renderer.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("matched streamed sheet"), size: wgpu::Extent3d {width,height,depth_or_array_layers:1},
        mip_level_count:1, sample_count:1, dimension:wgpu::TextureDimension::D2,
        format:wgpu::TextureFormat::Rgba8UnormSrgb, usage:wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::COPY_SRC, view_formats:&[],
    });
    renderer.draw(&texture.create_view(&Default::default()), &Default::default(), &matrix);
    let pitch = (width*4).div_ceil(256)*256;
    let read = renderer.device.create_buffer(&wgpu::BufferDescriptor {label:None,size:u64::from(pitch)*u64::from(height),usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});
    let mut encoder = renderer.device.create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo {buffer:&read,layout:wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},texture.size());
    renderer.queue.submit([encoder.finish()]);
    let(tx,rx)=std::sync::mpsc::channel();
    read.slice(..).map_async(wgpu::MapMode::Read,move|result|{let _=tx.send(result);});
    renderer.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(60))})?;
    rx.recv()??;
    let mapped=read.slice(..).get_mapped_range();
    let image=mapped.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize*4].iter().copied()).collect();
    drop(mapped);read.unmap();Ok(image)
}

fn ppm(path:&Path,image:&[u8],size:[u32;2])->Result<()> {
    let mut file=std::io::BufWriter::new(std::fs::File::create(path)?);
    write!(file,"P6\n{} {}\n255\n",size[0],size[1])?;
    for pixel in image.chunks_exact(4){file.write_all(&pixel[..3])?;}
    file.flush()?;Ok(())
}

async fn run(output:&Path, filter:Option<&str>)->Result<()> {
    std::fs::create_dir_all(output)?;
    let source=proto::Sheet {
        guid:"6d0aaee0-0430-4000-8000-000000000701".into(), name:"Authored sheet pens".into(),
        coords:vec![-700.,-300.,-2000.,700.,-300.,-2000.,-650.,0.,-2000.,650.,180.,-2000.,-600.,350.,-2000.,600.,350.,-2000.],
        colors:vec![0xff00_0000,0xffd0_5020,0xff00_0000], widths:vec![2.,0.,9.],
        segment_count:3, entity_count:4, meta:String::new(), source_ids:vec![1,2,3],
    };
    let bytes=source.encode_to_vec(); std::fs::write(output.join("source.sheet.pb"),&bytes)?;
    let mut records=Vec::new();let mut failures=Vec::new();
    for projection in ["orthographic","perspective"] { for placed in [false,true] { for density in [1u32,2] { for samples in [1,4] { for selected in [false,true] {
        let name=format!("{projection}-placed{placed}-dpr{density}-{samples}x-selected{selected}");
        if filter.is_some_and(|filter|!name.contains(filter)){continue;}
        eprintln!("START sheet {name}");
        let size=[320*density,240*density];
        let matrix=if projection=="perspective" {Xform::perspective(std::f64::consts::PI/3.,4./3.,5.,0.1)*Xform::scale_xyz(0.001,0.001,0.001)}
            else {Xform::orthographic(-1200.,1200.,-900.,900.,100.,5000.)};
        let matrix=matrix.to_f32();
        let place=if placed {&(&Xform::translation(100.,30.,0.)*&Xform::rotation_z(17.,true))*&Xform::scale_xyz(-0.8,1.2,1.)}else{Xform::identity()};
        let production=current_viewer::selftest::stroke_witness::render_sheet(SheetInit {
            name:source.name.clone(),url:"source.sheet.pb".into(),meta_url:None,place:place.clone(),
            rows:SheetRows {positions:source.coords.iter().map(|v|*v as f32).collect(),colors:source.colors.clone(),widths:source.widths.clone(),ids:source.source_ids.clone()},
            fields:SheetFields {count:source.segment_count,entities:source.entity_count,..Default::default()},resident:source.segment_count,
        },matrix,size,density as f32,samples,selected).await?;
        let expected=production.rgba;
        let ids=production.ids;
        let distinct=ids.iter().copied().filter(|id|id[0]!=0).collect::<std::collections::BTreeSet<_>>();
        ensure!(!distinct.is_empty(),"Actual production streamed sheet must render pickable ink");
        ensure!(distinct.iter().all(|id|id[0]==1),"Production sheet pixels must retain their original row");
        let mut scene=viewer_journey::scene::Scene::empty();
        let owner=scene.insert_sheet(Rc::new(viewer_journey::sheet_source::Prepared::new(Rc::new(source.clone())).map_err(anyhow::Error::msg)?)).map_err(anyhow::Error::msg)?;
        scene.place_sheet(owner,place.clone()).map_err(anyhow::Error::msg)?;
        let mut course=Renderer::new(production.device,production.queue,wgpu::TextureFormat::Rgba8UnormSrgb,&scene);
        course.resize(Viewport {width:size[0],height:size[1]});course.set_density(density as f32);course.set_samples(samples);
        course.set_ink_visibility(viewer_journey::ink_ranges::Visibility {edges:false,points:false,lines:true,vertices:false});
        if selected {course.select_objects(&scene,&[owner]).map_err(anyhow::Error::msg)?;}
        let actual=pixels(&course,matrix,size)?;
        let changed=expected.chunks_exact(4).zip(actual.chunks_exact(4)).filter(|(a,b)|a!=b).count();
        ppm(&output.join(format!("{name}-production.ppm")),&expected,size)?;
        ppm(&output.join(format!("{name}-course.ppm")),&actual,size)?;
        if changed!=0 {failures.push(name.clone());}
        records.push(serde_json::json!({"case":name,"source_sha256":digest(&bytes),"placement_f64":place.m,"matrix_f32":matrix,"density":density,"samples":samples,"selected":selected,"same_device_and_queue":true,"production_ingress":"Actual Scene::add_sheet(SheetInit, SheetFields, SheetRows), all segments resident; no SessionPB substitution or network-range claim","production_ids":distinct,"production":production.identity,"changed_rgba_pixels":changed}));
        std::fs::write(output.join("manifest.json"),serde_json::to_vec_pretty(&serde_json::json!({"scope":"Actual streamed-sheet ingress versus the same original course Sheet, full RGBA comparison","binary_sha256":digest(std::fs::read(std::env::current_exe()?)?),"cases":records,"failed_exact_pairs":failures}))?)?;
        eprintln!("FINISH sheet {name}: {changed} changed pixels");
    }}}}}
    ensure!(!records.is_empty(),"Filter selected no cases");ensure!(failures.is_empty(),"Sheet differs: {failures:?}");Ok(())
}

fn main()->Result<()> {
    let args=std::env::args().collect::<Vec<_>>();
    ensure!(args.len()>=2,"sheet-witness OUTPUT [CASE_FILTER]");
    pollster::block_on(run(Path::new(&args[1]),args.get(2).map(String::as_str)))
}
