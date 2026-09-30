#[cfg(not(target_arch = "wasm32"))]
include!("../src/lib.rs");

#[cfg(not(target_arch = "wasm32"))]
fn main() -> anyhow::Result<()> {
    use engine::gpu::{FrameInput, Gpu};
    use std::io::Write;

    let mut gpu = pollster::block_on(Gpu::new_headless(640, 480))?;
    gpu.view.show_grid = false;
    let input = FrameInput {
        view_proj: session_rust::Xform::new(),
        clear: wgpu::Color::WHITE,
        now_ms: 0.0,
    };
    let pixels = gpu.render_offscreen(&input);
    let mut file = std::fs::File::create("lesson.ppm")?;
    file.write_all(b"P6\n640 480\n255\n")?;

    for pixel in pixels.chunks_exact(4) {
        file.write_all(&pixel[..3])?;
    }

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {}

// From a lesson crate: cargo run --example course_frame --target x86_64-unknown-linux-gnu -j4.
