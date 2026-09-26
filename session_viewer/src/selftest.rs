//! Native harness: render a scene to a file without a browser.

use crate::app::manifest::Manifest;
use crate::app::scene::{FileDoc, Scene};
use crate::camera::FOVY_DEG;
use crate::camera::{Camera, View};
use crate::engine::gpu::{FrameInput, Gpu, Pick};
use crate::engine::performance::now_ms;
use crate::engine::text::{TextLabel, TextPlacement};
use session_rust::{Session, Xform};
use std::rc::Rc;

/// Background color.
const CLEAR: wgpu::Color = wgpu::Color {
    r: 0.9,
    g: 0.9,
    b: 0.9,
    a: 1.0,
};

/// One file to load.
pub struct SceneFile {
    pub path: String,       // the .pb file
    pub place: Xform,       // where it goes in the scene
    pub point_px: f32,      // point size, 0 for the default
    pub display_only: bool, // drop the source after upload
}

impl SceneFile {
    /// Files from the command line: manifests or single .pb files.
    pub fn from_args(args: &[String]) -> Vec<SceneFile> {
        let mut out = Vec::new();
        for p in args {
            // a plain .pb at the origin
            if !(p.ends_with(".json") || p.ends_with(".yaml") || p.ends_with(".yml")) {
                out.push(SceneFile {
                    path: p.clone(),
                    place: Xform::identity(),
                    point_px: 0.0,
                    display_only: false,
                });
                continue;
            }
            let bytes =
                std::fs::read(p).unwrap_or_else(|e| panic!("cannot read manifest {p}: {e}"));
            let man = Manifest::parse(&bytes)
                .unwrap_or_else(|e| panic!("cannot parse manifest {p}: {e}"));
            let root = assets_root(p, &man);
            // every file the manifest lists, placed as it says
            for (i, item) in man.items.iter().enumerate() {
                let path = root.join(&item.file).to_string_lossy().into_owned();
                out.push(SceneFile {
                    path,
                    place: man.place(i, [3000.0, 3000.0]),
                    point_px: item.point_size as f32,
                    display_only: item.display_only,
                });
            }
        }
        out
    }
}

/// The directory the manifest's files are in: its own, or its parent.
fn assets_root(manifest: &str, man: &Manifest) -> std::path::PathBuf {
    let here = std::path::Path::new(manifest)
        .parent()
        .unwrap_or(std::path::Path::new("."))
        .to_path_buf();
    let first = man
        .items
        .first()
        .map(|i| i.file.clone())
        .unwrap_or_default();
    if here.join(&first).exists() {
        here
    } else {
        here.join("..")
    }
}

/// The camera, set from VIEWER_* environment variables.
fn camera_from_env(gpu: &Gpu, aspect: f64) -> Camera {
    let mut camera = Camera::new();
    camera.fit(&gpu.bounds, aspect);
    // VIEWER_ORBIT=dx,dy in mouse pixels
    if let Ok(o) = std::env::var("VIEWER_ORBIT") {
        let mut it = o.split(',').filter_map(|v| v.trim().parse::<f32>().ok());
        camera.orbit(it.next().unwrap_or(0.0), it.next().unwrap_or(0.0));
    }
    // VIEWER_ORTHO=1 for orthographic
    if std::env::var("VIEWER_ORTHO").is_ok() {
        camera.toggle_projection();
    }
    // VIEWER_VIEW=top|bottom|front|right|iso
    if let Ok(v) = std::env::var("VIEWER_VIEW") {
        camera.set_view(match v.as_str() {
            "top" => View::Top,
            "bottom" => View::Bottom,
            "front" => View::Front,
            "right" => View::Right,
            _ => View::Iso,
        });
    }
    // VIEWER_ZOOM=N wheel steps, negative zooms out
    if let Ok(z) = std::env::var("VIEWER_ZOOM") {
        let n: i32 = z.trim().parse().unwrap_or(0);
        for _ in 0..n.abs() {
            camera.zoom(if n > 0 { 1.0 } else { -1.0 });
        }
    }
    // VIEWER_DISTANCE_SCALE=k multiplies the distance
    if let Ok(value) = std::env::var("VIEWER_DISTANCE_SCALE") {
        let scale: f64 = value
            .parse()
            .expect("VIEWER_DISTANCE_SCALE must be a number");
        assert!(
            scale.is_finite() && scale > 0.0,
            "VIEWER_DISTANCE_SCALE must be positive and finite"
        );
        camera.distance *= scale;
        camera.update_position();
    }
    report_camera(&camera);
    camera
}

/// Log the camera in scene units.
fn report_camera(camera: &Camera) {
    let units = camera.unit.to_meters();
    let eye = camera.position.map(|v| v / units); // meters to scene units
    let target = camera.target.map(|v| v / units);
    let forward = std::array::from_fn::<_, 3, _>(|i| {
        (camera.target[i] - camera.position[i]) / camera.distance
    });
    // half the view height, orthographic only
    let ortho_h = if camera.perspective {
        0.0
    } else {
        camera.distance_world() * (FOVY_DEG * 0.5).to_radians().tan()
    };
    log::info!(
        "camera: eye ({:.6}, {:.6}, {:.6}) mm, target ({:.6}, {:.6}, {:.6}) mm, distance {:.6} mm",
        eye[0],
        eye[1],
        eye[2],
        target[0],
        target[1],
        target[2],
        camera.distance_world()
    );
    log::info!(
        "census camera: CENSUS_EYE={:.12},{:.12},{:.12} CENSUS_FWD={:.12},{:.12},{:.12} CENSUS_UP={:.12},{:.12},{:.12} CENSUS_ORTHO_H={:.9}",
        camera.position[0],
        camera.position[1],
        camera.position[2],
        forward[0],
        forward[1],
        forward[2],
        camera.up[0],
        camera.up[1],
        camera.up[2],
        ortho_h
    );
}

/// Load every file into the scene, then upload once.
fn load_files(scene: &mut Scene, gpu: &mut Gpu, files: &[SceneFile]) {
    for f in files {
        let t0 = std::time::Instant::now();
        let bytes =
            std::fs::read(&f.path).unwrap_or_else(|e| panic!("cannot read {}: {e}", f.path));
        let session =
            Session::pb_loads(&bytes).unwrap_or_else(|e| panic!("cannot parse {}: {e:?}", f.path));
        let t1 = t0.elapsed();
        let name = f.path.rsplit('/').next().unwrap_or(&f.path).to_string();
        scene.add_file(FileDoc {
            name: name.clone(),
            session: Rc::new(session),
            place: f.place.clone(),
            point_px: f.point_px,
            display_only: f.display_only,
        });
        println!(
            "  {name}: {:.1} MB | decode {t1:?} | walk {:?}",
            bytes.len() as f64 / 1.048576e6,
            t0.elapsed() - t1
        );
    }
    scene.upload_to(gpu);
}

/// The matrix and clear color for one frame.
fn frame_input(gpu: &mut Gpu, camera: &Camera, aspect: f64) -> FrameInput {
    let now = now_ms();
    let rebase = gpu.rebase_anchor(&camera.origin(), camera.distance_world(), now);
    FrameInput {
        view_proj: camera.view_proj_anchored(aspect, &rebase.anchor),
        clear: CLEAR,
        now_ms: now,
    }
}

/// Drag frame times by tier, and how long the first slow frames took to reach each tier.
fn report_drag(ms: &[f64], tiers: &[u8]) {
    let percentile = |values: &mut Vec<f64>, p: f64| {
        values.sort_by(f64::total_cmp);
        values.get(((values.len() as f64 - 1.0) * p).round() as usize).copied().unwrap_or(0.0)
    };
    let mut all = ms.to_vec();
    println!(
        "drag: p50 {:.1} ms, p95 {:.1} ms",
        percentile(&mut all, 0.5),
        percentile(&mut all, 0.95)
    );
    // the first frame timed the pause before the drag
    let first_slow = ms.iter().skip(1).position(|t| *t > 33.0).map(|i| i + 1);

    for tier in 1..=crate::engine::performance::TOP_TIER {
        let Some(entered) = tiers.iter().position(|t| *t >= tier) else {
            continue;
        };
        let since = first_slow.map_or(0.0, |start| ms[start..entered].iter().sum::<f64>());
        let mut at: Vec<f64> = ms.iter().zip(tiers).filter(|(_, t)| **t == tier).map(|(v, _)| *v).collect();
        println!(
            "drag: tier {tier} from frame {entered}, {since:.0} ms after the first slow frame; p50 {:.1} ms there",
            percentile(&mut at, 0.5)
        );
    }
}

/// Forty labels fixed on screen; with `mixed`, forty nameplates and forty anchors in the scene too.
fn bench_labels(gpu: &Gpu, mixed: bool) -> Vec<TextLabel> {
    let b = &gpu.bounds;
    let label = |id: u32, placement| TextLabel {
        id,
        object: None,
        text: format!("Label {id} of the bench"),
        font_size: 14.0,
        line_height: 20.0,
        color: [0, 0, 0, 255],
        placement,
        clip: None,
    };
    let mut labels = Vec::new();

    for i in 0..40u32 {
        let (left, top) = (20.0 + (i % 4) as f32 * 180.0, 20.0 + (i / 4) as f32 * 24.0);
        labels.push(label(i + 1, TextPlacement::Screen { left, top }));

        if mixed {
            // spread over the scene box
            let t = f64::from(i) / 40.0;
            let world = [b.cx + b.hx * (t * 2.0 - 1.0), b.cy + b.hy * (1.0 - t * 2.0), b.cz];
            let padding = [6.0, 3.0];
            labels.push(label(i + 101, TextPlacement::Nameplate { world, padding, rounded: true }));
            labels.push(label(i + 201, TextPlacement::Anchor { world, offset: [0.0, 12.0] }));
        }
    }

    labels
}

/// Up to six clipping planes through the middle of `bounds`, each cutting a different side.
fn middle_planes(bounds: &session_rust::AABB, count: usize) -> Vec<crate::engine::gpu::clip::ClipPlane> {
    use crate::app::clipping::{Mode, clip_plane, plane_from};
    let c = [bounds.cx, bounds.cy, bounds.cz];
    let h = [bounds.hx, bounds.hy, bounds.hz];
    // (axis, sign): the side cut away, a fifth of the way out from the middle
    let sides = [(2, 1.0), (0, 1.0), (1, 1.0), (2, -1.0), (0, -1.0), (1, -1.0)];
    let mut planes = Vec::new();

    for &(axis, sign) in sides.iter().take(count.min(6)) {
        let mut origin = c;
        origin[axis] += sign * h[axis] * 0.2;
        let mut normal = origin;
        normal[axis] += sign;
        let plane = plane_from(Mode::Normal, &[origin, normal], h[0].max(h[1]).max(h[2]))
            .expect("a plane through the scene");
        planes.extend(clip_plane(&plane, &Xform::identity()));
    }

    planes
}

/// Write RGBA pixels as a PPM image.
pub(crate) fn write_ppm(path: &str, rgba: &[u8], w: u32, h: u32) -> std::io::Result<()> {
    use std::io::Write;
    let mut f = std::io::BufWriter::new(std::fs::File::create(path)?);
    write!(f, "P6\n{w} {h}\n255\n")?;
    for px in rgba.chunks_exact(4) {
        f.write_all(&px[..3])?;
    }
    f.flush()
}

/// Load, frame, draw one frame and write it to `out`.
pub fn render_scene(files: &[SceneFile], w: u32, h: u32, out: &str) -> String {
    let mut gpu = pollster::block_on(Gpu::new_headless(w, h)).expect("headless gpu");
    let mut scene = Scene::new();
    // VIEWER_CLIP=n cuts with n clipping planes through the scene's middle; solids get checked first
    let planes: usize = std::env::var("VIEWER_CLIP")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(0);

    if planes > 0 {
        crate::app::clipping::verify_solids();
    }

    load_files(&mut scene, &mut gpu, files);
    gpu.set_clip_planes(&middle_planes(&gpu.bounds, planes));
    gpu.find_solids(|row| scene.solid_faces(row));
    // VIEWER_SELECT=name highlights one object
    if let Ok(name) = std::env::var("VIEWER_SELECT") {
        let row = (0..scene.row_count() as u32)
            .find(|row| scene.object_name(*row) == name)
            .unwrap_or_else(|| panic!("VIEWER_SELECT object not found: {name}"));
        scene.selected = Some(row);
        gpu.set_selected(row, true);
    }
    let aspect = w as f64 / h as f64;
    let mut camera = camera_from_env(&gpu, aspect);
    // VIEWER_AO=1 turns ambient occlusion on
    gpu.view.ssao = std::env::var_os("VIEWER_AO").is_some();
    // VIEWER_LABELS=screen|mixed adds labels fixed on screen, or also anchored in the scene
    if let Ok(kind) = std::env::var("VIEWER_LABELS") {
        let labels = bench_labels(&gpu, kind == "mixed");
        gpu.text.set_labels(labels).expect("bench labels");
    }

    // VIEWER_FRAMES=N times N frames first
    if let Some(n) = std::env::var("VIEWER_FRAMES")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
    {
        // VIEWER_FRAMES_ORBIT=dx orbits by dx mouse pixels before each timed frame
        let step: f32 = std::env::var("VIEWER_FRAMES_ORBIT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.0);
        let mut ms: Vec<f64> = Vec::new();
        // VIEWER_GPU_TIMING=1 also times every pass on the GPU
        gpu.timer = crate::engine::gpu::timing::PassTimer::new(&gpu.ctx);
        let mut text_ms = 0.0;
        let prepared = gpu.text.stats.preparations;
        // VIEWER_INTERACT=1 draws the orbit as a drag, through the drag quality tiers
        let drag = std::env::var_os("VIEWER_INTERACT").is_some();
        let mut tiers = Vec::new();
        gpu.performance.interacting = drag;

        for _ in 0..n.max(1) {
            camera.orbit(step, 0.0);
            let input = frame_input(&mut gpu, &camera, aspect);
            let before = gpu.text.stats.preparations;
            let t = std::time::Instant::now();
            let _ = gpu.render_offscreen(&input);
            ms.push(t.elapsed().as_secs_f64() * 1000.0);
            tiers.push(gpu.performance.drag_tier());

            if drag {
                gpu.performance.frame(0, 0, now_ms(), false);
            }

            if gpu.text.stats.preparations > before {
                text_ms += gpu.text.stats.preparation_ms;
            }
        }

        gpu.performance.interacting = false;

        if drag {
            report_drag(&ms, &tiers);
        }

        camera.orbit(-step * n.max(1) as f32, 0.0);
        ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!(
            "frames: n={} median {:.1} ms ({:.0} fps) min {:.1} max {:.1}",
            ms.len(),
            ms[ms.len() / 2],
            1000.0 / ms[ms.len() / 2],
            ms[0],
            ms[ms.len() - 1]
        );

        if let Some(timer) = gpu.timer.take() {
            let spans: Vec<String> = timer
                .medians()
                .iter()
                .map(|(label, ms)| format!("{label} {ms:.3}"))
                .collect();
            println!("gpu ms, median per pass: {}", spans.join(", "));
            let (buffers, textures) = gpu.allocated_bytes();
            let tiles = gpu.arena.tiles.allocated_bytes().0;
            println!("gpu bytes: buffers {buffers}, textures {textures}, tile tables {tiles}");
            let count = gpu.text.stats.preparations - prepared;
            println!(
                "text: {count} preparations, {:.4} ms each, {:.4} ms per frame",
                text_ms / count.max(1) as f64,
                text_ms / n.max(1) as f64
            );
        }
    }

    let input = frame_input(&mut gpu, &camera, aspect);
    let rgba = gpu.render_offscreen(&input);
    write_ppm(out, &rgba, w, h).expect("write ppm");
    // VIEWER_IDS=path writes the id frame
    if let Ok(path) = std::env::var("VIEWER_IDS") {
        write_ids(&mut gpu, &scene, &input, &path);
    }

    // VIEWER_PICK=x,y reports what is under that pixel
    if let Ok(v) = std::env::var("VIEWER_PICK") {
        let mut it = v.split(',').filter_map(|t| t.trim().parse::<u32>().ok());
        if let (Some(px), Some(py)) = (it.next(), it.next()) {
            report_pick(&mut gpu, &scene, &input, (px, py));
        }
    }

    // pixels darker than the background
    let ink = rgba
        .chunks_exact(4)
        .filter(|p| p[0] < 200 || p[1] < 200 || p[2] < 200)
        .count();
    format!(
        "wrote {out}  {w}x{h}  non-background pixels: {ink} ({:.1}%)\n",
        100.0 * ink as f64 / (w * h) as f64
    )
}

/// Write the id frame and a guid-to-row map beside it.
fn write_ids(gpu: &mut Gpu, scene: &Scene, input: &FrameInput, path: &str) {
    use std::io::Write;
    let ids = gpu.render_ids_offscreen(input);
    let mut file = std::io::BufWriter::new(std::fs::File::create(path).expect("create ID frame"));
    file.write_all(b"HLI2").expect("write ID format");
    file.write_all(&gpu.config.width.to_le_bytes())
        .expect("write ID width");
    file.write_all(&gpu.config.height.to_le_bytes())
        .expect("write ID height");
    for id in ids {
        file.write_all(&id[0].to_le_bytes())
            .expect("write object ID");
        file.write_all(&id[1].to_le_bytes())
            .expect("write segment ID");
    }
    file.flush().expect("flush ID frame");
    // which guid is which row
    let mut mapping = std::collections::BTreeMap::new();
    for row in 0..gpu.objects.len() {
        if let Some(hit) = scene.resolve(Pick { row, sub: 0 }, gpu) {
            let range = scene.ribbon_range(row).unwrap_or(0..0);
            mapping.insert(hit.guid, serde_json::json!({ "object_id": row + 1, "ribbon_start": range.start, "ribbon_count": range.len() }));
        }
    }
    let map = std::fs::File::create(format!("{path}.json")).expect("create ID mapping");
    serde_json::to_writer_pretty(map, &mapping).expect("write ID mapping");
    println!(
        "wrote IDs {path}: {}x{}, {} object GUIDs; opaque picking core, alpha >= 0.5",
        gpu.config.width,
        gpu.config.height,
        mapping.len()
    );
}

/// Pick one pixel and print what it hit.
fn report_pick(gpu: &mut Gpu, scene: &Scene, input: &FrameInput, at: (u32, u32)) {
    gpu.pick.request(at.0, at.1);
    let _ = gpu.render_offscreen(input);
    let _ = gpu.ctx.device.poll(wgpu::PollType::Wait {
        submission_index: None,
        timeout: None,
    });
    let pick: Option<Pick> = gpu.pick.poll().flatten();
    match pick.and_then(|p| scene.resolve(p, gpu)) {
        Some(hit) => match hit.point {
            Some(pt) => println!(
                "pick: ({},{}) doc='{}' row={} point={} id={} pos=({:.0}, {:.0}, {:.0})",
                at.0,
                at.1,
                hit.doc,
                hit.row,
                pt.local,
                pt.id,
                pt.position[0],
                pt.position[1],
                pt.position[2]
            ),
            None => println!(
                "pick: ({},{}) doc='{}' guid={} row={}",
                at.0, at.1, hit.doc, hit.guid, hit.row
            ),
        },
        None => println!("pick: ({},{}) nothing", at.0, at.1),
    }
}

/// Upload and picking checks on one device.
pub mod lifecycle;

/// Check that every BRep edge keeps its id after upload.
pub fn check_cad_edges(paths: &[String]) {
    let mut gpu = pollster::block_on(Gpu::new_headless(64, 64)).unwrap();
    let mut scene = Scene::new();
    for path in paths {
        check_cad_edge_file(&mut scene, &mut gpu, path);
    }
}

/// Check one BRep file's edge ids.
fn check_cad_edge_file(scene: &mut Scene, gpu: &mut Gpu, path: &str) {
    scene.clear(gpu);
    let bytes = std::fs::read(path).unwrap();
    let session = Session::pb_loads(&bytes).unwrap();
    scene.add_file(FileDoc {
        name: path.into(),
        session: Rc::new(session),
        place: Xform::identity(),
        point_px: 0.0,
        display_only: false,
    });
    scene.upload_to(gpu);
    let Some(session_rust::Geometry::BRep(brep)) = scene.geometry(0) else {
        panic!("expected BRep fixture");
    };
    // the edges the file has
    let mut expected = std::collections::BTreeSet::new();
    for (edge, source) in brep.m_edges.iter().enumerate() {
        if !source.degenerated {
            expected.insert(edge as u32);
        }
    }
    // the edges the GPU segments resolve to
    let mut actual = std::collections::BTreeSet::new();
    let mut segment = 0u32;
    while let Some(edge) = scene.edge_at(Pick {
        row: 0,
        sub: 0x8000_0000 | segment,
    }) {
        assert!(expected.contains(&edge), "invented CAD edge {edge}");
        assert_eq!(
            scene.edge_at(Pick {
                row: 1,
                sub: 0x8000_0000 | segment
            }),
            None
        );
        actual.insert(edge);
        segment += 1;
    }
    assert_eq!(actual, expected, "missing source edge in {path}");
    println!(
        "{path}: {segment} segments retain {} source edge IDs",
        actual.len()
    );
}
