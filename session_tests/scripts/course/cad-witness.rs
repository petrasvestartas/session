use anyhow::{Result, ensure};
use prost::Message;
use serde_json::{Value, json};
use session_rust::{BRep, NurbsCurve, Point, Xform};
use sha2::{Digest, Sha256};
use std::{io::Write, path::Path};
use viewer_journey::{
    background::Background, renderer::Renderer, scene::Scene, viewport::Viewport,
};

// These two fixture constructors are copied unchanged from cad_wire_tests.rs.
fn add_wire(b: &mut BRep) -> usize {
    let points = [
        Point::new(-0.8, 0.7, -0.3),
        Point::new(0., 1., -0.3),
        Point::new(0.8, 0.7, -0.3),
    ];
    let curve = NurbsCurve::create(false, 2, &points);
    let a = b.add_vertex(&points[0], 0.);
    let z = b.add_vertex(&points[2], 0.);
    let c = b.add_curve_3d(&curve);
    let edge = b.add_edge(c as i32, a as i32, z as i32);
    b.add_wire(&[session_rust::brep::BRepRef::new(
        edge as i32,
        session_rust::brep::BRepOrientation::Forward,
    )]);
    edge
}
fn original(mixed: bool) -> BRep {
    let mut b = if mixed {
        BRep::create_block_with_hole(0.8, 0.6, 0.2, 0.12)
    } else {
        BRep::new()
    };
    b.set_guid(
        if mixed {
            "b6802f09-1be8-4cc6-a96c-bcae7869d902"
        } else {
            "b6802f09-1be8-4cc6-a96c-bcae7869d901"
        }
        .into(),
    );
    b.name = if mixed {
        "Original bore and detached wire"
    } else {
        "Original face-free wire"
    }
    .into();
    b.width = 4.;
    b.surfacecolor = session_rust::Color::new(0.35, 0.65, 0.85, 1.);
    add_wire(&mut b);
    b
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

fn specimen(name: &str) -> Result<(Vec<u8>, f64, f64, [f64; 3])> {
    let mut definitions = session_rust::proto::Objects::default();
    let (shift, distance, query) = match name {
        "wire" | "mixed" => {
            definitions.breps.push(original(name == "mixed").to_proto());
            (2.2, 6.0, [0.0, 0.85, -0.3])
        }
        "bore" => {
            let mut source = viewer_journey::cad_display::example();
            source.set_guid("b6802f09-1be8-4cc6-a96c-bcae7869d903".into());
            definitions.breps.push(source.to_proto());
            (6.0, 20.0, [1.5, 1.0, 0.3])
        }
        "hole-face" => {
            // The exact original perforated face used by cad_trim_tests.
            let mut source = BRep::create_block_with_hole(8.0, 6.0, 2.0, 1.0);
            source.m_faces = vec![source.m_faces[5].clone()];
            source.m_shells.clear();
            source.m_solids.clear();
            source.set_guid("b6802f09-1be8-4cc6-a96c-bcae7869d904".into());
            definitions.breps.push(source.to_proto());
            (12.0, 40.0, [3.0, 2.0, 1.0])
        }
        "surface" => {
            let mut source = viewer_journey::surface::example();
            source.set_guid("b6802f09-1be8-4cc6-a96c-bcae7869d905".into());
            // Exact original iso-curves from the canonical roof, saved as NurbsCurve sources.
            for direction in 0..2 {
                let (a, b) = source.domain(1 - direction).unwrap();
                let mut curve = source.iso_curve(direction, (a + b) * 0.5).unwrap();
                curve.set_guid(format!(
                    "b6802f09-1be8-4cc6-a96c-bcae7869d90{}",
                    direction + 6
                ));
                curve.width = 2.0;
                definitions.nurbscurves.push(curve.to_proto());
            }
            definitions.nurbssurfaces.push(source.to_proto());
            (3.0, 10.0, [0.5, 0.5, 0.2])
        }
        "surface-curved-border" => {
            let mut source = viewer_journey::surface::example();
            source.set_guid("b6802f09-1be8-4cc6-a96c-bcae7869d908".into());
            // Only this boundary control changes: the v-min border is genuinely curved in XY.
            ensure!(source.set_cv(1, 0, &Point::new(0.0, -1.7, 0.45)));
            let (u0, u1) = source.domain(0).unwrap();
            let (v0, _) = source.domain(1).unwrap();
            let a = source.point_at(u0, v0).unwrap();
            let b = source.point_at(u1, v0).unwrap();
            let middle = source.point_at((u0 + u1) * 0.5, v0).unwrap();
            ensure!((a[1] + 1.0).abs() < 1e-12 && (b[1] + 1.0).abs() < 1e-12);
            ensure!((middle[1] + 1.35).abs() < 1e-12 && (middle[2] - 0.225).abs() < 1e-12,
                "fixture must retain its non-straight original boundary");
            // No independent saved NurbsCurve: every visible edge belongs to the surface walker.
            definitions.nurbssurfaces.push(source.to_proto());
            (3.0, 10.0, [0.5, 0.5, 0.2])
        }
        _ => anyhow::bail!("case must be wire, mixed, bore, hole-face, surface, or surface-curved-border"),
    };
    let place = &Xform::translation(shift, 0.0, 0.0) * &Xform::scale_xyz(-1.0, 0.8, 1.3);
    let guids: Vec<_> = definitions
        .breps
        .iter()
        .map(|s| s.guid.clone())
        .chain(definitions.nurbssurfaces.iter().map(|s| s.guid.clone()))
        .chain(definitions.nurbscurves.iter().map(|s| s.guid.clone()))
        .collect();
    let instances = guids
        .iter()
        .enumerate()
        .flat_map(|(source, guid)| {
            [None, Some(place.to_proto())]
                .into_iter()
                .enumerate()
                .map(move |(copy, xform)| session_rust::proto::InstanceRef {
                    guid: format!("b6802f09-1be8-4cc6-a96c-{:012}", 911 + source * 2 + copy),
                    definition_guid: guid.clone(),
                    xform,
                    ..Default::default()
                })
        })
        .collect();
    let input = session_rust::proto::Session {
        objects: Some(session_rust::proto::Objects {
            instances,
            ..Default::default()
        }),
        definitions: Some(definitions),
        ..Default::default()
    };
    Ok((input.encode_to_vec(), shift, distance, query))
}

fn originals(scene: &Scene) -> Vec<Value> {
    let mut rows:Vec<Value> = scene.objects().iter().map(|row| {
        let (kind, guid, bytes) = if let Some(cad) = &row.cad {
            ("BRep", cad.source.guid(), cad.source.to_proto().encode_to_vec())
        } else {
            let source = &row.surface.as_ref().unwrap().source;
            ("NurbsSurface", source.guid(), source.to_proto().encode_to_vec())
        };
        json!({"kind":kind,"owner_guid":row.guid,"source_guid":guid,"model":row.model.m,"source_sha256":format!("{:x}",Sha256::digest(bytes))})
    }).collect();
    for row in scene.paths() {
        let viewer_journey::chain::Source::Curve(source, _) = &row.prepared.source else {
            panic!("original iso curve")
        };
        rows.push(json!({"kind":"NurbsCurve","owner_guid":row.guid,"source_guid":source.guid(),"model":row.model.m,"source_sha256":format!("{:x}",Sha256::digest(source.to_proto().encode_to_vec()))}));
    }
    rows.sort_by(|a, b| a["owner_guid"].as_str().cmp(&b["owner_guid"].as_str()));
    rows
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

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let output = Path::new(args.get(1).ok_or_else(||anyhow::anyhow!("cad-witness OUTPUT [mixed|wire|bore|hole-face|surface] [DPR] [samples] [all|no-lines|no-edges] [--require-parity] [--selected] [--source PATH]"))?);
    let name = args.get(2).map(String::as_str).unwrap_or("mixed");
    let density: f32 = args.get(3).map(String::as_str).unwrap_or("1").parse()?;
    let samples: u32 = args.get(4).map(String::as_str).unwrap_or("1").parse()?;
    let family = args.get(5).map(String::as_str).unwrap_or("all");
    let selected = args.iter().any(|arg| arg == "--selected");
    let mut source_override = None;
    for (index, arg) in args.iter().enumerate().skip(6) {
        if arg == "--source" {
            ensure!(source_override.is_none(), "--source may only be supplied once");
            let path = args.get(index + 1).filter(|path| !path.starts_with("--"))
                .ok_or_else(|| anyhow::anyhow!("--source requires a path"))?;
            source_override = Some(path);
        }
    }
    ensure!(matches!(density, 1.0 | 2.0) && matches!(samples, 1 | 4));
    ensure!(matches!(family, "all" | "no-lines" | "no-edges"));
    std::fs::create_dir_all(output)?;
    let (source, shift, distance, local_query) = if let Some(path) = source_override {
        ensure!(matches!(name, "wire" | "mixed"), "--source supports only wire or mixed");
        (std::fs::read(path)?, 2.2, 6.0, [0.0, 0.85, -0.3])
    } else {
        specimen(name)?
    };
    let bundle = if source_override.is_some() && source.starts_with(viewer_journey::archive::MAGIC) {
        let bundle = viewer_journey::archive::Bundle::decode(&source).map_err(anyhow::Error::msg)?;
        let metadata = &bundle.metadata;
        ensure!(bundle.documents.len() == 1 && metadata.documents.len() == 1,
            "--source archive must contain one original document");
        ensure!(metadata.documents[0].place == Xform::identity().m
            && metadata.documents[0].point_px == 0.0
            && metadata.created_doc.is_none() && metadata.current_layer.is_none()
            && metadata.hidden.is_empty() && metadata.locked.is_empty()
            && metadata.colors.is_empty() && metadata.edge_colors.as_ref().is_none_or(Vec::is_empty)
            && metadata.texts.is_empty() && metadata.groups.is_empty(),
            "--source archive has presentation overrides not represented by this witness");
        Some(bundle)
    } else {
        None
    };
    let production_source = bundle.as_ref().map_or(source.as_slice(), |bundle| bundle.documents[0].as_slice());
    let production_source_hash = format!("{:x}", Sha256::digest(production_source));
    let hash = format!("{:x}", Sha256::digest(&source));
    std::fs::write(output.join("source.pb"), &source)?;
    if source_override.is_some() {
        std::fs::write(output.join("production-source.pb"), production_source)?;
    }
    let mut scene = Scene::empty();
    if bundle.is_some() {
        viewer_journey::scene_document::open(&mut scene, &source).map_err(anyhow::Error::msg)?;
    } else {
        viewer_journey::scene_document::import(&mut scene, &source, "", &Xform::identity(), 0.0)
            .map_err(anyhow::Error::msg)?;
    }
    ensure!(scene.objects().len() == 2);
    if source_override.is_some() {
        let placed = &Xform::translation(2.2, 0.0, 0.0) * &Xform::scale_xyz(-1.0, 0.8, 1.3);
        for expected in [Xform::identity(), placed] {
            ensure!(scene.objects().iter().filter(|row| row.model.m == expected.m).count() == 1,
                "--source instance placements differ from the fixed browser wire/query fixture");
        }
        let guid = if name == "wire" { "b6802f09-1be8-4cc6-a96c-bcae7869d901" }
            else { "b6802f09-1be8-4cc6-a96c-bcae7869d902" };
        ensure!(scene.paths().is_empty() && scene.objects().iter().all(|row| row.cad.as_ref()
            .is_some_and(|cad| cad.source.guid() == guid && (cad.face_count == 0) == (name == "wire"))),
            "--source does not match the shared original wire/mixed fixture");
    }
    let selected_ids: Vec<_> = scene.objects().iter().filter(|_| selected).map(|row| row.id).collect();
    let selected_owners: Vec<_> = scene.objects().iter().filter(|_| selected)
        .map(|row| json!({"original_instance_guid":row.guid,"course_owner":format!("{:?}",row.id)}))
        .collect();
    let before = originals(&scene);
    let archive = viewer_journey::scene_archive::save(&scene).map_err(anyhow::Error::msg)?;
    std::fs::write(output.join("original.session"), &archive)?;
    let mut reopened = Scene::empty();
    viewer_journey::scene_document::open(&mut reopened, &archive).map_err(anyhow::Error::msg)?;
    ensure!(
        originals(&reopened) == before,
        "archive changed original CAD source, GUID or placement"
    );
    let mut camera = viewer_journey::camera::Camera::default();
    camera.set_view(viewer_journey::camera::View::Top);
    camera.target = [
        shift * 0.5,
        if matches!(name, "wire" | "mixed") {
            0.4
        } else {
            0.0
        },
        0.0,
    ];
    camera.distance = distance;
    camera.aspect = 4.0 / 3.0;
    camera.projection = viewer_journey::camera::Projection::Orthographic;
    let matrix = camera.uniform();
    let size = [320 * density as u32, 240 * density as u32];
    let families = current_viewer::selftest::stroke_witness::Families {
        lines: family != "no-lines",
        mesh_edges: family != "no-edges",
        selected_guids: scene.objects().iter().filter(|_| selected).map(|row| row.guid.clone()).collect(),
        ..Default::default()
    };
    let production =
        pollster::block_on(current_viewer::selftest::stroke_witness::render_families(
            production_source,
            matrix,
            size,
            density,
            samples,
            1.0,
            &[],
            &families,
        ))?;
    ensure!(
        production.identity["adapter"]["backend"] == "Dx12"
            && production.identity["adapter"]["device_type"] == "Cpu"
            && production.identity["adapter"]["name"] == "Microsoft Basic Render Driver"
    );
    ensure!(
        production.identity["matrix_f32"] == json!(matrix)
            && production.identity["viewport"] == json!(size)
            && production.identity["samples"] == samples
            && production.identity["density"] == density
    );
    ensure!(
        production.identity["format"] == "Rgba8UnormSrgb"
            && production.identity["anchor"] == json!([0.0, 0.0, 0.0])
    );
    if selected {
        ensure!(production.identity["selected_guids"] == json!(families.selected_guids));
    }
    std::fs::write(
        output.join("production-ribbon.wgsl"),
        &production.ribbon_source,
    )?;
    std::fs::write(
        output.join("production-vector.wgsl"),
        &production.vector_source,
    )?;
    std::fs::write(
        output.join("production.json"),
        serde_json::to_vec_pretty(&json!({
            "identity":production.identity,
            "ribbon_sha256":format!("{:x}",Sha256::digest(production.ribbon_source.as_bytes())),
            "vector_sha256":format!("{:x}",Sha256::digest(production.vector_source.as_bytes())),
            "source_sha256":hash,
            "source_override":source_override,"production_source_sha256":production_source_hash,
        }))?,
    )?;
    let walked = production.identity["walked_ink"]["objects"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("production source identity missing"))?;
    for row in &before {
        ensure!(
            walked
                .iter()
                .any(|native| native["guid"] == row["owner_guid"]
                    && native["placement_f64"] == row["model"]),
            "production source GUID/placement differs: {row}"
        );
    }
    let mut renderer = Renderer::new(
        production.device,
        production.queue,
        wgpu::TextureFormat::Rgba8UnormSrgb,
        &scene,
    );
    renderer.resize(Viewport {
        width: size[0],
        height: size[1],
    });
    renderer.set_density(density);
    renderer.set_samples(samples);
    renderer.set_lighting(false, false);
    renderer.set_ambient(false);
    renderer.set_opacity(1.0);
    renderer.set_outlines(false);
    renderer.set_ink_visibility(viewer_journey::ink_ranges::Visibility {
        edges: families.mesh_edges,
        lines: families.lines,
        points: false,
        vertices: false,
    });
    renderer.cloud_camera(&camera);
    if selected {
        renderer.select_objects(&scene, &selected_ids).map_err(anyhow::Error::msg)?;
    }
    ensure!(
        renderer.anchor() == [0.0; 3]
            && renderer.samples() == samples
            && Background::default().rgb() == [1.0; 3]
    );
    let actual = pixels(&renderer, matrix, size)?;
    ppm(&output.join("production.ppm"), &production.rgba, size)?;
    ppm(&output.join("course.ppm"), &actual, size)?;
    if selected {
        std::fs::write(output.join("production.rgba"), &production.rgba)?;
        std::fs::write(output.join("course.rgba"), &actual)?;
    }
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
    std::fs::write(
        output.join("pixel-baseline.json"),
        serde_json::to_vec_pretty(&json!({
            "case":name,"family":family,"selected":selected,"selected_owners":selected_owners,"source_sha256":hash,"source_override":source_override,"production_source_sha256":production_source_hash,"matrix":matrix,
            "changed_pixels":changed,"max_rgba_delta":max_delta,
            "production_ink":ink(&production.rgba),"course_ink":ink(&actual),"picks_pending":true,
        }))?,
    )?;
    let mut picks = Vec::new();
    for row in scene.objects() {
        let p =
            row.model
                .transform_point(&Point::new(local_query[0], local_query[1], local_query[2]));
        let world = [p[0], p[1], p[2]];
        let first = query(
            &mut renderer,
            &scene,
            matrix,
            world,
            viewer_journey::pick_sources::Mode::Object,
        )?;
        let repeat = query(
            &mut renderer,
            &scene,
            matrix,
            world,
            viewer_journey::pick_sources::Mode::Object,
        )?;
        ensure!(first == repeat, "fresh and cached object answers differ");
        let expected = if matches!(name, "wire" | "mixed") && !families.lines {
            None
        } else {
            Some(viewer_journey::pick_sources::Target {
                owner: row.id,
                part: viewer_journey::pick_sources::Part::Object,
            })
        };
        ensure!(
            first == expected,
            "original source owner query changed: {first:?} vs {expected:?}"
        );
        picks.push(json!({"owner_guid":row.guid,"world":world,"actual":format!("{first:?}"),"fresh_cached_equal":true}));
    }
    ensure!(originals(&scene) == before && format!("{:x}", Sha256::digest(&source)) == hash);
    if selected {
        ensure!(viewer_journey::scene_archive::save(&scene).map_err(anyhow::Error::msg)? == archive,
            "selection or queries changed the original archive");
    }
    let record = json!({"case":name,"family":family,"selected":selected,"selected_owners":selected_owners,"source_sha256":hash,"source_file":"source.pb","source_override":source_override,"source_is_archive":bundle.is_some(),"production_source_sha256":production_source_hash,"archive":"original.session","originals":before,"archive_roundtrip":true,"course_picks":picks,
        "production":production.identity,"course":{"shared_device":true,"matrix_f32":matrix,"viewport":size,"density":density,"samples":renderer.samples(),"format":"Rgba8UnormSrgb","anchor":renderer.anchor()},
        "diff":{"changed_pixels":changed,"max_rgba_delta":max_delta,"production_ink":ink(&production.rgba),"course_ink":ink(&actual)},
        "selection_scope":"When requested, both original BRep/Surface instance owners are selected; separate NurbsCurve isoline instances remain unselected. Selected runs additionally save tightly packed production.rgba and course.rgba in viewport row order.",
        "scope":"Actual original source rendering comparison; production picks and control-point checks are not exercised. Course picks check Object mode only. Surface isolines are saved exact NurbsCurve originals from the canonical roof, not an automatic display mode.","parity_accepted":changed==0});
    std::fs::write(
        output.join("manifest.json"),
        serde_json::to_vec_pretty(&record)?,
    )?;
    println!(
        "{name}/{family} DPR{density} samples{samples}: changed {changed}, max delta {max_delta}"
    );
    if args.iter().any(|arg| arg == "--require-parity") {
        ensure!(changed == 0, "actual production/course CAD pixels differ");
    }
    Ok(())
}
