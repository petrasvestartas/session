use super::{FileDoc, FrameInput, Gpu, Scene, Session, Xform};
use crate::engine::gpu::View;
use serde_json::{Value, json};
use std::rc::Rc;

pub use crate::engine::gpu::clip::ClipPlane;

pub struct Frame {
    pub rgba: Vec<u8>,
    pub identity: Value,
    pub ribbon_source: String,
    pub vector_source: String,
    pub plane_source: String,
    pub glyph_source: String,
    pub sphere_source: String,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

pub struct SheetFrame {
    pub rgba: Vec<u8>,
    pub ids: Vec<[u32; 2]>,
    pub identity: Value,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

pub async fn render_sheet(
    init: crate::app::scene::SheetInit, matrix: [f32; 16], viewport: [u32; 2],
    density: f32, samples: u32, selected: bool,
) -> anyhow::Result<SheetFrame> {
    anyhow::ensure!(viewport.iter().all(|v| *v > 0) && matrix.iter().all(|v| v.is_finite()));
    anyhow::ensure!(density.is_finite() && density > 0.0 && matches!(samples, 1 | 4));
    let mut gpu = Gpu::new_headless(viewport[0], viewport[1]).await?;
    let info = gpu.ctx.device.adapter_info();
    anyhow::ensure!(info.device_type == wgpu::DeviceType::Cpu
        && ((info.backend == wgpu::Backend::Dx12 && (info.vendor, info.device) == (0x1414, 0x8c))
            || (info.backend == wgpu::Backend::Vulkan && info.name.to_lowercase().contains("swiftshader"))),
        "Sheet witness requires a verified software adapter: {info:?}");
    gpu.logical_size = viewport.map(|v| f64::from(v) / f64::from(density));
    gpu.view = View {ssao:false,show_grid:false,show_points:false,show_lines:true,
        show_mesh_edges:false,show_outlines:false,markers:false,cloud_size:1.0,
        edl_strength:0.0,lod_px:0.0,thickness_px:1.0,feather_px:1.0,lit:false,
        backface:false,opacity:1.0,msaa_forced:Some(samples),perf:false,spin:false};
    let resident = init.resident;
    let placement = init.place.m;
    let mut scene = Scene::new();
    scene.add_sheet(init, &mut gpu);
    let rebase = gpu.rebase_anchor(&session_rust::Point::new(0.0, 0.0, 0.0), 1.0, 0.0);
    anyhow::ensure!(rebase.anchor[0] == 0.0 && rebase.anchor[1] == 0.0 && rebase.anchor[2] == 0.0);
    anyhow::ensure!(scene.tables.seg.sheet_rows.is_empty());
    anyhow::ensure!(gpu.segments.ribbon_count() == resident && gpu.segments.editable_ribbon_count() == 0,
        "Source rows must use the actual streamed-sheet GPU table");
    if selected {gpu.set_selected(0, true);}
    let input = FrameInput {view_proj:Xform::from_matrix(matrix.map(f64::from)),clear:wgpu::Color::WHITE,now_ms:0.0};
    let rgba = gpu.render_offscreen(&input);
    let ids = gpu.render_ids_offscreen(&input);
    anyhow::ensure!(gpu.frame.mvp_f32 == matrix && gpu.targets.samples == samples);
    let identity = json!({"adapter":format!("{info:?}"),"viewport":viewport,"density":density,
        "samples":samples,"format":format!("{:?}",gpu.config.format),"placement_f64":placement,
        "matrix_f32":gpu.frame.mvp_f32,"resident":resident,"selected":selected,
        "sheet_flag":gpu.objects.row(0).is_some_and(|row|row.flags & crate::engine::gpu::Instance::FLAG_SHEET != 0),
        "ingress":"Scene::add_sheet(SheetInit, SheetFields, SheetRows); resident slice, no network fetch"});
    Ok(SheetFrame {rgba, ids, identity, device:gpu.ctx.device, queue:gpu.ctx.queue})
}

/// Native grid comparison uses the maintained camera, background and actual grid pass.
pub async fn render_grid(size: u32) -> anyhow::Result<(Vec<u8>, [f32; 16], wgpu::Device, wgpu::Queue, Value)> {
    let mut gpu = Gpu::new_headless(size, size).await?;
    gpu.view.show_grid = true;
    gpu.view.ssao = false;
    gpu.view.show_outlines = false;
    gpu.view.msaa_forced = Some(1);
    let mut scene = Scene::new();
    scene.add_file(FileDoc {name:"empty grid".into(),place:Xform::identity(),session:Rc::new(Session::new("empty grid")),point_px:0.0,display_only:false});
    scene.upload_to(&mut gpu);
    let mut camera = super::Camera::new();
    camera.set_view(super::View::Top);
    camera.distance = 12.0;
    camera.perspective = false;
    camera.update_position();
    let matrix = camera.view_proj_anchored(1.0, &session_rust::Point::new(0.0, 0.0, 0.0));
    let rgba = gpu.render_offscreen(&FrameInput {view_proj:matrix.clone(),clear:wgpu::Color::WHITE,now_ms:0.0});
    let identity = json!({"adapter":format!("{:?}",gpu.ctx.device.adapter_info()),"viewport":[size,size],"samples":1,"camera_distance_metres":12,"plane":"XY","encoding":"Rgba8UnormSrgb"});
    Ok((rgba, matrix.to_f32(), gpu.ctx.device, gpu.ctx.queue, identity))
}

fn uploaded_pen(gpu: &Gpu) -> anyhow::Result<Value> {
    let ribbons = gpu
        .segments
        .first_ribbon_buffer()
        .ok_or_else(|| anyhow::anyhow!("no uploaded ribbon"))?;
    let read = gpu.ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("stroke witness uploaded pen"),
        size: 12,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut encoder = gpu.ctx.device.create_command_encoder(&Default::default());
    encoder.copy_buffer_to_buffer(ribbons, 12, &read, 0, 4);
    encoder.copy_buffer_to_buffer(gpu.frame.line_uniform_buffer(), 0, &read, 4, 4);
    encoder.copy_buffer_to_buffer(ribbons, 32, &read, 8, 4);
    gpu.ctx.queue.submit([encoder.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    read.slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            let _ = tx.send(result);
        });
    gpu.ctx.device.poll(wgpu::PollType::Wait {
        submission_index: None,
        timeout: Some(std::time::Duration::from_secs(30)),
    })?;
    rx.recv_timeout(std::time::Duration::from_secs(1))??;
    let bytes = read.slice(..).get_mapped_range();
    let radius = f32::from_le_bytes(bytes[0..4].try_into()?);
    let thickness = f32::from_le_bytes(bytes[4..8].try_into()?);
    let color = u32::from_le_bytes(bytes[8..12].try_into()?);
    let pen = json!({
        "radius": radius, "thickness": thickness, "packed_color": color,
        "packed_color_hex": format!("0x{color:08x}"), "readback_bytes": bytes.to_vec(),
        "ribbon_row": 0, "radius_offset": 12, "thickness_uniform_offset": 0, "color_offset": 32,
    });
    drop(bytes);
    read.unmap();
    Ok(pen)
}

fn uploaded_clip(gpu: &Gpu) -> anyhow::Result<Value> {
    let read = gpu.ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("section witness clipping uniform"),
        size: 448,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut encoder = gpu.ctx.device.create_command_encoder(&Default::default());
    encoder.copy_buffer_to_buffer(gpu.frame.clip_uniform_buffer(), 0, &read, 0, 448);
    gpu.ctx.queue.submit([encoder.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    read.slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            let _ = tx.send(result);
        });
    gpu.ctx.device.poll(wgpu::PollType::Wait {
        submission_index: None,
        timeout: Some(std::time::Duration::from_secs(30)),
    })?;
    rx.recv_timeout(std::time::Duration::from_secs(1))??;
    let bytes = read.slice(..).get_mapped_range();
    let count = u32::from_le_bytes(bytes[416..420].try_into()?);
    let samples = u32::from_le_bytes(bytes[420..424].try_into()?);
    anyhow::ensure!(count <= 6, "invalid uploaded clipping count");
    let planes: Vec<[f32; 4]> = (0..count as usize)
        .map(|index| {
            std::array::from_fn(|axis| {
                let at = index * 16 + axis * 4;
                f32::from_le_bytes(bytes[at..at + 4].try_into().unwrap())
            })
        })
        .collect();
    let value = json!({"count": count, "samples": samples, "planes": planes, "readback_bytes": bytes.to_vec()});
    drop(bytes);
    read.unmap();
    Ok(value)
}

/// Render original source units through the production scene upload and ribbon passes.
pub async fn render(
    source: &[u8],
    matrix: [f32; 16],
    viewport: [u32; 2],
    density: f32,
    samples: u32,
    thickness: f32,
) -> anyhow::Result<Frame> {
    render_clipped(source, matrix, viewport, density, samples, thickness, &[]).await
}

pub async fn render_clipped(
    source: &[u8],
    matrix: [f32; 16],
    viewport: [u32; 2],
    density: f32,
    samples: u32,
    thickness: f32,
    planes: &[ClipPlane],
) -> anyhow::Result<Frame> {
    render_inner(source, matrix, viewport, density, samples, thickness, planes, &Families::default(), true).await
}

pub struct Families {
    pub lines: bool,
    pub mesh_edges: bool,
    pub points: bool,
    pub markers: bool,
    pub selected_guids: Vec<String>,
}

impl Default for Families {
    fn default() -> Self {
        Self { lines: true, mesh_edges: false, points: false, markers: false, selected_guids: Vec::new() }
    }
}

pub async fn render_families(
    source: &[u8], matrix: [f32; 16], viewport: [u32; 2], density: f32,
    samples: u32, thickness: f32, planes: &[ClipPlane], families: &Families,
) -> anyhow::Result<Frame> {
    render_inner(source, matrix, viewport, density, samples, thickness, planes, families, false).await
}

/// Serial diagnostic call: use and restore the production plane-size setting.
pub async fn render_plane_size(
    source: &[u8], matrix: [f32; 16], viewport: [u32; 2], density: f32,
    samples: u32, thickness: f32, families: &Families, size: f64,
) -> anyhow::Result<Frame> {
    anyhow::ensure!(size.is_finite() && size > 0.0 && (size as f32).is_finite());
    struct Restore(f64);
    impl Drop for Restore {
        fn drop(&mut self) { crate::engine::gpu::planes::set_plane_size(self.0); }
    }
    let _restore = Restore(crate::engine::gpu::planes::plane_size());
    crate::engine::gpu::planes::set_plane_size(size);
    render_inner(source, matrix, viewport, density, samples, thickness, &[], families, false).await
}

fn uploaded_plane_size(gpu: &Gpu) -> anyhow::Result<f32> {
    let read=gpu.ctx.device.create_buffer(&wgpu::BufferDescriptor{label:Some("plane witness size uniform"),size:4,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::MAP_READ,mapped_at_creation:false});
    let mut encoder=gpu.ctx.device.create_command_encoder(&Default::default());
    encoder.copy_buffer_to_buffer(gpu.frame.line_uniform_buffer(),76,&read,0,4);
    gpu.ctx.queue.submit([encoder.finish()]);
    let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|v|{let _=tx.send(v);});
    gpu.ctx.device.poll(wgpu::PollType::Wait{submission_index:None,timeout:Some(std::time::Duration::from_secs(30))})?;
    rx.recv_timeout(std::time::Duration::from_secs(1))??;
    let bytes=read.slice(..).get_mapped_range();let value=f32::from_le_bytes(bytes[..4].try_into()?);drop(bytes);read.unmap();Ok(value)
}

fn walked_ink(scene: &Scene) -> Value {
    let objects: Vec<_> = scene.tables.obj.rows.iter().enumerate().map(|(row, object)| {
        let identity=scene.identity_of(row as u32);
        json!({"row":row,"document":identity.as_ref().map(|(doc,_)|*doc),
            "guid":identity.as_ref().map(|(_,guid)|guid.as_ref()),"placement_f64":object.place.m,
            "flags":object.flags,"color":object.color,"edge_color":object.edge_color})
    }).collect();
    let ribbons: Vec<_> = scene.tables.seg.ribbons.iter().map(|row|json!({
        "p0":row.p0,"p1":row.p1,"radius":row.radius,"instance":row.instance_id,
        "color":row.color,"facing":row.facing,
    })).collect();
    let heads: Vec<_> = scene.tables.lanes.get::<crate::engine::gpu::vectors::VectorRows>()
        .into_iter().flat_map(|rows|&rows.rows).map(|row|json!({
            "start":row.start,"end":row.end,"radius":row.radius,"instance":row.instance_id,
            "color":row.color,"head":row.head,"heads":row.heads,"pad":row.pad,
        })).collect();
    let chains:Vec<_>=scene.tables.seg.ribbon_chains.iter().map(|r|[r.start,r.end]).collect();
    let planes:Vec<_>=scene.tables.lanes.get::<crate::engine::gpu::planes::PlaneRows>()
        .into_iter().flat_map(|rows|&rows.rows).map(|row|json!({
            "origin":row.origin,"instance":row.instance_id,"x":row.x,"radius":row.radius,"y":row.y,"pad":row.pad,
        })).collect();
    let glyph=|row:&crate::engine::gpu::glyphs::GlyphPoint|json!({"center":row.center,"radius":row.radius,"color":row.color,"instance":row.instance_id,"facing":row.facing,"facing_ext":row.facing_ext});
    let dots:Vec<_>=scene.tables.glyph.dots.iter().map(glyph).collect();
    let spheres:Vec<_>=scene.tables.glyph.spheres.iter().map(glyph).collect();
    json!({"stage":"actual production Scene::add_file walk before upload","dots":dots,"spheres":spheres,
        "objects":objects,"ribbons":ribbons,"chains":chains,"head_marks":scene.tables.seg.ribbon_heads,"vectors":heads,"planes":planes,
        "pipe_count":scene.tables.seg.pipes.len(),"sheet_count":scene.tables.seg.sheet_rows.len()})
}

async fn render_inner(
    source: &[u8], matrix: [f32; 16], viewport: [u32; 2], density: f32,
    samples: u32, thickness: f32, planes: &[ClipPlane], families: &Families, require_pen: bool,
) -> anyhow::Result<Frame> {
    anyhow::ensure!(viewport.iter().all(|v| *v > 0));
    anyhow::ensure!(matrix.iter().all(|v| v.is_finite()));
    anyhow::ensure!(density.is_finite() && density > 0.0);
    anyhow::ensure!(thickness.is_finite() && thickness > 0.0);
    anyhow::ensure!(matches!(samples, 1 | 4));
    eprintln!("stroke witness: opening production GPU");
    let mut gpu = Gpu::new_headless(viewport[0], viewport[1]).await?;
    eprintln!("stroke witness: production GPU opened");
    let adapter = gpu.ctx.device.adapter_info();
    anyhow::ensure!(
        adapter.device_type == wgpu::DeviceType::Cpu
            && ((adapter.backend == wgpu::Backend::Vulkan
                && adapter.name.to_lowercase().contains("swiftshader"))
                || (adapter.backend == wgpu::Backend::Dx12
                    && adapter.name == "Microsoft Basic Render Driver")),
        "stroke witness requires a verified software adapter: {adapter:?}"
    );
    gpu.logical_size = viewport.map(|v| f64::from(v) / f64::from(density));
    gpu.view = View {
        ssao: false,
        show_grid: false,
        show_points: families.points,
        show_lines: families.lines,
        show_mesh_edges: families.mesh_edges,
        show_outlines: false,
        markers: families.markers,
        cloud_size: 1.0,
        edl_strength: 0.0,
        lod_px: 0.0,
        thickness_px: thickness,
        feather_px: 1.0,
        lit: false,
        backface: false,
        opacity: 1.0,
        msaa_forced: Some(samples),
        perf: false,
        spin: false,
    };
    let session = Session::pb_loads(source).map_err(|e| anyhow::anyhow!("{e:?}"))?;
    let mut scene = Scene::new();
    scene.add_file(FileDoc {
        name: "matched-stroke.pb".into(),
        session: Rc::new(session),
        place: Xform::identity(),
        point_px: 0.0,
        display_only: false,
    });
    let walked_ink=walked_ink(&scene);
    eprintln!("stroke witness: uploading original source");
    scene.upload_to(&mut gpu);
    let rebase = gpu.rebase_anchor(&session_rust::Point::new(0.0, 0.0, 0.0), 1.0, 0.0);
    anyhow::ensure!(rebase.anchor[0] == 0.0 && rebase.anchor[1] == 0.0 && rebase.anchor[2] == 0.0);
    for guid in &families.selected_guids {
        let row=scene.row_of(0,guid).ok_or_else(||anyhow::anyhow!("selected source GUID was not loaded: {guid}"))?;
        gpu.set_selected(row,true);
    }
    gpu.set_clip_planes(planes);
    eprintln!("stroke witness: rendering production frame");
    let rgba = gpu.render_offscreen(&FrameInput {
        view_proj: Xform::from_matrix(matrix.map(f64::from)),
        clear: wgpu::Color::WHITE,
        now_ms: 0.0,
    });
    eprintln!("stroke witness: production readback completed");
    let uploaded_pen = if require_pen || gpu.segments.first_ribbon_buffer().is_some() {uploaded_pen(&gpu)?} else {Value::Null};
    let uploaded_clip = uploaded_clip(&gpu)?;
    let uploaded_plane_size = if walked_ink["planes"].as_array().is_some_and(|rows|!rows.is_empty()) {Some(uploaded_plane_size(&gpu)?)} else {None};
    anyhow::ensure!(gpu.frame.mvp_f32 == matrix, "production matrix changed");
    anyhow::ensure!(
        gpu.targets.samples == samples,
        "production sample count changed"
    );
    let actual_density = gpu.config.width as f32 / gpu.logical_size[0] as f32;
    let uploaded_objects:Vec<_>=(0..gpu.objects.len()).filter_map(|row|gpu.objects.row(row).map(|object|
        json!({"row":row,"model_f32":object.model,"color":object.color,"flags":object.flags}))).collect();
    let identity = json!({
        "adapter": {"name": adapter.name, "driver": adapter.driver,
            "driver_info": adapter.driver_info, "backend": format!("{:?}", adapter.backend),
            "device_type": format!("{:?}", adapter.device_type),
            "vendor": adapter.vendor, "device": adapter.device},
        "viewport": [gpu.config.width, gpu.config.height], "logical_size": gpu.logical_size,
        "density": actual_density, "samples": gpu.targets.samples,
        "format": format!("{:?}", gpu.config.format), "clear_linear": [1.0, 1.0, 1.0, 1.0],
        "matrix_f32": gpu.frame.mvp_f32, "anchor": gpu.objects.anchor(),
        "eye": gpu.frame.eye, "ortho_half_height": gpu.frame.ortho_h,
        "thickness_css": gpu.view.thickness_px, "thickness_physical": gpu.view.thickness_px * actual_density,
        "feather": gpu.view.feather_px, "lit": gpu.view.lit, "ssao": gpu.view.ssao,
        "backface": gpu.view.backface, "opacity": gpu.view.opacity,
        "grid": gpu.view.show_grid, "lines": gpu.view.show_lines,
        "mesh_edges": gpu.view.show_mesh_edges, "points": gpu.view.show_points,
        "outlines": gpu.view.show_outlines, "markers": gpu.view.markers,
        "object_count": scene.object_count(), "ribbons": gpu.segments.ribbon_count(),
        "drawing_sheet": gpu.objects.row(0).is_some_and(|row|
            row.flags & crate::engine::gpu::Instance::FLAG_SHEET != 0),
        "uploaded_pen": uploaded_pen,
        "uploaded_clip": uploaded_clip,
        "walked_ink": walked_ink,
        "selected_guids": families.selected_guids,
        "pen_required":require_pen,
        "uploaded_objects":uploaded_objects,
        "uploaded_plane_size":uploaded_plane_size,
    });
    Ok(Frame {
        rgba,
        identity,
        ribbon_source: crate::engine::pipelines::shared(&crate::engine::pipelines::ink_source(
            shader!("ribbon.wgsl"),
        )),
        vector_source: crate::engine::pipelines::shared(&crate::engine::pipelines::ink_source(
            shader!("vector.wgsl"),
        )),
        plane_source: crate::engine::pipelines::shared(&crate::engine::pipelines::ink_source(
            shader!("plane.wgsl"),
        )),
        glyph_source: crate::engine::pipelines::shared(&crate::engine::pipelines::ink_source(shader!("glyph.wgsl"))),
        sphere_source: crate::engine::pipelines::shared(&crate::engine::pipelines::ink_source(shader!("sphere.wgsl"))),
        device: gpu.ctx.device.clone(),
        queue: gpu.ctx.queue.clone(),
    })
}
