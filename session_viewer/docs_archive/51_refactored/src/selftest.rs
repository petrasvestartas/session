//! Headless render harness — native only.
//!
//! The point of this module is narrow and important: a shader change can be LOOKED AT on this
//! machine, as a PNG-ish image file, before it is shipped to a browser for somebody else to
//! judge by eye. Four broken builds is what it costs not to have one.
//!
//! It reuses the real pipeline: `Gpu::new_headless` builds the same stack without a surface, and
//! `Gpu::render_offscreen` runs the same `encode_frame` the swapchain path runs.

use crate::app::manifest::Manifest;
use crate::app::scene::{FileDoc, Scene};
use crate::camera::Camera;
use crate::engine::gpu::{FrameInput, Gpu};
use crate::engine::performance::now_ms;
use session_rust::{Point, Session, Xform};

/// One file the harness loads: its path, where it sits, its per-file point size (px, 0 = the
/// pb's own) and whether its kernel `Session` is released after the walk.
pub struct SceneFile {
    pub path: String,
    pub place: Xform,
    pub point_px: f32,
    pub display_only: bool,
}

impl SceneFile {
    /// The harness's arguments. A `.json`/`.toml` argument is a SCENE MANIFEST, resolved the way
    /// the browser resolves it (paths relative to the assets root = the manifest's grandparent,
    /// a 3 m auto-grid), so what the harness renders is what the viewer renders; anything else is
    /// one .pb at its own origin - which is how a 0.156-unit bunny once became an invisible speck.
    pub fn from_args(args: &[String]) -> Vec<SceneFile> {
        let mut out = Vec::new();
        for p in args {
            if !(p.ends_with(".json") || p.ends_with(".toml")) {
                out.push(SceneFile { path: p.clone(), place: Xform::identity(), point_px: 0.0, display_only: false });
                continue;
            }
            let bytes = std::fs::read(p).unwrap_or_else(|e| panic!("cannot read manifest {p}: {e}"));
            let man = Manifest::parse(&bytes).unwrap_or_else(|| panic!("cannot parse manifest {p}"));
            let root = std::path::Path::new(p).parent().and_then(|d| d.parent())
                .unwrap_or(std::path::Path::new(".")).to_path_buf();
            for (i, item) in man.items.iter().enumerate() {
                let path = root.join(&item.file).to_string_lossy().into_owned();
                out.push(SceneFile { path, place: man.place(i, [3000.0, 3000.0]), point_px: item.point_size as f32, display_only: item.display_only });
            }
        }
        out
    }
}

/// Bytes as MB, for the table footprint lines.
fn mb(b: usize) -> f64 {
    b as f64 / 1.048576e6
}

/// The median of a sample, sorting it in place.
fn median(v: &mut Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

/// Write RGBA8 rows as a binary PPM (P6). No image crate needed, and every viewer opens it.
fn write_ppm(path: &str, rgba: &[u8], w: u32, h: u32) -> std::io::Result<()> {
    use std::io::Write;
    let mut f = std::io::BufWriter::new(std::fs::File::create(path)?);
    write!(f, "P6\n{w} {h}\n255\n")?;
    for px in rgba.chunks_exact(4) {
        f.write_all(&px[..3])?;
    }
    f.flush()
}

/// Resident set size in MB, straight from /proc. Coarse - it counts the allocator's slack and
/// never shrinks when memory is freed - which is exactly the right measure here, because a wasm32
/// heap does not shrink either: the PEAK is the budget.
#[cfg(target_os = "linux")]
fn rss_mb() -> f64 {
    std::fs::read_to_string("/proc/self/statm")
        .ok()
        .and_then(|s| s.split_whitespace().nth(1).and_then(|v| v.parse::<f64>().ok()))
        .map(|pages| pages * 4096.0 / 1.048576e6)
        .unwrap_or(0.0)
}
/// No /proc: the staged RSS lines print 0.
#[cfg(not(target_os = "linux"))]
fn rss_mb() -> f64 { 0.0 }

/// `VIEWER_GPU_REPORT=1`: wgpu's allocator report, one line per buffer/texture LABEL (bytes
/// summed over its allocations, largest first) and the totals - what the GPU actually holds,
/// which no CPU-side count can tell. `None` (a backend without the report) says so.
fn gpu_report(gpu: &Gpu) {
    if std::env::var("VIEWER_GPU_REPORT").is_err() {
        return;
    }
    let Some(report) = gpu.ctx.device.generate_allocator_report() else {
        println!("gpu report: unavailable on this backend");
        return;
    };
    let mut by_label: std::collections::BTreeMap<String, (u64, usize)> = std::collections::BTreeMap::new();
    for a in &report.allocations {
        let e = by_label.entry(a.name.clone()).or_insert((0, 0));
        e.0 += a.size;
        e.1 += 1;
    }
    let mut rows: Vec<(String, u64, usize)> = by_label.into_iter().map(|(k, (b, n))| (k, b, n)).collect();
    rows.sort_by(|a, b| b.1.cmp(&a.1));
    println!("gpu report: {} allocations, {:.1} MiB allocated, {:.1} MiB reserved in {} blocks",
        report.allocations.len(), mb(report.total_allocated_bytes as usize), mb(report.total_reserved_bytes as usize), report.blocks.len());
    for (label, bytes, n) in rows {
        println!("  {label:<28} {:>9.2} MiB  x{n}", mb(bytes as usize));
    }
}

/// Load `.pb` files, frame them, render one frame, and write it out.
pub fn render_scene(files: &[SceneFile], size: (u32, u32), out: &str) -> String {
    let (w, h) = size;
    let mut gpu = pollster::block_on(Gpu::new_headless(w, h)).expect("headless gpu");
    let mut scene = Scene::new();
    let incremental = std::env::var("VIEWER_INCREMENTAL").is_ok();
    // Staged RSS, so "the model costs 122 MB" can be attributed instead of guessed at. The three
    // numbers that matter are different levers: the file buffer is transient, the decode is the
    // protobuf intermediate, and the kernel figure is what the halfedge and the vertex/face maps
    // actually cost once built.
    let rss0 = rss_mb();
    for f in files {
        let path = &f.path;
        let t0 = std::time::Instant::now();
        let Ok(bytes) = std::fs::read(path) else { return format!("could not read {path}\n") };
        let nbytes = bytes.len();
        let rss_read = rss_mb();
        let t_read = t0.elapsed();
        let Ok(session) = Session::pb_loads(&bytes) else { return format!("could not parse {path}\n") };
        let t_decode = t0.elapsed() - t_read;
        let rss_parsed = rss_mb();
        drop(bytes);
        let name = path.rsplit('/').next().unwrap_or(path).to_string();
        println!(
            "  {name}: file {:.1} MB | after read {:.1} MB | after decode+build {:.1} MB (+{:.1}) | read {:?} decode {:?}",
            nbytes as f64 / 1.048576e6, rss_read - rss0, rss_parsed - rss0, rss_parsed - rss_read, t_read, t_decode
        );
        scene.add_file(FileDoc { name, session, place: f.place.clone(), point_px: f.point_px, display_only: f.display_only });
        println!("  after walk into GPU tables: {:.1} MB | walk {:?}", rss_mb() - rss0, t0.elapsed() - t_read - t_decode);
        // VIEWER_INCREMENTAL=1 uploads after EVERY file, which is what the browser does - each
        // fetched document is appended live. Batching all the files and uploading once (the
        // default here) hides exactly the cost that matters there: whether a lane re-sends the
        // whole scene per file or only the new rows.
        if incremental {
            let tu = std::time::Instant::now();
            scene.upload_to(&mut gpu);
            println!("  upload {:?} | RSS {:.1} MB", tu.elapsed(), rss_mb() - rss0);
        }
    }
    // Table footprint, before the upload hands them to the GPU: the numbers to quote when asking
    // "what does this model cost", and the ones that move when a struct is repacked.
    {
        let t = &scene.tables;
        let (v, i) = (t.arena.verts.len(), t.arena.idx.len());
        let (pipes, sph) = (t.seg.pipes.len(), t.glyph.spheres.len());
        println!(
            "tables: {v} verts {:.1} MB | {i} indices {:.1} MB | {pipes} edges {:.1} MB | {sph} markers {:.1} MB | total {:.1} MB",
            mb(v * std::mem::size_of::<session_rust::RenderVertex>()),
            mb(i * 4),
            mb(pipes * std::mem::size_of::<crate::engine::gpu::CylinderSegment>()),
            mb(sph * std::mem::size_of::<crate::engine::gpu::GlyphPoint>()),
            mb(v * std::mem::size_of::<session_rust::RenderVertex>() + i * 4
                + pipes * std::mem::size_of::<crate::engine::gpu::CylinderSegment>()
                + sph * std::mem::size_of::<crate::engine::gpu::GlyphPoint>()),
        );
    }

    if !incremental { scene.upload_to(&mut gpu); }

    // VIEWER_REBUILD=1 re-walks every document from its kernel Session and re-uploads from
    // scratch - the path a visibility toggle or a geometry edit takes. Every lane appends now, so
    // a rebuild has to REWIND every lane; forget one and the re-walked scene lands behind the copy
    // already on the GPU. The frame must come out pixel-identical to the same scene loaded once.
    if std::env::var("VIEWER_REBUILD").is_ok() {
        let t = std::time::Instant::now();
        scene.rebuild(&mut gpu);
        println!("rebuild {:?} | RSS {:.1} MB", t.elapsed(), rss_mb() - rss0);
    }

    let mut camera = Camera::new();
    camera.fit(gpu.bounds.min, gpu.bounds.max, w as f64 / h as f64);
    // One canned view is one sample of the failure space, and depth artifacts on mesh edges are
    // ANGLE-dependent - a face only grazes the eye from some directions. VIEWER_ORBIT="dx,dy"
    // orbits before framing so a sweep can be rendered and every frame looked at.
    if let Ok(o) = std::env::var("VIEWER_ORBIT") {
        let mut it = o.split(',').filter_map(|v| v.trim().parse::<f32>().ok());
        camera.orbit(it.next().unwrap_or(0.0), it.next().unwrap_or(0.0));
    }
    // VIEWER_ORTHO=1 flips to the orthographic projection after framing - the ink lanes take a
    // different uniform path there (ortho_h > 0), and it is where "lines through faces" shows.
    if std::env::var("VIEWER_ORTHO").is_ok() {
        camera.toggle_projection();
    }
    // VIEWER_VIEW=top|front|right|iso snaps to a named view (ortho), like the viewer's 1-7 keys.
    if let Ok(v) = std::env::var("VIEWER_VIEW") {
        use crate::camera::View;
        camera.set_view(match v.as_str() {
            "top" => View::Top, "bottom" => View::Bottom, "front" => View::Front, "right" => View::Right, _ => View::Iso,
        });
    }
    // VIEWER_ZOOM dollies in after framing. Needed because the interesting failures are the ones
    // where geometry crosses the eye plane, and a fit view never gets near that.
    if let Ok(z) = std::env::var("VIEWER_ZOOM") {
        if let Ok(n) = z.trim().parse::<i32>() {
            for _ in 0..n.abs() { camera.zoom(if n > 0 { 1.0 } else { -1.0 }); }
        }
    }
    let origin = camera.origin();
    let anchor = gpu.rebase_anchor(&origin, camera.distance_world(), now_ms()).anchor;
    let view_proj = camera.view_proj_anchored(w as f64 / h as f64, &anchor);
    let input = FrameInput { view_proj, clear: wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 }, now_ms: now_ms() };

    // The facing test depends on this being the real camera, so check it against the camera the
    // frame was actually built from - anchored world units, the space the instance table uses.
    {
        let solved = crate::math::eye_from_view_proj(&input.view_proj);
        let sc = camera.unit.to_meters();
        let truth = [0usize, 1, 2].map(|k| ((camera.position[k] / sc) - anchor[k]) as f32);
        let err = (0..3).map(|k| (solved[k] - truth[k]).powi(2)).sum::<f32>().sqrt();
        let mag = (0..3).map(|k| truth[k] * truth[k]).sum::<f32>().sqrt().max(1.0);
        // Silent unless it actually drifts - the facing test is only as good as this.
        if err / mag > 1e-4 {
            println!("EYE MISMATCH: solved {solved:?} truth {truth:?} rel err {:.3e}", err / mag);
        }
    }

    // VIEWER_FRAMES=N times N full offscreen frames
    // each one submits and reads back
    // so the wall clock includes the gpu actually finishing and reports the median
    // The camera is STILL across these frames, so the splat static skip applies: a cloud scene
    // measures its resolve, not its compute. `bench_frame` has the moving leg.
    if let Some(n) = std::env::var("VIEWER_FRAMES").ok().and_then(|v| v.parse::<usize>().ok()).map(|n| n.max(1)){
        let mut ms: Vec<f64> = Vec::new();
        for _ in 0..n {
            let t = std::time::Instant::now();
            let _ = gpu.render_offscreen(&input);
            ms.push(t.elapsed().as_secs_f64() * 1000.0);
        }
        ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!("frames (still camera): n={} median {:.1} ms ({:.0} fps) min {:.1} max {:.1} | cloud scale x{}",
            n, ms[n / 2], 1000.0 / ms[n / 2], ms[0], ms[n - 1], gpu.view.cloud_size);
    }

    let rgba = gpu.render_offscreen(&input);
    write_ppm(out, &rgba, w, h).expect("write ppm");
    gpu_report(&gpu);

    // VIEWER_CLEAR=1 clears the scene after the frame and reports again: what a cleared scene
    // still holds, on both sides, is what `Gpu::release` exists to hand back.
    if std::env::var("VIEWER_CLEAR").is_ok() {
        scene.clear(&mut gpu);
        let _ = gpu.ctx.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
        println!("after clear: RSS {:.1} MB", rss_mb() - rss0);
        gpu_report(&gpu);
    }

    let ink = rgba.chunks_exact(4).filter(|p| p[0] < 200 || p[1] < 200 || p[2] < 200).count();
    format!(
        "wrote {out}  {w}x{h}  non-background pixels: {ink} ({:.1}%)\n",
        100.0 * ink as f64 / (w * h) as f64
    )
}

/// Read and parse one file for the benches, panicking on a bad path - a bench has no report to
/// put an error in.
fn load(f: &SceneFile) -> FileDoc {
    let path = &f.path;
    let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("cannot read {path}: {e}"));
    let session = Session::pb_loads(&bytes).unwrap_or_else(|e| panic!("cannot parse {path}: {e:?}"));
    let name = path.rsplit('/').next().unwrap_or(path).to_string();
    FileDoc { name, session, place: f.place.clone(), point_px: f.point_px, display_only: f.display_only }
}

/// Bench BOTH line styles on the same loaded scene at two zooms (fit and far), same frames,
/// same camera - the flat-vs-tubes speed question answered on the real pipeline. Returns the
/// report; table sizes print on the way (they are IDENTICAL between styles - both lanes draw
/// the same 40 B/edge segment table, tubes just add a 6-sided unit template).
pub fn bench_scene(files: &[SceneFile], size: (u32, u32)) -> String {
    use crate::engine::gpu::LineStyle;
    let (w, h) = size;
    let mut gpu = pollster::block_on(Gpu::new_headless(w, h)).expect("headless gpu");
    let mut scene = Scene::new();
    for f in files {
        scene.add_file(load(f));
    }
    scene.upload_to(&mut gpu);
    // GPU-side counts: the CPU tables are dropped by the upload, so they would all print 0.
    println!("scene: {} edges ({:.1} MB segments), {} markers, {} verts | {} ribbons, {} dots, {} cloud points",
        gpu.segments.pipe_count(), gpu.segments.pipe_count() as f64 * 40.0 / 1.048576e6, gpu.glyphs.sphere_count(), gpu.arena.vert_count(),
        gpu.segments.ribbon_count(), gpu.glyphs.dot_count(), gpu.cloud.point_count);
    let aspect = w as f64 / h as f64;
    let n: u32 = std::env::var("BENCH_FRAMES").ok().and_then(|v| v.parse().ok()).unwrap_or(60);
    let mut out = String::new();
    for (label, zoom) in [("fit", 0i32), ("far", -12)] {
        let mut camera = Camera::new();
        camera.fit(gpu.bounds.min, gpu.bounds.max, aspect);
        for _ in 0..zoom.abs() { camera.zoom(if zoom > 0 { 1.0 } else { -1.0 }); }
        let origin = camera.origin();
        let anchor = gpu.rebase_anchor(&origin, camera.distance_world(), now_ms()).anchor;
        let vp = camera.view_proj_anchored(aspect, &anchor);
        for style in [LineStyle::Tubes, LineStyle::Flat] {
            gpu.view.line_style = style;
            let secs = gpu.bench_frames(&vp, n);
            out.push_str(&format!("{label:>4} {style:?}: {:7.2} ms/frame ({:5.0} fps)\n",
                secs * 1000.0 / n as f64, n as f64 / secs));
        }
    }
    out
}

/// Where a frame's milliseconds actually go, split three ways, for a STILL and a MOVING camera.
///
/// `bench_frames` above answers "how fast is the frame"; it cannot answer "whose fault is it",
/// and on a scene with 155k object rows the answer turned out to be neither the GPU nor the
/// draw calls. The three legs:
///
///   uniforms  `Gpu::clear` on a headless Gpu writes the per-frame uniform blocks and returns
///             before it touches a surface - so timing it IS `write_frame_uniforms`, which is
///             also where the per-object `FLAG_INSIDE` sweep lives.
///   encode    building the command buffer: the splat records, the render pass, the draws.
///   gpu       submit + poll to completion.
///
/// The moving pass orbits a hair per frame, because every cache in the frame path (the splat
/// static-skip, the re-anchor throttle, the inside-flag change detection) keys on the camera not
/// having moved - and "constant quality during motion" is exactly the case that must stay fast.
pub fn frame_profile(files: &[SceneFile], size: (u32, u32)) -> String {
    let (w, h) = size;
    let mut gpu = pollster::block_on(Gpu::new_headless(w, h)).expect("headless gpu");
    let mut scene = Scene::new();
    for f in files {
        scene.add_file(load(f));
    }
    scene.upload_to(&mut gpu);

    let aspect = w as f64 / h as f64;
    let n: usize = std::env::var("BENCH_FRAMES").ok().and_then(|v| v.parse().ok()).unwrap_or(120);
    let clear = wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 };
    let tex = gpu.ctx.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("profile.color"),
        size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: gpu.config.format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let view = tex.create_view(&wgpu::TextureViewDescriptor::default());

    let mut camera = Camera::new();
    camera.fit(gpu.bounds.min, gpu.bounds.max, aspect);
    let mut out = String::new();
    for (label, spin) in [("still", 0.0f32), ("moving", 0.35f32)] {
        let (mut uni, mut enc_ms, mut gpu_ms) = (Vec::new(), Vec::new(), Vec::new());
        for i in 0..n + 5 {
            camera.orbit(spin, 0.0);
            let origin = camera.origin();
            let now = now_ms();
            let anchor = gpu.rebase_anchor(&origin, camera.distance_world(), now).anchor;
            let vp = camera.view_proj_anchored(aspect, &anchor);

            let input = FrameInput { view_proj: vp, clear, now_ms: now };
            let t0 = std::time::Instant::now();
            let _ = gpu.clear(&input); // headless: uniforms, then returns
            let t1 = std::time::Instant::now();
            let mut encoder = gpu.ctx.device.create_command_encoder(&Default::default());
            gpu.encode_frame(&mut encoder, &view, clear);
            let t2 = std::time::Instant::now();
            gpu.ctx.queue.submit([encoder.finish()]);
            let _ = gpu.ctx.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
            let t3 = std::time::Instant::now();
            if i >= 5 { // warmup: pipeline + driver caches
                uni.push((t1 - t0).as_secs_f64() * 1000.0);
                enc_ms.push((t2 - t1).as_secs_f64() * 1000.0);
                gpu_ms.push((t3 - t2).as_secs_f64() * 1000.0);
            }
        }
        let (u, e, g) = (median(&mut uni), median(&mut enc_ms), median(&mut gpu_ms));
        out.push_str(&format!(
            "{label:>6}: uniforms {u:6.2} ms | encode {e:6.2} ms | gpu {g:6.2} ms | total {:6.2} ms ({:5.0} fps)\n",
            u + e + g, 1000.0 / (u + e + g)));
    }
    out.push_str(&rebase_profile(&mut gpu, &camera));
    gpu_report(&gpu);
    out
}

/// Bytes of viewer bookkeeping per OBJECT ROW: the scene is loaded display_only (the walk
/// releases every kernel document), uploaded, and what `live` still counts afterwards -
/// `Scene`'s columns plus `InstanceTable`'s mirrors - is divided by the object count. `live`
/// is the caller's counting allocator (examples/probe_objects.rs), read in MB.
pub fn object_bytes(files: &[SceneFile], live: fn() -> f64) -> String {
    let mut gpu = pollster::block_on(Gpu::new_headless(900, 700)).expect("headless gpu");
    let mut scene = Scene::new();
    let _ = gpu.ctx.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
    let base = live();
    for f in files {
        let doc = load(f);
        scene.add_file(FileDoc { display_only: true, ..doc });
        scene.upload_to(&mut gpu);
    }
    let _ = gpu.ctx.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
    let objects = gpu.objects.len() as f64;
    let held = live() - base;
    let docs = scene.docs.len();
    drop(scene);
    let gpu_side = live() - base;
    format!(
        "{} objects | live after upload {held:.1} MB = {:.0} B/object | Scene dropped ({docs} docs): {gpu_side:.1} MB = {:.0} B/object in InstanceTable\n",
        objects as u64, held * 1.048576e6 / objects, gpu_side * 1.048576e6 / objects)
}

/// What one forced re-anchor costs: the CPU loop over every object row plus the write
/// (`rebase`), then the submit + poll that lands it (`gpu`). Ten of them, the origin thrown past
/// the threshold band and the clock a second past the 200 ms throttle each time, medians reported.
fn rebase_profile(gpu: &mut Gpu, camera: &Camera) -> String {
    let base = camera.origin();
    let (mut cpu, mut gpu_ms) = (Vec::new(), Vec::new());
    let clock = now_ms();
    for i in 0..10 {
        let far = if i % 2 == 0 { 1.0e6 } else { 0.0 };
        let origin = Point::new(base[0] + far, base[1], base[2]);
        let t0 = std::time::Instant::now();
        let _ = gpu.rebase_anchor(&origin, camera.distance_world(), clock + 1000.0 * (i + 1) as f64);
        let t1 = std::time::Instant::now();
        gpu.ctx.queue.submit([]);
        let _ = gpu.ctx.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
        cpu.push((t1 - t0).as_secs_f64() * 1000.0);
        gpu_ms.push(t1.elapsed().as_secs_f64() * 1000.0);
    }
    format!("rebase: {} rows | cpu+write {:6.2} ms | gpu {:6.2} ms\n", gpu.objects.len(), median(&mut cpu), median(&mut gpu_ms))
}
