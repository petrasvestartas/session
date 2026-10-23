use crate::app::scene::{FileDoc, Scene};
use crate::camera::Camera;
use crate::engine::gpu::{FrameInput, Gpu, View};
use serde_json::{Value, json};
use session_rust::{Point, Quaternion, Session, Xform};
use std::rc::Rc;

pub mod lod;

pub struct Frame {
    pub rgba: Vec<u8>,
    pub identity: Value,
    pub point_source: String,
    pub resolve_source: String,
    pub matrix: [f32; 16],
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

pub async fn render(
    source: &[u8],
    viewport: [u32; 2],
    density: f32,
    samples: u32,
    edl: f32,
) -> anyhow::Result<Frame> {
    anyhow::ensure!(viewport.iter().all(|v| *v > 0));
    anyhow::ensure!(matches!(density, 1.0 | 2.0) && matches!(samples, 1 | 4));
    anyhow::ensure!(edl.is_finite() && edl >= 0.0);
    let mut gpu = Gpu::new_headless(viewport[0], viewport[1]).await?;
    let adapter = gpu.ctx.device.adapter_info();
    anyhow::ensure!(
        adapter.device_type == wgpu::DeviceType::Cpu
            && adapter.backend == wgpu::Backend::Dx12
            && adapter.name == "Microsoft Basic Render Driver",
        "cloud witness requires Windows WARP: {adapter:?}"
    );
    gpu.logical_size = viewport.map(|v| f64::from(v) / f64::from(density));
    gpu.view = View {
        ssao: false,
        show_grid: false,
        show_points: false,
        show_lines: false,
        show_mesh_edges: false,
        show_outlines: false,
        markers: false,
        cloud_size: 1.0,
        edl_strength: edl,
        lod_px: 2.0,
        thickness_px: 1.0,
        feather_px: 1.0,
        lit: false,
        backface: false,
        opacity: 1.0,
        msaa_forced: Some(samples),
        perf: false,
        spin: false,
    };
    let session = Session::pb_loads(source).map_err(|error| anyhow::anyhow!("{error:?}"))?;
    let mut scene = Scene::new();
    scene.add_file(FileDoc {
        name: "matched-cloud.pb".into(),
        session: Rc::new(session),
        place: Xform::identity(),
        point_px: 0.0,
        display_only: false,
    });
    scene.upload_to(&mut gpu);
    anyhow::ensure!(gpu.objects.anchor() == [0.0; 3]);
    let mut camera = Camera::new();
    camera.target = [0.0; 3];
    camera.distance = 3.0;
    camera.scene_extent = 0.0;
    camera.orientation = Quaternion::identity();
    camera.perspective = true;
    camera.update_position();
    let matrix = camera
        .view_proj_anchored(
            f64::from(viewport[0]) / f64::from(viewport[1]),
            &Point::new(0.0, 0.0, 0.0),
        )
        .to_f32();
    let rgba = gpu.render_offscreen(&FrameInput {
        view_proj: Xform::from_matrix(matrix.map(f64::from)),
        clear: wgpu::Color::WHITE,
        now_ms: 0.0,
    });
    anyhow::ensure!(gpu.targets.samples == samples && gpu.frame.mvp_f32 == matrix);
    anyhow::ensure!(gpu.cloud.clouds.len() == 1 && gpu.splat.records.len() == 1);
    let records: Vec<_> = gpu
        .splat
        .records
        .iter()
        .map(|record| {
            json!({
                "mvp_model": record.mvp_model, "tint": record.tint,
                "first": record.first, "count": record.count, "cum": record.cum, "k": record.k,
                "rot": record.rot, "nrm_first": record.nrm_first, "instance": record.instance,
                "flags": record.flags, "selected_point": record.selected_point,
                "cpu_upload_bytes": bytemuck::bytes_of(record),
            })
        })
        .collect();
    let cloud = &gpu.cloud.clouds[0];
    let row = gpu
        .objects
        .row(cloud.instance)
        .ok_or_else(|| anyhow::anyhow!("missing cloud row"))?;
    let actual_density = gpu.config.width as f32 / gpu.logical_size[0] as f32;
    anyhow::ensure!(actual_density == density && gpu.splat.total == cloud.resident);
    let identity = json!({
        "adapter": {"name": adapter.name, "backend": format!("{:?}", adapter.backend),
            "device_type": format!("{:?}", adapter.device_type), "driver": adapter.driver,
            "driver_info": adapter.driver_info, "vendor": adapter.vendor, "device": adapter.device},
        "record_evidence": "exact CPU records retained after production queue.write_buffer; not GPU readback",
        "viewport": viewport, "logical_size": gpu.logical_size, "density": actual_density,
        "samples": gpu.targets.samples, "format": format!("{:?}", gpu.config.format),
        "clear_linear": [1.0, 1.0, 1.0, 1.0], "edl": gpu.view.edl_strength,
        "cloud_scale": gpu.view.cloud_size, "lod_px": gpu.view.lod_px,
        "matrix_f32": matrix, "anchor": gpu.objects.anchor(), "eye": gpu.frame.eye,
        "ortho_half_height": gpu.frame.ortho_h,
        "camera": {"target_mm": [0.0, 0.0, 0.0], "distance_mm": 3000.0,
            "orientation_xyzw": [0.0, 0.0, 0.0, 1.0], "projection": "perspective",
            "unit": "millimeters", "scene_extent_mm": 0.0, "frame": [0.0, 0.0, 1.0, 1.0]},
        "cloud": {"spacing": cloud.spacing, "resident": cloud.resident,
            "node_count": cloud.node_count, "point_px_css": row.spacing},
        "records": records,
    });
    Ok(Frame {
        rgba,
        identity,
        matrix,
        point_source: crate::engine::pipelines::shared(shader!("splat.wgsl")),
        resolve_source: crate::engine::pipelines::shared(shader!("splat_resolve.wgsl")),
        device: gpu.ctx.device.clone(),
        queue: gpu.ctx.queue.clone(),
    })
}
