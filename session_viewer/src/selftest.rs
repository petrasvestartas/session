//! Headless render harness — native only.
//!
//! The point of this module is narrow and important: a shader change can be LOOKED AT on this
//! machine, as a PNG-ish image file, before it is shipped to a browser for somebody else to
//! judge by eye. Four broken builds is what it costs not to have one.
//!
//! It reuses the real pipeline: `Gpu::new_headless` builds the same stack without a surface, and
//! `Gpu::render_offscreen` runs the same `encode_frame` the swapchain path runs.

use crate::app::scene::Scene;
use crate::camera::Camera;
use crate::engine::gpu::Gpu;
use session_rust::{Session, Xform};

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
#[cfg(not(target_os = "linux"))]
fn rss_mb() -> f64 { 0.0 }

/// Load `.pb` files, frame them, render one frame, and write it out.
pub fn render_scene(files: &[(&str, Xform, f32, bool)], w: u32, h: u32, out: &str) -> String {
    let mut gpu = pollster::block_on(Gpu::new_headless(w, h)).expect("headless gpu");
    let mut scene = Scene::new();
    let incremental = std::env::var("VIEWER_INCREMENTAL").is_ok();
    // Staged RSS, so "the model costs 122 MB" can be attributed instead of guessed at. The three
    // numbers that matter are different levers: the file buffer is transient, the decode is the
    // protobuf intermediate, and the kernel figure is what the halfedge and the vertex/face maps
    // actually cost once built.
    let rss0 = rss_mb();
    for (path, place, px, only) in files {
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
        scene.add_file(name, session, place.clone(), *px, *only);
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
        let mb = |b: usize| b as f64 / 1.048576e6;
        let (v, i) = (t.verts.len(), t.idx.len());
        let (pipes, sph) = (t.pipes.len(), t.spheres.len());
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
    camera.fit(gpu.scene_min, gpu.scene_max, w as f64 / h as f64);
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
    let anchor = gpu.rebase_anchor(&origin, camera.distance_world());
    let view_proj = camera.view_proj_anchored(w as f64 / h as f64, &anchor);

    // The facing test depends on this being the real camera, so check it against the camera the
    // frame was actually built from - anchored world units, the space the instance table uses.
    {
        let solved = crate::math::eye_from_view_proj(&view_proj);
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
    if let Some(n) = std::env::var("VIEWER_FRAMES").ok().and_then(|v| v.parse::<usize>().ok()).map(|n| n.max(1)){
        let mut ms: Vec<f64> = Vec::new();
        for _ in 0..n {
            let t = std::time::Instant::now();
            let _ = gpu.render_offscreen(wgpu::Color {r: 0.9, g: 0.9, b:0.9, a:1.0}, &view_proj);
            ms.push(t.elapsed().as_secs_f64() * 1000.0);
        }
        ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!("frames: n={} median {:.1} ms ({:.0} fps) min {:.1} max {:.1} | cloud scale x{}",
            n, ms[n / 2], 1000.0 / ms[n / 2], ms[0], ms[n - 1], gpu.cloud_size);
    }

    let rgba = gpu.render_offscreen(wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 }, &view_proj);
    write_ppm(out, &rgba, w, h).expect("write ppm");

    // VIEWER_PICK="x,y": what point is under that pixel, read from the id target the point pass
    // wrote. Prints the row and whether the colour there is background, so a wrong answer is
    // visible rather than plausible.
    if let Ok(v) = std::env::var("VIEWER_PICK") {
        let mut it = v.split(',').filter_map(|t| t.trim().parse::<u32>().ok());
        if let (Some(px), Some(py)) = (it.next(), it.next()) {
            let i = ((py.min(h - 1) * w + px.min(w - 1)) * 4) as usize;
            let bg = rgba[i] >= 200 && rgba[i + 1] >= 200 && rgba[i + 2] >= 200;
            match gpu.pick_point(px, py).and_then(|r| gpu.cloud_row_of(r).map(|c| (r, c))) {
                Some((row, (inst, local))) => match scene.point_at(inst, local) {
                    Some(p) => println!("pick: ({px},{py}) row={row} doc='{}' instance={} local={} id={} pos=({:.0}, {:.0}, {:.0}) bg={bg}", p.doc, p.instance, p.local, p.id, p.local_pos[0], p.local_pos[1], p.local_pos[2]),
                    None => println!("pick: ({px},{py}) row={row} instance={inst} local={local} pos=UNRESOLVED bg={bg}"),
                },
                None => println!("pick: ({px},{py}) nothing bg={bg}"),
            }
        }
    }

    let ink = rgba.chunks_exact(4).filter(|p| p[0] < 200 || p[1] < 200 || p[2] < 200).count();
    format!(
        "wrote {out}  {w}x{h}  non-background pixels: {ink} ({:.1}%)\n",
        100.0 * ink as f64 / (w * h) as f64
    )
}

/// Bench BOTH line styles on the same loaded scene at two zooms (fit and far), same frames,
/// same camera - the flat-vs-tubes speed question answered on the real pipeline. Returns the
/// report; table sizes print on the way (they are IDENTICAL between styles - both lanes draw
/// the same 40 B/edge segment table, tubes just add a 6-sided unit template).
pub fn bench_scene(files: &[(&str, Xform)], w: u32, h: u32) -> String {
    use crate::engine::gpu::LineStyle;
    let mut gpu = pollster::block_on(Gpu::new_headless(w, h)).expect("headless gpu");
    let mut scene = Scene::new();
    for (path, place) in files {
        let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("cannot read {path}: {e}"));
        let session = Session::pb_loads(&bytes).unwrap_or_else(|e| panic!("cannot parse {path}: {e:?}"));
        let name = path.rsplit('/').next().unwrap_or(path).to_string();
        scene.add_file(name, session, place.clone(), 0.0, false);
    }
    scene.upload_to(&mut gpu);
    {
        let t = &scene.tables;
        println!("scene: {} edges ({:.1} MB segments), {} markers, {} verts",
            t.pipes.len(), t.pipes.len() as f64 * 40.0 / 1.048576e6, t.spheres.len(), t.verts.len());
    }
    let aspect = w as f64 / h as f64;
    let n: u32 = std::env::var("BENCH_FRAMES").ok().and_then(|v| v.parse().ok()).unwrap_or(60);
    let mut out = String::new();
    for (label, zoom) in [("fit", 0i32), ("far", -12)] {
        let mut camera = Camera::new();
        camera.fit(gpu.scene_min, gpu.scene_max, aspect);
        for _ in 0..zoom.abs() { camera.zoom(if zoom > 0 { 1.0 } else { -1.0 }); }
        let origin = camera.origin();
        let anchor = gpu.rebase_anchor(&origin, camera.distance_world());
        let vp = camera.view_proj_anchored(aspect, &anchor);
        for style in [LineStyle::Tubes, LineStyle::Flat] {
            gpu.line_style = style;
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
pub fn frame_profile(files: &[(&str, Xform, f32, bool)], w: u32, h: u32) -> String {
    let mut gpu = pollster::block_on(Gpu::new_headless(w, h)).expect("headless gpu");
    let mut scene = Scene::new();
    for (path, place, px, only) in files {
        let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("cannot read {path}: {e}"));
        let session = Session::pb_loads(&bytes).unwrap_or_else(|e| panic!("cannot parse {path}: {e:?}"));
        let name = path.rsplit('/').next().unwrap_or(path).to_string();
        scene.add_file(name, session, place.clone(), *px, *only);
    }
    scene.upload_to(&mut gpu);

    let aspect = w as f64 / h as f64;
    let n: usize = std::env::var("BENCH_FRAMES").ok().and_then(|v| v.parse().ok()).unwrap_or(120);
    let clear = wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 };
    let tex = gpu.device.create_texture(&wgpu::TextureDescriptor {
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
    camera.fit(gpu.scene_min, gpu.scene_max, aspect);
    let mut out = String::new();
    for (label, spin) in [("still", 0.0f32), ("moving", 0.35f32)] {
        let (mut uni, mut enc_ms, mut gpu_ms) = (Vec::new(), Vec::new(), Vec::new());
        for i in 0..n + 5 {
            camera.orbit(spin, 0.0);
            let origin = camera.origin();
            let anchor = gpu.rebase_anchor(&origin, camera.distance_world());
            let vp = camera.view_proj_anchored(aspect, &anchor);

            let t0 = std::time::Instant::now();
            let _ = gpu.clear(clear, &vp); // headless: uniforms, then returns
            let t1 = std::time::Instant::now();
            let mut encoder = gpu.device.create_command_encoder(&Default::default());
            gpu.encode_frame(&mut encoder, &view, clear);
            let t2 = std::time::Instant::now();
            gpu.queue.submit([encoder.finish()]);
            let _ = gpu.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
            let t3 = std::time::Instant::now();
            if i >= 5 { // warmup: pipeline + driver caches
                uni.push((t1 - t0).as_secs_f64() * 1000.0);
                enc_ms.push((t2 - t1).as_secs_f64() * 1000.0);
                gpu_ms.push((t3 - t2).as_secs_f64() * 1000.0);
            }
        }
        let med = |v: &mut Vec<f64>| { v.sort_by(|a, b| a.partial_cmp(b).unwrap()); v[v.len() / 2] };
        let (u, e, g) = (med(&mut uni), med(&mut enc_ms), med(&mut gpu_ms));
        out.push_str(&format!(
            "{label:>6}: uniforms {u:6.2} ms | encode {e:6.2} ms | gpu {g:6.2} ms | total {:6.2} ms ({:5.0} fps)\n",
            u + e + g, 1000.0 / (u + e + g)));
    }
    out
}
