use std::io::Write;
use viewer_journey::renderer::Renderer;

// COURSE_SIZE

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &log::Record<'_>) {
        eprintln!("{}", record.args());
    }

    fn flush(&self) {}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    log::set_logger(&Logger).unwrap();
    log::set_max_level(log::LevelFilter::Warn);
    pollster::block_on(render())
}

async fn render() -> Result<(), Box<dyn std::error::Error>> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        ..Default::default()
    }).await?;
    let (device, queue) = adapter.request_device(&Default::default()).await?;
    device.on_uncaptured_error(std::sync::Arc::new(|error| panic!("GPU validation: {error}")));
    let format = wgpu::TextureFormat::Rgba8UnormSrgb;
    // COURSE_NEW
    let texture = renderer.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("lesson evidence"),
        size: wgpu::Extent3d { width: WIDTH, height: HEIGHT, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let view = texture.create_view(&Default::default());
    // COURSE_DRAW
    let row_bytes = (WIDTH * 4).div_ceil(256) * 256;
    let buffer = renderer.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("lesson readback"),
        size: u64::from(row_bytes) * u64::from(HEIGHT),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut encoder = renderer.device.create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(
        texture.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0, bytes_per_row: Some(row_bytes), rows_per_image: Some(HEIGHT),
            },
        },
        texture.size(),
    );
    renderer.queue.submit([encoder.finish()]);
    let (sender, receiver) = std::sync::mpsc::channel();
    buffer.slice(..).map_async(wgpu::MapMode::Read, move |result| sender.send(result).unwrap());
    renderer.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None })?;
    receiver.recv()??;
    let mapped = buffer.slice(..).get_mapped_range();
    let pixels: Vec<u8> = mapped.chunks_exact(row_bytes as usize)
        .flat_map(|row| row[..WIDTH as usize * 4].iter().copied()).collect();
    verify_pixels(&pixels);
    let mut output = std::fs::File::create("frame.ppm")?;
    write!(output, "P6\n{WIDTH} {HEIGHT}\n255\n")?;
    for pixel in pixels.chunks_exact(4) {
        output.write_all(&pixel[..3])?;
    }
    Ok(())
}

fn verify_pixels(pixels: &[u8]) {
    let mode = std::env::var("COURSE_FRAME").expect("The course runner supplies the expected frame");
    let background = if mode == "light" { [243, 243, 243] } else { [255, 255, 255] };
    assert!(near(&pixels[..3], background), "Unexpected background: {:?}", &pixels[..3]);
    let changed = pixels.chunks_exact(4).filter(|pixel| !near(&pixel[..3], background)).count();

    if mode == "clear" {
        assert_eq!(changed, 0, "The clear frame contains unexpected geometry");
    } else {
        assert!((49_000..52_000).contains(&changed), "Triangle coverage: {changed}");
        let centre = (240 * 640 + 320) * 4;
        assert!(near(&pixels[centre..centre + 3], [243, 137, 179]), "Missing pink triangle centre");
    }
}

fn near(pixel: &[u8], expected: [u8; 3]) -> bool {
    pixel.iter().zip(expected).all(|(&a, b)| a.abs_diff(b) <= 2)
}
