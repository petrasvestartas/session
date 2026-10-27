use anyhow::{Result, ensure};
use prost::Message;
use serde_json::{Value, json};
use session_rust::{Color, Line, Mesh, Plane, Session, Xform};
use sha2::{Digest, Sha256};
use std::{io::Write, path::Path, time::Instant};
use viewer_journey::{background::Background, renderer::Renderer, viewport::Viewport};

const OBJECT: &str = "6d0aaee0-0430-4000-8000-000000000601";
const GUARD: &str = "6d0aaee0-0430-4000-8000-000000000602";
const LID: &str = "6d0aaee0-0430-4000-8000-000000000603";

fn digest(bytes: impl AsRef<[u8]>) -> String { format!("{:x}", Sha256::digest(bytes.as_ref())) }

fn specimen(name: &str) -> (Vec<u8>, Xform) {
    let mut session=Session::new(name);
    session.set_guid("6d0aaee0-0430-4000-8000-000000000001".into());
    session.objects.set_guid("6d0aaee0-0430-4000-8000-000000000002".into());
    session.tree.set_guid("6d0aaee0-0430-4000-8000-000000000003".into());
    session.graph.set_guid("6d0aaee0-0430-4000-8000-000000000004".into());
    let mut plane=Plane::xy_plane();plane.set_guid(OBJECT.into());plane.name="ordinary source Plane".into();plane.width=1.0;
    session.add_plane(plane,None);
    let model=if name=="placed" {&(&Xform::translation(20.0,10.0,-500.0)*&Xform::rotation_z(23.0,true))*&Xform::scale_xyz(-0.8,1.2,1.0)} else {Xform::translation(0.0,0.0,-500.0)};
    session.set_xform(OBJECT,model.clone());
    let mut guard=Line::new(10000.0,10000.0,-400.0,10100.0,10000.0,-600.0);guard.set_guid(GUARD.into());guard.name="offscreen spatial-classification guard".into();session.add_line(guard,None);
    if name=="covered" || name=="print" {
        let mut lid=Mesh::create_box(400.0,400.0,20.0);lid.set_guid(LID.into());lid.set_objectcolor(Color::new(0.2,0.4,0.8,1.0));lid.set_linecolors(vec![],vec![if name=="print"{0.0}else{1.0}]);session.add_mesh(lid,None);session.set_xform(LID,Xform::translation(0.0,0.0,-450.0));
    }
    for (i,node) in session.tree.nodes().iter().enumerate(){node.borrow_mut().set_guid(format!("6d0aaee0-0430-4000-9000-{i:012}"));}
    (session.to_proto().encode_to_vec(),model)
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

fn main()->Result<()> {
    let args=std::env::args_os().collect::<Vec<_>>();ensure!(args.len()>=3,"usage: plane-witness OUTPUT COURSE_SRC [CASE_FILTER] [ORIGINAL_COVERED_PB]; e.g. plain-dpr1-samples1-size100-unselected");
    let output=Path::new(&args[1]);let source_dir=Path::new(&args[2]);std::fs::create_dir_all(output)?;
    let filter=args.get(3).map(|v|v.to_string_lossy().into_owned());
    let supplied=args.get(4).map(Path::new);
    ensure!(supplied.is_none() || filter.as_deref().is_some_and(|filter|filter.starts_with("covered-")),"An original covered fixture requires a covered-case filter");
    let course_source=std::fs::read_to_string(source_dir.join("production_plane.wgsl"))?;
    ensure!(course_source==viewer_journey::production_plane::SOURCE,"source snapshot differs from linked course shader");
    let provenance:Value=serde_json::from_str(viewer_journey::production_plane::PROVENANCE)?;
    ensure!(digest(&course_source)==provenance["compiled_sha256"].as_str().unwrap());
    let adapter=&provenance["ownership_adapter"];
    let replacement=adapter["replacement"].as_str().unwrap();ensure!(course_source.matches(replacement).count()==1);
    let original=course_source.strip_suffix(adapter["declaration"].as_str().unwrap()).ok_or_else(||anyhow::anyhow!("ownership declaration missing"))?.replace(replacement,adapter["original"].as_str().unwrap());
    ensure!(digest(&original)==provenance["pristine_sha256"].as_str().unwrap());
    std::fs::write(output.join("course-plane-compiled.wgsl"),&course_source)?;
    let binary_hash=digest(std::fs::read(std::env::current_exe()?)?);
    let matrix=Xform::orthographic(-250.0,250.0,-187.5,187.5,1000.0,0.0).to_f32();
    let mut records=Vec::new();let mut failures=Vec::new();
    for name in ["plain","placed","covered","print"] {
        let(mut bytes,model)=specimen(name);
        if name=="covered" {if let Some(source)=supplied {bytes=std::fs::read(source)?;}}
        let input=session_rust::proto::Session::decode(bytes.as_slice())?;
        std::fs::write(output.join(format!("{name}.pb")),&bytes)?;
        for density in [1.0f32,2.0] {for samples in [1,4] {for half_size in [50.0,100.0] {for selected in [false,true] {
            let stem=format!("{name}-dpr{}-samples{samples}-size{}-{}",density as u32,half_size as u32,if selected{"selected"}else{"unselected"});
            if filter.as_ref().is_some_and(|f|!stem.contains(f)){continue;}
            eprintln!("START {stem}");let size=[320*density as u32,240*density as u32];
            let families=current_viewer::selftest::stroke_witness::Families{selected_guids:if selected{vec![OBJECT.into()]}else{Vec::new()},..Default::default()};
            let started=Instant::now();
            let production=pollster::block_on(current_viewer::selftest::stroke_witness::render_plane_size(&bytes,matrix,size,density,samples,1.0,&families,half_size))?;
            let production_ms=started.elapsed().as_millis();
            ensure!(production.plane_source==original,"Plane shader is not the complete actual production program");
            std::fs::write(output.join("production-plane-compiled.wgsl"),&production.plane_source)?;
            ensure!(production.identity["uploaded_plane_size"]==half_size);
            ensure!(production.identity["viewport"]==json!(size)&&production.identity["density"]==density&&production.identity["samples"]==samples);
            ensure!(production.identity["format"]=="Rgba8UnormSrgb"&&production.identity["matrix_f32"]==json!(matrix)&&production.identity["anchor"]==json!([0.0,0.0,0.0]));
            let walked=&production.identity["walked_ink"];
            let objects=walked["objects"].as_array().unwrap();let object=objects.iter().find(|r|r["guid"]==OBJECT).ok_or_else(||anyhow::anyhow!("Production lost Plane GUID"))?;
            ensure!(object["placement_f64"]==json!(model.m));
            let planes=walked["planes"].as_array().unwrap();ensure!(planes.len()==1&&planes[0]["instance"]==object["row"]);
            ensure!(planes[0]["origin"]==json!([0.0,0.0,0.0])&&planes[0]["x"]==json!([1.0,0.0,0.0])&&planes[0]["y"]==json!([0.0,1.0,0.0]));
            ensure!(planes[0]["radius"]==0.0&&planes[0]["pad"]==0);
            let mut scene=viewer_journey::scene::Scene::empty();viewer_journey::scene_document::import(&mut scene,&bytes,name,&Xform::identity(),0.0).map_err(anyhow::Error::msg)?;
            let row=scene.objects().iter().find(|r|r.guid==OBJECT).ok_or_else(||anyhow::anyhow!("Course lost Plane GUID"))?;
            let id=row.id;ensure!(row.model.m==model.m);
            let Some(session_rust::Geometry::Plane(plane))=&row.original else{anyhow::bail!("Course lost original Plane")};
            ensure!(plane.to_proto()==input.objects.as_ref().unwrap().planes[0]);
            let origin=scene.forest.documents.last().and_then(|d|d.header.as_ref()).ok_or_else(||anyhow::anyhow!("Course lost loaded source origin"))?;ensure!(origin.version.hex()==digest(&bytes));
            let saved=viewer_journey::scene_archive::save(&scene).map_err(anyhow::Error::msg)?;
            let bundle=viewer_journey::archive::Bundle::decode(&saved).map_err(anyhow::Error::msg)?;
            let archived=bundle.documents.iter().map(|v|session_rust::proto::Session::decode(v.as_slice())).collect::<std::result::Result<Vec<_>,_>>()?;
            ensure!(archived.iter().filter_map(|s|s.objects.as_ref()).flat_map(|o|&o.planes).any(|p|p==&input.objects.as_ref().unwrap().planes[0]),"Archive lost original Plane fields");
            std::fs::write(output.join(format!("{stem}.session")),&saved)?;
            let mut course=Renderer::new(production.device,production.queue,wgpu::TextureFormat::Rgba8UnormSrgb,&scene);
            course.resize(Viewport{width:size[0],height:size[1]});course.set_density(density);course.set_samples(samples);course.set_opacity(1.0);course.set_lighting(false,false);course.set_plane_size(half_size).map_err(anyhow::Error::msg)?;
            course.set_ink_visibility(viewer_journey::ink_ranges::Visibility{edges:false,points:false,lines:true,vertices:false});
            if selected{course.select_objects(&scene,&[id]).map_err(anyhow::Error::msg)?;}
            let started=Instant::now();eprintln!("course Plane readback begin {stem}");let actual=pixels(&course,matrix,size)?;let course_ms=started.elapsed().as_millis();
            ensure!(course.plane_usage()==[1,48]);
            let changed=production.rgba.chunks_exact(4).zip(actual.chunks_exact(4)).filter(|(a,b)|a!=b).count();
            let prod_ink=production.rgba.chunks_exact(4).filter(|p|p[..3].iter().any(|v|*v<250)).count();
            let course_ink=actual.chunks_exact(4).filter(|p|p[..3].iter().any(|v|*v<250)).count();
            ensure!(prod_ink>100&&course_ink>100,"Empty Plane witness");
            let mut picked=Value::Null;
            if name!="covered" && name!="print" {
                let point=model.transform_point(&session_rust::Point::new(-0.8*half_size,0.4*half_size,0.0));
                let ndc=[(point[0]/250.0)as f32,(point[1]/187.5)as f32];
                course.request_pick(ndc,matrix).map_err(anyhow::Error::msg)?;
                course.device.poll(wgpu::PollType::Wait{submission_index:None,timeout:Some(std::time::Duration::from_secs(60))})?;
                let pixel=course.take_pick(matrix).ok_or_else(||anyhow::anyhow!("Pick still pending"))?.map_err(anyhow::Error::msg)?.ok_or_else(||anyhow::anyhow!("Original Plane not picked"))?;
                ensure!(course.resolve_pick(&scene,pixel).is_some_and(|t|t.owner==id));picked=json!({"ndc":ndc,"raw":pixel,"owner":id.value(),"source_guid":OBJECT,"scope":"course owner ABI; production framebuffer and source identity are separately verified"});
            }
            ppm(&output.join(format!("{stem}-production.ppm")),&production.rgba,size)?;ppm(&output.join(format!("{stem}-course.ppm")),&actual,size)?;
            if changed!=0{failures.push(stem.clone());}
            let record=json!({"case":stem,"source_file":format!("{name}.pb"),"source_sha256":digest(&bytes),"archive_sha256":digest(&saved),"original_plane_guid":OBJECT,"placement_f64":model.m,"half_size":half_size,"selected":selected,"production":production.identity,"course":{"device":"same device and queue returned by actual production rendering","viewport":size,"density":density,"samples":samples,"matrix_f32":matrix,"anchor":course.anchor(),"output":"Rgba8UnormSrgb","plane_rows":course.plane_usage(),"pick":picked},"comparison":{"scope":"entire RGBA frame, no tolerance or cropped exclusion","changed_pixels":changed,"production_ink":prod_ink,"course_ink":course_ink,"exact":changed==0},"elapsed_ms":{"production":production_ms,"course_readback":course_ms}});
            std::fs::write(output.join(format!("{stem}.json")),serde_json::to_vec_pretty(&record)?)?;records.push(record);
            std::fs::write(output.join("manifest.json"),serde_json::to_vec_pretty(&json!({"binary_sha256":binary_hash,"input_override":supplied.map(|path|path.to_string_lossy()),"shader_provenance":provenance,"cases":records,"failed_exact_pairs":failures,"scope":"Actual ordinary Plane; source bytes, f64 placement, matrix, viewport, DPR, samples, pen, plane size, selection and output encoding matched. Offscreen guard is the same real original Line in both ingresses."}))?)?;
            eprintln!("FINISH {stem}: changed={changed}, production={production_ms}ms course={course_ms}ms");
        }}}}
    }
    ensure!(!records.is_empty(),"CASE_FILTER matched no cases");ensure!(failures.is_empty(),"Plane whole-frame differences: {failures:?}");Ok(())
}
