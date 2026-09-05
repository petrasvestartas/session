//! Headless harness, native only: the same `encode_frame` the browser runs, aimed at an
//! offscreen texture and read back, so a shader change can be LOOKED AT and measured here.
//! Every number in the docs comes through this file.

use crate::math::eye_from_view_proj;
use std::rc::Rc;
use crate::app::manifest::Manifest;
use crate::app::scene::{FileDoc, Scene};
use crate::camera::{Camera, View};
use crate::engine::gpu::{FrameInput, Gpu, Pick};
use crate::engine::performance::now_ms;
use session_rust::{Session, Xform};

/// Background colour of a harness frame.
const CLEAR: wgpu::Color = wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 };

/// One file the harness loads: its path, placement, point size and whether the session is
/// released after the walk.
pub struct SceneFile {
    pub path: String,
    pub place: Xform,
    pub point_px: f32,
    pub display_only: bool,
}

impl SceneFile {
    /// The harness's arguments: a `.yaml`/`.json` argument is a manifest resolved the way the
    /// browser resolves it (files relative to the directory holding `pb/`), anything else one
    /// `.pb` at its own origin.
    pub fn from_args(args: &[String]) -> Vec<SceneFile> {
        let mut out = Vec::new();
        for p in args {
            if !(p.ends_with(".json") || p.ends_with(".yaml") || p.ends_with(".yml")) {
                out.push(SceneFile { path: p.clone(), place: Xform::identity(), point_px: 0.0, display_only: false });
                continue;
            }
            let bytes = std::fs::read(p).unwrap_or_else(|e| panic!("cannot read manifest {p}: {e}"));
            let man = Manifest::parse(&bytes).unwrap_or_else(|e| panic!("cannot parse manifest {p}: {e}"));
            let root = assets_root(p, &man);
            for (i, item) in man.items.iter().enumerate() {
                let path = root.join(&item.file).to_string_lossy().into_owned();
                out.push(SceneFile { path, place: man.place(i, [3000.0, 3000.0]), point_px: item.point_size as f32, display_only: item.display_only });
            }
        }
        out
    }
}

/// The directory a manifest's `file` entries hang off: its own, or its parent.
fn assets_root(manifest: &str, man: &Manifest) -> std::path::PathBuf {
    let here = std::path::Path::new(manifest).parent().unwrap_or(std::path::Path::new(".")).to_path_buf();
    let first = man.items.first().map(|i| i.file.clone()).unwrap_or_default();
    if here.join(&first).exists() { here } else { here.join("..") }
}

/// The harness's camera knobs: `VIEWER_ORBIT="dx,dy"`, `VIEWER_ORTHO`, `VIEWER_VIEW`, `VIEWER_ZOOM`.
fn camera_from_env(gpu: &Gpu, aspect: f64) -> Camera {
    let mut camera = Camera::new();
    camera.fit(&gpu.bounds, aspect);
    if let Ok(o) = std::env::var("VIEWER_ORBIT") {
        let mut it = o.split(',').filter_map(|v| v.trim().parse::<f32>().ok());
        camera.orbit(it.next().unwrap_or(0.0), it.next().unwrap_or(0.0));
    }
    if std::env::var("VIEWER_ORTHO").is_ok() {
        camera.toggle_projection();
    }
    if let Ok(v) = std::env::var("VIEWER_VIEW") {
        camera.set_view(match v.as_str() {
            "top" => View::Top,
            "bottom" => View::Bottom,
            "front" => View::Front,
            "right" => View::Right,
            _ => View::Iso,
        });
    }
    if let Ok(z) = std::env::var("VIEWER_ZOOM") {
        let n: i32 = z.trim().parse().unwrap_or(0);
        for _ in 0..n.abs() {
            camera.zoom(if n > 0 { 1.0 } else { -1.0 });
        }
    }
    let eye = eye_from_view_proj(&camera.view_proj(aspect));
    log::info!("camera: eye ({:.1}, {:.1}, {:.1}) mm, target ({:.1}, {:.1}, {:.1}) mm, distance {:.1} mm", eye[0], eye[1], eye[2], camera.target[0], camera.target[1], camera.target[2], camera.distance);
    camera
}

/// Load every file into `scene`, uploading per file when `VIEWER_INCREMENTAL` is set (the
/// browser's path) and once at the end otherwise. Prints per-file load costs.
fn load_files(scene: &mut Scene, gpu: &mut Gpu, files: &[SceneFile]) {
    let incremental = std::env::var("VIEWER_INCREMENTAL").is_ok();
    for f in files {
        let t0 = std::time::Instant::now();
        let bytes = std::fs::read(&f.path).unwrap_or_else(|e| panic!("cannot read {}: {e}", f.path));
        let session = Session::pb_loads(&bytes).unwrap_or_else(|e| panic!("cannot parse {}: {e:?}", f.path));
        let t1 = t0.elapsed();
        let name = f.path.rsplit('/').next().unwrap_or(&f.path).to_string();
        scene.add_file(FileDoc { name: name.clone(), session: Rc::new(session), place: f.place.clone(), point_px: f.point_px, display_only: f.display_only });
        println!("  {name}: {:.1} MB | decode {t1:?} | walk {:?}", bytes.len() as f64 / 1.048576e6, t0.elapsed() - t1);
        if incremental {
            scene.upload_to(gpu);
        }
    }
    if !incremental {
        scene.upload_to(gpu);
    }
    if std::env::var("VIEWER_REBUILD").is_ok() {
        let t = std::time::Instant::now();
        scene.rebuild(gpu);
        println!("rebuild {:?}", t.elapsed());
    }
}

/// The frame input for `camera` on `gpu`, re-anchoring first.
fn frame_input(gpu: &mut Gpu, camera: &Camera, aspect: f64) -> FrameInput {
    let now = now_ms();
    let rebase = gpu.rebase_anchor(&camera.origin(), camera.distance_world(), now);
    FrameInput { view_proj: camera.view_proj_anchored(aspect, &rebase.anchor), clear: CLEAR, now_ms: now }
}

/// Write RGBA8 rows as a binary PPM (P6).
fn write_ppm(path: &str, rgba: &[u8], w: u32, h: u32) -> std::io::Result<()> {
    use std::io::Write;
    let mut f = std::io::BufWriter::new(std::fs::File::create(path)?);
    write!(f, "P6\n{w} {h}\n255\n")?;
    for px in rgba.chunks_exact(4) {
        f.write_all(&px[..3])?;
    }
    f.flush()
}

/// Load, frame, render one frame, write it out; `VIEWER_FRAMES=N` times N frames first and
/// `VIEWER_PICK="x,y"` reports what the id pass finds under that pixel.
pub fn render_scene(files: &[SceneFile], w: u32, h: u32, out: &str) -> String {
    let mut gpu = pollster::block_on(Gpu::new_headless(w, h)).expect("headless gpu");
    let mut scene = Scene::new();
    load_files(&mut scene, &mut gpu, files);
    let aspect = w as f64 / h as f64;
    let camera = camera_from_env(&gpu, aspect);

    if let Some(n) = std::env::var("VIEWER_FRAMES").ok().and_then(|v| v.parse::<usize>().ok()) {
        let mut ms: Vec<f64> = Vec::new();
        for _ in 0..n.max(1) {
            let input = frame_input(&mut gpu, &camera, aspect);
            let t = std::time::Instant::now();
            let _ = gpu.render_offscreen(&input);
            ms.push(t.elapsed().as_secs_f64() * 1000.0);
        }
        ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!("frames: n={} median {:.1} ms ({:.0} fps) min {:.1} max {:.1}", ms.len(), ms[ms.len() / 2], 1000.0 / ms[ms.len() / 2], ms[0], ms[ms.len() - 1]);
    }

    let input = frame_input(&mut gpu, &camera, aspect);
    let rgba = gpu.render_offscreen(&input);
    write_ppm(out, &rgba, w, h).expect("write ppm");

    if let Ok(v) = std::env::var("VIEWER_PICK") {
        let mut it = v.split(',').filter_map(|t| t.trim().parse::<u32>().ok());
        if let (Some(px), Some(py)) = (it.next(), it.next()) {
            report_pick(&mut gpu, &scene, &input, (px, py));
        }
    }

    let ink = rgba.chunks_exact(4).filter(|p| p[0] < 200 || p[1] < 200 || p[2] < 200).count();
    format!("wrote {out}  {w}x{h}  non-background pixels: {ink} ({:.1}%)\n", 100.0 * ink as f64 / (w * h) as f64)
}

/// Pick at `at` through the id pass, blocking on the GPU, and print the answer.
fn report_pick(gpu: &mut Gpu, scene: &Scene, input: &FrameInput, at: (u32, u32)) {
    gpu.pick.request(at.0, at.1);
    let _ = gpu.render_offscreen(input);
    let _ = gpu.ctx.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
    let pick: Option<Pick> = gpu.pick.poll().flatten();
    match pick.and_then(|p| scene.resolve(p, gpu)) {
        Some(hit) => match hit.point {
            Some(pt) => println!("pick: ({},{}) doc='{}' row={} point={} id={} pos=({:.0}, {:.0}, {:.0})", at.0, at.1, hit.doc, hit.row, pt.local, pt.id, pt.position[0], pt.position[1], pt.position[2]),
            None => println!("pick: ({},{}) doc='{}' guid={} row={}", at.0, at.1, hit.doc, hit.guid, hit.row),
        },
        None => println!("pick: ({},{}) nothing", at.0, at.1),
    }
}

/// Where a frame's milliseconds go - uniforms, encode, GPU - for a still and a moving camera.
pub fn frame_profile(files: &[SceneFile], w: u32, h: u32) -> String {
    let mut gpu = pollster::block_on(Gpu::new_headless(w, h)).expect("headless gpu");
    let mut scene = Scene::new();
    load_files(&mut scene, &mut gpu, files);
    let aspect = w as f64 / h as f64;
    let n: usize = std::env::var("BENCH_FRAMES").ok().and_then(|v| v.parse().ok()).unwrap_or(120);
    let mut camera = camera_from_env(&gpu, aspect);
    let mut out = String::new();
    for (label, spin) in [("still", 0.0f32), ("moving", 0.35f32)] {
        let mut ms: Vec<f64> = Vec::new();
        for i in 0..n + 5 {
            camera.orbit(spin, 0.0);
            let input = frame_input(&mut gpu, &camera, aspect);
            let secs = gpu.bench_frames(&input, 1);
            if i >= 5 {
                ms.push(secs * 1000.0);
            }
        }
        ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let med = ms[ms.len() / 2];
        out.push_str(&format!("{label:>6}: {med:6.2} ms/frame ({:5.0} fps)\n", 1000.0 / med));
    }
    out
}
