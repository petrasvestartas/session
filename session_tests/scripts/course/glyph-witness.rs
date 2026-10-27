use anyhow::{Result, ensure};
use prost::Message;
use serde_json::{Value, json};
use session_rust::{Color, Line, Point, Mesh, Session, Xform};
use sha2::{Digest, Sha256};
use std::{io::Write, path::Path};
use viewer_journey::{background::Background, renderer::Renderer, viewport::Viewport};

const OBJECT: &str = "6d0aaee0-0430-4000-8000-000000000601";
const GUARD: &str = "6d0aaee0-0430-4000-8000-000000000602";
const LID: &str = "6d0aaee0-0430-4000-8000-000000000603";

fn digest(bytes: impl AsRef<[u8]>) -> String { format!("{:x}", Sha256::digest(bytes.as_ref())) }

fn specimen(kind:&str)->Vec<u8>{
    let mut session=Session::new(kind);session.set_guid("6d0aaee0-0430-4000-8000-000000000001".into());
    if kind=="point"{let mut p=Point::new(0.,0.,0.);p.set_guid(OBJECT.into());p.width=12.;p.pointcolor=Color::new(0.8,0.1,0.1,1.);session.add_point(p,None);}
    else {let mut m=Mesh::new();for(key,p)in[11,700,90003,900004].into_iter().zip([[-80.,-60.,0.],[80.,-60.,0.],[80.,60.,0.],[-80.,60.,0.]]){m.add_vertex(Point::new(p[0],p[1],p[2]),Some(key));}m.add_face(vec![11,700,90003,900004],Some(37));m.set_linecolors(vec![],vec![7.]);m.set_objectcolor(Color::new(0.2,0.4,0.8,1.));m.set_guid(OBJECT.into());session.add_mesh(m,None);}
    session.set_xform(OBJECT,Xform::translation(0.,0.,-500.));
    let mut guard=Line::new(10000.,10000.,-400.,10100.,10000.,-600.);guard.set_guid(GUARD.into());session.add_line(guard,None);
    session.to_proto().encode_to_vec()
}

fn pixels(renderer:&Renderer,matrix:[f32;16],size:[u32;2])->Result<Vec<u8>> {
    let [width,height]=size;
    let texture=renderer.device.create_texture(&wgpu::TextureDescriptor{label:Some("matched course Plane"),size:wgpu::Extent3d{width,height,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8UnormSrgb,usage:wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::COPY_SRC,view_formats:&[]});
    renderer.draw(&texture.create_view(&Default::default()),&Background::default(),&matrix);
    let pitch=(width*4).div_ceil(256)*256;
    let read=renderer.device.create_buffer(&wgpu::BufferDescriptor{label:Some("matched course Plane readback"),size:u64::from(pitch)*u64::from(height),usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});
    let mut encoder=renderer.device.create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&read,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},texture.size());renderer.queue.submit([encoder.finish()]);
    let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|v|{let _=tx.send(v);});
    renderer.device.poll(wgpu::PollType::Wait{submission_index:None,timeout:Some(std::time::Duration::from_secs(60))})?;rx.recv()??;
    let mapped=read.slice(..).get_mapped_range();let rgba=mapped.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize*4].iter().copied()).collect();drop(mapped);read.unmap();Ok(rgba)
}

fn ppm(path:&Path,rgba:&[u8],size:[u32;2])->Result<()> {
    let mut file=std::io::BufWriter::new(std::fs::File::create(path)?);write!(file,"P6\n{} {}\n255\n",size[0],size[1])?;for pixel in rgba.chunks_exact(4){file.write_all(&pixel[..3])?;}file.flush()?;Ok(())
}


fn main()->Result<()>{
    let args=std::env::args_os().collect::<Vec<_>>();ensure!(args.len()>=3,"glyph-witness OUTPUT COURSE_SRC [CASE_FILTER]");
    let output=Path::new(&args[1]);let source_dir=Path::new(&args[2]);let filter=args.get(3).map(|s|s.to_string_lossy().into_owned());std::fs::create_dir_all(output)?;
    let provenance:Value=serde_json::from_str(viewer_journey::production_glyph::PROVENANCE)?;
    let mut originals=Vec::new();for(name,source)in[("glyph",viewer_journey::production_glyph::GLYPH),("sphere",viewer_journey::production_glyph::SPHERE)]{
        ensure!(std::fs::read_to_string(source_dir.join(format!("production_{name}.wgsl")))?==source);
        let p=&provenance[name];let a=&p["ownership_adapter"];let replacement=a["replacement"].as_str().unwrap();ensure!(source.matches(replacement).count()==1);
        let original=source.strip_suffix(a["declaration"].as_str().unwrap()).unwrap().replace(replacement,a["original"].as_str().unwrap());
        ensure!(digest(source)==p["compiled_sha256"].as_str().unwrap()&&digest(&original)==p["pristine_sha256"].as_str().unwrap());originals.push(original);
    }
    let matrix=Xform::orthographic(-250.,250.,-187.5,187.5,1000.,0.).to_f32();let mut records=Vec::new();let mut failures=Vec::new();
    for kind in["point","mesh"]{let bytes=specimen(kind);let input=session_rust::proto::Session::decode(bytes.as_slice())?;std::fs::write(output.join(format!("{kind}.pb")),&bytes)?;
        for density in[1f32,2.]{for samples in[1,4]{for selected in[false,true]{
            let stem=format!("{kind}-dpr{}-samples{samples}-{}",density as u32,if selected{"selected"}else{"unselected"});if filter.as_ref().is_some_and(|s|!stem.contains(s)){continue;}eprintln!("START {stem}");
            let size=[320*density as u32,240*density as u32];let families=current_viewer::selftest::stroke_witness::Families{points:true,mesh_edges:true,markers:true,selected_guids:if selected{vec![OBJECT.into()]}else{Vec::new()},..Default::default()};
            let production=pollster::block_on(current_viewer::selftest::stroke_witness::render_families(&bytes,matrix,size,density,samples,1.,&[],&families))?;
            ensure!(production.glyph_source==originals[0]&&production.sphere_source==originals[1],"Complete actual production shader differs");
            let walked=&production.identity["walked_ink"];let object=walked["objects"].as_array().unwrap().iter().find(|v|v["guid"]==OBJECT).ok_or_else(||anyhow::anyhow!("Production source identity missing"))?;
            ensure!(object["placement_f64"]==json!(Xform::translation(0.,0.,-500.).m));
            let mut scene=viewer_journey::scene::Scene::empty();viewer_journey::scene_document::import(&mut scene,&bytes,kind,&Xform::identity(),0.).map_err(anyhow::Error::msg)?;
            let id=if kind=="point"{let row=scene.points().iter().find(|r|r.guid==OBJECT).unwrap();ensure!(row.prepared.source.to_proto()==input.objects.as_ref().unwrap().points[0]);ensure!(walked["dots"][0]["radius"]==-6.);row.id}else{scene.objects().iter().find(|r|r.guid==OBJECT).unwrap().id};
            let saved=viewer_journey::scene_archive::save(&scene).map_err(anyhow::Error::msg)?;
            let mut rows=Default::default();
            viewer_journey::production_glyph::append(&scene,&mut rows,[0.;3],if selected{std::slice::from_ref(&id)}else{&[]});
            let mut course=Renderer::new(production.device,production.queue,wgpu::TextureFormat::Rgba8UnormSrgb,&scene);course.resize(Viewport{width:size[0],height:size[1]});course.set_density(density);course.set_samples(samples);course.set_opacity(1.);course.set_lighting(false,false);if selected{course.select_objects(&scene,&[id]).map_err(anyhow::Error::msg)?;}
            let actual=pixels(&course,matrix,size)?;let changed=production.rgba.chunks_exact(4).zip(actual.chunks_exact(4)).filter(|(a,b)|a!=b).count();let ink=actual.chunks_exact(4).filter(|p|p[..3].iter().any(|v|*v<250)).count();ensure!(ink>40,"Empty glyph witness");
            ppm(&output.join(format!("{stem}-production.ppm")),&production.rgba,size)?;ppm(&output.join(format!("{stem}-course.ppm")),&actual,size)?;std::fs::write(output.join(format!("{stem}.session")),&saved)?;
            if changed!=0{failures.push(stem.clone());}records.push(json!({"case":stem,"source_sha256":digest(&bytes),"archive_sha256":digest(&saved),"production":production.identity,"course":{"viewport":size,"density":density,"samples":samples,"matrix_f32":matrix,"anchor":course.anchor(),"encoding":"Rgba8UnormSrgb","device":"same actual production device and queue","glyph_rows_sha256":digest(&rows.dots),"sphere_rows_sha256":digest(&rows.spheres),"pick_owners":rows.pick_owners},"changed_pixels":changed,"course_ink_pixels":ink}));
            std::fs::write(output.join("manifest.json"),serde_json::to_vec_pretty(&json!({"cases":records,"failed_pairs":failures,"provenance":provenance,"comparison":"entire RGBA frame; zero tolerance; identical original Session bytes and production device"}))?)?;eprintln!("FINISH {stem}: {changed} changed pixels");
        }}}
    }
    ensure!(!records.is_empty(),"No matched cases");ensure!(failures.is_empty(),"Whole-frame glyph differences: {failures:?}");Ok(())
}
