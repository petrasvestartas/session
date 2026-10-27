use anyhow::{Result, ensure};
use prost::Message;
use serde_json::{Value, json};
use session_rust::{PointCloud, Quaternion, Session};
use sha2::{Digest, Sha256};
use std::{io::Write, path::Path};
use viewer_journey::{
    background::Background,
    camera::{Camera, Projection, Unit},
    comparison,
    renderer::Renderer,
    viewport::Viewport,
};

const CSS: [u32; 2] = [320, 240];

fn source() -> (Vec<u8>, Vec<[f64; 3]>) {
    let points: Vec<_> = (0..13)
        .flat_map(|z| {
            (0..17).map(move |x| {
                [
                    (x as f64 - 8.0) * 50.0,
                    if x < 8 { -50.0 } else { 50.0 },
                    (z as f64 - 6.0) * 50.0,
                ]
            })
        })
        .collect();
    let mut cloud = PointCloud::from_coords(
        points.iter().flatten().copied().collect(),
        points.iter().flat_map(|_| [80, 140, 220, 255]).collect(),
        vec![],
    );
    cloud.set_guid("6d0aaee0-0430-4000-8100-000000000101".into());
    cloud.point_size = 3.0;
    let mut document = Session::new("cloud-density-witness");
    document.set_guid("6d0aaee0-0430-4000-8100-000000000001".into());
    document
        .objects
        .set_guid("6d0aaee0-0430-4000-8100-000000000002".into());
    document
        .tree
        .set_guid("6d0aaee0-0430-4000-8100-000000000003".into());
    document
        .graph
        .set_guid("6d0aaee0-0430-4000-8100-000000000004".into());
    document.add_pointcloud(cloud, None).unwrap();
    for (i, node) in document.tree.nodes().iter().enumerate() {
        node.borrow_mut()
            .set_guid(format!("6d0aaee0-0430-4000-9100-{i:012}"));
    }
    (document.to_proto().encode_to_vec(), points)
}

fn pixels(renderer: &Renderer, matrix: [f32; 16], size: [u32; 2]) -> Result<Vec<u8>> {
    let [width, height] = size;
    let texture = renderer.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("course cloud witness"),
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
        label: Some("course cloud witness readback"),
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
    rx.recv_timeout(std::time::Duration::from_secs(1))??;
    let mapped = read.slice(..).get_mapped_range();
    let bytes = mapped
        .chunks_exact(pitch as usize)
        .flat_map(|row| row[..width as usize * 4].iter().copied())
        .collect();
    drop(mapped);
    read.unmap();
    Ok(bytes)
}

fn ppm(path: &Path, bytes: &[u8], size: [u32; 2]) -> Result<()> {
    let mut file = std::io::BufWriter::new(std::fs::File::create(path)?);
    write!(file, "P6\n{} {}\n255\n", size[0], size[1])?;
    for pixel in bytes.chunks_exact(4) {
        file.write_all(&pixel[..3])?;
    }
    file.flush()?;
    Ok(())
}

fn clip(matrix: [f32; 16], p: [f64; 3]) -> [f64; 4] {
    std::array::from_fn(|r| {
        (0..3)
            .map(|c| f64::from(matrix[c * 4 + r]) * p[c])
            .sum::<f64>()
            + f64::from(matrix[12 + r])
    })
}

fn matrix(value: &Value) -> Result<[f32; 16]> {
    serde_json::from_value(value.clone()).map_err(Into::into)
}

fn main() -> Result<()> {
    let require_parity = std::env::args().any(|argument| argument == "--require-parity");
    let output = std::env::args_os()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("supply evidence directory"))?;
    let output = Path::new(&output);
    std::fs::create_dir_all(output)?;
    let (bytes, points) = source();
    std::fs::write(output.join("source.pb"), &bytes)?;
    let mut camera = Camera::default();
    camera.unit = Unit::Millimeters;
    camera.target = [0.0; 3];
    camera.distance = 3000.0;
    camera.radius = 0.0;
    camera.orientation = Quaternion::identity();
    camera.projection = Projection::Perspective;
    camera.aspect = f64::from(CSS[0]) / f64::from(CSS[1]);
    let course_matrix = camera.view_projection_anchored([0.0; 3]).to_f32();
    let mut cases = Vec::new();
    for edl in [0.0, 0.25] {
        for density in [1.0_f32, 2.0] {
            for samples in [1, 4] {
                let size = CSS.map(|v| v * density as u32);
                let stem = format!("cloud-dpr{}-samples{samples}-edl{edl}", density as u32);
                eprintln!("START {stem}");
                let production =
                    pollster::block_on(current_viewer::selftest::cloud_witness::render(
                        &bytes, size, density, samples, edl,
                    ))?;
                ensure!(
                    viewer_journey::production_cloud::POINT_SOURCE == production.point_source
                        && viewer_journey::production_cloud::RESOLVE_SOURCE
                            == production.resolve_source,
                    "course cloud extraction differs from the actual production shader composition"
                );
                ppm(
                    &output.join(format!("{stem}-production.ppm")),
                    &production.rgba,
                    size,
                )?;
                std::fs::write(
                    output.join("production-splat-compiled.wgsl"),
                    &production.point_source,
                )?;
                std::fs::write(
                    output.join("production-resolve-compiled.wgsl"),
                    &production.resolve_source,
                )?;
                let scene = comparison::original_scene(&bytes).map_err(anyhow::Error::msg)?;
                let mut course = Renderer::new(
                    production.device,
                    production.queue,
                    wgpu::TextureFormat::Rgba8UnormSrgb,
                    &scene,
                );
                course.resize(Viewport {
                    width: size[0],
                    height: size[1],
                });
                course.set_density(density);
                course.set_samples(samples);
                course.cloud_camera(&camera);
                course.cloud_witness(edl).map_err(anyhow::Error::msg)?;
                ensure!(
                    course.anchor() == [0.0; 3]
                        && production.identity["anchor"] == json!([0.0, 0.0, 0.0])
                );
                ensure!(course.samples() == samples && production.identity["samples"] == samples);
                ensure!(
                    production.identity["density"] == density
                        && production.identity["viewport"] == json!(size)
                );
                ensure!(
                    production.identity["format"] == "Rgba8UnormSrgb"
                        && Background::default().rgb() == [1.0; 3]
                );
                let actual = pixels(&course, course_matrix, size)?;
                ppm(&output.join(format!("{stem}-course.ppm")), &actual, size)?;
                let diagnostic = course.cloud_witness(edl).map_err(anyhow::Error::msg)?;
                ensure!(diagnostic["prepared"] == true);
                ensure!(diagnostic["draws"].as_array().unwrap().len() == 1);
                let p = &production.identity["records"][0];
                let c = &diagnostic["draws"][0]["records"][0];
                ensure!(p["count"] == points.len() && c["count"] == points.len());
                ensure!(
                    p["selected_point"] == 0 && c["word39"] == 0,
                    "resident witness must isolate sizing from streamed ownership"
                );
                ensure!(
                    p["flags"].as_u64().unwrap() & 1 == 0 && c["flags"].as_u64().unwrap() & 1 == 0
                );
                ensure!(
                    production.identity["cloud"]["spacing"] == diagnostic["draws"][0]["spacing"]
                );
                let prod_m = matrix(&p["mvp_model"])?;
                let course_m = matrix(&c["mvp_model"])?;
                ensure!(prod_m == production.matrix && course_m == course_matrix);
                let pk = p["k"].as_f64().unwrap();
                let ck = c["k"].as_f64().unwrap();
                let mut projections = Vec::new();
                for point in &points {
                    let a = clip(prod_m, *point);
                    let b = clip(course_m, *point);
                    ensure!(a[3] > 0.0 && b[3] > 0.0);
                    let ndc_delta = (0..3)
                        .map(|i| (a[i] / a[3] - b[i] / b[3]).abs())
                        .fold(0.0_f64, f64::max);
                    ensure!(ndc_delta <= 1.0e-7, "camera NDC mismatch: {ndc_delta}");
                    ensure!(
                        (b[3] / a[3] - 1000.0).abs() < 0.001,
                        "homogeneous camera scale changed"
                    );
                    let prod_radius = pk * f64::from(size[1]) / a[3];
                    let course_radius = ck * f64::from(size[1]) / b[3];
                    projections.push(json!({
                        "point_mm": point, "production_clip": a, "course_clip": b,
                        "ndc_max_delta": ndc_delta, "production_k_over_w": pk / a[3],
                        "course_k_over_w": ck / b[3], "course_over_production_radius": course_radius / prod_radius,
                        "production_unclamped_radius": prod_radius, "course_unclamped_radius": course_radius,
                    }));
                }
                ensure!(actual.len() == production.rgba.len());
                let ink = |image: &[u8]| {
                    image
                        .chunks_exact(4)
                        .filter(|p| p[..3].iter().any(|v| *v < 250))
                        .count()
                };
                let yellow = |image: &[u8]| {
                    image
                        .chunks_exact(4)
                        .filter(|p| p[0] > 245 && p[1] > 245 && p[2] < 20)
                        .count()
                };
                ensure!(
                    ink(&actual) > 0 && ink(&production.rgba) > 0,
                    "blank witness"
                );
                ensure!(
                    yellow(&actual) == 0 && yellow(&production.rgba) == 0,
                    "unselected resident point is yellow"
                );
                let changed = actual
                    .chunks_exact(4)
                    .zip(production.rgba.chunks_exact(4))
                    .filter(|(a, b)| a != b)
                    .count();
                cases.push(json!({
                    "name": stem, "source_sha256": format!("{:x}", Sha256::digest(&bytes)),
                    "css_viewport": CSS, "shared_device": true, "production": production.identity,
                    "course": diagnostic, "projections": projections,
                    "compiled_production_splat_sha256": format!("{:x}", Sha256::digest(&production.point_source)),
                    "compiled_production_resolve_sha256": format!("{:x}", Sha256::digest(&production.resolve_source)),
                    "diff": {"changed_pixels": changed, "production_ink": ink(&production.rgba), "course_ink": ink(&actual)},
                    "parity_accepted": false,
                }));
                std::fs::write(
                    output.join("manifest.json"),
                    serde_json::to_vec_pretty(&json!({
                        "purpose": "actual production cloud projection and DPR diagnosis; resident source isolates page ownership",
                        "cases": cases,
                    }))?,
                )?;
                eprintln!("{stem}: {changed} changed pixels");
                if require_parity {
                    ensure!(changed == 0, "{stem}: actual production/course pixels differ");
                }
            }
        }
    }
    Ok(())
}
