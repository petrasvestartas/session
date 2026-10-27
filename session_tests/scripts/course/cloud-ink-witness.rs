use anyhow::{Result, ensure};
use prost::Message;
use serde_json::json;
use session_rust::{Color, Line, Mesh, Point, PointCloud, Session, Xform};
use sha2::{Digest, Sha256};
use std::{io::Write, path::Path};
use viewer_journey::{background::Background, comparison, renderer::Renderer, viewport::Viewport};

const SIZE: [u32; 2] = [320, 240];

fn source(front: bool, triangle: bool) -> Vec<u8> {
    let mut document = Session::new("cloud-ink-witness");
    let depth = if front { 0.1 } else { 0.6 };
    let mut line = Line::new(-0.4, 0.2, depth, 0.6, 0.2, depth);
    line.width = 6.0;
    line.linecolor = Color::new(1.0, 0.0, 0.0, 1.0);
    document.add_line(line, None);
    if triangle {
        let mut mesh = Mesh::from_vertices_and_faces(
            vec![Point::new(-2.0, -2.0, 0.6), Point::new(2.0, -2.0, 0.6), Point::new(0.0, 2.0, 0.6)],
            vec![vec![0, 1, 2]],
        );
        mesh.set_objectcolor(Color::new(0.1, 0.6, 0.1, 1.0));
        document.add_mesh(mesh, None).unwrap();
    }
    let coords = (-3..=3).flat_map(|y| (-3..=3).flat_map(move |x| [0.3+x as f64*0.03, 0.2+y as f64*0.03, 0.2])).collect();
    let mut cloud = PointCloud::from_coords(coords, [0, 0, 255, 255].repeat(49), vec![]);
    cloud.point_size = 96.0;
    document.add_pointcloud(cloud, None).unwrap();
    document.to_proto().encode_to_vec()
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

fn at(bytes: &[u8]) -> [u8; 4] {
    bytes[(96 * SIZE[0] as usize + 208) * 4..][..4].try_into().unwrap()
}

fn main() -> Result<()> {
    let arguments: Vec<_> = std::env::args().collect();
    let output = arguments.get(1).ok_or_else(|| anyhow::anyhow!("supply evidence directory [--all] [--require-correct]"))?;
    let output = Path::new(output);
    std::fs::create_dir_all(output)?;
    let all = arguments.iter().any(|argument| argument == "--all");
    let require_correct = arguments.iter().any(|argument| argument == "--require-correct");
    let mut matrix = Xform::identity().m.map(|v| v as f32);
    for column in 0..4 { matrix[column*4+2] = matrix[column*4+3]-matrix[column*4+2]; }
    let mut rows = Vec::new();
    let mut correct = true;
    // The triangle-free control exposes the visibility fallback's dependency on mesh tiles.
    for (front, triangle) in [(false, true), (true, true), (false, false)] {
        let bytes = source(front, triangle);
        let source_sha = format!("{:x}", Sha256::digest(&bytes));
        let source_name = format!("source-front{front}-triangle{triangle}.pb");
        std::fs::write(output.join(&source_name), &bytes)?;
        for density in [1.0_f32, 2.0] {
            for samples in [1, 4] {
                if !all && (density != 1.0 || samples != 1) { continue; }
                let stem = format!("front{front}-triangle{triangle}-dpr{density}-samples{samples}");
                eprintln!("START {stem}");
                let production = pollster::block_on(current_viewer::selftest::stroke_witness::render(&bytes, matrix, SIZE, density, samples, 1.0))?;
                let pen = &production.identity["uploaded_pen"];
                ensure!(pen["radius"] == -3.0 && pen["thickness"] == density, "wrong actual production pen: {pen}");
                ensure!(production.identity["drawing_sheet"] == false, "source was promoted to a sheet");
                ensure!(production.identity["matrix_f32"] == json!(matrix));
                ensure!(production.identity["samples"] == samples && production.identity["density"] == density);
                ensure!(production.ribbon_source == viewer_journey::production_ribbon::SOURCE, "course ribbon extraction differs from actual production");
                std::fs::write(output.join("production-ribbon.wgsl"), &production.ribbon_source)?;
                let scene = comparison::original_scene(&bytes).map_err(anyhow::Error::msg)?;
                let mut course = Renderer::new(production.device, production.queue, wgpu::TextureFormat::Rgba8UnormSrgb, &scene);
                course.resize(Viewport {width: SIZE[0], height: SIZE[1]});
                course.set_density(density);
                course.set_samples(samples);
                course.set_lighting(false, false);
                course.cloud_witness(0.0).map_err(anyhow::Error::msg)?;
                ensure!(course.anchor() == [0.0; 3] && production.identity["anchor"] == json!([0.0, 0.0, 0.0]));
                let actual = pixels(&course, matrix, SIZE)?;
                let production_at = at(&production.rgba);
                let course_at = at(&actual);
                let expected = if front { [255, 0, 0, 255] } else { [0, 0, 255, 255] };
                correct &= production_at == expected && course_at == expected;
                let changed = production.rgba.chunks_exact(4).zip(actual.chunks_exact(4)).filter(|(a,b)| a != b).count();
                ppm(&output.join(format!("{stem}-production.ppm")), &production.rgba, SIZE)?;
                ppm(&output.join(format!("{stem}-course.ppm")), &actual, SIZE)?;
                eprintln!("{stem}: production={production_at:?}, course={course_at:?}, expected={expected:?}, changed={changed}");
                rows.push(json!({"name": stem, "source_file": source_name, "source_sha256": source_sha,
                    "front": front, "triangle": triangle, "production": production.identity,
                    "course": {"matrix_f32":matrix, "density":density, "samples":samples, "viewport":SIZE},
                    "sample_pixel":[208,96], "production_pixel":production_at, "course_pixel":course_at,
                    "expected_pixel":expected, "changed_pixels":changed,
                    "source_and_settings_guards":true}));
            }
        }
    }
    std::fs::write(output.join("manifest.json"), serde_json::to_vec_pretty(&json!({
        "purpose":"actual production cloud occlusion of original ordinary ribbon",
        "projection":"identical reversed identity; homogeneous clip.w=1 in both renderers",
        "correct":correct, "cases":rows
    }))?)?;
    ensure!(!require_correct || correct, "cloud ink occlusion is incorrect; evidence retained");
    Ok(())
}
