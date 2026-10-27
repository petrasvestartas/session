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
    let backends = if cfg!(windows) { wgpu::Backends::DX12 }
        else if cfg!(target_os = "macos") { wgpu::Backends::METAL }
        else { wgpu::Backends::VULKAN };
    let mut backend_options = wgpu::BackendOptions::default();
    if cfg!(windows) {
        let compiler = std::path::PathBuf::from(std::env::var("VIEWER_DXC_PATH")
            .map_err(|_| "Set VIEWER_DXC_PATH to the task-local DXC 1.9 dxcompiler.dll")?);
        if !compiler.is_absolute() || !compiler.is_file()
            || !compiler.with_file_name("dxil.dll").is_file() {
            return Err("Task-local DXC and dxil DLLs are required".into());
        }
        backend_options.dx12.shader_compiler = wgpu::Dx12Compiler::DynamicDxc {
            dxc_path: compiler.to_string_lossy().into_owned(),
        };
    }
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options,
        display: None,
    });
    let adapter = if cfg!(windows) {
        instance.enumerate_adapters(backends).await.into_iter().find(|adapter| {
            let info = adapter.get_info();
            info.backend == wgpu::Backend::Dx12 && info.device_type == wgpu::DeviceType::Cpu
                && info.vendor == 0x1414 && info.device == 0x8c
        }).ok_or("Windows frame evidence requires the Microsoft WARP DX12 adapter")?
    } else { instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        ..Default::default()
    }).await? };
    eprintln!("Native frame adapter: {:?}", adapter.get_info());
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
