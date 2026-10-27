use anyhow::{Result, ensure};
use prost::Message;
use serde_json::{Value, json};
use session_rust::{Arrowhead, Color, Line, Point, Polyline, Session, Xform};
use sha2::{Digest, Sha256};
use std::{io::Write, path::Path};
use viewer_journey::{background::Background, renderer::Renderer, viewport::Viewport};
const CSS: [u32; 2] = [320, 240];
const OBJECT: &str = "6d0aaee0-0430-4000-8000-000000000501";
const GUARD: &str = "6d0aaee0-0430-4000-8000-000000000502";
fn digest(bytes: impl AsRef<[u8]>) -> String {
    format!("{:x}", Sha256::digest(bytes.as_ref()))
}
struct Specimen {
    name: &'static str,
    bytes: Vec<u8>,
    heads: usize,
    ribbons: usize,
    description: Value,
}
fn specimen(name: &'static str) -> Specimen {
    let mut session = Session::new(name);
    session.set_guid("6d0aaee0-0430-4000-8000-000000000001".into());
    session
        .objects
        .set_guid("6d0aaee0-0430-4000-8000-000000000002".into());
    session
        .tree
        .set_guid("6d0aaee0-0430-4000-8000-000000000003".into());
    session
        .graph
        .set_guid("6d0aaee0-0430-4000-8000-000000000004".into());
    let closed = name.starts_with("closed");
    let degenerate = name == "degenerate-line";
    let heads = if name == "closed-headless" || degenerate {
        0
    } else {
        2
    };
    let arrow = if name == "closed-headless" {
        Arrowhead::NONE
    } else {
        Arrowhead::BOTH
    };
    let mut points = vec![
        [-750.0, -350.0, -2000.0],
        [200.0, -350.0, -2001.0],
        [350.0, 450.0, -2002.0],
    ];
    if closed {
        points.push(points[0]);
    } else {
        points.push(*points.last().unwrap());
    }
    let line = name == "line-both" || degenerate;
    if line {
        points = vec![[-750.0, -350.0, -2000.0], [650.0, 400.0, -2001.0]];
        if degenerate {
            points[1] = points[0];
        }
        let [a, b] = [points[0], points[1]];
        let mut source = Line::new(a[0], a[1], a[2], b[0], b[1], b[2]);
        source.set_guid(OBJECT.into());
        source.name = name.into();
        source.width = 3.0;
        source.linecolor = Color::black();
        source.arrowhead = arrow;
        session.add_line(source, None);
    } else {
        let mut source = Polyline::new(
            points
                .iter()
                .map(|p| Point::new(p[0], p[1], p[2]))
                .collect(),
        );
        source.set_guid(OBJECT.into());
        source.name = name.into();
        source.width = 3.0;
        source.linecolor = Color::black();
        source.arrowhead = arrow;
        session.add_polyline(source, None).unwrap();
    }
    // This real offscreen source makes even a zero-length specimen a spatial file;
    // production's automatic planar-file 1mm-pen rule must not change the fixture.
    let mut guard = Line::new(10000.0, 10000.0, -1900.0, 10100.0, 10000.0, -2100.0);
    guard.set_guid(GUARD.into());
    guard.name = "offscreen spatial-classification guard".into();
    session.add_line(guard, None);
    let model = if name == "placed-polyline" {
        &(&Xform::translation(80.0, 30.0, 0.0) * &Xform::rotation_z(23.0, true))
            * &Xform::scale_xyz(-0.8, 1.2, 1.0)
    } else {
        Xform::identity()
    };
    session.set_xform(OBJECT, model.clone());
    for (i, node) in session.tree.nodes().iter().enumerate() {
        node.borrow_mut()
            .set_guid(format!("6d0aaee0-0430-4000-9000-{i:012}"));
    }
    let ribbons = 1 + if line {
        1
    } else if closed {
        3
    } else {
        2
    };
    Specimen {
        name,
        bytes: session.to_proto().encode_to_vec(),
        heads,
        ribbons,
        description: json!({"guid":OBJECT,"name":name,"family":if line{"Line"}else{"Polyline"},"coordinates_mm":points,"width_css":3.0,"rgba":[0.0,0.0,0.0,1.0],"arrowhead":format!("{arrow:?}"),"placement_f64":model.m,"offscreen_guard_guid":GUARD,"expected_head_rows":heads,"expected_ribbon_rows_including_guard":ribbons}),
    }
}
fn restored_shader(source: &str, provenance: &Value) -> Result<String> {
    ensure!(
        digest(source) == provenance["compiled_sha256"].as_str().unwrap(),
        "course source snapshot hash changed"
    );
    let suffix = format!(
        "\n{}\n",
        provenance["ownership_declaration"].as_str().unwrap()
    );
    let mut original = source
        .strip_suffix(&suffix)
        .ok_or_else(|| anyhow::anyhow!("missing ownership declaration"))?
        .to_owned();
    for edit in provenance["ownership_edits"].as_array().unwrap() {
        let before = edit[0].as_str().unwrap();
        let after = edit[1].as_str().unwrap();
        ensure!(original.contains(after));
        original = original.replacen(after, before, 1);
    }
    ensure!(digest(&original) == provenance["pristine_sha256"].as_str().unwrap());
    Ok(original)
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
    let args = std::env::args_os().collect::<Vec<_>>();
    ensure!(
        args.len() >= 3,
        "usage: vector-witness OUTPUT COURSE_SRC [SPECIMEN]; VECTOR_WITNESS_SELECTED=1 selects the specimen"
    );
    let output = Path::new(&args[1]);
    let course_source = Path::new(&args[2]);
    std::fs::create_dir_all(output)?;
    let filter = args.get(3).map(|v| v.to_string_lossy().into_owned());
    let selected = std::env::var("VECTOR_WITNESS_SELECTED").is_ok_and(|v| v == "1");
    let matrix = Xform::perspective(std::f64::consts::PI / 3.0, 4.0 / 3.0, 5000.0, 100.0).to_f32();
    let provenance: Value = serde_json::from_slice(&std::fs::read(
        course_source.join("production_vector_provenance.json"),
    )?)?;
    let joined = std::fs::read_to_string(course_source.join("production_joined.wgsl"))?;
    let vector = std::fs::read_to_string(course_source.join("production_vector.wgsl"))?;
    let restored_joined = restored_shader(&joined, &provenance["joined"])?;
    let restored_vector = restored_shader(&vector, &provenance["vector"])?;
    std::fs::write(output.join("course-joined-source-snapshot.wgsl"), &joined)?;
    std::fs::write(output.join("course-vector-source-snapshot.wgsl"), &vector)?;
    let binary_hash = digest(std::fs::read(std::env::current_exe()?)?);
    let mut records = Vec::new();
    let mut failures = Vec::new();
    for name in [
        "line-both",
        "polyline-both",
        "closed-headless",
        "closed-both",
        "degenerate-line",
        "placed-polyline",
    ] {
        if filter.as_ref().is_some_and(|filter| filter != name) {
            continue;
        }
        let specimen = specimen(name);
        let source_file = format!("{name}.pb");
        std::fs::write(output.join(&source_file), &specimen.bytes)?;
        for density in [1.0_f32, 2.0] {
            for samples in [1, 4] {
                let size = CSS.map(|v| v * density as u32);
                let stem = format!(
                    "{name}-dpr{}-samples{samples}{}",
                    density as u32,
                    if selected { "-selected" } else { "" }
                );
                eprintln!("START {stem}");
                let families = current_viewer::selftest::stroke_witness::Families {
                    selected_guids: if selected {
                        vec![OBJECT.into()]
                    } else {
                        Vec::new()
                    },
                    ..Default::default()
                };
                let production =
                    pollster::block_on(current_viewer::selftest::stroke_witness::render_families(
                        &specimen.bytes,
                        matrix,
                        size,
                        density,
                        samples,
                        1.0,
                        &[],
                        &families,
                    ))?;
                ensure!(
                    production.ribbon_source == restored_joined,
                    "course ribbon geometry is not the actual production closure"
                );
                ensure!(
                    production.vector_source == restored_vector,
                    "course vector geometry is not the actual production closure"
                );
                std::fs::write(
                    output.join("production-ribbon-compiled.wgsl"),
                    &production.ribbon_source,
                )?;
                std::fs::write(
                    output.join("production-vector-compiled.wgsl"),
                    &production.vector_source,
                )?;
                let walked = &production.identity["walked_ink"];
                let objects = walked["objects"].as_array().unwrap();
                let object = objects
                    .iter()
                    .find(|object| object["guid"] == OBJECT)
                    .ok_or_else(|| anyhow::anyhow!("production lost original GUID"))?;
                ensure!(
                    object["placement_f64"] == specimen.description["placement_f64"],
                    "production source placement changed"
                );
                let row = object["row"].as_u64().unwrap();
                ensure!(
                    walked["vectors"].as_array().unwrap().len() == specimen.heads,
                    "unexpected actual production head rows"
                );
                ensure!(
                    walked["ribbons"].as_array().unwrap().len() == specimen.ribbons,
                    "unexpected actual production ribbon rows"
                );
                for ribbon in walked["ribbons"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|r| r["instance"] == row)
                {
                    ensure!(
                        ribbon["radius"] == -1.5 && ribbon["color"] == 0xff00_0000u32,
                        "source pen changed in actual production walk"
                    );
                }
                ensure!(
                    production.identity["drawing_sheet"] == false,
                    "spatial fixture was classified as a planar drawing"
                );
                ensure!(production.identity["uploaded_pen"]["thickness"] == density);
                ensure!(
                    production.identity["viewport"] == json!(size)
                        && production.identity["samples"] == samples
                        && production.identity["density"] == density
                );
                ensure!(
                    production.identity["format"] == "Rgba8UnormSrgb"
                        && production.identity["matrix_f32"] == json!(matrix)
                );
                ensure!(production.identity["anchor"] == json!([0.0, 0.0, 0.0]));
                let mut scene = viewer_journey::scene::Scene::empty();
                viewer_journey::scene_document::import(
                    &mut scene,
                    &specimen.bytes,
                    name,
                    &Xform::identity(),
                    0.0,
                )
                .map_err(anyhow::Error::msg)?;
                let origin = scene
                    .forest
                    .documents
                    .last()
                    .and_then(|doc| doc.header.as_ref())
                    .ok_or_else(|| anyhow::anyhow!("course lost loaded source origin"))?;
                ensure!(origin.version.hex() == digest(&specimen.bytes));
                let loaded = scene
                    .lines()
                    .iter()
                    .map(|r| (r.id, r.guid.clone(), r.model.m))
                    .chain(
                        scene
                            .paths()
                            .iter()
                            .map(|r| (r.id, r.guid.clone(), r.model.m)),
                    )
                    .collect::<Vec<_>>();
                ensure!(loaded.len() == 2 && loaded.iter().any(|(_, guid, _)| guid == GUARD));
                let (id, _, model) = loaded
                    .iter()
                    .find(|(_, guid, _)| guid == OBJECT)
                    .ok_or_else(|| anyhow::anyhow!("course lost original GUID"))?;
                ensure!(
                    json!(model) == specimen.description["placement_f64"],
                    "course source placement changed"
                );
                let loaded_json:Vec<_>=loaded.iter().map(|(id,guid,model)|json!({"id":id.value(),"guid":guid,"placement_f64":model})).collect();
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
                if selected {
                    course
                        .select_objects(&scene, &[*id])
                        .map_err(anyhow::Error::msg)?;
                }
                ensure!(
                    course.anchor() == [0.0; 3]
                        && course.samples() == samples
                        && Background::default().rgb() == [1.0; 3]
                );
                let actual = pixels(&course, matrix, size)?;
                ensure!(production.rgba.len() == actual.len());
                let prod_file = format!("{stem}-production.ppm");
                let course_file = format!("{stem}-course.ppm");
                ppm(&output.join(&prod_file), &production.rgba, size)?;
                ppm(&output.join(&course_file), &actual, size)?;
                let changed = production
                    .rgba
                    .chunks_exact(4)
                    .zip(actual.chunks_exact(4))
                    .filter(|(a, b)| a != b)
                    .count();
                let max_delta = production
                    .rgba
                    .iter()
                    .zip(&actual)
                    .map(|(a, b)| a.abs_diff(*b))
                    .max()
                    .unwrap_or(0);
                let production_ink = ink(&production.rgba);
                let course_ink = ink(&actual);
                if name != "degenerate-line" {
                    ensure!(
                        production_ink > 0 && course_ink > 0,
                        "blank visible specimen"
                    );
                }
                let equal = changed == 0;
                if !equal {
                    failures.push(stem.clone());
                }
                let record = json!({"case":stem,"source":specimen.description,"source_file":source_file,"source_sha256":digest(&specimen.bytes),"selected":selected,"css_viewport":CSS,
                "production":production.identity,"production_ribbon_sha256":digest(&production.ribbon_source),"production_vector_sha256":digest(&production.vector_source),
                "course":{"shared_device":true,"loaded_origin_sha256":origin.version.hex(),"loaded_objects":loaded_json,"samples":samples,"density":density,"viewport":size,"matrix_f32":matrix,"anchor":course.anchor(),"format":"Rgba8UnormSrgb","clear_linear":[1.0,1.0,1.0,1.0],"ribbon_rows":course.stroke_usage()[0]+course.path_usage()[0]},
                "images":{"production":prod_file,"course":course_file},"diff":{"changed_pixels":changed,"max_rgba_delta":max_delta,"production_ink":production_ink,"course_ink":course_ink},"whole_frame_exact":equal});
                std::fs::write(
                    output.join(format!("{stem}.json")),
                    serde_json::to_vec_pretty(&record)?,
                )?;
                records.push(record);
                std::fs::write(
                    output.join("manifest.json"),
                    serde_json::to_vec_pretty(
                        &json!({"purpose":"paired actual production and course vector/joined ink; exact whole-frame parity required","binary_sha256":binary_hash,"course_source_snapshot":{"path":course_source,"joined_sha256":digest(&joined),"vector_sha256":digest(&vector),"note":"Snapshot hashes identify audited files; the paired pixel comparison exercises the linked course renderer."},"sheet_gap":"Not exercised: production sheets require streamed SheetInit/SheetFields ingress; a Session::pb_loads file route does not establish DrawingSet equivalence.","records":records,"failures":failures}),
                    )?,
                )?;
                println!(
                    "{stem}: changed {changed}, max delta {max_delta}, production ink {production_ink}, course ink {course_ink}"
                );
            }
        }
    }
    ensure!(!records.is_empty(), "no matching vector specimen");
    ensure!(
        failures.is_empty(),
        "exact whole-frame vector parity failed: {}",
        failures.join(", ")
    );
    Ok(())
}
