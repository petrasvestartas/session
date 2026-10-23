use anyhow::{Result, ensure};
use prost::Message;
use serde_json::json;
use session_rust::{Arrowhead, Color, Line, Plane, Point, Session, Vector, Xform};
use sha2::{Digest, Sha256};
use std::{io::Write, path::Path};
use viewer_journey::{background::Background, comparison, renderer::Renderer, viewport::Viewport};

const SIZE: [u32; 2] = [320, 240];
const ENDS: [[f64; 3]; 2] = [[-0.5, 0.2, -0.5], [0.5, 0.2, -0.5]];

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

fn specimen() -> Vec<u8> {
    let mut document = Session::new("coplanar ordinary section line");
    document.set_guid("93ff6870-5dd7-4000-8000-000000000001".into());
    document
        .objects
        .set_guid("93ff6870-5dd7-4000-8000-000000000002".into());
    document
        .tree
        .set_guid("93ff6870-5dd7-4000-8000-000000000003".into());
    document
        .graph
        .set_guid("93ff6870-5dd7-4000-8000-000000000004".into());
    let [a, b] = ENDS;
    let mut line = Line::new(a[0], a[1], a[2], b[0], b[1], b[2]);
    line.set_guid("93ff6870-5dd7-4000-8000-000000000101".into());
    line.name = "coplanar green line".into();
    line.width = 10.0;
    line.linecolor = Color::new(0.0, 1.0, 0.0, 1.0);
    line.arrowhead = Arrowhead::NONE;
    document.add_line(line, None);
    let mut point = Point::new(0.75, -0.7, -1.5);
    point.set_guid("93ff6870-5dd7-4000-8000-000000000102".into());
    point.name = "off-plane source extent; markers disabled".into();
    document.add_point(point, None);
    let mut plane = Plane::from_frame(
        Point::new(0.0, 0.0, -0.5),
        Vector::new(0.9, 0.0, 0.0),
        Vector::new(0.0, 0.9, 0.0),
        Vector::new(0.0, 0.0, 1.0),
    );
    plane.set_guid("93ff6870-5dd7-4000-8000-000000000103".into());
    plane.name = "Clipping Plane".into();
    plane.linecolor = Color::black();
    document.add_plane(plane, None);
    for (i, node) in document.tree.nodes().iter().enumerate() {
        node.borrow_mut()
            .set_guid(format!("93ff6870-5dd7-4000-9000-{i:012}"));
    }
    document.to_proto().encode_to_vec()
}

fn green_counts(rgba: &[u8]) -> (usize, usize) {
    let green = |pixel: &[u8]| pixel[1] > 160 && pixel[0] < 80 && pixel[2] < 80;
    let total = rgba.chunks_exact(4).filter(|pixel| green(pixel)).count();
    // The source clipping plane has a green Y axis in production. Exclude that axis from line evidence.
    let line_roi = rgba
        .chunks_exact(4)
        .enumerate()
        .filter(|(index, pixel)| {
            let (x, y) = (index % SIZE[0] as usize, index / SIZE[0] as usize);
            (72..248).contains(&x)
                && !(144..176).contains(&x)
                && (88..104).contains(&y)
                && green(pixel)
        })
        .count();
    (total, line_roi)
}

fn main() -> Result<()> {
    log::set_logger(&Logger).map_err(|error| anyhow::anyhow!("{error}"))?;
    log::set_max_level(log::LevelFilter::Info);
    let output = std::env::args_os()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("supply evidence directory"))?;
    let output = Path::new(&output);
    std::fs::create_dir_all(output)?;
    let bytes = specimen();
    std::fs::write(output.join("section.pb"), &bytes)?;
    let scene = comparison::original_scene(&bytes).map_err(anyhow::Error::msg)?;
    let rows = viewer_journey::clipping_rows::current(&scene).map_err(anyhow::Error::msg)?;
    ensure!(rows.len() == 1);
    let plane = rows[0].plane;
    ensure!(plane.normal == [0.0, 0.0, -1.0] && plane.offset == -0.5);
    let distances = ENDS.map(|point| plane.distance(point));
    ensure!(distances == [0.0; 2]);
    let native_plane = current_viewer::selftest::stroke_witness::ClipPlane {
        normal: plane.normal,
        offset: plane.offset,
        origin: plane.origin,
        hatch: plane.hatch,
    };
    let matrix = Xform::orthographic(-1.0, 1.0, -1.0, 1.0, 1.0, 0.0).to_f32();
    let mut records = Vec::new();
    for clipped in [false, true] {
        let name = if clipped { "clipped" } else { "unclipped" };
        let planes: &[_] = if clipped {
            std::slice::from_ref(&native_plane)
        } else {
            &[]
        };
        let production =
            pollster::block_on(current_viewer::selftest::stroke_witness::render_clipped(
                &bytes, matrix, SIZE, 1.0, 1, 1.0, planes,
            ))?;
        ppm(
            &output.join(format!("{name}-production.ppm")),
            &production.rgba,
            SIZE,
        )?;
        std::fs::write(
            output.join(format!("{name}-production.json")),
            serde_json::to_vec_pretty(&production.identity)?,
        )?;
        if !clipped {
            std::fs::write(
                output.join("production-ribbon-compiled.wgsl"),
                &production.ribbon_source,
            )?;
        }
        let actual_clip = &production.identity["uploaded_clip"];
        ensure!(actual_clip["count"] == u32::from(clipped));
        let mut uploaded_distances = None;
        if clipped {
            ensure!(actual_clip["planes"] == json!([[0.0, 0.0, -1.0, -0.5]]));
            let coefficients = actual_clip["planes"][0].as_array().unwrap();
            let actual_distances = ENDS.map(|point| {
                (0..3)
                    .map(|axis| coefficients[axis].as_f64().unwrap() * point[axis])
                    .sum::<f64>()
                    + coefficients[3].as_f64().unwrap()
            });
            ensure!(actual_distances == [0.0; 2]);
            uploaded_distances = Some(actual_distances);
        }
        ensure!(production.identity["drawing_sheet"] == false);
        ensure!(production.identity["ribbons"] == 8);
        ensure!(production.identity["uploaded_pen"]["radius"] == -5.0);
        ensure!(production.identity["uploaded_pen"]["packed_color"] == 0xff00_ff00_u32);
        ensure!(viewer_journey::production_ribbon::SOURCE == production.ribbon_source);
        let mut course = Renderer::new(
            production.device,
            production.queue,
            wgpu::TextureFormat::Rgba8UnormSrgb,
            &scene,
        );
        course.resize(Viewport {
            width: SIZE[0],
            height: SIZE[1],
        });
        course.set_density(1.0);
        course.set_samples(1);
        course.set_opacity(1.0);
        course.set_clipping(clipped, true);
        course.set_ink_visibility(viewer_journey::ink_ranges::Visibility {
            edges: false,
            lines: true,
            points: false,
            vertices: false,
        });
        ensure!(course.samples() == 1 && course.anchor() == [0.0; 3]);
        let course_pixels = pixels(&course, matrix, SIZE)?;
        ppm(
            &output.join(format!("{name}-course.ppm")),
            &course_pixels,
            SIZE,
        )?;
        let (production_green, production_line_green) = green_counts(&production.rgba);
        let (course_green, course_line_green) = green_counts(&course_pixels);
        records.push(json!({
            "clipped": clipped, "production": production.identity,
            "uploaded_endpoint_signed_distances": uploaded_distances,
            "production_green": production_green, "production_line_roi_green": production_line_green,
            "course_green": course_green, "course_line_roi_green": course_line_green,
            "course": {"shared_device": true, "matrix_f32": matrix, "viewport": SIZE, "density": 1.0, "samples": 1,
                "format": "Rgba8UnormSrgb", "clear_linear": [1.0, 1.0, 1.0, 1.0], "markers": false,
                "plane": {"normal": plane.normal, "offset": plane.offset, "origin": plane.origin, "hatch": plane.hatch}},
        }));
        std::fs::write(
            output.join("manifest.json"),
            serde_json::to_vec_pretty(&json!({
                "purpose": "production coplanar ordinary-line section diagnosis; no parity acceptance",
                "source_file": "section.pb", "source_sha256": format!("{:x}", Sha256::digest(&bytes)),
                "line_guid": "93ff6870-5dd7-4000-8000-000000000101", "line_ends": ENDS, "line_width_css": 10.0,
                "plane_equation": [0.0, 0.0, -1.0, -0.5], "endpoint_signed_distances": distances,
                "line_roi": {"x": [72, 248], "exclude_x": [144, 176], "y": [88, 104], "reason": "Exclude the production plane green Y axis"},
                "compiled_ribbon_sha256": format!("{:x}", Sha256::digest(viewer_journey::production_ribbon::SOURCE.as_bytes())),
                "records": records,
            }))?,
        )?;
        if !clipped {
            ensure!(
                production_line_green > 0 && course_line_green > 0,
                "unclipped original line must be visible in both renders"
            );
        }
        println!(
            "{name}: production line ROI green={production_line_green}, course line ROI green={course_line_green}; global green={production_green}/{course_green}"
        );
    }
    Ok(())
}
