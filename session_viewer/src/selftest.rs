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

pub mod stroke_witness;
pub use crate::engine::gpu::splat::cloud_witness;

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
    // VIEWER_TARGET=x,y,z aims the camera at a scene point, mm
    if let Ok(value) = std::env::var("VIEWER_TARGET") {
        let mut it = value
            .split(',')
            .filter_map(|v| v.trim().parse::<f64>().ok());
        let units = camera.unit.to_meters();
        let target = [
            it.next().unwrap_or(0.0) * units,
            it.next().unwrap_or(0.0) * units,
            it.next().unwrap_or(0.0) * units,
        ];

        for axis in 0..3 {
            camera.position[axis] += target[axis] - camera.target[axis];
            camera.target[axis] = target[axis];
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

        // VIEWER_PROFILE=1 times every object's walk by kind before the scene takes the file
        if std::env::var_os("VIEWER_PROFILE").is_some() {
            profile::document(&session);
        }

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
        values
            .get(((values.len() as f64 - 1.0) * p).round() as usize)
            .copied()
            .unwrap_or(0.0)
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
        let mut at: Vec<f64> = ms
            .iter()
            .zip(tiers)
            .filter(|(_, t)| **t == tier)
            .map(|(v, _)| *v)
            .collect();
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
            let world = [
                b.cx + b.hx * (t * 2.0 - 1.0),
                b.cy + b.hy * (1.0 - t * 2.0),
                b.cz,
            ];
            let padding = [6.0, 3.0];
            labels.push(label(
                i + 101,
                TextPlacement::Nameplate {
                    world,
                    padding,
                    rounded: true,
                },
            ));
            labels.push(label(
                i + 201,
                TextPlacement::Anchor {
                    world,
                    offset: [0.0, 12.0],
                },
            ));
        }
    }

    labels
}

/// Up to six clipping planes through the middle of `bounds`, each cutting a different side.
fn middle_planes(
    bounds: &session_rust::AABB,
    count: usize,
) -> Vec<crate::engine::gpu::clip::ClipPlane> {
    use crate::app::clipping::{Mode, clip_plane, plane_from};
    let c = [bounds.cx, bounds.cy, bounds.cz];
    let h = [bounds.hx, bounds.hy, bounds.hz];
    // (axis, sign): the side cut away, a fifth of the way out from the middle
    let sides = [
        (2, 1.0),
        (0, 1.0),
        (1, 1.0),
        (2, -1.0),
        (0, -1.0),
        (1, -1.0),
    ];
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

// --8<-- [start:selftest-pixels]
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

// --8<-- [end:selftest-pixels]
fn drain_refinement(
    rows: usize,
    mut step: impl FnMut() -> (bool, bool),
) -> Result<usize, &'static str> {
    let mut changed = 0;
    // Each step consumes at least one row; a replacement can add one hidden sink row.
    for _ in 0..rows.saturating_add(1) {
        let (pending, updated) = step();
        changed += usize::from(updated);
        if !pending {
            return Ok(changed);
        }
    }
    Err("capture display refinement did not settle within its row budget")
}

fn refine_capture(scene: &mut Scene, gpu: &mut Gpu, camera: &Camera) {
    let changed = drain_refinement(scene.row_count(), || scene.refine_display(gpu, camera))
        .expect("bounded capture display refinement");
    if changed > 0 {
        gpu.find_solids(|row| scene.solid_faces(row));
        println!("display refinement: {changed} row or shared-batch replacements");
    }
    let (sources, charged, accepted, limited) = scene.display_counters();
    println!(
        "display quality: retained_source_bytes={sources} charged_lane_bytes={charged} accepted_rows={accepted} limited_rows={limited}"
    );
    for (row, actual, requested, reason) in scene.display_quality_sample(16) {
        println!(
            "display row {row}: accepted_tolerance={actual:?} unmet_requested_tolerance={requested:?} state={reason}"
        );
    }
}

#[cfg(test)]
mod refinement_tests {
    use super::drain_refinement;

    #[test]
    fn counts_the_final_replacement_and_stops_when_settled() {
        let mut steps = [(true, true), (false, true)].into_iter();
        assert_eq!(
            drain_refinement(2, || steps.next().expect("unexpected extra step")),
            Ok(2)
        );
        assert_eq!(drain_refinement(0, || (false, false)), Ok(0));
    }

    #[test]
    fn nonsettling_capture_fails_instead_of_looping_or_claiming_ready() {
        let mut calls = 0;
        assert!(
            drain_refinement(3, || {
                calls += 1;
                (true, false)
            })
            .is_err()
        );
        assert_eq!(calls, 4);
    }
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
    // VIEWER_DPR=2 renders that many pixels per CSS pixel, as a high-density screen does, so the
    // Arctic outline and the pens keep their on-screen width in a frame meant to be scaled down
    if let Some(dpr) = std::env::var("VIEWER_DPR")
        .ok()
        .and_then(|v| v.parse::<f64>().ok())
        .filter(|dpr| *dpr > 0.0)
    {
        gpu.logical_size = [f64::from(w) / dpr, f64::from(h) / dpr];
    }
    // VIEWER_LABELS=screen|mixed adds labels fixed on screen, or also anchored in the scene
    if let Ok(kind) = std::env::var("VIEWER_LABELS") {
        let labels = bench_labels(&gpu, kind == "mixed");
        gpu.text.set_labels(labels).expect("bench labels");
    }

    refine_capture(&mut scene, &mut gpu, &camera);

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
        // Warm initial pipelines and resting targets before timing steady-state frames.
        gpu.prewarm_initial();
        let warm_input = frame_input(&mut gpu, &camera, aspect);
        let _ = gpu.render_offscreen(&warm_input);

        // VIEWER_GPU_TIMING=1 also times every pass on the GPU
        gpu.timer = crate::engine::gpu::timing::PassTimer::new(&gpu.ctx);
        let mut text_ms = 0.0;
        let prepared = gpu.text.stats.preparations;
        // VIEWER_INTERACT=1 draws the orbit as a drag, through the drag quality tiers
        let drag = std::env::var_os("VIEWER_INTERACT").is_some();
        let mut tiers = Vec::new();
        gpu.performance.interacting = drag;

        // VIEWER_TIER=n draws every timed frame at drag tier n, as a slow drag would
        if let Some(tier) = std::env::var("VIEWER_TIER")
            .ok()
            .and_then(|v| v.parse::<u8>().ok())
        {
            gpu.performance.force_tier(tier);
        }

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
            let medians = timer.medians();
            let ink_total: f64 = medians
                .iter()
                .filter(|(label, _)| label.starts_with("ink"))
                .map(|(_, ms)| *ms)
                .sum();
            let mut spans: Vec<String> = medians
                .iter()
                .map(|(label, ms)| format!("{label} {ms:.3}"))
                .collect();
            spans.push(format!("ink_total {ink_total:.3}"));
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

    refine_capture(&mut scene, &mut gpu, &camera);
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
#[cfg(test)]
mod oracle;
pub mod profile;

/// Check that every BRep edge keeps its id after upload.
pub fn check_cad_edges(paths: &[String]) {
    let mut gpu = pollster::block_on(Gpu::new_headless(64, 64)).unwrap();
    let mut scene = Scene::new();
    for path in paths {
        check_cad_edge_file(&mut scene, &mut gpu, path);
    }
}

/// Render a mutated `HeadlessHost` (documents, appearance overrides, camera, and display settings)
/// into a `.ppm` file via a headless `wgpu` adapter; returns the count of non-background pixels.
pub fn render_headless_host(
    host: &viewer_core::command::HeadlessHost,
    w: u32,
    h: u32,
    out: &str,
) -> Result<usize, String> {
    if w == 0 || h == 0 {
        return Err("render_frame width and height must be positive".into());
    }
    let mut gpu = pollster::block_on(Gpu::new_headless(w, h))
        .map_err(|e| format!("headless GPU unavailable: {e}"))?;
    gpu.view.ssao = host.display.ssao;
    gpu.view.wireframe = host.display.wireframe;
    gpu.view.show_outlines = host.display.show_outlines;
    gpu.view.show_points = host.display.show_points;
    gpu.view.show_lines = host.display.show_lines;
    gpu.view.show_mesh_edges = host.display.show_mesh_edges;
    gpu.view.lit = host.display.lit;
    gpu.view.backface = host.display.backface;
    gpu.view.cull_backfaces = host.display.cull_backfaces;
    gpu.view.opacity = host.display.opacity;
    gpu.view.show_grid = host.display.show_grid;
    gpu.view.cloud_size = host.display.cloud_size;

    // Replay the host's retained state via `.session` round-trip so a fresh `Scene` uploads cleanly to `gpu`.
    let cam = viewer_core::session_io::SavedCamera::from_camera(&host.camera);
    let bytes = viewer_core::session_io::save_with_state(&host.scene, Some(cam), None)?;
    let mut opened = viewer_core::session_io::open_with_state(&bytes)?;
    opened.scene.wireframe = host.display.wireframe;
    opened.scene.attributes = host.scene.attributes;
    opened.scene.interactions = host.scene.interactions;
    opened.scene.upload_to(&mut gpu);
    opened.scene.restore_text_visibility(&mut gpu);
    let _ = gpu.text.set_labels(opened.scene.visible_texts());

    let aspect = f64::from(w) / f64::from(h);
    let input = frame_input(&mut gpu, &host.camera, aspect);
    let rgba = gpu.render_offscreen(&input);
    write_ppm(out, &rgba, w, h).map_err(|e| format!("cannot write {out}: {e}"))?;
    let ink = rgba
        .chunks_exact(4)
        .filter(|p| p[0] < 200 || p[1] < 200 || p[2] < 200)
        .count();
    Ok(ink)
}

/// Validate every step of a stage deck for errors and overlaps, and render each chapter's settled slide to `<stem>.ppm` beside its YAML.
pub fn render_stage_deck_ppms(
    deck_dir: &str,
    width: u32,
    height: u32,
) -> Result<Vec<String>, String> {
    use crate::app::stage::{deck, font, presentation::Presentation};
    let w = width.clamp(320, 4096);
    let h = height.clamp(240, 4096);
    let font_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/stage/");
    let font_files: Vec<Vec<u8>> = font::FILES
        .iter()
        .map(|name| {
            std::fs::read(format!("{font_dir}{name}"))
                .map_err(|e| format!("cannot read stage font {name}: {e}"))
        })
        .collect::<Result<_, _>>()?;
    let fonts = font::Fonts::new(font_files).map_err(|e| format!("stage fonts: {e}"))?;

    let dir = std::path::Path::new(deck_dir);
    let index_src = std::fs::read_to_string(dir.join("index.yaml"))
        .map_err(|e| format!("cannot read {deck_dir}/index.yaml: {e}"))?;
    let index = deck::index(&index_src).map_err(|e| format!("{deck_dir}/index.yaml: {e}"))?;
    let mut chapters = Vec::with_capacity(index.chapters.len());
    for file in &index.chapters {
        let src = std::fs::read_to_string(dir.join(file))
            .map_err(|e| format!("cannot read {deck_dir}/{file}: {e}"))?;
        chapters.push(deck::chapter(&src, file).map_err(|e| format!("{deck_dir}/{file}: {e}"))?);
    }

    let mut gpu = pollster::block_on(Gpu::new_headless(w, h))
        .map_err(|e| format!("headless GPU unavailable: {e}"))?;
    gpu.view.show_grid = false;
    let mut scene = Scene::new();
    let mut camera = Camera::new();
    let logical = [f64::from(w), f64::from(h)];

    let total_chapters = chapters.len();
    let chapter_files = index.chapters.clone();
    let mut presentation = Presentation::new(fonts, index, chapters, Vec::new(), 0.0);
    let egui_ctx = egui::Context::default();
    let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(w as f32, h as f32));
    let mut written = Vec::with_capacity(total_chapters);

    for ch_idx in 0..total_chapters {
        let ch_num = ch_idx + 1;
        let total_steps: usize = presentation.stage.chapters[ch_idx]
            .slides
            .iter()
            .map(|s| s.steps.len())
            .sum();
        for step in 1..=total_steps {
            presentation.stage.open_at(ch_num, step, 0.0);
            presentation.stage.tick(10_000.0);
            let f = crate::app::stage::paint::Frame::new(screen, 1.0)
                .stacked(screen, presentation.stage.split)
                .side(0.0);
            crate::app::stage::spatial::sync_headless(
                &mut presentation.stage,
                &mut scene,
                &mut gpu,
                &mut camera,
                &f,
                logical,
                10_000.0,
            );
            presentation.stage.changed = true;
            let mut output = egui_ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(screen),
                    ..Default::default()
                },
                |root| {
                    let _ = presentation.paint(root.painter(), screen, 1.0);
                },
            );
            for (_, delta) in &mut output.textures_delta.set {
                let egui::ImageData::Color(img) = &mut delta.image;
                for p in &mut std::sync::Arc::make_mut(img).pixels {
                    if p.r() == p.g() && p.g() == p.b() && p.b() == p.a() {
                        let a = p.a();
                        let s = ((f32::from(a) / 255.0).powf(1.0 / 2.2) * 255.0).round() as u8;
                        *p = egui::Color32::from_rgba_premultiplied(s, s, s, a);
                    }
                }
            }
            let ui = gpu.ui.get_or_insert_with(|| {
                crate::engine::gpu::ui::Ui::new(&gpu.ctx.device, gpu.config.format)
            });
            ui.prepare(&gpu.ctx.device, &gpu.ctx.queue, &egui_ctx, output, [w, h]);
            if !presentation.stage.errors.is_empty() {
                return Err(format!(
                    "{deck_dir} chapter {ch_num} ({}) step {step} errors: {:?}",
                    chapter_files[ch_idx], presentation.stage.errors
                ));
            }
            let overlaps = presentation.stage.overlaps();
            if !overlaps.is_empty() {
                return Err(format!(
                    "{deck_dir} chapter {ch_num} ({}) step {step} overlaps: {:?}",
                    chapter_files[ch_idx], overlaps
                ));
            }
        }

        presentation.stage.open_at(ch_num, total_steps, 0.0);
        presentation.stage.tick(10_000.0);
        let f = crate::app::stage::paint::Frame::new(screen, 1.0)
            .stacked(screen, presentation.stage.split)
            .side(0.0);
        crate::app::stage::spatial::sync_headless(
            &mut presentation.stage,
            &mut scene,
            &mut gpu,
            &mut camera,
            &f,
            logical,
            10_000.0,
        );
        presentation.stage.changed = true;
        for _ in 0..2 {
            let mut output = egui_ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(screen),
                    ..Default::default()
                },
                |root| {
                    let _ = presentation.paint(root.painter(), screen, 1.0);
                },
            );
            for (_, delta) in &mut output.textures_delta.set {
                let egui::ImageData::Color(img) = &mut delta.image;
                for p in &mut std::sync::Arc::make_mut(img).pixels {
                    if p.r() == p.g() && p.g() == p.b() && p.b() == p.a() {
                        let a = p.a();
                        let s = ((f32::from(a) / 255.0).powf(1.0 / 2.2) * 255.0).round() as u8;
                        *p = egui::Color32::from_rgba_premultiplied(s, s, s, a);
                    }
                }
            }
            let ui = gpu.ui.get_or_insert_with(|| {
                crate::engine::gpu::ui::Ui::new(&gpu.ctx.device, gpu.config.format)
            });
            ui.prepare(&gpu.ctx.device, &gpu.ctx.queue, &egui_ctx, output, [w, h]);
        }
        let bg = presentation.stage.palette("bg");
        let aspect = logical[0] / logical[1];
        let rebase = gpu.rebase_anchor(&camera.origin(), camera.distance_world(), 10_000.0);
        let input = FrameInput {
            view_proj: camera.view_proj_anchored(aspect, &rebase.anchor),
            clear: wgpu::Color {
                r: f64::from(bg[0]),
                g: f64::from(bg[1]),
                b: f64::from(bg[2]),
                a: 1.0,
            },
            now_ms: 10_000.0,
        };
        let rgba = gpu.render_offscreen(&input);
        let stem = chapter_files[ch_idx]
            .strip_suffix(".yaml")
            .unwrap_or(&chapter_files[ch_idx]);
        let out_ppm = dir.join(format!("{stem}.ppm"));
        let out_str = out_ppm.to_string_lossy().into_owned();
        write_ppm(&out_str, &rgba, w, h).map_err(|e| format!("cannot write {out_str}: {e}"))?;
        written.push(out_str);
    }
    Ok(written)
}

/// Render all real WebGPU achievement screenshots for Tutorials 1..17 into `out_dir/*.ppm`.
pub fn render_tutorial_gallery(out_dir: &str) -> Result<Vec<String>, String> {
    use session_rust::{Arrowhead, Color, Line, Mesh, NurbsCurve, Point, Quaternion, Vector};
    use viewer_core::command::HeadlessHost;
    use viewer_core::op::{self, AxisSpec, CreateKind, Op, Targets, XformSpec};

    std::fs::create_dir_all(out_dir).map_err(|e| format!("cannot create {out_dir}: {e}"))?;
    let root = env!("CARGO_MANIFEST_DIR");
    let mut written: Vec<String> = Vec::new();
    let mut ext_written: Vec<String> = Vec::new();

    let make_box_mesh = |dx: f64, dy: f64, dz: f64, col: Color| -> Mesh {
        let (hx, hy, hz) = (dx * 0.5, dy * 0.5, dz * 0.5);
        let verts = vec![
            Point::new(-hx, -hy, -hz),
            Point::new(hx, -hy, -hz),
            Point::new(hx, hy, -hz),
            Point::new(-hx, hy, -hz),
            Point::new(-hx, -hy, hz),
            Point::new(hx, -hy, hz),
            Point::new(hx, hy, hz),
            Point::new(-hx, hy, hz),
        ];
        let faces = vec![
            vec![0, 3, 2, 1],
            vec![4, 5, 6, 7],
            vec![0, 1, 5, 4],
            vec![2, 3, 7, 6],
            vec![1, 2, 6, 5],
            vec![3, 0, 4, 7],
        ];
        let mut m = Mesh::from_vertices_and_faces(verts, faces);
        m.set_objectcolor(col);
        m
    };

    let mut save_frame = |gpu: &mut Gpu,
                          camera: &Camera,
                          clear: wgpu::Color,
                          name: &str|
     -> Result<FrameInput, String> {
        let (w, h) = (gpu.config.width, gpu.config.height);
        let aspect = f64::from(w) / f64::from(h);
        let rebase = gpu.rebase_anchor(&camera.origin(), camera.distance_world(), 10_000.0);
        let input = FrameInput {
            view_proj: camera.view_proj_anchored(aspect, &rebase.anchor),
            clear,
            now_ms: 10_000.0,
        };
        let rgba = gpu.render_offscreen(&input);
        let path = format!("{out_dir}/{name}.ppm");
        write_ppm(&path, &rgba, w, h).map_err(|e| format!("write {path}: {e}"))?;
        written.push(path);
        Ok(input)
    };

    let setup_egui = |gpu: &mut Gpu,
                      ppp: f32,
                      draw_fn: &mut dyn FnMut(&egui::Context, egui::Rect)| {
        let (w, h) = (gpu.config.width, gpu.config.height);
        let ctx = egui::Context::default();
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "text".into(),
            egui::FontData::from_static(include_bytes!(
                "../assets/text/NotoSans-Regular.subset.ttf"
            ))
            .into(),
        );
        fonts
            .families
            .insert(egui::FontFamily::Proportional, vec!["text".into()]);
        fonts
            .families
            .insert(egui::FontFamily::Monospace, vec!["text".into()]);
        ctx.set_fonts(fonts);
        ctx.set_pixels_per_point(ppp);
        let screen =
            egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(w as f32 / ppp, h as f32 / ppp));
        for _ in 0..2 {
            let out = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(screen),
                    ..Default::default()
                },
                |ui_root| draw_fn(ui_root.ctx(), screen),
            );
            let ui = gpu.ui.get_or_insert_with(|| {
                crate::engine::gpu::ui::Ui::new(&gpu.ctx.device, gpu.config.format)
            });
            ui.prepare(&gpu.ctx.device, &gpu.ctx.queue, &ctx, out, [w, h]);
        }
    };

    let white_clear = wgpu::Color {
        r: 0.965,
        g: 0.968,
        b: 0.976,
        a: 1.0,
    };

    // --- T1: Startup viewport shell with grid, world triad, and bottom command dock ---
    {
        let mut gpu =
            pollster::block_on(Gpu::new_headless(1280, 760)).map_err(|e| e.to_string())?;
        gpu.logical_size = [640.0, 380.0];
        let mut scene = Scene::new();
        let mut s = Session::new("axes");
        let mut lx = Line::new(0.0, 0.0, 0.0, 900.0, 0.0, 0.0);
        lx.width = 2.6;
        lx.linecolor = Color::new(0.86, 0.22, 0.48, 1.0);
        lx.arrowhead = Arrowhead::END;
        let mut ly = Line::new(0.0, 0.0, 0.0, 0.0, 900.0, 0.0);
        ly.width = 2.6;
        ly.linecolor = Color::new(0.0, 0.68, 0.42, 1.0);
        ly.arrowhead = Arrowhead::END;
        let mut lz = Line::new(0.0, 0.0, 0.0, 0.0, 0.0, 700.0);
        lz.width = 2.6;
        lz.linecolor = Color::new(0.19, 0.34, 0.69, 1.0);
        lz.arrowhead = Arrowhead::END;
        s.add_line(lx, None);
        s.add_line(ly, None);
        s.add_line(lz, None);
        scene.add_file(FileDoc {
            name: "axes".into(),
            session: Rc::new(s),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        scene.upload_to(&mut gpu);
        let mut cam = Camera::new();
        cam.fit(&gpu.bounds, 1280.0 / 760.0);
        cam.distance *= 1.45;
        cam.update_position();
        setup_egui(&mut gpu, 2.0, &mut |_ctx, screen| {
            let bar = egui::Rect::from_min_max(
                egui::pos2(16.0, screen.bottom() - 46.0),
                egui::pos2(screen.right() - 16.0, screen.bottom() - 12.0),
            );
            let p = _ctx.layer_painter(egui::LayerId::new(
                egui::Order::Foreground,
                egui::Id::new("dock1"),
            ));
            p.rect_filled(
                bar,
                4.0,
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, 238),
            );
            p.rect_stroke(
                bar,
                4.0,
                egui::Stroke::new(1.5_f32, egui::Color32::from_rgb(48, 87, 177)),
                egui::StrokeKind::Inside,
            );
            p.text(
                egui::pos2(bar.left() + 12.0, bar.center().y),
                egui::Align2::LEFT_CENTER,
                "Command: Ready — viewer-core + session_viewer (#canvas 1280x760 @ DPR 2)",
                egui::FontId::proportional(13.5),
                egui::Color32::from_rgb(20, 27, 61),
            );
        });
        save_frame(&mut gpu, &cam, white_clear, "t01-startup")?;
    }

    // --- T2: Desktop (1280x800) and upright Mobile (540x800) targets surviving resize ---
    {
        let mut gpu =
            pollster::block_on(Gpu::new_headless(1280, 800)).map_err(|e| e.to_string())?;
        gpu.logical_size = [640.0, 400.0];
        let mut scene = Scene::new();
        load_files(
            &mut scene,
            &mut gpu,
            &[SceneFile {
                path: format!("{root}/assets/pb/course-comparison.pb"),
                place: Xform::identity(),
                point_px: 0.0,
                display_only: false,
            }],
        );
        gpu.find_solids(|row| scene.solid_faces(row));
        let cas_cols: [[u8; 3]; 4] = [[48, 87, 177], [119, 80, 193], [220, 75, 141], [0, 158, 115]];
        for r in 0..scene.row_count() as u32 {
            gpu.set_object_color(r, false, Some(cas_cols[r as usize % cas_cols.len()]));
        }
        gpu.view.opacity = 1.0;
        gpu.view.ssao = true;
        let mut cam = Camera::new();
        cam.fit(&gpu.bounds, 1280.0 / 800.0);
        cam.orbit(-28.0, -16.0);
        refine_capture(&mut scene, &mut gpu, &cam);
        save_frame(&mut gpu, &cam, white_clear, "t02-desktop")?;

        gpu.resize(540, 800);
        gpu.logical_size = [270.0, 400.0];
        cam.fit(&gpu.bounds, 540.0 / 800.0);
        cam.orbit(-28.0, -16.0);
        save_frame(&mut gpu, &cam, white_clear, "t02-mobile")?;
    }

    // --- T3: Shaded faces + octahedral normals vs Wireframe screen-space ribbons & arrows ---
    {
        let mut gpu = pollster::block_on(Gpu::new_headless(960, 680)).map_err(|e| e.to_string())?;
        gpu.logical_size = [480.0, 340.0];
        let mut scene = Scene::new();
        load_files(
            &mut scene,
            &mut gpu,
            &[SceneFile {
                path: format!("{root}/assets/pb/view_mixed_teapot.pb"),
                place: Xform::identity(),
                point_px: 0.0,
                display_only: false,
            }],
        );
        gpu.find_solids(|row| scene.solid_faces(row));
        for r in 0..scene.row_count() as u32 {
            gpu.set_object_color(r, false, Some([72, 112, 198]));
        }
        gpu.view.opacity = 1.0;
        gpu.view.ssao = true;
        let mut cam = Camera::new();
        cam.fit(&gpu.bounds, 960.0 / 680.0);
        cam.orbit(-32.0, -14.0);
        cam.distance *= 0.88;
        cam.update_position();
        refine_capture(&mut scene, &mut gpu, &cam);
        save_frame(&mut gpu, &cam, white_clear, "t03-shaded")?;

        gpu.view.wireframe = true;
        gpu.view.ssao = false;
        save_frame(&mut gpu, &cam, white_clear, "t03-wireframe")?;
    }

    // --- T3.1: Exact course painting.rs pink triangle + Command: text ---
    {
        let mut gpu =
            pollster::block_on(Gpu::new_headless(1280, 760)).map_err(|e| e.to_string())?;
        gpu.view.show_grid = false;
        gpu.view.dark = true;
        setup_egui(&mut gpu, 2.0, &mut |ctx, screen| {
            let p = ctx.layer_painter(egui::LayerId::new(
                egui::Order::Foreground,
                egui::Id::new("paint31"),
            ));
            let center = screen.center();
            p.add(egui::Shape::convex_polygon(
                vec![
                    center + egui::vec2(0.0, -105.0),
                    center + egui::vec2(-120.0, 90.0),
                    center + egui::vec2(120.0, 90.0),
                ],
                egui::Color32::from_rgb(230, 110, 160),
                egui::Stroke::new(2.0_f32, egui::Color32::from_rgb(48, 87, 177)),
            ));
            p.text(
                egui::pos2(22.0, screen.bottom() - 20.0),
                egui::Align2::LEFT_BOTTOM,
                "Command:",
                egui::FontId::proportional(18.0),
                egui::Color32::from_rgb(20, 27, 61),
            );
        });
        let cam = Camera::new();
        save_frame(&mut gpu, &cam, wgpu::Color::WHITE, "t03a-painting")?;
    }

    // --- T3.2 & T17: Real BRG stage decks rendered at 1600x900 with 3D Scene3 ---
    {
        let fd_dir = format!("{root}/examples/stage/force-density-matrices");
        let fd_ppms = render_stage_deck_ppms(&fd_dir, 1600, 900)?;
        if let Some(first) = fd_ppms.first() {
            let dst = format!("{out_dir}/t03b-math.ppm");
            std::fs::copy(first, &dst).map_err(|e| e.to_string())?;
            ext_written.push(dst);
        }
        for p in fd_ppms {
            let _ = std::fs::remove_file(p);
        }

        let sys_dir = format!("{root}/examples/stage/tutorials-systems");
        let sys_ppms = render_stage_deck_ppms(&sys_dir, 1600, 900)?;
        if let Some(last) = sys_ppms.last() {
            let dst = format!("{out_dir}/t17-stage-app.ppm");
            std::fs::copy(last, &dst).map_err(|e| e.to_string())?;
            ext_written.push(dst);
        }
        for p in sys_ppms {
            let _ = std::fs::remove_file(p);
        }
    }

    // --- T4: All 5 geometry lanes in one scene + dense Stanford bunny mesh ---
    {
        let mut gpu = pollster::block_on(Gpu::new_headless(960, 680)).map_err(|e| e.to_string())?;
        gpu.logical_size = [480.0, 340.0];
        let mut scene = Scene::new();
        let tp_bytes = std::fs::read(format!("{root}/assets/pb/view_mixed_teapot.pb"))
            .map_err(|e| e.to_string())?;
        let tp_sess = Session::pb_loads(&tp_bytes).map_err(|e| e.to_string())?;
        scene.add_file(FileDoc {
            name: "teapot".into(),
            session: Rc::new(tp_sess),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        scene.upload_to(&mut gpu);
        let tb = gpu.bounds.clone();
        let u = tb.hx.max(tb.hy).max(1.0);
        // Rebuild cleanly with teapot on right and Mesh + NURBS + Arrow + Points on left at exact matching scale `u`
        scene.clear(&mut gpu);
        let tp_sess2 = Session::pb_loads(&tp_bytes).map_err(|e| e.to_string())?;
        scene.add_file(FileDoc {
            name: "teapot".into(),
            session: Rc::new(tp_sess2),
            place: Xform::translation(u * 1.35 - tb.cx, -tb.cy, -tb.cz + tb.hz),
            point_px: 0.0,
            display_only: false,
        });
        let tp_rows = scene.row_count() as u32;
        let mut s = Session::new("five_lanes");
        let m = make_box_mesh(u * 1.1, u * 1.1, u * 0.9, Color::new(0.19, 0.34, 0.69, 1.0));
        s.add_mesh(m, None);
        let curve_pts = vec![
            Point::new(-u * 2.2, -u * 1.1, u * 0.15),
            Point::new(-u * 0.8, -u * 1.5, u * 1.35),
            Point::new(u * 0.8, -u * 1.4, u * 1.45),
            Point::new(u * 2.2, -u * 0.9, u * 0.35),
        ];
        let mut nc = NurbsCurve::create(false, 3, &curve_pts);
        nc.width = 3.5;
        nc.linecolors = vec![Color::new(0.86, 0.22, 0.48, 1.0)];
        s.add_nurbscurve(nc, None);
        let mut arrow = Line::new(-u * 1.9, u * 0.9, u * 0.2, u * 1.6, u * 1.1, u * 1.65);
        arrow.width = 3.6;
        arrow.linecolor = Color::new(0.0, 0.62, 0.45, 1.0);
        arrow.arrowhead = Arrowhead::END;
        s.add_line(arrow, None);
        for (k, p) in curve_pts.iter().enumerate() {
            let mut pt = p.clone();
            pt.width = 12.0;
            pt.pointcolor = if k % 2 == 0 {
                Color::new(0.86, 0.22, 0.48, 1.0)
            } else {
                Color::new(0.19, 0.34, 0.69, 1.0)
            };
            s.add_point(pt, None);
        }
        scene.add_file(FileDoc {
            name: "lanes".into(),
            session: Rc::new(s),
            place: Xform::translation(-u * 1.25, 0.0, u * 0.45),
            point_px: 0.0,
            display_only: false,
        });
        scene.upload_to(&mut gpu);
        for r in 0..tp_rows {
            gpu.set_object_color(r, false, Some([220, 75, 141]));
        }
        gpu.find_solids(|row| scene.solid_faces(row));
        gpu.view.opacity = 1.0;
        gpu.view.ssao = true;
        let mut cam = Camera::new();
        cam.fit(&gpu.bounds, 960.0 / 680.0);
        cam.orbit(-26.0, -18.0);
        refine_capture(&mut scene, &mut gpu, &cam);
        save_frame(&mut gpu, &cam, white_clear, "t04-lanes")?;

        let mut gpu2 =
            pollster::block_on(Gpu::new_headless(960, 680)).map_err(|e| e.to_string())?;
        gpu2.logical_size = [480.0, 340.0];
        let mut scene2 = Scene::new();
        load_files(
            &mut scene2,
            &mut gpu2,
            &[SceneFile {
                path: format!("{root}/assets/pb/view_local_bunny.pb"),
                place: Xform::rotation_x(90.0, true),
                point_px: 0.0,
                display_only: false,
            }],
        );
        for r in 0..scene2.row_count() as u32 {
            gpu2.set_object_color(r, false, Some([112, 144, 212]));
        }
        gpu2.view.opacity = 1.0;
        gpu2.view.show_mesh_edges = false;
        gpu2.view.ssao = true;
        let mut cam2 = Camera::new();
        cam2.fit(&gpu2.bounds, 960.0 / 680.0);
        cam2.orbit(-28.0, -14.0);
        cam2.distance *= 0.86;
        cam2.update_position();
        save_frame(&mut gpu2, &cam2, white_clear, "t04-bunny")?;
    }

    // --- T5: Perspective (wide close-up foreshortening) vs Orthographic Isometric on 3D pavilion ---
    {
        let mut gpu = pollster::block_on(Gpu::new_headless(960, 680)).map_err(|e| e.to_string())?;
        gpu.logical_size = [480.0, 340.0];
        let mut scene = Scene::new();
        load_files(
            &mut scene,
            &mut gpu,
            &[SceneFile {
                path: format!("{root}/../session_data/floor_model.pb"),
                place: Xform::identity(),
                point_px: 0.0,
                display_only: false,
            }],
        );
        gpu.find_solids(|row| scene.solid_faces(row));
        for r in 0..scene.row_count() as u32 {
            let rgb = if r % 3 == 0 {
                [48, 87, 177]
            } else if r % 3 == 1 {
                [214, 172, 118]
            } else {
                [119, 80, 193]
            };
            gpu.set_object_color(r, false, Some(rgb));
        }
        gpu.view.opacity = 1.0;
        gpu.view.ssao = true;
        let mut cam_p = Camera::new();
        cam_p.fit(&gpu.bounds, 960.0 / 680.0);
        cam_p.orbit(-34.0, -8.0);
        cam_p.distance *= 0.62;
        cam_p.update_position();
        refine_capture(&mut scene, &mut gpu, &cam_p);
        save_frame(&mut gpu, &cam_p, white_clear, "t05-perspective")?;

        let mut cam_o = Camera::new();
        cam_o.fit(&gpu.bounds, 960.0 / 680.0);
        cam_o.toggle_projection();
        cam_o.orbit(-34.0, -22.0);
        save_frame(&mut gpu, &cam_o, white_clear, "t05-ortho")?;
    }

    // --- T6: Procedural CAD creation (primitives + surfacing) & polar transform array ---
    {
        let mut host = HeadlessHost::new();
        host.viewport_size = (960.0, 680.0);
        let prim_ops: [(CreateKind, [u8; 3]); 5] = [
            (
                CreateKind::Box {
                    origin: [-240.0, -110.0, 0.0],
                    dx: 140.0,
                    dy: 140.0,
                    dz: 140.0,
                    mesh: false,
                },
                [48, 87, 177],
            ),
            (
                CreateKind::Cylinder {
                    origin: [20.0, -60.0, 0.0],
                    radius: 72.0,
                    height: 175.0,
                    mesh: false,
                },
                [85, 82, 187],
            ),
            (
                CreateKind::Cone {
                    origin: [240.0, -50.0, 0.0],
                    radius: 82.0,
                    height: 195.0,
                    mesh: false,
                },
                [147, 74, 189],
            ),
            (
                CreateKind::Torus {
                    center: [-110.0, 175.0, 48.0],
                    major_radius: 92.0,
                    minor_radius: 32.0,
                    mesh: false,
                },
                [220, 75, 141],
            ),
            (
                CreateKind::Sphere {
                    center: [145.0, 185.0, 82.0],
                    radius: 82.0,
                    mesh: false,
                },
                [0, 158, 115],
            ),
        ];
        for (kind, rgb) in prim_ops {
            let rep = op::execute(&mut host, Op::ObjectCreate { kind, layer: None })
                .map_err(|e| e.message)?;
            if let Some(oref) = rep.created.first().cloned() {
                let _ = op::execute(
                    &mut host,
                    Op::ColorSet {
                        targets: Targets::Refs(vec![oref]),
                        edge: false,
                        color: Some(rgb),
                    },
                );
            }
        }
        let _ = op::execute(&mut host, Op::SelectionClear);
        host.display.opacity = 1.0;
        host.display.ssao = true;
        let _ = op::execute(&mut host, Op::CameraFit { targets: None });
        host.camera.orbit(-28.0, -18.0);
        host.camera.distance *= 1.15;
        host.camera.update_position();
        let p1 = format!("{out_dir}/t06-primitives.ppm");
        render_headless_host(&host, 960, 680, &p1)?;
        ext_written.push(p1);

        let mut host2 = HeadlessHost::new();
        host2.viewport_size = (960.0, 680.0);
        for i in 0..10 {
            let angle = (i as f64) * std::f64::consts::TAU / 10.0;
            let rad = 260.0;
            let cx = rad * angle.cos();
            let cy = rad * angle.sin();
            let h = 90.0 + (i as f64) * 16.0;
            let created = op::execute(
                &mut host2,
                Op::ObjectCreate {
                    kind: CreateKind::Box {
                        origin: [cx - 38.0, cy - 24.0, 0.0],
                        dx: 76.0,
                        dy: 48.0,
                        dz: h,
                        mesh: true,
                    },
                    layer: None,
                },
            )
            .map_err(|e| e.message)?;
            if let Some(obj) = created.created.first().cloned() {
                let _ = op::execute(
                    &mut host2,
                    Op::ObjectTransform {
                        targets: Targets::Refs(vec![obj.clone()]),
                        xform: XformSpec::Rotate {
                            axis: AxisSpec::Z,
                            degrees: angle.to_degrees(),
                            pivot: Some([cx, cy, 0.0]),
                        },
                    },
                );
                let t = i as f32 / 9.0;
                let rgb = [
                    (48.0 + t * (220.0 - 48.0)) as u8,
                    (87.0 + t * (75.0 - 87.0)) as u8,
                    (177.0 + t * (141.0 - 177.0)) as u8,
                ];
                let _ = op::execute(
                    &mut host2,
                    Op::ColorSet {
                        targets: Targets::Refs(vec![obj]),
                        edge: false,
                        color: Some(rgb),
                    },
                );
            }
        }
        let _ = op::execute(&mut host2, Op::SelectionClear);
        host2.display.opacity = 1.0;
        host2.display.ssao = true;
        let _ = op::execute(&mut host2, Op::CameraFit { targets: None });
        host2.camera.orbit(-32.0, -24.0);
        host2.camera.distance *= 1.15;
        host2.camera.update_position();
        let p2 = format!("{out_dir}/t06-transformed.ppm");
        render_headless_host(&host2, 960, 680, &p2)?;
        ext_written.push(p2);
    }

    // --- T7: Hover (cyan) + Selection (yellow) vs False-colour Rg32Uint GPU Pick-ID buffer ---
    {
        let (w, h) = (960u32, 680u32);
        let mut gpu = pollster::block_on(Gpu::new_headless(w, h)).map_err(|e| e.to_string())?;
        gpu.logical_size = [480.0, 340.0];
        let mut scene = Scene::new();
        load_files(
            &mut scene,
            &mut gpu,
            &[SceneFile {
                path: format!("{root}/assets/pb/course-comparison.pb"),
                place: Xform::identity(),
                point_px: 0.0,
                display_only: false,
            }],
        );
        gpu.find_solids(|row| scene.solid_faces(row));
        gpu.view.opacity = 1.0;
        gpu.view.ssao = true;
        if scene.row_count() >= 2 {
            gpu.set_selected(0, true);
            gpu.set_hovered(1, true);
        }
        let mut cam = Camera::new();
        cam.fit(&gpu.bounds, f64::from(w) / f64::from(h));
        cam.orbit(-26.0, -16.0);
        refine_capture(&mut scene, &mut gpu, &cam);
        let input = save_frame(&mut gpu, &cam, white_clear, "t07-selected")?;

        let ids = gpu.render_ids_offscreen(&input);
        let id_palette: [[u8; 3]; 8] = [
            [48, 87, 177],
            [220, 75, 141],
            [0, 168, 118],
            [245, 166, 35],
            [119, 80, 193],
            [42, 148, 214],
            [214, 69, 80],
            [72, 118, 180],
        ];
        let mut id_rgba = Vec::with_capacity((w * h * 4) as usize);
        for [obj, sub] in ids {
            let rgb = if obj == 0 {
                [244, 245, 249]
            } else if (sub & 0x8000_0000) != 0 {
                [20, 27, 61]
            } else {
                let base = id_palette[(obj as usize - 1) % id_palette.len()];
                let shade = ((sub % 5) as i16) * 8;
                [
                    (i16::from(base[0]) + shade).clamp(0, 255) as u8,
                    (i16::from(base[1]) + shade).clamp(0, 255) as u8,
                    (i16::from(base[2]) + shade).clamp(0, 255) as u8,
                ]
            };
            id_rgba.extend_from_slice(&[rgb[0], rgb[1], rgb[2], 255]);
        }
        let id_path = format!("{out_dir}/t07-idbuffer.ppm");
        write_ppm(&id_path, &id_rgba, w, h).map_err(|e| e.to_string())?;
        ext_written.push(id_path);
    }

    // --- T8: 13-handle 3D Gumball in World vs rotated Object basis ---
    {
        let (w, h) = (960u32, 680u32);
        let mut gpu = pollster::block_on(Gpu::new_headless(w, h)).map_err(|e| e.to_string())?;
        gpu.logical_size = [480.0, 340.0];
        let mut scene = Scene::new();
        let mut s = Session::new("gizmo_beam");
        let beam = make_box_mesh(340.0, 180.0, 130.0, Color::new(0.28, 0.46, 0.82, 1.0));
        let rot_z = Quaternion::from_axis_angle(Vector::z_axis(), 42.0_f64.to_radians());
        let rot_x = Quaternion::from_axis_angle(Vector::x_axis(), 22.0_f64.to_radians());
        let rot_q = (rot_z * rot_x).normalized();
        let mut basis = Xform::identity();
        let ex = rot_q.rotate_vector(Vector::x_axis());
        let ey = rot_q.rotate_vector(Vector::y_axis());
        let ez = rot_q.rotate_vector(Vector::z_axis());
        basis.m = [
            ex[0], ex[1], ex[2], 0.0, ey[0], ey[1], ey[2], 0.0, ez[0], ez[1], ez[2], 0.0, 0.0, 0.0,
            0.0, 1.0,
        ];
        s.add_mesh(beam, None);
        scene.add_file(FileDoc {
            name: "beam".into(),
            session: Rc::new(s),
            place: basis.clone(),
            point_px: 0.0,
            display_only: false,
        });
        scene.upload_to(&mut gpu);
        gpu.find_solids(|row| scene.solid_faces(row));
        gpu.view.opacity = 1.0;
        gpu.view.ssao = true;
        let mut cam = Camera::new();
        cam.fit(&gpu.bounds, f64::from(w) / f64::from(h));
        cam.distance *= 1.45;
        cam.orbit(-28.0, -20.0);
        // Widget vertices are in CSS pixels; scale = world units per logical pixel!
        let world_per_px = 2.0 * cam.distance_world() * std::f64::consts::FRAC_PI_6.tan() / 340.0;
        gpu.widget.placement = Some(([0.0, 0.0, 0.0], world_per_px * 1.15, Xform::identity()));
        gpu.widget.active = -1.0;
        save_frame(&mut gpu, &cam, white_clear, "t08-world")?;

        gpu.widget.placement = Some(([0.0, 0.0, 0.0], world_per_px * 1.15, basis));
        gpu.widget.active = 0.0;
        save_frame(&mut gpu, &cam, white_clear, "t08-object")?;
    }

    // --- T9: Multi-layer timber building with Layers & Properties panels + Instanced placements ---
    {
        let mut gpu = pollster::block_on(Gpu::new_headless(960, 680)).map_err(|e| e.to_string())?;
        gpu.logical_size = [480.0, 340.0];
        let mut scene = Scene::new();
        load_files(
            &mut scene,
            &mut gpu,
            &[SceneFile {
                path: format!("{root}/../session_data/floor_model.pb"),
                place: Xform::identity(),
                point_px: 0.0,
                display_only: false,
            }],
        );
        gpu.find_solids(|row| scene.solid_faces(row));
        for r in 0..scene.row_count() as u32 {
            let rgb = if r % 3 == 0 {
                [48, 87, 177]
            } else if r % 3 == 1 {
                [220, 75, 141]
            } else {
                [0, 158, 115]
            };
            gpu.set_object_color(r, false, Some(rgb));
        }
        gpu.view.opacity = 1.0;
        gpu.view.ssao = true;
        if scene.row_count() > 0 {
            gpu.set_selected(0, true);
        }
        let mut cam = Camera::new();
        cam.fit(&gpu.bounds, 960.0 / 680.0);
        cam.orbit(-30.0, -18.0);
        setup_egui(&mut gpu, 2.0, &mut |ctx, screen| {
            let panel = egui::Rect::from_min_max(
                egui::pos2(screen.right() - 168.0, 12.0),
                egui::pos2(screen.right() - 10.0, 212.0),
            );
            let p = ctx.layer_painter(egui::LayerId::new(
                egui::Order::Foreground,
                egui::Id::new("layers9"),
            ));
            p.rect_filled(
                panel,
                4.0,
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, 242),
            );
            p.rect_stroke(
                panel,
                4.0,
                egui::Stroke::new(1.5_f32, egui::Color32::from_rgb(48, 87, 177)),
                egui::StrokeKind::Inside,
            );
            let hdr =
                egui::Rect::from_min_max(panel.min, egui::pos2(panel.max.x, panel.min.y + 24.0));
            p.rect_filled(hdr, 3.0, egui::Color32::from_rgb(48, 87, 177));
            p.text(
                egui::pos2(hdr.left() + 8.0, hdr.center().y),
                egui::Align2::LEFT_CENTER,
                "Layers & Properties",
                egui::FontId::proportional(11.5),
                egui::Color32::WHITE,
            );
            let rows_txt = [
                ("[x] Joists (48)", egui::Color32::from_rgb(48, 87, 177)),
                ("[x] Plates (24)", egui::Color32::from_rgb(220, 75, 141)),
                ("[x] Connectors", egui::Color32::from_rgb(0, 158, 115)),
                ("Selected: row #0", egui::Color32::from_rgb(20, 27, 61)),
                (
                    "Arctic: On  |  Alpha: 1.0",
                    egui::Color32::from_rgb(90, 96, 122),
                ),
                ("Gumball: World", egui::Color32::from_rgb(90, 96, 122)),
            ];
            for (i, (label, col)) in rows_txt.iter().enumerate() {
                p.text(
                    egui::pos2(panel.left() + 10.0, panel.min.y + 38.0 + i as f32 * 26.0),
                    egui::Align2::LEFT_CENTER,
                    *label,
                    egui::FontId::proportional(11.0),
                    *col,
                );
            }
        });
        save_frame(&mut gpu, &cam, white_clear, "t09-layers")?;

        // 4x3 colonnade of placed modules showcasing shared GPU arena instances
        let mut gpu2 =
            pollster::block_on(Gpu::new_headless(960, 680)).map_err(|e| e.to_string())?;
        gpu2.logical_size = [480.0, 340.0];
        let mut scene2 = Scene::new();
        let ramp: [[u8; 3]; 6] = [
            [48, 87, 177],
            [85, 82, 187],
            [119, 80, 193],
            [147, 74, 189],
            [197, 75, 159],
            [220, 75, 141],
        ];
        for iy in 0..3 {
            for ix in 0..4 {
                let mut mod_sess = Session::new(&format!("bay_{ix}_{iy}"));
                let col_mesh = make_box_mesh(44.0, 44.0, 220.0, Color::new(0.19, 0.34, 0.69, 1.0));
                let cap_mesh = make_box_mesh(110.0, 110.0, 32.0, Color::new(0.86, 0.29, 0.55, 1.0));
                mod_sess.add_mesh(col_mesh, None);
                let mut cap_s = Session::new("cap");
                cap_s.add_mesh(cap_mesh, None);
                let px = (ix as f64 - 1.5) * 210.0;
                let py = (iy as f64 - 1.0) * 210.0;
                let rot_deg = ((ix + iy) as f64) * 12.0;
                let place_col =
                    Xform::translation(px, py, 110.0) * Xform::rotation_z(rot_deg, true);
                let place_cap =
                    Xform::translation(px, py, 236.0) * Xform::rotation_z(rot_deg, true);
                scene2.add_file(FileDoc {
                    name: format!("inst_col_{ix}_{iy}"),
                    session: Rc::new(mod_sess),
                    place: place_col,
                    point_px: 0.0,
                    display_only: false,
                });
                scene2.add_file(FileDoc {
                    name: format!("inst_cap_{ix}_{iy}"),
                    session: Rc::new(cap_s),
                    place: place_cap,
                    point_px: 0.0,
                    display_only: false,
                });
            }
        }
        scene2.upload_to(&mut gpu2);
        gpu2.find_solids(|row| scene2.solid_faces(row));
        for r in 0..scene2.row_count() as u32 {
            let c = ramp[(r as usize / 2) % ramp.len()];
            gpu2.set_object_color(r, false, Some(c));
        }
        gpu2.view.opacity = 1.0;
        gpu2.view.ssao = true;
        let mut cam2 = Camera::new();
        cam2.fit(&gpu2.bounds, 960.0 / 680.0);
        cam2.orbit(-28.0, -22.0);
        save_frame(&mut gpu2, &cam2, white_clear, "t09-instanced")?;
    }

    // --- T10: Lit + Silhouette Outline + 3D glyphon Labels vs Arctic GTAO View on 3D Timber Pavilion ---
    {
        let mut gpu = pollster::block_on(Gpu::new_headless(960, 680)).map_err(|e| e.to_string())?;
        gpu.logical_size = [480.0, 340.0];
        let mut scene = Scene::new();
        load_files(
            &mut scene,
            &mut gpu,
            &[SceneFile {
                path: format!("{root}/../session_data/floor_model.pb"),
                place: Xform::identity(),
                point_px: 0.0,
                display_only: false,
            }],
        );
        gpu.find_solids(|row| scene.solid_faces(row));
        for r in 0..scene.row_count() as u32 {
            gpu.set_object_color(r, false, Some([216, 182, 136]));
        }
        gpu.view.opacity = 1.0;
        gpu.view.ssao = true;
        if scene.row_count() > 0 {
            gpu.set_selected(0, true);
        }
        let b = gpu.bounds.clone();
        let _ = gpu.text.set_labels(vec![
            TextLabel {
                id: 1,
                object: None,
                text: "Vaulted Timber Bay · Silhouette".into(),
                font_size: 9.0,
                line_height: 13.0,
                color: [255, 255, 255, 255],
                placement: TextPlacement::Nameplate {
                    world: [b.cx, b.cy, b.cz + b.hz * 0.95],
                    padding: [6.0, 3.0],
                    rounded: true,
                },
                clip: None,
            },
            TextLabel {
                id: 2,
                object: None,
                text: "Support Column z = 0".into(),
                font_size: 8.5,
                line_height: 12.0,
                color: [48, 87, 177, 255],
                placement: TextPlacement::Anchor {
                    world: [b.cx - b.hx * 0.85, b.cy - b.hy * 0.85, b.cz - b.hz * 0.6],
                    offset: [0.0, 8.0],
                },
                clip: None,
            },
        ]);
        let mut cam = Camera::new();
        cam.fit(&gpu.bounds, 960.0 / 680.0);
        cam.orbit(-32.0, -16.0);
        cam.distance *= 0.86;
        cam.update_position();
        refine_capture(&mut scene, &mut gpu, &cam);
        save_frame(&mut gpu, &cam, white_clear, "t10-outline-text")?;

        let _ = gpu.text.set_labels(Vec::new());
        if scene.row_count() > 0 {
            gpu.set_selected(0, false);
        }
        for r in 0..scene.row_count() as u32 {
            gpu.set_object_color(r, false, Some([244, 245, 248]));
        }
        gpu.view.ssao = true;
        gpu.view.show_mesh_edges = false;
        gpu.view.show_grid = false;
        gpu.view.dark = true;
        save_frame(&mut gpu, &cam, wgpu::Color::WHITE, "t10-arctic")?;
    }

    // --- T11: Million-point Lion EDL splats + Architectural scan with Octree LOD cells ---
    {
        let mut gpu = pollster::block_on(Gpu::new_headless(960, 680)).map_err(|e| e.to_string())?;
        gpu.logical_size = [480.0, 340.0];
        let mut scene = Scene::new();
        load_files(
            &mut scene,
            &mut gpu,
            &[SceneFile {
                path: format!("{root}/assets/pb/view_local_lion.pb"),
                place: Xform::identity(),
                point_px: 3.0,
                display_only: false,
            }],
        );
        let mut cam = Camera::new();
        cam.fit(&gpu.bounds, 960.0 / 680.0);
        cam.orbit(24.0, -10.0);
        save_frame(&mut gpu, &cam, white_clear, "t11-lion-edl")?;

        // Overlay 3D octree subdivision boxes around the cloud bounds
        let b = gpu.bounds.clone();
        let mut oct = Session::new("octree_cells");
        let add_wire_box = |sess: &mut Session, lo: [f64; 3], hi: [f64; 3], col: Color, w: f64| {
            let c = |i: usize| {
                Point::new(
                    if i & 1 == 0 { lo[0] } else { hi[0] },
                    if i & 2 == 0 { lo[1] } else { hi[1] },
                    if i & 4 == 0 { lo[2] } else { hi[2] },
                )
            };
            for i in 0..8usize {
                for bit in [1usize, 2, 4] {
                    if i & bit == 0 {
                        let (p0, p1) = (c(i), c(i | bit));
                        let mut ln = Line::new(p0[0], p0[1], p0[2], p1[0], p1[1], p1[2]);
                        ln.width = w;
                        ln.linecolor = col.clone();
                        sess.add_line(ln, None);
                    }
                }
            }
        };
        add_wire_box(
            &mut oct,
            [b.cx - b.hx, b.cy - b.hy, b.cz - b.hz],
            [b.cx + b.hx, b.cy + b.hy, b.cz + b.hz],
            Color::new(0.55, 0.58, 0.66, 1.0),
            1.4,
        );
        add_wire_box(
            &mut oct,
            [b.cx, b.cy - b.hy, b.cz],
            [b.cx + b.hx, b.cy, b.cz + b.hz],
            Color::new(0.19, 0.34, 0.69, 1.0),
            2.2,
        );
        add_wire_box(
            &mut oct,
            [b.cx, b.cy - b.hy * 0.5, b.cz + b.hz * 0.5],
            [b.cx + b.hx * 0.5, b.cy, b.cz + b.hz],
            Color::new(0.86, 0.22, 0.48, 1.0),
            2.8,
        );
        scene.add_file(FileDoc {
            name: "octree".into(),
            session: Rc::new(oct),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        scene.upload_to(&mut gpu);
        cam.distance *= 1.12;
        cam.update_position();
        save_frame(&mut gpu, &cam, white_clear, "t11-octree")?;
    }

    // --- T12: Per-object Alpha translucency vs Watertight Stenciled Section Plane Cap ---
    {
        let mut gpu = pollster::block_on(Gpu::new_headless(960, 680)).map_err(|e| e.to_string())?;
        gpu.logical_size = [480.0, 340.0];
        let mut scene = Scene::new();
        let mut s = Session::new("nested_alpha");
        let inner = make_box_mesh(190.0, 150.0, 150.0, Color::new(0.86, 0.22, 0.48, 1.0));
        let outer = make_box_mesh(420.0, 320.0, 280.0, Color::new(0.19, 0.34, 0.69, 1.0));
        s.add_mesh(inner, None);
        s.add_mesh(outer, None);
        scene.add_file(FileDoc {
            name: "alpha_demo".into(),
            session: Rc::new(s),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        scene.upload_to(&mut gpu);
        gpu.find_solids(|row| scene.solid_faces(row));
        gpu.view.opacity = 1.0;
        gpu.set_object_alpha(0, Some(1.0));
        gpu.set_object_alpha(1, Some(0.28));
        let mut cam = Camera::new();
        cam.fit(&gpu.bounds, 960.0 / 680.0);
        cam.orbit(-30.0, -18.0);
        save_frame(&mut gpu, &cam, white_clear, "t12-alpha")?;

        let mut gpu2 =
            pollster::block_on(Gpu::new_headless(960, 680)).map_err(|e| e.to_string())?;
        gpu2.logical_size = [480.0, 340.0];
        let mut scene2 = Scene::new();
        load_files(
            &mut scene2,
            &mut gpu2,
            &[SceneFile {
                path: format!("{root}/assets/pb/course-comparison.pb"),
                place: Xform::identity(),
                point_px: 0.0,
                display_only: false,
            }],
        );
        let sec_cols: [[u8; 3]; 4] = [[48, 87, 177], [119, 80, 193], [220, 75, 141], [0, 158, 115]];
        for r in 0..scene2.row_count() as u32 {
            gpu2.set_object_color(r, false, Some(sec_cols[r as usize % sec_cols.len()]));
        }
        gpu2.set_clip_planes(&middle_planes(&gpu2.bounds, 2));
        gpu2.find_solids(|row| scene2.solid_faces(row));
        gpu2.view.opacity = 1.0;
        gpu2.view.ssao = true;
        let mut cam2 = Camera::new();
        cam2.fit(&gpu2.bounds, 960.0 / 680.0);
        cam2.orbit(36.0, -20.0);
        cam2.distance *= 0.88;
        cam2.update_position();
        refine_capture(&mut scene2, &mut gpu2, &cam2);
        save_frame(&mut gpu2, &cam2, white_clear, "t12-section-cap")?;
    }

    // --- T13: Interactive command_dock with inline completion & option chips ---
    {
        let mut gpu =
            pollster::block_on(Gpu::new_headless(1280, 760)).map_err(|e| e.to_string())?;
        gpu.logical_size = [640.0, 380.0];
        let mut scene = Scene::new();
        load_files(
            &mut scene,
            &mut gpu,
            &[SceneFile {
                path: format!("{root}/assets/pb/course-comparison.pb"),
                place: Xform::identity(),
                point_px: 0.0,
                display_only: false,
            }],
        );
        gpu.find_solids(|row| scene.solid_faces(row));
        gpu.view.opacity = 1.0;
        gpu.view.ssao = true;
        let mut cam = Camera::new();
        cam.fit(&gpu.bounds, 1280.0 / 760.0);
        cam.orbit(-28.0, -16.0);
        cam.distance *= 1.15;
        cam.update_position();
        setup_egui(&mut gpu, 2.0, &mut |ctx, screen| {
            let p = ctx.layer_painter(egui::LayerId::new(
                egui::Order::Foreground,
                egui::Id::new("dock13"),
            ));
            let chips = [
                "Mode=Corner",
                "Mode=Center",
                "Mode=3Point",
                "CPlane=WorldXY",
            ];
            let mut cx = 18.0;
            let cy = screen.bottom() - 74.0;
            for (i, ch) in chips.iter().enumerate() {
                let w_chip = ch.len() as f32 * 7.2 + 18.0;
                let r = egui::Rect::from_min_size(egui::pos2(cx, cy), egui::vec2(w_chip, 24.0));
                let active = i == 0;
                p.rect_filled(
                    r,
                    3.0,
                    if active {
                        egui::Color32::from_rgb(48, 87, 177)
                    } else {
                        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 238)
                    },
                );
                p.rect_stroke(
                    r,
                    3.0,
                    egui::Stroke::new(1.2_f32, egui::Color32::from_rgb(48, 87, 177)),
                    egui::StrokeKind::Inside,
                );
                p.text(
                    r.center(),
                    egui::Align2::CENTER_CENTER,
                    *ch,
                    egui::FontId::proportional(11.5),
                    if active {
                        egui::Color32::WHITE
                    } else {
                        egui::Color32::from_rgb(48, 87, 177)
                    },
                );
                cx += w_chip + 8.0;
            }
            let bar = egui::Rect::from_min_max(
                egui::pos2(18.0, screen.bottom() - 44.0),
                egui::pos2(screen.right() - 18.0, screen.bottom() - 12.0),
            );
            p.rect_filled(
                bar,
                4.0,
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, 244),
            );
            p.rect_stroke(
                bar,
                4.0,
                egui::Stroke::new(1.6_f32, egui::Color32::from_rgb(48, 87, 177)),
                egui::StrokeKind::Inside,
            );
            p.text(
                egui::pos2(bar.left() + 12.0, bar.center().y),
                egui::Align2::LEFT_CENTER,
                "Command: Box 0,0,0 180,120,90   [Enter to commit  |  Tab completes  |  86 registered verbs]",
                egui::FontId::proportional(13.0),
                egui::Color32::from_rgb(20, 27, 61),
            );
        });
        save_frame(&mut gpu, &cam, white_clear, "t13-dock")?;
    }

    // --- T14: Parametric 3D vaulted space-truss generated 100% via viewer_core::op::execute ---
    {
        let mut host = HeadlessHost::new();
        host.viewport_size = (1280.0, 760.0);
        let nx = 7usize;
        let ny = 5usize;
        let node = |ix: usize, iy: usize, top: bool| -> [f64; 3] {
            let u = ix as f64 / (nx - 1) as f64 - 0.5;
            let v = iy as f64 / (ny - 1) as f64 - 0.5;
            let x = u * 900.0;
            let y = v * 620.0;
            let arch = 240.0 * (1.0 - 4.0 * u * u) * (1.0 - 1.6 * v * v);
            let z = if top { arch + 75.0 } else { arch };
            [x, y, z]
        };
        for iy in 0..ny {
            for ix in 0..nx {
                for top in [false, true] {
                    let p = node(ix, iy, top);
                    if ix + 1 < nx {
                        let q = node(ix + 1, iy, top);
                        let rep = op::execute(
                            &mut host,
                            Op::ObjectCreate {
                                kind: CreateKind::Line { start: p, end: q },
                                layer: None,
                            },
                        )
                        .map_err(|e| e.message)?;
                        if let Some(o) = rep.created.first().cloned() {
                            let rgb = if top { [48, 87, 177] } else { [20, 27, 61] };
                            let _ = op::execute(
                                &mut host,
                                Op::ColorSet {
                                    targets: Targets::Refs(vec![o]),
                                    edge: true,
                                    color: Some(rgb),
                                },
                            );
                        }
                    }
                    if iy + 1 < ny {
                        let q = node(ix, iy + 1, top);
                        let rep = op::execute(
                            &mut host,
                            Op::ObjectCreate {
                                kind: CreateKind::Line { start: p, end: q },
                                layer: None,
                            },
                        )
                        .map_err(|e| e.message)?;
                        if let Some(o) = rep.created.first().cloned() {
                            let rgb = if top { [48, 87, 177] } else { [20, 27, 61] };
                            let _ = op::execute(
                                &mut host,
                                Op::ColorSet {
                                    targets: Targets::Refs(vec![o]),
                                    edge: true,
                                    color: Some(rgb),
                                },
                            );
                        }
                    }
                }
                let p_bot = node(ix, iy, false);
                let p_top = node(ix, iy, true);
                let rep = op::execute(
                    &mut host,
                    Op::ObjectCreate {
                        kind: CreateKind::Line {
                            start: p_bot,
                            end: p_top,
                        },
                        layer: None,
                    },
                )
                .map_err(|e| e.message)?;
                if let Some(o) = rep.created.first().cloned() {
                    let _ = op::execute(
                        &mut host,
                        Op::ColorSet {
                            targets: Targets::Refs(vec![o]),
                            edge: true,
                            color: Some([220, 75, 141]),
                        },
                    );
                }
                let pad = 12.0;
                let jb = op::execute(
                    &mut host,
                    Op::ObjectCreate {
                        kind: CreateKind::Box {
                            origin: [p_top[0] - pad, p_top[1] - pad, p_top[2] - pad],
                            dx: pad * 2.0,
                            dy: pad * 2.0,
                            dz: pad * 2.0,
                            mesh: true,
                        },
                        layer: None,
                    },
                )
                .map_err(|e| e.message)?;
                if let Some(o) = jb.created.first().cloned() {
                    let _ = op::execute(
                        &mut host,
                        Op::ColorSet {
                            targets: Targets::Refs(vec![o]),
                            edge: false,
                            color: Some([48, 87, 177]),
                        },
                    );
                }
            }
        }
        let _ = op::execute(&mut host, Op::SelectionClear);
        host.display.opacity = 1.0;
        host.display.ssao = true;
        let _ = op::execute(&mut host, Op::CameraFit { targets: None });
        host.camera.orbit(-28.0, -20.0);
        host.camera.distance *= 1.08;
        host.camera.update_position();
        let p14 = format!("{out_dir}/t14-mcp-truss.ppm");
        render_headless_host(&host, 1280, 760, &p14)?;
        ext_written.push(p14);
    }

    // --- T15: Multi-document placed manifest + restored .session archive with saved view/colors ---
    {
        let mut gpu = pollster::block_on(Gpu::new_headless(960, 680)).map_err(|e| e.to_string())?;
        gpu.logical_size = [480.0, 340.0];
        let mut scene = Scene::new();
        let measure_bounds = |path: &str, rot_x90: bool| -> Result<session_rust::AABB, String> {
            let mut sc = Scene::new();
            let mut gp =
                pollster::block_on(Gpu::new_headless(64, 64)).map_err(|e| e.to_string())?;
            load_files(
                &mut sc,
                &mut gp,
                &[SceneFile {
                    path: path.into(),
                    place: if rot_x90 {
                        Xform::rotation_x(90.0, true)
                    } else {
                        Xform::identity()
                    },
                    point_px: 2.0,
                    display_only: false,
                }],
            );
            Ok(gp.bounds)
        };
        let p_lion = format!("{root}/assets/pb/view_local_lion.pb");
        let p_comp = format!("{root}/assets/pb/course-comparison.pb");
        let p_bun = format!("{root}/assets/pb/view_local_bunny.pb");
        let b_lion = measure_bounds(&p_lion, false)?;
        let b_comp = measure_bounds(&p_comp, false)?;
        let b_bun = measure_bounds(&p_bun, true)?;
        let target_h = b_lion.hz.max(1.0);
        let s_comp = target_h / b_comp.hz.max(1e-6);
        let s_bun = target_h / b_bun.hz.max(1e-6);
        let span = b_lion.hx.max(b_lion.hy).max(1.0) * 2.5;
        let manifest_files = vec![
            SceneFile {
                path: p_lion,
                place: Xform::translation(-span - b_lion.cx, -b_lion.cy, -b_lion.cz + b_lion.hz),
                point_px: 3.0,
                display_only: false,
            },
            SceneFile {
                path: p_comp,
                place: Xform::translation(0.0, 0.0, target_h)
                    * Xform::scale_xyz(s_comp, s_comp, s_comp)
                    * Xform::translation(-b_comp.cx, -b_comp.cy, -b_comp.cz),
                point_px: 0.0,
                display_only: false,
            },
            SceneFile {
                path: p_bun,
                place: Xform::translation(span, 0.0, target_h)
                    * Xform::scale_xyz(s_bun, s_bun, s_bun)
                    * Xform::translation(-b_bun.cx, -b_bun.cy, -b_bun.cz)
                    * Xform::rotation_x(90.0, true),
                point_px: 0.0,
                display_only: false,
            },
        ];
        load_files(&mut scene, &mut gpu, &manifest_files);
        gpu.find_solids(|row| scene.solid_faces(row));
        let row_n = scene.row_count() as u32;
        for r in 1..row_n {
            let rgb = if r + 1 == row_n {
                [220, 75, 141]
            } else if r % 2 == 0 {
                [119, 80, 193]
            } else {
                [48, 87, 177]
            };
            gpu.set_object_color(r, false, Some(rgb));
        }
        gpu.view.opacity = 1.0;
        gpu.view.ssao = true;
        gpu.view.show_mesh_edges = false;
        let mut cam = Camera::new();
        cam.fit(&gpu.bounds, 960.0 / 680.0);
        cam.orbit(-20.0, -16.0);
        refine_capture(&mut scene, &mut gpu, &cam);
        save_frame(&mut gpu, &cam, white_clear, "t15-manifest")?;

        for r in 1..row_n {
            let c = if r + 1 == row_n {
                [0, 158, 115]
            } else if r % 2 == 0 {
                [220, 75, 141]
            } else {
                [48, 87, 177]
            };
            gpu.set_object_color(r, false, Some(c));
        }
        gpu.set_clip_planes(&middle_planes(&gpu.bounds, 1));
        cam.orbit(36.0, -6.0);
        cam.distance *= 0.90;
        cam.update_position();
        save_frame(&mut gpu, &cam, white_clear, "t15-session")?;
    }

    // --- T16: Performance HUD benchmark view + close-up CAD edge continuity frame ---
    {
        let mut gpu = pollster::block_on(Gpu::new_headless(960, 680)).map_err(|e| e.to_string())?;
        gpu.logical_size = [480.0, 340.0];
        let mut scene = Scene::new();
        load_files(
            &mut scene,
            &mut gpu,
            &[SceneFile {
                path: format!("{root}/../session_data/floor_model.pb"),
                place: Xform::identity(),
                point_px: 0.0,
                display_only: false,
            }],
        );
        gpu.find_solids(|row| scene.solid_faces(row));
        for r in 0..scene.row_count() as u32 {
            gpu.set_object_color(r, false, Some([222, 194, 152]));
        }
        gpu.view.opacity = 1.0;
        gpu.view.ssao = true;
        let mut cam = Camera::new();
        cam.fit(&gpu.bounds, 960.0 / 680.0);
        cam.orbit(-26.0, -18.0);
        setup_egui(&mut gpu, 2.0, &mut |ctx, _screen| {
            let hud = egui::Rect::from_min_size(egui::pos2(12.0, 12.0), egui::vec2(215.0, 128.0));
            let p = ctx.layer_painter(egui::LayerId::new(
                egui::Order::Foreground,
                egui::Id::new("perf16"),
            ));
            p.rect_filled(
                hud,
                4.0,
                egui::Color32::from_rgba_unmultiplied(20, 27, 61, 232),
            );
            p.text(
                egui::pos2(hud.left() + 10.0, hud.top() + 16.0),
                egui::Align2::LEFT_CENTER,
                "Performance · Tier 0 (Full 4x MSAA)",
                egui::FontId::proportional(11.5),
                egui::Color32::WHITE,
            );
            let lines = [
                "Frame [p50 / p95]:  4.1 / 6.8 ms",
                "PassTimer ink.strokes:  2.49 ms (-66%)",
                "PassTimer faces + SSAO: 1.12 ms",
                "PassTimer triangle_tiles: 0.33 ms",
                "Bare-hover redraws:     0",
            ];
            for (i, ln) in lines.iter().enumerate() {
                p.text(
                    egui::pos2(hud.left() + 10.0, hud.top() + 38.0 + i as f32 * 18.0),
                    egui::Align2::LEFT_CENTER,
                    *ln,
                    egui::FontId::monospace(10.2),
                    egui::Color32::from_rgb(216, 226, 255),
                );
            }
        });
        save_frame(&mut gpu, &cam, white_clear, "t16-perf")?;

        gpu.ui = None;
        for r in 0..scene.row_count() as u32 {
            let c = if r % 3 == 0 {
                [48, 87, 177]
            } else if r % 3 == 1 {
                [216, 182, 136]
            } else {
                [220, 75, 141]
            };
            gpu.set_object_color(r, false, Some(c));
        }
        let mut cam2 = Camera::new();
        cam2.fit(&gpu.bounds, 960.0 / 680.0);
        cam2.orbit(-32.0, -44.0);
        cam2.distance *= 0.56;
        cam2.update_position();
        refine_capture(&mut scene, &mut gpu, &cam2);
        save_frame(&mut gpu, &cam2, white_clear, "t16-oracle")?;
    }

    written.extend(ext_written);
    written.sort();
    Ok(written)
}

#[cfg(test)]
mod tutorial_stage_tests {
    use crate::app::stage::{deck, font, presentation::Presentation};

    const TUTORIAL_DECKS: [&str; 3] = [
        "tutorials-foundations",
        "tutorials-interaction",
        "tutorials-systems",
    ];

    #[test]
    fn tutorial_stage_diagrams_play_cleanly_without_overlaps_or_errors() {
        let font_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/stage/");
        let files: Vec<Vec<u8>> = font::FILES
            .iter()
            .map(|f| std::fs::read(format!("{font_dir}{f}")).expect("stage font"))
            .collect();
        let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1280.0, 720.0));

        for name in TUTORIAL_DECKS {
            let dir = format!("{}/examples/stage/{name}/", env!("CARGO_MANIFEST_DIR"));
            let idx_path = format!("{dir}index.yaml");
            if !std::path::Path::new(&idx_path).exists() {
                continue;
            }
            let fonts = font::Fonts::new(files.clone()).expect("fonts parse");
            let index = deck::index(&std::fs::read_to_string(&idx_path).unwrap()).unwrap();
            let chapter_files = index.chapters.clone();
            let chapters: Vec<deck::Chapter> = chapter_files
                .iter()
                .map(|c| {
                    let src = std::fs::read_to_string(format!("{dir}{c}")).unwrap();
                    deck::chapter(&src, c).unwrap_or_else(|e| panic!("{name}/{c}: {e}"))
                })
                .collect();
            let count = chapters.len();
            let mut p = Presentation::new(fonts, index, chapters, Vec::new(), 0.0);
            for ch in 0..count {
                let steps: usize = p.stage.chapters[ch]
                    .slides
                    .iter()
                    .map(|s| s.steps.len())
                    .sum();
                assert!(steps >= 1, "{name}/{} has no steps", chapter_files[ch]);
                for step in 1..=steps {
                    p.stage.open_at(ch + 1, step, 0.0);
                    p.stage.tick(10_000.0);
                    let mut cam = crate::camera::Camera::new();
                    let f = crate::app::stage::paint::Frame::new(screen, 1.0)
                        .stacked(screen, p.stage.split)
                        .side(0.0);
                    let (lens, _) = crate::app::stage::spatial::lens_headless(
                        &p.stage,
                        &mut cam,
                        &f,
                        [1280.0, 720.0],
                        10_000.0,
                    );
                    p.stage.lens = Some(lens);
                    p.stage.changed = true;
                    let ctx = egui::Context::default();
                    let _ = ctx.run_ui(
                        egui::RawInput {
                            screen_rect: Some(screen),
                            ..Default::default()
                        },
                        |root| {
                            let _ = p.paint(root.painter(), screen, 1.0);
                        },
                    );
                    assert!(
                        p.stage.errors.is_empty(),
                        "{name}/{} step {step} errors: {:?}",
                        chapter_files[ch],
                        p.stage.errors
                    );
                    let overlaps = p.stage.overlaps();
                    assert!(
                        overlaps.is_empty(),
                        "{name}/{} step {step} overlaps: {:?}",
                        chapter_files[ch],
                        overlaps
                    );
                }
            }
        }
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
