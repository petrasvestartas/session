use anyhow::{Result, ensure};
use prost::Message;
use serde_json::json;
use session_rust::{Arrowhead, Color, Line, Session, Xform};
use sha2::{Digest, Sha256};
use std::{io::Write, path::Path};
use viewer_journey::{background::Background, comparison, renderer::Renderer, viewport::Viewport};

const CSS: [u32; 2] = [320, 240];

struct Logger;
impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            eprintln!("{}", record.args());
        }
    }
    fn flush(&self) {}
}

fn source(name: &str, guid: &str, ends: [[f64; 3]; 2]) -> Vec<u8> {
    let mut document = Session::new(name);
    document.set_guid("6d0aaee0-0430-4000-8000-000000000001".into());
    document
        .objects
        .set_guid("6d0aaee0-0430-4000-8000-000000000002".into());
    document
        .tree
        .set_guid("6d0aaee0-0430-4000-8000-000000000003".into());
    document
        .graph
        .set_guid("6d0aaee0-0430-4000-8000-000000000004".into());
    let [a, b] = ends;
    let mut line = Line::new(a[0], a[1], a[2], b[0], b[1], b[2]);
    line.set_guid(guid.into());
    line.name = name.into();
    line.width = 1.0;
    line.linecolor = Color::black();
    line.arrowhead = Arrowhead::NONE;
    document.add_line(line, None);
    for (i, node) in document.tree.nodes().iter().enumerate() {
        node.borrow_mut()
            .set_guid(format!("6d0aaee0-0430-4000-9000-{i:012}"));
    }
    document.to_proto().encode_to_vec()
}

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

fn ink(rgba: &[u8]) -> usize {
    rgba.chunks_exact(4)
        .filter(|p| p[..3].iter().any(|v| *v < 250))
        .count()
}

fn main() -> Result<()> {
    log::set_logger(&Logger).map_err(|error| anyhow::anyhow!("{error}"))?;
    log::set_max_level(log::LevelFilter::Info);
    let output = std::env::args_os()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("supply evidence directory"))?;
    let output = Path::new(&output);
    std::fs::create_dir_all(output)?;
    // Both uploads keep millimetres. This one projection consumes those source units directly.
    let matrix = Xform::perspective(std::f64::consts::PI / 3.0, 4.0 / 3.0, 5000.0, 100.0).to_f32();
    let cases = [
        (
            "diagonal",
            "6d0aaee0-0430-4000-8000-000000000101",
            [[-900.0, -500.0, -2000.0], [800.0, 550.0, -2001.0]],
        ),
        (
            "near-crossing",
            "6d0aaee0-0430-4000-8000-000000000102",
            [[-100.0, -70.0, -50.0], [700.0, 500.0, -1200.0]],
        ),
    ];
    let mut records = Vec::new();
    let mut ribbon_hash = None;
    let ribbon_file = "production-ribbon-compiled.wgsl";
    for (name, guid, ends) in cases {
        let clip = |point: [f64; 3]| -> [f64; 4] {
            std::array::from_fn(|row| {
                (0..3)
                    .map(|col| f64::from(matrix[col * 4 + row]) * point[col])
                    .sum::<f64>()
                    + f64::from(matrix[12 + row])
            })
        };
        let clips = ends.map(clip);
        ensure!(clips.iter().all(|p| p[3] > 0.0));
        let near_distances = clips.map(|p| p[3] - p[2]);
        let near_intersection = if name == "near-crossing" {
            ensure!(near_distances[0] < 0.0 && near_distances[1] > 0.0);
            let t = near_distances[0] / (near_distances[0] - near_distances[1]);
            let point = std::array::from_fn(|i| ends[0][i] + t * (ends[1][i] - ends[0][i]));
            let p = clip(point);
            let pixel = [
                (p[0] / p[3] * 0.5 + 0.5) * f64::from(CSS[0]),
                (0.5 - p[1] / p[3] * 0.5) * f64::from(CSS[1]),
            ];
            ensure!(
                pixel
                    .iter()
                    .enumerate()
                    .all(|(i, v)| *v > 4.0 && *v < f64::from(CSS[i]) - 4.0)
            );
            Some(json!({"world_mm": point, "pixel_css": pixel}))
        } else {
            ensure!(near_distances.iter().all(|v| *v > 0.0));
            None
        };
        let bytes = source(name, guid, ends);
        let source_file = format!("{name}.pb");
        std::fs::write(output.join(&source_file), &bytes)?;
        for density in [1.0_f32, 2.0] {
            for samples in [1, 4] {
                let size = CSS.map(|v| v * density as u32);
                eprintln!("START {name} DPR{density} samples{samples}");
                let production =
                    pollster::block_on(current_viewer::selftest::stroke_witness::render(
                        &bytes, matrix, size, density, samples, 1.0,
                    ))?;
                let stem = format!("{name}-dpr{}-samples{samples}", density as u32);
                let prod_file = format!("{stem}-production.ppm");
                let course_file = format!("{stem}-course.ppm");
                ppm(&output.join(&prod_file), &production.rgba, size)?;
                let shader_hash =
                    format!("{:x}", Sha256::digest(production.ribbon_source.as_bytes()));
                std::fs::write(
                    output.join(format!("{stem}-production.json")),
                    serde_json::to_vec_pretty(&json!({
                        "production": production.identity,
                        "compiled_ribbon_file": ribbon_file, "compiled_ribbon_sha256": shader_hash,
                    }))?,
                )?;
                if let Some(expected) = &ribbon_hash {
                    ensure!(
                        expected == &shader_hash,
                        "compiled production ribbon source changed"
                    );
                } else {
                    std::fs::write(output.join(ribbon_file), &production.ribbon_source)?;
                    ribbon_hash = Some(shader_hash);
                }
                ensure!(
                    viewer_journey::production_ribbon::SOURCE == production.ribbon_source,
                    "course ordinary-line shader differs from the compiled production shader"
                );
                let pen = &production.identity["uploaded_pen"];
                ensure!(
                    pen["radius"] == 0.0
                        && pen["thickness"] == density
                        && pen["packed_color"] == 0xff00_0000_u32,
                    "uploaded production pen differs from the fixture: {pen}"
                );
                let adapter = &production.identity["adapter"];
                ensure!(
                    production.identity["drawing_sheet"] == false,
                    "ordinary pen fixture became a drawing sheet"
                );
                ensure!(
                    adapter["device_type"] == "Cpu"
                        && ((adapter["backend"] == "Vulkan"
                            && adapter["name"]
                                .as_str()
                                .unwrap_or("")
                                .to_lowercase()
                                .contains("swiftshader"))
                            || (adapter["backend"] == "Dx12"
                                && adapter["name"] == "Microsoft Basic Render Driver")),
                    "witness requires a verified software adapter: {adapter}"
                );
                let scene = comparison::original_scene(&bytes).map_err(anyhow::Error::msg)?;
                eprintln!("stroke witness: opening course renderer");
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
                ensure!(
                    course.anchor() == [0.0; 3]
                        && production.identity["anchor"] == json!([0.0, 0.0, 0.0])
                );
                ensure!(course.samples() == samples && production.identity["samples"] == samples);
                ensure!(
                    production.identity["density"] == density
                        && production.identity["viewport"] == json!(size)
                );
                ensure!(production.identity["format"] == "Rgba8UnormSrgb");
                ensure!(production.identity["matrix_f32"] == json!(matrix));
                ensure!(course.stroke_usage()[0] == 1 && production.identity["ribbons"] == 1);
                ensure!(Background::default().rgb() == [1.0; 3]);
                eprintln!("stroke witness: rendering course frame");
                let course_pixels = pixels(&course, matrix, size)?;
                ensure!(production.rgba.len() == course_pixels.len());
                ppm(&output.join(&course_file), &course_pixels, size)?;
                let changed = production
                    .rgba
                    .chunks_exact(4)
                    .zip(course_pixels.chunks_exact(4))
                    .filter(|(a, b)| a != b)
                    .count();
                let max_delta = production
                    .rgba
                    .iter()
                    .zip(&course_pixels)
                    .map(|(a, b)| a.abs_diff(*b))
                    .max()
                    .unwrap_or(0);
                let production_ink = ink(&production.rgba);
                let course_ink = ink(&course_pixels);
                records.push(json!({
                    "specimen": name, "guid": guid, "ends_mm": ends,
                    "source_file": source_file, "source_sha256": format!("{:x}", Sha256::digest(&bytes)),
                    "source_width_css": 1.0, "source_colour": [0.0, 0.0, 0.0, 1.0],
                    "source_arrowhead": "NONE", "css_viewport": CSS,
                    "near_mm": 100.0, "far_mm": 5000.0, "depth": "reversed [0,1]",
                    "endpoint_clip": clips, "near_distances": near_distances,
                    "near_intersection": near_intersection,
                    "production": production.identity,
                    "course": {"shared_device": true, "samples": course.samples(), "anchor": course.anchor(),
                        "density_set": density, "viewport": size, "matrix_f32": matrix,
                        "ribbon_sha256": format!("{:x}", Sha256::digest(viewer_journey::production_ribbon::SOURCE.as_bytes())),
                        "format": "Rgba8UnormSrgb", "clear_linear": [1.0, 1.0, 1.0, 1.0]},
                    "images": {"production": prod_file, "course": course_file},
                    "diff": {"changed_pixels": changed, "max_rgba_delta": max_delta,
                        "production_ink": production_ink, "course_ink": course_ink},
                    "parity_accepted": false,
                }));
                std::fs::write(
                    output.join("manifest.json"),
                    serde_json::to_vec_pretty(&json!({
                        "purpose": "matched plain-stroke diagnosis; no pixel parity acceptance",
                        "production_ribbon": {"file": ribbon_file, "sha256": ribbon_hash},
                        "records": records,
                    }))?,
                )?;
                ensure!(
                    production_ink > 0 && course_ink > 0,
                    "blank witness: {stem}"
                );
                println!(
                    "{stem}: production ink {production_ink}, course ink {course_ink}, changed {changed}, max delta {max_delta}"
                );
            }
        }
    }
    Ok(())
}
