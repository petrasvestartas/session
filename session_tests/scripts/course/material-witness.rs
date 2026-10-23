use anyhow::{Result,ensure};
use prost::Message;
use serde_json::{Value,json};
use session_rust::{Session,Xform};
use sha2::{Digest,Sha256};
use std::{io::Write,path::Path};
use viewer_journey::{renderer::Renderer,viewport::Viewport,background::Background};
fn digest(bytes:impl AsRef<[u8]>)->String{format!("{:x}",Sha256::digest(bytes.as_ref()))}
pub(crate) fn source(open:bool)->session_rust::Mesh{
 let corners=(0..8).map(|i|session_rust::Point::new(if i%4==1||i%4==2{0.6}else{-0.6},if i%4>=2{0.6}else{-0.6},if i>=4{-0.2}else{-0.8})).collect();let mut faces=vec![vec![0,3,2,1],vec![0,1,5,4],vec![2,3,7,6],vec![0,4,7,3],vec![1,2,6,5]];if !open{faces.push(vec![4,5,6,7]);}let mut mesh=session_rust::Mesh::from_vertices_and_faces(corners,faces);mesh.set_objectcolor(session_rust::Color::new(1.,0.,0.,1.));mesh
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


fn decoded_archive(bytes:&[u8])->Result<(Vec<session_rust::proto::Session>,Value)>{
    let bundle=viewer_journey::archive::Bundle::decode(bytes).map_err(anyhow::Error::msg)?;
    let documents=bundle.documents.iter().map(|bytes|session_rust::proto::Session::decode(bytes.as_slice())).collect::<std::result::Result<Vec<_>,_>>()?;
    Ok((documents,serde_json::to_value(bundle.metadata)?))
}
fn archive_snapshot(output:&Path,label:&str,bytes:&[u8])->Result<(Vec<session_rust::proto::Session>,Value)>{
    std::fs::write(output.join(format!("{label}.session")),bytes)?;
    let decoded=decoded_archive(bytes)?;
    std::fs::write(output.join(format!("{label}-decoded.txt")),format!("{:#?}",decoded.0))?;
    std::fs::write(output.join(format!("{label}-metadata.json")),serde_json::to_vec_pretty(&decoded.1)?)?;
    Ok(decoded)
}
fn main()->Result<()> {
    let arguments=std::env::args_os().collect::<Vec<_>>();
    ensure!(arguments.len()==2,"material-production-pairs FRESH_OUTPUT");
    let output=Path::new(&arguments[1]);ensure!(!output.exists(),"Preserve earlier output");std::fs::create_dir(output)?;
    let original_archive=include_bytes!("winding.session");
    let bundle=viewer_journey::archive::Bundle::decode(original_archive).map_err(anyhow::Error::msg)?;
    ensure!(bundle.documents.len()==1&&bundle.metadata.documents.len()==1,"Retained winding source must have exactly one document");
    ensure!(bundle.metadata.documents[0].place==Xform::identity().m,"Retained winding file placement must remain identity");
    let preparation:Value=serde_json::from_str(include_str!("preparation-base.json"))?;
    let winding=bundle.documents[0].clone();ensure!(digest(&winding)==preparation["windingDocumentSha256"].as_str().unwrap());
    ensure!(digest(original_archive)==preparation["windingArchiveSha256"].as_str().unwrap());
    let mut source_mesh=source(false);source_mesh.set_guid("material-closed-original-mesh".into());
    let mut hidden=session_rust::Line::new(-0.4,0.,-0.9,0.4,0.,-0.9);hidden.set_guid("material-hidden-original-line".into());hidden.linecolor=session_rust::Color::new(0.,1.,0.,1.);hidden.width=8.;
    let mut document=Session::new("Original opacity semantic fixture");document.set_guid("material-opacity-original-document".into());document.add_mesh(source_mesh,None);document.add_line(hidden,None);
    let closed=document.to_proto().encode_to_vec();
    std::fs::write(output.join("winding.pb"),&winding)?;std::fs::write(output.join("closed-hidden-line.pb"),&closed)?;
    // This is the real maintained source verification switch; no object flags are manufactured.
    current_viewer::app::clipping::verify_solids();
    ensure!(current_viewer::app::clipping::solids_verified());
    let mut records=Vec::new();let mut failures=Vec::new();
    for (kind,bytes,size) in [("winding",&winding,[640u32,480u32]),("closed",&closed,[320u32,240u32])] {
        let mut scene=viewer_journey::scene::Scene::empty();viewer_journey::scene_document::import(&mut scene,bytes,kind,&Xform::identity(),0.).map_err(anyhow::Error::msg)?;
        let matrix=if kind=="winding" {let mut camera=viewer_journey::camera::Camera::default();camera.isometric();camera.aspect=4./3.;camera.fit(&scene.bounds().ok_or_else(||anyhow::anyhow!("Missing winding bounds"))?);camera.uniform()}
            else {Xform::orthographic(-1.,1.,-1.,1.,1.,0.).to_f32()};
        let archive=viewer_journey::scene_archive::save(&scene).map_err(anyhow::Error::msg)?;
        let original_snapshot=archive_snapshot(output,&format!("{kind}-before"),&archive)?;
        let noop_archive=viewer_journey::scene_archive::save(&scene).map_err(anyhow::Error::msg)?;
        let noop_snapshot=archive_snapshot(output,&format!("{kind}-before-noop"),&noop_archive)?;
        ensure!(original_snapshot==noop_snapshot,"Repeated Save without a renderer changed decoded source or metadata");

        let modes=if kind=="winding" {vec![("unlit",false,false,1f32),("headlight",true,false,1.),("backface",false,true,1.),("restored-unlit",false,false,1.)]}
            else {vec![("opaque",false,false,1f32),("glass",false,false,0.5),("xray",false,false,0.),("restored-opaque",false,false,1.)]};
        let mut baseline:Option<(Vec<u8>,Vec<u8>)>=None;
        for (mode,lit,backface,opacity) in modes {
            let name=format!("{kind}-{mode}");eprintln!("START {name}");
            let families=current_viewer::selftest::stroke_witness::Families{lines:kind=="closed",mesh_edges:false,points:false,markers:false,selected_guids:Vec::new(),lit,backface,opacity};
            let production=pollster::block_on(current_viewer::selftest::stroke_witness::render_families(bytes,matrix,size,1.,1,1.,&[],&families))?;
            let identity=production.identity.clone();
            ensure!(identity["adapter"]["backend"]=="Dx12"&&identity["adapter"]["device_type"]=="Cpu"&&identity["adapter"]["vendor"]==5140&&identity["adapter"]["device"]==140,"Actual Microsoft WARP DX12 device required");
            ensure!(identity["matrix_f32"]==json!(matrix)&&identity["samples"]==1&&identity["density"]==1.0);
            ensure!(identity["lit"]==lit&&identity["backface"]==backface&&identity["opacity"]==opacity);
            ensure!(identity["format"]=="Rgba8UnormSrgb"&&identity["clear_linear"]==json!([1.,1.,1.,1.]));
            let uploaded_closed=if kind=="closed" {
                let original=identity["walked_ink"]["objects"].as_array().unwrap().iter().find(|row|row["guid"]=="material-closed-original-mesh").ok_or_else(||anyhow::anyhow!("Actual source Mesh GUID missing"))?;
                let row=original["row"].as_u64().unwrap();let uploaded=identity["uploaded_objects"].as_array().unwrap().iter().find(|value|value["row"]==row).unwrap();
                ensure!(original["flags"].as_u64().unwrap()&(1<<13)!=0&&uploaded["flags"].as_u64().unwrap()&(1<<13)!=0,"Genuine closed source and upload verification required");
                ensure!(identity["ribbons"].as_u64().unwrap()>0,"Original hidden Line must upload");Some(uploaded.clone())
            } else {None};
            let mut course=Renderer::new(production.device,production.queue,wgpu::TextureFormat::Rgba8UnormSrgb,&scene);
            course.resize(Viewport{width:size[0],height:size[1]});course.set_density(1.);course.set_samples(1);course.set_opacity(opacity);course.set_lighting(lit,backface);
            course.set_outlines(false);course.set_ambient(false);course.set_ink_visibility(viewer_journey::ink_ranges::Visibility{edges:false,lines:kind=="closed",points:false,vertices:false});
            let initialized_archive=viewer_journey::scene_archive::save(&scene).map_err(anyhow::Error::msg)?;
            let initialized_snapshot=archive_snapshot(output,&format!("{name}-after-init"),&initialized_archive)?;
            let actual=pixels(&course,matrix,size)?;ensure!(actual.len()==production.rgba.len());
            let changed=production.rgba.chunks_exact(4).zip(actual.chunks_exact(4)).filter(|(a,b)|a!=b).count();
            let max_delta=production.rgba.iter().zip(&actual).map(|(a,b)|a.abs_diff(*b)).max().unwrap();
            ppm(&output.join(format!("{name}-production.ppm")),&production.rgba,size)?;ppm(&output.join(format!("{name}-course.ppm")),&actual,size)?;
            std::fs::write(output.join(format!("{name}-production.rgba")),&production.rgba)?;std::fs::write(output.join(format!("{name}-course.rgba")),&actual)?;
            let after_archive=viewer_journey::scene_archive::save(&scene).map_err(anyhow::Error::msg)?;
            let after_snapshot=archive_snapshot(output,&format!("{name}-after-render"),&after_archive)?;
            let archive_review=json!({"contract":"Entire decoded protobuf Session equality and exact metadata equality; no values, GUIDs or list order normalized away. Protobuf map entry encoding order is immaterial.",
                "before_sha256":digest(&archive),"before_noop_sha256":digest(&noop_archive),"after_init_sha256":digest(&initialized_archive),"after_render_sha256":digest(&after_archive),
                "before_noop_documents_equal":original_snapshot.0==noop_snapshot.0,"before_noop_metadata_equal":original_snapshot.1==noop_snapshot.1,
                "after_init_documents_equal":original_snapshot.0==initialized_snapshot.0,"after_init_metadata_equal":original_snapshot.1==initialized_snapshot.1,
                "after_render_documents_equal":original_snapshot.0==after_snapshot.0,"after_render_metadata_equal":original_snapshot.1==after_snapshot.1,
                "rgba_changed_pixels":changed,"rgba_max_channel_delta":max_delta});
            std::fs::write(output.join(format!("{name}-archive-review.json")),serde_json::to_vec_pretty(&archive_review)?)?;
            ensure!(original_snapshot==initialized_snapshot,"Renderer initialization changed decoded original source or metadata; raw/decoded snapshots retained");
            ensure!(original_snapshot==after_snapshot,"Rendering changed decoded original source or metadata; raw/decoded snapshots retained");
            if let Some((first_production,first_course))=&baseline {
                if mode.starts_with("restored") {ensure!(&production.rgba==first_production&&&actual==first_course,"Returned baseline setting did not restore exact paired images");}
                else {ensure!(&production.rgba!=first_production&&&actual!=first_course,"Named control must change both actual images");}
            }else{baseline=Some((production.rgba.clone(),actual.clone()));}
            if changed!=0{failures.push(name.clone());}
            records.push(json!({"case":name,"source_sha256":digest(bytes),"course_archive_sha256":digest(&archive),"course_archive_integrity":archive_review,"production":identity,"uploaded_closed_mesh":uploaded_closed,
                "course":{"viewport":size,"density":1,"samples":1,"matrix_f32":matrix,"anchor":course.anchor(),"format":"Rgba8UnormSrgb","clear_linear":[1.,1.,1.,1.],"lit":lit,"backface":backface,"opacity":opacity,"device":"same actual production device and queue"},
                "production_rgba_sha256":digest(&production.rgba),"course_rgba_sha256":digest(&actual),"changed_pixels":changed,"max_channel_delta":max_delta}));
            std::fs::write(output.join("manifest.json"),serde_json::to_vec_pretty(&json!({"cases":records,"failed_pairs":failures,"comparison":"entire RGBA; zero tolerance; exact source/MVP/device/format/settings","scope":"Eight named single-density single-sample native images. Returned settings use fresh production instances; no production lifetime, pick or browser acceptance.","production_source_closedness":"Actual app::clipping::verify_solids before source walk; uploaded closed flag asserted, never forged"}))?)?;
            eprintln!("FINISH {name}: {changed} changed pixels");
        }
    }
    ensure!(records.len()==8&&failures.is_empty(),"Matched material differences: {failures:?}");Ok(())
}
