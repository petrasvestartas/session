//! Check that resize, rebuild and reload draw the same pixels and ids.

use crate::{
    app::scene::{FileDoc, Scene},
    camera::{Camera, View},
    engine::gpu::{FrameInput, Gpu, Pick},
};
use session_rust::{Color, Line, Point, Session, Xform};
use std::{collections::HashSet, io::Write, path::Path, rc::Rc};

type Source = (String, Rc<Session>, Xform); // name, session, placement

/// One rendered frame: the picture and the id picture.
#[derive(PartialEq, Eq)]
struct Frame {
    color: Vec<u8>, // RGBA pixels
    ids: Vec<[u32; 2]>, // object and segment id per pixel
}

/// Load the sources, uploading after each one or once at the end.
fn load(scene: &mut Scene, gpu: &mut Gpu, sources: &[Source], incremental: bool) {
    for (name, session, place) in sources {
        scene.add_file(FileDoc {
            name: name.clone(),
            session: Rc::clone(session),
            place: place.clone(),
            point_px: 0.0,
            display_only: false,
        });
        if incremental {
            scene.upload_to(gpu);
        }
    }
    if !incremental {
        scene.upload_to(gpu);
    }
}

/// Draw one frame and check every id in it resolves.
fn render(gpu: &mut Gpu, scene: &Scene, camera: &Camera) -> Frame {
    let anchor = gpu
        .rebase_anchor(&camera.origin(), camera.distance_world(), 0.0)
        .anchor;
    let input = FrameInput {
        view_proj: camera.view_proj_anchored(4.0 / 3.0, &anchor),
        clear: wgpu::Color {
            r: 0.9,
            g: 0.9,
            b: 0.9,
            a: 1.0,
        },
        now_ms: 0.0,
    };
    let color = gpu.render_offscreen(&input);
    let ids = gpu.render_ids_offscreen(&input);
    assert!(
        ids.iter().any(|id| id[0] != 0),
        "scene must produce pickable geometry"
    );
    // every distinct id drawn
    for [object, sub] in ids
        .iter()
        .copied()
        .filter(|id| id[0] != 0)
        .collect::<HashSet<_>>()
    {
        let row = object - 1; // ids are row + 1, 0 is background
        let decoded = sub.saturating_sub(1);
        let hit = scene
            .resolve(Pick { row, sub: decoded }, gpu)
            .expect("rendered object ID resolves");
        assert_eq!(hit.row, row);
        if sub & 0x8000_0000 != 0
            && let Some(range) = scene.ribbon_range(row)
        {
            assert!(
                range.contains(&((sub & 0x7fff_ffff) - 1)),
                "ribbon ID must stay inside its retained object range"
            );
        }
    }
    Frame { color, ids }
}

/// Assert two frames are pixel-identical.
fn same(label: &str, reference: &Frame, current: &Frame) {
    assert_eq!(
        (reference.color.len(), reference.ids.len()),
        (current.color.len(), current.ids.len()),
        "{label}: frame dimensions"
    );
    let color = reference
        .color
        .chunks_exact(4)
        .zip(current.color.chunks_exact(4))
        .filter(|(a, b)| a != b)
        .count();
    let ids = reference
        .ids
        .iter()
        .zip(&current.ids)
        .filter(|(a, b)| a != b)
        .count();
    assert_eq!(
        (color, ids),
        (0, 0),
        "{label}: spatial pixel/ID differences"
    );
    println!("{label}: identical full color and picking frames");
}

/// Write a frame as a PPM image.
fn write_frame(path: &Path, frame: &Frame) {
    let mut file = std::io::BufWriter::new(std::fs::File::create(path).unwrap());
    write!(file, "P6\n800 600\n255\n").unwrap();
    for pixel in frame.color.chunks_exact(4) {
        file.write_all(&pixel[..3]).unwrap();
    }
}

/// Render at MSAA 1 and 4, after a resize and back; all must repeat exactly.
fn states(gpu: &mut Gpu, scene: &Scene, camera: &Camera) -> Vec<Frame> {
    let mut frames = Vec::new();
    // each sample count draws the same twice
    for msaa in [1, 4] {
        gpu.view.msaa_forced = Some(msaa);
        gpu.resize(800, 600);
        frames.push(render(gpu, scene, camera));
        same(
            &format!("MSAA{msaa} repeat"),
            &frames[frames.len() - 1],
            &render(gpu, scene, camera),
        );
    }
    gpu.view.msaa_forced = Some(1);
    gpu.resize(800, 600);
    same("MSAA1→4→1", &frames[0], &render(gpu, scene, camera));
    gpu.resize(960, 720);
    let _ = render(gpu, scene, camera);
    gpu.resize(800, 600);
    same("resize→restore", &frames[0], &render(gpu, scene, camera));
    frames
}

/// Load, rebuild and reload one scene; every state must draw the same.
fn check(gpu: &mut Gpu, sources: &[Source], name: &str, out: &Path) {
    let mut scene = Scene::new();
    load(&mut scene, gpu, sources, false);
    if name == "lines" {
        assert_eq!(gpu.arena.face_count(), 0); // a lines-only scene has no faces
    }
    // perspective iso and orthographic top
    for (view_name, view) in [("iso", View::Iso), ("top", View::Top)] {
        let mut camera = Camera::new();
        camera.fit(&gpu.bounds, 4.0 / 3.0);
        if view_name == "top" {
            camera.set_view(view);
        }
        println!(
            "{name} {view_name}: eye {:?}, target {:?}, perspective {}",
            camera.position, camera.target, camera.perspective
        );
        let baseline = states(gpu, &scene, &camera);
        for (i, frame) in baseline.iter().enumerate() {
            write_frame(&out.join(format!("{name}_{view_name}_{i}.ppm")), frame);
        }
        // rebuilt rows draw the same
        scene.rebuild(gpu);
        let rebuilt = states(gpu, &scene, &camera);
        for (i, (a, b)) in baseline.iter().zip(&rebuilt).enumerate() {
            same(&format!("rebuild state{i}"), a, b);
        }
        // reloaded one file at a time draws the same
        scene.clear(gpu);
        load(&mut scene, gpu, sources, true);
        let incremental = states(gpu, &scene, &camera);
        for (i, (a, b)) in baseline.iter().zip(&incremental).enumerate() {
            same(&format!("incremental after release state{i}"), a, b);
        }
    }
    scene.clear(gpu);
}

/// Two small documents of lines and points.
fn lines() -> Vec<Source> {
    (0..2)
        .map(|i| {
            let name = format!("lines{i}");
            let mut session = Session::new(&name);
            for y in [-200.0, 0.0, 200.0] {
                let mut line = Line::new(-300.0, y, 0.0, 300.0, y + 70.0, 0.0);
                line.linecolor = Color::blue();
                line.width = -1.0;
                session.add_line(line, None);
                let mut point = Point::new(-300.0, y, 0.0);
                point.pointcolor = Color::red();
                point.width = 5.0;
                session.add_point(point, None);
            }
            (
                name,
                Rc::new(session),
                Xform::translation(i as f64 * 800.0, 0.0, i as f64 * 100.0),
            )
        })
        .collect()
}

/// Run the checks and write the frames to the output directory.
pub fn run() {
    let mut args = std::env::args().skip(1);
    let out = args.next().expect("output directory");
    let out = Path::new(&out);
    std::fs::create_dir_all(out).unwrap();
    let mut gpu = pollster::block_on(Gpu::new_headless(800, 600)).expect("headless GPU");
    gpu.view.show_grid = false;
    check(&mut gpu, &lines(), "lines", out);
    let files: Vec<_> = args
        .enumerate()
        .map(|(i, path)| {
            let session = Session::pb_loads(&std::fs::read(&path).unwrap()).unwrap();
            (
                path,
                Rc::new(session),
                Xform::translation(i as f64 * 6000.0, 0.0, 0.0),
            )
        })
        .collect();
    if !files.is_empty() {
        check(&mut gpu, &files, "meshes", out);
    }
    println!(
        "lifecycle OK: no-face rendering, runtime MSAA toggles, resize, rebuild, release, incremental uploads and picking"
    );
}
