//! Exercise visibility attachments, support-buffer rebasing and picking on one live device.
//! Usage: check_hidden_line_lifecycle /tmp/lifecycle [first.pb second.pb ...]
//! Always checks a generated lines/points-only scene; optional files form a second scene.

use std::{collections::HashSet, io::Write, path::Path, rc::Rc};
use session_rust::{Color, Line, Point, Session, Xform};
use crate::{
    app::scene::{FileDoc, Scene},
    camera::{Camera, View},
    engine::gpu::{view::LineStyle, FrameInput, Gpu, Pick},
};

type Source = (String, Rc<Session>, Xform);

#[derive(PartialEq, Eq)]
struct Frame {
    color: Vec<u8>,
    ids: Vec<[u32; 2]>,
}

/// Exercise the browser's append path and the harness's deferred upload with identical rows.
fn load(scene: &mut Scene, gpu: &mut Gpu, sources: &[Source], incremental: bool) {
    for (name, session, place) in sources {
        scene.add_file(FileDoc { name: name.clone(), session: Rc::clone(session), place: place.clone(), point_px: 0.0, display_only: false });
        if incremental { scene.upload_to(gpu); }
    }
    if !incremental { scene.upload_to(gpu); }
}

/// Capture complete spatial output and validate every visible object/segment identity.
fn render(gpu: &mut Gpu, scene: &Scene, camera: &Camera) -> Frame {
    let anchor = gpu.rebase_anchor(&camera.origin(), camera.distance_world(), 0.0).anchor;
    let input = FrameInput {
        view_proj: camera.view_proj_anchored(4.0 / 3.0, &anchor),
        clear: wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 }, now_ms: 0.0,
    };
    let color = gpu.render_offscreen(&input);
    let ids = gpu.render_ids_offscreen(&input);
    assert!(ids.iter().any(|id| id[0] != 0), "scene must produce pickable geometry");
    for [object, sub] in ids.iter().copied().filter(|id| id[0] != 0).collect::<HashSet<_>>() {
        let row = object - 1;
        let decoded = sub.saturating_sub(1);
        let hit = scene.resolve(Pick { row, sub: decoded }, gpu).expect("rendered object ID resolves");
        assert_eq!(hit.row, row);
        if sub & 0x8000_0000 != 0 && let Some(range) = scene.ribbon_range(row) {
            assert!(range.contains(&((sub & 0x7fff_ffff) - 1)), "ribbon ID must stay inside its retained object range");
        }
    }
    Frame { color, ids }
}

/// Fail on displaced pixels even when aggregate coverage totals agree.
fn same(label: &str, reference: &Frame, current: &Frame) {
    assert_eq!((reference.color.len(), reference.ids.len()), (current.color.len(), current.ids.len()), "{label}: frame dimensions");
    let color = reference.color.chunks_exact(4).zip(current.color.chunks_exact(4)).filter(|(a, b)| a != b).count();
    let ids = reference.ids.iter().zip(&current.ids).filter(|(a, b)| a != b).count();
    assert_eq!((color, ids), (0, 0), "{label}: spatial pixel/ID differences");
    println!("{label}: identical full color and picking frames");
}

/// Retain a reference image for visual continuity and marker inspection.
fn write_frame(path: &Path, frame: &Frame) {
    let mut file = std::io::BufWriter::new(std::fs::File::create(path).unwrap());
    write!(file, "P6\n800 600\n255\n").unwrap();
    for pixel in frame.color.chunks_exact(4) { file.write_all(&pixel[..3]).unwrap(); }
}

/// Switch live pipelines and targets, requiring exact restoration after every round trip.
fn states(gpu: &mut Gpu, scene: &Scene, camera: &Camera) -> Vec<Frame> {
    let mut frames = Vec::new();
    for msaa in [1, 4] {
        gpu.view.msaa_forced = Some(msaa);
        gpu.resize(800, 600);
        gpu.view.line_style = LineStyle::Flat;
        frames.push(render(gpu, scene, camera));
        gpu.view.toggle_line_style();
        frames.push(render(gpu, scene, camera));
        gpu.view.toggle_line_style();
        same(&format!("MSAA{msaa} FLAT→TUBE→FLAT"), &frames[frames.len() - 2], &render(gpu, scene, camera));
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

/// Rebuild and release/reload each scene under perspective and parallel camera rays.
fn check(gpu: &mut Gpu, sources: &[Source], name: &str, out: &Path) {
    let mut scene = Scene::new();
    load(&mut scene, gpu, sources, false);
    if name == "lines" { assert_eq!(gpu.arena.face_count(), 0); }
    for (view_name, view) in [("iso", View::Iso), ("top", View::Top)] {
        let mut camera = Camera::new();
        camera.fit(&gpu.bounds, 4.0 / 3.0);
        if view_name == "top" { camera.set_view(view); }
        println!("{name} {view_name}: eye {:?}, target {:?}, perspective {}", camera.position, camera.target, camera.perspective);
        let baseline = states(gpu, &scene, &camera);
        for (i, frame) in baseline.iter().enumerate() { write_frame(&out.join(format!("{name}_{view_name}_{i}.ppm")), frame); }
        scene.rebuild(gpu);
        let rebuilt = states(gpu, &scene, &camera);
        for (i, (a, b)) in baseline.iter().zip(&rebuilt).enumerate() { same(&format!("rebuild state{i}"), a, b); }
        scene.clear(gpu);
        load(&mut scene, gpu, sources, true);
        let incremental = states(gpu, &scene, &camera);
        for (i, (a, b)) in baseline.iter().zip(&incremental).enumerate() { same(&format!("incremental after release state{i}"), a, b); }
    }
    scene.clear(gpu);
}

/// Two independent documents ensure the empty-face case also grows row and ink buffers.
fn lines() -> Vec<Source> {
    (0..2).map(|i| {
        let name = format!("lines{i}");
        let mut session = Session::new(&name);
        for y in [-200.0, 0.0, 200.0] {
            let mut line = Line::new(-300.0, y, 0.0, 300.0, y + 70.0, 0.0);
            line.linecolor = Color::blue(); line.width = -1.0;
            session.add_line(line, None);
            let mut point = Point::new(-300.0, y, 0.0);
            point.pointcolor = Color::red(); point.width = 5.0;
            session.add_point(point, None);
        }
        (name, Rc::new(session), Xform::translation(i as f64 * 800.0, 0.0, i as f64 * 100.0))
    }).collect()
}

/// Run the native lifecycle verification and write its reference images.
pub fn run() {
    let mut args = std::env::args().skip(1);
    let out = args.next().expect("output directory");
    let out = Path::new(&out);
    std::fs::create_dir_all(out).unwrap();
    let mut gpu = pollster::block_on(Gpu::new_headless(800, 600)).expect("headless GPU");
    gpu.view.show_grid = false;
    check(&mut gpu, &lines(), "lines", out);
    let files: Vec<_> = args.enumerate().map(|(i, path)| {
        let session = Session::pb_loads(&std::fs::read(&path).unwrap()).unwrap();
        (path, Rc::new(session), Xform::translation(i as f64 * 6000.0, 0.0, 0.0))
    }).collect();
    if !files.is_empty() { check(&mut gpu, &files, "meshes", out); }
    println!("lifecycle OK: no-face rendering, runtime style/MSAA toggles, resize, rebuild, release, incremental uploads and picking");
}
