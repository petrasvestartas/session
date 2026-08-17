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

/// Load `.pb` files, frame them, render one frame, and write it out.
pub fn render_scene(files: &[(&str, Xform)], w: u32, h: u32, out: &str) -> String {
    let mut gpu = pollster::block_on(Gpu::new_headless(w, h)).expect("headless gpu");
    let mut scene = Scene::new();
    for (path, place) in files {
        let Ok(bytes) = std::fs::read(path) else { return format!("could not read {path}\n") };
        let Ok(session) = Session::pb_loads(&bytes) else { return format!("could not parse {path}\n") };
        let name = path.rsplit('/').next().unwrap_or(path).to_string();
        scene.add_file(name, session, place.clone());
    }
    scene.upload_to(&mut gpu);

    let mut camera = Camera::new();
    camera.fit(gpu.scene_min, gpu.scene_max, w as f64 / h as f64);
    // One canned view is one sample of the failure space, and depth artifacts on mesh edges are
    // ANGLE-dependent - a face only grazes the eye from some directions. VIEWER_ORBIT="dx,dy"
    // orbits before framing so a sweep can be rendered and every frame looked at.
    if let Ok(o) = std::env::var("VIEWER_ORBIT") {
        let mut it = o.split(',').filter_map(|v| v.trim().parse::<f32>().ok());
        camera.orbit(it.next().unwrap_or(0.0), it.next().unwrap_or(0.0));
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

    let rgba = gpu.render_offscreen(wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 }, &view_proj);
    write_ppm(out, &rgba, w, h).expect("write ppm");

    let ink = rgba.chunks_exact(4).filter(|p| p[0] < 200 || p[1] < 200 || p[2] < 200).count();
    format!(
        "wrote {out}  {w}x{h}  non-background pixels: {ink} ({:.1}%)\n",
        100.0 * ink as f64 / (w * h) as f64
    )
}
