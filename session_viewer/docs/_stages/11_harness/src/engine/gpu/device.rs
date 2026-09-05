//! Device negotiation: instance -> surface -> adapter -> device + queue -> surface format.
//! Produces one `DeviceSetup` and owns nothing afterwards. Headless callers pass no window
//! and get no surface.

use std::sync::Arc;
use winit::window::Window;

/// What `open` negotiated: the surface (None when headless), the device/queue pair, and the
/// surface configuration it was configured with.
pub struct DeviceSetup {
    pub surface: Option<wgpu::Surface<'static>>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
}

/// Set up the wgpu objects in order. `size` is the canvas in pixels; a zero side is clamped
/// to 1 so the surface can be configured.
pub async fn open(window: Option<Arc<Window>>, size: (u32, u32)) -> anyhow::Result<DeviceSetup> {
    // WebGPU only in the browser, never WebGL; Vulkan / Metal / DX12 for the native harness.
    let backends = if cfg!(target_arch = "wasm32") { wgpu::Backends::BROWSER_WEBGPU } else { wgpu::Backends::PRIMARY };
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });

    let surface = match &window {
        Some(w) => Some(instance.create_surface(w.clone())?),
        None => None,
    };

    // LowPower = the GPU the compositor runs on. On hybrid laptops the discrete GPU renders
    // fine but its frames cannot be shared to the compositor and the canvas stays black.
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            compatible_surface: surface.as_ref(),
            force_fallback_adapter: false,
        })
        .await?;
    let info = adapter.get_info();
    log::info!("adapter: {} ({:?}, {:?})", info.name, info.device_type, info.backend);
    if info.device_type == wgpu::DeviceType::Cpu {
        log::warn!("software adapter - rendering on the CPU will be slow");
    }

    // The default 128 MB storage-binding limit is smaller than one big cloud table.
    let hw = adapter.limits();
    let limits = wgpu::Limits {
        max_storage_buffer_binding_size: hw.max_storage_buffer_binding_size,
        max_buffer_size: hw.max_buffer_size,
        ..wgpu::Limits::default()
    };

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: limits,
            memory_hints: Default::default(),
            ..Default::default()
        })
        .await?;
    device.on_uncaptured_error(Arc::new(report_gpu_error));

    let (format, present_mode, alpha_mode) = match &surface {
        Some(s) => {
            let caps = s.get_capabilities(&adapter);
            let f = caps.formats.iter().find(|f| f.is_srgb()).copied().unwrap_or(caps.formats[0]);
            (f, caps.present_modes[0], caps.alpha_modes[0])
        }
        None => (wgpu::TextureFormat::Rgba8UnormSrgb, wgpu::PresentMode::Fifo, wgpu::CompositeAlphaMode::Auto),
    };
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: size.0.max(1),
        height: size.1.max(1),
        present_mode,
        alpha_mode,
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    if let Some(s) = &surface {
        s.configure(&device, &config);
    }

    Ok(DeviceSetup { surface, device, queue, config })
}

/// wgpu validation errors go to the log instead of a panic.
fn report_gpu_error(e: wgpu::Error) {
    log::error!("wgpu: {e}");
}
