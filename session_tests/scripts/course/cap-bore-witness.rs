use anyhow::{Result, ensure};
use prost::Message;
use serde_json::{Value, json};
use session_rust::{BRep, Point, Xform};
use sha2::{Digest, Sha256};
use std::{io::Write, path::Path};
use viewer_journey::{
    background::Background, renderer::Renderer, scene::Scene, viewport::Viewport,
};

fn pixels(renderer: &Renderer, matrix: [f32; 16], size: [u32; 2]) -> Result<Vec<u8>> {
    let [width, height] = size;
    let texture = renderer.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("matched course stroke"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    renderer.draw(
        &texture.create_view(&Default::default()),
        &Background::default(),
        &matrix,
    );
    let pitch = (width * 4).div_ceil(256) * 256;
    let read = renderer.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("matched course readback"),
        size: u64::from(pitch) * u64::from(height),
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut encoder = renderer.device.create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(
        texture.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &read,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(pitch),
                rows_per_image: Some(height),
            },
        },
        texture.size(),
    );
    renderer.queue.submit([encoder.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    read.slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            let _ = tx.send(result);
        });
    renderer.device.poll(wgpu::PollType::Wait {
        submission_index: None,
        timeout: Some(std::time::Duration::from_secs(60)),
    })?;
    rx.recv()??;
    let mapped = read.slice(..).get_mapped_range();
    let data = mapped
        .chunks_exact(pitch as usize)
        .flat_map(|row| row[..width as usize * 4].iter().copied())
        .collect();
    drop(mapped);
    read.unmap();
    Ok(data)
}

fn ppm(path: &Path, rgba: &[u8], size: [u32; 2]) -> Result<()> {
    let mut file = std::io::BufWriter::new(std::fs::File::create(path)?);
    write!(file, "P6\n{} {}\n255\n", size[0], size[1])?;
    for pixel in rgba.chunks_exact(4) {
        file.write_all(&pixel[..3])?;
    }
    file.flush()?;
    Ok(())
}

fn query(
    renderer: &mut Renderer,
    scene: &Scene,
    matrix: [f32; 16],
    point: [f64; 3],
    mode: viewer_journey::pick_sources::Mode,
) -> Result<Option<viewer_journey::pick_sources::Target>> {
    let clip: [f64; 4] = std::array::from_fn(|r| {
        (0..3)
            .map(|c| f64::from(matrix[c * 4 + r]) * point[c])
            .sum::<f64>()
            + f64::from(matrix[12 + r])
    });
    renderer.set_pick_mode(mode);
    renderer
        .request_pick(
            [(clip[0] / clip[3]) as f32, (clip[1] / clip[3]) as f32],
            matrix,
        )
        .map_err(anyhow::Error::msg)?;
    renderer.device.poll(wgpu::PollType::Wait {
        submission_index: None,
        timeout: Some(std::time::Duration::from_secs(60)),
    })?;
    Ok(renderer
        .take_pick(matrix)
        .ok_or_else(|| anyhow::anyhow!("query not ready"))?
        .map_err(anyhow::Error::msg)?
        .and_then(|hit| renderer.resolve_pick(scene, hit)))
}

fn retained_cap_sources(scene: &Scene) -> Vec<Value> {
    let mut rows = Vec::new();
    for row in scene.objects() {
        let (kind, source_guid, bytes) = if let Some(cad) = &row.cad {
            ("BRep", cad.source.guid().to_owned(), cad.source.to_proto().encode_to_vec())
        } else if let Some(session_rust::Geometry::Plane(plane)) = &row.original {
            ("Plane", plane.guid().to_owned(), plane.to_proto().encode_to_vec())
        } else { panic!("unexpected cap fixture source"); };
        rows.push(json!({"kind":kind,"owner_guid":row.guid,"source_guid":source_guid,
            "model":row.model.m,"source_sha256":format!("{:x}",Sha256::digest(bytes))}));
    }
    rows.sort_by(|a,b|a["owner_guid"].as_str().cmp(&b["owner_guid"].as_str()));
    rows
}

fn cap_screen(matrix: [f32;16], point: [f64;3], size: [u32;2]) -> Result<(usize,usize)> {
    let q:[f64;4]=std::array::from_fn(|r|(0..3).map(|c|f64::from(matrix[c*4+r])*point[c]).sum::<f64>()+f64::from(matrix[12+r]));
    ensure!(q.iter().all(|v|v.is_finite())&&q[3]>0.0);
    let x=((q[0]/q[3]+1.0)*0.5*f64::from(size[0])).floor();
    let y=((1.0-q[1]/q[3])*0.5*f64::from(size[1])).floor();
    ensure!(x>=1.0&&y>=1.0&&x<f64::from(size[0]-1)&&y<f64::from(size[1]-1));
    Ok((x as usize,y as usize))
}

fn main() -> Result<()> {
    let args:Vec<_>=std::env::args().collect();
    let output=Path::new(args.get(1).ok_or_else(||anyhow::anyhow!("cap-bore-witness FRESH_OUTPUT"))?);
    ensure!(!output.exists(),"preserve all prior cap evidence");
    std::fs::create_dir_all(output)?;
    // Existing actual cap fixture, now shared by both renderers as one original document.
    let mut brep=BRep::create_block_with_hole(4.0,3.0,0.6,0.65);
    brep.set_guid("b6802f09-1be8-4cc6-a96c-bcae7869d920".into());
    let mut plane=viewer_journey::clipping_geometry::plane_from(
        viewer_journey::clipping_geometry::Mode::Xy,&[[0.0,0.0,0.3]],10.0).map_err(anyhow::Error::msg)?;
    plane.set_guid("b6802f09-1be8-4cc6-a96c-bcae7869d921".into());
    let second=&Xform::translation(10.0,0.0,0.0)*&Xform::scale_xyz(-1.0,0.8,1.3);
    let owner_guids=["b6802f09-1be8-4cc6-a96c-bcae7869d922","b6802f09-1be8-4cc6-a96c-bcae7869d923"];
    let instances=vec![
        session_rust::proto::InstanceRef{guid:owner_guids[0].into(),definition_guid:brep.guid().into(),..Default::default()},
        session_rust::proto::InstanceRef{guid:owner_guids[1].into(),definition_guid:brep.guid().into(),xform:Some(second.to_proto()),..Default::default()},
        session_rust::proto::InstanceRef{guid:"b6802f09-1be8-4cc6-a96c-bcae7869d924".into(),definition_guid:plane.guid().into(),..Default::default()},
    ];
    let source=session_rust::proto::Session{objects:Some(session_rust::proto::Objects{instances,..Default::default()}),definitions:Some(session_rust::proto::Objects{breps:vec![brep.to_proto()],planes:vec![plane.to_proto()],..Default::default()}),..Default::default()}.encode_to_vec();
    std::fs::write(output.join("source.pb"),&source)?;
    let source_hash=format!("{:x}",Sha256::digest(&source));
    let mut scene=Scene::empty();
    viewer_journey::scene_document::import(&mut scene,&source,"",&Xform::identity(),0.0).map_err(anyhow::Error::msg)?;
    let cads:Vec<_>=scene.objects().iter().filter(|r|r.cad.is_some()).collect();
    ensure!(scene.objects().len()==3&&cads.len()==2);
    let selected=cads.iter().find(|r|r.guid==owner_guids[1]).ok_or_else(||anyhow::anyhow!("original placed cap owner missing"))?.id;
    let originals=retained_cap_sources(&scene);
    let archive=viewer_journey::scene_archive::save(&scene).map_err(anyhow::Error::msg)?;
    std::fs::write(output.join("original.session"),&archive)?;
    let mut reopened=Scene::empty();viewer_journey::scene_document::open(&mut reopened,&archive).map_err(anyhow::Error::msg)?;
    ensure!(retained_cap_sources(&reopened)==originals);
    let mut camera=viewer_journey::camera::Camera::default();camera.set_view(viewer_journey::camera::View::Top);
    camera.target=[5.0,1.5,0.0];camera.distance=20.0;camera.aspect=4.0/3.0;
    camera.projection=viewer_journey::camera::Projection::Orthographic;
    let matrix=camera.uniform();let size=[640,480];
    let clip=viewer_journey::clipping_rows::current(&scene).map_err(anyhow::Error::msg)?;
    ensure!(clip.len()==1);
    let planes:Vec<_>=clip.iter().map(|r|current_viewer::selftest::stroke_witness::ClipPlane{
        normal:r.plane.normal,offset:r.plane.offset,origin:r.plane.origin,hatch:r.plane.hatch}).collect();
    let families=current_viewer::selftest::stroke_witness::Families{
        lines:false,mesh_edges:false,points:false,markers:false,
        selected_guids:vec![owner_guids[1].into()],
    };
    let production=pollster::block_on(current_viewer::selftest::stroke_witness::render_caps(
        &source,matrix,size,1.0,1,1.0,&planes,&families))?;
    ensure!(production.identity["matrix_f32"]==json!(matrix)&&production.identity["viewport"]==json!(size));
    ensure!(production.identity["format"]=="Rgba8UnormSrgb"&&production.identity["samples"]==1);
    ensure!(production.identity["density"]==1.0&&production.identity["anchor"]==json!([0.0,0.0,0.0]));
    ensure!(production.identity["section_caps_enabled"]==true);
    ensure!(production.identity["verified_closed_rows"].as_array().is_some_and(|rows|rows.len()>=2));
    let ids=production.cap_ids.as_ref().ok_or_else(||anyhow::anyhow!("actual production cap ids missing"))?;
    ensure!(ids.len()==(size[0]*size[1])as usize);
    let mut renderer=Renderer::new(production.device,production.queue,wgpu::TextureFormat::Rgba8UnormSrgb,&scene);
    renderer.resize(Viewport{width:size[0],height:size[1]});renderer.set_density(1.0);renderer.set_samples(1);
    renderer.set_lighting(false,false);renderer.set_ambient(false);renderer.set_opacity(1.0);renderer.set_outlines(false);
    renderer.set_ink_visibility(viewer_journey::ink_ranges::Visibility{lines:false,edges:false,points:false,vertices:false});
    renderer.set_clipping(true,true);renderer.select_objects(&scene,&[selected]).map_err(anyhow::Error::msg)?;
    let mut probes=Vec::new();
    for row in cads {
        let native=production.identity["walked_ink"]["objects"].as_array().unwrap().iter().find(|r|r["guid"]==row.guid).ok_or_else(||anyhow::anyhow!("actual production original owner mapping missing"))?;
        let native_id=native["row"].as_u64().unwrap()as u32+1;
        ensure!(production.identity["uploaded_objects"].as_array().unwrap().iter().any(|r|r["row"]==native["row"]&&r["flags"].as_u64().is_some_and(|flags|flags&(1<<13)!=0)),"actual original must be verified closed");
        for(hole,p)in[(false,[0.5,0.5,0.3]),(true,[2.0,1.5,0.3])]{
            let q=row.model.transform_point(&Point::new(p[0],p[1],p[2]));let world=[q[0],q[1],0.3];
            let (x,y)=cap_screen(matrix,world,size)?;
            let native_pick=ids[y*size[0]as usize+x];
            ensure!(native_pick[0]==if hole{0}else{native_id},"production bore/section must retain its actual original owner");
            let actual=query(&mut renderer,&scene,matrix,world,viewer_journey::pick_sources::Mode::Object)?;
            let again=query(&mut renderer,&scene,matrix,world,viewer_journey::pick_sources::Mode::Object)?;
            let expected=if hole{None}else{Some(viewer_journey::pick_sources::Target{owner:row.id,part:viewer_journey::pick_sources::Part::Object})};
            ensure!(actual==expected&&again==expected,"course first/cached cap queries must retain original owner and empty bore");
            probes.push(json!({"owner_guid":row.guid,"hole":hole,"world":world,"pixel":[x,y],"production_pick":native_pick,"course_pick":format!("{actual:?}"),"selected":row.id==selected}));
        }
    }
    let actual=pixels(&renderer,matrix,size)?;
    ensure!(renderer.cap_usage()[1]==2,"both actual originals must classify as closed");
    for probe in &probes {
        let xy=probe["pixel"].as_array().unwrap();let at=((xy[1].as_u64().unwrap()*u64::from(size[0])+xy[0].as_u64().unwrap())*4)as usize;
        for image in [&production.rgba,&actual] {
            let p=&image[at..at+4];
            if probe["hole"]==true {ensure!(p[..3].iter().all(|v|*v>245),"actual bore is empty white");}
            else if probe["selected"]==true {ensure!(p[0]>220&&p[1]>180&&p[2]<80,"selected section is yellow");}
            else {ensure!(p[0]>100&&p[0]<245&&p[0].abs_diff(p[1])<=2&&p[1].abs_diff(p[2])<=2,"unselected section is actual grey cap, not blue face");}
        }
    }
    let changed=production.rgba.chunks_exact(4).zip(actual.chunks_exact(4)).filter(|(a,b)|a!=b).count();
    let max_delta=production.rgba.iter().zip(&actual).map(|(a,b)|a.abs_diff(*b)).max().unwrap_or(0);
    for(name,data)in[("production",&production.rgba),("course",&actual)]{
        ppm(&output.join(format!("{name}.ppm")),data,size)?;std::fs::write(output.join(format!("{name}.rgba")),data)?;
    }
    ensure!(retained_cap_sources(&scene)==originals&&format!("{:x}",Sha256::digest(&source))==source_hash);
    ensure!(viewer_journey::scene_archive::save(&scene).map_err(anyhow::Error::msg)?==archive);
    std::fs::write(output.join("manifest.json"),serde_json::to_vec_pretty(&json!({
        "scope":"One actual BRep bore section fixture; selected mirrored/nonuniform original plus unselected original in one frame; no cap matrix",
        "source_sha256":source_hash,"matrix":matrix,"size":size,"density":1,"samples":1,"format":"Rgba8UnormSrgb",
        "production":production.identity,"course_picks":probes,"source_archive_unchanged":true,
        "changed_pixels":changed,"max_rgba_delta":max_delta,"strict_parity_pass":changed==0
    }))?)?;
    ensure!(changed==0&&max_delta==0,"actual production/course cap frames differ");
    Ok(())
}
